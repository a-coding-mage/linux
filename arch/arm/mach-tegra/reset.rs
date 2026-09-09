// SPDX-License-Identifier: GPL-2.0-only
/*
 * arch/arm/mach-tegra/reset.c
 *
 * Copyright (C) 2011,2012 NVIDIA Corporation.
 */

// Linux dependencies supplied by the surrounding translated codebase.

const TEGRA_IRAM_RESET_BASE: usize = TEGRA_IRAM_BASE + TEGRA_IRAM_RESET_HANDLER_OFFSET;

static mut IS_ENABLED: bool = false;

unsafe fn tegra_cpu_reset_handler_set(reset_address: u32) {
    let evp_cpu_reset: *mut core::ffi::c_void =
        IO_ADDRESS(TEGRA_EXCEPTION_VECTORS_BASE + 0x100);
    let sb_ctrl: *mut core::ffi::c_void = IO_ADDRESS(TEGRA_SB_BASE);
    let mut reg: u32;

    /*
     * NOTE: This must be the one and only write to the EVP CPU reset
     *       vector in the entire system.
     */
    writel(reset_address, evp_cpu_reset);
    wmb();
    reg = readl(evp_cpu_reset);

    /*
     * Prevent further modifications to the physical reset vector.
     *  NOTE: Has no effect on chips prior to Tegra30.
     */
    reg = readl(sb_ctrl);
    reg |= 2;
    writel(reg, sb_ctrl);
    wmb();
}

unsafe fn tegra_cpu_reset_handler_enable() {
    let iram_base: *mut core::ffi::c_void = IO_ADDRESS(TEGRA_IRAM_RESET_BASE);
    let reset_address: u32 = (TEGRA_IRAM_RESET_BASE + tegra_cpu_reset_handler_offset) as u32;
    let err: i32;

    BUG_ON(IS_ENABLED);
    BUG_ON(tegra_cpu_reset_handler_size > TEGRA_IRAM_RESET_HANDLER_SIZE);

    memcpy_toio(
        iram_base,
        __tegra_cpu_reset_handler_start as *const core::ffi::c_void,
        tegra_cpu_reset_handler_size,
    );

    err = call_firmware_op(set_cpu_boot_addr, 0, reset_address);
    match err {
        -ENOSYS => {
            tegra_cpu_reset_handler_set(reset_address);
            // fallthrough
            IS_ENABLED = true;
        }
        0 => {
            IS_ENABLED = true;
        }
        _ => {
            pr_crit!("Cannot set CPU reset handler: %d\n", err);
            BUG();
        }
    }
}

pub unsafe fn tegra_cpu_reset_handler_init() {
    __tegra_cpu_reset_handler_data[TEGRA_RESET_TF_PRESENT] =
        trusted_foundations_registered();

    // #ifdef CONFIG_SMP
    __tegra_cpu_reset_handler_data[TEGRA_RESET_MASK_PRESENT] =
        *(cpu_possible_mask as *const u32);
    __tegra_cpu_reset_handler_data[TEGRA_RESET_STARTUP_SECONDARY] =
        __pa_symbol(secondary_startup as *const core::ffi::c_void);
    // #endif

    // #ifdef CONFIG_PM_SLEEP
    __tegra_cpu_reset_handler_data[TEGRA_RESET_STARTUP_LP1] =
        TEGRA_IRAM_LPx_RESUME_AREA;
    __tegra_cpu_reset_handler_data[TEGRA_RESET_STARTUP_LP2] =
        __pa_symbol(tegra_resume as *const core::ffi::c_void);
    // #endif

    tegra_cpu_reset_handler_enable();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
