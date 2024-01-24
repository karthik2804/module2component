use anyhow::anyhow;
use std::{
    borrow::Cow,
    env,
    fs::File,
    io::{Read, Write},
};
fn main() -> anyhow::Result<()> {
    let args: Vec<String> = env::args().collect();

    // Check if a file name is provided
    if args.len() != 2 {
        eprintln!("Usage: {} <file_path>", args[0]);
        std::process::exit(1);
    }

    // Get the file name from command-line arguments
    let file_name = &args[1];

    // Open the file
    let file = match File::open(file_name) {
        Ok(file) => file,
        Err(e) => {
            eprintln!("Error opening file: {}", e);
            std::process::exit(1);
        }
    };
    let mut buffer = Vec::new();
    match file.take(u64::MAX as u64).read_to_end(&mut buffer) {
        Ok(_) => {
            let bytes = spin_componentize::componentize_if_necessary(&buffer)
                .map_err(|e| anyhow!(e.to_string()))?;
            // Print the bytes
            let file_path = "output.bin";

            // Open the file for writing
            let mut file = match File::create(file_path) {
                Ok(file) => file,
                Err(e) => {
                    eprintln!("Error creating file: {}", e);
                    std::process::exit(1);
                }
            };

            if let Cow::Borrowed(slice) = bytes {
                match file.write_all(slice) {
                    Ok(_) => println!("Data written to file successfully."),
                    Err(e) => eprintln!("Error writing to file: {}", e),
                }
            } else if let Cow::Owned(vec) = bytes {
                match file.write_all(&vec) {
                    Ok(_) => println!("Data written to file successfully."),
                    Err(e) => eprintln!("Error writing to file: {}", e),
                }
            }
        }
        Err(e) => {
            eprintln!("Error reading file: {}", e);
            std::process::exit(1);
        }
    }
    Ok(())
}
