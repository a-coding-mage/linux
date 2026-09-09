// SPDX-License-Identifier: GPL-2.0-only
/*
 * OMAP4+ CPU idle Routines
 *
 * Copyright (C) 2011-2013 Texas Instruments, Inc.
 * Santosh Shilimkar <santosh.shilimkar@ti.com>
 * Rajendra Nayak <rnayak@ti.com>
 */

// Translated from C. Kernel-provided declarations and constants are external dependencies.

const MAX_CPUS: usize = 2;

#[repr(C)]
struct idle_statedata {
    cpu_state: u32,
    mpu_logic_state: u32,
    mpu_state: u32,
    mpu_state_vote: u32,
}

static mut omap4_idle_data: [idle_statedata; 3] = [
    idle_statedata { cpu_state: PWRDM_POWER_ON, mpu_state: PWRDM_POWER_ON, mpu_logic_state: PWRDM_POWER_RET, mpu_state_vote: 0 },
    idle_statedata { cpu_state: PWRDM_POWER_OFF, mpu_state: PWRDM_POWER_RET, mpu_logic_state: PWRDM_POWER_RET, mpu_state_vote: 0 },
    idle_statedata { cpu_state: PWRDM_POWER_OFF, mpu_state: PWRDM_POWER_RET, mpu_logic_state: PWRDM_POWER_OFF, mpu_state_vote: 0 },
];

static mut omap5_idle_data: [idle_statedata; 2] = [
    idle_statedata { cpu_state: PWRDM_POWER_ON, mpu_state: PWRDM_POWER_ON, mpu_logic_state: PWRDM_POWER_ON, mpu_state_vote: 0 },
    idle_statedata { cpu_state: PWRDM_POWER_RET, mpu_state: PWRDM_POWER_RET, mpu_logic_state: PWRDM_POWER_RET, mpu_state_vote: 0 },
];

static mut mpu_pd: *mut powerdomain = core::ptr::null_mut();
static mut cpu_pd: [*mut powerdomain; MAX_CPUS] = [core::ptr::null_mut(); MAX_CPUS];
static mut cpu_clkdm: [*mut clockdomain; MAX_CPUS] = [core::ptr::null_mut(); MAX_CPUS];
static mut abort_barrier: atomic_t = atomic_t { _private: 0 };
static mut cpu_done: [bool; MAX_CPUS] = [false; MAX_CPUS];
static mut state_ptr: *mut idle_statedata = core::ptr::null_mut();
static mut mpu_lock: raw_spinlock_t = raw_spinlock_t { _private: 0 };

#[repr(C)] struct powerdomain { _private: u8 }
#[repr(C)] struct clockdomain { _private: u8 }
#[repr(C)] struct atomic_t { _private: i32 }
#[repr(C)] struct raw_spinlock_t { _private: u8 }
#[repr(C)] struct cpuidle_device { cpu: i32 }
#[repr(C)] struct cpuidle_driver { _private: u8 }

extern "C" {
    static cpu_online_mask: core::ffi::c_void;
    fn omap_do_wfi();
    fn raw_spin_lock_irqsave(lock: *mut raw_spinlock_t, flags: *mut usize);
    fn raw_spin_unlock_irqrestore(lock: *mut raw_spinlock_t, flags: usize);
    fn num_online_cpus() -> u32;
    fn pwrdm_set_logic_retst(pd: *mut powerdomain, state: u32);
    fn omap_set_pwrdm_state(pd: *mut powerdomain, state: u32);
    fn omap4_enter_lowpower(cpu: i32, state: u32, flag: bool);
    fn pwrdm_read_pwrst(pd: *mut powerdomain) -> u32;
    fn cpu_relax();
    fn cpumask_test_cpu(cpu: i32, mask: *const core::ffi::c_void) -> bool;
    fn tick_broadcast_enable();
    fn tick_broadcast_enter();
    fn cpu_pm_enter() -> i32;
    fn cpu_cluster_pm_enter() -> i32;
    fn gic_dist_disable();
    fn clkdm_deny_idle(cd: *mut clockdomain);
    fn clkdm_allow_idle(cd: *mut clockdomain);
    fn udelay(usecs: u32);
    fn gic_dist_disabled() -> bool;
    fn gic_timer_retrigger();
    fn cpu_cluster_pm_exit();
    fn cpu_pm_exit();
    fn tick_broadcast_exit();
    fn cpuidle_coupled_parallel_barrier(dev: *mut cpuidle_device, barrier: *mut atomic_t);
    fn soc_is_omap54xx() -> bool;
    fn pwrdm_lookup(name: *const u8) -> *mut powerdomain;
    fn clkdm_lookup(name: *const u8) -> *mut clockdomain;
    fn cpuidle_register(driver: *mut cpuidle_driver, mask: *const core::ffi::c_void) -> i32;
}

unsafe fn omap_enter_idle_simple(_dev: *mut cpuidle_device, _drv: *mut cpuidle_driver, index: i32) -> i32 {
    omap_do_wfi();
    index
}

unsafe fn omap_enter_idle_smp(dev: *mut cpuidle_device, _drv: *mut cpuidle_driver, index: i32) -> i32 {
    let cx = state_ptr.add(index as usize);
    let mut flag = 0usize;
    raw_spin_lock_irqsave(&raw mut mpu_lock, &mut flag);
    (*cx).mpu_state_vote = (*cx).mpu_state_vote.wrapping_add(1);
    if (*cx).mpu_state_vote == num_online_cpus() {
        pwrdm_set_logic_retst(mpu_pd, (*cx).mpu_logic_state);
        omap_set_pwrdm_state(mpu_pd, (*cx).mpu_state);
    }
    raw_spin_unlock_irqrestore(&raw mut mpu_lock, flag);
    omap4_enter_lowpower((*dev).cpu, (*cx).cpu_state, true);
    raw_spin_lock_irqsave(&raw mut mpu_lock, &mut flag);
    if (*cx).mpu_state_vote == num_online_cpus() { omap_set_pwrdm_state(mpu_pd, PWRDM_POWER_ON); }
    (*cx).mpu_state_vote = (*cx).mpu_state_vote.wrapping_sub(1);
    raw_spin_unlock_irqrestore(&raw mut mpu_lock, flag);
    index
}

unsafe fn omap_enter_idle_coupled(dev: *mut cpuidle_device, _drv: *mut cpuidle_driver, mut index: i32) -> i32 {
    let mut cx = state_ptr.add(index as usize);
    let mut mpuss_can_lose_context = ((*cx).mpu_state == PWRDM_POWER_RET) && ((*cx).mpu_logic_state == PWRDM_POWER_OFF);
    if (*dev).cpu == 0 && cpumask_test_cpu(1, &raw const cpu_online_mask) {
        while pwrdm_read_pwrst(cpu_pd[1]) != PWRDM_POWER_OFF {
            cpu_relax();
            if cpu_done[1] { cpuidle_coupled_parallel_barrier(dev, &raw mut abort_barrier); cpu_done[(*dev).cpu as usize] = false; return index; }
        }
    }
    tick_broadcast_enable(); tick_broadcast_enter();
    let error = cpu_pm_enter();
    if error == 0 {
        if (*dev).cpu == 0 {
            pwrdm_set_logic_retst(mpu_pd, (*cx).mpu_logic_state); omap_set_pwrdm_state(mpu_pd, (*cx).mpu_state);
            if mpuss_can_lose_context {
                if cpu_cluster_pm_enter() != 0 { index = 0; cx = state_ptr; pwrdm_set_logic_retst(mpu_pd, (*cx).mpu_logic_state); omap_set_pwrdm_state(mpu_pd, (*cx).mpu_state); mpuss_can_lose_context = false; }
            }
        }
        omap4_enter_lowpower((*dev).cpu, (*cx).cpu_state, true); cpu_done[(*dev).cpu as usize] = true;
        if (*dev).cpu == 0 && cpumask_test_cpu(1, &raw const cpu_online_mask) {
            if IS_PM44XX_ERRATUM(PM_OMAP4_ROM_SMP_BOOT_ERRATUM_GICD) && mpuss_can_lose_context { gic_dist_disable(); }
            clkdm_deny_idle(cpu_clkdm[1]); omap_set_pwrdm_state(cpu_pd[1], PWRDM_POWER_ON); clkdm_allow_idle(cpu_clkdm[1]);
            if IS_PM44XX_ERRATUM(PM_OMAP4_ROM_SMP_BOOT_ERRATUM_GICD) && mpuss_can_lose_context {
                while gic_dist_disabled() { udelay(1); cpu_relax(); }
                gic_timer_retrigger();
            }
        }
        if (*dev).cpu == 0 && mpuss_can_lose_context { cpu_cluster_pm_exit(); }
        cpu_pm_exit();
    }
    tick_broadcast_exit(); cpuidle_coupled_parallel_barrier(dev, &raw mut abort_barrier); cpu_done[(*dev).cpu as usize] = false; index
}

static mut omap4_idle_driver: cpuidle_driver = cpuidle_driver { _private: 0 };
static mut omap5_idle_driver: cpuidle_driver = cpuidle_driver { _private: 0 };

pub unsafe fn omap4_idle_init() -> i32 {
    let idle_driver: *mut cpuidle_driver;
    if soc_is_omap54xx() { state_ptr = &raw mut omap5_idle_data[0]; idle_driver = &raw mut omap5_idle_driver; }
    else { state_ptr = &raw mut omap4_idle_data[0]; idle_driver = &raw mut omap4_idle_driver; }
    mpu_pd = pwrdm_lookup(b"mpu_pwrdm\0".as_ptr()); cpu_pd[0] = pwrdm_lookup(b"cpu0_pwrdm\0".as_ptr()); cpu_pd[1] = pwrdm_lookup(b"cpu1_pwrdm\0".as_ptr());
    if mpu_pd.is_null() || cpu_pd[0].is_null() || cpu_pd[1].is_null() { return -19; }
    cpu_clkdm[0] = clkdm_lookup(b"mpu0_clkdm\0".as_ptr()); cpu_clkdm[1] = clkdm_lookup(b"mpu1_clkdm\0".as_ptr());
    if cpu_clkdm[0].is_null() || cpu_clkdm[1].is_null() { return -19; }
    cpuidle_register(idle_driver, &raw const cpu_online_mask)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
