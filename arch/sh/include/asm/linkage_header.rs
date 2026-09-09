/* SPDX-License-Identifier: GPL-2.0 */

// C macro: #define __ALIGN .balign 4
// This assembler directive has no direct Rust item equivalent.
#[macro_export]
macro_rules! __ALIGN {
    () => {
        ".balign 4"
    };
}

pub const __ALIGN_STR: &str = ".balign 4";

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
