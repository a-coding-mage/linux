// SPDX-License-Identifier: GPL-2.0
// C dependencies: <linux/bpf.h>, <bpf/bpf_helpers.h>, <bpf/bpf_tracing.h>

unsafe extern "C" {
    pub type pt_regs;
}

#[unsafe(link_section = "license")]
#[unsafe(no_mangle)]
pub static mut _license: [::core::ffi::c_char; 4] = [
    b'G' as ::core::ffi::c_char,
    b'P' as ::core::ffi::c_char,
    b'L' as ::core::ffi::c_char,
    0,
];

#[unsafe(link_section = "kprobe.multi/")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn test_kprobe_empty(ctx: *mut pt_regs) -> ::core::ffi::c_int {
    0
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
