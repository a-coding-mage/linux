// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Port for PPC64 David Engebretsen, IBM Corp.
 * Contains common pci routines for ppc64 platform, pSeries and iSeries brands.
 *
 * Copyright (C) 2003 Anton Blanchard <anton@au.ibm.com>, IBM
 *   Rework, based on alpha PCI code.
 */

// C headers and build-provided declarations are supplied by the surrounding kernel.

pub static mut pci_io_base: ::core::ffi::c_ulong = 0;

unsafe extern "C" {
    static mut ppc_md: PpcMd;
    static mut hose_list: ListHead;
    static mut pci_root_buses: ListHead;
    static isa_io_base: ::core::ffi::c_ulong;

    fn pci_phys_mem_access_prot() -> Prot;
    fn pci_add_flags(flags: ::core::ffi::c_uint);
    fn pcibios_scan_phb(hose: *mut PciController);
    fn pcibios_resource_survey();
    fn pci_bus_add_devices(bus: *mut PciBus);
    fn __flush_hash_table_range(start: ::core::ffi::c_ulong, end: ::core::ffi::c_ulong);
    fn pci_bus_to_host(bus: *mut PciBus) -> *mut PciController;
    fn pci_name(dev: *mut PciDev) -> *const ::core::ffi::c_char;
    fn iounmap(addr: *mut ::core::ffi::c_void);
    fn __get_vm_area_caller(size: ::core::ffi::c_ulong, flags: ::core::ffi::c_ulong,
        start: ::core::ffi::c_ulong, end: ::core::ffi::c_ulong,
        caller: *const ::core::ffi::c_void) -> *mut VmStruct;
    fn ioremap_page_range(addr: ::core::ffi::c_ulong, end: ::core::ffi::c_ulong,
        phys: PhysAddr, prot: Prot) -> ::core::ffi::c_int;
    fn pgprot_noncached(prot: Prot) -> Prot;
    fn vunmap_range(addr: ::core::ffi::c_ulong, end: ::core::ffi::c_ulong);
    fn pcibios_io_space_offset(hose: *mut PciController) -> ::core::ffi::c_ulong;
    fn of_machine_is_compatible(s: *const ::core::ffi::c_char) -> bool;
    fn of_find_compatible_node(a: *mut DeviceNode, b: *mut DeviceNode,
        c: *const ::core::ffi::c_char) -> *mut DeviceNode;
    fn of_node_put(np: *mut DeviceNode);
}

pub unsafe fn pcibios_init() -> ::core::ffi::c_int {
    let mut hose: *mut PciController = core::ptr::null_mut();
    let mut tmp: *mut PciController = core::ptr::null_mut();
    printk("PCI: Probing PCI hardware\n");
    ppc_md.phys_mem_access_prot = Some(pci_phys_mem_access_prot);
    pci_add_flags(PCI_ENABLE_PROC_DOMAINS | PCI_COMPAT_DOMAIN_0);
    list_for_each_entry_safe(&mut hose, &mut tmp, &mut hose_list, PciController::list_node,
        pcibios_scan_phb);
    pcibios_resource_survey();
    list_for_each_entry_safe(&mut hose, &mut tmp, &mut hose_list, PciController::list_node,
        |h| pci_bus_add_devices((*h).bus));
    if let Some(fixup) = ppc_md.pcibios_fixup { fixup(); }
    printk("PCI: Probing PCI hardware done\n");
    0
}

pub unsafe fn pcibios_unmap_io_space(bus: *mut PciBus) -> ::core::ffi::c_int {
    warn_on(bus.is_null());
    if !(*bus).self_.is_null() {
        pr_debug("IO unmapping for PCI-PCI bridge %s\n", pci_name((*bus).self_));
        #[cfg(CONFIG_PPC_BOOK3S_64)] {
            let res = (*bus).resource[0];
            __flush_hash_table_range((*res).start + _IO_BASE, (*res).end + _IO_BASE + 1);
        }
        return 0;
    }
    let hose = pci_bus_to_host(bus);
    pr_debug("IO unmapping for PHB %pOF\n", (*hose).dn);
    pr_debug("  alloc=0x%p\n", (*hose).io_base_alloc);
    iounmap((*hose).io_base_alloc);
    0
}

pub unsafe fn ioremap_phb(paddr: PhysAddr, size: ::core::ffi::c_ulong) -> *mut ::core::ffi::c_void {
    warn_on_once(paddr & !PAGE_MASK != 0);
    warn_on_once(size & !PAGE_MASK != 0);
    let area = __get_vm_area_caller(size, VM_IOREMAP, PHB_IO_BASE, PHB_IO_END,
        core::ptr::addr_of!(ioremap_phb).cast());
    if area.is_null() { return core::ptr::null_mut(); }
    let addr = (*area).addr as ::core::ffi::c_ulong;
    if ioremap_page_range(addr, addr + size, paddr, pgprot_noncached(PAGE_KERNEL)) != 0 {
        vunmap_range(addr, addr + size);
        return core::ptr::null_mut();
    }
    addr as *mut ::core::ffi::c_void
}

unsafe fn pcibios_map_phb_io_space(hose: *mut PciController) -> ::core::ffi::c_int {
    let phys_page = (*hose).io_base_phys & !(PAGE_SIZE - 1);
    let size_page = ((*hose).pci_io_size + PAGE_SIZE - 1) & !(PAGE_SIZE - 1);
    (*hose).io_base_alloc = core::ptr::null_mut();
    if (*hose).pci_io_size == 0 || (*hose).io_base_phys == 0 { return 0; }
    (*hose).io_base_alloc = ioremap_phb(phys_page, size_page);
    if (*hose).io_base_alloc.is_null() { return -ENOMEM; }
    (*hose).io_base_virt = (*hose).io_base_alloc.add((*hose).io_base_phys - phys_page as _);
    pr_debug("IO mapping for PHB %pOF\n", (*hose).dn);
    pr_debug("  phys=0x%016llx, virt=0x%p (alloc=0x%p)\n", (*hose).io_base_phys, (*hose).io_base_virt, (*hose).io_base_alloc);
    pr_debug("  size=0x%016llx (alloc=0x%016lx)\n", (*hose).pci_io_size, size_page);
    let off = pcibios_io_space_offset(hose);
    (*hose).io_resource.start += off;
    (*hose).io_resource.end += off;
    pr_debug("  hose->io_resource=%pR\n", &(*hose).io_resource);
    0
}

pub unsafe fn pcibios_map_io_space(bus: *mut PciBus) -> ::core::ffi::c_int {
    warn_on(bus.is_null());
    if !(*bus).self_.is_null() {
        pr_debug("IO mapping for PCI-PCI bridge %s\n", pci_name((*bus).self_));
        pr_debug("  virt=0x%016llx...0x%016llx\n", (*(*bus).resource[0]).start + _IO_BASE, (*(*bus).resource[0]).end + _IO_BASE);
        return 0;
    }
    pcibios_map_phb_io_space(pci_bus_to_host(bus))
}

pub unsafe fn pcibios_setup_phb_io_space(hose: *mut PciController) { pcibios_map_phb_io_space(hose); }

pub const IOBASE_BRIDGE_NUMBER: ::core::ffi::c_long = 0;
pub const IOBASE_MEMORY: ::core::ffi::c_long = 1;
pub const IOBASE_IO: ::core::ffi::c_long = 2;
pub const IOBASE_ISA_IO: ::core::ffi::c_long = 3;
pub const IOBASE_ISA_MEM: ::core::ffi::c_long = 4;

pub unsafe fn pciconfig_iobase(which: ::core::ffi::c_long, mut in_bus: ::core::ffi::c_ulong,
    _in_devfn: ::core::ffi::c_ulong) -> ::core::ffi::c_long {
    let mut bus: *mut PciBus = core::ptr::null_mut();
    if in_bus == 0 && of_machine_is_compatible(c"MacRISC4".as_ptr()) {
        let agp = of_find_compatible_node(core::ptr::null_mut(), core::ptr::null_mut(), c"u3-agp".as_ptr());
        if !agp.is_null() { in_bus = 0xf0; }
        of_node_put(agp);
    }
    list_for_each_entry(&mut bus, &pci_root_buses, PciBus::node, |b| {
        if in_bus >= (*b).number && in_bus <= (*b).busn_res.end { bus = b; }
    });
    if bus.is_null() || (*bus).dev.of_node.is_null() { return -ENODEV; }
    let hose = (*PCI_DN((*bus).dev.of_node)).phb;
    match which {
        IOBASE_BRIDGE_NUMBER => (*hose).first_busno as _,
        IOBASE_MEMORY => (*hose).mem_offset[0] as _,
        IOBASE_IO => (*hose).io_base_phys as _,
        IOBASE_ISA_IO => isa_io_base as _,
        IOBASE_ISA_MEM => -EINVAL,
        _ => -EOPNOTSUPP,
    }
}

#[cfg(CONFIG_NUMA)]
pub unsafe fn pcibus_to_node(bus: *mut PciBus) -> ::core::ffi::c_int { (*pci_bus_to_host(bus)).node }

#[cfg(CONFIG_PPC_PMAC)]
pub unsafe fn pci_device_from_OF_node(np: *mut DeviceNode, bus: *mut u8, devfn: *mut u8) -> ::core::ffi::c_int {
    if PCI_DN(np).is_null() { return -ENODEV; }
    *bus = (*PCI_DN(np)).busno;
    *devfn = (*PCI_DN(np)).devfn;
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
