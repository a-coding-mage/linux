// SPDX-License-Identifier: GPL-2.0-only
/*
 *  Ralink RT3662/RT3883 SoC PCI support
 *
 *  Copyright (C) 2011-2013 Gabor Juhos <juhosg@openwrt.org>
 *
 *  Parts of this file are based on Ralink's 2.6.21 BSP
 */

// External Linux/kernel and platform declarations are supplied by dependencies.

const RT3883_MEMORY_BASE: u32 = 0x00000000;
const RT3883_MEMORY_SIZE: u32 = 0x02000000;
const RT3883_PCI_REG_PCICFG: u32 = 0x00;
const RT3883_PCICFG_P2P_BR_DEVNUM_M: u32 = 0xf;
const RT3883_PCICFG_P2P_BR_DEVNUM_S: u32 = 16;
const RT3883_PCICFG_PCIRST: u32 = 1 << 1;
const RT3883_PCI_REG_PCIRAW: u32 = 0x04;
const RT3883_PCI_REG_PCIINT: u32 = 0x08;
const RT3883_PCI_REG_PCIENA: u32 = 0x0c;
const RT3883_PCI_REG_CFGADDR: u32 = 0x20;
const RT3883_PCI_REG_CFGDATA: u32 = 0x24;
const RT3883_PCI_REG_MEMBASE: u32 = 0x28;
const RT3883_PCI_REG_IOBASE: u32 = 0x2c;
const RT3883_PCI_REG_ARBCTL: u32 = 0x80;
const RT3883_PCI_REG_BASE: fn(u32) -> u32 = |x| 0x1000 + x * 0x1000;
fn rt3883_pci_reg_bar0setup(x: u32) -> u32 { 0x1000 + x * 0x1000 + 0x10 }
fn rt3883_pci_reg_imbasebar0(x: u32) -> u32 { 0x1000 + x * 0x1000 + 0x18 }
fn rt3883_pci_reg_id(x: u32) -> u32 { 0x1000 + x * 0x1000 + 0x30 }
fn rt3883_pci_reg_class(x: u32) -> u32 { 0x1000 + x * 0x1000 + 0x34 }
fn rt3883_pci_reg_subid(x: u32) -> u32 { 0x1000 + x * 0x1000 + 0x38 }
fn rt3883_pci_reg_status(x: u32) -> u32 { 0x1000 + x * 0x1000 + 0x50 }
const RT3883_PCI_MODE_NONE: u32 = 0;
const RT3883_PCI_MODE_PCI: u32 = 1 << 0;
const RT3883_PCI_MODE_PCIE: u32 = 1 << 1;
const RT3883_PCI_MODE_BOTH: u32 = RT3883_PCI_MODE_PCI | RT3883_PCI_MODE_PCIE;
const RT3883_PCI_IRQ_COUNT: u32 = 32;
const RT3883_P2P_BR_DEVNUM: u32 = 1;

#[repr(C)]
struct Rt3883PciController {
    base: *mut core::ffi::c_void,
    intc_of_node: *mut DeviceNode,
    irq_domain: *mut IrqDomain,
    pci_controller: PciController,
    io_res: Resource,
    mem_res: Resource,
    pcie_ready: bool,
}

#[inline]
unsafe fn pci_bus_to_rt3883_controller(bus: *mut PciBus) -> *mut Rt3883PciController {
    let hose = (*bus).sysdata as *mut PciController;
    (hose as *mut u8).sub(core::mem::offset_of!(Rt3883PciController, pci_controller)) as *mut Rt3883PciController
}

#[inline]
unsafe fn rt3883_pci_r32(rpc: *mut Rt3883PciController, reg: u32) -> u32 {
    ioread32((*rpc).base.add(reg as usize) as *const u32)
}
#[inline]
unsafe fn rt3883_pci_w32(rpc: *mut Rt3883PciController, val: u32, reg: u32) {
    iowrite32(val, (*rpc).base.add(reg as usize) as *mut u32);
}
#[inline]
fn rt3883_pci_get_cfgaddr(bus: u32, slot: u32, func: u32, where_: u32) -> u32 {
    (bus << 16) | (slot << 11) | (func << 8) | (where_ & 0xfc) | 0x80000000
}

unsafe fn rt3883_pci_read_cfg32(rpc: *mut Rt3883PciController, bus: u32, slot: u32, func: u32, reg: u32) -> u32 {
    rt3883_pci_w32(rpc, rt3883_pci_get_cfgaddr(bus, slot, func, reg), RT3883_PCI_REG_CFGADDR);
    rt3883_pci_r32(rpc, RT3883_PCI_REG_CFGDATA)
}
unsafe fn rt3883_pci_write_cfg32(rpc: *mut Rt3883PciController, bus: u32, slot: u32, func: u32, reg: u32, val: u32) {
    rt3883_pci_w32(rpc, rt3883_pci_get_cfgaddr(bus, slot, func, reg), RT3883_PCI_REG_CFGADDR);
    rt3883_pci_w32(rpc, val, RT3883_PCI_REG_CFGDATA);
}

unsafe fn rt3883_pci_irq_handler(desc: *mut IrqDesc) {
    let rpc = irq_desc_get_handler_data(desc) as *mut Rt3883PciController;
    let mut pending = rt3883_pci_r32(rpc, RT3883_PCI_REG_PCIINT) & rt3883_pci_r32(rpc, RT3883_PCI_REG_PCIENA);
    if pending == 0 { spurious_interrupt(); return; }
    while pending != 0 {
        let bit = pending.trailing_zeros();
        generic_handle_domain_irq((*rpc).irq_domain, bit);
        pending &= !(1 << bit);
    }
}

unsafe fn rt3883_pci_irq_unmask(d: *mut IrqData) {
    let rpc = irq_data_get_irq_chip_data(d) as *mut Rt3883PciController;
    let t = rt3883_pci_r32(rpc, RT3883_PCI_REG_PCIENA);
    rt3883_pci_w32(rpc, t | (1 << (*d).hwirq), RT3883_PCI_REG_PCIENA); let _ = rt3883_pci_r32(rpc, RT3883_PCI_REG_PCIENA);
}
unsafe fn rt3883_pci_irq_mask(d: *mut IrqData) {
    let rpc = irq_data_get_irq_chip_data(d) as *mut Rt3883PciController;
    let t = rt3883_pci_r32(rpc, RT3883_PCI_REG_PCIENA);
    rt3883_pci_w32(rpc, t & !(1 << (*d).hwirq), RT3883_PCI_REG_PCIENA); let _ = rt3883_pci_r32(rpc, RT3883_PCI_REG_PCIENA);
}

unsafe fn rt3883_pci_config_read(bus: *mut PciBus, devfn: u32, where_: i32, size: i32, val: *mut u32) -> i32 {
    let rpc = pci_bus_to_rt3883_controller(bus);
    if !(*rpc).pcie_ready && (*bus).number == 1 { return PCIBIOS_DEVICE_NOT_FOUND; }
    rt3883_pci_w32(rpc, rt3883_pci_get_cfgaddr((*bus).number, PCI_SLOT(devfn), PCI_FUNC(devfn), where_ as u32), RT3883_PCI_REG_CFGADDR);
    let data = rt3883_pci_r32(rpc, RT3883_PCI_REG_CFGDATA);
    *val = match size { 1 => (data >> (((where_ as u32) & 3) << 3)) & 0xff, 2 => (data >> (((where_ as u32) & 3) << 3)) & 0xffff, 4 => data, _ => *val };
    PCIBIOS_SUCCESSFUL
}

unsafe fn rt3883_pci_config_write(bus: *mut PciBus, devfn: u32, where_: i32, size: i32, val: u32) -> i32 {
    let rpc = pci_bus_to_rt3883_controller(bus);
    if !(*rpc).pcie_ready && (*bus).number == 1 { return PCIBIOS_DEVICE_NOT_FOUND; }
    rt3883_pci_w32(rpc, rt3883_pci_get_cfgaddr((*bus).number, PCI_SLOT(devfn), PCI_FUNC(devfn), where_ as u32), RT3883_PCI_REG_CFGADDR);
    let mut data = rt3883_pci_r32(rpc, RT3883_PCI_REG_CFGDATA);
    let shift = ((where_ as u32) & 3) << 3;
    data = match size { 1 => (data & !(0xff << shift)) | (val << shift), 2 => (data & !(0xffff << shift)) | (val << shift), 4 => val, _ => data };
    rt3883_pci_w32(rpc, data, RT3883_PCI_REG_CFGDATA); PCIBIOS_SUCCESSFUL
}

unsafe fn rt3883_pci_preinit(rpc: *mut Rt3883PciController, mode: u32) {
    let mut rstctrl = rt_sysc_r32(RT3883_SYSC_REG_RSTCTRL);
    let mut syscfg1 = rt_sysc_r32(RT3883_SYSC_REG_SYSCFG1);
    let mut clkcfg1 = rt_sysc_r32(RT3883_SYSC_REG_CLKCFG1);
    if mode & RT3883_PCI_MODE_PCIE != 0 { rstctrl |= RT3883_RSTCTRL_PCIE; rt_sysc_w32(rstctrl, RT3883_SYSC_REG_RSTCTRL); syscfg1 = (syscfg1 & !0x30) | (2 << 4); rt_sysc_w32(syscfg1, RT3883_SYSC_REG_SYSCFG1); }
    syscfg1 |= RT3883_SYSCFG1_PCIE_RC_MODE | RT3883_SYSCFG1_PCI_HOST_MODE;
    clkcfg1 &= !(RT3883_CLKCFG1_PCI_CLK_EN | RT3883_CLKCFG1_PCIE_CLK_EN);
    if mode & RT3883_PCI_MODE_PCI != 0 { clkcfg1 |= RT3883_CLKCFG1_PCI_CLK_EN; rstctrl &= !RT3883_RSTCTRL_PCI; }
    if mode & RT3883_PCI_MODE_PCIE != 0 { clkcfg1 |= RT3883_CLKCFG1_PCIE_CLK_EN; rstctrl &= !RT3883_RSTCTRL_PCIE; }
    rt_sysc_w32(syscfg1, RT3883_SYSC_REG_SYSCFG1); rt_sysc_w32(rstctrl, RT3883_SYSC_REG_RSTCTRL); rt_sysc_w32(clkcfg1, RT3883_SYSC_REG_CLKCFG1); msleep(500);
    rt3883_pci_w32(rpc, RT3883_P2P_BR_DEVNUM << RT3883_PCICFG_P2P_BR_DEVNUM_S, RT3883_PCI_REG_PCICFG); let _ = rt3883_pci_r32(rpc, RT3883_PCI_REG_PCICFG); msleep(500);
    if mode & RT3883_PCI_MODE_PCIE != 0 { msleep(500); (*rpc).pcie_ready = rt3883_pci_r32(rpc, rt3883_pci_reg_status(1)) & 1 != 0; }
    rt3883_pci_w32(rpc, 0x79, RT3883_PCI_REG_ARBCTL);
}

#[no_mangle] pub unsafe extern "C" fn pcibios_map_irq(dev: *const PciDev, slot: u8, pin: u8) -> i32 { of_irq_parse_and_map_pci(dev, slot, pin) }
#[no_mangle] pub unsafe extern "C" fn pcibios_plat_dev_init(_dev: *mut PciDev) -> i32 { 0 }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
