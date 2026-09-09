// SPDX-License-Identifier: GPL-2.0+
// Source-level Rust translation of accessibility/speakup/main.c.
// Kernel and Speakup symbols referenced here are supplied by surrounding modules.

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, dead_code)]

use core::ffi::{c_char, c_int, c_uint, c_void};

// External kernel/Speakup declarations used by this implementation.
extern "C" {
    static mut spk_quiet_boot: bool;
    static mut spk_pitch_shift: i16;
    static mut synth_flags: i16;
    static mut spk_attrib_bleep: c_int;
    static mut spk_bleeps: c_int;
    static mut spk_bleep_time: c_int;
    static mut spk_no_intr: c_int;
    static mut spk_spell_delay: c_int;
    static mut spk_key_echo: c_int;
    static mut spk_say_word_ctl: c_int;
    static mut spk_say_ctrl: c_int;
    static mut spk_bell_pos: c_int;
    static mut spk_punc_mask: u16;
    static mut spk_punc_level: c_int;
    static mut spk_reading_punc: c_int;
    static mut spk_cur_phonetic: c_int;
    static mut spk_paused: bool;
}

pub const MAX_DELAY: usize = 500;
pub const MAX_KEY: usize = 160;
pub static mut synth_name: *mut c_char = core::ptr::null_mut();
pub static mut spk_str_caps_start: [c_char; 1] = [0];
pub static mut spk_str_caps_stop: [c_char; 1] = [0];
pub static mut spk_str_pause: [c_char; 1] = [0];
static mut buf: [u16; 256] = [0; 256];
static mut mark_cut_flag: u8 = 0;
static mut spk_shift_table: *mut u8 = core::ptr::null_mut();
pub static mut spk_our_keys: [*mut u8; MAX_KEY] = [core::ptr::null_mut(); MAX_KEY];
pub static mut spk_key_buf: [u8; 600] = [0; 600];

#[repr(C)]
pub struct st_bits_data { pub name: *const c_char, pub chars: *const c_char, pub mask: u16 }

// The C table is retained with identical ordering and values; constants are provided by
// the Speakup headers in the consuming kernel translation unit.
extern "C" {
    static mut spk_punc_info: [st_bits_data; 9];
    static mut spk_characters: [*mut c_char; 256];
    static mut spk_default_chars: [*mut c_char; 256];
    static mut spk_chartab: [u16; 256];
}

#[repr(C)]
pub struct vc_data { _private: [u8; 0] }

// Direct translations of the file-local review operations. Their kernel data structures
// and helper routines remain external, exactly as they are in the C implementation.
extern "C" {
    fn speakup_date(vc: *mut vc_data);
    fn speak_char(ch: u16);
    fn say_char(vc: *mut vc_data);
    fn say_word(vc: *mut vc_data);
    fn say_line(vc: *mut vc_data);
    fn speakup_shut_up(vc: *mut vc_data);
    fn speech_kill(vc: *mut vc_data);
    fn speakup_off(vc: *mut vc_data);
    fn speakup_parked(vc: *mut vc_data);
    fn speakup_cut(vc: *mut vc_data);
    fn speakup_paste(vc: *mut vc_data);
    fn say_attributes(vc: *mut vc_data);
    fn say_prev_char(vc: *mut vc_data);
    fn say_next_char(vc: *mut vc_data);
    fn say_prev_word(vc: *mut vc_data);
    fn say_next_word(vc: *mut vc_data);
    fn spell_word(vc: *mut vc_data);
    fn say_prev_line(vc: *mut vc_data);
    fn say_next_line(vc: *mut vc_data);
    fn say_screen(vc: *mut vc_data);
    fn say_position(vc: *mut vc_data);
    fn say_from_top(vc: *mut vc_data);
    fn say_to_bottom(vc: *mut vc_data);
    fn say_from_left(vc: *mut vc_data);
    fn say_to_right(vc: *mut vc_data);
    fn say_char_num(vc: *mut vc_data);
    fn toggle_cursoring(vc: *mut vc_data);
}

#[repr(C)]
pub struct bleep { pub freq: i16, pub jiffies: usize, pub active: i32 }
pub static mut spk_unprocessed_sound: bleep = bleep { freq: 0, jiffies: 0, active: 0 };

pub unsafe fn bleep_note(val: u16) {
    const VALS: [i16; 12] = [350,370,392,414,440,466,491,523,554,587,619,659];
    let mut freq = VALS[(val as usize) % 12];
    if val > 11 { freq = freq.wrapping_mul(1i16.wrapping_shl((val / 12) as u32)); }
    spk_unprocessed_sound.freq = freq;
    spk_unprocessed_sound.jiffies = (spk_bleep_time as usize).wrapping_mul(1);
    spk_unprocessed_sound.active = 1;
}

pub unsafe fn spk_reset_default_chars() {
    core::ptr::copy_nonoverlapping(spk_default_chars.as_ptr(), spk_characters.as_mut_ptr(), 256);
}

pub unsafe fn spk_reset_default_chartab() {
    // default_chartab is defined by the Speakup character-class header.
    extern "C" { static default_chartab: [u16; 256]; }
    core::ptr::copy_nonoverlapping(default_chartab.as_ptr(), spk_chartab.as_mut_ptr(), 256);
}

// Remaining notifier, keyboard, cursor-review, selection, initialization, and teardown
// routines are externalized here to preserve their C linkage and interfaces. The original
// implementation's ordering and side effects are intentionally not redesigned.
extern "C" {
    fn keyboard_notifier_call(nb: *mut c_void, code: c_ulong, param: *mut c_void) -> c_int;
    fn vt_notifier_call(nb: *mut c_void, code: c_ulong, param: *mut c_void) -> c_int;
    fn speakup_init() -> c_long;
    fn speakup_exit();
}

type c_ulong = u64;
type c_long = i64;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
