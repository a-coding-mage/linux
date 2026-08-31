// SPDX-License-Identifier: GPL-2.0-only
// C includes translated as external dependencies:
// <stddef.h>, <linux/bpf.h>, <bpf/bpf_helpers.h>, "bpf_misc.h"

#[repr(C)]
pub struct S {
    pub x: i32,
}

#[repr(C)]
pub struct __sk_buff {
    _private: [u8; 0],
}

extern "C" {
    fn bpf_get_prandom_u32() -> u32;
}

#[inline(never)]
pub unsafe extern "C" fn foo(s: *const S) -> i32 {
    if !s.is_null() {
        return (bpf_get_prandom_u32() < (*s).x as u32) as i32;
    }

    0
}

// SEC("cgroup_skb/ingress")
// __failure __msg("Caller passes invalid args into func#1")
pub unsafe extern "C" fn global_func13(skb: *mut __sk_buff) -> i32 {
    let _ = skb;
    let s: *const S = 0xbedabedausize as *const S;

    foo(s)
}
