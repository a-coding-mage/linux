// SPDX-License-Identifier: GPL-2.0-only
/*
 * arch/arm/mach-mv78xx0/pcie.c
 *
 * PCIe functions for Marvell MV78xx0 SoCs
 */

// C headers and symbols supplied by the kernel and platform dependencies are
// intentionally left as external Rust dependencies.

const fn mv78xx0_mbus_pcie_mem_target(port: i32, _lane: i32) -> u32 { if port != 0 { 8 } else { 4 } }
const fn mv78xx0_mbus_pcie_mem_attr(_port: i32, lane: i32) -> u32 { 0xf8 & !(0x10 << lane) }
const fn mv78xx0_mbus_pcie_io_target(port: i32, _lane: i32) -> u32 { if port != 0 { 8 } else { 4 } }
const fn mv78xx0_mbus_pcie_io_attr(_port: i32, lane: i32) -> u32 { 0xf0 & !(0x10 << lane) }

#[repr(C)]
pub struct PciePort {
    pub maj: u8,
    pub min: u8,
    pub root_bus_nr: u8,
    pub base: *mut core::ffi::c_void,
    pub conf_lock: Spinlock,
    pub mem_space_name: [i8; 20],
    pub res: Resource,
}

static mut PCIE_PORT: [PciePort; 8] = unsafe { core::mem::zeroed() };
static mut NUM_PCIE_PORTS: i32 = 0;
static mut PCIE_IO_SPACE: Resource = unsafe { core::mem::zeroed() };

pub unsafe extern "C" fn mv78xx0_pcie_id(dev: *mut u32, rev: *mut u32) {
    *dev = orion_pcie_dev_id(PCIE00_VIRT_BASE);
    *rev = orion_pcie_rev(PCIE00_VIRT_BASE);
}

#[no_mangle]
pub static mut pcie_port_size: [u32; 8] = [
    0, 0x20000000, 0x10000000, 0x10000000,
    0x08000000, 0x08000000, 0x08000000, 0x04000000,
];

unsafe extern "C" fn mv78xx0_pcie_preinit() {
    let mut size_each: u32;
    let mut start: u32;

    PCIE_IO_SPACE.name = b"PCIe I/O Space\0".as_ptr() as *mut i8;
    PCIE_IO_SPACE.start = MV78XX0_PCIE_IO_PHYS_BASE(0);
    PCIE_IO_SPACE.end = MV78XX0_PCIE_IO_PHYS_BASE(0) + MV78XX0_PCIE_IO_SIZE * 8 - 1;
    PCIE_IO_SPACE.flags = IORESOURCE_MEM;
    if request_resource(&mut iomem_resource, &mut PCIE_IO_SPACE) != 0 { panic!("can't allocate PCIe I/O space"); }

    if NUM_PCIE_PORTS > 7 { panic!("invalid number of PCIe ports"); }
    size_each = pcie_port_size[NUM_PCIE_PORTS as usize];
    start = MV78XX0_PCIE_MEM_PHYS_BASE;
    let mut i = 0;
    while i < NUM_PCIE_PORTS {
        let pp = &mut PCIE_PORT[i as usize];
        snprintf(pp.mem_space_name.as_mut_ptr(), core::mem::size_of_val(&pp.mem_space_name), b"PCIe %d.%d MEM\0".as_ptr(), pp.maj, pp.min);
        pp.mem_space_name[19] = 0;
        pp.res.name = pp.mem_space_name.as_mut_ptr();
        pp.res.flags = IORESOURCE_MEM;
        pp.res.start = start;
        pp.res.end = start + size_each - 1;
        start += size_each;
        if request_resource(&mut iomem_resource, &mut pp.res) != 0 { panic!("can't allocate PCIe MEM sub-space"); }
        mvebu_mbus_add_window_by_id(mv78xx0_mbus_pcie_mem_target(pp.maj as i32, pp.min as i32), mv78xx0_mbus_pcie_mem_attr(pp.maj as i32, pp.min as i32), pp.res.start, resource_size(&pp.res));
        mvebu_mbus_add_window_remap_by_id(mv78xx0_mbus_pcie_io_target(pp.maj as i32, pp.min as i32), mv78xx0_mbus_pcie_io_attr(pp.maj as i32, pp.min as i32), i as u32 * SZ_64K, SZ_64K, 0);
        i += 1;
    }
}

unsafe extern "C" fn mv78xx0_pcie_setup(nr: i32, sys: *mut PciSysData) -> i32 {
    if nr >= NUM_PCIE_PORTS { return 0; }
    let pp = &mut PCIE_PORT[nr as usize];
    (*sys).private_data = pp as *mut _ as *mut core::ffi::c_void;
    pp.root_bus_nr = (*sys).busnr as u8;
    orion_pcie_set_local_bus_nr(pp.base, (*sys).busnr);
    orion_pcie_setup(pp.base);
    let mut realio: Resource = core::mem::zeroed();
    realio.start = nr as u32 * SZ_64K;
    realio.end = realio.start + SZ_64K - 1;
    pci_remap_iospace(&mut realio, MV78XX0_PCIE_IO_PHYS_BASE(nr));
    pci_add_resource_offset(&mut (*sys).resources, &mut pp.res, (*sys).mem_offset);
    1
}

unsafe fn pcie_valid_config(pp: *mut PciePort, bus: i32, dev: i32) -> i32 {
    if bus == (*pp).root_bus_nr as i32 && dev > 1 { return 0; }
    1
}

unsafe extern "C" fn pcie_rd_conf(bus: *mut PciBus, devfn: u32, where_: i32, size: i32, val: *mut u32) -> i32 {
    let sys = (*bus).sysdata;
    let pp = (*sys).private_data as *mut PciePort;
    if pcie_valid_config(pp, (*bus).number, pci_slot(devfn)) == 0 { *val = 0xffff_ffff; return PCIBIOS_DEVICE_NOT_FOUND; }
    let mut flags = 0ul;
    spin_lock_irqsave(&mut (*pp).conf_lock, &mut flags);
    let ret = orion_pcie_rd_conf((*pp).base, bus, devfn, where_, size, val);
    spin_unlock_irqrestore(&mut (*pp).conf_lock, flags);
    ret
}

unsafe extern "C" fn pcie_wr_conf(bus: *mut PciBus, devfn: u32, where_: i32, size: i32, val: u32) -> i32 {
    let sys = (*bus).sysdata;
    let pp = (*sys).private_data as *mut PciePort;
    if pcie_valid_config(pp, (*bus).number, pci_slot(devfn)) == 0 { return PCIBIOS_DEVICE_NOT_FOUND; }
    let mut flags = 0ul;
    spin_lock_irqsave(&mut (*pp).conf_lock, &mut flags);
    let ret = orion_pcie_wr_conf((*pp).base, bus, devfn, where_, size, val);
    spin_unlock_irqrestore(&mut (*pp).conf_lock, flags);
    ret
}

static mut PCIE_OPS: PciOps = PciOps { read: Some(pcie_rd_conf), write: Some(pcie_wr_conf) };

/* The root complex class is hardwired to PCI_CLASS_MEMORY_OTHER and must be
 * changed to PCI_CLASS_BRIDGE_HOST while operating as a root complex. */
unsafe extern "C" fn rc_pci_fixup(dev: *mut PciDev) {
    if (*(*dev).bus).parent.is_null() && (*dev).devfn == 0 {
        (*dev).class &= 0xff;
        (*dev).class |= PCI_CLASS_BRIDGE_HOST << 8;
        let mut r: *mut Resource = core::ptr::null_mut();
        while pci_dev_for_each_resource(dev, &mut r) {
            (*r).start = 0; (*r).end = 0; (*r).flags = 0;
        }
    }
}

unsafe extern "C" fn mv78xx0_pcie_scan_bus(nr: i32, bridge: *mut PciHostBridge) -> i32 {
    let sys = pci_host_bridge_priv(bridge);
    if nr >= NUM_PCIE_PORTS { BUG(); return -EINVAL; }
    list_splice_init(&mut (*sys).resources, &mut (*bridge).windows);
    (*bridge).dev.parent = core::ptr::null_mut();
    (*bridge).sysdata = sys as *mut _ as *mut core::ffi::c_void;
    (*bridge).busnr = (*sys).busnr;
    (*bridge).ops = &mut PCIE_OPS;
    pci_scan_root_bus_bridge(bridge)
}

unsafe extern "C" fn mv78xx0_pcie_map_irq(dev: *const PciDev, _slot: u8, _pin: u8) -> i32 {
    let sys = (*(*dev).bus).sysdata;
    let pp = (*sys).private_data as *mut PciePort;
    IRQ_MV78XX0_PCIE_00 + ((*pp).maj as i32 << 2) + (*pp).min as i32
}

static mut MV78XX0_PCI: HwPci = HwPci { nr_controllers: 8, preinit: Some(mv78xx0_pcie_preinit), setup: Some(mv78xx0_pcie_setup), scan: Some(mv78xx0_pcie_scan_bus), map_irq: Some(mv78xx0_pcie_map_irq) };

unsafe fn add_pcie_port(maj: i32, min: i32, base: *mut core::ffi::c_void) {
    printk(KERN_INFO, b"MV78xx0 PCIe port %d.%d: \0".as_ptr(), maj, min);
    if orion_pcie_link_up(base) {
        let pp = &mut PCIE_PORT[NUM_PCIE_PORTS as usize]; NUM_PCIE_PORTS += 1;
        printk(b"link up\n\0".as_ptr()); pp.maj = maj as u8; pp.min = min as u8; pp.root_bus_nr = 255; pp.base = base; spin_lock_init(&mut pp.conf_lock); core::ptr::write_bytes(&mut pp.res as *mut Resource, 0, 1);
    } else { printk(b"link down, ignoring\n\0".as_ptr()); }
}

pub unsafe extern "C" fn mv78xx0_pcie_init(init_port0: i32, init_port1: i32) {
    vga_base = MV78XX0_PCIE_MEM_PHYS_BASE;
    if init_port0 != 0 { add_pcie_port(0, 0, PCIE00_VIRT_BASE); if orion_pcie_x4_mode(PCIE00_VIRT_BASE) == 0 { add_pcie_port(0, 1, PCIE01_VIRT_BASE); add_pcie_port(0, 2, PCIE02_VIRT_BASE); add_pcie_port(0, 3, PCIE03_VIRT_BASE); } }
    if init_port1 != 0 { add_pcie_port(1, 0, PCIE10_VIRT_BASE); if orion_pcie_x4_mode(PCIE10_VIRT_BASE) == 0 { add_pcie_port(1, 1, PCIE11_VIRT_BASE); add_pcie_port(1, 2, PCIE12_VIRT_BASE); add_pcie_port(1, 3, PCIE13_VIRT_BASE); } }
    pci_common_init(&mut MV78XX0_PCI);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
