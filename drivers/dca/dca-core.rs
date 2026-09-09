// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright(c) 2007 - 2009 Intel Corporation. All rights reserved.
 */

/* This driver supports an interface for DCA clients and providers to meet. */

// Kernel headers and build-time module declarations are supplied by the surrounding crate.

const DCA_VERSION: &str = "1.12.1";

static mut DCA_LOCK: RawSpinLock = RawSpinLock::new();
static mut DCA_DOMAINS: ListHead = ListHead::new();
static mut DCA_PROVIDER_CHAIN: BlockingNotifierHead = BlockingNotifierHead::new();
static mut DCA_PROVIDERS_BLOCKED: i32 = 0;

unsafe fn dca_pci_rc_from_dev(dev: *mut Device) -> *mut PciBus {
    let pdev = to_pci_dev(dev);
    let mut bus = (*pdev).bus;
    while !(*bus).parent.is_null() {
        bus = (*bus).parent;
    }
    bus
}

unsafe fn dca_allocate_domain(rc: *mut PciBus) -> *mut DcaDomain {
    let domain = kzalloc_obj::<DcaDomain>(GFP_NOWAIT);
    if domain.is_null() { return core::ptr::null_mut(); }
    INIT_LIST_HEAD(&mut (*domain).dca_providers);
    (*domain).pci_rc = rc;
    domain
}

unsafe fn dca_free_domain(domain: *mut DcaDomain) {
    list_del(&mut (*domain).node);
    kfree(domain as *mut core::ffi::c_void);
}

unsafe fn dca_provider_ioat_ver_3_0(dev: *mut Device) -> bool {
    let pdev = to_pci_dev(dev);
    (*pdev).vendor == PCI_VENDOR_ID_INTEL &&
        ((*pdev).device == PCI_DEVICE_ID_INTEL_IOAT_TBG0 ||
         (*pdev).device == PCI_DEVICE_ID_INTEL_IOAT_TBG1 ||
         (*pdev).device == PCI_DEVICE_ID_INTEL_IOAT_TBG2 ||
         (*pdev).device == PCI_DEVICE_ID_INTEL_IOAT_TBG3 ||
         (*pdev).device == PCI_DEVICE_ID_INTEL_IOAT_TBG4 ||
         (*pdev).device == PCI_DEVICE_ID_INTEL_IOAT_TBG5 ||
         (*pdev).device == PCI_DEVICE_ID_INTEL_IOAT_TBG6 ||
         (*pdev).device == PCI_DEVICE_ID_INTEL_IOAT_TBG7)
}

unsafe fn unregister_dca_providers() {
    let mut unregistered_providers = ListHead::new();
    blocking_notifier_call_chain(&mut DCA_PROVIDER_CHAIN, DCA_PROVIDER_REMOVE, core::ptr::null_mut());
    INIT_LIST_HEAD(&mut unregistered_providers);
    let flags = raw_spin_lock_irqsave(&mut DCA_LOCK);
    if list_empty(&DCA_DOMAINS) { raw_spin_unlock_irqrestore(&mut DCA_LOCK, flags); return; }
    let domain = list_first_entry::<DcaDomain>(&mut DCA_DOMAINS);
    let mut dca: *mut DcaProvider;
    let mut next: *mut DcaProvider;
    list_for_each_entry_safe(&mut dca, &mut next, &mut (*domain).dca_providers) {
        list_move(&mut (*dca).node, &mut unregistered_providers);
    }
    dca_free_domain(domain);
    raw_spin_unlock_irqrestore(&mut DCA_LOCK, flags);
    list_for_each_entry_safe(&mut dca, &mut next, &mut unregistered_providers) {
        dca_sysfs_remove_provider(dca);
        list_del(&mut (*dca).node);
    }
}

unsafe fn dca_find_domain(rc: *mut PciBus) -> *mut DcaDomain {
    let mut domain: *mut DcaDomain = core::ptr::null_mut();
    list_for_each_entry(&mut domain, &DCA_DOMAINS) {
        if (*domain).pci_rc == rc { return domain; }
    }
    core::ptr::null_mut()
}

unsafe fn dca_get_domain(dev: *mut Device) -> *mut DcaDomain {
    let rc = dca_pci_rc_from_dev(dev);
    let domain = dca_find_domain(rc);
    if domain.is_null() && dca_provider_ioat_ver_3_0(dev) && !list_empty(&DCA_DOMAINS) {
        DCA_PROVIDERS_BLOCKED = 1;
    }
    domain
}

unsafe fn dca_find_provider_by_dev(dev: *mut Device) -> *mut DcaProvider {
    let domain = if !dev.is_null() {
        let d = dca_find_domain(dca_pci_rc_from_dev(dev));
        if d.is_null() { return core::ptr::null_mut(); } d
    } else {
        if list_empty(&DCA_DOMAINS) { return core::ptr::null_mut(); }
        list_first_entry::<DcaDomain>(&mut DCA_DOMAINS)
    };
    let mut dca: *mut DcaProvider = core::ptr::null_mut();
    list_for_each_entry(&mut dca, &(*domain).dca_providers) {
        if dev.is_null() || ((*(*dca).ops).dev_managed)(dca, dev) { return dca; }
    }
    core::ptr::null_mut()
}

pub unsafe fn dca_add_requester(dev: *mut Device) -> i32 {
    if dev.is_null() { return -EFAULT; }
    let flags = raw_spin_lock_irqsave(&mut DCA_LOCK);
    if !dca_find_provider_by_dev(dev).is_null() { raw_spin_unlock_irqrestore(&mut DCA_LOCK, flags); return -EEXIST; }
    let domain = dca_find_domain(dca_pci_rc_from_dev(dev));
    if domain.is_null() { raw_spin_unlock_irqrestore(&mut DCA_LOCK, flags); return -ENODEV; }
    let mut slot = -ENODEV;
    let mut dca: *mut DcaProvider = core::ptr::null_mut();
    list_for_each_entry(&mut dca, &(*domain).dca_providers) {
        slot = ((*(*dca).ops).add_requester)(dca, dev);
        if slot >= 0 { break; }
    }
    raw_spin_unlock_irqrestore(&mut DCA_LOCK, flags);
    if slot < 0 { return slot; }
    let err = dca_sysfs_add_req(dca, dev, slot);
    if err != 0 {
        let flags = raw_spin_lock_irqsave(&mut DCA_LOCK);
        if dca == dca_find_provider_by_dev(dev) { ((*(*dca).ops).remove_requester)(dca, dev); }
        raw_spin_unlock_irqrestore(&mut DCA_LOCK, flags);
        return err;
    }
    0
}

pub unsafe fn dca_remove_requester(dev: *mut Device) -> i32 {
    if dev.is_null() { return -EFAULT; }
    let flags = raw_spin_lock_irqsave(&mut DCA_LOCK);
    let dca = dca_find_provider_by_dev(dev);
    if dca.is_null() { raw_spin_unlock_irqrestore(&mut DCA_LOCK, flags); return -ENODEV; }
    let slot = ((*(*dca).ops).remove_requester)(dca, dev);
    raw_spin_unlock_irqrestore(&mut DCA_LOCK, flags);
    if slot < 0 { return slot; }
    dca_sysfs_remove_req(dca, slot);
    0
}

unsafe fn dca_common_get_tag(dev: *mut Device, cpu: i32) -> u8 {
    let flags = raw_spin_lock_irqsave(&mut DCA_LOCK);
    let dca = dca_find_provider_by_dev(dev);
    if dca.is_null() { raw_spin_unlock_irqrestore(&mut DCA_LOCK, flags); return (-ENODEV) as u8; }
    let tag = ((*(*dca).ops).get_tag)(dca, dev, cpu);
    raw_spin_unlock_irqrestore(&mut DCA_LOCK, flags);
    tag
}

pub unsafe fn dca3_get_tag(dev: *mut Device, cpu: i32) -> u8 {
    if dev.is_null() { return (-EFAULT) as u8; }
    dca_common_get_tag(dev, cpu)
}

pub unsafe fn dca_get_tag(cpu: i32) -> u8 { dca_common_get_tag(core::ptr::null_mut(), cpu) }

pub unsafe fn alloc_dca_provider(ops: *const DcaOps, priv_size: i32) -> *mut DcaProvider {
    let dca = kzalloc((core::mem::size_of::<DcaProvider>() as i32 + priv_size) as usize, GFP_KERNEL);
    if dca.is_null() { return core::ptr::null_mut(); }
    (*dca).ops = ops; dca
}

pub unsafe fn free_dca_provider(dca: *mut DcaProvider) { kfree(dca as *mut core::ffi::c_void); }

pub unsafe fn register_dca_provider(dca: *mut DcaProvider, dev: *mut Device) -> i32 {
    let flags = raw_spin_lock_irqsave(&mut DCA_LOCK);
    if DCA_PROVIDERS_BLOCKED != 0 { raw_spin_unlock_irqrestore(&mut DCA_LOCK, flags); return -ENODEV; }
    raw_spin_unlock_irqrestore(&mut DCA_LOCK, flags);
    let err = dca_sysfs_add_provider(dca, dev); if err != 0 { return err; }
    let mut newdomain: *mut DcaDomain = core::ptr::null_mut();
    let flags = raw_spin_lock_irqsave(&mut DCA_LOCK);
    let mut domain = dca_get_domain(dev);
    if domain.is_null() {
        if DCA_PROVIDERS_BLOCKED != 0 { raw_spin_unlock_irqrestore(&mut DCA_LOCK, flags); dca_sysfs_remove_provider(dca); unregister_dca_providers(); return -ENODEV; }
        raw_spin_unlock_irqrestore(&mut DCA_LOCK, flags);
        newdomain = dca_allocate_domain(dca_pci_rc_from_dev(dev)); if newdomain.is_null() { return -ENODEV; }
        let flags = raw_spin_lock_irqsave(&mut DCA_LOCK);
        domain = dca_get_domain(dev);
        if domain.is_null() { domain = newdomain; newdomain = core::ptr::null_mut(); list_add(&mut (*domain).node, &mut DCA_DOMAINS); }
        raw_spin_unlock_irqrestore(&mut DCA_LOCK, flags);
    }
    list_add(&mut (*dca).node, &mut (*domain).dca_providers);
    raw_spin_unlock_irqrestore(&mut DCA_LOCK, flags);
    blocking_notifier_call_chain(&mut DCA_PROVIDER_CHAIN, DCA_PROVIDER_ADD, core::ptr::null_mut());
    if !newdomain.is_null() { kfree(newdomain as *mut core::ffi::c_void); }
    0
}

pub unsafe fn unregister_dca_provider(dca: *mut DcaProvider, dev: *mut Device) {
    blocking_notifier_call_chain(&mut DCA_PROVIDER_CHAIN, DCA_PROVIDER_REMOVE, core::ptr::null_mut());
    let flags = raw_spin_lock_irqsave(&mut DCA_LOCK);
    if list_empty(&DCA_DOMAINS) { raw_spin_unlock_irqrestore(&mut DCA_LOCK, flags); return; }
    list_del(&mut (*dca).node);
    let domain = dca_find_domain(dca_pci_rc_from_dev(dev));
    if list_empty(&(*domain).dca_providers) { dca_free_domain(domain); }
    raw_spin_unlock_irqrestore(&mut DCA_LOCK, flags);
    dca_sysfs_remove_provider(dca);
}

pub unsafe fn dca_register_notify(nb: *mut NotifierBlock) { blocking_notifier_chain_register(&mut DCA_PROVIDER_CHAIN, nb); }
pub unsafe fn dca_unregister_notify(nb: *mut NotifierBlock) { blocking_notifier_chain_unregister(&mut DCA_PROVIDER_CHAIN, nb); }

unsafe fn dca_init() -> i32 {
    pr_info("dca service started, version %s\n", DCA_VERSION);
    dca_sysfs_init()
}
unsafe fn dca_exit() { dca_sysfs_exit(); }

// Kernel initcall/module-exit registration corresponding to arch_initcall(dca_init) and module_exit(dca_exit).

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
