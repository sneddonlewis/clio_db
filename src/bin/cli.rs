use clio_db::{error::ClioResult, models::LoginRequest};
use std::io::{self};

fn main() {
    println!("cli");
    let user = read_login_credentials().unwrap();
    println!("{:?}", user);
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
