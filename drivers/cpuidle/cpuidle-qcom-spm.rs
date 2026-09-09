// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (c) 2011-2014, The Linux Foundation. All rights reserved.
 * Copyright (c) 2014,2015, Linaro Ltd.
 *
 * SAW power controller driver
 */

// Translated dependencies supplied by the Linux kernel and other source files.

use core::ffi::c_void;

const ENODEV: i32 = 19;
const ENOMEM: i32 = 12;
const EINVAL: i32 = 22;
const EPROBE_DEFER: i32 = 517;
const UINT_MAX: u32 = u32::MAX;
const PM_SLEEP_MODE_SPC: i32 = 0;
const PM_SLEEP_MODE_STBY: i32 = 1;
const QCOM_SCM_CPU_PWR_DOWN_L2_ON: i32 = 1;

#[repr(C)]
pub struct device { _private: [u8; 0] }
#[repr(C)]
pub struct device_node { _private: [u8; 0] }
#[repr(C)]
pub struct cpumask { _private: [u8; 0] }
#[repr(C)]
pub struct spm_driver_data { _private: [u8; 0] }

pub type cpuidle_enter_t = unsafe extern "C" fn(*mut cpuidle_device, *mut cpuidle_driver, i32) -> i32;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cpuidle_state {
    pub enter: Option<cpuidle_enter_t>,
    pub exit_latency: u32,
    pub target_residency: u32,
    pub power_usage: u32,
    pub name: *const u8,
    pub desc: *const u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cpuidle_driver {
    pub name: *const u8,
    pub owner: *mut c_void,
    pub states: [cpuidle_state; 1],
    pub cpumask: *mut cpumask,
}

#[repr(C)] pub struct cpuidle_device { _private: [u8; 0] }
#[repr(C)] pub struct platform_device { pub dev: device }
#[repr(C)] pub struct platform_driver { pub probe: Option<unsafe extern "C" fn(*mut platform_device) -> i32>, pub name: *const u8, pub suppress_bind_attrs: bool }

#[repr(C)]
pub struct cpuidle_qcom_spm_data {
    pub cpuidle_driver: cpuidle_driver,
    pub spm: *mut spm_driver_data,
}

extern "C" {
    fn qcom_scm_cpu_power_down(mode: i32);
    fn spm_set_low_power_mode(drv: *mut spm_driver_data, mode: i32);
    fn cpu_suspend(arg: u64, fn_: unsafe extern "C" fn(u64) -> i32) -> i32;
    fn of_cpu_device_node_get(cpu: i32) -> *mut device_node;
    fn of_parse_phandle(node: *mut device_node, name: *const u8, index: i32) -> *mut device_node;
    fn of_node_put(node: *mut device_node);
    fn of_find_device_by_node(node: *mut device_node) -> *mut platform_device;
    fn devm_kzalloc(dev: *mut device, size: usize, flags: u32) -> *mut c_void;
    fn put_device(dev: *mut device);
    fn dev_get_drvdata(dev: *mut device) -> *mut spm_driver_data;
    fn cpumask_of(cpu: i32) -> *const cpumask;
    fn dt_init_idle_driver(drv: *mut cpuidle_driver, match_: *const of_device_id, states: usize) -> i32;
    fn cpuidle_register(drv: *mut cpuidle_driver, cpumask: *const cpumask) -> i32;
    fn qcom_scm_is_available() -> bool;
    fn qcom_scm_set_warm_boot_addr(fn_: unsafe extern "C" fn()) -> i32;
    fn cpu_resume_arm();
    fn platform_driver_register(drv: *mut platform_driver) -> i32;
    fn platform_driver_unregister(drv: *mut platform_driver);
    fn platform_device_register_simple(name: *const u8, id: i32, data: *const c_void, size: usize) -> *mut platform_device;
    fn ptr_err(ptr: *mut platform_device) -> i32;
    fn of_device_is_available(node: *mut device_node) -> bool;
}

#[repr(C)]
pub struct of_device_id {
    pub compatible: *const u8,
    pub data: Option<cpuidle_enter_t>,
}

unsafe extern "C" fn qcom_pm_collapse(_unused: u64) -> i32 {
    qcom_scm_cpu_power_down(QCOM_SCM_CPU_PWR_DOWN_L2_ON);
    /* Returns here only if there was a pending interrupt and we did not
     * power down as a result. */
    -1
}

unsafe extern "C" fn qcom_cpu_spc(drv: *mut spm_driver_data) -> i32 {
    let ret: i32;
    spm_set_low_power_mode(drv, PM_SLEEP_MODE_SPC);
    ret = cpu_suspend(0, qcom_pm_collapse);
    /* ARM common code executes WFI without calling into our driver and
     * if the SPM mode is not reset, then we may accidentally power down the
     * cpu when we intended only to gate the cpu clock.
     * Ensure the state is set to standby before returning. */
    spm_set_low_power_mode(drv, PM_SLEEP_MODE_STBY);
    ret
}

unsafe extern "C" fn spm_enter_idle_state(dev: *mut cpuidle_device, drv: *mut cpuidle_driver, idx: i32) -> i32 {
    let data = (drv as *mut u8).sub(core::mem::offset_of!(cpuidle_qcom_spm_data, cpuidle_driver)) as *mut cpuidle_qcom_spm_data;
    let _ = dev;
    // CPU_PM_CPU_IDLE_ENTER_PARAM(qcom_cpu_spc, idx, data->spm)
    cpu_pm_cpu_idle_enter_param(qcom_cpu_spc, idx, (*data).spm)
}

extern "C" { fn cpu_pm_cpu_idle_enter_param(fn_: unsafe extern "C" fn(*mut spm_driver_data) -> i32, idx: i32, arg: *mut spm_driver_data) -> i32; }

static mut QCOM_SPM_IDLE_DRIVER: cpuidle_driver = cpuidle_driver {
    name: b"qcom_spm\0".as_ptr(), owner: core::ptr::null_mut(),
    states: [cpuidle_state { enter: Some(spm_enter_idle_state), exit_latency: 1, target_residency: 1, power_usage: UINT_MAX, name: b"WFI\0".as_ptr(), desc: b"ARM WFI\0".as_ptr() }],
    cpumask: core::ptr::null_mut(),
};

static QCOM_IDLE_STATE_MATCH: [of_device_id; 2] = [
    of_device_id { compatible: b"qcom,idle-state-spc\0".as_ptr(), data: Some(spm_enter_idle_state) },
    of_device_id { compatible: core::ptr::null(), data: None },
];

static mut SPM_CPUIDLE_DRIVER: platform_driver = platform_driver {
    probe: None, name: b"qcom-spm-cpuidle\0".as_ptr(), suppress_bind_attrs: true,
};

extern "C" {
    fn next_present_cpu(cpu: i32) -> i32;
    fn first_present_cpu() -> i32;
    fn next_of_cpu_node(node: *mut device_node) -> *mut device_node;
    fn first_of_cpu_node() -> *mut device_node;
    fn dev_err_probe(dev: *mut device, err: i32, fmt: *const u8) -> i32;
}

unsafe fn spm_cpuidle_register(cpuidle_dev: *mut device, cpu: i32) -> i32 {
    let cpu_node = of_cpu_device_node_get(cpu);
    if cpu_node.is_null() { return -ENODEV; }
    let saw_node = of_parse_phandle(cpu_node, b"qcom,saw\0".as_ptr(), 0);
    of_node_put(cpu_node);
    if saw_node.is_null() { return -ENODEV; }
    let pdev = of_find_device_by_node(saw_node);
    of_node_put(saw_node);
    if pdev.is_null() { return -ENODEV; }
    let data = devm_kzalloc(cpuidle_dev, core::mem::size_of::<cpuidle_qcom_spm_data>(), 0) as *mut cpuidle_qcom_spm_data;
    if data.is_null() { put_device(&mut (*pdev).dev); return -ENOMEM; }
    (*data).spm = dev_get_drvdata(&mut (*pdev).dev);
    put_device(&mut (*pdev).dev);
    if (*data).spm.is_null() { return -EINVAL; }
    (*data).cpuidle_driver = QCOM_SPM_IDLE_DRIVER;
    (*data).cpuidle_driver.cpumask = cpumask_of(cpu) as *mut cpumask;
    let ret = dt_init_idle_driver(&mut (*data).cpuidle_driver, QCOM_IDLE_STATE_MATCH.as_ptr(), 1);
    if ret <= 0 { return if ret != 0 { ret } else { -ENODEV }; }
    cpuidle_register(&mut (*data).cpuidle_driver, core::ptr::null())
}

unsafe fn spm_cpuidle_drv_probe(pdev: *mut platform_device) -> i32 {
    if !qcom_scm_is_available() { return -EPROBE_DEFER; }
    let ret = qcom_scm_set_warm_boot_addr(cpu_resume_arm);
    if ret != 0 { return dev_err_probe(&mut (*pdev).dev, ret, b"set warm boot addr failed\0".as_ptr()); }
    let mut cpu = first_present_cpu();
    while cpu >= 0 {
        let ret = spm_cpuidle_register(&mut (*pdev).dev, cpu);
        if ret != 0 && ret != -ENODEV { let _ = ret; }
        cpu = next_present_cpu(cpu);
    }
    0
}

unsafe fn qcom_spm_find_any_cpu() -> bool {
    let mut cpu_node = first_of_cpu_node();
    while !cpu_node.is_null() {
        let saw_node = of_parse_phandle(cpu_node, b"qcom,saw\0".as_ptr(), 0);
        if of_device_is_available(saw_node) {
            of_node_put(saw_node); of_node_put(cpu_node); return true;
        }
        of_node_put(saw_node);
        cpu_node = next_of_cpu_node(cpu_node);
    }
    false
}

unsafe fn qcom_spm_cpuidle_init() -> i32 {
    let ret = platform_driver_register(&raw mut SPM_CPUIDLE_DRIVER);
    if ret != 0 { return ret; }
    if !qcom_spm_find_any_cpu() { return 0; }
    let pdev = platform_device_register_simple(b"qcom-spm-cpuidle\0".as_ptr(), -1, core::ptr::null(), 0);
    if pdev as *mut c_void as isize == -1 { platform_driver_unregister(&raw mut SPM_CPUIDLE_DRIVER); return ptr_err(pdev); }
    0
}

// device_initcall(qcom_spm_cpuidle_init)

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
