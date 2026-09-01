// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2021 Facebook */
/* Rust translation of dependency intent: #include "bpf_tracing_net.h" */

use core::ffi::{c_char, c_int, c_void};
use core::mem::size_of_val;

#[unsafe(link_section = "license")]
#[no_mangle]
pub static _license: [u8; 4] = *b"GPL\0";

#[no_mangle]
pub static mut page_size: i32 = 0;

#[no_mangle]
pub static cc_reno: [c_char; TCP_CA_NAME_MAX] = {
    let mut buf = [0 as c_char; TCP_CA_NAME_MAX];
    buf[0] = b'r' as c_char;
    buf[1] = b'e' as c_char;
    buf[2] = b'n' as c_char;
    buf[3] = b'o' as c_char;
    buf
};

#[no_mangle]
pub static cc_cubic: [c_char; TCP_CA_NAME_MAX] = {
    let mut buf = [0 as c_char; TCP_CA_NAME_MAX];
    buf[0] = b'c' as c_char;
    buf[1] = b'u' as c_char;
    buf[2] = b'b' as c_char;
    buf[3] = b'i' as c_char;
    buf[4] = b'c' as c_char;
    buf
};

unsafe extern "C" {
    fn bpf_getsockopt(
        sk: *mut c_void,
        level: c_int,
        optname: c_int,
        optval: *mut c_void,
        optlen: c_int,
    ) -> c_int;
    fn bpf_setsockopt(
        sk: *mut c_void,
        level: c_int,
        optname: c_int,
        optval: *mut c_void,
        optlen: c_int,
    ) -> c_int;
    fn bpf_strncmp(s1: *const c_char, s1_sz: u32, s2: *const c_char) -> c_int;
}

#[unsafe(link_section = "cgroup/setsockopt")]
#[no_mangle]
pub unsafe extern "C" fn sockopt_qos_to_cc(ctx: *mut bpf_sockopt) -> c_int {
    let optval_end: *mut c_void = unsafe { (*ctx).optval_end };
    let optval: *mut c_int = unsafe { (*ctx).optval };
    let mut buf: [c_char; TCP_CA_NAME_MAX] = [0; TCP_CA_NAME_MAX];

    if unsafe { (*ctx).level } != SOL_IPV6 || unsafe { (*ctx).optname } != IPV6_TCLASS {
        /* out: */
        /* optval larger than PAGE_SIZE use kernel's buffer. */
        if unsafe { (*ctx).optlen } > unsafe { page_size } {
            unsafe {
                (*ctx).optlen = 0;
            }
        }
        return 1;
    }

    if unsafe { optval.add(1) as usize } > optval_end as usize {
        return 0; /* EPERM, bounds check */
    }

    if unsafe {
        bpf_getsockopt(
            (*ctx).sk,
            SOL_TCP,
            TCP_CONGESTION,
            buf.as_mut_ptr() as *mut c_void,
            size_of_val(&buf) as c_int,
        )
    } != 0
    {
        return 0;
    }

    if unsafe { bpf_strncmp(buf.as_ptr(), size_of_val(&buf) as u32, cc_cubic.as_ptr()) } != 0 {
        return 0;
    }

    if unsafe { *optval } == 0x2d {
        if unsafe {
            bpf_setsockopt(
                (*ctx).sk,
                SOL_TCP,
                TCP_CONGESTION,
                cc_reno.as_ptr() as *mut c_void,
                size_of_val(&cc_reno) as c_int,
            )
        } != 0
        {
            return 0;
        }
    }
    1
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
