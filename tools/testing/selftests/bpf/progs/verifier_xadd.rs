// SPDX-License-Identifier: GPL-2.0
/* Converted from tools/testing/selftests/bpf/verifier/xadd.c */

/*
 * C dependencies translated as external intent:
 * #include <linux/bpf.h>
 * #include <bpf/bpf_helpers.h>
 * #include "bpf_misc.h"
 */

use core::arch::asm;

extern "C" {
    static bpf_map_lookup_elem: usize;
}

const BPF_MAP_TYPE_HASH: u32 = 1;
const BPF_F_ANY_ALIGNMENT: u32 = 2;

#[repr(C)]
pub struct xdp_md {
    pub data: u32,
    pub data_end: u32,
}

#[repr(C)]
pub struct map_hash_8b_def {
    pub type_: u32,
    pub max_entries: u32,
    pub key: i64,
    pub value: i64,
}

#[link_section = ".maps"]
#[no_mangle]
pub static mut map_hash_8b: map_hash_8b_def = map_hash_8b_def {
    type_: BPF_MAP_TYPE_HASH,
    max_entries: 1,
    key: 0,
    value: 0,
};

// SEC("tc")
// __description("xadd/w check unaligned stack")
// __failure __msg("misaligned stack access off")
#[link_section = "tc"]
#[no_mangle]
pub unsafe extern "C" fn xadd_w_check_unaligned_stack() {
    asm!(
        "r0 = 1",
        "*(u64*)(r10 - 8) = r0",
        "lock *(u32 *)(r10 - 7) += w0",
        "r0 = *(u64*)(r10 - 8)",
        "exit",
        options(noreturn)
    );
}

// SEC("tc")
// __description("xadd/w check unaligned map")
// __failure __msg("misaligned value access off")
#[link_section = "tc"]
#[no_mangle]
pub unsafe extern "C" fn xadd_w_check_unaligned_map() {
    asm!(
        "r1 = 0",
        "*(u64*)(r10 - 8) = r1",
        "r2 = r10",
        "r2 += -8",
        "r1 = {map_hash_8b} ll",
        "call {bpf_map_lookup_elem}",
        "if r0 != 0 goto 0f",
        "exit",
        "0:",
        "r1 = 1",
        "lock *(u32 *)(r0 + 3) += w1",
        "r0 = *(u32*)(r0 + 3)",
        "exit",
        map_hash_8b = sym map_hash_8b,
        bpf_map_lookup_elem = sym bpf_map_lookup_elem,
        options(noreturn)
    );
}

// SEC("xdp")
// __description("xadd/w check unaligned pkt")
// __failure __msg("BPF_ATOMIC stores into R2 pkt is not allowed")
// __flag(BPF_F_ANY_ALIGNMENT)
#[link_section = "xdp"]
#[no_mangle]
pub unsafe extern "C" fn xadd_w_check_unaligned_pkt() {
    const XDP_MD_DATA: usize = core::mem::offset_of!(xdp_md, data);
    const XDP_MD_DATA_END: usize = core::mem::offset_of!(xdp_md, data_end);

    asm!(
        "r2 = *(u32*)(r1 + {xdp_md_data})",
        "r3 = *(u32*)(r1 + {xdp_md_data_end})",
        "r1 = r2",
        "r1 += 8",
        "if r1 < r3 goto 0f",
        "r0 = 99",
        "goto 1f",
        "0:",
        "r0 = 1",
        "r1 = 0",
        "*(u32*)(r2 + 0) = r1",
        "r1 = 0",
        "*(u32*)(r2 + 3) = r1",
        "lock *(u32 *)(r2 + 1) += w0",
        "lock *(u32 *)(r2 + 2) += w0",
        "r0 = *(u32*)(r2 + 1)",
        "1:",
        "exit",
        xdp_md_data = const XDP_MD_DATA,
        xdp_md_data_end = const XDP_MD_DATA_END,
        options(noreturn)
    );
}

// SEC("tc")
// __description("xadd/w check whether src/dst got mangled, 1")
// __success __retval(3)
#[link_section = "tc"]
#[no_mangle]
pub unsafe extern "C" fn src_dst_got_mangled_1() {
    asm!(
        "r0 = 1",
        "r6 = r0",
        "r7 = r10",
        "*(u64*)(r10 - 8) = r0",
        "lock *(u64 *)(r10 - 8) += r0",
        "lock *(u64 *)(r10 - 8) += r0",
        "if r6 != r0 goto 0f",
        "if r7 != r10 goto 0f",
        "r0 = *(u64*)(r10 - 8)",
        "exit",
        "0:",
        "r0 = 42",
        "exit",
        options(noreturn)
    );
}

// SEC("tc")
// __description("xadd/w check whether src/dst got mangled, 2")
// __success __retval(3)
#[link_section = "tc"]
#[no_mangle]
pub unsafe extern "C" fn src_dst_got_mangled_2() {
    asm!(
        "r0 = 1",
        "r6 = r0",
        "r7 = r10",
        "*(u32*)(r10 - 8) = r0",
        "lock *(u32 *)(r10 - 8) += w0",
        "lock *(u32 *)(r10 - 8) += w0",
        "if r6 != r0 goto 0f",
        "if r7 != r10 goto 0f",
        "r0 = *(u32*)(r10 - 8)",
        "exit",
        "0:",
        "r0 = 42",
        "exit",
        options(noreturn)
    );
}

#[link_section = "license"]
#[no_mangle]
pub static mut _license: [u8; 4] = *b"GPL\0";
