// SPDX-License-Identifier: GPL-2.0
/* Converted from tools/testing/selftests/bpf/verifier/raw_tp_writable.c */

// C includes translated as dependency intent:
// #include <linux/bpf.h>
// #include <bpf/bpf_helpers.h>
// #include "bpf_misc.h"

#![allow(non_upper_case_globals)]

unsafe extern "C" {
    fn bpf_map_lookup_elem(map: *mut core::ffi::c_void, key: *const core::ffi::c_void) -> *mut core::ffi::c_void;
}

const BPF_MAP_TYPE_HASH: u32 = 1;
const BPF_F_ANY_ALIGNMENT: u32 = 2;

#[repr(C)]
pub struct MapHash8b {
    // __uint(type, BPF_MAP_TYPE_HASH);
    pub type_: u32,
    // __uint(max_entries, 1);
    pub max_entries: u32,
    // __type(key, long long);
    // __type(value, long long);
}

#[unsafe(link_section = ".maps")]
#[unsafe(no_mangle)]
pub static mut map_hash_8b: MapHash8b = MapHash8b {
    type_: BPF_MAP_TYPE_HASH,
    max_entries: 1,
};

#[unsafe(link_section = "raw_tracepoint.w")]
#[unsafe(no_mangle)]
// __description("raw_tracepoint_writable: reject variable offset")
// __failure
// __msg("R6 invalid variable buffer offset: off=0, var_off=(0x0; 0xffffffff)")
// __flag(BPF_F_ANY_ALIGNMENT)
pub unsafe extern "C" fn tracepoint_writable_reject_variable_offset() {
    core::arch::asm!(
        "/* r6 is our tp buffer */",
        "r6 = *(u64*)(r1 + 0)",
        "r1 = {map_hash_8b} ll",
        "/* move the key (== 0) to r10-8 */",
        "w0 = 0",
        "r2 = r10",
        "r2 += -8",
        "*(u64*)(r2 + 0) = r0",
        "/* lookup in the map */",
        "call {bpf_map_lookup_elem}",
        "/* exit clean if null */",
        "if r0 != 0 goto 0f",
        "exit",
        "0:",
        "/* shift the buffer pointer to a variable location */",
        "r0 = *(u32*)(r0 + 0)",
        "r6 += r0",
        "/* clobber whatever's there */",
        "r7 = 4242",
        "*(u64*)(r6 + 0) = r7",
        "r0 = 0",
        "exit",
        map_hash_8b = sym map_hash_8b,
        bpf_map_lookup_elem = sym bpf_map_lookup_elem,
        options(noreturn)
    );
}

#[unsafe(link_section = "raw_tracepoint.w")]
#[unsafe(no_mangle)]
// __description("raw_tracepoint_writable: reject negative const offset")
// __failure
// __msg("invalid negative tracepoint buffer offset")
pub unsafe extern "C" fn tracepoint_writable_reject_negative_const_offset() {
    core::arch::asm!(
        "r6 = *(u64 *)(r1 + 0)",
        "r6 += -8",
        "r0 = *(u64 *)(r6 + 0)",
        "exit",
        options(noreturn)
    );
}

#[unsafe(link_section = "license")]
#[unsafe(no_mangle)]
pub static _license: [u8; 4] = *b"GPL\0";

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
