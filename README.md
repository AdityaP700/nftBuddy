# Shinobi 

**A CLI for Solana NFT intelligence, forged in Rust.**
<img width="1488" height="676" alt="image" src="https://github.com/user-attachments/assets/fb70099e-3859-4256-b699-65726704d9c1" />

`shinobi` is a command-line tool that bypasses slow, cluttered UIs to deliver instant, high-signal forensic reports on Solana NFTs and wallets directly in your terminal. It's built for speed, accuracy, and a beautiful user experience.

---

###  Features

*   **Forensic `unmask`:** Get a complete intelligence report on any NFT.
*   **Wallet `dossier`:** Generate a high-level profile on any wallet.
*   **Visual Confirmation (`--image`):** Render a full-color image of the NFT in your terminal.
*   **Interactive Shell:** Enter a persistent, interactive session for rapid analysis.
*   **Blazingly Fast:** Built in Rust on the Tokio async runtime.

### 演示 (Demo)

- with `unmask` command :
  <img width="1071" height="611" alt="image" src="https://github.com/user-attachments/assets/088fffd2-8402-4208-b6d9-2e71f88b637f" />

- with `unmask && --image` commmand :
 <img width="1514" height="688" alt="image" src="https://github.com/user-attachments/assets/f9ad082d-0363-4d69-a0b0-b2eca08225ef" />
 <img width="805" height="1011" alt="image" src="https://github.com/user-attachments/assets/54402009-48c3-4f9a-ae38-882b4d87ebf7" />

- with `dossier` command :
<img width="975" height="232" alt="image" src="https://github.com/user-attachments/assets/7a8f0487-ef1c-4f43-80df-7116eff22be9" />


---

###  Project Genesis & The Shinobi's Path

This project was born from a classic developer's challenge: bridging the gap between theory and practice.

#### The Motivation

After working through the first half of "The Rust Programming Language," I understood the core concepts of ownership, structs, and error handling. However, I wanted to apply these powerful ideas to a real-world problem that was both complex and interesting. With a personal goal of diving into Web3, specifically Solana, `shinobi` became the perfect mission. It was designed from the ground up to be a practical exercise in using Rust's fundamentals to tame the wild, asynchronous, and often unpredictable world of blockchain data.

#### The Challenges & Lessons Learned

Building `shinobi` was a journey of overcoming hurdles that are common when moving from simple exercises to a real-world application:

*   **🧠 The Rust Mindset:** Moving beyond syntax to truly think in terms of ownership and borrowing was the first great challenge. The compiler became a strict but fair Sensei, forcing a disciplined approach to data management that results in a safer, more efficient program.

*   **⛓️ The Blockchain's Chaos:** The Solana RPC doesn't always return clean, predictable data. The biggest breakthrough was engineering the tool to handle the messy reality of the chain:
    *   Distinguishing between **Fungible Tokens** (like Serum) and **Non-Fungible Tokens**.
    *   Gracefully handling **empty wallets** and **invalid addresses**.
    *   Parsing raw, binary **on-chain data** in addition to standard off-chain JSON.

*   **🚀 Taming the Async Beast:** Making multiple, chained network requests without freezing the application required a deep dive into Rust's `async/await` and the Tokio runtime. This was crucial for building a tool that feels fast and responsive, especially for the multi-request `dossier` and `--image` features.

*   **✨ Crafting a Polished Tool:** The final challenge was moving from a script that "works" to a professional CLI that is a joy to use. This meant designing a clean API with `clap`, creating a beautiful and informative display with `colored`, and building an immersive interactive shell.

---

### The Shinobi's Art: Forged in Rust

This tool is a practical showcase of the core Rust principles that make it ideal for high-performance blockchain applications.

*   **Fearless Concurrency (`async/await`):** Built on the Tokio async runtime to handle multiple network requests without freezing, providing a fast and responsive experience.
*   **Guaranteed Safety (Ownership & Borrowing):** Rust's ownership model guarantees memory safety at compile time, eliminating entire classes of bugs.
*   **Zero-Cost Abstractions (Structs & Enums):** Raw JSON from the Solana RPC is instantly transformed into strongly-typed Rust `struct`s using `serde`, making illegal states unrepresentable.
*   **Resilience by Default (`Result` & `?`):** Every network call and parsing operation is wrapped in a `Result`, ensuring the tool fails gracefully with clear errors instead of crashing.

---

### ⛩️ Installation

There are two ways to install and use `shinobi`.

#### Method 1: For Rust Developers (`cargo install`)

If you have the Rust toolchain installed, you can install `shinobi` directly from this repository:

```bash
cargo install --git https://github.com/<YourUsername>/shinobi.git
```
*(Once published on Crates.io, this will become a simple `cargo install shinobi`)*

#### Method 2: From GitHub Releases (For All Users)

Pre-compiled binaries for major operating systems are available on the [Releases Page](https://github.com/<YourUsername>/shinobi/releases).

1.  Download the latest release for your OS.
2.  Unzip the file.
3.  Place the `shinobi` executable in a directory that is in your system's `PATH`.

---

### 📜 Usage

`shinobi` is simple to use. The main commands are `unmask` and `dossier`.

#### Unmasking an NFT

To get a full forensic report on an NFT, use its **Mint Address**.

```bash
shinobi unmask <NFT_MINT_ADDRESS>
```

To also render the image in the terminal:
```bash
shinobi unmask <NFT_MINT_ADDRESS> --image
```

#### Generating a Wallet Dossier

To get a profile on a collector, use their **Wallet Address**.

```bash
shinobi dossier <WALLET_ADDRESS>
```

---

###  Trial Targets

Want to see it in action? Here are some guaranteed targets to try.

*   **Unmask a Mad Lads NFT (Full Report & Image):**
    ```bash
    shinobi unmask H7rwmPS41aJcn3x5GRjDa9e8jMyrCXRcbUR8x31JvyH --image
    ```

*   **Generate a Dossier on a Collector's Wallet:**
    ```bash
    shinobi dossier Gg9ja926hJd5Yksc235p21G32xH6e1zGDBAd95aT1xAF
    ```

*   **Analyze a Fungible Token (Edge Case Handling):**
    ```bash
    shinobi unmask SRMuApVNdxXokk5GT7XD5cUUgXMBCoAz2LHeuAoKWRt
    ```

---

### 🛠️ Configuration

The tool uses a Helius RPC for reliable and fast data fetching.

1.  Sign up for a free Helius API key at [helius.xyz](https://helius.xyz).
2.  Create a `.env` file in the root of the project (or in your home directory).
3.  Add your API key to the `.env` file:
    ```
    RPC_URL="https://mainnet.helius-rpc.com/?api-key=YOUR_API_KEY_HERE"
    ```
