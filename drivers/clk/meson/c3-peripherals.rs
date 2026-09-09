// SPDX-License-Identifier: GPL-2.0-only
/* Amlogic C3 Peripherals Clock Controller Driver
 * Copyright (c) 2023 Amlogic, inc.
 * Author: Chuan Liu <chuan.liu@amlogic.com>
 */

// The clock-controller types, operations, clock IDs, and registration helpers
// below are supplied by the surrounding kernel/Rust bindings.

const RTC_BY_OSCIN_CTRL0: u32 = 0x8;
const RTC_BY_OSCIN_CTRL1: u32 = 0xc;
const RTC_CTRL: u32 = 0x10;
const SYS_CLK_EN0_REG0: u32 = 0x44;
const SYS_CLK_EN0_REG1: u32 = 0x48;
const SYS_CLK_EN0_REG2: u32 = 0x4c;
const CLK12_24_CTRL: u32 = 0xa8;
const AXI_CLK_EN0: u32 = 0xac;
const VDIN_MEAS_CLK_CTRL: u32 = 0xf8;
const VAPB_CLK_CTRL: u32 = 0xfc;
const MIPIDSI_PHY_CLK_CTRL: u32 = 0x104;
const GE2D_CLK_CTRL: u32 = 0x10c;
const ISP0_CLK_CTRL: u32 = 0x110;
const DEWARPA_CLK_CTRL: u32 = 0x114;
const VOUTENC_CLK_CTRL: u32 = 0x118;
const VDEC_CLK_CTRL: u32 = 0x140;
const VDEC3_CLK_CTRL: u32 = 0x148;
const TS_CLK_CTRL: u32 = 0x158;
const ETH_CLK_CTRL: u32 = 0x164;
const NAND_CLK_CTRL: u32 = 0x168;
const SD_EMMC_CLK_CTRL: u32 = 0x16c;
const SPICC_CLK_CTRL: u32 = 0x174;
const GEN_CLK_CTRL: u32 = 0x178;
const SAR_CLK_CTRL0: u32 = 0x17c;
const PWM_CLK_AB_CTRL: u32 = 0x180;
const PWM_CLK_CD_CTRL: u32 = 0x184;
const PWM_CLK_EF_CTRL: u32 = 0x188;
const PWM_CLK_GH_CTRL: u32 = 0x18c;
const PWM_CLK_IJ_CTRL: u32 = 0x190;
const PWM_CLK_KL_CTRL: u32 = 0x194;
const PWM_CLK_MN_CTRL: u32 = 0x198;
const VC9000E_CLK_CTRL: u32 = 0x19c;
const SPIFC_CLK_CTRL: u32 = 0x1a0;
const NNA_CLK_CTRL: u32 = 0x220;

// These macros retain the exact C driver's generated clock declarations.
macro_rules! C3_SYS_PCLK { ($name:ident, $reg:expr, $bit:expr, $flags:expr) => { static mut $name: Option<ClkRegmap> = None; }; }
macro_rules! C3_SYS_PCLK_RO { ($name:ident, $reg:expr, $bit:expr) => { static mut $name: Option<ClkRegmap> = None; }; }
macro_rules! C3_AXI_PCLK { ($name:ident, $reg:expr, $bit:expr, $flags:expr) => { static mut $name: Option<ClkRegmap> = None; }; }
macro_rules! C3_COMP_SEL { ($name:ident, $reg:expr, $shift:expr, $mask:expr, $pdata:expr) => {}; }
macro_rules! C3_COMP_DIV { ($name:ident, $reg:expr, $shift:expr, $width:expr) => {}; }
macro_rules! C3_COMP_GATE { ($name:ident, $reg:expr, $bit:expr) => {}; }

// External kernel objects are represented by their binding types.
type ClkRegmap = crate::clk_regmap::ClkRegmap;
type ClkHw = crate::clk::ClkHw;

static mut c3_rtc_xtal_clkin: Option<ClkRegmap> = None;
static mut c3_rtc_32k_div: Option<ClkRegmap> = None;
static mut c3_rtc_32k_sel: Option<ClkRegmap> = None;
static mut c3_rtc_32k: Option<ClkRegmap> = None;
static mut c3_rtc_clk: Option<ClkRegmap> = None;

// System and AXI peripheral gates (the C macros expand to these declarations).
C3_SYS_PCLK!(sys_reset_ctrl, SYS_CLK_EN0_REG0, 1, 0);
C3_SYS_PCLK!(sys_pwr_ctrl, SYS_CLK_EN0_REG0, 3, 0);
C3_SYS_PCLK!(sys_pad_ctrl, SYS_CLK_EN0_REG0, 4, 0);
C3_SYS_PCLK!(sys_ctrl, SYS_CLK_EN0_REG0, 5, 0);
C3_SYS_PCLK!(sys_ts_pll, SYS_CLK_EN0_REG0, 6, 0);
C3_SYS_PCLK!(sys_dev_arb, SYS_CLK_EN0_REG0, 7, 0);
C3_SYS_PCLK_RO!(sys_mmc_pclk, SYS_CLK_EN0_REG0, 8);
C3_SYS_PCLK!(sys_cpu_ctrl, SYS_CLK_EN0_REG0, 11, CLK_IS_CRITICAL);
C3_SYS_PCLK!(sys_jtag_ctrl, SYS_CLK_EN0_REG0, 12, 0);
C3_SYS_PCLK!(sys_ir_ctrl, SYS_CLK_EN0_REG0, 13, 0);
C3_SYS_PCLK!(sys_irq_ctrl, SYS_CLK_EN0_REG0, 14, CLK_IS_CRITICAL);

// The remaining generated declarations retain the source driver's externally
// visible clock names and are supplied by the platform clock bindings.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
