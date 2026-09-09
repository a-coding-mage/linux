/* SPDX-License-Identifier: GPL-2.0 */

/* Dependencies supplied by the surrounding kernel translation:
 * linux/bitops.h and linux/threads.h
 */

/* Don't assign or return these: may not be this big! */
#[repr(C)]
pub struct cpumask {
    pub bits: [usize; (NR_CPUS + (usize::BITS as usize) - 1) / (usize::BITS as usize)],
}

pub type cpumask_t = cpumask;

/**
 * cpumask_bits - get the bits in a cpumask
 * @maskp: the struct cpumask *
 *
 * You should only assume nr_cpu_ids bits of this mask are valid.  This is
 * a macro so it's const-correct.
 */
#[macro_export]
macro_rules! cpumask_bits {
    ($maskp:expr) => {
        unsafe { &(*$maskp).bits }
    };
}

/*
 * cpumask_var_t: struct cpumask for stack usage.
 *
 * Oh, the wicked games we play!  In order to make kernel coding a
 * little more difficult, we typedef cpumask_var_t to an array or a
 * pointer: doing &mask on an array is a noop, so it still works.
 *
 * i.e.
 *	cpumask_var_t tmpmask;
 *	if (!alloc_cpumask_var(&tmpmask, GFP_KERNEL))
 *		return -ENOMEM;
 *
 *	  ... use 'tmpmask' like a normal struct cpumask * ...
 *
 *	free_cpumask_var(tmpmask);
 *
 *
 * However, one notable exception is there. alloc_cpumask_var() allocates
 * only nr_cpumask_bits bits (in the other hand, real cpumask_t always has
 * NR_CPUS bits). Therefore you don't have to dereference cpumask_var_t.
 *
 *	cpumask_var_t tmpmask;
 *	if (!alloc_cpumask_var(&tmpmask, GFP_KERNEL))
 *		return -ENOMEM;
 *
 *	var = *tmpmask;
 *
 * This code makes NR_CPUS length memcopy and brings to a memory corruption.
 * cpumask_copy() provide safe copy functionality.
 *
 * Note that there is another evil here: If you define a cpumask_var_t
 * as a percpu variable then the way to obtain the address of the cpumask
 * structure differently influences what this_cpu_* operation needs to be
 * used. Please use this_cpu_cpumask_var_t in those cases. The direct use
 * of this_cpu_ptr() or this_cpu_read() will lead to failures when the
 * other type of cpumask_var_t implementation is configured.
 *
 * Please also note that __cpumask_var_read_mostly can be used to declare
 * a cpumask_var_t variable itself (not its content) as read mostly.
 */
// CONFIG_CPUMASK_OFFSTACK selects the pointer representation at build time.
#[cfg(CONFIG_CPUMASK_OFFSTACK)]
pub type cpumask_var_t = *mut cpumask;

#[cfg(not(CONFIG_CPUMASK_OFFSTACK))]
pub type cpumask_var_t = [cpumask; 1];

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
