/* SPDX-License-Identifier: GPL-2.0 */

// C dependencies: <asm/ptrace.h>, <asm/atomic.h>

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum IpiVector {
    XenPlaceholderVector,
    // Xen IPIs go here
    XenNrIpis,
}

#[inline]
pub unsafe fn xen_irqs_disabled(regs: *mut crate::pt_regs) -> i32 {
    raw_irqs_disabled_flags((*regs).ARM_cpsr)
}

// Equivalent of the C xchg_xen_ulong macro. The referenced atomic and
// container helpers are supplied by the corresponding architecture code.
#[macro_export]
macro_rules! xchg_xen_ulong {
    ($ptr:expr, $val:expr) => {
        unsafe {
            atomic64_xchg(
                container_of(
                    ($ptr as *mut i64),
                    atomic64_t,
                    counter,
                ),
                $val,
            )
        }
    };
}

/* Rebind event channel is supported by default */
#[inline]
pub const fn xen_support_evtchn_rebind() -> bool {
    true
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
