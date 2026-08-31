// SPDX-License-Identifier: GPL-2.0
// Copyright (c) 2019 Facebook

// C dependencies translated as external Rust declarations:
// <stdint.h>, <string.h>, <linux/stddef.h>, <linux/bpf.h>,
// <bpf/bpf_helpers.h>, "bpf_compiler.h", "bpf_misc.h".

/* Max supported length of a string with unsigned long in base 10 (pow2 - 1). */
const MAX_ULONG_STR_LEN: usize = 0xF;

/* Max supported length of sysctl value string (pow2). */
const MAX_VALUE_STR_LEN: usize = 0x40;

#[repr(C)]
pub struct bpf_sysctl {
    pub write: u32,
    pub file_pos: u32,
}

unsafe extern "C" {
    fn bpf_sysctl_get_name(
        ctx: *mut bpf_sysctl,
        buf: *mut core::ffi::c_char,
        buf_len: u32,
        flags: u64,
    ) -> i32;
    fn bpf_sysctl_get_current_value(
        ctx: *mut bpf_sysctl,
        buf: *mut core::ffi::c_char,
        buf_len: u32,
    ) -> i32;
    fn bpf_strtoul(
        buf: *const core::ffi::c_char,
        buf_len: u64,
        flags: u64,
        res: *mut core::ffi::c_ulong,
    ) -> i32;
}

const TCP_MEM_NAME_LEN: usize = 17;

#[unsafe(no_mangle)]
pub static tcp_mem_name: [core::ffi::c_char; TCP_MEM_NAME_LEN] = [
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
    0,
];

#[inline(always)]
unsafe fn is_tcp_mem(ctx: *mut bpf_sysctl) -> i32 {
    let mut i: u8;
    let mut name: [core::ffi::c_char; TCP_MEM_NAME_LEN] = [0; TCP_MEM_NAME_LEN];
    let ret: i32;

    ret = unsafe {
        bpf_sysctl_get_name(
            ctx,
            name.as_mut_ptr(),
            core::mem::size_of_val(&name) as u32,
            0,
        )
    };
    if ret < 0 || ret != core::mem::size_of_val(&tcp_mem_name) as i32 - 1 {
        return 0;
    }

    // __pragma_loop_unroll_full
    i = 0;
    while (i as usize) < core::mem::size_of_val(&tcp_mem_name) {
        if name[i as usize] != tcp_mem_name[i as usize] {
            return 0;
        }
        i = i.wrapping_add(1);
    }

    1
}

// SEC("cgroup/sysctl")
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sysctl_tcp_mem(ctx: *mut bpf_sysctl) -> i32 {
    let mut tcp_mem: [core::ffi::c_ulong; 3] = [0, 0, 0];
    let mut value: [core::ffi::c_char; MAX_VALUE_STR_LEN] = [0; MAX_VALUE_STR_LEN];
    let mut i: u8;
    let mut off: u8 = 0;
    let mut ret: i32;

    if unsafe { (*ctx).write } != 0 {
        return 0;
    }

    if unsafe { is_tcp_mem(ctx) } == 0 {
        return 0;
    }

    ret = unsafe {
        bpf_sysctl_get_current_value(ctx, value.as_mut_ptr(), MAX_VALUE_STR_LEN as u32)
    };
    if ret < 0 || ret >= MAX_VALUE_STR_LEN as i32 {
        return 0;
    }

    // __pragma_loop_unroll_full
    i = 0;
    while (i as usize) < tcp_mem.len() {
        ret = unsafe {
            bpf_strtoul(
                value.as_mut_ptr().wrapping_add(off as usize),
                MAX_ULONG_STR_LEN as u64,
                0,
                tcp_mem.as_mut_ptr().wrapping_add(i as usize),
            )
        };
        if ret <= 0 || ret > MAX_ULONG_STR_LEN as i32 {
            return 0;
        }
        off = off.wrapping_add((ret as u8) & MAX_ULONG_STR_LEN as u8);

        i = i.wrapping_add(1);
    }

    (tcp_mem[0] < tcp_mem[1] && tcp_mem[1] < tcp_mem[2]) as i32
}

// SEC("license")
#[unsafe(no_mangle)]
pub static mut _license: [core::ffi::c_char; 4] = [
    b'G' as core::ffi::c_char,
    b'P' as core::ffi::c_char,
    b'L' as core::ffi::c_char,
    0,
];
