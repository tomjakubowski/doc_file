#![feature(phase)]
#![doc = "foo" ]

#[phase(plugin)] extern crate doc_file;

/// Ordinary documentation
pub fn foo() {
}
