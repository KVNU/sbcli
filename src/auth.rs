use std::collections::HashMap;

use serde::Deserialize;

use crate::config;

#[derive(Deserialize)]
struct LoginResponse {
    token: String,
}

/// POST /api/auth/login
pub fn login() -> anyhow::Result<()> {
    let mut cfg = config::Config::load()?;

    let password = rpassword::prompt_password("Password: ")?;
    // dbg!(&cfg.user, &password);

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
        println!("login successful");

        let body: LoginResponse = res.json()?;

        cfg.token = Some(body.token);
        config::Config::store(&cfg)?;
    } else {
        println!("login response indicates failure: {:?}", res);
        return Err(anyhow::anyhow!("login failed"));
    }

    Ok(())
}
