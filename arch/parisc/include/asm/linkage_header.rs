/* SPDX-License-Identifier: GPL-2.0 */

// Translated from the PA-RISC assembly linkage header.

#[cfg(not(any()))]
mod assembler_definitions {
    // In PA-RISC assembly a semicolon marks a comment while an exclamation
    // mark is used to separate independent lines.
    pub const __ALIGN: &str = ".align 4";
    pub const __ALIGN_STR: &str = ".align 4";
    pub const ASM_NL: char = '!';

    macro_rules! ENTRY {
        ($name:ident) => {
            concat!(__ALIGN_STR, " !\n", stringify!($name), ":\n.export ", stringify!($name))
        };
    }

    macro_rules! ENTRY_CFI {
        ($name:ident, $($callinfo:tt)*) => {
            concat!(
                ENTRY!($name), " !\n.proc !\n.callinfo ",
                stringify!($($callinfo)*), " !\n.entry !\nCFI_STARTPROC"
            )
        };
    }

    macro_rules! ENDPROC_CFI {
        ($name:ident) => {
            concat!(
                "CFI_ENDPROC !\n.exit !\n.procend !\nENDPROC(",
                stringify!($name), ")"
            )
        };
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
