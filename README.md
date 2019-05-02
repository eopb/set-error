# set-error

A very simple trait that overwrites errors with `Strings` for `Result`s and `Option`s.

[![pipeline status](https://gitlab.com/efunb/set-error/badges/stable/pipeline.svg)](https://gitlab.com/efunb/set-error/commits/stable)
[![License](https://img.shields.io/crates/l/set-error.svg)](https://crates.io/crates/set-error)
[![Latest version](https://img.shields.io/crates/v/set-error.svg)](https://crates.io/crates/set-error)
[![Latest Docs](https://docs.rs/set-error/badge.svg)](https://docs.rs/set-error/)
[![downloads-badge](https://img.shields.io/crates/d/set-error.svg)](https://crates.io/crates/set-error)

When used with the `?` operator it makes a nice pattern. [Documentation](https://docs.rs/set-error/)

```rust
File::open("config.yaml")
    .set_error("Failed to open config file.")?
    .read_to_string(&mut contents)
    .set_error("Failed to read file content.")?;
```

## Help

If you run into any issues or need help with using `read_input` in your project please email [incoming+efunb-set-error-12103568-issue-@incoming.gitlab.com](mailto:incoming+efunb-set-error-12103568-issue-@incoming.gitlab.com)

## **Warning**

**If you are viewing this from GitHub then this is a read only copy. Please contribute to the GitLab copy [here](https://gitlab.com/efunb/set-error).**