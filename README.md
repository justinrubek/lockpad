# lockpad

A simple authentication service REST API.
*lockpad* provides an HTTP service (via [axum](https://github.com/tokio-rs/axum)) that can handle user registration and login.
Authentication yields a JWT that is signed by the provided keys.
Utility functionality is included, especially for axum.

This is still being fleshed out, but is already useful for prototyping.

## Getting started

The primary development dependency is [nix](https://nixos.org/), with the project being bundled as a nix flake.

For tips on using the provided development environment, see [`HACKING.md`](HACKING.md).


### running

The authentication service can be ran: `cargo run --bin lockpad-cli server http`.
You'll need to configure environment variables for:

- Postgres connection
- Secret/public keys

The provided cli can be used to generate keys: `cargo run --bin lockpad-cli -- --help`
