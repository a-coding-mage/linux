// SPDX-License-Identifier: GPL-2.0-only
// C dependencies: <stddef.h>, <linux/bpf.h>, <bpf/bpf_helpers.h>, "bpf_misc.h"

#[repr(C)]
pub struct S {
    pub x: i32,
}

extern "C" {
    fn bpf_get_prandom_u32() -> u32;
}

#[inline(never)]
pub unsafe extern "C" fn foo(s: *const S) -> i32 {
    (bpf_get_prandom_u32() < (*s).x as u32) as i32
}

// SEC("cgroup_skb/ingress")
// __failure __msg("invalid mem access 'mem_or_null'")
#[unsafe(no_mangle)]
#[unsafe(link_section = "cgroup_skb/ingress")]
pub unsafe extern "C" fn global_func12(skb: *mut __sk_buff) -> i32 {
    let s: S = S {
        x: (*skb).len as i32,
    };

    foo(&s as *const S);

    1
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
