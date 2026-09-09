/* SPDX-License-Identifier: GPL-2.0-only */

// Dependency intent: linux/bits.h, asm/gpr-num.h, and linux/stringify.h.

pub const EX_TYPE_NONE: u32 = 0;
pub const EX_TYPE_BPF: u32 = 1;
pub const EX_TYPE_UACCESS_ERR_ZERO: u32 = 2;
pub const EX_TYPE_KACCESS_ERR_ZERO: u32 = 3;
pub const EX_TYPE_UACCESS_CPY: u32 = 4;
pub const EX_TYPE_LOAD_UNALIGNED_ZEROPAD: u32 = 5;

/* Data fields for EX_TYPE_UACCESS_ERR_ZERO */
pub const EX_DATA_REG_ERR_SHIFT: u32 = 0;
pub const EX_DATA_REG_ERR: u32 = 0x1f;
pub const EX_DATA_REG_ZERO_SHIFT: u32 = 5;
pub const EX_DATA_REG_ZERO: u32 = 0x3e0;

/* Data fields for EX_TYPE_LOAD_UNALIGNED_ZEROPAD */
pub const EX_DATA_REG_DATA_SHIFT: u32 = 0;
pub const EX_DATA_REG_DATA: u32 = 0x1f;
pub const EX_DATA_REG_ADDR_SHIFT: u32 = 5;
pub const EX_DATA_REG_ADDR: u32 = 0x3e0;

/* Data fields for EX_TYPE_UACCESS_CPY */
pub const EX_DATA_UACCESS_WRITE: u32 = 1 << 0;

/* __ASSEMBLER__ form: these macros emit assembler exception-table entries. */
#[macro_export]
macro_rules! __ASM_EXTABLE_RAW {
    ($insn:expr, $fixup:expr, $type:expr, $data:expr) => {
        concat!(
            ".pushsection __ex_table, \"a\"\n",
            ".align 2\n",
            ".long ((", $insn, ") - .)\n",
            ".long ((", $fixup, ") - .)\n",
            ".short (", $type, ")\n",
            ".short (", $data, ")\n",
            ".popsection\n"
        )
    };
}

#[macro_export]
macro_rules! EX_DATA_REG {
    ($reg:ident, $gpr:ident) => {
        concat!("((.L__gpr_num_", stringify!($gpr), ") << ", stringify!($reg), ")")
    };
}

#[macro_export]
macro_rules! _ASM_EXTABLE_UACCESS_ERR_ZERO {
    ($insn:ident, $fixup:ident, $err:ident, $zero:ident) => {
        concat!(
            stringify!($insn), stringify!($fixup),
            EX_TYPE_UACCESS_ERR_ZERO,
            EX_DATA_REG!(ERR, $err), " | ", EX_DATA_REG!(ZERO, $zero)
        )
    };
}

#[macro_export]
macro_rules! _ASM_EXTABLE_KACCESS_ERR_ZERO {
    ($insn:ident, $fixup:ident, $err:ident, $zero:ident) => {
        concat!(
            stringify!($insn), stringify!($fixup),
            EX_TYPE_KACCESS_ERR_ZERO,
            EX_DATA_REG!(ERR, $err), " | ", EX_DATA_REG!(ZERO, $zero)
        )
    };
}

#[macro_export]
macro_rules! _ASM_EXTABLE_UACCESS_ERR { ($insn:ident, $fixup:ident, $err:ident) => { _ASM_EXTABLE_UACCESS_ERR_ZERO!($insn, $fixup, $err, wzr) }; }
#[macro_export]
macro_rules! _ASM_EXTABLE_UACCESS { ($insn:ident, $fixup:ident) => { _ASM_EXTABLE_UACCESS_ERR_ZERO!($insn, $fixup, wzr, wzr) }; }
#[macro_export]
macro_rules! _ASM_EXTABLE_KACCESS_ERR { ($insn:ident, $fixup:ident, $err:ident) => { _ASM_EXTABLE_KACCESS_ERR_ZERO!($insn, $fixup, $err, wzr) }; }
#[macro_export]
macro_rules! _ASM_EXTABLE_KACCESS { ($insn:ident, $fixup:ident) => { _ASM_EXTABLE_KACCESS_ERR_ZERO!($insn, $fixup, wzr, wzr) }; }

#[macro_export]
macro_rules! _ASM_EXTABLE_LOAD_UNALIGNED_ZEROPAD {
    ($insn:ident, $fixup:ident, $data:ident, $addr:ident) => {
        concat!(
            stringify!($insn), stringify!($fixup),
            EX_TYPE_LOAD_UNALIGNED_ZEROPAD,
            EX_DATA_REG!(DATA, $data), " | ", EX_DATA_REG!(ADDR, $addr)
        )
    };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
