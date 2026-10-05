// SPDX-License-Identifier: GPL-2.0
/*
 * Continuation of the existing cid.rs at native baseline
 * 126a30fae3bba11420ec2fcbde51a0a01bab1b5b. The original entry points,
 * topology helpers and cmask intersection walker remain the implementation
 * spine. Native headers own all layouts and configured primitives.
 *
 * Native leaves are an explicit, unqualified C runtime boundary. This source
 * is not admitted for compilation or execution, and includes no C algorithm
 * fallback. Shared ext ownership and protection qualification remain pending.
 */
#![no_std]

compile_error!("SOURCE ONLY HOLD: sched_ext CID is not admitted");

use core::{marker::PhantomData, mem::size_of, ptr};
use kernel::bindings::sched_ext_cid_native::*;
use kernel::ffi::{c_int, c_uint, c_void};

const ENOMEM: c_int = LUPOS_SCX_CID_ENOMEM as c_int;
const EINVAL: c_int = LUPOS_SCX_CID_EINVAL as c_int;
const NUMA_NO_NODE: c_int = LUPOS_SCX_CID_NUMA_NO_NODE as c_int;
const SHARD_SIZE_DFL: c_uint = LUPOS_SCX_CID_SHARD_SIZE_DFL as c_uint;
const SHARD_MAX_CPUS: c_uint = LUPOS_SCX_CID_SHARD_MAX_CPUS as c_uint;

// Protected by scx_enable_mutex. Never exposed to lockless readers directly.
static mut CID_TABLES: *mut scx_cid_tables = ptr::null_mut();

const SCX_CID_TOPO_NEG: scx_cid_topo = scx_cid_topo {
    core_cid: -1,
    core_idx: -1,
    llc_cid: -1,
    llc_idx: -1,
    node_cid: -1,
    node_idx: -1,
    shard_cid: -1,
    shard_idx: -1,
};

// Stack-bound, non-Send guard. No sleeping allocation occurs while it lives.
struct RcuGuard(PhantomData<*mut ()>);

impl RcuGuard {
    unsafe fn lock() -> Self {
        // SAFETY: The caller's kfunc context permits the native RCU read lock.
        unsafe { lupos_scx_cid_rcu_lock() };
        Self(PhantomData)
    }
}

impl Drop for RcuGuard {
    fn drop(&mut self) {
        // SAFETY: Constructed only after taking one native read lock; not moved
        // to another thread, leaked, or retained beyond the synchronous call.
        unsafe { lupos_scx_cid_rcu_unlock() };
    }
}

// Owns only temporary kmalloc allocations, never a table's kvcalloc arrays.
struct Kfree(*mut c_void);

impl Drop for Kfree {
    fn drop(&mut self) {
        // SAFETY: Each pointer is NULL or one uniquely owned native allocation.
        unsafe { lupos_scx_cid_kfree(self.0) };
    }
}

unsafe fn cid_valid(sch: *mut scx_sched, cid: c_int) -> bool {
    // SAFETY: sch remains RCU-protected or enable-mutex protected by caller.
    unsafe {
        if lupos_scx_cid_likely_valid(cid >= 0 && (cid as c_uint) < lupos_scx_cid_nr_possible()) {
            return true;
        }
        lupos_scx_cid_error_invalid(sch, cid);
        false
    }
}

unsafe fn cpu_llc_mask(cpu: c_int, fallbacks: *mut cpumask) -> *const cpumask {
    // SAFETY: Hotplug read protection pins topology and both masks for this call.
    unsafe {
        let mask = lupos_scx_cid_cache_mask(cpu);
        if !mask.is_null() {
            return mask;
        }
        lupos_scx_cid_mask_set(cpu, fallbacks);
        // The topology walk excludes negative NUMA nodes before reaching here.
        lupos_scx_cid_node_mask(lupos_scx_cid_cpu_node(cpu))
    }
}

unsafe fn calc_shard_layout(llc: *const cpumask, shard_size: c_uint) -> (c_uint, c_uint) {
    // SAFETY: llc is a stable, borrowed native mask. Do not clear it while
    // counting: the builder still needs every CPU for its following walk.
    unsafe {
        let mut nr_cpus = 0u32;
        let mut nr_cores = 0;
        let limit = lupos_scx_cid_mask_scan_limit();
        let mut cpu = lupos_scx_cid_mask_scan(0, llc);
        while cpu < limit {
            nr_cpus += 1;
            if lupos_scx_cid_mask_first(lupos_scx_cid_sibling_mask(cpu as c_int)) == cpu {
                nr_cores += 1;
            }
            cpu = lupos_scx_cid_mask_scan(cpu + 1, llc);
        }
        // Preserve DIV_ROUND_UP's native unsigned (n + d - 1) / d expression.
        let requested = nr_cpus.wrapping_add(shard_size).wrapping_sub(1) / shard_size;
        let capped = nr_cpus.wrapping_add(SHARD_MAX_CPUS).wrapping_sub(1) / SHARD_MAX_CPUS;
        let nr_shards = requested.max(1).max(capped);
        (nr_cores / nr_shards, nr_cores % nr_shards)
    }
}

unsafe fn scx_cid_tables_free(t: *mut scx_cid_tables) {
    // SAFETY: t is NULL, unpublished construction storage, or retired storage
    // returned by RCU after every pre-existing reader is gone.
    unsafe {
        if t.is_null() {
            return;
        }
        lupos_scx_cid_kvfree((*t).cid_to_cpu.cast());
        lupos_scx_cid_kvfree((*t).cpu_to_cid.cast());
        lupos_scx_cid_kvfree((*t).cid_to_shard.cast());
        lupos_scx_cid_kvfree((*t).shard_node.cast());
        lupos_scx_cid_kvfree((*t).shard_ranges.cast());
        lupos_scx_cid_kvfree((*t).topo.cast());
        lupos_scx_cid_kfree(t.cast());
    }
}

/// Reclaim one table owner after the native RCU grace period.
///
/// # Safety
/// Only the native callback may pass the embedded callback_head of a retired
/// owner. The C spelling rcu_head is a macro alias, not a generated Rust type.
#[export_name = "lupos_scx_cid_free_rcu"]
pub unsafe extern "C" fn scx_cid_tables_free_rcufn(rcu: *mut callback_head) {
    // SAFETY: Native container_of uses the actual header's offset and type.
    unsafe { scx_cid_tables_free(lupos_scx_cid_from_rcu(rcu)) };
}

unsafe fn scx_cid_alloc_tables() -> *mut scx_cid_tables {
    // SAFETY: Native zeroed allocations use configured GFP_KERNEL primitives;
    // all sizes come from generated native types, with no hand-written layout.
    unsafe {
        let n = lupos_scx_cid_nr_possible() as usize;
        let cpu_slots = lupos_scx_cid_nr_cpu_ids() as usize;
        let t = lupos_scx_cid_kzalloc(size_of::<scx_cid_tables>()).cast::<scx_cid_tables>();
        if t.is_null() {
            return t;
        }
        (*t).cid_to_cpu = lupos_scx_cid_kvcalloc(n, size_of::<i16>()).cast();
        (*t).cpu_to_cid = lupos_scx_cid_kvcalloc(cpu_slots, size_of::<i16>()).cast();
        (*t).cid_to_shard = lupos_scx_cid_kvcalloc(n, size_of::<i32>()).cast();
        (*t).shard_node = lupos_scx_cid_kvcalloc(n, size_of::<i32>()).cast();
        (*t).shard_ranges = lupos_scx_cid_kvcalloc(n, size_of::<scx_cid_shard>()).cast();
        (*t).topo = lupos_scx_cid_kvcalloc(n, size_of::<scx_cid_topo>()).cast();
        if (*t).cid_to_cpu.is_null() || (*t).cpu_to_cid.is_null()
            || (*t).cid_to_shard.is_null() || (*t).shard_node.is_null()
            || (*t).shard_ranges.is_null() || (*t).topo.is_null()
        {
            scx_cid_tables_free(t);
            return ptr::null_mut();
        }
        t
    }
}

/// Publish the completed tables after the root's init_cids operation.
///
/// # Safety
/// Caller holds scx_enable_mutex and has a successfully initialized table set.
#[no_mangle]
pub unsafe extern "C" fn scx_cid_publish_tables() {
    // SAFETY: The native leaf retains rcu_assign_pointer for all six slots.
    unsafe {
        lupos_scx_cid_assert_enable_held();
        lupos_scx_cid_publish(CID_TABLES);
    }
}

/// Unpublish and enqueue the table owner for RCU reclamation.
///
/// # Safety
/// Caller holds scx_enable_mutex and cpus_read_lock, after unchecked readers
/// have drained. No table pointer may be used after handing its head to RCU.
#[no_mangle]
pub unsafe extern "C" fn scx_cid_retire_tables() {
    // SAFETY: Mutation is serialized, unpublication keeps RCU_INIT_POINTER,
    // and the callback receives this exact owner's embedded, non-null head.
    unsafe {
        lupos_scx_cid_assert_enable_held();
        lupos_scx_cid_assert_cpus_held();
        let t = CID_TABLES;
        if t.is_null() {
            return;
        }
        CID_TABLES = ptr::null_mut();
        lupos_scx_cid_unpublish();
        lupos_scx_cid_call_rcu(ptr::addr_of_mut!((*t).rcu));
    }
}

/// Build a private default mapping for a root scheduler.
///
/// # Safety
/// Caller holds cpus_read_lock and scx_enable_mutex. Root disable must retire
/// the stored table even when scratch allocation or topology checks fail.
#[no_mangle]
pub unsafe extern "C" fn scx_cid_init(sch: *mut scx_sched) -> c_int {
    // SAFETY: sch is live under the original locks; allocated table storage is
    // unpublished. Native automatic cpumasks survive only the build callback.
    unsafe {
        lupos_scx_cid_assert_cpus_held();
        lupos_scx_cid_assert_enable_held();
        let requested = lupos_scx_cid_shard_size(sch);
        let shard_size = if requested == 0 { SHARD_SIZE_DFL } else { requested };
        let t = scx_cid_alloc_tables();
        if t.is_null() {
            return -ENOMEM;
        }
        CID_TABLES = t;
        for si in 0..lupos_scx_cid_nr_possible() as usize {
            (*t).shard_node.add(si).write(NUMA_NO_NODE);
        }
        lupos_scx_cid_with_build_masks(t, shard_size)
    }
}

/// Continue initialization with six configured, zeroed native scratch masks.
///
/// # Safety
/// Called synchronously by the native scratch-storage adapter under the locks
/// of scx_cid_init. All pointers are distinct live storage; none is retained.
#[export_name = "lupos_scx_cid_build"]
pub unsafe extern "C" fn scx_cid_build(
    t: *mut scx_cid_tables,
    shard_size: c_uint,
    to_walk: *mut cpumask,
    node_scratch: *mut cpumask,
    llc_scratch: *mut cpumask,
    core_scratch: *mut cpumask,
    llc_fallback: *mut cpumask,
    online_no_topo: *mut cpumask,
) -> c_int {
    // SAFETY: The possible/online sets are stable under cpus_read_lock. Parent
    // intersections guarantee every assigned CPU occurs once, so table indices
    // stay within num_possible_cpus and raw CPU slots within nr_cpu_ids.
    unsafe {
        let nr_cpu_ids = lupos_scx_cid_nr_cpu_ids();
        let scan_limit = lupos_scx_cid_mask_scan_limit();
        let max_cids = shard_size.min(SHARD_MAX_CPUS);
        let mut next_cid = 0u32;
        let mut next_node_idx = 0i32;
        let mut next_llc_idx = 0i32;
        let mut next_core_idx = 0i32;
        let mut next_shard_idx = 0i32;

        for cpu in 0..nr_cpu_ids as usize {
            (*t).cpu_to_cid.add(cpu).write(-1);
        }
        lupos_scx_cid_mask_copy(to_walk, lupos_scx_cid_online_mask());
        while !lupos_scx_cid_mask_empty(to_walk) {
            let next_cpu = lupos_scx_cid_mask_first(to_walk) as c_int;
            let nid = lupos_scx_cid_cpu_node(next_cpu);
            let node_cid = next_cid as c_int;
            if nid < 0 {
                lupos_scx_cid_mask_clear(next_cpu, to_walk);
                continue;
            }
            let node_idx = next_node_idx;
            next_node_idx += 1;
            lupos_scx_cid_mask_and(node_scratch, to_walk, lupos_scx_cid_node_mask(nid));
            if lupos_scx_cid_warn_node(!lupos_scx_cid_mask_test(next_cpu, node_scratch)) {
                return -EINVAL;
            }

            while !lupos_scx_cid_mask_empty(node_scratch) {
                let ncpu = lupos_scx_cid_mask_first(node_scratch) as c_int;
                let llc_mask = cpu_llc_mask(ncpu, llc_fallback);
                let llc_cid = next_cid as c_int;
                let llc_idx = next_llc_idx;
                next_llc_idx += 1;
                lupos_scx_cid_mask_and(llc_scratch, node_scratch, llc_mask);
                if lupos_scx_cid_warn_llc(!lupos_scx_cid_mask_test(ncpu, llc_scratch)) {
                    return -EINVAL;
                }
                let (cores_per_shard, nr_large) = calc_shard_layout(llc_scratch, shard_size);
                let mut shard_local = 0;
                let mut cores_in_shard = 0;
                let mut cids_in_shard = 0;
                let mut shard_cid = next_cid as c_int;
                let mut shard_idx = next_shard_idx;
                next_shard_idx += 1;
                (*t).shard_node.add(shard_idx as usize).write(nid);

                while !lupos_scx_cid_mask_empty(llc_scratch) {
                    let lcpu = lupos_scx_cid_mask_first(llc_scratch) as c_int;
                    let siblings = lupos_scx_cid_sibling_mask(lcpu);
                    let core_cid = next_cid as c_int;
                    let core_idx = next_core_idx;
                    next_core_idx += 1;
                    lupos_scx_cid_mask_and(core_scratch, llc_scratch, siblings);
                    if lupos_scx_cid_warn_core(!lupos_scx_cid_mask_test(lcpu, core_scratch)) {
                        return -EINVAL;
                    }
                    let cids_in_core = lupos_scx_cid_mask_weight(core_scratch);
                    let max_cores = cores_per_shard + c_uint::from(shard_local < nr_large);
                    if cores_in_shard != 0
                        && (cores_in_shard >= max_cores || cids_in_shard + cids_in_core > max_cids)
                    {
                        shard_local += 1;
                        cores_in_shard = 0;
                        cids_in_shard = 0;
                        shard_cid = next_cid as c_int;
                        shard_idx = next_shard_idx;
                        next_shard_idx += 1;
                        (*t).shard_node.add(shard_idx as usize).write(nid);
                    }
                    cores_in_shard += 1;
                    cids_in_shard += cids_in_core;
                    let mut ccpu = lupos_scx_cid_mask_scan(0, core_scratch);
                    while ccpu < scan_limit {
                        let cid = next_cid as usize;
                        next_cid += 1;
                        (*t).cid_to_cpu.add(cid).write(ccpu as i16);
                        (*t).cpu_to_cid.add(ccpu as usize).write(cid as i16);
                        (*t).cid_to_shard.add(cid).write(shard_idx);
                        (*t).topo.add(cid).write(scx_cid_topo {
                            core_cid, core_idx, llc_cid, llc_idx,
                            node_cid, node_idx, shard_cid, shard_idx,
                        });
                        lupos_scx_cid_mask_clear(ccpu as c_int, llc_scratch);
                        lupos_scx_cid_mask_clear(ccpu as c_int, node_scratch);
                        lupos_scx_cid_mask_clear(ccpu as c_int, to_walk);
                        ccpu = lupos_scx_cid_mask_scan(ccpu + 1, core_scratch);
                    }
                }
            }
        }

        let possible = lupos_scx_cid_possible_mask();
        let mut notopo_in_shard = max_cids;
        let mut notopo_shard_cid = -1;
        let mut notopo_shard_idx = -1;
        let mut cpu = lupos_scx_cid_mask_scan(0, possible);
        while cpu < scan_limit {
            if *(*t).cpu_to_cid.add(cpu as usize) == -1 {
                if lupos_scx_cid_cpu_online(cpu as c_int) {
                    lupos_scx_cid_mask_set(cpu as c_int, online_no_topo);
                }
                let cid = next_cid as usize;
                next_cid += 1;
                (*t).cid_to_cpu.add(cid).write(cpu as i16);
                (*t).cpu_to_cid.add(cpu as usize).write(cid as i16);
                if notopo_in_shard >= max_cids {
                    notopo_shard_cid = cid as c_int;
                    notopo_shard_idx = next_shard_idx;
                    next_shard_idx += 1;
                    notopo_in_shard = 0;
                }
                notopo_in_shard += 1;
                (*t).cid_to_shard.add(cid).write(notopo_shard_idx);
                (*t).topo.add(cid).write(SCX_CID_TOPO_NEG);
                (*(*t).topo.add(cid)).shard_cid = notopo_shard_cid;
                (*(*t).topo.add(cid)).shard_idx = notopo_shard_idx;
            }
            cpu = lupos_scx_cid_mask_scan(cpu + 1, possible);
        }
        if !lupos_scx_cid_mask_empty(llc_fallback) {
            lupos_scx_cid_warn_cache(llc_fallback);
        }
        if !lupos_scx_cid_mask_empty(online_no_topo) {
            lupos_scx_cid_warn_notopo(online_no_topo);
        }
        for cid in 0..next_cid as usize {
            let si = *(*t).cid_to_shard.add(cid) as usize;
            let range = (*t).shard_ranges.add(si);
            if (*range).nr_cids == 0 {
                (*range).base_cid = cid as c_int;
            }
            (*range).nr_cids += 1;
        }
        (*t).nr_shards = next_shard_idx as c_uint;
        0
    }
}

unsafe fn pick_max_node(counts: *const c_uint, n: c_uint) -> c_int {
    // SAFETY: counts contains n initialized entries. Strict comparison preserves
    // the native tie break in favor of the lowest node number.
    unsafe {
        let mut best = NUMA_NO_NODE;
        let mut best_count = 0;
        for node in 0..n as usize {
            let count = *counts.add(node);
            if count > best_count {
                best_count = count;
                best = node as c_int;
            }
        }
        best
    }
}

/// Replace the unpublished mapping from root ops.init_cids.
///
/// # Safety
/// Invoked only through the native BTF adapter and original context filter.
/// Arena addresses are kernel-rebased and covered by the arena fault mechanism.
#[export_name = "lupos_scx_bpf_cid_override"]
pub unsafe extern "C" fn scx_bpf_cid_override(
    cpu_to_cid: *const c_int,
    cpu_cnt: c_uint,
    shard_start: *const c_int,
    shard_cnt: c_uint,
    aux: *const bpf_prog_aux,
) {
    // SAFETY: The adapter's automatic seen mask survives the synchronous
    // continuation, including every early error return.
    unsafe { lupos_scx_cid_with_seen(cpu_to_cid, cpu_cnt, shard_start, shard_cnt, aux) };
}

/// Continue the override with the configured native seen mask.
///
/// # Safety
/// Called only by the synchronous native adapter with its allocation status.
/// The original BPF/arena and root-init context restrictions still apply.
#[export_name = "lupos_scx_cid_override_seen"]
pub unsafe extern "C" fn scx_cid_override_seen(
    cpu_to_cid_arena: *const c_int,
    cpu_cnt: c_uint,
    shard_start_arena: *const c_int,
    shard_cnt: c_uint,
    aux: *const bpf_prog_aux,
    seen: *mut cpumask,
    allocated: bool,
) {
    // SAFETY: All sleeping allocations and bounded arena snapshots precede
    // the RCU guard. Validation finishes before any table mutation. Kfree
    // owners are declared first so RCU unlock precedes their destructors.
    unsafe {
        let npossible = lupos_scx_cid_nr_possible();
        let nr_cpu_ids = lupos_scx_cid_nr_cpu_ids();
        let nr_node_ids = lupos_scx_cid_nr_node_ids();
        let node_owner = Kfree(lupos_scx_cid_kcalloc(nr_node_ids as usize, size_of::<c_uint>()));
        let cpu_owner = Kfree(if cpu_cnt == nr_cpu_ids {
            lupos_scx_cid_kmemdup(cpu_to_cid_arena.cast(), cpu_cnt as usize * size_of::<c_int>())
        } else {
            ptr::null_mut()
        });
        let shard_owner = Kfree(if shard_cnt != 0 && shard_cnt <= npossible {
            lupos_scx_cid_kmemdup(shard_start_arena.cast(), shard_cnt as usize * size_of::<c_int>())
        } else {
            ptr::null_mut()
        });
        let node_counts = node_owner.0.cast::<c_uint>();
        let cpu_to_cid = cpu_owner.0.cast::<c_int>();
        let shard_start = shard_owner.0.cast::<c_int>();
        let _rcu = RcuGuard::lock();
        let sch = lupos_scx_cid_prog_sched(aux);
        if lupos_scx_cid_unlikely_override_no_sched(sch.is_null()) {
            return;
        }
        lupos_scx_cid_assert_enable_held();
        let t = CID_TABLES;
        if cpu_cnt != nr_cpu_ids {
            lupos_scx_cid_error_cpu_count(sch, nr_cpu_ids, cpu_cnt);
            return;
        }
        if shard_cnt == 0 || shard_cnt > npossible {
            lupos_scx_cid_error_shard_count(sch, shard_cnt);
            return;
        }
        if !allocated || node_counts.is_null() || cpu_to_cid.is_null() || shard_start.is_null() {
            lupos_scx_cid_error_alloc(sch);
            return;
        }
        if *shard_start != 0 {
            lupos_scx_cid_error_start(sch, *shard_start);
            return;
        }
        for si in 1..shard_cnt as usize {
            let start = *shard_start.add(si);
            let prev = *shard_start.add(si - 1);
            if start <= prev {
                lupos_scx_cid_error_order(sch, si as c_int);
                return;
            }
            if start as c_uint >= npossible {
                lupos_scx_cid_error_bound(sch, si as c_int, start, npossible);
                return;
            }
            if (start - prev) as c_uint > SHARD_MAX_CPUS {
                lupos_scx_cid_error_span(sch, si as c_int - 1, start - prev);
                return;
            }
        }
        let last = *shard_start.add(shard_cnt as usize - 1) as c_uint;
        if npossible - last > SHARD_MAX_CPUS {
            lupos_scx_cid_error_span(sch, shard_cnt as c_int - 1, (npossible - last) as c_int);
            return;
        }

        let possible_limit = lupos_scx_cid_possible_cpu_limit();
        let mut cpu = lupos_scx_cid_possible_cpu_scan(0);
        while cpu < possible_limit {
            let cid = *cpu_to_cid.add(cpu as usize);
            if !cid_valid(sch, cid) {
                return;
            }
            if lupos_scx_cid_mask_test_set(cid, seen) {
                lupos_scx_cid_error_duplicate(sch, cid);
                return;
            }
            cpu = lupos_scx_cid_possible_cpu_scan(cpu + 1);
        }
        cpu = lupos_scx_cid_possible_cpu_scan(0);
        while cpu < possible_limit {
            let cid = *cpu_to_cid.add(cpu as usize);
            (*t).cpu_to_cid.add(cpu as usize).write(cid as i16);
            (*t).cid_to_cpu.add(cid as usize).write(cpu as i16);
            cpu = lupos_scx_cid_possible_cpu_scan(cpu + 1);
        }

        for si in 0..shard_cnt as usize {
            let end = if si + 1 < shard_cnt as usize {
                *shard_start.add(si + 1) as c_uint
            } else {
                npossible
            };
            ptr::write_bytes(node_counts, 0, nr_node_ids as usize);
            for cid in *shard_start.add(si) as c_uint..end {
                let raw_cpu = *(*t).cid_to_cpu.add(cid as usize) as c_int;
                let node = lupos_scx_cid_cpu_node(raw_cpu);
                if lupos_scx_cid_node_valid(node) {
                    *node_counts.add(node as usize) += 1;
                }
            }
            (*t).shard_node.add(si).write(pick_max_node(node_counts, nr_node_ids));
        }
        let mut si = 0usize;
        for cid in 0..npossible as usize {
            if si + 1 < shard_cnt as usize && cid as c_int >= *shard_start.add(si + 1) {
                si += 1;
            }
            (*t).cid_to_shard.add(cid).write(si as c_int);
            (*t).topo.add(cid).write(SCX_CID_TOPO_NEG);
            (*(*t).topo.add(cid)).shard_cid = *shard_start.add(si);
            (*(*t).topo.add(cid)).shard_idx = si as c_int;
        }
        ptr::write_bytes((*t).shard_ranges, 0, npossible as usize);
        for si in 0..shard_cnt as usize {
            let start = *shard_start.add(si);
            let end = if si + 1 < shard_cnt as usize {
                *shard_start.add(si + 1)
            } else {
                npossible as c_int
            };
            (*t).shard_ranges.add(si).write(scx_cid_shard {
                base_cid: start,
                nr_cids: end - start,
            });
        }
        (*t).nr_shards = shard_cnt;
    }
}

/// Read a CID-to-CPU mapping through the native BTF adapter.
///
/// # Safety
/// aux is the implicit argument of the currently executing BPF program.
#[export_name = "lupos_scx_bpf_cid_to_cpu"]
pub unsafe extern "C" fn scx_bpf_cid_to_cpu(cid: c_int, aux: *const bpf_prog_aux) -> c_int {
    // SAFETY: The guard covers both scheduler resolution and table dereference.
    unsafe {
        let _rcu = RcuGuard::lock();
        let sch = lupos_scx_cid_prog_sched(aux);
        if lupos_scx_cid_unlikely_to_cpu_no_sched(sch.is_null()) {
            return -EINVAL;
        }
        let table = lupos_scx_cid_to_cpu_dereference();
        if !cid_valid(sch, cid) || lupos_scx_cid_unlikely_to_cpu_no_table(table.is_null()) {
            return -EINVAL;
        }
        *table.add(cid as usize) as c_int
    }
}

/// Read a CPU-to-CID mapping through the native BTF adapter.
///
/// # Safety
/// aux is the implicit argument of the currently executing BPF program.
#[export_name = "lupos_scx_bpf_cpu_to_cid"]
pub unsafe extern "C" fn scx_bpf_cpu_to_cid(cpu: c_int, aux: *const bpf_prog_aux) -> c_int {
    // SAFETY: Original CPU validation includes sparse-possible holes. The
    // guard keeps the scheduler and table live through the indexed read.
    unsafe {
        let _rcu = RcuGuard::lock();
        let sch = lupos_scx_cid_prog_sched(aux);
        if lupos_scx_cid_unlikely_to_cid_no_sched(sch.is_null()) {
            return -EINVAL;
        }
        let table = lupos_scx_cpu_to_cid_dereference();
        if !lupos_scx_cid_cpu_valid(sch, cpu) || lupos_scx_cid_unlikely_to_cid_no_table(table.is_null()) {
            return -EINVAL;
        }
        *table.add(cpu as usize) as c_int
    }
}

/// Copy topology through the native BTF adapter.
///
/// # Safety
/// out is writable native scx_cid_topo storage and aux is the implicit BPF arg.
#[export_name = "lupos_scx_bpf_cid_topo"]
pub unsafe extern "C" fn scx_bpf_cid_topo(
    cid: c_int,
    out: *mut scx_cid_topo,
    aux: *const bpf_prog_aux,
) {
    // SAFETY: out receives initialized fields on every path, and the RCU
    // guard covers the source copy. No Rust reference aliases shared storage.
    unsafe {
        let _rcu = RcuGuard::lock();
        let sch = lupos_scx_cid_prog_sched(aux);
        let topo = lupos_scx_cid_topo_dereference();
        if lupos_scx_cid_unlikely_topo_no_sched(sch.is_null()) || !cid_valid(sch, cid)
            || lupos_scx_cid_unlikely_topo_no_table(topo.is_null())
        {
            out.write(SCX_CID_TOPO_NEG);
            return;
        }
        ptr::copy(topo.add(cid as usize), out, 1);
    }
}

/// Zero the active cmask words; leave additional capacity untouched.
///
/// # Safety
/// m is valid writable storage with stable geometry and enough allocated words.
#[no_mangle]
pub unsafe extern "C" fn scx_cmask_clear(m: *mut scx_cmask) {
    // SAFETY: The native flexible-array accessor avoids inventing its offset.
    unsafe {
        if (*m).nr_cids == 0 {
            return;
        }
        let n = ((*m).base + (*m).nr_cids - 1) / 64 - (*m).base / 64 + 1;
        ptr::write_bytes(lupos_scx_cid_cmask_bits(m), 0, n as usize);
    }
}

/// Set active CID bits and clear the head and tail padding.
///
/// # Safety
/// m is valid writable storage with stable geometry and enough allocated words.
#[no_mangle]
pub unsafe extern "C" fn scx_cmask_fill(m: *mut scx_cmask) {
    // SAFETY: Only the active words and their padding are touched.
    unsafe {
        if (*m).nr_cids == 0 {
            return;
        }
        let n = ((*m).base + (*m).nr_cids - 1) / 64 - (*m).base / 64 + 1;
        let bits = lupos_scx_cid_cmask_bits(m);
        ptr::write_bytes(bits, 0xff, n as usize);
        let head = (*m).base & 63;
        if head != 0 {
            *bits &= !((1u64 << head) - 1);
        }
        let tail = ((*m).base + (*m).nr_cids) & 63;
        if tail != 0 {
            *bits.add(n as usize - 1) &= (1u64 << tail) - 1;
        }
    }
}

#[derive(Copy, Clone)]
enum CmaskOp2 {
    And,
    Or,
    Copy,
    Andnot,
    Subset,
    Intersects,
    RefOr,
    RefCopy,
}

impl CmaskOp2 {
    #[inline(always)]
    fn is_pred(self) -> bool {
        matches!(self, Self::Subset | Self::Intersects)
    }
}

#[inline(always)]
unsafe fn cmask_word_op2(a: *mut u64, b: *const u64, mask: u64, op: CmaskOp2) -> bool {
    // SAFETY: Caller supplies valid word locations and a range mask. Native
    // READ_ONCE/WRITE_ONCE retain the C concurrency semantics, including the
    // distinction between kernel dst reads and concurrently written arena dst.
    unsafe {
        match op {
            CmaskOp2::And => lupos_scx_cid_write_u64(a, *a & (!mask | lupos_scx_cid_read_u64(b))),
            CmaskOp2::Or => lupos_scx_cid_write_u64(a, *a | (lupos_scx_cid_read_u64(b) & mask)),
            CmaskOp2::Copy => lupos_scx_cid_write_u64(a, (*a & !mask) | (lupos_scx_cid_read_u64(b) & mask)),
            CmaskOp2::Andnot => lupos_scx_cid_write_u64(a, *a & !(lupos_scx_cid_read_u64(b) & mask)),
            CmaskOp2::Subset => return (lupos_scx_cid_read_u64(b) & !lupos_scx_cid_read_u64(a)) & mask != 0,
            CmaskOp2::Intersects => return (lupos_scx_cid_read_u64(a) & lupos_scx_cid_read_u64(b)) & mask != 0,
            CmaskOp2::RefOr => lupos_scx_cid_write_u64(a, lupos_scx_cid_read_u64(a) | (lupos_scx_cid_read_u64(b) & mask)),
            CmaskOp2::RefCopy => lupos_scx_cid_write_u64(a, (lupos_scx_cid_read_u64(a) & !mask) | (lupos_scx_cid_read_u64(b) & mask)),
        }
        false
    }
}

#[inline(always)]
unsafe fn cmask_walk_op2(
    a: *mut u64, ab: c_uint, an: c_uint,
    b: *const u64, bb: c_uint, bn: c_uint,
    op: CmaskOp2,
) -> bool {
    // SAFETY: Stable, validated geometry bounds both flexible arrays. This is
    // the existing shared intersection walk with Copy operations and native
    // word primitives. Bits outside the intersection are never changed.
    unsafe {
        let lo = ab.max(bb);
        let hi = (ab + an).min(bb + bn);
        if lo >= hi {
            return false;
        }
        let aw = ab / 64;
        let bw = bb / 64;
        let lw = lo / 64;
        let hw = (hi - 1) / 64;
        let head = !0u64 << (lo & 63);
        let tail = !0u64 >> (63 - ((hi - 1) & 63));
        if lw == hw {
            return cmask_word_op2(a.add((lw - aw) as usize), b.add((lw - bw) as usize), head & tail, op);
        }
        if cmask_word_op2(a.add((lw - aw) as usize), b.add((lw - bw) as usize), head, op) && op.is_pred() {
            return true;
        }
        for w in lw + 1..hw {
            if cmask_word_op2(a.add((w - aw) as usize), b.add((w - bw) as usize), !0, op) && op.is_pred() {
                return true;
            }
        }
        cmask_word_op2(a.add((hw - aw) as usize), b.add((hw - bw) as usize), tail, op)
    }
}

#[inline(always)]
unsafe fn cmask_walk_op1(bits: *const u64, base: c_uint, nr_cids: c_uint) -> bool {
    // SAFETY: bits starts at the word covering base and spans the active range.
    // This implements the sole native op1 (ANY_SET) without an invented ABI enum.
    unsafe {
        if nr_cids == 0 {
            return false;
        }
        let hi = base + nr_cids;
        let lw = base / 64;
        let hw = (hi - 1) / 64;
        let head = !0u64 << (base & 63);
        let tail = !0u64 >> (63 - ((hi - 1) & 63));
        if lw == hw {
            return lupos_scx_cid_read_u64(bits) & head & tail != 0;
        }
        if lupos_scx_cid_read_u64(bits) & head != 0 {
            return true;
        }
        for w in lw + 1..hw {
            if lupos_scx_cid_read_u64(bits.add((w - lw) as usize)) != 0 {
                return true;
            }
        }
        lupos_scx_cid_read_u64(bits.add((hw - lw) as usize)) & tail != 0
    }
}

unsafe fn cmask_any_set_in_range(m: *const scx_cmask, lo: c_uint, hi: c_uint) -> bool {
    // SAFETY: Caller ensures [lo, hi) is contained in m's active range.
    unsafe {
        if lo >= hi {
            return false;
        }
        let bits = lupos_scx_cid_cmask_bits_const(m).add((lo / 64 - (*m).base / 64) as usize);
        cmask_walk_op1(bits, lo, hi - lo)
    }
}

unsafe fn cmask_apply(dst: *mut scx_cmask, src: *const scx_cmask, op: CmaskOp2) {
    // SAFETY: Both native headers are stable; word pointers are not references.
    unsafe {
        cmask_walk_op2(
            lupos_scx_cid_cmask_bits(dst), (*dst).base, (*dst).nr_cids,
            lupos_scx_cid_cmask_bits_const(src), (*src).base, (*src).nr_cids,
            op,
        );
    }
}

/// AND the intersection of two cmask ranges.
///
/// # Safety
/// Headers and capacity are valid/stable; caller owns dst writes and ordering.
#[no_mangle]
pub unsafe extern "C" fn scx_cmask_and(dst: *mut scx_cmask, src: *const scx_cmask) {
    // SAFETY: Requirements passed through to the shared walker.
    unsafe { cmask_apply(dst, src, CmaskOp2::And) };
}

/// OR the intersection of two cmask ranges.
///
/// # Safety
/// Headers and capacity are valid/stable; caller owns dst writes and ordering.
#[no_mangle]
pub unsafe extern "C" fn scx_cmask_or(dst: *mut scx_cmask, src: *const scx_cmask) {
    // SAFETY: Requirements passed through to the shared walker.
    unsafe { cmask_apply(dst, src, CmaskOp2::Or) };
}

/// Copy the intersection, retaining dst bits outside src's range.
///
/// # Safety
/// Headers and capacity are valid/stable; caller owns dst writes and ordering.
#[no_mangle]
pub unsafe extern "C" fn scx_cmask_copy(dst: *mut scx_cmask, src: *const scx_cmask) {
    // SAFETY: Requirements passed through to the shared walker.
    unsafe { cmask_apply(dst, src, CmaskOp2::Copy) };
}

/// Clear dst bits set by src within their range intersection.
///
/// # Safety
/// Headers and capacity are valid/stable; caller owns dst writes and ordering.
#[no_mangle]
pub unsafe extern "C" fn scx_cmask_andnot(dst: *mut scx_cmask, src: *const scx_cmask) {
    // SAFETY: Requirements passed through to the shared walker.
    unsafe { cmask_apply(dst, src, CmaskOp2::Andnot) };
}

/// Test every set sub bit, including bits outside sup's range.
///
/// # Safety
/// Headers and capacity are valid/stable; caller supplies memory ordering.
#[no_mangle]
pub unsafe extern "C" fn scx_cmask_subset(sub: *const scx_cmask, sup: *const scx_cmask) -> bool {
    // SAFETY: Outside-range scans stay in sub. The common-range predicate only
    // reads words, including the const-to-mut pointer used by the shared walker.
    unsafe {
        let sup_end = (*sup).base + (*sup).nr_cids;
        let sub_end = (*sub).base + (*sub).nr_cids;
        if (*sub).base < (*sup).base
            && cmask_any_set_in_range(sub, (*sub).base, (*sup).base.min(sub_end))
        {
            return false;
        }
        if sub_end > sup_end
            && cmask_any_set_in_range(sub, (*sub).base.max(sup_end), sub_end)
        {
            return false;
        }
        !cmask_walk_op2(
            lupos_scx_cid_cmask_bits_const(sup).cast_mut(), (*sup).base, (*sup).nr_cids,
            lupos_scx_cid_cmask_bits_const(sub), (*sub).base, (*sub).nr_cids,
            CmaskOp2::Subset,
        )
    }
}

/// Test whether two cmasks share a set bit.
///
/// # Safety
/// Headers and capacity are valid/stable; caller supplies memory ordering.
#[no_mangle]
pub unsafe extern "C" fn scx_cmask_intersects(a: *const scx_cmask, b: *const scx_cmask) -> bool {
    // SAFETY: Intersects performs no write through the cast pointer.
    unsafe {
        cmask_walk_op2(
            lupos_scx_cid_cmask_bits_const(a).cast_mut(), (*a).base, (*a).nr_cids,
            lupos_scx_cid_cmask_bits_const(b), (*b).base, (*b).nr_cids,
            CmaskOp2::Intersects,
        )
    }
}

/// Test whether the active range contains no set bits.
///
/// # Safety
/// Header and capacity are valid/stable; caller supplies memory ordering.
#[no_mangle]
pub unsafe extern "C" fn scx_cmask_empty(m: *const scx_cmask) -> bool {
    // SAFETY: The scan covers exactly m's active range.
    unsafe { !cmask_any_set_in_range(m, (*m).base, (*m).base + (*m).nr_cids) }
}

unsafe fn cmask_ref_bind(
    sch: *mut scx_sched, src: *mut scx_cmask, base: c_uint,
    nr_cids: c_uint, nonempty: bool, out: *mut scx_cmask_ref,
) {
    // SAFETY: The caller keeps live scheduler tables pinned by their original
    // RCU or lock context. base is a valid CID even for an empty range.
    unsafe {
        (*out).sch = sch;
        (*out).src = src;
        (*out).base = base;
        (*out).nr_cids = nr_cids;
        let shards = lupos_scx_cid_to_shard_dereference();
        (*out).shard_first = *shards.add(base as usize);
        (*out).shard_end = if nonempty {
            *shards.add((base + nr_cids - 1) as usize) + 1
        } else {
            (*out).shard_first
        };
    }
}

/// Snapshot and validate the geometry of a rebased arena cmask.
///
/// # Safety
/// src is accessible arena storage, out is writable, and the caller protects
/// live scheduler tables and the ref's later use. No sleeping call is made.
#[no_mangle]
pub unsafe extern "C" fn scx_cmask_ref_init(
    sch: *mut scx_sched, src: *const scx_cmask, out: *mut scx_cmask_ref,
) -> c_int {
    // SAFETY: READ_ONCE snapshots prevent asynchronous arena header writes from
    // steering later sizing or offsets; refs keep only trusted geometry.
    unsafe {
        let base = lupos_scx_cid_read_u32(ptr::addr_of!((*src).base));
        let nr_cids = lupos_scx_cid_read_u32(ptr::addr_of!((*src).nr_cids));
        let alloc_words = lupos_scx_cid_read_u32(ptr::addr_of!((*src).alloc_words));
        let npossible = lupos_scx_cid_nr_possible();
        if lupos_scx_cid_unlikely_ref_invalid(
            base >= npossible || nr_cids > npossible - base
                || lupos_scx_cid_cmask_nr_words(nr_cids) > alloc_words,
        ) {
            return -EINVAL;
        }
        let nonempty = lupos_scx_cid_likely_ref_init_nonempty(nr_cids != 0);
        cmask_ref_bind(sch, src.cast_mut(), base, nr_cids, nonempty, out);
        0
    }
}

/// Bind kernel-trusted geometry and rewrite a potentially shared arena header.
///
/// # Safety
/// base/nr_cids are valid for the live tables and m's allocation. Caller pins
/// both storage and tables for the ref's lifetime, including subsequent use.
#[no_mangle]
pub unsafe extern "C" fn scx_cmask_ref_init_kern(
    sch: *mut scx_sched, m: *mut scx_cmask, base: c_uint,
    nr_cids: c_uint, out: *mut scx_cmask_ref,
) {
    // SAFETY: Header stores retain WRITE_ONCE. Geometry is never re-read from
    // BPF-mutable storage when binding the ref.
    unsafe {
        lupos_scx_cid_write_u32(ptr::addr_of_mut!((*m).base), base);
        lupos_scx_cid_write_u32(ptr::addr_of_mut!((*m).nr_cids), nr_cids);
        lupos_scx_cid_write_u32(
            ptr::addr_of_mut!((*m).alloc_words), lupos_scx_cid_cmask_nr_words(nr_cids),
        );
        let nonempty = lupos_scx_cid_likely_ref_kern_nonempty(nr_cids != 0);
        cmask_ref_bind(sch, m, base, nr_cids, nonempty, out);
    }
}

/// Copy one shard of an arena cmask through the ref's snapshotted geometry.
///
/// # Safety
/// r is a validated ref with pinned tables/storage; index belongs to its shard
/// range. out is a stable kernel cmask with its declared allocated capacity.
#[no_mangle]
pub unsafe extern "C" fn scx_cmask_ref_shard(
    r: *const scx_cmask_ref, index: c_int, out: *mut scx_cmask,
) {
    // SAFETY: The only arena accesses below are READ_ONCE word loads, bounded
    // by the saved geometry and the native shard range.
    unsafe {
        let shard = lupos_scx_cid_ranges_dereference().add(index as usize);
        let shard_base = (*shard).base_cid as c_uint;
        let shard_end = shard_base + (*shard).nr_cids as c_uint;
        let lo = (*r).base.max(shard_base);
        let hi = ((*r).base + (*r).nr_cids).min(shard_end);
        if lo >= hi {
            (*out).base = shard_base;
            (*out).nr_cids = 0;
            return;
        }
        let nr_words = (hi - 1) / 64 - lo / 64 + 1;
        if nr_words > (*out).alloc_words {
            lupos_scx_cid_error_capacity((*r).sch, (*out).alloc_words, nr_words, index);
            (*out).base = shard_base;
            (*out).nr_cids = 0;
            return;
        }
        (*out).base = lo;
        (*out).nr_cids = hi - lo;
        let src_offset = lo / 64 - (*r).base / 64;
        let src_bits = lupos_scx_cid_cmask_bits_const((*r).src);
        let out_bits = lupos_scx_cid_cmask_bits(out);
        for wi in 0..nr_words as usize {
            out_bits.add(wi).write(lupos_scx_cid_read_u64(src_bits.add(src_offset as usize + wi)));
        }
        *out_bits &= !0u64 << (lo & 63);
        *out_bits.add(nr_words as usize - 1) &= !0u64 >> (63 - ((hi - 1) & 63));
    }
}

/// OR a stable kernel cmask into the arena ref's range.
///
/// # Safety
/// r is validated, its storage stays live, and src has stable valid geometry.
#[no_mangle]
pub unsafe extern "C" fn scx_cmask_ref_or(r: *const scx_cmask_ref, src: *const scx_cmask) {
    // SAFETY: The arena header is not consulted; destination words retain
    // READ_ONCE/WRITE_ONCE under the caller's concurrency contract.
    unsafe {
        cmask_walk_op2(
            lupos_scx_cid_cmask_bits((*r).src), (*r).base, (*r).nr_cids,
            lupos_scx_cid_cmask_bits_const(src), (*src).base, (*src).nr_cids,
            CmaskOp2::RefOr,
        );
    }
}

/// Copy a stable kernel cmask into the arena ref's intersecting range.
///
/// # Safety
/// r is validated, its storage stays live, and src has stable valid geometry.
#[no_mangle]
pub unsafe extern "C" fn scx_cmask_ref_copy(r: *const scx_cmask_ref, src: *const scx_cmask) {
    // SAFETY: The shared walker uses the trusted snapshot for arena offsets.
    unsafe {
        cmask_walk_op2(
            lupos_scx_cid_cmask_bits((*r).src), (*r).base, (*r).nr_cids,
            lupos_scx_cid_cmask_bits_const(src), (*src).base, (*src).nr_cids,
            CmaskOp2::RefCopy,
        );
    }
}

/// Translate CPU mask membership into each word of a kernel-bound arena ref.
///
/// # Safety
/// r was bound from trusted geometry; mask is readable, and CID mappings remain
/// live under the caller's original RCU or lock protection for the entire walk.
#[no_mangle]
pub unsafe extern "C" fn scx_cmask_ref_from_cpumask(
    r: *const scx_cmask_ref, mask: *const cpumask,
) {
    // SAFETY: The active range determines every offset. Shared arena header
    // fields never influence this operation and all stores retain WRITE_ONCE.
    unsafe {
        let base = (*r).base;
        let nr_cids = (*r).nr_cids;
        if nr_cids == 0 {
            return;
        }
        let nr_words = (base + nr_cids - 1) / 64 - base / 64 + 1;
        let bits = lupos_scx_cid_cmask_bits((*r).src);
        for wi in 0..nr_words {
            let first_cid = (base / 64 + wi) * 64;
            let mut word = 0u64;
            for bit in 0..64 {
                let cid = first_cid + bit;
                if cid < base || cid >= base + nr_cids {
                    continue;
                }
                let mapping = lupos_scx_cid_to_cpu_dereference();
                let cpu = *mapping.add(cid as usize) as c_int;
                if lupos_scx_cid_mask_test(cpu, mask) {
                    word |= 1u64 << bit;
                }
            }
            lupos_scx_cid_write_u64(bits.add(wi as usize), word);
        }
    }
}

/// Register CID kfunc sets in their original short-circuit order.
///
/// # Safety
/// Called by scheduler initialization under the native BTF setup contract.
#[no_mangle]
pub unsafe extern "C" fn scx_cid_kfunc_init() -> c_int {
    // SAFETY: Each native leaf retains the BTF owner, context filter and flags.
    unsafe {
        let ret = lupos_scx_cid_register_init();
        if ret != 0 {
            return ret;
        }
        let ret = lupos_scx_cid_register_ops();
        if ret != 0 {
            return ret;
        }
        let ret = lupos_scx_cid_register_tracing();
        if ret != 0 {
            return ret;
        }
        lupos_scx_cid_register_syscall()
    }
}

// The starting Rust file recorded SOURCE-COMMIT:
// d482bb509b7d065808de40ce78b5bca39f40b783. This continuation is pinned to the
// supplied 126a30fae3bba11420ec2fcbde51a0a01bab1b5b Rust+C/header source set.
