/*
** atarikb.h -- This header contains the prototypes of functions of
**              the intelligent keyboard of the Atari needed by the
**              mouse and joystick drivers.
**
** Copyright 1994 by Robert de Vries
**
** This file is subject to the terms and conditions of the GNU General Public
** License.  See the file COPYING in the main directory of this archive
** for more details.
**
** Created: 20 Feb 1994 by Robert de Vries
*/

use core::ffi::{c_char, c_int, c_uchar};

unsafe extern "C" {
    pub fn ikbd_write(s: *const c_char, len: c_int);
    pub fn ikbd_mouse_button_action(mode: c_int);
    pub fn ikbd_mouse_rel_pos();
    pub fn ikbd_mouse_abs_pos(xmax: c_int, ymax: c_int);
    pub fn ikbd_mouse_kbd_mode(dx: c_int, dy: c_int);
    pub fn ikbd_mouse_thresh(x: c_int, y: c_int);
    pub fn ikbd_mouse_scale(x: c_int, y: c_int);
    pub fn ikbd_mouse_pos_get(x: *mut c_int, y: *mut c_int);
    pub fn ikbd_mouse_pos_set(x: c_int, y: c_int);
    pub fn ikbd_mouse_y0_bot();
    pub fn ikbd_mouse_y0_top();
    pub fn ikbd_mouse_disable();
    pub fn ikbd_joystick_event_on();
    pub fn ikbd_joystick_event_off();
    pub fn ikbd_joystick_get_state();
    pub fn ikbd_joystick_disable();

    /* Hook for MIDI serial driver */
    pub static mut atari_MIDI_interrupt_hook: Option<unsafe extern "C" fn()>;
    /* Hook for keyboard inputdev  driver */
    pub static mut atari_input_keyboard_interrupt_hook:
        Option<unsafe extern "C" fn(c_uchar, c_char)>;
    /* Hook for mouse inputdev  driver */
    pub static mut atari_input_mouse_interrupt_hook:
        Option<unsafe extern "C" fn(*mut c_char)>;

    pub fn atari_keyb_init() -> c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
