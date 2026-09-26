// SPDX-License-Identifier: GPL-2.0-only
//! Early reset and console-level callbacks for the unselected boot owner.

use super::{bindings, main_globals};
use kernel::ffi::{c_char, c_int};

#[link_section = ".init.text"]
pub(super) unsafe extern "C" fn set_reset_devices(_value: *mut c_char) -> c_int {
    // SAFETY: original boot-option parsing serializes this sole owner's state.
    unsafe { main_globals::reset_devices = 1; }
    1
}

#[link_section = ".init.text"]
pub(super) unsafe extern "C" fn debug_kernel(_value: *mut c_char) -> c_int {
    // SAFETY: console_printk is the actual external C array. Its declaration
    // has no bound; console_loglevel is its first element in linux/printk.h.
    unsafe {
        core::ptr::addr_of_mut!(bindings::console_printk).cast::<c_int>()
            .write(bindings::CONSOLE_LOGLEVEL_DEBUG as c_int);
    }
    0
}

#[link_section = ".init.text"]
pub(super) unsafe extern "C" fn quiet_kernel(_value: *mut c_char) -> c_int {
    // SAFETY: the same serialized boot callback and external array as above.
    unsafe {
        core::ptr::addr_of_mut!(bindings::console_printk).cast::<c_int>()
            .write(bindings::CONSOLE_LOGLEVEL_QUIET as c_int);
    }
    0
}

#[link_section = ".init.text"]
pub(super) unsafe extern "C" fn loglevel(mut value: *mut c_char) -> c_int {
    let mut level = 0;
    // SAFETY: the original parser supplies a terminated writable option string
    // (or NULL); get_option updates these two local outputs with its C rules.
    unsafe {
        if bindings::get_option(&mut value, &mut level) != 0 {
            core::ptr::addr_of_mut!(bindings::console_printk).cast::<c_int>().write(level);
            return 0;
        }
    }
    -(bindings::EINVAL as c_int)
}

super::main_setup::setup_param!("reset_devices", reset_devices_record, Some(set_reset_devices), 0);
super::main_setup::setup_param!("debug", debug_record, Some(debug_kernel), 1);
super::main_setup::setup_param!("quiet", quiet_record, Some(quiet_kernel), 1);
super::main_setup::setup_param!("loglevel", loglevel_record, Some(loglevel), 1);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
