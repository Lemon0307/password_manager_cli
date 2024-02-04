use std::io;
use mysql::*;

struct Password {
    username: String,
    password: String,
    isEncrypted: bool,
}

pub fn loadPassword() {

}

pub fn changePassword() {

}

pub fn createPassword() {

}

fn main() -> io::Result<()> {
    println!("Welcome to your password manager, please select the option you want: \n");
    println!("1: Load all passwords");
    println!("2: Add new password");
    println!("3: Change password");
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer)?;
    let option = buffer.trim().parse::<u8>().unwrap();
    match option {
        1 => {
            loadPassword();
        },
        2 => {
            createPassword();
        },
        3 => {
            changePassword();
        },
        0_u8 | 4_u8..=u8::MAX => todo!(),
    };
    Ok(())
}
