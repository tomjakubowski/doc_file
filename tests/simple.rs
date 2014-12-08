#![feature(phase)]

#![doc_file = "simple.markdown"]

#[phase(plugin)] extern crate doc_file;

#[doc_file = "complicated_thing.markdown"]
pub struct ComplicatedThing;
