/* SPDX-License-Identifier: GPL-2.0-only */

/*
 * C header guard and assembler exclusion removed; this file contains the
 * Rust translation of the non-assembler declarations and definitions.
 */

#[cfg(feature = "CONFIG_ARCH_OMAP1")]
extern "C" {
    /* NOTE: Please use ioremap + __raw_read/write where possible instead of these */
    pub fn omap_readb(pa: u32) -> u8;
    pub fn omap_readw(pa: u32) -> u16;
    pub fn omap_readl(pa: u32) -> u32;
    pub fn omap_writeb(v: u8, pa: u32);
    pub fn omap_writew(v: u16, pa: u32);
    pub fn omap_writel(v: u32, pa: u32);
}

#[cfg(feature = "CONFIG_COMPILE_TEST")]
#[inline]
pub fn omap_readb(_pa: u32) -> u8 { 0 }
#[cfg(feature = "CONFIG_COMPILE_TEST")]
#[inline]
pub fn omap_readw(_pa: u32) -> u16 { 0 }
#[cfg(feature = "CONFIG_COMPILE_TEST")]
#[inline]
pub fn omap_readl(_pa: u32) -> u32 { 0 }
#[cfg(feature = "CONFIG_COMPILE_TEST")]
#[inline]
pub fn omap_writeb(_v: u8, _pa: u32) {}
#[cfg(feature = "CONFIG_COMPILE_TEST")]
#[inline]
pub fn omap_writew(_v: u16, _pa: u32) {}
#[cfg(feature = "CONFIG_COMPILE_TEST")]
#[inline]
pub fn omap_writel(_v: u32, _pa: u32) {}

/* System control registers */
pub const MOD_CONF_CTRL_0: u32 = 0xfffe1080;
pub const MOD_CONF_CTRL_1: u32 = 0xfffe1110;

/* UPLD */
pub const ULPD_REG_BASE: u32 = 0xfffe0800;
pub const ULPD_IT_STATUS: u32 = ULPD_REG_BASE + 0x14;
pub const ULPD_SETUP_ANALOG_CELL_3: u32 = ULPD_REG_BASE + 0x24;
pub const ULPD_CLOCK_CTRL: u32 = ULPD_REG_BASE + 0x30;
pub const DIS_USB_PVCI_CLK: u32 = 1 << 5; /* no USB/FAC synch */
pub const USB_MCLK_EN: u32 = 1 << 4; /* enable W4_USB_CLKO */
pub const ULPD_SOFT_REQ: u32 = ULPD_REG_BASE + 0x34;
pub const SOFT_UDC_REQ: u32 = 1 << 4;
pub const SOFT_USB_CLK_REQ: u32 = 1 << 3;
pub const SOFT_DPLL_REQ: u32 = 1 << 0;
pub const ULPD_DPLL_CTRL: u32 = ULPD_REG_BASE + 0x3c;
pub const ULPD_STATUS_REQ: u32 = ULPD_REG_BASE + 0x40;
pub const ULPD_APLL_CTRL: u32 = ULPD_REG_BASE + 0x4c;
pub const ULPD_POWER_CTRL: u32 = ULPD_REG_BASE + 0x50;
pub const ULPD_SOFT_DISABLE_REQ_REG: u32 = ULPD_REG_BASE + 0x68;
pub const DIS_MMC2_DPLL_REQ: u32 = 1 << 11;
pub const DIS_MMC1_DPLL_REQ: u32 = 1 << 10;
pub const DIS_UART3_DPLL_REQ: u32 = 1 << 9;
pub const DIS_UART2_DPLL_REQ: u32 = 1 << 8;
pub const DIS_UART1_DPLL_REQ: u32 = 1 << 7;
pub const DIS_USB_HOST_DPLL_REQ: u32 = 1 << 6;
pub const ULPD_SDW_CLK_DIV_CTRL_SEL: u32 = ULPD_REG_BASE + 0x74;
pub const ULPD_CAM_CLK_CTRL: u32 = ULPD_REG_BASE + 0x7c;

/* Clocks */
pub const CLKGEN_REG_BASE: u32 = 0xfffece00;
pub const ARM_CKCTL: u32 = CLKGEN_REG_BASE + 0x0;
pub const ARM_IDLECT1: u32 = CLKGEN_REG_BASE + 0x4;
pub const ARM_IDLECT2: u32 = CLKGEN_REG_BASE + 0x8;
pub const ARM_EWUPCT: u32 = CLKGEN_REG_BASE + 0xC;
pub const ARM_RSTCT1: u32 = CLKGEN_REG_BASE + 0x10;
pub const ARM_RSTCT2: u32 = CLKGEN_REG_BASE + 0x14;
pub const ARM_SYSST: u32 = CLKGEN_REG_BASE + 0x18;
pub const ARM_IDLECT3: u32 = CLKGEN_REG_BASE + 0x24;
pub const CK_RATEF: u32 = 1;
pub const CK_IDLEF: u32 = 2;
pub const CK_ENABLEF: u32 = 4;
pub const CK_SELECTF: u32 = 8;
/* #define SETARM_IDLE_SHIFT has no replacement value in the source. */
pub const DPLL_CTL: u32 = 0xfffecf00;

/* DSP clock control. Must use __raw_readw() and __raw_writew() with these */
pub const DSP_CONFIG_REG_BASE: u32 = 0xe1008000;
pub const DSP_CKCTL: u32 = DSP_CONFIG_REG_BASE + 0x0;
pub const DSP_IDLECT1: u32 = DSP_CONFIG_REG_BASE + 0x4;
pub const DSP_IDLECT2: u32 = DSP_CONFIG_REG_BASE + 0x8;
pub const DSP_RSTCT2: u32 = DSP_CONFIG_REG_BASE + 0x14;

/* Pulse-Width Light */
pub const OMAP_PWL_BASE: u32 = 0xfffb5800;
pub const OMAP_PWL_ENABLE: u32 = OMAP_PWL_BASE + 0x00;
pub const OMAP_PWL_CLK_ENABLE: u32 = OMAP_PWL_BASE + 0x04;

/* Pin multiplexing registers */
pub const FUNC_MUX_CTRL_0: u32 = 0xfffe1000;
pub const FUNC_MUX_CTRL_1: u32 = 0xfffe1004;
pub const FUNC_MUX_CTRL_2: u32 = 0xfffe1008;
pub const COMP_MODE_CTRL_0: u32 = 0xfffe100c;
pub const FUNC_MUX_CTRL_3: u32 = 0xfffe1010;
pub const FUNC_MUX_CTRL_4: u32 = 0xfffe1014;
pub const FUNC_MUX_CTRL_5: u32 = 0xfffe1018;
pub const FUNC_MUX_CTRL_6: u32 = 0xfffe101C;
pub const FUNC_MUX_CTRL_7: u32 = 0xfffe1020;
pub const FUNC_MUX_CTRL_8: u32 = 0xfffe1024;
pub const FUNC_MUX_CTRL_9: u32 = 0xfffe1028;
pub const FUNC_MUX_CTRL_A: u32 = 0xfffe102C;
pub const FUNC_MUX_CTRL_B: u32 = 0xfffe1030;
pub const FUNC_MUX_CTRL_C: u32 = 0xfffe1034;
pub const FUNC_MUX_CTRL_D: u32 = 0xfffe1038;
pub const PULL_DWN_CTRL_0: u32 = 0xfffe1040;
pub const PULL_DWN_CTRL_1: u32 = 0xfffe1044;
pub const PULL_DWN_CTRL_2: u32 = 0xfffe1048;
pub const PULL_DWN_CTRL_3: u32 = 0xfffe104c;
pub const PULL_DWN_CTRL_4: u32 = 0xfffe10ac;

/* OMAP-1610 specific multiplexing registers */
pub const FUNC_MUX_CTRL_E: u32 = 0xfffe1090;
pub const FUNC_MUX_CTRL_F: u32 = 0xfffe1094;
pub const FUNC_MUX_CTRL_10: u32 = 0xfffe1098;
pub const FUNC_MUX_CTRL_11: u32 = 0xfffe109c;
pub const FUNC_MUX_CTRL_12: u32 = 0xfffe10a0;
pub const PU_PD_SEL_0: u32 = 0xfffe10b4;
pub const PU_PD_SEL_1: u32 = 0xfffe10b8;
pub const PU_PD_SEL_2: u32 = 0xfffe10bc;
pub const PU_PD_SEL_3: u32 = 0xfffe10c0;
pub const PU_PD_SEL_4: u32 = 0xfffe10c4;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
