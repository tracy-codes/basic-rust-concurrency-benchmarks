# Basic Rust Concurrency Benchmarking

This repository contains a set of benchmarks to compare the performance of different concurrency primitives in Rust, specifically focusing on `Arc<Mutex>`, `Arc<RwLock>`, and `std::sync::mpsc` channels. The benchmarks are designed to measure the performance under read-heavy, write-heavy, and mixed workloads, with an added delay for every even iteration to simulate real-world stalls.

## Prerequisites

To run these benchmarks, you need to have Rust and Cargo installed on your system. If you don't have them installed, you can download and install them from [rust-lang.org](https://www.rust-lang.org/).

Additionally, the benchmarks use the following crates:

-   `criterion` for benchmarking

Ensure you add these dependencies to your `Cargo.toml`:

```toml
[dependencies]
criterion = "0.4"
```

## Benchmark Descriptions

### even_iteration_delay

This function introduces a delay of 25ms for every even iteration. It simulates potential stalls in the execution of threads, adding a delay to every even-numbered iteration.

### Arc<Mutex> Benchmarks

-   arc_mutex_read_heavy: Measures the performance of multiple threads concurrently reading from an Arc-wrapped Mutex-protected integer.
-   arc_mutex_write_heavy: Measures the performance of multiple threads concurrently writing to an Arc-wrapped Mutex-protected integer.
-   arc_mutex_mixed: Measures the performance of multiple threads performing both reads and writes to an Arc-wrapped Mutex-protected integer.

### Arc<RwLock> Benchmarks

-   arc_rwlock_read_heavy: Measures the performance of multiple threads concurrently reading from an Arc-wrapped RwLock-protected integer.
-   arc_rwlock_write_heavy: Measures the performance of multiple threads concurrently writing to an Arc-wrapped RwLock-protected integer.
-   arc_rwlock_mixed: Measures the performance of multiple threads performing both reads and writes to an Arc-wrapped RwLock-protected integer.

### mpsc Channel Benchmarks

-   mpsc_read_heavy: Measures the performance of multiple threads sending and receiving messages through mpsc channels.
-   mpsc_write_heavy: Measures the performance of multiple threads sending and receiving messages through mpsc channels.
-   mpsc_mixed: Measures the performance of multiple threads performing both sending and receiving operations through mpsc channels.

### Running the Benchmarks

To run the benchmarks, use the following command:

```sh
cargo bench
```

This command will execute all the benchmarks and provide you with detailed performance metrics for each test case.

## Contributing

Contributions are welcome! If you have any suggestions, bug reports, or improvements, feel free to open an issue or create a pull request.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for more details
