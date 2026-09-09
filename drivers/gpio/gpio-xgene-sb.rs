// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * AppliedMicro X-Gene SoC GPIO-Standby Driver
 *
 * Copyright (c) 2014, Applied Micro Circuits Corporation
 * Author: Tin Huynh <tnhuynh@apm.com>.
 *        Y Vo <yvo@apm.com>.
 *        Quan Nguyen <qnguyen@apm.com>.
 */

// Kernel dependencies supplied by other translation units are intentionally external.

const XGENE_DFLT_MAX_NGPIO: u32 = 22;
const XGENE_DFLT_MAX_NIRQ: u16 = 6;
const XGENE_DFLT_IRQ_START_PIN: u16 = 8;
const MPA_GPIO_INT_LVL: usize = 0x0290;
const MPA_GPIO_OE_ADDR: usize = 0x029c;
const MPA_GPIO_OUT_ADDR: usize = 0x02a0;
const MPA_GPIO_IN_ADDR: usize = 0x02a4;
const MPA_GPIO_SEL_LO: usize = 0x0294;
const GPIO_INT_LEVEL_H: i32 = 0x000001;
const GPIO_INT_LEVEL_L: i32 = 0x000000;

#[inline]
const fn gpio_mask(x: u32) -> u32 { 1u32 << (x % 32) }

#[repr(C)]
pub struct GpioChip { pub parent: *mut Device, pub ngpio: u32, pub to_irq: Option<unsafe extern "C" fn(*mut GpioChip, u32) -> i32>, pub irq: GpioIrq }
#[repr(C)] pub struct GpioIrq { pub domain: *mut IrqDomain }
#[repr(C)] pub struct GpioGenericChip { pub gc: GpioChip }
#[repr(C)] pub struct Device { pub fwnode: *mut Fwnode }
#[repr(C)] pub struct PlatformDevice { pub dev: Device }
#[repr(C)] pub struct Fwnode;
#[repr(C)] pub struct IrqDomain { pub parent: *mut IrqDomain, pub host_data: *mut core::ffi::c_void }
#[repr(C)] pub struct IrqData { pub hwirq: u64, pub chip_data: *mut core::ffi::c_void }
#[repr(C)] pub struct IrqFwspec { pub fwnode: *mut Fwnode, pub param_count: u32, pub param: [u32; 3] }
#[repr(C)] pub struct IrqChip;
#[repr(C)] pub struct GpioGenericChipConfig { pub dev: *mut Device, pub sz: u32, pub dat: *mut u8, pub set: *mut u8, pub dirout: *mut u8 }

#[repr(C)]
pub struct XgeneGpioSb {
    pub chip: GpioGenericChip,
    pub regs: *mut u8,
    pub irq_domain: *mut IrqDomain,
    pub irq_start: u16,
    pub nirq: u16,
    pub parent_irq_base: u16,
}

extern "C" {
    fn to_gpio_generic_chip(gc: *mut GpioChip) -> *mut GpioGenericChip;
    fn gpio_generic_read_reg(chip: *mut GpioGenericChip, reg: *mut u8) -> u32;
    fn gpio_generic_write_reg(chip: *mut GpioGenericChip, reg: *mut u8, data: u32);
    fn irq_data_get_irq_chip_data(d: *mut IrqData) -> *mut core::ffi::c_void;
    fn irq_chip_set_type_parent(d: *mut IrqData, ty: u32) -> i32;
    fn irq_chip_mask_parent(d: *mut IrqData); fn irq_chip_unmask_parent(d: *mut IrqData);
    fn gpiochip_disable_irq(gc: *mut GpioChip, hwirq: u64); fn gpiochip_enable_irq(gc: *mut GpioChip, hwirq: u64);
    fn gpiochip_get_data(gc: *mut GpioChip) -> *mut core::ffi::c_void;
    fn irq_create_fwspec_mapping(spec: *mut IrqFwspec) -> i32;
    fn gpiochip_lock_as_irq(gc: *mut GpioChip, gpio: u32) -> i32; fn gpiochip_unlock_as_irq(gc: *mut GpioChip, gpio: u32);
    fn irq_domain_set_hwirq_and_chip(d: *mut IrqDomain, virq: u32, hwirq: u64, chip: *const IrqChip, data: *mut XgeneGpioSb);
    fn irq_domain_alloc_irqs_parent(d: *mut IrqDomain, virq: u32, nr: u32, arg: *mut IrqFwspec) -> i32;
    fn irq_domain_free_irqs_common(d: *mut IrqDomain, virq: u32, nr: u32);
    fn is_of_node(f: *mut Fwnode) -> bool; fn is_fwnode_irqchip(f: *mut Fwnode) -> bool;
    fn irq_domain_create_hierarchy(parent: *mut IrqDomain, flags: u32, size: u16, fwnode: *mut Fwnode, ops: *const IrqDomainOps, data: *mut XgeneGpioSb) -> *mut IrqDomain;
    fn irq_domain_remove(d: *mut IrqDomain);
    fn acpi_gpiochip_request_interrupts(gc: *mut GpioChip); fn acpi_gpiochip_free_interrupts(gc: *mut GpioChip);
}

#[inline] unsafe fn hwirq_to_gpio(priv_: *mut XgeneGpioSb, hwirq: u64) -> u32 { hwirq as u32 + (*priv_).irq_start as u32 }
#[inline] unsafe fn gpio_to_hwirq(priv_: *mut XgeneGpioSb, gpio: u32) -> u32 { gpio - (*priv_).irq_start as u32 }

unsafe fn xgene_gpio_set_bit(gc: *mut GpioChip, reg: *mut u8, gpio: u32, val: i32) {
    let chip = to_gpio_generic_chip(gc); let mut data = gpio_generic_read_reg(chip, reg);
    if val != 0 { data |= gpio_mask(gpio); } else { data &= !gpio_mask(gpio); }
    gpio_generic_write_reg(chip, reg, data);
}

unsafe extern "C" fn xgene_gpio_sb_irq_set_type(d: *mut IrqData, ty: u32) -> i32 {
    let priv_ = irq_data_get_irq_chip_data(d) as *mut XgeneGpioSb; let gpio = hwirq_to_gpio(priv_, (*d).hwirq); let mut lvl = GPIO_INT_LEVEL_H;
    match ty & 0xf { 1 | 4 => lvl = GPIO_INT_LEVEL_H, 2 | 8 => lvl = GPIO_INT_LEVEL_L, _ => {} }
    xgene_gpio_set_bit(&mut (*priv_).chip.gc, (*priv_).regs.add(MPA_GPIO_SEL_LO), gpio * 2, 1);
    xgene_gpio_set_bit(&mut (*priv_).chip.gc, (*priv_).regs.add(MPA_GPIO_INT_LVL), (*d).hwirq as u32, lvl);
    if ty & 0x3 != 0 { irq_chip_set_type_parent(d, 1) } else { irq_chip_set_type_parent(d, 4) }
}
unsafe extern "C" fn xgene_gpio_sb_irq_mask(d: *mut IrqData) { let p=irq_data_get_irq_chip_data(d) as *mut XgeneGpioSb; irq_chip_mask_parent(d); gpiochip_disable_irq(&mut (*p).chip.gc, (*d).hwirq); }
unsafe extern "C" fn xgene_gpio_sb_irq_unmask(d: *mut IrqData) { let p=irq_data_get_irq_chip_data(d) as *mut XgeneGpioSb; gpiochip_enable_irq(&mut (*p).chip.gc, (*d).hwirq); irq_chip_unmask_parent(d); }
unsafe extern "C" fn xgene_gpio_sb_to_irq(gc: *mut GpioChip, gpio: u32) -> i32 {
    let p=gpiochip_get_data(gc) as *mut XgeneGpioSb; if gpio < (*p).irq_start as u32 || gpio > hwirq_to_gpio(p, (*p).nirq as u64) { return -6; }
    let mut f=IrqFwspec { fwnode:(*gc).parent.as_ref().unwrap().fwnode, param_count:2, param:[gpio_to_hwirq(p,gpio),1,0] }; irq_create_fwspec_mapping(&mut f)
}
unsafe extern "C" fn xgene_gpio_sb_probe(_pdev: *mut PlatformDevice) -> i32 { -38 }
unsafe extern "C" fn xgene_gpio_sb_remove(_pdev: *mut PlatformDevice) {}

#[repr(C)] pub struct IrqDomainOps { pub translate: Option<unsafe extern "C" fn(*mut IrqDomain, *mut IrqFwspec, *mut u64, *mut u32) -> i32>, pub alloc: Option<unsafe extern "C" fn(*mut IrqDomain, u32, u32, *mut core::ffi::c_void) -> i32>, pub free: Option<unsafe extern "C" fn(*mut IrqDomain, u32, u32)>, pub activate: Option<unsafe extern "C" fn(*mut IrqDomain, *mut IrqData, bool) -> i32>, pub deactivate: Option<unsafe extern "C" fn(*mut IrqDomain, *mut IrqData)> }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
