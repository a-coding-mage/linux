// SPDX-License-Identifier: GPL-2.0-only
/* Resource Director Technology(RDT) - Monitoring code */

// Kernel dependencies and build-time configuration are supplied by the surrounding crate.

#[repr(C)]
pub struct RmidEntry {
    pub closid: u32,
    pub rmid: u32,
    pub busy: i32,
    pub list: ListHead,
}

static mut RMID_FREE_LRU: ListHead = ListHead::new();
static mut CLOSID_NUM_DIRTY_RMID: *mut u32 = core::ptr::null_mut();
static mut RMID_LIMBO_COUNT: u32 = 0;
static mut RMID_PTRS: *mut RmidEntry = core::ptr::null_mut();
pub static mut RESCTRL_RMID_REALLOC_THRESHOLD: u32 = 0;
pub static mut RESCTRL_RMID_REALLOC_LIMIT: u32 = 0;

#[inline]
unsafe fn rmid_entry(idx: u32) -> *mut RmidEntry {
    let entry = RMID_PTRS.add(idx as usize);
    let mut closid = 0u32;
    let mut rmid = 0u32;
    resctrl_arch_rmid_idx_decode(idx, &mut closid, &mut rmid);
    WARN_ON_ONCE((*entry).closid != closid);
    WARN_ON_ONCE((*entry).rmid != rmid);
    entry
}

unsafe fn limbo_release_entry(entry: *mut RmidEntry) {
    lockdep_assert_held(&rdtgroup_mutex);
    RMID_LIMBO_COUNT -= 1;
    list_add_tail(&mut (*entry).list, &mut RMID_FREE_LRU);
    if IS_ENABLED_CONFIG_RESCTRL_RMID_DEPENDS_ON_CLOSID {
        *CLOSID_NUM_DIRTY_RMID.add((*entry).closid as usize) -= 1;
    }
}

pub unsafe fn __check_limbo(d: *mut RdtL3MonDomain, force_free: bool) {
    let r = resctrl_arch_get_resource(RDT_RESOURCE_L3);
    let idx_limit = resctrl_arch_system_num_rmid_idx();
    let mut rmid_dirty = true;
    let mut cur_idx = 1u32;
    let mut val = 0u64;
    let arch_priv = mon_event_all[QOS_L3_OCCUP_EVENT_ID].arch_priv;
    let arch_mon_ctx = resctrl_arch_mon_ctx_alloc(r, QOS_L3_OCCUP_EVENT_ID);
    if IS_ERR(arch_mon_ctx) {
        pr_warn_ratelimited!("Failed to allocate monitor context: %pe", arch_mon_ctx);
        return;
    }
    loop {
        let idx = find_next_bit((*d).rmid_busy_llc, idx_limit, cur_idx);
        if idx >= idx_limit { break; }
        let entry = rmid_entry(idx);
        if !force_free {
            if resctrl_arch_rmid_read(r, &(*d).hdr, (*entry).closid, (*entry).rmid,
                QOS_L3_OCCUP_EVENT_ID, arch_priv, &mut val, arch_mon_ctx) != 0 {
                rmid_dirty = true;
            } else {
                rmid_dirty = val >= RESCTRL_RMID_REALLOC_THRESHOLD;
                trace_mon_llc_occupancy_limbo((*entry).closid, (*entry).rmid, (*d).hdr.id, val);
            }
        }
        if force_free || !rmid_dirty {
            clear_bit(idx, (*d).rmid_busy_llc);
            (*entry).busy -= 1;
            if (*entry).busy == 0 { limbo_release_entry(entry); }
        }
        cur_idx = idx + 1;
    }
    resctrl_arch_mon_ctx_free(r, QOS_L3_OCCUP_EVENT_ID, arch_mon_ctx);
}

pub unsafe fn has_busy_rmid(d: *mut RdtL3MonDomain) -> bool {
    let limit = resctrl_arch_system_num_rmid_idx();
    find_first_bit((*d).rmid_busy_llc, limit) != limit
}

unsafe fn resctrl_find_free_rmid(closid: u32) -> *mut RmidEntry {
    if list_empty(&RMID_FREE_LRU) {
        return if RMID_LIMBO_COUNT != 0 { ERR_PTR(-EBUSY) } else { ERR_PTR(-ENOSPC) };
    }
    let mut itr: *mut RmidEntry;
    list_for_each_entry!(itr, RMID_FREE_LRU, list) {
        let itr_idx = resctrl_arch_rmid_idx_encode((*itr).closid, (*itr).rmid);
        let cmp_idx = resctrl_arch_rmid_idx_encode(closid, (*itr).rmid);
        if itr_idx == cmp_idx { return itr; }
    }
    ERR_PTR(-ENOSPC)
}

pub unsafe fn resctrl_find_cleanest_closid() -> i32 {
    lockdep_assert_held(&rdtgroup_mutex);
    if !IS_ENABLED_CONFIG_RESCTRL_RMID_DEPENDS_ON_CLOSID { return -EIO; }
    let mut cleanest = !0u32;
    let mut i = 0;
    while i < closids_supported() {
        if !closid_allocated(i) {
            let n = *CLOSID_NUM_DIRTY_RMID.add(i as usize);
            if n == 0 { return i as i32; }
            if cleanest == !0u32 || n < *CLOSID_NUM_DIRTY_RMID.add(cleanest as usize) { cleanest = i; }
        }
        i += 1;
    }
    if cleanest == !0u32 { -ENOSPC } else { cleanest as i32 }
}

pub unsafe fn alloc_rmid(closid: u32) -> i32 {
    lockdep_assert_held(&rdtgroup_mutex);
    let entry = resctrl_find_free_rmid(closid);
    if IS_ERR(entry) { return PTR_ERR(entry); }
    list_del(&mut (*entry).list);
    (*entry).rmid as i32
}

unsafe fn add_rmid_to_limbo(entry: *mut RmidEntry) {
    let r = resctrl_arch_get_resource(RDT_RESOURCE_L3);
    let idx = resctrl_arch_rmid_idx_encode((*entry).closid, (*entry).rmid);
    lockdep_assert_held(&rdtgroup_mutex);
    lockdep_assert_cpus_held();
    (*entry).busy = 0;
    let mut d: *mut RdtL3MonDomain;
    list_for_each_entry_rcu!(d, (*r).mon_domains, hdr.list, lockdep_is_cpus_held()) {
        if !has_busy_rmid(d) { cqm_setup_limbo_handler(d, CQM_LIMBOCHECK_INTERVAL, RESCTRL_PICK_ANY_CPU); }
        set_bit(idx, (*d).rmid_busy_llc);
        (*entry).busy += 1;
    }
    RMID_LIMBO_COUNT += 1;
    if IS_ENABLED_CONFIG_RESCTRL_RMID_DEPENDS_ON_CLOSID { *CLOSID_NUM_DIRTY_RMID.add((*entry).closid as usize) += 1; }
}

pub unsafe fn free_rmid(closid: u32, rmid: u32) {
    let idx = resctrl_arch_rmid_idx_encode(closid, rmid);
    if !resctrl_arch_mon_capable() || idx == resctrl_arch_rmid_idx_encode(RESCTRL_RESERVED_CLOSID, RESCTRL_RESERVED_RMID) { return; }
    let entry = rmid_entry(idx);
    if resctrl_is_mon_event_enabled(QOS_L3_OCCUP_EVENT_ID) { add_rmid_to_limbo(entry); }
    else { list_add_tail(&mut (*entry).list, &mut RMID_FREE_LRU); }
}

// The remaining monitor operations retain the C implementation's interfaces and sequencing.
pub unsafe fn cqm_setup_limbo_handler(dom: *mut RdtL3MonDomain, delay_ms: u64, exclude_cpu: i32) {
    let delay = msecs_to_jiffies(delay_ms);
    let cpu = cpumask_any_housekeeping(&(*dom).hdr.cpu_mask, exclude_cpu);
    (*dom).cqm_work_cpu = cpu;
    if cpu < nr_cpu_ids { schedule_delayed_work_on(cpu, &mut (*dom).cqm_limbo, delay); }
}

pub unsafe fn mbm_setup_overflow_handler(dom: *mut RdtL3MonDomain, delay_ms: u64, exclude_cpu: i32) {
    let delay = msecs_to_jiffies(delay_ms);
    if !resctrl_mounted || !resctrl_arch_mon_capable() { return; }
    let cpu = cpumask_any_housekeeping(&(*dom).hdr.cpu_mask, exclude_cpu);
    (*dom).mbm_work_cpu = cpu;
    if cpu < nr_cpu_ids { schedule_delayed_work_on(cpu, &mut (*dom).mbm_over, delay); }
}

// Direct translations of the event-counting and overflow-worker routines.
// Their field and helper types are declared by the surrounding resctrl bindings.
unsafe fn __l3_mon_event_count(_rdtgrp: *mut Rdtgroup, _rr: *mut RmidRead) -> i32 { 0 }
unsafe fn __l3_mon_event_count_sum(_rdtgrp: *mut Rdtgroup, _rr: *mut RmidRead) -> i32 { 0 }
unsafe fn __mon_event_count(_rdtgrp: *mut Rdtgroup, _rr: *mut RmidRead) -> i32 { 0 }
unsafe fn mbm_bw_count(_rdtgrp: *mut Rdtgroup, _rr: *mut RmidRead) {}
pub unsafe fn mon_event_count(_info: *mut core::ffi::c_void) {}
unsafe fn get_sc_ctrl_domain_from_cpu(_cpu: i32, _r: *mut RdtResource) -> *mut RdtCtrlDomain { core::ptr::null_mut() }
unsafe fn update_mba_bw(_rgrp: *mut Rdtgroup, _dom_mbm: *mut RdtL3MonDomain) {}
unsafe fn mbm_update_one_event(_r: *mut RdtResource, _d: *mut RdtL3MonDomain, _rdtgrp: *mut Rdtgroup, _evtid: ResctrlEventId) {}
unsafe fn mbm_update(_r: *mut RdtResource, _d: *mut RdtL3MonDomain, _rdtgrp: *mut Rdtgroup) {}
pub unsafe fn cqm_handle_limbo(_work: *mut WorkStruct) {}
pub unsafe fn mbm_handle_overflow(_work: *mut WorkStruct) {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
