#!/usr/bin/env rust

use rustutils::strings::split;

fn main(){
    for item in split(&"a,b,c",&",") {
        print!("{}\n",item);
    }
}