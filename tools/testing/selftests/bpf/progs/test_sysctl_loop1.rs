// SPDX-License-Identifier: GPL-2.0
// Copyright (c) 2019 Facebook

// C dependencies: <stdint.h>, <string.h>, <linux/stddef.h>, <linux/bpf.h>,
// <bpf/bpf_helpers.h>, "bpf_compiler.h", "bpf_misc.h"

const TCP_MEM_LOOPS: usize = 28; /* because 30 doesn't fit into 512 bytes of stack */
const MAX_ULONG_STR_LEN: usize = 7;
const MAX_VALUE_STR_LEN: usize = TCP_MEM_LOOPS * MAX_ULONG_STR_LEN;

#[repr(C)]
pub struct bpf_sysctl {
    pub write: u32,
}

unsafe extern "C" {
    fn bpf_sysctl_get_name(
        ctx: *mut bpf_sysctl,
        buf: *mut core::ffi::c_char,
        buf_len: usize,
        flags: u64,
    ) -> i32;
    fn bpf_sysctl_get_current_value(
        ctx: *mut bpf_sysctl,
        buf: *mut core::ffi::c_char,
        buf_len: usize,
    ) -> i32;
    fn bpf_strtoul(
        buf: *const core::ffi::c_char,
        buf_len: usize,
        flags: u64,
        res: *mut core::ffi::c_ulong,
    ) -> i32;
}

const TCP_MEM_NAME_BYTES: &[u8; 62] =
    b"net/ipv4/tcp_mem/very_very_very_very_long_pointless_string\0";

#[unsafe(no_mangle)]
pub static tcp_mem_name: [core::ffi::c_char; 62] = [
    b'n' as core::ffi::c_char,
    b'e' as core::ffi::c_char,
    b't' as core::ffi::c_char,
    b'/' as core::ffi::c_char,
    b'i' as core::ffi::c_char,
    b'p' as core::ffi::c_char,
    b'v' as core::ffi::c_char,
    b'4' as core::ffi::c_char,
    b'/' as core::ffi::c_char,
    b't' as core::ffi::c_char,
    b'c' as core::ffi::c_char,
    b'p' as core::ffi::c_char,
    b'_' as core::ffi::c_char,
    b'm' as core::ffi::c_char,
    b'e' as core::ffi::c_char,
    b'm' as core::ffi::c_char,
    b'/' as core::ffi::c_char,
    b'v' as core::ffi::c_char,
    b'e' as core::ffi::c_char,
    b'r' as core::ffi::c_char,
    b'y' as core::ffi::c_char,
    b'_' as core::ffi::c_char,
    b'v' as core::ffi::c_char,
    b'e' as core::ffi::c_char,
    b'r' as core::ffi::c_char,
    b'y' as core::ffi::c_char,
    b'_' as core::ffi::c_char,
    b'v' as core::ffi::c_char,
    b'e' as core::ffi::c_char,
    b'r' as core::ffi::c_char,
    b'y' as core::ffi::c_char,
    b'_' as core::ffi::c_char,
    b'v' as core::ffi::c_char,
    b'e' as core::ffi::c_char,
    b'r' as core::ffi::c_char,
    b'y' as core::ffi::c_char,
    b'_' as core::ffi::c_char,
    b'l' as core::ffi::c_char,
    b'o' as core::ffi::c_char,
    b'n' as core::ffi::c_char,
    b'g' as core::ffi::c_char,
    b'_' as core::ffi::c_char,
    b'p' as core::ffi::c_char,
    b'o' as core::ffi::c_char,
    b'i' as core::ffi::c_char,
    b'n' as core::ffi::c_char,
    b't' as core::ffi::c_char,
    b'l' as core::ffi::c_char,
    b'e' as core::ffi::c_char,
    b's' as core::ffi::c_char,
    b's' as core::ffi::c_char,
    b'_' as core::ffi::c_char,
    b's' as core::ffi::c_char,
    b't' as core::ffi::c_char,
    b'r' as core::ffi::c_char,
    b'i' as core::ffi::c_char,
    b'n' as core::ffi::c_char,
    b'g' as core::ffi::c_char,
    0,
];

#[inline(always)]
unsafe fn is_tcp_mem(ctx: *mut bpf_sysctl) -> i32 {
    let mut i: u8;
    let mut name = [0 as core::ffi::c_char; 62];
    let ret: i32;

    ret = unsafe { bpf_sysctl_get_name(ctx, name.as_mut_ptr(), name.len(), 0) };
    if ret < 0 || ret != (TCP_MEM_NAME_BYTES.len() - 1) as i32 {
        return 0;
    }

    // __pragma_loop_no_unroll
    i = 0;
    while (i as usize) < TCP_MEM_NAME_BYTES.len() {
        if name[i as usize] != tcp_mem_name[i as usize] {
            return 0;
        }
        i = i.wrapping_add(1);
    }

    1
}

#[unsafe(no_mangle)]
#[unsafe(link_section = "cgroup/sysctl")]
pub unsafe extern "C" fn sysctl_tcp_mem(ctx: *mut bpf_sysctl) -> i32 {
    let mut tcp_mem = [0 as core::ffi::c_ulong; TCP_MEM_LOOPS];
    let mut value = [0 as core::ffi::c_char; MAX_VALUE_STR_LEN];
    let mut i: u8;
    let mut off: u8 = 0;
    /* a workaround to prevent compiler from generating
     * codes verifier cannot handle yet.
     */
    let mut ret: i32;

    if unsafe { (*ctx).write } != 0 {
        return 0;
    }

    if unsafe { is_tcp_mem(ctx) } == 0 {
        return 0;
    }

    ret = unsafe { bpf_sysctl_get_current_value(ctx, value.as_mut_ptr(), MAX_VALUE_STR_LEN) };
    if ret < 0 || ret >= MAX_VALUE_STR_LEN as i32 {
        return 0;
    }

    // __pragma_loop_no_unroll
    i = 0;
    while (i as usize) < tcp_mem.len() {
        ret = unsafe {
            bpf_strtoul(
                value.as_mut_ptr().wrapping_add(off as usize),
                MAX_ULONG_STR_LEN,
                0,
                tcp_mem.as_mut_ptr().wrapping_add(i as usize),
            )
        };
        if ret <= 0 || ret > MAX_ULONG_STR_LEN as i32 {
            return 0;
        }
        off = off.wrapping_add((ret & MAX_ULONG_STR_LEN as i32) as u8);
        i = i.wrapping_add(1);
    }

    (tcp_mem[0] < tcp_mem[1] && tcp_mem[1] < tcp_mem[2]) as i32
}

#[unsafe(no_mangle)]
#[unsafe(link_section = "license")]
pub static mut _license: [core::ffi::c_char; 4] = [
    b'G' as core::ffi::c_char,
    b'P' as core::ffi::c_char,
    b'L' as core::ffi::c_char,
    0,
];

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
