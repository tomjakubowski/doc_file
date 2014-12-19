#![feature(phase)]
#![doc_file="essay.md"]

#[phase(plugin)] extern crate doc_file;

/// Ordinary documentation
pub fn foo() {
}
