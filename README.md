# set-error

A very simple trait that overwrites errors with `Strings` for `Result`s and `Option`s.

When used with the `?` operator it makes a nice pattern.

```rust
File::open("config.yaml")
    .set_error("Failed to open config file.")?
    .read_to_string(&mut contents)
    .set_error("Failed to read file content.")?;
```

[docs](https://docs.rs/set-error/)