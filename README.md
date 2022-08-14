
# bosh-rs

bosh-rs is a highly configurable physics engine for the game Line Rider.

The official frontend of bosh-rs is [bosh]. The official implementation
of Line Rider can be found at [linerider.com].

This project was inspired by [conundrumer/lr-core], although it is structured
entirely differently in order to make it easier to use.

The only dependency of bosh is [serde] (and std).
This assures that the project is extremely portable and
even can run in WASM if one prefers.

# Installation

Add the following to your `Cargo.toml`:
```toml
[dependencies]
bosh-rs = "0"
```

[bosh]: https://github.com/deanveloper/bosh
[conundrumer/lr-core]: https://github.com/conundrumer/lr-core
[serde]: https://serde.rs/
[linerider.com]: https://linerider.com/
