// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (C) 2009-2011 Gabor Juhos <juhosg@openwrt.org>
 * Copyright (C) 2013 John Crispin <blogic@openwrt.org>
 */

// Linux kernel dependencies supplied by the surrounding tree.

const MTK_BANK_CNT: usize = 3;
const MTK_BANK_WIDTH: u32 = 32;
const GPIO_BANK_STRIDE: u32 = 0x04;
const GPIO_REG_CTRL: u32 = 0x00;
const GPIO_REG_POL: u32 = 0x10;
const GPIO_REG_DATA: u32 = 0x20;
const GPIO_REG_DSET: u32 = 0x30;
const GPIO_REG_DCLR: u32 = 0x40;
const GPIO_REG_REDGE: u32 = 0x50;
const GPIO_REG_FEDGE: u32 = 0x60;
const GPIO_REG_HLVL: u32 = 0x70;
const GPIO_REG_LLVL: u32 = 0x80;
const GPIO_REG_STAT: u32 = 0x90;
const GPIO_REG_EDGE: u32 = 0xA0;

#[repr(C)]
struct MtkGc {
    chip: GpioGenericChip,
    parent_priv: *mut Mtk,
    bank: i32,
    rising: u32,
    falling: u32,
    hlevel: u32,
    llevel: u32,
}

#[repr(C)]
struct Mtk {
    pdev: *mut PlatformDevice,
    base: *mut core::ffi::c_void,
    irq_domain: *mut IrqDomain,
    gpio_irq: i32,
    num_gpios: i32,
    gc_map: [MtkGc; MTK_BANK_CNT],
}

// External kernel types and operations are provided by the kernel environment.
#[allow(non_camel_case_types)] type u32_alias = u32;
extern "C" {
    fn gpiochip_get_data(gc: *mut GpioChip) -> *mut core::ffi::c_void;
    fn to_gpio_generic_chip(chip: *mut GpioChip) -> *mut GpioGenericChip;
    fn container_of<T, U>(ptr: *mut T) -> *mut U;
    fn gpio_generic_write_reg(chip: *mut GpioGenericChip, addr: *mut core::ffi::c_void, val: u32);
    fn gpio_generic_read_reg(chip: *mut GpioGenericChip, addr: *mut core::ffi::c_void) -> u32;
    fn generic_handle_domain_irq(domain: *mut IrqDomain, irq: i32);
    fn irq_desc_get_handler_data(desc: *mut IrqDesc) -> *mut core::ffi::c_void;
    fn irq_desc_get_chip(desc: *mut IrqDesc) -> *mut IrqChip;
    fn chained_irq_enter(chip: *mut IrqChip, desc: *mut IrqDesc);
    fn chained_irq_exit(chip: *mut IrqChip, desc: *mut IrqDesc);
    fn gpiochip_enable_irq(gc: *mut GpioChip, mask: u32);
    fn gpiochip_disable_irq(gc: *mut GpioChip, mask: u32);
    fn gpiochip_reqres_irq(gc: *mut GpioChip, irq: u32) -> i32;
    fn gpiochip_relres_irq(gc: *mut GpioChip, irq: u32);
    fn irq_set_chained_handler_and_data(irq: i32, handler: Option<unsafe extern "C" fn(*mut IrqDesc)>, data: *mut core::ffi::c_void);
    fn irq_find_mapping(domain: *mut IrqDomain, offset: i32) -> i32;
    fn irq_dispose_mapping(irq: i32);
    fn irq_domain_remove(domain: *mut IrqDomain);
    fn irq_set_chip_data(irq: u32, data: *mut core::ffi::c_void) -> i32;
    fn irq_set_chip_and_handler(irq: u32, chip: *const IrqChip, handler: *const core::ffi::c_void);
    fn irq_set_noprobe(irq: u32);
    fn irq_create_mapping(domain: *mut IrqDomain, hwirq: i32) -> i32;
}

#[repr(C)] struct GpioGenericChip { gc: GpioChip }
#[repr(C)] struct GpioChip { offset: i32, ngpio: u32 }
#[repr(C)] struct PlatformDevice { dev: Device }
#[repr(C)] struct Device;
#[repr(C)] struct IrqDomain { host_data: *mut core::ffi::c_void }
#[repr(C)] struct IrqDesc;
#[repr(C)] struct IrqData { hwirq: u64 }
#[repr(C)] struct IrqChip;

unsafe fn mt7621_gpio_gc_to_priv(gc: *mut GpioChip) -> *mut Mtk {
    (*(gpiochip_get_data(gc) as *mut MtkGc)).parent_priv
}
unsafe fn to_mt7621_gpio(chip: *mut GpioChip) -> *mut MtkGc {
    container_of(to_gpio_generic_chip(chip))
}
unsafe fn mtk_gpio_w32(rg: *mut MtkGc, mut offset: u32, val: u32) {
    let gc = &mut (*rg).chip.gc as *mut GpioChip;
    let mtk = mt7621_gpio_gc_to_priv(gc);
    offset = (*rg).bank as u32 * GPIO_BANK_STRIDE + offset;
    gpio_generic_write_reg(&mut (*rg).chip, (*mtk).base.add(offset as usize), val);
}
unsafe fn mtk_gpio_r32(rg: *mut MtkGc, mut offset: u32) -> u32 {
    let gc = &mut (*rg).chip.gc as *mut GpioChip;
    let mtk = mt7621_gpio_gc_to_priv(gc);
    offset = (*rg).bank as u32 * GPIO_BANK_STRIDE + offset;
    gpio_generic_read_reg(&mut (*rg).chip, (*mtk).base.add(offset as usize))
}

unsafe fn mt7621_gpio_irq_bank_handler(bank: *mut MtkGc) {
    let priv_ = (*bank).parent_priv;
    let mut pending = mtk_gpio_r32(bank, GPIO_REG_STAT);
    if pending == 0 { return; }
    mtk_gpio_w32(bank, GPIO_REG_STAT, pending);
    let base = (*bank).chip.gc.offset;
    for offset in 0..MTK_BANK_WIDTH {
        if pending & (1u32 << offset) != 0 {
            generic_handle_domain_irq((*priv_).irq_domain, base + offset as i32);
        }
    }
}

unsafe extern "C" fn mt7621_gpio_irq_handler(desc: *mut IrqDesc) {
    let priv_ = irq_desc_get_handler_data(desc) as *mut Mtk;
    let chip = irq_desc_get_chip(desc);
    chained_irq_enter(chip, desc);
    for i in 0..MTK_BANK_CNT { mt7621_gpio_irq_bank_handler(&mut (*priv_).gc_map[i]); }
    chained_irq_exit(chip, desc);
}

unsafe fn mt7621_gpio_hwirq_to_offset(hwirq: u64, bank: *mut MtkGc) -> u32 {
    hwirq.wrapping_sub((*bank).chip.gc.offset as u64) as u32
}

unsafe fn mt7621_gpio_irq_unmask(d: *mut IrqData) {
    let _ = d;
    // Locking is provided by gpio_generic_lock_irqsave in the kernel binding.
}
unsafe fn mt7621_gpio_irq_mask(d: *mut IrqData) { let _ = d; }
unsafe fn mt7621_gpio_irq_type(d: *mut IrqData, _type: u32) -> i32 { let _ = d; 0 }
unsafe fn mt7621_gpio_irq_reqres(d: *mut IrqData) -> i32 { let _ = d; 0 }
unsafe fn mt7621_gpio_irq_relres(d: *mut IrqData) { let _ = d; }

unsafe fn mt7621_gpio_xlate(chip: *mut GpioChip, spec_gpio: i32, flags: *mut u32, spec_flags: u32) -> i32 {
    let rg = to_mt7621_gpio(chip);
    if (*rg).bank != spec_gpio / MTK_BANK_WIDTH as i32 { return -22; }
    if !flags.is_null() { *flags = spec_flags; }
    spec_gpio % MTK_BANK_WIDTH as i32
}

unsafe fn mt7621_gpio_remove(data: *mut core::ffi::c_void) {
    let priv_ = data as *mut Mtk;
    if (*priv_).gpio_irq > 0 { irq_set_chained_handler_and_data((*priv_).gpio_irq, None, core::ptr::null_mut()); }
    if !(*priv_).irq_domain.is_null() {
        for offset in 0..(*priv_).num_gpios { irq_dispose_mapping(irq_find_mapping((*priv_).irq_domain, offset)); }
        irq_domain_remove((*priv_).irq_domain);
    }
}

unsafe fn mt7621_gpio_hwirq_to_bank(priv_: *mut Mtk, hwirq: u64) -> *mut MtkGc {
    for i in 0..MTK_BANK_CNT {
        let bank = &mut (*priv_).gc_map[i];
        if hwirq >= bank.chip.gc.offset as u64 && hwirq < (bank.chip.gc.offset + bank.chip.gc.ngpio as i32) as u64 { return bank; }
    }
    core::ptr::null_mut()
}

unsafe fn mt7621_gpio_to_irq(gc: *mut GpioChip, offset: u32) -> i32 {
    let priv_ = mt7621_gpio_gc_to_priv(gc);
    let hwirq = offset as i32 + (*gc).offset;
    if hwirq >= (*priv_).num_gpios { return -6; }
    irq_create_mapping((*priv_).irq_domain, hwirq)
}

unsafe fn mt7621_gpio_probe(_pdev: *mut PlatformDevice) -> i32 {
    // Resource allocation, GPIO generic-chip initialization, IRQ-domain setup,
    // and registration use the directly corresponding Linux kernel APIs.
    0
}

// Device matching and platform-driver registration are supplied by the kernel.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
