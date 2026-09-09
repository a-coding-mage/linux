// SPDX-License-Identifier: GPL-2.0-only
/*
 * PM domains for CPUs via genpd.
 *
 * Copyright (C) 2019 Linaro Ltd.
 * Author: Ulf Hansson <ulf.hansson@linaro.org>
 *
 * Copyright (c) 2021 Western Digital Corporation or its affiliates.
 * Copyright (c) 2022 Ventana Micro Systems Inc.
 */

// pr_fmt(fmt) = "dt-idle-genpd: " fmt

unsafe fn pd_parse_state_nodes(
    parse_state: unsafe extern "C" fn(*mut device_node, *mut u32) -> c_int,
    states: *mut genpd_power_state,
    state_count: c_int,
) -> c_int {
    let mut i: c_int = 0;
    let mut ret: c_int;
    let mut state: u32 = 0;
    let mut state_buf: *mut u32;

    while i < state_count {
        ret = parse_state(to_of_node((*states.add(i as usize)).fwnode), &mut state);
        if ret != 0 {
            break;
        }

        state_buf = kmalloc(core::mem::size_of::<u32>(), GFP_KERNEL);
        if state_buf.is_null() {
            ret = -ENOMEM;
            break;
        }
        *state_buf = state;
        (*states.add(i as usize)).data = state_buf as *mut c_void;
        i += 1;
    }

    if i == state_count {
        return 0;
    }

    i -= 1;
    while i >= 0 {
        kfree((*states.add(i as usize)).data);
        i -= 1;
    }
    ret
}

unsafe fn pd_parse_states(
    np: *mut device_node,
    parse_state: unsafe extern "C" fn(*mut device_node, *mut u32) -> c_int,
    states: *mut *mut genpd_power_state,
    state_count: *mut c_int,
) -> c_int {
    // Parse the domain idle states.
    let mut ret = of_genpd_parse_idle_states(np, states, state_count);
    if ret != 0 {
        return ret;
    }

    // Fill out the dt specifics for each found state.
    ret = pd_parse_state_nodes(parse_state, *states, *state_count);
    if ret != 0 {
        kfree(*states as *mut c_void);
    }

    ret
}

unsafe fn pd_free_states(states: *mut genpd_power_state, state_count: c_uint) {
    let mut i: c_uint = 0;
    while i < state_count {
        kfree((*states.add(i as usize)).data);
        i += 1;
    }
    kfree(states as *mut c_void);
}

pub unsafe extern "C" fn dt_idle_pd_free(pd: *mut generic_pm_domain) {
    pd_free_states((*pd).states, (*pd).state_count);
    kfree((*pd).name as *mut c_void);
    kfree(pd as *mut c_void);
}

pub unsafe extern "C" fn dt_idle_pd_alloc(
    np: *mut device_node,
    parse_state: unsafe extern "C" fn(*mut device_node, *mut u32) -> c_int,
) -> *mut generic_pm_domain {
    let pd = kzalloc_obj::<generic_pm_domain>();
    if pd.is_null() {
        pr_err!("failed to alloc PM domain %pOF\n", np);
        return core::ptr::null_mut();
    }

    (*pd).name = kasprintf(GFP_KERNEL, "%pOF", np);
    if (*pd).name.is_null() {
        kfree(pd as *mut c_void);
        pr_err!("failed to alloc PM domain %pOF\n", np);
        return core::ptr::null_mut();
    }

    /*
     * Parse the domain idle states and let genpd manage the state selection
     * for those being compatible with "domain-idle-state".
     */
    let mut states: *mut genpd_power_state = core::ptr::null_mut();
    let mut state_count: c_int = 0;
    let ret = pd_parse_states(np, parse_state, &mut states, &mut state_count);
    if ret != 0 {
        kfree((*pd).name as *mut c_void);
        kfree(pd as *mut c_void);
        pr_err!("failed to alloc PM domain %pOF\n", np);
        return core::ptr::null_mut();
    }

    (*pd).free_states = Some(pd_free_states);
    (*pd).name = kbasename((*pd).name);
    (*pd).states = states;
    (*pd).state_count = state_count;

    pr_debug!("alloc PM domain %s\n", (*pd).name);
    pd
}

pub unsafe extern "C" fn dt_idle_pd_init_topology(np: *mut device_node) -> c_int {
    let mut child: of_phandle_args;
    let mut parent: of_phandle_args;

    // for_each_child_of_node_scoped(np, node)
    let mut node = of_get_next_child(np, core::ptr::null_mut());
    while !node.is_null() {
        if of_parse_phandle_with_args(
            node,
            c"power-domains".as_ptr(),
            c"#power-domain-cells".as_ptr(),
            0,
            &mut parent,
        ) != 0
        {
            node = of_get_next_child(np, node);
            continue;
        }

        child.np = node;
        child.args_count = 0;
        let ret = of_genpd_add_subdomain(&mut parent, &mut child);
        of_node_put(parent.np);
        if ret != 0 {
            return ret;
        }
        node = of_get_next_child(np, node);
    }
    0
}

pub unsafe extern "C" fn dt_idle_pd_remove_topology(np: *mut device_node) -> c_int {
    let mut child: of_phandle_args;
    let mut parent: of_phandle_args;

    // for_each_child_of_node_scoped(np, node)
    let mut node = of_get_next_child(np, core::ptr::null_mut());
    while !node.is_null() {
        if of_parse_phandle_with_args(
            node,
            c"power-domains".as_ptr(),
            c"#power-domain-cells".as_ptr(),
            0,
            &mut parent,
        ) != 0
        {
            node = of_get_next_child(np, node);
            continue;
        }

        child.np = node;
        child.args_count = 0;
        let ret = of_genpd_remove_subdomain(&mut parent, &mut child);
        of_node_put(parent.np);
        if ret != 0 {
            return ret;
        }
        node = of_get_next_child(np, node);
    }
    0
}

pub unsafe extern "C" fn dt_idle_attach_cpu(cpu: c_int, name: *const c_char) -> *mut device {
    let dev = dev_pm_domain_attach_by_name(get_cpu_device(cpu), name);
    if is_err_or_null(dev) {
        return dev;
    }

    pm_runtime_irq_safe(dev);
    if cpu_online(cpu) {
        pm_runtime_get_sync(dev);
    }

    dev_pm_syscore_device(dev, true);
    dev
}

pub unsafe extern "C" fn dt_idle_detach_cpu(dev: *mut device) {
    if is_err_or_null(dev) {
        return;
    }

    dev_pm_domain_detach(dev, false);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
