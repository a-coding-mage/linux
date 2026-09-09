// SPDX-License-Identifier: GPL-2.0-only
/*
 * Purna Chandra Mandal, purna.mandal@microchip.com
 * Copyright (C) 2015 Microchip Technology Inc.  All rights reserved.
 */
// C dependencies supplied by the surrounding kernel translation.

const PIC32_CFGCON: u32 = 0x0000;
const PIC32_DEVID: u32 = 0x0020;
const PIC32_SYSKEY: u32 = 0x0030;
const PIC32_CFGEBIA: u32 = 0x00c0;
const PIC32_CFGEBIC: u32 = 0x00d0;
const PIC32_CFGCON2: u32 = 0x00f0;
const PIC32_RCON: u32 = 0x1240;

static mut pic32_conf_base: *mut core::ffi::c_void = core::ptr::null_mut();
static mut config_lock: core::ffi::c_void = core::mem::zeroed();
static mut pic32_reset_status: u32 = 0;

extern "C" {
    fn readl(addr: *mut core::ffi::c_void) -> u32;
    fn writel(value: u32, addr: *mut core::ffi::c_void);
    fn ioremap(addr: usize, size: usize) -> *mut core::ffi::c_void;
    fn panic(message: *const core::ffi::c_char) -> !;
    fn pr_debug(format: *const core::ffi::c_char, ...);
    fn pr_info(format: *const core::ffi::c_char, ...);
    fn spin_lock_irqsave(lock: *mut core::ffi::c_void, flags: *mut usize);
    fn spin_unlock_irqrestore(lock: *mut core::ffi::c_void, flags: usize);
    fn PIC32_CLR(addr: *mut core::ffi::c_void) -> *mut core::ffi::c_void;
    static PIC32_BASE_CONFIG: usize;
}

#[inline]
const fn bit(n: u32) -> u32 {
    1u32 << n
}

unsafe fn pic32_conf_get_reg_field(offset: u32, rshift: u32, mask: u32) -> u32 {
    let mut v: u32;

    v = readl((pic32_conf_base as *mut u8).add(offset as usize) as *mut core::ffi::c_void);
    v >>= rshift;
    v &= mask;

    v
}

unsafe fn pic32_conf_modify_atomic(offset: u32, mask: u32, set: u32) -> u32 {
    let mut v: u32;
    let mut flags: usize = 0;

    spin_lock_irqsave(&raw mut config_lock, &raw mut flags);
    v = readl((pic32_conf_base as *mut u8).add(offset as usize) as *mut core::ffi::c_void);
    v &= !mask;
    v |= set & mask;
    writel(v, (pic32_conf_base as *mut u8).add(offset as usize) as *mut core::ffi::c_void);
    spin_unlock_irqrestore(&raw mut config_lock, flags);

    0
}

#[no_mangle]
pub unsafe extern "C" fn pic32_enable_lcd() -> i32 {
    pic32_conf_modify_atomic(PIC32_CFGCON2, bit(31), bit(31)) as i32
}

#[no_mangle]
pub unsafe extern "C" fn pic32_disable_lcd() -> i32 {
    pic32_conf_modify_atomic(PIC32_CFGCON2, bit(31), 0) as i32
}

#[no_mangle]
pub unsafe extern "C" fn pic32_set_lcd_mode(mode: i32) -> i32 {
    let mask = if mode != 0 { bit(30) } else { 0 };

    pic32_conf_modify_atomic(PIC32_CFGCON2, bit(30), mask) as i32
}

#[no_mangle]
pub unsafe extern "C" fn pic32_set_sdhci_adma_fifo_threshold(rthrsh: u32, wthrsh: u32) -> i32 {
    let clr: u32;
    let set: u32;

    clr = (0x3ffu32 << 4) | (0x3ffu32 << 16);
    set = (rthrsh << 4) | (wthrsh << 16);
    pic32_conf_modify_atomic(PIC32_CFGCON2, clr, set) as i32
}

#[no_mangle]
pub unsafe extern "C" fn pic32_syskey_unlock_debug(func: *const core::ffi::c_char, line: usize) {
    let syskey = (pic32_conf_base as *mut u8).add(PIC32_SYSKEY as usize) as *mut core::ffi::c_void;

    // pr_debug("%s: called from %s:%lu\n", __func__, func, line);
    writel(0x00000000, syskey);
    writel(0xAA996655, syskey);
    writel(0x556699AA, syskey);
}

unsafe fn pic32_get_device_id() -> u32 {
    pic32_conf_get_reg_field(PIC32_DEVID, 0, 0x0fffffff)
}

unsafe fn pic32_get_device_version() -> u32 {
    pic32_conf_get_reg_field(PIC32_DEVID, 28, 0xf)
}

#[no_mangle]
pub unsafe extern "C" fn pic32_get_boot_status() -> u32 {
    pic32_reset_status
}

#[no_mangle]
pub unsafe extern "C" fn pic32_config_init() {
    pic32_conf_base = ioremap(PIC32_BASE_CONFIG, 0x110);
    if pic32_conf_base.is_null() {
        panic(b"pic32: config base not mapped\0".as_ptr() as *const core::ffi::c_char);
    }

    /* Boot Status */
    pic32_reset_status = readl((pic32_conf_base as *mut u8).add(PIC32_RCON as usize) as *mut core::ffi::c_void);
    writel(u32::MAX, PIC32_CLR((pic32_conf_base as *mut u8).add(PIC32_RCON as usize) as *mut core::ffi::c_void));

    /* Device Information */
    // pr_info("Device Id: 0x%08x, Device Ver: 0x%04x\n",
    //     pic32_get_device_id(), pic32_get_device_version());
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
