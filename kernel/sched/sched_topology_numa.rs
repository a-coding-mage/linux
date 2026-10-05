// SPDX-License-Identifier: GPL-2.0
/*
 * Continuation of topology.rs's NUMA state and cpu_numa_flags through
 * sched_numa_hop_mask at baseline 126a30fae3bba11420ec2fcbde51a0a01bab1b5b.
 * This fragment is included in topology.rs's lexical scope. Native headers
 * own the layouts; native leaves retain the original NUMA state, RCU,
 * bitmaps, architecture weak alias and export registrations.
 */

#[cfg(CONFIG_NUMA)]
compile_error!("SOURCE ONLY HOLD: scheduler topology NUMA is not admitted");

/* Preserve the original for_each_node_state(N_CPU), excluding offline_node. */
#[cfg(CONFIG_NUMA)]
macro_rules! for_each_cpu_node_but {
    ($node:ident, $offline:expr, $body:block) => {{
        let mut next_node = lupos_topology_numa_next_cpu_node(-1);
        while next_node < lupos_topology_numa_max_num_nodes() {
            let $node = next_node as kernel::ffi::c_int;
            next_node = lupos_topology_numa_next_cpu_node($node);
            if $node == $offline {
                continue;
            }
            $body
        }
    }};
}

/* The original __free(bitmap) releases the temporary on every return path. */
#[cfg(CONFIG_NUMA)]
struct LuposTopologyNumaBitmap(*mut kernel::ffi::c_ulong);

#[cfg(CONFIG_NUMA)]
impl Drop for LuposTopologyNumaBitmap {
    fn drop(&mut self) {
        if !self.0.is_null() {
            // SAFETY: This private guard uniquely owns the non-NULL result
            // of native bitmap_alloc; no pointer is retained after Drop.
            unsafe { lupos_topology_numa_bitmap_free(self.0) }
        }
    }
}

/// Supply the native flags for a NUMA topology level.
///
/// # Safety
/// Must use the configured native sched_domain_flags_f callback ABI. The
/// callback has no pointer, locking or lifetime preconditions of its own.
#[cfg(CONFIG_NUMA)]
unsafe extern "C" fn cpu_numa_flags() -> kernel::ffi::c_int {
    // SAFETY: The leaf returns the configured native SD_NUMA constant.
    unsafe { lupos_topology_numa_sd_flags() }
}

/// Borrow a CPU's node mask while constructing a NUMA domain.
///
/// # Safety
/// tl must point to a live native topology level with numa_level in the
/// completed NUMA array's bounds. cpu must map to a populated node entry.
/// The caller must exclude topology reset and retain that exclusion while
/// using the returned borrowed mask; the callback does not acquire RCU.
#[cfg(CONFIG_NUMA)]
unsafe extern "C" fn sd_numa_mask(
    tl: *mut sched_domain_topology_level,
    cpu: kernel::ffi::c_int,
) -> *const cpumask {
    // SAFETY: Domain construction supplies a valid topology level and holds
    // the original topology/hotplug exclusion over the NUMA mask arrays.
    unsafe {
        let masks = lupos_topology_numa_masks();
        *(*masks.add((*tl).numa_level as usize))
            .add(lupos_topology_numa_cpu_to_node(cpu) as usize)
    }
}

/// Print the physical-distance matrix once on the serialized setup path.
///
/// # Safety
/// message must point to a live NUL-terminated native string for this call.
/// The caller must serialize all calls and retain stable native node data;
/// the private DONE state is not synchronized for concurrent writers.
#[cfg(CONFIG_NUMA)]
unsafe fn sched_numa_warn(message: *const kernel::ffi::c_char) {
    // SAFETY: Called from the serialized topology construction path. The
    // message is NUL-terminated; native leaves retain printk formatting.
    unsafe {
        static mut DONE: bool = false;
        if DONE {
            return;
        }
        DONE = true;

        lupos_topology_numa_warn_begin(message);
        let mut i = 0;
        while (i as kernel::ffi::c_uint) < lupos_topology_numa_nr_node_ids() {
            lupos_topology_numa_warn_row_begin();
            let mut j = 0;
            while (j as kernel::ffi::c_uint) < lupos_topology_numa_nr_node_ids() {
                let has_cpus = lupos_topology_numa_node_has_cpu(i)
                    && lupos_topology_numa_node_has_cpu(j);
                lupos_topology_numa_warn_distance(
                    lupos_topology_numa_node_distance(i, j), has_cpus,
                );
                j += 1;
            }
            lupos_topology_numa_warn_row_end();
            i += 1;
        }
        lupos_topology_numa_warn_end();
    }
}

/// Search the published physical NUMA distance list.
///
/// # Safety
/// Native node-distance data must be initialized, including node zero, and
/// the caller must be in a context permitting an RCU read-side section.
/// Published distance storage is borrowed only while that section is held.
#[cfg(CONFIG_NUMA)]
#[no_mangle]
pub unsafe extern "C" fn find_numa_distance(distance: kernel::ffi::c_int) -> bool {
    // SAFETY: Pointer acquisition and every distance-array read are within
    // the original RCU section; the labeled exit always reaches unlock.
    unsafe {
        let mut found = false;
        if distance == lupos_topology_numa_node_distance(0, 0) {
            return true;
        }

        lupos_topology_numa_rcu_read_lock();
        'unlock: {
            let distances = lupos_topology_numa_node_distances_dereference();
            if distances.is_null() {
                break 'unlock;
            }
            let mut i = 0;
            while i < lupos_topology_numa_node_levels() {
                if *distances.add(i as usize) == distance {
                    found = true;
                    break;
                }
                i += 1;
            }
        }
        lupos_topology_numa_rcu_read_unlock();
        found
    }
}

/*
 * DIRECT: all nodes directly connected (or not NUMA).
 * GLUELESS_MESH: distant nodes can communicate through an intermediary node.
 * BACKPLANE: otherwise, the distant nodes communicate through a backplane.
 * Preserve the original maximum-distance pair and intermediary-node search.
 */
/// Classify the completed NUMA topology using physical distances.
///
/// # Safety
/// The caller must serialize topology construction and hold node/hotplug
/// state stable. offline_node is NUMA_NO_NODE or the node being removed;
/// the published level count and maximum distance must describe this setup.
#[cfg(CONFIG_NUMA)]
unsafe fn init_numa_topology_type(offline_node: kernel::ffi::c_int) {
    // SAFETY: Caller holds topology/hotplug exclusion during initialization.
    unsafe {
        let n = lupos_topology_numa_max_distance();
        if lupos_topology_numa_levels() <= 2 {
            lupos_topology_numa_type_direct();
            return;
        }

        for_each_cpu_node_but!(a, offline_node, {
            for_each_cpu_node_but!(b, offline_node, {
                /* Find two nodes furthest removed from each other. */
                if lupos_topology_numa_node_distance(a, b) < n {
                    continue;
                }
                /* Is there an intermediary node between a and b? */
                for_each_cpu_node_but!(c, offline_node, {
                    if lupos_topology_numa_node_distance(a, c) < n
                        && lupos_topology_numa_node_distance(b, c) < n
                    {
                        lupos_topology_numa_type_mesh();
                        return;
                    }
                });
                lupos_topology_numa_type_backplane();
                return;
            });
        });

        lupos_topology_numa_warn_type();
        lupos_topology_numa_type_direct();
    }
}

/*
 * Architectures may override the scheduler distance to change node grouping.
 * The native weak alias remains the authority for detecting that override.
 */
/// Adapt the native physical-distance operation to the recording callback.
///
/// # Safety
/// i and j must be valid initialized native node IDs. The caller must keep
/// their distance data stable until the callback returns.
#[cfg(CONFIG_NUMA)]
unsafe extern "C" fn numa_node_dist(
    i: kernel::ffi::c_int,
    j: kernel::ffi::c_int,
) -> kernel::ffi::c_int {
    // SAFETY: The caller supplies valid native node identifiers.
    unsafe { lupos_topology_numa_node_distance(i, j) }
}

/// Ask the native alias-identity boundary whether an override is installed.
///
/// # Safety
/// The configured native weak symbol and its target must have the declared
/// int(int, int) ABI; this helper does not dereference Rust-owned data.
#[cfg(CONFIG_NUMA)]
unsafe fn modified_sched_node_distance() -> bool {
    // SAFETY: Native C preserves the alias/function-pointer comparison.
    unsafe { lupos_topology_numa_modified_distance() }
}

/// Allocate the sorted unique distances produced by the supplied callback.
///
/// # Safety
/// dist and levels must be distinct, aligned writable output slots that
/// remain live for the call. The caller must serialize node-state changes;
/// offline_node is NUMA_NO_NODE or a node to exclude. n_dist must accept
/// every remaining N_CPU node pair with the declared ABI and not unwind.
/// On success, the caller owns the returned native allocation and must
/// eventually release it with the matching native free operation. On
/// failure the output slots are unchanged; the temporary bitmap is local.
#[cfg(CONFIG_NUMA)]
unsafe fn sched_record_numa_dist(
    offline_node: kernel::ffi::c_int,
    n_dist: unsafe extern "C" fn(kernel::ffi::c_int, kernel::ffi::c_int) -> kernel::ffi::c_int,
    dist: *mut *mut kernel::ffi::c_int,
    levels: *mut kernel::ffi::c_int,
) -> kernel::ffi::c_int {
    // SAFETY: Caller supplies writable output slots and a valid distance
    // callback. Node iteration is protected by topology/hotplug exclusion.
    unsafe {
        /* O(nr_nodes^2) de-duplicating selection of unique distances. */
        let distance_map = LuposTopologyNumaBitmap(lupos_topology_numa_bitmap_alloc());
        if distance_map.0.is_null() {
            return -(LUPOS_TOPOLOGY_NUMA_ENOMEM as kernel::ffi::c_int);
        }
        lupos_topology_numa_bitmap_zero(distance_map.0);
        for_each_cpu_node_but!(i, offline_node, {
            for_each_cpu_node_but!(j, offline_node, {
                let distance = n_dist(i, j);
                if distance < LUPOS_TOPOLOGY_NUMA_LOCAL_DISTANCE as kernel::ffi::c_int
                    || distance >= LUPOS_TOPOLOGY_NUMA_NR_DISTANCE_VALUES as kernel::ffi::c_int
                {
                    sched_numa_warn(c"Invalid distance value range".as_ptr());
                    return -(LUPOS_TOPOLOGY_NUMA_EINVAL as kernel::ffi::c_int);
                }
                lupos_topology_numa_bitmap_set(distance_map.0, distance);
            });
        });

        /* Allocate one entry for each unique distance, in ascending order. */
        let nr_levels = lupos_topology_numa_bitmap_weight(distance_map.0)
            as kernel::ffi::c_int;
        let distances = lupos_topology_numa_distances_alloc(nr_levels);
        if distances.is_null() {
            return -(LUPOS_TOPOLOGY_NUMA_ENOMEM as kernel::ffi::c_int);
        }
        let mut j: kernel::ffi::c_int = 0;
        for i in 0..nr_levels {
            j = lupos_topology_numa_find_next_bit(distance_map.0, j as kernel::ffi::c_ulong)
                as kernel::ffi::c_int;
            *distances.add(i as usize) = j;
            j += 1;
        }
        *dist = distances;
        *levels = nr_levels;
        0
    }
}

/// Build NUMA distance arrays, masks and topology levels.
///
/// # Safety
/// The caller must provide boot-time exclusion or the native hotplug/setup
/// serialization, in a context permitting GFP_KERNEL allocation. The
/// previous NUMA state must be absent or reset, the base topology must be
/// live and NULL-mask-terminated, and node data must be initialized and
/// stable. offline_node is NUMA_NO_NODE or the node being removed; at least
/// one N_CPU node must remain, as required by the final-distance lookup.
#[cfg(CONFIG_NUMA)]
#[no_mangle]
pub unsafe extern "C" fn sched_init_numa(offline_node: kernel::ffi::c_int) {
    // SAFETY: Caller exclusion keeps node IDs, base topology and private
    // slots stable. Native allocators provide the original array dimensions;
    // every node index comes from N_CPU and fits nr_node_ids. At least one
    // distance exists. The fresh topology cannot overlap the live source,
    // so copying its initialized native entries is valid. RCU publication
    // retains the original order; partial-allocation returns are unchanged.
    unsafe {
        let mut distances = core::ptr::null_mut();
        let mut domain_distances = core::ptr::null_mut();
        let mut nr_levels = 0;
        let mut nr_node_levels = 0;

        /* Record the NUMA distances from SLIT table. */
        if sched_record_numa_dist(
            offline_node, numa_node_dist, &raw mut distances, &raw mut nr_node_levels,
        ) != 0 {
            return;
        }

        /* Record modified NUMA distances for building sched domains. */
        if modified_sched_node_distance() {
            if sched_record_numa_dist(
                offline_node, arch_sched_node_distance,
                &raw mut domain_distances, &raw mut nr_levels,
            ) != 0 {
                lupos_topology_numa_free(distances.cast());
                return;
            }
        } else {
            domain_distances = distances;
            nr_levels = nr_node_levels;
        }
        lupos_topology_numa_node_distances_assign(distances);
        lupos_topology_numa_max_distance_write_once(
            *distances.add((nr_node_levels - 1) as usize),
        );
        lupos_topology_numa_node_levels_write_once(nr_node_levels);

        /*
         * nr_levels is the number of unique scheduling distances. Reset the
         * published level count until all mask arrays are constructed: an
         * allocation failure must not expose partially allocated rows.
         */
        lupos_topology_numa_domain_distances_assign(domain_distances);
        lupos_topology_numa_levels_set(0);
        let masks = lupos_topology_numa_mask_levels_alloc(nr_levels);
        if masks.is_null() {
            return;
        }

        /* For each distance level, build one reachable-CPU mask per node. */
        for i in 0..nr_levels {
            *masks.add(i as usize) = lupos_topology_numa_mask_nodes_alloc();
            if (*masks.add(i as usize)).is_null() {
                return;
            }
            for_each_cpu_node_but!(j, offline_node, {
                let mask = lupos_topology_numa_mask_alloc();
                if mask.is_null() {
                    return;
                }
                *(*masks.add(i as usize)).add(j as usize) = mask;
                for_each_cpu_node_but!(k, offline_node, {
                    if sched_debug()
                        && arch_sched_node_distance(j, k) != arch_sched_node_distance(k, j)
                    {
                        sched_numa_warn(c"Node-distance not symmetric".as_ptr());
                    }
                    if arch_sched_node_distance(j, k) > lupos_topology_numa_domain_distance(i) {
                        continue;
                    }
                    lupos_topology_numa_mask_or(mask, mask, lupos_topology_numa_node_mask(k));
                });
            });
        }
        lupos_topology_numa_masks_assign(masks);

        /* Compute default topology size. The composer owns its pointer. */
        let topology = lupos_topology_numa_get_topology();
        let mut i: kernel::ffi::c_int = 0;
        while (*topology.add(i as usize)).mask.is_some() {
            i += 1;
        }
        let tl = lupos_topology_numa_topology_alloc(i + nr_levels + 1);
        if tl.is_null() {
            return;
        }

        /* Copy the default topology bits. */
        i = 0;
        while (*topology.add(i as usize)).mask.is_some() {
            core::ptr::copy_nonoverlapping(topology.add(i as usize), tl.add(i as usize), 1);
            i += 1;
        }

        /* Add the NUMA identity distance, aka single NODE. */
        lupos_topology_numa_init_node(tl.add(i as usize), Some(sd_numa_mask));
        i += 1;

        /* Append the remaining NUMA levels. */
        for j in 1..nr_levels {
            lupos_topology_numa_init_level(tl.add(i as usize), Some(sd_numa_mask), Some(cpu_numa_flags));
            (*tl.add(i as usize)).numa_level = j;
            i += 1;
        }
        lupos_topology_numa_set_saved_topology(topology);
        lupos_topology_numa_set_topology(tl);
        lupos_topology_numa_levels_set(nr_levels);
        init_numa_topology_type(offline_node);
    }
}

/// Detach NUMA state, wait for readers and restore the saved base topology.
///
/// # Safety
/// The caller must exclude other topology writers and hotplug changes and
/// may sleep in synchronize_rcu. The caller must not hold an RCU read lock
/// or retain a reference to storage reclaimed here. Private pointer slots
/// must contain only matching native allocations or NULL, with the only
/// permitted distance alias being the shared physical/domain array. The
/// saved topology must remain live independently of the current allocation.
#[cfg(CONFIG_NUMA)]
unsafe fn sched_reset_numa() {
    // SAFETY: Caller holds topology/hotplug exclusion. Published pointers
    // are cleared before the original RCU grace period and reclamation.
    unsafe {
        let nr_levels = lupos_topology_numa_levels();
        lupos_topology_numa_node_levels_set(0);
        lupos_topology_numa_levels_set(0);
        lupos_topology_numa_max_distance_set(0);
        lupos_topology_numa_type_direct();
        let distances = lupos_topology_numa_node_distances();
        let mut dom_distances: *mut kernel::ffi::c_int = core::ptr::null_mut();
        if lupos_topology_numa_node_distances() != lupos_topology_numa_domain_distances() {
            dom_distances = lupos_topology_numa_domain_distances();
        }
        lupos_topology_numa_node_distances_clear();
        lupos_topology_numa_domain_distances_clear();
        let masks = lupos_topology_numa_masks();
        lupos_topology_numa_masks_clear();
        if !distances.is_null() || !masks.is_null() {
            lupos_topology_numa_synchronize_rcu();
            lupos_topology_numa_free(distances.cast());
            lupos_topology_numa_free(dom_distances.cast());
            let mut i = 0;
            while i < nr_levels && !masks.is_null() {
                if !(*masks.add(i as usize)).is_null() {
                    let mut j = lupos_topology_numa_next_possible_node(-1);
                    while j < lupos_topology_numa_max_num_nodes() {
                        lupos_topology_numa_free((*(*masks.add(i as usize)).add(j as usize)).cast());
                        j = lupos_topology_numa_next_possible_node(j as kernel::ffi::c_int);
                    }
                    lupos_topology_numa_free((*masks.add(i as usize)).cast());
                }
                i += 1;
            }
            lupos_topology_numa_free(masks.cast());
        }
        let saved = lupos_topology_numa_get_saved_topology();
        if !saved.is_null() {
            lupos_topology_numa_free(lupos_topology_numa_get_topology().cast());
            lupos_topology_numa_set_topology(saved);
            lupos_topology_numa_set_saved_topology(core::ptr::null_mut());
        }
    }
}

/// Rebuild topology when the first/last CPU on a node changes online state.
///
/// # Safety
/// The hotplug lock must be held, cpu must be a valid native CPU ID, and
/// online must describe its current hotplug transition. Native node masks
/// must reflect the first/last-CPU transition point used by the original
/// caller; reset/reinitialization require a sleepable context and at least
/// one remaining CPU node. No reclaimed topology pointer may be retained.
#[cfg(CONFIG_NUMA)]
#[no_mangle]
pub unsafe extern "C" fn sched_update_numa(cpu: kernel::ffi::c_int, online: bool) {
    // SAFETY: Native node masks and topology mutation share the hotplug lock.
    unsafe {
        let node = lupos_topology_numa_cpu_to_node(cpu);
        if lupos_topology_numa_mask_weight(lupos_topology_numa_node_mask(node)) != 1 {
            return;
        }
        sched_reset_numa();
        sched_init_numa(if online { LUPOS_TOPOLOGY_NUMA_NO_NODE as kernel::ffi::c_int } else { node });
    }
}

/// Add a CPU to each applicable remote-node NUMA mask.
///
/// # Safety
/// The caller must retain native hotplug/topology writer exclusion. cpu
/// must fit the native mask allocation and map to an initialized node;
/// every N_CPU node must have a populated row at each published NUMA level.
#[cfg(CONFIG_NUMA)]
#[no_mangle]
pub unsafe extern "C" fn sched_domains_numa_masks_set(cpu: kernel::ffi::c_uint) {
    // SAFETY: Masks and distance arrays remain live under caller exclusion.
    unsafe {
        let node = lupos_topology_numa_cpu_to_node(cpu as kernel::ffi::c_int);
        let mut i = 0;
        while i < lupos_topology_numa_levels() {
            let mut j = 0;
            while (j as kernel::ffi::c_uint) < lupos_topology_numa_nr_node_ids() {
                if lupos_topology_numa_node_has_cpu(j)
                    && arch_sched_node_distance(j, node) <= lupos_topology_numa_domain_distance(i)
                {
                    lupos_topology_numa_mask_set_cpu(
                        cpu, *(*lupos_topology_numa_masks().add(i as usize)).add(j as usize),
                    );
                }
                j += 1;
            }
            i += 1;
        }
    }
}

/// Remove a CPU from all populated NUMA masks.
///
/// # Safety
/// The caller must retain native hotplug/topology writer exclusion. cpu
/// must fit each native mask's valid CPU range; every published level must
/// have a live nr_node_ids-sized row, whose unused entries may be NULL.
#[cfg(CONFIG_NUMA)]
#[no_mangle]
pub unsafe extern "C" fn sched_domains_numa_masks_clear(cpu: kernel::ffi::c_uint) {
    // SAFETY: Mask arrays remain live under caller exclusion.
    unsafe {
        let mut i = 0;
        while i < lupos_topology_numa_levels() {
            let mut j = 0;
            while (j as kernel::ffi::c_uint) < lupos_topology_numa_nr_node_ids() {
                let mask = *(*lupos_topology_numa_masks().add(i as usize)).add(j as usize);
                if !mask.is_null() {
                    lupos_topology_numa_mask_clear_cpu(cpu, mask);
                }
                j += 1;
            }
            i += 1;
        }
    }
}

/// Find the closest CPU in cpus, or return nr_cpu_ids when none is found.
///
/// # Safety
/// cpus must be a readable native cpumask of the configured valid bit width
/// for the entire call. cpu must be a valid native CPU ID mapping within
/// nr_node_ids. The context must permit the internal RCU read-side section;
/// the caller retains cpus independently of that section's acquired arrays.
#[cfg(CONFIG_NUMA)]
#[no_mangle]
pub unsafe extern "C" fn sched_numa_find_closest(
    cpus: *const cpumask,
    mut cpu: kernel::ffi::c_int,
) -> kernel::ffi::c_int {
    // SAFETY: The acquired NUMA arrays are used entirely within RCU.
    unsafe {
        let j = lupos_topology_numa_cpu_to_node(cpu);
        let mut found = lupos_topology_numa_nr_cpu_ids() as kernel::ffi::c_int;
        lupos_topology_numa_rcu_read_lock();
        'unlock: {
            let masks = lupos_topology_numa_masks_dereference();
            if masks.is_null() {
                break 'unlock;
            }
            let mut i = 0;
            while i < lupos_topology_numa_levels() {
                let mask = *(*masks.add(i as usize)).add(j as usize);
                if mask.is_null() {
                    break;
                }
                cpu = lupos_topology_numa_mask_any_and_distribute(cpus, mask) as kernel::ffi::c_int;
                if (cpu as kernel::ffi::c_uint) < lupos_topology_numa_nr_cpu_ids() {
                    found = cpu;
                    break;
                }
                i += 1;
            }
        }
        lupos_topology_numa_rcu_read_unlock();
        found
    }
}

/* Original __cmp_key: native bsearch passes this private Rust key opaquely. */
#[cfg(CONFIG_NUMA)]
struct LuposTopologyNumaCmpKey {
    cpus: *const cpumask,
    masks: *mut *mut *mut cpumask,
    node: kernel::ffi::c_int,
    cpu: kernel::ffi::c_int,
    w: kernel::ffi::c_int,
}

/// Compare one hop with a search key and record the preceding hop's weight.
///
/// # Safety
/// a must be the raw pointer to the live, writable private Rust key passed
/// by sched_numa_find_nth_cpu. b must be an aligned element of that key's
/// masks allocation, selected by native bsearch; all node masks and cpus
/// must remain readable under the enclosing RCU section. No shared Rust
/// reference to the key may overlap this callback's write to w. The key is
/// opaque to C and the callback must not outlive the synchronous search.
#[cfg(CONFIG_NUMA)]
unsafe extern "C" fn hop_cmp(
    a: *const kernel::ffi::c_void,
    b: *const kernel::ffi::c_void,
) -> kernel::ffi::c_int {
    // SAFETY: The synchronous native search receives a raw mutable key
    // pointer and returns aligned elements within its live masks array.
    // The equality branch excludes the first element before subtracting
    // one. The caller keeps the indexed node masks and cpus readable in RCU.
    unsafe {
        let cur_hop = *(b as *const *mut *mut cpumask);
        let k = a as *mut LuposTopologyNumaCmpKey;
        if lupos_topology_numa_mask_weight_and((*k).cpus, *cur_hop.add((*k).node as usize))
            <= (*k).cpu as kernel::ffi::c_uint
        {
            return 1;
        }
        if b == (*k).masks.cast::<kernel::ffi::c_void>() as *const kernel::ffi::c_void {
            (*k).w = 0;
            return 0;
        }
        let prev_hop = *(b as *const *mut *mut cpumask).sub(1);
        (*k).w = lupos_topology_numa_mask_weight_and(
            (*k).cpus, *prev_hop.add((*k).node as usize),
        ) as kernel::ffi::c_int;
        if (*k).w <= (*k).cpu {
            return 0;
        }
        -1
    }
}

/// Find the Nth closest CPU in cpus ordered by distance from node.
/// Returns nr_cpu_ids when none is found.
///
/// # Safety
/// cpus must remain a readable native mask of the configured valid width.
/// cpu is the nonnegative ordinal to select. node is NUMA_NO_NODE or a
/// valid initialized node ID; for a real node, a CPU-bearing nearest node
/// must exist. The caller's context must permit the internal RCU section;
/// the NUMA_NO_NODE path also borrows the native online CPU mask.
#[cfg(CONFIG_NUMA)]
#[no_mangle]
pub unsafe extern "C" fn sched_numa_find_nth_cpu(
    cpus: *const cpumask,
    cpu: kernel::ffi::c_int,
    mut node: kernel::ffi::c_int,
) -> kernel::ffi::c_int {
    // SAFETY: The masks and raw mutable key remain live throughout native
    // bsearch, which neither retains nor inspects the private key. A found
    // hop is an element of k.masks, so offset_from uses one allocation.
    // Nearest-node mapping selects populated rows; RCU spans all array and
    // mask reads, and every post-lock exit reaches the unlock below.
    unsafe {
        let mut k = LuposTopologyNumaCmpKey {
            cpus, masks: core::ptr::null_mut(), node: 0, cpu, w: 0,
        };
        let mut ret = lupos_topology_numa_nr_cpu_ids() as kernel::ffi::c_int;
        if node == LUPOS_TOPOLOGY_NUMA_NO_NODE as kernel::ffi::c_int {
            return lupos_topology_numa_mask_nth_and(
                cpu as kernel::ffi::c_uint, cpus, lupos_topology_numa_online_mask(),
            ) as kernel::ffi::c_int;
        }
        lupos_topology_numa_rcu_read_lock();
        'unlock: {
            /* CPU-less node entries are uninitialized in the mask array. */
            node = lupos_topology_numa_nearest_cpu_node(node);
            k.node = node;
            k.masks = lupos_topology_numa_masks_dereference();
            if k.masks.is_null() {
                break 'unlock;
            }
            let hop_masks = lupos_topology_numa_bsearch(
                (&raw mut k).cast(), k.masks, lupos_topology_numa_levels(), Some(hop_cmp),
            );
            if hop_masks.is_null() {
                break 'unlock;
            }
            let hop = hop_masks.offset_from(k.masks) as kernel::ffi::c_int;
            ret = if hop != 0 {
                lupos_topology_numa_mask_nth_and_andnot(
                    (cpu - k.w) as kernel::ffi::c_uint, cpus,
                    *(*k.masks.add(hop as usize)).add(node as usize),
                    *(*k.masks.add((hop - 1) as usize)).add(node as usize),
                )
            } else {
                lupos_topology_numa_mask_nth_and(
                    cpu as kernel::ffi::c_uint, cpus, *(*k.masks).add(node as usize),
                )
            } as kernel::ffi::c_int;
        }
        lupos_topology_numa_rcu_read_unlock();
        ret
    }
}

/// Get CPUs at most hops hops from node, or the original encoded error.
///
/// # Safety
/// The caller holds RCU read lock; the returned mask remains valid only in
/// that read-side section and must be copied with native cpumask operations
/// if needed beyond it. The result must be checked for native error pointers
/// and NULL before dereferencing; a CPU-less node can have a NULL entry.
/// The borrowed mask must not be freed or accessed as a full Rust cpumask
/// value because its allocation can be smaller than the native type layout.
#[cfg(CONFIG_NUMA)]
#[no_mangle]
pub unsafe extern "C" fn sched_numa_hop_mask(
    node: kernel::ffi::c_uint,
    hops: kernel::ffi::c_uint,
) -> *const cpumask {
    // SAFETY: Bounds are checked before indexing the RCU-acquired arrays.
    unsafe {
        if node >= lupos_topology_numa_nr_node_ids()
            || hops >= lupos_topology_numa_levels() as kernel::ffi::c_uint
        {
            return lupos_topology_numa_error_mask(-(LUPOS_TOPOLOGY_NUMA_EINVAL as kernel::ffi::c_int));
        }
        let masks = lupos_topology_numa_masks_dereference();
        if masks.is_null() {
            return lupos_topology_numa_error_mask(-(LUPOS_TOPOLOGY_NUMA_EBUSY as kernel::ffi::c_int));
        }
        *(*masks.add(hops as usize)).add(node as usize)
    }
}
