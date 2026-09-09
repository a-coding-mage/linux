/* SPDX-License-Identifier: GPL-2.0-only */

/*
 * The C header's include guard is omitted; Rust items are scoped by the
 * containing module.
 */

/* Equivalent of RISCV_FENCE_ASM(p, s). */
macro_rules! RISCV_FENCE_ASM {
    ($p:ident, $s:ident) => {
        concat!("\tfence ", stringify!($p), ",", stringify!($s), "\n")
    };
}

/* Equivalent of RISCV_FENCE(p, s), including the volatile memory barrier. */
macro_rules! RISCV_FENCE {
    ($p:ident, $s:ident) => {{
        unsafe {
            core::arch::asm!(
                concat!("fence ", stringify!($p), ",", stringify!($s)),
                options(nostack)
            );
        }
    }};
}

/* CONFIG_SMP selects the SMP barrier strings at build time. */
#[cfg(CONFIG_SMP)]
pub const RISCV_ACQUIRE_BARRIER: &str = RISCV_FENCE_ASM!(r, rw);
#[cfg(CONFIG_SMP)]
pub const RISCV_RELEASE_BARRIER: &str = RISCV_FENCE_ASM!(rw, w);
#[cfg(CONFIG_SMP)]
pub const RISCV_FULL_BARRIER: &str = RISCV_FENCE_ASM!(rw, rw);

#[cfg(not(CONFIG_SMP))]
pub const RISCV_ACQUIRE_BARRIER: &str = "";
#[cfg(not(CONFIG_SMP))]
pub const RISCV_RELEASE_BARRIER: &str = "";
#[cfg(not(CONFIG_SMP))]
pub const RISCV_FULL_BARRIER: &str = "";

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
