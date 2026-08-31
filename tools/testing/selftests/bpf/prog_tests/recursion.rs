// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2021 Facebook */
// C dependencies: <test_progs.h>, "recursion.skel.h"

pub unsafe fn test_recursion() {
    let mut prog_info: bpf_prog_info = unsafe { core::mem::zeroed() };
    let mut prog_info_len: __u32 = core::mem::size_of::<bpf_prog_info>() as __u32;
    let mut skel: *mut recursion;
    let mut key: i32 = 0;
    let mut err: i32;

    skel = unsafe { recursion__open_and_load() };
    if !unsafe {
        ASSERT_OK_PTR(
            skel as *const core::ffi::c_void,
            b"skel_open_and_load\0".as_ptr() as *const core::ffi::c_char,
        )
    } {
        return;
    }

    err = unsafe { recursion__attach(skel) };
    if !unsafe { ASSERT_OK(err, b"skel_attach\0".as_ptr() as *const core::ffi::c_char) } {
        unsafe { recursion__destroy(skel) };
        return;
    }

    unsafe {
        ASSERT_EQ(
            (*(*skel).bss).pass1,
            0,
            b"pass1 == 0\0".as_ptr() as *const core::ffi::c_char,
        );
        bpf_map_delete_elem(bpf_map__fd((*skel).maps.hash1), &mut key as *mut _ as *const core::ffi::c_void);
        ASSERT_EQ(
            (*(*skel).bss).pass1,
            1,
            b"pass1 == 1\0".as_ptr() as *const core::ffi::c_char,
        );
        bpf_map_delete_elem(bpf_map__fd((*skel).maps.hash1), &mut key as *mut _ as *const core::ffi::c_void);
        ASSERT_EQ(
            (*(*skel).bss).pass1,
            2,
            b"pass1 == 2\0".as_ptr() as *const core::ffi::c_char,
        );

        ASSERT_EQ(
            (*(*skel).bss).pass2,
            0,
            b"pass2 == 0\0".as_ptr() as *const core::ffi::c_char,
        );
        bpf_map_delete_elem(bpf_map__fd((*skel).maps.hash2), &mut key as *mut _ as *const core::ffi::c_void);
        ASSERT_EQ(
            (*(*skel).bss).pass2,
            1,
            b"pass2 == 1\0".as_ptr() as *const core::ffi::c_char,
        );
        bpf_map_delete_elem(bpf_map__fd((*skel).maps.hash2), &mut key as *mut _ as *const core::ffi::c_void);
        ASSERT_EQ(
            (*(*skel).bss).pass2,
            2,
            b"pass2 == 2\0".as_ptr() as *const core::ffi::c_char,
        );

        err = bpf_prog_get_info_by_fd(
            bpf_program__fd((*skel).progs.on_delete),
            &mut prog_info as *mut bpf_prog_info,
            &mut prog_info_len as *mut __u32,
        );
        if !ASSERT_OK(err, b"get_prog_info\0".as_ptr() as *const core::ffi::c_char) {
            recursion__destroy(skel);
            return;
        }
        ASSERT_EQ(
            prog_info.recursion_misses,
            2,
            b"recursion_misses\0".as_ptr() as *const core::ffi::c_char,
        );

        recursion__destroy(skel);
    }
}
