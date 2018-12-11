extern crate rand;

mod knapsack;
mod math;
mod merkle_hellman;

use std::env;

fn main() {
    let args = env::args().collect::<Vec<String>>();
    let message = &args[1];
    let (public, private) = merkle_hellman::generate_keypair();

    let payload = public.encrypt(message);
    println!("{:?}", payload);

    let decrypted = private.decrypt(payload);
    println!("{}", decrypted);
}
