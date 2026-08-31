// SPDX-License-Identifier: GPL-2.0
// C dependencies: <vmlinux.h>, <bpf/bpf_helpers.h>, <bpf/bpf_tracing.h>

#[unsafe(link_section = "license")]
#[unsafe(no_mangle)]
pub static mut _license: [u8; 4] = *b"GPL\0";

#[unsafe(link_section = "fentry.multi")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn bench() -> i32 {
    return 0;
}
