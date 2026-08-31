// SPDX-License-Identifier: GPL-2.0
/* Converted from tools/testing/selftests/bpf/verifier/leak_ptr.c */

// C includes translated as external dependency intent:
// <linux/bpf.h>, <bpf/bpf_helpers.h>, "bpf_misc.h"

#[repr(C)]
pub struct map_hash_8b {
    // __uint(type, BPF_MAP_TYPE_HASH);
    // __uint(max_entries, 1);
    // __type(key, long long);
    // __type(value, long long);
    _private: [u8; 0],
}

#[link_section = ".maps"]
#[no_mangle]
pub static mut map_hash_8b: map_hash_8b = map_hash_8b { _private: [] };

#[link_section = "socket"]
#[no_mangle]
// __description("leak pointer into ctx 1")
// __failure
// __msg("BPF_ATOMIC stores into R1 ctx is not allowed")
// __failure_unpriv
// __msg_unpriv("R2 leaks addr into mem")
// __naked
pub unsafe extern "C" fn leak_pointer_into_ctx_1() {
    core::arch::asm!(
        "                   ",
        "r0 = 0;            ",
        "*(u64*)(r1 + {__sk_buff_cb_0}) = r0;",
        "r2 = {map_hash_8b} ll;",
        "lock *(u64 *)(r1 + {__sk_buff_cb_0}) += r2;",
        "exit;              ",
        __sk_buff_cb_0 = const __sk_buff_cb_0,
        map_hash_8b = sym map_hash_8b,
        options(noreturn)
    );
}

#[link_section = "socket"]
#[no_mangle]
// __description("leak pointer into ctx 2")
// __failure
// __msg("BPF_ATOMIC stores into R1 ctx is not allowed")
// __failure_unpriv
// __msg_unpriv("R10 leaks addr into mem")
// __naked
pub unsafe extern "C" fn leak_pointer_into_ctx_2() {
    core::arch::asm!(
        "                   ",
        "r0 = 0;            ",
        "*(u64*)(r1 + {__sk_buff_cb_0}) = r0;",
        "lock *(u64 *)(r1 + {__sk_buff_cb_0}) += r10;",
        "exit;              ",
        __sk_buff_cb_0 = const __sk_buff_cb_0,
        options(noreturn)
    );
}

#[link_section = "socket"]
#[no_mangle]
// __description("leak pointer into ctx 3")
// __success
// __failure_unpriv
// __msg_unpriv("R2 leaks addr into ctx")
// __retval(0)
// __naked
pub unsafe extern "C" fn leak_pointer_into_ctx_3() {
    core::arch::asm!(
        "                   ",
        "r0 = 0;            ",
        "r2 = {map_hash_8b} ll;",
        "*(u64*)(r1 + {__sk_buff_cb_0}) = r2;",
        "exit;              ",
        __sk_buff_cb_0 = const __sk_buff_cb_0,
        map_hash_8b = sym map_hash_8b,
        options(noreturn)
    );
}

#[link_section = "socket"]
#[no_mangle]
// __description("leak pointer into map val")
// __success
// __failure_unpriv
// __msg_unpriv("R6 leaks addr into mem")
// __retval(0)
// __naked
pub unsafe extern "C" fn leak_pointer_into_map_val() {
    core::arch::asm!(
        "                   ",
        "r6 = r1;           ",
        "r1 = 0;            ",
        "*(u64*)(r10 - 8) = r1;",
        "r2 = r10;          ",
        "r2 += -8;          ",
        "r1 = {map_hash_8b} ll;",
        "call {bpf_map_lookup_elem};",
        "if r0 == 0 goto 0f;",
        "r3 = 0;            ",
        "*(u64*)(r0 + 0) = r3;",
        "lock *(u64 *)(r0 + 0) += r6;",
        "0: r0 = 0;         ",
        "exit;              ",
        map_hash_8b = sym map_hash_8b,
        bpf_map_lookup_elem = sym bpf_map_lookup_elem,
        options(noreturn)
    );
}

extern "C" {
    fn bpf_map_lookup_elem() -> i64;
}

#[link_section = "license"]
#[no_mangle]
pub static mut _license: [u8; 4] = *b"GPL\0";

