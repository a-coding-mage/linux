// SPDX-License-Identifier: GPL-2.0
// C dependencies: <vmlinux.h>, <bpf/bpf_helpers.h>,
// "../test_kmods/bpf_testmod_kfunc.h"

extern "C" {
    fn bpf_testmod_test_mod_kfunc(arg: i32);
}

#[repr(C)]
pub struct __sk_buff {
    _private: [u8; 0],
}

#[no_mangle]
#[link_section = "tc"]
pub unsafe extern "C" fn kfunc_call_fail(ctx: *mut __sk_buff) -> i32 {
    let _ = ctx;
    bpf_testmod_test_mod_kfunc(0);
    0
}

#[no_mangle]
#[link_section = "license"]
pub static _license: [u8; 4] = *b"GPL\0";
