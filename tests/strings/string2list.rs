#!/usr/bin/env rust

use rustutils::strings::{join,joinEx,split,splitEx};

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
    let bad_data = [1,2,3];
    assert_eq!(split(&joined,&sep)==splited,true);
    assert_eq!(split(&joined_string,&sep)==splited_string,true);
    assert_eq!(splitEx(&joined,&sep)==splited_string,true);
    assert_eq!(splitEx(&joined_string,&sep)==splited_string,true);
    assert_eq!(join(&splited,&sep),joined);
    assert_eq!(join(&splited_string,&String::from(sep)),joined);
    assert_eq!(joinEx(&splited,&sep),joined_string);
    assert_eq!(joinEx(&splited,&0),"");
    assert_eq!(joinEx(&splited_string,&sep),joined);
    assert_eq!(joinEx(&bad_data,&sep),"");
}