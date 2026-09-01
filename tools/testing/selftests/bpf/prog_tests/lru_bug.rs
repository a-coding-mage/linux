// SPDX-License-Identifier: GPL-2.0
// C dependencies: <test_progs.h> and "lru_bug.skel.h".

use std::os::raw::{c_char, c_int, c_uint, c_void};

#[repr(C)]
pub struct lru_bug {
    pub data: *mut lru_bug__data,
}

#[repr(C)]
pub struct lru_bug__data {
    pub result: c_int,
}

unsafe extern "C" {
    fn lru_bug__open_and_load() -> *mut lru_bug;
    fn lru_bug__attach(skel: *mut lru_bug) -> c_int;
    fn lru_bug__destroy(skel: *mut lru_bug);
    fn usleep(usec: c_uint) -> c_int;

    fn ASSERT_OK_PTR(ptr: *const c_void, name: *const c_char) -> bool;
    fn ASSERT_OK(res: c_int, name: *const c_char) -> bool;
}

pub unsafe fn test_lru_bug() {
    let skel: *mut lru_bug;
    let mut ret: c_int;

    skel = unsafe { lru_bug__open_and_load() };
    if !unsafe {
        ASSERT_OK_PTR(
            skel as *const c_void,
            b"lru_bug__open_and_load\0".as_ptr() as *const c_char,
        )
    } {
        return;
    }
    ret = unsafe { lru_bug__attach(skel) };
    if !unsafe { ASSERT_OK(ret, b"lru_bug__attach\0".as_ptr() as *const c_char) } {
        unsafe { lru_bug__destroy(skel) };
        return;
    }
    unsafe {
        usleep(1);
    }
    unsafe {
        ASSERT_OK(
            (*(*skel).data).result,
            b"prealloc_lru_pop doesn't call check_and_init_map_value\0".as_ptr() as *const c_char,
        );
    }
    unsafe {
        lru_bug__destroy(skel);
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
