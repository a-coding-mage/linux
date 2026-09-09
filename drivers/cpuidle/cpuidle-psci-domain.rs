// SPDX-License-Identifier: GPL-2.0
/*
 * PM domains for CPUs via genpd - managed by cpuidle-psci.
 *
 * Copyright (C) 2019 Linaro Ltd.
 * Author: Ulf Hansson <ulf.hansson@linaro.org>
 */

// C includes and build-provided declarations are supplied by the surrounding
// kernel translation unit.

#[repr(C)]
struct PsciPdProvider {
    link: ListHead,
    node: *mut DeviceNode,
}

static mut PSCI_PD_PROVIDERS: ListHead = ListHead::new();

unsafe fn psci_pd_power_off(pd: *mut GenericPmDomain) -> i32 {
    let state = &mut (*pd).states[(*pd).state_idx as usize];
    if state.data.is_null() {
        return 0;
    }

    // OSI mode is enabled, set the corresponding domain state.
    let pd_state = state.data as *mut u32;
    psci_set_domain_state(pd, (*pd).state_idx, *pd_state);
    0
}

unsafe fn psci_pd_init(np: *mut DeviceNode, use_osi: bool) -> i32 {
    let mut pd: *mut GenericPmDomain;
    let mut pd_provider: *mut PsciPdProvider;
    let pd_gov: *mut DevPowerGovernor;
    let mut ret: i32 = -ENOMEM;

    pd = dt_idle_pd_alloc(np, psci_dt_parse_state_node);
    if pd.is_null() {
        goto_out(ret);
    }

    pd_provider = kzalloc_obj::<PsciPdProvider>();
    if pd_provider.is_null() {
        dt_idle_pd_free(pd);
        goto_out(ret);
    }

    (*pd).flags |= GENPD_FLAG_IRQ_SAFE | GENPD_FLAG_CPU_DOMAIN;

    /*
     * Allow power off when OSI has been successfully enabled.
     * On a PREEMPT_RT based configuration the domain idle states are
     * supported, but only during system-wide suspend.
     */
    if use_osi {
        (*pd).power_off = Some(psci_pd_power_off);
        (*pd).flags |= GENPD_FLAG_ACTIVE_WAKEUP;
        // CONFIG_PREEMPT_RT is a build-time condition.
        #[cfg(CONFIG_PREEMPT_RT)]
        { (*pd).flags |= GENPD_FLAG_RPM_ALWAYS_ON; }
    } else {
        (*pd).flags |= GENPD_FLAG_ALWAYS_ON;
    }

    // Use governor for CPU PM domains if it has some states to manage.
    pd_gov = if !(*pd).states.is_null() { &raw mut pm_domain_cpu_gov } else { core::ptr::null_mut() };

    ret = pm_genpd_init(pd, pd_gov, false);
    if ret != 0 {
        kfree(pd_provider);
        dt_idle_pd_free(pd);
        goto_out(ret);
    }

    ret = of_genpd_add_provider_simple(np, pd);
    if ret != 0 {
        pm_genpd_remove(pd);
        kfree(pd_provider);
        dt_idle_pd_free(pd);
        goto_out(ret);
    }

    (*pd_provider).node = of_node_get(np);
    list_add(&mut (*pd_provider).link, &raw mut PSCI_PD_PROVIDERS);
    pr_debug!("init PM domain {}\n", (*pd).name);
    return 0;

    fn goto_out(ret: i32) -> ! {
        pr_err!("failed to init PM domain ret={} %pOF\n", ret, core::ptr::null::<DeviceNode>());
        panic!("unreachable error label")
    }
}

unsafe fn psci_pd_remove() {
    // list_for_each_entry_safe_reverse over psci_pd_providers.
    let mut pd_provider: *mut PsciPdProvider;
    let mut it: *mut PsciPdProvider;
    let mut genpd: *mut GenericPmDomain;
    while let Some((provider, next)) = list_safe_reverse_pop::<PsciPdProvider>(&raw mut PSCI_PD_PROVIDERS) {
        pd_provider = provider;
        it = next;
        of_genpd_del_provider((*pd_provider).node);
        genpd = of_genpd_remove_last((*pd_provider).node);
        if !IS_ERR(genpd) { kfree(genpd); }
        of_node_put((*pd_provider).node);
        list_del(&mut (*pd_provider).link);
        kfree(pd_provider);
        let _ = it;
    }
}

static PSCI_OF_MATCH: [OfDeviceId; 2] = [
    OfDeviceId { compatible: "arm,psci-1.0" },
    OfDeviceId::empty(),
];

unsafe fn psci_cpuidle_domain_probe(pdev: *mut PlatformDevice) -> i32 {
    let np = (*pdev).dev.of_node;
    let use_osi = psci_has_osi_support();
    let mut ret: i32 = 0;
    let mut pd_count: i32 = 0;
    if np.is_null() { return -ENODEV; }

    // Parse child nodes and initialize genpd/provider pairs with power-domain-cells.
    for node in for_each_child_of_node_scoped(np) {
        if !of_property_present(node, "#power-domain-cells") { continue; }
        ret = psci_pd_init(node, use_osi);
        if ret != 0 { goto_exit(ret); }
        pd_count += 1;
    }
    if pd_count == 0 { return 0; }
    ret = dt_idle_pd_init_topology(np);
    if ret != 0 { dt_idle_pd_remove_topology(np); psci_pd_remove(); return ret; }
    ret = psci_set_osi_mode(use_osi);
    if ret != 0 { dt_idle_pd_remove_topology(np); psci_pd_remove(); return ret; }
    pr_info!("Initialized CPU PM domain topology using {} mode\n", if use_osi { "OSI" } else { "PC" });
    return 0;

    fn goto_exit(ret: i32) -> ! {
        pr_err!("failed to create CPU PM domains ret={}\n", ret);
        panic!("unreachable error label")
    }
}

static mut PSCI_CPUIDLE_DOMAIN_DRIVER: PlatformDriver = PlatformDriver {
    probe: Some(psci_cpuidle_domain_probe),
    driver: Driver { name: "psci-cpuidle-domain", of_match_table: &PSCI_OF_MATCH },
};

unsafe fn psci_idle_init_domains() -> i32 {
    platform_driver_register(&raw mut PSCI_CPUIDLE_DOMAIN_DRIVER)
}

// core_initcall(psci_idle_init_domains);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
