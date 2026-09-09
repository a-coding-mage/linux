// SPDX-License-Identifier: GPL-2.0-only
/*
 *
 * Copyright (C) 2016 ARM Limited
 */

// C dependencies are supplied by the surrounding kernel translation unit.

const NUM_SUSPEND_CYCLE: i32 = 10;

static mut NB_AVAILABLE_CPUS: u32 = 0;
static mut TOS_RESIDENT_CPU: i32 = -1;

static mut NB_ACTIVE_THREADS: atomic_t = atomic_t { counter: 0 };
static mut SUSPEND_THREADS_STARTED: completion = COMPLETION_INITIALIZER!();
static mut SUSPEND_THREADS_DONE: completion = COMPLETION_INITIALIZER!();

unsafe fn psci_ops_check() -> i32 {
    let mut migrate_type: i32 = -1;
    let mut cpu: i32;

    if !((*psci_ops).cpu_off.is_some()
        && (*psci_ops).cpu_on.is_some()
        && (*psci_ops).cpu_suspend.is_some())
    {
        pr_warn!("Missing PSCI operations, aborting tests\n");
        return -EOPNOTSUPP;
    }

    if let Some(migrate_info_type) = (*psci_ops).migrate_info_type {
        migrate_type = migrate_info_type();
    }

    if migrate_type == PSCI_0_2_TOS_UP_MIGRATE
        || migrate_type == PSCI_0_2_TOS_UP_NO_MIGRATE
    {
        /* There is a UP Trusted OS, find on which core it resides. */
        for_each_online_cpu!(cpu) {
            if psci_tos_resident_on(cpu) {
                TOS_RESIDENT_CPU = cpu;
                break;
            }
        }
        if TOS_RESIDENT_CPU == -1 {
            pr_warn!("UP Trusted OS resides on no online CPU\n");
        }
    }

    0
}

unsafe fn down_and_up_cpus(cpus: *const cpumask, offlined_cpus: *mut cpumask) -> u32 {
    let mut cpu: i32;
    let mut err: i32 = 0;

    cpumask_clear(offlined_cpus);

    for_each_cpu!(cpu, cpus) {
        let ret = remove_cpu(cpu);
        if cpumask_weight(offlined_cpus) + 1 == NB_AVAILABLE_CPUS {
            if ret != -EBUSY {
                pr_err!("Unexpected return code {} while trying to power down last online CPU {}\n", ret, cpu);
                err += 1;
            }
        } else if cpu == TOS_RESIDENT_CPU {
            if ret != -EPERM {
                pr_err!("Unexpected return code {} while trying to power down TOS resident CPU {}\n", ret, cpu);
                err += 1;
            }
        } else if ret != 0 {
            pr_err!("Error occurred ({}) while trying to power down CPU {}\n", ret, cpu);
            err += 1;
        }
        if ret == 0 { cpumask_set_cpu(cpu, offlined_cpus); }
    }

    for_each_cpu!(cpu, offlined_cpus) {
        let ret = add_cpu(cpu);
        if ret != 0 {
            pr_err!("Error occurred ({}) while trying to power up CPU {}\n", ret, cpu);
            err += 1;
        } else {
            cpumask_clear_cpu(cpu, offlined_cpus);
        }
    }

    WARN_ON!( !cpumask_empty(offlined_cpus) || num_online_cpus() != NB_AVAILABLE_CPUS );
    err as u32
}

unsafe fn free_cpu_groups(num: i32, pcpu_groups: *mut cpumask_var_t) {
    let cpu_groups = *pcpu_groups;
    for i in 0..num { free_cpumask_var(cpu_groups[i as usize]); }
    kfree(cpu_groups);
}

unsafe fn alloc_init_cpu_groups(pcpu_groups: *mut *mut cpumask_var_t) -> i32 {
    let mut num_groups = 0;
    let mut tmp: cpumask_var_t = core::ptr::null_mut();
    if !alloc_cpumask_var(&mut tmp, GFP_KERNEL) { return -ENOMEM; }
    let cpu_groups = kzalloc_objs!(cpumask_var_t, NB_AVAILABLE_CPUS);
    if cpu_groups.is_null() { free_cpumask_var(tmp); return -ENOMEM; }
    cpumask_copy(tmp, cpu_online_mask);
    while !cpumask_empty(tmp) {
        let cpu_group = topology_core_cpumask(cpumask_any(tmp));
        if !alloc_cpumask_var(&mut (*cpu_groups.add(num_groups as usize)), GFP_KERNEL) {
            free_cpumask_var(tmp);
            free_cpu_groups(num_groups, &mut *(cpu_groups as *mut cpumask_var_t));
            return -ENOMEM;
        }
        cpumask_copy(*cpu_groups.add(num_groups as usize), cpu_group);
        num_groups += 1;
        cpumask_andnot(tmp, tmp, cpu_group);
    }
    free_cpumask_var(tmp);
    *pcpu_groups = cpu_groups;
    num_groups
}

unsafe fn hotplug_tests() -> i32 {
    let mut offlined_cpus: cpumask_var_t = core::ptr::null_mut();
    if !alloc_cpumask_var(&mut offlined_cpus, GFP_KERNEL) { return -ENOMEM; }
    let nb_cpu_group = alloc_init_cpu_groups(&mut (core::ptr::null_mut()));
    if nb_cpu_group < 0 { free_cpumask_var(offlined_cpus); return -ENOMEM; }
    let mut err = down_and_up_cpus(cpu_online_mask, offlined_cpus) as i32;
    let mut cpu_groups: *mut cpumask_var_t = core::ptr::null_mut();
    let _ = alloc_init_cpu_groups(&mut cpu_groups);
    for i in 0..nb_cpu_group {
        pr_info!("Trying to turn off and on again group {} (CPUs %*pbl)\n", i, cpumask_pr_args!(*cpu_groups.add(i as usize)));
        err += down_and_up_cpus(*cpu_groups.add(i as usize), offlined_cpus) as i32;
    }
    free_cpu_groups(nb_cpu_group, &mut cpu_groups);
    free_cpumask_var(offlined_cpus);
    err
}

unsafe fn dummy_callback(_unused: *mut timer_list) {}

unsafe fn suspend_cpu(dev: *mut cpuidle_device, drv: *mut cpuidle_driver, index: i32) -> i32 {
    let state = &mut (*drv).states[index as usize];
    let broadcast = state.flags & CPUIDLE_FLAG_TIMER_STOP != 0;
    arch_cpu_idle_enter();
    if broadcast {
        let ret = tick_broadcast_enter();
        if ret != 0 { cpu_do_idle(); arch_cpu_idle_exit(); return 0; }
    }
    let ret = (state.enter.unwrap())(dev, drv, index);
    if broadcast { tick_broadcast_exit(); }
    arch_cpu_idle_exit();
    ret
}

// The remaining kernel-thread implementation retains the C control flow and
// uses the surrounding kernel bindings for all referenced types and helpers.
unsafe fn suspend_test_thread(arg: *mut core::ffi::c_void) -> i32 {
    let cpu = arg as isize as i32;
    let mut nb_suspend = 0;
    let mut nb_shallow_sleep = 0;
    let mut nb_err = 0;
    wait_for_completion(&mut SUSPEND_THREADS_STARTED);
    sched_set_fifo(current);
    let dev = this_cpu_read!(cpuidle_devices);
    let drv = cpuidle_get_cpu_driver(dev);
    pr_info!("CPU {} entering suspend cycles, states 1 through {}\n", cpu, (*drv).state_count - 1);
    let mut wakeup_timer: timer_list = core::mem::zeroed();
    timer_setup_on_stack!(&mut wakeup_timer, dummy_callback, 0);
    for i in 0..NUM_SUSPEND_CYCLE {
        for index in 1..(*drv).state_count {
            let state = &(*drv).states[index as usize];
            mod_timer!(&mut wakeup_timer, jiffies + usecs_to_jiffies(state.target_residency));
            local_irq_disable();
            let ret = suspend_cpu(dev, drv, index);
            local_irq_enable();
            if ret == index { nb_suspend += 1; } else if ret >= 0 { nb_shallow_sleep += 1; } else { pr_err!("Failed to suspend CPU {}: error {} (requested state {}, cycle {})\n", cpu, ret, index, i); nb_err += 1; }
        }
    }
    timer_delete(&mut wakeup_timer); timer_destroy_on_stack(&mut wakeup_timer);
    if atomic_dec_return_relaxed(&mut NB_ACTIVE_THREADS) == 0 { complete(&mut SUSPEND_THREADS_DONE); }
    loop { set_current_state!(TASK_INTERRUPTIBLE); if kthread_should_park() { break; } schedule(); }
    pr_info!("CPU {} suspend test results: success {}, shallow states {}, errors {}\n", cpu, nb_suspend, nb_shallow_sleep, nb_err);
    kthread_parkme();
    nb_err
}

unsafe fn suspend_tests() -> i32 {
    // Direct translation of the kernel thread orchestration; bindings supply
    // allocation, cpuidle, completion, and kthread primitives.
    let threads = kmalloc_objs!(task_struct *, NB_AVAILABLE_CPUS);
    if threads.is_null() { return -ENOMEM; }
    cpuidle_pause_and_lock();
    let mut nb_threads = 0;
    for_each_online_cpu!(cpu) {
        let dev = per_cpu!(cpuidle_devices, cpu);
        let drv = cpuidle_get_cpu_driver(dev);
        if dev.is_null() || drv.is_null() { pr_warn!("cpuidle not available on CPU {}\n", cpu); continue; }
        let thread = kthread_create_on_cpu(suspend_test_thread, cpu as isize as *mut _, cpu, c"psci_suspend_test".as_ptr());
        if IS_ERR(thread) { pr_err!("Failed to create kthread on CPU {}\n", cpu); } else { *threads.add(nb_threads as usize) = thread; nb_threads += 1; }
    }
    if nb_threads < 1 { cpuidle_resume_and_unlock(); kfree(threads); return -ENODEV; }
    atomic_set(&mut NB_ACTIVE_THREADS, nb_threads);
    for i in 0..nb_threads { wake_up_process(*threads.add(i as usize)); }
    complete_all(&mut SUSPEND_THREADS_STARTED); wait_for_completion(&mut SUSPEND_THREADS_DONE);
    let mut err = 0; for i in 0..nb_threads { err += kthread_park(*threads.add(i as usize)); err += kthread_stop(*threads.add(i as usize)); }
    cpuidle_resume_and_unlock(); kfree(threads); err
}

unsafe fn psci_checker() -> i32 {
    NB_AVAILABLE_CPUS = num_online_cpus();
    let mut ret = psci_ops_check(); if ret != 0 { return ret; }
    pr_info!("PSCI checker started using {} CPUs\n", NB_AVAILABLE_CPUS);
    pr_info!("Starting hotplug tests\n"); ret = hotplug_tests();
    if ret == 0 { pr_info!("Hotplug tests passed OK\n"); } else if ret > 0 { pr_err!("{} error(s) encountered in hotplug tests\n", ret); } else { pr_err!("Out of memory\n"); return ret; }
    pr_info!("Starting suspend tests ({} cycles per state)\n", NUM_SUSPEND_CYCLE);
    ret = suspend_tests();
    if ret == 0 { pr_info!("Suspend tests passed OK\n"); } else if ret > 0 { pr_err!("{} error(s) encountered in suspend tests\n", ret); } else { match ret { -ENOMEM => pr_err!("Out of memory\n"), -ENODEV => pr_warn!("Could not start suspend tests on any CPU\n"), _ => {} } }
    pr_info!("PSCI checker completed\n"); if ret < 0 { ret } else { 0 }
}

late_initcall!(psci_checker);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
