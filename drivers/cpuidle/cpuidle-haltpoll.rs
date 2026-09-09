// SPDX-License-Identifier: GPL-2.0
/*
 * cpuidle driver for haltpoll governor.
 *
 * Copyright 2019 Red Hat, Inc. and/or its affiliates.
 *
 * This work is licensed under the terms of the GNU GPL, version 2.  See
 * the COPYING file in the top-level directory.
 *
 * Authors: Marcelo Tosatti <mtosatti@redhat.com>
 */

// Linux kernel dependencies supplied by other translation units.
extern "C" {
    static mut boot_option_idle_override: i32;
    fn current_clr_polling_and_test() -> bool;
    fn arch_cpu_idle();
    fn per_cpu_ptr(ptr: *mut cpuidle_device, cpu: u32) -> *mut cpuidle_device;
    fn cpuidle_register_device(dev: *mut cpuidle_device) -> i32;
    fn cpuidle_unregister_device(dev: *mut cpuidle_device);
    fn arch_haltpoll_enable(cpu: u32);
    fn arch_haltpoll_disable(cpu: u32);
    fn cpuhp_remove_state(state: cpuhp_state);
    fn cpuidle_unregister_driver(drv: *mut cpuidle_driver);
    fn free_percpu(ptr: *mut cpuidle_device);
    fn kvm_para_has_hint(hint: u32) -> bool;
    fn kvm_para_available() -> bool;
    fn cpuidle_poll_state_init(drv: *mut cpuidle_driver);
    fn cpuidle_register_driver(drv: *mut cpuidle_driver) -> i32;
    fn alloc_percpu_cpuidle_device() -> *mut cpuidle_device;
    fn cpuhp_setup_state(state: i32, name: *const u8,
                         online: unsafe extern "C" fn(u32) -> i32,
                         offline: unsafe extern "C" fn(u32) -> i32) -> i32;
}

type cpuhp_state = i32;
const CPUHP_AP_ONLINE_DYN: i32 = 0;
const KVM_HINTS_REALTIME: u32 = 0;
const IDLE_NO_OVERRIDE: i32 = 0;

#[repr(C)]
pub struct cpuidle_device {
    pub registered: bool,
    pub cpu: u32,
}

#[repr(C)]
pub struct cpuidle_state {
    pub enter: Option<unsafe extern "C" fn(*mut cpuidle_device, *mut cpuidle_driver, i32) -> i32>,
    pub exit_latency: u32,
    pub target_residency: u32,
    pub power_usage: i32,
    pub name: *const u8,
    pub desc: *const u8,
}

#[repr(C)]
pub struct cpuidle_driver {
    pub name: *const u8,
    pub governor: *const u8,
    pub states: [cpuidle_state; 2],
    pub safe_state_index: u32,
    pub state_count: u32,
}

static mut force: bool = false;
static mut haltpoll_cpuidle_devices: *mut cpuidle_device = core::ptr::null_mut();
static mut haltpoll_hp_state: cpuhp_state = 0;

unsafe extern "C" fn default_enter_idle(
    _dev: *mut cpuidle_device,
    _drv: *mut cpuidle_driver,
    index: i32,
) -> i32 {
    if current_clr_polling_and_test() {
        return index;
    }

    arch_cpu_idle();
    index
}

static mut haltpoll_driver: cpuidle_driver = cpuidle_driver {
    name: b"haltpoll\0".as_ptr(),
    governor: b"haltpoll\0".as_ptr(),
    states: [
        cpuidle_state {
            enter: None,
            exit_latency: 0,
            target_residency: 0,
            power_usage: 0,
            name: core::ptr::null(),
            desc: core::ptr::null(),
        },
        cpuidle_state {
            enter: Some(default_enter_idle),
            exit_latency: 1,
            target_residency: 1,
            power_usage: -1,
            name: b"haltpoll idle\0".as_ptr(),
            desc: b"default architecture idle\0".as_ptr(),
        },
    ],
    safe_state_index: 0,
    state_count: 2,
};

unsafe extern "C" fn haltpoll_cpu_online(cpu: u32) -> i32 {
    let dev = per_cpu_ptr(haltpoll_cpuidle_devices, cpu);
    if !(*dev).registered {
        (*dev).cpu = cpu;
        if cpuidle_register_device(dev) != 0 {
            return -5; // -EIO
        }
        arch_haltpoll_enable(cpu);
    }
    0
}

unsafe extern "C" fn haltpoll_cpu_offline(cpu: u32) -> i32 {
    let dev = per_cpu_ptr(haltpoll_cpuidle_devices, cpu);
    if (*dev).registered {
        arch_haltpoll_disable(cpu);
        cpuidle_unregister_device(dev);
    }
    0
}

unsafe fn haltpoll_uninit() {
    if haltpoll_hp_state != 0 {
        cpuhp_remove_state(haltpoll_hp_state);
    }
    cpuidle_unregister_driver(&raw mut haltpoll_driver);
    free_percpu(haltpoll_cpuidle_devices);
    haltpoll_cpuidle_devices = core::ptr::null_mut();
}

unsafe fn haltpoll_want() -> bool {
    kvm_para_has_hint(KVM_HINTS_REALTIME) || force
}

unsafe extern "C" fn haltpoll_init() -> i32 {
    let drv = &raw mut haltpoll_driver;
    if boot_option_idle_override != IDLE_NO_OVERRIDE {
        return -19; // -ENODEV
    }
    if !kvm_para_available() || !haltpoll_want() {
        return -19; // -ENODEV
    }
    cpuidle_poll_state_init(drv);
    let mut ret = cpuidle_register_driver(drv);
    if ret < 0 { return ret; }
    haltpoll_cpuidle_devices = alloc_percpu_cpuidle_device();
    if haltpoll_cpuidle_devices.is_null() {
        cpuidle_unregister_driver(drv);
        return -12; // -ENOMEM
    }
    ret = cpuhp_setup_state(CPUHP_AP_ONLINE_DYN, b"cpuidle/haltpoll:online\0".as_ptr(),
                            haltpoll_cpu_online, haltpoll_cpu_offline);
    if ret < 0 {
        haltpoll_uninit();
    } else {
        haltpoll_hp_state = ret;
        ret = 0;
    }
    ret
}

unsafe extern "C" fn haltpoll_exit() {
    haltpoll_uninit();
}

// module_param(force, bool, 0444);
// MODULE_PARM_DESC(force, "Load unconditionally");
// module_init(haltpoll_init);
// module_exit(haltpoll_exit);
// MODULE_DESCRIPTION("cpuidle driver for haltpoll governor");
// MODULE_LICENSE("GPL");
// MODULE_AUTHOR("Marcelo Tosatti <mtosatti@redhat.com>");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
