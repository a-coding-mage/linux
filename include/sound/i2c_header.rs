/* SPDX-License-Identifier: GPL-2.0-or-later */

use core::ffi::{c_char, c_int, c_ulong, c_ushort, c_void};

pub const SND_I2C_DEVICE_ADDRTEN: c_int = 1 << 0; /* 10-bit I2C address */

#[repr(C)]
pub struct snd_i2c_device {
    pub list: list_head,
    pub bus: *mut snd_i2c_bus, /* I2C bus */
    pub name: [c_char; 32], /* some useful device name */
    pub flags: c_ushort, /* device flags */
    pub addr: c_ushort, /* device address (might be 10-bit) */
    pub private_value: c_ulong,
    pub private_data: *mut c_void,
    pub private_free: Option<unsafe extern "C" fn(device: *mut snd_i2c_device)>,
}

/* C macro: list_entry(n, struct snd_i2c_device, list). */
#[macro_export]
macro_rules! snd_i2c_device {
    ($n:expr) => { list_entry!($n, snd_i2c_device, list) };
}

#[repr(C)]
pub struct snd_i2c_bit_ops {
    pub start: Option<unsafe extern "C" fn(bus: *mut snd_i2c_bus)>, /* transfer start */
    pub stop: Option<unsafe extern "C" fn(bus: *mut snd_i2c_bus)>, /* transfer stop */
    pub direction: Option<unsafe extern "C" fn(bus: *mut snd_i2c_bus, clock: c_int, data: c_int)>, /* set line direction (0 = write, 1 = read) */
    pub setlines: Option<unsafe extern "C" fn(bus: *mut snd_i2c_bus, clock: c_int, data: c_int)>,
    pub getclock: Option<unsafe extern "C" fn(bus: *mut snd_i2c_bus) -> c_int>,
    pub getdata: Option<unsafe extern "C" fn(bus: *mut snd_i2c_bus, ack: c_int) -> c_int>,
}

#[repr(C)]
pub struct snd_i2c_ops {
    pub sendbytes: Option<unsafe extern "C" fn(device: *mut snd_i2c_device, bytes: *mut u8, count: c_int) -> c_int>,
    pub readbytes: Option<unsafe extern "C" fn(device: *mut snd_i2c_device, bytes: *mut u8, count: c_int) -> c_int>,
    pub probeaddr: Option<unsafe extern "C" fn(bus: *mut snd_i2c_bus, addr: c_ushort) -> c_int>,
}

#[repr(C)]
pub union snd_i2c_bus_hw_ops {
    pub bit: *mut snd_i2c_bit_ops,
    pub ops: *mut c_void,
}

#[repr(C)]
pub struct snd_i2c_bus {
    pub card: *mut snd_card, /* card which I2C belongs to */
    pub name: [c_char; 32], /* some useful label */
    pub lock_mutex: mutex,
    pub master: *mut snd_i2c_bus, /* master bus when SCK/SCL is shared */
    pub buses: list_head, /* master: slave buses sharing SCK/SCL, slave: link list */
    pub devices: list_head, /* attached devices to this bus */
    pub hw_ops: snd_i2c_bus_hw_ops, /* lowlevel operations */
    pub ops: *const snd_i2c_ops, /* midlevel operations */
    pub private_value: c_ulong,
    pub private_data: *mut c_void,
    pub private_free: Option<unsafe extern "C" fn(bus: *mut snd_i2c_bus)>,
}

/* C macro: list_entry(n, struct snd_i2c_bus, buses). */
#[macro_export]
macro_rules! snd_i2c_slave_bus {
    ($n:expr) => { list_entry!($n, snd_i2c_bus, buses) };
}

unsafe extern "C" {
    pub fn snd_i2c_bus_create(card: *mut snd_card, name: *const c_char, master: *mut snd_i2c_bus, ri2c: *mut *mut snd_i2c_bus) -> c_int;
    pub fn snd_i2c_device_create(bus: *mut snd_i2c_bus, name: *const c_char, addr: u8, rdevice: *mut *mut snd_i2c_device) -> c_int;
    pub fn snd_i2c_device_free(device: *mut snd_i2c_device) -> c_int;
    pub fn snd_i2c_sendbytes(device: *mut snd_i2c_device, bytes: *mut u8, count: c_int) -> c_int;
    pub fn snd_i2c_readbytes(device: *mut snd_i2c_device, bytes: *mut u8, count: c_int) -> c_int;
    pub fn snd_i2c_probeaddr(bus: *mut snd_i2c_bus, addr: c_ushort) -> c_int;
}

pub unsafe fn snd_i2c_lock(bus: *mut snd_i2c_bus) {
    if !(*bus).master.is_null() {
        mutex_lock(&mut (*(*bus).master).lock_mutex);
    } else {
        mutex_lock(&mut (*bus).lock_mutex);
    }
}

pub unsafe fn snd_i2c_unlock(bus: *mut snd_i2c_bus) {
    if !(*bus).master.is_null() {
        mutex_unlock(&mut (*(*bus).master).lock_mutex);
    } else {
        mutex_unlock(&mut (*bus).lock_mutex);
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
