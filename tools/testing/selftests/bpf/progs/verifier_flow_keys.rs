// SPDX-License-Identifier: GPL-2.0
/* Bounds checks for PTR_TO_FLOW_KEYS pointer arithmetic. */

// C dependencies:
// #include "vmlinux.h"
// #include <bpf/bpf_helpers.h>
// #include "bpf_misc.h"

/* sizeof(struct bpf_flow_keys) is well under 4096, so +0x1000 is OOB. */

// SEC("flow_dissector")
// __description("flow_keys: in-bounds constant pointer arithmetic accepted")
// __success
#[unsafe(no_mangle)]
#[unsafe(link_section = "flow_dissector")]
pub unsafe extern "C" fn flow_keys_const_inbounds() {
    core::arch::asm!(
        "r1 = *(u64 *)(r1 + {flow_keys});",
        "r1 += 8;",
        "r0 = *(u64 *)(r1 + 0);",
        "r0 = 0;",
        "exit;",
        flow_keys = const core::mem::offset_of!(__sk_buff, flow_keys),
        options(noreturn)
    );
}

// SEC("flow_dissector")
// __description("flow_keys: OOB via constant pointer arithmetic rejected")
// __failure __msg("invalid access to flow keys off=4096 size=8")
#[unsafe(no_mangle)]
#[unsafe(link_section = "flow_dissector")]
pub unsafe extern "C" fn flow_keys_const_oob_read() {
    core::arch::asm!(
        "r1 = *(u64 *)(r1 + {flow_keys});",
        "r1 += 4096;",
        "r0 = *(u64 *)(r1 + 0);",
        "r0 = 0;",
        "exit;",
        flow_keys = const core::mem::offset_of!(__sk_buff, flow_keys),
        options(noreturn)
    );
}

// SEC("flow_dissector")
// __description("flow_keys: OOB write via constant pointer arithmetic rejected")
// __failure __msg("invalid access to flow keys off=4096 size=8")
#[unsafe(no_mangle)]
#[unsafe(link_section = "flow_dissector")]
pub unsafe extern "C" fn flow_keys_const_oob_write() {
    core::arch::asm!(
        "r1 = *(u64 *)(r1 + {flow_keys});",
        "r1 += 4096;",
        "r2 = 0;",
        "*(u64 *)(r1 + 0) = r2;",
        "r0 = 0;",
        "exit;",
        flow_keys = const core::mem::offset_of!(__sk_buff, flow_keys),
        options(noreturn)
    );
}

/* Equivalent OOB expressed directly in insn->off; this form was always
 * rejected and is kept to show both forms now share one diagnostic.
 */
// SEC("flow_dissector")
// __description("flow_keys: OOB via insn->off rejected")
// __failure __msg("invalid access to flow keys off=4096 size=8")
#[unsafe(no_mangle)]
#[unsafe(link_section = "flow_dissector")]
pub unsafe extern "C" fn flow_keys_insn_off_oob() {
    core::arch::asm!(
        "r1 = *(u64 *)(r1 + {flow_keys});",
        "r0 = *(u64 *)(r1 + 4096);",
        "r0 = 0;",
        "exit;",
        flow_keys = const core::mem::offset_of!(__sk_buff, flow_keys),
        options(noreturn)
    );
}

// SEC("flow_dissector")
// __description("flow_keys: variable pointer arithmetic rejected")
// __failure __msg("R1 pointer arithmetic on flow_keys prohibited")
#[unsafe(no_mangle)]
#[unsafe(link_section = "flow_dissector")]
pub unsafe extern "C" fn flow_keys_var_read() {
    core::arch::asm!(
        "r6 = r1;",
        "call {bpf_get_prandom_u32};",
        "r0 &= 0xFFFF;",
        "r1 = *(u64 *)(r6 + {flow_keys});",
        "r1 += r0;",
        "r0 = *(u64 *)(r1 + 0);",
        "r0 = 0;",
        "exit;",
        flow_keys = const core::mem::offset_of!(__sk_buff, flow_keys),
        bpf_get_prandom_u32 = sym bpf_get_prandom_u32,
        options(noreturn)
    );
}

unsafe extern "C" {
    fn bpf_get_prandom_u32() -> u32;
}

#[unsafe(no_mangle)]
#[unsafe(link_section = "license")]
pub static _license: [u8; 4] = *b"GPL\0";
