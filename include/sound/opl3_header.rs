/* SPDX-License-Identifier: GPL-2.0-or-later */

/* Definitions of the OPL-3 registers. */

pub const OPL3_REG_TEST: u8 = 0x01;
pub const OPL3_ENABLE_WAVE_SELECT: u8 = 0x20;
pub const OPL3_REG_TIMER1: u8 = 0x02;
pub const OPL3_REG_TIMER2: u8 = 0x03;
pub const OPL3_REG_TIMER_CONTROL: u8 = 0x04;
pub const OPL3_IRQ_RESET: u8 = 0x80;
pub const OPL3_TIMER1_MASK: u8 = 0x40;
pub const OPL3_TIMER2_MASK: u8 = 0x20;
pub const OPL3_TIMER1_START: u8 = 0x01;
pub const OPL3_TIMER2_START: u8 = 0x02;
pub const OPL3_REG_CONNECTION_SELECT: u8 = 0x04;
pub const OPL3_LEFT_4OP_0: u8 = 0x01;
pub const OPL3_LEFT_4OP_1: u8 = 0x02;
pub const OPL3_LEFT_4OP_2: u8 = 0x04;
pub const OPL3_RIGHT_4OP_0: u8 = 0x08;
pub const OPL3_RIGHT_4OP_1: u8 = 0x10;
pub const OPL3_RIGHT_4OP_2: u8 = 0x20;
pub const OPL3_REG_MODE: u8 = 0x05;
pub const OPL3_OPL3_ENABLE: u8 = 0x01;
pub const OPL3_OPL4_ENABLE: u8 = 0x02;
pub const OPL3_REG_KBD_SPLIT: u8 = 0x08;
pub const OPL3_COMPOSITE_SINE_WAVE_MODE: u8 = 0x80;
pub const OPL3_KEYBOARD_SPLIT: u8 = 0x40;
pub const OPL3_REG_PERCUSSION: u8 = 0xbd;
pub const OPL3_TREMOLO_DEPTH: u8 = 0x80;
pub const OPL3_VIBRATO_DEPTH: u8 = 0x40;
pub const OPL3_PERCUSSION_ENABLE: u8 = 0x20;
pub const OPL3_BASSDRUM_ON: u8 = 0x10;
pub const OPL3_SNAREDRUM_ON: u8 = 0x08;
pub const OPL3_TOMTOM_ON: u8 = 0x04;
pub const OPL3_CYMBAL_ON: u8 = 0x02;
pub const OPL3_HIHAT_ON: u8 = 0x01;

pub const OPL3_REG_AM_VIB: u8 = 0x20;
pub const OPL3_TREMOLO_ON: u8 = 0x80;
pub const OPL3_VIBRATO_ON: u8 = 0x40;
pub const OPL3_SUSTAIN_ON: u8 = 0x20;
pub const OPL3_KSR: u8 = 0x10;
pub const OPL3_MULTIPLE_MASK: u8 = 0x0f;
pub const OPL3_REG_KSL_LEVEL: u8 = 0x40;
pub const OPL3_KSL_MASK: u8 = 0xc0;
pub const OPL3_TOTAL_LEVEL_MASK: u8 = 0x3f;
pub const OPL3_REG_ATTACK_DECAY: u8 = 0x60;
pub const OPL3_ATTACK_MASK: u8 = 0xf0;
pub const OPL3_DECAY_MASK: u8 = 0x0f;
pub const OPL3_REG_SUSTAIN_RELEASE: u8 = 0x80;
pub const OPL3_SUSTAIN_MASK: u8 = 0xf0;
pub const OPL3_RELEASE_MASK: u8 = 0x0f;
pub const OPL3_REG_WAVE_SELECT: u8 = 0xe0;
pub const OPL3_WAVE_SELECT_MASK: u8 = 0x07;
pub const OPL3_REG_FNUM_LOW: u8 = 0xa0;
pub const OPL3_REG_KEYON_BLOCK: u8 = 0xb0;
pub const OPL3_KEYON_BIT: u8 = 0x20;
pub const OPL3_BLOCKNUM_MASK: u8 = 0x1c;
pub const OPL3_FNUM_HIGH_MASK: u8 = 0x03;
pub const OPL3_REG_FEEDBACK_CONNECTION: u8 = 0xc0;
pub const OPL3_FEEDBACK_MASK: u8 = 0x0e;
pub const OPL3_CONNECTION_BIT: u8 = 0x01;
pub const OPL3_STEREO_BITS: u8 = 0x30;
pub const OPL3_VOICE_TO_LEFT: u8 = 0x10;
pub const OPL3_VOICE_TO_RIGHT: u8 = 0x20;

pub const OPL3_LEFT: u16 = 0x0000;
pub const OPL3_RIGHT: u16 = 0x0100;
pub const OPL3_HW_AUTO: u16 = 0x0000;
pub const OPL3_HW_OPL2: u16 = 0x0200;
pub const OPL3_HW_OPL3: u16 = 0x0300;
pub const OPL3_HW_OPL3_SV: u16 = 0x0301;
pub const OPL3_HW_OPL3_CS: u16 = 0x0302;
pub const OPL3_HW_OPL3_FM801: u16 = 0x0303;
pub const OPL3_HW_OPL3_CS4281: u16 = 0x0304;
pub const OPL3_HW_OPL4: u16 = 0x0400;
pub const OPL3_HW_OPL4_ML: u16 = 0x0401;
pub const OPL3_HW_MASK: u16 = 0xff00;
pub const MAX_OPL2_VOICES: usize = 9;
pub const MAX_OPL3_VOICES: usize = 18;

#[repr(C, packed)]
pub struct FmOperator { pub am_vib: u8, pub ksl_level: u8, pub attack_decay: u8, pub sustain_release: u8, pub wave_select: u8 }

#[repr(C, packed)]
pub struct FmInstrument {
    pub op: [FmOperator; 4], pub feedback_connection: [u8; 2], pub echo_delay: u8,
    pub echo_atten: u8, pub chorus_spread: u8, pub trnsps: u8, pub fix_dur: u8,
    pub modes: u8, pub fix_key: u8,
}

pub const FM_PATCH_OPL2: u8 = 0x01;
pub const FM_PATCH_OPL3: u8 = 0x02;

#[repr(C)]
pub struct FmPatch {
    pub prog: u8, pub bank: u8, pub type_: u8, pub inst: FmInstrument,
    pub name: [std::os::raw::c_char; 24], pub next: *mut FmPatch,
}

pub const SNDRV_OPL3_ST_OFF: i32 = 0;
pub const SNDRV_OPL3_ST_ON_2OP: i32 = 1;
pub const SNDRV_OPL3_ST_ON_4OP: i32 = 2;
pub const SNDRV_OPL3_ST_NOT_AVAIL: i32 = -1;

#[repr(C)]
pub struct SndOpl3Voice {
    pub state: i32, pub time: std::os::raw::c_uint, pub note: u8,
    pub note_off: std::os::raw::c_ulong, pub note_off_check: i32, pub keyon_reg: u8,
    pub chan: *mut SndMidiChannel,
}

#[repr(C)]
pub struct SndOpl3 {
    pub l_port: std::os::raw::c_ulong, pub r_port: std::os::raw::c_ulong,
    pub res_l_port: *mut Resource, pub res_r_port: *mut Resource, pub hardware: u16,
    pub command: Option<unsafe extern "C" fn(*mut SndOpl3, u16, u8)>, pub timer_enable: u16,
    pub seq_dev_num: i32, pub timer1: *mut SndTimer, pub timer2: *mut SndTimer,
    pub timer_lock: SpinlockT, pub private_data: *mut std::ffi::c_void,
    pub private_free: Option<unsafe extern "C" fn(*mut SndOpl3)>, pub hwdep: *mut SndHwdep,
    pub reg_lock: SpinlockT, pub card: *mut SndCard, pub fm_mode: u8, pub rhythm: u8,
    pub max_voices: u8,
    // Fields below are present when CONFIG_SND_SEQUENCER is enabled in the C build.
    pub synth_mode: i32, pub seq_client: i32, pub seq_dev: *mut SndSeqDevice,
    pub chset: *mut SndMidiChannelSet, pub oss_seq_dev: *mut SndSeqDevice,
    pub oss_chset: *mut SndMidiChannelSet, pub patch_table: [*mut FmPatch; 32],
    pub voices: [SndOpl3Voice; MAX_OPL3_VOICES], pub use_time: i32, pub connection_reg: u16,
    pub drum_reg: u8, pub voice_lock: SpinlockT, pub tlist: TimerList,
    pub sys_timer_status: i32, pub sys_timer_lock: SpinlockT,
}

pub const SNDRV_OPL3_MODE_SYNTH: i32 = 0;
pub const SNDRV_OPL3_MODE_SEQ: i32 = 1;

pub type SpinlockT = std::ffi::c_void;
pub type Resource = std::ffi::c_void;
pub type SndMidiChannel = std::ffi::c_void;
pub type SndTimer = std::ffi::c_void;
pub type SndHwdep = std::ffi::c_void;
pub type SndCard = std::ffi::c_void;
pub type SndSeqDevice = std::ffi::c_void;
pub type SndMidiChannelSet = std::ffi::c_void;
pub type TimerList = std::ffi::c_void;

extern "C" {
    pub fn snd_opl3_interrupt(hw: *mut SndHwdep);
    pub fn snd_opl3_new(card: *mut SndCard, hardware: u16, ropl3: *mut *mut SndOpl3) -> i32;
    pub fn snd_opl3_init(opl3: *mut SndOpl3) -> i32;
    pub fn snd_opl3_create(card: *mut SndCard, l_port: std::os::raw::c_ulong, r_port: std::os::raw::c_ulong, hardware: u16, integrated: i32, opl3: *mut *mut SndOpl3) -> i32;
    pub fn snd_opl3_timer_new(opl3: *mut SndOpl3, timer1_dev: i32, timer2_dev: i32) -> i32;
    pub fn snd_opl3_hwdep_new(opl3: *mut SndOpl3, device: i32, seq_device: i32, rhwdep: *mut *mut SndHwdep) -> i32;
    pub fn snd_opl3_open(hw: *mut SndHwdep, file: *mut std::ffi::c_void) -> i32;
    pub fn snd_opl3_ioctl(hw: *mut SndHwdep, file: *mut std::ffi::c_void, cmd: u32, arg: std::os::raw::c_ulong) -> i32;
    pub fn snd_opl3_release(hw: *mut SndHwdep, file: *mut std::ffi::c_void) -> i32;
    pub fn snd_opl3_reset(opl3: *mut SndOpl3);
    pub fn snd_opl3_write(hw: *mut SndHwdep, buf: *const u8, count: i64, offset: *mut i64) -> i64;
    pub fn snd_opl3_load_patch(opl3: *mut SndOpl3, prog: i32, bank: i32, type_: i32, name: *const std::os::raw::c_char, ext: *const u8, data: *const u8) -> i32;
    pub fn snd_opl3_find_patch(opl3: *mut SndOpl3, prog: i32, bank: i32, create_patch: i32) -> *mut FmPatch;
    pub fn snd_opl3_clear_patches(opl3: *mut SndOpl3);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
