// SPDX-License-Identifier: GPL-2.0-or-later

// Original C dependencies:
// #include "vmlinux.h"
// #include <bpf/bpf_helpers.h>

const NF_ACCEPT: i32 = 1;

#[no_mangle]
#[link_section = "netfilter"]
pub extern "C" fn nf_link_attach_test(ctx: *mut bpf_nf_ctx) -> i32 {
    NF_ACCEPT
}

#[no_mangle]
#[link_section = "license"]
pub static mut _license: [u8; 4] = *b"GPL\0";

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
