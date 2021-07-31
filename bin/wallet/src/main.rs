use redstone_rs::*;
use fern::colors::{Color, ColoredLevelConfig};
use log::*;
use redstone_rs::*;
use std::collections::HashMap;
use std::io;

fn gen_keypair() {
    let wallet = redstone_rs::keypair::Keypair::generate();
    println!("Your wallet address:{}", wallet.address());
    println!("Private key:{}", wallet.private_key);
    println!("Key pair:{:#?}", wallet);

}
fn commands(){
    println!("Command: 1 Generate a new wallet");
    println!("Usage: redstone_rs keygen");
    println!("Command: 2 Import wallet");
    println!("Usage: redstone_rs import <private key>");
    /*
    println!("Command: 3 Show wallet balance");
    println!("Usage: redstone_rs balance");
    println!("Command: 4 Send Redstone");
    println!("Usage: redstone_rs send <address> <amount>");
    println!("Command: 5 Show transaction history");
    println!("Usage: redstone_rs history");
    println!("Command: 6 Show transaction details");
    println!("Usage: redstone_rs details <txid>");
   */
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

fn wallet_control(command: i32) {
    if command == 1 {
        gen_keypair();
    }
    else if command == 2 {
        println!("Enter private key: ");
        let mut private_key = String::new();
        io::stdin().read_line(&mut private_key)
            .expect("Failed to read input.");
        let wallet = redstone_rs::keypair::Keypair::from_private_key(private_key.trim_end().to_string());
        println!("{}", wallet);
         //save to the file

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
           println!("Show wallet balance");
       }
       4 => {
           println!("Send money");
           println!("Usage: redstone_rs send <address> <amount>");
       }
       5 => {
           println!("Show transaction history");
       }
       6 => {
           println!("Show transaction details");
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
