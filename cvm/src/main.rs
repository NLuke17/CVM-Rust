use rand::Rng;
use std::collections::HashSet;
use std::error::Error;
use std::fs;
use std::fs::File;
use std::hash::Hash;
use std::io::{BufWriter, Write};
use std::time::Instant;

pub struct CVMHash<T> {
    storage: usize,     // maximum capacity before we adapt
    buffer: HashSet<T>, // current set of stored elements
    coinflips: u32,     // number of times we've increased the coinflip threshold
}

impl<T: Eq + Hash + Clone> CVMHash<T> {
    pub fn new(storage: usize) -> Self {
        CVMHash {
            storage,
            buffer: HashSet::with_capacity(storage),
            coinflips: 0,
        }
    }

    pub fn new_input(&mut self, input: T) {
        let mut rng = rand::thread_rng();

        if self.buffer.contains(&input) {
            if self.coinflips != 0 && rng.gen_range(0..(1 << self.coinflips)) != 0 {
                self.buffer.remove(&input);
            }
        } else {
            if self.coinflips == 0 || rng.gen_range(0..(1 << self.coinflips)) == 0 {
                self.buffer.insert(input);
            }
        }

        if self.buffer.len() >= self.storage {
            self.coinflips += 1;
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
    // Read and parse words into a Vec<String> once
    let text = fs::read_to_string("src/ipAddress.txt")?;
    let words: Vec<String> = text.split_whitespace().map(|s| s.to_string()).collect();

    let mut buffer = 100; // starting buffer size
    let increment = 100; // how much buffer is incrememented
    let buffer_amount = 20; //amount of buffers tested
    let trials = 100; //Trials per buffer

    let actual_distinct = 1522917;
    let output_path = "src/output.txt";
    let output_file = File::create(output_path)?;
    let mut writer = BufWriter::new(output_file);

    writeln!(writer, "Actual Distinct : {}", actual_distinct)?;
    writeln!(writer, "abs error, rel error, time")?;

    for _i in 1..=buffer_amount {
        writeln!(writer, "Buffer size:             {}\n", buffer)?;
        let mut confidence_counter = 0;
        for _j in 1..=trials {
            let mut estimator: CVMHash<String> = CVMHash::new(buffer);
            let start: Instant = Instant::now();
            for word in &words {
                estimator.new_input(word.clone());
            }
            let duration = start.elapsed();
            let estimate = estimator.get_estimate();
            let absolute_error = (estimate as i64 - actual_distinct as i64).abs();
            let relative_error = (absolute_error as f64 / actual_distinct as f64) * 100.0;
            if relative_error < 5.0 {
                confidence_counter += 1;
            }
            //write!(writer, " Estimated distinct count: {}", estimate)?;
            write!(writer, "{} ", absolute_error)?;
            write!(writer, "{:.2}% ", relative_error)?;
            writeln!(writer, "{:?}", duration)?;
        }
        let percent_confidence = (confidence_counter as f64) / (trials as f64);
        let percent = percent_confidence * 100.0;
        writeln!(
            writer,
            "Percent of trials with error of less than 5% :: {:.2}% \n\n",
            percent
        )?;
        buffer += increment;
    }

    Ok(())
}
