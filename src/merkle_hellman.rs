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

    pub fn decrypt(&self, payload: i32) -> i32 {
        let mut val = payload * self.multiplier_inverse % self.modulo;
        let mut decrypted = 0;
        for i in (0..self.knapsack.values.len()).rev() {
            if self.knapsack.values[i] <= val {
                decrypted += i32::pow(2, i as u32);
                val -= self.knapsack.values[i];
            }
        }
        decrypted
    }
}

impl PublicKey {
    fn derive_from_private(private: &PrivateKey) -> PublicKey {
        let mut values = Vec::new();

        for val in &private.knapsack.values {
            values.push(val * private.multiplier % private.modulo);
        }

        PublicKey {
            knapsack: Knapsack { values },
        }
    }

    pub fn encrypt(&self, mut message: u8) -> i32 {
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

pub fn generate_keypair(length: i32) -> (PublicKey, PrivateKey) {
    let private = PrivateKey::generate(length);
    let public = PublicKey::derive_from_private(&private);
    (public, private)
}
