// SPDX-License-Identifier: GPL-2.0-only
/* Manage a cache of file names' existence */

use std::os::raw::{c_char, c_int, c_long, c_void};

// Dependencies from:
// <pthread.h>, <stdlib.h>, <string.h>, <unistd.h>, "fncache.h", "hashmap.h"

#[repr(C)]
pub struct hashmap {
    _private: [u8; 0],
}

type size_t = usize;
type pthread_once_t = c_int;

const PTHREAD_ONCE_INIT: pthread_once_t = 0;
const R_OK: c_int = 4;

static mut fncache: *mut hashmap = std::ptr::null_mut();

unsafe extern "C" {
    fn str_hash(s: *const c_char) -> size_t;
    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
    fn strdup(s: *const c_char) -> *mut c_char;
    fn free(ptr: *mut c_void);
    fn access(pathname: *const c_char, mode: c_int) -> c_int;
    fn pthread_once(once_control: *mut pthread_once_t, init_routine: extern "C" fn()) -> c_int;

    fn hashmap__new(
        hash: extern "C" fn(c_long, *mut c_void) -> size_t,
        equal: extern "C" fn(c_long, c_long, *mut c_void) -> bool,
        ctx: *mut c_void,
    ) -> *mut hashmap;
    fn hashmap__find(map: *mut hashmap, key: *const c_char, value: *mut c_long) -> bool;
    fn hashmap__set(
        map: *mut hashmap,
        key: *mut c_char,
        value: bool,
        old_key: *mut *mut c_char,
        old_value: *mut c_long,
    );
}

extern "C" fn fncache__hash(key: c_long, _ctx: *mut c_void) -> size_t {
    unsafe { str_hash(key as *const c_char) }
}

extern "C" fn fncache__equal(key1: c_long, key2: c_long, _ctx: *mut c_void) -> bool {
    unsafe { strcmp(key1 as *const c_char, key2 as *const c_char) == 0 }
}

extern "C" fn fncache__init() {
    unsafe {
        fncache = hashmap__new(
            fncache__hash,
            fncache__equal,
            /*ctx=*/ std::ptr::null_mut(),
        );
    }
}

unsafe fn fncache__get() -> *mut hashmap {
    static mut fncache_once: pthread_once_t = PTHREAD_ONCE_INIT;

    unsafe {
        pthread_once(&mut fncache_once, fncache__init);

        fncache
    }
}

unsafe fn lookup_fncache(name: *const c_char, res: *mut bool) -> bool {
    let mut val: c_long = 0;

    unsafe {
        if !hashmap__find(fncache__get(), name, &mut val) {
            return false;
        }

        *res = val != 0;
    }
    true
}

unsafe fn update_fncache(name: *const c_char, res: bool) {
    let mut old_key: *mut c_char = std::ptr::null_mut();
    let key: *mut c_char = unsafe { strdup(name) };

    if !key.is_null() {
        unsafe {
            hashmap__set(
                fncache__get(),
                key,
                res,
                &mut old_key,
                /*old_value*/ std::ptr::null_mut(),
            );
            free(old_key as *mut c_void);
        }
    }
}

/* No LRU, only use when bounded in some other way. */
#[no_mangle]
pub unsafe extern "C" fn file_available(name: *const c_char) -> bool {
    let mut res: bool = false;

    unsafe {
        if lookup_fncache(name, &mut res) {
            return res;
        }
        res = access(name, R_OK) == 0;
        update_fncache(name, res);
    }
    res
}
