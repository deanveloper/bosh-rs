
# bosh-rs

bosh-rs is a highly configurable physics engine for the game Line Rider.

The official frontend of bosh-rs is [bosh]. The official implementation
of Line Rider can be found at [linerider.com].

This project was inspired by [conundrumer/lr-core], although it is structured
entirely differently in order to make it easier to use.

There are very few dependencies.
This assures that the project is extremely portable and
even can run in WASM if one prefers.

# Installation

Add the following to your `Cargo.toml`:
```toml
[dependencies]
bosh-rs = "0"
```

# Special Thanks

 * [Linerider-Advanced][lra] for making physics intuitive to
   understand without having to look at code
 * [LRA-Community-Edition][lra-ce] for being an actively maintained fork :)
 * [lr-core][conundrumer/lr-core], my go-to source code whenever I needed 
   to know the actual values for calculations
 * [ImHex] for being an easy-to-use tool for reverse-engineering binaries,
   which was very useful for decoding the .trk file format
 * The LineRider Community for being great :)

[bosh]: https://github.com/deanveloper/bosh
[conundrumer/lr-core]: https://github.com/conundrumer/lr-core
[serde]: https://serde.rs/
[linerider.com]: https://linerider.com/
[lra-ce]: https://github.com/RatherBeLunar/LRA-Community-Edition
[lra]: https://github.com/jealouscloud/linerider-advanced
