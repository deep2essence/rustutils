#!/usr/bin/env rust

// let f = |n|n*2; 
// run_cnt(f,1,10) -> 1*2^10
// run_cnt(f,1,3) == f(f(f(1))) 
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