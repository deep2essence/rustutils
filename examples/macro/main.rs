#!/usr/bin/env rust

extern crate rustutils;
use rustutils::{print_result,print_simple,print_comp};

fn main() {
    print_result!(1+2+3);
    print_comp!(1,2,3);
    print_simple!(1);
}