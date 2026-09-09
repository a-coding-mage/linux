/* SPDX-License-Identifier: GPL-2.0-or-later */
/* Copyright (c) 2004 Evgeniy Polyakov <zbr@ioremap.net> */

// Translated from linux/w1.h. The original include and header guards are omitted.
// Kernel-provided types and macros (device, list_head, atomic_t, container_of,
// and related declarations) remain external dependencies.

#[repr(C)]
pub struct w1_reg_num {
    // __u64 family:8, id:48, crc:8 in little-endian builds, and
    // __u64 crc:8, id:48, family:8 in big-endian builds.
    pub value: u64,
}

pub const W1_MAXNAMELEN: usize = 32;

pub const W1_SEARCH: u8 = 0xF0;
pub const W1_ALARM_SEARCH: u8 = 0xEC;
pub const W1_CONVERT_TEMP: u8 = 0x44;
pub const W1_SKIP_ROM: u8 = 0xCC;
pub const W1_COPY_SCRATCHPAD: u8 = 0x48;
pub const W1_WRITE_SCRATCHPAD: u8 = 0x4E;
pub const W1_READ_SCRATCHPAD: u8 = 0xBE;
pub const W1_READ_ROM: u8 = 0x33;
pub const W1_READ_PSUPPLY: u8 = 0xB4;
pub const W1_MATCH_ROM: u8 = 0x55;
pub const W1_RESUME_CMD: u8 = 0xA5;

#[repr(C)]
pub struct w1_slave {
    pub owner: *mut module,
    pub name: [u8; W1_MAXNAMELEN],
    pub w1_slave_entry: list_head,
    pub reg_num: w1_reg_num,
    pub refcnt: atomic_t,
    pub ttl: i32,
    pub flags: c_ulong,
    pub master: *mut w1_master,
    pub family: *mut w1_family,
    pub family_data: *mut core::ffi::c_void,
    pub dev: device,
    pub hwmon: *mut device,
}

pub type w1_slave_found_callback = unsafe extern "C" fn(*mut w1_master, u64);

#[repr(C)]
pub struct w1_bus_master {
    pub data: *mut core::ffi::c_void,
    pub read_bit: Option<unsafe extern "C" fn(*mut core::ffi::c_void) -> u8>,
    pub write_bit: Option<unsafe extern "C" fn(*mut core::ffi::c_void, u8)>,
    pub touch_bit: Option<unsafe extern "C" fn(*mut core::ffi::c_void, u8) -> u8>,
    pub read_byte: Option<unsafe extern "C" fn(*mut core::ffi::c_void) -> u8>,
    pub write_byte: Option<unsafe extern "C" fn(*mut core::ffi::c_void, u8)>,
    pub read_block: Option<unsafe extern "C" fn(*mut core::ffi::c_void, *mut u8, i32) -> u8>,
    pub write_block: Option<unsafe extern "C" fn(*mut core::ffi::c_void, *const u8, i32)>,
    pub triplet: Option<unsafe extern "C" fn(*mut core::ffi::c_void, u8) -> u8>,
    pub reset_bus: Option<unsafe extern "C" fn(*mut core::ffi::c_void) -> u8>,
    pub set_pullup: Option<unsafe extern "C" fn(*mut core::ffi::c_void, i32) -> u8>,
    pub search: Option<unsafe extern "C" fn(*mut core::ffi::c_void, *mut w1_master, u8, w1_slave_found_callback)>,
    pub dev_id: *mut i8,
}

#[repr(C)]
pub enum w1_master_flags {
    W1_ABORT_SEARCH = 0,
    W1_WARN_MAX_COUNT = 1,
}

#[repr(C)]
pub struct w1_master {
    pub w1_master_entry: list_head,
    pub owner: *mut module,
    pub name: [u8; W1_MAXNAMELEN],
    pub list_mutex: mutex,
    pub slist: list_head,
    pub async_list: list_head,
    pub max_slave_count: i32,
    pub slave_count: i32,
    pub attempts: c_ulong,
    pub slave_ttl: i32,
    pub initialized: i32,
    pub id: u32,
    pub search_count: i32,
    pub search_id: u64,
    pub refcnt: atomic_t,
    pub priv_: *mut core::ffi::c_void,
    pub enable_pullup: i32,
    pub pullup_duration: i32,
    pub flags: i64,
    pub thread: *mut task_struct,
    pub mutex: mutex,
    pub bus_mutex: mutex,
    pub driver: *mut device_driver,
    pub dev: device,
    pub bus_master: *mut w1_bus_master,
    pub seq: u32,
}

extern "C" {
    pub fn w1_add_master_device(master: *mut w1_bus_master) -> i32;
    pub fn w1_remove_master_device(master: *mut w1_bus_master);
}

#[repr(C)]
pub struct w1_family_ops {
    pub add_slave: Option<unsafe extern "C" fn(*mut w1_slave) -> i32>,
    pub remove_slave: Option<unsafe extern "C" fn(*mut w1_slave)>,
    pub groups: *const *const attribute_group,
    pub chip_info: *const hwmon_chip_info,
}

#[repr(C)]
pub struct w1_family {
    pub family_entry: list_head,
    pub fid: u8,
    pub fops: *const w1_family_ops,
    pub of_match_table: *const of_device_id,
    pub refcnt: atomic_t,
}

extern "C" {
    pub fn w1_register_family(family: *mut w1_family) -> i32;
    pub fn w1_unregister_family(family: *mut w1_family);
    pub fn w1_triplet(dev: *mut w1_master, bdir: i32) -> u8;
    pub fn w1_touch_bit(dev: *mut w1_master, bit: i32) -> u8;
    pub fn w1_write_8(dev: *mut w1_master, value: u8);
    pub fn w1_read_8(dev: *mut w1_master) -> u8;
    pub fn w1_reset_bus(dev: *mut w1_master) -> i32;
    pub fn w1_calc_crc8(data: *mut u8, len: i32) -> u8;
    pub fn w1_write_block(dev: *mut w1_master, data: *const u8, len: i32);
    pub fn w1_touch_block(dev: *mut w1_master, data: *mut u8, len: i32);
    pub fn w1_read_block(dev: *mut w1_master, data: *mut u8, len: i32) -> u8;
    pub fn w1_reset_select_slave(sl: *mut w1_slave) -> i32;
    pub fn w1_reset_resume_command(dev: *mut w1_master) -> i32;
    pub fn w1_next_pullup(dev: *mut w1_master, delay: i32);
}

#[inline]
pub unsafe fn dev_to_w1_slave(dev: *mut device) -> *mut w1_slave {
    container_of!(dev, w1_slave, dev)
}

#[inline]
pub unsafe fn kobj_to_w1_slave(kobj: *mut kobject) -> *mut w1_slave {
    dev_to_w1_slave(container_of!(kobj, device, kobj))
}

#[inline]
pub unsafe fn dev_to_w1_master(dev: *mut device) -> *mut w1_master {
    container_of!(dev, w1_master, dev)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
