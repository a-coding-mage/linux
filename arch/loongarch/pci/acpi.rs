// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (C) 2020-2022 Loongson Technology Corporation Limited
 */

// External Linux kernel declarations supplied by the surrounding repository.

#[repr(C)]
pub struct pci_root_info {
    pub common: acpi_pci_root_info,
    pub cfg: *mut pci_config_window,
}

pub unsafe fn pcibios_add_bus(bus: *mut pci_bus) {
    acpi_pci_add_bus(bus);
}

pub unsafe fn pcibios_root_bridge_prepare(bridge: *mut pci_host_bridge) -> i32 {
    let mut adev: *mut acpi_device = core::ptr::null_mut();
    let bus_dev: *mut device = &mut (*(*bridge).bus).dev;
    let cfg: *mut pci_config_window = (*(*bridge).bus).sysdata as *mut pci_config_window;

    if !acpi_disabled {
        adev = to_acpi_device((*cfg).parent);
    }

    ACPI_COMPANION_SET(&mut (*bridge).dev, adev);
    set_dev_node(bus_dev, pa_to_nid((*cfg).res.start));

    0
}

pub unsafe fn acpi_pci_bus_find_domain_nr(bus: *mut pci_bus) -> i32 {
    let cfg: *mut pci_config_window = (*bus).sysdata as *mut pci_config_window;
    let adev = to_acpi_device((*cfg).parent);
    let root = acpi_driver_data(adev);

    (*root).segment as i32
}

unsafe fn acpi_release_root_info(ci: *mut acpi_pci_root_info) {
    let info = container_of!(ci, pci_root_info, common);
    pci_ecam_free((*info).cfg);
    kfree((*ci).ops);
    kfree(info);
}

unsafe fn acpi_prepare_root_resources(ci: *mut acpi_pci_root_info) -> i32 {
    let mut status: i32;
    let mut pci_h: u64 = 0;
    let mut entry: *mut resource_entry;
    let mut tmp: *mut resource_entry;
    let device: *mut acpi_device = (*ci).bridge;

    acpi_remove_early_pio();

    status = acpi_pci_probe_root_resources(ci);
    if status > 0 {
        acpi_evaluate_integer((*device).handle, b"PCIH\0".as_ptr() as *const _, core::ptr::null(), &mut pci_h);
        if pci_h != 0 {
            return status;
        }

        resource_list_for_each_entry_safe!(entry, tmp, &mut (*ci).resources) {
            if (*(*entry).res).flags & IORESOURCE_MEM != 0 {
                (*entry).offset = (*(*ci).root).mcfg_addr & GENMASK_ULL(63, 40);
                (*(*entry).res).start |= (*entry).offset;
                (*(*entry).res).end |= (*entry).offset;
            }
        }
        return status;
    }

    resource_list_for_each_entry_safe!(entry, tmp, &mut (*ci).resources) {
        dev_dbg!(&(*device).dev, "host bridge window %pR (ignored)\n", (*entry).res);
        resource_list_destroy_entry(entry);
    }

    0
}

/*
 * Create a PCI config space window
 *  - reserve mem region
 *  - alloc struct pci_config_window with space for all mappings
 *  - ioremap the config space
 */
unsafe fn arch_pci_ecam_create(
    dev: *mut device,
    cfgres: *mut resource,
    busr: *mut resource,
    ops: *const pci_ecam_ops,
) -> *mut pci_config_window {
    let mut bsz: i32;
    let mut bus_range: usize;
    let mut err: i32;
    let mut conflict: *mut resource;
    let cfg: *mut pci_config_window = kzalloc_obj!();

    if (*busr).start > (*busr).end {
        return ERR_PTR!(-EINVAL);
    }
    if cfg.is_null() {
        return ERR_PTR!(-ENOMEM);
    }

    (*cfg).parent = dev;
    (*cfg).ops = ops;
    (*cfg).busr.start = (*busr).start;
    (*cfg).busr.end = (*busr).end;
    (*cfg).busr.flags = IORESOURCE_BUS;
    bus_range = resource_size(cfgres) >> (*ops).bus_shift;
    bsz = 1 << (*ops).bus_shift;

    (*cfg).res.start = (*cfgres).start;
    (*cfg).res.end = (*cfgres).end;
    (*cfg).res.flags = IORESOURCE_MEM | IORESOURCE_BUSY;
    (*cfg).res.name = b"PCI ECAM\0".as_ptr() as *mut _;

    conflict = request_resource_conflict(&mut iomem_resource, &mut (*cfg).res);
    if !conflict.is_null() {
        err = -EBUSY;
        dev_err!(dev, "can't claim ECAM area %pR: address conflict with %s %pR\n", &(*cfg).res, (*conflict).name, conflict);
        goto!(err_exit);
    }

    (*cfg).win = pci_remap_cfgspace((*cfgres).start, bus_range * bsz as usize);
    if (*cfg).win.is_null() {
        goto!(err_exit_iomap);
    }

    if let Some(init) = (*ops).init {
        err = init(cfg);
        if err != 0 {
            goto!(err_exit);
        }
    }
    dev_info!(dev, "ECAM at %pR for %pR\n", &(*cfg).res, &(*cfg).busr);
    return cfg;

err_exit_iomap:
    err = -ENOMEM;
    dev_err!(dev, "ECAM ioremap failed\n");
err_exit:
    pci_ecam_free(cfg);
    ERR_PTR!(err)
}

/* Lookup the bus range for the domain in MCFG, and set up config space mapping. */
unsafe fn pci_acpi_setup_ecam_mapping(root: *mut acpi_pci_root) -> *mut pci_config_window {
    let mut ret: i32;
    let bus_shift: u32;
    let seg: u16 = (*root).segment;
    let dev: *mut device = &mut (*(*root).device).dev;
    let mut cfgres: resource = core::mem::zeroed();
    let bus_res: *mut resource = &mut (*root).secondary;
    let cfg: *mut pci_config_window;
    let mut ecam_ops: *const pci_ecam_ops;

    ret = pci_mcfg_lookup(root, &mut cfgres, &mut ecam_ops);
    if ret < 0 {
        dev_err!(dev, "%04x:%pR ECAM region not found, use default value\n", seg, bus_res);
        ecam_ops = &loongson_pci_ecam_ops;
        (*root).mcfg_addr = mcfg_addr_init(0);
    }

    bus_shift = if (*ecam_ops).bus_shift != 0 { (*ecam_ops).bus_shift } else { 20 };
    if bus_shift == 20 {
        cfg = pci_ecam_create(dev, &mut cfgres, bus_res, ecam_ops);
    } else {
        cfgres.start = (*root).mcfg_addr + ((*bus_res).start << bus_shift);
        cfgres.end = cfgres.start + (resource_size(bus_res) << bus_shift) - 1;
        cfgres.end |= BIT(28) + (((PCI_CFG_SPACE_EXP_SIZE - 1) & 0xf00) << 16);
        cfgres.flags = IORESOURCE_MEM;
        cfg = arch_pci_ecam_create(dev, &mut cfgres, bus_res, ecam_ops);
    }

    if IS_ERR!(cfg) {
        dev_err!(dev, "%04x:%pR error %ld mapping ECAM\n", seg, bus_res, PTR_ERR!(cfg));
        return core::ptr::null_mut();
    }
    cfg
}

pub unsafe fn pci_acpi_scan_root(root: *mut acpi_pci_root) -> *mut pci_bus {
    let bus: *mut pci_bus;
    let info: *mut pci_root_info = kzalloc_obj!();
    let host: *mut pci_host_bridge;
    let root_ops: *mut acpi_pci_root_ops = kzalloc_obj!();
    let domain = (*root).segment;
    let busnum = (*root).secondary.start;

    if info.is_null() {
        pr_warn!("pci_bus %04x:%02x: ignored (out of memory)\n", domain, busnum);
        return core::ptr::null_mut();
    }
    if root_ops.is_null() {
        kfree(info);
        return core::ptr::null_mut();
    }

    (*info).cfg = pci_acpi_setup_ecam_mapping(root);
    if (*info).cfg.is_null() {
        kfree(info);
        kfree(root_ops);
        return core::ptr::null_mut();
    }

    (*root_ops).release_info = Some(acpi_release_root_info);
    (*root_ops).prepare_resources = Some(acpi_prepare_root_resources);
    (*root_ops).pci_ops = &(*(*info).cfg).ops.pci_ops as *const _ as *mut _;

    bus = pci_find_bus(domain as i32, busnum as i32);
    if !bus.is_null() {
        core::ptr::copy_nonoverlapping((*info).cfg as *const u8, (*bus).sysdata as *mut u8, core::mem::size_of::<pci_config_window>());
        kfree(info);
        kfree(root_ops);
    } else {
        let mut child: *mut pci_bus;
        let bus = acpi_pci_root_create(root, root_ops, &mut (*info).common, (*info).cfg);
        if bus.is_null() {
            kfree(info);
            kfree(root_ops);
            return core::ptr::null_mut();
        }
        host = pci_find_host_bridge(bus);
        if (*host).preserve_config {
            pci_bus_claim_resources(bus);
        }
        pci_assign_unassigned_root_bus_resources(bus);
        list_for_each_entry!(child, &mut (*bus).children, node) {
            pcie_bus_configure_settings(child);
        }
        return bus;
    }
    bus
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
