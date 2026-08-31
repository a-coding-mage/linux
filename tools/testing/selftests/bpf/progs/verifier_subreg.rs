// SPDX-License-Identifier: GPL-2.0
/* Converted from tools/testing/selftests/bpf/verifier/subreg.c */

#![allow(non_upper_case_globals)]

use core::arch::global_asm;

/* Original C dependencies:
 *   #include <linux/bpf.h>
 *   #include <bpf/bpf_helpers.h>
 *   #include "bpf_misc.h"
 */

extern "C" {
    fn bpf_get_prandom_u32() -> u32;
}

/* This file contains sub-register zero extension checks for insns defining
 * sub-registers, meaning:
 *   - All insns under BPF_ALU class. Their BPF_ALU32 variants or narrow width
 *     forms (BPF_END) could define sub-registers.
 *   - Narrow direct loads, BPF_B/H/W | BPF_LDX.
 *   - BPF_LD is not exposed to JIT back-ends, so no need for testing.
 *
 * "get_prandom_u32" is used to initialize low 32-bit of some registers to
 * prevent potential optimizations done by verifier or JIT back-ends which could
 * optimize register back into constant when range info shows one register is a
 * constant.
 */

macro_rules! bpf_prog {
    ($name:ident, $section:literal, $asm:literal) => {
        global_asm!(
            concat!(
                ".section \"", $section, "\",\"ax\"\n",
                ".global ", stringify!($name), "\n",
                stringify!($name), ":\n",
                $asm,
            )
        );
    };
}

/* __description("add32 reg zero extend check")
 * __success __success_unpriv __retval(0)
 */
bpf_prog!(add32_reg_zero_extend_check, "socket",
"call bpf_get_prandom_u32
r1 = r0
r0 = 0x100000000 ll
w0 += w1
r0 >>= 32
exit
");

/* __description("add32 imm zero extend check")
 * __success __success_unpriv __retval(0)
 */
bpf_prog!(add32_imm_zero_extend_check, "socket",
"call bpf_get_prandom_u32
r1 = 0x1000000000 ll
r0 |= r1
/* An insn could have no effect on the low 32-bit, for example:
 *   a = a + 0
 *   a = a | 0
 *   a = a & -1
 * But, they should still zero high 32-bit.
 */
w0 += 0
r0 >>= 32
r6 = r0
call bpf_get_prandom_u32
r1 = 0x1000000000 ll
r0 |= r1
w0 += -2
r0 >>= 32
r0 |= r6
exit
");

/* __description("sub32 reg zero extend check")
 * __success __success_unpriv __retval(0)
 */
bpf_prog!(sub32_reg_zero_extend_check, "socket",
"call bpf_get_prandom_u32
r1 = r0
r0 = 0x1ffffffff ll
w0 -= w1
r0 >>= 32
exit
");

/* __description("sub32 imm zero extend check")
 * __success __success_unpriv __retval(0)
 */
bpf_prog!(sub32_imm_zero_extend_check, "socket",
"call bpf_get_prandom_u32
r1 = 0x1000000000 ll
r0 |= r1
w0 -= 0
r0 >>= 32
r6 = r0
call bpf_get_prandom_u32
r1 = 0x1000000000 ll
r0 |= r1
w0 -= 1
r0 >>= 32
r0 |= r6
exit
");

/* __description("mul32 reg zero extend check")
 * __success __success_unpriv __retval(0)
 */
bpf_prog!(mul32_reg_zero_extend_check, "socket",
"call bpf_get_prandom_u32
r1 = r0
r0 = 0x100000001 ll
w0 *= w1
r0 >>= 32
exit
");

/* __description("mul32 imm zero extend check")
 * __success __success_unpriv __retval(0)
 */
bpf_prog!(mul32_imm_zero_extend_check, "socket",
"call bpf_get_prandom_u32
r1 = 0x1000000000 ll
r0 |= r1
w0 *= 1
r0 >>= 32
r6 = r0
call bpf_get_prandom_u32
r1 = 0x1000000000 ll
r0 |= r1
w0 *= -1
r0 >>= 32
r0 |= r6
exit
");

/* __description("div32 reg zero extend check")
 * __success __success_unpriv __retval(0)
 */
bpf_prog!(div32_reg_zero_extend_check, "socket",
"call bpf_get_prandom_u32
r1 = r0
r0 = -1
w0 /= w1
r0 >>= 32
exit
");

/* __description("div32 imm zero extend check")
 * __success __success_unpriv __retval(0)
 */
bpf_prog!(div32_imm_zero_extend_check, "socket",
"call bpf_get_prandom_u32
r1 = 0x1000000000 ll
r0 |= r1
w0 /= 1
r0 >>= 32
r6 = r0
call bpf_get_prandom_u32
r1 = 0x1000000000 ll
r0 |= r1
w0 /= 2
r0 >>= 32
r0 |= r6
exit
");

/* __description("or32 reg zero extend check")
 * __success __success_unpriv __retval(0)
 */
bpf_prog!(or32_reg_zero_extend_check, "socket",
"call bpf_get_prandom_u32
r1 = r0
r0 = 0x100000001 ll
w0 |= w1
r0 >>= 32
exit
");

/* __description("or32 imm zero extend check")
 * __success __success_unpriv __retval(0)
 */
bpf_prog!(or32_imm_zero_extend_check, "socket",
"call bpf_get_prandom_u32
r1 = 0x1000000000 ll
r0 |= r1
w0 |= 0
r0 >>= 32
r6 = r0
call bpf_get_prandom_u32
r1 = 0x1000000000 ll
r0 |= r1
w0 |= 1
r0 >>= 32
r0 |= r6
exit
");

/* __description("and32 reg zero extend check")
 * __success __success_unpriv __retval(0)
 */
bpf_prog!(and32_reg_zero_extend_check, "socket",
"call bpf_get_prandom_u32
r1 = 0x100000000 ll
r1 |= r0
r0 = 0x1ffffffff ll
w0 &= w1
r0 >>= 32
exit
");

/* __description("and32 imm zero extend check")
 * __success __success_unpriv __retval(0)
 */
bpf_prog!(and32_imm_zero_extend_check, "socket",
"call bpf_get_prandom_u32
r1 = 0x1000000000 ll
r0 |= r1
w0 &= -1
r0 >>= 32
r6 = r0
call bpf_get_prandom_u32
r1 = 0x1000000000 ll
r0 |= r1
w0 &= -2
r0 >>= 32
r0 |= r6
exit
");

/* __description("lsh32 reg zero extend check")
 * __success __success_unpriv __retval(0)
 */
bpf_prog!(lsh32_reg_zero_extend_check, "socket",
"call bpf_get_prandom_u32
r1 = 0x100000000 ll
r0 |= r1
r1 = 1
w0 <<= w1
r0 >>= 32
exit
");

/* __description("lsh32 imm zero extend check")
 * __success __success_unpriv __retval(0)
 */
bpf_prog!(lsh32_imm_zero_extend_check, "socket",
"call bpf_get_prandom_u32
r1 = 0x1000000000 ll
r0 |= r1
w0 <<= 0
r0 >>= 32
r6 = r0
call bpf_get_prandom_u32
r1 = 0x1000000000 ll
r0 |= r1
w0 <<= 1
r0 >>= 32
r0 |= r6
exit
");

/* __description("rsh32 reg zero extend check")
 * __success __success_unpriv __retval(0)
 */
bpf_prog!(rsh32_reg_zero_extend_check, "socket",
"call bpf_get_prandom_u32
r1 = 0x1000000000 ll
r0 |= r1
r1 = 1
w0 >>= w1
r0 >>= 32
exit
");

/* __description("rsh32 imm zero extend check")
 * __success __success_unpriv __retval(0)
 */
bpf_prog!(rsh32_imm_zero_extend_check, "socket",
"call bpf_get_prandom_u32
r1 = 0x1000000000 ll
r0 |= r1
w0 >>= 0
r0 >>= 32
r6 = r0
call bpf_get_prandom_u32
r1 = 0x1000000000 ll
r0 |= r1
w0 >>= 1
r0 >>= 32
r0 |= r6
exit
");

/* __description("neg32 reg zero extend check")
 * __success __success_unpriv __retval(0)
 */
bpf_prog!(neg32_reg_zero_extend_check, "socket",
"call bpf_get_prandom_u32
r1 = 0x1000000000 ll
r0 |= r1
w0 = -w0
r0 >>= 32
exit
");

/* __description("mod32 reg zero extend check")
 * __success __success_unpriv __retval(0)
 */
bpf_prog!(mod32_reg_zero_extend_check, "socket",
"call bpf_get_prandom_u32
r1 = r0
r0 = -1
w0 %= w1
r0 >>= 32
exit
");

/* __description("mod32 imm zero extend check")
 * __success __success_unpriv __retval(0)
 */
bpf_prog!(mod32_imm_zero_extend_check, "socket",
"call bpf_get_prandom_u32
r1 = 0x1000000000 ll
r0 |= r1
w0 %= 1
r0 >>= 32
r6 = r0
call bpf_get_prandom_u32
r1 = 0x1000000000 ll
r0 |= r1
w0 %= 2
r0 >>= 32
r0 |= r6
exit
");

/* __description("xor32 reg zero extend check")
 * __success __success_unpriv __retval(0)
 */
bpf_prog!(xor32_reg_zero_extend_check, "socket",
"call bpf_get_prandom_u32
r1 = r0
r0 = 0x100000000 ll
w0 ^= w1
r0 >>= 32
exit
");

/* __description("xor32 imm zero extend check")
 * __success __success_unpriv __retval(0)
 */
bpf_prog!(xor32_imm_zero_extend_check, "socket",
"call bpf_get_prandom_u32
r1 = 0x1000000000 ll
r0 |= r1
w0 ^= 1
r0 >>= 32
exit
");

/* __description("mov32 reg zero extend check")
 * __success __success_unpriv __retval(0)
 */
bpf_prog!(mov32_reg_zero_extend_check, "socket",
"call bpf_get_prandom_u32
r1 = 0x100000000 ll
r1 |= r0
r0 = 0x100000000 ll
w0 = w1
r0 >>= 32
exit
");

/* __description("mov32 imm zero extend check")
 * __success __success_unpriv __retval(0)
 */
bpf_prog!(mov32_imm_zero_extend_check, "socket",
"call bpf_get_prandom_u32
r1 = 0x1000000000 ll
r0 |= r1
w0 = 0
r0 >>= 32
r6 = r0
call bpf_get_prandom_u32
r1 = 0x1000000000 ll
r0 |= r1
w0 = 1
r0 >>= 32
r0 |= r6
exit
");

/* __description("arsh32 reg zero extend check")
 * __success __success_unpriv __retval(0)
 */
bpf_prog!(arsh32_reg_zero_extend_check, "socket",
"call bpf_get_prandom_u32
r1 = 0x1000000000 ll
r0 |= r1
r1 = 1
w0 s>>= w1
r0 >>= 32
exit
");

/* __description("arsh32 imm zero extend check")
 * __success __success_unpriv __retval(0)
 */
bpf_prog!(arsh32_imm_zero_extend_check, "socket",
"call bpf_get_prandom_u32
r1 = 0x1000000000 ll
r0 |= r1
w0 s>>= 0
r0 >>= 32
r6 = r0
call bpf_get_prandom_u32
r1 = 0x1000000000 ll
r0 |= r1
w0 s>>= 1
r0 >>= 32
r0 |= r6
exit
");

/* __description("arsh32 imm sign positive extend check")
 * __success __retval(0)
 * __log_level(2)
 * __msg("2: (57) r6 &= 4095                    ; R6=scalar(smin=smin32=0,smax=umax=smax32=umax32=4095,var_off=(0x0; 0xfff))")
 * __msg("3: (67) r6 <<= 32                     ; R6=scalar(smin=smin32=0,smax=umax=0xfff00000000,smax32=umax32=0,var_off=(0x0; 0xfff00000000))")
 * __msg("4: (c7) r6 s>>= 32                    ; R6=scalar(smin=smin32=0,smax=umax=smax32=umax32=4095,var_off=(0x0; 0xfff))")
 */
bpf_prog!(arsh32_imm_sign_extend_positive_check, "socket",
"call bpf_get_prandom_u32
r6 = r0
r6 &= 4095
r6 <<= 32
r6 s>>= 32
r0 = 0
exit
");

/* __description("arsh32 imm sign negative extend check")
 * __success __retval(0)
 * __log_level(2)
 * __msg("3: (17) r6 -= 4095                    ; R6=scalar(smin=smin32=-4095,smax=smax32=0)")
 * __msg("4: (67) r6 <<= 32                     ; R6=scalar(smin=0xfffff00100000000,smax=smax32=umax32=0,smin32=0,var_off=(0x0; 0xffffffff00000000))")
 * represents shorter of signed / unsigned 64-bit ranges
 * __msg("5: (c7) r6 s>>= 32                    ; R6=scalar(smin=smin32=-4095,smax=smax32=0)")
 */
bpf_prog!(arsh32_imm_sign_extend_negative_check, "socket",
"call bpf_get_prandom_u32
r6 = r0
r6 &= 4095
r6 -= 4095
r6 <<= 32
r6 s>>= 32
r0 = 0
exit
");

/* __description("arsh32 imm sign extend check")
 * __success __retval(0)
 * __log_level(2)
 * __msg("3: (17) r6 -= 2047                    ; R6=scalar(smin=smin32=-2047,smax=smax32=2048)")
 * __msg("4: (67) r6 <<= 32                     ; R6=scalar(smin=0xfffff80100000000,smax=0x80000000000,smin32=0,smax32=umax32=0,var_off=(0x0; 0xffffffff00000000))")
 * represents shorter of signed / unsigned 64-bit ranges
 * __msg("5: (c7) r6 s>>= 32                    ; R6=scalar(smin=smin32=-2047,smax=smax32=2048)")
 */
bpf_prog!(arsh32_imm_sign_extend_check, "socket",
"call bpf_get_prandom_u32
r6 = r0
r6 &= 4095
r6 -= 2047
r6 <<= 32
r6 s>>= 32
r0 = 0
exit
");

/* __description("end16 (to_le) reg zero extend check")
 * __success __success_unpriv __retval(0)
 */
bpf_prog!(le_reg_zero_extend_check_1, "socket",
"call bpf_get_prandom_u32
r6 = r0
r6 <<= 32
call bpf_get_prandom_u32
r0 |= r6
r0 = le16 r0
r0 >>= 32
exit
");

/* __description("end32 (to_le) reg zero extend check")
 * __success __success_unpriv __retval(0)
 */
bpf_prog!(le_reg_zero_extend_check_2, "socket",
"call bpf_get_prandom_u32
r6 = r0
r6 <<= 32
call bpf_get_prandom_u32
r0 |= r6
r0 = le32 r0
r0 >>= 32
exit
");

/* __description("end16 (to_be) reg zero extend check")
 * __success __success_unpriv __retval(0)
 */
bpf_prog!(be_reg_zero_extend_check_1, "socket",
"call bpf_get_prandom_u32
r6 = r0
r6 <<= 32
call bpf_get_prandom_u32
r0 |= r6
r0 = be16 r0
r0 >>= 32
exit
");

/* __description("end32 (to_be) reg zero extend check")
 * __success __success_unpriv __retval(0)
 */
bpf_prog!(be_reg_zero_extend_check_2, "socket",
"call bpf_get_prandom_u32
r6 = r0
r6 <<= 32
call bpf_get_prandom_u32
r0 |= r6
r0 = be32 r0
r0 >>= 32
exit
");

/* __description("ldx_b zero extend check")
 * __success __success_unpriv __retval(0)
 */
bpf_prog!(ldx_b_zero_extend_check, "socket",
"r6 = r10
r6 += -4
r7 = 0xfaceb00c
*(u32*)(r6 + 0) = r7
call bpf_get_prandom_u32
r1 = 0x1000000000 ll
r0 |= r1
r0 = *(u8*)(r6 + 0)
r0 >>= 32
exit
");

/* __description("ldx_h zero extend check")
 * __success __success_unpriv __retval(0)
 */
bpf_prog!(ldx_h_zero_extend_check, "socket",
"r6 = r10
r6 += -4
r7 = 0xfaceb00c
*(u32*)(r6 + 0) = r7
call bpf_get_prandom_u32
r1 = 0x1000000000 ll
r0 |= r1
r0 = *(u16*)(r6 + 0)
r0 >>= 32
exit
");

/* __description("ldx_w zero extend check")
 * __success __success_unpriv __retval(0)
 */
bpf_prog!(ldx_w_zero_extend_check, "socket",
"r6 = r10
r6 += -4
r7 = 0xfaceb00c
*(u32*)(r6 + 0) = r7
call bpf_get_prandom_u32
r1 = 0x1000000000 ll
r0 |= r1
r0 = *(u32*)(r6 + 0)
r0 >>= 32
exit
");

/* __success __success_unpriv __retval(0) */
bpf_prog!(arsh_31_and, "socket",
"/* Below is what LLVM generates in cilium's bpf_wiregard.o */
call bpf_get_prandom_u32
w2 = w0
w2 s>>= 31
w2 &= -134 /* w2 becomes 0 or -134 */
if w2 s> -1 goto +2
/* Branch always taken because w2 = -134 */
if w2 != -136 goto +1
w0 /= 0
w0 = 0
exit
");

/* __success __success_unpriv __retval(0) */
bpf_prog!(arsh_63_and, "socket",
"/* Copy of arsh_31 with s/w/r/ */
call bpf_get_prandom_u32
r2 = r0
r2 <<= 32
r2 s>>= 63
r2 &= -134
if r2 s> -1 goto +2
/* Branch always taken because w2 = -134 */
if r2 != -136 goto +1
r0 /= 0
r0 = 0
exit
");

/* __success __success_unpriv __retval(0) */
bpf_prog!(arsh_31_or, "socket",
"call bpf_get_prandom_u32
w2 = w0
w2 s>>= 31
w2 |= 134 /* w2 becomes -1 or 134 */
if w2 s> -1 goto +2
/* Branch always taken because w2 = -1 */
if w2 == -1 goto +1
w0 /= 0
w0 = 0
exit
");

/* __success __success_unpriv __retval(0) */
bpf_prog!(arsh_63_or, "socket",
"/* Copy of arsh_31 with s/w/r/ */
call bpf_get_prandom_u32
r2 = r0
r2 <<= 32
r2 s>>= 63
r2 |= 134 /* r2 becomes -1 or 134 */
if r2 s> -1 goto +2
/* Branch always taken because w2 = -1 */
if r2 == -1 goto +1
r0 /= 0
r0 = 0
exit
");

/* __success __retval(42) */
bpf_prog!(arsh32_imm1_value, "socket",
"r0 = 42
r1 = -2147483648
w1 s>>= 1 /* r1 = 0xC0000000 */
r2 = 0xC0000000 ll
if r1 == r2 goto l0_1
r0 /= 0 /* unreachable */
l0_1: exit
");

/* __success __retval(1) */
bpf_prog!(lsh32_reg0_zero_extend_check, "socket",
"r6 = 1
call bpf_get_prandom_u32
r1 = 0x1000000000 ll
r0 |= r1
w1 = 0
w0 <<= w1 /* reg shift by 0 */
r0 >>= 32 /* must be 0 */
if r0 == 0 goto l0_2
r6 /= 0 /* unreachable */
l0_2: r0 = r6
exit
");

/* __success __retval(1) */
bpf_prog!(rsh32_reg0_zero_extend_check, "socket",
"r6 = 1
call bpf_get_prandom_u32
r1 = 0x1000000000 ll
r0 |= r1
w1 = 0
w0 >>= w1 /* reg rsh by 0 */
r0 >>= 32 /* must be 0 */
if r0 == 0 goto l0_3
r6 /= 0 /* unreachable */
l0_3: r0 = r6
exit
");

/* __success __retval(1) */
bpf_prog!(arsh32_reg0_zero_extend_check, "socket",
"r6 = 1
call bpf_get_prandom_u32
r1 = 0x1000000000 ll
r0 |= r1
w1 = 0
w0 s>>= w1 /* reg arsh by 0 */
r0 >>= 32 /* must be 0 */
if r0 == 0 goto l0_4
r6 /= 0 /* unreachable */
l0_4: r0 = r6
exit
");

/* __success __retval(42) */
bpf_prog!(lsh32_imm31_value, "socket",
"r0 = 42
r1 = 1
w1 <<= 31 /* r1 = 0x80000000 */
r2 = 0x80000000 ll
if r1 == r2 goto l0_5
r0 /= 0 /* unreachable */
l0_5: exit
");

/* __success __retval(42) */
bpf_prog!(rsh32_imm31_value, "socket",
"r0 = 42
r1 = -2147483648 /* 0x80000000 */
w1 >>= 31 /* r1 = 1 */
if r1 == 1 goto l0_6
r0 /= 0 /* unreachable */
l0_6: exit
");

/* __success __retval(42) */
bpf_prog!(arsh32_imm31_value, "socket",
"r0 = 42
r1 = -2147483648 /* 0x80000000 */
w1 s>>= 31 /* r1 = 0xFFFFFFFF */
r2 = 0xFFFFFFFF ll
if r1 == r2 goto l0_7
r0 /= 0 /* unreachable */
l0_7: exit
");

/* __success __retval(1) */
bpf_prog!(lsh32_unknown_precise_bounds, "socket",
"r6 = 1
call bpf_get_prandom_u32
w0 &= 3 /* u32: [0, 3] */
w0 <<= 1 /* u32: [0, 6] */
if w0 < 7 goto l0_8
r6 /= 0 /* unreachable */
l0_8: r0 = r6
exit
");

/* __success __retval(1) */
bpf_prog!(rsh32_unknown_bounds, "socket",
"r6 = 1
call bpf_get_prandom_u32
w0 >>= 28 /* u32: [0, 15] */
if w0 < 16 goto l0_9
r6 /= 0 /* unreachable */
l0_9: r0 = r6
exit
");

#[link_section = "license"]
#[no_mangle]
pub static _license: [u8; 4] = *b"GPL\0";
