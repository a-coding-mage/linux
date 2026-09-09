// SPDX-License-Identifier: GPL-2.0
/*
 *  cpuidle-powernv - idle state cpuidle driver.
 *  Adapted from drivers/cpuidle/cpuidle-pseries
 *
 */

// Kernel and architecture dependencies supplied by other translation units.

const POWERNV_THRESHOLD_LATENCY_NS: u32 = 200000;

static mut powernv_idle_driver: cpuidle_driver = cpuidle_driver {
    name: *b"powernv_idle\0",
    owner: THIS_MODULE,
    ..cpuidle_driver::ZERO
};

static mut max_idle_state: i32 = 0;
static mut cpuidle_state_table: *mut cpuidle_state = core::ptr::null_mut();

#[repr(C)]
struct stop_psscr_table {
    val: u64,
    mask: u64,
}

static mut stop_psscr_table: [stop_psscr_table; CPUIDLE_STATE_MAX] =
    [stop_psscr_table { val: 0, mask: 0 }; CPUIDLE_STATE_MAX];

static mut default_snooze_timeout: u64 = 0;
static mut snooze_timeout_en: bool = false;

unsafe fn get_snooze_timeout(dev: *mut cpuidle_device, drv: *mut cpuidle_driver, index: i32) -> u64 {
    let mut i: i32;

    if unlikely(!snooze_timeout_en) {
        return default_snooze_timeout;
    }

    i = index + 1;
    while i < (*drv).state_count {
        if (*dev).states_usage[i as usize].disable {
            i += 1;
            continue;
        }
        return (*drv).states[i as usize].target_residency as u64 * tb_ticks_per_usec;
    }
    default_snooze_timeout
}

unsafe extern "C" fn snooze_loop(dev: *mut cpuidle_device, drv: *mut cpuidle_driver, index: i32) -> i32 {
    let snooze_exit_time: u64;

    set_thread_flag(TIF_POLLING_NRFLAG);
    local_irq_enable();
    snooze_exit_time = get_tb() + get_snooze_timeout(dev, drv, index);
    (*dev).poll_time_limit = false;
    ppc64_runlatch_off();
    HMT_very_low();
    while !need_resched() {
        if likely(snooze_timeout_en) && get_tb() > snooze_exit_time {
            /*
             * Task has not woken up but we are exiting the polling
             * loop anyway. Require a barrier after polling is
             * cleared to order subsequent test of need_resched().
             */
            clear_thread_flag(TIF_POLLING_NRFLAG);
            (*dev).poll_time_limit = true;
            smp_mb();
            break;
        }
    }
    HMT_medium();
    ppc64_runlatch_on();
    /* Avoid double clear when breaking */
    if !(*dev).poll_time_limit {
        clear_thread_flag(TIF_POLLING_NRFLAG);
    }
    local_irq_disable();
    index
}

unsafe extern "C" fn nap_loop(_dev: *mut cpuidle_device, _drv: *mut cpuidle_driver, index: i32) -> i32 {
    power7_idle_type(PNV_THREAD_NAP);
    index
}

/* Register for fastsleep only in oneshot mode of broadcast */
#[cfg(CONFIG_TICK_ONESHOT)]
unsafe extern "C" fn fastsleep_loop(_dev: *mut cpuidle_device, _drv: *mut cpuidle_driver, index: i32) -> i32 {
    let old_lpcr: usize = mfspr(SPRN_LPCR);
    let mut new_lpcr: usize;
    if unlikely(system_state < SYSTEM_RUNNING) { return index; }
    new_lpcr = old_lpcr;
    /* Do not exit powersave upon decrementer as we've setup the timer offload. */
    new_lpcr &= !LPCR_PECE1;
    mtspr(SPRN_LPCR, new_lpcr);
    power7_idle_type(PNV_THREAD_SLEEP);
    mtspr(SPRN_LPCR, old_lpcr);
    index
}

unsafe extern "C" fn stop_loop(_dev: *mut cpuidle_device, _drv: *mut cpuidle_driver, index: i32) -> i32 {
    arch300_idle_type(stop_psscr_table[index as usize].val, stop_psscr_table[index as usize].mask);
    index
}

/* States for dedicated partition case. */
static mut powernv_states: [cpuidle_state; CPUIDLE_STATE_MAX] = [
    cpuidle_state {
        name: *b"snooze\0", desc: *b"snooze\0", exit_latency: 0,
        target_residency: 0, enter: Some(snooze_loop), flags: CPUIDLE_FLAG_POLLING,
        ..cpuidle_state::ZERO
    }
];

unsafe extern "C" fn powernv_cpuidle_cpu_online(cpu: u32) -> i32 {
    let dev = per_cpu(cpuidle_devices, cpu);
    if !dev.is_null() && !cpuidle_get_driver().is_null() {
        cpuidle_pause_and_lock(); cpuidle_enable_device(dev); cpuidle_resume_and_unlock();
    }
    0
}

unsafe extern "C" fn powernv_cpuidle_cpu_dead(cpu: u32) -> i32 {
    let dev = per_cpu(cpuidle_devices, cpu);
    if !dev.is_null() && !cpuidle_get_driver().is_null() {
        cpuidle_pause_and_lock(); cpuidle_disable_device(dev); cpuidle_resume_and_unlock();
    }
    0
}

/* powernv_cpuidle_driver_init() */
unsafe fn powernv_cpuidle_driver_init() -> i32 {
    let drv = &mut powernv_idle_driver;
    (*drv).state_count = 0;
    let mut idle_state = 0;
    while idle_state < max_idle_state {
        if (*cpuidle_state_table.add(idle_state as usize)).enter.is_none() { idle_state += 1; continue; }
        (*drv).states[(*drv).state_count as usize] = *cpuidle_state_table.add(idle_state as usize);
        (*drv).state_count += 1;
        idle_state += 1;
    }
    (*drv).cpumask = cpu_present_mask as *mut cpumask;
    0
}

unsafe fn add_powernv_state(index: i32, name: *const u8, flags: u32,
    idle_fn: Option<unsafe extern "C" fn(*mut cpuidle_device, *mut cpuidle_driver, i32) -> i32>,
    target_residency: u32, exit_latency: u32, psscr_val: u64, psscr_mask: u64) {
    strscpy(powernv_states[index as usize].name.as_mut_ptr(), name, CPUIDLE_NAME_LEN);
    strscpy(powernv_states[index as usize].desc.as_mut_ptr(), name, CPUIDLE_NAME_LEN);
    powernv_states[index as usize].flags = flags;
    powernv_states[index as usize].target_residency = target_residency;
    powernv_states[index as usize].exit_latency = exit_latency;
    powernv_states[index as usize].enter = idle_fn;
    stop_psscr_table[index as usize].val = psscr_val;
    stop_psscr_table[index as usize].mask = psscr_mask;
}

extern "C" { fn pnv_get_supported_cpuidle_states() -> u32; }

// The remaining platform types, globals, constants, and functions are supplied by the kernel headers.
unsafe fn powernv_add_idle_states() -> i32 {
    let mut nr_idle_states = 1;
    let mut dt_idle_states: i32;
    let mut has_stop_states: u32 = 0;
    let supported_flags = pnv_get_supported_cpuidle_states();
    if nr_pnv_idle_states <= 0 { pr_warn!("cpuidle-powernv : Only Snooze is available\n"); return nr_idle_states; }
    dt_idle_states = nr_pnv_idle_states;
    if nr_pnv_idle_states > CPUIDLE_STATE_MAX - 1 { pr_warn!("cpuidle-powernv: discovered idle states more than allowed"); dt_idle_states = CPUIDLE_STATE_MAX - 1; }
    has_stop_states = pnv_idle_states[0].flags & (OPAL_PM_STOP_INST_FAST | OPAL_PM_STOP_INST_DEEP);
    let mut i = 0;
    while i < dt_idle_states {
        let state = &pnv_idle_states[i as usize];
        if state.flags & supported_flags != state.flags || state.latency_ns > POWERNV_THRESHOLD_LATENCY_NS || (has_stop_states != 0 && !state.valid) { i += 1; continue; }
        let exit_latency = DIV_ROUND_UP(state.latency_ns, 1000);
        let target_residency = DIV_ROUND_UP(state.residency_ns, 1000);
        let stops_timebase = state.flags & OPAL_PM_TIMEBASE_STOP != 0;
        if state.flags & OPAL_PM_NAP_ENABLED != 0 { add_powernv_state(nr_idle_states, b"Nap\0".as_ptr(), CPUIDLE_FLAG_NONE, Some(nap_loop), target_residency, exit_latency, 0, 0); }
        else if has_stop_states != 0 && !stops_timebase { add_powernv_state(nr_idle_states, state.name.as_ptr(), CPUIDLE_FLAG_NONE, Some(stop_loop), target_residency, exit_latency, state.psscr_val, state.psscr_mask); }
        // #ifdef CONFIG_TICK_ONESHOT
        else if state.flags & (OPAL_PM_SLEEP_ENABLED | OPAL_PM_SLEEP_ENABLED_ER1) != 0 {
            add_powernv_state(nr_idle_states, b"FastSleep\0".as_ptr(), CPUIDLE_FLAG_TIMER_STOP, Some(fastsleep_loop), target_residency, exit_latency, 0, 0);
        } else if has_stop_states != 0 && stops_timebase {
            add_powernv_state(nr_idle_states, state.name.as_ptr(), CPUIDLE_FLAG_TIMER_STOP, Some(stop_loop), target_residency, exit_latency, state.psscr_val, state.psscr_mask);
        }
        // #endif
        else { i += 1; continue; }
        nr_idle_states += 1; i += 1;
    }
    nr_idle_states
}

unsafe fn powernv_idle_probe() -> i32 {
    if cpuidle_disable != IDLE_NO_OVERRIDE { return -ENODEV; }
    if firmware_has_feature(FW_FEATURE_OPAL) { cpuidle_state_table = powernv_states.as_mut_ptr(); max_idle_state = powernv_add_idle_states(); default_snooze_timeout = TICK_USEC * tb_ticks_per_usec; if max_idle_state > 1 { snooze_timeout_en = true; } } else { return -ENODEV; }
    0
}

unsafe extern "C" fn powernv_processor_idle_init() -> i32 {
    let mut retval = powernv_idle_probe();
    if retval != 0 { return retval; }
    powernv_cpuidle_driver_init();
    retval = cpuidle_register(&mut powernv_idle_driver, core::ptr::null_mut());
    if retval != 0 { printk!(KERN_DEBUG "Registration of powernv driver failed.\n"); return retval; }
    retval = cpuhp_setup_state_nocalls(CPUHP_AP_ONLINE_DYN, b"cpuidle/powernv:online\0".as_ptr(), Some(powernv_cpuidle_cpu_online), None); WARN_ON(retval < 0);
    retval = cpuhp_setup_state_nocalls(CPUHP_CPUIDLE_DEAD, b"cpuidle/powernv:dead\0".as_ptr(), None, Some(powernv_cpuidle_cpu_dead)); WARN_ON(retval < 0);
    printk!(KERN_DEBUG "powernv_idle_driver registered\n");
    0
}

device_initcall!(powernv_processor_idle_init);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
