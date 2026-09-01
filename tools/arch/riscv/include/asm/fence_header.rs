/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copied from the kernel sources to tools/arch/riscv:
 */

macro_rules! RISCV_FENCE_ASM {
    ($p:tt, $s:tt) => {
        concat!("\tfence ", stringify!($p), ",", stringify!($s), "\n")
    };
}

macro_rules! RISCV_FENCE {
    ($p:tt, $s:tt) => {{
        unsafe {
            core::arch::asm!(
                RISCV_FENCE_ASM!($p, $s),
                options(nostack, preserves_flags)
            );
        }
    }};
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
