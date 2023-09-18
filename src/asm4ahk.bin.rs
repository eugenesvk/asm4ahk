#![allow(unused_imports,unused_variables,unreachable_code,dead_code,non_upper_case_globals,unused_mut)]
extern crate helper;
use helper        	::*; // gets macros
use helper::alias 	::*;
use helper::helper	::*;
use helper::parser	::*;

use asm4ahk_lib::{add,ret42,add_ext,ret42_ext};

fn main() {
  let libadd:usize = add(1,2);
  let libadd_ext:usize = add_ext(1,2);
  let libret42:i32 = ret42();
  let libret42_ext:i32 = ret42_ext();
}
