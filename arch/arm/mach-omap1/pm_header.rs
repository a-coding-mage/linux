/*
 * arch/arm/mach-omap1/pm.h
 *
 * Header file for OMAP1 Power Management Routines
 *
 * Author: MontaVista Software, Inc.
 *      support@mvista.com
 *
 * Copyright 2002 MontaVista Software Inc.
 *
 * Cleanup 2004 for Linux 2.6 by Dirk Behme <dirk.behme@de.bosch.com>
 */

// Dependency supplied by the surrounding OMAP1 code: OMAP1_IO_ADDRESS.

pub const CLKGEN_REG_ASM_BASE: _ = OMAP1_IO_ADDRESS(0xfffece00);
pub const ARM_IDLECT1_ASM_OFFSET: u32 = 0x04;
pub const ARM_IDLECT2_ASM_OFFSET: u32 = 0x08;

pub const TCMIF_ASM_BASE: _ = OMAP1_IO_ADDRESS(0xfffecc00);
pub const EMIFS_CONFIG_ASM_OFFSET: u32 = 0x0c;
pub const EMIFF_SDRAM_CONFIG_ASM_OFFSET: u32 = 0x20;

pub const IDLE_WAIT_CYCLES: u32 = 0x00000fff;
pub const PERIPHERAL_ENABLE: u32 = 0x2;

pub const SELF_REFRESH_MODE: u32 = 0x0c000001;
pub const IDLE_EMIFS_REQUEST: u32 = 0xc;
pub const MODEM_32K_EN: u32 = 0x1;
pub const PER_EN: u32 = 0x1;

pub const CPU_SUSPEND_SIZE: u32 = 200;
pub const ULPD_LOW_PWR_EN: u32 = 0x0001;
pub const ULPD_DEEP_SLEEP_TRANSITION_EN: u32 = 0x0010;
pub const ULPD_SETUP_ANALOG_CELL_3_VAL: u32 = 0;
pub const ULPD_POWER_CTRL_REG_VAL: u32 = 0x0219;

pub const DSP_IDLE_DELAY: u32 = 10;
pub const DSP_IDLE: u32 = 0x0040;
pub const DSP_RST: u32 = 0x0004;
pub const DSP_ENABLE: u32 = 0x0002;
pub const SUFFICIENT_DSP_RESET_TIME: u32 = 1000;
pub const DEFAULT_MPUI_CONFIG: u32 = 0x05cf;
pub const ENABLE_XORCLK: u32 = 0x2;
pub const DSP_CLOCK_ENABLE: u32 = 0x2000;
pub const DSP_IDLE_MODE: u32 = 0x2;
pub const TC_IDLE_REQUEST: u32 = 0x0000000c;

pub const IRQ_LEVEL2: u32 = 1 << 0;
pub const IRQ_KEYBOARD: u32 = 1 << 1;
pub const IRQ_UART2: u32 = 1 << 15;

pub const PDE_BIT: u32 = 0x08;
pub const PWD_EN_BIT: u32 = 0x04;
pub const EN_PERCK_BIT: u32 = 0x04;

pub const OMAP1510_DEEP_SLEEP_REQUEST: u32 = 0x0ec7;
pub const OMAP1510_BIG_SLEEP_REQUEST: u32 = 0x0cc5;
pub const OMAP1510_IDLE_LOOP_REQUEST: u32 = 0x0c00;
pub const OMAP1510_IDLE_CLOCK_DOMAINS: u32 = 0x2;

pub const OMAP1610_IDLECT1_SLEEP_VAL: u32 = 0x13c7;
pub const OMAP1610_IDLECT2_SLEEP_VAL: u32 = 0x09c7;
pub const OMAP1610_IDLECT3_VAL: u32 = 0x3f;
pub const OMAP1610_IDLECT3_SLEEP_ORMASK: u32 = 0x2c;
pub const OMAP1610_IDLECT3: u32 = 0xfffece24;
pub const OMAP1610_IDLE_LOOP_REQUEST: u32 = 0x0400;

extern "C" {
    pub static mut power_subsys: kset;
    pub fn prevent_idle_sleep();
    pub fn allow_idle_sleep();
    pub fn omap1_pm_idle();
    pub fn omap1_pm_suspend();
    pub fn omap1510_cpu_suspend(arg1: c_ulong, arg2: c_ulong);
    pub fn omap1610_cpu_suspend(arg1: c_ulong, arg2: c_ulong);
    pub static mut omap1510_cpu_suspend_sz: c_uint;
    pub static mut omap1610_cpu_suspend_sz: c_uint;
}

#[cfg(CONFIG_OMAP_SERIAL_WAKE)]
extern "C" {
    pub fn omap_serial_wake_trigger(enable: c_int);
}

#[cfg(not(CONFIG_OMAP_SERIAL_WAKE))]
#[macro_export]
macro_rules! omap_serial_wakeup_init { () => {}; }
#[cfg(not(CONFIG_OMAP_SERIAL_WAKE))]
#[macro_export]
macro_rules! omap_serial_wake_trigger { ($x:expr) => {}; }

#[macro_export]
macro_rules! ARM_SAVE { ($x:expr) => { arm_sleep_save[ARM_SLEEP_SAVE_$x] = omap_readl($x) }; }
#[macro_export]
macro_rules! ARM_RESTORE { ($x:expr) => { omap_writel(arm_sleep_save[ARM_SLEEP_SAVE_$x], $x) }; }
#[macro_export]
macro_rules! ARM_SHOW { ($x:expr) => { arm_sleep_save[ARM_SLEEP_SAVE_$x] }; }
#[macro_export]
macro_rules! DSP_SAVE { ($x:expr) => { dsp_sleep_save[DSP_SLEEP_SAVE_$x] = __raw_readw($x) }; }
#[macro_export]
macro_rules! DSP_RESTORE { ($x:expr) => { __raw_writew(dsp_sleep_save[DSP_SLEEP_SAVE_$x], $x) }; }
#[macro_export]
macro_rules! DSP_SHOW { ($x:expr) => { dsp_sleep_save[DSP_SLEEP_SAVE_$x] }; }
#[macro_export]
macro_rules! ULPD_SAVE { ($x:expr) => { ulpd_sleep_save[ULPD_SLEEP_SAVE_$x] = omap_readw($x) }; }
#[macro_export]
macro_rules! ULPD_RESTORE { ($x:expr) => { omap_writew(ulpd_sleep_save[ULPD_SLEEP_SAVE_$x], $x) }; }
#[macro_export]
macro_rules! ULPD_SHOW { ($x:expr) => { ulpd_sleep_save[ULPD_SLEEP_SAVE_$x] }; }
#[macro_export]
macro_rules! MPUI1510_SAVE { ($x:expr) => { mpui1510_sleep_save[MPUI1510_SLEEP_SAVE_$x] = omap_readl($x) }; }
#[macro_export]
macro_rules! MPUI1510_RESTORE { ($x:expr) => { omap_writel(mpui1510_sleep_save[MPUI1510_SLEEP_SAVE_$x], $x) }; }
#[macro_export]
macro_rules! MPUI1510_SHOW { ($x:expr) => { mpui1510_sleep_save[MPUI1510_SLEEP_SAVE_$x] }; }
#[macro_export]
macro_rules! MPUI1610_SAVE { ($x:expr) => { mpui1610_sleep_save[MPUI1610_SLEEP_SAVE_$x] = omap_readl($x) }; }
#[macro_export]
macro_rules! MPUI1610_RESTORE { ($x:expr) => { omap_writel(mpui1610_sleep_save[MPUI1610_SLEEP_SAVE_$x], $x) }; }
#[macro_export]
macro_rules! MPUI1610_SHOW { ($x:expr) => { mpui1610_sleep_save[MPUI1610_SLEEP_SAVE_$x] }; }

#[repr(C)]
pub enum arm_save_state {
    ARM_SLEEP_SAVE_START = 0,
    ARM_SLEEP_SAVE_ARM_CKCTL,
    ARM_SLEEP_SAVE_ARM_IDLECT1,
    ARM_SLEEP_SAVE_ARM_IDLECT2,
    ARM_SLEEP_SAVE_ARM_IDLECT3,
    ARM_SLEEP_SAVE_ARM_EWUPCT,
    ARM_SLEEP_SAVE_ARM_RSTCT1,
    ARM_SLEEP_SAVE_ARM_RSTCT2,
    ARM_SLEEP_SAVE_ARM_SYSST,
    ARM_SLEEP_SAVE_SIZE,
}

#[repr(C)]
pub enum dsp_save_state { DSP_SLEEP_SAVE_START = 0, DSP_SLEEP_SAVE_DSP_IDLECT2, DSP_SLEEP_SAVE_SIZE }

#[repr(C)]
pub enum ulpd_save_state {
    ULPD_SLEEP_SAVE_START = 0,
    ULPD_SLEEP_SAVE_ULPD_IT_STATUS,
    ULPD_SLEEP_SAVE_ULPD_CLOCK_CTRL,
    ULPD_SLEEP_SAVE_ULPD_SOFT_REQ,
    ULPD_SLEEP_SAVE_ULPD_STATUS_REQ,
    ULPD_SLEEP_SAVE_ULPD_DPLL_CTRL,
    ULPD_SLEEP_SAVE_ULPD_POWER_CTRL,
    ULPD_SLEEP_SAVE_SIZE,
}

#[repr(C)]
pub enum mpui1510_save_state {
    MPUI1510_SLEEP_SAVE_START = 0,
    MPUI1510_SLEEP_SAVE_MPUI_CTRL,
    MPUI1510_SLEEP_SAVE_MPUI_DSP_BOOT_CONFIG,
    MPUI1510_SLEEP_SAVE_MPUI_DSP_API_CONFIG,
    MPUI1510_SLEEP_SAVE_MPUI_DSP_STATUS,
    MPUI1510_SLEEP_SAVE_EMIFF_SDRAM_CONFIG,
    MPUI1510_SLEEP_SAVE_EMIFS_CONFIG,
    MPUI1510_SLEEP_SAVE_OMAP_IH1_MIR,
    MPUI1510_SLEEP_SAVE_OMAP_IH2_MIR,
    // CONFIG_ARCH_OMAP15XX: size is the preceding enumerator; otherwise it is zero.
    #[cfg(CONFIG_ARCH_OMAP15XX)] MPUI1510_SLEEP_SAVE_SIZE,
    #[cfg(not(CONFIG_ARCH_OMAP15XX))] MPUI1510_SLEEP_SAVE_SIZE = 0,
}

#[repr(C)]
pub enum mpui1610_save_state {
    MPUI1610_SLEEP_SAVE_START = 0,
    MPUI1610_SLEEP_SAVE_MPUI_CTRL,
    MPUI1610_SLEEP_SAVE_MPUI_DSP_BOOT_CONFIG,
    MPUI1610_SLEEP_SAVE_MPUI_DSP_API_CONFIG,
    MPUI1610_SLEEP_SAVE_MPUI_DSP_STATUS,
    MPUI1610_SLEEP_SAVE_EMIFF_SDRAM_CONFIG,
    MPUI1610_SLEEP_SAVE_EMIFS_CONFIG,
    MPUI1610_SLEEP_SAVE_OMAP_IH1_MIR,
    MPUI1610_SLEEP_SAVE_OMAP_IH2_0_MIR,
    MPUI1610_SLEEP_SAVE_OMAP_IH2_1_MIR,
    MPUI1610_SLEEP_SAVE_OMAP_IH2_2_MIR,
    MPUI1610_SLEEP_SAVE_OMAP_IH2_3_MIR,
    // CONFIG_ARCH_OMAP16XX: size is the preceding enumerator; otherwise it is zero.
    #[cfg(CONFIG_ARCH_OMAP16XX)] MPUI1610_SLEEP_SAVE_SIZE,
    #[cfg(not(CONFIG_ARCH_OMAP16XX))] MPUI1610_SLEEP_SAVE_SIZE = 0,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
