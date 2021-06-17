use redstone_rs::*;
fn main() {
    println!("Starting redstone wallet");
    let wallet = redstone_rs::keypair::Keypair::generate();
    println!("{:#?}, {}", wallet, wallet.address())
}
