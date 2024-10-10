use dotenv::dotenv;
use std::env;
use solana_sdk::{
    signature::{Keypair, Signer},
    pubkey::Pubkey,
};

fn main() {
    dotenv().ok();
    println!("Variabile incarcate!");
    let keypair = Keypair::new();

    
    println!("Public key: {}", keypair.pubkey());
    println!("Secret key: {:?}", keypair.to_bytes());

   let keypar_from_env = env::var("MY_SECRET_KEY").expect("MY_SECRET_KEY nu este setat");

    println!("My Secret key: {}", keypar_from_env);
   
}

