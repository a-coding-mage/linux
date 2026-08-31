// SPDX-License-Identifier: GPL-2.0-only
// C dependencies: <stddef.h>, <linux/bpf.h>, <bpf/bpf_helpers.h>, "bpf_misc.h"

#[repr(C)]
pub struct S {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct __sk_buff {
    _unused: [u8; 0],
}

unsafe extern "C" {
    fn bpf_get_prandom_u32() -> u32;
}

#[unsafe(no_mangle)]
#[inline(never)]
pub unsafe extern "C" fn foo(s: *const S) -> i32 {
    if !s.is_null() {
        return (unsafe { bpf_get_prandom_u32() } < unsafe { *(s as *const i32) } as u32) as i32;
    }

    0
}

// SEC("cgroup_skb/ingress")
// __failure
// __msg("reference type('FWD S') size cannot be determined")
#[unsafe(no_mangle)]
#[unsafe(link_section = "cgroup_skb/ingress")]
pub unsafe extern "C" fn global_func14(skb: *mut __sk_buff) -> i32 {
    let _ = skb;

    unsafe { foo(core::ptr::null()) }
}
