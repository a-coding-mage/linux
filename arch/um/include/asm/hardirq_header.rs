/* SPDX-License-Identifier: GPL-2.0 */

// Dependencies supplied by the surrounding kernel translation:
// linux/cache.h, linux/threads.h, and linux/irq.h.

pub const __ARCH_IRQ_EXIT_IRQS_DISABLED: ::core::ffi::c_int = 1;

#[repr(C)]
#[cfg_attr(target_pointer_width = "64", repr(align(64)))]
#[cfg_attr(target_pointer_width = "32", repr(align(32)))]
pub struct irq_cpustat_t {
    pub __softirq_pending: ::core::ffi::c_uint,
    // Corresponds to IS_ENABLED(CONFIG_SMP).
    #[cfg(feature = "CONFIG_SMP")]
    pub irq_resched_count: ::core::ffi::c_uint,
    #[cfg(feature = "CONFIG_SMP")]
    pub irq_call_count: ::core::ffi::c_uint,
}

// Corresponds to DECLARE_PER_CPU_SHARED_ALIGNED(irq_cpustat_t, irq_stat).
unsafe extern "C" {
    pub static mut irq_stat: irq_cpustat_t;
}

// Corresponds to this_cpu_inc(irq_stat.member); per-CPU access is supplied by
// the surrounding kernel translation.
#[macro_export]
macro_rules! inc_irq_stat {
    ($member:ident) => {
        unsafe {
            $crate::irq_stat.$member = $crate::irq_stat.$member.wrapping_add(1);
        }
    };
}

pub const __ARCH_IRQ_STAT: () = ();

unsafe extern "C" {
    fn pr_crit(fmt: *const ::core::ffi::c_char, ...);
}

#[inline]
pub unsafe fn ack_bad_irq(irq: ::core::ffi::c_uint) {
    static FORMAT: &[u8] = b"unexpected IRQ trap at vector %02x\n\0";
    unsafe {
        pr_crit(FORMAT.as_ptr() as *const ::core::ffi::c_char, irq);
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
