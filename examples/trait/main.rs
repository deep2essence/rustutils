#!/usr/bin/env rust

pub fn run_cnt<F,T>(f:F,input:T,cnt:u8) -> T
where
F: Fn(T) -> T
{
    let mut result:T = input;
    for _i in 0..cnt {
        result = f(result);
    }
    result
}

extern crate rustutils;
use rustutils::print_result;

fn main() {
    print_result!(run_cnt(|n|n*2,1,10));
}