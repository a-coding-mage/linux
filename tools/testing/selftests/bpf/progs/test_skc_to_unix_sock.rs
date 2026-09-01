// SPDX-License-Identifier: GPL-2.0
// Copyright (c) 2021 Hengqi Chen

// C dependencies:
// #include "vmlinux.h"
// #include <bpf/bpf_helpers.h>
// #include <bpf/bpf_tracing.h>
// #include "bpf_tracing_net.h"

use core::ffi::{c_char, c_int, c_short};
use core::mem::size_of;

extern "C" {
    fn bpf_get_current_pid_tgid() -> u64;
    fn bpf_skc_to_unix_sock(sk: *mut core::ffi::c_void) -> *mut unix_sock;
}

pub type pid_t = i32;

#[repr(C)]
pub struct socket {
    pub sk: *mut core::ffi::c_void,
}

#[repr(C)]
pub struct unix_sock {
    pub addr: *mut unix_address,
}

#[repr(C)]
pub struct unix_address {
    pub len: c_int,
    pub name: *mut sockaddr_un,
}

#[repr(C)]
pub struct sockaddr_un {
    pub sun_family: c_short,
    pub sun_path: [c_char; 108],
}

#[no_mangle]
pub static mut my_pid: pid_t = 0;

#[no_mangle]
pub static mut path: [c_char; 256] = [0; 256];

#[no_mangle]
#[link_section = "fentry/unix_listen"]
pub unsafe extern "C" fn unix_listen(sock: *mut socket, backlog: c_int) -> c_int {
    let pid: pid_t = (bpf_get_current_pid_tgid() >> 32) as pid_t;
    let mut unix_sk: *mut unix_sock;
    let mut i: c_int;
    let len: c_int;

    let _ = backlog;

    if pid != core::ptr::read_volatile(&my_pid) {
        return 0;
    }

    unix_sk = bpf_skc_to_unix_sock((*sock).sk);
    if unix_sk.is_null() {
        return 0;
    }

    if (*(*(*unix_sk).addr).name).sun_path[0] != 0 {
        return 0;
    }

    len = (*(*unix_sk).addr).len - size_of::<c_short>() as c_int;
    path[0] = b'@' as c_char;
    i = 1;
    while i < len {
        if i >= size_of::<sockaddr_un>() as c_int {
            break;
        }

        path[i as usize] = (*(*(*unix_sk).addr).name).sun_path[i as usize];
        i += 1;
    }
    return 0;
}

#[no_mangle]
#[link_section = "license"]
pub static _license: [u8; 4] = *b"GPL\0";

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
