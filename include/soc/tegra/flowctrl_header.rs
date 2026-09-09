/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Functions and macros to control the flowcontroller
 *
 * Copyright (c) 2010-2012, NVIDIA Corporation. All rights reserved.
 */

pub const FLOW_CTRL_HALT_CPU0_EVENTS: u32 = 0x0;
pub const FLOW_CTRL_WAITEVENT: u32 = 2 << 29;
pub const FLOW_CTRL_WAIT_FOR_INTERRUPT: u32 = 4 << 29;
pub const FLOW_CTRL_JTAG_RESUME: u32 = 1 << 28;
pub const FLOW_CTRL_SCLK_RESUME: u32 = 1 << 27;
pub const FLOW_CTRL_HALT_CPU_IRQ: u32 = 1 << 10;
pub const FLOW_CTRL_HALT_CPU_FIQ: u32 = 1 << 8;
pub const FLOW_CTRL_HALT_LIC_IRQ: u32 = 1 << 11;
pub const FLOW_CTRL_HALT_LIC_FIQ: u32 = 1 << 10;
pub const FLOW_CTRL_HALT_GIC_IRQ: u32 = 1 << 9;
pub const FLOW_CTRL_HALT_GIC_FIQ: u32 = 1 << 8;
pub const FLOW_CTRL_CPU0_CSR: u32 = 0x8;
pub const FLOW_CTRL_CSR_INTR_FLAG: u32 = 1 << 15;
pub const FLOW_CTRL_CSR_EVENT_FLAG: u32 = 1 << 14;
pub const FLOW_CTRL_CSR_ENABLE_EXT_CRAIL: u32 = 1 << 13;
pub const FLOW_CTRL_CSR_ENABLE_EXT_NCPU: u32 = 1 << 12;
pub const FLOW_CTRL_CSR_ENABLE_EXT_MASK: u32 =
    FLOW_CTRL_CSR_ENABLE_EXT_NCPU | FLOW_CTRL_CSR_ENABLE_EXT_CRAIL;
pub const FLOW_CTRL_CSR_ENABLE: u32 = 1 << 0;
pub const FLOW_CTRL_HALT_CPU1_EVENTS: u32 = 0x14;
pub const FLOW_CTRL_CPU1_CSR: u32 = 0x18;

pub const TEGRA20_FLOW_CTRL_CSR_WFE_CPU0: u32 = 1 << 4;
pub const TEGRA20_FLOW_CTRL_CSR_WFE_BITMAP: u32 = 3 << 4;
pub const TEGRA20_FLOW_CTRL_CSR_WFI_BITMAP: u32 = 0;

pub const TEGRA30_FLOW_CTRL_CSR_WFI_CPU0: u32 = 1 << 8;
pub const TEGRA30_FLOW_CTRL_CSR_WFE_BITMAP: u32 = 0xF << 4;
pub const TEGRA30_FLOW_CTRL_CSR_WFI_BITMAP: u32 = 0xF << 8;

/* CONFIG_SOC_TEGRA_FLOWCTRL: external flow-controller implementation. */
#[cfg(feature = "CONFIG_SOC_TEGRA_FLOWCTRL")]
extern "C" {
    pub fn flowctrl_read_cpu_csr(cpuid: u32) -> u32;
    pub fn flowctrl_write_cpu_csr(cpuid: u32, value: u32);
    pub fn flowctrl_write_cpu_halt(cpuid: u32, value: u32);
    pub fn flowctrl_cpu_suspend_enter(cpuid: u32);
    pub fn flowctrl_cpu_suspend_exit(cpuid: u32);
}

#[cfg(not(feature = "CONFIG_SOC_TEGRA_FLOWCTRL"))]
#[inline]
pub fn flowctrl_read_cpu_csr(_cpuid: u32) -> u32 {
    0
}

#[cfg(not(feature = "CONFIG_SOC_TEGRA_FLOWCTRL"))]
#[inline]
pub fn flowctrl_write_cpu_csr(_cpuid: u32, _value: u32) {}

#[cfg(not(feature = "CONFIG_SOC_TEGRA_FLOWCTRL"))]
#[inline]
pub fn flowctrl_write_cpu_halt(_cpuid: u32, _value: u32) {}

#[cfg(not(feature = "CONFIG_SOC_TEGRA_FLOWCTRL"))]
#[inline]
pub fn flowctrl_cpu_suspend_enter(_cpuid: u32) {}

#[cfg(not(feature = "CONFIG_SOC_TEGRA_FLOWCTRL"))]
#[inline]
pub fn flowctrl_cpu_suspend_exit(_cpuid: u32) {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
