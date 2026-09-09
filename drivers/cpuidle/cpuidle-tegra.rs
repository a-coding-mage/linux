// SPDX-License-Identifier: GPL-2.0-only
/* CPU idle driver for Tegra CPUs */

// C dependencies supplied by the surrounding kernel translation unit.

#[repr(C)]
#[derive(Copy, Clone)]
enum tegra_state {
    TEGRA_C1,
    TEGRA_C7,
    TEGRA_CC6,
    TEGRA_STATE_COUNT,
}

static mut tegra_idle_barrier: atomic_t = atomic_t { counter: 0 };
static mut tegra_abort_flag: atomic_t = atomic_t { counter: 0 };

unsafe fn tegra_cpuidle_report_cpus_state() {
    let mut cpu: c_ulong;
    let mut lcpu: c_ulong;
    let mut csr: c_ulong;

    for_each_cpu!(lcpu, cpu_possible_mask);
    {
        cpu = cpu_logical_map(lcpu);
        csr = flowctrl_read_cpu_csr(cpu);
        pr_err!("cpu{}: online={} flowctrl_csr=0x{:08x}\n", cpu, cpu_online(lcpu), csr);
    }
}

unsafe fn tegra_cpuidle_wait_for_secondary_cpus_parking() -> c_int {
    let mut retries: c_uint = 3;
    while retries != 0 {
        retries -= 1;
        let delay_us: c_uint = 10;
        let mut timeout_us: c_uint = 500 * 1000 / delay_us;
        // CPU0 waits for secondary shutdown so the CPU cluster can be powered off safely.
        loop {
            if tegra_cpu_rail_off_ready() { return 0; }
            udelay(delay_us);
            if timeout_us == 0 { break; }
            timeout_us -= 1;
        }
        pr_err!("secondary CPU taking too long to park\n");
        tegra_cpuidle_report_cpus_state();
    }
    pr_err!("timed out waiting secondaries to park\n");
    -ETIMEDOUT
}

unsafe fn tegra_cpuidle_unpark_secondary_cpus() {
    let mut cpu: c_uint;
    let mut lcpu: c_uint;
    for_each_cpu!(lcpu, cpu_online_mask);
    {
        cpu = cpu_logical_map(lcpu);
        if cpu > 0 {
            tegra_enable_cpu_clock(cpu);
            tegra_cpu_out_of_reset(cpu);
            flowctrl_write_cpu_halt(cpu, 0);
        }
    }
}

unsafe fn tegra_cpuidle_cc6_enter(cpu: c_uint) -> c_int {
    let ret;
    if cpu > 0 {
        ret = cpu_suspend(cpu, tegra_pm_park_secondary_cpu);
    } else {
        ret = tegra_cpuidle_wait_for_secondary_cpus_parking();
        let ret = if ret == 0 { tegra_pm_enter_lp2() } else { ret };
        tegra_cpuidle_unpark_secondary_cpus();
        return ret;
    }
    ret
}

unsafe fn tegra_cpuidle_c7_enter() -> c_int {
    let err = call_firmware_op!(prepare_idle, TF_PM_MODE_LP2_NOFLUSH_L2);
    if err != 0 && err != -ENOSYS { return err; }
    cpu_suspend(0, tegra30_pm_secondary_cpu_suspend)
}

unsafe fn tegra_cpuidle_coupled_barrier(dev: *mut cpuidle_device) -> c_int {
    if tegra_pending_sgi() { atomic_set(&mut tegra_abort_flag, 1); }
    cpuidle_coupled_parallel_barrier(dev, &mut tegra_idle_barrier);
    if atomic_read(&tegra_abort_flag) != 0 {
        cpuidle_coupled_parallel_barrier(dev, &mut tegra_idle_barrier);
        atomic_set(&mut tegra_abort_flag, 0);
        return -EINTR;
    }
    0
}

unsafe fn tegra_cpuidle_state_enter(dev: *mut cpuidle_device, index: c_int, cpu: c_uint) -> c_int {
    let mut err: c_int;
    if index == TEGRA_CC6 as c_int {
        err = tegra_cpuidle_coupled_barrier(dev);
        if err != 0 { return err; }
    }
    local_fiq_disable();
    tegra_pm_set_cpu_in_lp2();
    cpu_pm_enter();
    ct_cpuidle_enter();
    err = match index {
        x if x == TEGRA_C7 as c_int => tegra_cpuidle_c7_enter(),
        x if x == TEGRA_CC6 as c_int => tegra_cpuidle_cc6_enter(cpu),
        _ => -EINVAL,
    };
    ct_cpuidle_exit();
    cpu_pm_exit();
    tegra_pm_clear_cpu_in_lp2();
    local_fiq_enable();
    if err != 0 { err } else { index }
}

unsafe fn tegra_cpuidle_adjust_state_index(mut index: c_int, cpu: c_uint) -> c_int {
    if cpu > 0 || index != TEGRA_C7 as c_int || tegra_get_chip_id() != TEGRA30 { return index; }
    // CONFIG_PM_SLEEP is a build-time condition preserved from the C implementation.
    if !IS_ENABLED!(CONFIG_PM_SLEEP) || num_online_cpus() > 1 { index = TEGRA_C1 as c_int; }
    else { index = TEGRA_CC6 as c_int; }
    index
}

unsafe fn tegra_cpuidle_enter(dev: *mut cpuidle_device, drv: *mut cpuidle_driver, mut index: c_int) -> c_int {
    let do_rcu = (*drv).states[index as usize].flags & CPUIDLE_FLAG_RCU_IDLE != 0;
    let cpu = cpu_logical_map((*dev).cpu);
    let mut ret: c_int;
    index = tegra_cpuidle_adjust_state_index(index, cpu);
    if (*dev).states_usage[index as usize].disable { return -1; }
    if index == TEGRA_C1 as c_int {
        if do_rcu { ct_cpuidle_enter(); }
        ret = arm_cpuidle_simple_enter(dev, drv, index);
        if do_rcu { ct_cpuidle_exit(); }
    } else { ret = tegra_cpuidle_state_enter(dev, index, cpu); }
    if ret < 0 {
        if ret != -EINTR || index != TEGRA_CC6 as c_int { pr_err_once!("failed to enter state {} err: {}\n", index, ret); }
        -1
    } else { ret }
}

unsafe fn tegra114_enter_s2idle(dev: *mut cpuidle_device, drv: *mut cpuidle_driver, index: c_int) -> c_int {
    tegra_cpuidle_enter(dev, drv, index);
    0
}

// Legacy state mapping: LP3 -> C1, LP2 -> C7, LP2 -> CC6.
static mut tegra_idle_driver: cpuidle_driver = cpuidle_driver {
    name: "tegra_idle", states: [
        ARM_CPUIDLE_WFI_STATE_PWR!(600),
        cpuidle_state { enter: Some(tegra_cpuidle_enter), exit_latency: 2000, target_residency: 2200, power_usage: 100, flags: CPUIDLE_FLAG_TIMER_STOP | CPUIDLE_FLAG_RCU_IDLE, name: "C7", desc: "CPU core powered off", ..CPUIDLE_STATE_INIT },
        cpuidle_state { enter: Some(tegra_cpuidle_enter), exit_latency: 5000, target_residency: 10000, power_usage: 0, flags: CPUIDLE_FLAG_TIMER_STOP | CPUIDLE_FLAG_RCU_IDLE | CPUIDLE_FLAG_COUPLED, name: "CC6", desc: "CPU cluster powered off", ..CPUIDLE_STATE_INIT },
    ], state_count: TEGRA_STATE_COUNT as c_uint, safe_state_index: TEGRA_C1 as c_uint, ..CPUIDLE_DRIVER_INIT
};

unsafe fn tegra_cpuidle_disable_state(state: tegra_state) { cpuidle_driver_state_disabled(&mut tegra_idle_driver, state as c_uint, true); }

#[no_mangle]
pub unsafe extern "C" fn tegra_cpuidle_pcie_irqs_in_use() {
    let state_cc6 = &mut tegra_idle_driver.states[TEGRA_CC6 as usize];
    if state_cc6.flags & CPUIDLE_FLAG_UNUSABLE != 0 || tegra_get_chip_id() != TEGRA20 { return; }
    pr_info!("disabling CC6 state, since PCIe IRQs are in use\n");
    tegra_cpuidle_disable_state(TEGRA_CC6);
}

unsafe fn tegra_cpuidle_setup_tegra114_c7_state() {
    let s = &mut tegra_idle_driver.states[TEGRA_C7 as usize];
    s.enter_s2idle = Some(tegra114_enter_s2idle); s.target_residency = 1000; s.exit_latency = 500;
}

unsafe fn tegra_cpuidle_probe(_pdev: *mut platform_device) -> c_int {
    if tegra_pmc_get_suspend_mode() == TEGRA_SUSPEND_NOT_READY { return -EPROBE_DEFER; }
    if tegra_pmc_get_suspend_mode() < TEGRA_SUSPEND_LP2 { tegra_cpuidle_disable_state(TEGRA_CC6); }
    if !IS_ENABLED!(CONFIG_PM_SLEEP) { tegra_cpuidle_disable_state(TEGRA_C7); tegra_cpuidle_disable_state(TEGRA_CC6); }
    match tegra_get_chip_id() {
        TEGRA20 => tegra_cpuidle_disable_state(TEGRA_C7),
        TEGRA30 => (),
        TEGRA114 | TEGRA124 => { tegra_cpuidle_setup_tegra114_c7_state(); tegra_cpuidle_disable_state(TEGRA_CC6); },
        _ => return -EINVAL,
    }
    cpuidle_register(&mut tegra_idle_driver, cpu_possible_mask)
}

// Equivalent of builtin_platform_driver(tegra_cpuidle_driver); platform registration is supplied externally.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
