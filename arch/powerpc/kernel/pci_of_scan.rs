// SPDX-License-Identifier: GPL-2.0-only
/* Helper routines to scan the device tree for PCI devices and busses. */

// Dependencies supplied by the kernel's PCI, OF, and PowerPC bridge code.

const OF_PCI_ADDR0_SPACE_CFG: u32 = 0 << 24;
const OF_PCI_ADDR0_SPACE_IO: u32 = 1 << 24;
const OF_PCI_ADDR0_SPACE_MMIO32: u32 = 2 << 24;
const OF_PCI_ADDR0_SPACE_MMIO64: u32 = 3 << 24;
const OF_PCI_ADDR0_SPACE_MASK: u32 = 3 << 24;
const OF_PCI_ADDR0_RELOC: u32 = 1u32 << 31;
const OF_PCI_ADDR0_PREFETCH: u32 = 1u32 << 30;
const OF_PCI_ADDR0_ALIAS: u32 = 1u32 << 29;
const OF_PCI_ADDR0_BUS: u32 = 0x00ff0000;
const OF_PCI_ADDR0_DEV: u32 = 0x0000f800;
const OF_PCI_ADDR0_FN: u32 = 0x00000700;
const OF_PCI_ADDR0_BARREG: u32 = 0x000000ff;

unsafe fn get_int_prop(np: *mut device_node, name: *const i8, def: u32) -> u32 {
    let mut len: i32 = 0;
    let prop = of_get_property(np, name, &mut len);
    if !prop.is_null() && len >= 4 { of_read_number(prop, 1) as u32 } else { def }
}

pub unsafe fn pci_parse_of_flags(addr0: u32, bridge: i32) -> u32 {
    let as_ = addr0 & OF_PCI_ADDR0_SPACE_MASK;
    let mut flags: u32 = 0;
    if as_ == OF_PCI_ADDR0_SPACE_MMIO32 || as_ == OF_PCI_ADDR0_SPACE_MMIO64 {
        flags = IORESOURCE_MEM | PCI_BASE_ADDRESS_SPACE_MEMORY;
        if as_ == OF_PCI_ADDR0_SPACE_MMIO64 { flags |= PCI_BASE_ADDRESS_MEM_TYPE_64 | IORESOURCE_MEM_64; }
        if addr0 & OF_PCI_ADDR0_ALIAS != 0 { flags |= PCI_BASE_ADDRESS_MEM_TYPE_1M; }
        if addr0 & OF_PCI_ADDR0_PREFETCH != 0 { flags |= IORESOURCE_PREFETCH | PCI_BASE_ADDRESS_MEM_PREFETCH; }
        if bridge == 0 && (addr0 & OF_PCI_ADDR0_BARREG) == PCI_ROM_ADDRESS { flags |= IORESOURCE_READONLY; }
    } else if as_ == OF_PCI_ADDR0_SPACE_IO {
        flags = IORESOURCE_IO | PCI_BASE_ADDRESS_SPACE_IO;
    }
    if flags != 0 { flags |= IORESOURCE_SIZEALIGN; }
    flags
}

unsafe fn of_pci_parse_addrs(node: *mut device_node, dev: *mut pci_dev) {
    let mut addrs: *const __be32;
    let mut proplen: i32 = 0;
    let mut mark_unset = false;
    addrs = of_get_property(node, b"assigned-addresses\0".as_ptr() as *const i8, &mut proplen);
    if addrs.is_null() || proplen == 0 {
        addrs = of_get_property(node, b"reg\0".as_ptr() as *const i8, &mut proplen);
        if addrs.is_null() || proplen == 0 { return; }
        mark_unset = true;
    }
    pr_debug!("    parse addresses ({:?} bytes) @ {:?}\n", proplen, addrs);
    while proplen >= 20 {
        let flags = pci_parse_of_flags(of_read_number(addrs, 1) as u32, 0);
        if flags != 0 {
            let base = of_read_number(addrs.add(1), 2);
            let size = of_read_number(addrs.add(3), 2);
            if size != 0 {
                let i = (of_read_number(addrs, 1) as u32) & 0xff;
                let res: *mut resource;
                if PCI_BASE_ADDRESS_0 <= i && i <= PCI_BASE_ADDRESS_5 {
                    res = &mut (*dev).resource[((i - PCI_BASE_ADDRESS_0) >> 2) as usize];
                } else if i == (*dev).rom_base_reg {
                    res = &mut (*dev).resource[PCI_ROM_RESOURCE as usize];
                    (*res).flags |= IORESOURCE_READONLY;
                } else { printk!(KERN_ERR "PCI: bad cfg reg num 0x{:x}\n", i); addrs = addrs.add(5); proplen -= 20; continue; }
                (*res).flags = flags;
                if mark_unset { (*res).flags |= IORESOURCE_UNSET; }
                (*res).name = pci_name(dev);
                let mut region = pci_bus_region { start: base, end: base + size - 1 };
                pcibios_bus_to_resource((*dev).bus, res, &mut region);
            }
        }
        addrs = addrs.add(5); proplen -= 20;
    }
}

pub unsafe fn of_create_pci_dev(node: *mut device_node, bus: *mut pci_bus, devfn: i32) -> *mut pci_dev {
    let dev = pci_alloc_dev(bus); if dev.is_null() { return core::ptr::null_mut(); }
    (*dev).dev.of_node = of_node_get(node); (*dev).dev.parent = (*bus).bridge; (*dev).dev.bus = &pci_bus_type;
    (*dev).devfn = devfn; (*dev).multifunction = 0; (*dev).needs_freset = 0; set_pcie_port_type(dev);
    pci_dev_assign_slot(dev);
    (*dev).vendor = get_int_prop(node, b"vendor-id\0".as_ptr() as *const i8, 0xffff);
    (*dev).device = get_int_prop(node, b"device-id\0".as_ptr() as *const i8, 0xffff);
    (*dev).subsystem_vendor = get_int_prop(node, b"subsystem-vendor-id\0".as_ptr() as *const i8, 0);
    (*dev).subsystem_device = get_int_prop(node, b"subsystem-id\0".as_ptr() as *const i8, 0);
    (*dev).cfg_size = pci_cfg_space_size(dev);
    dev_set_name(&mut (*dev).dev, b"%04x:%02x:%02x.%d\0".as_ptr() as *const i8, pci_domain_nr(bus), (*(*dev).bus).number, PCI_SLOT(devfn), PCI_FUNC(devfn));
    (*dev).class = get_int_prop(node, b"class-code\0".as_ptr() as *const i8, 0); (*dev).revision = get_int_prop(node, b"revision-id\0".as_ptr() as *const i8, 0);
    (*dev).current_state = PCI_UNKNOWN; (*dev).error_state = pci_channel_io_normal; (*dev).dma_mask = 0xffffffff; (*dev).msi_addr_mask = DMA_BIT_MASK(64);
    pci_fixup_device(pci_fixup_early, dev);
    if of_node_is_type(node, b"pci\0".as_ptr() as *const i8) || of_node_is_type(node, b"pciex\0".as_ptr() as *const i8) { (*dev).hdr_type = PCI_HEADER_TYPE_BRIDGE; (*dev).rom_base_reg = PCI_ROM_ADDRESS1; set_pcie_hotplug_bridge(dev); }
    else if of_node_is_type(node, b"cardbus\0".as_ptr() as *const i8) { (*dev).hdr_type = PCI_HEADER_TYPE_CARDBUS; }
    else { (*dev).hdr_type = PCI_HEADER_TYPE_NORMAL; (*dev).rom_base_reg = PCI_ROM_ADDRESS; (*dev).irq = 0; }
    of_pci_parse_addrs(node, dev); pci_device_add(dev, bus); dev
}

pub unsafe fn of_scan_pci_bridge(dev: *mut pci_dev) {
    let node = (*dev).dev.of_node; let mut len = 0; let busrange = of_get_property(node, b"bus-range\0".as_ptr() as *const i8, &mut len); if busrange.is_null() || len != 8 { return; }
    let ranges = of_get_property(node, b"ranges\0".as_ptr() as *const i8, &mut len); if ranges.is_null() { return; }
    let bus = pci_find_bus(pci_domain_nr((*dev).bus), of_read_number(busrange, 1)); if bus.is_null() { return; }
    (*bus).primary = (*(*dev).bus).number; pci_bus_insert_busn_res(bus, of_read_number(busrange, 1), of_read_number(busrange.add(1), 1)); (*bus).bridge_ctl = 0;
    let mut res = &mut (*dev).resource[PCI_BRIDGE_RESOURCES as usize]; let mut i = 0; while i < PCI_NUM_RESOURCES - PCI_BRIDGE_RESOURCES { res.flags = 0; (*bus).resource[i as usize] = res; i += 1; res = res.add(1); }
    i = 1; let mut rp = ranges; while len >= 32 { let flags = pci_parse_of_flags(of_read_number(rp, 1) as u32, 1); let size = of_read_number(rp.add(6), 2); if flags != 0 && size != 0 { let slot = if flags & IORESOURCE_IO != 0 { 0 } else { let x=i; i+=1; x }; res = (*bus).resource[slot as usize]; res.flags = flags; let mut region = pci_bus_region { start: of_read_number(rp.add(1), 2), end: of_read_number(rp.add(1), 2)+size-1 }; pcibios_bus_to_resource((*dev).bus, res, &mut region); } len-=32; rp=rp.add(8); }
    let phb = pci_bus_to_host(bus); let mode = if (*phb).controller_ops.probe_mode.is_some() { ((*phb).controller_ops.probe_mode.unwrap())(bus) } else { PCI_PROBE_NORMAL }; if mode == PCI_PROBE_DEVTREE { of_scan_bus(node,bus); } else if mode == PCI_PROBE_NORMAL { pci_scan_child_bus(bus); }
}

pub unsafe fn of_scan_bus(node: *mut device_node, bus: *mut pci_bus) { __of_scan_bus(node,bus,0); }
pub unsafe fn of_rescan_bus(node: *mut device_node, bus: *mut pci_bus) { __of_scan_bus(node,bus,1); }

unsafe fn of_scan_pci_dev(bus: *mut pci_bus, dn: *mut device_node) -> *mut pci_dev {
    if !of_device_is_available(dn) { return core::ptr::null_mut(); }
    let mut reglen = 0; let reg = of_get_property(dn, b"reg\0".as_ptr() as *const i8, &mut reglen);
    if reg.is_null() || reglen < 20 { return core::ptr::null_mut(); }
    let devfn = ((of_read_number(reg, 1) >> 8) & 0xff) as i32;
    let dev = pci_get_slot(bus, devfn); if !dev.is_null() { pci_dev_put(dev); return dev; }
    of_create_pci_dev(dn, bus, devfn)
}

unsafe fn __of_scan_bus(node: *mut device_node, bus: *mut pci_bus, rescan_existing: i32) {
    // `for_each_child_of_node` and `for_each_pci_bridge` are kernel iteration
    // macros; their dependency-provided Rust equivalents supply these loops.
    let mut child: *mut device_node = core::ptr::null_mut();
    while next_child_of_node(node, &mut child) {
        let dev = of_scan_pci_dev(bus, child);
        if !dev.is_null() { /* device header type is logged by the C implementation */ }
    }
    if rescan_existing == 0 { pcibios_setup_bus_self(bus); }
    let mut dev: *mut pci_dev = core::ptr::null_mut();
    while next_pci_bridge(bus, &mut dev) { of_scan_pci_bridge(dev); }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
