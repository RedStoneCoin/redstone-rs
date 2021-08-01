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

fn gen_keypair() {
    let wallet = redstone_rs::keypair::Keypair::generate();
    println!("Your wallet address:{}", wallet.address());
    println!("Private key:{}", wallet.private_key);
    //save to the file
    println!("Enter wallet password: ");

    let mut pass = String::new();
    io::stdin().read_line(&mut pass)
        .expect("Failed to read input.");
    
        println!("Enter wallet filename: ");

    let mut filename = String::new();
    io::stdin().read_line(&mut filename)
        .expect("Failed to read input.");
    println!("{}", filename);

    fs::write(&filename.trim_end(), wallet.private_key.to_string());
           
    main();

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
    if command == 1 {
        gen_keypair();
        main();

    }
    else if command == 2 {
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
        println!("{}", filename);
        fs::write(&filename.trim_end(), private_key.trim_end().to_string());
        main();
    }
    else if command == 3 {
        let mut filename = String::new();
        io::stdin().read_line(&mut filename)
            .expect("Failed to read input.");
        println!("{}", filename);
        let private_key = fs::read_to_string(filename.trim_end())
            .expect("Something went wrong reading the file");
        println!("Enter wallet password: ");

        let mut pass = String::new();
        io::stdin().read_line(&mut pass)
            .expect("Failed to read input.");

        let wallet = redstone_rs::keypair::Keypair::from_private_key(private_key.to_string());
        print!("Wallet Imported!\n");
        main_login(wallet.private_key.to_string(),wallet.address());

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
