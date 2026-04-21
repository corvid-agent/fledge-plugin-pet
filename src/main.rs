use std::time::{SystemTime, UNIX_EPOCH};

use clap::{Parser, Subcommand, ValueEnum};
use corvid_pet::persistence::{self, PetState};
use corvid_pet::{Event, Mood, Personality, Pet, Species};

#[derive(Parser)]
#[command(name = "fledge-pet", version, about = "Your corvid dev companion — powered by corvid-pet")]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,

    #[arg(long, default_value = "Pip", global = true)]
    name: String,
}

#[derive(Subcommand)]
enum Commands {
    /// Feed your pet with dev activity
    Feed {
        #[arg(value_enum)]
        activity: Activity,
    },
    /// Play with your pet
    Play,
    /// Show pet stats
    Status,
    /// Rename your pet
    Rename {
        new_name: String,
    },
    /// Reset your pet (hatch a new egg)
    Reset,
}

#[derive(Clone, ValueEnum)]
enum Activity {
    Commit,
    Test,
    Review,
    Lint,
    Deploy,
}

fn now() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
}

fn load_pet(name: &str) -> Pet {
    match persistence::load_pet(name) {
        Ok(state) => {
            let mut pet = state.to_pet();
            pet.tick(now());
            pet
        }
        Err(_) => Pet::new(name.to_string(), Species::Crow)
            .with_simulation(Personality::Curious, now()),
    }
}

fn save_pet(pet: &Pet, name: &str) {
    let mut state = PetState::from_pet(pet);
    state.last_saved = Some(now());
    if let Err(e) = persistence::save_pet(&state, name) {
        eprintln!("Warning: could not save pet state: {e}");
    }
}

fn main() {
    let cli = Cli::parse();
    let pet_name = &cli.name;

    match &cli.command {
        None | Some(Commands::Status) => {
            let pet = load_pet(pet_name);
            println!("{}", pet.render_colored());

            if let (Some(stats), Some(stage), Some(age)) =
                (pet.stats(), pet.life_stage(), pet.age_display())
            {
                println!();
                println!("  {} the {} ({})", pet.name(), stage, age);
                println!("  Mood: {}", pet.mood());
                println!();
                println!("  Hunger:    {:>5.1}%", stats.hunger);
                println!("  Energy:    {:>5.1}%", stats.energy);
                println!("  Happiness: {:>5.1}%", stats.happiness);
                println!("  Health:    {:>5.1}%", stats.health);
                println!();
                println!("  Feed me with dev activity:");
                println!("    fledge pet feed commit    (success event)");
                println!("    fledge pet feed test      (progress event)");
                println!("    fledge pet feed review    (success event)");
                println!("    fledge pet feed lint      (progress event)");
                println!("    fledge pet feed deploy    (success event)");

                let critical = stats.critical_needs();
                if !critical.is_empty() {
                    let names: Vec<_> = critical.iter().map(|n| n.description()).collect();
                    println!("\n  ⚠ Needs attention: {}", names.join(", "));
                }
            } else {
                println!("\n  {} — a new corvid appears!", pet.name());
                println!("  {}", pet.comment());
            }
        }

        Some(Commands::Feed { activity }) => {
            let mut pet = load_pet(pet_name);

            let (event, label) = match activity {
                Activity::Commit => (Event::Success, "commit"),
                Activity::Test => (Event::Progress, "test run"),
                Activity::Review => (Event::Success, "code review"),
                Activity::Lint => (Event::Progress, "lint pass"),
                Activity::Deploy => (Event::Success, "deploy"),
            };

            pet.react(event);
            if let Some(result) = pet.feed(now()) {
                println!("{}", pet.render_colored());
                println!("\n  *caw!* Tasty {label}! {}", result.message);
            } else {
                println!("{}", pet.render_colored());
                println!("\n  *caw!* {label} received!");
            }
            println!("  {}", pet.comment());
            save_pet(&pet, pet_name);
        }

        Some(Commands::Play) => {
            let mut pet = load_pet(pet_name);
            if let Some(result) = pet.play(now()) {
                println!("{}", pet.render_colored());
                println!("\n  {}", result.message);
            } else {
                pet.set_mood(Mood::Happy);
                println!("{}", pet.render_colored());
                println!("\n  *happy chirp*");
            }
            save_pet(&pet, pet_name);
        }

        Some(Commands::Rename { new_name }) => {
            let pet = load_pet(pet_name);
            let mut state = PetState::from_pet(&pet);
            state.name = new_name.clone();
            if let Err(e) = persistence::delete_pet(pet_name) {
                eprintln!("Warning: {e}");
            }
            if let Err(e) = persistence::save_pet(&state, new_name) {
                eprintln!("Warning: {e}");
            }
            println!("  Your corvid is now named {}!", new_name);
        }

        Some(Commands::Reset) => {
            if let Err(e) = persistence::delete_pet(pet_name) {
                eprintln!("Warning: {e}");
            }
            println!("  Pet reset. A new egg has appeared...");
            let pet = Pet::new(pet_name.to_string(), Species::Crow)
                .with_simulation(Personality::Curious, now());
            println!("{}", pet.render_colored());
            save_pet(&pet, pet_name);
        }
    }
}
