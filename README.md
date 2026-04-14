# readme-code-extractor-core

Internal: `readme-code-extractor-core` is shared between `readme-code-extractor` and
`readme-code-extractor-proc`. Do not use directly. Only use through `readme-code-extractor`'s
macros.

## TOML only

We use only TOML deserialization with [`toml-rs/toml`](https://github.com/toml-rs). No other formats
(JSON, [`eternal-io/keon`](https://github.com/eternal-io/keon),
[`ron-rs/ron`](https://github.com/ron-rs/ron)... ). Why? Because TOML is

- simple and readable
- used by Rust community already
- both clean and expressive enough for simple Rust values, see `toml-rs/toml` ->
  - [`crates/toml/examples/enum_external.rs`](https://github.com/toml-rs/toml/blob/main/crates/toml/examples/enum_external.rs)
  - [`crates/toml/tests/serde/de_enum.rs`](https://github.com/toml-rs/toml/blob/main/crates/toml/tests/serde/de_enum.rs)
    -> `fn value_from_str()`
- supported by ["Extended **Embedded**
  Languages"](https://marketplace.visualstudio.com/items?itemName=ruschaaf.extended-embedded-languages)
  in VS Code, and that works for in raw strings passed to `#![doc = r#"..."#]` or `#[doc =
  r#"..."#]` (and other attributes).
