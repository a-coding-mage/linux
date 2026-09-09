/* SPDX-License-Identifier: GPL-2.0 */

// Dependency supplied by the Xen headers.

#[repr(C)]
pub enum ipi_vector {
    XEN_RESCHEDULE_VECTOR,
    XEN_CALL_FUNCTION_VECTOR,
    XEN_CALL_FUNCTION_SINGLE_VECTOR,
    XEN_SPIN_UNLOCK_VECTOR,
    XEN_IRQ_WORK_VECTOR,
    XEN_NMI_VECTOR,

    XEN_NR_IPIS,
}

pub unsafe fn xen_irqs_disabled(regs: *const pt_regs) -> bool {
    raw_irqs_disabled_flags((*regs).flags)
}

/* No need for a barrier -- XCHG is a barrier on x86. */
#[macro_export]
macro_rules! xchg_xen_ulong {
    ($ptr:expr, $val:expr) => {
        xchg($ptr, $val)
    };
}

extern "C" {
    static mut xen_have_vector_callback: bool;
}

/*
 * Events delivered via platform PCI interrupts are always
 * routed to vcpu 0 and hence cannot be rebound.
 */
pub fn xen_support_evtchn_rebind() -> bool {
    !xen_hvm_domain() || unsafe { xen_have_vector_callback }
}

extern "C" {
    static mut xen_percpu_upcall: bool;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
