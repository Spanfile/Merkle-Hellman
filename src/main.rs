mod knapsack;
mod math;
mod merkle_hellman;

fn main() {
    let message: u8 = 42;
    let (public, private) = merkle_hellman::generate_keypair(8);

    let payload = public.encrypt(message);
    println!("{}", payload);

    let decrypted = private.decrypt(payload);
    println!("{}", decrypted);
}
