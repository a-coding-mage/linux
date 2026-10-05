// SPDX-License-Identifier: GPL-2.0
// Continued lifecycle family, topology.c:449–834 at 126a30fae3bba11420ec2fcbde51a0a01bab1b5b.
/// Reclaims a root domain once its users and RCU readers are gone.
///
/// # Safety
/// `rcu` must belong to an initialized native root domain with no references.
/// Either its RCU grace period has completed, or it was never published.
#[no_mangle]
pub unsafe extern "C" fn lupos_topology_free_rootdomain(rcu: *mut callback_head) {
    // SAFETY: The caller grants exclusive reclamation of the containing domain,
    // its initialized priority/deadline state, masks and performance-domain list.
    unsafe {
        let rd = lupos_topology_root_from_rcu(rcu);
        cpupri_cleanup(&raw mut (*rd).cpupri);
        cpudl_cleanup(&raw mut (*rd).cpudl);
        lupos_topology_root_mask_free(rd, LUPOS_TOPOLOGY_ROOT_DLO);
        lupos_topology_root_mask_free(rd, LUPOS_TOPOLOGY_ROOT_RTO);
        lupos_topology_root_mask_free(rd, LUPOS_TOPOLOGY_ROOT_ONLINE);
        lupos_topology_root_mask_free(rd, LUPOS_TOPOLOGY_ROOT_SPAN);
        free_pd((*rd).pd);
        lupos_topology_free(rd.cast());
    }
}

/// Transfers a runqueue's reference to a root domain.
///
/// # Safety
/// `rq` and `rd` must be live initialized scheduler objects. The caller must
/// serialize domain attachment and CPU hotplug, and must not hold the rq lock.
/// It must retain `rd` until this function acquires the runqueue's reference.
#[no_mangle]
pub unsafe extern "C" fn rq_attach_root(rq: *mut rq, rd: *mut root_domain) {
    // SAFETY: The native lock initializes the rq_flags fields used by unlock and
    // protects rq updates. Root references protect masks until deferred free.
    unsafe {
        let mut old_rd: *mut root_domain = null_mut();
        let mut rf = core::mem::MaybeUninit::<rq_flags>::uninit();
        lupos_topology_rq_lock(rq, rf.as_mut_ptr());
        if !(*rq).rd.is_null() {
            old_rd = (*rq).rd;
            if lupos_topology_mask_test((*rq).cpu, lupos_topology_root_mask(old_rd, LUPOS_TOPOLOGY_ROOT_ONLINE)) {
                set_rq_offline(rq);
            }
            lupos_topology_mask_clear_cpu((*rq).cpu, lupos_topology_root_mask(old_rd, LUPOS_TOPOLOGY_ROOT_SPAN));
            if !lupos_topology_atomic_dec_and_test(&raw mut (*old_rd).refcount) { old_rd = null_mut(); }
        }
        lupos_topology_atomic_inc(&raw mut (*rd).refcount);
        (*rq).rd = rd;
        lupos_topology_mask_set_cpu((*rq).cpu, lupos_topology_root_mask(rd, LUPOS_TOPOLOGY_ROOT_SPAN));
        if lupos_topology_mask_test((*rq).cpu, lupos_topology_active_mask()) { set_rq_online(rq); }
        if lupos_topology_rq_fair_server_active(rq) { __dl_server_attach_root(&raw mut (*rq).fair_server, rq); }
        #[cfg(CONFIG_SCHED_CLASS_EXT)]
        if lupos_topology_rq_ext_server_active(rq) { __dl_server_attach_root(&raw mut (*rq).ext_server, rq); }
        lupos_topology_rq_unlock(rq, rf.as_mut_ptr());
        if !old_rd.is_null() { lupos_topology_root_call_rcu(old_rd); }
    }
}

/// Acquires another reference to a live root domain.
///
/// # Safety
/// The caller must keep `rd` live until its reference count is incremented.
#[no_mangle]
pub unsafe extern "C" fn sched_get_rd(rd: *mut root_domain) {
    // SAFETY: The caller already protects the initialized native refcount object.
    unsafe { lupos_topology_atomic_inc(&raw mut (*rd).refcount); }
}
/// Releases a root-domain reference and defers final reclamation through RCU.
///
/// # Safety
/// The caller must own a reference to initialized `rd` and relinquish it once.
#[no_mangle]
pub unsafe extern "C" fn sched_put_rd(rd: *mut root_domain) {
    // SAFETY: The owned reference keeps rd live until the atomic decrement; only
    // its final release queues the embedded native RCU callback.
    unsafe {
        if !lupos_topology_atomic_dec_and_test(&raw mut (*rd).refcount) { return; }
        lupos_topology_root_call_rcu(rd);
    }
}

unsafe fn init_rootdomain(rd: *mut root_domain) -> c_int {
    // SAFETY: The caller exclusively owns zeroed root storage. Each failure label
    // frees only successfully initialized native resources, in reverse order.
    unsafe {
        'out: {
            'free_span: {
                'free_online: {
                    'free_dlo_mask: {
                        'free_rto_mask: {
                            'free_cpudl: {
                                if !lupos_topology_root_mask_alloc(rd, LUPOS_TOPOLOGY_ROOT_SPAN) { break 'out; }
                                if !lupos_topology_root_mask_alloc(rd, LUPOS_TOPOLOGY_ROOT_ONLINE) { break 'free_span; }
                                if !lupos_topology_root_mask_alloc(rd, LUPOS_TOPOLOGY_ROOT_DLO) { break 'free_online; }
                                if !lupos_topology_root_mask_alloc(rd, LUPOS_TOPOLOGY_ROOT_RTO) { break 'free_dlo_mask; }
                                // The native header owns HAVE_RT_PUSH_IPI, not a guessed Rust cfg.
                                lupos_topology_root_init_push(rd);
                                (*rd).visit_cookie = 0;
                                init_dl_bw(&raw mut (*rd).dl_bw);
                                if cpudl_init(&raw mut (*rd).cpudl) != 0 { break 'free_rto_mask; }
                                if cpupri_init(&raw mut (*rd).cpupri) != 0 { break 'free_cpudl; }
                                return 0;
                            }
                            cpudl_cleanup(&raw mut (*rd).cpudl);
                        }
                        lupos_topology_root_mask_free(rd, LUPOS_TOPOLOGY_ROOT_RTO);
                    }
                    lupos_topology_root_mask_free(rd, LUPOS_TOPOLOGY_ROOT_DLO);
                }
                lupos_topology_root_mask_free(rd, LUPOS_TOPOLOGY_ROOT_ONLINE);
            }
            lupos_topology_root_mask_free(rd, LUPOS_TOPOLOGY_ROOT_SPAN);
        }
        -(LUPOS_TOPOLOGY_ENOMEM as c_int)
    }
}

/// Initializes the boot-time default root domain.
///
/// # Safety
/// Invoke once during scheduler initialization before init memory is discarded
/// and before the default root domain is exposed to concurrent users.
#[no_mangle]
#[link_section = ".init.text"]
pub unsafe extern "C" fn lupos_topology_init_defrootdomain() {
    // SAFETY: Boot serialization provides exclusive access to zeroed native
    // default-root storage; the oracle's ignored allocation result is preserved.
    unsafe {
        let rd = &raw mut def_root_domain;
        let _ = init_rootdomain(rd);
        lupos_topology_atomic_set(&raw mut (*rd).refcount, 1);
    }
}
unsafe fn alloc_rootdomain() -> *mut root_domain {
    // SAFETY: Native allocation returns exclusive zeroed storage; init_rootdomain
    // unwinds owned resources before a failed allocation is released here.
    unsafe {
        let rd = lupos_topology_root_alloc();
        if rd.is_null() { return null_mut(); }
        if init_rootdomain(rd) != 0 {
            lupos_topology_free(rd.cast());
            return null_mut();
        }
        rd
    }
}

unsafe fn free_sched_groups(mut sg: *mut sched_group, free_sgc: c_int) {
    // SAFETY: The caller owns references to every node of this circular list.
    // Read the next link before releasing each node and capacity reference.
    unsafe {
        if sg.is_null() { return; }
        let first = sg;
        loop {
            let tmp = (*sg).next;
            if free_sgc != 0 && lupos_topology_sgc_put((*sg).sgc) {
                lupos_topology_free((*sg).sgc.cast());
            }
            if lupos_topology_sg_put(sg) { lupos_topology_free(sg.cast()); }
            sg = tmp;
            if sg == first { break; }
        }
    }
}
unsafe fn free_sched_domain_shared(sds: *mut sched_domain_shared) {
    // SAFETY: A nonnull pointer carries an owned shared-object reference; only
    // the final atomic release frees the native allocation.
    unsafe {
        if !sds.is_null() && lupos_topology_sds_put(sds) { lupos_topology_free(sds.cast()); }
    }
}
unsafe fn destroy_sched_domain(sd: *mut sched_domain) {
    // SAFETY: The domain is unpublished or past its RCU grace period; the caller
    // transfers its group/shared references and optional LLC-count allocation.
    unsafe {
        free_sched_groups((*sd).groups, 1);
        free_sched_domain_shared((*sd).shared);
        #[cfg(CONFIG_SCHED_CACHE)]
        lupos_topology_free((*sd).llc_counts.cast());
        lupos_topology_free(sd.cast());
    }
}
/// Reclaims a retired scheduler-domain hierarchy after its RCU grace period.
///
/// # Safety
/// `rcu` must be the callback embedded in the bottom domain queued by the native
/// domain RCU leaf; the full parent chain must be exclusively reclaimable.
#[no_mangle]
pub unsafe extern "C" fn lupos_topology_destroy_sched_domains_rcu(rcu: *mut callback_head) {
    // SAFETY: RCU retirement protects the chain until callback entry. Save each
    // parent before releasing the current domain and its owned resources.
    unsafe {
        let mut sd = lupos_topology_domain_from_rcu(rcu);
        while !sd.is_null() {
            let parent = (*sd).parent;
            destroy_sched_domain(sd);
            sd = parent;
        }
    }
}
unsafe fn destroy_sched_domains(sd: *mut sched_domain) {
    // SAFETY: The caller has detached this chain and transfers its sole teardown
    // ownership to RCU; null denotes that there is no previous hierarchy.
    unsafe { if !sd.is_null() { lupos_topology_domain_call_rcu(sd); } }
}

unsafe fn update_top_cache_domain(cpu: c_int) {
    // SAFETY: The attachment path serializes topology and hotplug for a valid
    // CPU; native RCU accessors publish pointers into its live domain hierarchy.
    unsafe {
        let mut sds = null_mut();
        let mut id = cpu;
        let mut size = 1;
        let mut sd = lupos_topology_highest_flag_domain(cpu, SD_SHARE_LLC as c_int);
        if !sd.is_null() {
            id = lupos_topology_mask_first(lupos_topology_domain_span(sd)) as c_int;
            size = lupos_topology_mask_weight(lupos_topology_domain_span(sd)) as c_int;
            lupos_topology_warn_missing_shared((*sd).shared.is_null());
            sds = (*sd).shared;
        }
        lupos_topology_sd_llc_assign(cpu, sd);
        lupos_topology_sd_llc_size_set(cpu, size);
        lupos_topology_sd_llc_shared_assign(cpu, sds);
        sd = lupos_topology_lowest_flag_domain(cpu, SD_CLUSTER as c_int);
        if !sd.is_null() { id = lupos_topology_mask_first(lupos_topology_domain_span(sd)) as c_int; }
        lupos_topology_sd_share_id_set(cpu, id);
        sd = lupos_topology_lowest_flag_domain(cpu, SD_NUMA as c_int);
        lupos_topology_sd_numa_assign(cpu, sd);
        sd = lupos_topology_highest_flag_domain(cpu, SD_ASYM_PACKING as c_int);
        lupos_topology_sd_asym_packing_assign(cpu, sd);
        sd = lupos_topology_lowest_flag_domain(cpu, SD_ASYM_CPUCAPACITY_FULL as c_int);
        if !sd.is_null() && !(*sd).shared.is_null() { sds = (*sd).shared; }
        lupos_topology_sd_asym_cpucapacity_assign(cpu, sd);
        lupos_topology_sd_balance_shared_assign(cpu, sds);
    }
}

unsafe fn cpu_attach_domain(mut sd: *mut sched_domain, rd: *mut root_domain, cpu: c_int) {
    // SAFETY: The caller owns the newly built hierarchy and its group/shared
    // references under topology/hotplug serialization. Refcounts protect reused
    // shared state; retired published domains are released only through RCU.
    unsafe {
        let rq = lupos_topology_cpu_rq(cpu);
        let mut tmp = sd;
        while !tmp.is_null() {
            let parent = (*tmp).parent;
            if parent.is_null() { break; }
            if sd_parent_degenerate(tmp, parent) {
                (*tmp).parent = (*parent).parent;
                if !(*parent).shared.is_null() {
                    free_sched_domain_shared((*tmp).shared);
                    (*tmp).shared = (*parent).shared;
                    (*parent).shared = null_mut();
                }
                if !(*parent).parent.is_null() {
                    (*(*parent).parent).child = tmp;
                    (*(*(*parent).parent).groups).flags = (*tmp).flags;
                }
                if (*parent).flags & SD_PREFER_SIBLING != 0 { (*tmp).flags |= SD_PREFER_SIBLING; }
                destroy_sched_domain(parent);
            } else { tmp = (*tmp).parent; }
        }
        if !sd.is_null() && sd_degenerate(sd) {
            tmp = sd;
            sd = (*sd).parent;
            if !sd.is_null() {
                let sg = (*sd).groups;
                #[cfg(CONFIG_SCHED_CACHE)] {
                    (*sd).llc_counts = (*tmp).llc_counts;
                    (*sd).llc_max = (*tmp).llc_max;
                    (*sd).llc_bytes = (*tmp).llc_bytes;
                    (*tmp).llc_counts = null_mut();
                    (*tmp).llc_max = 0;
                    (*tmp).llc_bytes = 0;
                }
                // Preserve the oracle's do/while, including its unchanged sg cursor.
                loop {
                    (*sg).flags = 0;
                    if sg == (*sd).groups { break; }
                }
                (*sd).child = null_mut();
            }
            destroy_sched_domain(tmp);
        }
        sched_domain_debug(sd, cpu);
        rq_attach_root(rq, rd);
        tmp = (*rq).sd;
        lupos_topology_rq_sd_assign(rq, sd);
        dirty_sched_domain_sysctl(cpu);
        destroy_sched_domains(tmp);
        update_top_cache_domain(cpu);
    }
}
