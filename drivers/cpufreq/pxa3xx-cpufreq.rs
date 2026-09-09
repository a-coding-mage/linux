// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright (C) 2008 Marvell International Ltd.
 */

// Linux kernel dependencies supplied by other translation units.
use core::ffi::c_void;

const HSS_104M: u32 = 0;
const HSS_156M: u32 = 1;
const HSS_208M: u32 = 2;
const HSS_312M: u32 = 3;

const SMCFS_78M: u32 = 0;
const SMCFS_104M: u32 = 2;
const SMCFS_208M: u32 = 5;

const SFLFS_104M: u32 = 0;
const SFLFS_156M: u32 = 1;
const SFLFS_208M: u32 = 2;
const SFLFS_312M: u32 = 3;

const XSPCLK_156M: u32 = 0;
const XSPCLK_NONE: u32 = 3;

const DMCFS_26M: u32 = 0;
const DMCFS_260M: u32 = 3;

const ACCR_XPDIS: u32 = 1 << 31;
const ACCR_SPDIS: u32 = 1 << 30;
const ACCR_D0CS: u32 = 1 << 26;
const ACCR_PCCE: u32 = 1 << 11;
const ACCR_DDR_D0CS: u32 = 1 << 7;

const ACCR_SMCFS_MASK: u32 = 0x7 << 23;
const ACCR_SFLFS_MASK: u32 = 0x3 << 18;
const ACCR_XSPCLK_MASK: u32 = 0x3 << 16;
const ACCR_HSS_MASK: u32 = 0x3 << 14;
const ACCR_DMCFS_MASK: u32 = 0x3 << 12;
const ACCR_XN_MASK: u32 = 0x7 << 8;
const ACCR_XL_MASK: u32 = 0x1f;

const fn ACCR_SMCFS(x: u32) -> u32 { (x & 0x7) << 23 }
const fn ACCR_SFLFS(x: u32) -> u32 { (x & 0x3) << 18 }
const fn ACCR_XSPCLK(x: u32) -> u32 { (x & 0x3) << 16 }
const fn ACCR_HSS(x: u32) -> u32 { (x & 0x3) << 14 }
const fn ACCR_DMCFS(x: u32) -> u32 { (x & 0x3) << 12 }
const fn ACCR_XN(x: u32) -> u32 { (x & 0x7) << 8 }
const fn ACCR_XL(x: u32) -> u32 { x & 0x1f }

#[repr(C)]
struct pxa3xx_freq_info {
    cpufreq_mhz: u32,
    core_xl: u32,
    core_xn: u32,
    hss: u32,
    dmcfs: u32,
    smcfs: u32,
    sflfs: u32,
    df_clkdiv: u32,
    vcc_core: i32,
    vcc_sram: i32,
}

#[repr(C)]
struct cpufreq_frequency_table { driver_data: usize, frequency: u32 }
#[repr(C)]
struct cpufreq_policy {
    cpu: u32,
    cpuinfo: cpufreq_cpuinfo,
    freq_table: *mut cpufreq_frequency_table,
}
#[repr(C)]
struct cpufreq_cpuinfo { min_freq: u32, max_freq: u32, transition_latency: u32 }
#[repr(C)]
struct cpufreq_driver {
    flags: u32,
    verify: Option<unsafe extern "C" fn(*mut cpufreq_policy) -> i32>,
    target_index: Option<unsafe extern "C" fn(*mut cpufreq_policy, u32) -> i32>,
    init: Option<unsafe extern "C" fn(*mut cpufreq_policy) -> i32>,
    get: Option<unsafe extern "C" fn(u32) -> u32>,
    name: *const i8,
}

const CPUFREQ_TABLE_END: u32 = u32::MAX;
const CPUFREQ_NEED_INITIAL_FREQ_CHECK: u32 = 1 << 0;

extern "C" {
    fn pxa3xx_clk_update_accr(disable: u32, enable: u32, xclkcfg: u32, mask: u32);
    fn pxa3xx_get_clk_frequency_khz(clk: u32) -> u32;
    fn cpu_is_pxa320() -> bool;
    fn cpu_is_pxa300() -> bool;
    fn cpu_is_pxa310() -> bool;
    fn cpu_is_pxa3xx() -> bool;
    fn cpufreq_generic_frequency_table_verify(policy: *mut cpufreq_policy) -> i32;
    fn cpufreq_register_driver(driver: *mut cpufreq_driver) -> i32;
    fn cpufreq_unregister_driver(driver: *mut cpufreq_driver);
    fn local_irq_save(flags: *mut usize);
    fn local_irq_restore(flags: usize);
}

static mut pxa300_freqs: [pxa3xx_freq_info; 4] = [
    pxa3xx_freq_info { cpufreq_mhz: 104, core_xl: 8, core_xn: 1, hss: HSS_104M, dmcfs: DMCFS_260M, smcfs: SMCFS_78M, sflfs: SFLFS_104M, df_clkdiv: 3, vcc_core: 1000, vcc_sram: 1100 },
    pxa3xx_freq_info { cpufreq_mhz: 208, core_xl: 16, core_xn: 1, hss: HSS_104M, dmcfs: DMCFS_260M, smcfs: SMCFS_104M, sflfs: SFLFS_156M, df_clkdiv: 2, vcc_core: 1000, vcc_sram: 1100 },
    pxa3xx_freq_info { cpufreq_mhz: 416, core_xl: 16, core_xn: 2, hss: HSS_156M, dmcfs: DMCFS_260M, smcfs: SMCFS_104M, sflfs: SFLFS_208M, df_clkdiv: 2, vcc_core: 1100, vcc_sram: 1200 },
    pxa3xx_freq_info { cpufreq_mhz: 624, core_xl: 24, core_xn: 2, hss: HSS_208M, dmcfs: DMCFS_260M, smcfs: SMCFS_208M, sflfs: SFLFS_312M, df_clkdiv: 3, vcc_core: 1375, vcc_sram: 1400 },
];
static mut pxa320_freqs: [pxa3xx_freq_info; 5] = [pxa300_freqs[0], pxa300_freqs[1], pxa300_freqs[2], pxa300_freqs[3], pxa3xx_freq_info { cpufreq_mhz: 806, core_xl: 31, core_xn: 2, hss: HSS_208M, dmcfs: DMCFS_260M, smcfs: SMCFS_208M, sflfs: SFLFS_312M, df_clkdiv: 3, vcc_core: 1400, vcc_sram: 1400 }];
static mut pxa3xx_freqs_num: u32 = 0;
static mut pxa3xx_freqs: *mut pxa3xx_freq_info = core::ptr::null_mut();
static mut pxa3xx_freqs_table: *mut cpufreq_frequency_table = core::ptr::null_mut();

unsafe fn setup_freqs_table(policy: *mut cpufreq_policy, freqs: *mut pxa3xx_freq_info, num: i32) -> i32 {
    let table = kzalloc((num as usize + 1) * core::mem::size_of::<cpufreq_frequency_table>()) as *mut cpufreq_frequency_table;
    if table.is_null() { return -12; }
    for i in 0..num {
        let t = table.add(i as usize);
        (*t).driver_data = i as usize;
        (*t).frequency = (*freqs.add(i as usize)).cpufreq_mhz * 1000;
    }
    (*table.add(num as usize)).driver_data = num as usize;
    (*table.add(num as usize)).frequency = CPUFREQ_TABLE_END;
    pxa3xx_freqs = freqs; pxa3xx_freqs_num = num as u32; pxa3xx_freqs_table = table; (*policy).freq_table = table; 0
}

unsafe fn __update_core_freq(info: *mut pxa3xx_freq_info) {
    let mask = ACCR_XN_MASK | ACCR_XL_MASK;
    let disable = mask | ACCR_XSPCLK_MASK;
    let mut enable = ACCR_XN((*info).core_xn) | ACCR_XL((*info).core_xl);
    enable |= ACCR_XSPCLK(XSPCLK_NONE);
    let xclkcfg = if (*info).core_xn == 2 { 0x3 } else { 0x2 };
    pxa3xx_clk_update_accr(disable, enable, xclkcfg, mask);
}

unsafe fn __update_bus_freq(info: *mut pxa3xx_freq_info) {
    let mask = ACCR_SMCFS_MASK | ACCR_SFLFS_MASK | ACCR_HSS_MASK | ACCR_DMCFS_MASK;
    let enable = ACCR_SMCFS((*info).smcfs) | ACCR_SFLFS((*info).sflfs) | ACCR_HSS((*info).hss) | ACCR_DMCFS((*info).dmcfs);
    pxa3xx_clk_update_accr(mask, enable, 0, mask);
}

unsafe extern "C" fn pxa3xx_cpufreq_get(_cpu: u32) -> u32 { pxa3xx_get_clk_frequency_khz(0) }
unsafe extern "C" fn pxa3xx_cpufreq_set(policy: *mut cpufreq_policy, index: u32) -> i32 {
    if (*policy).cpu != 0 { return -22; }
    let next = pxa3xx_freqs.add(index as usize); let mut flags = 0usize;
    local_irq_save(&mut flags); __update_core_freq(next); __update_bus_freq(next); local_irq_restore(flags); 0
}
unsafe extern "C" fn pxa3xx_cpufreq_init(policy: *mut cpufreq_policy) -> i32 {
    (*policy).cpuinfo.min_freq = 104000; (*policy).cpuinfo.max_freq = if cpu_is_pxa320() { 806000 } else { 624000 }; (*policy).cpuinfo.transition_latency = 1000;
    let mut ret = -22;
    if cpu_is_pxa300() || cpu_is_pxa310() { ret = setup_freqs_table(policy, pxa300_freqs.as_mut_ptr(), 4); }
    if cpu_is_pxa320() { ret = setup_freqs_table(policy, pxa320_freqs.as_mut_ptr(), 5); }
    if ret != 0 { return ret; } 0
}

static mut pxa3xx_cpufreq_driver: cpufreq_driver = cpufreq_driver { flags: CPUFREQ_NEED_INITIAL_FREQ_CHECK, verify: Some(cpufreq_generic_frequency_table_verify), target_index: Some(pxa3xx_cpufreq_set), init: Some(pxa3xx_cpufreq_init), get: Some(pxa3xx_cpufreq_get), name: b"pxa3xx-cpufreq\0".as_ptr() as *const i8 };
unsafe extern "C" fn cpufreq_init() -> i32 { if cpu_is_pxa3xx() { cpufreq_register_driver(&mut pxa3xx_cpufreq_driver) } else { 0 } }
unsafe extern "C" fn cpufreq_exit() { cpufreq_unregister_driver(&mut pxa3xx_cpufreq_driver); }

extern "C" { fn kzalloc(size: usize) -> *mut c_void; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
