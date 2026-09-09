/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (c) 2010-2013, NVIDIA Corporation. All rights reserved.
 */

// Dependencies supplied by the corresponding iomap and irammap headers.

pub const TEGRA_ARM_PERIF_VIRT: usize =
    TEGRA_ARM_PERIF_BASE - IO_CPU_PHYS + IO_CPU_VIRT;
pub const TEGRA_FLOW_CTRL_VIRT: usize =
    TEGRA_FLOW_CTRL_BASE - IO_PPSB_PHYS + IO_PPSB_VIRT;
pub const TEGRA_CLK_RESET_VIRT: usize =
    TEGRA_CLK_RESET_BASE - IO_PPSB_PHYS + IO_PPSB_VIRT;
pub const TEGRA_APB_MISC_VIRT: usize =
    TEGRA_APB_MISC_BASE - IO_APB_PHYS + IO_APB_VIRT;
pub const TEGRA_PMC_VIRT: usize = TEGRA_PMC_BASE - IO_APB_PHYS + IO_APB_VIRT;

pub const TEGRA_IRAM_RESET_BASE_VIRT: usize =
    IO_IRAM_VIRT + TEGRA_IRAM_RESET_HANDLER_OFFSET;

/* PMC_SCRATCH37-39 and 41 are used for tegra_pen_lock and idle */
pub const PMC_SCRATCH37: u32 = 0x130;
pub const PMC_SCRATCH38: u32 = 0x134;
pub const PMC_SCRATCH39: u32 = 0x138;
pub const PMC_SCRATCH41: u32 = 0x140;

// Defined only when CONFIG_ARCH_TEGRA_2x_SOC is enabled.
#[cfg(CONFIG_ARCH_TEGRA_2x_SOC)]
pub const CPU_RESETTABLE: u32 = 2;
#[cfg(CONFIG_ARCH_TEGRA_2x_SOC)]
pub const CPU_RESETTABLE_SOON: u32 = 1;
#[cfg(CONFIG_ARCH_TEGRA_2x_SOC)]
pub const CPU_NOT_RESETTABLE: u32 = 0;

/* flag of tegra_disable_clean_inv_dcache to do LoUIS or all */
pub const TEGRA_FLUSH_CACHE_LOUIS: u32 = 0;
pub const TEGRA_FLUSH_CACHE_ALL: u32 = 1;

/* Macro interfaces defined for ARM assembly in the original header:
 * wait_until, cpu_to_halt_reg, cpu_to_csr_reg, cpu_id, mov32,
 * check_cpu_part_num, exit_smp, and tegra_get_soc_id.
 * Their instruction-level definitions are intentionally retained here as
 * comments because Rust has no direct equivalent for assembler macros.
 */

pub const APB_MISC_GP_HIDREV: u32 = 0x804;

unsafe extern "C" {
    pub fn tegra_resume();
    pub fn tegra_sleep_cpu_finish(arg: core::ffi::c_ulong) -> core::ffi::c_int;
    pub fn tegra_disable_clean_inv_dcache(flag: u32);

    pub fn tegra20_hotplug_shutdown();
    pub fn tegra30_hotplug_shutdown();

    pub fn tegra20_tear_down_cpu();
    pub fn tegra30_tear_down_cpu();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
