// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2024 Google LLC */

/*
 * C dependencies translated as external Rust dependencies:
 * vmlinux.h, string.h, bpf/bpf_helpers.h, bpf/bpf_endian.h,
 * bpf/bpf_core_read.h, and bpf_kfuncs.h.
 */

pub const REWRITE_ADDRESS_IP4: u32 = 0xc0a801fe; // 192.168.1.254
pub const REWRITE_ADDRESS_PORT4: u16 = 4040;

#[no_mangle]
#[link_section = "cgroup/getsockname4"]
pub unsafe extern "C" fn getsockname_v4_prog(ctx: *mut bpf_sock_addr) -> ::core::ffi::c_int {
    unsafe {
        (*ctx).user_ip4 = REWRITE_ADDRESS_IP4.to_be();
        (*ctx).user_port = REWRITE_ADDRESS_PORT4.to_be();
    }

    1
}

#[no_mangle]
#[link_section = "license"]
pub static _license: [u8; 4] = *b"GPL\0";

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
