mod config;

use std::path::PathBuf;

use clap::{Parser, Subcommand};

use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};

#[derive(Debug, Parser)]
struct Cli {
    /// Use a custom config
    #[arg(short, long)]
    config: Option<PathBuf>,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Debug, Subcommand)]
enum Commands {
    Dbg {
        #[arg(short, long)]
        print_cli: bool,
    },
    /// TODO: 1. token login 2. select course from list of options
    Login {
        username: String,
        // password: String,
        // course: String,
    },
    /// Download an exercise and save it in the exercise directory
    Download {
        #[arg(short, long)]
        url: String,
    },
    /// Submit an exercise to SmartBeans
    Submit { path: PathBuf },
    /// Run the tests for a local exercise
    Test { path: PathBuf },
}

/// Hash password using Argon2
fn hash_password(password: &str) -> anyhow::Result<String> {
    let salt = SaltString::generate(&mut OsRng);

    // Argon2 with default params (Argon2id v19)
    let argon2 = Argon2::default();

    // Hash password to PHC string ($argon2id$v=19$...)
    let password_hash = argon2
        .hash_password(password.as_bytes(), &salt)?
        .to_string();

    // Verify password against PHC string.
    //
    // NOTE: hash params from `parsed_hash` are used instead of what is configured in the
    // `Argon2` instance.
    // let parsed_hash = PasswordHash::new(&password_hash).unwrap();
    // assert!(Argon2::default()
    //     .verify_password(password.as_bytes(), &parsed_hash)
    //     .is_ok());

    Ok(password_hash)
}

/// TODO store the password securely.
/// TODO: token login. Needs changes to the web app.
fn login(username: &str) -> anyhow::Result<()> {
    let password = rpassword::prompt_password("Password: ")?;
    println!("your password: {}", password);
    println!("hashed: {}", hash_password(&password)?);

    Ok(())
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Dbg { print_cli }) => {
            if *print_cli {
                dbg!(cli);
            }
        }
        Some(Commands::Login { username }) => {
            let _ = login(username);
        }

        None => {}

        _ => {}
    }
}
