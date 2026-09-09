// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright 2013 Freescale Semiconductor, Inc.
 *
 * CPU Frequency Scaling driver for Freescale QorIQ SoCs.
 */

// Linux kernel dependencies supplied by the surrounding translation.

#[repr(C)]
pub struct device_node;
#[repr(C)]
pub struct device;
#[repr(C)]
pub struct platform_device { pub dev: device }
#[repr(C)]
pub struct clk;
#[repr(C)]
pub struct clk_hw { pub clk: *mut clk }
#[repr(C)]
pub struct cpumask;
#[repr(C)]
pub struct cpufreq_frequency_table { pub driver_data: u32, pub frequency: u32 }
#[repr(C)]
pub struct cpufreq_policy {
    pub cpus: *mut cpumask,
    pub clk: *mut clk,
    pub freq_table: *mut cpufreq_frequency_table,
    pub driver_data: *mut core::ffi::c_void,
    pub cpu: u32,
    pub cpuinfo: cpufreq_cpuinfo,
}
#[repr(C)]
pub struct cpufreq_cpuinfo { pub transition_latency: u64 }
#[repr(C)]
pub struct cpufreq_driver {
    pub name: *const u8,
    pub flags: u32,
    pub init: Option<unsafe extern "C" fn(*mut cpufreq_policy) -> i32>,
    pub exit: Option<unsafe extern "C" fn(*mut cpufreq_policy)>,
    pub verify: Option<unsafe extern "C" fn(*mut cpufreq_policy) -> i32>,
    pub target_index: Option<unsafe extern "C" fn(*mut cpufreq_policy, u32) -> i32>,
    pub get: Option<unsafe extern "C" fn(u32) -> u32>,
}
#[repr(C)]
pub struct of_device_id { pub compatible: *const u8 }
#[repr(C)]
pub struct platform_driver_inner { pub name: *const u8 }
#[repr(C)]
pub struct platform_driver {
    pub driver: platform_driver_inner,
    pub probe: Option<unsafe extern "C" fn(*mut platform_device) -> i32>,
    pub remove: Option<unsafe extern "C" fn(*mut platform_device)>,
}

extern "C" {
    fn of_find_node_by_type(from: *mut device_node, typ: *const u8) -> *mut device_node;
    fn of_property_read_u32(np: *mut device_node, propname: *const u8, out: *mut u32) -> i32;
    fn of_node_put(np: *mut device_node);
    fn clk_get(dev: *mut device, id: *const u8) -> *mut clk;
    fn clk_get_rate(clk: *mut clk) -> u64;
    fn of_get_cpu_node(cpu: u32, thread: *mut u32) -> *mut device_node;
    fn of_clk_get(np: *mut device_node, index: i32) -> *mut clk;
    fn cpu_present(cpu: i32) -> bool;
    fn for_each_present_cpu_next(cpu: i32) -> i32;
    fn clk_is_match(a: *mut clk, b: *mut clk) -> bool;
    fn cpumask_set_cpu(cpu: i32, dst: *mut cpumask);
    fn __clk_get_hw(clk: *mut clk) -> *const clk_hw;
    fn clk_hw_get_num_parents(hw: *const clk_hw) -> i32;
    fn clk_hw_get_parent_by_index(hw: *const clk_hw, index: i32) -> *const clk_hw;
    fn clk_set_parent(clk: *mut clk, parent: *mut clk) -> i32;
    fn cpufreq_generic_frequency_table_verify(policy: *mut cpufreq_policy) -> i32;
    fn cpufreq_generic_get(cpu: u32) -> u32;
    fn cpufreq_register_driver(driver: *mut cpufreq_driver) -> i32;
    fn cpufreq_unregister_driver(driver: *mut cpufreq_driver);
    fn of_find_matching_node(from: *mut device_node, matches: *const of_device_id) -> *mut device_node;
    fn dev_info(dev: *const device, fmt: *const u8, ...);
    fn pr_err(fmt: *const u8, ...);
}

const ENODEV: i32 = 19;
const CPUFREQ_ENTRY_INVALID: u32 = u32::MAX;
const CPUFREQ_TABLE_END: u32 = u32::MAX - 1;
const NSEC_PER_SEC: u64 = 1_000_000_000;
const CPUFREQ_CONST_LOOPS: u32 = 1 << 0;
const CPUFREQ_IS_COOLING_DEV: u32 = 1 << 1;

#[repr(C)]
pub struct cpu_data {
    pub pclk: *mut *mut clk,
    pub table: *mut cpufreq_frequency_table,
}

#[repr(C)]
pub struct soc_data { pub flags: u32 }

unsafe fn get_bus_freq() -> u32 {
    let soc = of_find_node_by_type(core::ptr::null_mut(), b"soc\0".as_ptr());
    if !soc.is_null() {
        let mut sysfreq = 0u32;
        let ret = of_property_read_u32(soc, b"bus-frequency\0".as_ptr(), &mut sysfreq);
        of_node_put(soc);
        if ret == 0 { return sysfreq; }
    }
    let pltclk = clk_get(core::ptr::null_mut(), b"cg-pll0-div1\0".as_ptr());
    if pltclk.is_null() { return -(1i32) as u32; }
    clk_get_rate(pltclk) as u32
}

unsafe fn cpu_to_clk(cpu: i32) -> *mut clk {
    if !cpu_present(cpu) { return core::ptr::null_mut(); }
    let np = of_get_cpu_node(cpu as u32, core::ptr::null_mut());
    if np.is_null() { return core::ptr::null_mut(); }
    let clk = of_clk_get(np, 0);
    of_node_put(np);
    clk
}

unsafe fn set_affected_cpus(policy: *mut cpufreq_policy) {
    let dstp = (*policy).cpus;
    let mut i = 0;
    loop {
        i = for_each_present_cpu_next(i);
        if i < 0 { break; }
        let clk = cpu_to_clk(i);
        if clk.is_null() { continue; }
        if clk_is_match((*policy).clk, clk) { cpumask_set_cpu(i, dstp); }
        i += 1;
    }
}

unsafe fn freq_table_redup(freq_table: *mut cpufreq_frequency_table, count: i32) {
    for i in 1..count {
        for j in 0..i {
            let fi = (*freq_table.add(i as usize)).frequency;
            let fj = (*freq_table.add(j as usize)).frequency;
            if fj == CPUFREQ_ENTRY_INVALID || fj != fi { continue; }
            (*freq_table.add(i as usize)).frequency = CPUFREQ_ENTRY_INVALID;
            break;
        }
    }
}

unsafe fn freq_table_sort(freq_table: *mut cpufreq_frequency_table, count: i32) {
    for i in 0..(count - 1) {
        let mut max_freq = (*freq_table.add(i as usize)).frequency;
        let mut ind = i;
        for j in (i + 1)..count {
            let freq = (*freq_table.add(j as usize)).frequency;
            if freq == CPUFREQ_ENTRY_INVALID || freq <= max_freq { continue; }
            ind = j;
            max_freq = freq;
        }
        if ind != i {
            core::ptr::swap(freq_table.add(i as usize), freq_table.add(ind as usize));
        }
    }
}

unsafe extern "C" fn qoriq_cpufreq_cpu_init(policy: *mut cpufreq_policy) -> i32 {
    let np = of_get_cpu_node((*policy).cpu, core::ptr::null_mut());
    if np.is_null() { return -ENODEV; }
    let data = Box::into_raw(Box::new(cpu_data { pclk: core::ptr::null_mut(), table: core::ptr::null_mut() }));
    (*policy).clk = of_clk_get(np, 0);
    if (*policy).clk.is_null() { drop(Box::from_raw(data)); of_node_put(np); return -ENODEV; }
    let hwclk = __clk_get_hw((*policy).clk);
    let count = clk_hw_get_num_parents(hwclk);
    let pclk = vec![core::ptr::null_mut(); count as usize].into_boxed_slice();
    (*data).pclk = Box::into_raw(pclk) as *mut *mut clk;
    let table = vec![cpufreq_frequency_table { driver_data: 0, frequency: 0 }; (count + 1) as usize].into_boxed_slice();
    let table_ptr = Box::into_raw(table) as *mut cpufreq_frequency_table;
    for i in 0..count {
        let clk = (*clk_hw_get_parent_by_index(hwclk, i)).clk;
        *(*data).pclk.add(i as usize) = clk;
        (*table_ptr.add(i as usize)).frequency = (clk_get_rate(clk) / 1000) as u32;
        (*table_ptr.add(i as usize)).driver_data = i as u32;
    }
    freq_table_redup(table_ptr, count);
    freq_table_sort(table_ptr, count);
    (*table_ptr.add(count as usize)).frequency = CPUFREQ_TABLE_END;
    (*policy).freq_table = table_ptr;
    (*data).table = table_ptr;
    set_affected_cpus(policy);
    (*policy).driver_data = data as *mut core::ffi::c_void;
    let bus = get_bus_freq() as u64;
    (*policy).cpuinfo.transition_latency = (12 * NSEC_PER_SEC) / bus + 1;
    of_node_put(np);
    0
}

unsafe extern "C" fn qoriq_cpufreq_cpu_exit(policy: *mut cpufreq_policy) {
    let data = (*policy).driver_data as *mut cpu_data;
    drop(Box::from_raw(core::slice::from_raw_parts_mut((*data).pclk, 0)));
    drop(Box::from_raw(core::slice::from_raw_parts_mut((*data).table, 0)));
    drop(Box::from_raw(data));
    (*policy).driver_data = core::ptr::null_mut();
}

unsafe extern "C" fn qoriq_cpufreq_target(policy: *mut cpufreq_policy, index: u32) -> i32 {
    let data = (*policy).driver_data as *mut cpu_data;
    let parent = *(*data).pclk.add((*(*data).table.add(index as usize)).driver_data as usize);
    clk_set_parent((*policy).clk, parent)
}

static mut qoriq_cpufreq_driver: cpufreq_driver = cpufreq_driver {
    name: b"qoriq_cpufreq\0".as_ptr(), flags: CPUFREQ_CONST_LOOPS | CPUFREQ_IS_COOLING_DEV,
    init: Some(qoriq_cpufreq_cpu_init), exit: Some(qoriq_cpufreq_cpu_exit),
    verify: Some(cpufreq_generic_frequency_table_verify), target_index: Some(qoriq_cpufreq_target),
    get: Some(cpufreq_generic_get),
};

static qoriq_cpufreq_blacklist: [of_device_id; 5] = [
    of_device_id { compatible: b"fsl,b4420-clockgen\0".as_ptr() },
    of_device_id { compatible: b"fsl,b4860-clockgen\0".as_ptr() },
    of_device_id { compatible: b"fsl,t2080-clockgen\0".as_ptr() },
    of_device_id { compatible: b"fsl,t4240-clockgen\0".as_ptr() },
    of_device_id { compatible: core::ptr::null() },
];

unsafe extern "C" fn qoriq_cpufreq_probe(pdev: *mut platform_device) -> i32 {
    let np = of_find_matching_node(core::ptr::null_mut(), qoriq_cpufreq_blacklist.as_ptr());
    if !np.is_null() { of_node_put(np); return -ENODEV; }
    let ret = cpufreq_register_driver(&mut qoriq_cpufreq_driver);
    if ret != 0 { return ret; }
    dev_info(&(*pdev).dev, b"Freescale QorIQ CPU frequency scaling driver\n\0".as_ptr());
    0
}

unsafe extern "C" fn qoriq_cpufreq_remove(_pdev: *mut platform_device) {
    cpufreq_unregister_driver(&mut qoriq_cpufreq_driver);
}

static mut qoriq_cpufreq_platform_driver: platform_driver = platform_driver {
    driver: platform_driver_inner { name: b"qoriq-cpufreq\0".as_ptr() },
    probe: Some(qoriq_cpufreq_probe), remove: Some(qoriq_cpufreq_remove),
};

// module_platform_driver(qoriq_cpufreq_platform_driver);
// MODULE_ALIAS("platform:qoriq-cpufreq");
// MODULE_LICENSE("GPL");
// MODULE_AUTHOR("Tang Yuantian <Yuantian.Tang@freescale.com>");
// MODULE_DESCRIPTION("cpufreq driver for Freescale QorIQ series SoCs");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
