// SPDX-License-Identifier: GPL-2.0-only
/*
 * Xen PCI - handle PCI (INTx) and MSI infrastructure calls for PV, HVM and
 * initial domain support. We also handle the DSDT _PRT callbacks for GSI's
 * used in HVM and initial domain mode (PV does not parse ACPI, so it has no
 * concept of GSIs). Under PV we hook under the pnbbios API for IRQs and
 * 0xcf8 PCI configuration read/write.
 */

// External kernel, Xen, PCI, ACPI, and architecture declarations are supplied
// by the surrounding translation unit.

unsafe fn xen_pcifront_enable_irq(dev: *mut pci_dev) -> c_int {
    let mut share: c_int = 1;
    let mut pirq: c_int;
    let mut gsi: u8 = 0;
    let rc = pci_read_config_byte(dev, PCI_INTERRUPT_LINE, &mut gsi);
    if rc != 0 {
        dev_warn(&(*dev).dev, "Xen PCI: failed to read interrupt line: %d\n", rc);
        return pcibios_err_to_errno(rc);
    }
    // In PV DomU the Xen PCI backend puts the PIRQ in the interrupt line.
    pirq = gsi as c_int;
    if (gsi as u32) < nr_legacy_irqs() { share = 0; }
    let rc = xen_bind_pirq_gsi_to_irq(gsi as u32, pirq, share, "pcifront");
    if rc < 0 {
        dev_warn(&(*dev).dev, "Xen PCI: failed to bind GSI%d (PIRQ%d) to IRQ: %d\n", gsi, pirq, rc);
        return rc;
    }
    (*dev).irq = rc;
    dev_info(&(*dev).dev, "Xen PCI mapped GSI%d to IRQ%d\n", gsi, (*dev).irq);
    0
}

#[cfg(CONFIG_ACPI)]
unsafe fn xen_register_pirq(gsi: u32, triggering: c_int, set_pirq: bool) -> c_int {
    let mut pirq: c_int = -1;
    let mut shareable: c_int = 0;
    let name: *const c_char;
    let mut map_irq: physdev_map_pirq = core::mem::zeroed();
    let mut irq = xen_irq_from_gsi(gsi);
    if irq > 0 { return irq; }
    if set_pirq { pirq = gsi as c_int; }
    map_irq.domid = DOMID_SELF;
    map_irq.type_ = MAP_PIRQ_TYPE_GSI;
    map_irq.index = gsi;
    map_irq.pirq = pirq;
    let rc = HYPERVISOR_physdev_op(PHYSDEVOP_map_pirq, &mut map_irq);
    if rc != 0 { printk(KERN_WARNING, "xen map irq failed %d\n", rc); return -1; }
    if triggering == ACPI_EDGE_SENSITIVE { shareable = 0; name = "ioapic-edge"; }
    else { shareable = 1; name = "ioapic-level"; }
    irq = xen_bind_pirq_gsi_to_irq(gsi, map_irq.pirq, shareable, name);
    if irq >= 0 { printk(KERN_DEBUG, "xen: --> pirq=%d -> irq=%d (gsi=%d)\n", map_irq.pirq, irq, gsi); }
    irq
}

#[cfg(CONFIG_ACPI)]
unsafe fn acpi_register_gsi_xen_hvm(_dev: *mut device, gsi: u32, trigger: c_int, _polarity: c_int) -> c_int {
    if !xen_hvm_domain() { return -1; }
    xen_register_pirq(gsi, trigger, false)
}

#[cfg(all(CONFIG_ACPI, CONFIG_XEN_PV_DOM0))]
unsafe fn xen_register_gsi(gsi: u32, triggering: c_int, polarity: c_int) -> c_int {
    let mut setup_gsi: physdev_setup_gsi = core::mem::zeroed();
    if !xen_pv_domain() { return -1; }
    printk(KERN_DEBUG, "xen: registering gsi %u triggering %d polarity %d\n", gsi, triggering, polarity);
    let irq = xen_register_pirq(gsi, triggering, true);
    setup_gsi.gsi = gsi;
    setup_gsi.triggering = if triggering == ACPI_EDGE_SENSITIVE { 0 } else { 1 };
    setup_gsi.polarity = if polarity == ACPI_ACTIVE_HIGH { 0 } else { 1 };
    let rc = HYPERVISOR_physdev_op(PHYSDEVOP_setup_gsi, &mut setup_gsi);
    if rc == -EEXIST { printk(KERN_INFO, "Already setup the GSI :%d\n", gsi); }
    else if rc != 0 { printk(KERN_ERR, "Failed to setup GSI :%d, err_code:%d\n", gsi, rc); }
    irq
}

#[cfg(all(CONFIG_ACPI, CONFIG_XEN_PV_DOM0))]
unsafe fn acpi_register_gsi_xen(_dev: *mut device, gsi: u32, trigger: c_int, polarity: c_int) -> c_int {
    xen_register_gsi(gsi, trigger, polarity)
}

#[cfg(CONFIG_PCI_MSI)]
struct xen_msi_ops {
    setup_msi_irqs: Option<unsafe fn(*mut pci_dev, c_int, c_int) -> c_int>,
    teardown_msi_irqs: Option<unsafe fn(*mut pci_dev)>,
}

#[cfg(CONFIG_PCI_MSI)]
static mut xen_pci_frontend: *mut xen_pci_frontend_ops = core::ptr::null_mut();
#[cfg(CONFIG_PCI_MSI)]
static mut xen_msi_ops: xen_msi_ops = xen_msi_ops { setup_msi_irqs: None, teardown_msi_irqs: None };

#[cfg(CONFIG_PCI_MSI)]
unsafe fn xen_setup_msi_irqs(dev: *mut pci_dev, nvec: c_int, ty: c_int) -> c_int {
    if ty == PCI_CAP_ID_MSI && nvec > 1 { return 1; }
    let v = kzalloc_objs::<c_int>(core::cmp::max(1, nvec));
    if v.is_null() { return -ENOMEM; }
    let mut ret = if ty == PCI_CAP_ID_MSIX { xen_pci_frontend_enable_msix(dev, v, nvec) } else { xen_pci_frontend_enable_msi(dev, v) };
    if ret != 0 { kfree(v); return ret; }
    let mut i = 0;
    msi_for_each_desc!(msidesc, &mut (*dev).dev, MSI_DESC_NOTASSOCIATED, {
        let irq = xen_bind_pirq_msi_to_irq(dev, msidesc, *v.add(i), if ty == PCI_CAP_ID_MSI { nvec } else { 1 }, if ty == PCI_CAP_ID_MSIX { "pcifront-msi-x" } else { "pcifront-msi" }, DOMID_SELF);
        if irq < 0 { ret = irq; }
        i += 1;
    });
    kfree(v);
    if ret == 0 { ret = msi_device_populate_sysfs(&mut (*dev).dev); }
    ret
}

#[cfg(CONFIG_PCI_MSI)]
unsafe fn xen_hvm_setup_msi_irqs(dev: *mut pci_dev, nvec: c_int, ty: c_int) -> c_int {
    if ty == PCI_CAP_ID_MSI && nvec > 1 { return 1; }
    let mut result = 0;
    msi_for_each_desc!(msidesc, &mut (*dev).dev, MSI_DESC_NOTASSOCIATED, {
        let pirq = xen_allocate_pirq_msi(dev, msidesc);
        if pirq < 0 { result = -ENODEV; }
        else {
            let mut msg: msi_msg = core::mem::zeroed();
            xen_msi_compose_msg(dev, pirq as u32, &mut msg);
            __pci_write_msi_msg(msidesc, &msg);
            result = xen_bind_pirq_msi_to_irq(dev, msidesc, pirq, if ty == PCI_CAP_ID_MSI { nvec } else { 1 }, if ty == PCI_CAP_ID_MSIX { "msi-x" } else { "msi" }, DOMID_SELF);
        }
    });
    if result < 0 { dev_err(&(*dev).dev, "Failed to create MSI! ret=%d!\n", result); return result; }
    msi_device_populate_sysfs(&mut (*dev).dev)
}

#[cfg(all(CONFIG_PCI_MSI, CONFIG_XEN_PV_DOM0))]
unsafe fn xen_initdom_setup_msi_irqs(dev: *mut pci_dev, nvec: c_int, ty: c_int) -> c_int {
    let mut ret = 0;
    msi_for_each_desc!(msidesc, &mut (*dev).dev, MSI_DESC_NOTASSOCIATED, {
        let domid = xen_find_device_domain_owner(dev);
        let domid = if domid < 0 { DOMID_SELF } else { domid as domid_t };
        let mut map_irq: physdev_map_pirq = core::mem::zeroed();
        map_irq.domid = domid; map_irq.type_ = MAP_PIRQ_TYPE_MSI_SEG; map_irq.index = -1; map_irq.pirq = -1;
        map_irq.bus = (*(*dev).bus).number | (pci_domain_nr((*dev).bus) << 16); map_irq.devfn = (*dev).devfn;
        if ty == PCI_CAP_ID_MSI && nvec > 1 { map_irq.type_ = MAP_PIRQ_TYPE_MULTI_MSI; map_irq.entry_nr = nvec; }
        else if ty == PCI_CAP_ID_MSIX { map_irq.entry_nr = (*msidesc).msi_index; }
        ret = HYPERVISOR_physdev_op(PHYSDEVOP_map_pirq, &mut map_irq);
        if ret == 0 { ret = xen_bind_pirq_msi_to_irq(dev, msidesc, map_irq.pirq, if ty == PCI_CAP_ID_MSI { nvec } else { 1 }, if ty == PCI_CAP_ID_MSIX { "msi-x" } else { "msi" }, domid); }
    });
    if ret == 0 { ret = msi_device_populate_sysfs(&mut (*dev).dev); }
    ret
}

#[cfg(all(CONFIG_PCI_MSI, CONFIG_XEN_PV_DOM0))]
unsafe fn xen_initdom_restore_msi(dev: *mut pci_dev) -> bool {
    if !xen_initial_domain() { return true; }
    let mut restore: physdev_pci_device = core::mem::zeroed();
    restore.seg = pci_domain_nr((*dev).bus); restore.bus = (*(*dev).bus).number; restore.devfn = (*dev).devfn;
    let ret = HYPERVISOR_physdev_op(PHYSDEVOP_restore_msi_ext, &mut restore);
    WARN(ret != 0 && ret != -ENOSYS, "restore_msi_ext -> %d\n", ret);
    false
}

#[cfg(CONFIG_PCI_MSI)]
unsafe fn xen_msi_domain_alloc_irqs(_domain: *mut irq_domain, dev: *mut device, nvec: c_int) -> c_int {
    if !dev_is_pci(dev) { return -EINVAL; }
    let pdev = to_pci_dev(dev);
    let ty = if (*pdev).msix_enabled { PCI_CAP_ID_MSIX } else { PCI_CAP_ID_MSI };
    (xen_msi_ops.setup_msi_irqs.unwrap())(pdev, nvec, ty)
}

#[cfg(CONFIG_PCI_MSI)]
unsafe fn xen_msi_domain_free_irqs(_domain: *mut irq_domain, dev: *mut device) {
    if !dev_is_pci(dev) { return; }
    (xen_msi_ops.teardown_msi_irqs.unwrap())(to_pci_dev(dev));
}

#[cfg(CONFIG_PCI_MSI)]
unsafe fn xen_create_pci_msi_domain() -> *mut irq_domain {
    let fn_ = irq_domain_alloc_named_fwnode("XEN-MSI");
    let d = if !fn_.is_null() { msi_create_irq_domain(fn_, &xen_pci_msi_domain_info, core::ptr::null_mut()) } else { core::ptr::null_mut() };
    BUG_ON(d.is_null());
    d
}

#[cfg(CONFIG_PCI_MSI)]
unsafe fn xen_setup_pci_msi() {
    if xen_pv_domain() {
        xen_msi_ops.setup_msi_irqs = if xen_initial_domain() { Some(xen_initdom_setup_msi_irqs) } else { Some(xen_setup_msi_irqs) };
        xen_msi_ops.teardown_msi_irqs = Some(xen_pv_teardown_msi_irqs);
    } else if xen_hvm_domain() {
        xen_msi_ops.setup_msi_irqs = Some(xen_hvm_setup_msi_irqs);
        xen_msi_ops.teardown_msi_irqs = Some(xen_teardown_msi_irqs);
    } else { WARN_ON_ONCE(true); return; }
    x86_init.irqs.create_pci_msi_domain = Some(xen_create_pci_msi_domain);
}

#[cfg(CONFIG_PCI_MSI)]
unsafe fn xen_msi_compose_msg(_pdev: *mut pci_dev, pirq: u32, msg: *mut msi_msg) {
    core::ptr::write_bytes(msg, 0, 1);
    (*msg).address_hi = X86_MSI_BASE_ADDRESS_HIGH;
    (*msg).arch_addr_hi.destid_8_31 = pirq >> 8;
    (*msg).arch_addr_lo.destid_0_7 = pirq & 0xff;
    (*msg).arch_addr_lo.base_address = X86_MSI_BASE_ADDRESS_LOW;
    (*msg).arch_data.delivery_mode = APIC_DELIVERY_MODE_EXTINT;
}

#[cfg(CONFIG_PCI_MSI)]
unsafe fn xen_teardown_msi_irqs(dev: *mut pci_dev) {
    let mut msidesc: *mut msi_desc = core::ptr::null_mut();
    msi_for_each_desc!(msidesc, &mut (*dev).dev, MSI_DESC_ASSOCIATED, {
        for i in 0..(*msidesc).nvec_used { xen_destroy_irq((*msidesc).irq + i as c_int); }
        (*msidesc).irq = 0;
    });
    msi_device_destroy_sysfs(&mut (*dev).dev);
}

#[cfg(CONFIG_PCI_MSI)]
unsafe fn xen_pv_teardown_msi_irqs(dev: *mut pci_dev) {
    if (*dev).msix_enabled { xen_pci_frontend_disable_msix(dev); }
    else { xen_pci_frontend_disable_msi(dev); }
    xen_teardown_msi_irqs(dev);
}

#[cfg(not(CONFIG_PCI_MSI))]
unsafe fn xen_setup_pci_msi() {}

unsafe fn pci_xen_init() -> c_int {
    if !xen_pv_domain() || xen_initial_domain() { return -ENODEV; }
    printk(KERN_INFO, "PCI: setting up Xen PCI frontend stub\n");
    pcibios_set_cache_line_size();
    pcibios_enable_irq = Some(xen_pcifront_enable_irq);
    pcibios_disable_irq = None;
    acpi_noirq_set();
    xen_setup_pci_msi();
    0
}

#[cfg(CONFIG_PCI_MSI)]
unsafe fn xen_hvm_msi_init() {
    if !apic_is_disabled {
        let eax = cpuid_eax(xen_cpuid_base() + 4);
        if ((eax & XEN_HVM_CPUID_X2APIC_VIRT) != 0 && x2apic_mode) ||
           ((eax & XEN_HVM_CPUID_APIC_ACCESS_VIRT) != 0 && boot_cpu_has(X86_FEATURE_APIC)) { return; }
    }
    xen_setup_pci_msi();
}

unsafe fn pci_xen_hvm_init() -> c_int {
    if !xen_have_vector_callback || !xen_feature(XENFEAT_hvm_pirqs) { return 0; }
    #[cfg(CONFIG_ACPI)] { __acpi_register_gsi = Some(acpi_register_gsi_xen_hvm); __acpi_unregister_gsi = None; }
    #[cfg(CONFIG_PCI_MSI)] { x86_platform.apic_post_init = Some(xen_hvm_msi_init); }
    0
}

#[cfg(CONFIG_XEN_PV_DOM0)]
unsafe fn pci_xen_initial_domain() -> c_int {
    xen_setup_pci_msi();
    __acpi_register_gsi = Some(acpi_register_gsi_xen);
    __acpi_unregister_gsi = None;
    for irq in 0..NR_IRQS_LEGACY {
        let mut trigger = 0; let mut polarity = 0;
        if acpi_get_override_irq(irq, &mut trigger, &mut polarity) == -1 { continue; }
        xen_register_pirq(irq as u32, if trigger != 0 { ACPI_LEVEL_SENSITIVE } else { ACPI_EDGE_SENSITIVE }, true);
    }
    if nr_ioapics == 0 { for irq in 0..nr_legacy_irqs() { xen_bind_pirq_gsi_to_irq(irq, irq as c_int, 0, "xt-pic"); } }
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
