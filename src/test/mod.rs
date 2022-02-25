#!/usr/bin/env rust

#[allow(unused_macros)]
macro_rules! test {
    ($x:expr) => {
        println!("{:?}" = "{:?}",
                stringify!($x),
                $x);
    };
}