// SPDX-License-Identifier: GPL-2.0-only
/*
 *  PC Speaker beeper driver for Linux
 *
 *  Copyright (c) 2002 Vojtech Pavlik
 *  Copyright (c) 1992 Orest Zborowski
 */

// Depends on Linux kernel declarations from:
// <linux/init.h>, <linux/input.h>, <linux/io.h>, "pcsp.h", "pcsp_input.h"

use core::ffi::{c_int, c_uint, c_ulong};

const ENOMEM: c_int = 12;
const BUS_ISA: u16 = 0x0010;
const EV_SND: c_uint = 0x12;
const SND_BELL: c_uint = 0x01;
const SND_TONE: c_uint = 0x02;
const PIT_TICK_RATE: c_uint = 1193182;

const fn BIT(nr: c_uint) -> c_ulong {
    1_u64.wrapping_shl(nr) as c_ulong
}

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct raw_spinlock_t {
    _private: [u8; 0],
}

#[repr(C)]
pub struct atomic_t {
    _private: [u8; 0],
}

#[repr(C)]
pub struct input_id {
    pub bustype: u16,
    pub vendor: u16,
    pub product: u16,
    pub version: u16,
}

#[repr(C)]
pub struct device_private {
    pub parent: *mut device,
}

#[repr(C)]
pub struct input_dev {
    pub name: *const u8,
    pub phys: *const u8,
    pub id: input_id,
    pub dev: device_private,
    pub evbit: [c_ulong; 1],
    pub sndbit: [c_ulong; 1],
    pub event: Option<
        unsafe extern "C" fn(
            dev: *mut input_dev,
            type_: c_uint,
            code: c_uint,
            value: c_int,
        ) -> c_int,
    >,
}

#[repr(C)]
pub struct pcsp_chip_t {
    pub timer_active: atomic_t,
    pub pcspkr: bool,
}

unsafe extern "C" {
    static mut i8253_lock: raw_spinlock_t;
    static mut pcsp_chip: pcsp_chip_t;

    fn raw_spin_lock_irqsave(lock: *mut raw_spinlock_t, flags: c_ulong);
    fn raw_spin_unlock_irqrestore(lock: *mut raw_spinlock_t, flags: c_ulong);
    fn outb_p(value: c_uint, port: c_uint);
    fn outb(value: c_uint, port: c_uint);
    fn inb_p(port: c_uint) -> c_uint;
    fn atomic_read(v: *const atomic_t) -> c_int;
    fn devm_input_allocate_device(dev: *mut device) -> *mut input_dev;
    fn input_register_device(dev: *mut input_dev) -> c_int;
}

unsafe fn pcspkr_do_sound(count: c_uint) {
    let mut flags: c_ulong = 0;

    raw_spin_lock_irqsave(&raw mut i8253_lock, flags);

    if count != 0 {
        /* set command for counter 2, 2 byte write */
        outb_p(0xB6, 0x43);
        /* select desired HZ */
        outb_p(count & 0xff, 0x42);
        outb((count >> 8) & 0xff, 0x42);
        /* enable counter 2 */
        outb_p(inb_p(0x61) | 3, 0x61);
    } else {
        /* disable counter 2 */
        outb(inb_p(0x61) & 0xFC, 0x61);
    }

    raw_spin_unlock_irqrestore(&raw mut i8253_lock, flags);
}

#[no_mangle]
pub unsafe extern "C" fn pcspkr_stop_sound() {
    pcspkr_do_sound(0);
}

unsafe extern "C" fn pcspkr_input_event(
    _dev: *mut input_dev,
    type_: c_uint,
    code: c_uint,
    mut value: c_int,
) -> c_int {
    let mut count: c_uint = 0;

    if atomic_read(&raw const pcsp_chip.timer_active) != 0 || !pcsp_chip.pcspkr {
        return 0;
    }

    match type_ {
        EV_SND => {
            match code {
                SND_BELL => {
                    if value != 0 {
                        value = 1000;
                    }
                }
                SND_TONE => {}
                _ => {
                    return -1;
                }
            }
        }

        _ => {
            return -1;
        }
    }

    if value > 20 && value < 32767 {
        count = PIT_TICK_RATE / value as c_uint;
    }

    pcspkr_do_sound(count);

    0
}

#[no_mangle]
pub unsafe extern "C" fn pcspkr_input_init(
    rdev: *mut *mut input_dev,
    dev: *mut device,
) -> c_int {
    let mut err: c_int;

    let input_dev: *mut input_dev = devm_input_allocate_device(dev);
    if input_dev.is_null() {
        return -ENOMEM;
    }

    (*input_dev).name = c"PC Speaker".as_ptr() as *const u8;
    (*input_dev).phys = c"isa0061/input0".as_ptr() as *const u8;
    (*input_dev).id.bustype = BUS_ISA;
    (*input_dev).id.vendor = 0x001f;
    (*input_dev).id.product = 0x0001;
    (*input_dev).id.version = 0x0100;
    (*input_dev).dev.parent = dev;

    (*input_dev).evbit[0] = BIT(EV_SND);
    (*input_dev).sndbit[0] = BIT(SND_BELL) | BIT(SND_TONE);
    (*input_dev).event = Some(pcspkr_input_event);

    err = input_register_device(input_dev);
    if err != 0 {
        return err;
    }

    *rdev = input_dev;
    0
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
