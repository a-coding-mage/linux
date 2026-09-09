/* SPDX-License-Identifier: GPL-2.0 */

// Declarations from spk_types.h and i18n.h are supplied by other translation units.

use core::ffi::{c_char, c_int, c_void};

pub const SPEAKUP_VERSION: &[u8] = b"3.1.6\0";
pub const KEY_MAP_VER: c_int = 119;
pub const SHIFT_TBL_SIZE: c_int = 64;
pub const MAX_DESC_LEN: c_int = 72;
pub const MAXVARLEN: c_int = 15;

pub const SYNTH_OK: u16 = 0x0001;
pub const B_ALPHA: u16 = 0x0002;
pub const ALPHA: u16 = 0x0003;
pub const B_CAP: u16 = 0x0004;
pub const A_CAP: u16 = 0x0007;
pub const B_NUM: u16 = 0x0008;
pub const NUM: u16 = 0x0009;
pub const ALPHANUM: u16 = B_ALPHA | B_NUM;
pub const SOME: u16 = 0x0010;
pub const MOST: u16 = 0x0020;
pub const PUNC: u16 = 0x0040;
pub const A_PUNC: u16 = 0x0041;
pub const B_WDLM: u16 = 0x0080;
pub const WDLM: u16 = 0x0081;
pub const B_EXNUM: u16 = 0x0100;
pub const CH_RPT: u16 = 0x0200;
pub const B_CTL: u16 = 0x0400;
pub const A_CTL: u16 = B_CTL + SYNTH_OK;
pub const B_SYM: u16 = 0x0800;
pub const B_CAPSYM: u16 = B_CAP | B_SYM;

// FIXME: u16 in the original implementation.
pub unsafe fn IS_WDLM(x: c_int) -> u16 { spk_chartab[(x as u8) as usize] & B_WDLM }
pub unsafe fn IS_CHAR(x: c_int, typ: u16) -> u16 { spk_chartab[(x as u8) as usize] & typ }
pub unsafe fn IS_TYPE(x: c_int, typ: u16) -> bool { (spk_chartab[(x as u8) as usize] & typ) == typ }

// External types and symbols are provided by the corresponding source files.
pub type special_func = unsafe extern "C" fn();
pub type var_id_t = c_int;
pub type u_char = u8;
pub type u16_t = u16;
pub type s32 = i32;

#[repr(C)] pub struct var_t { _private: [u8; 0] }
#[repr(C)] pub struct st_var_header { _private: [u8; 0] }
#[repr(C)] pub struct punc_var_t { _private: [u8; 0] }
#[repr(C)] pub struct vc_data { _private: [u8; 0] }
#[repr(C)] pub struct tty_struct { _private: [u8; 0] }
#[repr(C)] pub struct wait_queue_head_t { _private: [u8; 0] }
#[repr(C)] pub struct kobject { _private: [u8; 0] }
#[repr(C)] pub struct task_struct { _private: [u8; 0] }
#[repr(C)] pub struct mutex { _private: [u8; 0] }
#[repr(C)] pub struct st_spk_t { _private: [u8; 0] }
#[repr(C)] pub struct spk_synth { _private: [u8; 0] }
#[repr(C)] pub struct st_bits_data { _private: [u8; 0] }
#[repr(C)] pub struct bleep { _private: [u8; 0] }

extern "C" {
    pub fn speakup_thread(data: *mut c_void) -> c_int;
    pub fn spk_reset_default_chars();
    pub fn spk_reset_default_chartab();
    pub fn synth_start();
    pub fn synth_insert_next_index(sent_num: c_int);
    pub fn spk_reset_index_count(sc: c_int);
    pub fn spk_get_index_count(linecount: *mut c_int, sentcount: *mut c_int);
    pub fn spk_set_key_info(key_info: *const u_char, k_buffer: *mut u_char) -> c_int;
    pub fn spk_strlwr(s: *mut c_char) -> *mut c_char;
    pub fn spk_s2uchar(start: *mut c_char, dest: *mut c_char) -> *mut c_char;
    pub fn speakup_kobj_init() -> c_int;
    pub fn speakup_kobj_exit();
    pub fn spk_chartab_get_value(keyword: *mut c_char) -> c_int;
    pub fn speakup_register_var(var: *mut var_t);
    pub fn speakup_unregister_var(var_id: var_id_t);
    pub fn spk_get_var_header(var_id: var_id_t) -> *mut st_var_header;
    pub fn spk_var_header_by_name(name: *const c_char) -> *mut st_var_header;
    pub fn spk_get_punc_var(var_id: var_id_t) -> *mut punc_var_t;
    pub fn spk_set_num_var(val: c_int, var: *mut st_var_header, how: c_int) -> c_int;
    pub fn spk_set_string_var(page: *const c_char, var: *mut st_var_header, len: c_int) -> c_int;
    pub fn spk_set_mask_bits(input: *const c_char, which: c_int, how: c_int) -> c_int;
    pub static mut spk_special_handler: special_func;
    pub fn spk_handle_help(vc: *mut vc_data, typ: u_char, ch: u_char, key: u16) -> c_int;
    pub fn synth_init(name: *mut c_char) -> c_int;
    pub fn synth_release();
    pub fn spk_do_flush();
    pub fn speakup_start_ttys();
    pub fn synth_buffer_add(ch: u16);
    pub fn synth_buffer_clear();
    pub fn speakup_set_selection(tty: *mut tty_struct) -> c_int;
    pub fn speakup_cancel_selection();
    pub fn speakup_paste_selection(tty: *mut tty_struct) -> c_int;
    pub fn speakup_cancel_paste();
    pub fn speakup_register_devsynth();
    pub fn speakup_unregister_devsynth();
    pub fn synth_utf8_get(buf: *const c_char, count: usize, consumed: *mut usize, want: *mut usize) -> s32;
    pub fn synth_write(buf: *const c_char, count: usize);
    pub fn synth_writeu(buf: *const c_char, count: usize);
    pub fn synth_supports_indexing() -> c_int;

    pub static mut spk_sel_cons: *mut vc_data;
    pub static mut spk_xs: u16;
    pub static mut spk_ys: u16;
    pub static mut spk_xe: u16;
    pub static mut spk_ye: u16;
    pub static mut speakup_event: wait_queue_head_t;
    pub static mut speakup_kobj: *mut kobject;
    pub static mut speakup_task: *mut task_struct;
    pub static spk_key_defaults: [u_char; 0];
    pub static mut spk_mutex: mutex;
    pub static mut speakup_console: [*mut st_spk_t; 0];
    pub static mut synth: *mut spk_synth;
    pub static mut spk_pitch_buff: [c_char; 0];
    pub static mut spk_our_keys: [*mut u_char; 0];
    pub static mut spk_punc_masks: [u16; 0];
    pub static mut spk_str_caps_start: [c_char; 0];
    pub static mut spk_str_caps_stop: [c_char; 0];
    pub static mut spk_str_pause: [c_char; 0];
    pub static mut spk_paused: bool;
    pub static spk_punc_info: [st_bits_data; 0];
    pub static mut spk_key_buf: [u_char; 600];
    pub static mut spk_characters: [*mut c_char; 0];
    pub static mut spk_default_chars: [*mut c_char; 0];
    pub static mut spk_chartab: [u16; 0];
    pub static mut spk_no_intr: c_int;
    pub static mut spk_say_ctrl: c_int;
    pub static mut spk_say_word_ctl: c_int;
    pub static mut spk_punc_level: c_int;
    pub static mut spk_reading_punc: c_int;
    pub static mut spk_attrib_bleep: c_int;
    pub static mut spk_bleeps: c_int;
    pub static mut spk_bleep_time: c_int;
    pub static mut spk_bell_pos: c_int;
    pub static mut spk_spell_delay: c_int;
    pub static mut spk_key_echo: c_int;
    pub static mut spk_cur_phonetic: c_int;
    pub static mut spk_punc_mask: u16;
    pub static mut spk_pitch_shift: i16;
    pub static mut synth_flags: i16;
    pub static mut spk_quiet_boot: bool;
    pub static mut synth_name: *mut c_char;
    pub static mut spk_unprocessed_sound: bleep;

    pub fn speakup_add_virtual_keyboard() -> c_int;
    pub fn speakup_remove_virtual_keyboard();
    pub fn speakup_fake_down_arrow();
    pub fn speakup_fake_key_pressed() -> bool;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
