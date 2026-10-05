// SPDX-License-Identifier: GPL-2.0
// Retained topology.rs:1838–1910 and 2685–3536, repaired against topology.c
// at 126a30fae3bba11420ec2fcbde51a0a01bab1b5b. Lexically included by topology.rs.
// Native headers own all types/storage. Native leaves are unqualified runtime C.
compile_error!("SOURCE ONLY HOLD: scheduler topology build is not admitted");

/// Releases the unclaimed storage covered by the completed allocation stages.
///
/// # Safety
/// The caller must hold the scheduler-domain mutex and stabilize CPU topology.
/// `cpu_map` must be the live mask used for this allocation attempt. For any
/// stage other than `sa_none`, `d` must retain the matching initialized state;
/// claimed objects must already be removed from the allocation slots. A zero-ref
/// root domain at `sa_rootdomain` must never have been published. At `sa_none`,
/// `d` may point to uninitialized storage because it is not accessed.
unsafe fn __free_domain_allocs(d: *mut s_data, mut what: s_alloc, cpu_map: *const cpumask) {
    // SAFETY: The stage gates each access to initialized storage. Slot nulling
    // excludes transferred objects; a zero-ref unpublished root can be freed
    // directly through its real native callback_head rather than queued to RCU.
    unsafe {
        // Preserve the original switch's ordered fallthrough after partial allocation.
        if what == LUPOS_TOPOLOGY_BUILD_SA_ROOTDOMAIN as s_alloc {
            if lupos_topology_atomic_read(&raw const (*(*d).rd).refcount) == 0 {
                lupos_topology_free_rootdomain(&raw mut (*(*d).rd).rcu);
            }
            what = LUPOS_TOPOLOGY_BUILD_SA_SD as s_alloc;
        }
        if what == LUPOS_TOPOLOGY_BUILD_SA_SD as s_alloc {
            lupos_topology_build_data_sd_free(d);
            what = LUPOS_TOPOLOGY_BUILD_SA_SD_SHARED as s_alloc;
        }
        if what == LUPOS_TOPOLOGY_BUILD_SA_SD_SHARED as s_alloc {
            __sds_free(d, cpu_map);
            what = LUPOS_TOPOLOGY_BUILD_SA_SD_STORAGE as s_alloc;
        }
        if what == LUPOS_TOPOLOGY_BUILD_SA_SD_STORAGE as s_alloc {
            __sdt_free(cpu_map);
        }
    }
}

/// Allocates construction storage and reports the precise cleanup stage.
///
/// # Safety
/// `d` must point to writable, exclusively owned native `s_data` storage with no
/// resources to discard. The caller must hold the domain mutex in sleepable
/// context, stabilize topology and `cpu_map`, and own the empty topology slots.
/// The returned stage must be passed to `__free_domain_allocs` after use.
unsafe fn __visit_domain_allocation_hell(d: *mut s_data, cpu_map: *const cpumask) -> s_alloc {
    // SAFETY: Native zeroing initializes d before reads; each successful stage
    // installs its resource before a later stage can fail and trigger cleanup.
    unsafe {
        lupos_topology_build_data_zero(d);
        if __sdt_alloc(cpu_map) != 0 { return LUPOS_TOPOLOGY_BUILD_SA_SD_STORAGE as s_alloc; }
        if __sds_alloc(d, cpu_map) != 0 { return LUPOS_TOPOLOGY_BUILD_SA_SD_SHARED as s_alloc; }
        if !lupos_topology_build_data_sd_alloc(d) {
            return LUPOS_TOPOLOGY_BUILD_SA_SD_SHARED as s_alloc;
        }
        (*d).rd = alloc_rootdomain();
        if (*d).rd.is_null() { return LUPOS_TOPOLOGY_BUILD_SA_SD as s_alloc; }
        LUPOS_TOPOLOGY_BUILD_SA_ROOTDOMAIN as s_alloc
    }
}

/*
 * NULL the sd_data elements we've used to build the sched_domain and
 * sched_group structure so that the subsequent __free_domain_allocs()
 * will not free the data we're using.
 */
/// Removes domain-owned objects from the construction cleanup slots.
///
/// # Safety
/// The caller must serialize construction and provide a valid allocated CPU
/// slot in `d`, its acyclic domain chain, and live native `private` `sd_data`.
/// Groups/shared refcounts must reflect their constructed owners, and this CPU
/// must not have been claimed already. Claimed objects must later be attached
/// or released by their domain owners rather than allocation cleanup.
unsafe fn claim_allocations(cpu: c_int, d: *mut s_data) {
    // SAFETY: Construction owns the live slot arrays and domains exclusively.
    // Clearing a referenced slot transfers its cleanup responsibility without
    // freeing the object; native accessors select the real per-CPU/ref fields.
    unsafe {
        let sds = lupos_topology_build_data_sds(d, cpu);
        if lupos_topology_atomic_read(lupos_topology_build_sds_ref(sds)) != 0 {
            lupos_topology_build_data_sds_set(d, cpu, null_mut());
        }
        let mut sd = lupos_topology_data_sd(d, cpu);
        while !sd.is_null() {
            let sdd = lupos_topology_domain_private(sd);
            lupos_topology_build_warn_claim(lupos_topology_groups_sd(sdd, cpu) != sd);
            lupos_topology_build_sdd_sd_set(sdd, cpu, null_mut());
            let sg = lupos_topology_groups_sg(sdd, cpu);
            if lupos_topology_atomic_read(lupos_topology_build_sg_ref(sg)) != 0 {
                lupos_topology_build_sdd_sg_set(sdd, cpu, null_mut());
            }
            let sgc = lupos_topology_groups_sgc(sdd, cpu);
            if lupos_topology_atomic_read(lupos_topology_build_sgc_ref(sgc)) != 0 {
                lupos_topology_build_sdd_sgc_set(sdd, cpu, null_mut());
            }
            sd = (*sd).parent;
        }
    }
}

/// Allocates per-level domain and group construction slots.
///
/// # Safety
/// The caller must hold the domain mutex in sleepable context and stabilize the
/// native sentinel-terminated topology and readable valid-CPU `cpu_map`. All
/// topology allocation slots must initially be null; this caller exclusively
/// owns them until `__sdt_free`, including after a partial allocation failure.
unsafe fn __sdt_alloc(cpu_map: *const cpumask) -> c_int {
    // SAFETY: Native allocators zero per-CPU slots and size every object using
    // configured sizeof/cpumask_size. Each non-null result is installed before
    // the next allocation, so the matching cleanup can find every owned object.
    unsafe {
        let mut tl = lupos_topology_numa_get_topology();
        while (*tl).mask.is_some() {
            let sdd = &raw mut (*tl).data;
            if !lupos_topology_build_sdd_sd_alloc(sdd) { return -(LUPOS_TOPOLOGY_ENOMEM as c_int); }
            if !lupos_topology_build_sdd_sg_alloc(sdd) { return -(LUPOS_TOPOLOGY_ENOMEM as c_int); }
            if !lupos_topology_build_sdd_sgc_alloc(sdd) { return -(LUPOS_TOPOLOGY_ENOMEM as c_int); }
            for j in topology_groups_cpus(cpu_map) {
                let sd = lupos_topology_build_sd_alloc(j);
                if sd.is_null() { return -(LUPOS_TOPOLOGY_ENOMEM as c_int); }
                lupos_topology_build_sdd_sd_set(sdd, j, sd);
                let sg = lupos_topology_build_sg_alloc(j);
                if sg.is_null() { return -(LUPOS_TOPOLOGY_ENOMEM as c_int); }
                (*sg).next = sg;
                lupos_topology_build_sdd_sg_set(sdd, j, sg);
                let sgc = lupos_topology_build_sgc_alloc(j);
                if sgc.is_null() { return -(LUPOS_TOPOLOGY_ENOMEM as c_int); }
                (*sgc).id = j;
                lupos_topology_build_sdd_sgc_set(sdd, j, sgc);
            }
            tl = tl.add(1);
        }
        0
    }
}

/// Frees the still-owned topology construction slots, including partial ones.
///
/// # Safety
/// The caller must hold the domain mutex, stabilize topology and the allocation
/// mask, and exclusively own the slots from `__sdt_alloc`. Claimed slots must be
/// null. Any unclaimed NUMA domain must own its groups with the refcount state
/// expected by `free_sched_groups(groups, 0)`; no reader may retain freed data.
unsafe fn __sdt_free(cpu_map: *const cpumask) {
    // SAFETY: Null array/entry checks cover partial allocation. Only unclaimed
    // objects are freed, then native free_percpu releases and clears each array.
    unsafe {
        let mut tl = lupos_topology_numa_get_topology();
        while (*tl).mask.is_some() {
            let sdd = &raw mut (*tl).data;
            for j in topology_groups_cpus(cpu_map) {
                if !(*sdd).sd.is_null() {
                    let sd = lupos_topology_groups_sd(sdd, j);
                    if !sd.is_null() && (*sd).flags & SD_NUMA != 0 {
                        free_sched_groups((*sd).groups, 0);
                    }
                    lupos_topology_free(sd.cast());
                }
                if !(*sdd).sg.is_null() { lupos_topology_free(lupos_topology_groups_sg(sdd, j).cast()); }
                if !(*sdd).sgc.is_null() { lupos_topology_free(lupos_topology_groups_sgc(sdd, j).cast()); }
            }
            lupos_topology_build_sdd_sd_free(sdd);
            lupos_topology_build_sdd_sg_free(sdd);
            lupos_topology_build_sdd_sgc_free(sdd);
            tl = tl.add(1);
        }
    }
}

/// Allocates shared-domain construction objects for the selected CPUs.
///
/// # Safety
/// `d` must be exclusive initialized storage with no existing `sds` allocation.
/// The caller must serialize construction in sleepable context and keep the
/// valid-CPU mask stable. The caller owns every successful allocation and must
/// invoke `__sds_free` even when only a prefix was allocated.
unsafe fn __sds_alloc(d: *mut s_data, cpu_map: *const cpumask) -> c_int {
    // SAFETY: Native zeroed per-CPU allocation leaves unfilled entries null;
    // each successful shared object is stored before any subsequent failure.
    unsafe {
        if !lupos_topology_build_data_sds_alloc(d) { return -(LUPOS_TOPOLOGY_ENOMEM as c_int); }
        for j in topology_groups_cpus(cpu_map) {
            let sds = lupos_topology_build_sds_alloc(j);
            if sds.is_null() { return -(LUPOS_TOPOLOGY_ENOMEM as c_int); }
            lupos_topology_build_data_sds_set(d, j, sds);
        }
        0
    }
}

/// Frees unclaimed shared-domain objects and their per-CPU slot array.
///
/// # Safety
/// `d` must retain the initialized, exclusively owned result of `__sds_alloc`.
/// The caller must stabilize the original mask and serialize construction.
/// Objects transferred to domains must have null slots; all remaining non-null
/// objects must be unreferenced and reclaimable exactly once by this caller.
unsafe fn __sds_free(d: *mut s_data, cpu_map: *const cpumask) {
    // SAFETY: An absent array is skipped; zeroed or claimed entries permit
    // kfree(NULL). The native array release clears d->sds after its last use.
    unsafe {
        if (*d).sds.is_null() { return; }
        for j in topology_groups_cpus(cpu_map) {
            lupos_topology_free(lupos_topology_build_data_sds(d, j).cast());
        }
        lupos_topology_build_data_sds_free(d);
    }
}

/// Initializes one preallocated domain above an optional child.
///
/// # Safety
/// Under the domain mutex and stable CPU topology, `tl` must identify a live
/// level with a preallocated domain slot for `cpu` in `cpu_map`. The optional
/// child must be an exclusively owned, initialized lower domain with a valid
/// span; `attr` must be null or readable. Topology callbacks must return live
/// masks. The caller retains ownership until claim/attachment or cleanup.
unsafe fn build_sched_domain(tl: *mut sched_domain_topology_level, cpu_map: *const cpumask,
                            attr: *mut sched_domain_attr, child: *mut sched_domain,
                            cpu: c_int) -> *mut sched_domain {
    // SAFETY: sd_init consumes the caller's live allocation slot. Child and
    // parent remain construction-owned; native span helpers address their
    // allocated tail masks, and the mutex serializes the shared level maximum.
    unsafe {
        let sd = sd_init(tl, cpu_map, child, cpu);
        if !child.is_null() {
            (*sd).level = (*child).level + 1;
            sched_domain_level_max = core::cmp::max(sched_domain_level_max, (*sd).level);
            (*child).parent = sd;
            if !lupos_topology_mask_subset(lupos_topology_domain_span(child), lupos_topology_domain_span(sd)) {
                lupos_topology_build_debug_broken(child, sd);
                /* Fixup, ensure @sd has at least @child CPUs. */
                lupos_topology_mask_or(lupos_topology_domain_span(sd), lupos_topology_domain_span(sd),
                                       lupos_topology_domain_span(child));
            }
        }
        set_domain_attribute(sd, attr);
        sd
    }
}

/*
 * Ensure topology masks are sane, i.e. there are no conflicts (overlaps) for
 * any two given CPUs on non-NUMA topology levels.
 */
/// Checks equal-or-disjoint spans for non-NUMA topology levels.
///
/// # Safety
/// The caller must hold the domain mutex and stabilize the live CPU mask and
/// sentinel-terminated topology. Scratch masks must be allocated. Each invoked
/// topology callback must return a nonempty live mask containing only valid CPU
/// IDs, including when invoked for that mask's first CPU.
unsafe fn topology_span_sane(cpu_map: *const cpumask) -> bool {
    // SAFETY: The mutex exclusively protects both scratch masks; stable native
    // callbacks and valid first-CPU IDs permit the mask comparisons and indexing.
    unsafe {
        lupos_topology_assert_domains_locked();
        let covered = lupos_topology_tmpmask();
        let id_seen = lupos_topology_tmpmask2();
        let mut tl = lupos_topology_numa_get_topology();
        while (*tl).mask.is_some() {
            let mut tl_common_flags = 0;
            if lupos_topology_level_has_flags(tl) { tl_common_flags = lupos_topology_level_flags(tl); }
            /* NUMA levels are allowed to overlap. */
            if tl_common_flags & SD_NUMA != 0 { tl = tl.add(1); continue; }
            lupos_topology_mask_clear(covered);
            lupos_topology_mask_clear(id_seen);
            /* Non-NUMA levels must be completely equal or completely disjoint. */
            for cpu in topology_groups_cpus(cpu_map) {
                let tl_cpu_mask = lupos_topology_level_mask(tl, cpu);
                /* Lowest bit set in this mask is used as a unique id. */
                let id = lupos_topology_mask_first(tl_cpu_mask) as c_int;
                if lupos_topology_mask_test(id, id_seen) {
                    if !lupos_topology_mask_equal(lupos_topology_level_mask(tl, id), tl_cpu_mask) { return false; }
                } else {
                    if lupos_topology_mask_intersects(tl_cpu_mask, covered) { return false; }
                    lupos_topology_mask_or(covered, covered, tl_cpu_mask);
                    lupos_topology_mask_set_cpu(id, id_seen);
                }
            }
            tl = tl.add(1);
        }
        true
    }
}

/* Calculate an allowed NUMA imbalance such that LLCs do not get imbalanced. */
/// Propagates the native LLC-based NUMA imbalance allowance up the domain tree.
///
/// # Safety
/// `sd_llc` must be an exclusively construction-owned `SD_SHARE_LLC` domain with a
/// live parent and finite ancestor chain. All span weights must be initialized
/// and nonzero, and topology/attachment must remain serialized by the caller.
unsafe fn adjust_numa_imbalance(sd_llc: *mut sched_domain) {
    // SAFETY: The caller's established parent and nonzero span invariants make
    // the warned-about conditions nonfatal diagnostics, not pointer guards.
    // Ancestors remain construction-owned throughout the weight updates.
    unsafe {
        lupos_topology_build_warn_llc((*sd_llc).flags & SD_SHARE_LLC == 0);
        lupos_topology_build_warn_parent((*sd_llc).parent.is_null());
        /*
         * A single LLC per node allows up to 12.5% node imbalance. Multiple
         * LLCs allow imbalance until tasks would share a local LLC while
         * remote LLCs remain idle, preserving the original SMT/channel rule.
         */
        let nr_llcs = (*(*sd_llc).parent).span_weight / (*sd_llc).span_weight;
        let mut imb = if nr_llcs == 1 { (*(*sd_llc).parent).span_weight >> 3 } else { nr_llcs };
        imb = core::cmp::max(1, imb);
        (*(*sd_llc).parent).imb_numa_nr = imb;
        /* Start above the NODE domain to find the first NUMA domain. */
        let mut parent = (*(*sd_llc).parent).parent;
        while !parent.is_null() && (*parent).flags & SD_NUMA == 0 { parent = (*parent).parent; }
        let imb_span = if !parent.is_null() { (*parent).span_weight } else { (*(*sd_llc).parent).span_weight };
        /* Update the upper remainder of the topology. */
        parent = (*sd_llc).parent;
        while !parent.is_null() {
            let factor = core::cmp::max(1, (*parent).span_weight / imb_span);
            (*parent).imb_numa_nr = imb.wrapping_mul(factor);
            parent = (*parent).parent;
        }
    }
}

/// Attaches a referenced construction-time shared object to one domain.
///
/// # Safety
/// The caller must serialize construction and exclusively own `sd` with its
/// initialized nonempty span/weight and construction-phase shared field. Every CPU
/// in that span must have a live unclaimed shared allocation in `d->sds`.
/// `flags` must be the native LLC or asymmetry claim flag; the allocation union
/// must still be in its construction phase rather than runtime idle-scan use.
unsafe fn init_sched_domain_shared(d: *mut s_data, sd: *mut sched_domain, flags: c_int) {
    // SAFETY: Every visited slot is live, and the nonempty span supplies a last
    // object even on the warning fallback. Native accessors select alloc_flags
    // and ref; the new domain reference is recorded before ownership is claimed.
    unsafe {
        let mut sds: *mut sched_domain_shared = null_mut();
        /*
         * ASYM_CPUCAPACITY and SHARE_LLC may alias the same first CPU. Choose
         * an unclaimed or matching object; single-CPU domains may temporarily
         * overlap because they will degenerate later.
         */
        for cpu in topology_groups_cpus(lupos_topology_domain_span(sd)) {
            sds = lupos_topology_build_data_sds(d, cpu);
            let alloc_flags = lupos_topology_build_alloc_flags(sds);
            if alloc_flags == 0 || (*sd).span_weight == 1 || alloc_flags == flags {
                lupos_topology_build_alloc_flags_set(sds, flags);
                (*sd).shared = sds;
                break;
            }
        }
        /* Use the last CPU's object if no unclaimed object was available. */
        if lupos_topology_build_warn_shared((*sd).shared.is_null()) { (*sd).shared = sds; }
        /* nr_busy_cpus is read by the NOHZ kick path, not the asym path. */
        lupos_topology_atomic_set(&raw mut (*(*sd).shared).nr_busy_cpus, (*sd).span_weight as c_int);
        lupos_topology_atomic_inc(lupos_topology_build_sds_ref((*sd).shared));
    }
}

/*
 * Claim the innermost FULL ancestor only when it is not an overlapping NUMA
 * domain. A symmetric island may lack a FULL ancestor, so LLC then claims it.
 */
/// Claims shared state for the innermost eligible full-asymmetry ancestor.
///
/// # Safety
/// The caller must serialize construction and provide a valid `d->sd` CPU slot
/// whose optional domain chain is live, finite and not yet attached. Any
/// selected non-NUMA ancestor must satisfy `init_sched_domain_shared`, including
/// nonempty initialized span and live unclaimed shared allocation slots.
unsafe fn claim_asym_sched_domain_shared(d: *mut s_data, cpu: c_int) -> bool {
    // SAFETY: Null checks bound the live construction chain; only the eligible
    // ancestor is passed to the shared-state ownership/refcount initializer.
    unsafe {
        let sd = lupos_topology_data_sd(d, cpu);
        if sd.is_null() { return false; }
        let mut sd_asym = sd;
        while !sd_asym.is_null() && (*sd_asym).flags & SD_ASYM_CPUCAPACITY_FULL == 0 {
            sd_asym = (*sd_asym).parent;
        }
        if sd_asym.is_null() || (*sd_asym).flags & SD_NUMA != 0 { return false; }
        init_sched_domain_shared(d, sd_asym, SD_ASYM_CPUCAPACITY);
        true
    }
}

/// Reserves the first free native LLC ID and updates the allocation maximum.
///
/// # Safety
/// The caller must hold the domain mutex and stabilize CPU topology. The global
/// LLC allocation bitmap must be initialized, with valid IDs bounded by the
/// native CPU count; its ownership must agree with the per-CPU LLC-ID state.
unsafe fn __sched_domains_alloc_llc_id() -> c_int {
    // SAFETY: The mutex serializes non-atomic bitmap updates and max_lid; the
    // native CPU-count bound is checked before reserving the selected bit.
    unsafe {
        lupos_topology_assert_domains_locked();
        let allocmask = lupos_topology_llc_id_allocmask();
        let lid = lupos_topology_build_mask_first_zero(allocmask) as c_int;
        /* The LLC ID space cannot exceed the possible CPU count. */
        if lid as c_uint >= lupos_topology_nr_cpu_ids() { return -1; }
        lupos_topology_build_mask_set_nonatomic(lid, allocmask);
        let max = lupos_topology_build_mask_last(allocmask) as c_int;
        if max > max_lid { max_lid = max; }
        lid
    }
}

/// Releases an offline CPU's LLC ID after checking remaining sibling owners.
///
/// # Safety
/// The caller must hold the domain mutex and stabilize CPU hotplug/topology.
/// `cpu` must select a live possible-CPU slot being withdrawn from scheduling;
/// native LLC masks, initialized bitmap and all sibling ID slots must remain
/// valid. Other CPUs must retain their ID while they still own that LLC.
unsafe fn __sched_domains_free_llc_id(cpu: c_int) {
    // SAFETY: Stable per-CPU slots and LLC masks permit the sibling walk. The
    // mutex protects bitmap/max_lid, and a bit is cleared only after no sibling
    // retains the ID; the caller's CPU slot is relinquished first.
    unsafe {
        lupos_topology_assert_domains_locked();
        let lid = lupos_topology_build_llc_id(cpu);
        if lid == -1 || lid as c_uint >= lupos_topology_nr_cpu_ids() { return; }
        lupos_topology_build_llc_id_set(cpu, -1);
        for i in topology_groups_cpus(lupos_topology_llc_mask(cpu)) {
            /* An online CPU owns the llc_id. */
            if lupos_topology_build_llc_id(i) == lid { return; }
        }
        let allocmask = lupos_topology_llc_id_allocmask();
        lupos_topology_build_mask_clear_nonatomic(lid, allocmask);
        let max = lupos_topology_build_mask_last(allocmask) as c_int;
        /* Shrink max lid to save memory. */
        if max < max_lid { max_lid = max; }
    }
}

/// Drops a CPU's LLC-ID ownership during the serialized hotplug removal path.
///
/// # Safety
/// The caller must stabilize CPU hotplug/topology, supply a valid possible CPU
/// being withdrawn from scheduling, and meet `__sched_domains_free_llc_id`'s
/// initialized-state requirements. It must be sleepable and must not already
/// hold the scheduler-domain mutex, which this wrapper acquires itself.
#[no_mangle]
pub unsafe extern "C" fn sched_domains_free_llc_id(cpu: c_int) {
    // SAFETY: The caller supplies hotplug serialization and valid storage; the
    // balanced mutex pair establishes exclusive LLC bitmap/slot mutation.
    unsafe {
        sched_domains_mutex_lock();
        __sched_domains_free_llc_id(cpu);
        sched_domains_mutex_unlock();
    }
}

/* Build and attach sched domains for the given CPU set. */
/// Builds one partition and transfers its claimed domains to the runqueues.
///
/// # Safety
/// The caller must hold the domain mutex in sleepable context, stabilize CPU
/// hotplug/topology, and provide a live readable valid-CPU mask and null or
/// readable `attr`. `multi_llcs` must be exclusively writable and disjoint from
/// inputs. Native scratch/LLC state, runqueues and topology callbacks must be
/// initialized; topology slots must be available for this allocation attempt.
unsafe fn build_sched_domains(cpu_map: *const cpumask, attr: *mut sched_domain_attr,
                             multi_llcs: *mut bool) -> c_int {
    // SAFETY: Successful allocation initializes every accessed slot. The local
    // stage records precisely what cleanup may touch, including no d access on
    // an empty mask. Claims transfer ownership before attachment; the explicit
    // RCU section protects attached-domain queries, and attachment takes root
    // references before unclaimed construction storage is released.
    unsafe {
        let mut alloc_state = LUPOS_TOPOLOGY_BUILD_SA_NONE as s_alloc;
        let mut has_multi_llcs = false;
        let mut d_storage = core::mem::MaybeUninit::<s_data>::uninit();
        let d = d_storage.as_mut_ptr();
        let mut rq: *mut rq = null_mut();
        let mut ret = -(LUPOS_TOPOLOGY_ENOMEM as c_int);
        let mut has_asym = false;
        let mut has_cluster = false;
        'error: {
            if lupos_topology_build_warn_empty(lupos_topology_mask_empty(cpu_map)) { break 'error; }
            alloc_state = __visit_domain_allocation_hell(d, cpu_map);
            if alloc_state != LUPOS_TOPOLOGY_BUILD_SA_ROOTDOMAIN as s_alloc { break 'error; }
            /* Set up domains for CPUs specified by cpu_map. */
            for i in topology_groups_cpus(cpu_map) {
                let topology = lupos_topology_numa_get_topology();
                let mut tl = topology;
                let mut sd: *mut sched_domain = null_mut();
                while (*tl).mask.is_some() {
                    sd = build_sched_domain(tl, cpu_map, attr, sd, i);
                    has_asym |= (*sd).flags & SD_ASYM_CPUCAPACITY != 0;
                    if tl == topology { lupos_topology_build_data_sd_set(d, i, sd); }
                    if lupos_topology_mask_equal(cpu_map, lupos_topology_domain_span(sd)) { break; }
                    tl = tl.add(1);
                }
                let mut lid = lupos_topology_build_llc_id(i);
                if lid == -1 {
                    /* Try to reuse the LLC ID of a sibling. */
                    let mut j = lupos_topology_mask_first(lupos_topology_llc_mask(i)) as c_int;
                    while (j as c_uint) < lupos_topology_nr_cpu_ids() {
                        if i != j {
                            lid = lupos_topology_build_llc_id(j);
                            if lid != -1 {
                                lupos_topology_build_llc_id_set(i, lid);
                                break;
                            }
                        }
                        j = lupos_topology_mask_next(j, lupos_topology_llc_mask(i)) as c_int;
                    }
                    /* A new LLC was detected. */
                    if lid == -1 { lupos_topology_build_llc_id_set(i, __sched_domains_alloc_llc_id()); }
                }
            }
            if lupos_topology_build_warn_span(!topology_span_sane(cpu_map)) { break 'error; }
            /* Build groups for each domain, preserving overlap error unwinding. */
            for i in topology_groups_cpus(cpu_map) {
                let mut sd = lupos_topology_data_sd(d, i);
                while !sd.is_null() {
                    (*sd).span_weight = lupos_topology_mask_weight(lupos_topology_domain_span(sd));
                    if (*sd).flags & SD_NUMA != 0 {
                        if build_overlap_sched_groups(sd, i) != 0 { break 'error; }
                    } else if build_sched_groups(sd, i) != 0 { break 'error; }
                    sd = (*sd).parent;
                }
            }
            for i in topology_groups_cpus(cpu_map) {
                let mut sd = lupos_topology_data_sd(d, i);
                if sd.is_null() { continue; }
                if has_asym { claim_asym_sched_domain_shared(d, i); }
                /* First, find the topmost SD_SHARE_LLC domain. */
                while !(*sd).parent.is_null() && (*(*sd).parent).flags & SD_SHARE_LLC != 0 {
                    sd = (*sd).parent;
                }
                if (*sd).flags & SD_SHARE_LLC != 0 {
                    init_sched_domain_shared(d, sd, SD_SHARE_LLC);
                    /* Higher domains require NUMA imbalance hierarchy adjustment. */
                    if !(*sd).parent.is_null() {
                        if cfg!(CONFIG_NUMA) { adjust_numa_imbalance(sd); }
                        if sd_in_multi_llcs(sd) { has_multi_llcs = true; }
                    }
                }
            }
            /* Calculate CPU capacity for packages and nodes, in reverse CPU order. */
            for i in (0..lupos_topology_build_nr_cpumask_bits() as c_int).rev() {
                if !lupos_topology_mask_test(i, cpu_map) { continue; }
                claim_allocations(i, d);
                let mut sd = lupos_topology_data_sd(d, i);
                while !sd.is_null() {
                    init_sched_groups_capacity(i, sd);
                    sd = (*sd).parent;
                }
            }
            alloc_sd_llc(cpu_map, d);
            /* Attach the domains. */
            lupos_topology_groups_rcu_read_lock();
            for i in topology_groups_cpus(cpu_map) {
                rq = lupos_topology_cpu_rq(i);
                let sd = lupos_topology_data_sd(d, i);
                cpu_attach_domain(sd, (*d).rd, i);
                if !lupos_topology_lowest_flag_domain(i, SD_CLUSTER).is_null() { has_cluster = true; }
            }
            lupos_topology_groups_rcu_read_unlock();
            if has_asym { lupos_topology_build_asym_inc(); }
            if has_cluster { lupos_topology_build_cluster_inc(); }
            if !rq.is_null() && sched_debug_verbose { lupos_topology_build_debug_root(cpu_map); }
            ret = 0;
        }
        *multi_llcs = has_multi_llcs;
        __free_domain_allocs(d, alloc_state, cpu_map);
        ret
    }
}

/* Current domains, attributes, count and fallback storage are native-owned. */
/// Allocates an owned native partition-mask array, or returns null on failure.
///
/// # Safety
/// Invoke in sleepable kernel context where `GFP_KERNEL` allocation is permitted.
/// The returned non-null array belongs to the caller and must be released once
/// with `free_sched_domains` or transferred to `partition_sched_domains`; its
/// masks must be initialized by the caller before partition construction.
#[no_mangle]
pub unsafe extern "C" fn alloc_sched_domains(ndoms: c_uint) -> *mut cpumask_var_t {
    // SAFETY: Native allocation provides ndoms correctly sized cpumask_var_t
    // elements for either representation. Failure frees only the initialized
    // prefix, and no element is accessed after its enclosing array is released.
    unsafe {
        let doms = lupos_topology_build_masks_alloc(ndoms);
        if doms.is_null() { return null_mut(); }
        for i in 0..ndoms {
            if !lupos_topology_build_mask_alloc(doms, i) {
                free_sched_domains(doms, i);
                return null_mut();
            }
        }
        doms
    }
}

/// Releases an owned partition-mask array and its initialized mask prefix.
///
/// # Safety
/// `doms` must be an exclusively owned native allocation with at least `ndoms`
/// initialized mask elements and no live readers, or a native null/zero-size
/// allocation with `ndoms == 0`. It must not be fallback storage. Reclaiming the
/// previous `doms_cur` array requires the domain mutex and completion of all old
/// partition comparisons. An allocated larger array with only this prefix
/// initialized is permitted for allocation-failure cleanup.
#[no_mangle]
pub unsafe extern "C" fn free_sched_domains(doms: *mut cpumask_var_t, ndoms: c_uint) {
    // SAFETY: The caller relinquishes each initialized mask and the enclosing
    // allocation exactly once; native helpers own representation-specific frees.
    unsafe {
        for i in 0..ndoms { lupos_topology_build_mask_free(doms, i); }
        lupos_topology_free(doms.cast());
    }
}

/* Set up scheduler domains, excluding isolated CPUs. */
/// Initializes the boot-time domain partition through the native init wrapper.
///
/// # Safety
/// Invoke once from the native SMP initialization path while init memory is
/// live, with the domain mutex held and no concurrent CPU hotplug. `cpu_map`,
/// runqueues and native topology must be initialized and stable. This retains
/// the native boot path's unchecked scratch/fallback allocation policy; those
/// masks must exist when later operations use them.
#[no_mangle]
#[link_section = ".init.text"]
pub unsafe extern "C" fn lupos_topology_build_init_domains(cpu_map: *const cpumask) -> c_int {
    // SAFETY: Boot serialization gives this path ownership of global scratch,
    // fallback and initial partition state. Native helpers retain the exact
    // allocation representation; build_sched_domains handles its own staged
    // resources. The original unchecked boot-mask allocation policy is retained.
    unsafe {
        // The native source intentionally does not branch on these four results.
        lupos_topology_init_llc_id_allocmask();
        lupos_topology_init_tmpmask();
        lupos_topology_init_tmpmask2();
        lupos_topology_build_fallback_alloc();
        lupos_topology_build_arch_update();
        asym_cpu_capacity_scan();
        lupos_topology_build_ndoms_cur_set(1);
        let mut doms = alloc_sched_domains(lupos_topology_build_ndoms_cur() as c_uint);
        if doms.is_null() { doms = lupos_topology_build_fallback(); }
        lupos_topology_build_doms_cur_set(doms);
        let mask = lupos_topology_build_mask(doms, 0);
        lupos_topology_mask_and(mask, cpu_map, lupos_topology_build_housekeeping_mask());
        let mut multi_llcs = false;
        let err = build_sched_domains(mask, null_mut(), &raw mut multi_llcs);
        if err == 0 { sched_cache_set(multi_llcs); }
        err
    }
}

/* Detach the CPU set, attaching its CPUs to the NULL domain. */
/// Detaches a live partition and transfers its CPUs to the default root domain.
///
/// # Safety
/// The caller must hold the domain mutex and stabilize CPU hotplug. `cpu_map`
/// must be a nonempty live mask of the partition being removed, with valid
/// runqueues and balanced asymmetry/cluster static-key state. The default root
/// domain must be initialized, and the caller must not hold any rq lock.
unsafe fn detach_destroy_domains(cpu_map: *const cpumask) {
    // SAFETY: The nonempty stable mask supplies a valid representative CPU.
    // Native RCU access inspects presence only; attachment/ref transfer and
    // deferred destruction run within the explicit balanced RCU read section.
    unsafe {
        let cpu = lupos_topology_build_mask_any(cpu_map) as c_int;
        if lupos_topology_build_asym_present(cpu) { lupos_topology_build_asym_dec(); }
        if lupos_topology_build_cluster_active() { lupos_topology_build_cluster_dec(); }
        lupos_topology_groups_rcu_read_lock();
        for i in topology_groups_cpus(cpu_map) { cpu_attach_domain(null_mut(), &raw mut def_root_domain, i); }
        lupos_topology_groups_rcu_read_unlock();
    }
}

/* Handle null as "default". */
/// Compares optional native attributes, substituting the native default value.
///
/// # Safety
/// Each non-null array must remain readable at its respective nonnegative
/// index. Callers must prevent concurrent mutation of the compared attributes.
unsafe fn dattrs_equal(cur: *mut sched_domain_attr, idx_cur: c_int,
                      new: *mut sched_domain_attr, idx_new: c_int) -> c_int {
    // SAFETY: The caller bounds non-null array accesses. The native initializer
    // writes the stack default before native memcmp reads exactly its C size;
    // no guessed Rust layout or uninitialized Rust value is materialized.
    unsafe {
        if new.is_null() && cur.is_null() { return 1; }
        let mut tmp = core::mem::MaybeUninit::<sched_domain_attr>::uninit();
        lupos_topology_build_attr_init(tmp.as_mut_ptr());
        (lupos_topology_build_attr_compare(
            if cur.is_null() { tmp.as_ptr() } else { cur.add(idx_cur as usize) },
            if new.is_null() { tmp.as_ptr() } else { new.add(idx_new as usize) },
        ) == 0) as c_int
    }
}

/*
 * Partition into the supplied nonoverlapping cpumasks. Ownership of doms_new
 * transfers here; matching old partitions are reused. NULL requests a default
 * active/housekeeping partition, with fallback_doms on allocation failure;
 * ndoms_new == 0 only destroys existing domains. The hotplug lock and
 * sched_domains_mutex must be held.
 */
/// Replaces partition ownership, reusing unchanged domains where allowed.
///
/// # Safety
/// The caller must hold the domain mutex and hotplug serialization in sleepable
/// context. `ndoms_new` must be nonnegative. A non-null `doms_new` must be a fresh
/// owned native array of that length with initialized, nonempty, disjoint masks
/// of valid CPUs; null is valid only for the native zero/one-partition requests.
/// `dattr_new` must be null or an owned readable native array of matching length,
/// and must be null when `doms_new` is null. Neither allocation may alias current
/// partition/attribute storage. Ownership of both supplied allocations transfers
/// here. Current partitions, scratch state and scheduler objects must be valid.
unsafe fn partition_sched_domains_locked(ndoms_new: c_int, mut doms_new: *mut cpumask_var_t,
                                        dattr_new: *mut sched_domain_attr) {
    // SAFETY: Both locks stabilize old/new masks, attribute arrays and runqueues.
    // Reused domain trees are inspected inside RCU; new domains claim their own
    // allocations. Fallback storage is never freed. Old arrays are released only
    // after their final comparison, then the new owned arrays are installed.
    unsafe {
        let mut has_multi_llcs = false;
        lupos_topology_assert_domains_locked();
        /* Let the architecture update core mappings and capacity asymmetry. */
        let new_topology = lupos_topology_build_arch_update();
        if new_topology != 0 { asym_cpu_capacity_scan(); }
        let mut n;
        if doms_new.is_null() {
            lupos_topology_build_warn_attrs(!dattr_new.is_null());
            n = 0;
            doms_new = alloc_sched_domains(1);
            if !doms_new.is_null() {
                n = 1;
                lupos_topology_mask_and(lupos_topology_build_mask(doms_new, 0),
                    lupos_topology_active_mask(), lupos_topology_build_housekeeping_mask());
            }
        } else { n = ndoms_new; }
        let doms_cur = lupos_topology_build_doms_cur();
        let ndoms_cur = lupos_topology_build_ndoms_cur();
        let dattr_cur = lupos_topology_build_dattr_cur();
        /* Destroy deleted domains. */
        for i in 0..ndoms_cur {
            'match1: {
                for j in 0..n {
                    if new_topology != 0 { break; }
                    if lupos_topology_mask_equal(lupos_topology_build_mask(doms_cur, i as c_uint),
                                                lupos_topology_build_mask(doms_new, j as c_uint))
                        && dattrs_equal(dattr_cur, i, dattr_new, j) != 0 {
                        break 'match1;
                    }
                }
                detach_destroy_domains(lupos_topology_build_mask(doms_cur, i as c_uint));
            }
        }
        n = ndoms_cur;
        if doms_new.is_null() {
            n = 0;
            doms_new = lupos_topology_build_fallback();
            lupos_topology_mask_and(lupos_topology_build_mask(doms_new, 0),
                lupos_topology_active_mask(), lupos_topology_build_housekeeping_mask());
        }
        /* Build new domains. */
        for i in 0..ndoms_new {
            'match2: {
                for j in 0..n {
                    if new_topology != 0 { break; }
                    if lupos_topology_mask_equal(lupos_topology_build_mask(doms_new, i as c_uint),
                                                lupos_topology_build_mask(doms_cur, j as c_uint))
                        && dattrs_equal(dattr_new, i, dattr_cur, j) != 0 {
                        /*
                         * Reused multi-LLC partitions still count. Otherwise a
                         * newly built single-LLC partition could disable cache
                         * scheduling globally despite an active multi-LLC one.
                         */
                        let cpu = lupos_topology_mask_first(lupos_topology_build_mask(doms_cur, j as c_uint)) as c_int;
                        lupos_topology_groups_rcu_read_lock();
                        let mut sd = lupos_topology_build_cpu_domain(cpu);
                        while !sd.is_null() && !(*sd).parent.is_null()
                            && (*(*sd).parent).flags & SD_SHARE_LLC != 0 {
                            sd = (*sd).parent;
                        }
                        if !sd.is_null() && (*sd).flags & SD_SHARE_LLC != 0
                            && !(*sd).parent.is_null() && sd_in_multi_llcs(sd) {
                            has_multi_llcs = true;
                        }
                        lupos_topology_groups_rcu_read_unlock();
                        break 'match2;
                    }
                }
                let mut multi_llcs = false;
                build_sched_domains(lupos_topology_build_mask(doms_new, i as c_uint),
                    if dattr_new.is_null() { null_mut() } else { dattr_new.add(i as usize) },
                    &raw mut multi_llcs);
                has_multi_llcs |= multi_llcs;
            }
        }
        sched_cache_set(has_multi_llcs);
        #[cfg(all(CONFIG_ENERGY_MODEL, CONFIG_CPU_FREQ_GOV_SCHEDUTIL))]
        {
            let mut has_eas = false;
            /* Build perf domains, retaining matching domains unless EAS updates. */
            for i in 0..ndoms_new {
                'match3: {
                    for j in 0..n {
                        if lupos_topology_energy_update() { break; }
                        if lupos_topology_mask_equal(lupos_topology_build_mask(doms_new, i as c_uint),
                                                    lupos_topology_build_mask(doms_cur, j as c_uint)) {
                            let cpu = lupos_topology_mask_first(lupos_topology_build_mask(doms_cur, j as c_uint)) as c_int;
                            if !(*(*lupos_topology_cpu_rq(cpu)).rd).pd.is_null() {
                                has_eas = true;
                                break 'match3;
                            }
                        }
                    }
                    has_eas |= build_perf_domains(lupos_topology_build_mask(doms_new, i as c_uint));
                }
            }
            sched_energy_set(has_eas);
        }
        /* Remember the new sched domains. */
        if doms_cur != lupos_topology_build_fallback() { free_sched_domains(doms_cur, ndoms_cur as c_uint); }
        lupos_topology_free(dattr_cur.cast());
        lupos_topology_build_doms_cur_set(doms_new);
        lupos_topology_build_dattr_cur_set(dattr_new);
        lupos_topology_build_ndoms_cur_set(ndoms_new);
        lupos_topology_build_update_debugfs();
        lupos_topology_build_rebuild_dl();
    }
}

/* Call with hotplug lock held. */
/// Repartitions scheduler domains while acquiring the domain mutex.
///
/// # Safety
/// The caller must hold CPU-hotplug serialization, be sleepable, and not hold
/// the scheduler-domain mutex. It must meet the valid-mask, allocation ownership,
/// count and attribute requirements of `partition_sched_domains_locked`; both
/// supplied allocations transfer to the scheduler and must not be reused.
#[no_mangle]
pub unsafe extern "C" fn partition_sched_domains(ndoms_new: c_int, doms_new: *mut cpumask_var_t,
                                                dattr_new: *mut sched_domain_attr) {
    // SAFETY: The caller supplies hotplug and input ownership guarantees; the
    // balanced mutex pair supplies the remaining serialization for replacement.
    unsafe {
        sched_domains_mutex_lock();
        partition_sched_domains_locked(ndoms_new, doms_new, dattr_new);
        sched_domains_mutex_unlock();
    }
}
