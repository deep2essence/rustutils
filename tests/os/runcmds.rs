#!/usr/bin/env rust

use rustutils::os::runcmds;
use rustutils::os::list2file;

#[test]
fn test(){
    let filepath = "os.test";
    let input = vec!["abc","123"];
    let expected = vec!["abc","123"];
    list2file(filepath,input);
    let output = runcmds(format!("cat {}",filepath).as_str());
    assert_eq!(expected.join("\n")+"\n",output[0]);
    assert_eq!(format!("{}\n",expected.join("\n")),output[0]);
}