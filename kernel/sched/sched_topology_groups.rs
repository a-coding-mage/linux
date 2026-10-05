// SPDX-License-Identifier: GPL-2.0
// Source continuation of topology.rs:1066–1801 at
// 126a30fae3bba11420ec2fcbde51a0a01bab1b5b. Included lexically by topology.rs.
// Header operations below are explicit, unqualified runtime C boundaries.

// Lazy iteration is important: the core-count walk clears SMT siblings from
// its mask in the body, before the next CPU is selected.
unsafe fn topology_groups_cpus(mask: *const cpumask) -> impl Iterator<Item = c_int> {
    let mut cpu: c_int = -1;
    // SAFETY: The caller keeps the native mask live and protected for the iterator lifetime; each next call observes current mask contents.
    core::iter::from_fn(move || unsafe {
        let next = lupos_topology_mask_next(cpu, mask);
        if next >= lupos_topology_groups_mask_bits() {
            return None;
        }
        cpu = next as c_int;
        Some(cpu)
    })
}

unsafe fn topology_groups_cpus_wrap(
    mask: *const cpumask,
    start: c_int,
) -> impl Iterator<Item = c_int> {
    let mut previous = None;
    // SAFETY: The caller keeps the native mask live and protected for the iterator lifetime; each next call observes current mask contents.
    core::iter::from_fn(move || unsafe {
        let next = match previous {
            None => lupos_topology_groups_wrap_first(mask, start),
            Some(cpu) => lupos_topology_groups_wrap_next(mask, start, cpu),
        };
        if next >= lupos_topology_groups_mask_bits() {
            return None;
        }
        previous = Some(next as c_int);
        Some(next as c_int)
    })
}

/*
 * Return true if @sd belongs to an LLC group whose enclosing
 * partition spans more than one LLC. @sd must be the topmost
 * SD_SHARE_LLC domain.
 *
 * Any duplicated parent domains with the same span as @sd are
 * skipped: before cpu_attach_domain() degeneration these still
 * exist, after degeneration the loop is a no-op. This makes the
 * helper usable both during sched domain build and against an
 * already-attached domain tree.
 *
 * Note: For systems with a single LLC per node, cache-aware
 * scheduling is still enabled when multiple nodes exist.
 * However, NUMA balancing decisions take precedence over
 * cache-aware scheduling. Conversely, if there is only one
 * LLC per partition, cache-aware scheduling should be disabled.
 */
unsafe fn sd_in_multi_llcs(sd: *mut sched_domain) -> bool {
    // SAFETY: The caller pins the domain chain under the original topology exclusion.
    unsafe {
        let mut sdp = (*sd).parent;

        /* it does not make sense to aggregate to 1 CPU */
        if (*sd).span_weight == 1 {
            return false;
        }
        while !sdp.is_null() && (*sdp).span_weight == (*sd).span_weight {
            sdp = (*sdp).parent;
        }
        !sdp.is_null()
    }
}

/*
 * Return the canonical balance CPU for this group, this is the first CPU
 * of this group that's also in the balance mask.
 *
 * The balance mask are all those CPUs that could actually end up at this
 * group. See build_balance_mask().
 *
 * Also see should_we_balance().
 */
/// Return the native balance mask's first CPU.
///
/// # Safety
/// The caller keeps the group, capacity object and balance mask live and protected.
#[no_mangle]
pub unsafe extern "C" fn group_balance_cpu(sg: *mut sched_group) -> c_int {
    // SAFETY: The caller keeps sg, its capacity object and balance mask live for this read.
    unsafe { lupos_topology_mask_first(lupos_topology_balance_mask(sg)) as c_int }
}

/*
 * NUMA topology (first read the regular topology blurb below)
 *
 * Given a node-distance table, for example:
 *
 *   node   0   1   2   3
 *     0:  10  20  30  20
 *     1:  20  10  20  30
 *     2:  30  20  10  20
 *     3:  20  30  20  10
 *
 * which represents a 4 node ring topology like:
 *
 *   0 ----- 1
 *   |       |
 *   |       |
 *   |       |
 *   3 ----- 2
 *
 * We want to construct domains and groups to represent this. The way we go
 * about doing this is to build the domains on 'hops'. For each NUMA level we
 * construct the mask of all nodes reachable in @level hops.
 *
 * For the above NUMA topology that gives 3 levels:
 *
 * NUMA-2	0-3		0-3		0-3		0-3
 *  groups:	{0-1,3},{1-3}	{0-2},{0,2-3}	{1-3},{0-1,3}	{0,2-3},{0-2}
 *
 * NUMA-1	0-1,3		0-2		1-3		0,2-3
 *  groups:	{0},{1},{3}	{0},{1},{2}	{1},{2},{3}	{0},{2},{3}
 *
 * NUMA-0	0		1		2		3
 *
 *
 * As can be seen; things don't nicely line up as with the regular topology.
 * When we iterate a domain in child domain chunks some nodes can be
 * represented multiple times -- hence the "overlap" naming for this part of
 * the topology.
 *
 * In order to minimize this overlap, we only build enough groups to cover the
 * domain. For instance Node-0 NUMA-2 would only get groups: 0-1,3 and 1-3.
 *
 * Because:
 *
 *  - the first group of each domain is its child domain; this
 *    gets us the first 0-1,3
 *  - the only uncovered node is 2, who's child domain is 1-3.
 *
 * However, because of the overlap, computing a unique CPU for each group is
 * more complicated. Consider for instance the groups of NODE-1 NUMA-2, both
 * groups include the CPUs of Node-0, while those CPUs would not in fact ever
 * end up at those groups (they would end up in group: 0-1,3).
 *
 * To correct this we have to introduce the group balance mask. This mask
 * will contain those CPUs in the group that can reach this group given the
 * (child) domain tree.
 *
 * With this we can once again compute balance_cpu and sched_group_capacity
 * relations.
 *
 * XXX include words on how balance_cpu is unique and therefore can be
 * used for sched_group_capacity links.
 *
 *
 * Another 'interesting' topology is:
 *
 *   node   0   1   2   3
 *     0:  10  20  20  30
 *     1:  20  10  20  20
 *     2:  20  20  10  20
 *     3:  30  20  20  10
 *
 * Which looks a little like:
 *
 *   0 ----- 1
 *   |     / |
 *   |   /   |
 *   | /     |
 *   2 ----- 3
 *
 * This topology is asymmetric, nodes 1,2 are fully connected, but nodes 0,3
 * are not.
 *
 * This leads to a few particularly weird cases where the sched_domain's are
 * not of the same number for each CPU. Consider:
 *
 * NUMA-2	0-3						0-3
 *  groups:	{0-2},{1-3}					{1-3},{0-2}
 *
 * NUMA-1	0-2		0-3		0-3		1-3
 *
 * NUMA-0	0		1		2		3
 *
 */


/*
 * Build the balance mask; it contains only those CPUs that can arrive at this
 * group and should be considered to continue balancing.
 *
 * We do this during the group creation pass, therefore the group information
 * isn't complete yet, however since each group represents a (child) domain we
 * can fully construct this using the sched_domain bits (which are already
 * complete).
 */
unsafe fn build_balance_mask(
    sd: *mut sched_domain,
    sg: *mut sched_group,
    mask: *mut cpumask,
) {
    // SAFETY: Construction holds topology exclusion; span members have allocated per-CPU domain slots and the scratch mask is exclusively owned.
    unsafe {
        let sg_span = lupos_topology_group_span(sg);
        let sdd = lupos_topology_groups_sd_data(sd);

        lupos_topology_mask_clear(mask);
        for i in topology_groups_cpus(sg_span) {
            let sibling = lupos_topology_groups_sd(sdd, i);

            /*
             * Can happen in the asymmetric case, where these siblings are
             * unused. The mask will not be empty because those CPUs that
             * do have the top domain _should_ span the domain.
             */
            if (*sibling).child.is_null() {
                continue;
            }
            /* If we would not end up here, we can't continue from here */
            if !lupos_topology_mask_equal(
                sg_span, lupos_topology_domain_span((*sibling).child),
            ) {
                continue;
            }
            lupos_topology_groups_mask_set(i, mask);
        }
        /* We must not have empty masks here */
        lupos_topology_groups_warn_empty_balance(lupos_topology_mask_empty(mask));
    }
}

/*
 * XXX: This creates per-node group entries; since the load-balancer will
 * immediately access remote memory to construct this group's load-balance
 * statistics having the groups node local is of dubious benefit.
 */
unsafe fn build_group_from_child_sched_domain(
    sd: *mut sched_domain,
    cpu: c_int,
) -> *mut sched_group {
    // SAFETY: The sibling is live under topology exclusion; native allocation supplies the trailing cpumask and reference storage.
    unsafe {
        let sg = lupos_topology_groups_alloc(cpu);
        if sg.is_null() {
            return null_mut();
        }
        let sg_span = lupos_topology_group_span(sg);
        if !(*sd).child.is_null() {
            lupos_topology_groups_mask_copy(sg_span, lupos_topology_domain_span((*sd).child));
            (*sg).flags = (*(*sd).child).flags;
        } else {
            lupos_topology_groups_mask_copy(sg_span, lupos_topology_domain_span(sd));
        }
        lupos_topology_groups_ref_inc(sg);
        sg
    }
}

unsafe fn init_overlap_sched_group(sd: *mut sched_domain, sg: *mut sched_group) {
    // SAFETY: Construction owns sd and sg; referenced per-CPU capacity slots exist, and native atomics arbitrate shared initialization.
    unsafe {
        let mask = lupos_topology_tmpmask2();
        let sdd = lupos_topology_groups_sd_data(sd);

        build_balance_mask(sd, sg, mask);
        let cpu = lupos_topology_mask_first(mask) as c_int;
        (*sg).sgc = lupos_topology_groups_sgc(sdd, cpu);
        if lupos_topology_groups_capacity_ref_inc_return((*sg).sgc) == 1 {
            lupos_topology_groups_mask_copy(lupos_topology_balance_mask(sg), mask);
        } else {
            lupos_topology_groups_warn_balance_mismatch(
                !lupos_topology_mask_equal(lupos_topology_balance_mask(sg), mask),
            );
        }

        /*
         * Initialize sgc->capacity such that even if we mess up the
         * domains and no possible iteration will get us here, we won't
         * die on a /0 trap.
         */
        let sg_span = lupos_topology_group_span(sg);
        (*(*sg).sgc).capacity = (LUPOS_TOPOLOGY_CAPACITY_SCALE as c_ulong)
            .wrapping_mul(lupos_topology_mask_weight(sg_span) as c_ulong);
        (*(*sg).sgc).min_capacity = LUPOS_TOPOLOGY_CAPACITY_SCALE as c_ulong;
        (*(*sg).sgc).max_capacity = LUPOS_TOPOLOGY_CAPACITY_SCALE as c_ulong;
    }
}

unsafe fn find_descended_sibling(
    sd: *mut sched_domain,
    mut sibling: *mut sched_domain,
) -> *mut sched_domain {
    // SAFETY: Both domain chains stay live under topology construction exclusion; no pointer escapes that lifetime.
    unsafe {
        /* The proper descendant's child must not span out of sd. */
        while !(*sibling).child.is_null()
            && !lupos_topology_mask_subset(
                lupos_topology_domain_span((*sibling).child), lupos_topology_domain_span(sd),
            )
        {
            sibling = (*sibling).child;
        }

        /*
         * As we are referencing sgc across different topology level, we need
         * to go down to skip those sched_domains which don't contribute to
         * scheduling because they will be degenerated in cpu_attach_domain.
         */
        while !(*sibling).child.is_null()
            && lupos_topology_mask_equal(
                lupos_topology_domain_span((*sibling).child),
                lupos_topology_domain_span(sibling),
            )
        {
            sibling = (*sibling).child;
        }
        sibling
    }
}

unsafe fn build_overlap_sched_groups(sd: *mut sched_domain, cpu: c_int) -> c_int {
    // SAFETY: The caller owns this unpublished domain and serialized scratch masks; native allocation/refcount primitives govern group cleanup.
    unsafe {
        // Keep first in the failure-cleanup scope, unlike the damaged input.
        let mut first: *mut sched_group = null_mut();
        let mut last: *mut sched_group = null_mut();
        let span = lupos_topology_domain_span(sd);
        let covered = lupos_topology_tmpmask();
        let sdd = lupos_topology_groups_sd_data(sd);

        'fail: {
            lupos_topology_mask_clear(covered);
            for i in topology_groups_cpus_wrap(span, cpu) {
                if lupos_topology_mask_test(i, covered) {
                    continue;
                }
                let mut sibling = lupos_topology_groups_sd(sdd, i);
		/*
		 * Asymmetric node setups can result in situations where the
		 * domain tree is of unequal depth, make sure to skip domains
		 * that already cover the entire range.
		 *
		 * In that case build_sched_domains() will have terminated the
		 * iteration early and our sibling sd spans will be empty.
		 * Domains should always include the CPU they're built on, so
		 * check that.
		 */
                if !lupos_topology_mask_test(i, lupos_topology_domain_span(sibling)) {
                    continue;
                }
		/*
		 * Usually we build sched_group by sibling's child sched_domain
		 * But for machines whose NUMA diameter are 3 or above, we move
		 * to build sched_group by sibling's proper descendant's child
		 * domain because sibling's child sched_domain will span out of
		 * the sched_domain being built as below.
		 *
		 * Smallest diameter=3 topology is:
		 *
		 *   node   0   1   2   3
		 *     0:  10  20  30  40
		 *     1:  20  10  20  30
		 *     2:  30  20  10  20
		 *     3:  40  30  20  10
		 *
		 *   0 --- 1 --- 2 --- 3
		 *
		 * NUMA-3       0-3             N/A             N/A             0-3
		 *  groups:     {0-2},{1-3}                                     {1-3},{0-2}
		 *
		 * NUMA-2       0-2             0-3             0-3             1-3
		 *  groups:     {0-1},{1-3}     {0-2},{2-3}     {1-3},{0-1}     {2-3},{0-2}
		 *
		 * NUMA-1       0-1             0-2             1-3             2-3
		 *  groups:     {0},{1}         {1},{2},{0}     {2},{3},{1}     {3},{2}
		 *
		 * NUMA-0       0               1               2               3
		 *
		 * The NUMA-2 groups for nodes 0 and 3 are obviously buggered, as the
		 * group span isn't a subset of the domain span.
		 */
                if !(*sibling).child.is_null()
                    && !lupos_topology_mask_subset(
                        lupos_topology_domain_span((*sibling).child), span,
                    )
                {
                    sibling = find_descended_sibling(sd, sibling);
                }
                let sg = build_group_from_child_sched_domain(sibling, cpu);
                if sg.is_null() {
                    break 'fail;
                }
                let sg_span = lupos_topology_group_span(sg);
                lupos_topology_mask_or(covered, covered, sg_span);
                init_overlap_sched_group(sibling, sg);

                if first.is_null() {
                    first = sg;
                }
                if !last.is_null() {
                    (*last).next = sg;
                }
                last = sg;
                (*last).next = first;
            }
            (*sd).groups = first;
            return 0;
        }
        free_sched_groups(first, 0);
        -(LUPOS_TOPOLOGY_ENOMEM as c_int)
    }
}

/*
 * Package topology (also see the load-balance blurb in fair.c)
 *
 * The scheduler builds a tree structure to represent a number of important
 * topology features. By default (default_topology[]) these include:
 *
 *  - Simultaneous multithreading (SMT)
 *  - Multi-Core Cache (MC)
 *  - Package (PKG)
 *
 * Where the last one more or less denotes everything up to a NUMA node.
 *
 * The tree consists of 3 primary data structures:
 *
 *	sched_domain -> sched_group -> sched_group_capacity
 *	    ^ ^             ^ ^
 *          `-'             `-'
 *
 * The sched_domains are per-CPU and have a two way link (parent & child) and
 * denote the ever growing mask of CPUs belonging to that level of topology.
 *
 * Each sched_domain has a circular (double) linked list of sched_group's, each
 * denoting the domains of the level below (or individual CPUs in case of the
 * first domain level). The sched_group linked by a sched_domain includes the
 * CPU of that sched_domain [*].
 *
 * Take for instance a 2 threaded, 2 core, 2 cache cluster part:
 *
 * CPU   0   1   2   3   4   5   6   7
 *
 * PKG  [                             ]
 * MC   [             ] [             ]
 * SMT  [     ] [     ] [     ] [     ]
 *
 *  - or -
 *
 * PKG  0-7 0-7 0-7 0-7 0-7 0-7 0-7 0-7
 * MC	0-3 0-3 0-3 0-3 4-7 4-7 4-7 4-7
 * SMT  0-1 0-1 2-3 2-3 4-5 4-5 6-7 6-7
 *
 * CPU   0   1   2   3   4   5   6   7
 *
 * One way to think about it is: sched_domain moves you up and down among these
 * topology levels, while sched_group moves you sideways through it, at child
 * domain granularity.
 *
 * sched_group_capacity ensures each unique sched_group has shared storage.
 *
 * There are two related construction problems, both require a CPU that
 * uniquely identify each group (for a given domain):
 *
 *  - The first is the balance_cpu (see should_we_balance() and the
 *    load-balance blurb in fair.c); for each group we only want 1 CPU to
 *    continue balancing at a higher domain.
 *
 *  - The second is the sched_group_capacity; we want all identical groups
 *    to share a single sched_group_capacity.
 *
 * Since these topologies are exclusive by construction. That is, its
 * impossible for an SMT thread to belong to multiple cores, and cores to
 * be part of multiple caches. There is a very clear and unique location
 * for each CPU in the hierarchy.
 *
 * Therefore computing a unique CPU for each group is trivial (the iteration
 * mask is redundant and set all 1s; all CPUs in a group will end up at _that_
 * group), we can simply pick the first CPU in each group.
 *
 *
 * [*] in other words, the first group of each domain is its child domain.
 */
unsafe fn get_group(mut cpu: c_int, sdd: *mut sd_data) -> *mut sched_group {
    // SAFETY: The caller holds topology exclusion and supplies initialized native per-CPU domain/group/capacity slots.
    unsafe {
        let sd = lupos_topology_groups_sd(sdd, cpu);
        let child = (*sd).child;
        if !child.is_null() {
            cpu = lupos_topology_mask_first(lupos_topology_domain_span(child)) as c_int;
        }
        let sg = lupos_topology_groups_sg(sdd, cpu);
        (*sg).sgc = lupos_topology_groups_sgc(sdd, cpu);

        /* Increase refcounts for claim_allocations: */
        let already_visited = lupos_topology_groups_ref_inc_return(sg) > 1;
        /* sgc visits should follow a similar trend as sg */
        lupos_topology_groups_warn_refcount_mismatch(
            already_visited != (lupos_topology_groups_capacity_ref_inc_return((*sg).sgc) > 1),
        );
        /* If we have already visited that group, it's already initialized. */
        if already_visited {
            return sg;
        }
        if !child.is_null() {
            lupos_topology_groups_mask_copy(
                lupos_topology_group_span(sg), lupos_topology_domain_span(child),
            );
            lupos_topology_groups_mask_copy(
                lupos_topology_balance_mask(sg), lupos_topology_group_span(sg),
            );
            (*sg).flags = (*child).flags;
        } else {
            lupos_topology_groups_mask_set(cpu, lupos_topology_group_span(sg));
            lupos_topology_groups_mask_set(cpu, lupos_topology_balance_mask(sg));
        }
        (*(*sg).sgc).capacity = (LUPOS_TOPOLOGY_CAPACITY_SCALE as c_ulong)
            .wrapping_mul(lupos_topology_mask_weight(lupos_topology_group_span(sg)) as c_ulong);
        (*(*sg).sgc).min_capacity = LUPOS_TOPOLOGY_CAPACITY_SCALE as c_ulong;
        (*(*sg).sgc).max_capacity = LUPOS_TOPOLOGY_CAPACITY_SCALE as c_ulong;
        sg
    }
}

/*
 * build_sched_groups will build a circular linked list of the groups
 * covered by the given span, will set each group's ->cpumask correctly,
 * and will initialize their ->sgc.
 *
 * Assumes the sched_domain tree is fully constructed
 */
unsafe fn build_sched_groups(sd: *mut sched_domain, cpu: c_int) -> c_int {
    // SAFETY: The domains mutex protects scratch masks and the unpublished groups; the native assertion preserves the original lock contract.
    unsafe {
        let mut first: *mut sched_group = null_mut();
        let mut last: *mut sched_group = null_mut();
        let sdd = lupos_topology_groups_sd_data(sd);
        let span = lupos_topology_domain_span(sd);

        lupos_topology_assert_domains_locked();
        let covered = lupos_topology_tmpmask();
        lupos_topology_mask_clear(covered);
        for i in topology_groups_cpus_wrap(span, cpu) {
            if lupos_topology_mask_test(i, covered) {
                continue;
            }
            let sg = get_group(i, sdd);
            lupos_topology_mask_or(covered, covered, lupos_topology_group_span(sg));
            if first.is_null() {
                first = sg;
            }
            if !last.is_null() {
                (*last).next = sg;
            }
            last = sg;
        }
        (*last).next = first;
        (*sd).groups = first;
        0
    }
}

/*
 * Initialize sched groups cpu_capacity.
 *
 * cpu_capacity indicates the capacity of sched group, which is used while
 * distributing the load between different sched groups in a sched domain.
 * Typically cpu_capacity for all the groups in a sched domain will be same
 * unless there are asymmetries in the topology. If there are asymmetries,
 * group having more cpu_capacity will pickup more load compared to the
 * group having less cpu_capacity.
 */
unsafe fn init_sched_groups_capacity(cpu: c_int, sd: *mut sched_domain) {
    // SAFETY: The caller pins the completed circular group list and owns the scratch mask under topology exclusion.
    unsafe {
        let mut sg = (*sd).groups;
        let mask = lupos_topology_tmpmask2();

        lupos_topology_groups_warn_missing_group(sg.is_null());
        loop {
            'next: {
                let mut cores: c_int = 0;
                let mut max_cpu: c_int = -1;

                (*sg).group_weight = lupos_topology_mask_weight(lupos_topology_group_span(sg));
                lupos_topology_groups_mask_copy(mask, lupos_topology_group_span(sg));
                for cpu in topology_groups_cpus(mask) {
                    cores += 1;
                    lupos_topology_groups_mask_andnot(
                        mask, mask, lupos_topology_groups_smt_mask(cpu),
                    );
                }
                (*sg).cores = cores as c_uint;
                if (*sd).flags & (SD_ASYM_PACKING as c_int) == 0 {
                    break 'next;
                }
                for cpu in topology_groups_cpus(lupos_topology_group_span(sg)) {
                    if max_cpu < 0 {
                        max_cpu = cpu;
                    } else if lupos_topology_groups_asym_prefer(cpu, max_cpu) {
                        max_cpu = cpu;
                    }
                }
                (*sg).asym_prefer_cpu = max_cpu;
            }
            sg = (*sg).next;
            if sg == (*sd).groups {
                break;
            }
        }
        if cpu != group_balance_cpu(sg) {
            return;
        }
        update_group_capacity(sd, cpu);
    }
}

// Equivalent to guard(rcu)(): every return from the public update releases it.
struct TopologyGroupsRcuGuard;

impl Drop for TopologyGroupsRcuGuard {
    fn drop(&mut self) {
        // SAFETY: This guard is created only after the matching native RCU read lock, and drops in the same context.
        unsafe { lupos_topology_groups_rcu_read_unlock(); }
    }
}

/* Update the "asym_prefer_cpu" when arch_asym_cpu_priority() changes. */
/// Refresh the preferred CPU after a native architecture-priority change.
///
/// # Safety
/// The CPU and priority values must describe the requested native topology update.
/// The caller obeys the scheduler's hotplug and priority-update serialization.
#[no_mangle]
pub unsafe extern "C" fn sched_update_asym_prefer_cpu(
    cpu: c_int,
    old_prio: c_int,
    new_prio: c_int,
) {
    // SAFETY: The caller supplies a valid CPU; the native RCU lock and guard pin the domain/group hierarchy across every early return.
    unsafe {
        let mut asym_prefer_cpu = cpu;
        lupos_topology_groups_rcu_read_lock();
        let _guard = TopologyGroupsRcuGuard;
        let mut sd = lupos_topology_groups_cpu_domain(cpu);

        while !sd.is_null() {
            'domain: {
                if (*sd).flags & (SD_ASYM_PACKING as c_int) == 0 {
                    break 'domain;
                }
                /*
                 * Groups of overlapping domain are replicated per NUMA
                 * node and will require updating "asym_prefer_cpu" on
                 * each local copy.
                 *
                 * If you are hitting this warning, consider moving
                 * "sg->asym_prefer_cpu" to "sg->sgc->asym_prefer_cpu"
                 * which is shared by all the overlapping groups.
                 */
                lupos_topology_groups_warn_numa_preference(
                    (*sd).flags & (SD_NUMA as c_int) != 0,
                );
                let sg = (*sd).groups;
                if cpu != (*sg).asym_prefer_cpu {
                    /*
                     * Since the parent is a superset of the current group,
                     * if the cpu is not the "asym_prefer_cpu" at the
                     * current level, it cannot be the preferred CPU at a
                     * higher level either.
                     */
                    if !lupos_topology_groups_asym_prefer(cpu, (*sg).asym_prefer_cpu) {
                        return;
                    }
                    lupos_topology_groups_write_prefer_cpu(sg, cpu);
                    break 'domain;
                }
                /* Ranking has improved; CPU is still the preferred one. */
                if new_prio >= old_prio {
                    break 'domain;
                }
                for group_cpu in topology_groups_cpus(lupos_topology_group_span(sg)) {
                    if lupos_topology_groups_asym_prefer(group_cpu, asym_prefer_cpu) {
                        asym_prefer_cpu = group_cpu;
                    }
                }
                lupos_topology_groups_write_prefer_cpu(sg, asym_prefer_cpu);
            }
            sd = (*sd).parent;
        }
    }
}

/*
 * Set of available CPUs grouped by their corresponding capacities
 * Each list entry contains a CPU mask reflecting CPUs that share the same
 * capacity.
 * The lifespan of data is unlimited.
 */
// The native groups companion owns the original LIST_HEAD(asym_cap_list).
// Its list_head and asym_cap_data layouts come only from sched.h.

/*
 * Verify whether there is any CPU capacity asymmetry in a given sched domain.
 * Provides sd_flags reflecting the asymmetry scope.
 */
#[inline]
unsafe fn asym_cpu_capacity_classify(
    sd_span: *const cpumask,
    cpu_map: *const cpumask,
) -> c_int {
    // SAFETY: The caller serializes topology construction and keeps both masks and the capacity list live while it is traversed.
    unsafe {
        let mut count: c_int = 0;
        let mut miss: c_int = 0;
        let head = lupos_topology_groups_asym_head();
        let mut link = (*head).next;

        /*
         * Count how many unique CPU capacities this domain spans across
         * (compare sched_domain CPUs mask with ones representing available
         * CPUs capacities). Take into account CPUs that might be offline:
         * skip those.
         */
        while link != head {
            let entry = lupos_topology_groups_asym_entry(link);
            if lupos_topology_mask_intersects(
                sd_span, lupos_topology_groups_capacity_span(entry),
            ) {
                count += 1;
            } else if lupos_topology_mask_intersects(
                cpu_map, lupos_topology_groups_capacity_span(entry),
            ) {
                miss += 1;
            }
            link = (*link).next;
        }
        lupos_topology_groups_warn_missing_capacity(
            count == 0 && !lupos_topology_groups_asym_empty(),
        );
        /* No asymmetry detected */
        if count < 2 {
            return 0;
        }
        /* Some of the available CPU capacity values have not been detected */
        if miss != 0 {
            return SD_ASYM_CPUCAPACITY as c_int;
        }
        /* Full asymmetry */
        (SD_ASYM_CPUCAPACITY | SD_ASYM_CPUCAPACITY_FULL) as c_int
    }
}

// Native types.h spells struct rcu_head through #define rcu_head callback_head.
unsafe extern "C" fn free_asym_cap_entry(head: *mut callback_head) {
    // SAFETY: RCU invokes this callback once after the removed native capacity entry is no longer visible to readers.
    unsafe {
        let entry = lupos_topology_groups_asym_from_rcu(head);
        lupos_topology_groups_asym_free(entry);
    }
}

#[inline]
unsafe fn asym_cpu_capacity_update_data(cpu: c_int) {
    // SAFETY: Topology exclusion serializes capacity-list mutation; native allocation, RCU insertion and cpumask storage retain their original contracts.
    unsafe {
        let capacity = lupos_topology_groups_cpu_capacity(cpu);
        // Use the predecessor link, which can be the list head. This keeps
        // list_prev_entry's insertion semantics without a fictitious Rust
        // asym_cap_data projection from the head sentinel.
        let mut insert_link: *mut list_head = null_mut();
        let head = lupos_topology_groups_asym_head();
        let entry = 'done: {
            let mut link = (*head).next;
            /*
             * Search if capacity already exists. If not, track the entry
             * where we should insert to keep the list ordered descending.
             */
            while link != head {
                let entry = lupos_topology_groups_asym_entry(link);
                if capacity == (*entry).capacity {
                    break 'done entry;
                } else if insert_link.is_null() && capacity > (*entry).capacity {
                    insert_link = (*link).prev;
                }
                link = (*link).next;
            }
            let entry = lupos_topology_groups_asym_alloc();
            if lupos_topology_groups_warn_asym_alloc(entry.is_null()) {
                return;
            }
            (*entry).capacity = capacity;
            /* If NULL then the new capacity is the smallest, add last. */
            if insert_link.is_null() {
                lupos_topology_groups_asym_add_tail(entry);
            } else {
                lupos_topology_groups_asym_add_after(entry, insert_link);
            }
            entry
        };
        lupos_topology_groups_mask_set_nonatomic(
            cpu, lupos_topology_groups_capacity_span(entry),
        );
    }
}

/*
 * Build-up/update list of CPUs grouped by their capacities
 * An update requires explicit request to rebuild sched domains
 * with state indicating CPU topology changes.
 */
unsafe fn asym_cpu_capacity_scan() {
    // SAFETY: Topology exclusion serializes this update; each successor is saved before deletion and RCU delays entry reclamation.
    unsafe {
        let head = lupos_topology_groups_asym_head();
        let mut link = (*head).next;
        while link != head {
            let entry = lupos_topology_groups_asym_entry(link);
            lupos_topology_mask_clear(lupos_topology_groups_capacity_span(entry));
            link = (*link).next;
        }

        let mut cpu: c_int = -1;
        loop {
            let next = lupos_topology_groups_next_capacity_cpu(cpu);
            if next >= lupos_topology_groups_mask_bits() {
                break;
            }
            cpu = next as c_int;
            asym_cpu_capacity_update_data(cpu);
        }

        link = (*head).next;
        while link != head {
            // Match list_for_each_entry_safe: save the successor before
            // list_del_rcu/call_rcu, then never read the retired entry again.
            let next = (*link).next;
            let entry = lupos_topology_groups_asym_entry(link);
            if lupos_topology_mask_empty(lupos_topology_groups_capacity_span(entry)) {
                lupos_topology_groups_asym_del(entry);
                lupos_topology_groups_call_rcu(
                    core::ptr::addr_of_mut!((*entry).rcu), Some(free_asym_cap_entry),
                );
            }
            link = next;
        }

        /*
         * Only one capacity value has been detected i.e. this system is symmetric.
         * No need to keep this data around.
         */
        if lupos_topology_groups_asym_singular() {
            let entry = lupos_topology_groups_asym_entry((*head).next);
            lupos_topology_groups_asym_del(entry);
            lupos_topology_groups_call_rcu(
                core::ptr::addr_of_mut!((*entry).rcu), Some(free_asym_cap_entry),
            );
        }
    }
}
