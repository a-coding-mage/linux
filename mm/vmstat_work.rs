// SPDX-License-Identifier: GPL-2.0-only
#[cfg_attr(RUST_VMSTAT_READ_MOSTLY, link_section = ".data..read_mostly")]
static mut SYSCTL_STAT_INTERVAL: c_int = RUST_VMSTAT_HZ as c_int;
static mut VMSTAT_LATE_INIT_DONE: c_int = 0;
#[cfg(CONFIG_PROC_FS)]
unsafe extern "C" fn refresh_vm_stats(_work: *mut work_struct) {
    // SAFETY: this body preserves the native caller's pointer, locking and CPU-context contract.
    unsafe {
        refresh_cpu_vm_stats(true);
    }
}
#[cfg(CONFIG_PROC_FS)]
unsafe extern "C" fn vmstat_refresh(
    _table: *const ctl_table,
    write: c_int,
    _buffer: *mut c_void,
    lenp: *mut usize,
    ppos: *mut loff_t,
) -> c_int {
    // SAFETY: this body preserves the native caller's pointer, locking and CPU-context contract.
    unsafe {
        let err = schedule_on_each_cpu(Some(refresh_vm_stats));
        if err != 0 {
            return err;
        }
        for i in 0..NR_VM_ZONE_STAT_ITEMS as usize {
            if i == NR_ZONE_WRITE_PENDING as usize || i == NR_FREE_CMA_PAGES as usize {
                continue;
            }
            let value = rust_vmstat_atomic_read(addr_of!(vm_zone_stat[i]));
            if value < 0 {
                rust_vmstat_warn_negative(zone_stat_name(i), value);
            }
        }
        for i in 0..NR_VM_NODE_STAT_ITEMS as usize {
            if i == NR_WRITEBACK as usize {
                continue;
            }
            let value = rust_vmstat_atomic_read(addr_of!(vm_node_stat[i]));
            if value < 0 {
                rust_vmstat_warn_negative(node_stat_name(i), value);
            }
        }
        if write != 0 {
            *ppos = (*ppos).wrapping_add(*lenp as loff_t);
        } else {
            *lenp = 0;
        }
        0
    }
}
#[no_mangle]
unsafe extern "C" fn rust_vmstat_update(_work: *mut work_struct) {
    // SAFETY: this body preserves the native caller's pointer, locking and CPU-context contract.
    unsafe {
        if refresh_cpu_vm_stats(true) {
            queue_delayed_work_on(
                rust_vmstat_smp_processor_id(),
                MM_PERCPU_WQ,
                rust_vmstat_this_work(),
                round_jiffies_relative(SYSCTL_STAT_INTERVAL as c_ulong),
            );
        }
    }
}
unsafe fn need_update(cpu: c_int) -> bool {
    // SAFETY: this body preserves the native caller's pointer, locking and CPU-context contract.
    unsafe {
        let mut last: *mut pglist_data = null_mut();
        zones!(z, {
            let pz = rust_vmstat_zone_cpu((*z).per_cpu_zonestats, cpu);
            if !memchr_inv(
                addr_of!((*pz).vm_stat_diff).cast(),
                0,
                size_of::<[i8; NR_VM_ZONE_STAT_ITEMS as usize]>(),
            )
            .is_null()
            {
                return true;
            }
            let pgd = (*z).zone_pgdat;
            if last != pgd {
                last = pgd;
                let p = rust_vmstat_node_cpu((*pgd).per_cpu_nodestats, cpu);
                if !memchr_inv(
                    addr_of!((*p).vm_node_stat_diff).cast(),
                    0,
                    size_of::<[i8; NR_VM_NODE_STAT_ITEMS as usize]>(),
                )
                .is_null()
                {
                    return true;
                }
            }
        });
        false
    }
}
#[no_mangle]
unsafe extern "C" fn quiet_vmstat() {
    // SAFETY: this body preserves the native caller's pointer, locking and CPU-context contract.
    unsafe {
        if system_state != SYSTEM_RUNNING {
            return;
        }
        if !rust_vmstat_work_pending(rust_vmstat_this_work()) {
            return;
        }
        if !need_update(rust_vmstat_smp_processor_id()) {
            return;
        }
        refresh_cpu_vm_stats(false);
    }
}
#[no_mangle]
unsafe extern "C" fn vmstat_flush_workqueue() {
    // SAFETY: this body preserves the native caller's pointer, locking and CPU-context contract.
    unsafe {
        rust_vmstat_flush_workqueue(MM_PERCPU_WQ);
    }
}
#[no_mangle]
unsafe extern "C" fn rust_vmstat_shepherd(_work: *mut work_struct) {
    // SAFETY: this body preserves the native caller's pointer, locking and CPU-context contract.
    unsafe {
        rust_vmstat_cpus_read_lock();
        online_cpus!(cpu, {
            let dw = rust_vmstat_work_cpu(cpu);
            rust_vmstat_rcu_read_lock();
            let isolated = rust_vmstat_cpu_is_isolated(cpu);
            if !isolated && work_busy(addr_of_mut!((*dw).work)) == 0 && need_update(cpu) {
                queue_delayed_work_on(cpu, MM_PERCPU_WQ, dw, 0);
            }
            rust_vmstat_rcu_read_unlock();
            // scoped_guard's generated inner loop consumes the C continue.
            // Its cleanup releases RCU before this outer-loop reschedule point.
            rust_vmstat_cond_resched();
        });
        rust_vmstat_cpus_read_unlock();
        rust_vmstat_schedule_delayed(
            rust_vmstat_shepherd_work(),
            round_jiffies_relative(SYSCTL_STAT_INTERVAL as c_ulong),
        );
    }
}
#[link_section = ".init.text"]
#[cfg_attr(RUST_VMSTAT_INIT_COLD, cold)]
unsafe fn start_shepherd_timer() {
    // SAFETY: this body preserves the native caller's pointer, locking and CPU-context contract.
    unsafe {
        let mut cpu = rust_vmstat_next_possible_cpu(-1);
        while cpu < rust_vmstat_nr_cpu_ids() {
            let dw = rust_vmstat_work_cpu(cpu);
            rust_vmstat_init_cpu_work(cpu);
            if !rust_vmstat_cpu_online(cpu) {
                disable_delayed_work_sync(dw);
            }
            cpu = rust_vmstat_next_possible_cpu(cpu);
        }
        rust_vmstat_schedule_delayed(
            rust_vmstat_shepherd_work(),
            round_jiffies_relative(SYSCTL_STAT_INTERVAL as c_ulong),
        );
    }
}
#[link_section = ".init.text"]
#[cfg_attr(RUST_VMSTAT_INIT_COLD, cold)]
unsafe fn init_cpu_node_state() {
    // SAFETY: this body preserves the native caller's pointer, locking and CPU-context contract.
    unsafe {
        let mut node = rust_vmstat_next_online_node(-1);
        while node < RUST_VMSTAT_MAX_NUMNODES as c_int {
            if !rust_vmstat_node_cpumask_empty(node) {
                rust_vmstat_node_set_cpu(node);
            }
            node = rust_vmstat_next_online_node(node);
        }
    }
}
unsafe extern "C" fn vmstat_cpu_online(cpu: c_uint) -> c_int {
    // SAFETY: this body preserves the native caller's pointer, locking and CPU-context contract.
    unsafe {
        if VMSTAT_LATE_INIT_DONE != 0 {
            refresh_zone_stat_thresholds();
        }
        let node = rust_vmstat_cpu_to_node(cpu as c_int);
        if !rust_vmstat_node_has_cpu(node) {
            rust_vmstat_node_set_cpu(node);
        }
        enable_delayed_work(rust_vmstat_work_cpu(cpu as c_int));
        0
    }
}
unsafe extern "C" fn vmstat_cpu_down_prep(cpu: c_uint) -> c_int {
    // SAFETY: this body preserves the native caller's pointer, locking and CPU-context contract.
    unsafe {
        disable_delayed_work_sync(rust_vmstat_work_cpu(cpu as c_int));
        0
    }
}
unsafe extern "C" fn vmstat_cpu_dead(cpu: c_uint) -> c_int {
    // SAFETY: this body preserves the native caller's pointer, locking and CPU-context contract.
    unsafe {
        let node = rust_vmstat_cpu_to_node(cpu as c_int);
        refresh_zone_stat_thresholds();
        if !rust_vmstat_node_cpumask_empty(node) {
            return 0;
        }
        rust_vmstat_node_clear_cpu(node);
        0
    }
}
#[no_mangle]
#[link_section = ".init.text"]
#[cfg_attr(RUST_VMSTAT_INIT_COLD, cold)]
unsafe extern "C" fn rust_vmstat_late_init() -> c_int {
    // SAFETY: this body preserves the native caller's pointer, locking and CPU-context contract.
    unsafe {
        refresh_zone_stat_thresholds();
        VMSTAT_LATE_INIT_DONE = 1;
        0
    }
}
