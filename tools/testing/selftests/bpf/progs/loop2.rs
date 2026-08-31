// SPDX-License-Identifier: GPL-2.0
// Copyright (c) 2019 Facebook
// Depends on vmlinux.h, bpf/bpf_helpers.h, and bpf/bpf_tracing.h.

extern "C" {
    fn PT_REGS_RC(ctx: *mut pt_regs) -> i64;
}

#[repr(C)]
pub struct pt_regs {
    _private: [u8; 0],
}

#[no_mangle]
#[link_section = "license"]
pub static mut _license: [u8; 4] = *b"GPL\0";

#[no_mangle]
#[link_section = "raw_tracepoint/consume_skb"]
pub unsafe extern "C" fn while_true(ctx: *mut pt_regs) -> i32 {
    let mut i: i32 = 0;

    while true {
        if PT_REGS_RC(ctx) & 1 != 0 {
            i += 3;
        } else {
            i += 7;
        }
        if i > 40 {
            break;
        }
    }

    i
}
