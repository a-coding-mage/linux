/* SPDX-License-Identifier: GPL-2.0 */

// C dependencies: <linux/tty.h>, <linux/interrupt.h>, <linux/keyboard.h>

extern "C" {
    pub static mut func_table: [*mut ::core::ffi::c_char; MAX_NR_FUNC];
}

/*
 * kbd->xxx contains the VC-local things (flag settings etc..)
 *
 * Note: externally visible are LED_SCR, LED_NUM, LED_CAP defined in kd.h
 *       The code in KDGETLED / KDSETLED depends on the internal and
 *       external order being the same.
 *
 * Note: lockstate is used as index in the array key_map.
 */
#[repr(C)]
pub struct kbd_struct {
    pub lockstate: u8,
    /* 8 modifiers - the names do not have any meaning at all;
       they can be associated to arbitrarily chosen keys */
    pub slockstate: u8, // for `sticky' Shift, Ctrl, etc.
    pub ledmode: u8, // 1-bit C bit-field
    pub ledflagstate: u8, // 4-bit C bit-field; flags, not lights
    pub default_ledflagstate: u8, // 4-bit C bit-field
    pub kbdmode: u8, // 3-bit C bit-field; one 3-bit value
    pub modeflags: u8, // 5-bit C bit-field
}

pub const VC_SHIFTLOCK: i32 = KG_SHIFT;
pub const VC_ALTGRLOCK: i32 = KG_ALTGR;
pub const VC_CTRLLOCK: i32 = KG_CTRL;
pub const VC_ALTLOCK: i32 = KG_ALT;
pub const VC_SHIFTLLOCK: i32 = KG_SHIFTL;
pub const VC_SHIFTRLOCK: i32 = KG_SHIFTR;
pub const VC_CTRLLLOCK: i32 = KG_CTRLL;
pub const VC_CTRLRLOCK: i32 = KG_CTRLR;

pub const LED_SHOW_FLAGS: i32 = 0;
pub const LED_SHOW_IOCTL: i32 = 1;
pub const VC_SCROLLOCK: i32 = 0;
pub const VC_NUMLOCK: i32 = 1;
pub const VC_CAPSLOCK: i32 = 2;
pub const VC_KANALOCK: i32 = 3;
pub const VC_XLATE: i32 = 0;
pub const VC_MEDIUMRAW: i32 = 1;
pub const VC_RAW: i32 = 2;
pub const VC_UNICODE: i32 = 3;
pub const VC_OFF: i32 = 4;
pub const VC_APPLIC: i32 = 0;
pub const VC_CKMODE: i32 = 1;
pub const VC_REPEAT: i32 = 2;
pub const VC_CRLF: i32 = 3;
pub const VC_META: i32 = 4;

extern "C" {
    pub fn kbd_init() -> i32;
    pub fn setledstate(kbd: *mut kbd_struct, led: u32);
    pub static mut do_poke_blanked_console: i32;
    pub static mut kbd_ledfunc: Option<unsafe extern "C" fn(led: u32)>;
    pub fn set_console(nr: i32) -> i32;
    pub fn schedule_console_callback();
}

#[inline]
pub unsafe fn vc_kbd_mode(kbd: *mut kbd_struct, flag: i32) -> i32 {
    (((*kbd).modeflags as i32 >> flag) & 1)
}

#[inline]
pub unsafe fn vc_kbd_led(kbd: *mut kbd_struct, flag: i32) -> i32 {
    (((*kbd).ledflagstate as i32 >> flag) & 1)
}

#[inline]
pub unsafe fn set_vc_kbd_mode(kbd: *mut kbd_struct, flag: i32) {
    (*kbd).modeflags |= (1u8).wrapping_shl(flag as u32);
}

#[inline]
pub unsafe fn set_vc_kbd_led(kbd: *mut kbd_struct, flag: i32) {
    (*kbd).ledflagstate |= (1u8).wrapping_shl(flag as u32);
}

#[inline]
pub unsafe fn clr_vc_kbd_mode(kbd: *mut kbd_struct, flag: i32) {
    (*kbd).modeflags &= !(1u8.wrapping_shl(flag as u32));
}

#[inline]
pub unsafe fn clr_vc_kbd_led(kbd: *mut kbd_struct, flag: i32) {
    (*kbd).ledflagstate &= !(1u8.wrapping_shl(flag as u32));
}

#[inline]
pub unsafe fn chg_vc_kbd_lock(kbd: *mut kbd_struct, flag: i32) {
    (*kbd).lockstate ^= (1u8).wrapping_shl(flag as u32);
}

#[inline]
pub unsafe fn chg_vc_kbd_slock(kbd: *mut kbd_struct, flag: i32) {
    (*kbd).slockstate ^= (1u8).wrapping_shl(flag as u32);
}

#[inline]
pub unsafe fn chg_vc_kbd_mode(kbd: *mut kbd_struct, flag: i32) {
    (*kbd).modeflags ^= (1u8).wrapping_shl(flag as u32);
}

#[inline]
pub unsafe fn chg_vc_kbd_led(kbd: *mut kbd_struct, flag: i32) {
    (*kbd).ledflagstate ^= (1u8).wrapping_shl(flag as u32);
}

#[inline]
pub const fn u(x: i32) -> i32 { x ^ 0xf000 }

pub const BRL_UC_ROW: i32 = 0x2800;

pub struct console;

extern "C" {
    pub fn vt_set_leds_compute_shiftstate();
    pub static mut keymap_count: u32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
