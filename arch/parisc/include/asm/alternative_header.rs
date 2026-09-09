/* SPDX-License-Identifier: GPL-2.0 */

/* Translated from the PARISC alternative instruction header. */

pub const ALT_COND_ALWAYS: u8 = 0x80; /* always replace instruction */
pub const ALT_COND_NO_SMP: u8 = 0x01; /* when running UP instead of SMP */
pub const ALT_COND_NO_DCACHE: u8 = 0x02; /* if system has no d-cache */
pub const ALT_COND_NO_ICACHE: u8 = 0x04; /* if system has no i-cache */
pub const ALT_COND_NO_SPLIT_TLB: u8 = 0x08; /* if split_tlb == 0 */
pub const ALT_COND_NO_IOC_FDC: u8 = 0x10; /* if I/O cache does not need flushes */
pub const ALT_COND_RUN_ON_QEMU: u8 = 0x20; /* if running on QEMU */

pub const INSN_PxTLB: u32 = 0x02; /* modify pdtlb, pitlb */
pub const INSN_NOP: u32 = 0x08000240; /* nop */

#[repr(C, packed)]
pub struct AltInstr {
    pub orig_offset: i32, /* offset to original instructions */
    pub len: i16,         /* end of original instructions */
    pub cond: u16,       /* see ALT_COND_XXX */
    pub replacement: u32, /* replacement instruction or code */
}

extern "C" {
    pub fn set_kernel_text_rw(enable_read_write: i32);
    pub fn apply_alternatives_all();
    pub fn apply_alternatives(
        start: *mut AltInstr,
        end: *mut AltInstr,
        module_name: *const u8,
    );
}

/* Alternative SMP implementation. */
#[macro_export]
macro_rules! ALTERNATIVE {
    ($cond:expr, $replacement:expr) => {
        concat!(
            "!0:\t",
            ".section .altinstructions, \"a\"\t!",
            ".align 4\t\t\t\t!",
            ".word (0b-4-.)\t\t\t\t!",
            ".hword 1, ", stringify!($cond), "\t!",
            ".word ", stringify!($replacement), "\t!",
            ".previous"
        )
    };
}

/* To replace one single instruction by a new instruction (assembler form). */
#[macro_export]
macro_rules! ALTERNATIVE_ASM {
    ($from:expr, $to:expr, $cond:expr, $replacement:expr) => {
        concat!(
            ".section .altinstructions, \"a\"\t!",
            ".align 4\t\t\t!",
            ".word (", stringify!($from), " - .)\t!",
            ".hword (", stringify!($to), " - ", stringify!($from), ")/4, ", stringify!($cond), "\t!",
            ".word ", stringify!($replacement), "\t!",
            ".previous"
        )
    };
}

/* To replace multiple instructions by new code (assembler form). */
#[macro_export]
macro_rules! ALTERNATIVE_CODE {
    ($from:expr, $num_instructions:expr, $cond:expr, $new_instr_ptr:expr) => {
        concat!(
            ".section .altinstructions, \"a\"\t!",
            ".align 4\t\t\t!",
            ".word (", stringify!($from), " - .)\t!",
            ".hword -", stringify!($num_instructions), ", ", stringify!($cond), "\t!",
            ".word (", stringify!($new_instr_ptr), " - .)\t!",
            ".previous"
        )
    };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
