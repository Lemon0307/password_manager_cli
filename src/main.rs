use std::io;
use std::fs::File;
use std::io::Read;
use std::io::Write;
use std::fs::OpenOptions;
use mysql::*;

//TODO: implement enecryption and hashing

pub fn join_2D_vector(v: &Vec<Vec<&str>>) -> String {
    let mut sub_vector = Vec::new();
    for subvec in v.iter() {
        sub_vector.push(subvec.join(":"));
    }
    sub_vector.join("\n").to_string()
}

pub fn load_password(website: String) -> Result<()> {
    let mut file = File::open("passwords.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let lines = contents.split("\n");
    for line in lines {
        let x = line.split(":").collect::<Vec<&str>>();
        if x[0].to_lowercase() == website.trim().to_lowercase() {
            println!("{}", x[1]);
        }
    }
    Ok(())
}

pub fn change_password(website: String, password: String) -> Result<()> {
    let mut file = OpenOptions::new().write(true).read(true).open("passwords.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let lines = contents.split("\n");
    let mut temp = Vec::new();
    for line in lines {
        let mut x = line.split(":").collect::<Vec<&str>>();
        if x[0].to_lowercase() == website.trim().to_lowercase() {
            x[1] = password.trim();
        }
        temp.push(x);
    }
    let mut file = File::create("passwords.txt");
    file?.write(&join_2D_vector(&temp).into_bytes());
    Ok(())
}

pub fn create_password(website: String, password: String) -> Result<()> {
    let mut file = OpenOptions::new().append(true).open("passwords.txt").unwrap();
    write!(file, "{}:{}\n", website.trim(), password.trim());
    Ok(())
}

fn main() -> io::Result<()> {
    println!("Welcome to your password manager, please select the option you want: \n");
    println!("1: Load all passwords");
    println!("2: Add new password");
    println!("3: Change password");
    let mut buffer = String::new();
    let _ = io::stdin().read_line(&mut buffer)?;
    let option = buffer.trim().parse::<u8>().unwrap();
    match option {
        1 => {
            println!("Please provide the website you want your password from: ");
            let mut website = String::new();
            let _ = io::stdin().read_line(&mut website)?;
            let _ = load_password(website);
        },
        2 => {
            let mut website = String::new();
            let mut password = String::new();
            println!("Website name: ");
            let _ = io::stdin().read_line(&mut website);
            println!("Password: ");
            let _ = io::stdin().read_line(&mut password);
            let _ = create_password(website, password);
        },
        3 => {
            let mut website = String::new();
            let mut password = String::new();
            println!("Website name: ");
            let _ = io::stdin().read_line(&mut website);
            println!("Password: ");
            let _ = io::stdin().read_line(&mut password);
            let _ = change_password(website, password);
        },
        0_u8 | 4_u8..=u8::MAX => todo!(),
    };
    Ok(())
}
