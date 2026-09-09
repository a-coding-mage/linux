// SPDX-License-Identifier: GPL-2.0-only
/*
 * PSCI CPU idle driver.
 *
 * Copyright (C) 2019 ARM Ltd.
 * Author: Lorenzo Pieralisi <lorenzo.pieralisi@arm.com>
 */

// Dependencies supplied by the surrounding kernel translation.

#[repr(C)]
pub struct PsciCpuidleData {
    pub psci_states: *mut u32,
    pub dev: *mut Device,
}

#[repr(C)]
pub struct PsciCpuidleDomainState {
    pub pd: *mut GenericPmDomain,
    pub state_idx: c_uint,
    pub state: u32,
}

static mut psci_cpuidle_data: PerCpu<PsciCpuidleData> = PerCpu::new();
static mut psci_domain_state: PerCpu<PsciCpuidleDomainState> = PerCpu::new();
static mut psci_cpuidle_use_syscore: bool = false;

pub unsafe fn psci_set_domain_state(pd: *mut GenericPmDomain, state_idx: c_uint, state: u32) {
    let ds = this_cpu_ptr(&mut psci_domain_state);
    (*ds).pd = pd;
    (*ds).state_idx = state_idx;
    (*ds).state = state;
}

#[inline]
unsafe fn psci_clear_domain_state() {
    this_cpu_write(&mut psci_domain_state, |ds| (*ds).state = 0);
}

unsafe fn __psci_enter_domain_idle_state(
    dev: *mut CpuidleDevice, drv: *mut CpuidleDriver, idx: c_int, s2idle: bool,
) -> c_int {
    let data = this_cpu_ptr(&mut psci_cpuidle_data);
    let states = (*data).psci_states;
    let pd_dev = (*data).dev;
    let ds: *mut PsciCpuidleDomainState;
    let mut state = *states.add(idx as usize);
    let mut ret = cpu_pm_enter();
    if ret != 0 { return -1; }

    /* Do runtime PM to manage a hierarchical CPU toplogy. */
    if s2idle { dev_pm_genpd_suspend(pd_dev); } else { pm_runtime_put_sync_suspend(pd_dev); }

    ds = this_cpu_ptr(&mut psci_domain_state);
    if (*ds).state != 0 { state = (*ds).state; }

    trace_psci_domain_idle_enter((*dev).cpu, state, s2idle);
    ret = if psci_cpu_suspend_enter(state) { -1 } else { idx };
    trace_psci_domain_idle_exit((*dev).cpu, state, s2idle);

    if s2idle { dev_pm_genpd_resume(pd_dev); } else { pm_runtime_get_sync(pd_dev); }
    cpu_pm_exit();

    /* Correct domain-idlestate statistics if we failed to enter. */
    if ret == -1 && (*ds).state != 0 { pm_genpd_inc_rejected((*ds).pd, (*ds).state_idx); }
    /* Clear the domain state to start fresh when back from idle. */
    psci_clear_domain_state();
    ret
}

unsafe fn psci_enter_domain_idle_state(dev: *mut CpuidleDevice, drv: *mut CpuidleDriver, idx: c_int) -> c_int {
    __psci_enter_domain_idle_state(dev, drv, idx, false)
}

unsafe fn psci_enter_s2idle_domain_idle_state(dev: *mut CpuidleDevice, drv: *mut CpuidleDriver, idx: c_int) -> c_int {
    __psci_enter_domain_idle_state(dev, drv, idx, true)
}

unsafe fn psci_idle_cpuhp_up(cpu: c_uint) -> c_int {
    let pd_dev = this_cpu_read(&psci_cpuidle_data, |d| d.dev);
    if !pd_dev.is_null() {
        if !IS_ENABLED(CONFIG_PREEMPT_RT) { pm_runtime_get_sync(pd_dev); } else { dev_pm_genpd_resume(pd_dev); }
    }
    0
}

unsafe fn psci_idle_cpuhp_down(cpu: c_uint) -> c_int {
    let pd_dev = this_cpu_read(&psci_cpuidle_data, |d| d.dev);
    if !pd_dev.is_null() {
        if !IS_ENABLED(CONFIG_PREEMPT_RT) { pm_runtime_put_sync(pd_dev); } else { dev_pm_genpd_suspend(pd_dev); }
        /* Clear domain state to start fresh at next online. */
        psci_clear_domain_state();
    }
    0
}

unsafe fn psci_idle_syscore_switch(suspend: bool) {
    let mut cleared = false;
    let mut dev: *mut Device;
    let mut cpu: c_int;
    for_each_possible_cpu!(cpu) {
        dev = per_cpu_ptr(&psci_cpuidle_data, cpu).as_ref().unwrap().dev;
        if !dev.is_null() && suspend { dev_pm_genpd_suspend(dev); }
        else if !dev.is_null() {
            dev_pm_genpd_resume(dev);
            /* Account for userspace having offlined a CPU. */
            if pm_runtime_status_suspended(dev) { pm_runtime_set_active(dev); }
            /* Clear domain state to re-start fresh. */
            if !cleared { psci_clear_domain_state(); cleared = true; }
        }
    }
}

unsafe fn psci_idle_syscore_suspend(data: *mut c_void) -> c_int { psci_idle_syscore_switch(true); 0 }
unsafe fn psci_idle_syscore_resume(data: *mut c_void) { psci_idle_syscore_switch(false); }

static psci_idle_syscore_ops: SyscoreOps = SyscoreOps { suspend: Some(psci_idle_syscore_suspend), resume: Some(psci_idle_syscore_resume) };
static mut psci_idle_syscore: Syscore = Syscore { ops: &psci_idle_syscore_ops };

unsafe fn psci_idle_init_syscore() { if psci_cpuidle_use_syscore { register_syscore(&mut psci_idle_syscore); } }

unsafe fn psci_idle_init_cpuhp() {
    let err = cpuhp_setup_state_nocalls(CPUHP_AP_CPU_PM_STARTING, "cpuidle/psci:online", Some(psci_idle_cpuhp_up), Some(psci_idle_cpuhp_down));
    if err != 0 { pr_warn!("Failed {} while setup cpuhp state\n", err); }
}

unsafe fn psci_enter_idle_state(dev: *mut CpuidleDevice, drv: *mut CpuidleDriver, idx: c_int) -> c_int {
    let state = this_cpu_read(&psci_cpuidle_data, |d| d.psci_states);
    CPU_PM_CPU_IDLE_ENTER_PARAM_RCU!(psci_cpu_suspend_enter, idx, *state.add(idx as usize))
}

static psci_idle_state_match: [OfDeviceId; 2] = [
    OfDeviceId { compatible: c"arm,idle-state", data: psci_enter_idle_state as usize },
    OfDeviceId::empty(),
];

pub unsafe fn psci_dt_parse_state_node(np: *mut DeviceNode, state: *mut u32) -> c_int {
    let err = of_property_read_u32(np, c"arm,psci-suspend-param", state);
    if err != 0 { pr_warn!("%pOF missing arm,psci-suspend-param property\n", np); return err; }
    if !psci_power_state_is_valid(*state) { pr_warn!("Invalid PSCI power state %#x\n", *state); return -EINVAL; }
    0
}

unsafe fn psci_dt_cpu_init_topology(drv: *mut CpuidleDriver, data: *mut PsciCpuidleData, state_count: c_uint, cpu: c_int) -> c_int {
    /* Currently limit the hierarchical topology to be used in OSI mode. */
    if !psci_has_osi_support() { return 0; }
    (*data).dev = dt_idle_attach_cpu(cpu, c"psci");
    if IS_ERR_OR_NULL((*data).dev) { return PTR_ERR_OR_ZERO((*data).dev); }
    psci_cpuidle_use_syscore = true;
    (*drv).states[state_count as usize - 1].enter_s2idle = Some(psci_enter_s2idle_domain_idle_state);
    if !IS_ENABLED(CONFIG_PREEMPT_RT) { (*drv).states[state_count as usize - 1].enter = Some(psci_enter_domain_idle_state); }
    0
}

unsafe fn psci_dt_cpu_init_idle(dev: *mut Device, drv: *mut CpuidleDriver, cpu_node: *mut DeviceNode, mut state_count: c_uint, cpu: c_int) -> c_int {
    let mut ret = 0;
    let data = per_cpu_ptr(&mut psci_cpuidle_data, cpu);
    state_count += 1;
    let psci_states = devm_kcalloc(dev, state_count as usize, core::mem::size_of::<u32>(), GFP_KERNEL) as *mut u32;
    if psci_states.is_null() { return -ENOMEM; }
    let mut i = 1;
    while i < state_count {
        let state_node = of_get_cpu_state_node(cpu_node, i - 1);
        if state_node.is_null() { break; }
        ret = psci_dt_parse_state_node(state_node, psci_states.add(i as usize));
        of_node_put(state_node);
        if ret != 0 { return ret; }
        pr_debug!("psci-power-state %#x index %d\n", *psci_states.add(i as usize), i);
        i += 1;
    }
    if i != state_count { return -ENODEV; }
    ret = psci_dt_cpu_init_topology(drv, data, state_count, cpu);
    if ret < 0 { return ret; }
    (*data).psci_states = psci_states;
    0
}

unsafe fn psci_cpu_init_idle(dev: *mut Device, drv: *mut CpuidleDriver, cpu: c_uint, state_count: c_uint) -> c_int {
    if psci_ops.cpu_suspend.is_none() { return -EOPNOTSUPP; }
    let cpu_node = of_cpu_device_node_get(cpu);
    if cpu_node.is_null() { return -ENODEV; }
    let ret = psci_dt_cpu_init_idle(dev, drv, cpu_node, state_count, cpu as c_int);
    of_node_put(cpu_node);
    ret
}

unsafe fn psci_cpu_deinit_idle(cpu: c_int) {
    let data = per_cpu_ptr(&mut psci_cpuidle_data, cpu);
    dt_idle_detach_cpu((*data).dev);
    psci_cpuidle_use_syscore = false;
}

unsafe fn psci_idle_init_cpu(dev: *mut Device, cpu: c_int) -> c_int {
    let cpu_node = of_cpu_device_node_get(cpu as c_uint);
    if cpu_node.is_null() { return -ENODEV; }
    let enable_method = of_get_property(cpu_node, c"enable-method", core::ptr::null_mut());
    let ret = if enable_method.is_null() || strcmp(enable_method, c"psci") != 0 { -ENODEV } else { 0 };
    of_node_put(cpu_node);
    if ret != 0 { return ret; }
    let drv = devm_kzalloc(dev, core::mem::size_of::<CpuidleDriver>(), GFP_KERNEL) as *mut CpuidleDriver;
    if drv.is_null() { return -ENOMEM; }
    (*drv).name = c"psci_idle"; (*drv).owner = THIS_MODULE; (*drv).cpumask = cpumask_of(cpu) as *mut Cpumask;
    (*drv).states[0].enter = Some(psci_enter_idle_state); (*drv).states[0].exit_latency = 1; (*drv).states[0].target_residency = 1; (*drv).states[0].power_usage = UINT_MAX;
    strscpy((*drv).states[0].name.as_mut_ptr(), c"WFI", (*drv).states[0].name.len());
    strscpy((*drv).states[0].desc.as_mut_ptr(), c"ARM WFI", (*drv).states[0].desc.len());
    let mut ret = dt_init_idle_driver(drv, &psci_idle_state_match, 1);
    if ret <= 0 { return if ret != 0 { ret } else { -ENODEV }; }
    ret = psci_cpu_init_idle(dev, drv, cpu as c_uint, ret as c_uint);
    if ret != 0 { pr_err!("CPU {} failed to PSCI idle\n", cpu); return ret; }
    ret = cpuidle_register(drv, core::ptr::null_mut());
    if ret != 0 { psci_cpu_deinit_idle(cpu); return ret; }
    cpuidle_cooling_register(drv); 0
}

unsafe fn psci_cpuidle_probe(fdev: *mut FauxDevice) -> c_int {
    let mut cpu: c_int = 0;
    let mut ret;
    for_each_present_cpu!(cpu) { ret = psci_idle_init_cpu(&mut (*fdev).dev, cpu); if ret != 0 { while cpu > 0 { cpu -= 1; let dev = per_cpu(cpuidle_devices, cpu); let drv = cpuidle_get_cpu_driver(dev); cpuidle_unregister(drv); psci_cpu_deinit_idle(cpu); } return ret; } }
    psci_idle_init_syscore(); psci_idle_init_cpuhp(); 0
}

static mut psci_cpuidle_ops: FauxDeviceOps = FauxDeviceOps { probe: Some(psci_cpuidle_probe) };

unsafe fn dt_idle_state_present() -> bool {
    let cpu_node = of_cpu_device_node_get(cpumask_first(cpu_possible_mask));
    if cpu_node.is_null() { return false; }
    let state_node = of_get_cpu_state_node(cpu_node, 0);
    if state_node.is_null() { return false; }
    !of_match_node(&psci_idle_state_match, state_node).is_null()
}

unsafe fn psci_idle_init() -> c_int {
    if !dt_idle_state_present() { return 0; }
    let fdev = faux_device_create(c"psci-cpuidle", core::ptr::null_mut(), &psci_cpuidle_ops);
    if fdev.is_null() { pr_err!("Failed to create psci-cpuidle device\n"); return -ENODEV; }
    0
}

device_initcall!(psci_idle_init);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
