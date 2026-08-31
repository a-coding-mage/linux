/* SPDX-License-Identifier: GPL-2.0+ WITH Linux-syscall-note */
/* Auto-translated from asound.h: ALSA UAPI header. */
#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, dead_code)]

use core::ffi::{c_char, c_int, c_long, c_longlong, c_uint, c_ulong, c_void};

pub type size_t = usize;
pub type __u8 = u8; pub type __u16 = u16; pub type __u32 = u32; pub type __u64 = u64;
pub type __s32 = i32; pub type __s64 = i64;
pub type __kernel_off_t = isize; pub type __kernel_pid_t = i32; pub type __kernel_long_t = isize;
#[repr(C)] pub struct timespec { pub tv_sec: c_long, pub tv_nsec: c_long }
#[repr(C)] pub struct __time_pad { pub pad: [u8; core::mem::size_of::<c_long>() - core::mem::size_of::<c_int>()] }

pub const fn SNDRV_PROTOCOL_VERSION(major: c_uint, minor: c_uint, subminor: c_uint) -> c_uint { (major << 16) | (minor << 8) | subminor }
pub const fn SNDRV_PROTOCOL_MAJOR(version: c_uint) -> c_uint { (version >> 16) & 0xffff }
pub const fn SNDRV_PROTOCOL_MINOR(version: c_uint) -> c_uint { (version >> 8) & 0xff }
pub const fn SNDRV_PROTOCOL_MICRO(version: c_uint) -> c_uint { version & 0xff }
pub const fn SNDRV_PROTOCOL_INCOMPATIBLE(kversion: c_uint, uversion: c_uint) -> bool { SNDRV_PROTOCOL_MAJOR(kversion) != SNDRV_PROTOCOL_MAJOR(uversion) || (SNDRV_PROTOCOL_MAJOR(kversion) == SNDRV_PROTOCOL_MAJOR(uversion) && SNDRV_PROTOCOL_MINOR(kversion) != SNDRV_PROTOCOL_MINOR(uversion)) }

/****************************************************************************
 *        Digital audio interface
 ****************************************************************************/

pub const AES_IEC958_STATUS_SIZE: usize = 24;

#[repr(C)]
pub struct snd_aes_iec958 {
    pub status: [u8; AES_IEC958_STATUS_SIZE], /* AES/IEC958 channel status bits */
    pub subcode: [u8; 147], /* AES/IEC958 subcode bits */
    pub pad: u8, /* nothing */
    pub dig_subframe: [u8; 4], /* AES/IEC958 subframe bits */
}

#[repr(C)]
pub struct snd_cea_861_aud_if {
    pub db1_ct_cc: u8, /* coding type and channel count */
    pub db2_sf_ss: u8, /* sample frequency and size */
    pub db3: u8, /* not used, all zeros */
    pub db4_ca: u8, /* channel allocation code */
    pub db5_dminh_lsv: u8, /* downmix inhibit & level-shift values */
}

/****************************************************************************
 *      Section for driver hardware dependent interface - /dev/snd/hw?
 ****************************************************************************/

pub const SNDRV_HWDEP_VERSION: c_uint = SNDRV_PROTOCOL_VERSION(1, 0, 1);

pub const SNDRV_HWDEP_IFACE_OPL2: c_int = 0;
pub const SNDRV_HWDEP_IFACE_OPL3: c_int = 1;
pub const SNDRV_HWDEP_IFACE_OPL4: c_int = 2;
pub const SNDRV_HWDEP_IFACE_SB16CSP: c_int = 3; /* Creative Signal Processor */
pub const SNDRV_HWDEP_IFACE_EMU10K1: c_int = 4; /* FX8010 processor in EMU10K1 chip */
pub const SNDRV_HWDEP_IFACE_YSS225: c_int = 5; /* Yamaha FX processor */
pub const SNDRV_HWDEP_IFACE_ICS2115: c_int = 6; /* Wavetable synth */
pub const SNDRV_HWDEP_IFACE_SSCAPE: c_int = 7; /* Ensoniq SoundScape ISA card (MC68EC000) */
pub const SNDRV_HWDEP_IFACE_VX: c_int = 8; /* Digigram VX cards */
pub const SNDRV_HWDEP_IFACE_MIXART: c_int = 9; /* Digigram miXart cards */
pub const SNDRV_HWDEP_IFACE_USX2Y: c_int = 10; /* Tascam US122, US224 & US428 usb */
pub const SNDRV_HWDEP_IFACE_EMUX_WAVETABLE: c_int = 11; /* EmuX wavetable */
pub const SNDRV_HWDEP_IFACE_BLUETOOTH: c_int = 12; /* Bluetooth audio */
pub const SNDRV_HWDEP_IFACE_USX2Y_PCM: c_int = 13; /* Tascam US122, US224 & US428 rawusb pcm */
pub const SNDRV_HWDEP_IFACE_PCXHR: c_int = 14; /* Digigram PCXHR */
pub const SNDRV_HWDEP_IFACE_SB_RC: c_int = 15; /* SB Extigy/Audigy2NX remote control */
pub const SNDRV_HWDEP_IFACE_HDA: c_int = 16; /* HD-audio */
pub const SNDRV_HWDEP_IFACE_USB_STREAM: c_int = 17; /* direct access to usb stream */
pub const SNDRV_HWDEP_IFACE_FW_DICE: c_int = 18; /* TC DICE FireWire device */
pub const SNDRV_HWDEP_IFACE_FW_FIREWORKS: c_int = 19; /* Echo Audio Fireworks based device */
pub const SNDRV_HWDEP_IFACE_FW_BEBOB: c_int = 20; /* BridgeCo BeBoB based device */
pub const SNDRV_HWDEP_IFACE_FW_OXFW: c_int = 21; /* Oxford OXFW970/971 based device */
pub const SNDRV_HWDEP_IFACE_FW_DIGI00X: c_int = 22; /* Digidesign Digi 002/003 family */
pub const SNDRV_HWDEP_IFACE_FW_TASCAM: c_int = 23; /* TASCAM FireWire series */
pub const SNDRV_HWDEP_IFACE_LINE6: c_int = 24; /* Line6 USB processors */
pub const SNDRV_HWDEP_IFACE_FW_MOTU: c_int = 25; /* MOTU FireWire series */
pub const SNDRV_HWDEP_IFACE_FW_FIREFACE: c_int = 26; /* RME Fireface series */
pub const SNDRV_HWDEP_IFACE_LAST: c_int = SNDRV_HWDEP_IFACE_FW_FIREFACE;

#[repr(C)]
pub struct snd_hwdep_info {
    pub device: c_uint, /* WR: device number */
    pub card: c_int, /* R: card number */
    pub id: [u8; 64], /* ID (user selectable) */
    pub name: [u8; 80], /* hwdep name */
    pub iface: c_int, /* hwdep interface */
    pub reserved: [u8; 64], /* reserved for future */
}

#[repr(C)]
pub struct snd_hwdep_dsp_status {
    pub version: c_uint, /* R: driver-specific version */
    pub id: [u8; 32], /* R: driver-specific ID string */
    pub num_dsps: c_uint, /* R: number of DSP images to transfer */
    pub dsp_loaded: c_uint, /* R: bit flags indicating the loaded DSPs */
    pub chip_ready: c_uint, /* R: 1 = initialization finished */
    pub reserved: [u8; 16], /* reserved for future use */
}

#[repr(C)]
pub struct snd_hwdep_dsp_image {
    pub index: c_uint, /* W: DSP index */
    pub name: [u8; 64], /* W: ID (e.g. file name) */
    pub image: *mut u8, /* W: binary image */
    pub length: size_t, /* W: size of image in bytes */
    pub driver_data: c_ulong, /* W: driver-specific data */
}

/*
 * Ioctl macros depend on external _IO/_IOR/_IOW/_IOWR definitions from sys/ioctl.h:
 * SNDRV_HWDEP_IOCTL_PVERSION = _IOR('H', 0x00, int)
 * SNDRV_HWDEP_IOCTL_INFO = _IOR('H', 0x01, struct snd_hwdep_info)
 * SNDRV_HWDEP_IOCTL_DSP_STATUS = _IOR('H', 0x02, struct snd_hwdep_dsp_status)
 * SNDRV_HWDEP_IOCTL_DSP_LOAD = _IOW('H', 0x03, struct snd_hwdep_dsp_image)
 */

/****************************************************************************
 *             Digital Audio (PCM) interface - /dev/snd/pcm??
 ****************************************************************************/

pub const SNDRV_PCM_VERSION: c_uint = SNDRV_PROTOCOL_VERSION(2, 0, 18);
pub type snd_pcm_uframes_t = c_ulong;
pub type snd_pcm_sframes_t = c_long;

pub const SNDRV_PCM_CLASS_GENERIC: c_int = 0;
pub const SNDRV_PCM_CLASS_MULTI: c_int = 1;
pub const SNDRV_PCM_CLASS_MODEM: c_int = 2;
pub const SNDRV_PCM_CLASS_DIGITIZER: c_int = 3;
pub const SNDRV_PCM_CLASS_LAST: c_int = SNDRV_PCM_CLASS_DIGITIZER;
pub const SNDRV_PCM_SUBCLASS_GENERIC_MIX: c_int = 0;
pub const SNDRV_PCM_SUBCLASS_MULTI_MIX: c_int = 1;
pub const SNDRV_PCM_SUBCLASS_LAST: c_int = SNDRV_PCM_SUBCLASS_MULTI_MIX;
pub const SNDRV_PCM_STREAM_PLAYBACK: c_int = 0;
pub const SNDRV_PCM_STREAM_CAPTURE: c_int = 1;
pub const SNDRV_PCM_STREAM_LAST: c_int = SNDRV_PCM_STREAM_CAPTURE;

pub type snd_pcm_access_t = c_int;
pub const SNDRV_PCM_ACCESS_MMAP_INTERLEAVED: snd_pcm_access_t = 0;
pub const SNDRV_PCM_ACCESS_MMAP_NONINTERLEAVED: snd_pcm_access_t = 1;
pub const SNDRV_PCM_ACCESS_MMAP_COMPLEX: snd_pcm_access_t = 2;
pub const SNDRV_PCM_ACCESS_RW_INTERLEAVED: snd_pcm_access_t = 3;
pub const SNDRV_PCM_ACCESS_RW_NONINTERLEAVED: snd_pcm_access_t = 4;
pub const SNDRV_PCM_ACCESS_LAST: snd_pcm_access_t = SNDRV_PCM_ACCESS_RW_NONINTERLEAVED;

pub type snd_pcm_format_t = c_int;
pub const SNDRV_PCM_FORMAT_S8: snd_pcm_format_t = 0;
pub const SNDRV_PCM_FORMAT_U8: snd_pcm_format_t = 1;
pub const SNDRV_PCM_FORMAT_S16_LE: snd_pcm_format_t = 2;
pub const SNDRV_PCM_FORMAT_S16_BE: snd_pcm_format_t = 3;
pub const SNDRV_PCM_FORMAT_U16_LE: snd_pcm_format_t = 4;
pub const SNDRV_PCM_FORMAT_U16_BE: snd_pcm_format_t = 5;
pub const SNDRV_PCM_FORMAT_S24_LE: snd_pcm_format_t = 6; /* low three bytes */
pub const SNDRV_PCM_FORMAT_S24_BE: snd_pcm_format_t = 7; /* low three bytes */
pub const SNDRV_PCM_FORMAT_U24_LE: snd_pcm_format_t = 8; /* low three bytes */
pub const SNDRV_PCM_FORMAT_U24_BE: snd_pcm_format_t = 9; /* low three bytes */
pub const SNDRV_PCM_FORMAT_S32_LE: snd_pcm_format_t = 10;
pub const SNDRV_PCM_FORMAT_S32_BE: snd_pcm_format_t = 11;
pub const SNDRV_PCM_FORMAT_U32_LE: snd_pcm_format_t = 12;
pub const SNDRV_PCM_FORMAT_U32_BE: snd_pcm_format_t = 13;
pub const SNDRV_PCM_FORMAT_FLOAT_LE: snd_pcm_format_t = 14;
pub const SNDRV_PCM_FORMAT_FLOAT_BE: snd_pcm_format_t = 15;
pub const SNDRV_PCM_FORMAT_FLOAT64_LE: snd_pcm_format_t = 16;
pub const SNDRV_PCM_FORMAT_FLOAT64_BE: snd_pcm_format_t = 17;
pub const SNDRV_PCM_FORMAT_IEC958_SUBFRAME_LE: snd_pcm_format_t = 18;
pub const SNDRV_PCM_FORMAT_IEC958_SUBFRAME_BE: snd_pcm_format_t = 19;
pub const SNDRV_PCM_FORMAT_MU_LAW: snd_pcm_format_t = 20;
pub const SNDRV_PCM_FORMAT_A_LAW: snd_pcm_format_t = 21;
pub const SNDRV_PCM_FORMAT_IMA_ADPCM: snd_pcm_format_t = 22;
pub const SNDRV_PCM_FORMAT_MPEG: snd_pcm_format_t = 23;
pub const SNDRV_PCM_FORMAT_GSM: snd_pcm_format_t = 24;
pub const SNDRV_PCM_FORMAT_S20_LE: snd_pcm_format_t = 25;
pub const SNDRV_PCM_FORMAT_S20_BE: snd_pcm_format_t = 26;
pub const SNDRV_PCM_FORMAT_U20_LE: snd_pcm_format_t = 27;
pub const SNDRV_PCM_FORMAT_U20_BE: snd_pcm_format_t = 28;
pub const SNDRV_PCM_FORMAT_SPECIAL: snd_pcm_format_t = 31;
pub const SNDRV_PCM_FORMAT_S24_3LE: snd_pcm_format_t = 32;
pub const SNDRV_PCM_FORMAT_S24_3BE: snd_pcm_format_t = 33;
pub const SNDRV_PCM_FORMAT_U24_3LE: snd_pcm_format_t = 34;
pub const SNDRV_PCM_FORMAT_U24_3BE: snd_pcm_format_t = 35;
pub const SNDRV_PCM_FORMAT_S20_3LE: snd_pcm_format_t = 36;
pub const SNDRV_PCM_FORMAT_S20_3BE: snd_pcm_format_t = 37;
pub const SNDRV_PCM_FORMAT_U20_3LE: snd_pcm_format_t = 38;
pub const SNDRV_PCM_FORMAT_U20_3BE: snd_pcm_format_t = 39;
pub const SNDRV_PCM_FORMAT_S18_3LE: snd_pcm_format_t = 40;
pub const SNDRV_PCM_FORMAT_S18_3BE: snd_pcm_format_t = 41;
pub const SNDRV_PCM_FORMAT_U18_3LE: snd_pcm_format_t = 42;
pub const SNDRV_PCM_FORMAT_U18_3BE: snd_pcm_format_t = 43;
pub const SNDRV_PCM_FORMAT_G723_24: snd_pcm_format_t = 44;
pub const SNDRV_PCM_FORMAT_G723_24_1B: snd_pcm_format_t = 45;
pub const SNDRV_PCM_FORMAT_G723_40: snd_pcm_format_t = 46;
pub const SNDRV_PCM_FORMAT_G723_40_1B: snd_pcm_format_t = 47;
pub const SNDRV_PCM_FORMAT_DSD_U8: snd_pcm_format_t = 48;
pub const SNDRV_PCM_FORMAT_DSD_U16_LE: snd_pcm_format_t = 49;
pub const SNDRV_PCM_FORMAT_DSD_U32_LE: snd_pcm_format_t = 50;
pub const SNDRV_PCM_FORMAT_DSD_U16_BE: snd_pcm_format_t = 51;
pub const SNDRV_PCM_FORMAT_DSD_U32_BE: snd_pcm_format_t = 52;
pub const SNDRV_PCM_FORMAT_LAST: snd_pcm_format_t = SNDRV_PCM_FORMAT_DSD_U32_BE;
pub const SNDRV_PCM_FORMAT_FIRST: snd_pcm_format_t = SNDRV_PCM_FORMAT_S8;
#[cfg(target_endian = "little")] pub const SNDRV_PCM_FORMAT_S16: snd_pcm_format_t = SNDRV_PCM_FORMAT_S16_LE;
#[cfg(target_endian = "little")] pub const SNDRV_PCM_FORMAT_U16: snd_pcm_format_t = SNDRV_PCM_FORMAT_U16_LE;
#[cfg(target_endian = "little")] pub const SNDRV_PCM_FORMAT_S24: snd_pcm_format_t = SNDRV_PCM_FORMAT_S24_LE;
#[cfg(target_endian = "little")] pub const SNDRV_PCM_FORMAT_U24: snd_pcm_format_t = SNDRV_PCM_FORMAT_U24_LE;
#[cfg(target_endian = "little")] pub const SNDRV_PCM_FORMAT_S32: snd_pcm_format_t = SNDRV_PCM_FORMAT_S32_LE;
#[cfg(target_endian = "little")] pub const SNDRV_PCM_FORMAT_U32: snd_pcm_format_t = SNDRV_PCM_FORMAT_U32_LE;
#[cfg(target_endian = "little")] pub const SNDRV_PCM_FORMAT_FLOAT: snd_pcm_format_t = SNDRV_PCM_FORMAT_FLOAT_LE;
#[cfg(target_endian = "little")] pub const SNDRV_PCM_FORMAT_FLOAT64: snd_pcm_format_t = SNDRV_PCM_FORMAT_FLOAT64_LE;
#[cfg(target_endian = "little")] pub const SNDRV_PCM_FORMAT_IEC958_SUBFRAME: snd_pcm_format_t = SNDRV_PCM_FORMAT_IEC958_SUBFRAME_LE;
#[cfg(target_endian = "little")] pub const SNDRV_PCM_FORMAT_S20: snd_pcm_format_t = SNDRV_PCM_FORMAT_S20_LE;
#[cfg(target_endian = "little")] pub const SNDRV_PCM_FORMAT_U20: snd_pcm_format_t = SNDRV_PCM_FORMAT_U20_LE;
#[cfg(target_endian = "big")] pub const SNDRV_PCM_FORMAT_S16: snd_pcm_format_t = SNDRV_PCM_FORMAT_S16_BE;
#[cfg(target_endian = "big")] pub const SNDRV_PCM_FORMAT_U16: snd_pcm_format_t = SNDRV_PCM_FORMAT_U16_BE;
#[cfg(target_endian = "big")] pub const SNDRV_PCM_FORMAT_S24: snd_pcm_format_t = SNDRV_PCM_FORMAT_S24_BE;
#[cfg(target_endian = "big")] pub const SNDRV_PCM_FORMAT_U24: snd_pcm_format_t = SNDRV_PCM_FORMAT_U24_BE;
#[cfg(target_endian = "big")] pub const SNDRV_PCM_FORMAT_S32: snd_pcm_format_t = SNDRV_PCM_FORMAT_S32_BE;
#[cfg(target_endian = "big")] pub const SNDRV_PCM_FORMAT_U32: snd_pcm_format_t = SNDRV_PCM_FORMAT_U32_BE;
#[cfg(target_endian = "big")] pub const SNDRV_PCM_FORMAT_FLOAT: snd_pcm_format_t = SNDRV_PCM_FORMAT_FLOAT_BE;
#[cfg(target_endian = "big")] pub const SNDRV_PCM_FORMAT_FLOAT64: snd_pcm_format_t = SNDRV_PCM_FORMAT_FLOAT64_BE;
#[cfg(target_endian = "big")] pub const SNDRV_PCM_FORMAT_IEC958_SUBFRAME: snd_pcm_format_t = SNDRV_PCM_FORMAT_IEC958_SUBFRAME_BE;
#[cfg(target_endian = "big")] pub const SNDRV_PCM_FORMAT_S20: snd_pcm_format_t = SNDRV_PCM_FORMAT_S20_BE;
#[cfg(target_endian = "big")] pub const SNDRV_PCM_FORMAT_U20: snd_pcm_format_t = SNDRV_PCM_FORMAT_U20_BE;

pub type snd_pcm_subformat_t = c_int;
pub const SNDRV_PCM_SUBFORMAT_STD: snd_pcm_subformat_t = 0;
pub const SNDRV_PCM_SUBFORMAT_MSBITS_MAX: snd_pcm_subformat_t = 1;
pub const SNDRV_PCM_SUBFORMAT_MSBITS_20: snd_pcm_subformat_t = 2;
pub const SNDRV_PCM_SUBFORMAT_MSBITS_24: snd_pcm_subformat_t = 3;
pub const SNDRV_PCM_SUBFORMAT_LAST: snd_pcm_subformat_t = SNDRV_PCM_SUBFORMAT_MSBITS_24;

pub const SNDRV_PCM_INFO_MMAP: c_uint = 0x00000001;
pub const SNDRV_PCM_INFO_MMAP_VALID: c_uint = 0x00000002;
pub const SNDRV_PCM_INFO_DOUBLE: c_uint = 0x00000004;
pub const SNDRV_PCM_INFO_BATCH: c_uint = 0x00000010;
pub const SNDRV_PCM_INFO_SYNC_APPLPTR: c_uint = 0x00000020;
pub const SNDRV_PCM_INFO_PERFECT_DRAIN: c_uint = 0x00000040;
pub const SNDRV_PCM_INFO_INTERLEAVED: c_uint = 0x00000100;
pub const SNDRV_PCM_INFO_NONINTERLEAVED: c_uint = 0x00000200;
pub const SNDRV_PCM_INFO_COMPLEX: c_uint = 0x00000400;
pub const SNDRV_PCM_INFO_BLOCK_TRANSFER: c_uint = 0x00010000;
pub const SNDRV_PCM_INFO_OVERRANGE: c_uint = 0x00020000;
pub const SNDRV_PCM_INFO_RESUME: c_uint = 0x00040000;
pub const SNDRV_PCM_INFO_PAUSE: c_uint = 0x00080000;
pub const SNDRV_PCM_INFO_HALF_DUPLEX: c_uint = 0x00100000;
pub const SNDRV_PCM_INFO_JOINT_DUPLEX: c_uint = 0x00200000;
pub const SNDRV_PCM_INFO_SYNC_START: c_uint = 0x00400000;
pub const SNDRV_PCM_INFO_NO_PERIOD_WAKEUP: c_uint = 0x00800000;
pub const SNDRV_PCM_INFO_HAS_WALL_CLOCK: c_uint = 0x01000000;
pub const SNDRV_PCM_INFO_HAS_LINK_ATIME: c_uint = 0x01000000;
pub const SNDRV_PCM_INFO_HAS_LINK_ABSOLUTE_ATIME: c_uint = 0x02000000;
pub const SNDRV_PCM_INFO_HAS_LINK_ESTIMATED_ATIME: c_uint = 0x04000000;
pub const SNDRV_PCM_INFO_HAS_LINK_SYNCHRONIZED_ATIME: c_uint = 0x08000000;
pub const SNDRV_PCM_INFO_EXPLICIT_SYNC: c_uint = 0x10000000;
pub const SNDRV_PCM_INFO_NO_REWINDS: c_uint = 0x20000000;
pub const SNDRV_PCM_INFO_DRAIN_TRIGGER: c_uint = 0x40000000;
pub const SNDRV_PCM_INFO_FIFO_IN_FRAMES: c_uint = 0x80000000;

/* __SND_STRUCT_TIME64 is selected in C for 32-bit time64 userspace or kernel builds. */
pub type snd_pcm_state_t = c_int;
pub const SNDRV_PCM_STATE_OPEN: snd_pcm_state_t = 0;
pub const SNDRV_PCM_STATE_SETUP: snd_pcm_state_t = 1;
pub const SNDRV_PCM_STATE_PREPARED: snd_pcm_state_t = 2;
pub const SNDRV_PCM_STATE_RUNNING: snd_pcm_state_t = 3;
pub const SNDRV_PCM_STATE_XRUN: snd_pcm_state_t = 4;
pub const SNDRV_PCM_STATE_DRAINING: snd_pcm_state_t = 5;
pub const SNDRV_PCM_STATE_PAUSED: snd_pcm_state_t = 6;
pub const SNDRV_PCM_STATE_SUSPENDED: snd_pcm_state_t = 7;
pub const SNDRV_PCM_STATE_DISCONNECTED: snd_pcm_state_t = 8;
pub const SNDRV_PCM_STATE_LAST: snd_pcm_state_t = SNDRV_PCM_STATE_DISCONNECTED;

pub const SNDRV_PCM_MMAP_OFFSET_DATA: c_uint = 0x00000000;
pub const SNDRV_PCM_MMAP_OFFSET_STATUS_OLD: c_uint = 0x80000000;
pub const SNDRV_PCM_MMAP_OFFSET_CONTROL_OLD: c_uint = 0x81000000;
pub const SNDRV_PCM_MMAP_OFFSET_STATUS_NEW: c_uint = 0x82000000;
pub const SNDRV_PCM_MMAP_OFFSET_CONTROL_NEW: c_uint = 0x83000000;
#[cfg(any(target_pointer_width = "64"))]
pub const SNDRV_PCM_MMAP_OFFSET_STATUS: c_uint = SNDRV_PCM_MMAP_OFFSET_STATUS_OLD;
#[cfg(any(target_pointer_width = "64"))]
pub const SNDRV_PCM_MMAP_OFFSET_CONTROL: c_uint = SNDRV_PCM_MMAP_OFFSET_CONTROL_OLD;

#[repr(C)]
#[deprecated]
pub union snd_pcm_sync_id {
    pub id: [u8; 16],
    pub id16: [u16; 8],
    pub id32: [c_uint; 4],
}

#[repr(C)]
pub struct snd_pcm_info {
    pub device: c_uint, pub subdevice: c_uint, pub stream: c_int, pub card: c_int,
    pub id: [u8; 64], pub name: [u8; 80], pub subname: [u8; 32],
    pub dev_class: c_int, pub dev_subclass: c_int,
    pub subdevices_count: c_uint, pub subdevices_avail: c_uint,
    pub pad1: [u8; 16], pub reserved: [u8; 64],
}

pub type snd_pcm_hw_param_t = c_int;
pub const SNDRV_PCM_HW_PARAM_ACCESS: c_int = 0;
pub const SNDRV_PCM_HW_PARAM_FORMAT: c_int = 1;
pub const SNDRV_PCM_HW_PARAM_SUBFORMAT: c_int = 2;
pub const SNDRV_PCM_HW_PARAM_FIRST_MASK: c_int = SNDRV_PCM_HW_PARAM_ACCESS;
pub const SNDRV_PCM_HW_PARAM_LAST_MASK: c_int = SNDRV_PCM_HW_PARAM_SUBFORMAT;
pub const SNDRV_PCM_HW_PARAM_SAMPLE_BITS: c_int = 8;
pub const SNDRV_PCM_HW_PARAM_FRAME_BITS: c_int = 9;
pub const SNDRV_PCM_HW_PARAM_CHANNELS: c_int = 10;
pub const SNDRV_PCM_HW_PARAM_RATE: c_int = 11;
pub const SNDRV_PCM_HW_PARAM_PERIOD_TIME: c_int = 12;
pub const SNDRV_PCM_HW_PARAM_PERIOD_SIZE: c_int = 13;
pub const SNDRV_PCM_HW_PARAM_PERIOD_BYTES: c_int = 14;
pub const SNDRV_PCM_HW_PARAM_PERIODS: c_int = 15;
pub const SNDRV_PCM_HW_PARAM_BUFFER_TIME: c_int = 16;
pub const SNDRV_PCM_HW_PARAM_BUFFER_SIZE: c_int = 17;
pub const SNDRV_PCM_HW_PARAM_BUFFER_BYTES: c_int = 18;
pub const SNDRV_PCM_HW_PARAM_TICK_TIME: c_int = 19;
pub const SNDRV_PCM_HW_PARAM_FIRST_INTERVAL: c_int = SNDRV_PCM_HW_PARAM_SAMPLE_BITS;
pub const SNDRV_PCM_HW_PARAM_LAST_INTERVAL: c_int = SNDRV_PCM_HW_PARAM_TICK_TIME;
pub const SNDRV_PCM_HW_PARAMS_NORESAMPLE: c_uint = 1 << 0;
pub const SNDRV_PCM_HW_PARAMS_EXPORT_BUFFER: c_uint = 1 << 1;
pub const SNDRV_PCM_HW_PARAMS_NO_PERIOD_WAKEUP: c_uint = 1 << 2;
pub const SNDRV_PCM_HW_PARAMS_NO_DRAIN_SILENCE: c_uint = 1 << 3;

#[repr(C)]
pub struct snd_interval {
    pub min: c_uint,
    pub max: c_uint,
    pub flags: c_uint, /* C bitfields: openmin:1, openmax:1, integer:1, empty:1 */
}

pub const SNDRV_MASK_MAX: usize = 256;
#[repr(C)]
pub struct snd_mask { pub bits: [__u32; (SNDRV_MASK_MAX + 31) / 32] }

#[repr(C)]
pub struct snd_pcm_hw_params {
    pub flags: c_uint,
    pub masks: [snd_mask; (SNDRV_PCM_HW_PARAM_LAST_MASK - SNDRV_PCM_HW_PARAM_FIRST_MASK + 1) as usize],
    pub mres: [snd_mask; 5],
    pub intervals: [snd_interval; (SNDRV_PCM_HW_PARAM_LAST_INTERVAL - SNDRV_PCM_HW_PARAM_FIRST_INTERVAL + 1) as usize],
    pub ires: [snd_interval; 9],
    pub rmask: c_uint, pub cmask: c_uint, pub info: c_uint, pub msbits: c_uint,
    pub rate_num: c_uint, pub rate_den: c_uint,
    pub fifo_size: snd_pcm_uframes_t,
    pub sync: [u8; 16], pub reserved: [u8; 48],
}

pub const SNDRV_PCM_TSTAMP_NONE: c_int = 0;
pub const SNDRV_PCM_TSTAMP_ENABLE: c_int = 1;
pub const SNDRV_PCM_TSTAMP_LAST: c_int = SNDRV_PCM_TSTAMP_ENABLE;

#[repr(C)]
pub struct snd_pcm_sw_params {
    pub tstamp_mode: c_int, pub period_step: c_uint, pub sleep_min: c_uint,
    pub avail_min: snd_pcm_uframes_t, pub xfer_align: snd_pcm_uframes_t,
    pub start_threshold: snd_pcm_uframes_t, pub stop_threshold: snd_pcm_uframes_t,
    pub silence_threshold: snd_pcm_uframes_t, pub silence_size: snd_pcm_uframes_t,
    pub boundary: snd_pcm_uframes_t, pub proto: c_uint, pub tstamp_type: c_uint,
    pub reserved: [u8; 56],
}

#[repr(C)]
pub struct snd_pcm_channel_info {
    pub channel: c_uint,
    pub offset: __kernel_off_t,
    pub first: c_uint,
    pub step: c_uint,
}

pub const SNDRV_PCM_AUDIO_TSTAMP_TYPE_COMPAT: c_int = 0;
pub const SNDRV_PCM_AUDIO_TSTAMP_TYPE_DEFAULT: c_int = 1;
pub const SNDRV_PCM_AUDIO_TSTAMP_TYPE_LINK: c_int = 2;
pub const SNDRV_PCM_AUDIO_TSTAMP_TYPE_LINK_ABSOLUTE: c_int = 3;
pub const SNDRV_PCM_AUDIO_TSTAMP_TYPE_LINK_ESTIMATED: c_int = 4;
pub const SNDRV_PCM_AUDIO_TSTAMP_TYPE_LINK_SYNCHRONIZED: c_int = 5;
pub const SNDRV_PCM_AUDIO_TSTAMP_TYPE_LAST: c_int = SNDRV_PCM_AUDIO_TSTAMP_TYPE_LINK_SYNCHRONIZED;

#[repr(C)]
pub struct snd_pcm_status {
    pub state: snd_pcm_state_t, pub pad1: __time_pad,
    pub trigger_tstamp: timespec, pub tstamp: timespec,
    pub appl_ptr: snd_pcm_uframes_t, pub hw_ptr: snd_pcm_uframes_t,
    pub delay: snd_pcm_sframes_t, pub avail: snd_pcm_uframes_t,
    pub avail_max: snd_pcm_uframes_t, pub overrange: snd_pcm_uframes_t,
    pub suspended_state: snd_pcm_state_t, pub audio_tstamp_data: __u32,
    pub audio_tstamp: timespec, pub driver_tstamp: timespec,
    pub audio_tstamp_accuracy: __u32,
    pub reserved: [u8; 52 - 2 * core::mem::size_of::<timespec>()],
}

#[repr(C)] pub struct __snd_timespec { pub tv_sec: __s32, pub tv_nsec: __s32 }
#[repr(C)] pub struct __snd_timespec64 { pub tv_sec: __s64, pub tv_nsec: __s64 }

#[repr(C)]
pub struct __snd_pcm_mmap_status {
    pub state: snd_pcm_state_t, pub pad1: c_int, pub hw_ptr: snd_pcm_uframes_t,
    pub tstamp: __snd_timespec, pub suspended_state: snd_pcm_state_t,
    pub audio_tstamp: __snd_timespec,
}

#[repr(C)]
pub struct __snd_pcm_mmap_control {
    pub appl_ptr: snd_pcm_uframes_t,
    pub avail_min: snd_pcm_uframes_t,
}

pub const SNDRV_PCM_SYNC_PTR_HWSYNC: c_uint = 1 << 0;
pub const SNDRV_PCM_SYNC_PTR_APPL: c_uint = 1 << 1;
pub const SNDRV_PCM_SYNC_PTR_AVAIL_MIN: c_uint = 1 << 2;

#[repr(C)] pub union __snd_pcm_sync_ptr_s { pub status: core::mem::ManuallyDrop<__snd_pcm_mmap_status>, pub reserved: [u8; 64] }
#[repr(C)] pub union __snd_pcm_sync_ptr_c { pub control: core::mem::ManuallyDrop<__snd_pcm_mmap_control>, pub reserved: [u8; 64] }
#[repr(C)] pub struct __snd_pcm_sync_ptr { pub flags: c_uint, pub s: __snd_pcm_sync_ptr_s, pub c: __snd_pcm_sync_ptr_c }

#[cfg(target_endian = "big")] pub type __pad_before_uframe = [c_char; core::mem::size_of::<__u64>() - core::mem::size_of::<snd_pcm_uframes_t>()];
#[cfg(target_endian = "big")] pub type __pad_after_uframe = [c_char; 0];
#[cfg(target_endian = "little")] pub type __pad_before_uframe = [c_char; 0];
#[cfg(target_endian = "little")] pub type __pad_after_uframe = [c_char; core::mem::size_of::<__u64>() - core::mem::size_of::<snd_pcm_uframes_t>()];

#[repr(C)]
pub struct __snd_pcm_mmap_status64 {
    pub state: snd_pcm_state_t, pub pad1: __u32, pub __pad1: __pad_before_uframe,
    pub hw_ptr: snd_pcm_uframes_t, pub __pad2: __pad_after_uframe,
    pub tstamp: __snd_timespec64, pub suspended_state: snd_pcm_state_t,
    pub pad3: __u32, pub audio_tstamp: __snd_timespec64,
}

#[repr(C)]
pub struct __snd_pcm_mmap_control64 {
    pub __pad1: __pad_before_uframe, pub appl_ptr: snd_pcm_uframes_t,
    pub __pad2: __pad_before_uframe, /* kept for binary compatibility */
    pub __pad3: __pad_before_uframe, pub avail_min: snd_pcm_uframes_t,
    pub __pad4: __pad_after_uframe,
}

#[repr(C)] pub union __snd_pcm_sync_ptr64_s { pub status: core::mem::ManuallyDrop<__snd_pcm_mmap_status64>, pub reserved: [u8; 64] }
#[repr(C)] pub union __snd_pcm_sync_ptr64_c { pub control: core::mem::ManuallyDrop<__snd_pcm_mmap_control64>, pub reserved: [u8; 64] }
#[repr(C)] pub struct __snd_pcm_sync_ptr64 { pub flags: __u32, pub pad1: __u32, pub s: __snd_pcm_sync_ptr64_s, pub c: __snd_pcm_sync_ptr64_c }

pub type snd_pcm_mmap_status = __snd_pcm_mmap_status;
pub type snd_pcm_mmap_control = __snd_pcm_mmap_control;
pub type snd_pcm_sync_ptr = __snd_pcm_sync_ptr;

#[repr(C)] pub struct snd_xferi { pub result: snd_pcm_sframes_t, pub buf: *mut c_void, pub frames: snd_pcm_uframes_t }
#[repr(C)] pub struct snd_xfern { pub result: snd_pcm_sframes_t, pub bufs: *mut *mut c_void, pub frames: snd_pcm_uframes_t }

pub const SNDRV_PCM_TSTAMP_TYPE_GETTIMEOFDAY: c_int = 0;
pub const SNDRV_PCM_TSTAMP_TYPE_MONOTONIC: c_int = 1;
pub const SNDRV_PCM_TSTAMP_TYPE_MONOTONIC_RAW: c_int = 2;
pub const SNDRV_PCM_TSTAMP_TYPE_LAST: c_int = SNDRV_PCM_TSTAMP_TYPE_MONOTONIC_RAW;

pub const SNDRV_CHMAP_UNKNOWN: c_int = 0; pub const SNDRV_CHMAP_NA: c_int = 1; pub const SNDRV_CHMAP_MONO: c_int = 2;
pub const SNDRV_CHMAP_FL: c_int = 3; pub const SNDRV_CHMAP_FR: c_int = 4; pub const SNDRV_CHMAP_RL: c_int = 5; pub const SNDRV_CHMAP_RR: c_int = 6;
pub const SNDRV_CHMAP_FC: c_int = 7; pub const SNDRV_CHMAP_LFE: c_int = 8; pub const SNDRV_CHMAP_SL: c_int = 9; pub const SNDRV_CHMAP_SR: c_int = 10;
pub const SNDRV_CHMAP_RC: c_int = 11; pub const SNDRV_CHMAP_FLC: c_int = 12; pub const SNDRV_CHMAP_FRC: c_int = 13; pub const SNDRV_CHMAP_RLC: c_int = 14;
pub const SNDRV_CHMAP_RRC: c_int = 15; pub const SNDRV_CHMAP_FLW: c_int = 16; pub const SNDRV_CHMAP_FRW: c_int = 17; pub const SNDRV_CHMAP_FLH: c_int = 18;
pub const SNDRV_CHMAP_FCH: c_int = 19; pub const SNDRV_CHMAP_FRH: c_int = 20; pub const SNDRV_CHMAP_TC: c_int = 21; pub const SNDRV_CHMAP_TFL: c_int = 22;
pub const SNDRV_CHMAP_TFR: c_int = 23; pub const SNDRV_CHMAP_TFC: c_int = 24; pub const SNDRV_CHMAP_TRL: c_int = 25; pub const SNDRV_CHMAP_TRR: c_int = 26;
pub const SNDRV_CHMAP_TRC: c_int = 27; pub const SNDRV_CHMAP_TFLC: c_int = 28; pub const SNDRV_CHMAP_TFRC: c_int = 29; pub const SNDRV_CHMAP_TSL: c_int = 30;
pub const SNDRV_CHMAP_TSR: c_int = 31; pub const SNDRV_CHMAP_LLFE: c_int = 32; pub const SNDRV_CHMAP_RLFE: c_int = 33; pub const SNDRV_CHMAP_BC: c_int = 34;
pub const SNDRV_CHMAP_BLC: c_int = 35; pub const SNDRV_CHMAP_BRC: c_int = 36; pub const SNDRV_CHMAP_LAST: c_int = SNDRV_CHMAP_BRC;
pub const SNDRV_CHMAP_POSITION_MASK: c_uint = 0xffff;
pub const SNDRV_CHMAP_PHASE_INVERSE: c_uint = 0x01 << 16;
pub const SNDRV_CHMAP_DRIVER_SPEC: c_uint = 0x02 << 16;

/*
 * PCM ioctl request constants require external ioctl encoding macros:
 * SNDRV_PCM_IOCTL_PVERSION, INFO, TSTAMP, TTSTAMP, USER_PVERSION,
 * HW_REFINE, HW_PARAMS, HW_FREE, SW_PARAMS, STATUS, DELAY, HWSYNC,
 * __SNDRV_PCM_IOCTL_SYNC_PTR, __SNDRV_PCM_IOCTL_SYNC_PTR64,
 * SNDRV_PCM_IOCTL_SYNC_PTR, STATUS_EXT, CHANNEL_INFO, PREPARE, RESET,
 * START, DROP, DRAIN, PAUSE, REWIND, RESUME, XRUN, FORWARD,
 * WRITEI_FRAMES, READI_FRAMES, WRITEN_FRAMES, READN_FRAMES, LINK, UNLINK.
 */

/*****************************************************************************
 *                            MIDI v1.0 interface
 *****************************************************************************/

pub const SNDRV_RAWMIDI_VERSION: c_uint = SNDRV_PROTOCOL_VERSION(2, 0, 5);
pub const SNDRV_RAWMIDI_STREAM_OUTPUT: c_int = 0;
pub const SNDRV_RAWMIDI_STREAM_INPUT: c_int = 1;
pub const SNDRV_RAWMIDI_STREAM_LAST: c_int = SNDRV_RAWMIDI_STREAM_INPUT;

pub const SNDRV_RAWMIDI_INFO_OUTPUT: c_uint = 0x00000001;
pub const SNDRV_RAWMIDI_INFO_INPUT: c_uint = 0x00000002;
pub const SNDRV_RAWMIDI_INFO_DUPLEX: c_uint = 0x00000004;
pub const SNDRV_RAWMIDI_INFO_UMP: c_uint = 0x00000008;
pub const SNDRV_RAWMIDI_INFO_STREAM_INACTIVE: c_uint = 0x00000010;
pub const SNDRV_RAWMIDI_DEVICE_UNKNOWN: c_uint = 0;

#[repr(C)]
pub struct snd_rawmidi_info {
    pub device: c_uint, pub subdevice: c_uint, pub stream: c_int, pub card: c_int,
    pub flags: c_uint, pub id: [u8; 64], pub name: [u8; 80], pub subname: [u8; 32],
    pub subdevices_count: c_uint, pub subdevices_avail: c_uint,
    pub tied_device: c_int, pub reserved: [u8; 60],
}

pub const SNDRV_RAWMIDI_MODE_FRAMING_MASK: c_uint = 7 << 0;
pub const SNDRV_RAWMIDI_MODE_FRAMING_SHIFT: c_uint = 0;
pub const SNDRV_RAWMIDI_MODE_FRAMING_NONE: c_uint = 0 << 0;
pub const SNDRV_RAWMIDI_MODE_FRAMING_TSTAMP: c_uint = 1 << 0;
pub const SNDRV_RAWMIDI_MODE_CLOCK_MASK: c_uint = 7 << 3;
pub const SNDRV_RAWMIDI_MODE_CLOCK_SHIFT: c_uint = 3;
pub const SNDRV_RAWMIDI_MODE_CLOCK_NONE: c_uint = 0 << 3;
pub const SNDRV_RAWMIDI_MODE_CLOCK_REALTIME: c_uint = 1 << 3;
pub const SNDRV_RAWMIDI_MODE_CLOCK_MONOTONIC: c_uint = 2 << 3;
pub const SNDRV_RAWMIDI_MODE_CLOCK_MONOTONIC_RAW: c_uint = 3 << 3;
pub const SNDRV_RAWMIDI_FRAMING_DATA_LENGTH: usize = 16;

#[repr(C, packed)]
pub struct snd_rawmidi_framing_tstamp {
    pub frame_type: __u8,
    pub length: __u8, /* number of valid bytes in data field */
    pub reserved: [__u8; 2],
    pub tv_nsec: __u32,
    pub tv_sec: __u64,
    pub data: [__u8; SNDRV_RAWMIDI_FRAMING_DATA_LENGTH],
}

#[repr(C)]
pub struct snd_rawmidi_params {
    pub stream: c_int,
    pub buffer_size: size_t,
    pub avail_min: size_t,
    pub no_active_sensing: c_uint, /* C bitfield no_active_sensing:1 */
    pub mode: c_uint,
    pub reserved: [u8; 12],
}

#[repr(C)]
pub struct snd_rawmidi_status {
    pub stream: c_int,
    pub pad1: __time_pad,
    pub tstamp: timespec,
    pub avail: size_t,
    pub xruns: size_t,
    pub reserved: [u8; 16],
}

pub const SNDRV_UMP_EP_INFO_STATIC_BLOCKS: c_uint = 0x01;
pub const SNDRV_UMP_EP_INFO_PROTO_MIDI_MASK: c_uint = 0x0300;
pub const SNDRV_UMP_EP_INFO_PROTO_MIDI1: c_uint = 0x0100;
pub const SNDRV_UMP_EP_INFO_PROTO_MIDI2: c_uint = 0x0200;
pub const SNDRV_UMP_EP_INFO_PROTO_JRTS_MASK: c_uint = 0x0003;
pub const SNDRV_UMP_EP_INFO_PROTO_JRTS_TX: c_uint = 0x0001;
pub const SNDRV_UMP_EP_INFO_PROTO_JRTS_RX: c_uint = 0x0002;

#[repr(C, packed)]
pub struct snd_ump_endpoint_info {
    pub card: c_int, pub device: c_int, pub flags: c_uint, pub protocol_caps: c_uint,
    pub protocol: c_uint, pub num_blocks: c_uint, pub version: u16,
    pub family_id: u16, pub model_id: u16, pub manufacturer_id: c_uint,
    pub sw_revision: [u8; 4], pub padding: u16, pub name: [u8; 128],
    pub product_id: [u8; 128], pub reserved: [u8; 32],
}

pub const SNDRV_UMP_DIR_INPUT: c_uint = 0x01;
pub const SNDRV_UMP_DIR_OUTPUT: c_uint = 0x02;
pub const SNDRV_UMP_DIR_BIDIRECTION: c_uint = 0x03;
pub const SNDRV_UMP_BLOCK_IS_MIDI1: c_uint = 1 << 0;
pub const SNDRV_UMP_BLOCK_IS_LOWSPEED: c_uint = 1 << 1;
pub const SNDRV_UMP_BLOCK_UI_HINT_UNKNOWN: c_uint = 0x00;
pub const SNDRV_UMP_BLOCK_UI_HINT_RECEIVER: c_uint = 0x01;
pub const SNDRV_UMP_BLOCK_UI_HINT_SENDER: c_uint = 0x02;
pub const SNDRV_UMP_BLOCK_UI_HINT_BOTH: c_uint = 0x03;
pub const SNDRV_UMP_MAX_GROUPS: usize = 16;
pub const SNDRV_UMP_MAX_BLOCKS: usize = 32;

#[repr(C, packed)]
pub struct snd_ump_block_info {
    pub card: c_int, pub device: c_int, pub block_id: u8, pub direction: u8,
    pub active: u8, pub first_group: u8, pub num_groups: u8,
    pub midi_ci_version: u8, pub sysex8_streams: u8, pub ui_hint: u8,
    pub flags: c_uint, pub name: [u8; 128], pub reserved: [u8; 32],
}

/*
 * RawMIDI and UMP ioctl request constants require external ioctl encoding macros:
 * SNDRV_RAWMIDI_IOCTL_PVERSION, INFO, USER_PVERSION, PARAMS, STATUS, DROP, DRAIN;
 * SNDRV_UMP_IOCTL_ENDPOINT_INFO, SNDRV_UMP_IOCTL_BLOCK_INFO.
 */

/*****************************************************************************
 *  Timer section - /dev/snd/timer
 *****************************************************************************/

pub const SNDRV_TIMER_VERSION: c_uint = SNDRV_PROTOCOL_VERSION(2, 0, 8);
pub const SNDRV_TIMER_CLASS_NONE: c_int = -1;
pub const SNDRV_TIMER_CLASS_SLAVE: c_int = 0;
pub const SNDRV_TIMER_CLASS_GLOBAL: c_int = 1;
pub const SNDRV_TIMER_CLASS_CARD: c_int = 2;
pub const SNDRV_TIMER_CLASS_PCM: c_int = 3;
pub const SNDRV_TIMER_CLASS_LAST: c_int = SNDRV_TIMER_CLASS_PCM;
pub const SNDRV_TIMER_SCLASS_NONE: c_int = 0;
pub const SNDRV_TIMER_SCLASS_APPLICATION: c_int = 1;
pub const SNDRV_TIMER_SCLASS_SEQUENCER: c_int = 2;
pub const SNDRV_TIMER_SCLASS_OSS_SEQUENCER: c_int = 3;
pub const SNDRV_TIMER_SCLASS_LAST: c_int = SNDRV_TIMER_SCLASS_OSS_SEQUENCER;
pub const SNDRV_TIMER_GLOBAL_SYSTEM: c_uint = 0;
pub const SNDRV_TIMER_GLOBAL_RTC: c_uint = 1;
pub const SNDRV_TIMER_GLOBAL_HPET: c_uint = 2;
pub const SNDRV_TIMER_GLOBAL_HRTIMER: c_uint = 3;
pub const SNDRV_TIMER_GLOBAL_UDRIVEN: c_uint = 4;
pub const SNDRV_TIMER_FLG_SLAVE: c_uint = 1 << 0;

#[repr(C)] pub struct snd_timer_id { pub dev_class: c_int, pub dev_sclass: c_int, pub card: c_int, pub device: c_int, pub subdevice: c_int }
#[repr(C)]
pub struct snd_timer_ginfo {
    pub tid: snd_timer_id, pub flags: c_uint, pub card: c_int, pub id: [u8; 64],
    pub name: [u8; 80], pub reserved0: c_ulong, pub resolution: c_ulong,
    pub resolution_min: c_ulong, pub resolution_max: c_ulong, pub clients: c_uint,
    pub reserved: [u8; 32],
}
#[repr(C)] pub struct snd_timer_gparams { pub tid: snd_timer_id, pub period_num: c_ulong, pub period_den: c_ulong, pub reserved: [u8; 32] }
#[repr(C)] pub struct snd_timer_gstatus { pub tid: snd_timer_id, pub resolution: c_ulong, pub resolution_num: c_ulong, pub resolution_den: c_ulong, pub reserved: [u8; 32] }
#[repr(C)] pub struct snd_timer_select { pub id: snd_timer_id, pub reserved: [u8; 32] }
#[repr(C)] pub struct snd_timer_info { pub flags: c_uint, pub card: c_int, pub id: [u8; 64], pub name: [u8; 80], pub reserved0: c_ulong, pub resolution: c_ulong, pub reserved: [u8; 64] }

pub const SNDRV_TIMER_PSFLG_AUTO: c_uint = 1 << 0;
pub const SNDRV_TIMER_PSFLG_EXCLUSIVE: c_uint = 1 << 1;
pub const SNDRV_TIMER_PSFLG_EARLY_EVENT: c_uint = 1 << 2;
#[repr(C)] pub struct snd_timer_params { pub flags: c_uint, pub ticks: c_uint, pub queue_size: c_uint, pub reserved0: c_uint, pub filter: c_uint, pub reserved: [u8; 60] }
#[repr(C)] pub struct snd_timer_status { pub tstamp: timespec, pub resolution: c_uint, pub lost: c_uint, pub overrun: c_uint, pub queue: c_uint, pub reserved: [u8; 64] }
#[repr(C)] pub struct snd_timer_uinfo { pub resolution: __u64, pub fd: c_int, pub id: c_uint, pub reserved: [u8; 16] }
#[repr(C)] pub struct snd_timer_read { pub resolution: c_uint, pub ticks: c_uint }

pub const SNDRV_TIMER_EVENT_RESOLUTION: c_int = 0;
pub const SNDRV_TIMER_EVENT_TICK: c_int = 1;
pub const SNDRV_TIMER_EVENT_START: c_int = 2;
pub const SNDRV_TIMER_EVENT_STOP: c_int = 3;
pub const SNDRV_TIMER_EVENT_CONTINUE: c_int = 4;
pub const SNDRV_TIMER_EVENT_PAUSE: c_int = 5;
pub const SNDRV_TIMER_EVENT_EARLY: c_int = 6;
pub const SNDRV_TIMER_EVENT_SUSPEND: c_int = 7;
pub const SNDRV_TIMER_EVENT_RESUME: c_int = 8;
pub const SNDRV_TIMER_EVENT_MSTART: c_int = SNDRV_TIMER_EVENT_START + 10;
pub const SNDRV_TIMER_EVENT_MSTOP: c_int = SNDRV_TIMER_EVENT_STOP + 10;
pub const SNDRV_TIMER_EVENT_MCONTINUE: c_int = SNDRV_TIMER_EVENT_CONTINUE + 10;
pub const SNDRV_TIMER_EVENT_MPAUSE: c_int = SNDRV_TIMER_EVENT_PAUSE + 10;
pub const SNDRV_TIMER_EVENT_MSUSPEND: c_int = SNDRV_TIMER_EVENT_SUSPEND + 10;
pub const SNDRV_TIMER_EVENT_MRESUME: c_int = SNDRV_TIMER_EVENT_RESUME + 10;
#[repr(C)] pub struct snd_timer_tread { pub event: c_int, pub pad1: __time_pad, pub tstamp: timespec, pub val: c_uint, pub pad2: __time_pad }

/*
 * Timer ioctl request constants require external ioctl encoding macros:
 * SNDRV_TIMER_IOCTL_PVERSION, NEXT_DEVICE, TREAD_OLD, GINFO, GPARAMS, GSTATUS,
 * SELECT, INFO, PARAMS, STATUS, START, STOP, CONTINUE, PAUSE, TREAD64, CREATE,
 * TRIGGER, and conditional SNDRV_TIMER_IOCTL_TREAD.
 */

/****************************************************************************
 *        Section for driver control interface - /dev/snd/control?
 ****************************************************************************/

pub const SNDRV_CTL_VERSION: c_uint = SNDRV_PROTOCOL_VERSION(2, 0, 9);
#[repr(C)]
pub struct snd_ctl_card_info {
    pub card: c_int, pub pad: c_int, pub id: [u8; 16], pub driver: [u8; 16],
    pub name: [u8; 32], pub longname: [u8; 80], pub reserved_: [u8; 16],
    pub mixername: [u8; 80], pub components: [u8; 128],
}

pub type snd_ctl_elem_type_t = c_int;
pub const SNDRV_CTL_ELEM_TYPE_NONE: snd_ctl_elem_type_t = 0;
pub const SNDRV_CTL_ELEM_TYPE_BOOLEAN: snd_ctl_elem_type_t = 1;
pub const SNDRV_CTL_ELEM_TYPE_INTEGER: snd_ctl_elem_type_t = 2;
pub const SNDRV_CTL_ELEM_TYPE_ENUMERATED: snd_ctl_elem_type_t = 3;
pub const SNDRV_CTL_ELEM_TYPE_BYTES: snd_ctl_elem_type_t = 4;
pub const SNDRV_CTL_ELEM_TYPE_IEC958: snd_ctl_elem_type_t = 5;
pub const SNDRV_CTL_ELEM_TYPE_INTEGER64: snd_ctl_elem_type_t = 6;
pub const SNDRV_CTL_ELEM_TYPE_LAST: snd_ctl_elem_type_t = SNDRV_CTL_ELEM_TYPE_INTEGER64;

pub type snd_ctl_elem_iface_t = c_int;
pub const SNDRV_CTL_ELEM_IFACE_CARD: snd_ctl_elem_iface_t = 0;
pub const SNDRV_CTL_ELEM_IFACE_HWDEP: snd_ctl_elem_iface_t = 1;
pub const SNDRV_CTL_ELEM_IFACE_MIXER: snd_ctl_elem_iface_t = 2;
pub const SNDRV_CTL_ELEM_IFACE_PCM: snd_ctl_elem_iface_t = 3;
pub const SNDRV_CTL_ELEM_IFACE_RAWMIDI: snd_ctl_elem_iface_t = 4;
pub const SNDRV_CTL_ELEM_IFACE_TIMER: snd_ctl_elem_iface_t = 5;
pub const SNDRV_CTL_ELEM_IFACE_SEQUENCER: snd_ctl_elem_iface_t = 6;
pub const SNDRV_CTL_ELEM_IFACE_LAST: snd_ctl_elem_iface_t = SNDRV_CTL_ELEM_IFACE_SEQUENCER;

pub const SNDRV_CTL_ELEM_ACCESS_READ: c_uint = 1 << 0;
pub const SNDRV_CTL_ELEM_ACCESS_WRITE: c_uint = 1 << 1;
pub const SNDRV_CTL_ELEM_ACCESS_READWRITE: c_uint = SNDRV_CTL_ELEM_ACCESS_READ | SNDRV_CTL_ELEM_ACCESS_WRITE;
pub const SNDRV_CTL_ELEM_ACCESS_VOLATILE: c_uint = 1 << 2;
pub const SNDRV_CTL_ELEM_ACCESS_TLV_READ: c_uint = 1 << 4;
pub const SNDRV_CTL_ELEM_ACCESS_TLV_WRITE: c_uint = 1 << 5;
pub const SNDRV_CTL_ELEM_ACCESS_TLV_READWRITE: c_uint = SNDRV_CTL_ELEM_ACCESS_TLV_READ | SNDRV_CTL_ELEM_ACCESS_TLV_WRITE;
pub const SNDRV_CTL_ELEM_ACCESS_TLV_COMMAND: c_uint = 1 << 6;
pub const SNDRV_CTL_ELEM_ACCESS_INACTIVE: c_uint = 1 << 8;
pub const SNDRV_CTL_ELEM_ACCESS_LOCK: c_uint = 1 << 9;
pub const SNDRV_CTL_ELEM_ACCESS_OWNER: c_uint = 1 << 10;
pub const SNDRV_CTL_ELEM_ACCESS_TLV_CALLBACK: c_uint = 1 << 28;
pub const SNDRV_CTL_ELEM_ACCESS_USER: c_uint = 1 << 29;

pub const SNDRV_CTL_POWER_D0: c_uint = 0x0000;
pub const SNDRV_CTL_POWER_D1: c_uint = 0x0100;
pub const SNDRV_CTL_POWER_D2: c_uint = 0x0200;
pub const SNDRV_CTL_POWER_D3: c_uint = 0x0300;
pub const SNDRV_CTL_POWER_D3hot: c_uint = SNDRV_CTL_POWER_D3 | 0x0000;
pub const SNDRV_CTL_POWER_D3cold: c_uint = SNDRV_CTL_POWER_D3 | 0x0001;
pub const SNDRV_CTL_ELEM_ID_NAME_MAXLEN: usize = 44;

#[repr(C)]
pub struct snd_ctl_elem_id {
    pub numid: c_uint, pub iface: snd_ctl_elem_iface_t, pub device: c_uint,
    pub subdevice: c_uint, pub name: [u8; SNDRV_CTL_ELEM_ID_NAME_MAXLEN],
    pub index: c_uint,
}
#[repr(C)] pub struct snd_ctl_elem_list { pub offset: c_uint, pub space: c_uint, pub used: c_uint, pub count: c_uint, pub pids: *mut snd_ctl_elem_id, pub reserved: [u8; 50] }

#[repr(C)] pub struct snd_ctl_elem_info_integer { pub min: c_long, pub max: c_long, pub step: c_long }
#[repr(C)] pub struct snd_ctl_elem_info_integer64 { pub min: c_longlong, pub max: c_longlong, pub step: c_longlong }
#[repr(C)] pub struct snd_ctl_elem_info_enumerated { pub items: c_uint, pub item: c_uint, pub name: [c_char; 64], pub names_ptr: __u64, pub names_length: c_uint }
#[repr(C)] pub union snd_ctl_elem_info_value {
    pub integer: core::mem::ManuallyDrop<snd_ctl_elem_info_integer>,
    pub integer64: core::mem::ManuallyDrop<snd_ctl_elem_info_integer64>,
    pub enumerated: core::mem::ManuallyDrop<snd_ctl_elem_info_enumerated>,
    pub reserved: [u8; 128],
}
#[repr(C)]
pub struct snd_ctl_elem_info {
    pub id: snd_ctl_elem_id, pub type_: snd_ctl_elem_type_t, pub access: c_uint,
    pub count: c_uint, pub owner: __kernel_pid_t, pub value: snd_ctl_elem_info_value,
    pub reserved: [u8; 64],
}

#[repr(C)] pub union snd_ctl_elem_value_integer { pub value: [c_long; 128], pub value_ptr: *mut c_long }
#[repr(C)] pub union snd_ctl_elem_value_integer64 { pub value: [c_longlong; 64], pub value_ptr: *mut c_longlong }
#[repr(C)] pub union snd_ctl_elem_value_enumerated { pub item: [c_uint; 128], pub item_ptr: *mut c_uint }
#[repr(C)] pub union snd_ctl_elem_value_bytes { pub data: [u8; 512], pub data_ptr: *mut u8 }
#[repr(C)] pub union snd_ctl_elem_value_value {
    pub integer: core::mem::ManuallyDrop<snd_ctl_elem_value_integer>,
    pub integer64: core::mem::ManuallyDrop<snd_ctl_elem_value_integer64>,
    pub enumerated: core::mem::ManuallyDrop<snd_ctl_elem_value_enumerated>,
    pub bytes: core::mem::ManuallyDrop<snd_ctl_elem_value_bytes>,
    pub iec958: core::mem::ManuallyDrop<snd_aes_iec958>,
}
#[repr(C)]
pub struct snd_ctl_elem_value {
    pub id: snd_ctl_elem_id,
    pub indirect: c_uint, /* C bitfield indirect:1, obsoleted */
    pub value: snd_ctl_elem_value_value,
    pub reserved: [u8; 128],
}

#[repr(C)]
pub struct snd_ctl_tlv {
    pub numid: c_uint,
    pub length: c_uint,
    pub tlv: [c_uint; 0], /* flexible array member: first TLV */
}

/*
 * Control ioctl request constants require external ioctl encoding macros:
 * SNDRV_CTL_IOCTL_PVERSION, CARD_INFO, ELEM_LIST, ELEM_INFO, ELEM_READ,
 * ELEM_WRITE, ELEM_LOCK, ELEM_UNLOCK, SUBSCRIBE_EVENTS, ELEM_ADD, ELEM_REPLACE,
 * ELEM_REMOVE, TLV_READ, TLV_WRITE, TLV_COMMAND, HWDEP_NEXT_DEVICE, HWDEP_INFO,
 * PCM_NEXT_DEVICE, PCM_INFO, PCM_PREFER_SUBDEVICE, RAWMIDI_NEXT_DEVICE,
 * RAWMIDI_INFO, RAWMIDI_PREFER_SUBDEVICE, UMP_NEXT_DEVICE, UMP_ENDPOINT_INFO,
 * UMP_BLOCK_INFO, POWER, POWER_STATE.
 */

pub const SNDRV_CTL_EVENT_ELEM: c_int = 0;
pub const SNDRV_CTL_EVENT_LAST: c_int = SNDRV_CTL_EVENT_ELEM;
pub const SNDRV_CTL_EVENT_MASK_VALUE: c_uint = 1 << 0;
pub const SNDRV_CTL_EVENT_MASK_INFO: c_uint = 1 << 1;
pub const SNDRV_CTL_EVENT_MASK_ADD: c_uint = 1 << 2;
pub const SNDRV_CTL_EVENT_MASK_TLV: c_uint = 1 << 3;
pub const SNDRV_CTL_EVENT_MASK_REMOVE: c_uint = !0u32;

#[repr(C)] pub struct snd_ctl_event_elem { pub mask: c_uint, pub id: snd_ctl_elem_id }
#[repr(C)] pub union snd_ctl_event_data { pub elem: core::mem::ManuallyDrop<snd_ctl_event_elem>, pub data8: [u8; 60] }
#[repr(C)] pub struct snd_ctl_event { pub type_: c_int, pub data: snd_ctl_event_data }

pub const SNDRV_CTL_NAME_NONE: &str = "";
pub const SNDRV_CTL_NAME_PLAYBACK: &str = "Playback ";
pub const SNDRV_CTL_NAME_CAPTURE: &str = "Capture ";
pub const SNDRV_CTL_NAME_IEC958_NONE: &str = "";
pub const SNDRV_CTL_NAME_IEC958_SWITCH: &str = "Switch";
pub const SNDRV_CTL_NAME_IEC958_VOLUME: &str = "Volume";
pub const SNDRV_CTL_NAME_IEC958_DEFAULT: &str = "Default";
pub const SNDRV_CTL_NAME_IEC958_MASK: &str = "Mask";
pub const SNDRV_CTL_NAME_IEC958_CON_MASK: &str = "Con Mask";
pub const SNDRV_CTL_NAME_IEC958_PRO_MASK: &str = "Pro Mask";
pub const SNDRV_CTL_NAME_IEC958_PCM_STREAM: &str = "PCM Stream";
/* SNDRV_CTL_NAME_IEC958(expl,direction,what) concatenates string literals in C. */
