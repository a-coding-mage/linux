/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * arch/arm/mach-tegra/reset.h
 *
 * CPU reset dispatcher.
 *
 * Copyright (c) 2011, NVIDIA Corporation.
 */

/* The C header guard is intentionally omitted in Rust. */

pub const TEGRA_RESET_MASK_PRESENT: usize = 0;
pub const TEGRA_RESET_MASK_LP1: usize = 1;
pub const TEGRA_RESET_MASK_LP2: usize = 2;
pub const TEGRA_RESET_STARTUP_SECONDARY: usize = 3;
pub const TEGRA_RESET_STARTUP_LP2: usize = 4;
pub const TEGRA_RESET_STARTUP_LP1: usize = 5;
pub const TEGRA_RESET_TF_PRESENT: usize = 6;
pub const TEGRA_RESET_DATA_SIZE: usize = 7;

macro_rules! RESET_DATA {
    (MASK_PRESENT) => { TEGRA_RESET_MASK_PRESENT * 4 };
    (MASK_LP1) => { TEGRA_RESET_MASK_LP1 * 4 };
    (MASK_LP2) => { TEGRA_RESET_MASK_LP2 * 4 };
    (STARTUP_SECONDARY) => { TEGRA_RESET_STARTUP_SECONDARY * 4 };
    (STARTUP_LP2) => { TEGRA_RESET_STARTUP_LP2 * 4 };
    (STARTUP_LP1) => { TEGRA_RESET_STARTUP_LP1 * 4 };
    (TF_PRESENT) => { TEGRA_RESET_TF_PRESENT * 4 };
    (DATA_SIZE) => { TEGRA_RESET_DATA_SIZE * 4 };
}

/* `irammap.h` supplies TEGRA_IRAM_BASE, TEGRA_IRAM_RESET_HANDLER_OFFSET,
 * IO_ADDRESS, and the related platform types. */

extern "C" {
    pub static mut __tegra_cpu_reset_handler_data: [core::ffi::c_ulong; TEGRA_RESET_DATA_SIZE];

    pub fn __tegra_cpu_reset_handler_start();
    pub fn __tegra_cpu_reset_handler();
    pub fn __tegra20_cpu1_resettable_status_offset();
    pub fn __tegra_cpu_reset_handler_end();
    pub fn tegra_cpu_reset_handler_init();
}

/* CONFIG_PM_SLEEP controls whether these C macros are available. */
#[cfg(feature = "CONFIG_PM_SLEEP")]
macro_rules! tegra_cpu_lp1_mask {
    () => {
        IO_ADDRESS(
            TEGRA_IRAM_BASE + TEGRA_IRAM_RESET_HANDLER_OFFSET
                + ((unsafe { &__tegra_cpu_reset_handler_data[TEGRA_RESET_MASK_LP1] }
                    as *const _ as u32)
                    .wrapping_sub(__tegra_cpu_reset_handler_start as usize as u32)),
        )
    };
}

#[cfg(feature = "CONFIG_PM_SLEEP")]
macro_rules! tegra_cpu_lp2_mask {
    () => {
        IO_ADDRESS(
            TEGRA_IRAM_BASE + TEGRA_IRAM_RESET_HANDLER_OFFSET
                + ((unsafe { &__tegra_cpu_reset_handler_data[TEGRA_RESET_MASK_LP2] }
                    as *const _ as u32)
                    .wrapping_sub(__tegra_cpu_reset_handler_start as usize as u32)),
        )
    };
}

pub unsafe fn tegra_cpu_reset_handler_offset() -> u32 {
    (__tegra_cpu_reset_handler as usize as u32)
        .wrapping_sub(__tegra_cpu_reset_handler_start as usize as u32)
}

/* Preserves the original pointer subtraction; the symbols are supplied externally. */
pub unsafe fn tegra_cpu_reset_handler_size() -> usize {
    (__tegra_cpu_reset_handler_end as usize)
        .wrapping_sub(__tegra_cpu_reset_handler_start as usize)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
