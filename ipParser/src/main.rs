use std::error::Error;
use std::fs::File;
use std::io::{BufWriter, Write};

fn main() -> Result<(), Box<dyn Error>> {
    // Hardcoded file paths
    let input_path = "src/Dataset_IP.csv";
    let output_path = "src/output.txt";

    // Create CSV reader
    let input_file = File::open(input_path)?;
    let mut rdr = csv::Reader::from_reader(input_file);

    // Create output file writer
    let output_file = File::create(output_path)?;
    let mut writer = BufWriter::new(output_file);

    // Process records and write to file
    for result in rdr.records() {
        let record = result?;
        if let Some(ip) = record.get(0) {
            writeln!(writer, "{}", ip)?;
        }
    }

    // Ensure all data is flushed to disk
    writer.flush()?;

    println!("Successfully extracted IPs to {}", output_path);
    Ok(())
}
