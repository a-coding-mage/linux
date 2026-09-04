// SPDX-License-Identifier: GPL-2.0

#[repr(C)]
pub struct snd_usb_caiaqdev;

extern "C" {
    pub fn snd_usb_caiaq_control_init(cdev: *mut snd_usb_caiaqdev) -> i32;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
