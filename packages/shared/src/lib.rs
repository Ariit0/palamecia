use rand::{rngs::StdRng, Rng, SeedableRng};

pub enum RandomSeed {
    Empty,
    Is(u64),
}
/**
 * Adds one to the given number
 */
pub fn add_one(x: i32) -> i32 {
    x + 1
}

/**
 * Adds random number to the given number and seed
 */
pub fn add_random(x: i32, seed: RandomSeed) -> i32 {
    match seed {
        RandomSeed::Empty => x + rand::random::<i32>(),
        RandomSeed::Is(seed) => {
            let mut rng = StdRng::seed_from_u64(seed);
            x + rng.gen::<i32>()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_one() {
        assert_eq!(3, add_one(2));
    }

    #[test]
    fn test_add_random() {
        // Check if the result is within a specific range
        let x3: i32 = 100;
        let seed3 = 789; // Seed value for the random number generator
        let result3: i32 = add_random(x3, RandomSeed::Is(seed3));
        let max_possible_result: i32 = match x3.checked_add(i32::MAX) {
            Some(val) => val,
            None => i32::MAX,
        };
        assert!(
            result3 >= x3 && result3 <= max_possible_result,
            "Result is not within the expected range"
        );
    }
}
