/* SPDX-License-Identifier: GPL-2.0 */
/* Copyright (c) 2023 Isovalent */

/* C header dependency: <test_progs.h> */

/* C fallback macro:
 * #ifndef loopback
 * # define loopback 1
 * #endif
 */
pub const loopback: i32 = 1;

pub unsafe fn ifindex_from_link_fd(fd: i32) -> __u32 {
    let mut link_info: bpf_link_info = unsafe { core::mem::zeroed() };
    let mut link_info_len: __u32 = core::mem::size_of_val(&link_info) as __u32;
    let err: i32;

    err = unsafe {
        bpf_link_get_info_by_fd(
            fd,
            &mut link_info as *mut bpf_link_info,
            &mut link_info_len as *mut __u32,
        )
    };
    if !unsafe { ASSERT_OK(err, b"id_from_link_fd\0".as_ptr() as *const _) } {
        return 0;
    }

    link_info.tcx.ifindex
}

pub unsafe fn __assert_mprog_count(target: i32, expected: i32, ifindex: i32) {
    let mut count: __u32 = 0;
    let mut attach_flags: __u32 = 0;
    let err: i32;

    err = unsafe {
        bpf_prog_query(
            ifindex,
            target,
            0,
            &mut attach_flags as *mut __u32,
            core::ptr::null_mut(),
            &mut count as *mut __u32,
        )
    };
    unsafe {
        ASSERT_EQ(count, expected, b"count\0".as_ptr() as *const _);
        ASSERT_EQ(err, 0, b"prog_query\0".as_ptr() as *const _);
    }
}

pub unsafe fn assert_mprog_count(target: i32, expected: i32) {
    unsafe {
        __assert_mprog_count(target, expected, loopback);
    }
}

pub unsafe fn assert_mprog_count_ifindex(ifindex: i32, target: i32, expected: i32) {
    unsafe {
        __assert_mprog_count(target, expected, ifindex);
    }
}

pub unsafe fn tc_skel_reset_all_seen(skel: *mut test_tc_link) {
    unsafe {
        core::ptr::write_bytes(
            (*skel).bss as *mut u8,
            0,
            core::mem::size_of_val(&*(*skel).bss),
        );
    }
}
