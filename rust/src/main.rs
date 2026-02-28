#![forbid(unsafe_code)]
#![deny(warnings)]

mod crypto;
mod ffi;
mod mining;
mod network;
mod node;

fn main() {
    node::Node::start();
}
