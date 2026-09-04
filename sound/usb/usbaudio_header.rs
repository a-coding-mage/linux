// SPDX-License-Identifier: GPL-2.0-or-later
// (Tentative) USB Audio Driver for ALSA
// Copyright (c) 2002 by Takashi Iwai <tiwai@suse.de>

// Requires: #include <sound/core.h>

use core::ffi::c_void;

// handling of USB vendor/product ID pairs as 32-bit numbers
#[inline]
pub const fn usb_id(vendor: u32, product: u32) -> u32 {
    ((vendor << 16) | product)
}

#[inline]
pub const fn usb_id_vendor(id: u32) -> u32 {
    id >> 16
}

#[inline]
pub const fn usb_id_product(id: u32) -> u16 {
    id as u16
}

pub struct media_device {
    _private: c_void,
}
pub struct media_intf_devnode {
    _private: c_void,
}

pub const MAX_CARD_INTERFACES: usize = 16;

// Structure holding association between Audio Control Interface
// and given Streaming or Midi Interface.
#[repr(C)]
pub struct snd_intf_to_ctrl {
    pub interface: u8,
    pub ctrl_intf: *mut usb_host_interface,
}

pub struct usb_host_interface {
    _private: c_void,
}

pub struct usb_device {
    _private: c_void,
}

pub struct snd_card {
    _private: c_void,
}

pub struct usb_interface {
    _private: c_void,
}

pub struct mutex {
    _private: c_void,
}

pub struct atomic_t {
    _private: c_void,
}

pub struct snd_refcount {
    _private: c_void,
}

pub struct list_head {
    _private: c_void,
}

#[repr(C)]
pub struct snd_usb_audio {
    pub index: i32,
    pub dev: *mut usb_device,
    pub card: *mut snd_card,
    pub intf: [*mut usb_interface; MAX_CARD_INTERFACES],
    pub usb_id: u32,
    pub quirk_type: u16,
    pub mutex: mutex,
    pub system_suspend: u32,
    pub active: atomic_t,
    pub shutdown: atomic_t,
    pub usage_count: snd_refcount,
    pub quirk_flags: u64,
    pub need_delayed_register: u32,
    pub num_interfaces: i32,
    pub last_iface: i32,
    pub num_suspended_intf: i32,
    pub sample_rate_read_error: i32,
    pub badd_profile: i32,
    pub pcm_list: list_head,
    pub ep_list: list_head,
    pub iface_ref_list: list_head,
    pub clock_ref_list: list_head,
    pub pcm_devs: i32,
    pub num_rawmidis: u32,
    pub midi_list: list_head,
    pub midi_v2_list: list_head,
    pub mixer_list: list_head,
    pub setup: i32,
    pub generic_implicit_fb: bool,
    pub autoclock: bool,
    pub lowlatency: bool,
    pub ctrl_intf: *mut usb_host_interface,
    pub media_dev: *mut media_device,
    pub ctl_intf_media_devnode: *mut media_intf_devnode,
    pub num_intf_to_ctrl: u32,
    pub intf_to_ctrl: [snd_intf_to_ctrl; MAX_CARD_INTERFACES],
}

pub const USB_AUDIO_IFACE_UNUSED: *mut u8 = (-1isize) as *mut u8;

// Logging macros (usb_audio_err, usb_audio_warn, etc.) are defined in C as:
// usb_audio_err(chip, fmt, args...) calls dev_err(&(chip)->dev->dev, fmt, ##args)
// These require variadic function support and device logging infrastructure.
// Equivalent Rust logging would use the log crate or kernel logging interfaces.

// special values for .ifnum
pub const QUIRK_NODEV_INTERFACE: i16 = -3;
pub const QUIRK_NO_INTERFACE: i16 = -2;
pub const QUIRK_ANY_INTERFACE: i16 = -1;

#[repr(u32)]
pub enum quirk_type {
    QUIRK_IGNORE_INTERFACE = 0,
    QUIRK_COMPOSITE = 1,
    QUIRK_AUTODETECT = 2,
    QUIRK_MIDI_STANDARD_INTERFACE = 3,
    QUIRK_MIDI_FIXED_ENDPOINT = 4,
    QUIRK_MIDI_YAMAHA = 5,
    QUIRK_MIDI_ROLAND = 6,
    QUIRK_MIDI_MIDIMAN = 7,
    QUIRK_MIDI_NOVATION = 8,
    QUIRK_MIDI_RAW_BYTES = 9,
    QUIRK_MIDI_EMAGIC = 10,
    QUIRK_MIDI_CME = 11,
    QUIRK_MIDI_AKAI = 12,
    QUIRK_MIDI_US122L = 13,
    QUIRK_MIDI_FTDI = 14,
    QUIRK_MIDI_CH345 = 15,
    QUIRK_AUDIO_STANDARD_INTERFACE = 16,
    QUIRK_AUDIO_FIXED_ENDPOINT = 17,
    QUIRK_AUDIO_EDIROL_UAXX = 18,
    QUIRK_AUDIO_STANDARD_MIXER = 19,
    QUIRK_TYPE_COUNT = 20,
}

#[repr(C)]
pub struct snd_usb_audio_quirk {
    pub vendor_name: *const i8,
    pub product_name: *const i8,
    pub ifnum: i16,
    pub r#type: u16,
    pub data: *const c_void,
}

#[inline]
pub unsafe fn combine_word(s: *const u8) -> u32 {
    (*s as u32) | ((*s.add(1) as u32) << 8)
}

#[inline]
pub unsafe fn combine_triple(s: *const u8) -> u32 {
    combine_word(s) | ((*s.add(2) as u32) << 16)
}

#[inline]
pub unsafe fn combine_quad(s: *const u8) -> u32 {
    combine_triple(s) | ((*s.add(3) as u32) << 24)
}

extern "C" {
    pub fn snd_usb_lock_shutdown(chip: *mut snd_usb_audio) -> i32;
    pub fn snd_usb_unlock_shutdown(chip: *mut snd_usb_audio);
}

#[repr(C)]
pub struct __snd_usb_lock {
    pub chip: *mut snd_usb_audio,
    pub err: i32,
}

#[inline]
pub unsafe fn __snd_usb_lock_shutdown(chip: *mut snd_usb_audio) -> __snd_usb_lock {
    let mut t = __snd_usb_lock { chip, err: 0 };
    t.err = snd_usb_lock_shutdown(chip);
    t
}

#[inline]
pub unsafe fn __snd_usb_unlock_shutdown(lock: *mut __snd_usb_lock) {
    if (*lock).err == 0 {
        snd_usb_unlock_shutdown((*lock).chip);
    }
}

// DEFINE_CLASS(snd_usb_lock, struct __snd_usb_lock,
//              __snd_usb_unlock_shutdown(&(_T)), __snd_usb_lock_shutdown(chip),
//              struct snd_usb_audio *chip)
// This Linux kernel macro provides RAII-like cleanup for __snd_usb_lock.
// In Rust, this would typically use Drop or guard scope semantics.

extern "C" {
    pub static snd_usb_use_vmalloc: bool;
    pub static snd_usb_skip_validation: bool;
}

// Driver behavior quirk flag indices
pub const QUIRK_TYPE_GET_SAMPLE_RATE: u32 = 0;
pub const QUIRK_TYPE_SHARE_MEDIA_DEVICE: u32 = 1;
pub const QUIRK_TYPE_ALIGN_TRANSFER: u32 = 2;
pub const QUIRK_TYPE_TX_LENGTH: u32 = 3;
pub const QUIRK_TYPE_PLAYBACK_FIRST: u32 = 4;
pub const QUIRK_TYPE_SKIP_CLOCK_SELECTOR: u32 = 5;
pub const QUIRK_TYPE_IGNORE_CLOCK_SOURCE: u32 = 6;
pub const QUIRK_TYPE_ITF_USB_DSD_DAC: u32 = 7;
pub const QUIRK_TYPE_CTL_MSG_DELAY: u32 = 8;
pub const QUIRK_TYPE_CTL_MSG_DELAY_1M: u32 = 9;
pub const QUIRK_TYPE_CTL_MSG_DELAY_5M: u32 = 10;
pub const QUIRK_TYPE_IFACE_DELAY: u32 = 11;
pub const QUIRK_TYPE_VALIDATE_RATES: u32 = 12;
pub const QUIRK_TYPE_DISABLE_AUTOSUSPEND: u32 = 13;
pub const QUIRK_TYPE_IGNORE_CTL_ERROR: u32 = 14;
pub const QUIRK_TYPE_DSD_RAW: u32 = 15;
pub const QUIRK_TYPE_SET_IFACE_FIRST: u32 = 16;
pub const QUIRK_TYPE_GENERIC_IMPLICIT_FB: u32 = 17;
pub const QUIRK_TYPE_SKIP_IMPLICIT_FB: u32 = 18;
pub const QUIRK_TYPE_IFACE_SKIP_CLOSE: u32 = 19;
pub const QUIRK_TYPE_FORCE_IFACE_RESET: u32 = 20;
pub const QUIRK_TYPE_FIXED_RATE: u32 = 21;
pub const QUIRK_TYPE_MIC_RES_16: u32 = 22;
pub const QUIRK_TYPE_MIC_RES_384: u32 = 23;
pub const QUIRK_TYPE_MIXER_PLAYBACK_MIN_MUTE: u32 = 24;
pub const QUIRK_TYPE_MIXER_CAPTURE_MIN_MUTE: u32 = 25;
pub const QUIRK_TYPE_SKIP_IFACE_SETUP: u32 = 26;
pub const QUIRK_TYPE_MIXER_PLAYBACK_LINEAR_VOL: u32 = 27;
pub const QUIRK_TYPE_MIXER_CAPTURE_LINEAR_VOL: u32 = 28;
pub const QUIRK_TYPE_IFB_SILENCE_ON_EMPTY: u32 = 29;
pub const QUIRK_TYPE_MIXER_GET_CUR_OK: u32 = 30;
pub const QUIRK_TYPE_PLAYBACK_URB_FIXUP: u32 = 31;
pub const QUIRK_TYPE_ALWAYS_SET_RATE: u32 = 32;

#[inline]
pub const fn bit_u64(bit: u32) -> u64 {
    1u64 << bit
}

// Quirk flag constants
pub const QUIRK_FLAG_GET_SAMPLE_RATE: u64 = bit_u64(QUIRK_TYPE_GET_SAMPLE_RATE);
pub const QUIRK_FLAG_SHARE_MEDIA_DEVICE: u64 = bit_u64(QUIRK_TYPE_SHARE_MEDIA_DEVICE);
pub const QUIRK_FLAG_ALIGN_TRANSFER: u64 = bit_u64(QUIRK_TYPE_ALIGN_TRANSFER);
pub const QUIRK_FLAG_TX_LENGTH: u64 = bit_u64(QUIRK_TYPE_TX_LENGTH);
pub const QUIRK_FLAG_PLAYBACK_FIRST: u64 = bit_u64(QUIRK_TYPE_PLAYBACK_FIRST);
pub const QUIRK_FLAG_SKIP_CLOCK_SELECTOR: u64 = bit_u64(QUIRK_TYPE_SKIP_CLOCK_SELECTOR);
pub const QUIRK_FLAG_IGNORE_CLOCK_SOURCE: u64 = bit_u64(QUIRK_TYPE_IGNORE_CLOCK_SOURCE);
pub const QUIRK_FLAG_ITF_USB_DSD_DAC: u64 = bit_u64(QUIRK_TYPE_ITF_USB_DSD_DAC);
pub const QUIRK_FLAG_CTL_MSG_DELAY: u64 = bit_u64(QUIRK_TYPE_CTL_MSG_DELAY);
pub const QUIRK_FLAG_CTL_MSG_DELAY_1M: u64 = bit_u64(QUIRK_TYPE_CTL_MSG_DELAY_1M);
pub const QUIRK_FLAG_CTL_MSG_DELAY_5M: u64 = bit_u64(QUIRK_TYPE_CTL_MSG_DELAY_5M);
pub const QUIRK_FLAG_IFACE_DELAY: u64 = bit_u64(QUIRK_TYPE_IFACE_DELAY);
pub const QUIRK_FLAG_VALIDATE_RATES: u64 = bit_u64(QUIRK_TYPE_VALIDATE_RATES);
pub const QUIRK_FLAG_DISABLE_AUTOSUSPEND: u64 = bit_u64(QUIRK_TYPE_DISABLE_AUTOSUSPEND);
pub const QUIRK_FLAG_IGNORE_CTL_ERROR: u64 = bit_u64(QUIRK_TYPE_IGNORE_CTL_ERROR);
pub const QUIRK_FLAG_DSD_RAW: u64 = bit_u64(QUIRK_TYPE_DSD_RAW);
pub const QUIRK_FLAG_SET_IFACE_FIRST: u64 = bit_u64(QUIRK_TYPE_SET_IFACE_FIRST);
pub const QUIRK_FLAG_GENERIC_IMPLICIT_FB: u64 = bit_u64(QUIRK_TYPE_GENERIC_IMPLICIT_FB);
pub const QUIRK_FLAG_SKIP_IMPLICIT_FB: u64 = bit_u64(QUIRK_TYPE_SKIP_IMPLICIT_FB);
pub const QUIRK_FLAG_IFACE_SKIP_CLOSE: u64 = bit_u64(QUIRK_TYPE_IFACE_SKIP_CLOSE);
pub const QUIRK_FLAG_FORCE_IFACE_RESET: u64 = bit_u64(QUIRK_TYPE_FORCE_IFACE_RESET);
pub const QUIRK_FLAG_FIXED_RATE: u64 = bit_u64(QUIRK_TYPE_FIXED_RATE);
pub const QUIRK_FLAG_MIC_RES_16: u64 = bit_u64(QUIRK_TYPE_MIC_RES_16);
pub const QUIRK_FLAG_MIC_RES_384: u64 = bit_u64(QUIRK_TYPE_MIC_RES_384);
pub const QUIRK_FLAG_MIXER_PLAYBACK_MIN_MUTE: u64 = bit_u64(QUIRK_TYPE_MIXER_PLAYBACK_MIN_MUTE);
pub const QUIRK_FLAG_MIXER_CAPTURE_MIN_MUTE: u64 = bit_u64(QUIRK_TYPE_MIXER_CAPTURE_MIN_MUTE);
pub const QUIRK_FLAG_SKIP_IFACE_SETUP: u64 = bit_u64(QUIRK_TYPE_SKIP_IFACE_SETUP);
pub const QUIRK_FLAG_MIXER_PLAYBACK_LINEAR_VOL: u64 = bit_u64(QUIRK_TYPE_MIXER_PLAYBACK_LINEAR_VOL);
pub const QUIRK_FLAG_MIXER_CAPTURE_LINEAR_VOL: u64 = bit_u64(QUIRK_TYPE_MIXER_CAPTURE_LINEAR_VOL);
pub const QUIRK_FLAG_IFB_SILENCE_ON_EMPTY: u64 = bit_u64(QUIRK_TYPE_IFB_SILENCE_ON_EMPTY);
pub const QUIRK_FLAG_MIXER_GET_CUR_OK: u64 = bit_u64(QUIRK_TYPE_MIXER_GET_CUR_OK);
pub const QUIRK_FLAG_PLAYBACK_URB_FIXUP: u64 = bit_u64(QUIRK_TYPE_PLAYBACK_URB_FIXUP);
pub const QUIRK_FLAG_ALWAYS_SET_RATE: u64 = bit_u64(QUIRK_TYPE_ALWAYS_SET_RATE);

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
