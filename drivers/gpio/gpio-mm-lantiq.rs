// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 2012 John Crispin <john@phrozen.org>
 */

// Linux kernel dependencies supplied by the surrounding translation unit.

const LTQ_EBU_BUSCON: u32 = 0x1e7ff;
const LTQ_EBU_WP: u32 = 0x80000000;

#[repr(C)]
pub struct ltq_mm {
    pub gc: gpio_chip,
    pub regs: *mut core::ffi::c_void,
    pub shadow: u16,
}

extern "C" {
    static mut ebu_lock: spinlock_t;
    static mut LTQ_EBU_BUSCON1: u32;
    static mut THIS_MODULE: module;

    fn spin_lock_irqsave(lock: *mut spinlock_t, flags: *mut usize);
    fn spin_unlock_irqrestore(lock: *mut spinlock_t, flags: usize);
    fn ltq_ebu_w32(value: u32, reg: u32);
    fn __raw_writew(value: u16, addr: *mut core::ffi::c_void);
    fn gpiochip_get_data(gc: *mut gpio_chip) -> *mut core::ffi::c_void;
    fn CPHYSADDR(addr: *mut core::ffi::c_void) -> u32;
    fn devm_kzalloc(dev: *mut device, size: usize, flags: u32) -> *mut core::ffi::c_void;
    fn devm_kasprintf(dev: *mut device, flags: u32, fmt: *const i8, ...) -> *mut i8;
    fn devm_of_iomap(dev: *mut device, np: *mut device_node, index: i32, size: *mut usize) -> *mut core::ffi::c_void;
    fn IS_ERR(ptr: *mut core::ffi::c_void) -> bool;
    fn PTR_ERR(ptr: *mut core::ffi::c_void) -> i32;
    fn of_property_read_u32(np: *mut device_node, name: *const i8, value: *mut u32) -> i32;
    fn devm_gpiochip_add_data(dev: *mut device, gc: *mut gpio_chip, data: *mut core::ffi::c_void) -> i32;
    fn platform_driver_register(driver: *mut platform_driver) -> i32;
    fn platform_driver_unregister(driver: *mut platform_driver);
}

#[repr(C)] pub struct spinlock_t { _private: [u8; 0] }
#[repr(C)] pub struct module { _private: [u8; 0] }
#[repr(C)] pub struct device { pub of_node: *mut device_node }
#[repr(C)] pub struct device_node { _private: [u8; 0] }
#[repr(C)] pub struct platform_device { pub dev: device }
#[repr(C)] pub struct gpio_chip {
    pub base: i32,
    pub ngpio: u32,
    pub direction_output: Option<unsafe extern "C" fn(*mut gpio_chip, u32, i32) -> i32>,
    pub set: Option<unsafe extern "C" fn(*mut gpio_chip, u32, i32)>,
    pub parent: *mut device,
    pub owner: *mut module,
    pub label: *mut i8,
}
#[repr(C)] pub struct of_device_id { pub compatible: *const i8 }
#[repr(C)] pub struct device_driver { pub name: *const i8, pub of_match_table: *const of_device_id }
#[repr(C)] pub struct platform_driver { pub probe: Option<unsafe extern "C" fn(*mut platform_device) -> i32>, pub driver: device_driver }

unsafe fn ltq_mm_apply(chip: *mut ltq_mm) {
    let mut flags: usize = 0;
    spin_lock_irqsave(&mut ebu_lock, &mut flags);
    ltq_ebu_w32(LTQ_EBU_BUSCON, LTQ_EBU_BUSCON1);
    __raw_writew((*chip).shadow, (*chip).regs);
    ltq_ebu_w32(LTQ_EBU_BUSCON | LTQ_EBU_WP, LTQ_EBU_BUSCON1);
    spin_unlock_irqrestore(&mut ebu_lock, flags);
}

unsafe extern "C" fn ltq_mm_set(gc: *mut gpio_chip, offset: u32, value: i32) {
    let chip = gpiochip_get_data(gc) as *mut ltq_mm;
    if value != 0 { (*chip).shadow |= (1u16).wrapping_shl(offset); }
    else { (*chip).shadow &= !(1u16.wrapping_shl(offset)); }
    ltq_mm_apply(chip);
}

unsafe extern "C" fn ltq_mm_dir_out(gc: *mut gpio_chip, offset: u32, value: i32) -> i32 {
    ltq_mm_set(gc, offset, value); 0
}

unsafe fn ltq_mm_save_regs(chip: *mut ltq_mm) {
    ltq_ebu_w32(CPHYSADDR((*chip).regs) | 0x1, 0);
    ltq_mm_apply(chip);
}

unsafe extern "C" fn ltq_mm_probe(pdev: *mut platform_device) -> i32 {
    let dev = &mut (*pdev).dev as *mut device;
    let np = (*dev).of_node;
    let chip = devm_kzalloc(dev, core::mem::size_of::<ltq_mm>(), 0) as *mut ltq_mm;
    if chip.is_null() { return -12; }
    let gc = &mut (*chip).gc;
    gc.base = -1; gc.ngpio = 16;
    gc.direction_output = Some(ltq_mm_dir_out); gc.set = Some(ltq_mm_set);
    gc.parent = dev; gc.owner = &mut THIS_MODULE; gc.label = devm_kasprintf(dev, 0, b"%pOF\0".as_ptr() as *const i8, np);
    if gc.label.is_null() { return -12; }
    (*chip).regs = devm_of_iomap(dev, np, 0, core::ptr::null_mut());
    if IS_ERR((*chip).regs) { return PTR_ERR((*chip).regs); }
    ltq_mm_save_regs(chip);
    let mut shadow = 0u32;
    if of_property_read_u32(np, b"lantiq,shadow\0".as_ptr() as *const i8, &mut shadow) == 0 { (*chip).shadow = shadow as u16; }
    devm_gpiochip_add_data(dev, gc, chip as *mut core::ffi::c_void)
}

static mut LTQ_MM_MATCH: [of_device_id; 2] = [of_device_id { compatible: b"lantiq,gpio-mm\0".as_ptr() as *const i8 }, of_device_id { compatible: core::ptr::null() }];
static mut LTQ_MM_DRIVER: platform_driver = platform_driver { probe: Some(ltq_mm_probe), driver: device_driver { name: b"gpio-mm-ltq\0".as_ptr() as *const i8, of_match_table: LTQ_MM_MATCH.as_ptr() } };

unsafe extern "C" fn ltq_mm_init() -> i32 { platform_driver_register(&mut LTQ_MM_DRIVER) }
unsafe extern "C" fn ltq_mm_exit() { platform_driver_unregister(&mut LTQ_MM_DRIVER); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
