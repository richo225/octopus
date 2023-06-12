<h1 align="center">
  Octopus
</h1>
<p align="center">
  Trading <a href="https://lib.rs/crates/octopus-cli/" target="_blank">CLI</a> and <a href="https://lib.rs/crates/octopus-engine/" target="_blank">engine</a> for submitting and matching orders. 🐙
</p>
<p align="center">
  Built with <a href="https://www.rust-lang.org/" target="_blank">Rust</a>, consuming a <a href="https://lib.rs/crates/warp" target="_blank">Warp</a> API and hosted on <a href="https://www.railway.app/" target="_blank">Railway</a>.
</p>

<p align="center">
  <a href="https://lib.rs/crates/octopus-cli" target="_blank">
    <img src="https://img.shields.io/crates/v/octopus-cli?label=cli" />
  </a>
  <a href="https://lib.rs/crates/octopus-engine" target="_blank">
    <img src="https://img.shields.io/crates/v/octopus-engine?label=engine" />
  </a>
  <a href="https://github.com/richo225/octopus/actions/workflows/ci.yml" target="_blank">
    <img src="https://github.com/richo225/octopus/actions/workflows/ci.yml/badge.svg" />
  </a>
  <a href="https://github.com/richo225/octopus/blob/main/LICENSE.txt" target="_blank">
    <img src="https://img.shields.io/badge/license-MIT-blue.svg" />
  </a>
  <a href="https://octopus-web.up.railway.app" target="_blank">
    <img src="https://img.shields.io/website?label=railway&&up_message=success&url=https%3A%2F%2Foctopus-web.up.railway.app%2F" />
  </a>
</p>

![demo](https://github.com/richo225/octopus/assets/18379191/0a3efa31-f6c3-4b40-9942-e7c14d88e019)

## Installation

### Cargo

Install the rust toolchain in order to have cargo installed by using [this](https://www.rust-lang.org/tools/install) guide. Then install the cli crate with the following;

```shell
cargo install octopus-cli
```

You can also install the crate by cloning this repository and building it using Cargo. Run the following commands in your terminal:

### Build

```shell
git clone https://github.com/richo225/octopus.git
cd octopus
cargo build --target octopus_cli --release
```

After a successful build, the binary will be available in the target/release directory.

## Usage

To run the crate, use the following command in your terminal:

```shell
octopus-cli
```

If you would like to run the octopus server locally, execute the binary with:

```shell
RUST_LOG=trace cargo run --bin octopus-web
```

And then run the CLI, pointing to the local server:

```shell
cargo run --bin octopus-cli -- http://localhost:8080
```

Full documentation for the engine can be found at https://docs.rs/octopus-engine/0.1.0/octopus_engine/

## Commands

### `deposit`

Allows users to create an account or deposit funds into an existing account.

### `withdraw`

Withdraw funds from a users account.

### `send`

Send funds to another user's account.

### `submit_order`

The submit_order command enables users to submit an order for processing by the engine. A receipt will be returned along with any matches.

### `orderbook`

Retrieves the current order book.

### `account`

Retrieves the user's account balance.

### `txlog`

The txlog command retrieves the entire transaction log on the platform.

## Testing

To run the tests for the crate, use the following command in your terminal:

```shell
cargo test
```

This will execute the test cases for both the cli and engine and provide you with the test results.

## Contributing

Contributions are welcome! If you find a bug, have a feature request, or want to contribute code, please follow the guidelines in the contributing file.

## License

This crate is licensed under the MIT License.
