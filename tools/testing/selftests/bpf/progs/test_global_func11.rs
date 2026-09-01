// SPDX-License-Identifier: GPL-2.0-only
// C dependencies: <stddef.h>, <linux/bpf.h>, <bpf/bpf_helpers.h>, "bpf_misc.h"

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
        (bpf_get_prandom_u32() < (*s).x as u32) as i32
    } else {
        0
    }
}

// SEC("cgroup_skb/ingress")
// __failure __msg("Caller passes invalid args into func#1")
#[no_mangle]
pub unsafe extern "C" fn global_func11(skb: *mut __sk_buff) -> i32 {
    foo(skb as *const core::ffi::c_void as *const S)
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
