// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2021 Facebook */

// C dependencies removed from executable Rust:
// #include "bpf_tracing_net.h"
// #include <bpf/bpf_helpers.h>
// #include <bpf/bpf_tracing.h>

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(improper_ctypes)]

use core::ffi::c_void;

extern "C" {
    pub type sock;
    pub type tcp_congestion_ops;

    fn bpf_setsockopt(
        sk: *mut sock,
        level: i32,
        optname: i32,
        optval: *mut c_void,
        optlen: i32,
    ) -> i32;
}

extern "C" {
    static SOL_TCP: i32;
    static TCP_CONGESTION: i32;
}

#[link_section = "license"]
#[no_mangle]
pub static mut _license: [u8; 4] = *b"GPL\0";

#[no_mangle]
pub static cubic: [u8; 6] = *b"cubic\0";

// SEC("struct_ops")
#[link_section = "struct_ops"]
#[no_mangle]
pub unsafe extern "C" fn dctcp_nouse_release(sk: *mut sock) {
    bpf_setsockopt(
        sk,
        SOL_TCP,
        TCP_CONGESTION,
        cubic.as_ptr() as *mut c_void,
        core::mem::size_of_val(&cubic) as i32,
    );
}

#[repr(C)]
pub struct tcp_congestion_ops_init {
    pub release: *mut c_void,
    pub name: [u8; 13],
}

// SEC(".struct_ops")
#[link_section = ".struct_ops"]
#[no_mangle]
pub static mut dctcp_rel: tcp_congestion_ops_init = tcp_congestion_ops_init {
    release: dctcp_nouse_release as *mut c_void,
    name: *b"bpf_dctcp_rel",
};

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
