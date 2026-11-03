# solana-foundation-kora

Relayer implementation: a library and a CLI crate that let a service sign and
submit Solana transactions on behalf of a user.

## Scope

- builds transactions from a request
- signs with a pluggable back end (`src/signer.rs`)
- enforces a lamport ceiling per request (`src/fees.rs`)
- never custody funds; the fee payer is configured, not derived at runtime

## CLI

```sh
kora config     # prints the effective configuration
kora signer     # prints the configured public key
```

The fee ceiling defaults to 5,000,000 lamports and is the first thing a caller
hits in production when a request is wrong.

## Layout

```
src/lib.rs      library entry
src/config.rs   config parsing with defaults
src/fees.rs     fee policy
src/signer.rs   signing trait + in-memory back end
src/main.rs     CLI
```

## Status

Early. `cargo test` covers the signer contract and the fee policy.
