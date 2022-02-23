#!/usr/bin/env rust

pub fn string2list<'a>(string:&'a str,sep:&'a str)-> Vec<&'a str>{
    let items:Vec<&'a str>=string.split(sep).collect();
    items
}
