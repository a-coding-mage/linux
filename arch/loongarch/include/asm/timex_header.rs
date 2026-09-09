/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (C) 2020-2022 Loongson Technology Corporation Limited
 */

/* Translated from the C header; kernel-only conditional preserved. */
#[cfg(__KERNEL__)]
mod kernel {
    // Dependencies supplied by the corresponding architecture headers.
    use super::{rdtime_d, rdtime_h, rdtime_l};

    pub type cycles_t = usize;

    // C: #define get_cycles get_cycles
    #[inline]
    pub unsafe fn get_cycles() -> cycles_t {
        #[cfg(CONFIG_32BIT)]
        {
            rdtime_l()
        }
        #[cfg(not(CONFIG_32BIT))]
        {
            rdtime_d()
        }
    }

    #[cfg(CONFIG_32BIT)]
    pub unsafe fn get_cycles_hi() -> cycles_t {
        rdtime_h()
    }

    #[inline]
    pub unsafe fn get_cycles64() -> u64 {
        #[cfg(CONFIG_32BIT)]
        {
            let mut hi: u32;
            let mut lo: u32;

            loop {
                hi = rdtime_h();
                lo = rdtime_l();
                if hi == rdtime_h() {
                    break;
                }
            }

            ((hi as u64) << 32) | lo as u64
        }
        #[cfg(not(CONFIG_32BIT))]
        {
            rdtime_d()
        }
    }
}

// External architecture-provided time-counter operations.
extern "Rust" {
    fn rdtime_l() -> usize;
    fn rdtime_h() -> u32;
    fn rdtime_d() -> u64;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
