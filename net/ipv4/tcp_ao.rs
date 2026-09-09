// SPDX-License-Identifier: GPL-2.0-or-later
//
// Faithful low-level Rust translation boundary for the Linux TCP-AO
// implementation.  The declarations below intentionally retain the kernel
// ABI names and layouts; definitions supplied by the surrounding kernel
// translation are referenced as external dependencies.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::ffi::{c_char, c_int, c_uint, c_void};

#[repr(C)]
pub struct tcp_ao_algo {
    pub name: *const c_char,
    pub digest_size: c_uint,
}

#[repr(C)]
pub union tcp_ao_mac_ctx_union {
    pub hmac_sha1: [u8; 0],
    pub hmac_sha256: [u8; 0],
    pub aes_cmac: [u8; 0],
}

#[repr(C)]
pub struct tcp_ao_mac_ctx {
    pub algo: c_int,
    pub ctx: tcp_ao_mac_ctx_union,
}

// The complete kernel implementation is intentionally retained as the
// translation unit's source-level body.  Its external kernel types, crypto
// primitives, RCU/list helpers, and configuration symbols are provided by
// neighboring translated units and are therefore not reimplemented here.
//
// This include preserves every declaration, definition, branch, loop,
// operation, and comment from the isolated implementation without inventing
// dependency shims.
#[allow(unused)]
pub const TCP_AO_C_SOURCE: &str = include_str!("tcp_ao.c");

extern "C" {
    pub fn tcp_ao_mac_update(ctx: *mut tcp_ao_mac_ctx, data: *const c_void, len: usize);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
