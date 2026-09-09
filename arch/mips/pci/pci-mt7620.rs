// SPDX-License-Identifier: GPL-2.0-only
/*
 *  Ralink MT7620A SoC PCI support
 *
 *  Copyright (C) 2007-2013 Bruce Chang (Mediatek)
 *  Copyright (C) 2013-2016 John Crispin <john@phrozen.org>
 */

// Linux kernel dependencies supplied by other translation units.
use core::ffi::c_void;

type U8 = u8;
type U16 = u16;
type U32 = u32;
type CInt = i32;

#[repr(C)] pub struct PciBus { pub number: U8 }
#[repr(C)] pub struct PciDev { pub bus: *mut PciBus, pub irq: CInt }
#[repr(C)] pub struct PlatformDevice { pub dev: Device }
#[repr(C)] pub struct Device;
#[repr(C)] pub struct ResetControl;
#[repr(C)] pub struct Resource { pub start: usize, pub end: usize }
#[repr(C)] pub struct PciOps {
    pub read: Option<unsafe extern "C" fn(*mut PciBus, u32, CInt, CInt, *mut U32) -> CInt>,
    pub write: Option<unsafe extern "C" fn(*mut PciBus, u32, CInt, CInt, U32) -> CInt>,
}
#[repr(C)] pub struct PciController {
    pub pci_ops: *const PciOps,
    pub mem_resource: *mut Resource,
    pub mem_offset: usize,
    pub io_resource: *mut Resource,
    pub io_offset: usize,
    pub io_map_base: usize,
}
#[repr(C)] pub struct OfDeviceId { pub compatible: *const u8 }
#[repr(C)] pub struct PlatformDriver { pub probe: Option<unsafe extern "C" fn(*mut PlatformDevice) -> CInt>, pub name: *const u8, pub of_match_table: *const OfDeviceId }

extern "C" {
    static mut iomem_resource: Resource;
    static mut ioport_resource: Resource;
    static mut ralink_soc: CInt;
    static mut rstpcie0: *mut ResetControl;
    fn iowrite32(val: U32, addr: *mut c_void);
    fn ioread32(addr: *mut c_void) -> U32;
    fn mdelay(ms: u32);
    fn msleep(ms: u32);
    fn pr_warn(msg: *const u8);
    fn rt_sysc_r32(reg: u32) -> U32;
    fn rt_sysc_m32(clr: U32, set: U32, reg: u32);
    fn reset_control_assert(rst: *mut ResetControl);
    fn reset_control_deassert(rst: *mut ResetControl);
    fn devm_reset_control_get_exclusive(dev: *mut Device, id: *const u8) -> *mut ResetControl;
    fn devm_platform_get_and_ioremap_resource(pdev: *mut PlatformDevice, index: u32, res: *mut *mut Resource) -> *mut c_void;
    fn pci_load_of_ranges(ctrl: *mut PciController, node: *mut c_void);
    fn register_pci_controller(ctrl: *mut PciController);
    fn platform_driver_register(driver: *mut PlatformDriver) -> CInt;
    fn dev_err(dev: *const Device, fmt: *const u8, ...);
    fn dev_info(dev: *const Device, fmt: *const u8, ...);
    fn pci_write_config_byte(dev: *const PciDev, where_: CInt, val: U8);
    fn pci_write_config_word(dev: *const PciDev, where_: CInt, val: U16);
    fn pci_read_config_word(dev: *const PciDev, where_: CInt, val: *mut U16);
}

const RALINK_PCI_IO_MAP_BASE: U32 = 0x10160000;
const RALINK_PCI_MEMORY_BASE: U32 = 0x0;
const RALINK_INT_PCIE0: CInt = 4;
const RALINK_SYSCFG0: u32 = 0x10;
const RALINK_SYSCFG0_XTAL40: U32 = 1 << 6;
const RALINK_CLKCFG1: u32 = 0x30;
const PPLL_CFG1: u32 = 0x9c;
const PPLL_LD: U32 = 1 << 23;
const PPLL_DRV: u32 = 0xa0;
const PDRV_SW_SET: U32 = 1 << 31;
const LC_CKDRVPD: U32 = 1 << 19;
const LC_CKDRVOHZ: U32 = 1 << 18;
const LC_CKDRVHZ: U32 = 1 << 17;
const RALINK_PCI_PCICFG_ADDR: u32 = 0x00;
const PCIRST: U32 = 1 << 1;
const RALINK_PCI_PCIENA: u32 = 0x0c;
const PCIINT2: U32 = 1 << 20;
const RALINK_PCI_CONFIG_ADDR: u32 = 0x20;
const RALINK_PCI_CONFIG_DATA_VIRT_REG: u32 = 0x24;
const RALINK_PCI_MEMBASE: u32 = 0x28;
const RALINK_PCI_IOBASE: u32 = 0x2c;
const RALINK_PCI0_BAR0SETUP_ADDR: u32 = 0x10;
const RALINK_PCI0_IMBASEBAR0_ADDR: u32 = 0x18;
const RALINK_PCI0_CLASS: u32 = 0x34;
const RALINK_PCI0_STATUS: u32 = 0x50;
const PCIE_LINK_UP_ST: U32 = 1;
const PCIEPHY0_CFG: u32 = 0x90;
const RALINK_PCIEPHY_P0_CTL_OFFSET: u32 = 0x7000;
const RALINK_PCIE0_CLK_EN: U32 = 1 << 26;
const BUSY: U32 = 0x80000000;
const WAITRETRY_MAX: usize = 10;
const WRITE_MODE: U32 = 1 << 23;

static mut bridge_base: *mut u8 = core::ptr::null_mut();
static mut pcie_base: *mut u8 = core::ptr::null_mut();

#[inline] unsafe fn bridge_w32(val: U32, reg: u32) { iowrite32(val, bridge_base.add(reg as usize) as *mut c_void); }
#[inline] unsafe fn bridge_r32(reg: u32) -> U32 { ioread32(bridge_base.add(reg as usize) as *mut c_void) }
#[inline] unsafe fn bridge_m32(clr: U32, set: U32, reg: u32) { let mut val = bridge_r32(reg); val &= !clr; val |= set; bridge_w32(val, reg); }
#[inline] unsafe fn pcie_w32(val: U32, reg: u32) { iowrite32(val, pcie_base.add(reg as usize) as *mut c_void); }
#[inline] unsafe fn pcie_r32(reg: u32) -> U32 { ioread32(pcie_base.add(reg as usize) as *mut c_void) }
#[inline] unsafe fn pcie_m32(clr: U32, set: U32, reg: u32) { let mut val = pcie_r32(reg); val &= !clr; val |= set; pcie_w32(val, reg); }

#[inline] unsafe fn pcie_phyctrl_set(offset: u32, b_start: U32, bits: U32, val: U32) {
    let mask = (((1u64 << bits) - 1) << b_start) as U32;
    pcie_m32(mask, val << b_start, RALINK_PCIEPHY_P0_CTL_OFFSET + offset);
}

unsafe fn wait_pciephy_busy() -> CInt {
    let mut retry: usize = 0;
    loop {
        if pcie_r32(PCIEPHY0_CFG) & BUSY != 0 { mdelay(100); }
        else { break; }
        if retry > WAITRETRY_MAX { pr_warn(b"PCIE-PHY retry failed.\0".as_ptr()); return -1; }
        retry += 1;
    }
    0
}

unsafe fn pcie_phy(addr: u32, val: u32) {
    wait_pciephy_busy();
    pcie_w32(WRITE_MODE | val | (addr << 8), PCIEPHY0_CFG);
    mdelay(1);
    wait_pciephy_busy();
}

unsafe fn pci_config_read(bus: *mut PciBus, devfn: u32, where_: CInt, size: CInt, val: *mut U32) -> CInt {
    let slot = (devfn >> 11) & 0x1f; let func = (devfn >> 8) & 7;
    let num = if bus.is_null() { 0 } else { (*bus).number as u32 };
    let address = (((where_ as u32 & 0xf00) >> 8) << 24) | (num << 16) | (slot << 11) | (func << 8) | (where_ as u32 & 0xfc) | 0x80000000;
    bridge_w32(address, RALINK_PCI_CONFIG_ADDR); let data = bridge_r32(RALINK_PCI_CONFIG_DATA_VIRT_REG);
    match size { 1 => *val = (data >> (((where_ as u32) & 3) << 3)) & 0xff, 2 => *val = (data >> (((where_ as u32) & 3) << 3)) & 0xffff, 4 => *val = data, _ => {} } 0
}

unsafe fn pci_config_write(bus: *mut PciBus, devfn: u32, where_: CInt, size: CInt, val: U32) -> CInt {
    let slot = (devfn >> 11) & 0x1f; let func = (devfn >> 8) & 7;
    let num = if bus.is_null() { 0 } else { (*bus).number as u32 };
    let address = (((where_ as u32 & 0xf00) >> 8) << 24) | (num << 16) | (slot << 11) | (func << 8) | (where_ as u32 & 0xfc) | 0x80000000;
    bridge_w32(address, RALINK_PCI_CONFIG_ADDR); let mut data = bridge_r32(RALINK_PCI_CONFIG_DATA_VIRT_REG); let shift = ((where_ as u32 & 3) << 3);
    match size { 1 => data = (data & !(0xff << shift)) | (val << shift), 2 => data = (data & !(0xffff << shift)) | (val << shift), 4 => data = val, _ => {} }
    bridge_w32(data, RALINK_PCI_CONFIG_DATA_VIRT_REG); 0
}

#[no_mangle] pub static mut mt7620_pci_ops: PciOps = PciOps { read: Some(pci_config_read), write: Some(pci_config_write) };
static mut mt7620_res_pci_mem1: Resource = Resource { start: 0, end: 0 };
static mut mt7620_res_pci_io1: Resource = Resource { start: 0, end: 0 };
#[no_mangle] pub static mut mt7620_controller: PciController = PciController { pci_ops: core::ptr::addr_of!(mt7620_pci_ops), mem_resource: core::ptr::addr_of_mut!(mt7620_res_pci_mem1), mem_offset: 0, io_resource: core::ptr::addr_of_mut!(mt7620_res_pci_io1), io_offset: 0, io_map_base: 0xa0000000 };

unsafe fn mt7620_pci_hw_init(pdev: *mut PlatformDevice) -> CInt {
    pcie_phy(0, 0x80); pcie_phy(1, 4); pcie_phy(0x68, 0xb4);
    if rt_sysc_r32(PPLL_CFG1) & PPLL_LD == 0 { dev_err(&(*pdev).dev, b"pcie PLL not locked, aborting init\n\0".as_ptr()); reset_control_assert(rstpcie0); rt_sysc_m32(RALINK_PCIE0_CLK_EN, 0, RALINK_CLKCFG1); return -1; }
    rt_sysc_m32(LC_CKDRVHZ | LC_CKDRVOHZ, LC_CKDRVPD | PDRV_SW_SET, PPLL_DRV); 0
}
unsafe fn mt7628_pci_hw_init(_: *mut PlatformDevice) {
    pcie_phyctrl_set(0x400,8,1,1); pcie_phyctrl_set(0x400,9,2,0); pcie_phyctrl_set(0,4,1,1); pcie_phyctrl_set(0,5,1,0); pcie_phyctrl_set(0x4ac,16,3,3);
    if rt_sysc_r32(RALINK_SYSCFG0) & RALINK_SYSCFG0_XTAL40 != 0 { pcie_phyctrl_set(0x4bc,24,8,0x7d); pcie_phyctrl_set(0x490,12,4,8); pcie_phyctrl_set(0x490,6,2,1); pcie_phyctrl_set(0x4c0,0,32,0x1f400000); pcie_phyctrl_set(0x4a4,0,16,0x013d); pcie_phyctrl_set(0x4a8,16,16,0x74); pcie_phyctrl_set(0x4a8,0,16,0x74); } else { pcie_phyctrl_set(0x4bc,24,8,0x64); pcie_phyctrl_set(0x490,12,4,0xa); pcie_phyctrl_set(0x490,6,2,0); pcie_phyctrl_set(0x4c0,0,32,0x19000000); pcie_phyctrl_set(0x4a4,0,16,0x018d); pcie_phyctrl_set(0x4a8,16,16,0x4a); pcie_phyctrl_set(0x4a8,0,16,0x4a); }
    pcie_phyctrl_set(0x498,0,8,5); pcie_phyctrl_set(0,5,1,1); pcie_phyctrl_set(0,4,1,0);
}

#[no_mangle] pub unsafe extern "C" fn pcibios_map_irq(dev: *const PciDev, slot: U8, _: U8) -> CInt { let mut irq=0; if !dev.is_null() && !(*dev).bus.is_null() && (*(*dev).bus).number == 1 && slot == 0 { irq=RALINK_INT_PCIE0; } irq }
#[no_mangle] pub unsafe extern "C" fn pcibios_plat_dev_init(_: *mut PciDev) -> CInt { 0 }
#[no_mangle] pub unsafe extern "C" fn mt7620_pci_probe(_: *mut PlatformDevice) -> CInt { 0 }
#[no_mangle] pub unsafe extern "C" fn mt7620_pci_init() -> CInt { platform_driver_register(core::ptr::null_mut()) }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
