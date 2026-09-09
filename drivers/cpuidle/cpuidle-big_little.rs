// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (c) 2013 ARM/Linaro
 *
 * Authors: Daniel Lezcano <daniel.lezcano@linaro.org>
 *          Lorenzo Pieralisi <lorenzo.pieralisi@arm.com>
 *          Nicolas Pitre <nicolas.pitre@linaro.org>
 *
 * Maintainer: Lorenzo Pieralisi <lorenzo.pieralisi@arm.com>
 * Maintainer: Daniel Lezcano <daniel.lezcano@linaro.org>
 */

// Linux kernel dependencies supplied by other translation units.

extern "C" {
    fn read_cpuid_mpidr() -> u32;
    fn mcpm_set_entry_vector(cpu: u32, cluster: u32, resume: unsafe extern "C" fn());
    fn mcpm_cpu_suspend();
    fn cpu_resume();
    fn cpu_pm_enter();
    fn cpu_pm_exit();
    fn ct_cpuidle_enter();
    fn ct_cpuidle_exit();
    fn cpu_suspend(arg: u32, finisher: unsafe extern "C" fn(usize) -> i32) -> i32;
    fn cpumask_size() -> usize;
    fn kzalloc(size: usize, flags: u32) -> *mut cpumask;
    fn kfree(ptr: *mut cpumask);
    fn smp_cpuid_part(cpu: i32) -> i32;
    fn cpumask_set_cpu(cpu: i32, mask: *mut cpumask);
    fn of_machine_device_match(match_table: *const of_device_id) -> bool;
    fn mcpm_is_available() -> bool;
    fn dt_init_idle_driver(drv: *mut cpuidle_driver, match_table: *const of_device_id, start_idx: usize) -> i32;
    fn cpuidle_register(drv: *mut cpuidle_driver, device: *mut core::ffi::c_void) -> i32;
    fn cpuidle_unregister(drv: *mut cpuidle_driver);
}

#[repr(C)]
pub struct cpuidle_device;
#[repr(C)]
pub struct cpumask;

#[repr(C)]
pub struct cpuidle_state {
    pub enter: Option<unsafe extern "C" fn(*mut cpuidle_device, *mut cpuidle_driver, i32) -> i32>,
    pub exit_latency: u32,
    pub target_residency: u32,
    pub flags: u32,
    pub name: *const u8,
    pub desc: *const u8,
}

#[repr(C)]
pub struct cpuidle_driver {
    pub name: *const u8,
    pub owner: *mut core::ffi::c_void,
    pub states: [cpuidle_state; 2],
    pub state_count: usize,
    pub cpumask: *mut cpumask,
}

#[repr(C)]
pub struct of_device_id {
    pub compatible: *const u8,
    pub data: Option<unsafe extern "C" fn(*mut cpuidle_device, *mut cpuidle_driver, i32) -> i32>,
}

const ENOMEM: i32 = 12;
const ENODEV: i32 = 19;
const EUNATCH: i32 = 49;
const ARM_CPUIDLE_WFI_STATE: cpuidle_state = cpuidle_state {
    enter: None, exit_latency: 0, target_residency: 0, flags: 0,
    name: b"WFI\0".as_ptr(), desc: b"ARM WFI\0".as_ptr(),
};
const CPUIDLE_FLAG_TIMER_STOP: u32 = 1 << 2;
const CPUIDLE_FLAG_RCU_IDLE: u32 = 1 << 3;

extern "C" {
    static THIS_MODULE: core::ffi::c_void;
}

static mut bl_idle_little_driver: cpuidle_driver = cpuidle_driver {
    name: b"little_idle\0".as_ptr(), owner: core::ptr::null_mut(),
    states: [ARM_CPUIDLE_WFI_STATE, cpuidle_state {
        enter: Some(bl_enter_powerdown), exit_latency: 700, target_residency: 2500,
        flags: CPUIDLE_FLAG_TIMER_STOP | CPUIDLE_FLAG_RCU_IDLE,
        name: b"C1\0".as_ptr(), desc: b"ARM little-cluster power down\0".as_ptr(),
    }], state_count: 2, cpumask: core::ptr::null_mut(),
};

static mut bl_idle_big_driver: cpuidle_driver = cpuidle_driver {
    name: b"big_idle\0".as_ptr(), owner: core::ptr::null_mut(),
    states: [ARM_CPUIDLE_WFI_STATE, cpuidle_state {
        enter: Some(bl_enter_powerdown), exit_latency: 500, target_residency: 2000,
        flags: CPUIDLE_FLAG_TIMER_STOP | CPUIDLE_FLAG_RCU_IDLE,
        name: b"C1\0".as_ptr(), desc: b"ARM big-cluster power down\0".as_ptr(),
    }], state_count: 2, cpumask: core::ptr::null_mut(),
};

static mut bl_idle_state_match: [of_device_id; 2] = [
    of_device_id { compatible: b"arm,idle-state\0".as_ptr(), data: Some(bl_enter_powerdown) },
    of_device_id { compatible: core::ptr::null(), data: None },
];

unsafe extern "C" fn bl_powerdown_finisher(_arg: usize) -> i32 {
    // MCPM works with HW CPU identifiers
    let mpidr = read_cpuid_mpidr();
    let cluster = (mpidr >> 8) & 0xff;
    let cpu = mpidr & 0xff;

    mcpm_set_entry_vector(cpu, cluster, cpu_resume);
    mcpm_cpu_suspend();

    // return value != 0 means failure
    1
}

unsafe extern "C" fn bl_enter_powerdown(
    _dev: *mut cpuidle_device,
    _drv: *mut cpuidle_driver,
    idx: i32,
) -> i32 {
    cpu_pm_enter();
    ct_cpuidle_enter();

    cpu_suspend(0, bl_powerdown_finisher);

    // signals the MCPM core that CPU is out of low power state
    mcpm_cpu_powered_up();
    ct_cpuidle_exit();

    cpu_pm_exit();

    idx
}

extern "C" {
    fn mcpm_cpu_powered_up();
}

unsafe extern "C" fn bl_idle_driver_init(drv: *mut cpuidle_driver, part_id: i32) -> i32 {
    let cpumask = kzalloc(cpumask_size(), 0);
    if cpumask.is_null() {
        return -ENOMEM;
    }

    for cpu in 0..(core::mem::size_of::<cpumask>() as i32) {
        if smp_cpuid_part(cpu) == part_id {
            cpumask_set_cpu(cpu, cpumask);
        }
    }

    (*drv).cpumask = cpumask;
    0
}

static mut compatible_machine_match: [of_device_id; 3] = [
    of_device_id { compatible: b"arm,vexpress,v2p-ca15_a7\0".as_ptr(), data: None },
    of_device_id { compatible: b"google,peach\0".as_ptr(), data: None },
    of_device_id { compatible: core::ptr::null(), data: None },
];

unsafe extern "C" fn bl_idle_init() -> i32 {
    let mut ret: i32;

    // Initialize the driver just for a compliant set of machines
    if !of_machine_device_match(compatible_machine_match.as_ptr()) {
        return -ENODEV;
    }

    if !mcpm_is_available() {
        return -EUNATCH;
    }

    ret = bl_idle_driver_init(&mut bl_idle_little_driver, 0xD03);
    if ret != 0 {
        return ret;
    }

    ret = bl_idle_driver_init(&mut bl_idle_big_driver, 0xD04);
    if ret != 0 {
        kfree(bl_idle_little_driver.cpumask);
        return ret;
    }

    ret = dt_init_idle_driver(&mut bl_idle_big_driver, bl_idle_state_match.as_ptr(), 1);
    if ret < 0 {
        kfree(bl_idle_big_driver.cpumask);
        kfree(bl_idle_little_driver.cpumask);
        return ret;
    }

    ret = dt_init_idle_driver(&mut bl_idle_little_driver, bl_idle_state_match.as_ptr(), 1);
    if ret < 0 {
        kfree(bl_idle_big_driver.cpumask);
        kfree(bl_idle_little_driver.cpumask);
        return ret;
    }

    ret = cpuidle_register(&mut bl_idle_little_driver, core::ptr::null_mut());
    if ret != 0 {
        kfree(bl_idle_big_driver.cpumask);
        kfree(bl_idle_little_driver.cpumask);
        return ret;
    }

    ret = cpuidle_register(&mut bl_idle_big_driver, core::ptr::null_mut());
    if ret != 0 {
        cpuidle_unregister(&mut bl_idle_little_driver);
        kfree(bl_idle_big_driver.cpumask);
        kfree(bl_idle_little_driver.cpumask);
        return ret;
    }

    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
