// SPDX-License-Identifier: GPL-2.0

// Dependencies: linux/cpumask.h, linux/export.h, linux/numa.h, linux/topology.h.
//
// CONFIG_CPUMASK_OFFSTACK=y keeps the original C provider: lib/Kconfig only
// offers RUST_CPUMASK without off-stack masks, whose allocators are not here.

use core::ptr::{addr_of, read_volatile};
use kernel::bindings::{__num_online_cpus, cpumask};
use kernel::ffi::{c_int, c_uint};

extern "C" {
    fn lupos_cpumask_nr_cpu_ids() -> c_uint;
    fn lupos_cpumask_find_nth_online(i: c_uint, node: c_int) -> c_uint;
    fn lupos_cpumask_warn_local_spread();
    fn lupos_cpumask_distribute_prev_read() -> c_int;
    fn lupos_cpumask_distribute_prev_write(next: c_uint);
    fn lupos_cpumask_next_and_wrap(n: c_int, src1p: *const cpumask, src2p: *const cpumask) -> c_uint;
    fn lupos_cpumask_next_wrap(n: c_int, src: *const cpumask) -> c_uint;
}

/// num_online_cpus(): the raw atomic read of `__num_online_cpus`.
fn num_online_cpus() -> c_uint {
    // SAFETY: __num_online_cpus is an always-valid kernel atomic_t, read with
    // the volatile access of raw_atomic_read().
    unsafe { read_volatile(addr_of!(__num_online_cpus.counter)) as c_uint }
}

/**
 * cpumask_local_spread - select the i'th cpu based on NUMA distances
 * @i: index number
 * @node: local numa_node
 *
 * Return: online CPU according to a numa aware policy; local cpus are returned
 * first, followed by non-local ones, then it wraps around.
 *
 * For those who wants to enumerate all CPUs based on their NUMA distances,
 * i.e. call this function in a loop, like:
 *
 * for (i = 0; i < num_online_cpus(); i++) {
 *	cpu = cpumask_local_spread(i, node);
 *	do_something(cpu);
 * }
 *
 * There's a better alternative based on for_each()-like iterators:
 *
 *	for_each_numa_hop_mask(mask, node) {
 *		for_each_cpu_andnot(cpu, mask, prev)
 *			do_something(cpu);
 *		prev = mask;
 *	}
 *
 * It's simpler and more verbose than above. Complexity of iterator-based
 * enumeration is O(sched_domains_numa_levels * nr_cpu_ids), while
 * cpumask_local_spread() when called for each cpu is
 * O(sched_domains_numa_levels * nr_cpu_ids * log(nr_cpu_ids)).
 */
/// Returns the `i`'th online CPU in NUMA distance order from `node`.
#[no_mangle]
pub extern "C" fn cpumask_local_spread(mut i: c_uint, node: c_int) -> c_uint {
    /* Wrap: we always want a cpu. */
    i %= num_online_cpus();

    // SAFETY: plain native calls on the online mask and CPU-count globals.
    unsafe {
        let cpu = lupos_cpumask_find_nth_online(i, node);

        if cpu >= lupos_cpumask_nr_cpu_ids() {
            lupos_cpumask_warn_local_spread();
        }
        cpu
    }
}
// EXPORT_SYMBOL(cpumask_local_spread);

/**
 * cpumask_any_and_distribute - Return an arbitrary cpu within src1p & src2p.
 * @src1p: first &cpumask for intersection
 * @src2p: second &cpumask for intersection
 *
 * Iterated calls using the same srcp1 and srcp2 will be distributed within
 * their intersection.
 *
 * Return: >= nr_cpu_ids if the intersection is empty.
 */
/// # Safety
/// `src1p` and `src2p` point to valid cpumasks.
#[no_mangle]
pub unsafe extern "C" fn cpumask_any_and_distribute(
    src1p: *const cpumask,
    src2p: *const cpumask,
) -> c_uint {
    // SAFETY: the masks are valid; the per-CPU slot is this CPU's.
    unsafe {
        /* NOTE: our first selection will skip 0. */
        let prev = lupos_cpumask_distribute_prev_read() as c_uint;

        let next = lupos_cpumask_next_and_wrap(prev as c_int, src1p, src2p);
        if next < lupos_cpumask_nr_cpu_ids() {
            lupos_cpumask_distribute_prev_write(next);
        }

        next
    }
}
// EXPORT_SYMBOL(cpumask_any_and_distribute);

/**
 * cpumask_any_distribute - Return an arbitrary cpu from srcp
 * @srcp: &cpumask for selection
 *
 * Return: >= nr_cpu_ids if the intersection is empty.
 */
/// # Safety
/// `srcp` points to a valid cpumask.
#[no_mangle]
pub unsafe extern "C" fn cpumask_any_distribute(srcp: *const cpumask) -> c_uint {
    // SAFETY: the mask is valid; the per-CPU slot is this CPU's.
    unsafe {
        /* NOTE: our first selection will skip 0. */
        let prev = lupos_cpumask_distribute_prev_read() as c_uint;
        let next = lupos_cpumask_next_wrap(prev as c_int, srcp);
        if next < lupos_cpumask_nr_cpu_ids() {
            lupos_cpumask_distribute_prev_write(next);
        }

        next
    }
}
// EXPORT_SYMBOL(cpumask_any_distribute);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
