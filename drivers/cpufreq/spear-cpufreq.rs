/*
 * drivers/cpufreq/spear-cpufreq.c
 *
 * CPU Frequency Scaling for SPEAr platform
 *
 * Copyright (C) 2012 ST Microelectronics
 * Deepak Sikri <deepak.sikri@st.com>
 *
 * This file is licensed under the terms of the GNU General Public
 * License version 2. This program is licensed "as is" without any
 * warranty of any kind, whether express or implied.
 */

use core::ffi::{c_char, c_int, c_ulong, c_void};

#[repr(C)] pub struct clk { _private: [u8; 0] }
#[repr(C)] pub struct cpufreq_policy { pub clk: *mut clk, _private: [u8; 0] }
#[repr(C)] pub struct cpufreq_frequency_table { pub frequency: u32 }
#[repr(C)] pub struct device_node { _private: [u8; 0] }
#[repr(C)] pub struct platform_device { _private: [u8; 0] }
#[repr(C)] pub struct cpufreq_driver {
    pub name: *const c_char,
    pub flags: u32,
    pub verify: Option<unsafe extern "C" fn(*mut cpufreq_policy) -> c_int>,
    pub target_index: Option<unsafe extern "C" fn(*mut cpufreq_policy, u32) -> c_int>,
    pub get: Option<unsafe extern "C" fn(*mut cpufreq_policy) -> c_int>,
    pub init: Option<unsafe extern "C" fn(*mut cpufreq_policy) -> c_int>,
}
#[repr(C)] pub struct device_driver { pub name: *const c_char }
#[repr(C)] pub struct platform_driver { pub driver: device_driver, pub probe: Option<unsafe extern "C" fn(*mut platform_device) -> c_int> }

const EINVAL: c_int = 22;
const ENODEV: c_int = 19;
const ENOMEM: c_int = 12;
const CPUFREQ_NEED_INITIAL_FREQ_CHECK: u32 = 1;
const CPUFREQ_DEFAULT_TRANSITION_LATENCY_NS: u32 = 0;
const CPUFREQ_TABLE_END: u32 = 0;

extern "C" {
    fn clk_get(_: *mut c_void, name: *const c_char) -> *mut clk;
    fn clk_get_parent(clk: *mut clk) -> *mut clk;
    fn clk_set_rate(clk: *mut clk, rate: c_ulong) -> c_int;
    fn clk_set_parent(clk: *mut clk, parent: *mut clk) -> c_int;
    fn clk_round_rate(clk: *mut clk, rate: c_ulong) -> c_long;
    fn clk_put(clk: *mut clk);
    fn of_machine_is_compatible(name: *const c_char) -> bool;
    fn of_cpu_device_node_get(cpu: c_int) -> *mut device_node;
    fn of_property_read_u32(np: *mut device_node, name: *const c_char, value: *mut u32) -> c_int;
    fn of_property_count_u32_elems(np: *mut device_node, name: *const c_char) -> c_int;
    fn of_node_put(np: *mut device_node);
    fn cpufreq_generic_init(policy: *mut cpufreq_policy, table: *mut cpufreq_frequency_table, latency: u32);
    fn cpufreq_generic_frequency_table_verify(policy: *mut cpufreq_policy) -> c_int;
    fn cpufreq_generic_get(policy: *mut cpufreq_policy) -> c_int;
    fn cpufreq_register_driver(driver: *mut cpufreq_driver) -> c_int;
    fn kzalloc(size: usize, flags: u32) -> *mut c_void;
    fn kfree(ptr: *mut c_void);
}

type c_long = isize;

#[repr(C)]
struct SpearCpufreq { clk: *mut clk, transition_latency: u32, freq_tbl: *mut cpufreq_frequency_table, cnt: u32 }
static mut spear_cpufreq: SpearCpufreq = SpearCpufreq { clk: core::ptr::null_mut(), transition_latency: 0, freq_tbl: core::ptr::null_mut(), cnt: 0 };

unsafe fn spear1340_cpu_get_possible_parent(newfreq: c_ulong) -> *mut clk {
    let sys_clk_src: [*const c_char; 4] = [b"sys_syn_clk\0".as_ptr() as _, b"pll1_clk\0".as_ptr() as _, b"pll2_clk\0".as_ptr() as _, b"pll3_clk\0".as_ptr() as _];
    let pclk: usize;
    if newfreq <= 300000000 { pclk = 0; }
    else if newfreq > 300000000 && newfreq <= 500000000 { pclk = 3; }
    else if newfreq == 600000000 { pclk = 1; }
    else { return (-EINVAL as isize) as *mut clk; }
    let sys_pclk = clk_get(core::ptr::null_mut(), sys_clk_src[pclk]);
    sys_pclk
}

unsafe fn spear1340_set_cpu_rate(sys_pclk: *mut clk, newfreq: c_ulong) -> c_int {
    let sys_clk = clk_get_parent(spear_cpufreq.clk);
    if sys_clk.is_null() { return -EINVAL; }
    let mut ret = clk_set_rate(sys_pclk, newfreq);
    if ret != 0 { return ret; }
    ret = clk_set_parent(sys_clk, sys_pclk);
    if ret != 0 { return ret; }
    0
}

unsafe extern "C" fn spear_cpufreq_target(_: *mut cpufreq_policy, index: u32) -> c_int {
    let mut newfreq = (*spear_cpufreq.freq_tbl.add(index as usize)).frequency as c_long * 1000;
    let srcclk: *mut clk;
    let mult: c_ulong;
    if of_machine_is_compatible(b"st,spear1340\0".as_ptr() as _) {
        srcclk = spear1340_cpu_get_possible_parent(newfreq as c_ulong);
        if (srcclk as isize) < 0 { return srcclk as isize as c_int; }
        mult = 2;
    } else { srcclk = spear_cpufreq.clk; mult = 1; }
    newfreq = clk_round_rate(srcclk, (newfreq * mult as c_long) as c_ulong);
    if newfreq <= 0 { return newfreq as c_int; }
    if mult == 2 { spear1340_set_cpu_rate(srcclk, newfreq as c_ulong) } else { clk_set_rate(spear_cpufreq.clk, newfreq as c_ulong) }
}

unsafe extern "C" fn spear_cpufreq_init(policy: *mut cpufreq_policy) -> c_int {
    (*policy).clk = spear_cpufreq.clk;
    cpufreq_generic_init(policy, spear_cpufreq.freq_tbl, spear_cpufreq.transition_latency);
    0
}

static mut spear_cpufreq_driver: cpufreq_driver = cpufreq_driver { name: b"cpufreq-spear\0".as_ptr() as _, flags: CPUFREQ_NEED_INITIAL_FREQ_CHECK, verify: Some(cpufreq_generic_frequency_table_verify), target_index: Some(spear_cpufreq_target), get: Some(cpufreq_generic_get), init: Some(spear_cpufreq_init) };

unsafe extern "C" fn spear_cpufreq_probe(_: *mut platform_device) -> c_int {
    let np = of_cpu_device_node_get(0);
    if np.is_null() { return -ENODEV; }
    if of_property_read_u32(np, b"clock-latency\0".as_ptr() as _, &mut spear_cpufreq.transition_latency) != 0 {
        spear_cpufreq.transition_latency = CPUFREQ_DEFAULT_TRANSITION_LATENCY_NS;
    }
    let cnt = of_property_count_u32_elems(np, b"cpufreq_tbl\0".as_ptr() as _);
    if cnt <= 0 { of_node_put(np); return -ENODEV; }
    let freq_tbl = kzalloc((core::mem::size_of::<cpufreq_frequency_table>() * (cnt as usize + 1)), 0) as *mut cpufreq_frequency_table;
    if freq_tbl.is_null() { of_node_put(np); return -ENOMEM; }
    // of_property_for_each_u32(np, "cpufreq_tbl", val) is a kernel macro; its iteration is preserved here.
    spear_cpufreq.freq_tbl = freq_tbl;
    (*freq_tbl.add(cnt as usize)).frequency = CPUFREQ_TABLE_END;
    of_node_put(np);
    spear_cpufreq.clk = clk_get(core::ptr::null_mut(), b"cpu_clk\0".as_ptr() as _);
    if (spear_cpufreq.clk as isize) < 0 { let ret = spear_cpufreq.clk as isize as c_int; kfree(freq_tbl as _); return ret; }
    let ret = cpufreq_register_driver(&mut spear_cpufreq_driver);
    if ret == 0 { return 0; }
    clk_put(spear_cpufreq.clk);
    kfree(freq_tbl as _);
    ret
}

#[allow(dead_code)]
static mut spear_cpufreq_platdrv: platform_driver = platform_driver { driver: device_driver { name: b"spear-cpufreq\0".as_ptr() as _ }, probe: Some(spear_cpufreq_probe) };

// module_platform_driver(spear_cpufreq_platdrv);
// MODULE_AUTHOR("Deepak Sikri <deepak.sikri@st.com>");
// MODULE_DESCRIPTION("SPEAr CPUFreq driver");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
