/* SPDX-License-Identifier: GPL-2.0-or-later */
/* Rust translation of sound/emux_synth.h. */

/* External kernel types and constants are supplied by other translated headers. */

pub const SNDRV_EMUX_USE_RAW_EFFECT: bool = true;

pub struct snd_emux;
pub struct snd_emux_port;
pub struct snd_emux_voice;
pub struct snd_emux_effect_table;

#[repr(C)]
pub struct snd_emux_operators {
    pub owner: *mut module,
    pub get_voice: Option<unsafe extern "C" fn(*mut snd_emux, *mut snd_emux_port) -> *mut snd_emux_voice>,
    pub prepare: Option<unsafe extern "C" fn(*mut snd_emux_voice) -> i32>,
    pub trigger: Option<unsafe extern "C" fn(*mut snd_emux_voice)>,
    pub release: Option<unsafe extern "C" fn(*mut snd_emux_voice)>,
    pub update: Option<unsafe extern "C" fn(*mut snd_emux_voice, i32)>,
    pub terminate: Option<unsafe extern "C" fn(*mut snd_emux_voice)>,
    pub free_voice: Option<unsafe extern "C" fn(*mut snd_emux_voice)>,
    pub reset: Option<unsafe extern "C" fn(*mut snd_emux, i32)>,
    pub sample_new: Option<unsafe extern "C" fn(*mut snd_emux, *mut snd_sf_sample, *mut snd_util_memhdr, *const core::ffi::c_void, libc::c_long) -> i32>,
    pub sample_free: Option<unsafe extern "C" fn(*mut snd_emux, *mut snd_sf_sample, *mut snd_util_memhdr) -> i32>,
    pub sample_reset: Option<unsafe extern "C" fn(*mut snd_emux)>,
    pub load_fx: Option<unsafe extern "C" fn(*mut snd_emux, i32, i32, *const core::ffi::c_void, libc::c_long) -> i32>,
    pub sysex: Option<unsafe extern "C" fn(*mut snd_emux, *mut core::ffi::c_char, i32, i32, *mut snd_midi_channel_set)>,
    #[cfg(feature = "CONFIG_SND_SEQUENCER_OSS")]
    pub oss_ioctl: Option<unsafe extern "C" fn(*mut snd_emux, i32, i32, i32) -> i32>,
    pub get_pitch_shift: Option<unsafe extern "C" fn(*mut snd_emux) -> i32>,
}

pub const SNDRV_EMUX_MAX_PORTS: usize = 32;
pub const SNDRV_EMUX_MAX_VOICES: usize = 64;
pub const SNDRV_EMUX_MAX_MULTI_VOICES: usize = 16;
pub const SNDRV_EMUX_ACCEPT_ROM: libc::c_ulong = 1 << 0;

#[repr(C)]
pub struct snd_emux {
    pub card: *mut snd_card,
    pub max_voices: i32,
    pub mem_size: i32,
    pub num_ports: i32,
    pub ops: snd_emux_operators,
    pub hw: *mut core::ffi::c_void,
    pub flags: libc::c_ulong,
    pub midi_ports: i32,
    pub midi_devidx: i32,
    /* C bit-field linear_panning: 1; represented by its containing unsigned int. */
    pub linear_panning: u32,
    pub hwdep_idx: i32,
    pub hwdep: *mut snd_hwdep,
    pub num_voices: i32,
    pub sflist: *mut snd_sf_list,
    pub voices: *mut snd_emux_voice,
    pub use_time: i32,
    pub voice_lock: spinlock_t,
    pub register_mutex: mutex,
    pub client: i32,
    pub ports: [i32; SNDRV_EMUX_MAX_PORTS],
    pub portptrs: [*mut snd_emux_port; SNDRV_EMUX_MAX_PORTS],
    pub used: i32,
    pub name: *const core::ffi::c_char,
    pub vmidi: *mut *mut snd_rawmidi,
    pub tlist: timer_list,
    pub timer_active: i32,
    pub memhdr: *mut snd_util_memhdr,
    #[cfg(feature = "CONFIG_SND_PROC_FS")]
    pub proc: *mut snd_info_entry,
    #[cfg(feature = "CONFIG_SND_SEQUENCER_OSS")]
    pub oss_synth: *mut snd_seq_device,
}

#[repr(C)]
pub struct snd_emux_port {
    pub emu: *mut snd_emux,
    pub port_mode: core::ffi::c_char,
    pub volume_atten: i32,
    pub drum_flags: libc::c_ulong,
    pub ctrls: [i32; EMUX_MD_END as usize],
    pub effect: *mut snd_emux_effect_table,
    #[cfg(feature = "CONFIG_SND_SEQUENCER_OSS")]
    pub oss_arg: *mut snd_seq_oss_arg,
    pub chset: snd_midi_channel_set,
}

pub const SNDRV_EMUX_PORT_MODE_MIDI: i32 = 0;
pub const SNDRV_EMUX_PORT_MODE_OSS_SYNTH: i32 = 1;
pub const SNDRV_EMUX_PORT_MODE_OSS_MIDI: i32 = 2;

pub const SNDRV_EMUX_ST_OFF: i32 = 0x00;
pub const SNDRV_EMUX_ST_ON: i32 = 0x01;
pub const SNDRV_EMUX_ST_RELEASED: i32 = 0x02 | SNDRV_EMUX_ST_ON;
pub const SNDRV_EMUX_ST_SUSTAINED: i32 = 0x04 | SNDRV_EMUX_ST_ON;
pub const SNDRV_EMUX_ST_STANDBY: i32 = 0x08 | SNDRV_EMUX_ST_ON;
pub const SNDRV_EMUX_ST_PENDING: i32 = 0x10 | SNDRV_EMUX_ST_ON;
pub const SNDRV_EMUX_ST_LOCKED: i32 = 0x100;

#[repr(C)]
pub struct snd_emux_voice {
    pub ch: i32,
    pub state: i32,
    pub time: u32,
    pub note: u8,
    pub key: u8,
    pub velocity: u8,
    pub zone: *mut snd_sf_zone,
    pub block: *mut core::ffi::c_void,
    pub chan: *mut snd_midi_channel,
    pub port: *mut snd_emux_port,
    pub emu: *mut snd_emux,
    pub hw: *mut core::ffi::c_void,
    pub ontime: libc::c_ulong,
    pub reg: soundfont_voice_info,
    pub avol: i32,
    pub acutoff: i32,
    pub apitch: i32,
    pub apan: i32,
    pub aaux: i32,
    pub ptarget: i32,
    pub vtarget: i32,
    pub ftarget: i32,
}

pub const SNDRV_EMUX_UPDATE_VOLUME: i32 = 1 << 0;
pub const SNDRV_EMUX_UPDATE_PITCH: i32 = 1 << 1;
pub const SNDRV_EMUX_UPDATE_PAN: i32 = 1 << 2;
pub const SNDRV_EMUX_UPDATE_FMMOD: i32 = 1 << 3;
pub const SNDRV_EMUX_UPDATE_TREMFREQ: i32 = 1 << 4;
pub const SNDRV_EMUX_UPDATE_FM2FRQ2: i32 = 1 << 5;
pub const SNDRV_EMUX_UPDATE_Q: i32 = 1 << 6;

#[repr(C)]
pub struct snd_emux_effect_table {
    pub val: [i16; EMUX_NUM_EFFECTS as usize],
    pub flag: [u8; EMUX_NUM_EFFECTS as usize],
}

extern "C" {
    pub fn snd_emux_new(remu: *mut *mut snd_emux) -> i32;
    pub fn snd_emux_register(emu: *mut snd_emux, card: *mut snd_card, index: i32, name: *mut core::ffi::c_char) -> i32;
    pub fn snd_emux_free(emu: *mut snd_emux) -> i32;
    pub fn snd_emux_terminate_all(emu: *mut snd_emux);
    pub fn snd_emux_lock_voice(emu: *mut snd_emux, voice: i32);
    pub fn snd_emux_unlock_voice(emu: *mut snd_emux, voice: i32);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
