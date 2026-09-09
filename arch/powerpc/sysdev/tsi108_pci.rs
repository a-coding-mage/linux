// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Common routines for Tundra Semiconductor TSI108 host bridge.
 *
 * 2004-2005 (c) Tundra Semiconductor Corp.
 * Author: Alex Bounine (alexandreb@tundra.com)
 * Author: Roy Zang (tie-fei.zang@freescale.com)
 *         Add pci interrupt router host
 */

// Linux and architecture dependencies are supplied by the surrounding kernel translation.

pub static mut tsi108_pci_cfg_base: u32 = 0;
static mut tsi108_pci_cfg_phys: u32 = 0;
pub static mut tsi108_csr_vir_base: u32 = 0;
static mut pci_irq_host: *mut irq_domain = core::ptr::null_mut();

extern "C" {
    fn get_vir_csrbase() -> u32;
    fn tsi108_read_reg(reg_offset: u32) -> u32;
    fn tsi108_write_reg(reg_offset: u32, val: u32);
    fn pci_bus_to_host(bus: *mut pci_bus) -> *mut pci_controller;
    fn ioremap(phys_addr: u32, size: usize) -> u32;
    fn out_8(addr: *mut u8, val: u32);
    fn out_le16(addr: *mut u16, val: u32);
    fn out_le32(addr: *mut u32, val: u32);
    fn irq_set_status_flags(irq: u32, flags: u32);
    fn irq_set_chip(irq: u32, chip: *mut irq_chip);
    fn irq_domain_create_legacy(fwnode: *mut core::ffi::c_void, size: u32, first_hwirq: u32,
                                hwirq: u32, ops: *const irq_domain_ops, host_data: *mut core::ffi::c_void)
                                -> *mut irq_domain;
    fn of_fwnode_handle(node: *mut device_node) -> *mut core::ffi::c_void;
    fn generic_handle_irq(irq: u32);
    fn irq_desc_get_chip(desc: *mut irq_desc) -> *mut irq_chip;
}

#[repr(C)] pub struct pci_bus { pub number: u8 }
#[repr(C)] pub struct pci_controller { pub first_busno: u8, pub last_busno: u8, pub ops: *const pci_ops }
#[repr(C)] pub struct device_node;
#[repr(C)] pub struct resource { pub start: u64 }
#[repr(C)] pub struct irq_domain;
#[repr(C)] pub struct irq_data { pub irq: u32 }
#[repr(C)] pub struct irq_desc { pub irq_data: irq_data }
#[repr(C)] pub struct irq_chip { pub name: *const u8, pub irq_mask: Option<unsafe extern "C" fn(*mut irq_data)>, pub irq_ack: Option<unsafe extern "C" fn(*mut irq_data)>, pub irq_unmask: Option<unsafe extern "C" fn(*mut irq_data)>, pub irq_eoi: Option<unsafe extern "C" fn(*mut irq_data)> }
#[repr(C)] pub struct pci_ops { pub read: Option<unsafe extern "C" fn(*mut pci_bus, u32, i32, i32, *mut u32) -> i32>, pub write: Option<unsafe extern "C" fn(*mut pci_bus, u32, i32, i32, u32) -> i32> }
#[repr(C)] pub struct irq_domain_ops { pub map: Option<unsafe extern "C" fn(*mut irq_domain, u32, u64) -> i32>, pub xlate: Option<unsafe extern "C" fn(*mut irq_domain, *mut device_node, *const u32, u32, *mut u64, *mut u32) -> i32> }

extern "C" { static mut ppc_md: ppc_md_struct; }
#[repr(C)] pub struct ppc_md_struct { pub pci_exclude_device: Option<unsafe extern "C" fn(*mut pci_controller, u8, u32) -> i32> }

unsafe fn tsi_mk_config_addr(bus: u8, devfunc: u32, offset: i32) -> u32 {
    ((u32::from(bus) << 16) | (devfunc << 8) | ((offset as u32) & 0xfc)).wrapping_add(tsi108_pci_cfg_base)
}

pub unsafe extern "C" fn tsi108_direct_write_config(bus: *mut pci_bus, devfunc: u32, offset: i32, len: i32, val: u32) -> i32 {
    let hose = pci_bus_to_host(bus);
    if let Some(exclude) = ppc_md.pci_exclude_device { if exclude(hose, (*bus).number, devfunc) != 0 { return PCIBIOS_DEVICE_NOT_FOUND; } }
    let cfg_addr = (tsi_mk_config_addr((*bus).number, devfunc, offset) | ((offset as u32) & 3)) as *mut u8;
    match len { 1 => out_8(cfg_addr, val), 2 => out_le16(cfg_addr as *mut u16, val), _ => out_le32(cfg_addr as *mut u32, val) }
    PCIBIOS_SUCCESSFUL
}

pub unsafe fn tsi108_clear_pci_error(pci_cfg_base: u32) {
    let err_stat = tsi108_read_reg(TSI108_PB_OFFSET + TSI108_PB_ERRCS);
    let err_addr = tsi108_read_reg(TSI108_PB_OFFSET + TSI108_PB_AERR);
    if err_stat & TSI108_PB_ERRCS_ES != 0 {
        tsi108_write_reg(TSI108_PB_OFFSET + TSI108_PB_ERRCS, TSI108_PB_ERRCS_ES);
        tsi108_write_reg(TSI108_PB_OFFSET + TSI108_PB_ISR, TSI108_PB_ISR_PBS_RD_ERR);
        if err_addr & 0xff000000 == pci_cfg_base { let pci_stat = tsi108_read_reg(TSI108_PCI_OFFSET + TSI108_PCI_CSR); tsi108_write_reg(TSI108_PCI_OFFSET + TSI108_PCI_CSR, pci_stat); }
    }
}

pub unsafe extern "C" fn tsi108_direct_read_config(bus: *mut pci_bus, devfn: u32, offset: i32, len: i32, val: *mut u32) -> i32 {
    let hose = pci_bus_to_host(bus);
    if let Some(exclude) = ppc_md.pci_exclude_device { if exclude(hose, (*bus).number, devfn) != 0 { return PCIBIOS_DEVICE_NOT_FOUND; } }
    let addr = (tsi_mk_config_addr((*bus).number, devfn, offset) | ((offset as u32) & 3)) as *const u8;
    let temp = match len { 1 => core::ptr::read_volatile(addr), 2 => u16::from_le(core::ptr::read_volatile(addr as *const u16)) as u32, _ => u32::from_le(core::ptr::read_volatile(addr as *const u32)) };
    *val = temp; PCIBIOS_SUCCESSFUL
}

pub unsafe fn tsi108_clear_pci_cfg_error() { tsi108_clear_pci_error(tsi108_pci_cfg_phys); }

static mut tsi108_direct_pci_ops: pci_ops = pci_ops { read: Some(tsi108_direct_read_config), write: Some(tsi108_direct_write_config) };

pub unsafe extern "C" fn tsi108_setup_pci(dev: *mut device_node, cfg_phys: u32, _primary: i32) -> i32 {
    tsi108_pci_cfg_base = ioremap(cfg_phys, TSI108_PCI_CFG_SIZE as usize); tsi108_pci_cfg_phys = cfg_phys;
    let hose = pcibios_alloc_controller(dev); if hose.is_null() { return -12; }
    (*hose).first_busno = 0; (*hose).last_busno = 0xff; (*hose).ops = &tsi108_direct_pci_ops; 0
}

unsafe fn tsi108_pci_int_mask(irq: u32) { let line = irq.wrapping_sub(IRQ_PCI_INTAD_BASE); let mut cfg = tsi108_read_reg(TSI108_PCI_OFFSET + TSI108_PCI_IRP_CFG_CTL); cfg |= 1 << line; cfg &= !(3 << (8 + line * 2)); tsi108_write_reg(TSI108_PCI_OFFSET + TSI108_PCI_IRP_CFG_CTL, cfg); }
unsafe fn tsi108_pci_int_unmask(irq: u32) { let line = irq.wrapping_sub(IRQ_PCI_INTAD_BASE); let mut cfg = tsi108_read_reg(TSI108_PCI_OFFSET + TSI108_PCI_IRP_CFG_CTL); cfg &= !(1 << line); cfg |= 3 << (8 + line * 2); tsi108_write_reg(TSI108_PCI_OFFSET + TSI108_PCI_IRP_CFG_CTL, cfg); }
unsafe fn init_pci_source() { tsi108_write_reg(TSI108_PCI_OFFSET + TSI108_PCI_IRP_CFG_CTL, 0x0000ff00); tsi108_write_reg(TSI108_PCI_OFFSET + TSI108_PCI_IRP_ENABLE, TSI108_PCI_IRP_ENABLE_P_INT); }
static mut mask: i32 = 0;
unsafe fn get_pci_source() -> i32 { let stat = tsi108_read_reg(TSI108_PCI_OFFSET + TSI108_PCI_IRP_STAT); if stat & TSI108_PCI_IRP_STAT_P_INT != 0 { let temp = tsi108_read_reg(TSI108_PCI_OFFSET + TSI108_PCI_IRP_INTAD) & 0xf; for _i in 0..4 { let m = mask; mask += 1; if temp & (1 << (m % 4)) != 0 { mask += 1; return (IRQ_PCI_INTA + (m % 4) as u32) as i32; } } } -1 }

unsafe extern "C" fn tsi108_pci_irq_unmask(d: *mut irq_data) { tsi108_pci_int_unmask((*d).irq); let v = tsi108_read_reg(TSI108_PCI_OFFSET + TSI108_PCI_IRP_ENABLE); tsi108_write_reg(TSI108_PCI_OFFSET + TSI108_PCI_IRP_ENABLE, v | TSI108_PCI_IRP_ENABLE_P_INT); }
unsafe extern "C" fn tsi108_pci_irq_mask(d: *mut irq_data) { tsi108_pci_int_mask((*d).irq); }
unsafe extern "C" fn tsi108_pci_irq_ack(d: *mut irq_data) { tsi108_pci_int_mask((*d).irq); }
static mut tsi108_pci_irq: irq_chip = irq_chip { name: b"tsi108_PCI_int\0".as_ptr(), irq_mask: Some(tsi108_pci_irq_mask), irq_ack: Some(tsi108_pci_irq_ack), irq_unmask: Some(tsi108_pci_irq_unmask), irq_eoi: None };

unsafe extern "C" fn pci_irq_host_xlate(_h: *mut irq_domain, _ct: *mut device_node, intspec: *const u32, _n: u32, out: *mut u64, flags: *mut u32) -> i32 { *out = *intspec as u64; *flags = IRQ_TYPE_LEVEL_HIGH; 0 }
unsafe extern "C" fn pci_irq_host_map(_h: *mut irq_domain, virq: u32, _hw: u64) -> i32 { if (1..=4).contains(&virq) { let irq = virq + IRQ_PCI_INTAD_BASE - 1; irq_set_status_flags(irq, IRQ_LEVEL); irq_set_chip(irq, &mut tsi108_pci_irq); } 0 }
static pci_irq_domain_ops: irq_domain_ops = irq_domain_ops { map: Some(pci_irq_host_map), xlate: Some(pci_irq_host_xlate) };

pub unsafe extern "C" fn tsi108_pci_int_init(node: *mut device_node) { pci_irq_host = irq_domain_create_legacy(of_fwnode_handle(node), NR_IRQS_LEGACY, 0, 0, &pci_irq_domain_ops, core::ptr::null_mut()); if !pci_irq_host.is_null() { init_pci_source(); } }
pub unsafe extern "C" fn tsi108_irq_cascade(desc: *mut irq_desc) { let irq = get_pci_source(); if irq != -1 { generic_handle_irq(irq as u32); } if let Some(eoi) = (*irq_desc_get_chip(desc)).irq_eoi { eoi(&mut (*desc).irq_data); } }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
