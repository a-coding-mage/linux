// SPDX-License-Identifier: GPL-2.0
/*
 * leon_pci.c: LEON Host PCI support
 *
 * Copyright (C) 2011 Aeroflex Gaisler AB, Daniel Hellstrom
 *
 * Code is partially derived from pcic.c
 */

/* External Linux and LEON declarations are supplied by the surrounding tree. */
use core::mem::MaybeUninit;

extern "C" {
    fn pci_alloc_host_bridge(size: usize) -> *mut pci_host_bridge;
    fn pci_add_resource_offset(resources: *mut list_head, resource: *mut resource,
                               offset: resource_size_t);
    fn pci_add_resource(resources: *mut list_head, resource: *mut resource);
    fn list_splice_init(list: *mut list_head, head: *mut list_head);
    fn pci_scan_root_bus_bridge(bridge: *mut pci_host_bridge) -> c_int;
    fn pci_free_host_bridge(bridge: *mut pci_host_bridge);
    fn pci_assign_unassigned_resources();
    fn pci_bus_add_devices(bus: *mut pci_bus);
    fn pci_common_swizzle(dev: *mut pci_dev, slot: u8, pin: u8) -> u8;
}

extern "C" {
    static IORESOURCE_BUS: resource_size_t;
}

/* The LEON architecture does not rely on a BIOS or bootloader to setup
 * PCI for us. The Linux generic routines are used to setup resources,
 * reset values of configuration-space register settings are preserved.
 *
 * PCI Memory and Prefetchable Memory is direct-mapped. However I/O Space is
 * accessed through a Window which is translated to low 64KB in PCI space, the
 * first 4KB is not used so 60KB is available.
 */
#[allow(non_snake_case)]
pub unsafe extern "C" fn leon_pci_init(
    ofdev: *mut platform_device,
    info: *mut leon_pci_info,
) {
    let mut resources = MaybeUninit::<list_head>::uninit();
    let mut bridge: *mut pci_host_bridge;
    let mut root_bus: *mut pci_bus;
    let mut ret: c_int;

    bridge = pci_alloc_host_bridge(0);
    if bridge.is_null() {
        return;
    }

    pci_add_resource_offset(
        resources.as_mut_ptr(),
        &mut (*info).io_space,
        (*info).io_space.start.wrapping_sub(0x1000),
    );
    pci_add_resource(resources.as_mut_ptr(), &mut (*info).mem_space);
    (*info).busn.flags = IORESOURCE_BUS;
    pci_add_resource(resources.as_mut_ptr(), &mut (*info).busn);

    list_splice_init(resources.as_mut_ptr(), &mut (*bridge).windows);
    (*bridge).dev.parent = &mut (*ofdev).dev;
    (*bridge).sysdata = info as *mut core::ffi::c_void;
    (*bridge).busnr = 0;
    (*bridge).ops = (*info).ops;
    (*bridge).swizzle_irq = Some(pci_common_swizzle);
    (*bridge).map_irq = (*info).map_irq;

    ret = pci_scan_root_bus_bridge(bridge);
    if ret != 0 {
        pci_free_host_bridge(bridge);
        return;
    }

    root_bus = (*bridge).bus;

    /* Assign devices with resources */
    pci_assign_unassigned_resources();
    pci_bus_add_devices(root_bus);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
