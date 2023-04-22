mod auth;
mod config;
mod tasks;

use std::{collections::HashMap, default, fmt::format, path::PathBuf};

use clap::{Parser, Subcommand};

use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use config::Config;
use reqwest::header;
use tasks::submit_task;

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
    Configure {
        #[arg(short, long)]
        username: String,
        #[arg(short, long)]
        course: String,
        #[arg(long)]
        host: String,
    },
    /// TODO: 1. token login 2. select course from list of options
    Login,
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

fn check_if_configured() -> anyhow::Result<()> {
    let cfg = Config::load()?;

    if cfg.user.is_empty() || cfg.course.is_empty() || cfg.host.is_empty() {
        anyhow::bail!("Please configure the CLI first");
    }

    Ok(())
}

/// send a request to the SmartBeans API with the authorization header
/// `Authorization: Bearer <session token>`
fn send_with_authorization_header() -> anyhow::Result<()> {
    let cfg = Config::load().unwrap();

    let client = reqwest::blocking::Client::new();

    let res = client
        .get(&cfg.host)
        .header(
            header::AUTHORIZATION,
            format!("Bearer {}", cfg.token.unwrap()),
        )
        .send()?;

    dbg!(res);

    Ok(())
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    if cli.config.is_some() {
        Config::store(&Config::default())?;
        return Ok(());
    }

    match &cli.command {
        Some(Commands::Dbg { print_cli }) => {
            if *print_cli {
                dbg!(cli);
            }
        }

        Some(Commands::Configure {
            username,
            course,
            host,
        }) => {
            let mut cfg = Config::load()?;

            cfg.version = env!("CARGO_PKG_VERSION").to_string();
            cfg.course = course.to_string();
            cfg.user = username.to_string();
            cfg.host = host.to_string();

            Config::store(&cfg)?;
        }

        Some(Commands::Login) => {
            check_if_configured()?;
            let _ = auth::login();
        }

        Some(Commands::Submit { path }) => {
            let _ = submit_task(1, path.to_path_buf())?;
        }

        None => {}

        _ => {}
    };

    Ok(())

    // random token with ASCII characters
    // let token = rand::thread_rng()
    //     .sample_iter(&rand::distributions::Alphanumeric)
    //     .take(30)
    //     .collect::<String>();
}
