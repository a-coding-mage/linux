/* SPDX-License-Identifier: GPL-2.0 */

// C preprocessor header guard: __ASM_LINKAGE_H

// #define __ALIGN .align 0
// The original macro expands to an assembler alignment directive.
#[macro_export]
macro_rules! __ALIGN {
    () => {
        ".align 0"
    };
}

// #define __ALIGN_STR ".align 0"
pub const __ALIGN_STR: &str = ".align 0";

// #define ENDPROC(name) \
//   .type name, %function; \
//   END(name)
// `END` is supplied by the surrounding assembler environment.
#[macro_export]
macro_rules! ENDPROC {
    ($name:ident) => {
        core::arch::global_asm!(concat!(
            ".type ", stringify!($name), ", %function;\\n",
            "END(", stringify!($name), ")"
        ));
    };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
