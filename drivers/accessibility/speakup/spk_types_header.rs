/* SPDX-License-Identifier: GPL-2.0 */
/* Translated from spk_types.h. Kernel-provided types and symbols remain external dependencies. */

#[repr(C)]
pub enum var_type_t {
    VAR_NUM = 0,
    VAR_TIME,
    VAR_STRING,
    VAR_PROC,
}

#[repr(C)]
pub enum __anonymous_enum_0 {
    E_DEFAULT = 0,
    E_SET,
    E_INC,
    E_DEC,
    E_NEW_DEFAULT,
}

#[repr(C)]
pub enum var_id_t {
    VERSION = 0,
    SYNTH,
    SILENT,
    SYNTH_DIRECT,
    KEYMAP,
    CHARS,
    PUNC_SOME,
    PUNC_MOST,
    PUNC_ALL,
    DELIM,
    REPEATS,
    EXNUMBER,
    DELAY,
    TRIGGER,
    JIFFY,
    FULL,
    BLEEP_TIME,
    CURSOR_TIME,
    BELL_POS,
    SAY_CONTROL,
    SAY_WORD_CTL,
    NO_INTERRUPT,
    KEY_ECHO,
    SPELL_DELAY,
    PUNC_LEVEL,
    READING_PUNC,
    ATTRIB_BLEEP,
    BLEEPS,
    RATE,
    PITCH,
    VOL,
    TONE,
    PUNCT,
    VOICE,
    FREQUENCY,
    LANG,
    DIRECT,
    PAUSE,
    CAPS_START,
    CAPS_STOP,
    CHARTAB,
    INFLECTION,
    FLUSH,
    CUR_PHONETIC,
    MAXVARS,
}

pub type special_func = unsafe extern "C" fn(
    vc: *mut vc_data,
    type_: u8,
    ch: u8,
    key: u16,
) -> i32;

pub const COLOR_BUFFER_SIZE: usize = 160;

#[repr(C)]
pub struct spk_highlight_color_track {
    pub bgcount: [u32; 8],
    pub highbuf: [[u16; COLOR_BUFFER_SIZE]; 8],
    pub highsize: [u32; 8],
    pub rpos: [u64; 8],
    pub rx: [u64; 8],
    pub ry: [u64; 8],
    pub cy: u64,
}

#[repr(C)]
pub struct st_spk_t {
    pub reading_x: u64,
    pub cursor_x: u64,
    pub reading_y: u64,
    pub cursor_y: u64,
    pub reading_pos: u64,
    pub cursor_pos: u64,
    pub go_x: u64,
    pub go_pos: u64,
    pub w_top: u64,
    pub w_bottom: u64,
    pub w_left: u64,
    pub w_right: u64,
    pub w_start: u8,
    pub w_enabled: u8,
    pub reading_attr: u8,
    pub old_attr: u8,
    pub parked: i8,
    pub shut_up: i8,
    pub ht: spk_highlight_color_track,
    pub tty_stopped: i32,
}

#[macro_export]
macro_rules! spk_shut_up { ($vc:expr) => { unsafe { (*(*speakup_console.add((*$vc).vc_num as usize))).shut_up } }; }
#[macro_export]
macro_rules! spk_killed { ($vc:expr) => { (spk_shut_up!($vc) & 0x40) }; }
#[macro_export]
macro_rules! spk_x { ($vc:expr) => { unsafe { (*(*speakup_console.add((*$vc).vc_num as usize))).reading_x } }; }
#[macro_export]
macro_rules! spk_cx { ($vc:expr) => { unsafe { (*(*speakup_console.add((*$vc).vc_num as usize))).cursor_x } }; }
#[macro_export]
macro_rules! spk_y { ($vc:expr) => { unsafe { (*(*speakup_console.add((*$vc).vc_num as usize))).reading_y } }; }
#[macro_export]
macro_rules! spk_cy { ($vc:expr) => { unsafe { (*(*speakup_console.add((*$vc).vc_num as usize))).cursor_y } }; }
#[macro_export]
macro_rules! spk_pos { ($vc:expr) => { unsafe { (*(*speakup_console.add((*$vc).vc_num as usize))).reading_pos } }; }
#[macro_export]
macro_rules! spk_cp { ($vc:expr) => { unsafe { (*(*speakup_console.add((*$vc).vc_num as usize))).cursor_pos } }; }
#[macro_export]
macro_rules! goto_pos { ($vc:expr) => { unsafe { (*(*speakup_console.add((*$vc).vc_num as usize))).go_pos } }; }
#[macro_export]
macro_rules! goto_x { ($vc:expr) => { unsafe { (*(*speakup_console.add((*$vc).vc_num as usize))).go_x } }; }
#[macro_export]
macro_rules! win_top { ($vc:expr) => { unsafe { (*(*speakup_console.add((*$vc).vc_num as usize))).w_top } }; }
#[macro_export]
macro_rules! win_bottom { ($vc:expr) => { unsafe { (*(*speakup_console.add((*$vc).vc_num as usize))).w_bottom } }; }
#[macro_export]
macro_rules! win_left { ($vc:expr) => { unsafe { (*(*speakup_console.add((*$vc).vc_num as usize))).w_left } }; }
#[macro_export]
macro_rules! win_right { ($vc:expr) => { unsafe { (*(*speakup_console.add((*$vc).vc_num as usize))).w_right } }; }
#[macro_export]
macro_rules! win_start { ($vc:expr) => { unsafe { (*(*speakup_console.add((*$vc).vc_num as usize))).w_start } }; }
#[macro_export]
macro_rules! win_enabled { ($vc:expr) => { unsafe { (*(*speakup_console.add((*$vc).vc_num as usize))).w_enabled } }; }
#[macro_export]
macro_rules! spk_attr { ($vc:expr) => { unsafe { (*(*speakup_console.add((*$vc).vc_num as usize))).reading_attr } }; }
#[macro_export]
macro_rules! spk_old_attr { ($vc:expr) => { unsafe { (*(*speakup_console.add((*$vc).vc_num as usize))).old_attr } }; }
#[macro_export]
macro_rules! spk_parked { ($vc:expr) => { unsafe { (*(*speakup_console.add((*$vc).vc_num as usize))).parked } }; }

#[repr(C)]
pub struct st_var_header { pub name: *mut i8, pub var_id: var_id_t, pub var_type: var_type_t, pub p_val: *mut core::ffi::c_void, pub data: *mut core::ffi::c_void }
#[repr(C)]
pub struct num_var_t { pub synth_fmt: *mut i8, pub default_val: i32, pub low: i32, pub high: i32, pub offset: i16, pub multiplier: i16, pub out_str: *mut i8, pub value: i32 }
#[repr(C)]
pub struct punc_var_t { pub var_id: var_id_t, pub value: i16 }
#[repr(C)]
pub struct string_var_t { pub default_val: *mut i8 }
#[repr(C)]
pub union var_t_u { pub n: num_var_t, pub s: string_var_t }
#[repr(C)]
pub struct var_t { pub var_id: var_id_t, pub u: var_t_u }
#[repr(C)]
pub struct st_bits_data { pub name: *mut i8, pub value: *mut i8, pub mask: i16 }
#[repr(C)]
pub struct synth_indexing { pub command: *mut i8, pub lowindex: u8, pub highindex: u8, pub currindex: u8 }

#[repr(C)]
pub struct spk_io_ops {
    pub synth_out: Option<unsafe extern "C" fn(*mut spk_synth, i8) -> i32>,
    pub synth_out_unicode: Option<unsafe extern "C" fn(*mut spk_synth, u16) -> i32>,
    pub send_xchar: Option<unsafe extern "C" fn(*mut spk_synth, i8)>,
    pub tiocmset: Option<unsafe extern "C" fn(*mut spk_synth, u32, u32)>,
    pub synth_in: Option<unsafe extern "C" fn(*mut spk_synth) -> u8>,
    pub synth_in_nowait: Option<unsafe extern "C" fn(*mut spk_synth) -> u8>,
    pub flush_buffer: Option<unsafe extern "C" fn(*mut spk_synth)>,
    pub wait_for_xmitr: Option<unsafe extern "C" fn(*mut spk_synth) -> i32>,
}

#[repr(C)]
pub struct spk_synth {
    pub node: list_head,
    pub name: *const i8, pub version: *const i8, pub long_name: *const i8, pub init: *const i8,
    pub procspeech: i8, pub clear: i8, pub delay: i32, pub trigger: i32, pub jiffies: i32, pub full: i32,
    pub flush_time: i32, pub ser: i32, pub dev_name: *mut i8, pub flags: i16, pub startup: i16,
    pub checkval: i32, pub vars: *mut var_t, pub default_pitch: *mut i32, pub default_vol: *mut i32,
    pub io_ops: *mut spk_io_ops,
    pub probe: Option<unsafe extern "C" fn(*mut spk_synth) -> i32>,
    pub release: Option<unsafe extern "C" fn(*mut spk_synth)>,
    pub synth_immediate: Option<unsafe extern "C" fn(*mut spk_synth, *const i8) -> *const i8>,
    pub catch_up: Option<unsafe extern "C" fn(*mut spk_synth)>, pub flush: Option<unsafe extern "C" fn(*mut spk_synth)>,
    pub is_alive: Option<unsafe extern "C" fn(*mut spk_synth) -> i32>,
    pub synth_adjust: Option<unsafe extern "C" fn(*mut spk_synth, *mut st_var_header) -> i32>,
    pub read_buff_add: Option<unsafe extern "C" fn(u8)>, pub get_index: Option<unsafe extern "C" fn(*mut spk_synth) -> u8>,
    pub indexing: synth_indexing, pub alive: i32, pub attributes: attribute_group, pub dev: *mut core::ffi::c_void,
}

#[macro_export]
macro_rules! module_spk_synth { ($synth:expr) => { module_driver!($synth, synth_add, synth_remove) }; }

#[repr(C)]
pub struct speakup_info_t { pub spinlock: spinlock_t, pub port_tts: i32, pub flushing: i32 }
#[repr(C)]
pub struct bleep { pub freq: i16, pub jiffies: u64, pub active: i32 }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
