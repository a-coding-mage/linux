/* SPDX-License-Identifier: GPL-2.0 */

// Dependencies supplied by the corresponding architecture headers.

#[repr(C)]
pub enum ipi_vector {
    XEN_PLACEHOLDER_VECTOR,

    /* Xen IPIs go here */
    XEN_NR_IPIS,
}

#[inline]
pub unsafe fn xen_irqs_disabled(regs: *mut pt_regs) -> ::core::ffi::c_int {
    regs_irqs_disabled(regs)
}

#[inline]
pub unsafe fn xchg_xen_ulong<T>(ptr: *mut T, val: T) -> T {
    xchg(ptr, val)
}

/* Rebind event channel is supported by default */
#[inline]
pub fn xen_support_evtchn_rebind() -> bool {
    true
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
