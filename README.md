# rpi-db

This project is a simple key-value database built in Rust, designed to explore and enhance understanding of Rust's features and performance characteristics. The project includes integration tests and benchmarks to ensure reliability and efficiency.

## Features

- **In-Memory Key-Value Store**: Efficient storage and retrieval of key-value pairs.
- **REPL Interaction**: Provides a REPL interface for direct interaction that can be accessed in port 8081.
- **Concurrency**: Designed to handle concurrent read/write operations safely.
- **Persistence**: Supports saving the in-memory data to disk to allow restarting the program with its current values.

## Usage

### Running the Database

To start the database and interact via the REPL:
```bash
cargo run
```

### Integration Tests

To  run the integration testz, use the following command:

``` bash
cargo test --test integration
```

### Benchmarking

To benchmark the performance of the database:

``` bash
cargo bench
```

### Learning Purpose

This project was created with the purpose of learning Rust's concepts, particularly:
- Handling concurrency.
- Managing input/output with REPLs and servers.
- Learning how to benchmark with Rust.

### Contributing

Feel free to open issues or submit pull requests if you have suggestions or improvements!
