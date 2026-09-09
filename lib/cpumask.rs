// SPDX-License-Identifier: GPL-2.0
// External Linux-kernel types, constants, macros, and functions are supplied
// by the surrounding translation unit.

/* These are not inline because of header tangles. */
#[cfg(feature = "CONFIG_CPUMASK_OFFSTACK")]
/// Allocate a struct cpumask on a given node.
pub unsafe fn alloc_cpumask_var_node(mask: *mut cpumask_var_t, flags: gfp_t, node: i32) -> bool {
    unsafe {
        *mask = kmalloc_node(cpumask_size(), flags, node);

        #[cfg(feature = "CONFIG_DEBUG_PER_CPU_MAPS")]
        if (*mask).is_null() {
            printk(KERN_ERR, "=> alloc_cpumask_var: failed!\n");
            dump_stack();
        }

        !(*mask).is_null()
    }
}

#[cfg(feature = "CONFIG_CPUMASK_OFFSTACK")]
pub unsafe fn alloc_bootmem_cpumask_var(mask: *mut cpumask_var_t) {
    unsafe {
        *mask = memblock_alloc_or_panic(cpumask_size(), SMP_CACHE_BYTES);
    }
}

#[cfg(feature = "CONFIG_CPUMASK_OFFSTACK")]
pub unsafe fn free_cpumask_var(mask: cpumask_var_t) {
    unsafe {
        kfree(mask);
    }
}

#[cfg(feature = "CONFIG_CPUMASK_OFFSTACK")]
pub unsafe fn free_bootmem_cpumask_var(mask: cpumask_var_t) {
    unsafe {
        memblock_free(mask, cpumask_size());
    }
}

/// Select the i'th cpu based on NUMA distances.
pub unsafe fn cpumask_local_spread(i: u32, node: i32) -> u32 {
    let mut i = i;
    let cpu: u32;

    /* Wrap: we always want a cpu. */
    i %= num_online_cpus();

    cpu = unsafe { sched_numa_find_nth_cpu(cpu_online_mask, i, node) };

    unsafe { WARN_ON(cpu >= nr_cpu_ids) };
    cpu
}

static mut distribute_cpu_mask_prev: i32 = 0;

/// Return an arbitrary cpu within src1p & src2p.
pub unsafe fn cpumask_any_and_distribute(
    src1p: *const struct_cpumask,
    src2p: *const struct_cpumask,
) -> u32 {
    let prev: u32;
    let next: u32;

    /* NOTE: our first selection will skip 0. */
    prev = unsafe { distribute_cpu_mask_prev as u32 };

    next = unsafe { cpumask_next_and_wrap(prev, src1p, src2p) };
    if next < unsafe { nr_cpu_ids } {
        unsafe { distribute_cpu_mask_prev = next as i32 };
    }

    next
}

/// Return an arbitrary cpu from srcp.
pub unsafe fn cpumask_any_distribute(srcp: *const struct_cpumask) -> u32 {
    let prev: u32;
    let next: u32;

    /* NOTE: our first selection will skip 0. */
    prev = unsafe { distribute_cpu_mask_prev as u32 };
    next = unsafe { cpumask_next_wrap(prev, srcp) };
    if next < unsafe { nr_cpu_ids } {
        unsafe { distribute_cpu_mask_prev = next as i32 };
    }

    next
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
