// SPDX-License-Identifier: GPL-2.0-only
/*
 * arch/arm/mach-dove/pcie.c
 *
 * PCIe functions for Marvell Dove 88AP510 SoC
 */

// Linux and platform dependencies are supplied by the surrounding kernel bindings.

#[repr(C)]
struct PciePort {
    index: u8,
    root_bus_nr: u8,
    base: *mut core::ffi::c_void,
    conf_lock: SpinlockT,
    mem_space_name: [core::ffi::c_char; 16],
    res: Resource,
}

static mut PCIE_PORT: [PciePort; 2] = [/* dependency-provided zero value */ unsafe { core::mem::zeroed() }, unsafe { core::mem::zeroed() }];
static mut NUM_PCIE_PORTS: i32 = 0;

unsafe fn dove_pcie_setup(nr: i32, sys: *mut PciSysData) -> i32 {
    let pp: *mut PciePort;
    let mut realio: Resource = core::mem::zeroed();

    if nr >= NUM_PCIE_PORTS { return 0; }

    pp = &mut PCIE_PORT[nr as usize];
    (*sys).private_data = pp as *mut core::ffi::c_void;
    (*pp).root_bus_nr = (*sys).busnr as u8;

    // Generic PCIe unit setup.
    orion_pcie_set_local_bus_nr((*pp).base, (*sys).busnr);
    orion_pcie_setup((*pp).base);

    realio.start = ((*sys).busnr as u64).wrapping_mul(SZ_64K as u64);
    realio.end = realio.start.wrapping_add(SZ_64K as u64).wrapping_sub(1);
    pci_remap_iospace(&mut realio, if (*pp).index == 0 { DOVE_PCIE0_IO_PHYS_BASE } else { DOVE_PCIE1_IO_PHYS_BASE });

    // IORESOURCE_MEM
    snprintf((*pp).mem_space_name.as_mut_ptr(), 16, b"PCIe %d MEM\0".as_ptr() as *const i8, (*pp).index as i32);
    (*pp).mem_space_name[15] = 0;
    (*pp).res.name = (*pp).mem_space_name.as_mut_ptr();
    if (*pp).index == 0 {
        (*pp).res.start = DOVE_PCIE0_MEM_PHYS_BASE;
        (*pp).res.end = (*pp).res.start.wrapping_add(DOVE_PCIE0_MEM_SIZE).wrapping_sub(1);
    } else {
        (*pp).res.start = DOVE_PCIE1_MEM_PHYS_BASE;
        (*pp).res.end = (*pp).res.start.wrapping_add(DOVE_PCIE1_MEM_SIZE).wrapping_sub(1);
    }
    (*pp).res.flags = IORESOURCE_MEM;
    if request_resource(&mut iomem_resource, &mut (*pp).res) != 0 { panic!("Request PCIe Memory resource failed\n"); }
    pci_add_resource_offset(&mut (*sys).resources, &mut (*pp).res, (*sys).mem_offset);
    1
}

unsafe fn pcie_valid_config(pp: *mut PciePort, bus: i32, dev: i32) -> i32 {
    // Don't go out when trying to access nonexisting devices on the local bus.
    if bus == (*pp).root_bus_nr as i32 && dev > 1 { return 0; }
    1
}

unsafe fn pcie_rd_conf(bus: *mut PciBus, devfn: u32, where_: i32, size: i32, val: *mut u32) -> i32 {
    let sys = (*bus).sysdata as *mut PciSysData;
    let pp = (*sys).private_data as *mut PciePort;
    let mut flags: u64 = 0;
    let ret: i32;
    if pcie_valid_config(pp, (*bus).number, PCI_SLOT(devfn)) == 0 {
        *val = 0xffff_ffff; return PCIBIOS_DEVICE_NOT_FOUND;
    }
    spin_lock_irqsave(&mut (*pp).conf_lock, &mut flags);
    ret = orion_pcie_rd_conf((*pp).base, bus, devfn, where_, size, val);
    spin_unlock_irqrestore(&mut (*pp).conf_lock, flags);
    ret
}

unsafe fn pcie_wr_conf(bus: *mut PciBus, devfn: u32, where_: i32, size: i32, val: u32) -> i32 {
    let sys = (*bus).sysdata as *mut PciSysData;
    let pp = (*sys).private_data as *mut PciePort;
    let mut flags: u64 = 0;
    if pcie_valid_config(pp, (*bus).number, PCI_SLOT(devfn)) == 0 { return PCIBIOS_DEVICE_NOT_FOUND; }
    spin_lock_irqsave(&mut (*pp).conf_lock, &mut flags);
    let ret = orion_pcie_wr_conf((*pp).base, bus, devfn, where_, size, val);
    spin_unlock_irqrestore(&mut (*pp).conf_lock, flags);
    ret
}

static mut PCIE_OPS: PciOps = PciOps { read: Some(pcie_rd_conf), write: Some(pcie_wr_conf) };

// The root complex has a hardwired class of PCI_CLASS_MEMORY_OTHER; switch it
// to PCI_CLASS_BRIDGE_HOST so Linux does not process the device BARs.
unsafe fn rc_pci_fixup(dev: *mut PciDev) {
    if (*(*dev).bus).parent.is_null() && (*dev).devfn == 0 {
        (*dev).class &= 0xff;
        (*dev).class |= PCI_CLASS_BRIDGE_HOST << 8;
        let mut r: *mut Resource = core::ptr::null_mut();
        pci_dev_for_each_resource(dev, &mut r);
        while !r.is_null() {
            (*r).start = 0; (*r).end = 0; (*r).flags = 0;
            pci_dev_for_each_resource(dev, &mut r);
        }
    }
}

// DECLARE_PCI_FIXUP_HEADER(PCI_VENDOR_ID_MARVELL, PCI_ANY_ID, rc_pci_fixup);

unsafe fn dove_pcie_scan_bus(nr: i32, bridge: *mut PciHostBridge) -> i32 {
    let sys = pci_host_bridge_priv(bridge);
    if nr >= NUM_PCIE_PORTS { BUG!(); return -EINVAL; }
    list_splice_init(&mut (*sys).resources, &mut (*bridge).windows);
    (*bridge).dev.parent = core::ptr::null_mut();
    (*bridge).sysdata = sys as *mut core::ffi::c_void;
    (*bridge).busnr = (*sys).busnr;
    (*bridge).ops = &mut PCIE_OPS;
    pci_scan_root_bus_bridge(bridge)
}

unsafe fn dove_pcie_map_irq(dev: *const PciDev, _slot: u8, _pin: u8) -> i32 {
    let sys = (*dev).sysdata as *mut PciSysData;
    let pp = (*sys).private_data as *mut PciePort;
    if (*pp).index != 0 { IRQ_DOVE_PCIE1 } else { IRQ_DOVE_PCIE0 }
}

static mut DOVE_PCI: HwPci = HwPci { nr_controllers: 2, setup: Some(dove_pcie_setup), scan: Some(dove_pcie_scan_bus), map_irq: Some(dove_pcie_map_irq) };

unsafe fn add_pcie_port(index: i32, base: *mut core::ffi::c_void) {
    printk(KERN_INFO, b"Dove PCIe port %d: \0".as_ptr(), index);
    if orion_pcie_link_up(base) {
        let pp = &mut PCIE_PORT[NUM_PCIE_PORTS as usize]; NUM_PCIE_PORTS += 1;
        let clk = clk_get_sys(b"pcie\0".as_ptr(), if index != 0 { b"1\0".as_ptr() } else { b"0\0".as_ptr() });
        if !IS_ERR(clk) { clk_prepare_enable(clk); }
        printk(KERN_INFO, b"link up\n\0".as_ptr());
        (*pp).index = index as u8; (*pp).root_bus_nr = 255; (*pp).base = base;
        spin_lock_init(&mut (*pp).conf_lock); memset(&mut (*pp).res, 0, core::mem::size_of::<Resource>());
    } else { printk(KERN_INFO, b"link down, ignoring\n\0".as_ptr()); }
}

pub unsafe fn dove_pcie_init(init_port0: i32, init_port1: i32) {
    vga_base = DOVE_PCIE0_MEM_PHYS_BASE;
    if init_port0 != 0 { add_pcie_port(0, DOVE_PCIE0_VIRT_BASE); }
    if init_port1 != 0 { add_pcie_port(1, DOVE_PCIE1_VIRT_BASE); }
    pci_common_init(&mut DOVE_PCI);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
