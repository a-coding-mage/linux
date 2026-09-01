// SPDX-License-Identifier: GPL-2.0-only
//
// Common code for Cirrus side-codecs.
//
// Copyright (C) 2021, 2023 Cirrus Logic, Inc. and
//               Cirrus Logic International Semiconductor Ltd.

// C dependencies:
// #include <linux/dev_printk.h>
// #include <linux/gpio/consumer.h>
// #include <linux/module.h>
// #include "cirrus_scodec.h"

use core::ffi::{c_char, c_int, c_void};
use core::ptr;

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct gpio_desc {
    _private: [u8; 0],
}

pub const ENOENT: c_int = 2;
pub const EINVAL: c_int = 22;

// External Linux GPIO/dev_printk helpers supplied by kernel headers/bindings.
extern "C" {
    static GPIOD_IN: c_int;

    fn dev_dbg(dev: *mut device, fmt: *const c_char, ...);
    fn gpiod_get_index(
        dev: *mut device,
        con_id: *const c_char,
        idx: c_uint,
        flags: c_int,
    ) -> *mut gpio_desc;
    fn gpiod_get_value_cansleep(desc: *mut gpio_desc) -> c_int;
    fn gpiod_put(desc: *mut gpio_desc);
    fn gpiod_count(dev: *mut device, con_id: *const c_char) -> c_int;
    fn IS_ERR(ptr: *const c_void) -> bool;
    fn PTR_ERR(ptr: *const c_void) -> c_int;
}

#[allow(non_camel_case_types)]
type c_uint = u32;

#[no_mangle]
pub unsafe extern "C" fn cirrus_scodec_get_speaker_id(
    dev: *mut device,
    amp_index: c_int,
    num_amps: c_int,
    fixed_gpio_id: c_int,
) -> c_int {
    let mut speaker_id_desc: *mut gpio_desc;
    let mut speaker_id: c_int = -ENOENT;

    if fixed_gpio_id >= 0 {
        dev_dbg(
            dev,
            b"Found Fixed Speaker ID GPIO (index = %d)\n\0".as_ptr() as *const c_char,
            fixed_gpio_id,
        );
        speaker_id_desc = gpiod_get_index(dev, ptr::null(), fixed_gpio_id as c_uint, GPIOD_IN);
        if IS_ERR(speaker_id_desc as *const c_void) {
            speaker_id = PTR_ERR(speaker_id_desc as *const c_void);
            return speaker_id;
        }
        speaker_id = gpiod_get_value_cansleep(speaker_id_desc);
        gpiod_put(speaker_id_desc);
    } else {
        let mut base_index: c_int;
        let mut gpios_per_amp: c_int;
        let count: c_int;
        let mut tmp: c_int;
        let mut i: c_int;

        count = gpiod_count(dev, b"spk-id\0".as_ptr() as *const c_char);
        if count > 0 {
            speaker_id = 0;
            gpios_per_amp = count / num_amps;
            base_index = gpios_per_amp * amp_index;

            if count % num_amps != 0 {
                return -EINVAL;
            }

            dev_dbg(
                dev,
                b"Found %d Speaker ID GPIOs per Amp\n\0".as_ptr() as *const c_char,
                gpios_per_amp,
            );

            i = 0;
            while i < gpios_per_amp {
                speaker_id_desc = gpiod_get_index(
                    dev,
                    b"spk-id\0".as_ptr() as *const c_char,
                    (i + base_index) as c_uint,
                    GPIOD_IN,
                );
                if IS_ERR(speaker_id_desc as *const c_void) {
                    speaker_id = PTR_ERR(speaker_id_desc as *const c_void);
                    break;
                }
                tmp = gpiod_get_value_cansleep(speaker_id_desc);
                gpiod_put(speaker_id_desc);
                if tmp < 0 {
                    speaker_id = tmp;
                    break;
                }
                speaker_id |= tmp << i;
                i += 1;
            }
        }
    }

    dev_dbg(
        dev,
        b"Speaker ID = %d\n\0".as_ptr() as *const c_char,
        speaker_id,
    );

    speaker_id
}

// EXPORT_SYMBOL_NS_GPL(cirrus_scodec_get_speaker_id, "SND_HDA_CIRRUS_SCODEC");

// MODULE_DESCRIPTION("HDA Cirrus side-codec library");
// MODULE_AUTHOR("Richard Fitzgerald <rf@opensource.cirrus.com>");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
