// SPDX-License-Identifier: GPL-2.0-only
/*
 *  Ralink RT288x SoC PCI register definitions
 *
 *  Copyright (C) 2009 John Crispin <john@phrozen.org>
 *  Copyright (C) 2009 Gabor Juhos <juhosg@openwrt.org>
 *
 *  Parts of this file are based on Ralink's 2.6.21 BSP
 */

// Linux kernel dependencies supplied by other files.

const RT2880_PCI_BASE: usize = 0x00440000;
const RT288X_CPU_IRQ_PCI: i32 = 4;

const RT2880_PCI_MEM_BASE: u32 = 0x20000000;
const RT2880_PCI_MEM_SIZE: u32 = 0x10000000;
const RT2880_PCI_IO_BASE: u32 = 0x00460000;
const RT2880_PCI_IO_SIZE: u32 = 0x00010000;

const RT2880_PCI_REG_PCICFG_ADDR: u32 = 0x00;
const RT2880_PCI_REG_PCIMSK_ADDR: u32 = 0x0c;
const RT2880_PCI_REG_BAR0SETUP_ADDR: u32 = 0x10;
const RT2880_PCI_REG_IMBASEBAR0_ADDR: u32 = 0x18;
const RT2880_PCI_REG_CONFIG_ADDR: u32 = 0x20;
const RT2880_PCI_REG_CONFIG_DATA: u32 = 0x24;
const RT2880_PCI_REG_MEMBASE: u32 = 0x28;
const RT2880_PCI_REG_IOBASE: u32 = 0x2c;
const RT2880_PCI_REG_ID: u32 = 0x30;
const RT2880_PCI_REG_CLASS: u32 = 0x34;
const RT2880_PCI_REG_SUBID: u32 = 0x38;
const RT2880_PCI_REG_ARBCTL: u32 = 0x80;

type U32 = u32;

#[repr(C)]
pub struct PciBus {
    pub number: u32,
}

#[repr(C)]
pub struct PciDev {
    pub bus: *mut PciBus,
    pub devfn: u32,
}

#[repr(C)]
pub struct PlatformDevice {
    pub dev: Device,
}

#[repr(C)]
pub struct Device {
    pub of_node: *mut core::ffi::c_void,
}

#[repr(C)]
pub struct PciOps {
    pub read: Option<unsafe extern "C" fn(*mut PciBus, u32, i32, i32, *mut u32) -> i32>,
    pub write: Option<unsafe extern "C" fn(*mut PciBus, u32, i32, i32, u32) -> i32>,
}

#[repr(C)]
pub struct Resource {
    pub name: *const u8,
    pub start: u32,
    pub end: u32,
    pub flags: u32,
}

#[repr(C)]
pub struct PciController {
    pub pci_ops: *mut PciOps,
    pub mem_resource: *mut Resource,
    pub io_resource: *mut Resource,
    pub io_map_base: usize,
    pub of_node: *mut core::ffi::c_void,
}

#[repr(C)]
pub struct OfDeviceId {
    pub compatible: *const u8,
}

#[repr(C)]
pub struct PlatformDriver {
    pub probe: Option<unsafe extern "C" fn(*mut PlatformDevice) -> i32>,
    pub driver: Driver,
}

#[repr(C)]
pub struct Driver {
    pub name: *const u8,
    pub of_match_table: *const OfDeviceId,
}

extern "C" {
    fn readl(addr: *const core::ffi::c_void) -> u32;
    fn writel(value: u32, addr: *mut core::ffi::c_void);
    fn ioremap(addr: usize, size: usize) -> *mut core::ffi::c_void;
    fn udelay(usecs: u32);
    fn set_io_port_base(base: usize);
    fn register_pci_controller(controller: *mut PciController);
    fn platform_driver_register(driver: *mut PlatformDriver) -> i32;
    fn pci_bus_write_config_dword(bus: *mut PciBus, devfn: u32, where_: u32, val: u32) -> i32;
    fn pci_bus_read_config_dword(bus: *mut PciBus, devfn: u32, where_: u32, val: *mut u32) -> i32;
    fn pci_bus_read_config_word(bus: *mut PciBus, devfn: u32, where_: u32, val: *mut u16) -> i32;
    fn pci_bus_write_config_word(bus: *mut PciBus, devfn: u32, where_: u32, val: u16) -> i32;
    static mut ioport_resource: Resource;
}

static mut rt2880_pci_base: *mut core::ffi::c_void = core::ptr::null_mut();

unsafe fn rt2880_pci_reg_read(reg: u32) -> u32 {
    readl(rt2880_pci_base.add(reg as usize))
}

unsafe fn rt2880_pci_reg_write(val: u32, reg: u32) {
    writel(val, rt2880_pci_base.add(reg as usize));
}

#[inline]
unsafe fn rt2880_pci_get_cfgaddr(bus: u32, slot: u32, func: u32, where_: u32) -> u32 {
    (bus << 16) | (slot << 11) | (func << 8) | (where_ & 0xfc) | 0x80000000
}

unsafe extern "C" fn rt2880_pci_config_read(bus: *mut PciBus, devfn: u32, where_: i32, size: i32, val: *mut u32) -> i32 {
    let address = rt2880_pci_get_cfgaddr((*bus).number, (devfn >> 3) & 0x1f, devfn & 7, where_ as u32);
    rt2880_pci_reg_write(address, RT2880_PCI_REG_CONFIG_ADDR);
    let data = rt2880_pci_reg_read(RT2880_PCI_REG_CONFIG_DATA);
    match size {
        1 => *val = (data >> (((where_ as u32) & 3) << 3)) & 0xff,
        2 => *val = (data >> (((where_ as u32) & 3) << 3)) & 0xffff,
        4 => *val = data,
        _ => (),
    }
    0
}

unsafe extern "C" fn rt2880_pci_config_write(bus: *mut PciBus, devfn: u32, where_: i32, size: i32, val: u32) -> i32 {
    let address = rt2880_pci_get_cfgaddr((*bus).number, (devfn >> 3) & 0x1f, devfn & 7, where_ as u32);
    rt2880_pci_reg_write(address, RT2880_PCI_REG_CONFIG_ADDR);
    let mut data = rt2880_pci_reg_read(RT2880_PCI_REG_CONFIG_DATA);
    let shift = ((where_ as u32) & 3) << 3;
    match size {
        1 => data = (data & !(0xff << shift)) | (val << shift),
        2 => data = (data & !(0xffff << shift)) | (val << shift),
        4 => data = val,
        _ => (),
    }
    rt2880_pci_reg_write(data, RT2880_PCI_REG_CONFIG_DATA);
    0
}

static mut rt2880_pci_ops: PciOps = PciOps { read: Some(rt2880_pci_config_read), write: Some(rt2880_pci_config_write) };

static mut rt2880_pci_mem_resource: Resource = Resource { name: b"PCI MEM space\0".as_ptr(), start: RT2880_PCI_MEM_BASE, end: RT2880_PCI_MEM_BASE + RT2880_PCI_MEM_SIZE - 1, flags: 0x00000200 };
static mut rt2880_pci_io_resource: Resource = Resource { name: b"PCI IO space\0".as_ptr(), start: RT2880_PCI_IO_BASE, end: RT2880_PCI_IO_BASE + RT2880_PCI_IO_SIZE - 1, flags: 0x00000100 };
static mut rt2880_pci_controller: PciController = PciController { pci_ops: &raw mut rt2880_pci_ops, mem_resource: &raw mut rt2880_pci_mem_resource, io_resource: &raw mut rt2880_pci_io_resource, io_map_base: 0, of_node: core::ptr::null_mut() };

unsafe fn rt2880_pci_read_u32(reg: usize) -> u32 {
    let address = rt2880_pci_get_cfgaddr(0, 0, 0, reg as u32);
    rt2880_pci_reg_write(address, RT2880_PCI_REG_CONFIG_ADDR);
    rt2880_pci_reg_read(RT2880_PCI_REG_CONFIG_DATA)
}

unsafe fn rt2880_pci_write_u32(reg: usize, val: u32) {
    let address = rt2880_pci_get_cfgaddr(0, 0, 0, reg as u32);
    rt2880_pci_reg_write(address, RT2880_PCI_REG_CONFIG_ADDR);
    rt2880_pci_reg_write(val, RT2880_PCI_REG_CONFIG_DATA);
}

#[no_mangle]
pub unsafe extern "C" fn pcibios_map_irq(dev: *const PciDev, _slot: u8, _pin: u8) -> i32 {
    let mut irq = -1;
    if (*(*dev).bus).number != 0 { return irq; }
    match ((*dev).devfn >> 3) & 0x1f {
        0x00 => (),
        0x11 => irq = RT288X_CPU_IRQ_PCI,
        _ => panic!("trying to alloc unknown pci irq"),
    }
    irq
}

unsafe extern "C" fn rt288x_pci_probe(pdev: *mut PlatformDevice) -> i32 {
    rt2880_pci_base = ioremap(RT2880_PCI_BASE, 4096);
    let io_map_base = ioremap(RT2880_PCI_IO_BASE as usize, RT2880_PCI_IO_SIZE as usize);
    rt2880_pci_controller.io_map_base = io_map_base as usize;
    set_io_port_base(io_map_base as usize);
    ioport_resource.start = RT2880_PCI_IO_BASE;
    ioport_resource.end = RT2880_PCI_IO_BASE + RT2880_PCI_IO_SIZE - 1;
    rt2880_pci_reg_write(0, RT2880_PCI_REG_PCICFG_ADDR); udelay(1);
    rt2880_pci_reg_write(0x79, RT2880_PCI_REG_ARBCTL);
    rt2880_pci_reg_write(0x07FF0001, RT2880_PCI_REG_BAR0SETUP_ADDR);
    rt2880_pci_reg_write(RT2880_PCI_MEM_BASE, RT2880_PCI_REG_MEMBASE);
    rt2880_pci_reg_write(RT2880_PCI_IO_BASE, RT2880_PCI_REG_IOBASE);
    rt2880_pci_reg_write(0x08000000, RT2880_PCI_REG_IMBASEBAR0_ADDR);
    rt2880_pci_reg_write(0x08021814, RT2880_PCI_REG_ID);
    rt2880_pci_reg_write(0x00800001, RT2880_PCI_REG_CLASS);
    rt2880_pci_reg_write(0x28801814, RT2880_PCI_REG_SUBID);
    rt2880_pci_reg_write(0x000c0000, RT2880_PCI_REG_PCIMSK_ADDR);
    rt2880_pci_write_u32(0x10, 0x08000000); let _ = rt2880_pci_read_u32(0x10);
    rt2880_pci_controller.of_node = (*pdev).dev.of_node;
    register_pci_controller(&raw mut rt2880_pci_controller); 0
}

#[no_mangle]
pub unsafe extern "C" fn pcibios_plat_dev_init(dev: *mut PciDev) -> i32 {
    static mut slot0_init: bool = false;
    if !slot0_init && (*(*dev).bus).number == 0 {
        let mut cmd: u16 = 0; let mut bar0: u32 = 0; slot0_init = true;
        pci_bus_write_config_dword((*dev).bus, 0, 0x10, 0x08000000);
        pci_bus_read_config_dword((*dev).bus, 0, 0x10, &mut bar0);
        pci_bus_read_config_word((*dev).bus, 0, 0x04, &mut cmd);
        cmd |= 0x0007;
        pci_bus_write_config_word((*dev).bus, 0, 0x04, cmd);
    }
    0
}

static rt288x_pci_match: [OfDeviceId; 2] = [OfDeviceId { compatible: b"ralink,rt288x-pci\0".as_ptr() }, OfDeviceId { compatible: core::ptr::null() }];
static mut rt288x_pci_driver: PlatformDriver = PlatformDriver { probe: Some(rt288x_pci_probe), driver: Driver { name: b"rt288x-pci\0".as_ptr(), of_match_table: rt288x_pci_match.as_ptr() } };

#[allow(dead_code)]
unsafe fn pcibios_init() -> i32 {
    let ret = platform_driver_register(&raw mut rt288x_pci_driver);
    if ret != 0 { /* pr_info("rt288x-pci: Error registering platform driver!") */ }
    ret
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
