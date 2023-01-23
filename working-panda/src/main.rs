use std::{env, path::Path};

use crate::podman::Podman;
mod podman;

fn main() {
    println!("Starting panda...");
    let (path, tag) = obtain_info_from_args(&env::args().collect()).expect("Failed to parse argument");
    let mut podman = Podman::new("harbor.dst.tk-inline.net/s227564");
    println!("Building image...");
    match podman.build(&path, &tag) {
        Err(e) => panic!("{:?}", e),
        Ok(o) => println!("{}", String::from_utf8(o.stdout).unwrap())
    }
    match podman.tag(&tag) {
        Err(e) => panic!("{:?}", e),
        Ok(o) => println!("{}", String::from_utf8(o.stdout).unwrap())
    }
    match podman.push(&tag) {
        Err(e) => panic!("{:?}", e),
        Ok(o) => println!("{}", String::from_utf8(o.stdout).unwrap())
    }
}

fn obtain_info_from_args(args: &Vec<String>) -> Result<(Box<Path>, String), &str> {
    if args.len() != 2 {
        return Err("Please provide a path as its only argument");
    }
    let path = Path::new(args.last().unwrap());
    if !path.is_file() {
        return Err("Path does not exist");
    }

    let tag = path.file_name().unwrap().to_str().unwrap().split('.').last();

    match tag {
        None => return Err("Failed to extract tag from file name"),
        Some(t) => return Ok((path.into(), t.into())),
    }
}
