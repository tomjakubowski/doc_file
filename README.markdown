# doc_file

[![Build Status][travis-img]][travis]

Move your Rust crate's documentation into external files.

## Why?

This is a proof of concept implementation of [rust-lang/rust#15470][issue].
I hope to iron out questions and discover issues out of tree and then
contribute this back to `rustc` itself.

## Example usage

```rust
#![feature(phase)]

// Paths are relative to the source file.
#![doc_file = "example_crate.markdown"]

#[phase(plugin)] extern crate doc_file;

#[doc_file = "complicated_thing.markdown"]
pub struct ComplicatedThing;
```

## Open issues

The syntax `#[doc_file = "..."]` is a little ugly.  Ideally it would be
something like `#[doc(file = "...")]`.

[travis]: https://travis-ci.org/tomjakubowski/doc_file
[travis-img]: https://travis-ci.org/tomjakubowski/doc_file.svg
[issue]: https://github.com/rust-lang/rust/issues/15470
