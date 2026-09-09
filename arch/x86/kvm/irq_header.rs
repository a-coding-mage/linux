/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * irq.h: in kernel interrupt controller related definitions
 * Copyright (c) 2007, Intel Corporation.
 *
 * Authors:
 *   Yaozu (Eddie) Dong <Eddie.dong@intel.com>
 */

// C dependencies: linux/mm_types.h, linux/hrtimer.h, linux/kvm_host.h,
// linux/spinlock.h, kvm/iodev.h, and lapic.h.

#[cfg(feature = "CONFIG_KVM_IOAPIC")]
pub const PIC_NUM_PINS: usize = 16;

#[cfg(feature = "CONFIG_KVM_IOAPIC")]
#[inline]
pub const fn SELECT_PIC(irq: i32) -> i32 {
    if irq < 8 { KVM_IRQCHIP_PIC_MASTER } else { KVM_IRQCHIP_PIC_SLAVE }
}

pub enum kvm {}
pub enum kvm_vcpu {}

#[cfg(feature = "CONFIG_KVM_IOAPIC")]
#[repr(C)]
pub struct kvm_kpic_state {
    pub last_irr: u8,
    pub irr: u8,
    pub imr: u8,
    pub isr: u8,
    pub priority_add: u8,
    pub irq_base: u8,
    pub read_reg_select: u8,
    pub poll: u8,
    pub special_mask: u8,
    pub init_state: u8,
    pub auto_eoi: u8,
    pub rotate_on_auto_eoi: u8,
    pub special_fully_nested_mode: u8,
    pub init4: u8,
    pub elcr: u8,
    pub elcr_mask: u8,
    pub isr_ack: u8,
    pub pics_state: *mut kvm_pic,
}

#[cfg(feature = "CONFIG_KVM_IOAPIC")]
#[repr(C)]
pub struct kvm_pic {
    pub lock: spinlock_t,
    pub wakeup_needed: bool,
    pub pending_acks: ::core::ffi::c_uint,
    pub kvm: *mut kvm,
    pub pics: [kvm_kpic_state; 2],
    pub output: i32,
    pub dev_master: kvm_io_device,
    pub dev_slave: kvm_io_device,
    pub dev_elcr: kvm_io_device,
    pub irq_states: [::core::ffi::c_ulong; PIC_NUM_PINS],
}

#[cfg(feature = "CONFIG_KVM_IOAPIC")]
extern "C" {
    pub fn kvm_pic_init(kvm: *mut kvm) -> i32;
    pub fn kvm_pic_destroy(kvm: *mut kvm);
    pub fn kvm_pic_read_irq(kvm: *mut kvm) -> i32;
    pub fn kvm_pic_update_irq(s: *mut kvm_pic);
    pub fn kvm_pic_set_irq(e: *mut kvm_kernel_irq_routing_entry, kvm: *mut kvm,
                           irq_source_id: i32, level: i32, line_status: bool) -> i32;
    pub fn kvm_setup_default_ioapic_and_pic_routing(kvm: *mut kvm) -> i32;
    pub fn kvm_vm_ioctl_get_irqchip(kvm: *mut kvm, chip: *mut kvm_irqchip) -> i32;
    pub fn kvm_vm_ioctl_set_irqchip(kvm: *mut kvm, chip: *mut kvm_irqchip) -> i32;
}

#[inline]
pub unsafe fn irqchip_full(kvm: *mut kvm) -> i32 {
    // Matches smp_wmb() when setting irqchip_mode.
    #[cfg(feature = "CONFIG_KVM_IOAPIC")]
    {
        let mode = (*kvm).arch.irqchip_mode;
        smp_rmb();
        return (mode == KVM_IRQCHIP_KERNEL) as i32;
    }
    #[cfg(not(feature = "CONFIG_KVM_IOAPIC"))]
    { let _ = kvm; 0 }
}

#[inline]
pub unsafe fn pic_in_kernel(kvm: *mut kvm) -> i32 {
    irqchip_full(kvm)
}

#[inline]
pub unsafe fn irqchip_split(kvm: *mut kvm) -> i32 {
    let mode = (*kvm).arch.irqchip_mode;
    // Matches smp_wmb() when setting irqchip_mode.
    smp_rmb();
    (mode == KVM_IRQCHIP_SPLIT) as i32
}

#[inline]
pub unsafe fn irqchip_in_kernel(kvm: *mut kvm) -> i32 {
    let mode = (*kvm).arch.irqchip_mode;
    // Matches smp_wmb() when setting irqchip_mode.
    smp_rmb();
    (mode != KVM_IRQCHIP_NONE) as i32
}

extern "C" {
    pub fn kvm_cpu_has_injectable_intr(v: *mut kvm_vcpu) -> i32;
    pub fn kvm_cpu_has_interrupt(vcpu: *mut kvm_vcpu) -> i32;
    pub fn kvm_cpu_has_extint(v: *mut kvm_vcpu) -> i32;
    pub fn kvm_cpu_get_extint(v: *mut kvm_vcpu) -> i32;
    pub fn kvm_cpu_get_interrupt(v: *mut kvm_vcpu) -> i32;
    pub fn kvm_inject_pending_timer_irqs(vcpu: *mut kvm_vcpu);
    pub fn kvm_inject_apic_timer_irqs(vcpu: *mut kvm_vcpu);
    pub fn kvm_apic_nmi_wd_deliver(vcpu: *mut kvm_vcpu);
    pub fn __kvm_migrate_apic_timer(vcpu: *mut kvm_vcpu);
    pub fn __kvm_migrate_pit_timer(vcpu: *mut kvm_vcpu);
    pub fn __kvm_migrate_timers(vcpu: *mut kvm_vcpu);
    pub fn apic_has_pending_timer(vcpu: *mut kvm_vcpu) -> i32;
}

#[inline]
pub unsafe fn kvm_warn_on_lost_irq(vcpu: *mut kvm_vcpu) {
    // WARN if an IRQ was lost between detecting the IRQ and grabbing the IRQ
    // for injection, except for the documented in-kernel PIC and Xen cases.
    WARN_ON_ONCE(pic_in_kernel((*vcpu).kvm) == 0 && !IS_ENABLED_CONFIG_KVM_XEN);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
