# eip1962-evmc

[EIP1962] implementation as an [EVMC] precompile module ([EIP2003] defines the semantics for EVMC precompiles).

*Note*: It doesn't currently implement gas handling properly and doesn't support the pairing operation.

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
[EIP1962]: https://eips.ethereum.org/EIPS/eip-1962
[EIP2003]: https://github.com/ethereum/EIPs/pull/2003
