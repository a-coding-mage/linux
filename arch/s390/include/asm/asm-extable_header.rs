/* SPDX-License-Identifier: GPL-2.0 */
// Translated from asm-extable.h.  The original include dependencies provide
// stringify and assembly-constant helpers; Rust's stringify! is used here.

pub const EX_TYPE_NONE: u32 = 0;
pub const EX_TYPE_FIXUP: u32 = 1;
pub const EX_TYPE_BPF: u32 = 2;
pub const EX_TYPE_UA_FAULT: u32 = 3;
pub const EX_TYPE_UA_LOAD_REG: u32 = 5;
pub const EX_TYPE_UA_LOAD_REGPAIR: u32 = 6;
pub const EX_TYPE_ZEROPAD: u32 = 7;
pub const EX_TYPE_FPC: u32 = 8;
pub const EX_TYPE_UA_MVCOS_TO: u32 = 9;
pub const EX_TYPE_UA_MVCOS_FROM: u32 = 10;

pub const EX_DATA_REG_ERR_SHIFT: u32 = 0;
pub const EX_DATA_REG_ERR: u32 = 0xF;
pub const EX_DATA_REG_ADDR_SHIFT: u32 = 4;
pub const EX_DATA_REG_ADDR: u32 = 0xF0;
pub const EX_DATA_LEN_SHIFT: u32 = 8;
pub const EX_DATA_LEN: u32 = 0xF00;

// The source macro emits s390 assembler into the exception-table section.
// Keep the assembler structure literal; its arguments are converted to text
// at expansion time, matching the C stringify/inline-assembly behavior.
#[macro_export]
macro_rules! __EX_TABLE {
    ($section:ident, $fault:tt, $target:tt, $type:expr, $regerr:tt, $regaddr:tt, $len:expr) => {
        core::arch::global_asm!(concat!(
            ".section ", stringify!($section), ",\"a\";\n",
            ".balign 4;\n",
            ".long (", stringify!($fault), ") - .;\n",
            ".long (", stringify!($target), ") - .;\n",
            ".short (", stringify!($type), ");\n",
            ".short (", stringify!($regerr), ") << ", stringify!(EX_DATA_REG_ERR_SHIFT),
            " | (", stringify!($regaddr), ") << ", stringify!(EX_DATA_REG_ADDR_SHIFT),
            " | ", stringify!($len), " << ", stringify!(EX_DATA_LEN_SHIFT), ";\n",
            ".previous\n"
        ));
    };
}

#[macro_export]
macro_rules! EX_TABLE {
    ($fault:tt, $target:tt) => {
        $crate::__EX_TABLE!(__ex_table, $fault, $target, $crate::EX_TYPE_FIXUP, %%r0, %%r0, 0)
    };
}

#[macro_export]
macro_rules! EX_TABLE_AMODE31 {
    ($fault:tt, $target:tt) => {
        $crate::__EX_TABLE!(.amode31.ex_table, $fault, $target, $crate::EX_TYPE_FIXUP, %%r0, %%r0, 0)
    };
}

#[macro_export]
macro_rules! EX_TABLE_UA_FAULT {
    ($fault:tt, $target:tt, $regerr:tt) => {
        $crate::__EX_TABLE!(__ex_table, $fault, $target, $crate::EX_TYPE_UA_FAULT, $regerr, $regerr, 0)
    };
}

#[macro_export]
macro_rules! EX_TABLE_UA_LOAD_REG {
    ($fault:tt, $target:tt, $regerr:tt, $regzero:tt) => {
        $crate::__EX_TABLE!(__ex_table, $fault, $target, $crate::EX_TYPE_UA_LOAD_REG, $regerr, $regzero, 0)
    };
}

#[macro_export]
macro_rules! EX_TABLE_UA_LOAD_REGPAIR {
    ($fault:tt, $target:tt, $regerr:tt, $regzero:tt) => {
        $crate::__EX_TABLE!(__ex_table, $fault, $target, $crate::EX_TYPE_UA_LOAD_REGPAIR, $regerr, $regzero, 0)
    };
}

#[macro_export]
macro_rules! EX_TABLE_ZEROPAD {
    ($fault:tt, $target:tt, $regdata:tt, $regaddr:tt) => {
        $crate::__EX_TABLE!(__ex_table, $fault, $target, $crate::EX_TYPE_ZEROPAD, $regdata, $regaddr, 0)
    };
}

#[macro_export]
macro_rules! EX_TABLE_FPC {
    ($fault:tt, $target:tt) => {
        $crate::__EX_TABLE!(__ex_table, $fault, $target, $crate::EX_TYPE_FPC, %%r0, %%r0, 0)
    };
}

#[macro_export]
macro_rules! EX_TABLE_UA_MVCOS_TO {
    ($fault:tt, $target:tt) => {
        $crate::__EX_TABLE!(__ex_table, $fault, $target, $crate::EX_TYPE_UA_MVCOS_TO, %%r0, %%r0, 0)
    };
}

#[macro_export]
macro_rules! EX_TABLE_UA_MVCOS_FROM {
    ($fault:tt, $target:tt) => {
        $crate::__EX_TABLE!(__ex_table, $fault, $target, $crate::EX_TYPE_UA_MVCOS_FROM, %%r0, %%r0, 0)
    };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
