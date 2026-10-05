// SPDX-License-Identifier: GPL-2.0
// Continued cache family, topology.c:849–1052 at 126a30fae3bba11420ec2fcbde51a0a01bab1b5b.
#[cfg(CONFIG_SCHED_CACHE)]
mod topology_cache {
    use super::*;
    unsafe fn get_effective_llc_bytes(cpu: c_int, sd: *mut sched_domain) -> c_ulong {
        // SAFETY: The caller keeps sd live and CPU hotplug serialized, which
        // stabilizes the native cacheinfo object and its shared-CPU mask.
        unsafe {
            let ci = lupos_topology_cache_info(cpu);
            if ci.is_null() { return 0; }
            let hw_weight = lupos_topology_mask_weight(&raw const (*ci).shared_cpu_map);
            if hw_weight == 0 { return 0; }
            lupos_topology_cache_div_u64(((*ci).size as u64).wrapping_mul((*sd).span_weight as u64), hw_weight) as c_ulong
        }
    }
    pub(super) unsafe fn alloc_sd_llc(cpu_map: *const cpumask, d: *mut s_data) -> bool {
        // SAFETY: The build path owns initialized per-CPU domain slots and the
        // live mask. Only those domains receive or release count allocations;
        // the error path clears every released pointer and associated metadata.
        unsafe {
            'err: {
                let mut i = lupos_topology_mask_first(cpu_map);
                while i < lupos_topology_mask_bound() {
                    let sd = lupos_topology_data_sd(d, i as c_int);
                    if sd.is_null() { break 'err; }
                    let p = lupos_topology_cache_counts_alloc(i as c_int);
                    if p.is_null() { break 'err; }
                    let mut top_llc = sd;
                    loop {
                        let parent = lupos_topology_cache_parent(top_llc);
                        if parent.is_null() || (*parent).flags & SD_SHARE_LLC == 0 { break; }
                        top_llc = parent;
                    }
                    if (*top_llc).flags & SD_SHARE_LLC != 0 {
                        (*sd).llc_max = max_lid.wrapping_add(1) as c_uint;
                        (*sd).llc_counts = p;
                        (*sd).llc_bytes = get_effective_llc_bytes(i as c_int, top_llc);
                    } else { lupos_topology_free(p.cast()); }
                    i = lupos_topology_mask_next(i as c_int, cpu_map);
                }
                return true;
            }
            let mut i = lupos_topology_mask_first(cpu_map);
            while i < lupos_topology_mask_bound() {
                let sd = lupos_topology_data_sd(d, i as c_int);
                if !sd.is_null() {
                    lupos_topology_free((*sd).llc_counts.cast());
                    (*sd).llc_counts = null_mut();
                    (*sd).llc_max = 0;
                    (*sd).llc_bytes = 0;
                }
                i = lupos_topology_mask_next(i as c_int, cpu_map);
            }
            false
        }
    }
    unsafe fn _sched_cache_active_set() {
        // SAFETY: Callers hold the domain mutex and stabilize CPU hotplug with
        // its lock or early-boot serialization, as in the native scheduler path.
        unsafe {
            lupos_topology_cache_assert_cpus_held();
            lupos_topology_assert_domains_locked();
            if !lupos_topology_cache_present() {
                lupos_topology_cache_active_disable();
                if sched_debug() { lupos_topology_cache_debug_unsupported(); }
                return;
            }
            if sysctl_sched_cache_user != 0 {
                lupos_topology_cache_active_enable();
                if sched_debug() { lupos_topology_cache_debug_enable(); }
            } else {
                lupos_topology_cache_active_disable();
                if sched_debug() { lupos_topology_cache_debug_disable(); }
            }
        }
    }
    /// Applies the requested cache-aware scheduling setting.
    ///
    /// # Safety
    /// Call in sleepable context without holding the CPU-hotplug write lock or
    /// scheduler-domain mutex acquired by this operation.
    #[no_mangle]
    pub unsafe extern "C" fn sched_cache_active_set() {
        // SAFETY: The caller permits sleeping; native hotplug locking precedes
        // the domain mutex and both remain held while static keys are changed.
        unsafe {
            lupos_topology_cache_cpus_read_lock();
            sched_domains_mutex_lock();
            _sched_cache_active_set();
            sched_domains_mutex_unlock();
            lupos_topology_cache_cpus_read_unlock();
        }
    }
    /// Refreshes effective LLC bytes for a CPU and its LLC siblings.
    ///
    /// # Safety
    /// `cpu` must identify an initialized runqueue. The caller must hold the CPU
    /// hotplug lock and permit acquiring the scheduler-domain mutex.
    #[no_mangle]
    pub unsafe extern "C" fn sched_update_llc_bytes(cpu: c_uint) {
        // SAFETY: Hotplug and the acquired domain mutex stabilize cacheinfo and
        // the RCU-protected domain chain while sibling byte counts are updated.
        unsafe {
            sched_domains_mutex_lock();
            let sdp = lupos_topology_cache_sd_llc_dereference(cpu);
            if !sdp.is_null() {
                let mask = lupos_topology_domain_span(sdp);
                let mut i = lupos_topology_mask_first(mask);
                while i < lupos_topology_mask_bound() {
                    let sd = lupos_topology_cache_rq_sd_dereference(i);
                    if !sd.is_null() { (*sd).llc_bytes = get_effective_llc_bytes(i as c_int, sdp); }
                    i = lupos_topology_mask_next(i as c_int, mask);
                }
            }
            sched_domains_mutex_unlock();
        }
    }
    pub(super) unsafe fn sched_cache_set(has_multi_llcs: bool) {
        // SAFETY: Partition callers hold hotplug/domain synchronization. The
        // boot caller holds the domain mutex before userspace can cause hotplug.
        unsafe {
            if has_multi_llcs { lupos_topology_cache_present_enable(); }
            else { lupos_topology_cache_present_disable(); }
            _sched_cache_active_set();
        }
    }
}
#[cfg(CONFIG_SCHED_CACHE)]
use topology_cache::{alloc_sd_llc, sched_cache_set};
#[cfg(not(CONFIG_SCHED_CACHE))]
unsafe fn alloc_sd_llc(_cpu_map: *const cpumask, _d: *mut s_data) -> bool { false }
#[cfg(not(CONFIG_SCHED_CACHE))]
unsafe fn sched_cache_set(_has_multi_llcs: bool) { /* Exact native !CACHE branch. */ }
