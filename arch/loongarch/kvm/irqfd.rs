// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (C) 2024 Loongson Technology Corporation Limited
 */

// Translated dependencies:
// #include <linux/kvm_host.h>
// #include <trace/events/kvm.h>
// #include <asm/kvm_pch_pic.h>

use crate::*;

extern "C" {
    fn pch_pic_set_irq(pch_pic: *mut pch_pic, pin: u32, level: i32);
    fn pch_msi_set_irq(
        kvm: *mut kvm,
        e: *mut kvm_kernel_irq_routing_entry,
        level: i32,
    ) -> i32;
    fn kvm_arch_irqchip_in_kernel(kvm: *mut kvm) -> bool;
}

unsafe fn kvm_set_pic_irq(
    e: *mut kvm_kernel_irq_routing_entry,
    kvm: *mut kvm,
    _irq_source_id: i32,
    level: i32,
    _line_status: bool,
) -> i32 {
    /* PCH-PIC pin (0 ~ 64) <---> GSI (0 ~ 64) */
    pch_pic_set_irq((*kvm).arch.pch_pic, (*e).irqchip.pin, level);

    0
}

/*
 * kvm_set_msi: inject the MSI corresponding to the
 * MSI routing entry
 *
 * This is the entry point for irqfd MSI injection
 * and userspace MSI injection.
 */
unsafe extern "C" fn kvm_set_msi(
    e: *mut kvm_kernel_irq_routing_entry,
    kvm: *mut kvm,
    _irq_source_id: i32,
    level: i32,
    _line_status: bool,
) -> i32 {
    if level == 0 {
        return -1;
    }

    pch_msi_set_irq(kvm, e, level)
}

/*
 * kvm_set_routing_entry: populate a kvm routing entry
 * from a user routing entry
 *
 * @kvm: the VM this entry is applied to
 * @e: kvm kernel routing entry handle
 * @ue: user api routing entry handle
 * return 0 on success, -EINVAL on errors.
 */
unsafe extern "C" fn kvm_set_routing_entry(
    kvm: *mut kvm,
    e: *mut kvm_kernel_irq_routing_entry,
    ue: *const kvm_irq_routing_entry,
) -> i32 {
    match (*ue).type_ {
        KVM_IRQ_ROUTING_IRQCHIP => {
            (*e).set = Some(kvm_set_pic_irq);
            (*e).irqchip.irqchip = (*ue).u.irqchip.irqchip;
            (*e).irqchip.pin = (*ue).u.irqchip.pin;

            if (*e).irqchip.pin >= KVM_IRQCHIP_NUM_PINS
                || (*e).irqchip.irqchip >= KVM_NR_IRQCHIPS
            {
                return -EINVAL;
            }

            0
        }
        KVM_IRQ_ROUTING_MSI => {
            (*e).set = Some(kvm_set_msi);
            (*e).msi.address_lo = (*ue).u.msi.address_lo;
            (*e).msi.address_hi = (*ue).u.msi.address_hi;
            (*e).msi.data = (*ue).u.msi.data;
            0
        }
        _ => -EINVAL,
    }
}

unsafe extern "C" fn kvm_arch_set_irq_inatomic(
    e: *mut kvm_kernel_irq_routing_entry,
    kvm: *mut kvm,
    _irq_source_id: i32,
    level: i32,
    _line_status: bool,
) -> i32 {
    if level == 0 {
        return -EWOULDBLOCK;
    }

    match (*e).type_ {
        KVM_IRQ_ROUTING_IRQCHIP => {
            pch_pic_set_irq((*kvm).arch.pch_pic, (*e).irqchip.pin, level);
            0
        }
        KVM_IRQ_ROUTING_MSI => pch_msi_set_irq(kvm, e, level),
        _ => -EWOULDBLOCK,
    }
}

unsafe extern "C" fn kvm_arch_intc_initialized(kvm: *mut kvm) -> bool {
    kvm_arch_irqchip_in_kernel(kvm)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
