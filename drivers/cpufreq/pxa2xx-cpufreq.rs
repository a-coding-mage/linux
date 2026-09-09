// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *  Copyright (C) 2002,2003 Intrinsyc Software
 *
 * History:
 *   31-Jul-2002 : Initial version [FB]
 *   29-Jan-2003 : added PXA255 support [FB]
 *   20-Apr-2003 : ported to v2.5 (Dustin McIntire, Sensoria Corp.)
 *
 * Note:
 *   This driver may change the memory bus clock rate, but will not do any
 *   platform specific access timing changes... for example if you have flash
 *   memory connected to CS0, you will need to register a platform specific
 *   notifier which will adjust the memory access strobes to maintain a
 *   minimum strobe width.
 */

// C dependencies supplied by the surrounding kernel translation.

#[cfg(feature = "DEBUG")]
static mut freq_debug: u32 = 0;
#[cfg(not(feature = "DEBUG"))]
const freq_debug: u32 = 0;

extern "C" {
    static mut vcc_core: *mut regulator;
    static mut pxa27x_maxfreq: u32;
    static mut pxa255_turbo_table: u32;
}

#[repr(C)]
pub struct regulator { _private: [u8; 0] }
#[repr(C)]
pub struct clk { _private: [u8; 0] }
#[repr(C)]
pub struct cpufreq_policy {
    pub cpuinfo: cpufreq_cpuinfo,
    pub cur: u32,
    pub freq_table: *mut cpufreq_frequency_table,
}
#[repr(C)]
pub struct cpufreq_cpuinfo { pub transition_latency: u32 }
#[repr(C)]
pub struct cpufreq_frequency_table { pub frequency: u32, pub driver_data: usize }
#[repr(C)]
pub struct cpufreq_driver {
    pub flags: u32,
    pub verify: Option<unsafe extern "C" fn(*mut cpufreq_policy) -> i32>,
    pub target_index: Option<unsafe extern "C" fn(*mut cpufreq_policy, u32) -> i32>,
    pub init: Option<unsafe extern "C" fn(*mut cpufreq_policy) -> i32>,
    pub get: Option<unsafe extern "C" fn(u32) -> u32>,
    pub name: *const u8,
    pub driver_data: *mut pxa_cpufreq_data,
}

#[repr(C)]
struct pxa_cpufreq_data { clk_core: *mut clk }
static mut pxa_cpufreq_data: pxa_cpufreq_data = pxa_cpufreq_data { clk_core: core::ptr::null_mut() };

#[repr(C)]
struct pxa_freqs { khz: u32, vmin: i32, vmax: i32 }

static pxa255_run_freqs: [pxa_freqs; 6] = [
    pxa_freqs { khz: 99500, vmin: -1, vmax: -1 }, pxa_freqs { khz: 132700, vmin: -1, vmax: -1 },
    pxa_freqs { khz: 199100, vmin: -1, vmax: -1 }, pxa_freqs { khz: 265400, vmin: -1, vmax: -1 },
    pxa_freqs { khz: 331800, vmin: -1, vmax: -1 }, pxa_freqs { khz: 398100, vmin: -1, vmax: -1 },
];
static pxa255_turbo_freqs: [pxa_freqs; 5] = [
    pxa_freqs { khz: 99500, vmin: -1, vmax: -1 }, pxa_freqs { khz: 199100, vmin: -1, vmax: -1 },
    pxa_freqs { khz: 298500, vmin: -1, vmax: -1 }, pxa_freqs { khz: 298600, vmin: -1, vmax: -1 },
    pxa_freqs { khz: 398100, vmin: -1, vmax: -1 },
];
static mut pxa255_run_freq_table: [cpufreq_frequency_table; 7] = [cpufreq_frequency_table { frequency: 0, driver_data: 0 }; 7];
static mut pxa255_turbo_freq_table: [cpufreq_frequency_table; 6] = [cpufreq_frequency_table { frequency: 0, driver_data: 0 }; 6];
static mut pxa27x_freqs: [pxa_freqs; 7] = [
    pxa_freqs { khz: 104000, vmin: 900000, vmax: 1705000 }, pxa_freqs { khz: 156000, vmin: 1000000, vmax: 1705000 },
    pxa_freqs { khz: 208000, vmin: 1180000, vmax: 1705000 }, pxa_freqs { khz: 312000, vmin: 1250000, vmax: 1705000 },
    pxa_freqs { khz: 416000, vmin: 1350000, vmax: 1705000 }, pxa_freqs { khz: 520000, vmin: 1450000, vmax: 1705000 },
    pxa_freqs { khz: 624000, vmin: 1550000, vmax: 1705000 },
];
static mut pxa27x_freq_table: [cpufreq_frequency_table; 8] = [cpufreq_frequency_table { frequency: 0, driver_data: 0 }; 8];

extern "C" {
    fn cpu_is_pxa25x() -> bool; fn cpu_is_pxa27x() -> bool; fn BUG() -> !;
    fn clk_get_rate(c: *mut clk) -> u64; fn clk_set_rate(c: *mut clk, rate: u64) -> i32;
    fn cpufreq_get_driver_data() -> *mut pxa_cpufreq_data;
    fn regulator_set_voltage(r: *mut regulator, min: i32, max: i32) -> i32;
    fn regulator_get(dev: *mut core::ffi::c_void, name: *const u8) -> *mut regulator;
    fn clk_get_sys(dev: *const u8, name: *const u8) -> *mut clk;
    fn cpufreq_register_driver(d: *mut cpufreq_driver) -> i32;
    fn cpufreq_unregister_driver(d: *mut cpufreq_driver);
    fn cpufreq_generic_frequency_table_verify(p: *mut cpufreq_policy) -> i32;
    fn pr_info(fmt: *const u8, ...); fn pr_err(fmt: *const u8, ...); fn pr_debug(fmt: *const u8, ...);
    fn PTR_ERR(p: *mut core::ffi::c_void) -> i32;
}
const CPUFREQ_TABLE_END: u32 = u32::MAX;
const CPUFREQ_NEED_INITIAL_FREQ_CHECK: u32 = 1;

#[cfg(feature = "CONFIG_REGULATOR")]
unsafe fn pxa_cpufreq_change_voltage(f: *const pxa_freqs) -> i32 {
    if !cpu_is_pxa27x() || (*f).vmin == -1 || (*f).vmax == -1 { return 0; }
    let ret = regulator_set_voltage(vcc_core, (*f).vmin, (*f).vmax);
    if ret != 0 { pr_err(b"Failed to set vcc_core in [%dmV..%dmV]\0".as_ptr(), (*f).vmin, (*f).vmax); }
    ret
}
#[cfg(feature = "CONFIG_REGULATOR")]
unsafe fn pxa_cpufreq_init_voltages() {
    vcc_core = regulator_get(core::ptr::null_mut(), b"vcc_core\0".as_ptr());
    if vcc_core.is_null() { pr_info(b"Didn't find vcc_core regulator\n\0".as_ptr()); }
    else { pr_info(b"Found vcc_core regulator\n\0".as_ptr()); }
}
#[cfg(not(feature = "CONFIG_REGULATOR"))]
unsafe fn pxa_cpufreq_change_voltage(_: *const pxa_freqs) -> i32 { 0 }
#[cfg(not(feature = "CONFIG_REGULATOR"))]
unsafe fn pxa_cpufreq_init_voltages() {}

unsafe fn find_freq_tables(table: *mut *mut cpufreq_frequency_table, freqs: *mut *const pxa_freqs) {
    if cpu_is_pxa25x() { if pxa255_turbo_table == 0 { *freqs = pxa255_run_freqs.as_ptr(); *table = pxa255_run_freq_table.as_mut_ptr(); } else { *freqs = pxa255_turbo_freqs.as_ptr(); *table = pxa255_turbo_freq_table.as_mut_ptr(); } }
    else if cpu_is_pxa27x() { *freqs = pxa27x_freqs.as_ptr(); *table = pxa27x_freq_table.as_mut_ptr(); }
    else { BUG(); }
}
unsafe fn pxa27x_guess_max_freq() { if pxa27x_maxfreq == 0 { pxa27x_maxfreq = 416000; } else { pxa27x_maxfreq *= 1000; } }
unsafe extern "C" fn pxa_cpufreq_get(_: u32) -> u32 { (clk_get_rate((*cpufreq_get_driver_data()).clk_core) / 1000) as u32 }
unsafe extern "C" fn pxa_set_target(policy: *mut cpufreq_policy, idx: u32) -> i32 {
    let mut table = core::ptr::null_mut(); let mut settings = core::ptr::null(); find_freq_tables(&mut table, &mut settings);
    let new_freq = (*settings.add(idx as usize)).khz; let data = cpufreq_get_driver_data();
    if !vcc_core.is_null() && new_freq > (*policy).cur { if pxa_cpufreq_change_voltage(settings.add(idx as usize)) != 0 { return -1; } }
    clk_set_rate((*data).clk_core, (new_freq as u64) * 1000);
    if !vcc_core.is_null() && new_freq < (*policy).cur { let _ = pxa_cpufreq_change_voltage(settings.add(idx as usize)); } 0
}

unsafe extern "C" fn pxa_cpufreq_init(policy: *mut cpufreq_policy) -> i32 {
    if cpu_is_pxa27x() { pxa27x_guess_max_freq(); } pxa_cpufreq_init_voltages(); (*policy).cpuinfo.transition_latency = 1000;
    for i in 0..6 { pxa255_run_freq_table[i] = cpufreq_frequency_table { frequency: pxa255_run_freqs[i].khz, driver_data: i }; } pxa255_run_freq_table[6].frequency = CPUFREQ_TABLE_END;
    for i in 0..5 { pxa255_turbo_freq_table[i] = cpufreq_frequency_table { frequency: pxa255_turbo_freqs[i].khz, driver_data: i }; } pxa255_turbo_freq_table[5].frequency = CPUFREQ_TABLE_END;
    pxa255_turbo_table = (pxa255_turbo_table != 0) as u32;
    let mut i = 0; while i < 7 && pxa27x_freqs[i].khz <= pxa27x_maxfreq { pxa27x_freq_table[i] = cpufreq_frequency_table { frequency: pxa27x_freqs[i].khz, driver_data: i }; i += 1; } pxa27x_freq_table[i].driver_data = i; pxa27x_freq_table[i].frequency = CPUFREQ_TABLE_END;
    if cpu_is_pxa25x() { let mut t = core::ptr::null_mut(); let mut f = core::ptr::null(); find_freq_tables(&mut t, &mut f); (*policy).freq_table = t; } else if cpu_is_pxa27x() { (*policy).freq_table = pxa27x_freq_table.as_mut_ptr(); } 0
}

static mut pxa_cpufreq_driver: cpufreq_driver = cpufreq_driver { flags: CPUFREQ_NEED_INITIAL_FREQ_CHECK, verify: Some(cpufreq_generic_frequency_table_verify), target_index: Some(pxa_set_target), init: Some(pxa_cpufreq_init), get: Some(pxa_cpufreq_get), name: b"PXA2xx\0".as_ptr(), driver_data: core::ptr::null_mut() };
unsafe extern "C" fn pxa_cpu_init() -> i32 { pxa_cpufreq_data.clk_core = clk_get_sys(core::ptr::null(), b"core\0".as_ptr()); if pxa_cpufreq_data.clk_core.is_null() { return -19; } if cpu_is_pxa25x() || cpu_is_pxa27x() { cpufreq_register_driver(&mut pxa_cpufreq_driver) } else { -19 } }
unsafe extern "C" fn pxa_cpu_exit() { cpufreq_unregister_driver(&mut pxa_cpufreq_driver); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
