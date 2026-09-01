// SPDX-License-Identifier: GPL-2.0
// Copyright (c) 2020 Facebook

// C dependencies:
// - "vmlinux.h"
// - <bpf/bpf_helpers.h>
// - <bpf/bpf_tracing.h>

use core::ffi::{c_char, c_int};

#[no_mangle]
#[link_section = "freplace/btf_unreliable_kprobe"]
/* context type is what BPF verifier expects for kprobe context, but target
 * program has `stuct whatever *ctx` argument, so freplace operation will be
 * rejected with the following message:
 *
 * arg0 replace_btf_unreliable_kprobe(struct pt_regs *) doesn't match btf_unreliable_kprobe(struct whatever *)
 */
pub unsafe extern "C" fn replace_btf_unreliable_kprobe(
    ctx: *mut bpf_user_pt_regs_t,
) -> c_int {
    0
}

#[no_mangle]
#[link_section = "license"]
pub static mut _license: [c_char; 4] = [
    b'G' as c_char,
    b'P' as c_char,
    b'L' as c_char,
    0,
];

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
