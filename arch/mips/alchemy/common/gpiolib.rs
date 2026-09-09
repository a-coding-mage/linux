/*
 *  Copyright (C) 2007-2009, OpenWrt.org, Florian Fainelli <florian@openwrt.org>
 *	GPIOLIB support for Alchemy chips.
 *
 *  This program is free software; you can redistribute  it and/or modify it
 *  under the terms of the GNU General Public License as published by the
 *  Free Software Foundation; either version 2 of the License, or (at your
 *  option) any later version.
 *
 *  THIS SOFTWARE IS PROVIDED ``AS IS'' AND ANY EXPRESS OR IMPLIED
 *  WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE IMPLIED WARRANTIES OF
 *  MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE DISCLAIMED.
 */

use core::ffi::c_void;

// Linux kernel types, constants, GPIO helpers, and initcall registration are
// supplied by the surrounding translation unit/dependencies.
extern "C" {
    fn alchemy_gpio2_get_value(gpio: u32) -> i32;
    fn alchemy_gpio2_set_value(gpio: u32, value: i32);
    fn alchemy_gpio2_direction_input(gpio: u32) -> i32;
    fn alchemy_gpio2_direction_output(gpio: u32, value: i32) -> i32;
    fn alchemy_gpio2_to_irq(gpio: u32) -> i32;
    fn alchemy_gpio1_get_value(gpio: u32) -> i32;
    fn alchemy_gpio1_set_value(gpio: u32, value: i32);
    fn alchemy_gpio1_direction_input(gpio: u32) -> i32;
    fn alchemy_gpio1_direction_output(gpio: u32, value: i32) -> i32;
    fn alchemy_gpio1_to_irq(gpio: u32) -> i32;
    fn au1300_gpio_get_value(gpio: u32) -> i32;
    fn au1300_gpio_set_value(gpio: u32, value: i32);
    fn au1300_gpio_direction_input(gpio: u32) -> i32;
    fn au1300_gpio_direction_output(gpio: u32, value: i32) -> i32;
    fn au1300_gpio_to_irq(gpio: u32) -> i32;
    fn software_node_register_node_group(group: *const *const SoftwareNode) -> i32;
    fn software_node_fwnode(node: *const SoftwareNode) -> *mut c_void;
    fn gpiochip_add_data(chip: *mut GpioChip, data: *mut c_void) -> i32;
    fn alchemy_get_cputype() -> i32;
}

#[repr(C)]
pub struct SoftwareNode {
    pub name: *const u8,
}

#[repr(C)]
pub struct GpioChip {
    pub label: *const u8,
    pub direction_input: Option<unsafe extern "C" fn(*mut GpioChip, u32) -> i32>,
    pub direction_output: Option<unsafe extern "C" fn(*mut GpioChip, u32, i32) -> i32>,
    pub get: Option<unsafe extern "C" fn(*mut GpioChip, u32) -> i32>,
    pub set: Option<unsafe extern "C" fn(*mut GpioChip, u32, i32)>,
    pub to_irq: Option<unsafe extern "C" fn(*mut GpioChip, u32) -> i32>,
    pub base: i32,
    pub ngpio: u32,
    pub fwnode: *mut c_void,
}

// Constants are provided by the Alchemy GPIO headers.
extern "C" {
    static ALCHEMY_GPIO1_BASE: u32;
    static ALCHEMY_GPIO2_BASE: u32;
    static ALCHEMY_GPIO1_NUM: u32;
    static ALCHEMY_GPIO2_NUM: u32;
    static AU1300_GPIO_BASE: u32;
    static AU1300_GPIO_NUM: u32;
    static ALCHEMY_CPU_AU1000: i32;
    static ALCHEMY_CPU_AU1500: i32;
    static ALCHEMY_CPU_AU1200: i32;
    static ALCHEMY_CPU_AU1300: i32;
}

unsafe extern "C" fn gpio2_get(_chip: *mut GpioChip, offset: u32) -> i32 {
    (alchemy_gpio2_get_value(offset + ALCHEMY_GPIO2_BASE) != 0) as i32
}

unsafe extern "C" fn gpio2_set(_chip: *mut GpioChip, offset: u32, value: i32) {
    alchemy_gpio2_set_value(offset + ALCHEMY_GPIO2_BASE, value);
}

unsafe extern "C" fn gpio2_direction_input(_chip: *mut GpioChip, offset: u32) -> i32 {
    alchemy_gpio2_direction_input(offset + ALCHEMY_GPIO2_BASE)
}

unsafe extern "C" fn gpio2_direction_output(_chip: *mut GpioChip, offset: u32, value: i32) -> i32 {
    alchemy_gpio2_direction_output(offset + ALCHEMY_GPIO2_BASE, value)
}

unsafe extern "C" fn gpio2_to_irq(_chip: *mut GpioChip, offset: u32) -> i32 {
    alchemy_gpio2_to_irq(offset + ALCHEMY_GPIO2_BASE)
}

unsafe extern "C" fn gpio1_get(_chip: *mut GpioChip, offset: u32) -> i32 {
    (alchemy_gpio1_get_value(offset + ALCHEMY_GPIO1_BASE) != 0) as i32
}

unsafe extern "C" fn gpio1_set(_chip: *mut GpioChip, offset: u32, value: i32) {
    alchemy_gpio1_set_value(offset + ALCHEMY_GPIO1_BASE, value);
}

unsafe extern "C" fn gpio1_direction_input(_chip: *mut GpioChip, offset: u32) -> i32 {
    alchemy_gpio1_direction_input(offset + ALCHEMY_GPIO1_BASE)
}

unsafe extern "C" fn gpio1_direction_output(_chip: *mut GpioChip, offset: u32, value: i32) -> i32 {
    alchemy_gpio1_direction_output(offset + ALCHEMY_GPIO1_BASE, value)
}

unsafe extern "C" fn gpio1_to_irq(_chip: *mut GpioChip, offset: u32) -> i32 {
    alchemy_gpio1_to_irq(offset + ALCHEMY_GPIO1_BASE)
}

pub static ALCHEMY_GPIO1_NODE: SoftwareNode = SoftwareNode { name: b"alchemy-gpio1\0".as_ptr() };
pub static ALCHEMY_GPIO2_NODE: SoftwareNode = SoftwareNode { name: b"alchemy-gpio2\0".as_ptr() };
pub static ALCHEMY_GPIC_NODE: SoftwareNode = SoftwareNode { name: b"alchemy-gpic\0".as_ptr() };

static ALCHEMY_GPIO_NODE_GROUP: [*const SoftwareNode; 4] = [
    &ALCHEMY_GPIO1_NODE,
    &ALCHEMY_GPIO2_NODE,
    &ALCHEMY_GPIC_NODE,
    core::ptr::null(),
];

static mut ALCHEMY_GPIO_CHIP: [GpioChip; 2] = [
    GpioChip { label: b"alchemy-gpio1\0".as_ptr(), direction_input: Some(gpio1_direction_input), direction_output: Some(gpio1_direction_output), get: Some(gpio1_get), set: Some(gpio1_set), to_irq: Some(gpio1_to_irq), base: 0, ngpio: 0, fwnode: core::ptr::null_mut() },
    GpioChip { label: b"alchemy-gpio2\0".as_ptr(), direction_input: Some(gpio2_direction_input), direction_output: Some(gpio2_direction_output), get: Some(gpio2_get), set: Some(gpio2_set), to_irq: Some(gpio2_to_irq), base: 0, ngpio: 0, fwnode: core::ptr::null_mut() },
];

unsafe extern "C" fn alchemy_gpic_get(_chip: *mut GpioChip, off: u32) -> i32 {
    (au1300_gpio_get_value(off + AU1300_GPIO_BASE) != 0) as i32
}
unsafe extern "C" fn alchemy_gpic_set(_chip: *mut GpioChip, off: u32, v: i32) { au1300_gpio_set_value(off + AU1300_GPIO_BASE, v); }
unsafe extern "C" fn alchemy_gpic_dir_input(_chip: *mut GpioChip, off: u32) -> i32 { au1300_gpio_direction_input(off + AU1300_GPIO_BASE) }
unsafe extern "C" fn alchemy_gpic_dir_output(_chip: *mut GpioChip, off: u32, v: i32) -> i32 { au1300_gpio_direction_output(off + AU1300_GPIO_BASE, v) }
unsafe extern "C" fn alchemy_gpic_gpio_to_irq(_chip: *mut GpioChip, off: u32) -> i32 { au1300_gpio_to_irq(off + AU1300_GPIO_BASE) }

static mut AU1300_GPIOCHIP: GpioChip = GpioChip { label: b"alchemy-gpic\0".as_ptr(), direction_input: Some(alchemy_gpic_dir_input), direction_output: Some(alchemy_gpic_dir_output), get: Some(alchemy_gpic_get), set: Some(alchemy_gpic_set), to_irq: Some(alchemy_gpic_gpio_to_irq), base: 0, ngpio: 0, fwnode: core::ptr::null_mut() };

unsafe fn alchemy_gpio_nodes_init() -> i32 {
    let ret = software_node_register_node_group(ALCHEMY_GPIO_NODE_GROUP.as_ptr());
    if ret != 0 { return ret; }
    ALCHEMY_GPIO_CHIP[0].fwnode = software_node_fwnode(&ALCHEMY_GPIO1_NODE);
    ALCHEMY_GPIO_CHIP[1].fwnode = software_node_fwnode(&ALCHEMY_GPIO2_NODE);
    AU1300_GPIOCHIP.fwnode = software_node_fwnode(&ALCHEMY_GPIC_NODE);
    0
}

unsafe fn alchemy_gpiochip_init() -> i32 {
    let mut ret = 0;
    match alchemy_get_cputype() {
        x if x == ALCHEMY_CPU_AU1000 => ret = gpiochip_add_data(&mut ALCHEMY_GPIO_CHIP[0], core::ptr::null_mut()),
        x if x >= ALCHEMY_CPU_AU1500 && x <= ALCHEMY_CPU_AU1200 => {
            ret = gpiochip_add_data(&mut ALCHEMY_GPIO_CHIP[0], core::ptr::null_mut());
            ret |= gpiochip_add_data(&mut ALCHEMY_GPIO_CHIP[1], core::ptr::null_mut());
        }
        x if x == ALCHEMY_CPU_AU1300 => ret = gpiochip_add_data(&mut AU1300_GPIOCHIP, core::ptr::null_mut()),
        _ => {}
    }
    ret
}

// postcore_initcall(alchemy_gpio_nodes_init);
// arch_initcall(alchemy_gpiochip_init);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
