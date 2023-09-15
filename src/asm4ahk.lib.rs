#![allow(unused_imports,unused_variables,unreachable_code,dead_code,non_upper_case_globals)]
// pub mod bar;
pub mod str2ahk;
pub mod msg2ahk;

// +++ can find
pub fn ret42() -> i32 { 42 }
pub fn add(left: usize, right: usize) -> usize { left + right }
  // asm4ahk_lib.s snippet
  //	.def 	_ZN11asm4ahk_lib3add17hd06d888a8d7bfb4bE;
  //	.scl 	2;
  //	.type	32;
  //	.endef
  //	.section	.text,"xr",one_only,_ZN11asm4ahk_lib3add17hd06d888a8d7bfb4bE
  //	.globl  	_ZN11asm4ahk_lib3add17hd06d888a8d7bfb4bE
  // _ZN11asm4ahk_lib3add17hd06d888a8d7bfb4bE:
  // .Lfunc_begin0:
  //	.cv_func_id 0
  //	.cv_file	1 "asm4ahk.lib.rs" "C725B2F45B50BDC3E14237D7D99E8F6172807E50" 2
  //	.cv_loc 	0 1 9 0
  //	lea     	rax, [rcx + rdx]
  //	.cv_loc 	0 1 10 0
  //	ret
  // .Ltmp0:
  // .Lfunc_end0:

// --- can NOT find
#[no_mangle] pub extern "C" fn add_ext(left: usize, right: usize) -> usize { left + right }
  // asm4ahk_lib.s snippet
  //	.def 	add;
  //	.scl 	2;
  //	.type	32;
  //	.endef
  //	.section	.text,"xr",one_only,add
  //	.globl  	add
  // add:
  // .Lfunc_begin0:
  //	.cv_func_id 0
  //	.cv_file	1 "asm4ahk.lib.rs" "F66D68E3D835A5B53EEBE372406934D98C0ED0B4" 2
  //	.cv_loc 	0 1 5 0
  //	lea     	rax, [rcx + rdx]
  //	.cv_loc 	0 1 6 0
  //	ret
  // .Ltmp0:
  // .Lfunc_end0:

#[no_mangle] pub extern "C" fn ret42_ext() -> i32 { 42 }
