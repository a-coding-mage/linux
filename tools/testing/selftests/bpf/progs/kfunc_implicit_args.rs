// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2026 Meta Platforms, Inc. and affiliates. */

// C dependencies:
// #include <vmlinux.h>
// #include <bpf/bpf_helpers.h>
// #include "bpf_misc.h"

extern "C" {
    #[link_name = "bpf_kfunc_implicit_arg"]
    fn bpf_kfunc_implicit_arg(a: core::ffi::c_int) -> core::ffi::c_int;

    /* illegal */
    #[link_name = "bpf_kfunc_implicit_arg_impl"]
    fn bpf_kfunc_implicit_arg_impl(
        a: core::ffi::c_int,
        aux: *mut bpf_prog_aux,
    ) -> core::ffi::c_int;

    #[link_name = "bpf_kfunc_implicit_arg_legacy"]
    fn bpf_kfunc_implicit_arg_legacy(
        a: core::ffi::c_int,
        b: core::ffi::c_int,
    ) -> core::ffi::c_int;

    #[link_name = "bpf_kfunc_implicit_arg_legacy_impl"]
    fn bpf_kfunc_implicit_arg_legacy_impl(
        a: core::ffi::c_int,
        b: core::ffi::c_int,
        aux: *mut bpf_prog_aux,
    ) -> core::ffi::c_int;
}

#[repr(C)]
pub struct bpf_prog_aux {
    _private: [u8; 0],
}

#[no_mangle]
#[link_section = "license"]
pub static mut _license: [core::ffi::c_char; 4] = [b'G' as core::ffi::c_char, b'P' as core::ffi::c_char, b'L' as core::ffi::c_char, 0];

#[no_mangle]
#[link_section = "syscall"]
// __retval(5)
pub unsafe extern "C" fn test_kfunc_implicit_arg(ctx: *mut core::ffi::c_void) -> core::ffi::c_int {
    let _ = ctx;
    bpf_kfunc_implicit_arg(5)
}

#[no_mangle]
#[link_section = "syscall"]
// __failure __msg("cannot find address for kernel function bpf_kfunc_implicit_arg_impl")
pub unsafe extern "C" fn test_kfunc_implicit_arg_impl_illegal(
    ctx: *mut core::ffi::c_void,
) -> core::ffi::c_int {
    let _ = ctx;
    bpf_kfunc_implicit_arg_impl(5, core::ptr::null_mut())
}

#[no_mangle]
#[link_section = "syscall"]
// __retval(7)
pub unsafe extern "C" fn test_kfunc_implicit_arg_legacy(
    ctx: *mut core::ffi::c_void,
) -> core::ffi::c_int {
    let _ = ctx;
    bpf_kfunc_implicit_arg_legacy(3, 4)
}

#[no_mangle]
#[link_section = "syscall"]
// __retval(11)
pub unsafe extern "C" fn test_kfunc_implicit_arg_legacy_impl(
    ctx: *mut core::ffi::c_void,
) -> core::ffi::c_int {
    let _ = ctx;
    bpf_kfunc_implicit_arg_legacy_impl(5, 6, core::ptr::null_mut())
}
