// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2023 Red Hat, Inc. */
// C dependencies: <linux/bpf.h>, <bpf/bpf_helpers.h>, <bpf/bpf_tracing.h>

#[unsafe(link_section = "license")]
#[unsafe(no_mangle)]
pub static mut _license: [u8; 4] = *b"GPL\0";

/* Dummy fentry bpf prog for testing fentry attachment chains */
#[unsafe(link_section = "fentry/XXX")]
#[unsafe(no_mangle)]
pub extern "C" fn recursive_attach(a: ::core::ffi::c_int) -> ::core::ffi::c_int {
    let _ = a;
    return 0;
}
