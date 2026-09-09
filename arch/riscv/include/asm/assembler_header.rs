/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (C) 2023 StarFive Technology Co., Ltd.
 *
 * Author: Jee Heng Sia <jeeheng.sia@starfivetech.com>
 */

// Original header is intended for inclusion from RISC-V assembly code.
// Dependencies supplied by the surrounding kernel build provide the assembly
// helpers, offsets, CSR names, and BIT definition.

/*
 * suspend_restore_csrs - restore CSRs
 */
#[macro_export]
macro_rules! suspend_restore_csrs {
    () => {
        core::arch::global_asm!(
            "REG_L t0, (SUSPEND_CONTEXT_REGS + PT_EPC)(a0)\n\
             csrw CSR_EPC, t0\n\
             REG_L t0, (SUSPEND_CONTEXT_REGS + PT_STATUS)(a0)\n\
             csrw CSR_STATUS, t0\n\
             REG_L t0, (SUSPEND_CONTEXT_REGS + PT_BADADDR)(a0)\n\
             csrw CSR_TVAL, t0\n\
             REG_L t0, (SUSPEND_CONTEXT_REGS + PT_CAUSE)(a0)\n\
             csrw CSR_CAUSE, t0"
        );
    };
}

/*
 * suspend_restore_regs - Restore registers (except A0 and T0-T6)
 */
#[macro_export]
macro_rules! suspend_restore_regs {
    () => {
        core::arch::global_asm!(
            "REG_L ra, (SUSPEND_CONTEXT_REGS + PT_RA)(a0)\n\
             REG_L sp, (SUSPEND_CONTEXT_REGS + PT_SP)(a0)\n\
             REG_L gp, (SUSPEND_CONTEXT_REGS + PT_GP)(a0)\n\
             REG_L tp, (SUSPEND_CONTEXT_REGS + PT_TP)(a0)\n\
             REG_L s0, (SUSPEND_CONTEXT_REGS + PT_S0)(a0)\n\
             REG_L s1, (SUSPEND_CONTEXT_REGS + PT_S1)(a0)\n\
             REG_L a1, (SUSPEND_CONTEXT_REGS + PT_A1)(a0)\n\
             REG_L a2, (SUSPEND_CONTEXT_REGS + PT_A2)(a0)\n\
             REG_L a3, (SUSPEND_CONTEXT_REGS + PT_A3)(a0)\n\
             REG_L a4, (SUSPEND_CONTEXT_REGS + PT_A4)(a0)\n\
             REG_L a5, (SUSPEND_CONTEXT_REGS + PT_A5)(a0)\n\
             REG_L a6, (SUSPEND_CONTEXT_REGS + PT_A6)(a0)\n\
             REG_L a7, (SUSPEND_CONTEXT_REGS + PT_A7)(a0)\n\
             REG_L s2, (SUSPEND_CONTEXT_REGS + PT_S2)(a0)\n\
             REG_L s3, (SUSPEND_CONTEXT_REGS + PT_S3)(a0)\n\
             REG_L s4, (SUSPEND_CONTEXT_REGS + PT_S4)(a0)\n\
             REG_L s5, (SUSPEND_CONTEXT_REGS + PT_S5)(a0)\n\
             REG_L s6, (SUSPEND_CONTEXT_REGS + PT_S6)(a0)\n\
             REG_L s7, (SUSPEND_CONTEXT_REGS + PT_S7)(a0)\n\
             REG_L s8, (SUSPEND_CONTEXT_REGS + PT_S8)(a0)\n\
             REG_L s9, (SUSPEND_CONTEXT_REGS + PT_S9)(a0)\n\
             REG_L s10, (SUSPEND_CONTEXT_REGS + PT_S10)(a0)\n\
             REG_L s11, (SUSPEND_CONTEXT_REGS + PT_S11)(a0)"
        );
    };
}

/*
 * copy_page - copy 1 page (4KB) of data from source to destination
 * @a0 - destination
 * @a1 - source
 */
#[macro_export]
macro_rules! copy_page {
    ($a0:ident, $a1:ident) => {
        core::arch::global_asm!(
            "lui a2, 0x1\n\
             add a2, a2, a0\n\
             1: REG_L t0, 0(a1)\n\
             REG_L t1, SZREG(a1)\n\
             REG_S t0, 0(a0)\n\
             REG_S t1, SZREG(a0)\n\
             addi a0, a0, 2 * SZREG\n\
             addi a1, a1, 2 * SZREG\n\
             bne a2, a0, 1b"
        );
    };
}

// Conditional on VDSO_CFI and a 64-bit RISC-V target in the original source.
#[macro_export]
macro_rules! vdso_lpad {
    ($label:expr) => {
        core::arch::global_asm!(concat!("lpad ", $label));
    };
}

pub const NT_GNU_PROPERTY_TYPE_0: u32 = 5;
pub const GNU_PROPERTY_RISCV_FEATURE_1_AND: u32 = 0xc000_0000;
pub const GNU_PROPERTY_RISCV_FEATURE_1_ZICFILP: u32 = 1u32 << 0;
pub const GNU_PROPERTY_RISCV_FEATURE_1_ZICFISS: u32 = 1u32 << 1;

// Defined only when VDSO_CFI is enabled on RV64 in the original source.
#[cfg(all(feature = "vdso_cfi", target_pointer_width = "64"))]
pub const GNU_PROPERTY_RISCV_FEATURE_1_DEFAULT: u32 =
    GNU_PROPERTY_RISCV_FEATURE_1_ZICFILP | GNU_PROPERTY_RISCV_FEATURE_1_ZICFISS;

// The original emit_riscv_feature_1_and macro emits a GNU property note
// section when GNU_PROPERTY_RISCV_FEATURE_1_DEFAULT is defined; otherwise it
// expands to no assembly.
#[macro_export]
macro_rules! emit_riscv_feature_1_and {
    ($feat:expr) => {
        core::arch::global_asm!(concat!(
            ".pushsection .note.gnu.property, \"a\"\n",
            ".p2align 3\n",
            ".word 4\n",
            ".word 16\n",
            ".word NT_GNU_PROPERTY_TYPE_0\n",
            ".asciz \"GNU\"\n",
            ".word GNU_PROPERTY_RISCV_FEATURE_1_AND\n",
            ".word 4\n",
            ".word ", $feat, "\n",
            ".word 0\n",
            ".popsection"
        ));
    };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
