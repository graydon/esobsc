#![feature(never_type)]

#![feature(plugin)]
#![plugin(oak)]

#[macro_use] extern crate lazy_static;
extern crate oak_runtime;
extern crate regex;

mod ast;
mod parser;
mod eval;

use std::io::{Read, stdin};
use std::env::args;
use std::fs::File;

fn main() {
    let mut code = String::new();
    match args().nth(1) {
        Some(fname) => {
            File::open(fname).unwrap()
                .read_to_string(&mut code).unwrap();
        },
        None => {
            stdin().read_to_string(&mut code).unwrap();
        },
    };
    let syntax = parser::parse(code).unwrap();
    let ast = ast::Arited::from_expression(syntax);
    eval::Machine::new().execute_program(&ast).unwrap();
}
