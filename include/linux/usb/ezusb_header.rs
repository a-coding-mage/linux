/* SPDX-License-Identifier: GPL-2.0 */

// Header guard: __EZUSB_H

use core::ffi::{c_char, c_int};

// `struct usb_device` is supplied by an external dependency.
pub enum usb_device {}

unsafe extern "C" {
    pub fn ezusb_fx1_set_reset(dev: *mut usb_device, reset_bit: u8) -> c_int;
    pub fn ezusb_fx1_ihex_firmware_download(
        dev: *mut usb_device,
        firmware_path: *const c_char,
    ) -> c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
