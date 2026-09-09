// SPDX-License-Identifier: GPL-2.0-only
/*
 * DT idle states parsing code.
 *
 * Copyright (C) 2014 ARM Ltd.
 * Author: Lorenzo Pieralisi <lorenzo.pieralisi@arm.com>
 */

// #define pr_fmt(fmt) "DT idle-states: " fmt
// Dependencies are supplied by the surrounding kernel translation unit.

unsafe fn init_state_node(
    idle_state: *mut cpuidle_state,
    match_id: *const of_device_id,
    state_node: *mut device_node,
) -> i32 {
    let mut err: i32;
    let mut desc: *const c_char;

    /*
     * CPUidle drivers are expected to initialize the const void *data
     * pointer of the passed in struct of_device_id array to the idle
     * state enter function.
     */
    (*idle_state).enter = (*match_id).data;
    /*
     * Since this is not a "coupled" state, it's safe to assume interrupts
     * won't be enabled when it exits allowing the tick to be frozen
     * safely. So enter() can be also enter_s2idle() callback.
     */
    (*idle_state).enter_s2idle = (*match_id).data;

    err = of_property_read_u32(
        state_node,
        c"wakeup-latency-us".as_ptr() as *const c_char,
        &mut (*idle_state).exit_latency,
    );
    if err != 0 {
        let mut entry_latency: u32 = 0;
        let mut exit_latency: u32 = 0;

        err = of_property_read_u32(
            state_node,
            c"entry-latency-us".as_ptr() as *const c_char,
            &mut entry_latency,
        );
        if err != 0 {
            pr_debug!(" * %pOF missing entry-latency-us property\n", state_node);
            return -EINVAL;
        }

        err = of_property_read_u32(
            state_node,
            c"exit-latency-us".as_ptr() as *const c_char,
            &mut exit_latency,
        );
        if err != 0 {
            pr_debug!(" * %pOF missing exit-latency-us property\n", state_node);
            return -EINVAL;
        }
        /* If wakeup-latency-us is missing, default to entry+exit latencies. */
        (*idle_state).exit_latency = entry_latency.wrapping_add(exit_latency);
    }

    err = of_property_read_u32(
        state_node,
        c"min-residency-us".as_ptr() as *const c_char,
        &mut (*idle_state).target_residency,
    );
    if err != 0 {
        pr_debug!(" * %pOF missing min-residency-us property\n", state_node);
        return -EINVAL;
    }

    err = of_property_read_string(
        state_node,
        c"idle-state-name".as_ptr() as *const c_char,
        &mut desc,
    );
    if err != 0 {
        desc = (*state_node).name;
    }

    (*idle_state).flags = CPUIDLE_FLAG_RCU_IDLE;
    if of_property_read_bool(state_node, c"local-timer-stop".as_ptr() as *const c_char) {
        (*idle_state).flags |= CPUIDLE_FLAG_TIMER_STOP;
    }
    /* TODO: replace with kstrdup and pointer assignment when name and desc become string pointers. */
    strscpy((*idle_state).name.as_mut_ptr(), (*state_node).name, CPUIDLE_NAME_LEN);
    strscpy((*idle_state).desc.as_mut_ptr(), desc, CPUIDLE_DESC_LEN);
    0
}

/* Check that the idle state is uniform across all CPUs in the driver cpumask. */
unsafe fn idle_state_valid(
    state_node: *mut device_node,
    idx: c_uint,
    cpumask: *const cpumask_t,
) -> bool {
    let mut cpu = cpumask_first(cpumask).wrapping_add(1);
    // Equivalent of for_each_cpu_from(cpu, cpumask).
    while let Some(next_cpu) = cpumask_next(cpu.wrapping_sub(1), cpumask) {
        cpu = next_cpu;
        let cpu_node = of_cpu_device_node_get(cpu);
        let curr_state_node = of_get_cpu_state_node(cpu_node, idx);
        of_node_put(curr_state_node);
        of_node_put(cpu_node);
        if state_node != curr_state_node {
            return false;
        }
    }
    true
}

pub unsafe fn dt_init_idle_driver(
    drv: *mut cpuidle_driver,
    matches: *const of_device_id,
    start_idx: c_uint,
) -> i32 {
    let mut state_node: *mut device_node = core::ptr::null_mut();
    let mut err: i32 = 0;
    let mut state_idx = start_idx;

    if state_idx >= CPUIDLE_STATE_MAX {
        return -EINVAL;
    }

    let cpumask = if !(*drv).cpumask.is_null() {
        (*drv).cpumask
    } else {
        cpu_possible_mask
    };
    let cpu_node = of_cpu_device_node_get(cpumask_first(cpumask));

    let mut i: c_int = 0;
    loop {
        state_node = of_get_cpu_state_node(cpu_node, i as c_uint);
        if state_node.is_null() {
            break;
        }

        let match_id = of_match_node(matches, state_node);
        if match_id.is_null() {
            err = -ENODEV;
            break;
        }

        if !of_device_is_available(state_node) {
            of_node_put(state_node);
            i += 1;
            continue;
        }

        if !idle_state_valid(state_node, i as c_uint, cpumask) {
            pr_warn!("%pOF idle state not valid, bailing out\n", state_node);
            err = -EINVAL;
            break;
        }

        if state_idx == CPUIDLE_STATE_MAX {
            pr_warn!("State index reached static CPU idle driver states array size\n");
            break;
        }

        let idle_state = &mut (*drv).states[state_idx as usize] as *mut cpuidle_state;
        state_idx += 1;
        err = init_state_node(idle_state, match_id, state_node);
        if err != 0 {
            pr_err!("Parsing idle state node %pOF failed with err %d\n", state_node, err);
            err = -EINVAL;
            break;
        }
        of_node_put(state_node);
        i += 1;
    }

    of_node_put(state_node);
    of_node_put(cpu_node);
    if err != 0 {
        return err;
    }

    /* Set the number of total supported idle states. */
    (*drv).state_count = state_idx;
    state_idx.wrapping_sub(start_idx) as i32
}

// EXPORT_SYMBOL_GPL(dt_init_idle_driver);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
