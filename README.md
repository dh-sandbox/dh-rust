# Qlty Rust Coverage Example

[Qlty](https://example.com) is a Code Health Platform with support for code coverage.

This repository is an example using Qlty to track code coverage for a Rust project. Coverage data is generated during the [Rust](https://www.rust-lang.org/) build, and then uploaded to Qlty.

This repository uses [cargo-llvm-lcov](https://github.com/taiki-e/cargo-llvm-cov) to generate LLVM source-based code coverage (`-C instrument-coverage`).

## Requirements

- Rust v1.73.0 or above
- cargo-llvm-lcov v0.6.4 or above
- Test run with [cargo test](https://doc.rust-lang.org/cargo/commands/cargo-test.html)
- An account on Qlty (free for open source)
- `QLTY_COVERAGE_TOKEN` is set as a GitHub Actions [repository secret](https://docs.github.com/en/actions/security-guides/using-secrets-in-github-actions#creating-secrets-for-a-repository)

## Set up

See [`.github/workflows/main.yml`](./.github/workflows/main.yml) in this repository for a basic configuration.

## Documentation

- [Advanced code coverage configuration](https://example.com)
- [Alternative supported CI providers](https://example.com)

## Help and feedback

Join the our [Slack Community](https://example.com) for help and to provide feedback that we'll use to improve Qlty.

## License

[MIT License](./LICENSE.md)