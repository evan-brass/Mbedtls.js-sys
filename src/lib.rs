// use wasm_bindgen::prelude::*;
// use std::alloc::{System, GlobalAlloc, Layout};
// use std::mem::MaybeUninit;
// use wasm_bindgen::prelude::*;

#[allow(non_upper_case_globals)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub mod mbedtls {
	include!(concat!(env!("OUT_DIR"), "/mbedtls.rs"));
}

// #[wasm_bindgen]
// extern "C" {
// 	#[wasm_bindgen(js_namespace = console)]
// 	fn log(msg: &str);
// }

#[no_mangle]
pub unsafe extern "C" fn malloc(len: usize) -> *mut u8 {
	let size = (len / std::mem::size_of::<usize>()) + 2;
	let ret = Box::into_raw(vec![0usize; size].into_boxed_slice());
	let ret = (*ret).as_mut_ptr();
	ret.write(size);
	let ret = ret.offset(1) as *mut u8;
	// log(&format!("malloc: {ret:?} ({size})"));
	ret
}
#[no_mangle]
pub unsafe extern "C" fn free(ptr: *mut u8) {
	if ptr.is_null() { return; }

	// log(&format!("free: {ptr:?}"));
	let ptr = ptr as *mut usize;
	let ptr = ptr.offset(-1);
	let size = ptr.read();
	let _ = Box::from_raw(std::slice::from_raw_parts_mut(ptr, size).as_mut_ptr());
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
