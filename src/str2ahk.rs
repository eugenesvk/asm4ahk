#![allow(non_snake_case,non_upper_case_globals,non_camel_case_types,unused_imports,unused_mut,unused_variables,dead_code,unused_assignments,unused_macros)]
// potential alternative to into_raw: https://users.rust-lang.org/t/correct-way-to-implement-a-function-which-returns-a-c-string/315
  // cs.as_ptr()  lifetime of pointer returned from CString ends as soon as cs goes out of scope
// either return a string that requires an explicit deallocation call to return it back to Rust and dealloc there
// OR write string to a passed argument buffer, but then the length of this buffer isn't known in advance, so
  // either do 2 calls, one sets the length based on the result, 2nd writes the result (no better than calling dealloc, also a 2nd call)
    use core::ptr;
    use std::ffi::c_int;
    const ERR_INSUFFICIENT_BUFFER_SIZE:c_int = 1;
    unsafe extern "C" fn twostep_1size_2fill(buffer:*mut c_char, buffer_len:c_int) -> c_int { // Writes string→buffer (caller provided), return # of bytes written
      let some_string = "I'm made in Rust!";
      if buffer.is_null()                                	{ return some_string.len() as c_int; // 1 return the buffer size so the caller knows how much memory to allocate
      } else if (buffer_len as usize) < some_string.len()	{ return ERR_INSUFFICIENT_BUFFER_SIZE; } // or can truncate as mmap function does for errors when it's not important to be wrong
      let mut buffer = std::slice::from_raw_parts_mut(buffer as *mut u8, buffer_len as usize); // 2 otherwise, do the actual copy
      buffer.copy_from_slice(some_string.as_bytes());
      some_string.len() as c_int
    }
  // OR set some known-in-advance max result length
  // OR? ↓ use the same allocator as AHK? to allow AHK not to have to explicitly call the deallocator function
    // use std::alloc::System;
    // #[global_allocator] static GLOBAL: System = System;


/* use in AutoHotkey
test_rust_dll()
test_rust_dll() {
  hModule := DllCall("LoadLibrary", "Str","asm4ahk_lib.dll", "Ptr")  ; Avoids the need for DllCall in the loop to load the library
  testlib_return_s_self      	:= DllCall.Bind("asm4ahk_lib\return_s_self"      	, 'str',unset	, 'str')
  testlib_how_many_characters	:= DllCall.Bind("asm4ahk_lib\how_many_characters"	, 'str',unset	, 'uptr')
  testlib_return_s           	:= DllCall.Bind("asm4ahk_lib\return_s"           	             	, 'str')
  testlib_return_as          	:= DllCall.Bind("asm4ahk_lib\return_as"          	             	, 'Ptr')
  testlib_return_as_ptr      	:= testlib_return_as()
  if testlib_return_as_ptr := testlib_post_message_to_ahk() {
    testlib_return_as_str	:= StrGet(got_ptr_res,,enc:='CP0') ; or use utf-8 for return_s_utf8 or none for U16String
    testlib_dealloc_str_passed_to_ahk(got_ptr_res) ; string is copied ↑, so should be fine deallocating
  } else {
    testlib_return_as_str	:= ''
  }
  ; free from AHK
  testlib_dealloc_str_passed_to_ahk := DllCall.Bind('asm4ahk_lib\dealloc_str_passed_to_ahk', 'Ptr',unset)
  testlib_return_s_osw     	:= DllCall.Bind("asm4ahk_lib\return_s_osw"     	             	, 'str')
  testlib_return_s_modified	:= DllCall.Bind("asm4ahk_lib\return_s_modified"	, 'str',unset	, 'str')
  msgbox(''
   . '`n' testlib_return_s_self("inAHK")      	'`t' 'testlib_return_s_self'
   . '`n' testlib_how_many_characters("inAHK")	'`t' 'testlib_how_many_characters'
   . '`n' testlib_return_s()                  	'`t' 'testlib_return_s'
   . '`n' testlib_return_as_str()             	'`t' 'testlib_return_as_str'
   . '`n' testlib_return_s_osw()              	'`t' 'return_s_osw'
   . '`n' testlib_return_s_modified('inAHK')  	'`t' 'testlib_return_s_modified'
   )
  dbgTT(0,"🖰Scroll Excel 9! ¦" SubStr(got_str_res,1,20) '¦',t:=3)
  DllCall("FreeLibrary", "Ptr",hModule)  ; To conserve memory, the DLL may be unloaded after use
}
*/

use std::ffi::{CString,CStr,c_char,c_short,c_ushort};
use widestring::{
  U16Str,U16String,       	// U16String and U32String, on the other hand, are similar to (but not the same as), OsString, and are designed around working with FFI. Unlike the UTF variants, these strings do not have a defined encoding, and can work with any wide character strings, regardless of the encoding. They can be converted to and from OsString (but may require an encoding conversion depending on the platform), although that string type is an OS-specified encoding, so take special care.
  WideString ,WideChar,   	// alias for u16|u32 to match C wchar_t size (per platform)
  WideCString,WideCStr,   	// aliases U16CString or U32CString
  U16CString ,U16CStr,    	// U16/U32-CString wide version of the standard CString type
  Utf16Str   ,Utf16String,	// UTF-16 encoded, growable owned string
  u16str,u16cstr,utf16str 	// macros
};
// Notes
  // bind CString to a var before calling .as_ptr
    // pointer from as_ptr does not carry any lifetime information and the CString is deallocated immediately after the CString::new("New").expect("x").as_ptr() expression is evaluated

// 1 No input, return generated string
#[no_mangle] pub extern "system"
fn return_s_utf8() -> *const c_char {
  let str_utf8 = "✗123";
  let c_string = CString::new(str_utf8).expect("CString::new failed");
  let ptr_c_string	= c_string.into_raw();
  ptr_c_string
}

#[no_mangle] pub extern "C"
fn return_s() -> *const WideChar { // alias to u16 on Windows
  // let w_str	= WideString::from(u16str! ("WideString::from(u16str !")); //WideString=U16String on Windows
  let w_cstr  	= U16CString ::from(u16cstr! ("U16CString ::from(u16cstr !"));
  let w_str   	= U16String  ::from(u16str!  ("U16String  ::from(u16str  !")); //WideString=U16String on Windows
  let w_16str 	= Utf16String::from(utf16str!("Utf16String::from(utf16str!"));

  // w_cstr.as_ptr() // fails
  // w_str.as_ptr() //works
  w_16str.as_ptr() //works
}
#[no_mangle] pub extern "C"
fn return_s_osw() -> *const u16 {
  use std::os::windows::ffi::OsStrExt;
  use std::ffi::OsStr;
  let os_str_w = OsStr::new("OsStr to be encoded wide, added null, and vectorized")
    .encode_wide()
    .chain(Some(0)) // add NULL termination
    .collect::<Vec<_>>();
  os_str_w.as_ptr()
}
#[no_mangle] pub extern "system"
fn return_as() -> *const c_char {
  let c_string = CString::new("✗123").expect("CString::new failed");
  c_string.as_ptr() // fails, AHK can't get thi string

  // let ptr_c_string = c_string.into_raw(); // works, but leaks memory per https://doc.rust-lang.org/stable/std/ffi/struct.CString.html#method.into_raw
    // testlib_post_message_to_ahk := DllCall.Bind('asm4ahk_lib\post_message_to_ahk', 'Ptr')
    // got_ptr_res := testlib_post_message_to_ahk()
    // got_str_res := StrGet(got_ptr_res,,enc:='CP0')
  // free from AHK
    // testlib_dealloc_str_passed_to_ahk := DllCall.Bind('asm4ahk_lib\dealloc_str_passed_to_ahk', 'Ptr',unset)
    // testlib_dealloc_str_passed_to_ahk(got_ptr_res)
  // ptr_c_string
}
#[no_mangle] pub extern "system"
fn dealloc_str_passed_to_ahk(str_ptr:*mut i8) {unsafe{let _ = CString::from_raw(str_ptr);}}
// # SAFETY Must be called only with a pointer generated by another Rust function via `.into_raw`, the pointer can't be used after this call (the FFI receiver of this pointer can't edit it)

fn ret_error_from_utf8(err_msg:&str, err_sz:u32,err_ptr:*mut WideChar) -> *const WideChar { // create a buffer from pointer/size and fill it in
  let err_err	= U16CString::from(u16cstr!("Error²: Failed to convert error message to U16CString!"));
  let err_msg_cs = match U16CString::from_str(err_msg) {
    Ok(cs) => cs,
    Err(_) => err_err,};
  ret_error(&err_msg_cs,err_sz,err_ptr)
}
fn ret_error(err_msg:&U16CStr, err_sz:u32,err_ptr:*mut WideChar) -> *const WideChar { // create a buffer from pointer/size and fill it in
  let err_msg_bufer   	= unsafe{slice::from_raw_parts_mut::<WideChar>(err_ptr, err_sz as usize)};
  let err_msg_b:&[u16]	= err_msg.as_slice_with_nul(); // converts to a slice of the underlying elements, including the nul terminator.
  let max_buff_len    	= std::cmp::min(err_msg_b.len(),err_sz as usize);
  err_msg_bufer[..max_buff_len].copy_from_slice(&err_msg_b[..max_buff_len]);
  ptr::null()
}

// 2 String input
// 2.1 return self
#[no_mangle] pub extern "C"
fn return_s_self(s: &WideChar) -> *const WideChar {
  let w_str = unsafe { U16CStr::from_ptr_str(s) };
  w_str.as_ptr()
}

// 2.2 return int based on string
#[no_mangle] pub extern "C"
// fn how_many_characters(s: *const c_char) -> u32 { ; works with AHK astr
    // assert!(!s.is_null());
    // U16CStr::from_ptr(s)
    // CStr::from_ptr(s)
  // let r_str = c_str.to_str().expect("Can not read string argument.");
  // r_str.chars().count() as u32 // returns 1
// fn how_many_characters(s: *const u16) -> u32 { // works with AHK wstr
fn how_many_characters(s: &WideChar) -> u32 { // works with ahk wstr
  let w_str = unsafe { U16CStr::from_ptr_str(s) };
  w_str.chars().count() as u32 //assumes the string is UTF-16. Since it may consist of invalid UTF-16, the iterator returned by this method is an iterator over Result<char, DecodeUtf16Error>
  // w_str.chars_lossy().count() as u32
}

// 2.3 return modified string
#[no_mangle] pub extern "C"
fn return_s_modified(s: &WideChar) -> *const WideChar {
  let w_cstr = unsafe { U16CStr::from_ptr_str(s) }; // Constructs a wide C string slice from a nul-terminated string pointer
    // panics if s is null
  let w_str:&U16Str = w_cstr.as_ustr(); //16b wide string slice with undefined encoding. NO NULL-term

  // reject invalid UTF16 (skip check with from_ustr_unchecked if certain input is valid UTF16)
  let w16_str:&Utf16Str = Utf16Str::from_ustr(w_str).expect("Found invalid UTF16 sequences!");
  // let w16_str:&Utf16Str = match Utf16Str::from_ustr(w_str) {
    // Ok(s)  	=> s,
    // Err(_e)	=> return [0u16].as_ptr()
  // };
  // Convert to UTF8
  let mut utf8_str:String = w16_str.to_string(); // since it's valid UTF16, conversion is lossless and non-fallible
  // Append
  let borrowed_string: &str = "¦world¦";
  utf8_str.push_str(borrowed_string);
  let ret_w16cstr = U16CString::from_str(utf8_str.as_str()).expect("Some null lurking inside!");
  // Return // call dealloc from AHK to avoid memory leak!
  let ptr_c_string = ret_w16cstr.into_raw();
  ptr_c_string
}

// can also use constants
const MY_STR: &U16CStr = u16cstr!("A constant, nul-terminated UTF-16 string!");


use std::slice;
#[no_mangle] pub extern "C"
fn write_string_to_arg_buffer(pre:&WideChar,s:&WideChar, err_ptr:*mut u8) -> *const WideChar { // call dealloc from AHK to avoid memory leak!
  let err_none 	= U16CString::from(u16cstr!("123"));
  let err_utf16	= U16CString::from(u16cstr!("failed to convert to C string!"));
  let header_size=2u32;
  let err_msg = "abc";
  let err_msg_cs = match CString::new(err_msg) {
      Ok(cs) => cs,
      Err(_) => return err_utf16.into_raw(),
  };
  let err_msg_b    	= err_msg_cs.as_bytes_with_nul();
  let err_msg_bufer	= unsafe{slice::from_raw_parts_mut(err_ptr, header_size as usize)};
  let max_buff_len 	= std::cmp::min(err_msg_b.len(),header_size as usize);
  err_msg_bufer[..max_buff_len].copy_from_slice(&err_msg_b[..max_buff_len]);
  // ↑ get via err_msg := StrGet(err_buff,,'UTF-8')

  err_none.into_raw()
}

