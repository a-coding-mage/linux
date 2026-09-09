// SPDX-License-Identifier: GPL-2.0-only
/*
 * Support of MSI, HPET and DMAR interrupts.
 *
 * Copyright (C) 1997, 1998, 1999, 2000, 2009 Ingo Molnar, Hajnalka Szabo
 *	Moved from arch/x86/kernel/apic/io_apic.c.
 * Jiang Liu <jiang.liu@linux.intel.com>
 *	Convert to hierarchical irqdomain
 */

// C dependencies: linux/mm.h, linux/interrupt.h, linux/irq.h, linux/pci.h,
// linux/dmar.h, linux/hpet.h, linux/msi.h, asm/irqdomain.h, asm/hpet.h,
// asm/hw_irq.h, asm/apic.h, asm/irq_remapping.h, asm/xen/hypervisor.h

static mut x86_pci_msi_default_domain: *mut irq_domain = core::ptr::null_mut();

unsafe fn irq_msi_update_msg(irqd: *mut irq_data, cfg: *mut irq_cfg) {
    let mut msg: [msi_msg; 2] = [core::mem::zeroed(), core::mem::zeroed()];

    __irq_msi_compose_msg(cfg, msg.as_mut_ptr(), false);
    ((*irq_data_get_irq_chip(irqd)).irq_write_msi_msg)(irqd, msg.as_mut_ptr());
}

unsafe fn msi_set_affinity(
    irqd: *mut irq_data,
    mask: *const cpumask,
    force: bool,
) -> i32 {
    let mut old_cfg: irq_cfg = core::mem::zeroed();
    let cfg: *mut irq_cfg = irqd_cfg(irqd);
    let parent: *mut irq_data = (*irqd).parent_data;
    let cpu: u32;
    let ret: i32;

    /* Save the current configuration */
    cpu = cpumask_first(irq_data_get_effective_affinity_mask(irqd));
    old_cfg = *cfg;

    /* Allocate a new target vector */
    ret = ((*(*parent).chip).irq_set_affinity)(parent, mask, force);
    if ret < 0 || ret == IRQ_SET_MASK_OK_DONE {
        return ret;
    }

    /*
     * For non-maskable and non-remapped MSI interrupts the migration
     * to a different destination CPU and a different vector has to be
     * done careful to handle the possible stray interrupt which can be
     * caused by the non-atomic update of the address/data pair.
     *
     * Direct update is possible when:
     * - The MSI is maskable (remapped MSI does not use this code path).
     *   The reservation mode bit is set in this case.
     * - The new vector is the same as the old vector
     * - The old vector is MANAGED_IRQ_SHUTDOWN_VECTOR (interrupt starts up)
     * - The interrupt is not yet started up
     * - The new destination CPU is the same as the old destination CPU
     */
    if !irqd_can_reserve(irqd)
        || (*cfg).vector == old_cfg.vector
        || old_cfg.vector == MANAGED_IRQ_SHUTDOWN_VECTOR
        || !irqd_is_started(irqd)
        || (*cfg).dest_apicid == old_cfg.dest_apicid
    {
        irq_msi_update_msg(irqd, cfg);
        return ret;
    }

    /* Paranoia: Validate that the interrupt target is the local CPU. */
    if WARN_ON_ONCE(cpu != smp_processor_id()) {
        irq_msi_update_msg(irqd, cfg);
        return ret;
    }

    lock_vector_lock();

    if IS_ERR_OR_NULL(this_cpu_read(vector_irq[(*cfg).vector as usize])) {
        this_cpu_write(vector_irq[(*cfg).vector as usize], VECTOR_RETRIGGERED);
    }

    /* Redirect it to the new vector on the local CPU temporarily */
    old_cfg.vector = (*cfg).vector;
    irq_msi_update_msg(irqd, &mut old_cfg);

    /* Now transition it to the target CPU */
    irq_msi_update_msg(irqd, cfg);

    unlock_vector_lock();

    if lapic_vector_set_in_irr((*cfg).vector) {
        ((*irq_data_get_irq_chip(irqd)).irq_retrigger)(irqd);
    }

    ret
}

/**
 * pci_dev_has_default_msi_parent_domain - Check whether the device has the default
 *                                           MSI parent domain associated
 * @dev:     Pointer to the PCI device
 */
unsafe fn pci_dev_has_default_msi_parent_domain(dev: *mut pci_dev) -> bool {
    let mut domain = dev_get_msi_domain(&mut (*dev).dev);

    if domain.is_null() {
        domain = dev_get_msi_domain(&mut (*(*dev).bus).dev);
    }
    if domain.is_null() {
        return false;
    }

    domain == x86_vector_domain
}

/** x86_msi_prepare - Setup of msi_alloc_info_t for allocations */
unsafe fn x86_msi_prepare(
    domain: *mut irq_domain,
    _dev: *mut device,
    _nvec: i32,
    alloc: *mut msi_alloc_info_t,
) -> i32 {
    let info: *mut msi_domain_info = (*domain).host_data;

    init_irq_alloc_info(alloc, core::ptr::null_mut());

    match (*info).bus_token {
        DOMAIN_BUS_PCI_DEVICE_MSI => {
            (*alloc).type_ = X86_IRQ_ALLOC_TYPE_PCI_MSI;
            0
        }
        DOMAIN_BUS_PCI_DEVICE_MSIX => {
            (*alloc).type_ = X86_IRQ_ALLOC_TYPE_PCI_MSIX;
            0
        }
        _ => -EINVAL,
    }
}

/** x86_init_dev_msi_info - Domain info setup for MSI domains */
unsafe fn x86_init_dev_msi_info(
    _dev: *mut device,
    domain: *mut irq_domain,
    real_parent: *mut irq_domain,
    info: *mut msi_domain_info,
) -> bool {
    let pops: *const msi_parent_ops = (*real_parent).msi_parent_ops;

    match (*real_parent).bus_token {
        DOMAIN_BUS_ANY => {
            if WARN_ON_ONCE(domain != real_parent) {
                return false;
            }
            (*(*info).chip).irq_set_affinity = Some(msi_set_affinity);
            (*(*info).chip).flags |= IRQCHIP_MOVE_DEFERRED;
        }
        DOMAIN_BUS_DMAR | DOMAIN_BUS_AMDVI => {}
        _ => {
            WARN_ON_ONCE(true);
            return false;
        }
    }

    match (*info).bus_token {
        DOMAIN_BUS_PCI_DEVICE_MSI | DOMAIN_BUS_PCI_DEVICE_MSIX => {}
        _ => {
            WARN_ON_ONCE(true);
            return false;
        }
    }

    (*info).flags &= (*pops).supported_flags;
    (*info).flags |= X86_VECTOR_MSI_FLAGS_REQUIRED;
    (*(*info).ops).msi_prepare = Some(x86_msi_prepare);
    (*(*info).chip).irq_ack = Some(irq_chip_ack_parent);
    (*(*info).chip).irq_retrigger = Some(irq_chip_retrigger_hierarchy);
    (*(*info).chip).flags |= IRQCHIP_SKIP_SET_WAKE | IRQCHIP_AFFINITY_PRE_STARTUP;
    (*info).handler = Some(handle_edge_irq);
    (*info).handler_name = b"edge\0".as_ptr() as *const i8;
    true
}

static x86_vector_msi_parent_ops: msi_parent_ops = msi_parent_ops {
    supported_flags: X86_VECTOR_MSI_FLAGS_SUPPORTED,
    init_dev_msi_info: Some(x86_init_dev_msi_info),
};

unsafe fn native_create_pci_msi_domain() -> *mut irq_domain {
    if apic_is_disabled {
        return core::ptr::null_mut();
    }

    (*x86_vector_domain).flags |= IRQ_DOMAIN_FLAG_MSI_PARENT;
    (*x86_vector_domain).msi_parent_ops = &x86_vector_msi_parent_ops;
    x86_vector_domain
}

unsafe fn x86_create_pci_msi_domain() {
    x86_pci_msi_default_domain = (x86_init.irqs.create_pci_msi_domain)();
}

/* Keep around for hyperV */
unsafe fn pci_msi_prepare(
    _domain: *mut irq_domain,
    dev: *mut device,
    _nvec: i32,
    arg: *mut msi_alloc_info_t,
) -> i32 {
    init_irq_alloc_info(arg, core::ptr::null_mut());

    if (*to_pci_dev(dev)).msix_enabled {
        (*arg).type_ = X86_IRQ_ALLOC_TYPE_PCI_MSIX;
    } else {
        (*arg).type_ = X86_IRQ_ALLOC_TYPE_PCI_MSI;
    }
    0
}

// EXPORT_SYMBOL_GPL(pci_msi_prepare);

#[cfg(CONFIG_DMAR_TABLE)]
mod dmar {
    /*
     * The Intel IOMMU (ab)uses the high bits of the MSI address to contain the
     * high bits of the destination APIC ID. This can't be done in the general
     * case for MSIs as it would be targeting real memory above 4GiB not the
     * APIC.
     */
    unsafe fn dmar_msi_compose_msg(data: *mut irq_data, msg: *mut msi_msg) {
        __irq_msi_compose_msg(irqd_cfg(data), msg, true);
    }

    unsafe fn dmar_msi_write_msg(data: *mut irq_data, msg: *mut msi_msg) {
        dmar_msi_write((*data).irq, msg);
    }

    static mut dmar_msi_controller: irq_chip = irq_chip {
        name: b"DMAR-MSI\0".as_ptr() as *const i8,
        irq_unmask: Some(dmar_msi_unmask),
        irq_mask: Some(dmar_msi_mask),
        irq_ack: Some(irq_chip_ack_parent),
        irq_set_affinity: Some(msi_domain_set_affinity),
        irq_retrigger: Some(irq_chip_retrigger_hierarchy),
        irq_compose_msi_msg: Some(dmar_msi_compose_msg),
        irq_write_msi_msg: Some(dmar_msi_write_msg),
        flags: IRQCHIP_SKIP_SET_WAKE | IRQCHIP_MOVE_DEFERRED | IRQCHIP_AFFINITY_PRE_STARTUP,
        ..core::mem::zeroed()
    };

    unsafe fn dmar_msi_init(
        domain: *mut irq_domain,
        info: *mut msi_domain_info,
        virq: u32,
        _hwirq: irq_hw_number_t,
        arg: *mut msi_alloc_info_t,
    ) -> i32 {
        irq_domain_set_info(domain, virq, (*arg).devid, (*info).chip, core::ptr::null_mut(),
                            Some(handle_edge_irq), (*arg).data, b"edge\0".as_ptr() as *const i8);
        0
    }

    static mut dmar_msi_domain_ops: msi_domain_ops = msi_domain_ops {
        msi_init: Some(dmar_msi_init),
        ..core::mem::zeroed()
    };

    static mut dmar_msi_domain_info: msi_domain_info = msi_domain_info {
        ops: &mut dmar_msi_domain_ops,
        chip: &mut dmar_msi_controller,
        flags: MSI_FLAG_USE_DEF_DOM_OPS,
        ..core::mem::zeroed()
    };

    unsafe fn dmar_get_irq_domain() -> *mut irq_domain {
        static mut dmar_domain: *mut irq_domain = core::ptr::null_mut();
        static mut dmar_lock: mutex = mutex::zeroed();
        let fn_: *mut fwnode_handle;

        mutex_lock(&mut dmar_lock);
        if !dmar_domain.is_null() {
            mutex_unlock(&mut dmar_lock);
            return dmar_domain;
        }

        fn_ = irq_domain_alloc_named_fwnode(b"DMAR-MSI\0".as_ptr() as *const i8);
        if !fn_.is_null() {
            dmar_domain = msi_create_irq_domain(fn_, &mut dmar_msi_domain_info, x86_vector_domain);
            if dmar_domain.is_null() {
                irq_domain_free_fwnode(fn_);
            }
        }
        mutex_unlock(&mut dmar_lock);
        dmar_domain
    }

    unsafe fn dmar_alloc_hwirq(id: i32, node: i32, arg: *mut core::ffi::c_void) -> i32 {
        let domain = dmar_get_irq_domain();
        let mut info: irq_alloc_info = core::mem::zeroed();

        if domain.is_null() {
            return -1;
        }
        init_irq_alloc_info(&mut info, core::ptr::null_mut());
        info.type_ = X86_IRQ_ALLOC_TYPE_DMAR;
        info.devid = id;
        info.hwirq = id;
        info.data = arg;
        irq_domain_alloc_irqs(domain, 1, node, &mut info)
    }

    unsafe fn dmar_free_hwirq(irq: i32) {
        irq_domain_free_irqs(irq, 1);
    }
}

unsafe fn arch_restore_msi_irqs(dev: *mut pci_dev) -> bool {
    xen_initdom_restore_msi(dev)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
