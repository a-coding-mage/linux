/* SPDX-License-Identifier: GPL-2.0 */

// Dependencies supplied by the surrounding kernel translation unit:
// linux/entry-common.h, linux/kvm_types.h, linux/hrtimer_rearm.h,
// asm/fred.h, and asm/desc.h.

#[cfg(feature = "config_kvm_intel")]
pub unsafe fn x86_entry_from_kvm(event_type: ::core::ffi::c_uint, vector: ::core::ffi::c_uint) {
    if event_type == EVENT_TYPE_EXTINT {
        #[cfg(target_arch = "x86_64")]
        {
            /*
             * Use FRED dispatch, even when running IDT. The dispatch
             * tables are kept in sync between FRED and IDT, and the FRED
             * dispatch works well with CFI.
             */
            fred_entry_from_kvm(event_type, vector);
        }
        #[cfg(not(target_arch = "x86_64"))]
        {
            idt_entry_from_kvm(vector);
        }

        /*
         * Strictly speaking, only the NMI path requires noinstr.
         */
        instrumentation_begin();
        /*
         * KVM/VMX will dispatch from IRQ-disabled but for a context
         * that will have IRQs-enabled. This confuses the entry code
         * and it will not have reprogrammed the timer. Do so now.
         */
        hrtimer_rearm_deferred();
        instrumentation_end();

        return;
    }

    WARN_ON_ONCE(event_type != EVENT_TYPE_NMI);

    #[cfg(target_arch = "x86_64")]
    if cpu_feature_enabled(X86_FEATURE_FRED) {
        return fred_entry_from_kvm(event_type, vector);
    }

    /*
     * Notably, we must use IDT dispatch for NMI when running in IDT mode.
     * The FRED NMI context is significantly different and will not work
     * right (specifically FRED fixed the NMI recursion issue).
     */
    idt_do_nmi_irqoff();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
