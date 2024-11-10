# Dubstep

**Dubstep** is a lightweight and straightforward way to act as a spy node on the **Solana Gossip Protocol**. Designed with simplicity in mind, it allows users to monitor the Solana network and capture key information relayed across the gossip protocol without the need for a full validator setup. 

## Features

- Connects to the Solana Gossip Protocol with minimal resource usage.
- Monitors and captures real-time network gossip information.
- Lightweight and optimized for simple deployment as a spy node.
- Built with flexibility to be integrated with other Solana monitoring or analysis tools.

## Getting Started

### Prerequisites

To run Dubstep, ensure you have:

- **Rust** (latest stable version) installed. [Install Rust](https://www.rust-lang.org/tools/install).
- A basic understanding of the Solana protocol and gossip network.

## Development

### Project Structure

- **src/**: Contains the main Rust source files, including network connection and gossip handling.
- **Cargo.toml**: Project dependencies and metadata.

### Running Tests

Run tests to validate the functionality:

```bash
cargo test
```

## Contributing

Contributions to Dubstep are welcome! If you’d like to contribute, please fork the repository and create a pull request.

1. Fork the project
2. Create your feature branch (`git checkout -b feature/YourFeature`)
3. Commit your changes (`git commit -m 'Add YourFeature'`)
4. Push to the branch (`git push origin feature/YourFeature`)
5. Open a pull request

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

---

**Note**: This project is not affiliated with or endorsed by Solana Labs. Dubstep is developed independently as a tool for interacting with the Solana Gossip Protocol.

---

### Disclaimer

Dubstep is intended for educational and monitoring purposes only. Use it responsibly and ensure compliance with any applicable Solana network policies.
