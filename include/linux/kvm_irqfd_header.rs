/* SPDX-License-Identifier: GPL-2.0-only */
/*
 *
 * irqfd: Allows an fd to be used to inject an interrupt to the guest
 * Credit goes to Avi Kivity for the original idea.
 */

/* Dependencies supplied by the surrounding kernel translation. */

/*
 * Resampling irqfds are a special variety of irqfds used to emulate
 * level triggered interrupts.  The interrupt is asserted on eventfd
 * trigger.  On acknowledgment through the irq ack notifier, the
 * interrupt is de-asserted and userspace is notified through the
 * resamplefd.  All resamplers on the same gsi are de-asserted
 * together, so we don't need to track the state of each individual
 * user.  We can also therefore share the same irq source ID.
 */
#[repr(C)]
pub struct kvm_kernel_irqfd_resampler {
    pub kvm: *mut kvm,
    /*
     * List of resampling struct _irqfd objects sharing this gsi.
     * RCU list modified under kvm->irqfds.resampler_lock
     */
    pub list: list_head,
    pub notifier: kvm_irq_ack_notifier,
    /*
     * Entry in list of kvm->irqfd.resampler_list.  Use for sharing
     * resamplers among irqfds on the same gsi.
     * RCU list modified under kvm->irqfds.resampler_lock
     */
    pub link: list_head,
}

#[repr(C)]
pub struct kvm_kernel_irqfd {
    /* Used for MSI fast-path */
    pub kvm: *mut kvm,
    pub wait: wait_queue_entry_t,
    /* Update side is protected by irqfds.lock */
    pub irq_entry: kvm_kernel_irq_routing_entry,
    pub irq_entry_sc: seqcount_spinlock_t,
    /* Used for level IRQ fast-path */
    pub gsi: i32,
    pub inject: work_struct,
    /* The resampler used by this irqfd (resampler-only) */
    pub resampler: *mut kvm_kernel_irqfd_resampler,
    /* Eventfd notified on resample (resampler-only) */
    pub resamplefd: *mut eventfd_ctx,
    /* Entry in list of irqfds for a resampler (resampler-only) */
    pub resampler_link: list_head,
    /* Used for setup/shutdown */
    pub eventfd: *mut eventfd_ctx,
    pub list: list_head,
    pub shutdown: work_struct,
    pub consumer: irq_bypass_consumer,
    pub producer: *mut irq_bypass_producer,

    pub irq_bypass_vcpu: *mut kvm_vcpu,
    pub vcpu_list: list_head,
    pub irq_bypass_data: *mut core::ffi::c_void,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
