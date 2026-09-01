// SPDX-License-Identifier: GPL-2.0
/* Copyright Amazon.com Inc. or its affiliates. */
/* Translated from:
 * #include <vmlinux.h>
 * #include "bpf_tracing_net.h"
 * #include <bpf/bpf_helpers.h>
 * #include <limits.h>
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

type c_int = i32;
type c_char = i8;
type c_void = core::ffi::c_void;

const INT_MAX: c_int = 2_147_483_647;
const SOL_SOCKET: c_int = 1;
const SO_SNDBUF: c_int = 7;

const AUTOBIND_LEN: usize = 6;

#[no_mangle]
pub static mut sun_path: [c_char; AUTOBIND_LEN] = [0; AUTOBIND_LEN];

const NR_CASES: usize = 5;

#[no_mangle]
pub static mut sndbuf_setsockopt: [c_int; NR_CASES] =
    [-1, 0, 8192, INT_MAX / 2, INT_MAX];

#[no_mangle]
pub static mut sndbuf_getsockopt: [c_int; NR_CASES] = [-1, -1, -1, -1, -1];

#[no_mangle]
pub static mut sndbuf_getsockopt_expected: [c_int; NR_CASES] = [0; NR_CASES];

#[repr(C)]
pub struct sockaddr_un {
    pub sun_path: [c_char; AUTOBIND_LEN],
}

#[repr(C)]
pub struct unix_address {
    pub name: *mut sockaddr_un,
}

#[repr(C)]
pub struct unix_sock {
    pub addr: *mut unix_address,
}

#[repr(C)]
pub struct bpf_iter__unix {
    pub unix_sk: *mut unix_sock,
}

extern "C" {
    fn bpf_setsockopt(
        sk: *mut unix_sock,
        level: c_int,
        optname: c_int,
        optval: *const c_void,
        optlen: c_int,
    ) -> c_int;

    fn bpf_getsockopt(
        sk: *mut unix_sock,
        level: c_int,
        optname: c_int,
        optval: *mut c_void,
        optlen: c_int,
    ) -> c_int;
}

#[inline(always)]
unsafe fn cmpname(unix_sk: *mut unix_sock) -> c_int {
    let mut i: usize;

    i = 0;
    while i < AUTOBIND_LEN {
        if (*(*(*unix_sk).addr).name).sun_path[i] != sun_path[i] {
            return -1;
        }

        i += 1;
    }

    0
}

#[no_mangle]
#[link_section = "iter/unix"]
pub unsafe extern "C" fn change_sndbuf(ctx: *mut bpf_iter__unix) -> c_int {
    let unix_sk: *mut unix_sock = (*ctx).unix_sk;
    let mut i: usize;
    let mut err: c_int;

    if unix_sk.is_null() || (*unix_sk).addr.is_null() {
        return 0;
    }

    if (*(*(*unix_sk).addr).name).sun_path[0] != 0 {
        return 0;
    }

    if cmpname(unix_sk) != 0 {
        return 0;
    }

    i = 0;
    while i < NR_CASES {
        err = bpf_setsockopt(
            unix_sk,
            SOL_SOCKET,
            SO_SNDBUF,
            &sndbuf_setsockopt[i] as *const c_int as *const c_void,
            core::mem::size_of_val(&sndbuf_setsockopt[i]) as c_int,
        );
        if err != 0 {
            break;
        }

        err = bpf_getsockopt(
            unix_sk,
            SOL_SOCKET,
            SO_SNDBUF,
            &mut sndbuf_getsockopt[i] as *mut c_int as *mut c_void,
            core::mem::size_of_val(&sndbuf_getsockopt[i]) as c_int,
        );
        if err != 0 {
            break;
        }

        i += 1;
    }

    0
}

#[no_mangle]
#[link_section = "license"]
pub static _license: [u8; 4] = *b"GPL\0";

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
