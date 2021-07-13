use redstone_rs::*;
fn genaddr() {
    let wallet = redstone_rs::keypair::Keypair::generate();
    println!("Your wallet address:{}", wallet.address());
    println!("Private key:{}", &wallet.private_key.to_string());
}
fn main() {

    println!("Starting redstone wallet");
    genaddr();
   


}
