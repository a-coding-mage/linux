// SPDX-License-Identifier: GPL-2.0
// C dependencies: <vmlinux.h>, <bpf/bpf_helpers.h>, <bpf/bpf_tracing.h>

#[unsafe(link_section = "license")]
#[unsafe(no_mangle)]
pub static mut _license: [u8; 4] = *b"GPL\0";

#[unsafe(link_section = "fentry.multi")]
#[unsafe(no_mangle)]
pub extern "C" fn test_fentry() -> i32 {
    return 0;
}

#[unsafe(link_section = "fentry.multi.s")]
#[unsafe(no_mangle)]
pub extern "C" fn test_fentry_s() -> i32 {
    return 0;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
