// SPDX-License-Identifier: GPL-2.0

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::arch::asm;

// Dependencies from <linux/bpf.h>, <bpf/bpf_helpers.h>, and "bpf_misc.h".
extern "C" {
    fn bpf_ktime_get_ns() -> u64;
    fn bpf_get_prandom_u32() -> u32;
    fn bpf_map_lookup_elem(map: *mut core::ffi::c_void, key: *const core::ffi::c_void) -> *mut core::ffi::c_void;
}

pub const BPF_MAP_TYPE_HASH: u32 = 1;
pub const BPF_F_TEST_STATE_FREQ: u32 = 0;
pub const BPF_F_TEST_RND_HI32: u32 = 0;

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

/* Check that precision marks propagate through scalar IDs.
 * Registers r{0,1,2} have the same scalar ID.
 * Range information is propagated for scalars sharing same ID.
 * Check that precision mark for r0 causes precision marks for r{1,2}
 * when range information is propagated for 'if <reg> <op> <const>' insn.
 */
#[link_section = "socket"]
#[no_mangle]
pub unsafe extern "C" fn linked_regs_bpf_k() {
    asm!(
        "call {bpf_ktime_get_ns}",
        "r0 &= 0xff",
        "r1 = r0",
        "r2 = r0",
        "if r1 > 7 goto +0",
        "r3 = r10",
        "r3 += r0",
        "r1 = r1",
        "r2 = r2",
        "r0 = 0",
        "exit",
        bpf_ktime_get_ns = sym bpf_ktime_get_ns,
        options(noreturn)
    );
}

/* Registers r{0,1,2} share same ID when 'if r1 > ...' insn is processed,
 * check that verifier marks r{1,2} as precise while backtracking
 * 'if r1 > ...' with r0 already marked.
 */
#[link_section = "socket"]
#[no_mangle]
pub unsafe extern "C" fn linked_regs_bpf_x_src() {
    asm!(
        "call {bpf_ktime_get_ns}",
        "r0 &= 0xff",
        "r1 = r0",
        "r2 = r0",
        "r3 = 7",
        "if r1 > r3 goto +0",
        "r4 = r10",
        "r4 += r0",
        "r1 = r1",
        "r2 = r2",
        "r0 = 0",
        "exit",
        bpf_ktime_get_ns = sym bpf_ktime_get_ns,
        options(noreturn)
    );
}

/* Registers r{0,1,2} share same ID when 'if r1 > r3' insn is processed,
 * check that verifier marks r{0,1,2} as precise while backtracking
 * 'if r1 > r3' with r3 already marked.
 */
#[link_section = "socket"]
#[no_mangle]
pub unsafe extern "C" fn linked_regs_bpf_x_dst() {
    asm!(
        "call {bpf_ktime_get_ns}",
        "r0 &= 0xff",
        "r1 = r0",
        "r2 = r0",
        "r3 = 7",
        "if r1 > r3 goto +0",
        "r4 = r10",
        "r4 += r3",
        "r0 = r0",
        "r1 = r1",
        "r2 = r2",
        "r0 = 0",
        "exit",
        bpf_ktime_get_ns = sym bpf_ktime_get_ns,
        options(noreturn)
    );
}

/* Same as linked_regs_bpf_k, but break one of the
 * links, note that r1 is absent from regs=... in __msg below.
 */
#[link_section = "socket"]
#[no_mangle]
pub unsafe extern "C" fn linked_regs_broken_link() {
    asm!(
        "call {bpf_ktime_get_ns}",
        "r0 &= 0xff",
        "r1 = r0",
        "r2 = r0",
        "r1 = 0",
        "if r0 > 7 goto +0",
        "r3 = r10",
        "r3 += r0",
        "r1 = r1",
        "r2 = r2",
        "r0 = 0",
        "exit",
        bpf_ktime_get_ns = sym bpf_ktime_get_ns,
        options(noreturn)
    );
}

/* Check that precision marks propagate through scalar IDs.
 * Use the same scalar ID in multiple stack frames, check that
 * precision information is propagated up the call stack.
 */
#[link_section = "socket"]
#[no_mangle]
pub unsafe extern "C" fn precision_many_frames() {
    asm!(
        "call {bpf_ktime_get_ns}",
        "r0 &= 0xff",
        "r1 = r0",
        "r6 = r0",
        "call precision_many_frames__foo",
        "r6 = r6",
        "exit",
        bpf_ktime_get_ns = sym bpf_ktime_get_ns,
        options(noreturn)
    );
}

#[no_mangle]
pub unsafe extern "C" fn precision_many_frames__foo() {
    asm!(
        "r6 = r1",
        "r7 = r1",
        "call precision_many_frames__bar",
        "r6 = r6",
        "r7 = r7",
        "exit",
        options(noreturn)
    );
}

#[no_mangle]
pub unsafe extern "C" fn precision_many_frames__bar() {
    asm!(
        "if r1 > 7 goto +0",
        "r6 = 0",
        "r7 = 0",
        "r2 = r10",
        "r2 += r1",
        "r0 = 0",
        "exit",
        options(noreturn)
    );
}

/* Check that scalars with the same IDs are marked precise on stack as
 * well as in registers.
 */
#[link_section = "socket"]
#[no_mangle]
pub unsafe extern "C" fn precision_stack() {
    asm!(
        "call {bpf_ktime_get_ns}",
        "r0 &= 0xff",
        "r1 = r0",
        "*(u64*)(r10 - 8) = r1",
        "call precision_stack__foo",
        "r0 = *(u64*)(r10 - 8)",
        "r0 = 0",
        "exit",
        bpf_ktime_get_ns = sym bpf_ktime_get_ns,
        options(noreturn)
    );
}

#[no_mangle]
pub unsafe extern "C" fn precision_stack__foo() {
    asm!(
        "*(u64*)(r10 - 8) = r1",
        "*(u64*)(r10 - 16) = r1",
        "if r1 > 7 goto +0",
        "r2 = r10",
        "r2 += r1",
        "r0 = *(u64*)(r10 - 8)",
        "r0 = *(u64*)(r10 - 16)",
        "exit",
        options(noreturn)
    );
}

/* Use two separate scalar IDs to check that these are propagated
 * independently.
 */
#[link_section = "socket"]
#[no_mangle]
pub unsafe extern "C" fn precision_two_ids() {
    asm!(
        "call {bpf_ktime_get_ns}",
        "r0 &= 0xff",
        "r6 = r0",
        "r7 = r0",
        "call {bpf_ktime_get_ns}",
        "r0 &= 0xff",
        "r8 = r0",
        "r9 = r0",
        "r0 = 0",
        "if r7 > 7 goto +0",
        "if r9 > 7 goto +0",
        "r3 = r10",
        "r3 += r7",
        "r3 += r9",
        "r6 = r6",
        "r8 = r8",
        "exit",
        bpf_ktime_get_ns = sym bpf_ktime_get_ns,
        options(noreturn)
    );
}

#[link_section = "socket"]
#[no_mangle]
pub unsafe extern "C" fn linked_regs_too_many_regs() {
    asm!(
        "call {bpf_ktime_get_ns}",
        "r0 &= 0xff",
        "r1 = r0",
        "r2 = r0",
        "r3 = r0",
        "r4 = r0",
        "r5 = r0",
        "if r0 > 7 goto +0",
        "r1 = r1",
        "r2 = r2",
        "r3 = r3",
        "r4 = r4",
        "r5 = r5",
        "r7 = r10",
        "r7 += r0",
        "r0 = 0",
        "exit",
        bpf_ktime_get_ns = sym bpf_ktime_get_ns,
        options(noreturn)
    );
}

#[link_section = "socket"]
#[no_mangle]
pub unsafe extern "C" fn linked_regs_broken_link_2() {
    asm!(
        "call {bpf_get_prandom_u32}",
        "r7 = r0",
        "r8 = r0",
        "call {bpf_get_prandom_u32}",
        "if r0 > 1 goto +0",
        "if r8 >= r0 goto 1f",
        "r8 += r8",
        "if r7 == 0 goto 1f",
        "r0 /= 0",
        "1:",
        "r0 = 42",
        "exit",
        bpf_get_prandom_u32 = sym bpf_get_prandom_u32,
        options(noreturn)
    );
}

/* Check that mark_chain_precision() for one of the conditional jump
 * operands does not trigger equal scalars precision propagation.
 */
#[link_section = "socket"]
#[no_mangle]
pub unsafe extern "C" fn cjmp_no_linked_regs_trigger() {
    asm!(
        "call {bpf_ktime_get_ns}",
        "r0 &= 0xff",
        "r1 = r0",
        "if r1 > 256 goto +0",
        "r0 = 0",
        "exit",
        bpf_ktime_get_ns = sym bpf_ktime_get_ns,
        options(noreturn)
    );
}

/* Verify that check_ids() is used by regsafe() for scalars. */
#[link_section = "socket"]
#[no_mangle]
pub unsafe extern "C" fn check_ids_in_regsafe() {
    asm!(
        "r1 = 0",
        "*(u64*)(r10 - 8) = r1",
        "r9 = r10",
        "r9 += -8",
        "call {bpf_ktime_get_ns}",
        "r7 = r0",
        "call {bpf_ktime_get_ns}",
        "r6 = r0",
        "if r6 > r7 goto l1_0",
        "r7 = r6",
        "l1_0:",
        "if r7 > 4 goto l2_0",
        "r9 += r6",
        "r0 = *(u8*)(r9 + 0)",
        "l2_0:",
        "r0 = 0",
        "exit",
        bpf_ktime_get_ns = sym bpf_ktime_get_ns,
        options(noreturn)
    );
}

/* Similar to check_ids_in_regsafe. */
#[link_section = "socket"]
#[no_mangle]
pub unsafe extern "C" fn check_ids_in_regsafe_2() {
    asm!(
        "r1 = 0",
        "*(u64*)(r10 - 8) = r1",
        "r9 = r10",
        "r9 += -16",
        "call {bpf_ktime_get_ns}",
        "r8 = r0",
        "call {bpf_ktime_get_ns}",
        "r7 = r0",
        "call {bpf_ktime_get_ns}",
        "r6 = r0",
        "r0 = 0",
        "if r6 > r7 goto l1_1",
        "r6 = r7",
        "l0_1:",
        "if r7 > 4 goto l2_1",
        "r9 += r6",
        "r9 += r7",
        "r9 += r8",
        "r0 = *(u8*)(r9 + 0)",
        "l2_1:",
        "r0 = 0",
        "exit",
        "l1_1:",
        "r6 = r8",
        "goto l0_1",
        bpf_ktime_get_ns = sym bpf_ktime_get_ns,
        options(noreturn)
    );
}

/* Check that scalar IDs *are not* generated on register to register
 * assignments if source register is a constant.
 */
#[link_section = "socket"]
#[no_mangle]
pub unsafe extern "C" fn no_scalar_id_for_const() {
    asm!(
        "call {bpf_ktime_get_ns}",
        "if r0 > 7 goto l0_2",
        "r1 = 0",
        "r1 ^= r1",
        "r1 = r1",
        "r3 = r1",
        "r4 = r1",
        "goto l1_2",
        "l0_2:",
        "r1 = 0",
        "r1 ^= r1",
        "r2 = 0",
        "r2 ^= r2",
        "r3 = r1",
        "r4 = r2",
        "l1_2:",
        "if r3 == r4 goto +0",
        "r0 = 0",
        "exit",
        bpf_ktime_get_ns = sym bpf_ktime_get_ns,
        options(noreturn)
    );
}

/* Same as no_scalar_id_for_const() but for 32-bit values */
#[link_section = "socket"]
#[no_mangle]
pub unsafe extern "C" fn no_scalar_id_for_const32() {
    asm!(
        "call {bpf_ktime_get_ns}",
        "if r0 > 7 goto l0_3",
        "w1 = 0",
        "w1 ^= w1",
        "w1 = w1",
        "w3 = w1",
        "w4 = w1",
        "goto l1_3",
        "l0_3:",
        "w1 = 0",
        "w1 ^= w1",
        "w2 = 0",
        "w2 ^= w2",
        "w3 = w1",
        "w4 = w2",
        "l1_3:",
        "if w3 == w4 goto +0",
        "r0 = 0",
        "exit",
        bpf_ktime_get_ns = sym bpf_ktime_get_ns,
        options(noreturn)
    );
}

/* Check that unique scalar IDs are ignored when new verifier state is
 * compared to cached verifier state. For this test:
 * - cached state has no id on r1
 * - new state has a unique id on r1
 */
#[link_section = "socket"]
#[no_mangle]
pub unsafe extern "C" fn ignore_unique_scalar_ids_cur() {
    asm!(
        "call {bpf_ktime_get_ns}",
        "r6 = r0",
        "call {bpf_ktime_get_ns}",
        "r0 &= 0xff",
        "r1 = r0",
        "r0 = 0",
        "if r6 > 7 goto l0_4",
        "r1 &= 0xff",
        "l0_4:",
        "r2 = r10",
        "r2 += r1",
        "exit",
        bpf_ktime_get_ns = sym bpf_ktime_get_ns,
        options(noreturn)
    );
}

/* Check that unique scalar IDs are ignored when new verifier state is
 * compared to cached verifier state. For this test:
 * - cached state has a unique id on r1
 * - new state has no id on r1
 */
#[link_section = "socket"]
#[no_mangle]
pub unsafe extern "C" fn ignore_unique_scalar_ids_old() {
    asm!(
        "call {bpf_ktime_get_ns}",
        "r6 = r0",
        "call {bpf_ktime_get_ns}",
        "r0 &= 0xff",
        "r1 = r0",
        "r0 = 0",
        "if r6 > 7 goto l1_4",
        "goto l0_5",
        "l1_4:",
        "r1 &= 0xff",
        "l0_5:",
        "r2 = r10",
        "r2 += r1",
        "exit",
        bpf_ktime_get_ns = sym bpf_ktime_get_ns,
        options(noreturn)
    );
}

/* Check that two registers with 0 scalar IDs in a verified state can be mapped
 * to the same scalar ID in current state.
 */
#[link_section = "socket"]
#[no_mangle]
pub unsafe extern "C" fn two_nil_old_ids_one_cur_id() {
    asm!(
        "call {bpf_ktime_get_ns}",
        "r0 &= 0xff",
        "r6 = r0",
        "r6 *= 1",
        "call {bpf_ktime_get_ns}",
        "r0 &= 0xff",
        "r7 = r0",
        "r7 *= 1",
        "r0 = 0",
        "if r6 > r7 goto l0_6",
        "goto l1_5",
        "l0_6:",
        "r6 = r7",
        "l1_5:",
        "r2 = r10",
        "r2 += r6",
        "r2 += r7",
        "exit",
        bpf_ktime_get_ns = sym bpf_ktime_get_ns,
        options(noreturn)
    );
}

/* Check that two different scalar IDs in a verified state can't be
 * mapped to the same scalar ID in current state.
 */
#[link_section = "socket"]
#[no_mangle]
pub unsafe extern "C" fn two_old_ids_one_cur_id() {
    asm!(
        "call {bpf_ktime_get_ns}",
        "r0 &= 0xff",
        "r6 = r0",
        "r8 = r0",
        "call {bpf_ktime_get_ns}",
        "r0 &= 0xff",
        "r7 = r0",
        "r9 = r0",
        "r0 = 0",
        "if r6 > r7 goto l0_7",
        "goto l1_6",
        "l0_7:",
        "r6 = r7",
        "l1_6:",
        "r2 = r10",
        "r2 += r6",
        "r2 += r7",
        "r9 += r8",
        "exit",
        bpf_ktime_get_ns = sym bpf_ktime_get_ns,
        options(noreturn)
    );
}

#[link_section = "socket"]
#[no_mangle]
pub unsafe extern "C" fn linked_regs_and_subreg_def() {
    asm!(
        "call {bpf_ktime_get_ns}",
        "r0 &= 0x7fffffff",
        "w1 = w0",
        "if w0 < 10 goto +0",
        "r1 >>= 32",
        "r0 = r1",
        "exit",
        bpf_ktime_get_ns = sym bpf_ktime_get_ns,
        options(noreturn)
    );
}

/*
 * A scalar is spilled to the stack and then filled twice: once via a
 * sign-extending load (BPF_MEMSX) into r4 and once via a zero-extending
 * load (BPF_MEM) into r5. coerce_reg_to_size_sx() gives r4 a different
 * value than the spilled/zero-extended siblings, so r4 must not keep the
 * shared scalar id. Otherwise the later 'if r5 == 0x80000000' refines r4
 * through sync_linked_regs() to a known 0x80000000, while at runtime r4
 * is the sign-extended 0xffffffff80000000. The test turns that discrepancy
 * into an out-of-bounds map value access (r4 >> 63 is believed 0 but is 1
 * at runtime), which must be rejected.
 */
#[link_section = "socket"]
#[no_mangle]
pub unsafe extern "C" fn ldsx_fill_scalar_id_not_shared() {
    asm!(
        "r1 = 0",
        "*(u64*)(r10 - 8) = r1",
        "r2 = r10",
        "r2 += -8",
        "r1 = {map_hash_8b} ll",
        "call {bpf_map_lookup_elem}",
        "if r0 == 0 goto l0_8",
        "r7 = *(u32*)(r0 + 0)",
        "r2 = 0x80000000 ll",
        "r7 &= r2",
        "r6 = r7",
        "*(u32*)(r10 - 8) = r7",
        "r4 = *(s32*)(r10 - 8)",
        "r5 = *(u32*)(r10 - 8)",
        "if r5 != r2 goto l0_8",
        "r4 >>= 63",
        "r0 += r4",
        "r0 = *(u8*)(r0 + 7)",
        "l0_8:",
        "r0 = 0",
        "exit",
        bpf_map_lookup_elem = sym bpf_map_lookup_elem,
        map_hash_8b = sym map_hash_8b,
        options(noreturn)
    );
}

#[link_section = "license"]
#[no_mangle]
pub static _license: [u8; 4] = *b"GPL\0";

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
