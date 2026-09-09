// SPDX-License-Identifier: GPL-2.0-only
// Copyright (C) 2020-2021 Samuel Holland <samuel@sholland.org>

// Translated from the Linux kernel C implementation. External kernel types and
// functions are intentionally referenced but not implemented here.

const MBUS_CR: usize = 0x0000;
const MBUS_TMR: usize = 0x000c;
const MBUS_PMU_CFG: usize = 0x009c;
const MBUS_PMU_BWCR_BASE: usize = 0x00a0;
const MBUS_MDFSCR: usize = 0x0100;
const MBUS_MDFSMRMR: usize = 0x0108;
const DRAM_PWRCTL: usize = 0x0004;
const DRAM_RFSHTMG: usize = 0x0090;
const DRAM_VTFCR: usize = 0x00b8;
const DRAM_ODTMAP: usize = 0x0120;
const DRAM_DX_MAX: u32 = 4;

const MBUS_PMU_CFG_UNIT_KB: u32 = 0x1 << 1;
const MBUS_PMU_CFG_ENABLE: u32 = 0x1;
const MBUS_MDFSCR_BUFFER_TIMING: u32 = 0x1 << 15;
const MBUS_MDFSCR_PAD_HOLD: u32 = 0x1 << 13;
const MBUS_MDFSCR_BYPASS: u32 = 0x1 << 4;
const MBUS_MDFSCR_MODE_DFS: u32 = 0;
const MBUS_MDFSCR_START: u32 = 1;
const DRAM_PWRCTL_SELFREF_EN: u32 = 1;
const DRAM_VTFCR_VTF_ENABLE: u32 = 0x3 << 8;
const DRAM_DXnGCR0_DXODT: u32 = 0x3 << 4;
const DRAM_DXnGCR0_DXODT_DYNAMIC: u32 = 0;
const DRAM_DXnGCR0_DXODT_DISABLED: u32 = 0x2 << 4;
const DRAM_DXnGCR0_DXEN: u32 = 1;
const USEC_PER_SEC: u64 = 1_000_000;

#[repr(C)]
pub struct Sun8iA33MbusVariant { pub min_dram_divider: u32, pub max_dram_divider: u32, pub odt_freq_mhz: u32 }

#[repr(C)]
pub struct Sun8iA33Mbus {
    pub variant: *const Sun8iA33MbusVariant,
    pub reg_dram: *mut u8,
    pub reg_mbus: *mut u8,
    pub clk_bus: *mut Clk,
    pub clk_dram: *mut Clk,
    pub clk_mbus: *mut Clk,
    pub devfreq_dram: *mut Devfreq,
    pub gov_data: DevfreqSimpleOndemandData,
    pub profile: DevfreqDevProfile,
    pub data_width: u32,
    pub nominal_bw: u32,
    pub odtmap: u32,
    pub tREFI_ns: u32,
    pub tRFC_ns: u32,
    pub freq_table: [u64; 0],
}

#[repr(C)] pub struct Clk;
#[repr(C)] pub struct Device;
#[repr(C)] pub struct PlatformDevice { pub dev: Device }
#[repr(C)] pub struct Devfreq;
#[repr(C)] pub struct DevfreqSimpleOndemandData { pub upthreshold: u32, pub downdifferential: u32 }
#[repr(C)] pub struct DevfreqDevProfile { pub initial_freq: u64, pub polling_ms: u32, pub target: Option<unsafe extern "C" fn(*mut Device, *mut u64, u32) -> i32>, pub get_dev_status: Option<unsafe extern "C" fn(*mut Device, *mut DevfreqDevStatus) -> i32>, pub freq_table: *mut u64, pub max_state: u32 }
#[repr(C)] pub struct DevfreqDevStatus { pub busy_time: u64, pub total_time: u64, pub current_frequency: u64 }

extern "C" {
    fn readl_relaxed(addr: *mut u8) -> u32; fn readl(addr: *mut u8) -> u32;
    fn writel(value: u32, addr: *mut u8); fn writel_relaxed(value: u32, addr: *mut u8);
    fn clk_set_rate(clk: *mut Clk, rate: u64) -> i32; fn clk_get_rate(clk: *mut Clk) -> u64;
    fn clk_get_parent(clk: *mut Clk) -> *mut Clk;
    fn dev_get_drvdata(dev: *mut Device) -> *mut Sun8iA33Mbus;
    fn readl_poll_timeout_atomic(addr: *mut u8, value: *mut u32, condition: bool, delay: u32, timeout: u32) -> i32;
    fn devfreq_recommended_opp(dev: *mut Device, freq: *mut u64, flags: u32) -> *mut Opaque;
    fn dev_pm_opp_put(opp: *mut Opaque); fn ptr_err(p: *mut Opaque) -> i32;
    fn devfreq_remove_device(df: *mut Devfreq); fn dev_pm_opp_remove_all_dynamic(dev: *mut Device);
}
#[repr(C)] pub struct Opaque;

static mut pmu_period: i32 = 50000;
#[inline] fn mbus_pmu_bwcr(n: usize) -> usize { MBUS_PMU_BWCR_BASE + 4 * n }
#[inline] fn dram_dxn_gcr0(n: usize) -> usize { 0x0344 + 0x80 * n }
#[inline] fn add(p: *mut u8, o: usize) -> *mut u8 { unsafe { p.add(o) } }
#[inline] fn mbus_cr_get_dram_type(x: u32) -> u32 { (x >> 16) & 7 }

unsafe fn sun8i_a33_mbus_get_peak_bw(priv_: *mut Sun8iA33Mbus) -> u32 { readl_relaxed(add((*priv_).reg_mbus, mbus_pmu_bwcr(5))) }
unsafe fn sun8i_a33_mbus_restart_pmu_counters(priv_: *mut Sun8iA33Mbus) { let cfg = (((pmu_period as u32).wrapping_sub(1)) << 16) | MBUS_PMU_CFG_UNIT_KB; writel_relaxed(cfg, add((*priv_).reg_mbus, MBUS_PMU_CFG)); writel_relaxed(cfg | MBUS_PMU_CFG_ENABLE, add((*priv_).reg_mbus, MBUS_PMU_CFG)); }
unsafe fn sun8i_a33_mbus_update_nominal_bw(priv_: *mut Sun8iA33Mbus, ddr_freq_mhz: u32) { (*priv_).nominal_bw = ddr_freq_mhz * pmu_period as u32 * (*priv_).data_width / 1024; }

unsafe fn sun8i_a33_mbus_set_dram_freq(priv_: *mut Sun8iA33Mbus, freq: u64) -> i32 {
    let ddr = (freq / USEC_PER_SEC) as u32; let dram = ddr / 2; let mctl = dram / 2;
    let ret = clk_set_rate((*priv_).clk_dram, freq); if ret != 0 { return ret; }
    let mut pwr = readl_relaxed(add((*priv_).reg_dram, DRAM_PWRCTL)) & !DRAM_PWRCTL_SELFREF_EN;
    writel_relaxed(pwr, add((*priv_).reg_dram, DRAM_PWRCTL)); let vtf = readl_relaxed(add((*priv_).reg_dram, DRAM_VTFCR)); writel_relaxed(vtf & !DRAM_VTFCR_VTF_ENABLE, add((*priv_).reg_dram, DRAM_VTFCR));
    let mdfs = MBUS_MDFSCR_MODE_DFS | MBUS_MDFSCR_BYPASS | MBUS_MDFSCR_PAD_HOLD | MBUS_MDFSCR_BUFFER_TIMING; writel(mdfs, add((*priv_).reg_mbus, MBUS_MDFSCR));
    let trefi = (*priv_).tREFI_ns * mctl / 1000 / 32; let trfc = ((*priv_).tRFC_ns * mctl + 999) / 1000; writel((trefi << 16) | trfc, add((*priv_).reg_dram, DRAM_RFSHTMG));
    let dxodt = if (*priv_).odtmap != 0 && dram > (*(*priv_).variant).odt_freq_mhz { writel((*priv_).odtmap, add((*priv_).reg_dram, DRAM_ODTMAP)); DRAM_DXnGCR0_DXODT_DYNAMIC } else { writel(0, add((*priv_).reg_dram, DRAM_ODTMAP)); DRAM_DXnGCR0_DXODT_DISABLED };
    let mut i = 0; while i < DRAM_DX_MAX { let r = add((*priv_).reg_dram, dram_dxn_gcr0(i as usize)); writel((readl(r) & !DRAM_DXnGCR0_DXODT) | dxodt, r); i += 1; }
    writel(mdfs | MBUS_MDFSCR_START, add((*priv_).reg_mbus, MBUS_MDFSCR)); let mut status = 0; let ret = readl_poll_timeout_atomic(add((*priv_).reg_mbus, MBUS_MDFSCR), &mut status, (status & MBUS_MDFSCR_START) == 0, 10, 1000); if ret != 0 { return ret; }
    writel(0, add((*priv_).reg_mbus, MBUS_MDFSCR)); writel_relaxed(vtf, add((*priv_).reg_dram, DRAM_VTFCR)); if freq == *(*priv_).freq_table { pwr |= DRAM_PWRCTL_SELFREF_EN; } writel_relaxed(pwr, add((*priv_).reg_dram, DRAM_PWRCTL)); sun8i_a33_mbus_restart_pmu_counters(priv_); sun8i_a33_mbus_update_nominal_bw(priv_, ddr); 0
}

unsafe fn sun8i_a33_mbus_get_dram_status(_dev: *mut Device, stat: *mut DevfreqDevStatus, priv_: *mut Sun8iA33Mbus) -> i32 { (*stat).busy_time = sun8i_a33_mbus_get_peak_bw(priv_) as u64; (*stat).total_time = (*priv_).nominal_bw as u64; sun8i_a33_mbus_restart_pmu_counters(priv_); 0 }

unsafe fn sun8i_a33_mbus_set_dram_target(dev: *mut Device, freq: *mut u64, flags: u32) -> i32 {
    let priv_ = dev_get_drvdata(dev); let ret = sun8i_a33_mbus_set_dram_freq(priv_, *freq); if ret != 0 { return ret; } 0
}

unsafe fn sun8i_a33_mbus_hw_init(_dev: *mut Device, priv_: *mut Sun8iA33Mbus, ddr_freq: u64) -> i32 {
    let cr = readl_relaxed(add((*priv_).reg_mbus, MBUS_CR)); match mbus_cr_get_dram_type(cr) { 2 | 3 | 4 => { (*priv_).tREFI_ns = 7800; (*priv_).tRFC_ns = 350; }, 6 | 7 => { (*priv_).tREFI_ns = 3900; (*priv_).tRFC_ns = 210; }, _ => return -22 }
    (*priv_).odtmap = readl_relaxed(add((*priv_).reg_dram, DRAM_ODTMAP)); let mut i = 0; while i < DRAM_DX_MAX { if readl_relaxed(add((*priv_).reg_dram, dram_dxn_gcr0(i as usize))) & DRAM_DXnGCR0_DXEN == 0 { break; } i += 1; } (*priv_).data_width = i; let mbus_freq = clk_get_rate((*priv_).clk_mbus) / USEC_PER_SEC; writel_relaxed((mbus_freq as u32).wrapping_sub(1), add((*priv_).reg_mbus, MBUS_TMR)); writel_relaxed(0xffff_ffff, add((*priv_).reg_mbus, MBUS_MDFSMRMR)); sun8i_a33_mbus_restart_pmu_counters(priv_); sun8i_a33_mbus_update_nominal_bw(priv_, (ddr_freq / USEC_PER_SEC) as u32); 0
}

unsafe fn sun8i_a33_mbus_suspend(_dev: *mut Device) -> i32 { 0 }
unsafe fn sun8i_a33_mbus_resume(_dev: *mut Device) -> i32 { 0 }

#[repr(C)] pub struct Sun50iA64Mbus { pub min_dram_divider: u32, pub max_dram_divider: u32, pub odt_freq_mhz: u32 }
static SUN50I_A64_MBUS: Sun50iA64Mbus = Sun50iA64Mbus { min_dram_divider: 1, max_dram_divider: 4, odt_freq_mhz: 400 };

#[no_mangle] pub unsafe extern "C" fn sun8i_a33_mbus_probe(pdev: *mut PlatformDevice) -> i32 { let _ = pdev; -38 }
#[no_mangle] pub unsafe extern "C" fn sun8i_a33_mbus_remove(pdev: *mut PlatformDevice) { let _ = pdev; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
