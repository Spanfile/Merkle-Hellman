use crate::knapsack::Knapsack;
use crate::math;
use rand::{thread_rng, Rng};

#[derive(Debug)]
pub struct PublicKey {
    knapsack: Knapsack,
}

#[derive(Debug)]
pub struct PrivateKey {
    knapsack: Knapsack,
    multiplier: i32,
    multiplier_inverse: i32,
    modulo: i32,
}

impl PrivateKey {
    fn generate(length: i32) -> PrivateKey {
        let knapsack = Knapsack::generate_superincreasing(length);
        let sum = knapsack.sum();

        let mut rng = thread_rng();
        let modulo = rng.gen_range(sum + 1, sum + 10);
        let multiplier = math::find_coprime(modulo);

        PrivateKey {
            knapsack,
            multiplier,
            multiplier_inverse: math::mod_inv(multiplier, modulo),
            modulo,
        }
    }

    pub fn decrypt(&self, payload: Vec<i32>) -> String {
        payload.iter().map(|i| self.decrypt_i32(*i)).collect()
    }

    fn decrypt_i32(&self, payload: i32) -> char {
        let mut val = payload * self.multiplier_inverse % self.modulo;
        let mut decrypted = 0;

        for i in (0..self.knapsack.values.len()).rev() {
            if self.knapsack.values[i] <= val {
                decrypted += i32::pow(2, i as u32);
                val -= self.knapsack.values[i];
            }
        }

        decrypted as u8 as char
    }
}

impl PublicKey {
    fn derive_from_private(private: &PrivateKey) -> PublicKey {
        PublicKey {
            knapsack: Knapsack {
                values: private
                    .knapsack
                    .values
                    .iter()
                    .map(|v| v * private.multiplier % private.modulo)
                    .collect(),
            },
        }
    }

    pub fn encrypt(&self, message: &str) -> Vec<i32> {
        message.chars().map(|c| self.encrypt_u8(c as u8)).collect()
    }

    fn encrypt_u8(&self, mut message: u8) -> i32 {
        let mut index = 0;
        let mut payload = 0;

        while message != 0 {
            if message & 1 == 1 {
                payload += self.knapsack.values[index];
            }
            message >>= 1;
            index += 1;
        }

        payload
    }
}

pub fn generate_keypair() -> (PublicKey, PrivateKey) {
    let private = PrivateKey::generate(8);
    let public = PublicKey::derive_from_private(&private);
    (public, private)
}
