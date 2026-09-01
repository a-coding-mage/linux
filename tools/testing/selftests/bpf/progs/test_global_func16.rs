// SPDX-License-Identifier: GPL-2.0-only
// C includes translated as external dependencies:
// <stddef.h>, <linux/bpf.h>, <bpf/bpf_helpers.h>, "bpf_misc.h"

#[repr(C)]
pub struct __sk_buff {
    _unused: [u8; 0],
}

#[inline(never)]
pub unsafe extern "C" fn foo(arr: *mut [i32; 10]) -> i32 {
    if !arr.is_null() {
        return (*arr)[9];
    }

    0
}

// SEC("cgroup_skb/ingress")
// __success
#[no_mangle]
pub unsafe extern "C" fn global_func16(skb: *mut __sk_buff) -> i32 {
    let mut array: [i32; 10] = core::mem::MaybeUninit::uninit().assume_init();

    let rv: i32 = foo(&mut array);

    if rv != 0 { 1 } else { 0 }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
