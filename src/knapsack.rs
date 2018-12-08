use rand::{thread_rng, Rng};

#[derive(Debug)]
pub struct Knapsack {
    pub values: Vec<i32>,
}

impl Knapsack {
    pub fn generate_superincreasing(length: i32) -> Knapsack {
        let mut rng = thread_rng();
        let mut values = Vec::new();
        let mut max = 1;

        for _ in 0..length {
            let val = rng.gen_range(max, max + 10);
            values.push(val);
            max += val;
        }

        Knapsack { values }
    }

    pub fn sum(&self) -> i32 {
        self.values.iter().sum()
    }
}
