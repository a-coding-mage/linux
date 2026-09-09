/* SPDX-License-Identifier: GPL-2.0 */

/*
 * linux/include/linux/atari_joystick.h
 * header file for Atari Joystick driver
 * by Robert de Vries (robert@and.nl) on 19Jul93
 */

use core::ffi::{c_char, c_int};

unsafe extern "C" {
    pub fn atari_joystick_interrupt(arg: *mut c_char);
    pub fn atari_joystick_init() -> c_int;
    pub static mut atari_mouse_buttons: c_int;
}

#[repr(C)]
pub struct joystick_status {
    pub fire: c_char,
    pub dir: c_char,
    pub ready: c_int,
    pub active: c_int,
    pub wait: wait_queue_head_t,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
