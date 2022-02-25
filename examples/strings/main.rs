#!/usr/bin/env rust

extern crate rustutils;
use rustutils::print_result;
use rustutils::strings::split;

fn main(){
    print_result!(split("a,b,c,d",","));
    // print_result!(vec!["a", "b", "c", "d"].merge(","))
}