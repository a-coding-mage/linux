// SPDX-License-Identifier: GPL-2.0-or-later

// Depends on types from midi.h and USB kernel headers

use std::os::raw::{c_int, c_uint};

// External opaque types
#[repr(C)]
pub struct usb_interface;

#[repr(C)]
pub struct snd_usb_audio_quirk;

#[repr(C)]
pub struct snd_card;

#[repr(C)]
pub struct list_head;

// snd_usb_audio structure with fields accessed by inline functions below
#[repr(C)]
pub struct snd_usb_audio {
    pub card: *mut snd_card,
    pub midi_list: list_head,
    pub num_rawmidis: c_uint,
}

// CONFIG_SND_USB_AUDIO_MIDI_V2: when enabled, declares full MIDI v2 support
#[cfg(feature = "CONFIG_SND_USB_AUDIO_MIDI_V2")]
extern "C" {
    pub fn snd_usb_midi_v2_create(
        chip: *mut snd_usb_audio,
        iface: *mut usb_interface,
        quirk: *const snd_usb_audio_quirk,
        usb_id: c_uint,
    ) -> c_int;

    pub fn snd_usb_midi_v2_suspend_all(chip: *mut snd_usb_audio);
    pub fn snd_usb_midi_v2_resume_all(chip: *mut snd_usb_audio);
    pub fn snd_usb_midi_v2_disconnect_all(chip: *mut snd_usb_audio);
    pub fn snd_usb_midi_v2_free_all(chip: *mut snd_usb_audio);
}

// CONFIG_SND_USB_AUDIO_MIDI_V2: when disabled, fallback to MIDI 1.0 creation
#[cfg(not(feature = "CONFIG_SND_USB_AUDIO_MIDI_V2"))]
extern "C" {
    fn __snd_usbmidi_create(
        card: *mut snd_card,
        iface: *mut usb_interface,
        midi_list: *mut list_head,
        quirk: *const snd_usb_audio_quirk,
        usb_id: c_uint,
        num_rawmidis: *mut c_uint,
    ) -> c_int;
}

#[cfg(not(feature = "CONFIG_SND_USB_AUDIO_MIDI_V2"))]
#[inline]
pub fn snd_usb_midi_v2_create(
    chip: *mut snd_usb_audio,
    iface: *mut usb_interface,
    quirk: *const snd_usb_audio_quirk,
    usb_id: c_uint,
) -> c_int {
    unsafe {
        __snd_usbmidi_create(
            (*chip).card,
            iface,
            &mut (*chip).midi_list,
            quirk,
            usb_id,
            &mut (*chip).num_rawmidis,
        )
    }
}

#[cfg(not(feature = "CONFIG_SND_USB_AUDIO_MIDI_V2"))]
#[inline]
pub fn snd_usb_midi_v2_suspend_all(_chip: *mut snd_usb_audio) {}

#[cfg(not(feature = "CONFIG_SND_USB_AUDIO_MIDI_V2"))]
#[inline]
pub fn snd_usb_midi_v2_resume_all(_chip: *mut snd_usb_audio) {}

#[cfg(not(feature = "CONFIG_SND_USB_AUDIO_MIDI_V2"))]
#[inline]
pub fn snd_usb_midi_v2_disconnect_all(_chip: *mut snd_usb_audio) {}

#[cfg(not(feature = "CONFIG_SND_USB_AUDIO_MIDI_V2"))]
#[inline]
pub fn snd_usb_midi_v2_free_all(_chip: *mut snd_usb_audio) {}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
