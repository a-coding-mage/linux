// SPDX-License-Identifier: GPL-2.0
/*
 * Irqdomain for Linux to run as the root partition on Microsoft Hypervisor.
 *
 * Authors:
 *  Sunil Muthuswamy <sunilmut@microsoft.com>
 *  Wei Liu <wei.liu@kernel.org>
 */

// Dependencies supplied by the surrounding kernel translation.

unsafe fn hv_map_interrupt(
    hv_devid: hv_device_id,
    level: bool,
    cpu: i32,
    vector: i32,
    ret_entry: *mut hv_interrupt_entry,
) -> i32 {
    let input: *mut hv_input_map_device_interrupt;
    let output: *mut hv_output_map_device_interrupt;
    let intr_desc: *mut hv_device_interrupt_descriptor;
    let mut flags: usize = 0;
    let mut status: u64;
    let nr_bank: i32;
    let var_size: i32;

    local_irq_save(&mut flags);
    input = *this_cpu_ptr(hyperv_pcpu_input_arg);
    output = *this_cpu_ptr(hyperv_pcpu_output_arg);

    intr_desc = &mut (*input).interrupt_descriptor;
    core::ptr::write_bytes(input as *mut u8, 0, core::mem::size_of::<hv_input_map_device_interrupt>());
    (*input).partition_id = hv_current_partition_id;
    (*input).device_id = hv_devid.as_uint64;
    (*intr_desc).interrupt_type = HV_X64_INTERRUPT_TYPE_FIXED;
    (*intr_desc).vector_count = 1;
    (*intr_desc).target.vector = vector;
    (*intr_desc).trigger_mode = if level { HV_INTERRUPT_TRIGGER_MODE_LEVEL } else { HV_INTERRUPT_TRIGGER_MODE_EDGE };
    (*intr_desc).target.vp_set.valid_bank_mask = 0;
    (*intr_desc).target.vp_set.format = HV_GENERIC_SET_SPARSE_4K;
    nr_bank = cpumask_to_vpset(&mut (*intr_desc).target.vp_set, cpumask_of(cpu));
    if nr_bank < 0 {
        local_irq_restore(flags);
        pr_err("{}: unable to generate VP set\n", "hv_map_interrupt");
        return -EINVAL;
    }
    (*intr_desc).target.flags = HV_DEVICE_INTERRUPT_TARGET_PROCESSOR_SET;
    var_size = nr_bank + 1;
    status = hv_do_rep_hypercall(HVCALL_MAP_DEVICE_INTERRUPT, 0, var_size, input, output);
    *ret_entry = (*output).interrupt_entry;
    local_irq_restore(flags);
    if !hv_result_success(status) { hv_status_err(status, "\n"); }
    hv_result_to_errno(status)
}

unsafe fn hv_unmap_interrupt(id: u64, irq_entry: *const hv_interrupt_entry) -> i32 {
    let input: *mut hv_input_unmap_device_interrupt;
    let mut flags: usize = 0;
    let status: u64;
    local_irq_save(&mut flags);
    input = *this_cpu_ptr(hyperv_pcpu_input_arg);
    core::ptr::write_bytes(input as *mut u8, 0, core::mem::size_of::<hv_input_unmap_device_interrupt>());
    (*input).partition_id = hv_current_partition_id;
    (*input).device_id = id;
    (*input).interrupt_entry = *irq_entry;
    status = hv_do_hypercall(HVCALL_UNMAP_DEVICE_INTERRUPT, input, core::ptr::null_mut());
    local_irq_restore(flags);
    if !hv_result_success(status) { hv_status_err(status, "\n"); }
    hv_result_to_errno(status)
}

#[cfg(CONFIG_PCI_MSI)]
struct rid_data { bridge: *mut pci_dev, rid: u32 }

#[cfg(CONFIG_PCI_MSI)]
unsafe fn get_rid_cb(pdev: *mut pci_dev, alias: u16, data: *mut core::ffi::c_void) -> i32 {
    let rd = &mut *(data as *mut rid_data);
    let bus = PCI_BUS_NUM(rd.rid);
    if (*(*pdev).bus).number != bus || PCI_BUS_NUM(alias) != bus {
        rd.bridge = pdev;
        rd.rid = alias as u32;
    }
    0
}

#[cfg(CONFIG_PCI_MSI)]
unsafe fn hv_build_devid_type_pci(pdev: *mut pci_dev) -> hv_device_id {
    let mut hv_devid: hv_device_id = core::mem::zeroed();
    let mut data = rid_data { bridge: core::ptr::null_mut(), rid: PCI_DEVID((*(*pdev).bus).number, (*pdev).devfn) };
    pci_for_each_dma_alias(pdev, get_rid_cb, &mut data as *mut _ as *mut core::ffi::c_void);
    hv_devid.as_uint64 = 0;
    hv_devid.device_type = HV_DEVICE_TYPE_PCI;
    hv_devid.pci.segment = pci_domain_nr((*pdev).bus);
    hv_devid.pci.bdf.bus = PCI_BUS_NUM(data.rid);
    hv_devid.pci.bdf.device = PCI_SLOT(data.rid);
    hv_devid.pci.bdf.function = PCI_FUNC(data.rid);
    hv_devid.pci.source_shadow = HV_SOURCE_SHADOW_NONE;
    if data.bridge.is_null() { return hv_devid; }
    let pos = pci_find_capability(data.bridge, PCI_CAP_ID_PCIX);
    if pos != 0 {
        let mut status: u16 = 0;
        pci_read_config_word(data.bridge, pos + PCI_X_BRIDGE_SSTATUS, &mut status);
        if status & PCI_X_SSTATUS_FREQ != 0 {
            hv_devid.pci.source_shadow = HV_SOURCE_SHADOW_BRIDGE_BUS_RANGE;
            let mut sec_bus = 0u8; let mut sub_bus = 0u8;
            pci_read_config_byte(data.bridge, PCI_SECONDARY_BUS, &mut sec_bus);
            hv_devid.pci.shadow_bus_range.secondary_bus = sec_bus;
            pci_read_config_byte(data.bridge, PCI_SUBORDINATE_BUS, &mut sub_bus);
            hv_devid.pci.shadow_bus_range.subordinate_bus = sub_bus;
        }
    }
    hv_devid
}

#[cfg(CONFIG_PCI_MSI)]
pub unsafe fn hv_map_msi_interrupt(data: *mut irq_data, out_entry: *mut hv_interrupt_entry) -> i32 {
    let cfg = irqd_cfg(data);
    let mut dummy: hv_interrupt_entry = core::mem::zeroed();
    let msidesc = irq_data_get_msi_desc(data);
    let pdev = msi_desc_to_pci_dev(msidesc);
    let hv_devid = hv_build_devid_type_pci(pdev);
    let cpu = cpumask_first(irq_data_get_effective_affinity_mask(data));
    hv_map_interrupt(hv_devid, false, cpu, (*cfg).vector, if !out_entry.is_null() { out_entry } else { &mut dummy })
}

#[cfg(CONFIG_PCI_MSI)]
unsafe fn entry_to_msi_msg(entry: *const hv_interrupt_entry, msg: *mut msi_msg) {
    (*msg).address_hi = 0;
    (*msg).address_lo = (*entry).msi_entry.address.as_uint32;
    (*msg).data = (*entry).msi_entry.data.as_uint32;
}

#[cfg(CONFIG_PCI_MSI)]
static mut hv_pci_msi_controller: irq_chip = irq_chip {
    name: "HV-PCI-MSI",
    irq_ack: Some(irq_chip_ack_parent),
    irq_compose_msi_msg: Some(hv_irq_compose_msi_msg),
    irq_set_affinity: Some(irq_chip_set_affinity_parent),
};

#[cfg(CONFIG_PCI_MSI)]
const HV_MSI_FLAGS_SUPPORTED: u32 = MSI_GENERIC_FLAGS_MASK | MSI_FLAG_PCI_MSIX;
#[cfg(CONFIG_PCI_MSI)]
const HV_MSI_FLAGS_REQUIRED: u32 = MSI_FLAG_USE_DEF_DOM_OPS | MSI_FLAG_USE_DEF_CHIP_OPS;

#[cfg(CONFIG_PCI_MSI)]
static mut hv_msi_parent_ops: msi_parent_ops = msi_parent_ops {
    supported_flags: HV_MSI_FLAGS_SUPPORTED,
    required_flags: HV_MSI_FLAGS_REQUIRED,
    bus_select_token: DOMAIN_BUS_NEXUS,
    bus_select_mask: MATCH_PCI_MSI,
    chip_flags: MSI_CHIP_FLAG_SET_ACK,
    prefix: "HV-",
    init_dev_msi_info: Some(hv_init_dev_msi_info),
};

// The original source supplies hv_msi_domain_ops with select=msi_lib_irq_domain_select,
// alloc=hv_msi_domain_alloc, and free=hv_msi_domain_free.

#[cfg(CONFIG_PCI_MSI)]
unsafe fn hv_unmap_msi_interrupt(pdev: *mut pci_dev, irq_entry: *const hv_interrupt_entry) -> i32 {
    let hv_devid = hv_build_devid_type_pci(pdev);
    hv_unmap_interrupt(hv_devid.as_uint64, irq_entry)
}

#[cfg(CONFIG_PCI_MSI)]
unsafe fn hv_irq_compose_msi_msg(data: *mut irq_data, msg: *mut msi_msg) {
    let cfg = irqd_cfg(data);
    let msidesc = irq_data_get_msi_desc(data);
    let pdev = msi_desc_to_pci_dev(msidesc);
    if cfg.is_null() { pr_debug!("{}: cfg is NULL", "hv_irq_compose_msi_msg"); return; }
    if !(*data).chip_data.is_null() {
        let stored_entry = (*data).chip_data as *mut hv_interrupt_entry;
        (*data).chip_data = core::ptr::null_mut();
        let ret = hv_unmap_msi_interrupt(pdev, stored_entry);
        kfree(stored_entry as *mut core::ffi::c_void);
        if ret != 0 { return; }
    }
    let stored_entry = kzalloc_obj::<hv_interrupt_entry>(GFP_ATOMIC);
    if stored_entry.is_null() { return; }
    let ret = hv_map_msi_interrupt(data, stored_entry);
    if ret != 0 { kfree(stored_entry as *mut core::ffi::c_void); return; }
    (*data).chip_data = stored_entry as *mut core::ffi::c_void;
    entry_to_msi_msg(stored_entry, msg);
}

#[cfg(CONFIG_PCI_MSI)]
unsafe fn hv_teardown_msi_irq(pdev: *mut pci_dev, irqd: *mut irq_data) {
    if (*irqd).chip_data.is_null() { pr_debug!("{}: no chip data\n!", "hv_teardown_msi_irq"); return; }
    let irq_entry = *((*irqd).chip_data as *mut hv_interrupt_entry);
    let mut msg: msi_msg = core::mem::zeroed();
    entry_to_msi_msg(&irq_entry, &mut msg);
    kfree((*irqd).chip_data);
    (*irqd).chip_data = core::ptr::null_mut();
    let _ = hv_unmap_msi_interrupt(pdev, &irq_entry);
}

#[cfg(CONFIG_PCI_MSI)]
unsafe fn hv_init_dev_msi_info(dev: *mut device, domain: *mut irq_domain, real_parent: *mut irq_domain, info: *mut msi_domain_info) -> bool {
    let chip = (*info).chip;
    if !msi_lib_init_dev_msi_info(dev, domain, real_parent, info) { return false; }
    (*chip).flags |= IRQCHIP_SKIP_SET_WAKE | IRQCHIP_MOVE_DEFERRED;
    (*info).ops.msi_prepare = pci_msi_prepare;
    true
}

#[cfg(CONFIG_PCI_MSI)]
unsafe fn hv_msi_domain_alloc(d: *mut irq_domain, virq: u32, nr_irqs: u32, arg: *mut core::ffi::c_void) -> i32 {
    let ret = irq_domain_alloc_irqs_parent(d, virq, nr_irqs, arg);
    if ret != 0 { return ret; }
    for i in 0..nr_irqs { irq_domain_set_info(d, virq + i, 0, &hv_pci_msi_controller, core::ptr::null_mut(), handle_edge_irq, core::ptr::null_mut(), "edge"); }
    0
}

#[cfg(CONFIG_PCI_MSI)]
unsafe fn hv_msi_domain_free(d: *mut irq_domain, virq: u32, nr_irqs: u32) {
    for _ in 0..nr_irqs {
        let irqd = irq_domain_get_irq_data(d, virq);
        let desc = irq_data_get_msi_desc(irqd);
        if desc.is_null() || (*desc).irq == 0 || (WARN_ON_ONCE(!dev_is_pci((*desc).dev))) { continue; }
        hv_teardown_msi_irq(to_pci_dev((*desc).dev), irqd);
    }
    irq_domain_free_irqs_top(d, virq, nr_irqs);
}

#[cfg(CONFIG_PCI_MSI)]
unsafe fn hv_create_pci_msi_domain() -> *mut irq_domain {
    let fwnode = irq_domain_alloc_named_fwnode("HV-PCI-MSI");
    let mut d = core::ptr::null_mut();
    if !fwnode.is_null() { d = msi_create_parent_irq_domain(fwnode, &hv_msi_parent_ops); }
    BUG_ON(d.is_null());
    d
}

pub unsafe fn hv_unmap_ioapic_interrupt(ioapic_id: i32, entry: *const hv_interrupt_entry) -> i32 {
    let mut hv_devid: hv_device_id = core::mem::zeroed();
    hv_devid.as_uint64 = 0;
    hv_devid.device_type = HV_DEVICE_TYPE_IOAPIC;
    hv_devid.ioapic.ioapic_id = ioapic_id as u8;
    hv_unmap_interrupt(hv_devid.as_uint64, entry)
}

pub unsafe fn hv_map_ioapic_interrupt(ioapic_id: i32, level: bool, cpu: i32, vector: i32, entry: *mut hv_interrupt_entry) -> i32 {
    let mut hv_devid: hv_device_id = core::mem::zeroed();
    hv_devid.as_uint64 = 0;
    hv_devid.device_type = HV_DEVICE_TYPE_IOAPIC;
    hv_devid.ioapic.ioapic_id = ioapic_id as u8;
    hv_map_interrupt(hv_devid, level, cpu, vector, entry)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
