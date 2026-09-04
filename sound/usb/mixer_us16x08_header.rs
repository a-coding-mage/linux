// SPDX-License-Identifier: GPL-2.0

pub const SND_US16X08_MAX_CHANNELS: usize = 16;

// define some bias, cause some alsa-mixers wont work with
// negative ranges or if mixer-min != 0
pub const SND_US16X08_NO_BIAS: u32 = 0;
pub const SND_US16X08_FADER_BIAS: u32 = 127;
pub const SND_US16X08_EQ_HIGHFREQ_BIAS: u32 = 0x20;
pub const SND_US16X08_COMP_THRESHOLD_BIAS: u32 = 0x20;
pub const SND_US16X08_COMP_ATTACK_BIAS: u32 = 2;
pub const SND_US16X08_COMP_RELEASE_BIAS: u32 = 1;

// get macro for components of kcontrol private_value
#[inline]
pub fn SND_US16X08_KCBIAS(x: u32) -> u32 {
    (x >> 24) & 0xff
}

#[inline]
pub fn SND_US16X08_KCSTEP(x: u32) -> u32 {
    (x >> 16) & 0xff
}

#[inline]
pub fn SND_US16X08_KCMIN(x: u32) -> u32 {
    (x >> 8) & 0xff
}

#[inline]
pub fn SND_US16X08_KCMAX(x: u32) -> u32 {
    (x >> 0) & 0xff
}

// set macro for kcontrol private_value
#[inline]
pub fn SND_US16X08_KCSET(bias: u32, step: u32, min: u32, max: u32) -> u32 {
    ((bias) << 24) | ((step) << 16) | ((min) << 8) | (max)
}

// the URB request/type to control Tascam mixers
pub const SND_US16X08_URB_REQUEST: u32 = 0x1D;
pub const SND_US16X08_URB_REQUESTTYPE: u32 = 0x40;

// the URB params to retrieve meter ranges
pub const SND_US16X08_URB_METER_REQUEST: u32 = 0x1e;
pub const SND_US16X08_URB_METER_REQUESTTYPE: u32 = 0xc0;

#[inline]
pub fn MUA0(x: &[u8], y: usize) -> u8 {
    x[y * 10 + 4]
}

#[inline]
pub fn MUA1(x: &[u8], y: usize) -> u8 {
    x[y * 10 + 5]
}

#[inline]
pub fn MUA2(x: &[u8], y: usize) -> u8 {
    x[y * 10 + 6]
}

#[inline]
pub fn MUB0(x: &[u8], y: usize) -> u8 {
    x[y * 10 + 7]
}

#[inline]
pub fn MUB1(x: &[u8], y: usize) -> u8 {
    x[y * 10 + 8]
}

#[inline]
pub fn MUB2(x: &[u8], y: usize) -> u8 {
    x[y * 10 + 9]
}

#[inline]
pub fn MUC0(x: &[u8], y: usize) -> u8 {
    x[y * 10 + 10]
}

#[inline]
pub fn MUC1(x: &[u8], y: usize) -> u8 {
    x[y * 10 + 11]
}

#[inline]
pub fn MUC2(x: &[u8], y: usize) -> u8 {
    x[y * 10 + 12]
}

#[inline]
pub fn MUC3(x: &[u8], y: usize) -> u8 {
    x[y * 10 + 13]
}

// Common Channel control IDs
pub const SND_US16X08_ID_BYPASS: u32 = 0x45;
pub const SND_US16X08_ID_BUSS_OUT: u32 = 0x44;
pub const SND_US16X08_ID_PHASE: u32 = 0x85;
pub const SND_US16X08_ID_MUTE: u32 = 0x83;
pub const SND_US16X08_ID_FADER: u32 = 0x81;
pub const SND_US16X08_ID_PAN: u32 = 0x82;
pub const SND_US16X08_ID_METER: u32 = 0xB1;

pub const SND_US16X08_ID_EQ_BAND_COUNT: usize = 4;
pub const SND_US16X08_ID_EQ_PARAM_COUNT: usize = 4;

// EQ level IDs
pub const SND_US16X08_ID_EQLOWLEVEL: u32 = 0x01;
pub const SND_US16X08_ID_EQLOWMIDLEVEL: u32 = 0x02;
pub const SND_US16X08_ID_EQHIGHMIDLEVEL: u32 = 0x03;
pub const SND_US16X08_ID_EQHIGHLEVEL: u32 = 0x04;

// EQ frequence IDs
pub const SND_US16X08_ID_EQLOWFREQ: u32 = 0x11;
pub const SND_US16X08_ID_EQLOWMIDFREQ: u32 = 0x12;
pub const SND_US16X08_ID_EQHIGHMIDFREQ: u32 = 0x13;
pub const SND_US16X08_ID_EQHIGHFREQ: u32 = 0x14;

// EQ width IDs
pub const SND_US16X08_ID_EQLOWMIDWIDTH: u32 = 0x22;
pub const SND_US16X08_ID_EQHIGHMIDWIDTH: u32 = 0x23;

pub const SND_US16X08_ID_EQENABLE: u32 = 0x30;

#[inline]
pub fn EQ_STORE_BAND_IDX(x: u32) -> u32 {
    (x) & 0xf
}

#[inline]
pub fn EQ_STORE_PARAM_IDX(x: u32) -> u32 {
    ((x) & 0xf0) >> 4
}

pub const SND_US16X08_ID_ROUTE: u32 = 0x00;

// Compressor Ids
pub const SND_US16X08_ID_COMP_BASE: u32 = 0x32;
pub const SND_US16X08_ID_COMP_THRESHOLD: u32 = SND_US16X08_ID_COMP_BASE;
pub const SND_US16X08_ID_COMP_RATIO: u32 = SND_US16X08_ID_COMP_BASE + 1;
pub const SND_US16X08_ID_COMP_ATTACK: u32 = SND_US16X08_ID_COMP_BASE + 2;
pub const SND_US16X08_ID_COMP_RELEASE: u32 = SND_US16X08_ID_COMP_BASE + 3;
pub const SND_US16X08_ID_COMP_GAIN: u32 = SND_US16X08_ID_COMP_BASE + 4;
pub const SND_US16X08_ID_COMP_SWITCH: u32 = SND_US16X08_ID_COMP_BASE + 5;
pub const SND_US16X08_ID_COMP_COUNT: usize = 6;

#[inline]
pub fn COMP_STORE_IDX(x: u32) -> u32 {
    (x) - SND_US16X08_ID_COMP_BASE
}

#[repr(C)]
pub struct snd_us16x08_eq_store {
    pub val: [[[u8; SND_US16X08_MAX_CHANNELS]; SND_US16X08_ID_EQ_PARAM_COUNT]; SND_US16X08_ID_EQ_BAND_COUNT],
}

#[repr(C)]
pub struct snd_us16x08_comp_store {
    pub val: [[u8; SND_US16X08_MAX_CHANNELS]; SND_US16X08_ID_COMP_COUNT],
}

#[repr(C)]
pub struct snd_us16x08_meter_store {
    pub meter_level: [i32; SND_US16X08_MAX_CHANNELS],
    pub master_level: [i32; 2],
    pub comp_index: i32,
    pub comp_active_index: i32,
    pub comp_level: [i32; 16],
    pub comp_store: *mut snd_us16x08_comp_store,
}

#[repr(C)]
pub struct snd_us16x08_control_params {
    pub kcontrol_new: *const snd_kcontrol_new,
    pub control_id: i32,
    pub r#type: i32,
    pub num_channels: i32,
    pub name: *const i8,
    pub default_val: i32,
}

pub use snd_ctl_boolean_mono_info as snd_us16x08_switch_info;

extern "C" {
    pub fn snd_us16x08_controls_create(mixer: *mut usb_mixer_interface) -> i32;
}

#[repr(C)]
pub struct snd_kcontrol_new;

#[repr(C)]
pub struct usb_mixer_interface;

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
