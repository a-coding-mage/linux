// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *   ALSA sequencer /proc interface
 *   Copyright (c) 1998 by Frank van de Pol <fvdpol@coil.demon.nl>
 */

// C dependencies:
// #include <linux/init.h>
// #include <linux/export.h>
// #include <sound/core.h>
// #include "seq_info.h"
// #include "seq_clientmgr.h"
// #include "seq_timer.h"

use core::ffi::{c_char, c_int, c_void};

const SNDRV_INFO_CONTENT_TEXT: c_int = 0;
const ENOMEM: c_int = 12;

#[repr(C)]
pub struct snd_info_buffer {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_info_entry {
    pub content: c_int,
    pub c: snd_info_entry__bindgen_ty_1,
}

#[repr(C)]
pub union snd_info_entry__bindgen_ty_1 {
    pub text: snd_info_entry_text,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_info_entry_text {
    pub read: Option<unsafe extern "C" fn(*mut snd_info_entry, *mut snd_info_buffer)>,
}

unsafe extern "C" {
    static mut THIS_MODULE: *mut c_void;
    static mut snd_seq_root: *mut snd_info_entry;

    fn snd_info_create_module_entry(
        module: *mut c_void,
        name: *mut c_char,
        parent: *mut snd_info_entry,
    ) -> *mut snd_info_entry;
    fn snd_info_register(entry: *mut snd_info_entry) -> c_int;
    fn snd_info_free_entry(entry: *mut snd_info_entry);

    fn snd_seq_info_queues_read(entry: *mut snd_info_entry, buffer: *mut snd_info_buffer);
    fn snd_seq_info_clients_read(entry: *mut snd_info_entry, buffer: *mut snd_info_buffer);
    fn snd_seq_info_timer_read(entry: *mut snd_info_entry, buffer: *mut snd_info_buffer);
}

static mut queues_entry: *mut snd_info_entry = core::ptr::null_mut();
static mut clients_entry: *mut snd_info_entry = core::ptr::null_mut();
static mut timer_entry: *mut snd_info_entry = core::ptr::null_mut();

// __init
unsafe fn create_info_entry(
    name: *mut c_char,
    read: Option<unsafe extern "C" fn(*mut snd_info_entry, *mut snd_info_buffer)>,
) -> *mut snd_info_entry {
    let entry: *mut snd_info_entry;

    entry = unsafe { snd_info_create_module_entry(THIS_MODULE, name, snd_seq_root) };
    if entry == core::ptr::null_mut() {
        return core::ptr::null_mut();
    }
    unsafe {
        (*entry).content = SNDRV_INFO_CONTENT_TEXT;
        (*entry).c.text.read = read;
    }
    if unsafe { snd_info_register(entry) } < 0 {
        unsafe {
            snd_info_free_entry(entry);
        }
        return core::ptr::null_mut();
    }
    entry
}

#[no_mangle]
pub unsafe extern "C" fn snd_seq_info_done() {
    unsafe {
        snd_info_free_entry(queues_entry);
        snd_info_free_entry(clients_entry);
        snd_info_free_entry(timer_entry);
    }
}

/* create all our /proc entries */
// __init
#[no_mangle]
pub unsafe extern "C" fn snd_seq_info_init() -> c_int {
    unsafe {
        queues_entry = create_info_entry(
            b"queues\0".as_ptr() as *mut c_char,
            Some(snd_seq_info_queues_read),
        );
        clients_entry = create_info_entry(
            b"clients\0".as_ptr() as *mut c_char,
            Some(snd_seq_info_clients_read),
        );
        timer_entry =
            create_info_entry(b"timer\0".as_ptr() as *mut c_char, Some(snd_seq_info_timer_read));
        if queues_entry == core::ptr::null_mut()
            || clients_entry == core::ptr::null_mut()
            || timer_entry == core::ptr::null_mut()
        {
            snd_seq_info_done();
            return -ENOMEM;
        }
    }
    0
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
