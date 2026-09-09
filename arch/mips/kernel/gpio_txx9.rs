// SPDX-License-Identifier: GPL-2.0-only
/*
 * A gpio chip driver for TXx9 SoCs
 *
 * Copyright (C) 2008 Atsushi Nemoto <anemo@mba.ocn.ne.jp>
 */

// Dependencies supplied by the kernel and architecture headers are declared
// here as external Rust items.

#[repr(C)]
pub struct txx9_pio_reg {
    pub din: u32,
    pub dout: u32,
    pub dir: u32,
}

#[repr(C)]
pub struct gpio_chip {
    pub get: Option<unsafe extern "C" fn(*mut gpio_chip, u32) -> i32>,
    pub set: Option<unsafe extern "C" fn(*mut gpio_chip, u32, i32)>,
    pub direction_input: Option<unsafe extern "C" fn(*mut gpio_chip, u32) -> i32>,
    pub direction_output: Option<unsafe extern "C" fn(*mut gpio_chip, u32, i32) -> i32>,
    pub label: *const core::ffi::c_char,
    pub base: i32,
    pub ngpio: u32,
}

extern "C" {
    fn ioremap(baseaddr: usize, size: usize) -> *mut txx9_pio_reg;
    fn gpiochip_add_data(chip: *mut gpio_chip, data: *mut core::ffi::c_void) -> i32;
    fn __raw_readl(addr: *const u32) -> u32;
    fn __raw_writel(value: u32, addr: *mut u32);
    fn mmiowb();
    fn spin_lock_irqsave(lock: *mut u8, flags: *mut usize);
    fn spin_unlock_irqrestore(lock: *mut u8, flags: usize);
}

static mut txx9_gpio_lock: u8 = 0;
static mut txx9_pioptr: *mut txx9_pio_reg = core::ptr::null_mut();

unsafe extern "C" fn txx9_gpio_get(_chip: *mut gpio_chip, offset: u32) -> i32 {
    (__raw_readl(&(*txx9_pioptr).din as *const u32) & (1u32 << offset) != 0) as i32
}

unsafe fn txx9_gpio_set_raw(offset: u32, value: i32) {
    let mut val: u32;
    val = __raw_readl(&(*txx9_pioptr).dout as *const u32);
    if value != 0 {
        val |= 1u32 << offset;
    } else {
        val &= !(1u32 << offset);
    }
    __raw_writel(val, &mut (*txx9_pioptr).dout as *mut u32);
}

unsafe extern "C" fn txx9_gpio_set(_chip: *mut gpio_chip, offset: u32, value: i32) {
    let mut flags: usize = 0;
    spin_lock_irqsave(&raw mut txx9_gpio_lock, &mut flags);
    txx9_gpio_set_raw(offset, value);
    mmiowb();
    spin_unlock_irqrestore(&raw mut txx9_gpio_lock, flags);
}

unsafe extern "C" fn txx9_gpio_dir_in(_chip: *mut gpio_chip, offset: u32) -> i32 {
    let mut flags: usize = 0;
    spin_lock_irqsave(&raw mut txx9_gpio_lock, &mut flags);
    __raw_writel(
        __raw_readl(&(*txx9_pioptr).dir as *const u32) & !(1u32 << offset),
        &mut (*txx9_pioptr).dir as *mut u32,
    );
    mmiowb();
    spin_unlock_irqrestore(&raw mut txx9_gpio_lock, flags);
    0
}

unsafe extern "C" fn txx9_gpio_dir_out(_chip: *mut gpio_chip, offset: u32, value: i32) -> i32 {
    let mut flags: usize = 0;
    spin_lock_irqsave(&raw mut txx9_gpio_lock, &mut flags);
    txx9_gpio_set_raw(offset, value);
    __raw_writel(
        __raw_readl(&(*txx9_pioptr).dir as *const u32) | (1u32 << offset),
        &mut (*txx9_pioptr).dir as *mut u32,
    );
    mmiowb();
    spin_unlock_irqrestore(&raw mut txx9_gpio_lock, flags);
    0
}

static mut txx9_gpio_chip: gpio_chip = gpio_chip {
    get: Some(txx9_gpio_get),
    set: Some(txx9_gpio_set),
    direction_input: Some(txx9_gpio_dir_in),
    direction_output: Some(txx9_gpio_dir_out),
    label: b"TXx9\0".as_ptr() as *const core::ffi::c_char,
    base: 0,
    ngpio: 0,
};

pub unsafe extern "C" fn txx9_gpio_init(baseaddr: usize, num: u32) -> i32 {
    txx9_pioptr = ioremap(baseaddr, core::mem::size_of::<txx9_pio_reg>());
    if txx9_pioptr.is_null() {
        return -19; // -ENODEV
    }
    txx9_gpio_chip.base = -1;
    txx9_gpio_chip.ngpio = num;
    gpiochip_add_data(&raw mut txx9_gpio_chip, core::ptr::null_mut())
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
