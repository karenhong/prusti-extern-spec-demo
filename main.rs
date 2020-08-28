#![allow(unused_imports)]
#![allow(unused_attributes)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_must_use)]
#![feature(type_ascription)]

#![feature(register_tool)]
#![register_tool(prusti)]

extern crate prusti_contracts;
use prusti_contracts::*;

mod extern_specs;

mod simple;
mod insert;

fn main() {}
