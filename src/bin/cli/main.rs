use clio_db::{error::ClioResult, models::LoginRequest};

use reqwest::{header::AUTHORIZATION, Client};
use std::io::{self};

#[tokio::main]
async fn main() {
    if let Err(e) = run_cli().await {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}

async fn run_cli() -> ClioResult<()> {
    println!("cli");
    let user = read_login_credentials()?;
    println!("{:?}", user);

    println!("attempting to login");
    let token = post_login_request(user).await?;
    println!("access token: {}", &token);

    let ping_response = ping(token).await?;
    println!("ping response: {}", ping_response);

    Ok(())
}

fn read_login_credentials() -> ClioResult<LoginRequest> {
    let mut username = String::new();
    let mut password = String::new();
    println!("username: ");
    io::stdin().read_line(&mut username)?;
    println!();

    println!("password: ");
    io::stdin().read_line(&mut password)?;
    println!();

    username.pop();
    password.pop();

    Ok(LoginRequest { username, password })
}

async fn post_login_request(request: LoginRequest) -> ClioResult<String> {
    let uri = "http://localhost:3000/login".to_string();

    let response = Client::new().post(uri).json(&request).send().await?;

    let token = response
        .headers()
        .get(AUTHORIZATION)
        .unwrap()
        .to_str()
        .unwrap()
        .to_string();

    Ok(token)
}

async fn ping(bearer_token: String) -> ClioResult<String> {
    let uri = "http://localhost:3000/ping".to_string();

    let response = Client::new()
        .get(uri)
        .header(AUTHORIZATION, format!("Bearer {}", bearer_token))
        .send()
        .await?
        .text()
        .await?;

    Ok(response)
}
