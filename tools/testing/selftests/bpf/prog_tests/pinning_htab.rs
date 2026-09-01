// SPDX-License-Identifier: GPL-2.0

// C dependencies:
// #include <test_progs.h>
// #include "test_pinning_htab.skel.h"

use core::ffi::{c_char, c_int, c_void};

#[repr(C)]
pub struct bpf_map {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_object {
    _private: [u8; 0],
}

#[repr(C)]
pub struct test_pinning_htab {
    pub obj: *mut bpf_object,
}

extern "C" {
    fn test_pinning_htab__open_and_load() -> *mut test_pinning_htab;
    fn test_pinning_htab__destroy(skel: *mut test_pinning_htab);

    fn bpf_object__find_map_by_name(
        obj: *mut bpf_object,
        name: *const c_char,
    ) -> *mut bpf_map;
    fn bpf_map__pin(map: *mut bpf_map, path: *const c_char) -> c_int;
    fn bpf_map__unpin(map: *mut bpf_map, path: *const c_char) -> c_int;

    fn test__start_subtest(name: *const c_char) -> bool;

    fn ASSERT_OK_PTR(ptr: *mut c_void, name: *const c_char) -> bool;
    fn ASSERT_OK(err: c_int, name: *const c_char) -> bool;
}

unsafe fn unpin_map(map_name: *const c_char, pin_path: *const c_char) {
    let skel: *mut test_pinning_htab;
    let map: *mut bpf_map;
    let mut err: c_int;

    skel = test_pinning_htab__open_and_load();
    if !ASSERT_OK_PTR(
        skel as *mut c_void,
        b"skel open_and_load\0".as_ptr() as *const c_char,
    ) {
        return;
    }

    map = bpf_object__find_map_by_name((*skel).obj, map_name);
    if !ASSERT_OK_PTR(
        map as *mut c_void,
        b"bpf_object__find_map_by_name\0".as_ptr() as *const c_char,
    ) {
        test_pinning_htab__destroy(skel);
        return;
    }

    err = bpf_map__pin(map, pin_path);
    if !ASSERT_OK(err, b"bpf_map__pin\0".as_ptr() as *const c_char) {
        test_pinning_htab__destroy(skel);
        return;
    }

    err = bpf_map__unpin(map, pin_path);
    ASSERT_OK(err, b"bpf_map__unpin\0".as_ptr() as *const c_char);

    test_pinning_htab__destroy(skel);
}

#[no_mangle]
pub unsafe extern "C" fn test_pinning_htab() {
    if test__start_subtest(b"timer_prealloc\0".as_ptr() as *const c_char) {
        unpin_map(
            b"timer_prealloc\0".as_ptr() as *const c_char,
            b"/sys/fs/bpf/timer_prealloc\0".as_ptr() as *const c_char,
        );
    }
    if test__start_subtest(b"timer_no_prealloc\0".as_ptr() as *const c_char) {
        unpin_map(
            b"timer_no_prealloc\0".as_ptr() as *const c_char,
            b"/sys/fs/bpf/timer_no_prealloc\0".as_ptr() as *const c_char,
        );
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
