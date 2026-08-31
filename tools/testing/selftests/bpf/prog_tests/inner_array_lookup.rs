// SPDX-License-Identifier: GPL-2.0-only

// Translated from C source that included:
// <test_progs.h>
// "inner_array_lookup.skel.h"

use core::ffi::c_void;

#[repr(C)]
pub struct inner_array_lookup {
    pub maps: inner_array_lookup_maps,
}

#[repr(C)]
pub struct inner_array_lookup_maps {
    pub inner_map1: *mut c_void,
}

extern "C" {
    fn inner_array_lookup__open_and_load() -> *mut inner_array_lookup;
    fn inner_array_lookup__attach(skel: *mut inner_array_lookup) -> i32;
    fn inner_array_lookup__destroy(skel: *mut inner_array_lookup);
    fn bpf_map__fd(map: *mut c_void) -> i32;
    fn bpf_map_update_elem(fd: i32, key: *const c_void, value: *const c_void, flags: u64) -> i32;
    fn bpf_map_lookup_elem(fd: i32, key: *const c_void, value: *mut c_void) -> i32;
    fn ASSERT_OK_PTR(ptr: *const c_void, name: *const u8) -> bool;
    fn ASSERT_OK(err: i32, name: *const u8) -> bool;
    fn ASSERT_EQ(actual: i32, expected: i32, name: *const u8);
}

pub unsafe extern "C" fn test_inner_array_lookup() {
    let map1_fd: i32;
    let mut err: i32;
    let mut key: i32 = 3;
    let mut val: i32 = 1;
    let skel: *mut inner_array_lookup;

    skel = inner_array_lookup__open_and_load();
    if !ASSERT_OK_PTR(skel as *const c_void, b"open_load_skeleton\0".as_ptr()) {
        return;
    }

    err = inner_array_lookup__attach(skel);
    if !ASSERT_OK(err, b"skeleton_attach\0".as_ptr()) {
        goto_cleanup(skel);
        return;
    }

    map1_fd = bpf_map__fd((*skel).maps.inner_map1);
    bpf_map_update_elem(
        map1_fd,
        &mut key as *mut i32 as *const c_void,
        &mut val as *mut i32 as *const c_void,
        0,
    );

    /* Probe should have set the element at index 3 to 2 */
    bpf_map_lookup_elem(
        map1_fd,
        &mut key as *mut i32 as *const c_void,
        &mut val as *mut i32 as *mut c_void,
    );
    ASSERT_EQ(val, 2, b"value_is_2\0".as_ptr());

    goto_cleanup(skel);
}

unsafe fn goto_cleanup(skel: *mut inner_array_lookup) {
    inner_array_lookup__destroy(skel);
}
