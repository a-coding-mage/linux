// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2024 Meta Platforms, Inc. and affiliates. */

// C dependencies:
// #include <test_progs.h>
// #include "struct_ops_multi_pages.skel.h"

#[repr(C)]
pub struct bpf_link {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_map {
    _private: [u8; 0],
}

#[repr(C)]
pub struct struct_ops_multi_pages_maps {
    pub multi_pages: *mut bpf_map,
}

#[repr(C)]
pub struct struct_ops_multi_pages {
    pub maps: struct_ops_multi_pages_maps,
}

unsafe extern "C" {
    fn struct_ops_multi_pages__open_and_load() -> *mut struct_ops_multi_pages;
    fn struct_ops_multi_pages__destroy(skel: *mut struct_ops_multi_pages);
    fn bpf_map__attach_struct_ops(map: *mut bpf_map) -> *mut bpf_link;
    fn bpf_link__destroy(link: *mut bpf_link);
    fn ASSERT_OK_PTR(ptr: *const ::core::ffi::c_void, name: *const ::core::ffi::c_char) -> bool;
    fn test__start_subtest(name: *const ::core::ffi::c_char) -> bool;
}

unsafe fn do_struct_ops_multi_pages() {
    let mut skel: *mut struct_ops_multi_pages;
    let link: *mut bpf_link;

    /* The size of all trampolines of skel->maps.multi_pages should be
     * over 1 page (at least for x86).
     */
    skel = struct_ops_multi_pages__open_and_load();
    if !ASSERT_OK_PTR(
        skel as *const ::core::ffi::c_void,
        c"struct_ops_multi_pages_open_and_load".as_ptr(),
    ) {
        return;
    }

    link = bpf_map__attach_struct_ops((*skel).maps.multi_pages);
    ASSERT_OK_PTR(
        link as *const ::core::ffi::c_void,
        c"attach_multi_pages".as_ptr(),
    );

    bpf_link__destroy(link);
    struct_ops_multi_pages__destroy(skel);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn test_struct_ops_multi_pages() {
    if test__start_subtest(c"multi_pages".as_ptr()) {
        do_struct_ops_multi_pages();
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
