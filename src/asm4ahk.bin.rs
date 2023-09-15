#![allow(unused_imports)]
use asm4ahk_lib::{add,ret42,add_ext,ret42_ext};

fn main() {
  let libadd:usize = add(1,2);
  let libadd_ext:usize = add_ext(1,2);
  let libret42:i32 = ret42();
  let libret42_ext:i32 = ret42_ext();
  println!("Hello, world! libret42{} libadd{} libadd_ext{} libret42_ext{}"
   ,                      libret42  ,libadd   ,libadd_ext  ,libret42_ext);
}
