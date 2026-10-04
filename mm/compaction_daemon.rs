// SPDX-License-Identifier: GPL-2.0
#[cfg_attr(RUST_COMPACTION_READ_MOSTLY, link_section = ".data..read_mostly")]
static mut sysctl_compact_unevictable_allowed: c_int =
    RUST_COMPACTION_CONFIG_COMPACT_UNEVICTABLE_DEFAULT;
#[cfg_attr(RUST_COMPACTION_READ_MOSTLY, link_section = ".data..read_mostly")]
static mut sysctl_compaction_proactiveness: c_uint = 20;
static mut sysctl_extfrag_threshold: c_int = 500;
#[cfg_attr(RUST_COMPACTION_READ_MOSTLY, link_section = ".data..read_mostly")]
static mut sysctl_compact_memory: c_int = 0;
unsafe fn compact_node(pgdat: *mut pg_data_t, proactive: bool) -> c_int {
    let mut cc: compact_control = zeroed();
    cc.order = -1;
    cc.mode = if proactive {
        MIGRATE_SYNC_LIGHT
    } else {
        MIGRATE_SYNC
    };
    cc.ignore_skip_hint = true;
    cc.whole_zone = true;
    cc.gfp_mask = RUST_COMPACTION_GFP_KERNEL;
    cc.proactive_compaction = proactive;
    for zoneid in 0..MAX_NR_ZONES as usize {
        let zone = addr_of_mut!((*pgdat).node_zones[zoneid]);
        if !rust_compaction_populated_zone(zone) {
            continue;
        }
        if rust_compaction_fatal_signal_pending(rust_compaction_current()) {
            return -(RUST_COMPACTION_EINTR as c_int);
        }
        cc.zone = zone;
        compact_zone(&mut cc, null_mut());
        if proactive {
            count_compact_events!(KCOMPACTD_MIGRATE_SCANNED, cc.total_migrate_scanned);
            count_compact_events!(KCOMPACTD_FREE_SCANNED, cc.total_free_scanned);
        }
    }
    0
}
unsafe fn compact_nodes() -> c_int {
    lru_add_drain_all();
    let mut nid = rust_compaction_first_online_node();
    while nid < RUST_COMPACTION_MAX_NUMNODES {
        let ret = compact_node(rust_compaction_NODE_DATA(nid), false);
        if ret != 0 {
            return ret;
        }
        nid = rust_compaction_next_online_node(nid);
    }
    0
}
#[cfg(CONFIG_SYSCTL)]
unsafe extern "C" fn compaction_proactiveness_sysctl_handler(
    table: *const ctl_table,
    write: c_int,
    buffer: *mut c_void,
    length: *mut usize,
    ppos: *mut loff_t,
) -> c_int {
    let rc = proc_dointvec_minmax(table, write, buffer, length, ppos);
    if rc != 0 {
        return rc;
    }
    if write != 0 && sysctl_compaction_proactiveness != 0 {
        let mut nid = rust_compaction_first_online_node();
        while nid < RUST_COMPACTION_MAX_NUMNODES {
            let pgdat = rust_compaction_NODE_DATA(nid);
            if !(*pgdat).proactive_compact_trigger {
                (*pgdat).proactive_compact_trigger = true;
                rust_compaction_trace_wakeup_kcompactd(
                    (*pgdat).node_id,
                    -1,
                    (*pgdat).nr_zones.wrapping_sub(1) as zone_type,
                );
                rust_compaction_wake_up_interruptible(addr_of_mut!((*pgdat).kcompactd_wait));
            }
            nid = rust_compaction_next_online_node(nid);
        }
    }
    0
}
#[cfg(CONFIG_SYSCTL)]
unsafe extern "C" fn sysctl_compaction_handler(
    table: *const ctl_table,
    write: c_int,
    buffer: *mut c_void,
    length: *mut usize,
    ppos: *mut loff_t,
) -> c_int {
    let mut ret = proc_dointvec(table, write, buffer, length, ppos);
    if ret != 0 {
        return ret;
    }
    if sysctl_compact_memory != 1 {
        return -(RUST_COMPACTION_EINVAL as c_int);
    }
    if write != 0 {
        ret = compact_nodes();
    }
    ret
}
#[cfg(all(CONFIG_SYSFS, CONFIG_NUMA))]
unsafe extern "C" fn compact_store(
    dev: *mut device,
    _attr: *mut device_attribute,
    _buf: *const kernel::ffi::c_char,
    count: usize,
) -> isize {
    let nid = rust_compaction_device_id(dev);
    if nid >= 0 && nid < rust_compaction_nr_node_ids() && rust_compaction_node_online(nid) {
        lru_add_drain_all();
        compact_node(rust_compaction_NODE_DATA(nid), false);
    }
    count as isize
}
#[cfg(all(CONFIG_SYSFS, CONFIG_NUMA))]
struct CompactDeviceAttribute(device_attribute);
#[cfg(all(CONFIG_SYSFS, CONFIG_NUMA))]
unsafe impl Sync for CompactDeviceAttribute {}
#[cfg(all(CONFIG_SYSFS, CONFIG_NUMA))]
static dev_attr_compact: CompactDeviceAttribute = CompactDeviceAttribute(device_attribute {
    attr: attribute {
        name: b"compact\0".as_ptr().cast(),
        mode: 0o200,
        ..unsafe { zeroed() }
    },
    show: None,
    store: Some(compact_store),
});
#[cfg(all(CONFIG_SYSFS, CONFIG_NUMA))]
#[no_mangle]
pub unsafe extern "C" fn compaction_register_node(node: *mut node) -> c_int {
    device_create_file(
        rust_compaction_node_device(node),
        addr_of!(dev_attr_compact.0),
    )
}
#[cfg(all(CONFIG_SYSFS, CONFIG_NUMA))]
#[no_mangle]
pub unsafe extern "C" fn compaction_unregister_node(node: *mut node) {
    device_remove_file(
        rust_compaction_node_device(node),
        addr_of!(dev_attr_compact.0),
    );
}
unsafe extern "C" fn kcompactd_work_requested(pgdat: *mut pg_data_t) -> bool {
    (*pgdat).kcompactd_max_order > 0 || kthread_should_stop() || (*pgdat).proactive_compact_trigger
}
unsafe fn kcompactd_node_suitable(pgdat: *mut pg_data_t) -> bool {
    let highest_zoneidx = (*pgdat).kcompactd_highest_zoneidx;
    let alloc_flags = if defrag_mode != 0 {
        RUST_COMPACTION_ALLOC_WMARK_HIGH
    } else {
        RUST_COMPACTION_ALLOC_WMARK_MIN
    };
    for zoneid in 0..=highest_zoneidx as usize {
        let zone = addr_of_mut!((*pgdat).node_zones[zoneid]);
        if !rust_compaction_populated_zone(zone) {
            continue;
        }
        if compaction_suit_allocation_order(
            zone,
            (*pgdat).kcompactd_max_order as c_uint,
            highest_zoneidx as c_int,
            alloc_flags,
            false,
            true,
        ) == COMPACT_CONTINUE
        {
            return true;
        }
    }
    false
}
unsafe fn kcompactd_do_work(pgdat: *mut pg_data_t) {
    let mut cc: compact_control = zeroed();
    cc.order = (*pgdat).kcompactd_max_order;
    cc.search_order = (*pgdat).kcompactd_max_order as _;
    cc.highest_zoneidx = (*pgdat).kcompactd_highest_zoneidx as c_int;
    cc.mode = MIGRATE_SYNC_LIGHT;
    cc.ignore_skip_hint = false;
    cc.gfp_mask = RUST_COMPACTION_GFP_KERNEL;
    cc.alloc_flags = if defrag_mode != 0 {
        RUST_COMPACTION_ALLOC_WMARK_HIGH
    } else {
        RUST_COMPACTION_ALLOC_WMARK_MIN
    };
    rust_compaction_trace_kcompactd_wake(
        (*pgdat).node_id,
        cc.order,
        cc.highest_zoneidx as zone_type,
    );
    count_compact_event!(KCOMPACTD_WAKE);
    for zoneid in 0..=cc.highest_zoneidx {
        let zone = addr_of_mut!((*pgdat).node_zones[zoneid as usize]);
        if !rust_compaction_populated_zone(zone) || compaction_deferred(zone, cc.order) {
            continue;
        }
        if compaction_suit_allocation_order(
            zone,
            cc.order as c_uint,
            zoneid,
            cc.alloc_flags,
            false,
            true,
        ) != COMPACT_CONTINUE
        {
            continue;
        }
        if kthread_should_stop() {
            return;
        }
        cc.zone = zone;
        let status = compact_zone(&mut cc, null_mut());
        if status == COMPACT_SUCCESS {
            compaction_defer_reset(zone, cc.order, false);
        } else if status == COMPACT_PARTIAL_SKIPPED || status == COMPACT_COMPLETE {
            drain_all_pages(zone);
            defer_compaction(zone, cc.order);
        }
        count_compact_events!(KCOMPACTD_MIGRATE_SCANNED, cc.total_migrate_scanned);
        count_compact_events!(KCOMPACTD_FREE_SCANNED, cc.total_free_scanned);
    }
    if (*pgdat).kcompactd_max_order <= cc.order {
        (*pgdat).kcompactd_max_order = 0;
    }
    if (*pgdat).kcompactd_highest_zoneidx >= cc.highest_zoneidx as zone_type {
        (*pgdat).kcompactd_highest_zoneidx = (*pgdat).nr_zones.wrapping_sub(1) as zone_type;
    }
}
#[no_mangle]
pub unsafe extern "C" fn wakeup_kcompactd(
    pgdat: *mut pg_data_t,
    order: c_int,
    highest_zoneidx: c_int,
) {
    if order == 0 {
        return;
    }
    if (*pgdat).kcompactd_max_order < order {
        (*pgdat).kcompactd_max_order = order;
    }
    if (*pgdat).kcompactd_highest_zoneidx > highest_zoneidx as zone_type {
        (*pgdat).kcompactd_highest_zoneidx = highest_zoneidx as zone_type;
    }
    if !rust_compaction_wq_has_sleeper(addr_of_mut!((*pgdat).kcompactd_wait))
        || !kcompactd_node_suitable(pgdat)
    {
        return;
    }
    rust_compaction_trace_wakeup_kcompactd((*pgdat).node_id, order, highest_zoneidx as zone_type);
    rust_compaction_wake_up_interruptible(addr_of_mut!((*pgdat).kcompactd_wait));
}
unsafe extern "C" fn kcompactd(p: *mut c_void) -> c_int {
    let pgdat = p.cast::<pg_data_t>();
    let default_timeout =
        rust_compaction_msecs_to_jiffies(HPAGE_FRAG_CHECK_INTERVAL_MSEC) as c_long;
    let mut timeout = default_timeout;
    rust_compaction_current_set_kcompactd(true);
    rust_compaction_set_freezable();
    (*pgdat).kcompactd_max_order = 0;
    (*pgdat).kcompactd_highest_zoneidx = (*pgdat).nr_zones.wrapping_sub(1) as zone_type;
    while !kthread_should_stop() {
        let mut pflags = 0;
        if sysctl_compaction_proactiveness == 0 {
            timeout = RUST_COMPACTION_MAX_SCHEDULE_TIMEOUT;
        }
        rust_compaction_trace_kcompactd_sleep((*pgdat).node_id);
        if rust_compaction_wait_event_freezable_timeout(
            pgdat,
            Some(kcompactd_work_requested),
            timeout,
        ) != 0
            && !(*pgdat).proactive_compact_trigger
        {
            rust_compaction_psi_memstall_enter(&mut pflags);
            kcompactd_do_work(pgdat);
            rust_compaction_psi_memstall_leave(&mut pflags);
            timeout = default_timeout;
            continue;
        }
        timeout = default_timeout;
        if should_proactive_compact_node(pgdat) {
            let prev_score = fragmentation_score_node(pgdat);
            compact_node(pgdat, true);
            let score = fragmentation_score_node(pgdat);
            if score >= prev_score {
                timeout = default_timeout.wrapping_shl(COMPACT_MAX_DEFER_SHIFT);
            }
        }
        if (*pgdat).proactive_compact_trigger {
            (*pgdat).proactive_compact_trigger = false;
        }
    }
    rust_compaction_current_set_kcompactd(false);
    0
}
#[no_mangle]
#[cfg_attr(not(CONFIG_MEMORY_HOTPLUG), link_section = ".init.text")]
#[cfg_attr(all(not(CONFIG_MEMORY_HOTPLUG), RUST_COMPACTION_INIT_COLD), cold)]
pub unsafe extern "C" fn kcompactd_run(nid: c_int) {
    let pgdat = rust_compaction_NODE_DATA(nid);
    if !(*pgdat).kcompactd.is_null() {
        return;
    }
    (*pgdat).kcompactd = rust_compaction_kthread_create_on_node(Some(kcompactd), pgdat.cast(), nid);
    if rust_compaction_IS_ERR((*pgdat).kcompactd.cast()) {
        rust_compaction_report_kcompactd_start_failure(nid);
        (*pgdat).kcompactd = null_mut();
    } else {
        wake_up_process((*pgdat).kcompactd);
    }
}
#[no_mangle]
#[cfg_attr(not(CONFIG_MEMORY_HOTPLUG), link_section = ".init.text")]
#[cfg_attr(all(not(CONFIG_MEMORY_HOTPLUG), RUST_COMPACTION_INIT_COLD), cold)]
pub unsafe extern "C" fn kcompactd_stop(nid: c_int) {
    let task = (*rust_compaction_NODE_DATA(nid)).kcompactd;
    if !task.is_null() {
        kthread_stop(task);
        (*rust_compaction_NODE_DATA(nid)).kcompactd = null_mut();
    }
}
#[cfg(CONFIG_SYSCTL)]
unsafe extern "C" fn proc_dointvec_minmax_warn_RT_change(
    table: *const ctl_table,
    write: c_int,
    buffer: *mut c_void,
    lenp: *mut usize,
    ppos: *mut loff_t,
) -> c_int {
    if !cfg!(CONFIG_PREEMPT_RT) || write == 0 {
        return proc_dointvec_minmax(table, write, buffer, lenp, ppos);
    }
    let old = *(*table).data.cast::<c_int>();
    let ret = proc_dointvec_minmax(table, write, buffer, lenp, ppos);
    if ret != 0 {
        return ret;
    }
    if old != *(*table).data.cast::<c_int>() {
        rust_compaction_warn_RT_change((*table).procname);
    }
    ret
}
// Immutable native ctl_table layout, with C's zero-initialized omitted fields.
// Pointer-valued SYSCTL_* macros are represented as addresses into their real
// native sysctl_vals object, using indices derived from those macros in C.
#[cfg(CONFIG_SYSCTL)]
struct CompactionSysctls([ctl_table; 4]);
#[cfg(CONFIG_SYSCTL)]
unsafe impl Sync for CompactionSysctls {}
#[cfg(CONFIG_SYSCTL)]
static vm_compaction: CompactionSysctls = CompactionSysctls([
    ctl_table {
        procname: b"compact_memory\0".as_ptr().cast(),
        data: addr_of_mut!(sysctl_compact_memory).cast(),
        maxlen: size_of::<c_int>() as c_int,
        mode: 0o200,
        proc_handler: Some(sysctl_compaction_handler),
        ..unsafe { zeroed() }
    },
    ctl_table {
        procname: b"compaction_proactiveness\0".as_ptr().cast(),
        data: addr_of_mut!(sysctl_compaction_proactiveness).cast(),
        maxlen: size_of::<c_uint>() as c_int,
        mode: 0o644,
        proc_handler: Some(compaction_proactiveness_sysctl_handler),
        extra1: unsafe {
            addr_of!(sysctl_vals)
                .cast::<c_int>()
                .wrapping_add(RUST_COMPACTION_SYSCTL_ZERO_INDEX as usize)
                .cast_mut()
                .cast()
        },
        extra2: unsafe {
            addr_of!(sysctl_vals)
                .cast::<c_int>()
                .wrapping_add(RUST_COMPACTION_SYSCTL_ONE_HUNDRED_INDEX as usize)
                .cast_mut()
                .cast()
        },
        ..unsafe { zeroed() }
    },
    ctl_table {
        procname: b"extfrag_threshold\0".as_ptr().cast(),
        data: addr_of_mut!(sysctl_extfrag_threshold).cast(),
        maxlen: size_of::<c_int>() as c_int,
        mode: 0o644,
        proc_handler: Some(proc_dointvec_minmax),
        extra1: unsafe {
            addr_of!(sysctl_vals)
                .cast::<c_int>()
                .wrapping_add(RUST_COMPACTION_SYSCTL_ZERO_INDEX as usize)
                .cast_mut()
                .cast()
        },
        extra2: unsafe {
            addr_of!(sysctl_vals)
                .cast::<c_int>()
                .wrapping_add(RUST_COMPACTION_SYSCTL_ONE_THOUSAND_INDEX as usize)
                .cast_mut()
                .cast()
        },
        ..unsafe { zeroed() }
    },
    ctl_table {
        procname: b"compact_unevictable_allowed\0".as_ptr().cast(),
        data: addr_of_mut!(sysctl_compact_unevictable_allowed).cast(),
        maxlen: size_of::<c_int>() as c_int,
        mode: 0o644,
        proc_handler: Some(proc_dointvec_minmax_warn_RT_change),
        extra1: unsafe {
            addr_of!(sysctl_vals)
                .cast::<c_int>()
                .wrapping_add(RUST_COMPACTION_SYSCTL_ZERO_INDEX as usize)
                .cast_mut()
                .cast()
        },
        extra2: unsafe {
            addr_of!(sysctl_vals)
                .cast::<c_int>()
                .wrapping_add(RUST_COMPACTION_SYSCTL_ONE_INDEX as usize)
                .cast_mut()
                .cast()
        },
        ..unsafe { zeroed() }
    },
]);
#[no_mangle]
#[link_section = ".init.text"]
#[cfg_attr(RUST_COMPACTION_INIT_COLD, cold)]
pub unsafe extern "C" fn rust_compaction_kcompactd_init() -> c_int {
    let mut nid = rust_compaction_first_memory_node();
    while nid < RUST_COMPACTION_MAX_NUMNODES {
        kcompactd_run(nid);
        nid = rust_compaction_next_memory_node(nid);
    }
    #[cfg(CONFIG_SYSCTL)]
    rust_compaction_register_sysctl_init(
        b"vm\0".as_ptr().cast(),
        vm_compaction.0.as_ptr(),
        vm_compaction.0.len(),
        b"vm_compaction\0".as_ptr().cast(),
    );
    0
}
// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
