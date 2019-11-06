# eip1962-evmc

[EIP-1962] implementation as an [EVMC] precompile module ([EIP-2003] defines the semantics for EVMC precompiles).
Uses Matter Labs' [eip1962](https://github.com/matter-labs/eip1962) crate.

### Building

Run:
```shell
$ cargo +nightly build --release
```

The resulting binary will be at `target/release/libeip1962.{so,dylib}`.

It can be used with any EVMC compatible client.

## Author(s)

Alex Beregszaszi

## License

Apache 2.0

[EVMC]: https://github.com/ethereum/evmc
[EIP-1962]: https://eips.ethereum.org/EIPS/eip-1962
[EIP-2003]: https://github.com/ethereum/EIPs/pull/2003
