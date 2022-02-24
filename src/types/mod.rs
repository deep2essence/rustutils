#!/usr/bin/env rust

use std::any::{type_name,Any,TypeId};

pub fn type_of<T>(_:T)-> &'static str {
    type_name::<T>()
}

pub fn is_str<T>(param:&T) -> bool {
    type_of(param) == "&str"
}

pub fn is_cstring(param: &dyn Any) -> bool {
    TypeId::of::<String>() == param.type_id()
}

pub fn is_string(param: &dyn Any) -> bool {
    let tid = param.type_id();
    TypeId::of::<String>() == tid || TypeId::of::<&str>() == tid
}

pub fn is_pointer<T>(param:T)-> bool {
    type_of(param).contains("&")
}

pub fn is_func<T>(param:T) ->bool {
    type_of(param).contains("&")
}

pub fn is_slice<T>(param:T) ->bool {
    type_of(param).contains("[")
}

pub fn is_vector<T>(param:T) ->bool {
    type_of(param).contains("alloc::vec::Vec")
}