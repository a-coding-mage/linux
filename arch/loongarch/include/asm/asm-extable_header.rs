/* SPDX-License-Identifier: GPL-2.0-only */

// The following constants correspond to the C preprocessor definitions.
pub const EX_TYPE_NONE: i32 = 0;
pub const EX_TYPE_FIXUP: i32 = 1;
pub const EX_TYPE_UACCESS_ERR_ZERO: i32 = 2;
pub const EX_TYPE_BPF: i32 = 3;

// __ASSEMBLER__ form:
//
// #define __ASM_EXTABLE_RAW(insn, fixup, type, data) \
//     .pushsection __ex_table, "a"; \
//     .balign 4; \
//     .long ((insn) - .); \
//     .long ((fixup) - .); \
//     .short (type); \
//     .short (data); \
//     .popsection;
//
// .macro _asm_extable, insn, fixup
// __ASM_EXTABLE_RAW(\insn, \fixup, EX_TYPE_FIXUP, 0)
// .endm

// Non-assembler form.  These macros preserve the original generated assembly
// strings and require the external __DEFINE_ASM_GPR_NUMS dependency.
#[macro_export]
macro_rules! __ASM_EXTABLE_RAW {
    ($insn:expr, $fixup:expr, $type:expr, $data:expr) => {
        concat!(
            ".pushsection\t__ex_table, \"a\"\n",
            ".balign\t4\n",
            ".long\t((", $insn, ") - .)\n",
            ".long\t((", $fixup, ") - .)\n",
            ".short\t(", $type, ")\n",
            ".short\t(", $data, ")\n",
            ".popsection\n"
        )
    };
}

#[macro_export]
macro_rules! _ASM_EXTABLE {
    ($insn:ident, $fixup:ident) => {
        $crate::__ASM_EXTABLE_RAW!(
            stringify!($insn),
            stringify!($fixup),
            stringify!(EX_TYPE_FIXUP),
            "0"
        )
    };
}

pub const EX_DATA_REG_ERR_SHIFT: i32 = 0;
pub const EX_DATA_REG_ERR: u32 = ((1u32 << (4 - 0 + 1)) - 1) << 0;
pub const EX_DATA_REG_ZERO_SHIFT: i32 = 5;
pub const EX_DATA_REG_ZERO: u32 = ((1u32 << (9 - 5 + 1)) - 1) << 5;

#[macro_export]
macro_rules! EX_DATA_REG {
    ($reg:ident, $gpr:ident) => {
        concat!(
            "((.L__gpr_num_", stringify!($gpr), ") << ",
            stringify!(EX_DATA_REG_ $reg _SHIFT), ")"
        )
    };
}

#[macro_export]
macro_rules! _ASM_EXTABLE_UACCESS_ERR_ZERO {
    ($insn:ident, $fixup:ident, $err:ident, $zero:ident) => {
        // __DEFINE_ASM_GPR_NUMS
        $crate::__ASM_EXTABLE_RAW!(
            stringify!($insn),
            stringify!($fixup),
            stringify!(EX_TYPE_UACCESS_ERR_ZERO),
            concat!(
                "(",
                $crate::EX_DATA_REG!(ERR, $err),
                " | ",
                $crate::EX_DATA_REG!(ZERO, $zero),
                ")"
            )
        )
    };
}

#[macro_export]
macro_rules! _ASM_EXTABLE_UACCESS_ERR {
    ($insn:ident, $fixup:ident, $err:ident) => {
        // Preserves the original macro's reference to the surrounding `zero`
        // token: _ASM_EXTABLE_UACCESS_ERR_ZERO(insn, fixup, err, zero).
        $crate::_ASM_EXTABLE_UACCESS_ERR_ZERO!($insn, $fixup, $err, zero)
    };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
