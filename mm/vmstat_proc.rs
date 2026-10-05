// SPDX-License-Identifier: GPL-2.0-only
#[cfg(CONFIG_NUMA)]
static mut SYSCTL_VM_NUMA_STAT: c_int = 1;
#[cfg(CONFIG_NUMA)]
unsafe fn zero_zone_numa_counters(z: *mut zone) {
    // SAFETY: this body preserves the native caller's pointer, locking and CPU-context contract.
    unsafe {
        for i in 0..NR_VM_NUMA_EVENT_ITEMS as usize {
            rust_vmstat_atomic_set(addr_of_mut!((*z).vm_numa_event[i]), 0);
            online_cpus!(cpu, {
                let pz = rust_vmstat_zone_cpu((*z).per_cpu_zonestats, cpu);
                (*pz).vm_numa_event[i] = 0;
            });
        }
    }
}
#[cfg(CONFIG_NUMA)]
unsafe fn zero_zones_numa_counters() {
    // SAFETY: this body preserves the native caller's pointer, locking and CPU-context contract.
    unsafe {
        zones!(z, {
            zero_zone_numa_counters(z);
        });
    }
}
#[cfg(CONFIG_NUMA)]
unsafe fn zero_global_numa_counters() {
    // SAFETY: this body preserves the native caller's pointer, locking and CPU-context contract.
    unsafe {
        for i in 0..NR_VM_NUMA_EVENT_ITEMS as usize {
            rust_vmstat_atomic_set(addr_of_mut!(vm_numa_event[i]), 0);
        }
    }
}
#[cfg(CONFIG_NUMA)]
unsafe fn invalid_numa_statistics() {
    // SAFETY: this body preserves the native caller's pointer, locking and CPU-context contract.
    unsafe {
        zero_zones_numa_counters();
        zero_global_numa_counters();
    }
}
#[cfg(CONFIG_NUMA)]
unsafe extern "C" fn sysctl_vm_numa_stat_handler(
    table: *const ctl_table,
    write: c_int,
    buffer: *mut c_void,
    length: *mut usize,
    ppos: *mut loff_t,
) -> c_int {
    // SAFETY: this body preserves the native caller's pointer, locking and CPU-context contract.
    unsafe {
        rust_vmstat_numa_lock();
        let old = if write != 0 { SYSCTL_VM_NUMA_STAT } else { 0 };
        let ret = proc_dointvec_minmax(table, write, buffer, length, ppos);
        if ret == 0 && write != 0 && old != SYSCTL_VM_NUMA_STAT {
            if SYSCTL_VM_NUMA_STAT == 1 {
                rust_vmstat_numa_enable();
                rust_vmstat_log_numa_enabled();
            } else {
                rust_vmstat_numa_disable();
                invalid_numa_statistics();
                rust_vmstat_log_numa_disabled();
            }
        }
        rust_vmstat_numa_unlock();
        ret
    }
}
unsafe fn frag_show_print(m: *mut seq_file, p: *mut pglist_data, z: *mut zone) {
    // SAFETY: this body preserves the native caller's pointer, locking and CPU-context contract.
    unsafe {
        seq_printf(m, c"Node %d, zone %8s ".as_ptr(), (*p).node_id, (*z).name);
        for order in 0..RUST_VMSTAT_NR_PAGE_ORDERS {
            seq_printf(m, c"%6lu ".as_ptr(), rust_vmstat_free_blocks(z, order));
        }
        seq_putc(m, b'\n' as c_char);
    }
}
unsafe extern "C" fn frag_show(m: *mut seq_file, arg: *mut c_void) -> c_int {
    // SAFETY: this body preserves the native caller's pointer, locking and CPU-context contract.
    unsafe {
        walk_zones_in_node(m, arg.cast(), true, true, frag_show_print);
        0
    }
}
unsafe fn pagetypeinfo_showfree_print(m: *mut seq_file, p: *mut pglist_data, z: *mut zone) {
    // SAFETY: this body preserves the native caller's pointer, locking and CPU-context contract.
    unsafe {
        for mtype in 0..MIGRATE_TYPES as usize {
            seq_printf(
                m,
                c"Node %4d, zone %8s, type %12s ".as_ptr(),
                (*p).node_id,
                (*z).name,
                migratetype_names[mtype],
            );
            for order in 0..RUST_VMSTAT_NR_PAGE_ORDERS as usize {
                let head = addr_of_mut!((*z).free_area[order].free_list[mtype]);
                let mut curr = (*head).next;
                let mut freecount: c_ulong = 0;
                let mut overflow = false;
                while curr != head {
                    freecount = freecount.wrapping_add(1);
                    if freecount >= 100000 {
                        overflow = true;
                        break;
                    }
                    curr = (*curr).next;
                }
                seq_printf(
                    m,
                    c"%s%6lu ".as_ptr(),
                    if overflow {
                        c">".as_ptr()
                    } else {
                        c"".as_ptr()
                    },
                    freecount,
                );
                rust_vmstat_zone_unlock_irq(z);
                rust_vmstat_cond_resched();
                rust_vmstat_zone_lock_irq(z);
            }
            seq_putc(m, b'\n' as c_char);
        }
    }
}
unsafe fn pagetypeinfo_showfree(m: *mut seq_file, p: *mut pglist_data) {
    // SAFETY: this body preserves the native caller's pointer, locking and CPU-context contract.
    unsafe {
        seq_printf(
            m,
            c"%-43s ".as_ptr(),
            c"Free pages count per migrate type at order".as_ptr(),
        );
        for order in 0..RUST_VMSTAT_NR_PAGE_ORDERS {
            seq_printf(m, c"%6d ".as_ptr(), order as c_int);
        }
        seq_putc(m, b'\n' as c_char);
        walk_zones_in_node(m, p, true, false, pagetypeinfo_showfree_print);
    }
}
unsafe fn pagetypeinfo_showblockcount_print(m: *mut seq_file, p: *mut pglist_data, z: *mut zone) {
    // SAFETY: this body preserves the native caller's pointer, locking and CPU-context contract.
    unsafe {
        let end = rust_vmstat_zone_end_pfn(z);
        let mut counts = [0 as c_ulong; MIGRATE_TYPES as usize];
        let mut pfn = (*z).zone_start_pfn;
        while pfn < end {
            let page = rust_vmstat_pfn_to_online_page(pfn);
            if !page.is_null() && rust_vmstat_page_zone(page) == z {
                let mtype = rust_vmstat_get_pageblock_migratetype(page) as c_int;
                if mtype < MIGRATE_TYPES as c_int {
                    counts[mtype as usize] = counts[mtype as usize].wrapping_add(1);
                }
            }
            pfn = pfn.wrapping_add(rust_vmstat_pageblock_nr_pages());
        }
        seq_printf(m, c"Node %d, zone %8s ".as_ptr(), (*p).node_id, (*z).name);
        for mtype in 0..MIGRATE_TYPES as usize {
            seq_printf(m, c"%12lu ".as_ptr(), counts[mtype]);
        }
        seq_putc(m, b'\n' as c_char);
    }
}
unsafe fn pagetypeinfo_showblockcount(m: *mut seq_file, p: *mut pglist_data) {
    // SAFETY: this body preserves the native caller's pointer, locking and CPU-context contract.
    unsafe {
        seq_printf(m, c"\n%-23s".as_ptr(), c"Number of blocks type ".as_ptr());
        for mtype in 0..MIGRATE_TYPES as usize {
            seq_printf(m, c"%12s ".as_ptr(), migratetype_names[mtype]);
        }
        seq_putc(m, b'\n' as c_char);
        walk_zones_in_node(m, p, true, false, pagetypeinfo_showblockcount_print);
    }
}
#[cfg(CONFIG_PAGE_OWNER)]
unsafe fn showmixed_print(m: *mut seq_file, p: *mut pglist_data, z: *mut zone) {
    // SAFETY: this body preserves the native caller's pointer, locking and CPU-context contract.
    unsafe {
        pagetypeinfo_showmixedcount_print(m, p, z);
    }
}
#[cfg(CONFIG_PAGE_OWNER)]
unsafe fn pagetypeinfo_showmixedcount(m: *mut seq_file, p: *mut pglist_data) {
    // SAFETY: this body preserves the native caller's pointer, locking and CPU-context contract.
    unsafe {
        #[cfg(CONFIG_PAGE_OWNER)]
        {
            if !rust_vmstat_page_owner_inited() {
                return;
            }
            drain_all_pages(null_mut());
            seq_printf(m, c"\n%-23s".as_ptr(), c"Number of mixed blocks ".as_ptr());
            for mtype in 0..MIGRATE_TYPES as usize {
                seq_printf(m, c"%12s ".as_ptr(), migratetype_names[mtype]);
            }
            seq_putc(m, b'\n' as c_char);
            walk_zones_in_node(m, p, true, true, showmixed_print);
        }
    }
}
unsafe extern "C" fn pagetypeinfo_show(m: *mut seq_file, arg: *mut c_void) -> c_int {
    // SAFETY: this body preserves the native caller's pointer, locking and CPU-context contract.
    unsafe {
        let p = arg.cast::<pglist_data>();
        if !rust_vmstat_node_has_memory((*p).node_id) {
            return 0;
        }
        seq_printf(
            m,
            c"Page block order: %d\n".as_ptr(),
            rust_vmstat_pageblock_order() as c_int,
        );
        seq_printf(
            m,
            c"Pages per block:  %lu\n".as_ptr(),
            rust_vmstat_pageblock_nr_pages(),
        );
        seq_putc(m, b'\n' as c_char);
        pagetypeinfo_showfree(m, p);
        pagetypeinfo_showblockcount(m, p);
        #[cfg(CONFIG_PAGE_OWNER)]
        pagetypeinfo_showmixedcount(m, p);
        0
    }
}
static FRAGMENTATION_OP: seq_operations = seq_operations {
    start: Some(frag_start),
    next: Some(frag_next),
    stop: Some(frag_stop),
    show: Some(frag_show),
};
static PAGETYPEINFO_OP: seq_operations = seq_operations {
    start: Some(frag_start),
    next: Some(frag_next),
    stop: Some(frag_stop),
    show: Some(pagetypeinfo_show),
};
unsafe fn is_zone_first_populated(p: *mut pglist_data, z: *mut zone) -> bool {
    // SAFETY: this body preserves the native caller's pointer, locking and CPU-context contract.
    unsafe {
        for zid in 0..MAX_NR_ZONES as usize {
            let compare = addr_of_mut!((*p).node_zones[zid]);
            if populated(compare) {
                return z == compare;
            }
        }
        false
    }
}
unsafe fn zoneinfo_show_print(m: *mut seq_file, p: *mut pglist_data, z: *mut zone) {
    // SAFETY: this body preserves the native caller's pointer, locking and CPU-context contract.
    unsafe {
        seq_printf(m, c"Node %d, zone %8s".as_ptr(), (*p).node_id, (*z).name);
        if is_zone_first_populated(p, z) {
            seq_printf(m, c"\n  per-node stats".as_ptr());
            for i in 0..NR_VM_NODE_STAT_ITEMS as usize {
                let mut pages = node_state_pages(p, i);
                if rust_vmstat_item_print_in_thp(i as _) {
                    pages /= RUST_VMSTAT_HPAGE_PMD_NR;
                }
                seq_printf(m, c"\n      %-12s %lu".as_ptr(), node_stat_name(i), pages);
            }
        }
        seq_printf(m, c"\n  pages free     %lu\n        boost    %lu\n        min      %lu\n        low      %lu\n        high     %lu\n        promo    %lu\n        spanned  %lu\n        present  %lu\n        managed  %lu\n        cma      %lu".as_ptr(),
        zone_state(z, NR_FREE_PAGES as usize), (*z).watermark_boost, rust_vmstat_min_wmark(z), rust_vmstat_low_wmark(z), rust_vmstat_high_wmark(z), rust_vmstat_promo_wmark(z), (*z).spanned_pages, (*z).present_pages, rust_vmstat_managed_pages(z), rust_vmstat_cma_pages(z));
        seq_printf(
            m,
            c"\n        protection: (%ld".as_ptr(),
            (*z).lowmem_reserve[0] as c_long,
        );
        for i in 1..(*z).lowmem_reserve.len() {
            seq_printf(m, c", %ld".as_ptr(), (*z).lowmem_reserve[i] as c_long);
        }
        seq_putc(m, b')' as c_char);
        if !populated(z) {
            seq_putc(m, b'\n' as c_char);
            return;
        }
        for i in 0..NR_VM_ZONE_STAT_ITEMS as usize {
            seq_printf(
                m,
                c"\n      %-12s %lu".as_ptr(),
                zone_stat_name(i),
                zone_state(z, i),
            );
        }
        #[cfg(CONFIG_NUMA)]
        {
            fold_vm_zone_numa_events(z);
            for i in 0..NR_VM_NUMA_EVENT_ITEMS as usize {
                seq_printf(
                    m,
                    c"\n      %-12s %lu".as_ptr(),
                    numa_stat_name(i),
                    rust_vmstat_atomic_read(addr_of!((*z).vm_numa_event[i])) as c_ulong,
                );
            }
        }
        seq_printf(m, c"\n  pagesets".as_ptr());
        online_cpus!(cpu, {
            let pcp = rust_vmstat_pages_cpu((*z).per_cpu_pageset, cpu);
            seq_printf(m, c"\n    cpu: %i\n              count:    %i\n              high:     %i\n              batch:    %i\n              high_min: %i\n              high_max: %i".as_ptr(), cpu, (*pcp).count, (*pcp).high, (*pcp).batch, (*pcp).high_min, (*pcp).high_max);
            #[cfg(CONFIG_SMP)]
            {
                let pz = rust_vmstat_zone_cpu((*z).per_cpu_zonestats, cpu);
                seq_printf(
                    m,
                    c"\n  vm stats threshold: %d".as_ptr(),
                    (*pz).stat_threshold as c_int,
                );
            }
        });
        seq_printf(m, c"\n  node_unreclaimable:  %u\n  start_pfn:           %lu\n  reserved_highatomic: %lu\n  free_highatomic:     %lu".as_ptr(), kswapd_test_hopeless(p) as c_uint, (*z).zone_start_pfn, (*z).nr_reserved_highatomic, (*z).nr_free_highatomic);
        seq_putc(m, b'\n' as c_char);
    }
}
unsafe extern "C" fn zoneinfo_show(m: *mut seq_file, arg: *mut c_void) -> c_int {
    // SAFETY: this body preserves the native caller's pointer, locking and CPU-context contract.
    unsafe {
        walk_zones_in_node(m, arg.cast(), false, false, zoneinfo_show_print);
        0
    }
}
static ZONEINFO_OP: seq_operations = seq_operations {
    start: Some(frag_start),
    next: Some(frag_next),
    stop: Some(frag_stop),
    show: Some(zoneinfo_show),
};
unsafe extern "C" fn vmstat_start(m: *mut seq_file, pos: *mut loff_t) -> *mut c_void {
    // SAFETY: this body preserves the native caller's pointer, locking and CPU-context contract.
    unsafe {
        if *pos >= NR_VMSTAT_ITEMS as loff_t {
            return null_mut();
        }
        #[cfg(CONFIG_NUMA)]
        fold_vm_numa_events();
        let base = rust_vmstat_kmalloc_array(
            NR_VMSTAT_ITEMS,
            size_of::<c_ulong>(),
            RUST_VMSTAT_GFP_KERNEL,
        )
        .cast::<c_ulong>();
        (*m).private = base.cast();
        if base.is_null() {
            return rust_vmstat_err_ptr(-(RUST_VMSTAT_ENOMEM as c_long));
        }
        let mut v = base;
        for i in 0..NR_VM_ZONE_STAT_ITEMS as usize {
            *v.add(i) = clamped_read(addr_of!(vm_zone_stat[i]));
        }
        v = v.add(NR_VM_ZONE_STAT_ITEMS as usize);
        #[cfg(CONFIG_NUMA)]
        {
            for i in 0..NR_VM_NUMA_EVENT_ITEMS as usize {
                *v.add(i) = rust_vmstat_atomic_read(addr_of!(vm_numa_event[i])) as c_ulong;
            }
            v = v.add(NR_VM_NUMA_EVENT_ITEMS as usize);
        }
        for i in 0..NR_VM_NODE_STAT_ITEMS as usize {
            *v.add(i) = clamped_read(addr_of!(vm_node_stat[i]));
            if rust_vmstat_item_print_in_thp(i as _) {
                *v.add(i) /= RUST_VMSTAT_HPAGE_PMD_NR;
            }
        }
        v = v.add(NR_VM_NODE_STAT_ITEMS as usize);
        global_dirty_limits(
            v.add(NR_DIRTY_BG_THRESHOLD as usize),
            v.add(NR_DIRTY_THRESHOLD as usize),
        );
        *v.add(NR_MEMMAP_PAGES as usize) =
            rust_vmstat_atomic_read(addr_of!(NR_MEMMAP_PAGES_STORAGE)) as c_ulong;
        *v.add(NR_MEMMAP_BOOT_PAGES as usize) =
            rust_vmstat_atomic_read(addr_of!(NR_MEMMAP_BOOT_PAGES_STORAGE)) as c_ulong;
        #[cfg(CONFIG_VM_EVENT_COUNTERS)]
        {
            v = v.add(NR_VM_STAT_ITEMS as usize);
            all_vm_events(v);
            *v.add(PGPGIN as usize) /= 2;
            *v.add(PGPGOUT as usize) /= 2;
        }
        base.offset(*pos as isize).cast()
    }
}
unsafe extern "C" fn vmstat_next(
    m: *mut seq_file,
    _arg: *mut c_void,
    pos: *mut loff_t,
) -> *mut c_void {
    // SAFETY: this body preserves the native caller's pointer, locking and CPU-context contract.
    unsafe {
        *pos = (*pos).wrapping_add(1);
        if *pos >= NR_VMSTAT_ITEMS as loff_t {
            return null_mut();
        }
        (*m).private.cast::<c_ulong>().offset(*pos as isize).cast()
    }
}
unsafe extern "C" fn vmstat_show(m: *mut seq_file, arg: *mut c_void) -> c_int {
    // SAFETY: this body preserves the native caller's pointer, locking and CPU-context contract.
    unsafe {
        let l = arg.cast::<c_ulong>();
        let off = l.offset_from((*m).private.cast::<c_ulong>()) as usize;
        __seq_puts(m, VMSTAT_TEXT.0[off]);
        seq_put_decimal_ull(m, c" ".as_ptr(), *l as u64);
        seq_putc(m, b'\n' as c_char);
        if off == NR_VMSTAT_ITEMS - 1 {
            __seq_puts(m, c"nr_unstable 0\n".as_ptr());
        }
        0
    }
}
unsafe extern "C" fn vmstat_stop(m: *mut seq_file, _arg: *mut c_void) {
    // SAFETY: this body preserves the native caller's pointer, locking and CPU-context contract.
    unsafe {
        kfree((*m).private);
        (*m).private = null_mut();
    }
}
static VMSTAT_OP: seq_operations = seq_operations {
    start: Some(vmstat_start),
    next: Some(vmstat_next),
    stop: Some(vmstat_stop),
    show: Some(vmstat_show),
};
