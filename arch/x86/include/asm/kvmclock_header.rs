/* SPDX-License-Identifier: GPL-2.0 */

// Dependency supplied by the kernel percpu infrastructure.
// `hv_clock_per_cpu` is a per-CPU pointer to `pvclock_vsyscall_time_info`.
extern "C" {
    pub static mut hv_clock_per_cpu:
        *mut crate::pvclock_vsyscall_time_info;
}

/// Return this CPU's pvclock vCPU time information.
#[inline(always)]
pub unsafe fn this_cpu_pvti() -> *mut crate::pvclock_vcpu_time_info {
    // `this_cpu_read` is the external per-CPU access primitive corresponding
    // to the Linux kernel macro used by the original header.
    &mut (*this_cpu_read!(hv_clock_per_cpu)).pvti
}

/// Return this CPU's pvclock syscall time information.
#[inline]
pub unsafe fn this_cpu_hvclock() -> *mut crate::pvclock_vsyscall_time_info {
    this_cpu_read!(hv_clock_per_cpu)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
