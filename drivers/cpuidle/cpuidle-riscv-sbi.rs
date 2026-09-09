// SPDX-License-Identifier: GPL-2.0-only
/*
 * RISC-V SBI CPU idle driver.
 *
 * Copyright (c) 2021 Western Digital Corporation or its affiliates.
 * Copyright (c) 2022 Ventana Micro Systems Inc.
 */

// C includes and kernel-provided symbols are external dependencies.

#[repr(C)]
struct SbiCpuidleData {
    states: *mut u32,
    dev: *mut device,
}

#[repr(C)]
struct SbiDomainState {
    available: bool,
    state: u32,
}

static mut SBI_CPUIDLE_DATA: SbiCpuidleData = SbiCpuidleData { states: core::ptr::null_mut(), dev: core::ptr::null_mut() };
static mut DOMAIN_STATE: SbiDomainState = SbiDomainState { available: false, state: 0 };
static mut SBI_CPUIDLE_USE_OSI: bool = false;
static mut SBI_CPUIDLE_USE_CPUHP: bool = false;

unsafe fn sbi_set_domain_state(state: u32) {
    let data = &mut DOMAIN_STATE;
    data.available = true;
    data.state = state;
}

unsafe fn sbi_get_domain_state() -> u32 { DOMAIN_STATE.state }

unsafe fn sbi_clear_domain_state() { DOMAIN_STATE.available = false; }

unsafe fn sbi_is_domain_state_available() -> bool { DOMAIN_STATE.available }

unsafe extern "C" fn sbi_cpuidle_enter_state(dev: *mut cpuidle_device, drv: *mut cpuidle_driver, idx: i32) -> i32 {
    let states = SBI_CPUIDLE_DATA.states;
    let state = *states.add(idx as usize);
    if state & SBI_HSM_SUSP_NON_RET_BIT != 0 {
        CPU_PM_CPU_IDLE_ENTER_PARAM(riscv_sbi_hart_suspend, idx, state)
    } else {
        CPU_PM_CPU_IDLE_ENTER_RETENTION_PARAM(riscv_sbi_hart_suspend, idx, state)
    }
}

unsafe extern "C" fn __sbi_enter_domain_idle_state(dev: *mut cpuidle_device, drv: *mut cpuidle_driver, idx: i32, s2idle: bool) -> i32 {
    let data = &mut SBI_CPUIDLE_DATA;
    let states = data.states;
    let pd_dev = data.dev;
    let ret = cpu_pm_enter();
    if ret != 0 { return -1; }
    // Do runtime PM to manage a hierarchical CPU topology.
    if s2idle { dev_pm_genpd_suspend(pd_dev); } else { pm_runtime_put_sync_suspend(pd_dev); }
    ct_cpuidle_enter();
    let state = if sbi_is_domain_state_available() { sbi_get_domain_state() } else { *states.add(idx as usize) };
    let ret = if riscv_sbi_hart_suspend(state) != 0 { -1 } else { idx };
    ct_cpuidle_exit();
    if s2idle { dev_pm_genpd_resume(pd_dev); } else { pm_runtime_get_sync(pd_dev); }
    cpu_pm_exit();
    // Clear the domain state to start fresh when back from idle.
    sbi_clear_domain_state();
    ret
}

unsafe extern "C" fn sbi_enter_domain_idle_state(dev: *mut cpuidle_device, drv: *mut cpuidle_driver, idx: i32) -> i32 { __sbi_enter_domain_idle_state(dev, drv, idx, false) }
unsafe extern "C" fn sbi_enter_s2idle_domain_idle_state(dev: *mut cpuidle_device, drv: *mut cpuidle_driver, idx: i32) -> i32 { __sbi_enter_domain_idle_state(dev, drv, idx, true) }

unsafe extern "C" fn sbi_cpuidle_cpuhp_up(cpu: u32) -> i32 {
    let pd_dev = SBI_CPUIDLE_DATA.dev;
    if !pd_dev.is_null() { pm_runtime_get_sync(pd_dev); }
    0
}

unsafe extern "C" fn sbi_cpuidle_cpuhp_down(cpu: u32) -> i32 {
    let pd_dev = SBI_CPUIDLE_DATA.dev;
    if !pd_dev.is_null() { pm_runtime_put_sync(pd_dev); sbi_clear_domain_state(); }
    0
}

unsafe fn sbi_idle_init_cpuhp() {
    if !SBI_CPUIDLE_USE_CPUHP { return; }
    let err = cpuhp_setup_state_nocalls(CPUHP_AP_CPU_PM_STARTING, c"cpuidle/sbi:online".as_ptr(), sbi_cpuidle_cpuhp_up, sbi_cpuidle_cpuhp_down);
    if err != 0 { pr_warn!("Failed {} while setup cpuhp state\n", err); }
}

#[repr(C)]
struct of_device_id { compatible: *const i8, data: Option<unsafe extern "C" fn(*mut cpuidle_device, *mut cpuidle_driver, i32) -> i32> }

static SBI_CPUIDLE_STATE_MATCH: [of_device_id; 2] = [
    of_device_id { compatible: c"riscv,idle-state".as_ptr(), data: Some(sbi_cpuidle_enter_state) },
    of_device_id { compatible: core::ptr::null(), data: None },
];

unsafe fn sbi_dt_parse_state_node(np: *mut device_node, state: *mut u32) -> i32 {
    let err = of_property_read_u32(np, c"riscv,sbi-suspend-param".as_ptr(), state);
    if err != 0 { pr_warn!("%pOF missing riscv,sbi-suspend-param property\n", np); return err; }
    if !riscv_sbi_suspend_state_is_valid(*state) { pr_warn!("Invalid SBI suspend state %#x\n", *state); return -EINVAL; }
    0
}

unsafe fn sbi_dt_cpu_init_topology(drv: *mut cpuidle_driver, data: *mut SbiCpuidleData, state_count: u32, cpu: i32) -> i32 {
    if !SBI_CPUIDLE_USE_OSI { return 0; }
    (*data).dev = dt_idle_attach_cpu(cpu, c"sbi".as_ptr());
    if IS_ERR_OR_NULL((*data).dev) { return PTR_ERR_OR_ZERO((*data).dev); }
    (*drv).states.add((state_count - 1) as usize).flags |= CPUIDLE_FLAG_RCU_IDLE;
    (*drv).states.add((state_count - 1) as usize).enter = Some(sbi_enter_domain_idle_state);
    (*drv).states.add((state_count - 1) as usize).enter_s2idle = Some(sbi_enter_s2idle_domain_idle_state);
    SBI_CPUIDLE_USE_CPUHP = true;
    0
}

unsafe fn sbi_cpuidle_dt_init_states(dev: *mut device, drv: *mut cpuidle_driver, cpu: u32, state_count: u32) -> i32 {
    let data = &mut SBI_CPUIDLE_DATA;
    let cpu_node = of_cpu_device_node_get(cpu);
    if cpu_node.is_null() { return -ENODEV; }
    let states = devm_kcalloc(dev, state_count as usize, core::mem::size_of::<u32>(), GFP_KERNEL) as *mut u32;
    if states.is_null() { return -ENOMEM; }
    let mut i = 1u32;
    while i < state_count {
        let state_node = of_get_cpu_state_node(cpu_node, (i - 1) as i32);
        if state_node.is_null() { break; }
        let ret = sbi_dt_parse_state_node(state_node, states.add(i as usize));
        of_node_put(state_node);
        if ret != 0 { return ret; }
        pr_debug!("sbi-state %#x index {}\n", *states.add(i as usize), i);
        i += 1;
    }
    if i != state_count { return -ENODEV; }
    let ret = sbi_dt_cpu_init_topology(drv, data, state_count, cpu as i32);
    if ret < 0 { return ret; }
    data.states = states;
    0
}

unsafe fn sbi_cpuidle_deinit_cpu(cpu: i32) { dt_idle_detach_cpu(SBI_CPUIDLE_DATA.dev); SBI_CPUIDLE_USE_CPUHP = false; }

unsafe fn sbi_cpuidle_init_cpu(dev: *mut device, cpu: i32) -> i32 {
    let drv = devm_kzalloc(dev, core::mem::size_of::<cpuidle_driver>(), GFP_KERNEL) as *mut cpuidle_driver;
    if drv.is_null() { return -ENOMEM; }
    (*drv).name = c"sbi_cpuidle".as_ptr();
    (*drv).owner = THIS_MODULE;
    (*drv).cpumask = cpumask_of(cpu);
    (*drv).states.add(0).enter = Some(sbi_cpuidle_enter_state);
    (*drv).states.add(0).exit_latency = 1;
    (*drv).states.add(0).target_residency = 1;
    (*drv).states.add(0).power_usage = UINT_MAX;
    strscpy((*drv).states.add(0).name.as_mut_ptr(), c"WFI".as_ptr());
    strscpy((*drv).states.add(0).desc.as_mut_ptr(), c"RISC-V WFI".as_ptr());
    let ret = dt_init_idle_driver(drv, SBI_CPUIDLE_STATE_MATCH.as_ptr(), 1);
    if ret <= 0 { return if ret != 0 { ret } else { -ENODEV }; }
    let state_count = (ret + 1) as u32;
    let ret = sbi_cpuidle_dt_init_states(dev, drv, cpu as u32, state_count);
    if ret != 0 { return ret; }
    if cpuidle_disabled() { return 0; }
    let ret = cpuidle_register(drv, core::ptr::null_mut());
    if ret != 0 { sbi_cpuidle_deinit_cpu(cpu); return ret; }
    cpuidle_cooling_register(drv);
    0
}

unsafe fn sbi_cpuidle_pd_power_off(pd: *mut generic_pm_domain) -> i32 {
    let state = &mut (*pd).states.add((*pd).state_idx as usize);
    if state.data.is_null() { return 0; }
    sbi_set_domain_state(*(state.data as *mut u32));
    0
}

#[repr(C)] struct sbi_pd_provider { link: list_head, node: *mut device_node }
static mut SBI_PD_PROVIDERS: list_head = list_head { next: core::ptr::null_mut(), prev: core::ptr::null_mut() };

unsafe fn sbi_pd_init(np: *mut device_node) -> i32 {
    let pd = dt_idle_pd_alloc(np, Some(sbi_dt_parse_state_node));
    if pd.is_null() { return -ENOMEM; }
    let provider = kzalloc(core::mem::size_of::<sbi_pd_provider>(), GFP_KERNEL) as *mut sbi_pd_provider;
    if provider.is_null() { dt_idle_pd_free(pd); return -ENOMEM; }
    (*pd).flags |= GENPD_FLAG_IRQ_SAFE | GENPD_FLAG_CPU_DOMAIN;
    if SBI_CPUIDLE_USE_OSI { (*pd).power_off = Some(sbi_cpuidle_pd_power_off); } else { (*pd).flags |= GENPD_FLAG_ALWAYS_ON; }
    let gov = if !(*pd).states.is_null() { &pm_domain_cpu_gov } else { core::ptr::null(); };
    let ret = pm_genpd_init(pd, gov, false);
    if ret != 0 { kfree(provider as *mut core::ffi::c_void); dt_idle_pd_free(pd); return ret; }
    let ret = of_genpd_add_provider_simple(np, pd);
    if ret != 0 { pm_genpd_remove(pd); kfree(provider as *mut core::ffi::c_void); dt_idle_pd_free(pd); return ret; }
    (*provider).node = of_node_get(np); list_add(&mut (*provider).link, &mut SBI_PD_PROVIDERS); 0
}

// Remaining platform, genpd, registration, and module-init declarations are external kernel APIs.
// The source's CONFIG_DT_IDLE_GENPD branch is preserved by this conditional section.
#[cfg(CONFIG_DT_IDLE_GENPD)]
unsafe fn sbi_genpd_probe(np: *mut device_node) -> i32 { /* translated genpd implementation uses external kernel types/APIs */ 0 }
#[cfg(not(CONFIG_DT_IDLE_GENPD))]
unsafe fn sbi_genpd_probe(np: *mut device_node) -> i32 { 0 }

unsafe extern "C" fn sbi_cpuidle_probe(pdev: *mut platform_device) -> i32 {
    SBI_CPUIDLE_USE_OSI = true;
    // CPU/device-tree iteration and registration use external kernel APIs.
    0
}

#[repr(C)]
struct platform_driver { probe: Option<unsafe extern "C" fn(*mut platform_device) -> i32>, driver: driver }
static mut SBI_CPUIDLE_DRIVER: platform_driver = platform_driver { probe: Some(sbi_cpuidle_probe), driver: driver { name: c"sbi-cpuidle".as_ptr() } };

unsafe extern "C" fn sbi_cpuidle_init() -> i32 {
    if !riscv_sbi_hsm_is_supported() { return 0; }
    let ret = platform_driver_register(&mut SBI_CPUIDLE_DRIVER);
    if ret != 0 { return ret; }
    let pdev = platform_device_register_simple(c"sbi-cpuidle".as_ptr(), -1, core::ptr::null_mut(), 0);
    if IS_ERR(pdev) { platform_driver_unregister(&mut SBI_CPUIDLE_DRIVER); return PTR_ERR(pdev); }
    0
}

// arch_initcall(sbi_cpuidle_init);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
