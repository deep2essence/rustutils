#!/usr/bin/env rust

pub fn string2list<'a>(string:&'a str,sep:&'a str)-> Vec<&'a str>{
    let items:Vec<&'a str>=string.split(sep).collect();
    items
}

pub fn list2string<'a>(items: Vec<&'a str>,sep:&'a str)-> String{
    let output = items.join(sep);
    output
}