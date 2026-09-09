// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright (C) 2007 PA Semi, Inc
 *
 * Authors: Egor Martovetsky <egor@pasemi.com>
 *          Olof Johansson <olof@lixom.net>
 *
 * Maintained by: Olof Johansson <olof@lixom.net>
 *
 * Based on arch/powerpc/platforms/cell/cbe_cpufreq.c:
 * (C) Copyright IBM Deutschland Entwicklung GmbH 2005
 */

// Kernel dependencies supplied by other translation units.
use core::ffi::c_void;

const SDCASR_REG: usize = 0x0100;
const SDCASR_REG_STRIDE: usize = 0x1000;
const SDCPWR_CFGA0_REG: usize = 0x0100;
const SDCPWR_PWST0_REG: usize = 0x0000;
const SDCPWR_GIZTIME_REG: usize = 0x0440;
const SDCPWR_GIZTIME_GR: u32 = 0x80000000;
const SDCPWR_GIZTIME_LONGLOCK: u32 = 0x000000ff;
const SDCASR_OFFSET: usize = 0x120000;

const CPUFREQ_TABLE_END: u32 = 0xffff_ffff;
const CPUFREQ_CONST_LOOPS: u32 = 1;
const SYSTEM_RUNNING: u32 = 1;
const ENODEV: i32 = 19;
const EINVAL: i32 = 22;

#[repr(C)]
pub struct cpufreq_frequency_table {
    pub flags: u32,
    pub driver_data: u32,
    pub frequency: u32,
}

#[repr(C)]
pub struct cpufreq_policy {
    pub cpu: u32,
    pub cur: u32,
}

#[repr(C)]
pub struct resource { pub start: usize }
#[repr(C)] pub struct device_node { _private: [u8; 0] }
#[repr(C)] pub struct cpufreq_driver {
    pub name: *const i8,
    pub flags: u32,
    pub init: Option<unsafe extern "C" fn(*mut cpufreq_policy) -> i32>,
    pub exit: Option<unsafe extern "C" fn(*mut cpufreq_policy)>,
    pub verify: Option<unsafe extern "C" fn(*mut cpufreq_policy) -> i32>,
    pub target_index: Option<unsafe extern "C" fn(*mut cpufreq_policy, u32) -> i32>,
}

extern "C" {
    fn in_le32(addr: *const u32) -> u32;
    fn out_le32(addr: *mut u32, value: u32);
    fn hard_smp_processor_id() -> u32;
    fn local_irq_save(flags: *mut usize);
    fn local_irq_restore(flags: usize);
    fn ioremap(start: usize, size: usize) -> *mut c_void;
    fn iounmap(addr: *mut c_void);
    fn of_get_cpu_node(cpu: u32, thread: *mut c_void) -> *mut device_node;
    fn of_get_property(node: *mut device_node, name: *const i8, len: *mut usize) -> *const u32;
    fn of_node_put(node: *mut device_node);
    fn of_find_compatible_node(from: *mut device_node, typ: *const i8, compatible: *const i8) -> *mut device_node;
    fn of_address_to_resource(node: *mut device_node, index: u32, res: *mut resource) -> i32;
    fn cpufreq_generic_init(policy: *mut cpufreq_policy, table: *mut cpufreq_frequency_table, latency: u32) -> i32;
    fn cpufreq_generic_frequency_table_verify(policy: *mut cpufreq_policy) -> i32;
    fn cpufreq_register_driver(driver: *mut cpufreq_driver) -> i32;
    fn cpufreq_unregister_driver(driver: *mut cpufreq_driver);
    fn of_machine_is_compatible(name: *const i8) -> bool;
    fn for_each_online_cpu(cpu: *mut i32);
    static mut ppc_proc_freq: u32;
    static mut system_state: u32;
}

static mut sdcpwr_mapbase: *mut u8 = core::ptr::null_mut();
static mut sdcasr_mapbase: *mut u8 = core::ptr::null_mut();
static mut current_astate: i32 = 0;

static mut pas_freqs: [cpufreq_frequency_table; 6] = [
    cpufreq_frequency_table { flags: 0, driver_data: 0, frequency: 0 },
    cpufreq_frequency_table { flags: 0, driver_data: 1, frequency: 0 },
    cpufreq_frequency_table { flags: 0, driver_data: 2, frequency: 0 },
    cpufreq_frequency_table { flags: 0, driver_data: 3, frequency: 0 },
    cpufreq_frequency_table { flags: 0, driver_data: 4, frequency: 0 },
    cpufreq_frequency_table { flags: 0, driver_data: 0, frequency: CPUFREQ_TABLE_END },
];

unsafe fn get_astate_freq(astate: i32) -> i32 {
    let ret = in_le32(sdcpwr_mapbase.add(SDCPWR_CFGA0_REG).add((astate * 0x10) as usize) as *const u32);
    (ret & 0x3f) as i32
}

unsafe fn get_cur_astate(cpu: i32) -> i32 {
    let ret = in_le32(sdcpwr_mapbase.add(SDCPWR_PWST0_REG) as *const u32);
    ((ret >> (cpu * 4)) & 0x7) as i32
}

unsafe fn get_gizmo_latency() -> u32 {
    let giztime = in_le32(sdcpwr_mapbase.add(SDCPWR_GIZTIME_REG) as *const u32);
    if giztime & SDCPWR_GIZTIME_GR != 0 { (giztime & SDCPWR_GIZTIME_LONGLOCK) * 128000 } else { (giztime & SDCPWR_GIZTIME_LONGLOCK) * 1000 }
}

unsafe fn set_astate(cpu: i32, astate: u32) {
    if sdcasr_mapbase.is_null() { return; }
    let mut flags = 0usize;
    local_irq_save(&mut flags);
    out_le32(sdcasr_mapbase.add(SDCASR_REG).add(SDCASR_REG_STRIDE * cpu as usize) as *mut u32, astate);
    local_irq_restore(flags);
}

pub unsafe fn check_astate() -> i32 { get_cur_astate(hard_smp_processor_id() as i32) }
pub unsafe fn restore_astate(cpu: i32) { set_astate(cpu, current_astate as u32); }

unsafe extern "C" fn pas_cpufreq_cpu_init(policy: *mut cpufreq_policy) -> i32 {
    let mut err = -ENODEV;
    let cpu = of_get_cpu_node((*policy).cpu, core::ptr::null_mut());
    if cpu.is_null() { return err; }
    let max_freqp = of_get_property(cpu, b"clock-frequency\0".as_ptr() as *const i8, core::ptr::null_mut());
    of_node_put(cpu);
    if max_freqp.is_null() { return -EINVAL; }
    let max_freq = *max_freqp / 1000;
    let mut dn = of_find_compatible_node(core::ptr::null_mut(), core::ptr::null(), b"1682m-sdc\0".as_ptr() as *const i8);
    if dn.is_null() { dn = of_find_compatible_node(core::ptr::null_mut(), core::ptr::null(), b"pasemi,pwrficient-sdc\0".as_ptr() as *const i8); }
    if dn.is_null() { return err; }
    let mut res = resource { start: 0 };
    err = of_address_to_resource(dn, 0, &mut res);
    of_node_put(dn);
    if err != 0 { return err; }
    sdcasr_mapbase = ioremap(res.start + SDCASR_OFFSET, 0x2000) as *mut u8;
    if sdcasr_mapbase.is_null() { return -EINVAL; }
    dn = of_find_compatible_node(core::ptr::null_mut(), core::ptr::null(), b"1682m-gizmo\0".as_ptr() as *const i8);
    if dn.is_null() { dn = of_find_compatible_node(core::ptr::null_mut(), core::ptr::null(), b"pasemi,pwrficient-gizmo\0".as_ptr() as *const i8); }
    if dn.is_null() { iounmap(sdcasr_mapbase as *mut c_void); return -ENODEV; }
    err = of_address_to_resource(dn, 0, &mut res);
    of_node_put(dn);
    if err != 0 { iounmap(sdcasr_mapbase as *mut c_void); return err; }
    sdcpwr_mapbase = ioremap(res.start, 0x1000) as *mut u8;
    if sdcpwr_mapbase.is_null() { iounmap(sdcasr_mapbase as *mut c_void); return -EINVAL; }
    let mut idx = 0;
    while idx < 5 { pas_freqs[idx].frequency = (get_astate_freq(pas_freqs[idx].driver_data as i32) * 100000) as u32; idx += 1; }
    let cur_astate = get_cur_astate((*policy).cpu as i32);
    (*policy).cur = pas_freqs[cur_astate as usize].frequency;
    ppc_proc_freq = (*policy).cur * 1000;
    cpufreq_generic_init(policy, pas_freqs.as_mut_ptr(), get_gizmo_latency());
    let _ = max_freq;
    0
}

unsafe extern "C" fn pas_cpufreq_cpu_exit(_policy: *mut cpufreq_policy) {
    if system_state >= SYSTEM_RUNNING { return; }
    if !sdcasr_mapbase.is_null() { iounmap(sdcasr_mapbase as *mut c_void); }
    if !sdcpwr_mapbase.is_null() { iounmap(sdcpwr_mapbase as *mut c_void); }
}

unsafe extern "C" fn pas_cpufreq_target(policy: *mut cpufreq_policy, pas_astate_new: u32) -> i32 {
    current_astate = pas_astate_new as i32;
    let mut i = 0i32;
    for_each_online_cpu(&mut i);
    set_astate(i, pas_astate_new);
    ppc_proc_freq = pas_freqs[pas_astate_new as usize].frequency * 1000;
    let _ = policy;
    0
}

static mut pas_cpufreq_driver: cpufreq_driver = cpufreq_driver {
    name: b"pas-cpufreq\0".as_ptr() as *const i8,
    flags: CPUFREQ_CONST_LOOPS,
    init: Some(pas_cpufreq_cpu_init), exit: Some(pas_cpufreq_cpu_exit),
    verify: Some(cpufreq_generic_frequency_table_verify), target_index: Some(pas_cpufreq_target),
};

unsafe extern "C" fn pas_cpufreq_init() -> i32 {
    if !of_machine_is_compatible(b"PA6T-1682M\0".as_ptr() as *const i8) && !of_machine_is_compatible(b"pasemi,pwrficient\0".as_ptr() as *const i8) { return -ENODEV; }
    cpufreq_register_driver(&mut pas_cpufreq_driver)
}
unsafe extern "C" fn pas_cpufreq_exit() { cpufreq_unregister_driver(&mut pas_cpufreq_driver); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
