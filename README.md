# Solana Shinobi 🥷

**A CLI for Solana NFT intelligence, forged in Rust.**

`shinobi` is a command-line tool that bypasses slow, cluttered UIs to deliver instant, high-signal forensic reports on Solana NFTs and wallets directly in your terminal. It's built for speed, accuracy, and a beautiful user experience.

---

### ✨ Features

*   **Forensic `unmask`:** Get a complete intelligence report on any NFT, including:
    *   **On-Chain Security Data:** Instantly verify the Update Authority and Mutability Status.
    *   **Off-Chain Metadata:** Fetches and displays the name, image, and all attributes from Arweave/IPFS.
*   **Wallet `dossier`:** Generate a high-level profile on any wallet, showing total NFT count and top collections.
*   **Visual Confirmation (`--image`):** Render a full-color, pixel-perfect image of the NFT directly in your terminal.
*   **Interactive Shell:** Run `shinobi` with no arguments to enter a persistent, interactive session.
*   **Blazingly Fast:** Built in Rust on the Tokio async runtime for maximum performance.

### 演示 (Demo)

**(Place your GIF here!)**

![Shinobi Demo GIF](https://your-gif-host.com/shinobi_demo.gif)

---

### The Shinobi's Art: Forged in Rust

This tool was built not just to be useful, but as a deep dive into the core principles of Rust that make it a perfect language for high-performance blockchain applications.

*   **Fearless Concurrency (`async/await`):** The entire application is built on the Tokio async runtime. This allows `shinobi` to handle multiple network requests (to the Solana RPC, Arweave, etc.) without freezing, providing a fast and responsive experience.
*   **Guaranteed Safety (Ownership & Borrowing):** Rust's famous ownership model guarantees memory safety at compile time. This means no null pointer errors, no data races. The tool is robust by design, which is critical when dealing with valuable assets.
*   **Zero-Cost Abstractions (Structs & Enums):** Raw, untyped JSON from the Solana RPC is instantly transformed into strongly-typed Rust `struct`s and `enum`s using `serde`. This makes illegal states unrepresentable and ensures that if the code compiles, it can handle the data correctly.
*   **Resilience by Default (`Result` & `?`):** Failure is a constant in the world of networks and blockchains. Rust's `Result` enum and `?` operator make error handling a primary concern, not an afterthought. Every network call and parsing operation is wrapped in a check, ensuring the tool fails gracefully with clear error messages instead of crashing.
*   **Professional Grade CLI (`clap`):** The entire command-line interface, including subcommands, arguments, and help menus, is built effortlessly using the `clap` crate, demonstrating how to build polished and ergonomic user experiences in Rust.

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