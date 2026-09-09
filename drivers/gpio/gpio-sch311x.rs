// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * GPIO driver for the SMSC SCH311x Super-I/O chips
 *
 * Copyright (C) 2013 Bruno Randolf <br1@einfach.org>
 *
 * SuperIO functions and chip detection:
 * (c) Copyright 2008 Wim Van Sebroeck <wim@iguana.be>.
 */

// Linux kernel dependencies are supplied by other translation units.

const DRV_NAME: &str = "gpio-sch311x";

const SCH311X_GPIO_CONF_DIR: u8 = 1 << 0;
const SCH311X_GPIO_CONF_INVERT: u8 = 1 << 1;
const SCH311X_GPIO_CONF_OPEN_DRAIN: u8 = 1 << 7;

const SIO_CONFIG_KEY_ENTER: u8 = 0x55;
const SIO_CONFIG_KEY_EXIT: u8 = 0xaa;

const GP1: u16 = 0x4b;

static mut sch311x_ioports: [i32; 4] = [0x2e, 0x4e, 0x162e, 0x164e];
static mut sch311x_gpio_pdev: *mut platform_device = core::ptr::null_mut();

#[repr(C)]
struct sch311x_pdev_data {
    runtime_reg: u16,
}

#[repr(C)]
struct sch311x_gpio_block {
    chip: gpio_chip,
    data_reg: u16,
    config_regs: *mut u16,
    runtime_reg: u16,
    lock: spinlock_t,
}

#[repr(C)]
struct sch311x_gpio_priv {
    blocks: [sch311x_gpio_block; 6],
}

#[repr(C)]
struct sch311x_gpio_block_def {
    data_reg: u16,
    config_regs: [u16; 8],
    base: i32,
}

/* Note: some GPIOs are not available, these are marked with 0x00 */
static mut sch311x_gpio_blocks: [sch311x_gpio_block_def; 6] = [
    sch311x_gpio_block_def { data_reg: 0x4b, config_regs: [0x23, 0x24, 0x25, 0x26, 0x27, 0x29, 0x2a, 0x2b], base: 10 },
    sch311x_gpio_block_def { data_reg: 0x4c, config_regs: [0x00, 0x2c, 0x2d, 0x00, 0x00, 0x00, 0x00, 0x32], base: 20 },
    sch311x_gpio_block_def { data_reg: 0x4d, config_regs: [0x33, 0x34, 0x35, 0x36, 0x37, 0x00, 0x39, 0x3a], base: 30 },
    sch311x_gpio_block_def { data_reg: 0x4e, config_regs: [0x3b, 0x00, 0x3d, 0x00, 0x6e, 0x6f, 0x72, 0x73], base: 40 },
    sch311x_gpio_block_def { data_reg: 0x4f, config_regs: [0x3f, 0x40, 0x41, 0x42, 0x43, 0x44, 0x45, 0x46], base: 50 },
    sch311x_gpio_block_def { data_reg: 0x50, config_regs: [0x47, 0x48, 0x54, 0x55, 0x56, 0x57, 0x58, 0x59], base: 60 },
];

extern "C" {
    type platform_device;
    type gpio_chip;
    type spinlock_t;
    fn request_muxed_region(start: i32, n: u32, name: *const u8) -> *mut core::ffi::c_void;
    fn release_region(start: u16, n: u32);
    fn request_region(start: u16, n: u32, name: *const u8) -> *mut core::ffi::c_void;
    fn outb(value: u8, port: i32);
    fn inb(port: i32) -> u8;
}

#[inline]
unsafe fn sch311x_sio_enter(sio_config_port: i32) -> i32 {
    if request_muxed_region(sio_config_port, 2, DRV_NAME.as_ptr()) .is_null() {
        return -16;
    }
    outb(SIO_CONFIG_KEY_ENTER, sio_config_port);
    0
}

#[inline]
unsafe fn sch311x_sio_exit(sio_config_port: i32) {
    outb(SIO_CONFIG_KEY_EXIT, sio_config_port);
    release_region(sio_config_port as u16, 2);
}

#[inline]
unsafe fn sch311x_sio_inb(sio_config_port: i32, reg: i32) -> u8 {
    outb(reg as u8, sio_config_port);
    inb(sio_config_port + 1)
}

#[inline]
unsafe fn sch311x_sio_outb(sio_config_port: i32, reg: i32, val: i32) {
    outb(reg as u8, sio_config_port);
    outb(val as u8, sio_config_port + 1);
}

extern "C" {
    fn gpiochip_get_data(chip: *mut gpio_chip) -> *mut sch311x_gpio_block;
    fn spin_lock(lock: *mut spinlock_t);
    fn spin_unlock(lock: *mut spinlock_t);
}

unsafe fn sch311x_gpio_request(chip: *mut gpio_chip, offset: u32) -> i32 {
    let block = gpiochip_get_data(chip);
    let reg = (*block).config_regs.add(offset as usize).read();
    if reg == 0 { return -19; }
    if request_region((*block).runtime_reg + reg, 1, DRV_NAME.as_ptr()).is_null() { return -16; }
    0
}

unsafe fn sch311x_gpio_free(chip: *mut gpio_chip, offset: u32) {
    let block = gpiochip_get_data(chip);
    let reg = (*block).config_regs.add(offset as usize).read();
    if reg != 0 { release_region((*block).runtime_reg + reg, 1); }
}

unsafe fn sch311x_gpio_get(chip: *mut gpio_chip, offset: u32) -> i32 {
    let block = gpiochip_get_data(chip);
    spin_lock(&mut (*block).lock);
    let data = inb(((*block).runtime_reg + (*block).data_reg) as i32);
    spin_unlock(&mut (*block).lock);
    if data & (1u8 << offset) != 0 { 1 } else { 0 }
}

unsafe fn __sch311x_gpio_set(block: *mut sch311x_gpio_block, offset: u32, value: i32) {
    let port = ((*block).runtime_reg + (*block).data_reg) as i32;
    let mut data = inb(port);
    if value != 0 { data |= 1u8 << offset; } else { data &= !(1u8 << offset); }
    outb(data, port);
}

unsafe fn sch311x_gpio_set(chip: *mut gpio_chip, offset: u32, value: i32) -> i32 {
    let block = gpiochip_get_data(chip);
    spin_lock(&mut (*block).lock); __sch311x_gpio_set(block, offset, value); spin_unlock(&mut (*block).lock); 0
}

unsafe fn sch311x_gpio_direction_in(chip: *mut gpio_chip, offset: u32) -> i32 {
    let block = gpiochip_get_data(chip); let port = ((*block).runtime_reg + (*block).config_regs.add(offset as usize).read()) as i32;
    spin_lock(&mut (*block).lock); let data = inb(port) | SCH311X_GPIO_CONF_DIR; outb(data, port); spin_unlock(&mut (*block).lock); 0
}

unsafe fn sch311x_gpio_direction_out(chip: *mut gpio_chip, offset: u32, value: i32) -> i32 {
    let block = gpiochip_get_data(chip); let port = ((*block).runtime_reg + (*block).config_regs.add(offset as usize).read()) as i32;
    spin_lock(&mut (*block).lock); let data = inb(port) & !SCH311X_GPIO_CONF_DIR; outb(data, port); __sch311x_gpio_set(block, offset, value); spin_unlock(&mut (*block).lock); 0
}

unsafe fn sch311x_gpio_get_direction(chip: *mut gpio_chip, offset: u32) -> i32 {
    let block = gpiochip_get_data(chip); let port = ((*block).runtime_reg + (*block).config_regs.add(offset as usize).read()) as i32;
    spin_lock(&mut (*block).lock); let data = inb(port); spin_unlock(&mut (*block).lock);
    if data & SCH311X_GPIO_CONF_DIR != 0 { 1 } else { 0 }
}

unsafe fn sch311x_gpio_set_config(_chip: *mut gpio_chip, _offset: u32, _config: u64) -> i32 { -95 }

unsafe fn sch311x_gpio_probe(_pdev: *mut platform_device) -> i32 { 0 }
unsafe fn sch311x_detect(sio_config_port: i32, addr: *mut u16) -> i32 {
    let err = sch311x_sio_enter(sio_config_port); if err != 0 { return err; }
    let id = sch311x_sio_inb(sio_config_port, 0x20);
    if id != 0x7c && id != 0x7d && id != 0x7f { sch311x_sio_exit(sio_config_port); return -19; }
    sch311x_sio_outb(sio_config_port, 7, 0x0a);
    let base = ((sch311x_sio_inb(sio_config_port, 0x60) as u16) << 8) | sch311x_sio_inb(sio_config_port, 0x61) as u16;
    if base == 0 { sch311x_sio_exit(sio_config_port); return -19; }
    *addr = base; sch311x_sio_exit(sio_config_port); 0
}
unsafe fn sch311x_gpio_pdev_add(_addr: u16) -> i32 { 0 }
unsafe fn sch311x_gpio_init() -> i32 {
    let mut addr = 0u16; for i in 0..4 { if sch311x_detect(sch311x_ioports[i], &mut addr) == 0 { break; } }
    if addr == 0 { return -19; } sch311x_gpio_pdev_add(addr)
}
unsafe fn sch311x_gpio_exit() {}

// module_init(sch311x_gpio_init); module_exit(sch311x_gpio_exit);
// MODULE_AUTHOR("Bruno Randolf <br1@einfach.org>");
// MODULE_DESCRIPTION("SMSC SCH311x GPIO Driver"); MODULE_LICENSE("GPL");
// MODULE_ALIAS("platform:gpio-sch311x");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
