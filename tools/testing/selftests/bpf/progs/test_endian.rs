// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2020 Facebook */

// Depends on definitions normally supplied by vmlinux.h, bpf_helpers.h, and
// bpf_endian.h in the original C source.

type __u16 = u16;
type __u32 = u32;
type __u64 = u64;

const IN16: __u16 = 0x1234;
const IN32: __u32 = 0x12345678u32;
const IN64: __u64 = 0x123456789abcdef0u64;

#[no_mangle]
pub static mut in16: __u16 = 0;
#[no_mangle]
pub static mut in32: __u32 = 0;
#[no_mangle]
pub static mut in64: __u64 = 0;

#[no_mangle]
pub static mut out16: __u16 = 0;
#[no_mangle]
pub static mut out32: __u32 = 0;
#[no_mangle]
pub static mut out64: __u64 = 0;

#[no_mangle]
pub static mut const16: __u16 = 0;
#[no_mangle]
pub static mut const32: __u32 = 0;
#[no_mangle]
pub static mut const64: __u64 = 0;

#[inline]
const fn ___bpf_swab16(x: __u16) -> __u16 {
    x.swap_bytes()
}

#[inline]
const fn ___bpf_swab32(x: __u32) -> __u32 {
    x.swap_bytes()
}

#[inline]
const fn ___bpf_swab64(x: __u64) -> __u64 {
    x.swap_bytes()
}

#[no_mangle]
#[link_section = "raw_tp/sys_enter"]
pub unsafe extern "C" fn sys_enter(ctx: *const core::ffi::c_void) -> i32 {
    let _ = ctx;

    out16 = in16.swap_bytes();
    out32 = in32.swap_bytes();
    out64 = in64.swap_bytes();
    const16 = ___bpf_swab16(IN16);
    const32 = ___bpf_swab32(IN32);
    const64 = ___bpf_swab64(IN64);

    return 0;
}

#[no_mangle]
#[link_section = "license"]
pub static _license: [u8; 4] = *b"GPL\0";

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
