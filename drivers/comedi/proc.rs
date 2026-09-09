// SPDX-License-Identifier: GPL-2.0+
/*
 * /proc interface for comedi
 *
 * COMEDI - Linux Control and Measurement Device Interface
 * Copyright (C) 1998 David A. Schleef <ds@schleef.org>
 */

/*
 * This is some serious bloatware.
 *
 * Taken from Dave A.'s PCL-711 driver, 'cuz I thought it
 * was cool.
 */

use core::ffi::{c_char, c_int, c_uint, c_void};

// These declarations are supplied by the surrounding comedi and kernel code.
extern "C" {
    fn seq_printf(m: *mut seq_file, fmt: *const c_char, ...) -> c_int;
    fn seq_puts(m: *mut seq_file, s: *const c_char) -> c_int;
    fn comedi_dev_get_from_minor(minor: c_int) -> *mut comedi_device;
    fn comedi_dev_put(dev: *mut comedi_device);
    fn down_read(lock: *mut rw_semaphore);
    fn up_read(lock: *mut rw_semaphore);
    fn mutex_lock(lock: *mut mutex);
    fn mutex_unlock(lock: *mut mutex);
    fn proc_create_single(
        name: *const c_char,
        mode: c_uint,
        parent: *mut proc_dir_entry,
        read: unsafe extern "C" fn(*mut seq_file, *mut c_void) -> c_int,
    ) -> *mut proc_dir_entry;
    fn remove_proc_entry(name: *const c_char, parent: *mut proc_dir_entry);
    fn pr_warn(fmt: *const c_char, ...);
}

// Types and constants are supplied by the included comedi headers.
#[allow(non_camel_case_types)]
#[repr(C)]
pub struct seq_file {
    _private: [u8; 0],
}

#[allow(non_camel_case_types)]
#[repr(C)]
pub struct rw_semaphore {
    _private: [u8; 0],
}

#[allow(non_camel_case_types)]
#[repr(C)]
pub struct mutex {
    _private: [u8; 0],
}

#[allow(non_camel_case_types)]
#[repr(C)]
pub struct proc_dir_entry {
    _private: [u8; 0],
}

extern "C" {
    static mut comedi_drivers: *mut comedi_driver;
    static mut comedi_drivers_list_lock: mutex;
}

#[repr(C)]
pub struct comedi_device {
    pub attach_lock: rw_semaphore,
    pub attached: bool,
    pub driver: *mut comedi_driver,
    pub board_name: *const c_char,
    pub n_subdevices: c_int,
}

#[repr(C)]
pub struct comedi_driver {
    pub driver_name: *const c_char,
    pub board_name: *const c_char,
    pub num_names: c_int,
    pub offset: usize,
    pub next: *mut comedi_driver,
}

extern "C" {
    static COMEDI_NUM_BOARD_MINORS: c_int;
}

unsafe extern "C" fn comedi_read(m: *mut seq_file, _v: *mut c_void) -> c_int {
    let mut i: c_int;
    let mut devices_q: c_int = 0;
    let mut driv: *mut comedi_driver;

    seq_printf(
        m,
        b"comedi version %s\nformat string: %s\n\0".as_ptr() as *const c_char,
        b"COMEDI_RELEASE\0".as_ptr() as *const c_char,
        b"\"%2d: %-20s %-20s %4d\", i, driver_name, board_name, n_subdevices\0".as_ptr()
            as *const c_char,
    );

    i = 0;
    while i < COMEDI_NUM_BOARD_MINORS {
        let dev: *mut comedi_device = comedi_dev_get_from_minor(i);

        if dev.is_null() {
            i += 1;
            continue;
        }

        down_read(&mut (*dev).attach_lock);
        if (*dev).attached {
            devices_q = 1;
            seq_printf(
                m,
                b"%2d: %-20s %-20s %4d\n\0".as_ptr() as *const c_char,
                i,
                (*(*dev).driver).driver_name,
                (*dev).board_name,
                (*dev).n_subdevices,
            );
        }
        up_read(&mut (*dev).attach_lock);
        comedi_dev_put(dev);
        i += 1;
    }
    if devices_q == 0 {
        seq_puts(m, b"no devices\n\0".as_ptr() as *const c_char);
    }

    mutex_lock(&raw mut comedi_drivers_list_lock);
    driv = comedi_drivers;
    while !driv.is_null() {
        seq_printf(
            m,
            b"%s:\n\0".as_ptr() as *const c_char,
            (*driv).driver_name,
        );
        i = 0;
        while i < (*driv).num_names {
            let board_name = ((*driv).board_name as *const u8)
                .add((i as usize).wrapping_mul((*driv).offset))
                as *const *const c_char;
            seq_printf(
                m,
                b" %s\n\0".as_ptr() as *const c_char,
                *board_name,
            );
            i += 1;
        }

        if (*driv).num_names == 0 {
            seq_printf(
                m,
                b" %s\n\0".as_ptr() as *const c_char,
                (*driv).driver_name,
            );
        }
        driv = (*driv).next;
    }
    mutex_unlock(&raw mut comedi_drivers_list_lock);

    0
}

pub unsafe extern "C" fn comedi_proc_init() {
    if proc_create_single(
        b"comedi\0".as_ptr() as *const c_char,
        0o444,
        core::ptr::null_mut(),
        comedi_read,
    )
    .is_null()
    {
        pr_warn(b"comedi: unable to create proc entry\n\0".as_ptr() as *const c_char);
    }
}

pub unsafe extern "C" fn comedi_proc_cleanup() {
    remove_proc_entry(
        b"comedi\0".as_ptr() as *const c_char,
        core::ptr::null_mut(),
    );
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
