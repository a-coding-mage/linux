// SPDX-License-Identifier: GPL-2.0

// maximum number of endpoints per interface
pub const MIDI_MAX_ENDPOINTS: usize = 2;

// data for QUIRK_MIDI_FIXED_ENDPOINT
#[repr(C)]
pub struct snd_usb_midi_endpoint_info {
    pub out_ep: i8,          // ep number, 0 autodetect
    pub out_interval: u8,    // interval for interrupt endpoints
    pub in_ep: i8,
    pub in_interval: u8,
    pub out_cables: u16,     // bitmask
    pub in_cables: u16,      // bitmask
    pub assoc_in_jacks: [i16; 16],
    pub assoc_out_jacks: [i16; 16],
}

// for QUIRK_MIDI_YAMAHA, data is NULL

// for QUIRK_MIDI_MIDIMAN, data points to a snd_usb_midi_endpoint_info
// structure (out_cables and in_cables only)

// for QUIRK_COMPOSITE, data points to an array of snd_usb_audio_quirk
// structures, terminated with .ifnum = -1

// for QUIRK_AUDIO_FIXED_ENDPOINT, data points to an audioformat structure

// for QUIRK_AUDIO/MIDI_STANDARD_INTERFACE, data is NULL

// for QUIRK_AUDIO_EDIROL_UA700_UA25/UA1000, data is NULL

// for QUIRK_IGNORE_INTERFACE, data is NULL

// for QUIRK_MIDI_NOVATION and _RAW, data is NULL

// for QUIRK_MIDI_EMAGIC, data points to a snd_usb_midi_endpoint_info
// structure (out_cables and in_cables only)

// for QUIRK_MIDI_CME, data is NULL

// for QUIRK_MIDI_AKAI, data is NULL

extern "C" {
    pub fn __snd_usbmidi_create(
        card: *mut snd_card,
        iface: *mut usb_interface,
        midi_list: *mut list_head,
        quirk: *const snd_usb_audio_quirk,
        usb_id: u32,
        num_rawmidis: *mut u32,
    ) -> i32;
}

pub fn snd_usbmidi_create(
    card: *mut snd_card,
    iface: *mut usb_interface,
    midi_list: *mut list_head,
    quirk: *const snd_usb_audio_quirk,
) -> i32 {
    unsafe { __snd_usbmidi_create(card, iface, midi_list, quirk, 0, core::ptr::null_mut()) }
}

extern "C" {
    pub fn snd_usbmidi_input_stop(p: *mut list_head);
    pub fn snd_usbmidi_input_start(p: *mut list_head);
    pub fn snd_usbmidi_disconnect(p: *mut list_head);
    pub fn snd_usbmidi_suspend(p: *mut list_head);
    pub fn snd_usbmidi_resume(p: *mut list_head);
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
