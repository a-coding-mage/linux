// SPDX-License-Identifier: GPL-2.0

// C dependencies:
//   <linux/console.h>
//   <linux/device.h>
//   <linux/module.h>
//   <asm/prom.h>
//   <asm/video.h>

use core::ffi::c_char;

#[repr(C)]
pub struct device {
    pub of_node: *mut device_node,
}

#[repr(C)]
pub struct device_node {
    _private: [u8; 0],
}

extern "C" {
    pub static mut console_set_on_cmdline: bool;
    pub static mut of_console_device: *mut device_node;
}

#[no_mangle]
pub unsafe extern "C" fn video_is_primary_device(dev: *mut device) -> bool {
    let node: *mut device_node = (*dev).of_node;

    if console_set_on_cmdline {
        return false;
    }

    if !node.is_null() && node == of_console_device {
        return true;
    }

    false
}

// EXPORT_SYMBOL(video_is_primary_device);

// MODULE_DESCRIPTION("Sparc video helpers");
pub static MODULE_DESCRIPTION: &[u8] = b"Sparc video helpers\0";

// MODULE_LICENSE("GPL");
pub static MODULE_LICENSE: &[u8] = b"GPL\0";


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
