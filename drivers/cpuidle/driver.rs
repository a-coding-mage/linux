/*
 * driver.c - driver support
 *
 * (C) 2006-2007 Venkatesh Pallipadi <venkatesh.pallipadi@intel.com>
 *               Shaohua Li <shaohua.li@intel.com>
 *               Adam Belay <abelay@novell.com>
 *
 * This code is licenced under the GPL.
 */

// Dependencies supplied by the surrounding kernel translation.

pub static mut cpuidle_driver_lock: SpinLock = DEFINE_SPINLOCK!();

#[cfg(CONFIG_CPU_IDLE_MULTIPLE_DRIVERS)]
static mut cpuidle_drivers: PerCpu<*mut cpuidle_driver> = DEFINE_PER_CPU!();

#[cfg(CONFIG_CPU_IDLE_MULTIPLE_DRIVERS)]
unsafe fn __cpuidle_get_cpu_driver(cpu: i32) -> *mut cpuidle_driver {
    per_cpu!(cpuidle_drivers, cpu)
}

#[cfg(CONFIG_CPU_IDLE_MULTIPLE_DRIVERS)]
unsafe fn __cpuidle_unset_driver(drv: *mut cpuidle_driver) {
    let mut cpu: i32;
    for_each_cpu!(cpu, (*drv).cpumask) {
        if drv != __cpuidle_get_cpu_driver(cpu) {
            continue;
        }
        per_cpu!(cpuidle_drivers, cpu) = core::ptr::null_mut();
    }
}

#[cfg(CONFIG_CPU_IDLE_MULTIPLE_DRIVERS)]
unsafe fn __cpuidle_set_driver(drv: *mut cpuidle_driver) -> i32 {
    let mut cpu: i32;
    for_each_cpu!(cpu, (*drv).cpumask) {
        let old_drv = __cpuidle_get_cpu_driver(cpu);
        if !old_drv.is_null() && old_drv != drv {
            return -EBUSY;
        }
    }
    for_each_cpu!(cpu, (*drv).cpumask) {
        per_cpu!(cpuidle_drivers, cpu) = drv;
    }
    0
}

#[cfg(not(CONFIG_CPU_IDLE_MULTIPLE_DRIVERS))]
static mut cpuidle_curr_driver: *mut cpuidle_driver = core::ptr::null_mut();

#[cfg(not(CONFIG_CPU_IDLE_MULTIPLE_DRIVERS))]
unsafe fn __cpuidle_get_cpu_driver(_cpu: i32) -> *mut cpuidle_driver {
    cpuidle_curr_driver
}

#[cfg(not(CONFIG_CPU_IDLE_MULTIPLE_DRIVERS))]
unsafe fn __cpuidle_set_driver(drv: *mut cpuidle_driver) -> i32 {
    if !cpuidle_curr_driver.is_null() {
        return -EBUSY;
    }
    cpuidle_curr_driver = drv;
    0
}

#[cfg(not(CONFIG_CPU_IDLE_MULTIPLE_DRIVERS))]
unsafe fn __cpuidle_unset_driver(drv: *mut cpuidle_driver) {
    if drv == cpuidle_curr_driver {
        cpuidle_curr_driver = core::ptr::null_mut();
    }
}

unsafe fn cpuidle_setup_broadcast_timer(arg: *mut core::ffi::c_void) {
    if !arg.is_null() {
        tick_broadcast_enable();
    } else {
        tick_broadcast_disable();
    }
}

unsafe fn __cpuidle_driver_init(drv: *mut cpuidle_driver) {
    if (*drv).cpumask.is_null() {
        (*drv).cpumask = cpu_possible_mask as *mut cpumask;
    }
    for i in 0..(*drv).state_count {
        let s = &mut (*drv).states[i as usize];
        if s.flags & CPUIDLE_FLAG_TIMER_STOP != 0 {
            (*drv).bctimer = 1;
        }
        if s.target_residency > 0 {
            s.target_residency_ns = s.target_residency * NSEC_PER_USEC;
        } else if s.target_residency_ns < 0 {
            s.target_residency_ns = 0;
        } else {
            s.target_residency = div_u64(s.target_residency_ns, NSEC_PER_USEC);
        }
        if s.exit_latency > 0 {
            s.exit_latency_ns = mul_u32_u32(s.exit_latency, NSEC_PER_USEC);
        } else if s.exit_latency_ns < 0 {
            s.exit_latency_ns = 0;
        } else {
            s.exit_latency = div_u64(s.exit_latency_ns, NSEC_PER_USEC);
        }
    }
}

unsafe fn __cpuidle_register_driver(drv: *mut cpuidle_driver) -> i32 {
    if drv.is_null() || (*drv).state_count == 0 { return -EINVAL; }
    let mut ret = cpuidle_coupled_state_verify(drv);
    if ret != 0 { return ret; }
    if cpuidle_disabled() { return -ENODEV; }
    __cpuidle_driver_init(drv);
    ret = __cpuidle_set_driver(drv);
    if ret != 0 { return ret; }
    if (*drv).bctimer != 0 {
        on_each_cpu_mask((*drv).cpumask, cpuidle_setup_broadcast_timer,
                         1 as *mut core::ffi::c_void, 1);
    }
    0
}

unsafe fn __cpuidle_unregister_driver(drv: *mut cpuidle_driver) {
    if (*drv).bctimer != 0 {
        (*drv).bctimer = 0;
        on_each_cpu_mask((*drv).cpumask, cpuidle_setup_broadcast_timer,
                         core::ptr::null_mut(), 1);
    }
    __cpuidle_unset_driver(drv);
}

pub unsafe fn cpuidle_register_driver(drv: *mut cpuidle_driver) -> i32 {
    let mut ret: i32;
    spin_lock(&mut cpuidle_driver_lock);
    ret = __cpuidle_register_driver(drv);
    spin_unlock(&mut cpuidle_driver_lock);
    if ret == 0 && strlen(param_governor) == 0 && !drv.is_null() && !(*drv).governor.is_null()
        && cpuidle_get_driver() == drv {
        mutex_lock(&mut cpuidle_lock);
        let gov = cpuidle_find_governor((*drv).governor);
        if !gov.is_null() {
            cpuidle_prev_governor = cpuidle_curr_governor;
            if cpuidle_switch_governor(gov) < 0 { cpuidle_prev_governor = core::ptr::null_mut(); }
        }
        mutex_unlock(&mut cpuidle_lock);
    }
    ret
}

pub unsafe fn cpuidle_unregister_driver(drv: *mut cpuidle_driver) {
    let enabled = cpuidle_get_driver() == drv;
    spin_lock(&mut cpuidle_driver_lock);
    __cpuidle_unregister_driver(drv);
    spin_unlock(&mut cpuidle_driver_lock);
    if !enabled { return; }
    mutex_lock(&mut cpuidle_lock);
    if !cpuidle_prev_governor.is_null() {
        if cpuidle_switch_governor(cpuidle_prev_governor) == 0 { cpuidle_prev_governor = core::ptr::null_mut(); }
    }
    mutex_unlock(&mut cpuidle_lock);
}

pub unsafe fn cpuidle_get_driver() -> *mut cpuidle_driver {
    let cpu = get_cpu();
    let drv = __cpuidle_get_cpu_driver(cpu);
    put_cpu();
    drv
}

pub unsafe fn cpuidle_get_cpu_driver(dev: *mut cpuidle_device) -> *mut cpuidle_driver {
    if dev.is_null() { return core::ptr::null_mut(); }
    __cpuidle_get_cpu_driver((*dev).cpu)
}

pub unsafe fn cpuidle_driver_state_disabled(drv: *mut cpuidle_driver, idx: i32, disable: bool) {
    let mut cpu: u32;
    mutex_lock(&mut cpuidle_lock);
    spin_lock(&mut cpuidle_driver_lock);
    if (*drv).cpumask.is_null() {
        (*drv).states[idx as usize].flags |= CPUIDLE_FLAG_UNUSABLE;
        goto_unlock!();
    }
    for_each_cpu!(cpu, (*drv).cpumask) {
        let dev = per_cpu!(cpuidle_devices, cpu);
        if dev.is_null() { continue; }
        if disable {
            (*dev).states_usage[idx as usize].disable |= CPUIDLE_STATE_DISABLED_BY_DRIVER;
        } else {
            (*dev).states_usage[idx as usize].disable &= !CPUIDLE_STATE_DISABLED_BY_DRIVER;
        }
    }
    spin_unlock(&mut cpuidle_driver_lock);
    mutex_unlock(&mut cpuidle_lock);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
