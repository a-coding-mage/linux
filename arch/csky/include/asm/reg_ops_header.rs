/* SPDX-License-Identifier: GPL-2.0 */

// Translated from the C header __ASM_REGS_OPS_H.

macro_rules! mfcr {
    ($reg:literal) => {{
        let mut tmp: u32;
        unsafe {
            core::arch::asm!(
                concat!("mfcr {0}, ", $reg, "\n"),
                out(reg) tmp,
            );
        }
        tmp
    }};
}

macro_rules! mtcr {
    ($reg:literal, $val:expr) => {{
        unsafe {
            core::arch::asm!(
                concat!("mtcr {0}, ", $reg, "\n"),
                in(reg) $val,
            );
        }
    }};
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
