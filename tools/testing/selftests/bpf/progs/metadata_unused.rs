// SPDX-License-Identifier: GPL-2.0-only

// C dependencies: <linux/bpf.h>, <bpf/bpf_helpers.h>

#[repr(C)]
pub struct xdp_md {
    _unused: [u8; 0],
}

#[unsafe(link_section = ".rodata")]
#[unsafe(no_mangle)]
pub static bpf_metadata_a: [u8; 4] = *b"foo\0";

#[unsafe(link_section = ".rodata")]
#[unsafe(no_mangle)]
pub static bpf_metadata_b: i32 = 1;

#[unsafe(link_section = "cgroup_skb/egress")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn prog(ctx: *mut xdp_md) -> i32 {
    let _ = ctx;
    return 0;
}

#[unsafe(link_section = "license")]
#[unsafe(no_mangle)]
pub static mut _license: [u8; 4] = *b"GPL\0";

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
