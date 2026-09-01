// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2023 Red Hat, Inc. */
// C includes translated as external dependencies:
// linux/bpf.h, bpf/bpf_helpers.h, bpf/bpf_tracing.h

#[no_mangle]
#[link_section = "license"]
pub static mut _license: [u8; 4] = *b"GPL\0";

#[repr(C)]
pub struct pt_regs {
    _unused: [u8; 0],
}

/* Dummy fentry bpf prog for testing fentry attachment chains. It's going to be
 * a start of the chain.
 */
#[no_mangle]
#[link_section = "fentry/bpf_testmod_fentry_test1"]
pub unsafe extern "C" fn test1(a: ::core::ffi::c_int) -> ::core::ffi::c_int {
    let _ = a;
    return 0;
}

/* Dummy bpf prog for testing attach_btf presence when attaching an fentry
 * program.
 */
#[no_mangle]
#[link_section = "raw_tp/sys_enter"]
pub unsafe extern "C" fn fentry_target(
    regs: *mut pt_regs,
    id: ::core::ffi::c_long,
) -> ::core::ffi::c_int {
    let _ = regs;
    let _ = id;
    return 0;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
