use std::collections::HashMap;

use crate::config;

/// POST /api/auth/login
/// 200 OK Headers:
/// set-cookie: token=TOKEN; Path=/; HttpOnly; Secure; SameSite=Lax
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

    // check status
    if res.status().is_success() {
        println!("login successful");
        let headers = res.headers();

        // parse the token from the set-cookie header
        let token = headers
            .get("set-cookie")
            .unwrap()
            .to_str()
            .unwrap()
            .split(';')
            .next()
            .unwrap()
            .split('=')
            .last()
            .unwrap();
        // dbg!(token);

        // save session token
        cfg.token = Some(token.to_string());
        config::Config::store(&cfg)?;

        // println!(
        //     "stored token in config at {}\n",
        //     confy::get_configuration_file_path("sbcli", "config")?.display()
        // );
    } else {
        println!("login response indicates failure: {:?}", res);
        return Err(anyhow::anyhow!("login failed"));
    }

    Ok(())
}
