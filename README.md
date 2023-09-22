# esso

[Latest Version]: https://img.shields.io/crates/v/esso.svg

Esso is a small and simple crate offering a handy macro for writing tests, akin to Jest.

The idea behind it is to make it easier to read and write tests in Rust.

It accommodates both synchronous and asynchronous tests. Have a look at the `it.rs` source code if you want to see what
the tests expand to it.

## Example

**Synchronous example**

```rust
it!("should check if a file exists", {
     assert!(std::fs::metadata("/tmp/hello.txt").is_ok());
});
```

**Asynchronous example**

```rust
it!("should asynchronously check if a file exists", async {
     assert!(tokio::fs::metadata("/tmp/hello.txt").await.is_ok());
});
```

## Installation

```
[dependencies]
esso = "1.0.0"
```

## Contributing

Contributions are welcome! Please open an issue or submit a pull request.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details