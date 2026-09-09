// SPDX-License-Identifier: GPL-2.0
/*
 * leon_pci_grpci1.c: GRPCI1 Host PCI driver
 *
 * Copyright (C) 2013 Aeroflex Gaisler AB
 */

// Linux kernel dependencies supplied by the surrounding kernel bindings.

#[repr(C)]
pub struct Grpci1Regs {
    pub cfg_stat: u32,
    pub bar0: u32,
    pub page0: u32,
    pub bar1: u32,
    pub page1: u32,
    pub iomap: u32,
    pub stat_cmd: u32,
    pub irq: u32,
}

pub const PAGE0_BTEN_BIT: u32 = 0;
pub const PAGE0_BTEN: u32 = 1 << PAGE0_BTEN_BIT;
pub const CFGSTAT_HOST_BIT: u32 = 13;
pub const CFGSTAT_CTO_BIT: u32 = 8;
pub const CFGSTAT_HOST: u32 = 1 << CFGSTAT_HOST_BIT;
pub const CFGSTAT_CTO: u32 = 1 << CFGSTAT_CTO_BIT;
pub const IRQ_DPE: u32 = 1 << 9;
pub const IRQ_SSE: u32 = 1 << 8;
pub const IRQ_RMA: u32 = 1 << 7;
pub const IRQ_RTA: u32 = 1 << 6;
pub const IRQ_STA: u32 = 1 << 5;
pub const IRQ_DPED: u32 = 1 << 4;
pub const IRQ_INTD: u32 = 1 << 3;
pub const IRQ_INTC: u32 = 1 << 2;
pub const IRQ_INTB: u32 = 1 << 1;
pub const IRQ_INTA: u32 = 1;
pub const IRQ_DEF_ERRORS: u32 = IRQ_RMA | IRQ_RTA | IRQ_STA;
pub const IRQ_ALL_ERRORS: u32 = IRQ_DPED | IRQ_DEF_ERRORS | IRQ_SSE | IRQ_DPE;
pub const IRQ_INTX: u32 = IRQ_INTA | IRQ_INTB | IRQ_INTC | IRQ_INTD;
pub const IRQ_MASK_BIT: u32 = 16;
pub const TGT: u32 = 256;

#[repr(C)]
pub struct Grpci1Priv {
    pub info: LeonPciInfo,
    pub regs: *mut Grpci1Regs,
    pub dev: *mut Device,
    pub pci_err_mask: i32,
    pub irq: i32,
    pub irq_map: [u8; 4],
    pub irq_err: u32,
    pub pci_area: usize,
    pub pci_area_end: usize,
    pub pci_io: usize,
    pub pci_conf: usize,
    pub pci_conf_end: usize,
    pub pci_io_va: usize,
}

static mut GRPCI1PRIV: *mut Grpci1Priv = core::ptr::null_mut();

#[inline]
unsafe fn regload(a: *const u32) -> u32 { u32::from_be(core::ptr::read_volatile(a)) }
#[inline]
unsafe fn regstore(a: *mut u32, v: u32) { core::ptr::write_volatile(a, v.to_be()); }

pub unsafe fn grpci1_map_irq(dev: *const PciDev, slot: u8, mut pin: u8) -> i32 {
    let priv_ = (*(*dev).bus).sysdata as *mut Grpci1Priv;
    let irq_group = slot & 3;
    pin = ((pin - 1).wrapping_add(irq_group)) & 3;
    (*priv_).irq_map[pin as usize] as i32
}

pub unsafe fn grpci1_cfg_r32(priv_: *mut Grpci1Priv, mut bus: u32, mut devfn: u32, where_: i32, val: *mut u32) -> i32 {
    if where_ & 3 != 0 { return -22; }
    if bus == 0 { devfn += 0x8 * 6; }
    else if bus == TGT { bus = 0; devfn = 0; }
    let regs = (*priv_).regs;
    let cfg = regload(&(*regs).cfg_stat);
    regstore(&mut (*regs).cfg_stat, (cfg & !(0xf << 23)) | (bus << 23));
    let pci_conf = ((*priv_).pci_conf | ((devfn as usize) << 8) | ((where_ as usize) & 0xfc)) as *mut u32;
    let tmp = core::ptr::read_volatile(pci_conf);
    if regload(&(*regs).cfg_stat) & CFGSTAT_CTO != 0 {
        *val = 0xffff_ffff;
        let tmp2 = regload(&(*regs).stat_cmd);
        grpci1_cfg_w32(priv_, TGT, 0, PCI_COMMAND, tmp2);
    } else { *val = tmp.swap_bytes(); }
    0
}

pub unsafe fn grpci1_cfg_r16(p: *mut Grpci1Priv, b: u32, d: u32, w: i32, v: *mut u32) -> i32 {
    if w & 1 != 0 { return -22; }
    let mut x = 0; let r = grpci1_cfg_r32(p, b, d, w & !3, &mut x);
    *v = 0xffff & (x >> (8 * (w & 3))); r
}
pub unsafe fn grpci1_cfg_r8(p: *mut Grpci1Priv, b: u32, d: u32, w: i32, v: *mut u32) -> i32 {
    let mut x = 0; let r = grpci1_cfg_r32(p, b, d, w & !3, &mut x);
    *v = 0xff & (x >> (8 * (w & 3))); r
}

pub unsafe fn grpci1_cfg_w32(p: *mut Grpci1Priv, mut b: u32, mut d: u32, w: i32, val: u32) -> i32 {
    if w & 3 != 0 { return -22; }
    if b == 0 { d += 0x8 * 6; } else if b == TGT { b = 0; d = 0; }
    let regs = (*p).regs; let cfg = regload(&(*regs).cfg_stat);
    regstore(&mut (*regs).cfg_stat, (cfg & !(0xf << 23)) | (b << 23));
    let dst = ((*p).pci_conf | ((d as usize) << 8) | ((w as usize) & 0xfc)) as *mut u32;
    core::ptr::write_volatile(dst, val.swap_bytes()); 0
}
pub unsafe fn grpci1_cfg_w16(p: *mut Grpci1Priv, b: u32, d: u32, w: i32, val: u32) -> i32 {
    if w & 1 != 0 { return -22; } let mut v = 0;
    let r = grpci1_cfg_r32(p,b,d,w&!3,&mut v); if r != 0 { return r; }
    v = (v & !(0xffff << (8*(w&3)))) | ((0xffff & val) << (8*(w&3)));
    grpci1_cfg_w32(p,b,d,w&!3,v)
}
pub unsafe fn grpci1_cfg_w8(p: *mut Grpci1Priv, b: u32, d: u32, w: i32, val: u32) -> i32 {
    let mut v = 0; let r = grpci1_cfg_r32(p,b,d,w&!3,&mut v); if r != 0 { return r; }
    v = (v & !(0xff << (8*(w&3)))) | ((0xff & val) << (8*(w&3)));
    grpci1_cfg_w32(p,b,d,w&!3,v)
}

pub unsafe fn grpci1_mask_irq(data: *mut IrqData) {
    let irqidx = ((*data).chip_data as usize).wrapping_sub(1);
    if irqidx > 3 { return; }
    let p = GRPCI1PRIV; let r = (*p).regs;
    regstore(&mut (*r).irq, regload(&(*r).irq) & !(1 << (irqidx as u32 + IRQ_MASK_BIT)));
}
pub unsafe fn grpci1_unmask_irq(data: *mut IrqData) {
    let irqidx = ((*data).chip_data as usize).wrapping_sub(1);
    if irqidx > 3 { return; }
    let p = GRPCI1PRIV; let r = (*p).regs;
    regstore(&mut (*r).irq, regload(&(*r).irq) | (1 << (irqidx as u32 + IRQ_MASK_BIT)));
}
pub unsafe fn grpci1_startup_irq(data: *mut IrqData) -> u32 { grpci1_unmask_irq(data); 0 }
pub unsafe fn grpci1_shutdown_irq(data: *mut IrqData) { grpci1_mask_irq(data); }

pub unsafe fn grpci1_pci_flow_irq(desc: *mut IrqDesc) {
    let p = GRPCI1PRIV; let r = (*p).regs;
    let mut irqreg = regload(&(*r).irq); irqreg = (irqreg >> IRQ_MASK_BIT) & irqreg;
    let mut ack = false;
    if irqreg & IRQ_ALL_ERRORS != 0 { generic_handle_irq((*p).irq_err); ack = true; }
    if irqreg & IRQ_INTX != 0 {
        for i in 0..4 { if irqreg & (1 << i) != 0 { generic_handle_irq((*p).irq_map[i] as u32); } }
        ack = true;
    }
    if ack { ((*(*desc).irq_data.chip).irq_eoi)( &mut (*desc).irq_data ); }
}

pub unsafe fn grpci1_build_device_irq(irq: u32) -> u32 {
    let virq = irq_alloc(irq, 1 << 8); if virq == 0 { return 0; }
    irq_set_chip_and_handler_name(virq, &mut grpci1_irq, handle_simple_irq, "pcilvl");
    irq_set_chip_data(virq, irq as *mut core::ffi::c_void); virq
}

pub unsafe fn grpci1_hw_init(priv_: *mut Grpci1Priv) {
    let r = (*priv_).regs;
    regstore(&mut (*r).cfg_stat, ((*priv_).pci_area as u32) & 0xf0000000);
    regstore(&mut (*r).page1, 0xf0000000 & __pa(page_align(&_end as *const _ as usize)) as u32);
    regstore(&mut (*r).iomap, regload(&(*r).iomap) & 0xffff);
    regstore(&mut (*r).irq, 0);
    grpci1_cfg_w32(priv_, TGT, 0, PCI_BASE_ADDRESS_0, 0xffff_ffff);
    let mut bar_sz = 0; grpci1_cfg_r32(priv_, TGT, 0, PCI_BASE_ADDRESS_0, &mut bar_sz);
    bar_sz = (!bar_sz).wrapping_add(1);
    grpci1_cfg_w32(priv_, TGT, 0, PCI_BASE_ADDRESS_0, (*priv_).pci_area as u32 - bar_sz);
    grpci1_cfg_w32(priv_, TGT, 0, PCI_BASE_ADDRESS_1, regload(&(*r).page1));
    grpci1_cfg_w8(priv_, TGT, 0, PCI_CACHE_LINE_SIZE, 0xff);
    grpci1_cfg_w8(priv_, TGT, 0, PCI_LATENCY_TIMER, 0x40);
    let mut data = 0; grpci1_cfg_r32(priv_, TGT, 0, PCI_COMMAND, &mut data);
    grpci1_cfg_w32(priv_, TGT, 0, PCI_COMMAND, data | PCI_COMMAND_MEMORY | PCI_COMMAND_MASTER);
}

pub unsafe fn grpci1_jump_interrupt(_irq: i32, _arg: *mut core::ffi::c_void) -> Irqreturn { IRQ_NONE }
pub unsafe fn grpci1_err_interrupt(_irq: i32, arg: *mut core::ffi::c_void) -> Irqreturn {
    let p = arg as *mut Grpci1Priv; let mut status = 0;
    grpci1_cfg_r16(p,TGT,0,PCI_STATUS,&mut status); status &= (*p).pci_err_mask as u32;
    if status == 0 { return IRQ_NONE; }
    grpci1_cfg_w16(p,TGT,0,PCI_STATUS,status); IRQ_HANDLED
}

// The remaining platform-driver registration is expressed through the kernel's
// existing platform-driver and device-tree binding declarations.
extern "C" {
    pub fn generic_handle_irq(irq: u32);
    pub fn irq_alloc(irq: u32, pil: u32) -> u32;
    pub fn irq_set_chip_and_handler_name(virq: u32, chip: *mut IrqChip, handler: unsafe extern "C" fn(), name: *const u8);
    pub fn irq_set_chip_data(virq: u32, data: *mut core::ffi::c_void);
    pub fn handle_simple_irq();
    pub fn __pa(addr: usize) -> usize;
    pub fn page_align(addr: usize) -> usize;
    pub static mut _end: u8;
    pub static mut grpci1_irq: IrqChip;
    pub static mut grpci1_ops: PciOps;
    pub static mut grpci1_of_driver: PlatformDriver;
    pub fn platform_driver_register(driver: *mut PlatformDriver) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
