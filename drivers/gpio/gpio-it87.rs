// SPDX-License-Identifier: GPL-2.0-only
/*
 *  GPIO interface for IT87xx Super I/O chips
 *
 *  Author: Diego Elio Pettenò <flameeyes@flameeyes.eu>
 *  Copyright (c) 2017 Google, Inc.
 *
 *  Based on it87_wdt.c     by Oliver Schuster
 *           gpio-it8761e.c by Denis Turischev
 *           gpio-stmpe.c   by Rabin Vincent
 */

// C headers and kernel-provided symbols are supplied by the surrounding build.

const NO_DEV_ID: u16 = 0xffff;
const IT8613_ID: u16 = 0x8613;
const IT8620_ID: u16 = 0x8620;
const IT8628_ID: u16 = 0x8628;
const IT8718_ID: u16 = 0x8718;
const IT8728_ID: u16 = 0x8728;
const IT8732_ID: u16 = 0x8732;
const IT8761_ID: u16 = 0x8761;
const IT8772_ID: u16 = 0x8772;
const IT8786_ID: u16 = 0x8786;

const REG: u16 = 0x2e;
const VAL: u16 = 0x2f;
const GPIO: u8 = 0x07;
const LDNREG: u8 = 0x07;
const CHIPID: u8 = 0x20;
const CHIPREV: u8 = 0x22;

#[repr(C)]
struct it87_gpio {
    chip: gpio_chip,
    lock: spinlock_t,
    io_base: u16,
    io_size: u16,
    output_base: u8,
    simple_base: u8,
    simple_size: u8,
}

static mut it87_gpio_chip: it87_gpio = it87_gpio {
    chip: gpio_chip::default(),
    lock: __SPIN_LOCK_UNLOCKED(),
    io_base: 0,
    io_size: 0,
    output_base: 0,
    simple_base: 0,
    simple_size: 0,
};

unsafe fn superio_enter() -> i32 {
    if request_muxed_region(REG, 2, KBUILD_MODNAME).is_null() {
        return -EBUSY;
    }
    outb(0x87, REG);
    outb(0x01, REG);
    outb(0x55, REG);
    outb(0x55, REG);
    0
}

unsafe fn superio_exit() {
    outb(0x02, REG);
    outb(0x02, VAL);
    release_region(REG, 2);
}

unsafe fn superio_select(ldn: i32) {
    outb(LDNREG as u8, REG);
    outb(ldn as u8, VAL);
}

unsafe fn superio_inb(reg: i32) -> i32 {
    outb(reg as u8, REG);
    inb(VAL) as i32
}

unsafe fn superio_outb(val: i32, reg: i32) {
    outb(reg as u8, REG);
    outb(val as u8, VAL);
}

unsafe fn superio_inw(mut reg: i32) -> i32 {
    outb(reg as u8, REG);
    let mut val = (inb(VAL) as i32) << 8;
    reg += 1;
    outb(reg as u8, REG);
    val |= inb(VAL) as i32;
    val
}

unsafe fn superio_set_mask(mask: i32, reg: i32) {
    let curr_val = superio_inb(reg) as u8;
    let new_val = curr_val | mask as u8;
    if curr_val != new_val {
        superio_outb(new_val as i32, reg);
    }
}

unsafe fn superio_clear_mask(mask: i32, reg: i32) {
    let curr_val = superio_inb(reg) as u8;
    let new_val = curr_val & !(mask as u8);
    if curr_val != new_val {
        superio_outb(new_val as i32, reg);
    }
}

unsafe fn it87_gpio_request(chip: *mut gpio_chip, gpio_num: u32) -> i32 {
    let mask = 1u8 << (gpio_num % 8);
    let group = (gpio_num / 8) as u8;
    let it87_gpio = gpiochip_get_data(chip) as *mut it87_gpio;
    let mut rc = 0;

    spin_lock(&mut (*it87_gpio).lock);
    rc = superio_enter();
    if rc != 0 {
        spin_unlock(&mut (*it87_gpio).lock);
        return rc;
    }
    if group < (*it87_gpio).simple_size {
        superio_set_mask(mask as i32, group.wrapping_add((*it87_gpio).simple_base) as i32);
    }
    superio_clear_mask(mask as i32, group.wrapping_add((*it87_gpio).output_base) as i32);
    superio_exit();
    spin_unlock(&mut (*it87_gpio).lock);
    rc
}

unsafe fn it87_gpio_get(chip: *mut gpio_chip, gpio_num: u32) -> i32 {
    let it87_gpio = gpiochip_get_data(chip) as *mut it87_gpio;
    let mask = 1u8 << (gpio_num % 8);
    let reg = (gpio_num / 8) as u16 + (*it87_gpio).io_base;
    ((inb(reg) & mask) != 0) as i32
}

unsafe fn it87_gpio_direction_in(chip: *mut gpio_chip, gpio_num: u32) -> i32 {
    let it87_gpio = gpiochip_get_data(chip) as *mut it87_gpio;
    let mask = 1u8 << (gpio_num % 8);
    let group = (gpio_num / 8) as u8;
    spin_lock(&mut (*it87_gpio).lock);
    let rc = superio_enter();
    if rc != 0 {
        spin_unlock(&mut (*it87_gpio).lock);
        return rc;
    }
    superio_clear_mask(mask as i32, group.wrapping_add((*it87_gpio).output_base) as i32);
    superio_exit();
    spin_unlock(&mut (*it87_gpio).lock);
    rc
}

unsafe fn it87_gpio_set(chip: *mut gpio_chip, gpio_num: u32, val: i32) -> i32 {
    let it87_gpio = gpiochip_get_data(chip) as *mut it87_gpio;
    let mask = 1u8 << (gpio_num % 8);
    let reg = (gpio_num / 8) as u16 + (*it87_gpio).io_base;
    let curr_vals = inb(reg);
    if val != 0 {
        outb(curr_vals | mask, reg);
    } else {
        outb(curr_vals & !mask, reg);
    }
    0
}

unsafe fn it87_gpio_direction_out(chip: *mut gpio_chip, gpio_num: u32, val: i32) -> i32 {
    let it87_gpio = gpiochip_get_data(chip) as *mut it87_gpio;
    let mask = 1u8 << (gpio_num % 8);
    let group = (gpio_num / 8) as u8;
    guard_spinlock(&mut (*it87_gpio).lock);
    let rc = superio_enter();
    if rc != 0 {
        return rc;
    }
    superio_set_mask(mask as i32, group.wrapping_add((*it87_gpio).output_base) as i32);
    let rc = it87_gpio_set(chip, gpio_num, val);
    superio_exit();
    rc
}

static it87_template_chip: gpio_chip = gpio_chip {
    label: KBUILD_MODNAME,
    owner: THIS_MODULE,
    request: Some(it87_gpio_request),
    get: Some(it87_gpio_get),
    direction_input: Some(it87_gpio_direction_in),
    set: Some(it87_gpio_set),
    direction_output: Some(it87_gpio_direction_out),
    base: -1,
    ..gpio_chip::default()
};

unsafe fn it87_gpio_init() -> i32 {
    let it87_gpio = &mut it87_gpio_chip;
    let rc = superio_enter();
    if rc != 0 { return rc; }
    let chip_type = superio_inw(CHIPID) as u16;
    let chip_rev = (superio_inb(CHIPREV) as u8) & 0x0f;
    superio_exit();
    it87_gpio.chip = it87_template_chip;

    let gpio_ba_reg: u8;
    match chip_type {
        IT8613_ID => { gpio_ba_reg = 0x62; it87_gpio.io_size = 8; it87_gpio.output_base = 0xc8; it87_gpio.simple_base = 0xc0; it87_gpio.simple_size = 6; it87_gpio.chip.ngpio = 64; }
        IT8620_ID | IT8628_ID => { gpio_ba_reg = 0x62; it87_gpio.io_size = 11; it87_gpio.output_base = 0xc8; it87_gpio.simple_size = 0; it87_gpio.chip.ngpio = 64; }
        IT8718_ID | IT8728_ID | IT8732_ID | IT8772_ID | IT8786_ID => { gpio_ba_reg = 0x62; it87_gpio.io_size = 8; it87_gpio.output_base = 0xc8; it87_gpio.simple_base = 0xc0; it87_gpio.simple_size = 5; it87_gpio.chip.ngpio = 64; }
        IT8761_ID => { gpio_ba_reg = 0x60; it87_gpio.io_size = 4; it87_gpio.output_base = 0xf0; it87_gpio.simple_size = 0; it87_gpio.chip.ngpio = 16; }
        NO_DEV_ID => { pr_err!("no device\n"); return -ENODEV; }
        _ => { pr_err!("Unknown Chip found, Chip {:04x} Revision {:x}\n", chip_type, chip_rev); return -ENODEV; }
    }
    let rc = superio_enter();
    if rc != 0 { return rc; }
    superio_select(GPIO as i32);
    it87_gpio.io_base = superio_inw(gpio_ba_reg as i32) as u16;
    superio_exit();
    pr_info!("Found Chip IT{:04x} rev {:x}. {} GPIO lines starting at {:04x}h\n", chip_type, chip_rev, it87_gpio.chip.ngpio, it87_gpio.io_base);
    if request_region(it87_gpio.io_base, it87_gpio.io_size, KBUILD_MODNAME).is_null() { return -EBUSY; }

    let label_len = core::mem::size_of::<[u8; 10]>();
    let labels = kcalloc(it87_gpio.chip.ngpio as usize, label_len, GFP_KERNEL) as *mut u8;
    let labels_table = kcalloc(it87_gpio.chip.ngpio as usize, core::mem::size_of::<*const u8>(), GFP_KERNEL) as *mut *const u8;
    if labels.is_null() || labels_table.is_null() {
        kfree(labels_table as *mut core::ffi::c_void); kfree(labels as *mut core::ffi::c_void);
        release_region(it87_gpio.io_base, it87_gpio.io_size); return -ENOMEM;
    }
    for i in 0..it87_gpio.chip.ngpio as usize {
        let label = labels.add(i * label_len);
        sprintf(label, b"it87_gp%u%u\0".as_ptr(), 1 + i / 8, i % 8);
        *labels_table.add(i) = label;
    }
    it87_gpio.chip.names = labels_table as *const *const u8;
    let rc = gpiochip_add_data(&mut it87_gpio.chip, it87_gpio as *mut it87_gpio);
    if rc != 0 { kfree(labels_table as *mut core::ffi::c_void); kfree(labels as *mut core::ffi::c_void); release_region(it87_gpio.io_base, it87_gpio.io_size); return rc; }
    0
}

unsafe fn it87_gpio_exit() {
    let it87_gpio = &mut it87_gpio_chip;
    gpiochip_remove(&mut it87_gpio.chip);
    release_region(it87_gpio.io_base, it87_gpio.io_size);
    kfree((*it87_gpio.chip.names).cast_mut() as *mut core::ffi::c_void);
    kfree(it87_gpio.chip.names as *mut core::ffi::c_void);
}

module_init!(it87_gpio_init);
module_exit!(it87_gpio_exit);
module_author!("Diego Elio Pettenò <flameeyes@flameeyes.eu>");
module_description!("GPIO interface for IT87xx Super I/O chips");
module_license!("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
