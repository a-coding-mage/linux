// SPDX-License-Identifier: GPL-2.0
/*
 * Description: PMUs specific to running nested KVM-HV guests
 * on Book3S processors (specifically POWER9 and later).
 */

// Kernel headers and symbols referenced by this translation are supplied by
// the surrounding kernel environment.

#[repr(u32)]
enum KvmppcPmuEventid {
    KVMPPC_EVENT_HOST_HEAP,
    KVMPPC_EVENT_HOST_HEAP_MAX,
    KVMPPC_EVENT_HOST_PGTABLE,
    KVMPPC_EVENT_HOST_PGTABLE_MAX,
    KVMPPC_EVENT_HOST_PGTABLE_RECLAIM,
    KVMPPC_EVENT_MAX,
}

// PMU_EVENT_ATTR_ID(host_heap, kvmppc_events_sysfs_show, KVMPPC_EVENT_HOST_HEAP)
// PMU_EVENT_ATTR_ID(host_heap_max, kvmppc_events_sysfs_show, KVMPPC_EVENT_HOST_HEAP_MAX)
// PMU_EVENT_ATTR_ID(host_pagetable, kvmppc_events_sysfs_show, KVMPPC_EVENT_HOST_PGTABLE)
// PMU_EVENT_ATTR_ID(host_pagetable_max, kvmppc_events_sysfs_show, KVMPPC_EVENT_HOST_PGTABLE_MAX)
// PMU_EVENT_ATTR_ID(host_pagetable_reclaim, kvmppc_events_sysfs_show, KVMPPC_EVENT_HOST_PGTABLE_RECLAIM)

unsafe fn kvmppc_events_sysfs_show(
    dev: *mut device,
    attr: *mut device_attribute,
    page: *mut core::ffi::c_char,
) -> isize {
    let pmu_attr: *mut perf_pmu_events_attr = container_of!(attr, perf_pmu_events_attr, attr);
    sysfs_emit!(page, "event=0x{:02llx}\n", (*pmu_attr).id)
}

/* Holds the hostwide stats */
#[repr(C)]
struct KvmppcHostwideStats {
    guest_heap: u64,
    guest_heap_max: u64,
    guest_pgtable_size: u64,
    guest_pgtable_size_max: u64,
    guest_pgtable_reclaim: u64,
}

static mut l0_stats: KvmppcHostwideStats = KvmppcHostwideStats {
    guest_heap: 0,
    guest_heap_max: 0,
    guest_pgtable_size: 0,
    guest_pgtable_size_max: 0,
    guest_pgtable_reclaim: 0,
};

/* Protect access to l0_stats */
static mut lock_l0_stats: spinlock_t = DEFINE_SPINLOCK!();

/* GSB related structs needed to talk to L0 */
static mut gsm_l0_stats: *mut kvmppc_gs_msg = core::ptr::null_mut();
static mut gsb_l0_stats: *mut kvmppc_gs_buff = core::ptr::null_mut();
static mut gsp_l0_stats: kvmppc_gs_parser = core::mem::zeroed();

static mut kvmppc_pmu_events_attr: [*mut attribute; 6] = [
    pmu_event_attr_id!(host_heap, kvmppc_events_sysfs_show, KVMPPC_EVENT_HOST_HEAP),
    pmu_event_attr_id!(host_heap_max, kvmppc_events_sysfs_show, KVMPPC_EVENT_HOST_HEAP_MAX),
    pmu_event_attr_id!(host_pagetable, kvmppc_events_sysfs_show, KVMPPC_EVENT_HOST_PGTABLE),
    pmu_event_attr_id!(host_pagetable_max, kvmppc_events_sysfs_show, KVMPPC_EVENT_HOST_PGTABLE_MAX),
    pmu_event_attr_id!(host_pagetable_reclaim, kvmppc_events_sysfs_show, KVMPPC_EVENT_HOST_PGTABLE_RECLAIM),
    core::ptr::null_mut(),
];

static kvmppc_pmu_events_group: attribute_group = attribute_group {
    name: c"events".as_ptr(),
    attrs: unsafe { kvmppc_pmu_events_attr.as_mut_ptr() },
};

// PMU_FORMAT_ATTR(event, "config:0-5");
static mut kvmppc_pmu_format_attr: [*mut attribute; 2] = [
    &mut format_attr_event.attr,
    core::ptr::null_mut(),
];

static mut kvmppc_pmu_format_group: attribute_group = attribute_group {
    name: c"format".as_ptr(),
    attrs: kvmppc_pmu_format_attr.as_mut_ptr(),
};

static mut kvmppc_pmu_attr_groups: [*const attribute_group; 3] = [
    &kvmppc_pmu_events_group,
    &kvmppc_pmu_format_group,
    core::ptr::null(),
];

/*
 * Issue the hcall to get the L0-host stats.
 * Should be called with l0-stat lock held
 */
unsafe fn kvmppc_update_l0_stats() -> i32 {
    /* With HOST_WIDE flags guestid and vcpuid will be ignored */
    let mut rc = kvmppc_gsb_recv(gsb_l0_stats, KVMPPC_GS_FLAGS_HOST_WIDE);
    if rc != 0 { return rc; }

    /* Parse the guest state buffer is successful */
    rc = kvmppc_gse_parse(&mut gsp_l0_stats, gsb_l0_stats);
    if rc != 0 { return rc; }

    /* Update the l0 returned stats*/
    core::ptr::write_bytes(&mut l0_stats, 0, 1);
    rc = kvmppc_gsm_refresh_info(gsm_l0_stats, gsb_l0_stats);
    rc
}

/* Update the value of the given perf_event */
unsafe fn kvmppc_pmu_event_update(event: *mut perf_event) -> i32 {
    let mut rc: i32;
    let mut curr_val: u64 = 0;
    let prev_val: u64;
    let mut flags: ulong = 0;
    let config = (*event).attr.config as u32;

    /* Ensure no one else is modifying the l0_stats */
    spin_lock_irqsave(&mut lock_l0_stats, &mut flags);

    rc = kvmppc_update_l0_stats();
    if rc == 0 {
        match config {
            x if x == KvmppcPmuEventid::KVMPPC_EVENT_HOST_HEAP as u32 => curr_val = l0_stats.guest_heap,
            x if x == KvmppcPmuEventid::KVMPPC_EVENT_HOST_HEAP_MAX as u32 => curr_val = l0_stats.guest_heap_max,
            x if x == KvmppcPmuEventid::KVMPPC_EVENT_HOST_PGTABLE as u32 => curr_val = l0_stats.guest_pgtable_size,
            x if x == KvmppcPmuEventid::KVMPPC_EVENT_HOST_PGTABLE_MAX as u32 => curr_val = l0_stats.guest_pgtable_size_max,
            x if x == KvmppcPmuEventid::KVMPPC_EVENT_HOST_PGTABLE_RECLAIM as u32 => curr_val = l0_stats.guest_pgtable_reclaim,
            _ => rc = -ENOENT,
        }
    }

    spin_unlock_irqrestore(&mut lock_l0_stats, flags);

    /* If no error than update the perf event */
    if rc == 0 {
        prev_val = local64_xchg(&mut (*event).hw.prev_count, curr_val);
        if curr_val > prev_val { local64_add(curr_val - prev_val, &mut (*event).count); }
    }
    rc
}

unsafe fn kvmppc_pmu_event_init(event: *mut perf_event) -> i32 {
    let config = (*event).attr.config as u32;
    pr_debug!("{}: Event({:p}) id={} cpu={} on_cpu={} config={}", __func__, event, (*event).id, (*event).cpu, (*event).oncpu, config);
    if (*event).attr.type_ != (*event).pmu.type_ { return -ENOENT; }
    if config >= KvmppcPmuEventid::KVMPPC_EVENT_MAX as u32 { return -EINVAL; }
    local64_set(&mut (*event).hw.prev_count, 0);
    local64_set(&mut (*event).count, 0);
    0
}

unsafe fn kvmppc_pmu_del(event: *mut perf_event, _flags: i32) { kvmppc_pmu_event_update(event); }

unsafe fn kvmppc_pmu_add(event: *mut perf_event, flags: i32) -> i32 {
    if flags & PERF_EF_START != 0 { return kvmppc_pmu_event_update(event); }
    0
}

unsafe fn kvmppc_pmu_read(event: *mut perf_event) { kvmppc_pmu_event_update(event); }

/* Return the size of the needed guest state buffer */
unsafe fn hostwide_get_size(_gsm: *mut kvmppc_gs_msg) -> usize {
    let mut size = 0usize;
    let ids: [u16; 5] = [KVMPPC_GSID_L0_GUEST_HEAP, KVMPPC_GSID_L0_GUEST_HEAP_MAX, KVMPPC_GSID_L0_GUEST_PGTABLE_SIZE, KVMPPC_GSID_L0_GUEST_PGTABLE_SIZE_MAX, KVMPPC_GSID_L0_GUEST_PGTABLE_RECLAIM];
    for i in 0..ids.len() { size += kvmppc_gse_total_size(kvmppc_gsid_size(ids[i])); }
    size
}

/* Populate the request guest state buffer */
unsafe fn hostwide_fill_info(gsb: *mut kvmppc_gs_buff, gsm: *mut kvmppc_gs_msg) -> i32 {
    let mut rc = 0;
    let stats = (*gsm).data as *mut KvmppcHostwideStats;
    if kvmppc_gsm_includes(gsm, KVMPPC_GSID_L0_GUEST_HEAP) != 0 { rc = kvmppc_gse_put_u64(gsb, KVMPPC_GSID_L0_GUEST_HEAP, (*stats).guest_heap); }
    if rc == 0 && kvmppc_gsm_includes(gsm, KVMPPC_GSID_L0_GUEST_HEAP_MAX) != 0 { rc = kvmppc_gse_put_u64(gsb, KVMPPC_GSID_L0_GUEST_HEAP_MAX, (*stats).guest_heap_max); }
    if rc == 0 && kvmppc_gsm_includes(gsm, KVMPPC_GSID_L0_GUEST_PGTABLE_SIZE) != 0 { rc = kvmppc_gse_put_u64(gsb, KVMPPC_GSID_L0_GUEST_PGTABLE_SIZE, (*stats).guest_pgtable_size); }
    if rc == 0 && kvmppc_gsm_includes(gsm, KVMPPC_GSID_L0_GUEST_PGTABLE_SIZE_MAX) != 0 { rc = kvmppc_gse_put_u64(gsb, KVMPPC_GSID_L0_GUEST_PGTABLE_SIZE_MAX, (*stats).guest_pgtable_size_max); }
    if rc == 0 && kvmppc_gsm_includes(gsm, KVMPPC_GSID_L0_GUEST_PGTABLE_RECLAIM) != 0 { rc = kvmppc_gse_put_u64(gsb, KVMPPC_GSID_L0_GUEST_PGTABLE_RECLAIM, (*stats).guest_pgtable_reclaim); }
    rc
}

/* Parse and update the host wide stats from returned gsb */
unsafe fn hostwide_refresh_info(gsm: *mut kvmppc_gs_msg, gsb: *mut kvmppc_gs_buff) -> i32 {
    let mut gsp: kvmppc_gs_parser = core::mem::zeroed();
    let stats = (*gsm).data as *mut KvmppcHostwideStats;
    let mut rc = kvmppc_gse_parse(&mut gsp, gsb);
    if rc < 0 { return rc; }
    let mut gse = kvmppc_gsp_lookup(&mut gsp, KVMPPC_GSID_L0_GUEST_HEAP); if !gse.is_null() { (*stats).guest_heap = kvmppc_gse_get_u64(gse); }
    gse = kvmppc_gsp_lookup(&mut gsp, KVMPPC_GSID_L0_GUEST_HEAP_MAX); if !gse.is_null() { (*stats).guest_heap_max = kvmppc_gse_get_u64(gse); }
    gse = kvmppc_gsp_lookup(&mut gsp, KVMPPC_GSID_L0_GUEST_PGTABLE_SIZE); if !gse.is_null() { (*stats).guest_pgtable_size = kvmppc_gse_get_u64(gse); }
    gse = kvmppc_gsp_lookup(&mut gsp, KVMPPC_GSID_L0_GUEST_PGTABLE_SIZE_MAX); if !gse.is_null() { (*stats).guest_pgtable_size_max = kvmppc_gse_get_u64(gse); }
    gse = kvmppc_gsp_lookup(&mut gsp, KVMPPC_GSID_L0_GUEST_PGTABLE_RECLAIM); if !gse.is_null() { (*stats).guest_pgtable_reclaim = kvmppc_gse_get_u64(gse); }
    rc = 0; rc
}

/* gsb-message ops for setting up/parsing */
static mut gsb_ops_l0_stats: kvmppc_gs_msg_ops = kvmppc_gs_msg_ops { get_size: hostwide_get_size, fill_info: hostwide_fill_info, refresh_info: hostwide_refresh_info };

unsafe fn kvmppc_init_hostwide() -> i32 {
    let mut rc = 0; let mut flags: ulong = 0;
    spin_lock_irqsave(&mut lock_l0_stats, &mut flags);
    if !gsm_l0_stats.is_null() { spin_unlock_irqrestore(&mut lock_l0_stats, flags); return 0; }
    gsm_l0_stats = kvmppc_gsm_new(&mut gsb_ops_l0_stats, &mut l0_stats, GSM_SEND, GFP_KERNEL);
    if gsm_l0_stats.is_null() { rc = -ENOMEM; }
    if rc == 0 {
        kvmppc_gsm_include(gsm_l0_stats, KVMPPC_GSID_L0_GUEST_HEAP); kvmppc_gsm_include(gsm_l0_stats, KVMPPC_GSID_L0_GUEST_HEAP_MAX); kvmppc_gsm_include(gsm_l0_stats, KVMPPC_GSID_L0_GUEST_PGTABLE_SIZE); kvmppc_gsm_include(gsm_l0_stats, KVMPPC_GSID_L0_GUEST_PGTABLE_SIZE_MAX); kvmppc_gsm_include(gsm_l0_stats, KVMPPC_GSID_L0_GUEST_PGTABLE_RECLAIM);
        gsb_l0_stats = kvmppc_gsb_new(kvmppc_gsm_size(gsm_l0_stats), 0, 0, GFP_KERNEL);
        if gsb_l0_stats.is_null() { rc = -ENOMEM; } else { rc = kvmppc_gsm_fill_info(gsm_l0_stats, gsb_l0_stats); }
    }
    if rc != 0 { if !gsm_l0_stats.is_null() { kvmppc_gsm_free(gsm_l0_stats); } if !gsb_l0_stats.is_null() { kvmppc_gsb_free(gsb_l0_stats); } gsm_l0_stats = core::ptr::null_mut(); gsb_l0_stats = core::ptr::null_mut(); }
    spin_unlock_irqrestore(&mut lock_l0_stats, flags); rc
}

unsafe fn kvmppc_cleanup_hostwide() {
    let mut flags: ulong = 0; spin_lock_irqsave(&mut lock_l0_stats, &mut flags);
    if !gsm_l0_stats.is_null() { kvmppc_gsm_free(gsm_l0_stats); } if !gsb_l0_stats.is_null() { kvmppc_gsb_free(gsb_l0_stats); }
    gsm_l0_stats = core::ptr::null_mut(); gsb_l0_stats = core::ptr::null_mut(); spin_unlock_irqrestore(&mut lock_l0_stats, flags);
}

/* L1 wide counters PMU */
static mut kvmppc_pmu: pmu = pmu {
    module: THIS_MODULE, task_ctx_nr: perf_sw_context, name: c"kvm-hv".as_ptr(), event_init: kvmppc_pmu_event_init, add: kvmppc_pmu_add, del: kvmppc_pmu_del, read: kvmppc_pmu_read, attr_groups: kvmppc_pmu_attr_groups.as_ptr(), type_: -1, scope: PERF_PMU_SCOPE_SYS_WIDE, capabilities: PERF_PMU_CAP_NO_EXCLUDE | PERF_PMU_CAP_NO_INTERRUPT,
};

unsafe fn kvmppc_register_pmu() -> i32 {
    let mut rc = -EOPNOTSUPP;
    /* only support events for nestedv2 right now */
    if kvmhv_is_nestedv2() != 0 { rc = kvmppc_init_hostwide(); if rc != 0 { return rc; } rc = perf_pmu_register(&mut kvmppc_pmu, kvmppc_pmu.name, -1); if rc != 0 { return rc; } pr_info!("Registered kvm-hv pmu"); }
    rc
}

unsafe fn kvmppc_unregister_pmu() {
    if kvmhv_is_nestedv2() != 0 { kvmppc_cleanup_hostwide(); if kvmppc_pmu.type_ != -1 { perf_pmu_unregister(&mut kvmppc_pmu); } pr_info!("kvmhv_pmu unregistered.\n"); }
}

// module_init(kvmppc_register_pmu);
// module_exit(kvmppc_unregister_pmu);
// MODULE_DESCRIPTION("KVM PPC Book3s-hv PMU");
// MODULE_AUTHOR("Vaibhav Jain <vaibhav@linux.ibm.com>");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
