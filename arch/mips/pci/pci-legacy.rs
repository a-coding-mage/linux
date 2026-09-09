// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *
 * Copyright (C) 2003, 04, 11 Ralf Baechle (ralf@linux-mips.org)
 * Copyright (C) 2011 Wind River Systems,
 *   written by Ralf Baechle (ralf@linux-mips.org)
 */

// If PCI_PROBE_ONLY in pci_flags is set, we don't change any PCI resource
// assignments.

// The PCI controller list.
static mut controllers: ListHead = ListHead::new();

static mut pci_initialized: i32 = 0;

pub unsafe fn pci_address_to_pio(address: phys_addr_t) -> c_ulong {
    if address > IO_SPACE_LIMIT {
        return (-1i32) as c_ulong;
    }

    address as c_ulong
}

/*
 * We need to avoid collisions with `mirrored' VGA ports
 * and other strange ISA hardware, so we always want the
 * addresses to be allocated in the 0x000-0x0ff region
 * modulo 0x400.
 *
 * Why? Because some silly external IO cards only decode
 * the low 10 bits of the IO address. The 0x00-0xff region
 * is reserved for motherboard devices that decode all 16
 * bits, so it's ok to allocate at, say, 0x2800-0x28ff,
 * but we want to try to avoid allocating at 0x2900-0x2bff
 * which might have be mirrored at 0x0100-0x03ff..
 */
pub unsafe fn pcibios_align_resource(
    data: *mut c_void,
    res: *const Resource,
    empty_res: *const Resource,
    size: resource_size_t,
    align: resource_size_t,
) -> resource_size_t {
    let dev = data as *mut PciDev;
    let hose = (*dev).sysdata as *mut PciController;
    let mut start = (*res).start;

    if (*res).flags & IORESOURCE_IO != 0 {
        // Make sure we start at our min on all hoses
        if start < PCIBIOS_MIN_IO + (*(*hose).io_resource).start {
            start = PCIBIOS_MIN_IO + (*(*hose).io_resource).start;
        }

        // Put everything into 0x00-0xff region modulo 0x400
        if start & 0x300 != 0 {
            start = (start + 0x3ff) & !0x3ff;
        }
    } else if (*res).flags & IORESOURCE_MEM != 0 {
        start = pci_align_resource(dev, res, empty_res, size, align);

        // Make sure we start at our min on all hoses
        if start < PCIBIOS_MIN_MEM + (*(*hose).mem_resource).start {
            start = PCIBIOS_MIN_MEM + (*(*hose).mem_resource).start;
        }
    }

    start
}

unsafe fn pcibios_scanbus(hose: *mut PciController) {
    static mut next_busno: i32 = 0;
    static mut need_domain_info: i32 = 0;
    let mut resources = ListHead::new();
    let mut bus: *mut PciBus;
    let bridge: *mut PciHostBridge;
    let ret: i32;

    bridge = pci_alloc_host_bridge(0);
    if bridge.is_null() {
        return;
    }

    if !(*hose).get_busno.is_null() && pci_has_flag(PCI_PROBE_ONLY) != 0 {
        next_busno = (*(*hose).get_busno)();
    }

    pci_add_resource_offset(&mut resources, (*hose).mem_resource, (*hose).mem_offset);
    pci_add_resource_offset(&mut resources, (*hose).io_resource, (*hose).io_offset);
    list_splice_init(&mut resources, &mut (*bridge).windows);
    (*bridge).dev.parent = core::ptr::null_mut();
    (*bridge).sysdata = hose as *mut c_void;
    (*bridge).busnr = next_busno;
    (*bridge).ops = (*hose).pci_ops;
    (*bridge).swizzle_irq = Some(pci_common_swizzle);
    (*bridge).map_irq = Some(pcibios_map_irq);
    ret = pci_scan_root_bus_bridge(bridge);
    if ret != 0 {
        pci_free_host_bridge(bridge);
        return;
    }

    (*hose).bus = bus = (*bridge).bus;
    need_domain_info = (need_domain_info != 0 || pci_domain_nr(bus) != 0) as i32;
    set_pci_need_domain_info(hose, need_domain_info);

    next_busno = (*bus).busn_res.end + 1;
    // Don't allow 8-bit bus number overflow inside the hose - reserve some space for bridges.
    if next_busno > 224 {
        next_busno = 0;
        need_domain_info = 1;
    }

    // We insert PCI resources into the iomem_resource and ioport_resource trees
    // in either pci_bus_claim_resources() or pci_bus_assign_resources().
    if pci_has_flag(PCI_PROBE_ONLY) != 0 {
        pci_bus_claim_resources(bus);
    } else {
        pci_bus_size_bridges(bus);
        pci_bus_assign_resources(bus);
        let mut child: *mut PciBus;
        list_for_each_entry!(child, &(*bus).children, node, {
            pcie_bus_configure_settings(child);
        });
    }
    pci_bus_add_devices(bus);
}

// The CONFIG_OF conditional is retained as a Rust configuration condition.
#[cfg(CONFIG_OF)]
pub unsafe fn pci_load_of_ranges(hose: *mut PciController, node: *mut DeviceNode) {
    let mut range: OfPciRange = core::mem::zeroed();
    let mut parser: OfPciRangeParser = core::mem::zeroed();

    (*hose).of_node = node;

    if of_pci_range_parser_init(&mut parser, node) != 0 {
        return;
    }

    for_each_of_pci_range!(&mut parser, &mut range, {
        let mut res: *mut Resource = core::ptr::null_mut();

        match range.flags & IORESOURCE_TYPE_BITS {
            IORESOURCE_IO => {
                (*hose).io_map_base = ioremap(range.cpu_addr, range.size) as c_ulong;
                res = (*hose).io_resource;
            }
            IORESOURCE_MEM => {
                res = (*hose).mem_resource;
            }
            _ => {}
        }
        if !res.is_null() {
            (*res).name = (*node).full_name;
            (*res).flags = range.flags;
            (*res).start = range.cpu_addr;
            (*res).end = range.cpu_addr + range.size - 1;
            (*res).parent = core::ptr::null_mut();
            (*res).child = core::ptr::null_mut();
            (*res).sibling = core::ptr::null_mut();
        }
    });
}

#[cfg(CONFIG_OF)]
pub unsafe fn pcibios_get_phb_of_node(bus: *mut PciBus) -> *mut DeviceNode {
    let hose = (*bus).sysdata as *mut PciController;
    of_node_get((*hose).of_node)
}

static mut pci_scan_mutex: Mutex = Mutex::new();

pub unsafe fn register_pci_controller(hose: *mut PciController) {
    let mut parent: *mut Resource;

    parent = (*(*hose).mem_resource).parent;
    if parent.is_null() {
        parent = &raw mut iomem_resource;
    }
    if request_resource(parent, (*hose).mem_resource) < 0 {
        goto_out();
        return;
    }

    parent = (*(*hose).io_resource).parent;
    if parent.is_null() {
        parent = &raw mut ioport_resource;
    }
    if request_resource(parent, (*hose).io_resource) < 0 {
        release_resource((*hose).mem_resource);
        goto_out();
        return;
    }

    INIT_LIST_HEAD!(&mut (*hose).list);
    list_add_tail!(&mut (*hose).list, &raw mut controllers);

    // Do not panic here but later - this might happen before console init.
    if (*hose).io_map_base == 0 {
        printk!(KERN_WARNING, "registering PCI controller with io_map_base unset\n");
    }

    // Scan the bus if it is register after the PCI subsystem initialization.
    if pci_initialized != 0 {
        mutex_lock(&raw mut pci_scan_mutex);
        pcibios_scanbus(hose);
        mutex_unlock(&raw mut pci_scan_mutex);
    }
    return;

    unsafe fn goto_out() {
        printk!(KERN_WARNING, "Skipping PCI bus scan due to resource conflict\n");
    }
}

#[init]
unsafe fn pcibios_init() -> i32 {
    let mut hose: *mut PciController;

    // Scan all of the recorded PCI controllers.
    list_for_each_entry!(hose, &raw mut controllers, list, {
        pcibios_scanbus(hose);
    });

    pci_initialized = 1;
    0
}

subsys_initcall!(pcibios_init);

pub unsafe fn pcibios_enable_device(dev: *mut PciDev, mask: i32) -> i32 {
    let err = pci_enable_resources(dev, mask);
    if err < 0 {
        return err;
    }
    pcibios_plat_dev_init(dev)
}

pub unsafe fn pcibios_fixup_bus(bus: *mut PciBus) {
    let dev = (*bus).self_;

    if pci_has_flag(PCI_PROBE_ONLY) != 0
        && !dev.is_null()
        && ((*dev).class >> 8) == PCI_CLASS_BRIDGE_PCI
    {
        pci_read_bridge_bases(bus);
    }
}

pub static mut pcibios_plat_setup: Option<unsafe extern "C" fn(*mut c_char) -> *mut c_char> = None;

pub unsafe fn pcibios_setup(str_: *mut c_char) -> *mut c_char {
    if let Some(setup) = pcibios_plat_setup {
        return setup(str_);
    }
    str_
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
