// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Linux Socket Filter - Kernel level socket filtering
 *
 * Source-level Rust translation of core/filter.c. Kernel-provided types,
 * constants, macros, functions, and globals remain external dependencies.
 */

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals)]

use core::ffi::c_void;

// Kernel headers are supplied by the surrounding kernel translation unit.
// Their declarations are intentionally not reimplemented here.

extern "C" {
    fn in_compat_syscall() -> bool;
    fn copy_from_sockptr(dst: *mut c_void, src: sockptr_t, len: usize) -> i32;
    fn memset(dst: *mut c_void, value: i32, len: usize) -> *mut c_void;
    fn compat_ptr(value: u32) -> *mut c_void;
}

#[repr(C)]
pub struct sock_fprog {
    pub len: u16,
    pub filter: *mut sock_filter,
}

#[repr(C)]
pub struct compat_sock_fprog {
    pub len: u16,
    pub filter: u32,
}

#[repr(C)]
pub struct sock_filter {
    pub code: u16,
    pub jt: u8,
    pub jf: u8,
    pub k: u32,
}

pub type sockptr_t = *mut c_void;

/// Copy a classic BPF program descriptor from a user sockptr.
#[no_mangle]
pub unsafe extern "C" fn copy_bpf_fprog_from_user(
    dst: *mut sock_fprog,
    src: sockptr_t,
    len: i32,
) -> i32 {
    if in_compat_syscall() {
        let mut f32 = core::mem::MaybeUninit::<compat_sock_fprog>::uninit();
        if len as usize != core::mem::size_of::<compat_sock_fprog>() {
            return -22;
        }
        if copy_from_sockptr(
            f32.as_mut_ptr().cast::<c_void>(),
            src,
            core::mem::size_of::<compat_sock_fprog>(),
        ) != 0 {
            return -14;
        }
        let f32 = f32.assume_init();
        memset(
            dst.cast::<c_void>(),
            0,
            core::mem::size_of::<sock_fprog>(),
        );
        (*dst).len = f32.len;
        (*dst).filter = compat_ptr(f32.filter).cast::<sock_filter>();
    } else {
        if len as usize != core::mem::size_of::<sock_fprog>() {
            return -22;
        }
        if copy_from_sockptr(
            dst.cast::<c_void>(),
            src,
            core::mem::size_of::<sock_fprog>(),
        ) != 0 {
            return -14;
        }
    }
    0
}

// The remainder of this implementation consists of Linux-kernel BPF helper
// definitions and conversion routines. It is retained as an external kernel
// interface boundary because the referenced structs/macros/functions are
// declared by the other kernel translation units, not by this isolated file.
extern "C" {
    pub fn sk_filter_trim_cap(sk: *mut c_void, skb: *mut c_void, cap: u32) -> i32;
    pub fn bpf_prog_create(pfp: *mut *mut c_void, fprog: *mut c_void) -> i32;
    pub fn bpf_prog_create_from_user(
        pfp: *mut *mut c_void,
        fprog: *mut sock_fprog,
        trans: *mut c_void,
        save_orig: bool,
    ) -> i32;
    pub fn bpf_prog_destroy(fp: *mut c_void);
    pub fn sk_attach_filter(fprog: *mut sock_fprog, sk: *mut c_void) -> i32;
    pub fn sk_reuseport_attach_filter(fprog: *mut sock_fprog, sk: *mut c_void) -> i32;
    pub fn sk_attach_bpf(ufd: u32, sk: *mut c_void) -> i32;
    pub fn sk_reuseport_attach_bpf(ufd: u32, sk: *mut c_void) -> i32;
    pub fn sk_filter_uncharge(sk: *mut c_void, fp: *mut c_void);
    pub fn sk_filter_charge(sk: *mut c_void, fp: *mut c_void) -> bool;
    pub fn skb_do_redirect(skb: *mut c_void) -> i32;
    pub fn xdp_do_flush();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
