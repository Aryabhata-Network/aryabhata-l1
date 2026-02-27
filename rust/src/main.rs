#![no_std]
#![forbid(unsafe_code)]
#![deny(warnings)]

extern crate alloc;

mod crypto;
mod ffi;
mod mining;
mod network;
mod node;

use node::Node;

fn main() {
    Node::start();
}
