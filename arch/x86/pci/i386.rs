// SPDX-License-Identifier: GPL-2.0
/*
 * Low-Level PCI Access for i386 machines
 *
 * Copyright 1993, 1994 Drew Eckhardt
 * Copyright 1997--2000 Martin Mares
 */

// Linux kernel dependencies supplied by other translation units are intentionally
// referenced but not reimplemented here.

#[repr(C)]
struct PcibiosFwaddrmap {
    list: ListHead,
    dev: *mut PciDev,
    fw_addr: [ResourceSizeT; DEVICE_COUNT_RESOURCE],
}

static mut PCIBIOS_FWADDRMAPPINGS: ListHead = ListHead::new();
static mut PCIBIOS_FWADDRMAP_LOCK: Spinlock = Spinlock::new();
static mut PCIBIOS_FW_ADDR_DONE: bool = false;

/* Must be called with `pcibios_fwaddrmap_lock` held. */
unsafe fn pcibios_fwaddrmap_lookup(dev: *mut PciDev) -> *mut PcibiosFwaddrmap {
    lockdep_assert_held(&PCIBIOS_FWADDRMAP_LOCK);
    let mut map: *mut PcibiosFwaddrmap;
    list_for_each_entry!(map, &PCIBIOS_FWADDRMAPPINGS, list) {
        if (*map).dev == dev {
            return map;
        }
    }
    core::ptr::null_mut()
}

unsafe fn pcibios_save_fw_addr(dev: *mut PciDev, idx: i32, fw_addr: ResourceSizeT) {
    let mut flags: CULong = 0;
    let mut map: *mut PcibiosFwaddrmap;

    if PCIBIOS_FW_ADDR_DONE {
        return;
    }

    spin_lock_irqsave(&PCIBIOS_FWADDRMAP_LOCK, &mut flags);
    map = pcibios_fwaddrmap_lookup(dev);
    if map.is_null() {
        spin_unlock_irqrestore(&PCIBIOS_FWADDRMAP_LOCK, flags);
        map = kzalloc_obj::<PcibiosFwaddrmap>();
        if map.is_null() {
            return;
        }

        (*map).dev = pci_dev_get(dev);
        (*map).fw_addr[idx as usize] = fw_addr;
        INIT_LIST_HEAD!(&mut (*map).list);

        spin_lock_irqsave(&PCIBIOS_FWADDRMAP_LOCK, &mut flags);
        list_add_tail!(&mut (*map).list, &mut PCIBIOS_FWADDRMAPPINGS);
    } else {
        (*map).fw_addr[idx as usize] = fw_addr;
    }
    spin_unlock_irqrestore(&PCIBIOS_FWADDRMAP_LOCK, flags);
}

pub unsafe fn pcibios_retrieve_fw_addr(dev: *mut PciDev, idx: i32) -> ResourceSizeT {
    let mut flags: CULong = 0;
    let map: *mut PcibiosFwaddrmap;
    let mut fw_addr: ResourceSizeT = 0;

    if PCIBIOS_FW_ADDR_DONE {
        return 0;
    }
    spin_lock_irqsave(&PCIBIOS_FWADDRMAP_LOCK, &mut flags);
    map = pcibios_fwaddrmap_lookup(dev);
    if !map.is_null() {
        fw_addr = (*map).fw_addr[idx as usize];
    }
    spin_unlock_irqrestore(&PCIBIOS_FWADDRMAP_LOCK, flags);
    fw_addr
}

unsafe fn pcibios_fw_addr_list_del() {
    let mut flags: CULong = 0;
    let mut entry: *mut PcibiosFwaddrmap;
    let mut next: *mut PcibiosFwaddrmap;

    spin_lock_irqsave(&PCIBIOS_FWADDRMAP_LOCK, &mut flags);
    list_for_each_entry_safe!(entry, next, &mut PCIBIOS_FWADDRMAPPINGS, list) {
        list_del!(&mut (*entry).list);
        pci_dev_put((*entry).dev);
        kfree(entry);
    }
    spin_unlock_irqrestore(&PCIBIOS_FWADDRMAP_LOCK, flags);
    PCIBIOS_FW_ADDR_DONE = true;
}

unsafe fn skip_isa_ioresource_align(dev: *mut PciDev) -> i32 {
    if (pci_probe & PCI_CAN_SKIP_ISA_ALIGN) != 0
        && ((*(*dev).bus).bridge_ctl & PCI_BRIDGE_CTL_ISA) == 0
    {
        return 1;
    }
    0
}

pub unsafe fn pcibios_align_resource(
    data: *mut core::ffi::c_void,
    res: *const Resource,
    empty_res: *const Resource,
    size: ResourceSizeT,
    align: ResourceSizeT,
) -> ResourceSizeT {
    let dev = data as *mut PciDev;
    let mut start = (*res).start;

    if ((*res).flags & IORESOURCE_IO) != 0 {
        if skip_isa_ioresource_align(dev) != 0 {
            return start;
        }
        if (start & 0x300) != 0 {
            start = (start.wrapping_add(0x3ff)) & !0x3ff;
        }
    } else if ((*res).flags & IORESOURCE_MEM) != 0 {
        start = pci_align_resource(dev, res, empty_res, size, align);
        if start < BIOS_END {
            start = BIOS_END;
        }
    }
    start
}

unsafe fn pcibios_allocate_bridge_resources(dev: *mut PciDev) {
    for idx in PCI_BRIDGE_RESOURCES..PCI_NUM_RESOURCES {
        let r = &mut (*dev).resource[idx as usize];
        if r.flags == 0 || !r.parent.is_null() {
            continue;
        }
        if r.start == 0 || pci_claim_bridge_resource(dev, idx) < 0 {
            r.start = 0;
            r.end = 0;
            r.flags = 0;
        }
    }
}

unsafe fn pcibios_allocate_bus_resources(bus: *mut PciBus) {
    if !(*bus).self_.is_null() {
        pcibios_allocate_bridge_resources((*bus).self_);
    }
    let mut child: *mut PciBus;
    list_for_each_entry!(child, &(*bus).children, node) {
        pcibios_allocate_bus_resources(child);
    }
}

#[repr(C)]
struct PciCheckIdxRange { start: i32, end: i32 }

unsafe fn pcibios_allocate_dev_resources(dev: *mut PciDev, pass: i32) {
    let ranges = [
        PciCheckIdxRange { start: PCI_STD_RESOURCES, end: PCI_STD_RESOURCE_END },
        // CONFIG_PCI_IOV range is included by the corresponding build configuration.
        PciCheckIdxRange { start: PCI_IOV_RESOURCES, end: PCI_IOV_RESOURCE_END },
    ];
    let mut command: u16 = 0;
    pci_read_config_word(dev, PCI_COMMAND, &mut command);
    for range in ranges {
        for idx in range.start..=range.end {
            let r = &mut (*dev).resource[idx as usize];
            if !r.parent.is_null() || r.start == 0 { continue; }
            let disabled = if (r.flags & IORESOURCE_IO) != 0 {
                (command & PCI_COMMAND_IO) == 0
            } else { (command & PCI_COMMAND_MEMORY) == 0 };
            if pass == disabled as i32 && pci_claim_resource(dev, idx) < 0 {
                if (r.flags & IORESOURCE_PCI_FIXED) == 0 {
                    pcibios_save_fw_addr(dev, idx, r.start);
                    r.end = r.end.wrapping_sub(r.start);
                    r.start = 0;
                }
            }
        }
    }
    if pass == 0 {
        let r = &mut (*dev).resource[PCI_ROM_RESOURCE as usize];
        if (r.flags & IORESOURCE_ROM_ENABLE) != 0 {
            let mut reg: u32 = 0;
            r.flags &= !IORESOURCE_ROM_ENABLE;
            pci_read_config_dword(dev, (*dev).rom_base_reg, &mut reg);
            pci_write_config_dword(dev, (*dev).rom_base_reg, reg & !PCI_ROM_ADDRESS_ENABLE);
        }
    }
}

unsafe fn pcibios_allocate_resources(bus: *mut PciBus, pass: i32) {
    let mut dev: *mut PciDev;
    list_for_each_entry!(dev, &(*bus).devices, bus_list) {
        pcibios_allocate_dev_resources(dev, pass);
        if !(*dev).subordinate.is_null() {
            pcibios_allocate_resources((*dev).subordinate, pass);
        }
    }
}

unsafe fn pcibios_allocate_dev_rom_resource(dev: *mut PciDev) {
    let r = &mut (*dev).resource[PCI_ROM_RESOURCE as usize];
    if r.flags == 0 || r.start == 0 || !r.parent.is_null() { return; }
    if pci_claim_resource(dev, PCI_ROM_RESOURCE) < 0 {
        r.end = r.end.wrapping_sub(r.start);
        r.start = 0;
    }
}

unsafe fn pcibios_allocate_rom_resources(bus: *mut PciBus) {
    let mut dev: *mut PciDev;
    list_for_each_entry!(dev, &(*bus).devices, bus_list) {
        pcibios_allocate_dev_rom_resource(dev);
        if !(*dev).subordinate.is_null() {
            pcibios_allocate_rom_resources((*dev).subordinate);
        }
    }
}

unsafe fn pcibios_assign_resources() -> i32 {
    if (pci_probe & PCI_ASSIGN_ROMS) == 0 {
        let mut bus: *mut PciBus;
        list_for_each_entry!(bus, &pci_root_buses, node) { pcibios_allocate_rom_resources(bus); }
    }
    pci_assign_unassigned_resources();
    pcibios_fw_addr_list_del();
    0
}

// fs_initcall(pcibios_assign_resources)

pub unsafe fn pcibios_resource_survey_bus(bus: *mut PciBus) {
    dev_printk(KERN_DEBUG, &mut (*bus).dev, "Allocating resources\n");
    pcibios_allocate_bus_resources(bus);
    pcibios_allocate_resources(bus, 0);
    pcibios_allocate_resources(bus, 1);
    if (pci_probe & PCI_ASSIGN_ROMS) == 0 { pcibios_allocate_rom_resources(bus); }
}

pub unsafe fn pcibios_resource_survey() {
    DBG!("PCI: Allocating resources\n");
    let mut bus: *mut PciBus;
    list_for_each_entry!(bus, &pci_root_buses, node) { pcibios_allocate_bus_resources(bus); }
    list_for_each_entry!(bus, &pci_root_buses, node) { pcibios_allocate_resources(bus, 0); }
    list_for_each_entry!(bus, &pci_root_buses, node) { pcibios_allocate_resources(bus, 1); }
    e820__reserve_resources_late();
    ioapic_insert_resources();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
