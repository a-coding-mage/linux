// SPDX-License-Identifier: GPL-2.0
/*
 *  This code maintains a list of active profiling data structures.
 *
 *    Copyright IBM Corp. 2009
 *    Author(s): Peter Oberparleiter <oberpar@linux.vnet.ibm.com>
 *
 *    Uses gcc-internal data definitions.
 *    Based on the gcov-kernel patch by:
 *             Hubertus Franke <frankeh@us.ibm.com>
 *             Nigel Hinds <nhinds@us.ibm.com>
 *             Rajan Ravindran <rajancr@us.ibm.com>
 *             Peter Oberparleiter <oberpar@linux.vnet.ibm.com>
 *             Paul Larson
 */

use core::ffi::c_void;

// linux/init.h, linux/module.h, linux/mutex.h, linux/sched.h, and gcov.h
// provide the concrete definitions and external interfaces referenced here.

pub type u32 = core::ffi::c_uint;
pub type u64 = core::ffi::c_ulonglong;

#[repr(C)]
pub struct gcov_info {
    _private: [u8; 0],
}

#[repr(C)]
pub struct module {
    _private: [u8; 0],
}

#[repr(C)]
pub struct mutex {
    _private: [u8; 0],
}

#[repr(C)]
pub struct notifier_block {
    pub notifier_call: Option<unsafe extern "C" fn(*mut notifier_block, usize, *mut c_void) -> i32>,
}

pub const GCOV_ADD: i32 = 0;
pub const GCOV_REMOVE: i32 = 1;
pub const MODULE_STATE_GOING: usize = 0;
pub const NOTIFY_OK: i32 = 0x0001;

#[no_mangle]
pub static mut gcov_events_enabled: i32 = 0;

#[no_mangle]
pub static mut gcov_lock: mutex = mutex { _private: [] };

extern "C" {
    fn mutex_lock(lock: *mut mutex);
    fn mutex_unlock(lock: *mut mutex);
    fn cond_resched();
    fn gcov_info_next(info: *mut gcov_info) -> *mut gcov_info;
    fn gcov_event(event: i32, info: *mut gcov_info);

    #[cfg(feature = "CONFIG_MODULES")]
    fn gcov_info_within_module(info: *mut gcov_info, module: *mut module) -> bool;
    #[cfg(feature = "CONFIG_MODULES")]
    fn gcov_info_unlink(previous: *mut gcov_info, info: *mut gcov_info);
    #[cfg(feature = "CONFIG_MODULES")]
    fn register_module_notifier(nb: *mut notifier_block) -> i32;
}

/**
 * gcov_enable_events - enable event reporting through gcov_event()
 *
 * Turn on reporting of profiling data load/unload-events through the
 * gcov_event() callback. Also replay all previous events once. This function
 * is needed because some events are potentially generated too early for the
 * callback implementation to handle them initially.
 */
#[no_mangle]
pub unsafe extern "C" fn gcov_enable_events() {
    let mut info: *mut gcov_info = core::ptr::null_mut();

    mutex_lock(&raw mut gcov_lock);
    gcov_events_enabled = 1;

    /* Perform event callback for previously registered entries. */
    while {
        info = gcov_info_next(info);
        !info.is_null()
    } {
        gcov_event(GCOV_ADD, info);
        cond_resched();
    }

    mutex_unlock(&raw mut gcov_lock);
}

/**
 * store_gcov_u32 - store 32 bit number in gcov format to buffer
 * @buffer: target buffer or NULL
 * @off: offset into the buffer
 * @v: value to be stored
 *
 * Number format defined by gcc: numbers are recorded in the 32 bit
 * unsigned binary form of the endianness of the machine generating the
 * file. Returns the number of bytes stored. If @buffer is %NULL, doesn't
 * store anything.
 */
#[no_mangle]
pub unsafe extern "C" fn store_gcov_u32(buffer: *mut c_void, off: usize, v: u32) -> usize {
    if !buffer.is_null() {
        let data = (buffer as *mut u8).add(off) as *mut u32;
        *data = v;
    }

    core::mem::size_of::<u32>()
}

/**
 * store_gcov_u64 - store 64 bit number in gcov format to buffer
 * @buffer: target buffer or NULL
 * @off: offset into the buffer
 * @v: value to be stored
 *
 * Number format defined by gcc: numbers are recorded in the 32 bit
 * unsigned binary form of the endianness of the machine generating the
 * file. 64 bit numbers are stored as two 32 bit numbers, the low part
 * first. Returns the number of bytes stored. If @buffer is %NULL, doesn't
 * store anything.
 */
#[no_mangle]
pub unsafe extern "C" fn store_gcov_u64(buffer: *mut c_void, off: usize, v: u64) -> usize {
    if !buffer.is_null() {
        let data = (buffer as *mut u8).add(off) as *mut u32;

        *data.add(0) = (v & 0xffffffffu64) as u32;
        *data.add(1) = (v >> 32) as u32;
    }

    core::mem::size_of::<u32>() * 2
}

// #ifdef CONFIG_MODULES
// Update list and generate events when modules are unloaded.
#[cfg(feature = "CONFIG_MODULES")]
unsafe extern "C" fn gcov_module_notifier(
    _nb: *mut notifier_block,
    event: usize,
    data: *mut c_void,
) -> i32 {
    let module = data as *mut module;
    let mut info: *mut gcov_info = core::ptr::null_mut();
    let mut prev: *mut gcov_info = core::ptr::null_mut();

    if event != MODULE_STATE_GOING {
        return NOTIFY_OK;
    }
    mutex_lock(&raw mut gcov_lock);

    /* Remove entries located in module from linked list. */
    while {
        info = gcov_info_next(info);
        !info.is_null()
    } {
        if gcov_info_within_module(info, module) {
            gcov_info_unlink(prev, info);
            if gcov_events_enabled != 0 {
                gcov_event(GCOV_REMOVE, info);
            }
        } else {
            prev = info;
        }
    }

    mutex_unlock(&raw mut gcov_lock);

    NOTIFY_OK
}

#[cfg(feature = "CONFIG_MODULES")]
static mut gcov_nb: notifier_block = notifier_block {
    notifier_call: Some(gcov_module_notifier),
};

#[cfg(feature = "CONFIG_MODULES")]
unsafe extern "C" fn gcov_init() -> i32 {
    register_module_notifier(&raw mut gcov_nb)
}

// device_initcall(gcov_init);
// #endif /* CONFIG_MODULES */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
