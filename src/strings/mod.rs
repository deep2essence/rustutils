#!/usr/bin/env rust
#![allow(non_snake_case)]
#![allow(dead_code)]

// returning String is okay
// returning &str is unstable
// evolutions
// 1. seperated functions for both String and &str
// 2. use AsRef & T, Any to accept both String and &str
// 3. use trait & impl to insert function to String and &str respectively. 
// 4. pub impl is not supported, trait->impl->fn: fn can be pubed.
pub trait SplitString {
    fn splited(&self,seq:&str) -> Vec<String>;
}

impl SplitString for String {
    fn splited(&self,seq:&str) -> Vec<String> {
        let mut output = Vec::new();
        let items:Vec<&str>=self.as_str().split(seq).collect();
        for item in items{
            output.push(String::from(item))
        }
        return output        
    }
}

impl SplitString for &str {
    fn splited(&self,seq:&str) -> Vec<String> {
        let mut output = Vec::new();
        let items:Vec<&str>=self.split(seq).collect();
        for item in items{
            output.push(String::from(item))
        }
        return output        
    }
}

impl SplitString for &&str {
    fn splited(&self,seq:&str) -> Vec<String> {
        let mut output = Vec::new();
        let items:Vec<&str>=self.split(seq).collect();
        for item in items{
            output.push(String::from(item))
        }
        return output        
    }
}

pub fn split<T:SplitString>(line:T,seq:&str) -> Vec<String> 
{
    line.splited(seq)
}

pub trait MergeString {
    fn merge(&self,seq:&str) -> String;
}

impl MergeString for Vec<String> {
    fn merge(&self,seq:&str) -> String {
        let output = self.join(seq);
        output     
    }    
}

impl MergeString for Vec<&str> {
    fn merge(&self,seq:&str) -> String {
        let output = self.join(seq);
        output     
    }    
}

pub fn merge<T:MergeString>(line:T,seq:&str) -> String
{
    line.merge(seq)
}