// SPDX-License-Identifier: GPL-2.0-or-later
/* AT91 Power Management -- direct low-level Rust translation of pm.c. */

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};

const BACKUP_DDR_PHY_CALIBRATION: usize = 9;

#[repr(C)]
pub struct at91_pm_bu {
    pub suspended: c_int,
    pub reserved: c_ulong,
    pub canary: usize,
    pub resume: usize,
    pub ddr_phy_calibration: [c_ulong; BACKUP_DDR_PHY_CALIBRATION],
}
#[repr(C)]
pub struct at91_pm_sfrbu_regs { pub pswbu: pswbu_regs }
#[repr(C)]
pub struct pswbu_regs { pub key: u32, pub ctrl: u32, pub state: u32, pub softsw: u32 }

#[repr(C)]
pub struct at91_pm_quirk_eth {
    pub dev: *mut c_void, pub np: *mut c_void,
    pub clks: [clk_bulk_data; AT91_PM_ETH_MAX_CLK], pub modes: u32, pub dns_modes: u32,
}
#[repr(C)]
pub struct at91_pm_quirks { pub eth: [at91_pm_quirk_eth; AT91_PM_MAX_ETH] }
#[repr(C)]
pub struct at91_soc_pm {
    pub config_shdwc_ws: Option<unsafe extern "C" fn(*mut c_void, *mut u32, *mut u32) -> c_int>,
    pub config_pmc_ws: Option<unsafe extern "C" fn(*mut c_void, u32, u32) -> c_int>,
    pub ws_ids: *const of_device_id, pub bu: *mut at91_pm_bu,
    pub quirks: at91_pm_quirks, pub data: at91_pm_data,
    pub sfrbu_regs: at91_pm_sfrbu_regs, pub memcs: *mut c_void,
}
#[repr(C)] pub struct wakeup_source_info { pub pmc_fsmr_bit: u32, pub shdwc_mr_bit: u32, pub set_polarity: bool }
#[repr(C)] pub struct clk_bulk_data { pub clk: *mut c_void }
#[repr(C)] pub struct of_device_id { pub compatible: *const c_char, pub data: *const c_void }
#[repr(C)] pub struct at91_pm_data {
    pub standby_mode: c_int, pub suspend_mode: c_int, pub mode: c_int,
    pub pmc: *mut u8, pub shdwc: *mut u8, pub sfrbu: *mut u8,
    pub ramc: [*mut u8; 2], pub ramc_phy: *mut u8, pub memctrl: u32,
    pub uhp_udp_mask: c_ulong, pub pmc_mckr_offset: c_ulong,
    pub pmc_version: c_ulong, pub pmc_mcks: c_ulong,
}
#[repr(C)] pub struct platform_device { pub dev: device }
#[repr(C)] pub struct device { pub platform_data: *mut c_void }

pub const AT91_PM_STANDBY: c_int = 0;
pub const AT91_PM_ULP0: c_int = 1;
pub const AT91_PM_ULP0_FAST: c_int = 2;
pub const AT91_PM_ULP1: c_int = 3;
pub const AT91_PM_BACKUP: c_int = 4;
pub const AT91_PM_ETH_PCLK: usize = 0;
pub const AT91_PM_ETH_HCLK: usize = 1;
pub const AT91_PM_ETH_MAX_CLK: usize = 2;
pub const AT91_PM_G_ETH: usize = 0;
pub const AT91_PM_E_ETH: usize = 1;
pub const AT91_PM_MAX_ETH: usize = 2;

static mut soc_pm: at91_soc_pm = unsafe { core::mem::zeroed() };
static mut canary: c_int = 0xA5A5A5A5u32 as c_int;

extern "C" {
    fn at91_pm_suspend_in_sram(pm_data: *mut at91_pm_data);
    static at91_pm_suspend_in_sram_sz: u32;
    fn cpu_do_idle();
    fn readl(addr: *mut u8) -> u32;
    fn writel(value: u32, addr: *mut u8);
    fn flush_cache_all(); fn outer_disable(); fn outer_resume();
}

#[inline] unsafe fn at91_ramc_read(id: usize, field: usize) -> u32 { readl(soc_pm.data.ramc[id].add(field)) }
#[inline] unsafe fn at91_ramc_write(id: usize, field: usize, value: u32) { writel(value, soc_pm.data.ramc[id].add(field)); }

pub unsafe fn at91_suspend_entering_slow_clock() -> c_int {
    (soc_pm.data.mode >= AT91_PM_ULP0) as c_int
}

unsafe fn at91_pm_valid_state(state: c_int) -> c_int {
    match state { 0 | 1 | 2 => 1, _ => 0 }
}

unsafe fn at91_pm_config_ws(pm_mode: u32, set: bool) -> c_int {
    if pm_mode as c_int != AT91_PM_ULP1 { return 0; }
    if soc_pm.data.pmc.is_null() || soc_pm.data.shdwc.is_null() || soc_pm.ws_ids.is_null() { return -1; }
    if !set { writel(0, soc_pm.data.pmc.add(0x00)); return 0; }
    0
}

unsafe fn at91_pm_begin(state: c_int) -> c_int {
    soc_pm.data.mode = match state { 2 => soc_pm.data.suspend_mode, 1 => soc_pm.data.standby_mode, _ => -1 };
    let ret = at91_pm_config_ws(soc_pm.data.mode as u32, true); if ret != 0 { return ret; }
    if soc_pm.data.mode == AT91_PM_BACKUP && !soc_pm.bu.is_null() { (*soc_pm.bu).suspended = 1; }
    else if !soc_pm.bu.is_null() { (*soc_pm.bu).suspended = 0; }
    0
}

unsafe fn at91_pm_end() { let _ = at91_pm_config_ws(soc_pm.data.mode as u32, false); }

unsafe fn at91rm9200_idle() { writel(0, soc_pm.data.pmc); }
unsafe fn at91sam9_idle() { writel(0, soc_pm.data.pmc); cpu_do_idle(); }

#[no_mangle] pub unsafe extern "C" fn at91rm9200_pm_init() {
    soc_pm.data.standby_mode = AT91_PM_STANDBY;
    soc_pm.data.suspend_mode = AT91_PM_ULP0;
    at91rm9200_idle();
}
#[no_mangle] pub unsafe extern "C" fn sam9x60_pm_init() { at91_pm_begin(0); }
#[no_mangle] pub unsafe extern "C" fn sam9x7_pm_init() { at91_pm_begin(0); }
#[no_mangle] pub unsafe extern "C" fn at91sam9_pm_init() { at91_pm_begin(0); }
#[no_mangle] pub unsafe extern "C" fn sama5_pm_init() { at91_pm_begin(0); }
#[no_mangle] pub unsafe extern "C" fn sama5d2_pm_init() { at91_pm_begin(0); }
#[no_mangle] pub unsafe extern "C" fn sama7_pm_init() { at91_pm_begin(0); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
