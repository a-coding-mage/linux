// SPDX-License-Identifier: GPL-2.0
/* Converted from tools/testing/selftests/bpf/verifier/bounds_mix_sign_unsign.c */

use core::arch::asm;
use core::ffi::c_void;

// Dependencies from the original C includes:
// <linux/bpf.h>, <bpf/bpf_helpers.h>, and "bpf_misc.h".
const BPF_MAP_TYPE_HASH: u32 = 1;

unsafe extern "C" {
    fn bpf_ktime_get_ns() -> u64;
    fn bpf_map_lookup_elem(map: *mut c_void, key: *const c_void) -> *mut c_void;
    fn bpf_skb_load_bytes(skb: *mut c_void, offset: u32, to: *mut c_void, len: u32) -> i64;
}

#[repr(C)]
pub struct MapHash8b {
    pub type_: *const [u32; BPF_MAP_TYPE_HASH as usize],
    pub max_entries: *const [u32; 1],
    pub key: *mut i64,
    pub value: *mut i64,
}

#[unsafe(link_section = ".maps")]
#[unsafe(no_mangle)]
pub static mut map_hash_8b: MapHash8b = MapHash8b {
    type_: core::ptr::null(),
    max_entries: core::ptr::null(),
    key: core::ptr::null_mut(),
    value: core::ptr::null_mut(),
};

// SEC("socket")
// __description("bounds checks mixing signed and unsigned, positive bounds")
// __failure __msg("unbounded min value")
// __failure_unpriv
#[unsafe(no_mangle)]
pub unsafe extern "C" fn signed_and_unsigned_positive_bounds() {
    unsafe {
        asm!(
            "call {bpf_ktime_get_ns}",
            "*(u64*)(r10 - 16) = r0",
            "r1 = 0",
            "*(u64*)(r10 - 8) = r1",
            "r2 = r10",
            "r2 += -8",
            "r1 = {map_hash_8b} ll",
            "call {bpf_map_lookup_elem}",
            "if r0 == 0 goto 0f",
            "r1 = *(u64*)(r10 - 16)",
            "r2 = 2",
            "if r2 >= r1 goto 0f",
            "if r1 s> 4 goto 0f",
            "r0 += r1",
            "r1 = 0",
            "*(u8*)(r0 + 0) = r1",
            "0:",
            "r0 = 0",
            "exit",
            bpf_ktime_get_ns = sym bpf_ktime_get_ns,
            bpf_map_lookup_elem = sym bpf_map_lookup_elem,
            map_hash_8b = sym map_hash_8b,
        );
    }
}

// SEC("socket")
// __description("bounds checks mixing signed and unsigned")
// __failure __msg("unbounded min value")
// __failure_unpriv
#[unsafe(no_mangle)]
pub unsafe extern "C" fn checks_mixing_signed_and_unsigned() {
    unsafe {
        asm!(
            "call {bpf_ktime_get_ns}",
            "*(u64*)(r10 - 16) = r0",
            "r1 = 0",
            "*(u64*)(r10 - 8) = r1",
            "r2 = r10",
            "r2 += -8",
            "r1 = {map_hash_8b} ll",
            "call {bpf_map_lookup_elem}",
            "if r0 == 0 goto 0f",
            "r1 = *(u64*)(r10 - 16)",
            "r2 = -1",
            "if r1 > r2 goto 0f",
            "if r1 s> 1 goto 0f",
            "r0 += r1",
            "r1 = 0",
            "*(u8*)(r0 + 0) = r1",
            "0:",
            "r0 = 0",
            "exit",
            bpf_ktime_get_ns = sym bpf_ktime_get_ns,
            bpf_map_lookup_elem = sym bpf_map_lookup_elem,
            map_hash_8b = sym map_hash_8b,
        );
    }
}

// SEC("socket")
// __description("bounds checks mixing signed and unsigned, variant 2")
// __failure __msg("unbounded min value")
// __failure_unpriv
#[unsafe(no_mangle)]
pub unsafe extern "C" fn signed_and_unsigned_variant_2() {
    unsafe {
        asm!(
            "call {bpf_ktime_get_ns}",
            "*(u64*)(r10 - 16) = r0",
            "r1 = 0",
            "*(u64*)(r10 - 8) = r1",
            "r2 = r10",
            "r2 += -8",
            "r1 = {map_hash_8b} ll",
            "call {bpf_map_lookup_elem}",
            "if r0 == 0 goto 0f",
            "r1 = *(u64*)(r10 - 16)",
            "r2 = -1",
            "if r1 > r2 goto 0f",
            "r8 = 0",
            "r8 += r1",
            "if r8 s> 1 goto 0f",
            "r0 += r8",
            "r0 = 0",
            "*(u8*)(r8 + 0) = r0",
            "0:",
            "r0 = 0",
            "exit",
            bpf_ktime_get_ns = sym bpf_ktime_get_ns,
            bpf_map_lookup_elem = sym bpf_map_lookup_elem,
            map_hash_8b = sym map_hash_8b,
        );
    }
}

// SEC("socket")
// __description("bounds checks mixing signed and unsigned, variant 3")
// __failure __msg("unbounded min value")
// __failure_unpriv
#[unsafe(no_mangle)]
pub unsafe extern "C" fn signed_and_unsigned_variant_3() {
    unsafe {
        asm!(
            "call {bpf_ktime_get_ns}",
            "*(u64*)(r10 - 16) = r0",
            "r1 = 0",
            "*(u64*)(r10 - 8) = r1",
            "r2 = r10",
            "r2 += -8",
            "r1 = {map_hash_8b} ll",
            "call {bpf_map_lookup_elem}",
            "if r0 == 0 goto 0f",
            "r1 = *(u64*)(r10 - 16)",
            "r2 = -1",
            "if r1 > r2 goto 0f",
            "r8 = r1",
            "if r8 s> 1 goto 0f",
            "r0 += r8",
            "r0 = 0",
            "*(u8*)(r8 + 0) = r0",
            "0:",
            "r0 = 0",
            "exit",
            bpf_ktime_get_ns = sym bpf_ktime_get_ns,
            bpf_map_lookup_elem = sym bpf_map_lookup_elem,
            map_hash_8b = sym map_hash_8b,
        );
    }
}

// SEC("socket")
// __description("bounds checks mixing signed and unsigned, variant 4")
// __success __success_unpriv __retval(0)
#[unsafe(no_mangle)]
pub unsafe extern "C" fn signed_and_unsigned_variant_4() {
    unsafe {
        asm!(
            "call {bpf_ktime_get_ns}",
            "*(u64*)(r10 - 16) = r0",
            "r1 = 0",
            "*(u64*)(r10 - 8) = r1",
            "r2 = r10",
            "r2 += -8",
            "r1 = {map_hash_8b} ll",
            "call {bpf_map_lookup_elem}",
            "if r0 == 0 goto 0f",
            "r1 = *(u64*)(r10 - 16)",
            "r2 = 1",
            "r1 &= r2",
            "if r1 s> 1 goto 0f",
            "r0 += r1",
            "r1 = 0",
            "*(u8*)(r0 + 0) = r1",
            "0:",
            "r0 = 0",
            "exit",
            bpf_ktime_get_ns = sym bpf_ktime_get_ns,
            bpf_map_lookup_elem = sym bpf_map_lookup_elem,
            map_hash_8b = sym map_hash_8b,
        );
    }
}

// SEC("socket")
// __description("bounds checks mixing signed and unsigned, variant 5")
// __failure __msg("unbounded min value")
// __failure_unpriv
#[unsafe(no_mangle)]
pub unsafe extern "C" fn signed_and_unsigned_variant_5() {
    unsafe {
        asm!(
            "call {bpf_ktime_get_ns}",
            "*(u64*)(r10 - 16) = r0",
            "r1 = 0",
            "*(u64*)(r10 - 8) = r1",
            "r2 = r10",
            "r2 += -8",
            "r1 = {map_hash_8b} ll",
            "call {bpf_map_lookup_elem}",
            "if r0 == 0 goto 0f",
            "r1 = *(u64*)(r10 - 16)",
            "r2 = -1",
            "if r1 > r2 goto 0f",
            "if r1 s> 1 goto 0f",
            "r0 += 4",
            "r0 -= r1",
            "r1 = 0",
            "*(u8*)(r0 + 0) = r1",
            "r0 = 0",
            "0:",
            "exit",
            bpf_ktime_get_ns = sym bpf_ktime_get_ns,
            bpf_map_lookup_elem = sym bpf_map_lookup_elem,
            map_hash_8b = sym map_hash_8b,
        );
    }
}

// SEC("socket")
// __description("bounds checks mixing signed and unsigned, variant 6")
// __failure __msg("R4 min value is negative, either use unsigned")
// __failure_unpriv
#[unsafe(no_mangle)]
pub unsafe extern "C" fn signed_and_unsigned_variant_6() {
    unsafe {
        asm!(
            "r9 = r1",
            "call {bpf_ktime_get_ns}",
            "*(u64*)(r10 - 16) = r0",
            "r1 = r9",
            "r2 = 0",
            "r3 = r10",
            "r3 += -512",
            "r4 = *(u64*)(r10 - 16)",
            "r6 = -1",
            "if r4 > r6 goto 0f",
            "if r4 s> 1 goto 0f",
            "r4 += 1",
            "r5 = 0",
            "r6 = 0",
            "*(u16*)(r10 - 512) = r6",
            "call {bpf_skb_load_bytes}",
            "0:",
            "r0 = 0",
            "exit",
            bpf_ktime_get_ns = sym bpf_ktime_get_ns,
            bpf_skb_load_bytes = sym bpf_skb_load_bytes,
        );
    }
}

// SEC("socket")
// __description("bounds checks mixing signed and unsigned, variant 7")
// __success __success_unpriv __retval(0)
#[unsafe(no_mangle)]
pub unsafe extern "C" fn signed_and_unsigned_variant_7() {
    unsafe {
        asm!(
            "call {bpf_ktime_get_ns}",
            "*(u64*)(r10 - 16) = r0",
            "r1 = 0",
            "*(u64*)(r10 - 8) = r1",
            "r2 = r10",
            "r2 += -8",
            "r1 = {map_hash_8b} ll",
            "call {bpf_map_lookup_elem}",
            "if r0 == 0 goto 0f",
            "r1 = *(u64*)(r10 - 16)",
            "r2 = {__imm_0}",
            "if r1 > r2 goto 0f",
            "if r1 s> 1 goto 0f",
            "r0 += r1",
            "r1 = 0",
            "*(u8*)(r0 + 0) = r1",
            "0:",
            "r0 = 0",
            "exit",
            bpf_ktime_get_ns = sym bpf_ktime_get_ns,
            bpf_map_lookup_elem = sym bpf_map_lookup_elem,
            map_hash_8b = sym map_hash_8b,
            __imm_0 = const 1024 * 1024 * 1024,
        );
    }
}

// SEC("socket")
// __description("bounds checks mixing signed and unsigned, variant 8")
// __failure __msg("unbounded min value")
// __failure_unpriv
#[unsafe(no_mangle)]
pub unsafe extern "C" fn signed_and_unsigned_variant_8() {
    unsafe {
        asm!(
            "call {bpf_ktime_get_ns}",
            "*(u64*)(r10 - 16) = r0",
            "r1 = 0",
            "*(u64*)(r10 - 8) = r1",
            "r2 = r10",
            "r2 += -8",
            "r1 = {map_hash_8b} ll",
            "call {bpf_map_lookup_elem}",
            "if r0 == 0 goto 0f",
            "r1 = *(u64*)(r10 - 16)",
            "r2 = -1",
            "if r2 > r1 goto 1f",
            "r0 = 0",
            "exit",
            "1:",
            "if r1 s> 1 goto 0f",
            "r0 += r1",
            "r1 = 0",
            "*(u8*)(r0 + 0) = r1",
            "0:",
            "r0 = 0",
            "exit",
            bpf_ktime_get_ns = sym bpf_ktime_get_ns,
            bpf_map_lookup_elem = sym bpf_map_lookup_elem,
            map_hash_8b = sym map_hash_8b,
        );
    }
}

// SEC("socket")
// __description("bounds checks mixing signed and unsigned, variant 9")
// __success __success_unpriv __retval(0)
#[unsafe(no_mangle)]
pub unsafe extern "C" fn signed_and_unsigned_variant_9() {
    unsafe {
        asm!(
            "call {bpf_ktime_get_ns}",
            "*(u64*)(r10 - 16) = r0",
            "r1 = 0",
            "*(u64*)(r10 - 8) = r1",
            "r2 = r10",
            "r2 += -8",
            "r1 = {map_hash_8b} ll",
            "call {bpf_map_lookup_elem}",
            "if r0 == 0 goto 0f",
            "r1 = *(u64*)(r10 - 16)",
            "r2 = -9223372036854775808 ll",
            "if r2 > r1 goto 1f",
            "r0 = 0",
            "exit",
            "1:",
            "if r1 s> 1 goto 0f",
            "r0 += r1",
            "r1 = 0",
            "*(u8*)(r0 + 0) = r1",
            "0:",
            "r0 = 0",
            "exit",
            bpf_ktime_get_ns = sym bpf_ktime_get_ns,
            bpf_map_lookup_elem = sym bpf_map_lookup_elem,
            map_hash_8b = sym map_hash_8b,
        );
    }
}

// SEC("socket")
// __description("bounds checks mixing signed and unsigned, variant 10")
// __failure __msg("unbounded min value")
// __failure_unpriv
#[unsafe(no_mangle)]
pub unsafe extern "C" fn signed_and_unsigned_variant_10() {
    unsafe {
        asm!(
            "call {bpf_ktime_get_ns}",
            "*(u64*)(r10 - 16) = r0",
            "r1 = 0",
            "*(u64*)(r10 - 8) = r1",
            "r2 = r10",
            "r2 += -8",
            "r1 = {map_hash_8b} ll",
            "call {bpf_map_lookup_elem}",
            "if r0 == 0 goto 0f",
            "r1 = *(u64*)(r10 - 16)",
            "r2 = -1",
            "if r2 > r1 goto 1f",
            "r0 = 0",
            "exit",
            "1:",
            "if r1 s> 1 goto 0f",
            "r0 += r1",
            "r1 = 0",
            "*(u8*)(r0 + 0) = r1",
            "0:",
            "r0 = 0",
            "exit",
            bpf_ktime_get_ns = sym bpf_ktime_get_ns,
            bpf_map_lookup_elem = sym bpf_map_lookup_elem,
            map_hash_8b = sym map_hash_8b,
        );
    }
}

// SEC("socket")
// __description("bounds checks mixing signed and unsigned, variant 11")
// __failure __msg("unbounded min value")
// __failure_unpriv
#[unsafe(no_mangle)]
pub unsafe extern "C" fn signed_and_unsigned_variant_11() {
    unsafe {
        asm!(
            "call {bpf_ktime_get_ns}",
            "*(u64*)(r10 - 16) = r0",
            "r1 = 0",
            "*(u64*)(r10 - 8) = r1",
            "r2 = r10",
            "r2 += -8",
            "r1 = {map_hash_8b} ll",
            "call {bpf_map_lookup_elem}",
            "if r0 == 0 goto 0f",
            "r1 = *(u64*)(r10 - 16)",
            "r2 = -1",
            "if r2 >= r1 goto 1f",
            "r0 = 0",
            "exit",
            "1:",
            "if r1 s> 1 goto 0f",
            "r0 += r1",
            "r1 = 0",
            "*(u8*)(r0 + 0) = r1",
            "0:",
            "r0 = 0",
            "exit",
            bpf_ktime_get_ns = sym bpf_ktime_get_ns,
            bpf_map_lookup_elem = sym bpf_map_lookup_elem,
            map_hash_8b = sym map_hash_8b,
        );
    }
}

// SEC("socket")
// __description("bounds checks mixing signed and unsigned, variant 12")
// __failure __msg("unbounded min value")
// __failure_unpriv
#[unsafe(no_mangle)]
pub unsafe extern "C" fn signed_and_unsigned_variant_12() {
    unsafe {
        asm!(
            "call {bpf_ktime_get_ns}",
            "*(u64*)(r10 - 16) = r0",
            "r1 = 0",
            "*(u64*)(r10 - 8) = r1",
            "r2 = r10",
            "r2 += -8",
            "r1 = {map_hash_8b} ll",
            "call {bpf_map_lookup_elem}",
            "if r0 == 0 goto 0f",
            "r1 = *(u64*)(r10 - 16)",
            "r2 = -6",
            "if r2 >= r1 goto 1f",
            "r0 = 0",
            "exit",
            "1:",
            "if r1 s> 1 goto 0f",
            "r0 += r1",
            "r1 = 0",
            "*(u8*)(r0 + 0) = r1",
            "0:",
            "r0 = 0",
            "exit",
            bpf_ktime_get_ns = sym bpf_ktime_get_ns,
            bpf_map_lookup_elem = sym bpf_map_lookup_elem,
            map_hash_8b = sym map_hash_8b,
        );
    }
}

// SEC("socket")
// __description("bounds checks mixing signed and unsigned, variant 13")
// __failure __msg("unbounded min value")
// __failure_unpriv
#[unsafe(no_mangle)]
pub unsafe extern "C" fn signed_and_unsigned_variant_13() {
    unsafe {
        asm!(
            "call {bpf_ktime_get_ns}",
            "*(u64*)(r10 - 16) = r0",
            "r1 = 0",
            "*(u64*)(r10 - 8) = r1",
            "r2 = r10",
            "r2 += -8",
            "r1 = {map_hash_8b} ll",
            "call {bpf_map_lookup_elem}",
            "if r0 == 0 goto 0f",
            "r1 = *(u64*)(r10 - 16)",
            "r2 = 2",
            "if r2 >= r1 goto 0f",
            "r7 = 1",
            "if r7 s> 0 goto 1f",
            "0:",
            "r0 = 0",
            "exit",
            "1:",
            "r7 += r1",
            "if r7 s> 4 goto 2f",
            "r0 += r7",
            "r1 = 0",
            "*(u8*)(r0 + 0) = r1",
            "2:",
            "r0 = 0",
            "exit",
            bpf_ktime_get_ns = sym bpf_ktime_get_ns,
            bpf_map_lookup_elem = sym bpf_map_lookup_elem,
            map_hash_8b = sym map_hash_8b,
        );
    }
}

// SEC("socket")
// __description("bounds checks mixing signed and unsigned, variant 14")
// __failure __msg("unbounded min value")
// __failure_unpriv
#[unsafe(no_mangle)]
pub unsafe extern "C" fn signed_and_unsigned_variant_14() {
    unsafe {
        asm!(
            "r9 = *(u32*)(r1 + {__sk_buff_mark})",
            "call {bpf_ktime_get_ns}",
            "*(u64*)(r10 - 16) = r0",
            "r1 = 0",
            "*(u64*)(r10 - 8) = r1",
            "r2 = r10",
            "r2 += -8",
            "r1 = {map_hash_8b} ll",
            "call {bpf_map_lookup_elem}",
            "if r0 == 0 goto 0f",
            "r1 = *(u64*)(r10 - 16)",
            "r2 = -1",
            "r8 = 2",
            "if r9 == 42 goto 1f",
            "if r8 s> r1 goto 2f",
            "3:",
            "if r1 s> 1 goto 2f",
            "r0 += r1",
            "0:",
            "r1 = 0",
            "*(u8*)(r0 + 0) = r1",
            "2:",
            "r0 = 0",
            "exit",
            "1:",
            "if r1 > r2 goto 2f",
            "goto 3b",
            bpf_ktime_get_ns = sym bpf_ktime_get_ns,
            bpf_map_lookup_elem = sym bpf_map_lookup_elem,
            map_hash_8b = sym map_hash_8b,
            __sk_buff_mark = const 8,
        );
    }
}

// SEC("socket")
// __description("bounds checks mixing signed and unsigned, variant 15")
// __failure __msg("unbounded min value")
// __failure_unpriv
#[unsafe(no_mangle)]
pub unsafe extern "C" fn signed_and_unsigned_variant_15() {
    unsafe {
        asm!(
            "call {bpf_ktime_get_ns}",
            "*(u64*)(r10 - 16) = r0",
            "r1 = 0",
            "*(u64*)(r10 - 8) = r1",
            "r2 = r10",
            "r2 += -8",
            "r1 = {map_hash_8b} ll",
            "call {bpf_map_lookup_elem}",
            "if r0 == 0 goto 0f",
            "r1 = *(u64*)(r10 - 16)",
            "r2 = -6",
            "if r2 >= r1 goto 1f",
            "0:",
            "r0 = 0",
            "exit",
            "1:",
            "r0 += r1",
            "if r0 > 1 goto 2f",
            "r0 = 0",
            "exit",
            "2:",
            "r1 = 0",
            "*(u8*)(r0 + 0) = r1",
            "r0 = 0",
            "exit",
            bpf_ktime_get_ns = sym bpf_ktime_get_ns,
            bpf_map_lookup_elem = sym bpf_map_lookup_elem,
            map_hash_8b = sym map_hash_8b,
        );
    }
}

#[unsafe(link_section = "license")]
#[unsafe(no_mangle)]
pub static _license: [u8; 4] = *b"GPL\0";
