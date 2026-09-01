// SPDX-License-Identifier: GPL-2.0
// Copyright (c) 2019 Facebook
// C dependencies: "vmlinux.h", <bpf/bpf_helpers.h>, <bpf/bpf_tracing.h>

#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]

unsafe extern "C" {
    pub type pt_regs;

    fn PT_REGS_RC(ctx: *mut pt_regs) -> i32;
}

#[no_mangle]
#[link_section = "license"]
pub static mut _license: [u8; 4] = *b"GPL\0";

#[no_mangle]
#[link_section = "raw_tracepoint/kfree_skb"]
pub unsafe extern "C" fn nested_loops(ctx: *mut pt_regs) -> i32 {
    let mut i: i32;
    let mut j: i32;
    let mut sum: i32 = 0;
    let mut m: i32;

    j = 0;
    while j < 300 {
        i = 0;
        while i < j {
            if (j & 1) != 0 {
                m = unsafe { PT_REGS_RC(ctx) };
            } else {
                m = j;
            }
            sum += i * m;
            i += 1;
        }
        j += 1;
    }

    sum
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
