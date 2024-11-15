# Dubstep

**Dubstep** is a lightweight and straightforward way to act as a spy node on the **Solana Gossip Protocol**. Designed with simplicity in mind, it allows users to monitor the Solana network and capture key information relayed across the gossip protocol without the need for a full validator setup. 

## Features

- Connects to the Solana Gossip Protocol with minimal resource usage.
- Monitors and captures real-time network gossip information.
- Lightweight and optimized for simple deployment as a spy node.
- Built with flexibility to be integrated with other Solana monitoring or analysis tools.

## TODO

This project aims to implement a Solana-inspired gossip protocol to act like a spy node. Below are key features and tasks:

### Core Functionality
- [ ] **Create Connections**  
  Establish secure and reliable connections between nodes. Include peer discovery and management.

- [ ] **Implement Ping-Pong Mechanism**  
  Implement the `Ping` and `Pong` message exchange to check the liveness of nodes and maintain active connections.

- [ ] **Implement PullRequest**  
  Allow nodes to exchange information about each other's states and connected peers. This includes implementing `PullRequest` to request information.

### Testing and Validation
- [ ] **Unit Tests**  
  Write unit tests for each protocol message (Ping, Pong, PullRequest, etc.) to ensure correct functionality.

- [ ] **Integration Tests**  
  Create integration tests to simulate interactions between multiple nodes, testing the full gossip protocol under various network conditions.


### Documentation
- [ ] **Detailed Documentation of Each Protocol Step**  
  Document each part of the protocol, with usage examples and diagrams if necessary, to help others understand the implementation.

- [ ] **Add Usage Examples**  
  Add example scripts or code snippets that demonstrate how to use each protocol feature in a real-world scenario.


## Getting Started

### Prerequisites

To run Dubstep, ensure you have:

- **Rust** (latest stable version) installed. [Install Rust](https://www.rust-lang.org/tools/install).
- A basic understanding of the Solana protocol and gossip network.

## Development

### Project Structure
```bash
-dubstep/                      # The workspace
├── gossip/                    # The crate
│  ├── src/                    # Contains code and unit tests
│  │   ├── lib.rs             
│  │   └── ...                 # Others modules
│  ├── tests/                  # Contains integration test files
│  └── Cargo.toml              # Crate dependecies and metadata
└── Cargo.toml                 # Workspace dependencies and metadata
```

### Running Tests

!!!You need set the env file to run the tests

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
