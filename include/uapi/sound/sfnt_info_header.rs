/* SPDX-License-Identifier: GPL-2.0+ WITH Linux-syscall-note */
/* Translated from sound/sfnt_info.h; definitions from sound/asound.h are external dependencies. */

#[cfg(not(target_endian = "big"))]
pub const fn SNDRV_OSS_PATCHKEY(id: u16) -> u16 { (id << 8) | 0xfd }
#[cfg(target_endian = "big")]
pub const fn SNDRV_OSS_PATCHKEY(id: u16) -> u16 { 0xfd00 | id }

#[repr(C)]
pub struct soundfont_patch_info {
    pub key: u16,
    pub device_no: i16,
    pub sf_id: u16,
    pub optarg: i16,
    pub len: i32,
    pub r#type: i16,
    pub reserved: i16,
}
pub const SNDRV_OSS_SOUNDFONT_PATCH: u16 = SNDRV_OSS_PATCHKEY(0x07);
pub const SNDRV_SFNT_LOAD_INFO: i16 = 0;
pub const SNDRV_SFNT_LOAD_DATA: i16 = 1;
pub const SNDRV_SFNT_OPEN_PATCH: i16 = 2;
pub const SNDRV_SFNT_CLOSE_PATCH: i16 = 3;
pub const SNDRV_SFNT_REPLACE_DATA: i16 = 5;
pub const SNDRV_SFNT_MAP_PRESET: i16 = 6;
pub const SNDRV_SFNT_PROBE_DATA: i16 = 8;
pub const SNDRV_SFNT_REMOVE_INFO: i16 = 9;

pub const SNDRV_SFNT_PATCH_NAME_LEN: usize = 32;
#[repr(C)]
pub struct soundfont_open_parm {
    pub r#type: u16,
    pub reserved: i16,
    pub name: [core::ffi::c_char; SNDRV_SFNT_PATCH_NAME_LEN],
}
pub const SNDRV_SFNT_PAT_TYPE_MISC: u16 = 0;
pub const SNDRV_SFNT_PAT_TYPE_GUS: u16 = 6;
pub const SNDRV_SFNT_PAT_TYPE_MAP: u16 = 7;
pub const SNDRV_SFNT_PAT_LOCKED: u16 = 0x100;
pub const SNDRV_SFNT_PAT_SHARED: u16 = 0x200;

#[repr(C)]
pub struct soundfont_voice_parm {
    pub moddelay: u16, pub modatkhld: u16, pub moddcysus: u16, pub modrelease: u16,
    pub modkeyhold: i16, pub modkeydecay: i16,
    pub voldelay: u16, pub volatkhld: u16, pub voldcysus: u16, pub volrelease: u16,
    pub volkeyhold: i16, pub volkeydecay: i16,
    pub lfo1delay: u16, pub lfo2delay: u16, pub pefe: u16, pub fmmod: u16,
    pub tremfrq: u16, pub fm2frq2: u16,
    pub cutoff: u8, pub filterQ: u8, pub chorus: u8, pub reverb: u8,
    pub reserved: [u16; 4],
}

#[repr(C)]
pub struct soundfont_voice_info {
    pub sf_id: u16, pub sample: u16, pub start: i32, pub end: i32,
    pub loopstart: i32, pub loopend: i32, pub rate_offset: i16, pub mode: u16,
    pub root: i16, pub tune: i16, pub low: u8, pub high: u8, pub vellow: u8, pub velhigh: u8,
    pub fixkey: i8, pub fixvel: i8, pub pan: i8, pub fixpan: i8, pub exclusiveClass: i16,
    pub amplitude: u8, pub attenuation: u8, pub scaleTuning: i16,
    pub parm: soundfont_voice_parm, pub sample_mode: u16,
}
pub const SNDRV_SFNT_MODE_ROMSOUND: u16 = 0x8000;
pub const SNDRV_SFNT_MODE_STEREO: u16 = 1;
pub const SNDRV_SFNT_MODE_LOOPING: u16 = 2;
pub const SNDRV_SFNT_MODE_NORELEASE: u16 = 4;
pub const SNDRV_SFNT_MODE_INIT_PARM: u16 = 8;

#[repr(C)]
pub struct soundfont_voice_rec_hdr { pub bank: u8, pub instr: u8, pub nvoices: i8, pub write_mode: i8 }
pub const SNDRV_SFNT_WR_APPEND: i8 = 0;
pub const SNDRV_SFNT_WR_EXCLUSIVE: i8 = 1;
pub const SNDRV_SFNT_WR_REPLACE: i8 = 2;

#[repr(C)]
pub struct soundfont_sample_info {
    pub sf_id: u16, pub sample: u16, pub start: i32, pub end: i32,
    pub loopstart: i32, pub loopend: i32, pub size: i32, pub dummy: i16,
    pub mode_flags: u16, pub truesize: u32,
}
pub const SNDRV_SFNT_SAMPLE_8BITS: u16 = 1;
pub const SNDRV_SFNT_SAMPLE_UNSIGNED: u16 = 2;
pub const SNDRV_SFNT_SAMPLE_NO_BLANK: u16 = 4;
pub const SNDRV_SFNT_SAMPLE_SINGLESHOT: u16 = 8;
pub const SNDRV_SFNT_SAMPLE_BIDIR_LOOP: u16 = 16;
pub const SNDRV_SFNT_SAMPLE_STEREO_LEFT: u16 = 32;
pub const SNDRV_SFNT_SAMPLE_STEREO_RIGHT: u16 = 64;
pub const SNDRV_SFNT_SAMPLE_REVERSE_LOOP: u16 = 128;

#[repr(C)]
pub struct soundfont_voice_map { pub map_bank: i32, pub map_instr: i32, pub map_key: i32, pub src_bank: i32, pub src_instr: i32, pub src_key: i32 }

pub const SNDRV_EMUX_HWDEP_NAME: &str = "Emux WaveTable";
pub const SNDRV_EMUX_VERSION: u32 = (1 << 16) | (0 << 8) | 0;
#[repr(C)]
pub struct snd_emux_misc_mode { pub port: i32, pub mode: i32, pub value: i32, pub value2: i32 }

/* The following ioctl values use _IOR/_IOWR/_IO/_IOW from the external ioctl ABI. */
// SNDRV_EMUX_IOCTL_VERSION       = _IOR('H', 0x80, unsigned int)
// SNDRV_EMUX_IOCTL_LOAD_PATCH    = _IOWR('H', 0x81, struct soundfont_patch_info)
// SNDRV_EMUX_IOCTL_RESET_SAMPLES  = _IO('H', 0x82)
// SNDRV_EMUX_IOCTL_REMOVE_LAST_SAMPLES = _IO('H', 0x83)
// SNDRV_EMUX_IOCTL_MEM_AVAIL      = _IOW('H', 0x84, int)
// SNDRV_EMUX_IOCTL_MISC_MODE     = _IOWR('H', 0x84, struct snd_emux_misc_mode)

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
