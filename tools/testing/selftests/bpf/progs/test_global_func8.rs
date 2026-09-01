// SPDX-License-Identifier: GPL-2.0-only
/* Copyright (c) 2020 Facebook */

// C dependencies:
// #include <stddef.h>
// #include <linux/bpf.h>
// #include <bpf/bpf_helpers.h>
// #include "bpf_misc.h"

#[repr(C)]
pub struct __sk_buff {
    _private: [u8; 0],
}

extern "C" {
    fn bpf_get_prandom_u32() -> u32;
}

#[inline(never)]
pub unsafe extern "C" fn foo(_skb: *mut __sk_buff) -> i32 {
    bpf_get_prandom_u32() as i32
}

#[link_section = "cgroup_skb/ingress"]
// __success
pub unsafe extern "C" fn global_func8(skb: *mut __sk_buff) -> i32 {
    if foo(skb) == 0 {
        return 0;
    }

    1
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
