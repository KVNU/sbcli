use std::collections::HashMap;

use serde::Deserialize;

use crate::config;

#[derive(Deserialize)]
struct LoginResponse {
    token: String,
}

// TODO: check if session is still valid
pub fn ensure_auth() -> anyhow::Result<()> {
    let cfg = config::Config::load()?;

    if cfg.token.is_none() || cfg.last_login_time.is_none() || !cfg.is_token_valid() {
        login()?;
    }

    Ok(())
}

/// POST /api/auth/login
pub fn login() -> anyhow::Result<()> {
    let mut cfg = config::Config::load()?;

    println!("You need to login to continue.");
    let password = rpassword::prompt_password("Password: ")?;

    let client = reqwest::blocking::Client::new();

    let mut data = HashMap::new();
    data.insert("username", &cfg.user);
    data.insert("password", &password);
    data.insert("course", &cfg.course);

    let res = client
        .post(format!("{}/api/auth/login", cfg.host))
        .json(&data)
        .send()?;

    if res.status().is_success() {
        println!("Login successful");

        let body: LoginResponse = res.json()?;

        cfg.token = Some(body.token);
        cfg.last_login_time = Some(chrono::Utc::now());
        config::Config::store(&cfg)?;
    } else {
        println!("Login response indicates failure: {:?}", res);
        return Err(anyhow::anyhow!("Login failed"));
    }

    Ok(())
}
