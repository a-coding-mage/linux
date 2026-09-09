// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 2024-2025, Ventana Micro Systems Inc
 *	Author: Sunil V L <sunilvl@ventanamicro.com>
 */

// Dependencies are supplied by the surrounding kernel translation unit.

#[repr(C)]
struct RimtFwnode {
    list: ListHead,
    rimt_node: *mut AcpiRimtNode,
    fwnode: *mut FwnodeHandle,
}

static mut RIMT_FW_NODE_LIST: ListHead = ListHead { next: core::ptr::null_mut(), prev: core::ptr::null_mut() };
static mut RIMT_FW_NODE_LOCK: Spinlock = Spinlock {};
static mut RIMT_TABLE: *mut AcpiTableHeader = core::ptr::null_mut();

const RIMT_IOMMU_TYPE: u8 = 1 << 0;
const fn rimt_type_mask(ty: u8) -> u8 { 1 << ty }

unsafe fn rimt_set_fwnode(node: *mut AcpiRimtNode, fwnode: *mut FwnodeHandle) -> i32 {
    let np = kzalloc_rimt_fwnode();
    if np.is_null() { return -12; }
    init_list_head(&mut (*np).list);
    (*np).rimt_node = node;
    (*np).fwnode = fwnode;
    spin_lock(&mut RIMT_FW_NODE_LOCK);
    list_add_tail(&mut (*np).list, &mut RIMT_FW_NODE_LIST);
    spin_unlock(&mut RIMT_FW_NODE_LOCK);
    0
}

unsafe fn rimt_match_node_callback(node: *mut AcpiRimtNode, context: *mut Device) -> AcpiStatus {
    let mut status = AE_NOT_FOUND;
    if (*node).node_type == ACPI_RIMT_NODE_TYPE_IOMMU {
        let iommu = &*((&(*node).node_data) as *const _ as *const AcpiRimtIommu);
        if dev_is_pci(context) {
            let pdev = to_pci_dev(context);
            let bdf = pci_devid((*pdev).bus.as_ref().unwrap().number, (*pdev).devfn);
            status = if pci_domain_nr((*pdev).bus) == iommu.pcie_segment_number && bdf == iommu.pcie_bdf { AE_OK } else { AE_NOT_FOUND };
        } else {
            let pdev = to_platform_device(context);
            let res = platform_get_resource(pdev, IORESOURCE_MEM, 0);
            status = if !res.is_null() && (*res).start == iommu.base_address { AE_OK } else { AE_NOT_FOUND };
        }
    } else if (*node).node_type == ACPI_RIMT_NODE_TYPE_PCIE_ROOT_COMPLEX {
        let pci_rc = (*node).node_data as *const AcpiRimtPcieRc;
        let bus = to_pci_bus(context);
        status = if (*pci_rc).pcie_segment_number == pci_domain_nr(bus) { AE_OK } else { AE_NOT_FOUND };
    } else if (*node).node_type == ACPI_RIMT_NODE_TYPE_PLAT_DEVICE {
        let mut plat_dev = context;
        let mut adev: *mut AcpiDevice = core::ptr::null_mut();
        while !plat_dev.is_null() {
            adev = acpi_companion(plat_dev);
            if !adev.is_null() { break; }
            plat_dev = (*plat_dev).parent;
        }
        if adev.is_null() { return status; }
        let mut buf = AcpiBuffer { length: ACPI_ALLOCATE_BUFFER, pointer: core::ptr::null_mut() };
        status = acpi_get_name((*adev).handle, ACPI_FULL_PATHNAME, &mut buf);
        if acpi_failure(status) { dev_warn(plat_dev, "Can't get device full path name\n"); return status; }
        let ncomp = (*node).node_data as *const AcpiRimtPlatformDevice;
        status = if c_str_eq((*ncomp).device_name, buf.pointer) { AE_OK } else { AE_NOT_FOUND };
        acpi_os_free(buf.pointer);
    }
    status
}

unsafe fn rimt_scan_node(ty: AcpiRimtNodeType, context: *mut Device) -> *mut AcpiRimtNode {
    if RIMT_TABLE.is_null() { return core::ptr::null_mut(); }
    let rimt = RIMT_TABLE as *mut AcpiTableRimt;
    let mut node = acpi_add_ptr::<AcpiRimtNode>(rimt, (*rimt).node_offset as usize);
    let end = acpi_add_ptr::<AcpiRimtNode>(RIMT_TABLE, (*RIMT_TABLE).length as usize);
    for _ in 0..(*rimt).num_nodes {
        if node >= end { return core::ptr::null_mut(); }
        if (*node).node_type == ty && acpi_success(rimt_match_node_callback(node, context)) { return node; }
        node = acpi_add_ptr::<AcpiRimtNode>(node, (*node).length as usize);
    }
    core::ptr::null_mut()
}

pub unsafe fn rimt_iommu_register(dev: *mut Device) -> i32 {
    let node = rimt_scan_node(ACPI_RIMT_NODE_TYPE_IOMMU, dev);
    if node.is_null() { pr_err!("Could not find IOMMU node in RIMT\n"); return -19; }
    let fwnode = if dev_is_pci(dev) {
        let f = acpi_alloc_fwnode_static();
        if f.is_null() { return -12; }
        (*f).dev = dev;
        if (*dev).fwnode.is_null() { (*dev).fwnode = f; }
        f
    } else { (*dev).fwnode };
    rimt_set_fwnode(node, fwnode);
    0
}

#[cfg(CONFIG_IOMMU_API)]
unsafe fn rimt_get_fwnode(node: *mut AcpiRimtNode) -> *mut FwnodeHandle {
    let mut result = core::ptr::null_mut();
    spin_lock(&mut RIMT_FW_NODE_LOCK);
    let mut curr = list_first_entry::<RimtFwnode>(&RIMT_FW_NODE_LIST);
    while !curr.is_null() {
        if (*curr).rimt_node == node { result = (*curr).fwnode; break; }
        curr = list_next_entry::<RimtFwnode>(curr);
    }
    spin_unlock(&mut RIMT_FW_NODE_LOCK);
    result
}

#[cfg(CONFIG_IOMMU_API)]
unsafe fn rimt_pcie_rc_supports_ats(node: *mut AcpiRimtNode) -> bool {
    ((*((*node).node_data as *const AcpiRimtPcieRc)).flags & ACPI_RIMT_PCIE_ATS_SUPPORTED) != 0
}

#[cfg(CONFIG_IOMMU_API)]
unsafe fn rimt_iommu_xlate(dev: *mut Device, node: *mut AcpiRimtNode, deviceid: u32) -> i32 {
    if node.is_null() { return -19; }
    let fwnode = rimt_get_fwnode(node);
    if fwnode.is_null() { return driver_deferred_probe_check_state(dev); }
    device_link_add(dev, (*fwnode).dev, DL_FLAG_AUTOREMOVE_CONSUMER);
    acpi_iommu_fwspec_init(dev, deviceid, fwnode)
}

#[repr(C)]
struct RimtPciAliasInfo { dev: *mut Device, node: *mut AcpiRimtNode, ops: *const IommuOps }

#[cfg(CONFIG_IOMMU_API)]
unsafe fn rimt_id_map(map: *mut AcpiRimtIdMapping, _ty: u8, rid_in: u32, rid_out: *mut u32) -> i32 {
    if rid_in < (*map).source_id_base || rid_in > (*map).source_id_base + (*map).num_ids { return -6; }
    *rid_out = (*map).dest_id_base + rid_in - (*map).source_id_base;
    0
}

#[cfg(CONFIG_IOMMU_API)]
unsafe fn rimt_node_map_id(mut node: *mut AcpiRimtNode, id_in: u32, id_out: *mut u32, type_mask: u8) -> *mut AcpiRimtNode {
    let original = id_in; let mut id = id_in;
    while !node.is_null() {
        if rimt_type_mask((*node).node_type as u8) & type_mask != 0 { if !id_out.is_null() { *id_out = id; } return node; }
        let (off, count) = match (*node).node_type {
            ACPI_RIMT_NODE_TYPE_PCIE_ROOT_COMPLEX => { let n = &*((*node).node_data as *const AcpiRimtPcieRc); (n.id_mapping_offset, n.num_id_mappings) },
            ACPI_RIMT_NODE_TYPE_PLAT_DEVICE => { let n = &*((*node).node_data as *const AcpiRimtPlatformDevice); (n.id_mapping_offset, n.num_id_mappings) },
            _ => break,
        };
        if off == 0 || count == 0 { break; }
        let mut map = acpi_add_ptr::<AcpiRimtIdMapping>(node, off as usize);
        let mut found = false;
        for _ in 0..count { if rimt_id_map(map, (*node).node_type as u8, id, &mut id) == 0 { found = true; break; } map = map.add(1); }
        if !found { break; }
        node = acpi_add_ptr::<AcpiRimtNode>(RIMT_TABLE, (*map).dest_offset as usize);
    }
    if !id_out.is_null() { *id_out = original; }
    core::ptr::null_mut()
}

#[cfg(CONFIG_IOMMU_API)]
pub unsafe fn rimt_iommu_configure_id(dev: *mut Device, id_in: *const u32) -> i32 {
    let node = rimt_scan_node(if dev_is_pci(dev) { ACPI_RIMT_NODE_TYPE_PCIE_ROOT_COMPLEX } else { ACPI_RIMT_NODE_TYPE_PLAT_DEVICE }, dev);
    if node.is_null() { return -19; }
    let id = if id_in.is_null() { 0 } else { *id_in };
    let parent = rimt_node_map_id(node, id, core::ptr::null_mut(), RIMT_IOMMU_TYPE);
    rimt_iommu_xlate(dev, parent, id)
}

pub unsafe fn riscv_acpi_rimt_init() {
    let status = acpi_get_table(ACPI_SIG_RIMT, 0, &mut RIMT_TABLE);
    if acpi_failure(status) && status != AE_NOT_FOUND { pr_err!("Failed to get table, %s\n", acpi_format_exception(status)); }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
