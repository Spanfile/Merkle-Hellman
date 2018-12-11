use rand::{thread_rng, Rng};

#[derive(Debug)]
pub struct Knapsack {
    pub values: Vec<i32>,
}

impl Knapsack {
    pub fn generate_superincreasing(length: i32) -> Knapsack {
        let mut rng = thread_rng();
        let mut max = 1;

        Knapsack {
            values: (0..length)
                .map(|_| {
                    let val = rng.gen_range(max, max + 10);
                    max += val;
                    val
                })
                .collect(),
        }
    }

    pub fn sum(&self) -> i32 {
        self.values.iter().sum()
    }
}
