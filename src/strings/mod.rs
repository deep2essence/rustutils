#!/usr/bin/env rust
#![allow(non_snake_case)]
#![allow(dead_code)]

// returning String is okay
// returning &str is err

// pub fn string2list<'a>(string:&'a str,sep:&'a str)-> Vec<&'a str>{
//    let items:Vec<&'a str>=string.split(sep).collect();
//     items
// }

// pub fn string2listex(line:String,sep:String) -> Vec<String> {
//     let mut output = Vec::new();
//     let items:Vec<&str>=line.as_str().split(sep.as_str()).collect();
//     for item in items{
//         output.push(String::from(item))
//     }
//     return output
// }

// pub fn list2string<'a>(items: Vec<&'a str>,sep:&'a str)-> String{
//     let output = items.join(sep);
//     output
// }

// pub fn list2stringex(items: Vec<String>,sep:String)-> String{
//     let output = items.join(sep.as_str());
//     output
// }

pub fn join<T: AsRef<str>>(items: &[T], sep:&str) -> String {
    let mut output = String::new();
    for item in items {
        output.push_str(item.as_ref());
        output.push_str(sep);
    }
    String::from(&output[0..output.len()-sep.len()])
}

use std::any::Any;
use crate::types::is_string;

pub fn joinEx(a:&dyn Any,b:&dyn Any) -> String {
    let output = String::new();
    let seq:String;
    if !is_string(b) {
        return output
    }
 
    if let Some(v) = b.downcast_ref::<&str>() {
        seq = String::from(*v);
    } else if let Some(v) = b.downcast_ref::<String>() {
        seq = v.clone();
    } else {
        return output
    }
    if let Some(v) = a.downcast_ref::<Vec<&str>>() {
        return join(v,seq.as_str());
    } else if let Some(v) = a.downcast_ref::<Vec<String>>() 
    {
        return join(v,seq.as_str());
    } else {
        return output;
    }
}

pub fn split<'a, T: AsRef<str>>(line: &'a T, sep:&'a str) -> Vec<&'a str> {
    let items:Vec<&str>=line.as_ref().split(sep).collect();
    items 
}

pub fn splitEx<'a, T: AsRef<str>>(a: &'a T, b:&dyn Any) -> Vec<String> {
    let mut output = Vec::new();
    let seq:String;
    if let Some(v) = b.downcast_ref::<&str>() {
        seq = String::from(*v);
    } else if let Some(v) = b.downcast_ref::<String>() {
        seq = v.clone();
    } else {
        return output
    }
    let items:Vec<&str>=a.as_ref().split(seq.as_str()).collect();
    for item in items{
        output.push(String::from(item));
    }
    return output
}