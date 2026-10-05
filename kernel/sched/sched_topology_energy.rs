// SPDX-License-Identifier: GPL-2.0
// Existing EAS family, topology.c:211–447, continued against 126a30fae3bba11420ec2fcbde51a0a01bab1b5b.
#[cfg(all(CONFIG_ENERGY_MODEL, CONFIG_CPU_FREQ_GOV_SCHEDUTIL))]
mod topology_energy {
    use super::*;

    unsafe fn sched_is_eas_possible(cpu_mask: *const cpumask) -> bool {
        // SAFETY: The caller supplies a live CPU mask. Native accessors retain
        // the oracle's access-only RCU observation and scheduler feature checks.
        unsafe {
            let mut any_asym_capacity = false;
            let mut i = lupos_topology_mask_first(cpu_mask);
            while i < lupos_topology_mask_bound() {
                if !lupos_topology_energy_asym_access(i as c_int).is_null() {
                    any_asym_capacity = true;
                    break;
                }
                i = lupos_topology_mask_next(i as c_int, cpu_mask);
            }
            if !any_asym_capacity {
                if sched_debug() { lupos_topology_energy_debug_asym(cpu_mask); }
                return false;
            }
            if lupos_topology_energy_smt_active() {
                if sched_debug() { lupos_topology_energy_debug_smt(cpu_mask); }
                return false;
            }
            if !lupos_topology_energy_freq_invariant() {
                if sched_debug() { lupos_topology_energy_debug_freq(cpu_mask); }
                return false;
            }
            if !lupos_topology_energy_cpufreq_ready(cpu_mask) {
                if sched_debug() { lupos_topology_energy_debug_cpufreq(cpu_mask); }
                return false;
            }
            true
        }
    }

    /// Rebuilds domains while serializing the EAS update request.
    ///
    /// # Safety
    /// Call in sleepable context without holding locks taken by a domain rebuild.
    #[no_mangle]
    pub unsafe extern "C" fn rebuild_sched_domains_energy() {
        // SAFETY: The caller allows sleeping; the native mutex encloses the
        // update flag and the configured cpuset/hotplug rebuild implementation.
        unsafe {
            lupos_topology_energy_lock();
            lupos_topology_energy_set_update(true);
            lupos_topology_energy_rebuild_domains();
            lupos_topology_energy_set_update(false);
            lupos_topology_energy_unlock();
        }
    }

    /// Handles the native energy-aware scheduler sysctl.
    ///
    /// # Safety
    /// Arguments must satisfy the native `proc_handler` contract: live table,
    /// buffer, length and position objects, with access valid for this request.
    /// The caller must permit sleeping and domain rebuilds.
    #[cfg(CONFIG_SYSCTL)]
    #[no_mangle]
    pub unsafe extern "C" fn lupos_topology_energy_aware_handler(table: *const ctl_table, write: c_int,
        buffer: *mut c_void, lenp: *mut usize, ppos: *mut loff_t) -> c_int {
        // SAFETY: The sysctl core owns these request objects through the call;
        // the native handler performs the buffer conversion and range checks.
        unsafe {
            if write != 0 && !lupos_topology_energy_admin() { return -(LUPOS_TOPOLOGY_EPERM as c_int); }
            if !sched_is_eas_possible(lupos_topology_active_mask()) {
                if write != 0 { return -(LUPOS_TOPOLOGY_EOPNOTSUPP as c_int); }
                *lenp = 0;
                return 0;
            }
            let ret = proc_dointvec_minmax(table, write, buffer, lenp, ppos);
            if ret == 0 && write != 0 {
                if lupos_topology_energy_requested() != lupos_topology_energy_enabled() as c_uint {
                    rebuild_sched_domains_energy();
                }
            }
            ret
        }
    }

    pub(super) unsafe fn free_pd(mut pd: *mut perf_domain) {
        // SAFETY: The caller owns this unpublished list or has completed its RCU
        // grace period. Save the next link before freeing each native allocation.
        unsafe {
            while !pd.is_null() {
                let tmp = (*pd).next;
                lupos_topology_free(pd.cast());
                pd = tmp;
            }
        }
    }

    unsafe fn find_pd(mut pd: *mut perf_domain, cpu: c_int) -> *mut perf_domain {
        // SAFETY: The build path owns the partial list and each referenced EM
        // domain; all nodes and their native CPU masks remain live for this walk.
        unsafe {
            while !pd.is_null() {
                if lupos_topology_mask_test(cpu, lupos_topology_energy_pd_span(pd)) { return pd; }
                pd = (*pd).next;
            }
            null_mut()
        }
    }

    unsafe fn pd_init(cpu: c_int) -> *mut perf_domain {
        // SAFETY: The caller supplies a CPU from the build mask. A successful
        // native zeroed allocation is exclusively owned until list publication.
        unsafe {
            let obj = lupos_topology_energy_em_cpu_get(cpu);
            if obj.is_null() {
                if sched_debug() { lupos_topology_energy_debug_no_em(cpu); }
                return null_mut();
            }
            let pd = lupos_topology_energy_pd_alloc();
            if pd.is_null() { return null_mut(); }
            (*pd).em_pd = obj;
            pd
        }
    }

    unsafe fn perf_domain_debug(cpu_map: *const cpumask, mut pd: *mut perf_domain) {
        // SAFETY: The build path keeps the mask, partial list and referenced EM
        // domains live while native diagnostics inspect their fields.
        unsafe {
            if !sched_debug() || pd.is_null() { return; }
            lupos_topology_energy_debug_root(cpu_map);
            while !pd.is_null() {
                lupos_topology_energy_debug_pd(pd);
                pd = (*pd).next;
            }
            lupos_topology_energy_debug_end();
        }
    }

    /// Frees a retired performance-domain list after its RCU grace period.
    ///
    /// # Safety
    /// `rp` must be the embedded callback of the list head queued by the native
    /// performance-domain RCU leaf, with exclusive reclamation ownership.
    #[no_mangle]
    pub unsafe extern "C" fn lupos_topology_destroy_perf_domain_rcu(rp: *mut callback_head) {
        // SAFETY: The RCU callback contract makes the containing list reclaimable.
        unsafe { free_pd(lupos_topology_energy_pd_from_rcu(rp)); }
    }

    pub(super) unsafe fn sched_energy_set(has_eas: bool) {
        // SAFETY: Partition callers hold CPU-hotplug and domain synchronization,
        // as required by the native cpuslocked static-key operations.
        unsafe {
            if !has_eas && lupos_topology_energy_enabled() {
                if sched_debug() { lupos_topology_energy_debug_stop(); }
                lupos_topology_energy_disable();
            } else if has_eas && !lupos_topology_energy_enabled() {
                if sched_debug() { lupos_topology_energy_debug_start(); }
                lupos_topology_energy_enable();
            }
        }
    }

    pub(super) unsafe fn build_perf_domains(cpu_map: *const cpumask) -> bool {
        // SAFETY: The serialized partition build supplies a nonempty live mask
        // and referenced root domain. New lists are private until RCU publication;
        // old lists are reclaimed only through the native grace-period callback.
        unsafe {
            let mut pd: *mut perf_domain = null_mut();
            let cpu = lupos_topology_mask_first(cpu_map) as c_int;
            let rd = (*lupos_topology_cpu_rq(cpu)).rd;
            'free: {
                if lupos_topology_energy_requested() == 0 { break 'free; }
                if !sched_is_eas_possible(cpu_map) { break 'free; }
                let mut i = lupos_topology_mask_first(cpu_map);
                while i < lupos_topology_mask_bound() {
                    if find_pd(pd, i as c_int).is_null() {
                        let tmp = pd_init(i as c_int);
                        if tmp.is_null() { break 'free; }
                        (*tmp).next = pd;
                        pd = tmp;
                    }
                    i = lupos_topology_mask_next(i as c_int, cpu_map);
                }
                perf_domain_debug(cpu_map, pd);
                let tmp = (*rd).pd;
                lupos_topology_energy_pd_assign(rd, pd);
                if !tmp.is_null() { lupos_topology_energy_pd_call_rcu(tmp); }
                return !pd.is_null();
            }
            free_pd(pd);
            let tmp = (*rd).pd;
            lupos_topology_energy_pd_clear(rd);
            if !tmp.is_null() { lupos_topology_energy_pd_call_rcu(tmp); }
            false
        }
    }
}
#[cfg(all(CONFIG_ENERGY_MODEL, CONFIG_CPU_FREQ_GOV_SCHEDUTIL))]
use topology_energy::{free_pd, sched_energy_set, build_perf_domains};
#[cfg(not(all(CONFIG_ENERGY_MODEL, CONFIG_CPU_FREQ_GOV_SCHEDUTIL)))]
unsafe fn free_pd(_pd: *mut perf_domain) { /* Exact native !EAS branch. */ }
