use std::alloc::{System, GlobalAlloc, Layout};
// use std::mem::MaybeUninit;
// use wasm_bindgen::prelude::*;

#[allow(non_upper_case_globals)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub mod mbedtls {
	include!(concat!(env!("OUT_DIR"), "/mbedtls.rs"));
}

#[no_mangle]
pub unsafe extern "C" fn malloc(len: usize) -> *mut usize {
	let size = len + std::mem::size_of::<usize>();
	let ret = System.alloc(Layout::from_size_align(size, 1).unwrap()) as *mut usize;
	ret.write(size);
	ret.offset(1)
}
#[no_mangle]
pub unsafe extern "C" fn free(ptr: *mut usize) {
	let ptr = ptr.offset(-1);
	let size = ptr.read();
	System.dealloc(ptr as *mut u8, Layout::from_size_align(size, 1).unwrap());
}
#[no_mangle]
pub extern "C" fn time(dest: *mut i64) -> i64 {
	let now = js_sys::Date::now();
	let secs = (now / 1000.0) as i64;
	if !dest.is_null() {
		unsafe { dest.write(secs); }
	}
	secs
}
#[no_mangle]
pub unsafe extern "C" fn f_rng(_: *mut std::ffi::c_void, ptr: *mut std::ffi::c_uchar, len: usize) -> std::ffi::c_int {
	let buf = unsafe { std::slice::from_raw_parts_mut(ptr, len) };
	web_sys::window().unwrap()
		.crypto().unwrap()
		.get_random_values_with_u8_array(buf).unwrap();
	0
}
