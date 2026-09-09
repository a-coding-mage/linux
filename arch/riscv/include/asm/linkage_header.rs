/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (C) 2015 Regents of the University of California
 */

// Original header guard: _ASM_RISCV_LINKAGE_H

/// Corresponds to the assembler alignment directive `.balign 4`.
#[macro_export]
macro_rules! __ALIGN {
    () => {
        core::arch::asm!(".balign 4")
    };
}

/// Corresponds to the string form of the assembler alignment directive.
pub const __ALIGN_STR: &str = ".balign 4";

/// Read the current instruction pointer using `auipc` with an immediate of 0.
#[macro_export]
macro_rules! _THIS_IP_ {
    () => {{
        let mut __ip: usize;
        unsafe {
            core::arch::asm!("auipc {0}, 0", out(reg) __ip);
        }
        __ip
    }};
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
