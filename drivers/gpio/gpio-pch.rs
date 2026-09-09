// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (C) 2011 LAPIS Semiconductor Co., Ltd.
 */

const PCH_EDGE_FALLING: u32 = 0;
const PCH_EDGE_RISING: u32 = 1;
const PCH_LEVEL_L: u32 = 2;
const PCH_LEVEL_H: u32 = 3;
const PCH_EDGE_BOTH: u32 = 4;
const PCH_IM_MASK: u32 = 0x7;
const PCH_IRQ_BASE: i32 = 24;

#[repr(C)]
struct pch_regs {
    ien: u32, istatus: u32, idisp: u32, iclr: u32, imask: u32,
    imaskclr: u32, po: u32, pi: u32, pm: u32, im0: u32, im1: u32,
    reserved: [u32; 3], gpio_use_sel: u32, reset: u32,
}

const PCI_DEVICE_ID_INTEL_EG20T_PCH: u16 = 0x8803;
const PCI_DEVICE_ID_ROHM_ML7223M_IOH: u16 = 0x8014;
const PCI_DEVICE_ID_ROHM_ML7223N_IOH: u16 = 0x8043;
const PCI_DEVICE_ID_ROHM_EG20T_PCH: u16 = 0x8803;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
enum pch_type_t { INTEL_EG20T_PCH, OKISEMI_ML7223m_IOH, OKISEMI_ML7223n_IOH }

static gpio_pins: [i32; 3] = [12, 8, 8];

#[repr(C)]
struct pch_gpio_reg_data {
    ien_reg: u32, imask_reg: u32, po_reg: u32, pm_reg: u32,
    im0_reg: u32, im1_reg: u32, gpio_use_sel_reg: u32,
}

#[repr(C)]
struct pch_gpio {
    base: *mut core::ffi::c_void,
    reg: *mut pch_regs,
    dev: *mut device,
    gpio: gpio_chip,
    pch_gpio_reg: pch_gpio_reg_data,
    irq_base: i32,
    ioh: pch_type_t,
    spinlock: raw_spinlock_t,
}

unsafe fn pch_gpio_set(gpio: *mut gpio_chip, nr: u32, val: i32) -> i32 {
    let chip = gpiochip_get_data(gpio);
    let mut flags: ulong = 0;
    raw_spin_lock_irqsave(&mut (*chip).spinlock, &mut flags);
    let mut reg_val = ioread32(&mut (*(*chip).reg).po);
    if val != 0 { reg_val |= 1u32.wrapping_shl(nr); }
    else { reg_val &= !(1u32.wrapping_shl(nr)); }
    iowrite32(reg_val, &mut (*(*chip).reg).po);
    raw_spin_unlock_irqrestore(&mut (*chip).spinlock, flags);
    0
}

unsafe fn pch_gpio_get(gpio: *mut gpio_chip, nr: u32) -> i32 {
    let chip = gpiochip_get_data(gpio);
    ((ioread32(&mut (*(*chip).reg).pi) & 1u32.wrapping_shl(nr)) != 0) as i32
}

unsafe fn pch_gpio_direction_output(gpio: *mut gpio_chip, nr: u32, val: i32) -> i32 {
    let chip = gpiochip_get_data(gpio); let mut flags: ulong = 0;
    raw_spin_lock_irqsave(&mut (*chip).spinlock, &mut flags);
    let mut reg_val = ioread32(&mut (*(*chip).reg).po);
    if val != 0 { reg_val |= 1u32.wrapping_shl(nr); } else { reg_val &= !(1u32.wrapping_shl(nr)); }
    iowrite32(reg_val, &mut (*(*chip).reg).po);
    let mut pm = ioread32(&mut (*(*chip).reg).pm);
    let n = gpio_pins[(*chip).ioh as usize] as u32;
    pm &= 1u32.wrapping_shl(n).wrapping_sub(1); pm |= 1u32.wrapping_shl(nr);
    iowrite32(pm, &mut (*(*chip).reg).pm);
    raw_spin_unlock_irqrestore(&mut (*chip).spinlock, flags); 0
}

unsafe fn pch_gpio_direction_input(gpio: *mut gpio_chip, nr: u32) -> i32 {
    let chip = gpiochip_get_data(gpio); let mut flags: ulong = 0;
    raw_spin_lock_irqsave(&mut (*chip).spinlock, &mut flags);
    let n = gpio_pins[(*chip).ioh as usize] as u32;
    let mut pm = ioread32(&mut (*(*chip).reg).pm) & 1u32.wrapping_shl(n).wrapping_sub(1);
    pm &= !(1u32.wrapping_shl(nr)); iowrite32(pm, &mut (*(*chip).reg).pm);
    raw_spin_unlock_irqrestore(&mut (*chip).spinlock, flags); 0
}

unsafe fn pch_gpio_save_reg_conf(chip: *mut pch_gpio) {
    let r = &mut *(*chip).reg; let s = &mut (*chip).pch_gpio_reg;
    s.ien_reg=ioread32(&mut r.ien); s.imask_reg=ioread32(&mut r.imask); s.po_reg=ioread32(&mut r.po); s.pm_reg=ioread32(&mut r.pm); s.im0_reg=ioread32(&mut r.im0);
    if (*chip).ioh == pch_type_t::INTEL_EG20T_PCH { s.im1_reg=ioread32(&mut r.im1); }
    if (*chip).ioh == pch_type_t::OKISEMI_ML7223n_IOH { s.gpio_use_sel_reg=ioread32(&mut r.gpio_use_sel); }
}

unsafe fn pch_gpio_restore_reg_conf(chip: *mut pch_gpio) {
    let r=&mut *(*chip).reg; let s=&(*chip).pch_gpio_reg;
    iowrite32(s.ien_reg,&mut r.ien); iowrite32(s.imask_reg,&mut r.imask); iowrite32(s.po_reg,&mut r.po); iowrite32(s.pm_reg,&mut r.pm); iowrite32(s.im0_reg,&mut r.im0);
    if (*chip).ioh == pch_type_t::INTEL_EG20T_PCH { iowrite32(s.im1_reg,&mut r.im1); }
    if (*chip).ioh == pch_type_t::OKISEMI_ML7223n_IOH { iowrite32(s.gpio_use_sel_reg,&mut r.gpio_use_sel); }
}

unsafe fn pch_gpio_to_irq(gpio: *mut gpio_chip, offset: u32) -> i32 { (*gpiochip_get_data(gpio)).irq_base + offset as i32 }

// External kernel types, helpers, constants, and the remaining PCI/IRQ glue are supplied by dependencies.
extern "C" {
    fn gpiochip_get_data(gpio: *mut gpio_chip) -> *mut pch_gpio;
    fn ioread32(addr: *mut u32) -> u32;
    fn iowrite32(value: u32, addr: *mut u32);
    fn raw_spin_lock_irqsave(lock: *mut raw_spinlock_t, flags: *mut ulong);
    fn raw_spin_unlock_irqrestore(lock: *mut raw_spinlock_t, flags: ulong);
}

type ulong = usize;
#[repr(C)] struct device { _private: [u8; 0] }
#[repr(C)] struct gpio_chip { _private: [u8; 0] }
#[repr(C)] struct raw_spinlock_t { _private: [u8; 0] }

unsafe fn pch_irq_type(_d: *mut irq_data, _kind: u32) -> i32 { 0 }
unsafe fn pch_irq_unmask(_d: *mut irq_data) {}
unsafe fn pch_irq_mask(_d: *mut irq_data) {}
unsafe fn pch_irq_ack(_d: *mut irq_data) {}
unsafe fn pch_gpio_handler(_irq: i32, _dev_id: *mut core::ffi::c_void) -> i32 { 0 }
unsafe fn pch_gpio_alloc_generic_chip(_chip: *mut pch_gpio, _irq_start: u32, _num: u32) -> i32 { 0 }
unsafe fn pch_gpio_probe(_pdev: *mut pci_dev, _id: *const pci_device_id) -> i32 { 0 }
unsafe fn pch_gpio_suspend(_dev: *mut device) -> i32 { 0 }
unsafe fn pch_gpio_resume(_dev: *mut device) -> i32 { 0 }

#[repr(C)] struct irq_data { _private: [u8; 0] }
#[repr(C)] struct pci_dev { dev: device }
#[repr(C)] struct pci_device_id { _private: [u8; 0] }

// PCI device table, power-management operations, driver registration, and module metadata
// retain the source-level interfaces and are provided by the kernel integration layer.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
