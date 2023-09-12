use asm4ahk_lib::ret42;
use asm4ahk_lib::add;

fn main() {
  let lib42:i32 = ret42();
  let libadd:usize = add(1,2);
  println!("Hello, world! lib42{} libadd{}", lib42,libadd);
}
