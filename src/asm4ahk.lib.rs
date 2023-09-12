pub mod bar;
#[no_mangle] pub extern "C"
fn add(left: usize, right: usize) -> usize {
  left + right
}

pub fn ret42() -> i32 {
  42
}
