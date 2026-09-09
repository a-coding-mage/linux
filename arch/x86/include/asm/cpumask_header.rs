/* SPDX-License-Identifier: GPL-2.0 */

// Dependencies supplied by the surrounding translation unit:
// linux/compiler.h and linux/cpumask.h

unsafe extern "C" {
    pub fn setup_cpu_local_masks();
}

/*
 * NMI and MCE exceptions need cpu_is_offline() _really_ early,
 * provide an arch_ special for them to avoid instrumentation.
 *
 * The original implementation is selected at build time by NR_CPUS > 1.
 */
#[inline(always)]
pub unsafe fn arch_cpu_online(cpu: ::core::ffi::c_int) -> bool {
    if NR_CPUS > 1 {
        arch_test_bit(cpu, cpumask_bits(cpu_online_mask))
    } else {
        cpu == 0
    }
}

#[inline(always)]
pub unsafe fn arch_cpumask_clear_cpu(
    cpu: ::core::ffi::c_int,
    dstp: *mut cpumask,
) {
    if NR_CPUS > 1 {
        arch_clear_bit(cpumask_check(cpu), cpumask_bits(dstp));
    } else {
        // No operation when NR_CPUS == 1.
    }
}

#[inline(always)]
pub unsafe fn arch_cpu_is_offline(cpu: ::core::ffi::c_int) -> bool {
    // unlikely(!arch_cpu_online(cpu))
    !arch_cpu_online(cpu)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
