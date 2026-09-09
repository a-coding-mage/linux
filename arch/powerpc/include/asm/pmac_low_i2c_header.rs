/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 *  include/asm-ppc/pmac_low_i2c.h
 *
 *  Copyright (C) 2003 Ben. Herrenschmidt (benh@kernel.crashing.org)
 */

/* Declarations from the C header guard and __KERNEL__ conditional. */

/* i2c mode (based on the platform functions format) */
pub const pmac_i2c_mode_dumb: i32 = 1;
pub const pmac_i2c_mode_std: i32 = 2;
pub const pmac_i2c_mode_stdsub: i32 = 3;
pub const pmac_i2c_mode_combined: i32 = 4;

/* RW bit in address */
pub const pmac_i2c_read: i32 = 0x01;
pub const pmac_i2c_write: i32 = 0x00;

/* i2c bus type */
pub const pmac_i2c_bus_keywest: i32 = 0;
pub const pmac_i2c_bus_pmu: i32 = 1;
pub const pmac_i2c_bus_smu: i32 = 2;

/* i2c bus features */
/* can_largesub : supports >1 byte subaddresses (SMU only) */
pub const pmac_i2c_can_largesub: u32 = 0x00000001u32;

/* multibus : device node holds multiple busses, bus number is
 * encoded in bits 0xff00 of "reg" of a given device
 */
pub const pmac_i2c_multibus: u32 = 0x00000002u32;

/* i2c busses in the system */
#[repr(C)]
pub struct pmac_i2c_bus {
    _private: [u8; 0],
}

#[repr(C)]
pub struct i2c_adapter {
    _private: [u8; 0],
}

#[repr(C)]
pub struct device_node {
    _private: [u8; 0],
}

extern "C" {
    /* Init, called early during boot */
    pub fn pmac_i2c_init() -> i32;

    /* Lookup an i2c bus for a device-node. The node can be either the bus
     * node itself or a device below it. In the case of a multibus, the bus
     * node itself is the controller node, else, it's a child of the controller
     * node
     */
    pub fn pmac_i2c_find_bus(node: *mut device_node) -> *mut pmac_i2c_bus;

    /* Get the address for an i2c device. This strips the bus number if
     * necessary. The 7 bits address is returned 1 bit right shifted so that
     * the direction can be directly ored in
     */
    pub fn pmac_i2c_get_dev_addr(device: *mut device_node) -> u8;

    /* Get infos about a bus */
    pub fn pmac_i2c_get_controller(bus: *mut pmac_i2c_bus) -> *mut device_node;
    pub fn pmac_i2c_get_bus_node(bus: *mut pmac_i2c_bus) -> *mut device_node;
    pub fn pmac_i2c_get_type(bus: *mut pmac_i2c_bus) -> i32;
    pub fn pmac_i2c_get_flags(bus: *mut pmac_i2c_bus) -> i32;
    pub fn pmac_i2c_get_channel(bus: *mut pmac_i2c_bus) -> i32;

    /* i2c layer adapter helpers */
    pub fn pmac_i2c_get_adapter(bus: *mut pmac_i2c_bus) -> *mut i2c_adapter;
    pub fn pmac_i2c_adapter_to_bus(adapter: *mut i2c_adapter) -> *mut pmac_i2c_bus;

    /* March a device or bus with an i2c adapter structure, to be used by drivers
     * to match device-tree nodes with i2c adapters during adapter discovery
     * callbacks
     */
    pub fn pmac_i2c_match_adapter(dev: *mut device_node, adapter: *mut i2c_adapter) -> i32;

    /* Access functions for platform code */
    pub fn pmac_i2c_open(bus: *mut pmac_i2c_bus, polled: i32) -> i32;
    pub fn pmac_i2c_close(bus: *mut pmac_i2c_bus);
    pub fn pmac_i2c_setmode(bus: *mut pmac_i2c_bus, mode: i32) -> i32;
    pub fn pmac_i2c_xfer(
        bus: *mut pmac_i2c_bus,
        addrdir: u8,
        subsize: i32,
        subaddr: u32,
        data: *mut u8,
        len: i32,
    ) -> i32;

    /* Suspend/resume code called by via-pmu directly for now */
    pub fn pmac_pfunc_i2c_suspend();
    pub fn pmac_pfunc_i2c_resume();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
