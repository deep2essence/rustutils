#!/usr/bin/env rust
// #![deny(clippy::all, missing_docs, unsafe_code)]
// #![allow(clippy::upper_case_acronyms, clippy::from_over_into, clippy::match_like_matches_macro)]

// #[cfg(test)]
// mod tests {
//     #[test]
//     fn it_works() {
//         assert_eq!(2 + 2, 4);
//     }
// }

pub mod strings;
pub mod os;
pub mod types;

// #[cfg(feature = "spin_no_std")]
// #[path=print/print.rs]
// #[doc(hidden)]
// pub mod print;

// #[macro_export(local_inner_macros)]
// with this tag, you can use default macros in your macro body.
// i.e, you can't use println! & stringify!

#[macro_export]
macro_rules! print_result {
    ($x:expr) => {
        println!("{} = {:?}",
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
        print!("{:?}\n",$x);
    };
    ($x:expr,$($y:expr),+) => {
        print!("{:?},",$x);
        print_comp!($($y),+);
    };
}

#[macro_export]
macro_rules! print_compx {
    ($($arg:expr),+) => {
        let mut s = String::new();
        $(
            s += String::from($arg).as_str();
            s += ",";
        )*
        s.remove(s.len()-1);
        print_simple!(s);
    };
}

#[macro_export]
macro_rules! print_compt {
    ($($arg:expr),+) => {
        // print_compx!($($arg),*);
        print_compx!($($arg),+);
    };
}
