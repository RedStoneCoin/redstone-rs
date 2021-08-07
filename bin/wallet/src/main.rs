#![feature(proc_macro_hygiene, decl_macro)]

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

mod api;

fn save_wallet(wallet: String,pass: String,filename: String) {
       let encrypted = {
           let encryptor = age::Encryptor::with_user_passphrase(Secret::new(pass.to_owned()));
           let mut encrypted = vec![];
           let mut writer = encryptor.wrap_output(&mut encrypted).unwrap();
           writer.write_all(wallet.as_bytes()).unwrap();
           writer.finish().unwrap();
           encrypted
       };
       fs::write(&filename, encrypted);
       println!("WALLET SAVED AT: {}", filename);
}
fn open_wallet(pass: String, filename: String) {
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
    let decrypted1 = String::from_utf8(decrypted);
    let wallet = redstone_rs::keypair::Keypair::from_private_key(decrypted1.unwrap());
    print!("Wallet imported successfully!\n");
    main_login(wallet.private_key.to_string(),wallet.address());
}
fn gen_keypair() {
    let wallet = redstone_rs::keypair::Keypair::generate();
    println!("Your wallet address:{}", wallet.address());
    println!("Private key:{}", wallet.private_key);
    println!("Enter Filename: ");
    let mut filename = String::new();
    io::stdin().read_line(&mut filename)
        .expect("Failed to read input.");
    println!("Enter Password: ");
    let mut pass = String::new();
    io::stdin().read_line(&mut pass)
        .expect("Failed to read input.");
    save_wallet(wallet.private_key,pass,filename.trim_end().to_string());
    main();
}

fn commands(){
    println!("Command: 1 Generate a new wallet");
    println!("Command: 2 Import private key");
    println!("Command: 3 Import wallet file");
    println!("Command: 4 exit");
}

fn commands_logged(){
    println!("Command: 3 Show wallet balance");
    println!("Command: 4 Send Redstone");
    println!("Command: 5 Show transaction history");
    println!("Command: 6 Show transaction details");
    println!("Command: 7 exit");
}

fn main_login(pik: String,pbk: String){
    println!("Your wallet address:{}", pbk);
    println!("Private key:{}", pik);
    commands_logged();
    let mut input = String::new();
    // Reads the input from STDIN and places it in the String named input.
    println!("Enter a value:");
    io::stdin().read_line(&mut input)
        .expect("Failed to read input.");
    // Convert to an i32.
    let input: i32 = input.trim().parse().unwrap();
    match input {
        1 => {
            println!("Commint soon!");
        },
        7 => {
            println!("Bye!");
        }
        _ => {
            main_login(pik,pbk);
            println!("Unknown command");
            //dont exit loop back in here
        }
    }
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
        println!("Enter Password: ");

        let mut pass = String::new();
        io::stdin().read_line(&mut pass)
            .expect("Failed to read input.");
    
        save_wallet(wallet.private_key.to_string(),pass,filename.trim_end().to_string());
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
        open_wallet(pass,filename);

    } 
    _ => {
        main();
        println!("Unknown command");

    }
  }
}
 
fn command_control(command: i32) {
   match command {
       1 => {
            wallet_control(1);
         }
       2 => {
           println!("Import wallet");
           wallet_control(2);
       }
       3 => {
           println!("Import wallet file");
           wallet_control(3);

       }
       4 => {
        println!("Exited");
        //save enverything

       }
       _ => {
           main();
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
    println!("ALPHA 0.1!");
    println!("Until testnet wallet can only create wallets!");

    commands();
    get_input_int();
}
