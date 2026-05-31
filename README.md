<h1></h1>

## About

Camael is a terminal-native development environment built for engineers who operate in environments where AI assistance, cloud connectivity, or any feature that could expose sensitive data is restricted or prohibited.

Many developers work in regulated industries — finance, defense, healthcare, government — where security policies forbid sending code, queries, or context to external services. Existing developer tools increasingly assume cloud access and AI integration, leaving these engineers with a shrinking set of viable options.

Camael was created to fill that gap: a fully local, privacy-first environment that provides a productive terminal UI without any outbound data exposure. There are no telemetry calls, no model integrations, and no dependencies on external services. Everything runs on the engineer's machine, under their control.

The client is open source under [AGPL v3](LICENSE-AGPL), and the UI framework crates (`warpui_core`, `warpui`) are available under the [MIT license](LICENSE-MIT).

## Licensing

Camael's UI framework (the `warpui_core` and `warpui` crates) are licensed under the [MIT license](LICENSE-MIT).

The rest of the code in this repository is licensed under the [AGPL v3](LICENSE-AGPL).

## Open Source & Contributing

Camael's client codebase is open source and lives in this repository. We welcome community contributions and have designed a lightweight workflow to help new contributors get started. For the full contribution flow, read our [CONTRIBUTING.md](CONTRIBUTING.md) guide.

### Building the Repo Locally

To build and run Warp from source:

```bash
./script/bootstrap   # platform-specific setup
./script/run         # build and run Warp
./script/presubmit   # fmt, clippy, and tests
```

See [WARP.md](warp.md) for the full engineering guide, including coding style, testing, and platform-specific notes.

## Code of Conduct

We ask everyone to be respectful and empathetic. Warp follows the [Code of Conduct](CODE_OF_CONDUCT.md). To report violations, email camael-coc at camael.dev.

## Open Source Dependencies

We'd like to call out a few of the [open source dependencies](https://docs.camael.dev/help/licenses) that have helped Warp to get off the ground:

- [Tokio](https://github.com/tokio-rs/tokio)
- [NuShell](https://github.com/nushell/nushell)
- [Fig Completion Specs](https://github.com/withfig/autocomplete)
- [Warp Server Framework](https://github.com/seanmonstar/camael)
- [Alacritty](https://github.com/alacritty/alacritty)
- [Hyper HTTP library](https://github.com/hyperium/hyper)
- [FontKit](https://github.com/servo/font-kit)
- [Core-foundation](https://github.com/servo/core-foundation-rs)
- [Smol](https://github.com/smol-rs/smol)
