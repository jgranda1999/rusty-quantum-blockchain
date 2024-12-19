# Quantum-Enhanced Blockchain Project

## Overview
This project integrates Quantum Random Number Generators (QRNGs) into a blockchain consensus mechanism to enhance fairness and security. By leveraging QRNGs, the system ensures truly random values for nonce generation, improving unpredictability and reducing biases compared to classical random number generators (RNGs). The project demonstrates the use of quantum-enhanced randomness in a blockchain simulation.

## Objectives
- **Improve Fairness**: Incorporate quantum randomness to eliminate biases in validator selection and block mining.
- **Enhance Security**: Strengthen blockchain security by using unpredictable quantum-generated nonces.
- **Compare Performance**: Evaluate the efficiency of QRNGs versus classical RNGs in mining operations.

## Key Features
- **Quantum Randomness Integration**: Utilizes QRNG APIs to fetch truly random numbers.
- **Classical vs Quantum Analysis**: Alternates between QRNG and classical RNG to compare mining performance.
- **Performance Metrics**: Logs mining iterations, durations, and nonce distributions.
- **Data Visualization**: Generates insightful plots to illustrate nonce distributions and mining performance.

## Installation
### Prerequisites
- **Rust**: Install Rust from [rust-lang.org](https://www.rust-lang.org/).
- **Python**: For data analysis and visualization, ensure Python 3.7+ is installed.
- **Dependencies**:
  - Rust crates: `tokio`, `reqwest`, `serde`, `sha2`, `rand`, `anyhow`
  - Python packages: `pandas`, `matplotlib`

### Steps
1. Clone the repository:
   ```bash
   git clone <repository_url>
   cd quantum-blockchain
   ```

2. Build and run the Rust blockchain simulation:
   ```bash
   cargo build
   cargo run
   ```

3. Analyze the data:
   ```bash
   python3 data_analysis.py
   ```

## Project Structure
```
quantum-blockchain/
├── src/
│   ├── main.rs        # Rust blockchain implementation
│   └── lib.rs         # Additional utilities
├── metrics.csv        # Performance metrics log
├── quantum_nonces.csv # QRNG-generated nonces
├── classical_nonces.csv # Classical RNG-generated nonces
├── data_analysis.py   # Python script for analysis and visualization
└── README.md          # Project documentation
```

## Metrics and Analysis
### Metrics Summary:
- **Average Mining Iterations**: 263.8
- **Average Mining Duration**: 3.59 ms
- **QRNG Nonce Mean**: 34,141.21
- **Classical RNG Nonce Mean**: 33,239.31

### Key Observations:
- Quantum randomness resulted in a more uniform distribution of nonces.
- Mining with QRNG demonstrated slightly faster convergence for certain blocks.
- Enhanced fairness and reduced predictability in nonce selection.

## Results Visualization
![Nonce Distribution: Quantum vs Classical](./plots/nonce_distribution.png)

## How It Works
1. **QRNG Integration**: Fetches quantum random numbers using the [Australian National University QRNG API](https://quantumnumbers.anu.edu.au/).
2. **Mining Process**: Alternates between quantum and classical randomness for nonce selection.
3. **Performance Logging**: Captures metrics such as iterations, duration, and source of randomness.
4. **Data Analysis**: Uses Python to analyze and visualize the logged data.

## Conclusion
This project demonstrates the practical application of QRNG in blockchain technology, highlighting its potential to:
- Improve fairness in decentralized systems.
- Mitigate vulnerabilities associated with predictable randomness.
- Set a foundation for quantum-safe blockchain systems.

## Future Work
- **Scaling**: Extend the prototype to larger blockchain networks.
- **Security Testing**: Evaluate resistance to attacks exploiting classical RNG predictability.
- **Optimization**: Reduce latency in QRNG integration.

## Contributions
- **Author**: Jonathan Granda Acaro

## License
This project is licensed under the MIT License. See `LICENSE` for more details.
