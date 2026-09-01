// SPDX-License-Identifier: GPL-2.0
// C dependencies:
// #include <vmlinux.h>
// #include <bpf/bpf_helpers.h>
// #include "../test_kmods/bpf_testmod_kfunc.h"

extern "C" {
    fn bpf_kfunc_call_test_destructive();
}

#[no_mangle]
#[link_section = "tc"]
pub unsafe extern "C" fn kfunc_destructive_test() -> i32 {
    unsafe {
        bpf_kfunc_call_test_destructive();
    }
    0
}

#[no_mangle]
#[link_section = "license"]
pub static _license: [u8; 4] = *b"GPL\0";

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
