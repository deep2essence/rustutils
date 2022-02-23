#!/usr/bin/env rust

use rustutils::strings::string2list;

#[test]
fn test(){
    let string="A,B,C";
    let sep=",";
    let expected=vec!["A","B","C"];
    assert_eq!(expected,string2list(string,sep));
}