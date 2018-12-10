# Esr _(Pronounced "esser")_

[![Build Status](https://travis-ci.com/tzvipm/esr.svg?branch=master)](https://travis-ci.com/tzvipm/esr)
[![Crates.io](https://img.shields.io/crates/v/esr.svg)](https://crates.io/crates/esr)

**Esr** is a high performance toolchain for ECMAScript-style languages with a Rust core. Its goal is to make static analysis and compilation of ECMAScript-style languages more snappy.

This repo is managed as a virtual workspace:

- `esr` contains the main Rust codebase that does all the heavy lifting.
- `esr-codegen`, `esr-transformer`, and `esr-visitor` contain tools for code generation, transformation, and AST traversal.
- `esr-wasm` contains code for wasm compilation of Esr.
- `ffi` contains the Node.js wrapper around the Rust core with [Neon](http://neon.rustbridge.io/) bindings.

## Features

### Lexer

- Emits generic symbol names (ex. `QuestionMark` instead of `ConditionalOperator`) to facilitate abstraction of new language features (ex. optional chaining).

- Supports syntax for tc39 proposals:

  - Decorators
  - Optional Chaining
  - Private methods and fields (including static)
  - Numeric separators

  **Note** the stage 1 proposal for numeric extensions presents an ambiguity regarding numeric separators
  Until a solution is presented to the tc39 committee in May 2019, we have elected not to support numeric
  extensions at this time.

- Only joins symbols that are guaranteed to go together, for example:

  - no `>=` symbol, because of ambiguity with type lists such as `const a:SomeInterface<TypeArg>=value;`
  - no `>>=` or `>>>=` symbols, because of ambiguity with embedded type lists

  **Note** for consistency, we have omitted both the `<=` and `>=` symbols, all bitshift operators, and all compound assignment operators from our lexicon. These must be handled by parsers as multiple tokens.

## Performance

Here are the latest benchmarks using `Esr`'s `ffi` library:

```
  esr x 33,615 ops/sec ±0.94% (91 runs sampled)
  acorn x 23,719 ops/sec ±0.67% (90 runs sampled)
  babel x 12,764 ops/sec ±2.93% (86 runs sampled)
  esformatter-parser x 733 ops/sec ±4.61% (79 runs sampled)
  espree x 16,456 ops/sec ±1.86% (91 runs sampled)
  esprima x 27,239 ops/sec ±2.15% (84 runs sampled)
  flow x 1,107 ops/sec ±3.05% (82 runs sampled)
  recast x 2,458 ops/sec ±1.38% (86 runs sampled)
  typescript x 8,304 ops/sec ±1.55% (86 runs sampled)
```

Note that these are not performance guarantees; rather, simply a benchmark from my development device (MacBook Pro, 15 inch, 2017).

## License

This code is distributed under the terms the MIT license.

See [LICENSE.md](LICENSE.md) for details.
