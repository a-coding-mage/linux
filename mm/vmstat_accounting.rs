// SPDX-License-Identifier: GPL-2.0-only
#[no_mangle]
unsafe extern "C" fn calculate_pressure_threshold(z: *mut zone) -> c_int {
    // SAFETY: this body preserves the native caller's pointer, locking and CPU-context contract.
    unsafe {
        // C first narrows the unsigned watermark difference to int, then promotes
        // that int to unsigned for division by num_online_cpus() (unsigned int).
        let distance = rust_vmstat_low_wmark(z).wrapping_sub(rust_vmstat_min_wmark(z)) as c_int;
        min(
            125,
            max(
                1,
                ((distance as c_uint) / rust_vmstat_num_online_cpus()) as c_int,
            ),
        )
    }
}
#[no_mangle]
unsafe extern "C" fn calculate_normal_threshold(z: *mut zone) -> c_int {
    // SAFETY: this body preserves the native caller's pointer, locking and CPU-context contract.
    unsafe {
        let mem = (rust_vmstat_managed_pages(z) >> (27 - RUST_VMSTAT_PAGE_SHIFT)) as c_int;
        min(
            125,
            2i32.wrapping_mul(rust_vmstat_fls(rust_vmstat_num_online_cpus() as c_uint))
                .wrapping_mul(1i32.wrapping_add(rust_vmstat_fls(mem as c_uint))),
        )
    }
}
#[no_mangle]
unsafe extern "C" fn refresh_zone_stat_thresholds() {
    // SAFETY: this body preserves the native caller's pointer, locking and CPU-context contract.
    unsafe {
        nodes!(p, {
            online_cpus!(cpu, {
                (*rust_vmstat_node_cpu((*p).per_cpu_nodestats, cpu)).stat_threshold = 0;
            });
        });
        zones!(z, {
            let p = (*z).zone_pgdat;
            let threshold = calculate_normal_threshold(z);
            online_cpus!(cpu, {
                (*rust_vmstat_zone_cpu((*z).per_cpu_zonestats, cpu)).stat_threshold =
                    threshold as i8;
                let ns = rust_vmstat_node_cpu((*p).per_cpu_nodestats, cpu);
                (*ns).stat_threshold = max(threshold, (*ns).stat_threshold as c_int) as i8;
            });
            let tolerate = rust_vmstat_low_wmark(z).wrapping_sub(rust_vmstat_min_wmark(z));
            let drift = rust_vmstat_num_online_cpus().wrapping_mul(threshold as c_uint) as c_ulong;
            if drift > tolerate {
                (*z).percpu_drift_mark = rust_vmstat_high_wmark(z).wrapping_add(drift);
            }
        });
    }
}
#[no_mangle]
unsafe extern "C" fn set_pgdat_percpu_threshold(
    p: *mut pglist_data,
    calc: Option<unsafe extern "C" fn(*mut zone) -> c_int>,
) {
    // SAFETY: this body preserves the native caller's pointer, locking and CPU-context contract.
    unsafe {
        for i in 0..(*p).nr_zones as usize {
            let z = addr_of_mut!((*p).node_zones[i]);
            if (*z).percpu_drift_mark == 0 {
                continue;
            }
            // Native API requires a non-NULL callback.
            let t = calc.unwrap_unchecked()(z);
            online_cpus!(cpu, {
                (*rust_vmstat_zone_cpu((*z).per_cpu_zonestats, cpu)).stat_threshold = t as i8;
            });
        }
    }
}
#[no_mangle]
unsafe extern "C" fn __mod_zone_page_state(z: *mut zone, item: zone_stat_item, delta: c_long) {
    // SAFETY: this body preserves the native caller's pointer, locking and CPU-context contract.
    unsafe {
        let pcp = (*z).per_cpu_zonestats;
        let p = addr_of_mut!((*pcp).vm_stat_diff[item as usize]);
        rust_vmstat_preempt_disable_nested();
        let mut x = delta.wrapping_add(rust_vmstat_raw_read_s8(p) as c_long);
        let t = rust_vmstat_raw_read_s8(addr_of!((*pcp).stat_threshold)) as c_long;
        if x.wrapping_abs() > t {
            zone_add(x, z, item as usize);
            x = 0;
        }
        rust_vmstat_raw_write_s8(p, x as i8);
        rust_vmstat_preempt_enable_nested();
    }
}
#[no_mangle]
unsafe extern "C" fn __mod_node_page_state(
    pgd: *mut pglist_data,
    item: node_stat_item,
    mut delta: c_long,
) {
    // SAFETY: this body preserves the native caller's pointer, locking and CPU-context contract.
    unsafe {
        let pcp = (*pgd).per_cpu_nodestats;
        let p = addr_of_mut!((*pcp).vm_node_stat_diff[item as usize]);
        if rust_vmstat_item_in_bytes(item as c_int) {
            rust_vmstat_warn_mod_node_bytes(delta & (RUST_VMSTAT_PAGE_SIZE as c_long - 1) != 0);
            delta >>= RUST_VMSTAT_PAGE_SHIFT;
        }
        rust_vmstat_preempt_disable_nested();
        let mut x = delta.wrapping_add(rust_vmstat_raw_read_s8(p) as c_long);
        let t = rust_vmstat_raw_read_s8(addr_of!((*pcp).stat_threshold)) as c_long;
        if x.wrapping_abs() > t {
            node_add(x, pgd, item as usize);
            x = 0;
        }
        rust_vmstat_raw_write_s8(p, x as i8);
        rust_vmstat_preempt_enable_nested();
    }
}
#[no_mangle]
unsafe extern "C" fn __inc_zone_state(z: *mut zone, item: zone_stat_item) {
    // SAFETY: this body preserves the native caller's pointer, locking and CPU-context contract.
    unsafe {
        let pcp = (*z).per_cpu_zonestats;
        let p = addr_of_mut!((*pcp).vm_stat_diff[item as usize]);
        rust_vmstat_preempt_disable_nested();
        let v = rust_vmstat_raw_inc_s8(p);
        let t = rust_vmstat_raw_read_s8(addr_of!((*pcp).stat_threshold));
        if v > t {
            let over = t >> 1;
            zone_add(v as c_long + over as c_long, z, item as usize);
            rust_vmstat_raw_write_s8(p, over.wrapping_neg());
        }
        rust_vmstat_preempt_enable_nested();
    }
}
#[no_mangle]
unsafe extern "C" fn __inc_node_state(pgd: *mut pglist_data, item: node_stat_item) {
    // SAFETY: this body preserves the native caller's pointer, locking and CPU-context contract.
    unsafe {
        let pcp = (*pgd).per_cpu_nodestats;
        let p = addr_of_mut!((*pcp).vm_node_stat_diff[item as usize]);
        rust_vmstat_warn_inc_node_bytes(rust_vmstat_item_in_bytes(item as c_int));
        rust_vmstat_preempt_disable_nested();
        let v = rust_vmstat_raw_inc_s8(p);
        let t = rust_vmstat_raw_read_s8(addr_of!((*pcp).stat_threshold));
        if v > t {
            let over = t >> 1;
            node_add(v as c_long + over as c_long, pgd, item as usize);
            rust_vmstat_raw_write_s8(p, over.wrapping_neg());
        }
        rust_vmstat_preempt_enable_nested();
    }
}
#[no_mangle]
unsafe extern "C" fn __dec_zone_state(z: *mut zone, item: zone_stat_item) {
    // SAFETY: this body preserves the native caller's pointer, locking and CPU-context contract.
    unsafe {
        let pcp = (*z).per_cpu_zonestats;
        let p = addr_of_mut!((*pcp).vm_stat_diff[item as usize]);
        rust_vmstat_preempt_disable_nested();
        let v = rust_vmstat_raw_dec_s8(p);
        let t = rust_vmstat_raw_read_s8(addr_of!((*pcp).stat_threshold));
        if (v as c_int) < -(t as c_int) {
            let over = t >> 1;
            zone_add(v as c_long - over as c_long, z, item as usize);
            rust_vmstat_raw_write_s8(p, over);
        }
        rust_vmstat_preempt_enable_nested();
    }
}
#[no_mangle]
unsafe extern "C" fn __dec_node_state(pgd: *mut pglist_data, item: node_stat_item) {
    // SAFETY: this body preserves the native caller's pointer, locking and CPU-context contract.
    unsafe {
        let pcp = (*pgd).per_cpu_nodestats;
        let p = addr_of_mut!((*pcp).vm_node_stat_diff[item as usize]);
        rust_vmstat_warn_dec_node_bytes(rust_vmstat_item_in_bytes(item as c_int));
        rust_vmstat_preempt_disable_nested();
        let v = rust_vmstat_raw_dec_s8(p);
        let t = rust_vmstat_raw_read_s8(addr_of!((*pcp).stat_threshold));
        if (v as c_int) < -(t as c_int) {
            let over = t >> 1;
            node_add(v as c_long - over as c_long, pgd, item as usize);
            rust_vmstat_raw_write_s8(p, over);
        }
        rust_vmstat_preempt_enable_nested();
    }
}
#[no_mangle]
unsafe extern "C" fn __inc_zone_page_state(p: *mut page, item: zone_stat_item) {
    // SAFETY: this body preserves the native caller's pointer, locking and CPU-context contract.
    unsafe {
        __inc_zone_state(rust_vmstat_page_zone(p), item);
    }
}
#[no_mangle]
unsafe extern "C" fn __dec_zone_page_state(p: *mut page, item: zone_stat_item) {
    // SAFETY: this body preserves the native caller's pointer, locking and CPU-context contract.
    unsafe {
        __dec_zone_state(rust_vmstat_page_zone(p), item);
    }
}
#[no_mangle]
unsafe extern "C" fn __inc_node_page_state(p: *mut page, item: node_stat_item) {
    // SAFETY: this body preserves the native caller's pointer, locking and CPU-context contract.
    unsafe {
        __inc_node_state(rust_vmstat_page_pgdat(p), item);
    }
}
#[no_mangle]
unsafe extern "C" fn __dec_node_page_state(p: *mut page, item: node_stat_item) {
    // SAFETY: this body preserves the native caller's pointer, locking and CPU-context contract.
    unsafe {
        __dec_node_state(rust_vmstat_page_pgdat(p), item);
    }
}

#[cfg(CONFIG_HAVE_CMPXCHG_LOCAL)]
unsafe fn mod_zone_state(z: *mut zone, item: zone_stat_item, delta: c_long, overstep: c_int) {
    // SAFETY: this body preserves the native caller's pointer, locking and CPU-context contract.
    unsafe {
        let pcp = (*z).per_cpu_zonestats;
        let p = addr_of_mut!((*pcp).vm_stat_diff[item as usize]);
        let mut old = rust_vmstat_this_read_s8(p);
        let spill = loop {
            let mut spill: c_long = 0;
            let t = rust_vmstat_this_read_s8(addr_of!((*pcp).stat_threshold)) as c_long;
            let mut n = delta.wrapping_add(old as c_long);
            if n.wrapping_abs() > t {
                let os = (overstep as c_long).wrapping_mul(t >> 1) as c_int;
                spill = n.wrapping_add(os as c_long);
                n = os.wrapping_neg() as c_long;
            }
            if rust_vmstat_this_cmpxchg_s8(p, &mut old, n as i8) {
                break spill;
            }
        };
        if spill != 0 {
            zone_add(spill, z, item as usize);
        }
    }
}
#[cfg(CONFIG_HAVE_CMPXCHG_LOCAL)]
unsafe fn mod_node_state(
    pgd: *mut pglist_data,
    item: node_stat_item,
    mut delta: c_int,
    overstep: c_int,
) {
    // SAFETY: this body preserves the native caller's pointer, locking and CPU-context contract.
    unsafe {
        let pcp = (*pgd).per_cpu_nodestats;
        let p = addr_of_mut!((*pcp).vm_node_stat_diff[item as usize]);
        if rust_vmstat_item_in_bytes(item as c_int) {
            rust_vmstat_warn_cmpxchg_node_bytes(
                (delta as c_ulong & (RUST_VMSTAT_PAGE_SIZE - 1)) != 0,
            );
            delta >>= RUST_VMSTAT_PAGE_SHIFT;
        }
        let mut old = rust_vmstat_this_read_s8(p);
        let spill = loop {
            let mut spill: c_long = 0;
            let t = rust_vmstat_this_read_s8(addr_of!((*pcp).stat_threshold)) as c_long;
            let mut n = (delta as c_long).wrapping_add(old as c_long);
            if n.wrapping_abs() > t {
                let os = (overstep as c_long).wrapping_mul(t >> 1) as c_int;
                spill = n.wrapping_add(os as c_long);
                n = os.wrapping_neg() as c_long;
            }
            if rust_vmstat_this_cmpxchg_s8(p, &mut old, n as i8) {
                break spill;
            }
        };
        if spill != 0 {
            node_add(spill, pgd, item as usize);
        }
    }
}
#[no_mangle]
unsafe extern "C" fn mod_zone_page_state(z: *mut zone, item: zone_stat_item, delta: c_long) {
    // SAFETY: this body preserves the native caller's pointer, locking and CPU-context contract.
    unsafe {
        #[cfg(CONFIG_HAVE_CMPXCHG_LOCAL)]
        {
            mod_zone_state(z, item, delta, 0);
        }
        #[cfg(not(CONFIG_HAVE_CMPXCHG_LOCAL))]
        {
            let flags = rust_vmstat_irq_save();
            __mod_zone_page_state(z, item, delta);
            rust_vmstat_irq_restore(flags);
        }
    }
}
#[no_mangle]
unsafe extern "C" fn mod_node_page_state(p: *mut pglist_data, item: node_stat_item, delta: c_long) {
    // SAFETY: this body preserves the native caller's pointer, locking and CPU-context contract.
    unsafe {
        // Native cmpxchg helper intentionally accepts int, narrowing the long API.
        #[cfg(CONFIG_HAVE_CMPXCHG_LOCAL)]
        {
            mod_node_state(p, item, delta as c_int, 0);
        }
        #[cfg(not(CONFIG_HAVE_CMPXCHG_LOCAL))]
        {
            let flags = rust_vmstat_irq_save();
            __mod_node_page_state(p, item, delta);
            rust_vmstat_irq_restore(flags);
        }
    }
}
#[no_mangle]
unsafe extern "C" fn inc_zone_page_state(p: *mut page, item: zone_stat_item) {
    // SAFETY: this body preserves the native caller's pointer, locking and CPU-context contract.
    unsafe {
        let z = rust_vmstat_page_zone(p);
        #[cfg(CONFIG_HAVE_CMPXCHG_LOCAL)]
        {
            mod_zone_state(z, item, 1, 1);
        }
        #[cfg(not(CONFIG_HAVE_CMPXCHG_LOCAL))]
        {
            let flags = rust_vmstat_irq_save();
            __inc_zone_state(z, item);
            rust_vmstat_irq_restore(flags);
        }
    }
}
#[no_mangle]
unsafe extern "C" fn dec_zone_page_state(p: *mut page, item: zone_stat_item) {
    // SAFETY: this body preserves the native caller's pointer, locking and CPU-context contract.
    unsafe {
        #[cfg(CONFIG_HAVE_CMPXCHG_LOCAL)]
        {
            mod_zone_state(rust_vmstat_page_zone(p), item, -1, -1);
        }
        #[cfg(not(CONFIG_HAVE_CMPXCHG_LOCAL))]
        {
            let flags = rust_vmstat_irq_save();
            __dec_zone_page_state(p, item);
            rust_vmstat_irq_restore(flags);
        }
    }
}
#[no_mangle]
unsafe extern "C" fn inc_node_page_state(p: *mut page, item: node_stat_item) {
    // SAFETY: this body preserves the native caller's pointer, locking and CPU-context contract.
    unsafe {
        let pgd = rust_vmstat_page_pgdat(p);
        #[cfg(CONFIG_HAVE_CMPXCHG_LOCAL)]
        {
            mod_node_state(pgd, item, 1, 1);
        }
        #[cfg(not(CONFIG_HAVE_CMPXCHG_LOCAL))]
        {
            let flags = rust_vmstat_irq_save();
            __inc_node_state(pgd, item);
            rust_vmstat_irq_restore(flags);
        }
    }
}
#[no_mangle]
unsafe extern "C" fn dec_node_page_state(p: *mut page, item: node_stat_item) {
    // SAFETY: this body preserves the native caller's pointer, locking and CPU-context contract.
    unsafe {
        #[cfg(CONFIG_HAVE_CMPXCHG_LOCAL)]
        {
            mod_node_state(rust_vmstat_page_pgdat(p), item, -1, -1);
        }
        #[cfg(not(CONFIG_HAVE_CMPXCHG_LOCAL))]
        {
            let flags = rust_vmstat_irq_save();
            __dec_node_page_state(p, item);
            rust_vmstat_irq_restore(flags);
        }
    }
}

unsafe fn fold_diff(
    zd: &[c_int; NR_VM_ZONE_STAT_ITEMS as usize],
    nd: &[c_int; NR_VM_NODE_STAT_ITEMS as usize],
) -> bool {
    // SAFETY: this body preserves the native caller's pointer, locking and CPU-context contract.
    unsafe {
        let mut changed = false;
        for i in 0..NR_VM_ZONE_STAT_ITEMS as usize {
            if zd[i] != 0 {
                rust_vmstat_atomic_add(zd[i] as c_long, addr_of_mut!(vm_zone_stat[i]));
                changed = true;
            }
        }
        for i in 0..NR_VM_NODE_STAT_ITEMS as usize {
            if nd[i] != 0 {
                rust_vmstat_atomic_add(nd[i] as c_long, addr_of_mut!(vm_node_stat[i]));
                changed = true;
            }
        }
        changed
    }
}
unsafe fn refresh_cpu_vm_stats(do_pagesets: bool) -> bool {
    // SAFETY: this body preserves the native caller's pointer, locking and CPU-context contract.
    unsafe {
        let mut zd = [0 as c_int; NR_VM_ZONE_STAT_ITEMS as usize];
        let mut nd = [0 as c_int; NR_VM_NODE_STAT_ITEMS as usize];
        let mut changed = false;
        zones!(z, {
            let pz = (*z).per_cpu_zonestats;
            let pcp = (*z).per_cpu_pageset;
            for i in 0..NR_VM_ZONE_STAT_ITEMS as usize {
                let v = rust_vmstat_this_xchg_s8(addr_of_mut!((*pz).vm_stat_diff[i]), 0) as c_int;
                if v != 0 {
                    rust_vmstat_atomic_add(v as c_long, addr_of_mut!((*z).vm_stat[i]));
                    zd[i] = zd[i].wrapping_add(v);
                    #[cfg(CONFIG_NUMA)]
                    rust_vmstat_raw_write_u8(addr_of_mut!((*pcp).expire), 3);
                }
            }
            if do_pagesets {
                rust_vmstat_cond_resched();
                if decay_pcp_high(z, rust_vmstat_this_pages(pcp)) {
                    changed = true;
                }
                #[cfg(CONFIG_NUMA)]
                {
                    // A labeled block reproduces the original loop continues without
                    // skipping the iterator advance in zones!.
                    'pageset: {
                        if rust_vmstat_raw_read_u8(addr_of!((*pcp).expire)) == 0
                            || rust_vmstat_raw_read_int(addr_of!((*pcp).count)) == 0
                        {
                            break 'pageset;
                        }
                        if rust_vmstat_zone_to_nid(z) == rust_vmstat_numa_node_id() {
                            rust_vmstat_raw_write_u8(addr_of_mut!((*pcp).expire), 0);
                            break 'pageset;
                        }
                        if rust_vmstat_raw_dec_u8(addr_of_mut!((*pcp).expire)) != 0 {
                            changed = true;
                            break 'pageset;
                        }
                        if rust_vmstat_raw_read_int(addr_of!((*pcp).count)) != 0 {
                            drain_zone_pages(z, rust_vmstat_this_pages(pcp));
                            changed = true;
                        }
                    }
                }
            }
        });
        nodes!(pgd, {
            let p = (*pgd).per_cpu_nodestats;
            for i in 0..NR_VM_NODE_STAT_ITEMS as usize {
                let v =
                    rust_vmstat_this_xchg_s8(addr_of_mut!((*p).vm_node_stat_diff[i]), 0) as c_int;
                if v != 0 {
                    rust_vmstat_atomic_add(v as c_long, addr_of_mut!((*pgd).vm_stat[i]));
                    nd[i] = nd[i].wrapping_add(v);
                }
            }
        });
        if fold_diff(&zd, &nd) {
            changed = true;
        }
        changed
    }
}
#[no_mangle]
unsafe extern "C" fn cpu_vm_stats_fold(cpu: c_int) {
    // SAFETY: this body preserves the native caller's pointer, locking and CPU-context contract.
    unsafe {
        let mut zd = [0 as c_int; NR_VM_ZONE_STAT_ITEMS as usize];
        let mut nd = [0 as c_int; NR_VM_NODE_STAT_ITEMS as usize];
        zones!(z, {
            let pz = rust_vmstat_zone_cpu((*z).per_cpu_zonestats, cpu);
            for i in 0..NR_VM_ZONE_STAT_ITEMS as usize {
                let v = (*pz).vm_stat_diff[i] as c_int;
                if v != 0 {
                    (*pz).vm_stat_diff[i] = 0;
                    rust_vmstat_atomic_add(v as c_long, addr_of_mut!((*z).vm_stat[i]));
                    zd[i] = zd[i].wrapping_add(v);
                }
            }
            #[cfg(CONFIG_NUMA)]
            for i in 0..NR_VM_NUMA_EVENT_ITEMS as usize {
                let v = (*pz).vm_numa_event[i];
                if v != 0 {
                    (*pz).vm_numa_event[i] = 0;
                    numa_add(v as c_long, z, i);
                }
            }
        });
        nodes!(pgd, {
            let p = rust_vmstat_node_cpu((*pgd).per_cpu_nodestats, cpu);
            for i in 0..NR_VM_NODE_STAT_ITEMS as usize {
                let v = (*p).vm_node_stat_diff[i] as c_int;
                if v != 0 {
                    (*p).vm_node_stat_diff[i] = 0;
                    rust_vmstat_atomic_add(v as c_long, addr_of_mut!((*pgd).vm_stat[i]));
                    nd[i] = nd[i].wrapping_add(v);
                }
            }
        });
        fold_diff(&zd, &nd);
    }
}
#[no_mangle]
unsafe extern "C" fn drain_zonestat(z: *mut zone, pz: *mut per_cpu_zonestat) {
    // SAFETY: this body preserves the native caller's pointer, locking and CPU-context contract.
    unsafe {
        for i in 0..NR_VM_ZONE_STAT_ITEMS as usize {
            let v = (*pz).vm_stat_diff[i] as c_ulong;
            if v != 0 {
                (*pz).vm_stat_diff[i] = 0;
                zone_add(v as c_long, z, i);
            }
        }
        #[cfg(CONFIG_NUMA)]
        for i in 0..NR_VM_NUMA_EVENT_ITEMS as usize {
            let v = (*pz).vm_numa_event[i];
            if v != 0 {
                (*pz).vm_numa_event[i] = 0;
                numa_add(v as c_long, z, i);
            }
        }
    }
}
