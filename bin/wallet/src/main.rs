use redstone_rs::*;
use fern::colors::{Color, ColoredLevelConfig};
use log::*;
use redstone_rs::*;
use std::collections::HashMap;
use std::io;
use std::fs::File;
use std::io::prelude::*;
use std::io::Read;
use std::fs;
use std::io::{Write};
use secrecy::Secret;
use encryptfile as ef;

fn gen_keypair() {
    let wallet = redstone_rs::keypair::Keypair::generate();
    println!("Your wallet address:{}", wallet.address());
    println!("Private key:{}", wallet.private_key);
    //save to the file
    println!("Enter wallet password: ");

    let mut pass = String::new();
    io::stdin().read_line(&mut pass)
        .expect("Failed to read input.");
   
    println!("Enter wallet location: ");
    let mut filename = String::new();
    io::stdin().read_line(&mut filename)
        .expect("Failed to read input.");
    println!("{}", filename);

    let encrypted = {
        let encryptor = age::Encryptor::with_user_passphrase(Secret::new(pass.to_owned()));
    
        let mut encrypted = vec![];
        let mut writer = encryptor.wrap_output(&mut encrypted).unwrap();

        writer.write_all(wallet.private_key.as_bytes()).unwrap();

        writer.finish().unwrap();

    
        encrypted
    };
    
    fs::write(&filename.trim_end(), encrypted);
    println!("Crated file");
    println!("{}", filename.trim_end());
    //encrypt the file
    print!("encrypted");


}


fn commands(){
    println!("Command: 1 Generate a new wallet");
    println!("Usage: redstone_rs keygen");
    println!("Command: 2 Import private key");
    println!("Usage: redstone_rs import <private key>");
    println!("Command: 3 Import wallet file");
    println!("Usage: redstone_rs import <wallet file>");

}
fn commands_logged(){
    println!("Command: 3 Show wallet balance");
    println!("Usage: redstone_rs balance");
    println!("Command: 4 Send Redstone");
    println!("Usage: redstone_rs send <address> <amount>");
    println!("Command: 5 Show transaction history");
    println!("Usage: redstone_rs history");
    println!("Command: 6 Show transaction details");
    println!("Usage: redstone_rs details <txid>");
}
fn main_login(pik: String,pbk: String){
    println!("Your wallet address:{}", pbk);
    println!("Private key:{}", pik);
}
fn wallet_control(command: i32) {
    match command {
    1 => {
            gen_keypair();
            main();
    },
    2 => {
        println!("Enter private key: ");
        let mut private_key = String::new();
        io::stdin().read_line(&mut private_key)
            .expect("Failed to read input.");
        let wallet = redstone_rs::keypair::Keypair::from_private_key(private_key.trim_end().to_string());
        println!("{:?}", wallet);
        //save to the file
        println!("Enter wallet filename: ");

        let mut filename = String::new();
        io::stdin().read_line(&mut filename)
            .expect("Failed to read input.");
        let mut pass = String::new();
        io::stdin().read_line(&mut pass)
            .expect("Failed to read input.");
    
        println!("{}", filename);
        let plaintext = private_key;

        let encrypted = {
            let encryptor = age::Encryptor::with_user_passphrase(Secret::new(pass.to_owned()));
        
            let mut encrypted = vec![];
            let mut writer = encryptor.wrap_output(&mut encrypted).unwrap();

            writer.write_all(plaintext.as_bytes()).unwrap();

            writer.finish().unwrap();

        
            encrypted
        };
        
        fs::write(&filename.trim_end(), encrypted);
        //encrypt the file
        print!("encrypted");
        main();
    },
    3 => {
        let mut filename = String::new();
        io::stdin().read_line(&mut filename)
            .expect("Failed to read input.");
        println!("{}", filename);
        println!("Enter wallet password: ");
        let mut pass = String::new();
        io::stdin().read_line(&mut pass)
            .expect("Failed to read input.");
        //decryptit
        //then read it
        let private_key = std::fs::read(filename.trim_end())
    
            .expect("Something went wrong reading the file");

        let decrypted = {
                let decryptor = match age::Decryptor::new(&private_key[..]).unwrap() {
                    age::Decryptor::Passphrase(d) => d,
                    _ => unreachable!(),
                };
            
                let mut decrypted = vec![];
                let mut reader = decryptor.decrypt(&Secret::new(pass.to_owned()), None).unwrap();
                reader.read_to_end(&mut decrypted);
            
                decrypted
        };
        println!("{:?}", decrypted);
        let decrypted1 = String::from_utf8(decrypted);
        let wallet = redstone_rs::keypair::Keypair::from_private_key(decrypted1.unwrap());
        println!("{:?}", wallet);

        print!("Wallet Imported!\n");
        main_login(wallet.private_key.to_string(),wallet.address());

    } 
    _ => {
        println!("Unknown command");
    }
    }

 }
 
fn command_control(command: i32) {
   match command {
       1 => {
           println!("Generate a new wallet");
           gen_keypair();
       }
       2 => {
           println!("Import wallet");
           wallet_control(2);
       }
       3 => {
           println!("Import wallet file");
           wallet_control(3);

       }
       _ => {
           println!("Unknown command");
       }
   }
}

pub fn get_input_int() {
    let mut input = String::new();
    // Reads the input from STDIN and places it in the String named input.
    println!("Enter a value:");
    io::stdin().read_line(&mut input)
        .expect("Failed to read input.");
    // Convert to an i32.
    let input: i32 = input.trim().parse().unwrap();
    command_control(input);
    
}
pub fn get_input_wallet() {
    let mut input = String::new();
    // Reads the input from STDIN and places it in the String named input.
    println!("Enter a value:");
    io::stdin().read_line(&mut input)
        .expect("Failed to read input.");
    // Convert to an i32.
    let input: i32 = input.trim().parse().unwrap();
    wallet_control(input);
    
}

fn main() {
    let art = " 
    ██████╗ ███████╗██████╗ ███████╗████████╗ ██████╗ ███╗   ██╗███████╗
    ██╔══██╗██╔════╝██╔══██╗██╔════╝╚══██╔══╝██╔═══██╗████╗  ██║██╔════╝
    ██████╔╝█████╗  ██║  ██║███████╗   ██║   ██║   ██║██╔██╗ ██║█████╗  
    ██╔══██╗██╔══╝  ██║  ██║╚════██║   ██║   ██║   ██║██║╚██╗██║██╔══╝  
    ██║  ██║███████╗██████╔╝███████║   ██║   ╚██████╔╝██║ ╚████║███████╗
    ╚═╝  ╚═╝╚══════╝╚═════╝ ╚══════╝   ╚═╝    ╚═════╝ ╚═╝  ╚═══╝╚══════╝
    ";
    println!("{}",art);
    
    println!("Welcome Redstone Wallet!");

    commands();
    get_input_int();
}
