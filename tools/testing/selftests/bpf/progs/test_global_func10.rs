// SPDX-License-Identifier: GPL-2.0-only
// C dependencies removed from executable Rust:
// <stddef.h>, <linux/bpf.h>, <bpf/bpf_helpers.h>, "bpf_misc.h"
// Original C condition:
// #if !defined(__clang__)
// #pragma GCC diagnostic ignored "-Wmaybe-uninitialized"
// #endif

#[repr(C)]
pub struct Small {
    pub x: core::ffi::c_long,
}

#[repr(C)]
pub struct Big {
    pub x: core::ffi::c_long,
    pub y: core::ffi::c_long,
}

// External dependency supplied by BPF helper headers in the original C source.
extern "C" {
    fn bpf_get_prandom_u32() -> u32;
}

#[repr(C)]
pub struct __sk_buff {
    pub len: u32,
}

#[no_mangle]
#[inline(never)]
pub unsafe extern "C" fn foo(big: *const Big) -> core::ffi::c_int {
    if big.is_null() {
        return 0;
    }

    (bpf_get_prandom_u32() < (*big).y as u32) as core::ffi::c_int
}

// SEC("cgroup_skb/ingress")
// __failure __msg("invalid read from stack")
#[no_mangle]
pub unsafe extern "C" fn global_func10(skb: *mut __sk_buff) -> core::ffi::c_int {
    let small = Small {
        x: (*skb).len as core::ffi::c_long,
    };

    if foo((&small as *const Small).cast::<Big>()) != 0 {
        1
    } else {
        0
    }
}
