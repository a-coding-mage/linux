/*
 * edac_module.c
 *
 * (C) 2007 www.softwarebitmaker.com
 *
 * This file is licensed under the terms of the GNU General Public
 * License version 2. This program is licensed "as is" without any
 * warranty of any kind, whether express or implied.
 *
 * Author: Doug Thompson <dougthompson@xmission.com>
 */

use core::ffi::{c_char, c_int, c_ulong, c_void};

/* Dependencies supplied by the surrounding kernel translation. */
extern "C" {
    fn kstrtoul(buf: *const c_char, base: c_uint, val: *mut c_ulong) -> c_int;
    fn param_set_int(buf: *const c_char, kp: *const kernel_param) -> c_int;
    fn subsys_system_register(subsys: *const bus_type, groups: *const *const c_void) -> c_int;
    fn printk(fmt: *const c_char, ...);
    fn bus_unregister(subsys: *const bus_type);
    fn edac_printk(level: c_int, area: c_int, fmt: *const c_char, ...);
    fn edac_pci_clear_parity_errors();
    fn edac_mc_sysfs_init() -> c_int;
    fn edac_mc_sysfs_exit();
    fn edac_debugfs_init();
    fn edac_debugfs_exit();
    fn edac_workqueue_setup() -> c_int;
    fn edac_workqueue_teardown();
    fn edac_dbg(level: c_int, fmt: *const c_char, ...);
}

type c_uint = u32;

#[repr(C)]
pub struct kernel_param {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bus_type {
    pub name: *const c_char,
    pub dev_name: *const c_char,
}

const EDAC_VERSION: &[u8] = b"Ver: 3.0.0\0";

#[cfg(CONFIG_EDAC_DEBUG)]
unsafe fn edac_set_debug_level(buf: *const c_char, kp: *const kernel_param) -> c_int {
    let mut val: c_ulong = 0;
    let ret = kstrtoul(buf, 0, &mut val);
    if ret != 0 {
        return ret;
    }
    if val > 4 {
        return -22; // -EINVAL
    }
    param_set_int(buf, kp)
}

#[cfg(CONFIG_EDAC_DEBUG)]
#[no_mangle]
pub static mut edac_debug_level: c_int = 2;

/* Values of 0 to 4 will generate output. */

pub const OP_RUNNING_POLL: c_int = 0;
pub const OP_RUNNING_INTERRUPT: c_int = 1;
pub const OP_RUNNING_POLL_INTR: c_int = 2;
pub const OP_ALLOC: c_int = 3;
pub const OP_OFFLINE: c_int = 4;

pub unsafe fn edac_op_state_to_string(opstate: c_int) -> *mut c_char {
    if opstate == OP_RUNNING_POLL {
        b"POLLED\0" as *const u8 as *mut c_char
    } else if opstate == OP_RUNNING_INTERRUPT {
        b"INTERRUPT\0" as *const u8 as *mut c_char
    } else if opstate == OP_RUNNING_POLL_INTR {
        b"POLL-INTR\0" as *const u8 as *mut c_char
    } else if opstate == OP_ALLOC {
        b"ALLOC\0" as *const u8 as *mut c_char
    } else if opstate == OP_OFFLINE {
        b"OFFLINE\0" as *const u8 as *mut c_char
    } else {
        b"UNKNOWN\0" as *const u8 as *mut c_char
    }
}

/* sysfs object: /sys/devices/system/edac; need to export to other files. */
static edac_subsys: bus_type = bus_type {
    name: b"edac\0".as_ptr() as *const c_char,
    dev_name: b"edac\0".as_ptr() as *const c_char,
};

unsafe fn edac_subsys_init() -> c_int {
    let err = subsys_system_register(&edac_subsys, core::ptr::null());
    if err != 0 {
        printk(b"Error registering toplevel EDAC sysfs dir\n\0".as_ptr() as *const c_char);
    }
    err
}

unsafe fn edac_subsys_exit() {
    bus_unregister(&edac_subsys);
}

pub unsafe fn edac_get_sysfs_subsys() -> *const bus_type {
    &edac_subsys
}

unsafe fn edac_init() -> c_int {
    let mut err: c_int = 0;
    edac_printk(6, 0, b"Ver: 3.0.0\n\0".as_ptr() as *const c_char);
    err = edac_subsys_init();
    if err != 0 {
        return err;
    }
    edac_pci_clear_parity_errors();
    err = edac_mc_sysfs_init();
    if err != 0 {
        edac_subsys_exit();
        return err;
    }
    edac_debugfs_init();
    err = edac_workqueue_setup();
    if err != 0 {
        edac_printk(3, 0, b"Failure initializing workqueue\n\0".as_ptr() as *const c_char);
        edac_debugfs_exit();
        edac_mc_sysfs_exit();
        edac_subsys_exit();
    }
    err
}

unsafe fn edac_exit() {
    edac_dbg(0, b"\n\0".as_ptr() as *const c_char);
    edac_workqueue_teardown();
    edac_mc_sysfs_exit();
    edac_debugfs_exit();
    edac_subsys_exit();
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
