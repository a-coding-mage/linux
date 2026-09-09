// SPDX-License-Identifier: GPL-2.0-only
/*
 * gpio-reg: single register individually fixed-direction GPIOs
 *
 * Copyright (C) 2016 Russell King
 */

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};

// Kernel-provided types and functions are supplied by the surrounding tree.
type u32 = core::primitive::u32;
type spinlock_t = c_void;
type irq_domain = c_void;
type device = c_void;

const GPIO_LINE_DIRECTION_IN: c_int = 1;
const GPIO_LINE_DIRECTION_OUT: c_int = 0;
const ENOTSUPP: c_int = 524;
const ENOMEM: c_int = 12;

#[repr(C)]
pub struct gpio_chip {
    pub label: *const c_char,
    pub get_direction: Option<unsafe extern "C" fn(*mut gpio_chip, c_uint) -> c_int>,
    pub direction_input: Option<unsafe extern "C" fn(*mut gpio_chip, c_uint) -> c_int>,
    pub direction_output: Option<unsafe extern "C" fn(*mut gpio_chip, c_uint, c_int) -> c_int>,
    pub set: Option<unsafe extern "C" fn(*mut gpio_chip, c_uint, c_int) -> c_int>,
    pub get: Option<unsafe extern "C" fn(*mut gpio_chip, c_uint) -> c_int>,
    pub set_multiple: Option<unsafe extern "C" fn(*mut gpio_chip, *mut c_ulong, *mut c_ulong) -> c_int>,
    pub to_irq: Option<unsafe extern "C" fn(*mut gpio_chip, c_uint) -> c_int>,
    pub base: c_int,
    pub ngpio: c_uint,
    pub names: *const *const c_char,
}

#[repr(C)]
struct gpio_reg {
    gc: gpio_chip,
    lock: spinlock_t,
    direction: u32,
    out: u32,
    reg: *mut c_void,
    irqdomain: *mut irq_domain,
    irqs: *const c_int,
}

unsafe extern "C" {
    fn spin_lock_init(lock: *mut spinlock_t);
    fn spin_lock_irqsave(lock: *mut spinlock_t, flags: *mut c_ulong);
    fn spin_unlock_irqrestore(lock: *mut spinlock_t, flags: c_ulong);
    fn writel_relaxed(value: u32, address: *mut c_void);
    fn readl_relaxed(address: *mut c_void) -> u32;
    fn irq_find_mapping(domain: *mut irq_domain, hwirq: c_uint) -> c_int;
    fn devm_kzalloc(dev: *mut device, size: usize, flags: c_uint) -> *mut c_void;
    fn kzalloc_obj(size: usize) -> *mut c_void;
    fn devm_gpiochip_add_data(dev: *mut device, gc: *mut gpio_chip, data: *mut c_void) -> c_int;
    fn gpiochip_add_data(gc: *mut gpio_chip, data: *mut c_void) -> c_int;
}

const GFP_KERNEL: c_uint = 0;

#[inline]
unsafe fn to_gpio_reg(x: *mut gpio_chip) -> *mut gpio_reg {
    x as *mut gpio_reg
}

unsafe extern "C" fn gpio_reg_get_direction(gc: *mut gpio_chip, offset: c_uint) -> c_int {
    let r = to_gpio_reg(gc);
    if (*r).direction & (1u32.wrapping_shl(offset)) != 0 {
        GPIO_LINE_DIRECTION_IN
    } else {
        GPIO_LINE_DIRECTION_OUT
    }
}

unsafe extern "C" fn gpio_reg_direction_output(gc: *mut gpio_chip, offset: c_uint, value: c_int) -> c_int {
    let r = to_gpio_reg(gc);
    if (*r).direction & (1u32.wrapping_shl(offset)) != 0 { return -ENOTSUPP; }
    ((*gc).set.unwrap())(gc, offset, value);
    0
}

unsafe extern "C" fn gpio_reg_direction_input(gc: *mut gpio_chip, offset: c_uint) -> c_int {
    let r = to_gpio_reg(gc);
    if (*r).direction & (1u32.wrapping_shl(offset)) != 0 { 0 } else { -ENOTSUPP }
}

unsafe extern "C" fn gpio_reg_set(gc: *mut gpio_chip, offset: c_uint, value: c_int) -> c_int {
    let r = to_gpio_reg(gc); let mut flags = 0; let mask = 1u32.wrapping_shl(offset);
    spin_lock_irqsave(&mut (*r).lock, &mut flags);
    let mut val = (*r).out;
    if value != 0 { val |= mask; } else { val &= !mask; }
    (*r).out = val; writel_relaxed(val, (*r).reg);
    spin_unlock_irqrestore(&mut (*r).lock, flags); 0
}

unsafe extern "C" fn gpio_reg_get(gc: *mut gpio_chip, offset: c_uint) -> c_int {
    let r = to_gpio_reg(gc); let mask = 1u32.wrapping_shl(offset);
    let val = if (*r).direction & mask != 0 { readl_relaxed((*r).reg); readl_relaxed((*r).reg) } else { (*r).out };
    if val & mask != 0 { 1 } else { 0 }
}

unsafe extern "C" fn gpio_reg_set_multiple(gc: *mut gpio_chip, mask: *mut c_ulong, bits: *mut c_ulong) -> c_int {
    let r = to_gpio_reg(gc); let mut flags = 0;
    spin_lock_irqsave(&mut (*r).lock, &mut flags);
    (*r).out = ((*r).out as c_ulong & !*mask | (*bits & *mask)) as u32;
    writel_relaxed((*r).out, (*r).reg);
    spin_unlock_irqrestore(&mut (*r).lock, flags); 0
}

unsafe extern "C" fn gpio_reg_to_irq(gc: *mut gpio_chip, offset: c_uint) -> c_int {
    let r = to_gpio_reg(gc); let mut irq = *(*r).irqs.add(offset as usize);
    if irq >= 0 && !(*r).irqdomain.is_null() { irq = irq_find_mapping((*r).irqdomain, irq as c_uint); } irq
}

pub unsafe extern "C" fn gpio_reg_init(dev: *mut device, reg: *mut c_void, base: c_int, num: c_int, label: *const c_char, direction: u32, def_out: u32, names: *const *const c_char, irqdom: *mut irq_domain, irqs: *const c_int) -> *mut gpio_chip {
    let r = if !dev.is_null() { devm_kzalloc(dev, core::mem::size_of::<gpio_reg>(), GFP_KERNEL) } else { kzalloc_obj(core::mem::size_of::<gpio_reg>()) } as *mut gpio_reg;
    if r.is_null() { return (-ENOMEM) as isize as *mut gpio_chip; }
    spin_lock_init(&mut (*r).lock);
    (*r).gc.label = label; (*r).gc.get_direction = Some(gpio_reg_get_direction); (*r).gc.direction_input = Some(gpio_reg_direction_input); (*r).gc.direction_output = Some(gpio_reg_direction_output); (*r).gc.set = Some(gpio_reg_set); (*r).gc.get = Some(gpio_reg_get); (*r).gc.set_multiple = Some(gpio_reg_set_multiple);
    if !irqs.is_null() { (*r).gc.to_irq = Some(gpio_reg_to_irq); }
    (*r).gc.base = base; (*r).gc.ngpio = num as c_uint; (*r).gc.names = names; (*r).direction = direction; (*r).out = def_out; (*r).reg = reg; (*r).irqdomain = irqdom; (*r).irqs = irqs;
    let ret = if !dev.is_null() { devm_gpiochip_add_data(dev, &mut (*r).gc, r as *mut c_void) } else { gpiochip_add_data(&mut (*r).gc, r as *mut c_void) };
    if ret != 0 { ret as isize as *mut gpio_chip } else { &mut (*r).gc }
}

pub unsafe extern "C" fn gpio_reg_resume(gc: *mut gpio_chip) -> c_int {
    let r = to_gpio_reg(gc); let mut flags = 0;
    spin_lock_irqsave(&mut (*r).lock, &mut flags); writel_relaxed((*r).out, (*r).reg); spin_unlock_irqrestore(&mut (*r).lock, flags); 0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
