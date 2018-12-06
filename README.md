# Esr _(Pronounced "esser")_

[![Build Status](https://travis-ci.com/tzvipm/esr.svg?branch=master)](https://travis-ci.com/tzvipm/esr)
[![Crates.io](https://img.shields.io/crates/v/esr.svg)](https://crates.io/crates/esr)

**Esr** is a high performance toolchain for ECMAScript-style languages with a Rust core. Its goal is to make static analysis and compilation of ECMAScript-style languages more snappy.

This repo is split in two separate folders:

- `core` contains the main Rust codebase that does all the heavy lifting.
- `ffi` contains the Node.js wrapper around the Rust core with [Neon](http://neon.rustbridge.io/) bindings.

## License

This code is distributed under the terms the MIT license.

See [LICENSE.md](LICENSE.md) for details.
