//In this file, we will write the code that executes our quantum blockchain

//Environment variable to enable backtrace
// RUST_BACKTRACE=1; 

//imports 
use serde::{Serialize, Deserialize}; 
use sha2::{Sha256, Digest}; 
use std::time::{SystemTime, UNIX_EPOCH}; 
use reqwest; 

//block struct
#[derive(Serialize, Deserialize, Debug)] //derive the Serialize and Deserialize traits
pub struct Block {
    pub index: u32, 
    pub timestamp: u64, 
    pub data: String, 
    pub previous_hash: String, 
    pub nonce: u64, 
    pub hash: String, 
}

//Blockchain struct 
pub struct Blockchain{
    pub chain: Vec<Block>, 
    pub difficulty: usize, 
}

//Block implementation
impl Block {
    //Create New Block
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

    //Calculate Hash
    pub fn calculate_hash(&self) -> String {
        let input = format!(
            "{}{}{}{}{}", 
            self.index,self.timestamp, self.data, self.previous_hash, self.nonce
        ); 
        let mut hasher = Sha256::new(); 
        hasher.update(input); 
        format!("{:x}", hasher.finalize())
    }
} 

//Blockchain implementation
impl Blockchain {
    // Create a new blockchain with the genesis block
    pub fn new() -> Self {
        Blockchain {
            chain: vec![Blockchain::create_genesis_block()], 
            difficulty: 3,
        }
    }

    //Create Genesis Block
    fn create_genesis_block() -> Block {
        Block::new(0, "Dr. Kogge buys Ancient Aliens Live T-shirt for 20 USD.".to_string(), "0".to_string())
    }

    //Add a new block to the blockchain
    pub async fn add_block(&mut self, data: String) {
        let previous_hash = self.chain.last().unwrap().hash.clone();
        let mut new_block = Block::new(self.chain.len() as u32, data, previous_hash);
        self.proof_of_work(&mut new_block).await; // Await the async PoW
        self.chain.push(new_block);
    }

    //Perform Proof of Work
    //Standard Proof of Work algorithm
    // fn proof_of_work(&self, block: &mut Block) { 
    //     while !block.hash.starts_with(&"0".repeat(self.difficulty)) { 
    //         block.nonce +=1; 
    //         block.hash = block.calculate_hash();
    //     }
    //     println!("Block {} mined with hash: {}", block.index, block.hash);
    // }
    //Perform Proof of Work with Quantum Random Number Generator
    pub async fn proof_of_work(&self, block: &mut Block) {
        while !block.hash.starts_with(&"0".repeat(self.difficulty)) {
            let new_nonce = match fetch_quantum_random_number().await {
                Ok(random_nonce) => random_nonce,
                Err(_) => block.nonce + 1, // Fallback to incrementing the nonce
            };

            block.nonce = new_nonce;
            block.hash = block.calculate_hash();
        }

        println!(
            "Block {} mined with QRNG-powered nonce: {} and hash: {}",
            block.index, block.nonce, block.hash
        );
    }
    
    //Validate the blockchain
    pub fn is_valid(&self) -> bool {
        for i in 1..self.chain.len() {
            let current = &self.chain[i]; 
            let previous = &self.chain[i-1]; 

            //Check if the current block hash is correct
            if current.hash != current.calculate_hash() {
                return false; 
            }

            //Check if the current block's previous hash matches the previous block's hash
            if current.previous_hash != previous.hash {
                return false; 
            }
        }
        true
    }
}

//Get current timestamp as a Unix Timestamp
pub fn now() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs()
}
//Quantum Random Number Generator integration 
pub async fn fetch_quantum_random_number() -> Result<u64, reqwest::Error> {
    let response = reqwest::get("https://qrng.anu.edu.au/API/jsonI.php?length=1&type=uint16")
    .await?
    .json::<serde_json::Value>()
    .await?;

    //parse the random number from the response
    let random_number = response["data"][0]
        .as_u64()
        .expect("Could not parse random number");

    Ok(random_number)
}

//main function
//Standard Proof of Work main execution function 
// fn main() { 
//     //Create a new blockchain
//     let mut blockchain = Blockchain::new(); 

//     //Add new blocks to the blockchain
//     blockchain.add_block("Dr. Kogge's sells his Ancient Aliens Live T-shirt for 10 BTC to Jonathan.".to_string()); 
//     blockchain.add_block("Jonathan sells the Ancient Aliens Live T-shirt for 1000 BTC on Ebay".to_string());
//     blockchain.add_block("The Ancient Aliens Live T-shirt is sold for 1,000,000 BTC to the Smithsonian".to_string());
//     blockchain.add_block("The Martians trade Mars for the Ancient Aliens Live T-shirt".to_string());

//     //Display the blockchain
//     for block in blockchain.chain.iter() {
//         println!("{:?}", block);
//     }

//     //Validate the blockchain
//     println!("Is the blockchain valid? {}", blockchain.is_valid());
// }

//Quantum Random Number Generator Proof of Work main execution function
#[tokio::main]
async fn main() {
    //Create a new blockchain 
    let mut blockchain = Blockchain::new(); 

    //Add new blocks to the blockchain
    blockchain.add_block("Dr. Kogge's sells his Ancient Aliens Live T-shirt for 10 BTC to Jonathan.".to_string()).await; 
    blockchain.add_block("Jonathan sells the Ancient Aliens Live T-shirt for 1000 BTC on Ebay".to_string()).await;
    blockchain.add_block("The Ancient Aliens Live T-shirt is sold for 1,000,000 BTC to the Smithsonian".to_string()).await;
    blockchain.add_block("The Martians trade Mars for the Ancient Aliens Live T-shirt".to_string()).await;

    //Display the blockchain
    for block in blockchain.chain.iter() {
        println!("{:?}", block); 
    }
    
    //Validate the blockchain
    println!("Is the blockchain valid? {}", blockchain.is_valid());
}
