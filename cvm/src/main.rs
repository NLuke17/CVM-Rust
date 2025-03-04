use rand::Rng;
use std::collections::HashSet;
use std::error::Error;
use std::fs;
use std::hash::Hash;
use std::time::Instant;

pub struct CVMHash<T> {
    storage: usize,      // maximum capacity before we adapt
    buffer: HashSet<T>,  // current set of stored elements
    coinflips: u32,      // number of times we've increased the coinflip threshold
}

impl<T: Eq + Hash + Clone> CVMHash<T> {
    /// Create a new estimator with a given storage capacity.
    pub fn new(storage: usize) -> Self {
        CVMHash {
            storage,
            buffer: HashSet::with_capacity(storage),
            coinflips: 0,
        }
    }

    /// Process a new input element.
    ///
    /// - If the element is already present and `coinflips != 0`, we perform a coin toss.
    ///   If the toss (simulated by generating a random number in [0, 2^coinflips))
    ///   returns a nonzero value, the element is removed.
    /// - If the element is not in the buffer, we insert it if either `coinflips == 0`
    ///   or a coin toss yields zero.
    /// - If the buffer size reaches or exceeds `storage`, we increment `coinflips`
    ///   and then remove some elements randomly (each with probability 1/2).
    pub fn new_input(&mut self, input: T) {
        let mut rng = rand::thread_rng();

        if self.buffer.contains(&input) {
            // If the element is already in the buffer, sometimes remove it.
            if self.coinflips != 0 && rng.gen_range(0..(1 << self.coinflips)) != 0 {
                self.buffer.remove(&input);
            }
        } else {
            // If it's a new element, sometimes add it.
            if self.coinflips == 0 || rng.gen_range(0..(1 << self.coinflips)) == 0 {
                self.buffer.insert(input);
            }
        }

        // When the buffer is too full, adapt by increasing coinflips and randomly removing elements.
        if self.buffer.len() >= self.storage {
            self.coinflips += 1;
            // Collect a list of items to remove (each with 50% chance).
            let to_remove: Vec<T> = self
                .buffer
                .iter()
                .filter(|_| rng.gen_range(0..2) != 0)
                .cloned()
                .collect();
            for item in to_remove {
                self.buffer.remove(&item);
            }
        }
    }

    /// Returns the current number of objects stored.
    pub fn storage_objects(&self) -> usize {
        self.buffer.len()
    }

    /// Returns the current estimate of distinct elements.
    ///
    /// The estimate is computed as: (number of stored elements) * 2^coinflips.
    pub fn get_estimate(&self) -> usize {
        self.buffer.len() * (1 << self.coinflips)
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    // Read the file "data.txt" (each word separated by whitespace).
    let text = fs::read_to_string("src\\output.txt")?;
    let words = text.split_whitespace();

    // Create a CVMHash estimator with a chosen storage capacity.
    let mut estimator = CVMHash::new(500);

    // Benchmark the processing of words.
    let start = Instant::now();
    for word in words {
        // Convert each &str into a String.
        estimator.new_input(word.to_string());
    }
    let duration = start.elapsed();

    println!("Estimated distinct count: {}", estimator.get_estimate());
    println!("Number of stored objects: {}", estimator.storage_objects());
    println!("Time elapsed: {:?}", duration);

    Ok(())
}
