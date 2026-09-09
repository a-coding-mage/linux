/* SPDX-License-Identifier: GPL-2.0 */

// C header dependencies:
// #include <linux/objtool.h>
// #include <asm/orc_types.h>

// The following assembler macros are represented as Rust macro definitions.
// They preserve the original UNWIND_HINT invocations and rely on the
// corresponding externally supplied Rust macro and constants.

#[macro_export]
macro_rules! UNWIND_HINT_UNDEFINED {
    () => {
        UNWIND_HINT!(type = UNWIND_HINT_TYPE_UNDEFINED);
    };
}

#[macro_export]
macro_rules! UNWIND_HINT_END_OF_STACK {
    () => {
        UNWIND_HINT!(type = UNWIND_HINT_TYPE_END_OF_STACK);
    };
}

#[macro_export]
macro_rules! UNWIND_HINT_REGS {
    () => {
        UNWIND_HINT!(sp_reg = ORC_REG_SP, type = UNWIND_HINT_TYPE_REGS);
    };
}

#[macro_export]
macro_rules! UNWIND_HINT_FUNC {
    () => {
        UNWIND_HINT!(sp_reg = ORC_REG_SP, type = UNWIND_HINT_TYPE_CALL);
    };
}

// !__ASSEMBLER__ branch of the original header.

#[macro_export]
macro_rules! UNWIND_HINT_SAVE {
    () => {
        UNWIND_HINT!(UNWIND_HINT_TYPE_SAVE, 0, 0, 0);
    };
}

#[macro_export]
macro_rules! UNWIND_HINT_RESTORE {
    () => {
        UNWIND_HINT!(UNWIND_HINT_TYPE_RESTORE, 0, 0, 0);
    };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
