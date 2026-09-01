// SPDX-License-Identifier: GPL-2.0

// C dependencies:
// #include <vmlinux.h>
// #include <bpf/bpf_helpers.h>
// #include "bpf_misc.h"

// SEC("fentry/bpf_fentry_test_sinfo")
// __description("typedef: resolve")
// __success __retval(0)
#[unsafe(link_section = "fentry/bpf_fentry_test_sinfo")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn resolve_typedef() {
    // __naked void resolve_typedef(void)
    // %[frags_offs] is the C expression:
    // offsetof(struct skb_shared_info, frags)
    unsafe {
        core::arch::asm!(
            "r1 = *(u64 *)(r1 +0)",
            "r2 = *(u64 *)(r1 +{frags_offs})",
            "r0 = 0",
            "exit",
            frags_offs = const TODO_SKB_SHARED_INFO_FRAGS_OFFSET,
            options(noreturn)
        );
    }
}

// TODO: provided by translated vmlinux bindings or build-time BPF constants:
// offsetof(struct skb_shared_info, frags)
pub const TODO_SKB_SHARED_INFO_FRAGS_OFFSET: i32 = 0;

#[unsafe(link_section = "license")]
#[unsafe(no_mangle)]
pub static mut _license: [u8; 4] = *b"GPL\0";

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
