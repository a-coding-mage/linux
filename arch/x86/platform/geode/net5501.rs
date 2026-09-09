// SPDX-License-Identifier: GPL-2.0-only
/*
 * System Specific setup for Soekris net5501
 * At the moment this means setup of GPIO control of LEDs and buttons
 * on net5501 boards.
 *
 * Copyright (C) 2008-2009 Tower Technologies
 * Written by Alessandro Zummo <a.zummo@towertech.it>
 *
 * Copyright (C) 2008 Constantin Baranov <const@mimas.ru>
 * Copyright (C) 2011 Ed Wildgoose <kernel@wildgooses.com>
 *                and Philip Prindeville <philipp@redfish-solutions.com>
 */

use core::ffi::{c_char, c_int, c_void};

const BIOS_REGION_BASE: usize = 0xffff0000;
const BIOS_REGION_SIZE: usize = 0x00010000;
const KERN_ERR: *const c_char = c"<3>".as_ptr();
const KERN_INFO: *const c_char = c"<6>".as_ptr();

#[repr(C)]
pub struct geode_led {
    pub gpio: u32,
    pub active_low: bool,
}

unsafe extern "C" {
    fn geode_create_restart_key(gpio: u32);
    fn geode_create_leds(name: *const c_char, leds: *const geode_led, count: usize);
    fn ioremap(phys_addr: usize, size: usize) -> *mut c_void;
    fn iounmap(addr: *mut c_void);
    fn memcmp(s1: *const c_void, s2: *const c_void, n: usize) -> c_int;
    fn printk(level: *const c_char, ...) -> c_int;
    fn is_geode() -> bool;
}

static NET5501_LEDS: [geode_led; 1] = [geode_led {
    gpio: 6,
    active_low: true,
}];

unsafe fn register_net5501() {
    geode_create_restart_key(24);
    geode_create_leds(c"net5501".as_ptr(), NET5501_LEDS.as_ptr(), NET5501_LEDS.len());
}

#[repr(C)]
pub struct net5501_board {
    pub offset: u16,
    pub len: u16,
    pub sig: *mut c_char,
}

static mut BOARDS: [net5501_board; 2] = [
    net5501_board {
        offset: 0xb7b,
        len: 7,
        sig: c"net5501".as_ptr() as *mut c_char,
    },
    net5501_board {
        offset: 0xb1f,
        len: 7,
        sig: c"net5501".as_ptr() as *mut c_char,
    },
];

unsafe fn net5501_present() -> bool {
    let mut found = false;
    let rombase = ioremap(BIOS_REGION_BASE, BIOS_REGION_SIZE - 1) as *mut u8;
    if rombase.is_null() {
        printk(KERN_ERR, c"%s: failed to get rombase\n".as_ptr(), c"net5501".as_ptr());
        return found;
    }

    let bios = rombase.add(0x20); // null terminated

    if memcmp(bios as *const c_void, c"comBIOS".as_ptr() as *const c_void, 7) != 0 {
        iounmap(rombase as *mut c_void);
        return found;
    }

    for i in 0..BOARDS.len() {
        let board = &BOARDS[i];
        let model = rombase.add(board.offset as usize);

        if memcmp(model as *const c_void, board.sig as *const c_void, board.len as usize) == 0 {
            printk(
                KERN_INFO,
                c"%s: system is recognized as \"%s\"\n".as_ptr(),
                c"net5501".as_ptr(),
                model,
            );
            found = true;
            break;
        }
    }

    iounmap(rombase as *mut c_void);
    found
}

unsafe fn net5501_init() -> c_int {
    if !is_geode() {
        return 0;
    }

    if !net5501_present() {
        return 0;
    }

    register_net5501();
    0
}

// device_initcall(net5501_init);
#[allow(dead_code)]
static NET5501_INIT: unsafe fn() -> c_int = net5501_init;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
