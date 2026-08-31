// SPDX-License-Identifier: GPL-2.0

#![allow(non_snake_case)]
#![allow(unused_attributes)]
#![allow(unsafe_op_in_unsafe_fn)]
#![feature(naked_functions)]

// C dependencies removed from executable Rust:
// <linux/bpf.h>, <limits.h>, <bpf/bpf_helpers.h>, "bpf_misc.h".
// Original preprocessor condition:
// (arm64 || x86 || (riscv && __riscv_xlen == 64) || arm || s390 || loongarch)
// && __clang_major__ >= 18.

pub const INT_MIN: i32 = i32::MIN;
pub const LLONG_MIN: i64 = i64::MIN;

macro_rules! bpf_naked_test {
    (
        $(#[$meta:meta])*
        fn $name:ident($asm_text:expr $(, $asm_arg:tt = $asm_kind:ident $asm_value:expr)*);
    ) => {
        $(#[$meta])*
        #[unsafe(no_mangle)]
        #[unsafe(link_section = "socket")]
        pub unsafe extern "C" fn $name() {
            core::arch::asm!(
                $asm_text,
                $($asm_arg = $asm_kind $asm_value,)*
                options(noreturn),
            );
        }
    };
}

bpf_naked_test!(
    /// __description("SDIV32, non-zero imm divisor, check 1")
    /// __success __success_unpriv __retval(-20)
    fn sdiv32_non_zero_imm_1("w0 = -41; w0 s/= 2; exit;");
);

bpf_naked_test!(
    /// __description("SDIV32, non-zero imm divisor, check 2")
    /// __success __success_unpriv __retval(-20)
    fn sdiv32_non_zero_imm_2("w0 = 41; w0 s/= -2; exit;");
);

bpf_naked_test!(
    /// __description("SDIV32, non-zero imm divisor, check 3")
    /// __success __success_unpriv __retval(20)
    fn sdiv32_non_zero_imm_3("w0 = -41; w0 s/= -2; exit;");
);

bpf_naked_test!(
    /// __description("SDIV32, non-zero imm divisor, check 4")
    /// __success __success_unpriv __retval(-21)
    fn sdiv32_non_zero_imm_4("w0 = -42; w0 s/= 2; exit;");
);

bpf_naked_test!(
    /// __description("SDIV32, non-zero imm divisor, check 5")
    /// __success __success_unpriv __retval(-21)
    fn sdiv32_non_zero_imm_5("w0 = 42; w0 s/= -2; exit;");
);

bpf_naked_test!(
    /// __description("SDIV32, non-zero imm divisor, check 6")
    /// __success __success_unpriv __retval(21)
    fn sdiv32_non_zero_imm_6("w0 = -42; w0 s/= -2; exit;");
);

bpf_naked_test!(
    /// __description("SDIV32, non-zero imm divisor, check 7")
    /// __success __success_unpriv __retval(21)
    fn sdiv32_non_zero_imm_7("w0 = 42; w0 s/= 2; exit;");
);

bpf_naked_test!(
    /// __description("SDIV32, non-zero imm divisor, check 8")
    /// __success __success_unpriv __retval(20)
    fn sdiv32_non_zero_imm_8("w0 = 41; w0 s/= 2; exit;");
);

bpf_naked_test!(
    /// __description("SDIV32, non-zero reg divisor, check 1")
    /// __success __success_unpriv __retval(-20)
    fn sdiv32_non_zero_reg_1("w0 = -41; w1 = 2; w0 s/= w1; exit;");
);

bpf_naked_test!(
    /// __description("SDIV32, non-zero reg divisor, check 2")
    /// __success __success_unpriv __retval(-20)
    fn sdiv32_non_zero_reg_2("w0 = 41; w1 = -2; w0 s/= w1; exit;");
);

bpf_naked_test!(
    /// __description("SDIV32, non-zero reg divisor, check 3")
    /// __success __success_unpriv __retval(20)
    fn sdiv32_non_zero_reg_3("w0 = -41; w1 = -2; w0 s/= w1; exit;");
);

bpf_naked_test!(
    /// __description("SDIV32, non-zero reg divisor, check 4")
    /// __success __success_unpriv __retval(-21)
    fn sdiv32_non_zero_reg_4("w0 = -42; w1 = 2; w0 s/= w1; exit;");
);

bpf_naked_test!(
    /// __description("SDIV32, non-zero reg divisor, check 5")
    /// __success __success_unpriv __retval(-21)
    fn sdiv32_non_zero_reg_5("w0 = 42; w1 = -2; w0 s/= w1; exit;");
);

bpf_naked_test!(
    /// __description("SDIV32, non-zero reg divisor, check 6")
    /// __success __success_unpriv __retval(21)
    fn sdiv32_non_zero_reg_6("w0 = -42; w1 = -2; w0 s/= w1; exit;");
);

bpf_naked_test!(
    /// __description("SDIV32, non-zero reg divisor, check 7")
    /// __success __success_unpriv __retval(21)
    fn sdiv32_non_zero_reg_7("w0 = 42; w1 = 2; w0 s/= w1; exit;");
);

bpf_naked_test!(
    /// __description("SDIV32, non-zero reg divisor, check 8")
    /// __success __success_unpriv __retval(20)
    fn sdiv32_non_zero_reg_8("w0 = 41; w1 = 2; w0 s/= w1; exit;");
);

bpf_naked_test!(
    /// __description("SDIV64, non-zero imm divisor, check 1")
    /// __success __success_unpriv __retval(-20)
    fn sdiv64_non_zero_imm_1("r0 = -41; r0 s/= 2; exit;");
);

bpf_naked_test!(
    /// __description("SDIV64, non-zero imm divisor, check 2")
    /// __success __success_unpriv __retval(-20)
    fn sdiv64_non_zero_imm_2("r0 = 41; r0 s/= -2; exit;");
);

bpf_naked_test!(
    /// __description("SDIV64, non-zero imm divisor, check 3")
    /// __success __success_unpriv __retval(20)
    fn sdiv64_non_zero_imm_3("r0 = -41; r0 s/= -2; exit;");
);

bpf_naked_test!(
    /// __description("SDIV64, non-zero imm divisor, check 4")
    /// __success __success_unpriv __retval(-21)
    fn sdiv64_non_zero_imm_4("r0 = -42; r0 s/= 2; exit;");
);

bpf_naked_test!(
    /// __description("SDIV64, non-zero imm divisor, check 5")
    /// __success __success_unpriv __retval(-21)
    fn sdiv64_non_zero_imm_5("r0 = 42; r0 s/= -2; exit;");
);

bpf_naked_test!(
    /// __description("SDIV64, non-zero imm divisor, check 6")
    /// __success __success_unpriv __retval(21)
    fn sdiv64_non_zero_imm_6("r0 = -42; r0 s/= -2; exit;");
);

bpf_naked_test!(
    /// __description("SDIV64, non-zero reg divisor, check 1")
    /// __success __success_unpriv __retval(-20)
    fn sdiv64_non_zero_reg_1("r0 = -41; r1 = 2; r0 s/= r1; exit;");
);

bpf_naked_test!(
    /// __description("SDIV64, non-zero reg divisor, check 2")
    /// __success __success_unpriv __retval(-20)
    fn sdiv64_non_zero_reg_2("r0 = 41; r1 = -2; r0 s/= r1; exit;");
);

bpf_naked_test!(
    /// __description("SDIV64, non-zero reg divisor, check 3")
    /// __success __success_unpriv __retval(20)
    fn sdiv64_non_zero_reg_3("r0 = -41; r1 = -2; r0 s/= r1; exit;");
);

bpf_naked_test!(
    /// __description("SDIV64, non-zero reg divisor, check 4")
    /// __success __success_unpriv __retval(-21)
    fn sdiv64_non_zero_reg_4("r0 = -42; r1 = 2; r0 s/= r1; exit;");
);

bpf_naked_test!(
    /// __description("SDIV64, non-zero reg divisor, check 5")
    /// __success __success_unpriv __retval(-21)
    fn sdiv64_non_zero_reg_5("r0 = 42; r1 = -2; r0 s/= r1; exit;");
);

bpf_naked_test!(
    /// __description("SDIV64, non-zero reg divisor, check 6")
    /// __success __success_unpriv __retval(21)
    fn sdiv64_non_zero_reg_6("r0 = -42; r1 = -2; r0 s/= r1; exit;");
);

bpf_naked_test!(
    /// __description("SMOD32, non-zero imm divisor, check 1")
    /// __success __success_unpriv __retval(-1)
    fn smod32_non_zero_imm_1("w0 = -41; w0 s%= 2; exit;");
);

bpf_naked_test!(
    /// __description("SMOD32, non-zero imm divisor, check 2")
    /// __success __success_unpriv __retval(1)
    fn smod32_non_zero_imm_2("w0 = 41; w0 s%= -2; exit;");
);

bpf_naked_test!(
    /// __description("SMOD32, non-zero imm divisor, check 3")
    /// __success __success_unpriv __retval(-1)
    fn smod32_non_zero_imm_3("w0 = -41; w0 s%= -2; exit;");
);

bpf_naked_test!(
    /// __description("SMOD32, non-zero imm divisor, check 4")
    /// __success __success_unpriv __retval(0)
    fn smod32_non_zero_imm_4("w0 = -42; w0 s%= 2; exit;");
);

bpf_naked_test!(
    /// __description("SMOD32, non-zero imm divisor, check 5")
    /// __success __success_unpriv __retval(0)
    fn smod32_non_zero_imm_5("w0 = 42; w0 s%= -2; exit;");
);

bpf_naked_test!(
    /// __description("SMOD32, non-zero imm divisor, check 6")
    /// __success __success_unpriv __retval(0)
    fn smod32_non_zero_imm_6("w0 = -42; w0 s%= -2; exit;");
);

bpf_naked_test!(
    /// __description("SMOD32, non-zero reg divisor, check 1")
    /// __success __success_unpriv __retval(-1)
    fn smod32_non_zero_reg_1("w0 = -41; w1 = 2; w0 s%= w1; exit;");
);

bpf_naked_test!(
    /// __description("SMOD32, non-zero reg divisor, check 2")
    /// __success __success_unpriv __retval(1)
    fn smod32_non_zero_reg_2("w0 = 41; w1 = -2; w0 s%= w1; exit;");
);

bpf_naked_test!(
    /// __description("SMOD32, non-zero reg divisor, check 3")
    /// __success __success_unpriv __retval(-1)
    fn smod32_non_zero_reg_3("w0 = -41; w1 = -2; w0 s%= w1; exit;");
);

bpf_naked_test!(
    /// __description("SMOD32, non-zero reg divisor, check 4")
    /// __success __success_unpriv __retval(0)
    fn smod32_non_zero_reg_4("w0 = -42; w1 = 2; w0 s%= w1; exit;");
);

bpf_naked_test!(
    /// __description("SMOD32, non-zero reg divisor, check 5")
    /// __success __success_unpriv __retval(0)
    fn smod32_non_zero_reg_5("w0 = 42; w1 = -2; w0 s%= w1; exit;");
);

bpf_naked_test!(
    /// __description("SMOD32, non-zero reg divisor, check 6")
    /// __success __success_unpriv __retval(0)
    fn smod32_non_zero_reg_6("w0 = -42; w1 = -2; w0 s%= w1; exit;");
);

bpf_naked_test!(
    /// __description("SMOD64, non-zero imm divisor, check 1")
    /// __success __success_unpriv __retval(-1)
    fn smod64_non_zero_imm_1("r0 = -41; r0 s%= 2; exit;");
);

bpf_naked_test!(
    /// __description("SMOD64, non-zero imm divisor, check 2")
    /// __success __success_unpriv __retval(1)
    fn smod64_non_zero_imm_2("r0 = 41; r0 s%= -2; exit;");
);

bpf_naked_test!(
    /// __description("SMOD64, non-zero imm divisor, check 3")
    /// __success __success_unpriv __retval(-1)
    fn smod64_non_zero_imm_3("r0 = -41; r0 s%= -2; exit;");
);

bpf_naked_test!(
    /// __description("SMOD64, non-zero imm divisor, check 4")
    /// __success __success_unpriv __retval(0)
    fn smod64_non_zero_imm_4("r0 = -42; r0 s%= 2; exit;");
);

bpf_naked_test!(
    /// __description("SMOD64, non-zero imm divisor, check 5")
    /// __success __success_unpriv __retval(-0)
    fn smod64_non_zero_imm_5("r0 = 42; r0 s%= -2; exit;");
);

bpf_naked_test!(
    /// __description("SMOD64, non-zero imm divisor, check 6")
    /// __success __success_unpriv __retval(0)
    fn smod64_non_zero_imm_6("r0 = -42; r0 s%= -2; exit;");
);

bpf_naked_test!(
    /// __description("SMOD64, non-zero imm divisor, check 7")
    /// __success __success_unpriv __retval(0)
    fn smod64_non_zero_imm_7("r0 = 42; r0 s%= 2; exit;");
);

bpf_naked_test!(
    /// __description("SMOD64, non-zero imm divisor, check 8")
    /// __success __success_unpriv __retval(1)
    fn smod64_non_zero_imm_8("r0 = 41; r0 s%= 2; exit;");
);

bpf_naked_test!(
    /// __description("SMOD64, non-zero reg divisor, check 1")
    /// __success __success_unpriv __retval(-1)
    fn smod64_non_zero_reg_1("r0 = -41; r1 = 2; r0 s%= r1; exit;");
);

bpf_naked_test!(
    /// __description("SMOD64, non-zero reg divisor, check 2")
    /// __success __success_unpriv __retval(1)
    fn smod64_non_zero_reg_2("r0 = 41; r1 = -2; r0 s%= r1; exit;");
);

bpf_naked_test!(
    /// __description("SMOD64, non-zero reg divisor, check 3")
    /// __success __success_unpriv __retval(-1)
    fn smod64_non_zero_reg_3("r0 = -41; r1 = -2; r0 s%= r1; exit;");
);

bpf_naked_test!(
    /// __description("SMOD64, non-zero reg divisor, check 4")
    /// __success __success_unpriv __retval(0)
    fn smod64_non_zero_reg_4("r0 = -42; r1 = 2; r0 s%= r1; exit;");
);

bpf_naked_test!(
    /// __description("SMOD64, non-zero reg divisor, check 5")
    /// __success __success_unpriv __retval(0)
    fn smod64_non_zero_reg_5("r0 = 42; r1 = -2; r0 s%= r1; exit;");
);

bpf_naked_test!(
    /// __description("SMOD64, non-zero reg divisor, check 6")
    /// __success __success_unpriv __retval(0)
    fn smod64_non_zero_reg_6("r0 = -42; r1 = -2; r0 s%= r1; exit;");
);

bpf_naked_test!(
    /// __description("SMOD64, non-zero reg divisor, check 7")
    /// __success __success_unpriv __retval(0)
    fn smod64_non_zero_reg_7("r0 = 42; r1 = 2; r0 s%= r1; exit;");
);

bpf_naked_test!(
    /// __description("SMOD64, non-zero reg divisor, check 8")
    /// __success __success_unpriv __retval(1)
    fn smod64_non_zero_reg_8("r0 = 41; r1 = 2; r0 s%= r1; exit;");
);

bpf_naked_test!(
    /// __description("SDIV32, zero divisor")
    /// __success __success_unpriv __retval(0)
    fn sdiv32_zero_divisor("w0 = 42; w1 = 0; w2 = -1; w2 s/= w1; w0 = w2; exit;");
);

bpf_naked_test!(
    /// __description("SDIV64, zero divisor")
    /// __success __success_unpriv __retval(0)
    fn sdiv64_zero_divisor("r0 = 42; r1 = 0; r2 = -1; r2 s/= r1; r0 = r2; exit;");
);

bpf_naked_test!(
    /// __description("SMOD32, zero divisor")
    /// __success __success_unpriv __retval(-1)
    fn smod32_zero_divisor("w0 = 42; w1 = 0; w2 = -1; w2 s%= w1; w0 = w2; exit;");
);

bpf_naked_test!(
    /// __description("SMOD64, zero divisor")
    /// __success __success_unpriv __retval(-1)
    fn smod64_zero_divisor("r0 = 42; r1 = 0; r2 = -1; r2 s%= r1; r0 = r2; exit;");
);

bpf_naked_test!(
    /// __description("SDIV64, overflow r/r, LLONG_MIN/-1")
    /// __success __retval(1)
    /// __arch_x86_64
    /// __xlated("0: r2 = 0x8000000000000000")
    /// __xlated("2: r3 = -1")
    /// __xlated("3: r4 = r2")
    /// __xlated("4: r12 = r3")
    /// __xlated("5: r12 += 1")
    /// __xlated("6: if r12 > 0x1 goto pc+4")
    /// __xlated("7: if r12 == 0x0 goto pc+1")
    /// __xlated("8: r2 = 0")
    /// __xlated("9: r2 = -r2")
    /// __xlated("10: goto pc+1")
    /// __xlated("11: r2 s/= r3")
    /// __xlated("12: r0 = 0")
    /// __xlated("13: if r2 != r4 goto pc+1")
    /// __xlated("14: r0 = 1")
    /// __xlated("15: exit")
    fn sdiv64_overflow_rr("r2 = {llong_min}; r3 = -1; r4 = r2; r2 s/= r3; r0 = 0; if r2 != r4 goto +1; r0 = 1; exit;", llong_min = const LLONG_MIN);
);

bpf_naked_test!(
    /// __description("SDIV64, r/r, small_val/-1")
    /// __success __retval(-5)
    /// __arch_x86_64
    /// __xlated("0: r2 = 5")
    /// __xlated("1: r3 = -1")
    /// __xlated("2: r12 = r3")
    /// __xlated("3: r12 += 1")
    /// __xlated("4: if r12 > 0x1 goto pc+4")
    /// __xlated("5: if r12 == 0x0 goto pc+1")
    /// __xlated("6: r2 = 0")
    /// __xlated("7: r2 = -r2")
    /// __xlated("8: goto pc+1")
    /// __xlated("9: r2 s/= r3")
    /// __xlated("10: r0 = r2")
    /// __xlated("11: exit")
    fn sdiv64_rr_divisor_neg_1("r2 = 5; r3 = -1; r2 s/= r3; r0 = r2; exit;");
);

bpf_naked_test!(
    /// __description("SDIV64, overflow r/i, LLONG_MIN/-1")
    /// __success __retval(1)
    /// __arch_x86_64
    /// __xlated("0: r2 = 0x8000000000000000")
    /// __xlated("2: r4 = r2")
    /// __xlated("3: r2 = -r2")
    /// __xlated("4: r0 = 0")
    /// __xlated("5: if r2 != r4 goto pc+1")
    /// __xlated("6: r0 = 1")
    /// __xlated("7: exit")
    fn sdiv64_overflow_ri("r2 = {llong_min}; r4 = r2; r2 s/= -1; r0 = 0; if r2 != r4 goto +1; r0 = 1; exit;", llong_min = const LLONG_MIN);
);

bpf_naked_test!(
    /// __description("SDIV64, r/i, small_val/-1")
    /// __success __retval(-5)
    /// __arch_x86_64
    /// __xlated("0: r2 = 5")
    /// __xlated("1: r4 = r2")
    /// __xlated("2: r2 = -r2")
    /// __xlated("3: r0 = r2")
    /// __xlated("4: exit")
    fn sdiv64_ri_divisor_neg_1("r2 = 5; r4 = r2; r2 s/= -1; r0 = r2; exit;");
);

bpf_naked_test!(
    /// __description("SDIV32, overflow r/r, INT_MIN/-1")
    /// __success __retval(1)
    /// __arch_x86_64
    /// __xlated("0: w2 = -2147483648")
    /// __xlated("1: w3 = -1")
    /// __xlated("2: w4 = w2")
    /// __xlated("3: r12 = r3")
    /// __xlated("4: w12 += 1")
    /// __xlated("5: if w12 > 0x1 goto pc+4")
    /// __xlated("6: if w12 == 0x0 goto pc+1")
    /// __xlated("7: w2 = 0")
    /// __xlated("8: w2 = -w2")
    /// __xlated("9: goto pc+1")
    /// __xlated("10: w2 s/= w3")
    /// __xlated("11: r0 = 0")
    /// __xlated("12: if w2 != w4 goto pc+1")
    /// __xlated("13: r0 = 1")
    /// __xlated("14: exit")
    fn sdiv32_overflow_rr("w2 = {int_min}; w3 = -1; w4 = w2; w2 s/= w3; r0 = 0; if w2 != w4 goto +1; r0 = 1; exit;", int_min = const INT_MIN);
);

bpf_naked_test!(
    /// __description("SDIV32, r/r, small_val/-1")
    /// __success __retval(5)
    /// __arch_x86_64
    /// __xlated("0: w2 = -5")
    /// __xlated("1: w3 = -1")
    /// __xlated("2: w4 = w2")
    /// __xlated("3: r12 = r3")
    /// __xlated("4: w12 += 1")
    /// __xlated("5: if w12 > 0x1 goto pc+4")
    /// __xlated("6: if w12 == 0x0 goto pc+1")
    /// __xlated("7: w2 = 0")
    /// __xlated("8: w2 = -w2")
    /// __xlated("9: goto pc+1")
    /// __xlated("10: w2 s/= w3")
    /// __xlated("11: w0 = w2")
    /// __xlated("12: exit")
    fn sdiv32_rr_divisor_neg_1("w2 = -5; w3 = -1; w4 = w2; w2 s/= w3; w0 = w2; exit;");
);

bpf_naked_test!(
    /// __description("SDIV32, overflow r/i, INT_MIN/-1")
    /// __success __retval(1)
    /// __arch_x86_64
    /// __xlated("0: w2 = -2147483648")
    /// __xlated("1: w4 = w2")
    /// __xlated("2: w2 = -w2")
    /// __xlated("3: r0 = 0")
    /// __xlated("4: if w2 != w4 goto pc+1")
    /// __xlated("5: r0 = 1")
    /// __xlated("6: exit")
    fn sdiv32_overflow_ri("w2 = {int_min}; w4 = w2; w2 s/= -1; r0 = 0; if w2 != w4 goto +1; r0 = 1; exit;", int_min = const INT_MIN);
);

bpf_naked_test!(
    /// __description("SDIV32, r/i, small_val/-1")
    /// __success __retval(-5)
    /// __arch_x86_64
    /// __xlated("0: w2 = 5")
    /// __xlated("1: w4 = w2")
    /// __xlated("2: w2 = -w2")
    /// __xlated("3: w0 = w2")
    /// __xlated("4: exit")
    fn sdiv32_ri_divisor_neg_1("w2 = 5; w4 = w2; w2 s/= -1; w0 = w2; exit;");
);

bpf_naked_test!(
    /// __description("SMOD64, overflow r/r, LLONG_MIN/-1")
    /// __success __retval(0)
    /// __arch_x86_64
    /// __xlated("0: r2 = 0x8000000000000000")
    /// __xlated("2: r3 = -1")
    /// __xlated("3: r4 = r2")
    /// __xlated("4: r12 = r3")
    /// __xlated("5: r12 += 1")
    /// __xlated("6: if r12 > 0x1 goto pc+3")
    /// __xlated("7: if r12 == 0x1 goto pc+3")
    /// __xlated("8: w2 = 0")
    /// __xlated("9: goto pc+1")
    /// __xlated("10: r2 s%= r3")
    /// __xlated("11: r0 = r2")
    /// __xlated("12: exit")
    fn smod64_overflow_rr("r2 = {llong_min}; r3 = -1; r4 = r2; r2 s%= r3; r0 = r2; exit;", llong_min = const LLONG_MIN);
);

bpf_naked_test!(
    /// __description("SMOD64, r/r, small_val/-1")
    /// __success __retval(0)
    /// __arch_x86_64
    /// __xlated("0: r2 = 5")
    /// __xlated("1: r3 = -1")
    /// __xlated("2: r4 = r2")
    /// __xlated("3: r12 = r3")
    /// __xlated("4: r12 += 1")
    /// __xlated("5: if r12 > 0x1 goto pc+3")
    /// __xlated("6: if r12 == 0x1 goto pc+3")
    /// __xlated("7: w2 = 0")
    /// __xlated("8: goto pc+1")
    /// __xlated("9: r2 s%= r3")
    /// __xlated("10: r0 = r2")
    /// __xlated("11: exit")
    fn smod64_rr_divisor_neg_1("r2 = 5; r3 = -1; r4 = r2; r2 s%= r3; r0 = r2; exit;");
);

bpf_naked_test!(
    /// __description("SMOD64, overflow r/i, LLONG_MIN/-1")
    /// __success __retval(0)
    /// __arch_x86_64
    /// __xlated("0: r2 = 0x8000000000000000")
    /// __xlated("2: r4 = r2")
    /// __xlated("3: w2 = 0")
    /// __xlated("4: r0 = r2")
    /// __xlated("5: exit")
    fn smod64_overflow_ri("r2 = {llong_min}; r4 = r2; r2 s%= -1; r0 = r2; exit;", llong_min = const LLONG_MIN);
);

bpf_naked_test!(
    /// __description("SMOD64, r/i, small_val/-1")
    /// __success __retval(0)
    /// __arch_x86_64
    /// __xlated("0: r2 = 5")
    /// __xlated("1: r4 = r2")
    /// __xlated("2: w2 = 0")
    /// __xlated("3: r0 = r2")
    /// __xlated("4: exit")
    fn smod64_ri_divisor_neg_1("r2 = 5; r4 = r2; r2 s%= -1; r0 = r2; exit;");
);

bpf_naked_test!(
    /// __description("SMOD32, overflow r/r, INT_MIN/-1")
    /// __success __retval(0)
    /// __arch_x86_64
    /// __xlated("0: w2 = -2147483648")
    /// __xlated("1: w3 = -1")
    /// __xlated("2: w4 = w2")
    /// __xlated("3: r12 = r3")
    /// __xlated("4: w12 += 1")
    /// __xlated("5: if w12 > 0x1 goto pc+3")
    /// __xlated("6: if w12 == 0x1 goto pc+4")
    /// __xlated("7: w2 = 0")
    /// __xlated("8: goto pc+1")
    /// __xlated("9: w2 s%= w3")
    /// __xlated("10: goto pc+1")
    /// __xlated("11: w2 = w2")
    /// __xlated("12: r0 = r2")
    /// __xlated("13: exit")
    fn smod32_overflow_rr("w2 = {int_min}; w3 = -1; w4 = w2; w2 s%= w3; r0 = r2; exit;", int_min = const INT_MIN);
);

bpf_naked_test!(
    /// __description("SMOD32, r/r, small_val/-1")
    /// __success __retval(0)
    /// __arch_x86_64
    /// __xlated("0: w2 = -5")
    /// __xlated("1: w3 = -1")
    /// __xlated("2: w4 = w2")
    /// __xlated("3: r12 = r3")
    /// __xlated("4: w12 += 1")
    /// __xlated("5: if w12 > 0x1 goto pc+3")
    /// __xlated("6: if w12 == 0x1 goto pc+4")
    /// __xlated("7: w2 = 0")
    /// __xlated("8: goto pc+1")
    /// __xlated("9: w2 s%= w3")
    /// __xlated("10: goto pc+1")
    /// __xlated("11: w2 = w2")
    /// __xlated("12: r0 = r2")
    /// __xlated("13: exit")
    fn smod32_rr_divisor_neg_1("w2 = -5; w3 = -1; w4 = w2; w2 s%= w3; r0 = r2; exit;");
);

bpf_naked_test!(
    /// __description("SMOD32, overflow r/i, INT_MIN/-1")
    /// __success __retval(0)
    /// __arch_x86_64
    /// __xlated("0: w2 = -2147483648")
    /// __xlated("1: w4 = w2")
    /// __xlated("2: w2 = 0")
    /// __xlated("3: r0 = r2")
    /// __xlated("4: exit")
    fn smod32_overflow_ri("w2 = {int_min}; w4 = w2; w2 s%= -1; r0 = r2; exit;", int_min = const INT_MIN);
);

bpf_naked_test!(
    /// __description("SMOD32, r/i, small_val/-1")
    /// __success __retval(0)
    /// __arch_x86_64
    /// __xlated("0: w2 = 5")
    /// __xlated("1: w4 = w2")
    /// __xlated("2: w2 = 0")
    /// __xlated("3: w0 = w2")
    /// __xlated("4: exit")
    fn smod32_ri_divisor_neg_1("w2 = 5; w4 = w2; w2 s%= -1; w0 = w2; exit;");
);

bpf_naked_test!(
    /// __description("SDIV32, INT_MIN divided by 2, imm")
    /// __success __success_unpriv __retval(-1073741824)
    fn sdiv32_int_min_div_2_imm("w0 = {int_min}; w0 s/= 2; exit;", int_min = const INT_MIN);
);

bpf_naked_test!(
    /// __description("SDIV32, INT_MIN divided by 2, reg")
    /// __success __success_unpriv __retval(-1073741824)
    fn sdiv32_int_min_div_2_reg("w0 = {int_min}; w1 = 2; w0 s/= w1; exit;", int_min = const INT_MIN);
);

bpf_naked_test!(
    /// __description("SMOD32, INT_MIN modulo 2, imm")
    /// __success __success_unpriv __retval(0)
    fn smod32_int_min_mod_2_imm("w0 = {int_min}; w0 s%= 2; exit;", int_min = const INT_MIN);
);

bpf_naked_test!(
    /// __description("SMOD32, INT_MIN modulo -2, imm")
    /// __success __success_unpriv __retval(0)
    fn smod32_int_min_mod_neg2_imm("w0 = {int_min}; w0 s%= -2; exit;", int_min = const INT_MIN);
);

/// __description("cpuv4 is not supported by compiler or jit, use a dummy test")
/// __success
#[unsafe(no_mangle)]
#[unsafe(link_section = "socket")]
pub extern "C" fn dummy_test() -> i32 {
    0
}

#[unsafe(no_mangle)]
#[unsafe(link_section = "license")]
pub static mut _license: [u8; 4] = *b"GPL\0";
