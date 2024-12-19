import pandas as pd
import matplotlib.pyplot as plt

# Load data
metrics_file = "metrics.csv"
quantum_file = "quantum_nonces.csv"
classical_file = "classical_nonces.csv"

# Load metrics data
metrics_df = pd.read_csv(metrics_file, names=["Block Index", "Source", "Iterations", "Duration"], sep=",", header=None)
metrics_df["Duration"] = pd.to_numeric(metrics_df["Duration"].str.replace("ms", ""), errors="coerce")

# Load nonces data
quantum_nonces_df = pd.read_csv(quantum_file, names=["Nonce"], header=None)
quantum_nonces_df["Source"] = "Quantum"

classical_nonces_df = pd.read_csv(classical_file, names=["Nonce"], header=None)
classical_nonces_df["Source"] = "Classical"

# Combine nonce data
nonces_df = pd.concat([quantum_nonces_df, classical_nonces_df], ignore_index=True)

# Analyze Metrics Data
print("Metrics Analysis:")
print(metrics_df.describe())

# Analyze Nonces Data
print("\nNonce Analysis:")
print(nonces_df.groupby("Source").describe())

# Visualization

# 1. Nonce Distribution
plt.figure(figsize=(10, 6))
plt.hist(
    quantum_nonces_df["Nonce"], bins=30, alpha=0.6, label="Quantum", color="blue", edgecolor="black"
)
plt.hist(
    classical_nonces_df["Nonce"], bins=30, alpha=0.6, label="Classical", color="orange", edgecolor="black"
)
plt.title("Nonce Distribution: Quantum vs Classical", fontsize=16, fontweight='bold')
plt.xlabel("Nonce", fontsize=14)
plt.ylabel("Frequency", fontsize=14)
plt.legend(title="Source", fontsize=12, title_fontsize=14, loc="upper right")
plt.grid(True)
plt.show()

# 2. Mining Iterations
plt.figure(figsize=(10, 6))
for source, group in metrics_df.groupby("Source"):
    plt.bar(group["Block Index"], group["Iterations"], label=source, alpha=0.7)
plt.title("Mining Iterations per Block", fontsize=16, fontweight='bold')
plt.xlabel("Block Index", fontsize=14)
plt.ylabel("Iterations", fontsize=14)
plt.legend(title="Source", fontsize=12, title_fontsize=14)
plt.grid(True)
plt.show()

# 3. Mining Duration
plt.figure(figsize=(10, 6))
for source, group in metrics_df.groupby("Source"):
    plt.bar(group["Block Index"], group["Duration"], label=source, alpha=0.7)
plt.title("Mining Duration per Block", fontsize=16, fontweight='bold')
plt.xlabel("Block Index", fontsize=14)
plt.ylabel("Duration (ms)", fontsize=14)
plt.legend(title="Source", fontsize=12, title_fontsize=14)
plt.grid(True)
plt.show()

# 4. Iterations vs Duration
plt.figure(figsize=(10, 6))
for source, group in metrics_df.groupby("Source"):
    plt.scatter(group["Iterations"], group["Duration"], label=source, alpha=0.7)
plt.title("Iterations vs Duration", fontsize=16, fontweight='bold')
plt.xlabel("Iterations", fontsize=14)
plt.ylabel("Duration (ms)", fontsize=14)
plt.legend(title="Source", fontsize=12, title_fontsize=14)
plt.grid(True)
plt.show()
