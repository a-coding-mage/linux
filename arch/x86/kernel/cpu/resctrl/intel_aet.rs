// SPDX-License-Identifier: GPL-2.0-only
/*
 * Resource Director Technology(RDT)
 * - Intel Application Energy Telemetry
 *
 * Copyright (C) 2025 Intel Corporation
 *
 * Author:
 *    Tony Luck <tony.luck@intel.com>
 */

/* External Linux kernel declarations and macros are supplied by the surrounding crate. */

#[repr(C)]
pub struct pmt_event {
    pub id: resctrl_event_id,
    pub idx: u32,
    pub bin_bits: u32,
}

#[repr(C)]
pub struct event_group {
    pub pfname: *const core::ffi::c_char,
    pub pfg: *mut pmt_feature_group,
    pub force_off: bool,
    pub force_on: bool,
    pub guid: u32,
    pub num_rmid: u32,
    pub mmio_size: usize,
    pub num_events: u32,
    pub evts: *mut pmt_event,
}

static mut ENERGY_EVENTS: [pmt_event; 2] = [
    pmt_event { id: PMT_EVENT_ENERGY, idx: 0, bin_bits: 18 },
    pmt_event { id: PMT_EVENT_ACTIVITY, idx: 1, bin_bits: 18 },
];
static mut PERF_EVENTS: [pmt_event; 7] = [
    pmt_event { id: PMT_EVENT_STALLS_LLC_HIT, idx: 0, bin_bits: 0 },
    pmt_event { id: PMT_EVENT_C1_RES, idx: 1, bin_bits: 0 },
    pmt_event { id: PMT_EVENT_UNHALTED_CORE_CYCLES, idx: 2, bin_bits: 0 },
    pmt_event { id: PMT_EVENT_STALLS_LLC_MISS, idx: 3, bin_bits: 0 },
    pmt_event { id: PMT_EVENT_AUTO_C6_RES, idx: 4, bin_bits: 0 },
    pmt_event { id: PMT_EVENT_UNHALTED_REF_CYCLES, idx: 5, bin_bits: 0 },
    pmt_event { id: PMT_EVENT_UOPS_RETIRED, idx: 6, bin_bits: 0 },
];

static mut ENERGY_0X26696143: event_group = event_group {
    pfname: b"energy\0".as_ptr() as *const _, pfg: core::ptr::null_mut(), force_off: false,
    force_on: false, guid: 0x26696143, num_rmid: 576, mmio_size: (576 * 2 + 3) * 8,
    num_events: 2, evts: unsafe { ENERGY_EVENTS.as_mut_ptr() },
};
static mut PERF_0X26557651: event_group = event_group {
    pfname: b"perf\0".as_ptr() as *const _, pfg: core::ptr::null_mut(), force_off: false,
    force_on: false, guid: 0x26557651, num_rmid: 576, mmio_size: (576 * 7 + 3) * 8,
    num_events: 7, evts: unsafe { PERF_EVENTS.as_mut_ptr() },
};

static mut KNOWN_EVENT_GROUPS: [*mut event_group; 2] = [
    unsafe { &mut ENERGY_0X26696143 }, unsafe { &mut PERF_0X26557651 },
];

pub unsafe fn intel_handle_aet_option(force_off: bool, mut tok: *mut core::ffi::c_char) -> bool {
    if tok.is_null() { return false; }
    let name = strsep(&mut tok, b":".as_ptr() as *const _);
    let mut guid = 0u32;
    if !tok.is_null() && kstrtou32(tok, 16, &mut guid) != 0 { return false; }
    let mut ret = false;
    for peg in KNOWN_EVENT_GROUPS.iter() {
        let e = &mut **peg;
        if strcmp(name, e.pfname) != 0 || (guid != 0 && e.guid != guid) { continue; }
        if force_off { e.force_off = true; } else { e.force_on = true; }
        ret = true;
    }
    ret
}

unsafe fn skip_telem_region(tr: *mut telemetry_region, e: *mut event_group) -> bool {
    if (*tr).guid != (*e).guid { return true; }
    if (*tr).plat_info.package_id >= topology_max_packages() {
        pr_warn!("Bad package %u in guid 0x%x\n", (*tr).plat_info.package_id, (*tr).guid);
        return true;
    }
    if (*tr).size != (*e).mmio_size {
        pr_warn!("MMIO space wrong size (%zu bytes) for guid 0x%x. Expected %zu bytes.\n", (*tr).size, (*e).guid, (*e).mmio_size);
        return true;
    }
    false
}

unsafe fn group_has_usable_regions(e: *mut event_group, p: *mut pmt_feature_group) -> bool {
    let mut usable = false;
    for i in 0..(*p).count {
        let tr = (*p).regions.add(i as usize);
        if skip_telem_region(tr, e) { (*tr).addr = core::ptr::null_mut(); continue; }
        usable = true;
    }
    usable
}

unsafe fn all_regions_have_sufficient_rmid(e: *mut event_group, p: *mut pmt_feature_group) -> bool {
    for i in 0..(*p).count {
        let tr = (*p).regions.add(i as usize);
        if (*tr).addr.is_null() { continue; }
        if (*tr).num_rmids < (*e).num_rmid { (*e).force_off = true; return false; }
    }
    true
}

unsafe fn enable_events(e: *mut event_group, p: *mut pmt_feature_group) -> bool {
    let r = &mut rdt_resources_all[RDT_RESOURCE_PERF_PKG].r_resctrl;
    if (*e).force_off || !group_has_usable_regions(e, p) { return false; }
    if !all_regions_have_sufficient_rmid(e, p) && !(*e).force_on { return false; }
    for i in 0..(*p).count { let tr = (*p).regions.add(i as usize); if !(*tr).addr.is_null() { (*e).num_rmid = min((*e).num_rmid, (*tr).num_rmids); } }
    let mut skipped = 0;
    for j in 0..(*e).num_events { if !resctrl_enable_mon_event((*e).evts.add(j as usize).read().id, true, (*e).evts.add(j as usize).read().bin_bits, (*e).evts.add(j as usize)) { skipped += 1; } }
    if skipped == (*e).num_events { return false; }
    r.mon.num_rmid = if r.mon.num_rmid != 0 { min(r.mon.num_rmid, (*e).num_rmid) } else { (*e).num_rmid };
    true
}

pub unsafe fn intel_aet_get_events() -> bool {
    let mut ret = false;
    for peg in KNOWN_EVENT_GROUPS.iter() { let e = *peg; let p = intel_pmt_get_regions_by_feature(lookup_pfid((*e).pfname)); if !IS_ERR_OR_NULL(p) { if enable_events(e, p) { (*e).pfg = p; ret = true; } else { intel_pmt_put_feature_group(p); } } }
    ret
}

unsafe fn lookup_pfid(pfname: *const core::ffi::c_char) -> pmt_feature_id { if strcmp(pfname, b"energy\0".as_ptr() as *const _) == 0 { FEATURE_PER_RMID_ENERGY_TELEM } else if strcmp(pfname, b"perf\0".as_ptr() as *const _) == 0 { FEATURE_PER_RMID_PERF_TELEM } else { pr_warn!("Unknown PMT feature name\n"); FEATURE_INVALID } }

pub unsafe fn intel_aet_exit() { for peg in KNOWN_EVENT_GROUPS.iter() { let e = &mut **peg; if !e.pfg.is_null() { intel_pmt_put_feature_group(e.pfg); e.pfg = core::ptr::null_mut(); } } }

pub unsafe fn intel_aet_read_event(domid: i32, rmid: u32, arch_priv: *mut core::ffi::c_void, val: *mut u64) -> i32 {
    let pevt = arch_priv as *mut pmt_event;
    let e = container_of_event_group(pevt);
    let idx = rmid.wrapping_mul((*e).num_events).wrapping_add((*pevt).idx);
    if (idx as usize) * core::mem::size_of::<u64>() + core::mem::size_of::<u64>() > (*e).mmio_size { pr_warn_once!("MMIO index %u out of range\n", idx); return -EIO; }
    let mut total = 0u64;
    let mut valid = false;
    for i in 0..(*(*e).pfg).count {
        let tr = (*(*e).pfg).regions.add(i as usize);
        if (*tr).addr.is_null() || (*tr).plat_info.package_id != domid { continue; }
        let evtcount = readq((*tr).addr.add((idx as usize) * core::mem::size_of::<u64>()));
        if evtcount & (1u64 << 63) == 0 { continue; }
        total = total.wrapping_add(evtcount & ((1u64 << 63) - 1)); valid = true;
    }
    if valid { *val = total; 0 } else { -EINVAL }
}

unsafe fn container_of_event_group(pevt: *mut pmt_event) -> *mut event_group {
    let base = (pevt as *mut u8).sub((*pevt).idx as usize * core::mem::size_of::<pmt_event>());
    base.sub(core::mem::offset_of!(event_group, evts)) as *mut event_group
}

pub unsafe fn intel_aet_mon_domain_setup(cpu: i32, id: i32, r: *mut rdt_resource, add_pos: *mut list_head) {
    let d = kzalloc_node(core::mem::size_of::<rdt_perf_pkg_mon_domain>(), GFP_KERNEL, cpu_to_node(cpu));
    if d.is_null() { return; }
    (*d).hdr.id = id; (*d).hdr.type_ = RESCTRL_MON_DOMAIN; (*d).hdr.rid = RDT_RESOURCE_PERF_PKG;
    cpumask_set_cpu(cpu, &mut (*d).hdr.cpu_mask);
    let err = resctrl_online_mon_domain(r, &mut (*d).hdr);
    if err != 0 { kfree(d as *mut core::ffi::c_void); return; }
    list_add_tail_rcu(&mut (*d).hdr.list, add_pos);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
