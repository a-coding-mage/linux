// SPDX-License-Identifier: GPL-2.0
// C source defined _GNU_SOURCE and included "main.h" and <assert.h>.

use std::os::raw::{c_uint, c_void};

/* stub implementation: useful for measuring overhead */
#[no_mangle]
pub extern "C" fn alloc_ring() {}

/* guest side */
#[no_mangle]
pub extern "C" fn add_inbuf(_len: c_uint, _buf: *mut c_void, _datap: *mut c_void) -> i32 {
    0
}

/*
 * skb_array API provides no way for producer to find out whether a given
 * buffer was consumed.  Our tests merely require that a successful get_buf
 * implies that add_inbuf succeed in the past, and that add_inbuf will succeed,
 * fake it accordingly.
 */
#[no_mangle]
pub extern "C" fn get_buf(_lenp: *mut c_uint, _bufp: *mut *mut c_void) -> *mut c_void {
    b"Buffer\0".as_ptr() as *mut c_void
}

#[no_mangle]
pub extern "C" fn used_empty() -> bool {
    false
}

#[no_mangle]
pub extern "C" fn disable_call() {
    assert!(false);
}

#[no_mangle]
pub extern "C" fn enable_call() -> bool {
    assert!(false);
    false
}

#[no_mangle]
pub extern "C" fn kick_available() {
    assert!(false);
}

/* host side */
#[no_mangle]
pub extern "C" fn disable_kick() {
    assert!(false);
}

#[no_mangle]
pub extern "C" fn enable_kick() -> bool {
    assert!(false);
    false
}

#[no_mangle]
pub extern "C" fn avail_empty() -> bool {
    false
}

#[no_mangle]
pub extern "C" fn use_buf(_lenp: *mut c_uint, _bufp: *mut *mut c_void) -> bool {
    true
}

#[no_mangle]
pub extern "C" fn call_used() {
    assert!(false);
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
