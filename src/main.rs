// Environment variable to enable backtrace
// RUST_BACKTRACE=1; 

// Imports
use serde::{Serialize, Deserialize}; 
use sha2::{Sha256, Digest}; 
use std::time::{SystemTime, UNIX_EPOCH}; 
use reqwest; 
use serde_json::Value;
use rand::Rng; // Import RNG
use std::fs::OpenOptions;
use std::io::Write;
use std::time::{Duration, Instant};
use tokio::time::sleep;
use anyhow::{Result, Error}; // Use Error from anyhow


// Global variable to track the last request time
static mut LAST_QRNG_REQUEST: Option<Instant> = None;

// Block struct
#[derive(Serialize, Deserialize, Debug)]
pub struct Block {
    pub index: u32,
    pub timestamp: u64,
    pub data: String,
    pub previous_hash: String,
    pub nonce: u64,
    pub hash: String,
}

// Blockchain struct
pub struct Blockchain {
    pub chain: Vec<Block>,
    pub difficulty: usize,
    pub qrng_numbers: Vec<u64>, // QRNG number pool
}

// Block implementation
impl Block {
    // Create New Block
    pub fn new(index: u32, data: String, previous_hash: String) -> Self { 
        let timestamp = now(); 
        let mut block = Block {
            index, 
            timestamp,
            data, 
            previous_hash, 
            nonce: 0,
            hash: String::new()
        }; 

        block.hash = block.calculate_hash(); 
        block
    }

    // Calculate Hash
    pub fn calculate_hash(&self) -> String {
        let input = format!(
            "{}{}{}{}{}", 
            self.index, self.timestamp, self.data, self.previous_hash, self.nonce
        ); 
        let mut hasher = Sha256::new(); 
        hasher.update(input); 
        format!("{:x}", hasher.finalize())
    }
}

// Generate classical RNG numbers
pub fn generate_classical_random_numbers(batch_size: usize) -> Vec<u64> {
    let mut rng = rand::thread_rng();
    (0..batch_size).map(|_| rng.gen_range(0..65536)).collect() // Generate u16 range numbers
}

// Function to Log Nonce Data
pub fn log_nonce_data(file_name: &str, nonces: &[u64], source: &str) -> Result<()> {
    let mut file = OpenOptions::new().create(true).append(true).open(file_name)?;
    for nonce in nonces {
        writeln!(file, "{},{}", source, nonce)?;
    }
    Ok(())
}

// Function to log individual nonce data to a specific file
pub fn log_nonce(file_name: &str, nonce: u64) -> Result<()> {
    let mut file = OpenOptions::new().create(true).append(true).open(file_name)?;
    writeln!(file, "{}", nonce)?;
    Ok(())
}

// Blockchain implementation
impl Blockchain {
    // Create a new blockchain with the genesis block
    pub fn new(qrng_numbers: Vec<u64>) -> Self {
        Blockchain {
            chain: vec![Blockchain::create_genesis_block()],
            difficulty: 2,
            qrng_numbers,
        }
    }

    // Create Genesis Block
    fn create_genesis_block() -> Block {
        Block::new(0, "Dr. Kogge buys Ancient Aliens Live T-shirt for 20 USD.".to_string(), "0".to_string())
    }

    // Add a new block to the blockchain
    pub async fn add_block(
        &mut self,
        data: String,
        use_classical_rng: bool,
    ) -> Result<()> {
        let previous_hash = self.chain.last().unwrap().hash.clone();
        let mut new_block = Block::new(self.chain.len() as u32, data, previous_hash);

        // Get metrics from proof_of_work
        let (iterations, duration) = self.proof_of_work(&mut new_block, use_classical_rng).await?;

        // Log the metrics
        let source = if use_classical_rng { "Classical" } else { "Quantum" };
        let log_entry = format!(
            "{},{},{:?},{:?}\n",
            new_block.index, source, iterations, duration
        );
        let mut file = OpenOptions::new().create(true).append(true).open("metrics.csv")?;
        file.write_all(log_entry.as_bytes())?;

        // Add the block to the chain if mining succeeds
        self.chain.push(new_block);
        Ok(())
    }
    
    // Perform Proof of Work with Quantum Random Number Generator
    // Blockchain implementation: Modify proof_of_work
    pub async fn proof_of_work(
        &mut self,
        block: &mut Block,
        use_classical_rng: bool,
    ) -> Result<(u32, Duration)> {
        let mut random_index = 0;
        let mut iterations = 0; // Count the number of attempts
        let start_time = Instant::now(); // Track start time for mining
    
        while !block.hash.starts_with(&"0".repeat(self.difficulty)) {
            iterations += 1;
    
            if use_classical_rng {
                let classical_numbers = generate_classical_random_numbers(1);
                block.nonce = classical_numbers[0];

                // Log nonce to classical_nonces.csv
                log_nonce("classical_nonces.csv", block.nonce)?;
            } else {
                if random_index >= self.qrng_numbers.len() {
                    println!("Fetching more QRNG numbers...");
                    match fetch_quantum_random_numbers(500).await {
                        Ok(numbers) => self.qrng_numbers.extend(numbers),
                        Err(e) => {
                            println!("Error fetching QRNG numbers: {}", e);
                            return Err(Error::msg("Failed to fetch additional QRNG numbers; mining stopped."));
                        }
                    }
                    random_index = 0;
                }
                block.nonce = self.qrng_numbers[random_index];

                // Log nonce to quantum_nonces.csv
                log_nonce("quantum_nonces.csv", block.nonce)?;
                random_index += 1;
            }
    
            block.hash = block.calculate_hash();
        }
    
        let duration = start_time.elapsed(); // Calculate mining duration
        println!(
            "Block {} mined with nonce: {} in {} iterations and took {:?}",
            block.index, block.nonce, iterations, duration
        );
        Ok((iterations, duration))
    }
    
    // Validate the blockchain
    pub fn is_valid(&self) -> bool {
        for i in 1..self.chain.len() {
            let current = &self.chain[i]; 
            let previous = &self.chain[i - 1]; 

            // Check if the current block hash is correct
            if current.hash != current.calculate_hash() {
                return false; 
            }

            // Check if the current block's previous hash matches the previous block's hash
            if current.previous_hash != previous.hash {
                return false; 
            }
        }
        true
    }
}

// Get current timestamp as a Unix Timestamp
pub fn now() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs()
}

// Quantum Random Number Generator integration
pub async fn fetch_quantum_random_numbers(batch_size: usize) -> Result<Vec<u64>, String> {
    unsafe {
        // Ensure only one request per minute
        if let Some(last_request) = LAST_QRNG_REQUEST {
            let elapsed = last_request.elapsed();
            if elapsed < Duration::from_secs(60) {
                let wait_time = Duration::from_secs(60) - elapsed;
                println!("Rate limit reached. Waiting for {} seconds...", wait_time.as_secs());
                sleep(wait_time).await;
            }
        }

        // Update the last request time
        LAST_QRNG_REQUEST = Some(Instant::now());
    }

    let url = format!(
        "https://qrng.anu.edu.au/API/jsonI.php?length={}&type=uint16",
        batch_size
    );

    let response = reqwest::get(&url)
        .await
        .map_err(|_| "Failed to fetch QRNG numbers")?
        .json::<Value>()
        .await
        .map_err(|_| "Failed to parse QRNG response")?;

    let new_numbers = response["data"]
        .as_array()
        .ok_or("Failed to parse QRNG numbers")?
        .iter()
        .map(|num| num.as_u64().unwrap_or(0))
        .collect::<Vec<u64>>();

    Ok(new_numbers)
}

// Modify the main function to log metrics for each block
#[tokio::main]
async fn main() {
    // Fetch a batch of QRNG numbers
    let qrng_numbers = match fetch_quantum_random_numbers(500).await {
        Ok(numbers) => numbers,
        Err(_) => {
            println!("Failed to fetch QRNG numbers. Exiting...");
            return;
        }
    };

    // Create blockchain with QRNG numbers
    let mut blockchain = Blockchain::new(qrng_numbers);

    // Add blocks with both RNGs
    let data = vec![
        "Dr. Kogge sells his Ancient Aliens Live T-shirt for 10 BTC to Jonathan.",
        "Jonathan sells the Ancient Aliens Live T-shirt for 1000 BTC on Ebay", 
        "The Ancient Aliens Live T-shirt is sold for 1,000,000 BTC in an auction to the Smithsonian", 
        "The Smithsonian trades the Acient Aliens Live T-shirt for Mars with the Martians",
        "The Martians trade the Ancient Aliens Live T-shirt for the Death Star with Darth Vader", 
        "Darth Vader trades the Ancient Aliens Live T-shirt to beat Obi-Wan Kenobi in a lightsaber duel"
    ];

    for (i, block_data) in data.iter().enumerate() {
        let use_classical_rng = i % 2 == 1; // Toggle RNG type every block
        match blockchain.add_block(block_data.to_string(), use_classical_rng).await {
            Ok(_) => println!("Block added successfully."),
            Err(e) => {
                println!("Failed to add block: {}", e);
                break;
            }
        }
    }    

    // Display the blockchain
    for block in blockchain.chain.iter() {
        println!("{:?}", block);
    }

    // Validate the blockchain
    println!("Is the blockchain valid? {}", blockchain.is_valid());
}