#!/usr/bin/env rust

use rustutils::strings::{merge,split};

#[test]
fn test(){
    let joined="A,B,C";
    let joined_string=String::from(joined);
    let sep=",";
    let splited=vec!["A","B","C"];
    let splited_string:Vec<String>=splited.clone()
    .into_iter()
    .map(|i|{
        let item=i.to_string();
        item.clone()
    })
    .collect();
    assert_eq!(split(joined,sep)==splited,true);
    assert_eq!(split(joined_string,sep)==splited_string,true);
    assert_eq!(merge(splited,sep),joined);
    assert_eq!(merge(splited_string,sep),joined);
}