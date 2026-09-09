// SPDX-License-Identifier: GPL-2.0-only
/*
 * arch/arm/common/bL_switcher_dummy_if.c -- b.L switcher dummy interface
 *
 * Created by:\tNicolas Pitre, November 2012
 * Copyright: \t(C) 2012-2013  Linaro Limited
 *
 * Dummy interface to user space for debugging purpose only.
 */

use core::ffi::{c_char, c_int, c_ulong, c_void};

// Linux kernel declarations supplied by the surrounding build.
#[repr(C)]
pub struct file {
    _private: [u8; 0],
}

#[repr(C)]
pub struct file_operations {
    pub write: Option<unsafe extern "C" fn(*mut file, *const c_char, usize, *mut i64) -> isize>,
    pub owner: *mut c_void,
}

#[repr(C)]
pub struct miscdevice {
    pub minor: c_int,
    pub name: *const c_char,
    pub fops: *const file_operations,
}

extern "C" {
    fn copy_from_user(to: *mut c_void, from: *const c_void, n: usize) -> c_ulong;
    fn bL_switch_request(cpu: u32, cluster: u32) -> c_int;
    static THIS_MODULE: c_void;
}

const EINVAL: isize = 22;
const EFAULT: isize = 14;
const MISC_DYNAMIC_MINOR: c_int = 255;

unsafe extern "C" fn bL_switcher_write(
    _file: *mut file,
    buf: *const c_char,
    len: usize,
    _pos: *mut i64,
) -> isize {
    // pr_debug("%s\n", __func__);

    if len < 3 {
        return -EINVAL;
    }

    let mut val = [0u8; 3];
    if copy_from_user(
        val.as_mut_ptr() as *mut c_void,
        buf as *const c_void,
        3,
    ) != 0
    {
        return -EFAULT;
    }

    /* format: <cpu#>,<cluster#> */
    if val[0] < b'0'
        || val[0] > b'9'
        || val[1] != b','
        || val[2] < b'0'
        || val[2] > b'1'
    {
        return -EINVAL;
    }

    let cpu = (val[0] - b'0') as u32;
    let cluster = (val[2] - b'0') as u32;
    let ret = bL_switch_request(cpu, cluster);

    if ret != 0 {
        ret as isize
    } else {
        len as isize
    }
}

static bL_switcher_fops: file_operations = file_operations {
    write: Some(bL_switcher_write),
    owner: unsafe { &THIS_MODULE as *const c_void as *mut c_void },
};

static mut bL_switcher_device: miscdevice = miscdevice {
    minor: MISC_DYNAMIC_MINOR,
    name: b"b.L_switcher\0".as_ptr() as *const c_char,
    fops: &bL_switcher_fops,
};

// module_misc_device(bL_switcher_device);

// MODULE_AUTHOR("Nicolas Pitre <nico@linaro.org>");
// MODULE_LICENSE("GPL v2");
// MODULE_DESCRIPTION("big.LITTLE switcher dummy user interface");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
