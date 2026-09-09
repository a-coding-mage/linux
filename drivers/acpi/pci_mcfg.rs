// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 2016 Broadcom
 * Copyright (C) 2016 Semihalf
 */

// Translated from acpi/pci_mcfg.c.  Kernel-provided types, constants, and
// functions referenced below are supplied by the surrounding Rust port.

#[repr(C)]
pub struct McfgEntry {
    pub list: ListHead,
    pub addr: PhysAddrT,
    pub segment: u16,
    pub bus_start: u8,
    pub bus_end: u8,
}

#[cfg(feature = "CONFIG_PCI_QUIRKS")]
#[repr(C)]
pub struct McfgFixup {
    pub oem_id: [libc::c_char; ACPI_OEM_ID_SIZE + 1],
    pub oem_table_id: [libc::c_char; ACPI_OEM_TABLE_ID_SIZE + 1],
    pub oem_revision: u32,
    pub segment: u16,
    pub bus_range: Resource,
    pub ops: *const PciEcamOps,
    pub cfgres: Resource,
}

#[cfg(feature = "CONFIG_PCI_QUIRKS")]
static mut MCFG_OEM_ID: [libc::c_char; ACPI_OEM_ID_SIZE] = [0; ACPI_OEM_ID_SIZE];
#[cfg(feature = "CONFIG_PCI_QUIRKS")]
static mut MCFG_OEM_TABLE_ID: [libc::c_char; ACPI_OEM_TABLE_ID_SIZE] = [0; ACPI_OEM_TABLE_ID_SIZE];
#[cfg(feature = "CONFIG_PCI_QUIRKS")]
static mut MCFG_OEM_REVISION: u32 = 0;

extern "C" {
    static mut pci_mcfg_list: ListHead;
}

#[cfg(feature = "CONFIG_PCI_QUIRKS")]
unsafe fn pci_mcfg_quirk_matches(f: *mut McfgFixup, segment: u16, bus_range: *mut Resource) -> i32 {
    if libc::memcmp((*f).oem_id.as_ptr(), MCFG_OEM_ID.as_ptr(), ACPI_OEM_ID_SIZE) == 0
        && libc::memcmp((*f).oem_table_id.as_ptr(), MCFG_OEM_TABLE_ID.as_ptr(), ACPI_OEM_TABLE_ID_SIZE) == 0
        && (*f).oem_revision == MCFG_OEM_REVISION
        && (*f).segment == segment
        && resource_contains(&(*f).bus_range, &*bus_range)
    { 1 } else { 0 }
}

unsafe fn pci_mcfg_apply_quirks(
    root: *mut AcpiPciRoot,
    cfgres: *mut Resource,
    ecam_ops: *mut *const PciEcamOps,
) {
    #[cfg(feature = "CONFIG_PCI_QUIRKS")]
    {
        let segment = (*root).segment;
        let bus_range = &mut (*root).secondary as *mut Resource;
        let mut f = MCFG_QUIRKS.as_mut_ptr();
        let mut i = 0usize;
        while i < MCFG_QUIRKS.len() {
            if pci_mcfg_quirk_matches(f, segment, bus_range) != 0 {
                if (*f).cfgres.start != 0 { *cfgres = (*f).cfgres; }
                if !(*f).ops.is_null() { *ecam_ops = (*f).ops; }
                dev_info(&(*(*root).device).dev, "MCFG quirk: ECAM at %pR for %pR with %ps\n", cfgres, bus_range, *ecam_ops);
                return;
            }
            i += 1;
            f = f.add(1);
        }
    }
}

#[cfg(feature = "CONFIG_PCI_QUIRKS")]
static mut MCFG_QUIRKS: [McfgFixup; 0] = [];

#[no_mangle]
pub unsafe extern "C" fn pci_mcfg_lookup(root: *mut AcpiPciRoot, cfgres: *mut Resource, ecam_ops: *mut *const PciEcamOps) -> i32 {
    let mut ops: *const PciEcamOps = &pci_generic_ecam_ops;
    let bus_res = &mut (*root).secondary as *mut Resource;
    let seg = (*root).segment;
    let mut res: Resource = core::mem::zeroed();

    if (*root).mcfg_addr == 0 {
        let mut pos = pci_mcfg_list.next;
        while pos != &mut pci_mcfg_list as *mut ListHead {
            let e = container_of!(pos, McfgEntry, list);
            if (*e).segment == seg && (*e).bus_start as u64 <= (*bus_res).start && (*e).bus_end as u64 >= (*bus_res).end {
                (*root).mcfg_addr = (*e).addr;
            }
            pos = (*pos).next;
        }
    }
    if (*root).mcfg_addr != 0 {
        res.start = (*root).mcfg_addr + ((*bus_res).start << 20);
        res.end = res.start + (resource_size(&*bus_res) << 20) - 1;
        res.flags = IORESOURCE_MEM;
    }
    pci_mcfg_apply_quirks(root, &mut res, &mut ops);
    if res.start == 0 { return -ENXIO; }
    *cfgres = res;
    *ecam_ops = ops;
    0
}

unsafe extern "C" fn pci_mcfg_parse(header: *mut AcpiTableHeader) -> i32 {
    if (*header).length < core::mem::size_of::<AcpiTableMcfg>() { return -EINVAL; }
    let n = ((*header).length as usize - core::mem::size_of::<AcpiTableMcfg>()) / core::mem::size_of::<AcpiMcfgAllocation>();
    let mptr = (header as *mut u8).add(core::mem::size_of::<AcpiTableMcfg>()) as *mut AcpiMcfgAllocation;
    let arr = kzalloc_objs::<McfgEntry>(n);
    if arr.is_null() { return -ENOMEM; }
    for i in 0..n {
        let e = arr.add(i);
        let m = mptr.add(i);
        (*e).segment = (*m).pci_segment;
        (*e).addr = (*m).address;
        (*e).bus_start = (*m).start_bus_number;
        (*e).bus_end = (*m).end_bus_number;
        list_add(&mut (*e).list, &mut pci_mcfg_list);
    }
    #[cfg(feature = "CONFIG_PCI_QUIRKS")]
    {
        libc::memcpy(MCFG_OEM_ID.as_mut_ptr(), (*header).oem_id.as_ptr(), ACPI_OEM_ID_SIZE);
        libc::memcpy(MCFG_OEM_TABLE_ID.as_mut_ptr(), (*header).oem_table_id.as_ptr(), ACPI_OEM_TABLE_ID_SIZE);
        MCFG_OEM_REVISION = (*header).oem_revision;
    }
    pr_info!("MCFG table detected, {} entries\n", n);
    0
}

#[no_mangle]
pub unsafe extern "C" fn pci_mmcfg_late_init() {
    let err = acpi_table_parse(ACPI_SIG_MCFG, Some(pci_mcfg_parse));
    if err != 0 { pr_debug!("Failed to parse MCFG ({})\n", err); }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
