/* SPDX-License-Identifier: GPL-2.0-only */

/* C header guard: __ASM_ASM_EXTABLE_H */

pub const EX_TYPE_NONE: i32 = 0;
pub const EX_TYPE_FIXUP: i32 = 1;
pub const EX_TYPE_BPF: i32 = 2;
pub const EX_TYPE_UACCESS_ERR_ZERO: i32 = 3;
pub const EX_TYPE_LOAD_UNALIGNED_ZEROPAD: i32 = 4;

/* The following declarations are active when CONFIG_MMU is enabled. */

/*
 * The assembler form emits entries into __ex_table.  It is preserved as a
 * Rust macro so assembly users retain the same source-level interface.
 */
#[cfg(feature = "CONFIG_MMU")]
macro_rules! __ASM_EXTABLE_RAW {
    ($insn:expr, $fixup:expr, $type:expr, $data:expr) => {
        concat!(
            ".pushsection __ex_table, \"a\"\n",
            ".balign 4\n",
            ".long ((", $insn, ") - .)\n",
            ".long ((", $fixup, ") - .)\n",
            ".short (", $type, ")\n",
            ".short (", $data, ")\n",
            ".popsection\n"
        )
    };
}

#[cfg(feature = "CONFIG_MMU")]
macro_rules! _ASM_EXTABLE {
    ($insn:tt, $fixup:tt) => {
        __ASM_EXTABLE_RAW!(stringify!($insn), stringify!($fixup), stringify!(EX_TYPE_FIXUP), "0")
    };
}

#[cfg(feature = "CONFIG_MMU")]
pub const EX_DATA_REG_ERR_SHIFT: u32 = 0;
#[cfg(feature = "CONFIG_MMU")]
pub const EX_DATA_REG_ERR: u32 = 0x1f;
#[cfg(feature = "CONFIG_MMU")]
pub const EX_DATA_REG_ZERO_SHIFT: u32 = 5;
#[cfg(feature = "CONFIG_MMU")]
pub const EX_DATA_REG_ZERO: u32 = 0x3e0;

#[cfg(feature = "CONFIG_MMU")]
pub const EX_DATA_REG_DATA_SHIFT: u32 = 0;
#[cfg(feature = "CONFIG_MMU")]
pub const EX_DATA_REG_DATA: u32 = 0x1f;
#[cfg(feature = "CONFIG_MMU")]
pub const EX_DATA_REG_ADDR_SHIFT: u32 = 5;
#[cfg(feature = "CONFIG_MMU")]
pub const EX_DATA_REG_ADDR: u32 = 0x3e0;

/* Depends on the external __DEFINE_ASM_GPR_NUMS assembler definitions. */
#[cfg(feature = "CONFIG_MMU")]
macro_rules! EX_DATA_REG {
    ($reg:ident, $gpr:ident) => {
        concat!(
            "((.L__gpr_num_", stringify!($gpr), ") << ",
            stringify!($reg), "_SHIFT)"
        )
    };
}

#[cfg(feature = "CONFIG_MMU")]
macro_rules! _ASM_EXTABLE_UACCESS_ERR_ZERO {
    ($insn:tt, $fixup:tt, $err:ident, $zero:ident) => {
        __ASM_EXTABLE_RAW!(
            stringify!($insn),
            stringify!($fixup),
            stringify!(EX_TYPE_UACCESS_ERR_ZERO),
            concat!("(", EX_DATA_REG!(ERR, $err), " | ", EX_DATA_REG!(ZERO, $zero), ")")
        )
    };
}

#[cfg(feature = "CONFIG_MMU")]
macro_rules! _ASM_EXTABLE_UACCESS_ERR {
    ($insn:tt, $fixup:tt, $err:ident) => {
        _ASM_EXTABLE_UACCESS_ERR_ZERO!($insn, $fixup, $err, zero)
    };
}

#[cfg(feature = "CONFIG_MMU")]
macro_rules! _ASM_EXTABLE_LOAD_UNALIGNED_ZEROPAD {
    ($insn:tt, $fixup:tt, $data:ident, $addr:ident) => {
        __ASM_EXTABLE_RAW!(
            stringify!($insn),
            stringify!($fixup),
            stringify!(EX_TYPE_LOAD_UNALIGNED_ZEROPAD),
            concat!("(", EX_DATA_REG!(DATA, $data), " | ", EX_DATA_REG!(ADDR, $addr), ")")
        )
    };
}

/* Without CONFIG_MMU, the C macro intentionally expands to nothing. */
#[cfg(not(feature = "CONFIG_MMU"))]
macro_rules! _ASM_EXTABLE_UACCESS_ERR {
    ($insn:tt, $fixup:tt, $err:ident) => {};
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
