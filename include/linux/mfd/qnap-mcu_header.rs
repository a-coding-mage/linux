/* SPDX-License-Identifier: GPL-2.0+ */
/*
 * Core definitions for QNAP MCU MFD driver.
 * Copyright (C) 2024 Heiko Stuebner <heiko@sntech.de>
 */

// C dependency: <linux/types.h>

#[repr(C)]
pub struct qnap_mcu {
    _private: [u8; 0],
}

#[repr(C)]
pub struct qnap_mcu_variant {
    pub baud_rate: u32,
    pub num_drives: i32,
    pub fan_pwm_min: i32,
    pub fan_pwm_max: i32,
    pub usb_led: bool,
}

unsafe extern "C" {
    pub fn qnap_mcu_exec(
        mcu: *mut qnap_mcu,
        cmd_data: *const u8,
        cmd_data_size: usize,
        reply_data: *mut u8,
        reply_data_size: usize,
    ) -> i32;

    pub fn qnap_mcu_exec_with_ack(
        mcu: *mut qnap_mcu,
        cmd_data: *const u8,
        cmd_data_size: usize,
    ) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
