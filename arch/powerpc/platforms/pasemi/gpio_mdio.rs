// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 2006-2007 PA Semi, Inc
 *
 * Author: Olof Johansson, PA Semi
 *
 * Maintained by: Olof Johansson <olof@lixom.net>
 *
 * Based on drivers/net/fs_enet/mii-bitbang.c.
 */

const DELAY: u32 = 1;

extern "C" {
    static mut gpio_regs: *mut core::ffi::c_void;
}

#[repr(C)]
struct gpio_priv {
    mdc_pin: i32,
    mdio_pin: i32,
}

#[allow(non_camel_case_types)]
type u8 = core::ffi::c_uchar;
#[allow(non_camel_case_types)]
type u16 = core::ffi::c_ushort;

#[repr(C)]
struct mii_bus {
    name: *const core::ffi::c_char,
    read: Option<unsafe extern "C" fn(*mut mii_bus, i32, i32) -> i32>,
    write: Option<unsafe extern "C" fn(*mut mii_bus, i32, i32, u16) -> i32>,
    reset: Option<unsafe extern "C" fn(*mut mii_bus) -> i32>,
    id: [core::ffi::c_char; 32],
    priv_: *mut core::ffi::c_void,
    parent: *mut device,
}

#[repr(C)]
struct device { _private: [u8; 0] }
#[repr(C)]
struct device_node { _private: [u8; 0] }
#[repr(C)]
struct platform_device { dev: device }
#[repr(C)]
struct of_device_id { compatible: *const core::ffi::c_char }
#[repr(C)]
struct platform_driver { probe: Option<unsafe extern "C" fn(*mut platform_device) -> i32>, remove: Option<unsafe extern "C" fn(*mut platform_device)>, driver: driver }
#[repr(C)]
struct driver { name: *const core::ffi::c_char, of_match_table: *const of_device_id }

extern "C" {
    fn out_le32(addr: *mut core::ffi::c_void, value: u32);
    fn in_le32(addr: *mut core::ffi::c_void) -> u32;
    fn udelay(usecs: u32);
    fn kzalloc_obj<T>() -> *mut T;
    fn kfree(ptr: *mut core::ffi::c_void);
    fn mdiobus_alloc() -> *mut mii_bus;
    fn mdiobus_free(bus: *mut mii_bus);
    fn mdiobus_unregister(bus: *mut mii_bus);
    fn of_get_property(np: *mut device_node, name: *const core::ffi::c_char, len: *mut usize) -> *const u32;
    fn of_mdiobus_register(bus: *mut mii_bus, np: *mut device_node) -> i32;
    fn dev_set_drvdata(dev: *mut device, data: *mut core::ffi::c_void);
    fn dev_get_drvdata(dev: *mut device) -> *mut core::ffi::c_void;
    fn of_find_compatible_node(from: *mut device_node, typ: *const core::ffi::c_char, compatible: *const core::ffi::c_char) -> *mut device_node;
    fn of_iomap(np: *mut device_node, index: i32) -> *mut core::ffi::c_void;
    fn of_node_put(np: *mut device_node);
    fn platform_driver_register(driver: *mut platform_driver) -> i32;
    fn platform_driver_unregister(driver: *mut platform_driver);
    fn iounmap(addr: *mut core::ffi::c_void);
}

unsafe fn mdio_lo(bus: *mut mii_bus) { out_le32((gpio_regs as *mut u8).add(0x10) as _, 1u32 << (*( (*bus).priv_ as *mut gpio_priv)).mdio_pin); }
unsafe fn mdio_hi(bus: *mut mii_bus) { out_le32(gpio_regs, 1u32 << (*( (*bus).priv_ as *mut gpio_priv)).mdio_pin); }
unsafe fn mdc_lo(bus: *mut mii_bus) { out_le32((gpio_regs as *mut u8).add(0x10) as _, 1u32 << (*( (*bus).priv_ as *mut gpio_priv)).mdc_pin); }
unsafe fn mdc_hi(bus: *mut mii_bus) { out_le32(gpio_regs, 1u32 << (*( (*bus).priv_ as *mut gpio_priv)).mdc_pin); }
unsafe fn mdio_active(bus: *mut mii_bus) { let p = (*bus).priv_ as *mut gpio_priv; out_le32((gpio_regs as *mut u8).add(0x20) as _, (1u32 << (*p).mdc_pin) | (1u32 << (*p).mdio_pin)); }
unsafe fn mdio_tristate(bus: *mut mii_bus) { out_le32((gpio_regs as *mut u8).add(0x30) as _, 1u32 << (*( (*bus).priv_ as *mut gpio_priv)).mdio_pin); }
unsafe fn mdio_read(bus: *mut mii_bus) -> i32 { ((in_le32((gpio_regs as *mut u8).add(0x40) as _) & (1u32 << (*( (*bus).priv_ as *mut gpio_priv)).mdio_pin)) != 0) as i32 }

unsafe fn clock_out(bus: *mut mii_bus, bit: i32) { if bit != 0 { mdio_hi(bus); } else { mdio_lo(bus); } udelay(DELAY); mdc_hi(bus); udelay(DELAY); mdc_lo(bus); }

unsafe fn bitbang_pre(bus: *mut mii_bus, read: i32, mut addr: u8, mut reg: u8) {
    mdio_active(bus);
    for _ in 0..40 { clock_out(bus, 1); }
    clock_out(bus, 0); clock_out(bus, 1); clock_out(bus, read); clock_out(bus, (!read) & 1);
    for _ in 0..5 { clock_out(bus, ((addr & 0x10) != 0) as i32); addr <<= 1; }
    for _ in 0..5 { clock_out(bus, ((reg & 0x10) != 0) as i32); reg <<= 1; }
}

unsafe extern "C" fn gpio_mdio_read(bus: *mut mii_bus, phy_id: i32, location: i32) -> i32 {
    bitbang_pre(bus, 1, (phy_id & 0xff) as u8, (location & 0xff) as u8); mdio_tristate(bus); udelay(DELAY); mdc_hi(bus); udelay(DELAY); mdc_lo(bus);
    let mut rdreg: u16 = 0; for _ in 0..16 { mdc_lo(bus); udelay(DELAY); mdc_hi(bus); udelay(DELAY); mdc_lo(bus); udelay(DELAY); rdreg = (rdreg << 1) | mdio_read(bus) as u16; }
    mdc_hi(bus); udelay(DELAY); mdc_lo(bus); udelay(DELAY); rdreg as i32
}

unsafe extern "C" fn gpio_mdio_write(bus: *mut mii_bus, phy_id: i32, location: i32, val: u16) -> i32 {
    bitbang_pre(bus, 0, (phy_id & 0xff) as u8, (location & 0xff) as u8); mdc_lo(bus); mdio_hi(bus); udelay(DELAY); mdc_hi(bus); udelay(DELAY); mdc_lo(bus); mdio_lo(bus); udelay(DELAY); mdc_hi(bus); udelay(DELAY);
    let mut value = val; for _ in 0..16 { mdc_lo(bus); if value & 0x8000 != 0 { mdio_hi(bus); } else { mdio_lo(bus); } udelay(DELAY); mdc_hi(bus); udelay(DELAY); value <<= 1; }
    mdio_tristate(bus); mdc_lo(bus); udelay(DELAY); mdc_hi(bus); udelay(DELAY); 0
}

unsafe extern "C" fn gpio_mdio_reset(_bus: *mut mii_bus) -> i32 { 0 }

unsafe extern "C" fn gpio_mdio_probe(ofdev: *mut platform_device) -> i32 {
    let dev = &mut (*ofdev).dev as *mut device; let np = core::ptr::null_mut::<device_node>();
    let priv_ = kzalloc_obj::<gpio_priv>(); if priv_.is_null() { return -12; }
    let bus = mdiobus_alloc(); if bus.is_null() { kfree(priv_ as _); return -12; }
    (*bus).name = b"pasemi gpio mdio bus\0".as_ptr() as _; (*bus).read = Some(gpio_mdio_read); (*bus).write = Some(gpio_mdio_write); (*bus).reset = Some(gpio_mdio_reset); (*bus).priv_ = priv_ as _; (*bus).parent = dev;
    dev_set_drvdata(dev, bus as _); let err = of_mdiobus_register(bus, np); if err != 0 { kfree(bus as _); kfree(priv_ as _); return err; } 0
}

unsafe extern "C" fn gpio_mdio_remove(dev: *mut platform_device) { let bus = dev_get_drvdata(&mut (*dev).dev) as *mut mii_bus; mdiobus_unregister(bus); dev_set_drvdata(&mut (*dev).dev, core::ptr::null_mut()); kfree((*bus).priv_); (*bus).priv_ = core::ptr::null_mut(); mdiobus_free(bus); }

static mut GPIO_MDIO_MATCH: [of_device_id; 2] = [of_device_id { compatible: b"gpio-mdio\0".as_ptr() as _ }, of_device_id { compatible: core::ptr::null() }];
static mut GPIO_MDIO_DRIVER: platform_driver = platform_driver { probe: Some(gpio_mdio_probe), remove: Some(gpio_mdio_remove), driver: driver { name: b"gpio-mdio-bitbang\0".as_ptr() as _, of_match_table: GPIO_MDIO_MATCH.as_ptr() } };

unsafe extern "C" fn gpio_mdio_init() -> i32 { let mut np = of_find_compatible_node(core::ptr::null_mut(), core::ptr::null(), b"1682m-gpio\0".as_ptr() as _); if np.is_null() { np = of_find_compatible_node(core::ptr::null_mut(), core::ptr::null(), b"pasemi,pwrficient-gpio\0".as_ptr() as _); } if np.is_null() { return -19; } gpio_regs = of_iomap(np, 0); of_node_put(np); if gpio_regs.is_null() { return -19; } platform_driver_register(&mut GPIO_MDIO_DRIVER) }
unsafe extern "C" fn gpio_mdio_exit() { platform_driver_unregister(&mut GPIO_MDIO_DRIVER); if !gpio_regs.is_null() { iounmap(gpio_regs); } }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
