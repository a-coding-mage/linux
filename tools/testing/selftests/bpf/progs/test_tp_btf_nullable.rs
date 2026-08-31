// SPDX-License-Identifier: GPL-2.0

// C dependencies:
// #include "vmlinux.h"
// #include <bpf/bpf_helpers.h>
// #include <bpf/bpf_tracing.h>
// #include "../test_kmods/bpf_testmod.h"
// #include "bpf_misc.h"

#[repr(C)]
pub struct bpf_testmod_test_read_ctx {
    pub len: i32,
}

// SEC("tp_btf/bpf_testmod_test_nullable_bare_tp")
// __failure __msg("R1 invalid mem access 'trusted_ptr_or_null_'")
#[no_mangle]
pub unsafe extern "C" fn handle_tp_btf_nullable_bare1(
    nullable_ctx: *mut bpf_testmod_test_read_ctx,
) -> i32 {
    unsafe { (*nullable_ctx).len }
}

// SEC("tp_btf/bpf_testmod_test_nullable_bare_tp")
#[no_mangle]
pub unsafe extern "C" fn handle_tp_btf_nullable_bare2(
    nullable_ctx: *mut bpf_testmod_test_read_ctx,
) -> i32 {
    if !nullable_ctx.is_null() {
        return unsafe { (*nullable_ctx).len };
    }
    0
}

#[no_mangle]
#[link_section = "license"]
pub static mut _license: [u8; 4] = *b"GPL\0";
