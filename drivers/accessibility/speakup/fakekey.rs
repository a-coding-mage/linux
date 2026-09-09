// SPDX-License-Identifier: GPL-2.0+
/* fakekey.c
 * Functions for simulating key presses.
 *
 * Copyright (C) 2010 the Speakup Team
 */

use core::ffi::{c_char, c_int, c_ulong, c_void};

// Linux kernel dependencies supplied by other translation units.
#[repr(C)]
pub struct input_id {
    pub bustype: u16,
}

#[repr(C)]
pub struct device {
    pub parent: *mut device,
}

#[repr(C)]
pub struct input_dev {
    pub name: *const c_char,
    pub id: input_id,
    pub phys: *const c_char,
    pub dev: device,
    pub evbit: *mut c_ulong,
    pub keybit: *mut c_ulong,
}

unsafe extern "C" {
    fn input_allocate_device() -> *mut input_dev;
    fn input_register_device(dev: *mut input_dev) -> c_int;
    fn input_free_device(dev: *mut input_dev);
    fn input_unregister_device(dev: *mut input_dev);
    fn __set_bit(nr: c_ulong, addr: *mut c_ulong);
    fn input_report_key(dev: *mut input_dev, code: c_ulong, value: c_int);
    fn input_sync(dev: *mut input_dev);
    fn local_irq_save(flags: *mut c_ulong);
    fn local_irq_restore(flags: c_ulong);
    fn preempt_disable();
    fn preempt_enable();
}

const PRESSED: c_int = 1;
const RELEASED: c_int = 0;

// DEFINE_PER_CPU(int, reporting_keystroke);
static mut REPORTING_KEYSTROKE: c_int = 0;

static mut virt_keyboard: *mut input_dev = core::ptr::null_mut();

pub unsafe fn speakup_add_virtual_keyboard() -> c_int {
    let mut err: c_int;

    virt_keyboard = input_allocate_device();

    if virt_keyboard.is_null() {
        return -12; // -ENOMEM
    }

    (*virt_keyboard).name = c"Speakup".as_ptr();
    (*virt_keyboard).id.bustype = 0x06; // BUS_VIRTUAL
    (*virt_keyboard).phys = c"speakup/input0".as_ptr();
    (*virt_keyboard).dev.parent = core::ptr::null_mut();

    __set_bit(1, (*virt_keyboard).evbit); // EV_KEY
    __set_bit(108, (*virt_keyboard).keybit); // KEY_DOWN

    err = input_register_device(virt_keyboard);
    if err != 0 {
        input_free_device(virt_keyboard);
        virt_keyboard = core::ptr::null_mut();
    }

    err
}

pub unsafe fn speakup_remove_virtual_keyboard() {
    if !virt_keyboard.is_null() {
        input_unregister_device(virt_keyboard);
        virt_keyboard = core::ptr::null_mut();
    }
}

/*
 * Send a simulated down-arrow to the application.
 */
pub unsafe fn speakup_fake_down_arrow() {
    let mut flags: c_ulong = 0;

    /* disable keyboard interrupts */
    local_irq_save(&mut flags as *mut c_ulong);
    /* don't change CPU */
    preempt_disable();

    REPORTING_KEYSTROKE = 1;
    input_report_key(virt_keyboard, 108, PRESSED); // KEY_DOWN
    input_report_key(virt_keyboard, 108, RELEASED); // KEY_DOWN
    input_sync(virt_keyboard);
    REPORTING_KEYSTROKE = 0;

    /* re-enable preemption */
    preempt_enable();
    /* re-enable keyboard interrupts */
    local_irq_restore(flags);
}

/*
 * Are we handling a simulated key press on the current CPU?
 * Returns a boolean.
 */
pub unsafe fn speakup_fake_key_pressed() -> bool {
    REPORTING_KEYSTROKE != 0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
