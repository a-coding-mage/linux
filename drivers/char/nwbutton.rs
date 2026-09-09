// SPDX-License-Identifier: GPL-2.0-only
/*
 * NetWinder Button Driver-
 * Copyright (C) Alex Holden <alex@linuxhacker.org> 1998, 1999.
 */

// Linux kernel headers and "nwbutton.h" supply the constants, types, and
// external functions referenced below.

use core::ffi::{c_char, c_int, c_void};

extern "C" {
    static mut jiffies: usize;
    fn kill_cad_pid(sig: c_int, val: c_int);
    fn mod_timer(timer: *mut timer_list, expires: usize) -> c_int;
    fn wake_up_interruptible(queue: *mut wait_queue_head);
    fn misc_register(device: *mut miscdevice) -> c_int;
    fn misc_deregister(device: *mut miscdevice);
    fn request_irq(irq: c_int, handler: unsafe extern "C" fn(c_int, *mut c_void) -> irqreturn_t,
                   flags: c_int, name: *const c_char, dev_id: *mut c_void) -> c_int;
    fn free_irq(irq: c_int, dev_id: *mut c_void);
    fn copy_to_user(to: *mut c_void, from: *const c_void, n: usize) -> usize;
    fn schedule();
    fn prepare_to_wait(queue: *mut wait_queue_head, wait: *mut wait_queue_entry, state: c_int);
    fn finish_wait(queue: *mut wait_queue_head, wait: *mut wait_queue_entry);
    fn sprintf(buf: *mut c_char, fmt: *const c_char, ...) -> c_int;
    fn printk(fmt: *const c_char, ...);
}

#[repr(C)]
pub struct timer_list;
#[repr(C)]
pub struct wait_queue_head;
#[repr(C)]
pub struct wait_queue_entry;
#[repr(C)]
pub struct file;
#[repr(C)]
pub struct file_operations;
#[repr(C)]
pub struct miscdevice;

pub type irqreturn_t = c_int;

#[repr(C)]
struct button_callback {
    callback: Option<unsafe extern "C" fn()>,
    count: c_int,
}

extern "C" {
    static mut button_timer: timer_list;
    static mut button_wait_queue: wait_queue_head;
}

static mut button_press_count: c_int = 0;
static mut button_output_buffer: [c_char; 32] = [0; 32];
static mut bcount: c_int = 0;
static mut bdelay: c_int = BUTTON_DELAY;
static mut button_callback_list: [button_callback; 32] = [button_callback {
    callback: None,
    count: 0,
}; 32];
static mut callback_count: c_int = 0;
static mut reboot_count: c_int = NUM_PRESSES_REBOOT;

extern "C" {
    fn noop_llseek();
}

pub unsafe extern "C" fn button_add_callback(callback: Option<unsafe extern "C" fn()>, count: c_int) -> c_int {
    let mut lp: usize = 0;
    if callback_count == 32 {
        return -ENOMEM;
    }
    if callback.is_none() {
        return -EINVAL;
    }
    callback_count += 1;
    while button_callback_list[lp].callback.is_some() {
        lp += 1;
    }
    button_callback_list[lp].callback = callback;
    button_callback_list[lp].count = count;
    0
}

pub unsafe extern "C" fn button_del_callback(callback: Option<unsafe extern "C" fn()>) -> c_int {
    if callback.is_none() {
        return -EINVAL;
    }
    let mut lp: c_int = 31;
    while lp >= 0 {
        if button_callback_list[lp as usize].callback == callback {
            button_callback_list[lp as usize].callback = None;
            button_callback_list[lp as usize].count = 0;
            callback_count -= 1;
            return 0;
        }
        lp -= 1;
    }
    -EINVAL
}

unsafe fn button_consume_callbacks(bpcount: c_int) {
    let mut lp: usize = 0;
    while lp <= 31 {
        if button_callback_list[lp].count == bpcount {
            if let Some(callback) = button_callback_list[lp].callback {
                callback();
            }
        }
        lp += 1;
    }
}

unsafe extern "C" fn button_sequence_finished(_unused: *mut timer_list) {
    // CONFIG_NWBUTTON_REBOOT is a build-time kernel configuration condition.
    if button_press_count == reboot_count {
        kill_cad_pid(SIGINT, 1);
    }
    button_consume_callbacks(button_press_count);
    bcount = sprintf(button_output_buffer.as_mut_ptr(), b"%d\0".as_ptr() as *const c_char, button_press_count);
    button_press_count = 0;
    wake_up_interruptible(&mut button_wait_queue);
}

unsafe extern "C" fn button_handler(_irq: c_int, _dev_id: *mut c_void) -> irqreturn_t {
    button_press_count += 1;
    mod_timer(&mut button_timer, jiffies.wrapping_add(bdelay as usize));
    IRQ_HANDLED
}

unsafe extern "C" fn button_read(_filp: *mut file, buffer: *mut c_char, _count: usize, _ppos: *mut i64) -> c_int {
    let mut wait = wait_queue_entry;
    prepare_to_wait(&mut button_wait_queue, &mut wait, TASK_INTERRUPTIBLE);
    schedule();
    finish_wait(&mut button_wait_queue, &mut wait);
    if copy_to_user(buffer as *mut c_void, button_output_buffer.as_ptr() as *const c_void, bcount as usize) != 0 {
        -EFAULT
    } else {
        bcount
    }
}

// The following file-operation and misc-device initializers correspond to the
// C structures; their kernel-specific field layout is supplied by the headers.
static mut button_fops: file_operations = unsafe { core::mem::zeroed() };
static mut button_misc_device: miscdevice = unsafe { core::mem::zeroed() };

unsafe extern "C" fn nwbutton_init() -> c_int {
    if !machine_is_netwinder() {
        return -ENODEV;
    }
    if misc_register(&mut button_misc_device) != 0 {
        return -EBUSY;
    }
    if request_irq(IRQ_NETWINDER_BUTTON, button_handler, 0, b"nwbutton\0".as_ptr() as *const c_char, core::ptr::null_mut()) != 0 {
        misc_deregister(&mut button_misc_device);
        return -EIO;
    }
    0
}

unsafe extern "C" fn nwbutton_exit() {
    free_irq(IRQ_NETWINDER_BUTTON, core::ptr::null_mut());
    misc_deregister(&mut button_misc_device);
}

extern "C" {
    fn machine_is_netwinder() -> bool;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
