use rustutils::os::runcmds;

fn main() {
    println!("Hello, world!");
    // let items:Vec<&str> = string2list("Hello, world!"," ");
    for item in runcmds("cat Cargo.toml") {
        println!("{}",item)
    }
}