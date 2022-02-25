#!/usr/bin/env rust

extern crate print;

#[macro_export]
macro_rules! print_result {
    ($x:expr) => {
        println!("{:?} = {:?}",
                stringify!($x),
                $x);
    };
}

#[macro_export]
macro_rules! print_simple {
    ($x:expr) => {
        println!("{:?}",$x);
    };
}

// #[allow(unused_macros)]
#[macro_export]
macro_rules! print_comp {
    ($x:expr) => {
        print!("{:?}",$x);
    };
    ($x:expr,$($y:expr),+) => {
        print!("{:?},",$x);
        print_comp!($($y),+);
    };
}