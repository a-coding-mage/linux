// SPDX-License-Identifier: GPL-2.0

use core::arch::asm;

extern "C" {
    fn bpf_get_prandom_u32() -> u32;
    fn bpf_iter_num_new(iter: *mut bpf_iter_num, start: i32, end: i32) -> i32;
    fn bpf_iter_num_next(iter: *mut bpf_iter_num) -> *mut i32;
    fn bpf_iter_num_destroy(iter: *mut bpf_iter_num);
    fn __sink<T>(arg: T);
}

#[repr(C)]
pub struct bpf_iter_num {
    _private: [u8; 0],
}

const INT_MIN: i32 = i32::MIN;
const INT_MAX: i32 = i32::MAX;
const BPF_F_TEST_STATE_FREQ: u32 = 1;

// SEC("socket")
// __description("scalars: find linked scalars")
// __failure
// __msg("math between fp pointer and 2147483647 is not allowed")
// __naked
pub unsafe extern "C" fn scalars() {
    asm!(
        "r0 = 0;",
        "r1 = 0x80000001 ll;",
        "r1 /= 1;",
        "r2 = r1;",
        "r4 = r1;",
        "w2 += 0x7FFFFFFF;",
        "w4 += 0;",
        "if r2 == 0 goto l0_{0%=};",
        "exit;",
        "l0_{0%=}:",
        "r4 >>= 63;",
        "r3 = 1;",
        "r3 -= r4;",
        "r3 *= 0x7FFFFFFF;",
        "r3 += r10;",
        "*(u8*)(r3 - 1) = r0;",
        "exit;",
    );
}

/*
 * Test that sync_linked_regs() preserves register IDs.
 *
 * The sync_linked_regs() function copies bounds from known_reg to linked
 * registers. When doing so, it must preserve each register's original id
 * to allow subsequent syncs from the same source to work correctly.
 *
 */
// SEC("socket")
// __success
// __naked
pub unsafe extern "C" fn sync_linked_regs_preserves_id() {
    asm!(
        "call {bpf_get_prandom_u32};",
        "r0 &= 0xff;",
        "r1 = r0;",
        "r1 += 4;",
        "if r1 < 10 goto l0_{0%=};",
        "r2 = r0;",
        "if r1 < 14 goto l0_{0%=};",
        "if r0 >= 10 goto l0_{0%=};",
        "r0 /= 0;",
        "l0_{0%=}:",
        "r0 = 0;",
        "exit;",
        bpf_get_prandom_u32 = sym bpf_get_prandom_u32,
    );
}

// SEC("socket")
// __success
// __naked
pub unsafe extern "C" fn scalars_neg() {
    asm!(
        "call {bpf_get_prandom_u32};",
        "r0 &= 0xff;",
        "r1 = r0;",
        "r1 += -4;",
        "if r1 s< 0 goto l0_{0%=};",
        "if r0 != 0 goto l0_{0%=};",
        "r0 /= 0;",
        "l0_{0%=}:",
        "r0 = 0;",
        "exit;",
        bpf_get_prandom_u32 = sym bpf_get_prandom_u32,
    );
}

/* Same test but using BPF_SUB instead of BPF_ADD with negative immediate */
// SEC("socket")
// __success
// __naked
pub unsafe extern "C" fn scalars_neg_sub() {
    asm!(
        "call {bpf_get_prandom_u32};",
        "r0 &= 0xff;",
        "r1 = r0;",
        "r1 -= 4;",
        "if r1 s< 0 goto l0_{0%=};",
        "if r0 != 0 goto l0_{0%=};",
        "r0 /= 0;",
        "l0_{0%=}:",
        "r0 = 0;",
        "exit;",
        bpf_get_prandom_u32 = sym bpf_get_prandom_u32,
    );
}

/* alu32 with negative offset */
// SEC("socket")
// __success
// __naked
pub unsafe extern "C" fn scalars_neg_alu32_add() {
    asm!(
        "call {bpf_get_prandom_u32};",
        "w0 &= 0xff;",
        "w1 = w0;",
        "w1 += -4;",
        "if w1 s< 0 goto l0_{0%=};",
        "if w0 != 0 goto l0_{0%=};",
        "r0 /= 0;",
        "l0_{0%=}:",
        "r0 = 0;",
        "exit;",
        bpf_get_prandom_u32 = sym bpf_get_prandom_u32,
    );
}

/* alu32 with negative offset using SUB */
// SEC("socket")
// __success
// __naked
pub unsafe extern "C" fn scalars_neg_alu32_sub() {
    asm!(
        "call {bpf_get_prandom_u32};",
        "w0 &= 0xff;",
        "w1 = w0;",
        "w1 -= 4;",
        "if w1 s< 0 goto l0_{0%=};",
        "if w0 != 0 goto l0_{0%=};",
        "r0 /= 0;",
        "l0_{0%=}:",
        "r0 = 0;",
        "exit;",
        bpf_get_prandom_u32 = sym bpf_get_prandom_u32,
    );
}

/* Positive offset: r1 = r0 + 4, then if r1 >= 6, r0 >= 2, so r0 != 0 */
// SEC("socket")
// __success
// __naked
pub unsafe extern "C" fn scalars_pos() {
    asm!(
        "call {bpf_get_prandom_u32};",
        "r0 &= 0xff;",
        "r1 = r0;",
        "r1 += 4;",
        "if r1 < 6 goto l0_{0%=};",
        "if r0 != 0 goto l0_{0%=};",
        "r0 /= 0;",
        "l0_{0%=}:",
        "r0 = 0;",
        "exit;",
        bpf_get_prandom_u32 = sym bpf_get_prandom_u32,
    );
}

/* SUB with negative immediate: r1 -= -4 is equivalent to r1 += 4 */
// SEC("socket")
// __success
// __naked
pub unsafe extern "C" fn scalars_sub_neg_imm() {
    asm!(
        "call {bpf_get_prandom_u32};",
        "r0 &= 0xff;",
        "r1 = r0;",
        "r1 -= -4;",
        "if r1 < 6 goto l0_{0%=};",
        "if r0 != 0 goto l0_{0%=};",
        "r0 /= 0;",
        "l0_{0%=}:",
        "r0 = 0;",
        "exit;",
        bpf_get_prandom_u32 = sym bpf_get_prandom_u32,
    );
}

/* Double ADD clears the ID (can't accumulate offsets) */
// SEC("socket")
// __failure
// __msg("div by zero")
// __naked
pub unsafe extern "C" fn scalars_double_add() {
    asm!(
        "call {bpf_get_prandom_u32};",
        "r0 &= 0xff;",
        "r1 = r0;",
        "r1 += 2;",
        "r1 += 2;",
        "if r1 < 6 goto l0_{0%=};",
        "if r0 != 0 goto l0_{0%=};",
        "r0 /= 0;",
        "l0_{0%=}:",
        "r0 = 0;",
        "exit;",
        bpf_get_prandom_u32 = sym bpf_get_prandom_u32,
    );
}

/*
 * Test that sync_linked_regs() correctly handles large offset differences.
 * r1.off = S32_MIN, r2.off = 1, delta = S32_MIN - 1 requires 64-bit math.
 */
// SEC("socket")
// __success
// __naked
pub unsafe extern "C" fn scalars_sync_delta_overflow() {
    asm!(
        "call {bpf_get_prandom_u32};",
        "r0 &= 0xff;",
        "r1 = r0;",
        "r2 = r0;",
        "r1 += {s32_min};",
        "r2 += 1;",
        "if r2 s< 100 goto l0_{0%=};",
        "if r1 s< 0 goto l0_{0%=};",
        "r0 /= 0;",
        "l0_{0%=}:",
        "r0 = 0;",
        "exit;",
        bpf_get_prandom_u32 = sym bpf_get_prandom_u32,
        s32_min = const INT_MIN,
    );
}

/*
 * Another large delta case: r1.off = S32_MAX, r2.off = -1.
 * delta = S32_MAX - (-1) = S32_MAX + 1 requires 64-bit math.
 */
// SEC("socket")
// __success
// __naked
pub unsafe extern "C" fn scalars_sync_delta_overflow_large_range() {
    asm!(
        "call {bpf_get_prandom_u32};",
        "r0 &= 0xff;",
        "r1 = r0;",
        "r2 = r0;",
        "r1 += {s32_max};",
        "r2 += -1;",
        "if r2 s< 0 goto l0_{0%=};",
        "if r1 s>= 0 goto l0_{0%=};",
        "r0 /= 0;",
        "l0_{0%=}:",
        "r0 = 0;",
        "exit;",
        bpf_get_prandom_u32 = sym bpf_get_prandom_u32,
        s32_max = const INT_MAX,
    );
}

/*
 * Test linked scalar tracking with alu32 and large positive offset (0x7FFFFFFF).
 * After w1 += 0x7FFFFFFF, w1 wraps to negative for any r0 >= 1.
 * If w1 is signed-negative, then r0 >= 1, so r0 != 0.
 */
// SEC("socket")
// __success
// __naked
pub unsafe extern "C" fn scalars_alu32_big_offset() {
    asm!(
        "call {bpf_get_prandom_u32};",
        "w0 &= 0xff;",
        "w1 = w0;",
        "w1 += 0x7FFFFFFF;",
        "if w1 s>= 0 goto l0_{0%=};",
        "if w0 != 0 goto l0_{0%=};",
        "r0 /= 0;",
        "l0_{0%=}:",
        "r0 = 0;",
        "exit;",
        bpf_get_prandom_u32 = sym bpf_get_prandom_u32,
    );
}

// SEC("socket")
// __failure
// __msg("div by zero")
// __naked
pub unsafe extern "C" fn scalars_alu32_basic() {
    asm!(
        "call {bpf_get_prandom_u32};",
        "r1 = r0;",
        "w1 += 1;",
        "if r1 > 10 goto 1f;",
        "r0 >>= 32;",
        "if r0 == 0 goto 1f;",
        "r0 /= 0;",
        "1:",
        "r0 = 0;",
        "exit;",
        bpf_get_prandom_u32 = sym bpf_get_prandom_u32,
    );
}

/*
 * Test alu32 linked register tracking with wrapping.
 * R0 is bounded to [0xffffff00, 0xffffffff] (high 32-bit values)
 * w1 += 0x100 causes R1 to wrap to [0, 0xff]
 *
 * After sync_linked_regs, if bounds are computed correctly:
 *   R0 should be [0x00000000_ffffff00, 0x00000000_ffffff80]
 *   R0 >> 32 == 0, so div by zero is unreachable
 *
 * If bounds are computed incorrectly (64-bit underflow):
 *   R0 becomes [0xffffffff_ffffff00, 0xffffffff_ffffff80]
 *   R0 >> 32 == 0xffffffff != 0, so div by zero is reachable
 */
// SEC("socket")
// __success
// __naked
pub unsafe extern "C" fn scalars_alu32_wrap() {
    asm!(
        "call {bpf_get_prandom_u32};",
        "w0 |= 0xffffff00;",
        "r1 = r0;",
        "w1 += 0x100;",
        "if r1 > 0x80 goto l0_{0%=};",
        "r2 = r0;",
        "r2 >>= 32;",
        "if r2 == 0 goto l0_{0%=};",
        "r0 /= 0;",
        "l0_{0%=}:",
        "r0 = 0;",
        "exit;",
        bpf_get_prandom_u32 = sym bpf_get_prandom_u32,
    );
}

/*
 * Test that sync_linked_regs() checks reg->id (the linked target register)
 * for BPF_ADD_CONST32 rather than known_reg->id (the branch register).
 */
// SEC("socket")
// __success
// __naked
pub unsafe extern "C" fn scalars_alu32_zext_linked_reg() {
    asm!(
        "call {bpf_get_prandom_u32};",
        "w6 = w0;",
        "r7 = r6;",
        "w7 += 1;",
        "r8 = 0xFFFFffff ll;",
        "if r6 < r8 goto l0_{0%=};",
        "r7 >>= 32;",
        "if r7 == 0 goto l0_{0%=};",
        "r0 /= 0;",
        "l0_{0%=}:",
        "r0 = 0;",
        "exit;",
        bpf_get_prandom_u32 = sym bpf_get_prandom_u32,
    );
}

/*
 * Test that sync_linked_regs() skips propagation when one register used
 * alu32 (BPF_ADD_CONST32) and the other used alu64 (BPF_ADD_CONST64).
 * The delta relationship doesn't hold across different ALU widths.
 */
// SEC("socket")
// __failure __msg("div by zero")
// __naked
pub unsafe extern "C" fn scalars_alu32_alu64_cross_type() {
    asm!(
        "call {bpf_get_prandom_u32};",
        "w6 = w0;",
        "r7 = r6;",
        "w7 += 1;",
        "r8 = r6;",
        "r8 += 2;",
        "r9 = 0xFFFFffff ll;",
        "if r7 < r9 goto l0_{0%=};",
        "if r8 > 0 goto l1_{0%=};",
        "goto l0_{0%=};",
        "l1_{0%=}:",
        "r0 /= 0;",
        "l0_{0%=}:",
        "r0 = 0;",
        "exit;",
        bpf_get_prandom_u32 = sym bpf_get_prandom_u32,
    );
}

/*
 * Test that regsafe() prevents pruning when two paths reach the same program
 * point with linked registers carrying different ADD_CONST flags (one
 * BPF_ADD_CONST32 from alu32, another BPF_ADD_CONST64 from alu64).
 */
// SEC("socket")
// __failure __msg("div by zero")
// __flag(BPF_F_TEST_STATE_FREQ)
// __naked
pub unsafe extern "C" fn scalars_alu32_alu64_regsafe_pruning() {
    asm!(
        "call {bpf_get_prandom_u32};",
        "w6 = w0;",
        "r7 = r6;",
        "call {bpf_get_prandom_u32};",
        "if r0 > 0 goto l_pathb_{0%=};",
        "w7 += 1;",
        "goto l_merge_{0%=};",
        "l_pathb_{0%=}:",
        "r7 += 1;",
        "l_merge_{0%=}:",
        "r9 = 0xFFFFffff ll;",
        "if r6 < r9 goto l0_{0%=};",
        "r7 >>= 32;",
        "if r7 == 0 goto l0_{0%=};",
        "r0 /= 0;",
        "l0_{0%=}:",
        "r0 = 0;",
        "exit;",
        bpf_get_prandom_u32 = sym bpf_get_prandom_u32,
    );
}

// SEC("socket")
// __success
pub unsafe extern "C" fn alu32_negative_offset() {
    let mut path: [i8; 5] = [0; 5];
    let offset: i32 = bpf_get_prandom_u32() as i32;
    let off: i32 = offset;

    if off >= 5 && off < 10 {
        core::ptr::write_volatile(path.as_mut_ptr().offset((off - 5) as isize), b'.' as i8);
    }

    /* So compiler doesn't say: error: variable 'path' set but not used */
    __sink(core::ptr::read_volatile(path.as_ptr()));
}

pub unsafe extern "C" fn dummy_calls() {
    bpf_iter_num_new(core::ptr::null_mut(), 0, 0);
    bpf_iter_num_next(core::ptr::null_mut());
    bpf_iter_num_destroy(core::ptr::null_mut());
}

// SEC("socket")
// __success
// __flag(BPF_F_TEST_STATE_FREQ)
pub unsafe extern "C" fn spurious_precision_marks(ctx: *mut core::ffi::c_void) -> i32 {
    let mut iter = core::mem::MaybeUninit::<bpf_iter_num>::uninit();

    asm!(
        "r1 = {iter};",
        "r2 = 0;",
        "r3 = 10;",
        "call {bpf_iter_num_new};",
        "1:",
        "r1 = {iter};",
        "call {bpf_iter_num_next};",
        "if r0 == 0 goto 4f;",
        "r7 = *(u32 *)(r0 + 0);",
        "r8 = *(u32 *)(r0 + 0);",
        /* This jump can't be predicted and does not change r7 or r8 state. */
        "if r7 > r8 goto 2f;",
        /* Branch explored first ties r2 and r7 as having the same id. */
        "r2 = r7;",
        "goto 3f;",
        "2:",
        /* Branch explored second does not tie r2 and r7 but has a function call. */
        "call {bpf_get_prandom_u32};",
        "3:",
        /*
         * A checkpoint.
         * When first branch is explored, this would inject linked registers
         * r2 and r7 into the jump history.
         * When second branch is explored, this would be a cache hit point,
         * triggering propagate_precision().
         */
        "if r7 <= 42 goto +0;",
        /*
         * Mark r7 as precise using an if condition that is always true.
         * When reached via the second branch, this triggered a bug in the backtrack_insn()
         * because r2 (tied to r7) was propagated as precise to a call.
         */
        "if r7 <= 0xffffFFFF goto +0;",
        "goto 1b;",
        "4:",
        "r1 = {iter};",
        "call {bpf_iter_num_destroy};",
        iter = in(reg) iter.as_mut_ptr(),
        bpf_iter_num_new = sym bpf_iter_num_new,
        bpf_iter_num_next = sym bpf_iter_num_next,
        bpf_iter_num_destroy = sym bpf_iter_num_destroy,
        bpf_get_prandom_u32 = sym bpf_get_prandom_u32,
    );

    let _ = ctx;
    0
}

/*
 * Test that r += r (self-add, src_reg == dst_reg) clears the scalar ID
 * so that sync_linked_regs() does not propagate an incorrect delta.
 */
// SEC("socket")
// __failure
// __msg("div by zero")
// __naked
pub unsafe extern "C" fn scalars_self_add_clears_id() {
    asm!(
        "call {bpf_get_prandom_u32};",
        "r6 = r0;",
        "r7 = r6;",
        "call {bpf_get_prandom_u32};",
        "r8 = r0;",
        "r9 = r8;",
        "if r7 != 1 goto l_exit_{0%=};",
        "r6 += r6;",
        "if r7 == r9 goto l_exit_{0%=};",
        "if r6 == 3 goto l_exit_{0%=};",
        "r0 /= 0;",
        "l_exit_{0%=}:",
        "r0 = 0;",
        "exit;",
        bpf_get_prandom_u32 = sym bpf_get_prandom_u32,
    );
}

/* Same as above but with alu32 such that w6 += w6 also clears id. */
// SEC("socket")
// __failure
// __msg("div by zero")
// __naked
pub unsafe extern "C" fn scalars_self_add_alu32_clears_id() {
    asm!(
        "call {bpf_get_prandom_u32};",
        "w6 = w0;",
        "w7 = w6;",
        "call {bpf_get_prandom_u32};",
        "w8 = w0;",
        "w9 = w8;",
        "if w7 != 1 goto l_exit_{0%=};",
        "w6 += w6;",
        "if w7 == w9 goto l_exit_{0%=};",
        "if w6 == 3 goto l_exit_{0%=};",
        "r0 /= 0;",
        "l_exit_{0%=}:",
        "r0 = 0;",
        "exit;",
        bpf_get_prandom_u32 = sym bpf_get_prandom_u32,
    );
}

/*
 * Test that stale delta from a cleared BPF_ADD_CONST does not leak
 * through assign_scalar_id_before_mov() into a new id, causing
 * sync_linked_regs() to compute an incorrect offset.
 */
// SEC("socket")
// __failure
// __msg("div by zero")
// __naked
pub unsafe extern "C" fn scalars_stale_delta_from_cleared_id() {
    asm!(
        "call {bpf_get_prandom_u32};",
        "r6 = r0;",
        "r6 += 5;",
        "r6 ^= 0;",
        "r8 = r6;",
        "r8 += 3;",
        "r9 = r6;",
        "if r9 != 10 goto l_exit_{0%=};",
        "if r8 == 8 goto l_exit_{0%=};",
        "r0 /= 0;",
        "l_exit_{0%=}:",
        "r0 = 0;",
        "exit;",
        bpf_get_prandom_u32 = sym bpf_get_prandom_u32,
    );
}

/* Same as above but with alu32. */
// SEC("socket")
// __failure
// __msg("div by zero")
// __naked
pub unsafe extern "C" fn scalars_stale_delta_from_cleared_id_alu32() {
    asm!(
        "call {bpf_get_prandom_u32};",
        "w6 = w0;",
        "w6 += 5;",
        "w6 ^= 0;",
        "w8 = w6;",
        "w8 += 3;",
        "w9 = w6;",
        "if w9 != 10 goto l_exit_{0%=};",
        "if w8 == 8 goto l_exit_{0%=};",
        "r0 /= 0;",
        "l_exit_{0%=}:",
        "r0 = 0;",
        "exit;",
        bpf_get_prandom_u32 = sym bpf_get_prandom_u32,
    );
}

/*
 * Test that regsafe() verifies base_id consistency for BPF_ADD_CONST
 * linked scalars during state pruning.
 *
 * The false branch (explored first) links R3 to R2 via ADD_CONST.
 * The true branch (runtime path) links R3 to R4 (unrelated base_id).
 * At the merge point, pruning must fail because the linkage topology
 * differs.
 */
// SEC("socket")
// __description("linked scalars: add_const base_id must be consistent for pruning")
// __failure __msg("invalid variable-offset")
// __flag(BPF_F_TEST_STATE_FREQ)
// __naked
pub unsafe extern "C" fn add_const_base_id_pruning() {
    asm!(
        "r1 = 0;",
        "*(u64*)(r10 - 16) = r1;",
        "call {bpf_get_prandom_u32};",
        "r6 = r0;",
        "r6 &= 1;",
        "if r6 >= 1 goto l_true_{0%=};",
        "call {bpf_get_prandom_u32};",
        "r2 = r0;",
        "r2 &= 0xff;",
        "r3 = r2;",
        "r3 += 10;",
        "r6 = 0;",
        "goto l_merge_{0%=};",
        "l_true_{0%=}:",
        "call {bpf_get_prandom_u32};",
        "r2 = r0;",
        "r2 &= 0xff;",
        "r4 = r0;",
        "r4 &= 0xff;",
        "r3 = r4;",
        "r3 += 10;",
        "r6 = 0;",
        "l_merge_{0%=}:",
        "if r2 >= 6 goto l_exit_{0%=};",
        "r3 -= 10;",
        "r9 = r10;",
        "r9 += -16;",
        "r9 += r3;",
        "*(u8*)(r9 + 0) = r6;",
        "l_exit_{0%=}:",
        "r0 = 0;",
        "exit;",
        bpf_get_prandom_u32 = sym bpf_get_prandom_u32,
    );
}

// char _license[] SEC("license") = "GPL";
#[no_mangle]
#[link_section = "license"]
pub static _license: [u8; 4] = *b"GPL\0";
