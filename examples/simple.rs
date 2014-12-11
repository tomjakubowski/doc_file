#![feature(phase)]

#![doc(file = "simple.markdown")]

#[phase(plugin)] extern crate doc_file;

#[doc(file = "complicated_thing.markdown")]
#[deriving(Copy)]
pub struct ComplicatedThing;

/// Document simple things the usual way.
#[deriving(Copy)]
pub struct SimpleThing;

fn main() {
}
