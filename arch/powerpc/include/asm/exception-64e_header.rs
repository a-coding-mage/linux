/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Definitions for use by exception code on Book3-E.
 * Direct Rust translation of exception-64e.h.
 */

// The following names are supplied by other PowerPC headers/build configuration.
pub const PACA_EXGDBELL: usize = PACA_EXGEN;

pub const EX_R1: usize = 0 * 8;
pub const EX_CR: usize = 1 * 8;
pub const EX_R10: usize = 2 * 8;
pub const EX_R11: usize = 3 * 8;
pub const EX_R14: usize = 4 * 8;
pub const EX_R15: usize = 5 * 8;

pub const EX_TLB_R10: usize = 0 * 8;
pub const EX_TLB_R11: usize = 1 * 8;
pub const EX_TLB_R14: usize = 2 * 8;
pub const EX_TLB_R15: usize = 3 * 8;
pub const EX_TLB_R16: usize = 4 * 8;
pub const EX_TLB_CR: usize = 5 * 8;
pub const EX_TLB_R12: usize = 6 * 8;
pub const EX_TLB_R13: usize = 7 * 8;
pub const EX_TLB_DEAR: usize = 8 * 8; // Level 0 and 2 only.
pub const EX_TLB_ESR: usize = 9 * 8; // Level 0 and 2 only.
pub const EX_TLB_SRR0: usize = 10 * 8;
pub const EX_TLB_SRR1: usize = 11 * 8;
pub const EX_TLB_R7: usize = 12 * 8;
pub const EX_TLB_SIZE: usize = 13 * 8;

// Assembly source macros are preserved as Rust macros.  Their instruction
// sequences intentionally remain target-specific and refer to external symbols.
#[macro_export]
macro_rules! START_EXCEPTION {
    ($label:ident) => {
        core::arch::global_asm!(concat!(
            ".globl exc_", stringify!($label), "_book3e\n",
            "exc_", stringify!($label), "_book3e:\n"
        ));
    };
}

#[macro_export]
macro_rules! TLB_MISS_PROLOG {
    () => {
        core::arch::global_asm!(
            "mtspr SPRN_SPRG_TLB_SCRATCH,r12\n\
             mfspr r12,SPRN_SPRG_TLB_EXFRAME\n\
             std r10,EX_TLB_R10(r12)\n\
             mfcr r10\n\
             std r11,EX_TLB_R11(r12)\n\
             mfspr r11,SPRN_SPRG_TLB_SCRATCH\n\
             std r13,EX_TLB_R13(r12)\n\
             mfspr r13,SPRN_SPRG_PACA\n\
             std r14,EX_TLB_R14(r12)\n\
             addi r14,r12,EX_TLB_SIZE\n\
             std r15,EX_TLB_R15(r12)\n\
             mfspr r15,SPRN_SRR1\n\
             std r16,EX_TLB_R16(r12)\n\
             mfspr r16,SPRN_SRR0\n\
             std r10,EX_TLB_CR(r12)\n\
             std r11,EX_TLB_R12(r12)\n\
             mtspr SPRN_SPRG_TLB_EXFRAME,r14\n\
             std r15,EX_TLB_SRR1(r12)\n\
             std r16,EX_TLB_SRR0(r12)"
        );
    };
}

#[macro_export]
macro_rules! TLB_MISS_RESTORE {
    ($freg:ident) => {
        core::arch::global_asm!(concat!(
            "ld r14,EX_TLB_CR(r12)\n",
            "ld r10,EX_TLB_R10(r12)\n",
            "ld r15,EX_TLB_SRR0(r12)\n",
            "ld r16,EX_TLB_SRR1(r12)\n",
            "mtspr SPRN_SPRG_TLB_EXFRAME,", stringify!($freg), "\n",
            "ld r11,EX_TLB_R11(r12)\n",
            "mtcr r14\n",
            "ld r13,EX_TLB_R13(r12)\n",
            "ld r14,EX_TLB_R14(r12)\n",
            "mtspr SPRN_SRR0,r15\n",
            "ld r15,EX_TLB_R15(r12)\n",
            "mtspr SPRN_SRR1,r16\n",
            "ld r16,EX_TLB_R16(r12)\n",
            "ld r12,EX_TLB_R12(r12)"
        ));
    };
}

#[macro_export]
macro_rules! TLB_MISS_EPILOG_SUCCESS { () => { TLB_MISS_RESTORE!(r12); }; }
#[macro_export]
macro_rules! TLB_MISS_EPILOG_ERROR {
    () => { core::arch::global_asm!("addi r12,r13,PACA_EXTLB"); TLB_MISS_RESTORE!(r12); };
}
#[macro_export]
macro_rules! TLB_MISS_EPILOG_ERROR_SPECIAL {
    () => { core::arch::global_asm!("addi r11,r13,PACA_EXTLB"); TLB_MISS_RESTORE!(r11); };
}

// C declaration: extern unsigned int interrupt_base_book3e;
extern "C" {
    pub static mut interrupt_base_book3e: u32;
}

#[macro_export]
macro_rules! SET_IVOR {
    ($vector_number:tt, $vector_offset:expr) => {
        core::arch::global_asm!(concat!(
            "LOAD_REG_ADDR(r3,interrupt_base_book3e);",
            "ori r3,r3,", stringify!($vector_offset), "@l;",
            "mtspr SPRN_IVOR", stringify!($vector_number), ",r3;"
        ));
    };
}

#[macro_export]
macro_rules! RFI_TO_KERNEL { () => { core::arch::asm!("rfi"); }; }
#[macro_export]
macro_rules! RFI_TO_USER { () => { core::arch::asm!("rfi"); }; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
