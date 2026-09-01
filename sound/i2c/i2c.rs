// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *   Generic i2c interface for ALSA
 *
 *   (c) 1998 Gerd Knorr <kraxel@cs.tu-berlin.de>
 *   Modified for the ALSA driver by Jaroslav Kysela <perex@perex.cz>
 */

use core::ffi::{c_char, c_int, c_uchar, c_uint, c_ulong, c_void};
use core::mem::size_of;
use core::ptr;

const EINVAL: c_int = 22;
const EIO: c_int = 5;
const ENOMEM: c_int = 12;
const GFP_KERNEL: c_uint = 0;
const SNDRV_DEV_BUS: c_int = 0;
const SND_I2C_DEVICE_ADDRTEN: c_uint = 1;

#[repr(C)]
pub struct list_head {
    pub next: *mut list_head,
    pub prev: *mut list_head,
}

#[repr(C)]
pub struct snd_card {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_device {
    pub device_data: *mut c_void,
}

#[repr(C)]
pub struct snd_device_ops {
    pub dev_free: Option<unsafe extern "C" fn(*mut snd_device) -> c_int>,
}

#[repr(C)]
pub struct snd_i2c_ops {
    pub sendbytes:
        Option<unsafe extern "C" fn(*mut snd_i2c_device, *mut c_uchar, c_int) -> c_int>,
    pub readbytes:
        Option<unsafe extern "C" fn(*mut snd_i2c_device, *mut c_uchar, c_int) -> c_int>,
    pub probeaddr: Option<unsafe extern "C" fn(*mut snd_i2c_bus, u16) -> c_int>,
}

#[repr(C)]
pub struct snd_i2c_bit_ops {
    pub start: Option<unsafe extern "C" fn(*mut snd_i2c_bus)>,
    pub stop: Option<unsafe extern "C" fn(*mut snd_i2c_bus)>,
    pub direction: Option<unsafe extern "C" fn(*mut snd_i2c_bus, c_int, c_int)>,
    pub setlines: unsafe extern "C" fn(*mut snd_i2c_bus, c_int, c_int),
    pub getclock: Option<unsafe extern "C" fn(*mut snd_i2c_bus) -> c_int>,
    pub getdata: unsafe extern "C" fn(*mut snd_i2c_bus, c_int) -> c_int,
}

#[repr(C)]
pub union snd_i2c_hw_ops {
    pub bit: *mut snd_i2c_bit_ops,
}

#[repr(C)]
pub struct mutex {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_i2c_bus {
    pub lock_mutex: mutex,
    pub devices: list_head,
    pub buses: list_head,
    pub card: *mut snd_card,
    pub ops: *const snd_i2c_ops,
    pub master: *mut snd_i2c_bus,
    pub private_free: Option<unsafe extern "C" fn(*mut snd_i2c_bus)>,
    pub hw_ops: snd_i2c_hw_ops,
    pub name: [c_char; 32],
}

#[repr(C)]
pub struct snd_i2c_device {
    pub list: list_head,
    pub bus: *mut snd_i2c_bus,
    pub private_free: Option<unsafe extern "C" fn(*mut snd_i2c_device)>,
    pub addr: c_uchar,
    pub flags: c_uint,
    pub name: [c_char; 32],
}

unsafe extern "C" {
    fn kzalloc(size: usize, flags: c_uint) -> *mut c_void;
    fn kfree(ptr: *mut c_void);
    fn mutex_init(lock: *mut mutex);
    fn snd_BUG_ON(cond: bool) -> bool;
    fn snd_device_new(
        card: *mut snd_card,
        device: c_int,
        device_data: *mut c_void,
        ops: *const snd_device_ops,
    ) -> c_int;
    fn snd_device_free(card: *mut snd_card, device_data: *mut c_void) -> c_int;
    fn strscpy(dest: *mut c_char, src: *const c_char, count: c_ulong) -> c_long;
}

type c_long = isize;

unsafe fn INIT_LIST_HEAD(list: *mut list_head) {
    unsafe {
        (*list).next = list;
        (*list).prev = list;
    }
}

unsafe fn list_empty(head: *const list_head) -> bool {
    unsafe { (*head).next == head as *mut list_head }
}

unsafe fn __list_add(new: *mut list_head, prev: *mut list_head, next: *mut list_head) {
    unsafe {
        (*next).prev = new;
        (*new).next = next;
        (*new).prev = prev;
        (*prev).next = new;
    }
}

unsafe fn list_add_tail(new: *mut list_head, head: *mut list_head) {
    unsafe {
        __list_add(new, (*head).prev, head);
    }
}

unsafe fn __list_del(prev: *mut list_head, next: *mut list_head) {
    unsafe {
        (*next).prev = prev;
        (*prev).next = next;
    }
}

unsafe fn list_del(entry: *mut list_head) {
    unsafe {
        __list_del((*entry).prev, (*entry).next);
        (*entry).next = ptr::null_mut();
        (*entry).prev = ptr::null_mut();
    }
}

unsafe fn snd_i2c_device(ptr: *mut list_head) -> *mut snd_i2c_device {
    ptr as *mut snd_i2c_device
}

unsafe fn snd_i2c_slave_bus(ptr: *mut list_head) -> *mut snd_i2c_bus {
    unsafe { (ptr as *mut u8).sub(core::mem::offset_of!(snd_i2c_bus, buses)) as *mut snd_i2c_bus }
}

static snd_i2c_bit_ops: snd_i2c_ops = snd_i2c_ops {
    sendbytes: Some(snd_i2c_bit_sendbytes),
    readbytes: Some(snd_i2c_bit_readbytes),
    probeaddr: Some(snd_i2c_bit_probeaddr),
};

unsafe extern "C" fn snd_i2c_bus_free(bus: *mut snd_i2c_bus) -> c_int {
    let mut slave: *mut snd_i2c_bus;
    let mut device: *mut snd_i2c_device;

    unsafe {
        if snd_BUG_ON(bus.is_null()) {
            return -EINVAL;
        }
        while !list_empty(&mut (*bus).devices) {
            device = snd_i2c_device((*bus).devices.next);
            snd_i2c_device_free(device);
        }
        if !(*bus).master.is_null() {
            list_del(&mut (*bus).buses);
        } else {
            while !list_empty(&mut (*bus).buses) {
                slave = snd_i2c_slave_bus((*bus).buses.next);
                snd_device_free((*bus).card, slave as *mut c_void);
            }
        }
        if let Some(private_free) = (*bus).private_free {
            private_free(bus);
        }
        kfree(bus as *mut c_void);
    }
    0
}

unsafe extern "C" fn snd_i2c_bus_dev_free(device: *mut snd_device) -> c_int {
    let bus: *mut snd_i2c_bus = unsafe { (*device).device_data as *mut snd_i2c_bus };
    unsafe { snd_i2c_bus_free(bus) }
}

#[no_mangle]
pub unsafe extern "C" fn snd_i2c_bus_create(
    card: *mut snd_card,
    name: *const c_char,
    master: *mut snd_i2c_bus,
    ri2c: *mut *mut snd_i2c_bus,
) -> c_int {
    let mut bus: *mut snd_i2c_bus;
    let err: c_int;
    static ops: snd_device_ops = snd_device_ops {
        dev_free: Some(snd_i2c_bus_dev_free),
    };

    unsafe {
        *ri2c = ptr::null_mut();
        bus = kzalloc(size_of::<snd_i2c_bus>(), GFP_KERNEL) as *mut snd_i2c_bus;
        if bus.is_null() {
            return -ENOMEM;
        }
        mutex_init(&mut (*bus).lock_mutex);
        INIT_LIST_HEAD(&mut (*bus).devices);
        INIT_LIST_HEAD(&mut (*bus).buses);
        (*bus).card = card;
        (*bus).ops = &snd_i2c_bit_ops;
        if !master.is_null() {
            list_add_tail(&mut (*bus).buses, &mut (*master).buses);
            (*bus).master = master;
        }
        strscpy((*bus).name.as_mut_ptr(), name, (*bus).name.len() as c_ulong);
        err = snd_device_new(card, SNDRV_DEV_BUS, bus as *mut c_void, &ops);
        if err < 0 {
            snd_i2c_bus_free(bus);
            return err;
        }
        *ri2c = bus;
    }
    0
}

#[no_mangle]
pub unsafe extern "C" fn snd_i2c_device_create(
    bus: *mut snd_i2c_bus,
    name: *const c_char,
    addr: c_uchar,
    rdevice: *mut *mut snd_i2c_device,
) -> c_int {
    let device: *mut snd_i2c_device;

    unsafe {
        *rdevice = ptr::null_mut();
        if snd_BUG_ON(bus.is_null()) {
            return -EINVAL;
        }
        device = kzalloc(size_of::<snd_i2c_device>(), GFP_KERNEL) as *mut snd_i2c_device;
        if device.is_null() {
            return -ENOMEM;
        }
        (*device).addr = addr;
        strscpy(
            (*device).name.as_mut_ptr(),
            name,
            (*device).name.len() as c_ulong,
        );
        list_add_tail(&mut (*device).list, &mut (*bus).devices);
        (*device).bus = bus;
        *rdevice = device;
    }
    0
}

#[no_mangle]
pub unsafe extern "C" fn snd_i2c_device_free(device: *mut snd_i2c_device) -> c_int {
    unsafe {
        if !(*device).bus.is_null() {
            list_del(&mut (*device).list);
        }
        if let Some(private_free) = (*device).private_free {
            private_free(device);
        }
        kfree(device as *mut c_void);
    }
    0
}

#[no_mangle]
pub unsafe extern "C" fn snd_i2c_sendbytes(
    device: *mut snd_i2c_device,
    bytes: *mut c_uchar,
    count: c_int,
) -> c_int {
    unsafe { ((*(*(*device).bus).ops).sendbytes.unwrap())(device, bytes, count) }
}

#[no_mangle]
pub unsafe extern "C" fn snd_i2c_readbytes(
    device: *mut snd_i2c_device,
    bytes: *mut c_uchar,
    count: c_int,
) -> c_int {
    unsafe { ((*(*(*device).bus).ops).readbytes.unwrap())(device, bytes, count) }
}

#[no_mangle]
pub unsafe extern "C" fn snd_i2c_probeaddr(bus: *mut snd_i2c_bus, addr: u16) -> c_int {
    unsafe { ((*(*bus).ops).probeaddr.unwrap())(bus, addr) }
}

/*
 *  bit-operations
 */

unsafe fn snd_i2c_bit_hw_start(bus: *mut snd_i2c_bus) {
    unsafe {
        if let Some(start) = (*(*bus).hw_ops.bit).start {
            start(bus);
        }
    }
}

unsafe fn snd_i2c_bit_hw_stop(bus: *mut snd_i2c_bus) {
    unsafe {
        if let Some(stop) = (*(*bus).hw_ops.bit).stop {
            stop(bus);
        }
    }
}

unsafe fn snd_i2c_bit_direction(bus: *mut snd_i2c_bus, clock: c_int, data: c_int) {
    unsafe {
        if let Some(direction) = (*(*bus).hw_ops.bit).direction {
            direction(bus, clock, data);
        }
    }
}

unsafe fn snd_i2c_bit_set(bus: *mut snd_i2c_bus, clock: c_int, data: c_int) {
    unsafe {
        ((*(*bus).hw_ops.bit).setlines)(bus, clock, data);
    }
}

/*
 * Original C kept this helper under `#if 0`.
 *
 * static int snd_i2c_bit_clock(struct snd_i2c_bus *bus)
 * {
 *     if (bus->hw_ops.bit->getclock)
 *         return bus->hw_ops.bit->getclock(bus);
 *     return -ENXIO;
 * }
 */

unsafe fn snd_i2c_bit_data(bus: *mut snd_i2c_bus, ack: c_int) -> c_int {
    unsafe { ((*(*bus).hw_ops.bit).getdata)(bus, ack) }
}

unsafe fn snd_i2c_bit_start(bus: *mut snd_i2c_bus) {
    unsafe {
        snd_i2c_bit_hw_start(bus);
        snd_i2c_bit_direction(bus, 1, 1); /* SCL - wr, SDA - wr */
        snd_i2c_bit_set(bus, 1, 1);
        snd_i2c_bit_set(bus, 1, 0);
        snd_i2c_bit_set(bus, 0, 0);
    }
}

unsafe fn snd_i2c_bit_stop(bus: *mut snd_i2c_bus) {
    unsafe {
        snd_i2c_bit_set(bus, 0, 0);
        snd_i2c_bit_set(bus, 1, 0);
        snd_i2c_bit_set(bus, 1, 1);
        snd_i2c_bit_hw_stop(bus);
    }
}

unsafe fn snd_i2c_bit_send(bus: *mut snd_i2c_bus, data: c_int) {
    unsafe {
        snd_i2c_bit_set(bus, 0, data);
        snd_i2c_bit_set(bus, 1, data);
        snd_i2c_bit_set(bus, 0, data);
    }
}

unsafe fn snd_i2c_bit_ack(bus: *mut snd_i2c_bus) -> c_int {
    let ack: c_int;

    unsafe {
        snd_i2c_bit_set(bus, 0, 1);
        snd_i2c_bit_set(bus, 1, 1);
        snd_i2c_bit_direction(bus, 1, 0); /* SCL - wr, SDA - rd */
        ack = snd_i2c_bit_data(bus, 1);
        snd_i2c_bit_direction(bus, 1, 1); /* SCL - wr, SDA - wr */
        snd_i2c_bit_set(bus, 0, 1);
    }
    if ack != 0 {
        -EIO
    } else {
        0
    }
}

unsafe fn snd_i2c_bit_sendbyte(bus: *mut snd_i2c_bus, data: c_uchar) -> c_int {
    let mut i: c_int;
    let err: c_int;

    i = 7;
    while i >= 0 {
        unsafe {
            snd_i2c_bit_send(bus, if (data as c_int & (1 << i)) != 0 { 1 } else { 0 });
        }
        i -= 1;
    }
    err = unsafe { snd_i2c_bit_ack(bus) };
    if err < 0 {
        return err;
    }
    0
}

unsafe fn snd_i2c_bit_readbyte(bus: *mut snd_i2c_bus, last: c_int) -> c_int {
    let mut i: c_int;
    let mut data: c_uchar = 0;

    unsafe {
        snd_i2c_bit_set(bus, 0, 1);
        snd_i2c_bit_direction(bus, 1, 0); /* SCL - wr, SDA - rd */
        i = 7;
        while i >= 0 {
            snd_i2c_bit_set(bus, 1, 1);
            if snd_i2c_bit_data(bus, 0) != 0 {
                data |= (1 << i) as c_uchar;
            }
            snd_i2c_bit_set(bus, 0, 1);
            i -= 1;
        }
        snd_i2c_bit_direction(bus, 1, 1); /* SCL - wr, SDA - wr */
        snd_i2c_bit_send(bus, if last != 0 { 1 } else { 0 });
    }
    data as c_int
}

unsafe extern "C" fn snd_i2c_bit_sendbytes(
    device: *mut snd_i2c_device,
    mut bytes: *mut c_uchar,
    mut count: c_int,
) -> c_int {
    let bus: *mut snd_i2c_bus = unsafe { (*device).bus };
    let mut err: c_int;
    let mut res: c_int = 0;

    unsafe {
        if ((*device).flags & SND_I2C_DEVICE_ADDRTEN) != 0 {
            return -EIO; /* not yet implemented */
        }
        snd_i2c_bit_start(bus);
        err = snd_i2c_bit_sendbyte(bus, ((*device).addr as c_int << 1) as c_uchar);
        if err < 0 {
            snd_i2c_bit_hw_stop(bus);
            return err;
        }
        while {
            let old = count;
            count -= 1;
            old > 0
        } {
            err = snd_i2c_bit_sendbyte(bus, *bytes);
            bytes = bytes.add(1);
            if err < 0 {
                snd_i2c_bit_hw_stop(bus);
                return err;
            }
            res += 1;
        }
        snd_i2c_bit_stop(bus);
    }
    res
}

unsafe extern "C" fn snd_i2c_bit_readbytes(
    device: *mut snd_i2c_device,
    mut bytes: *mut c_uchar,
    mut count: c_int,
) -> c_int {
    let bus: *mut snd_i2c_bus = unsafe { (*device).bus };
    let mut err: c_int;
    let mut res: c_int = 0;

    unsafe {
        if ((*device).flags & SND_I2C_DEVICE_ADDRTEN) != 0 {
            return -EIO; /* not yet implemented */
        }
        snd_i2c_bit_start(bus);
        err = snd_i2c_bit_sendbyte(bus, (((*device).addr as c_int << 1) | 1) as c_uchar);
        if err < 0 {
            snd_i2c_bit_hw_stop(bus);
            return err;
        }
        while {
            let old = count;
            count -= 1;
            old > 0
        } {
            err = snd_i2c_bit_readbyte(bus, if count == 0 { 1 } else { 0 });
            if err < 0 {
                snd_i2c_bit_hw_stop(bus);
                return err;
            }
            *bytes = err as c_uchar;
            bytes = bytes.add(1);
            res += 1;
        }
        snd_i2c_bit_stop(bus);
    }
    res
}

unsafe extern "C" fn snd_i2c_bit_probeaddr(bus: *mut snd_i2c_bus, addr: u16) -> c_int {
    let err: c_int;

    unsafe {
        if (addr & 0x8000) != 0 {
            return -EIO; /* 10-bit address - not yet implemented */
        }
        if (addr & 0x7f80) != 0 {
            return -EINVAL; /* invalid address */
        }
        snd_i2c_bit_start(bus);
        err = snd_i2c_bit_sendbyte(bus, (addr << 1) as c_uchar);
        snd_i2c_bit_stop(bus);
    }
    err
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
