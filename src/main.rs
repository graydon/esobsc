#![feature(plugin)]
#![plugin(oak)]

// Will be removed when VS Code stops annoying me for no reason
#![allow(dead_code)]

extern crate oak_runtime;

mod ast;
mod parser;

fn main() {
    println!("Hello, world!");
}
