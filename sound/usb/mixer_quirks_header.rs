// SPDX-License-Identifier: GPL-2.0

// Opaque struct types from external C modules
#[repr(C)]
pub struct usb_mixer_interface {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct snd_usb_audio {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct usb_mixer_elem_info {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct snd_kcontrol {
    _opaque: [u8; 0],
}

extern "C" {
    pub fn snd_usb_mixer_apply_create_quirk(mixer: *mut usb_mixer_interface) -> i32;

    pub fn snd_emuusb_set_samplerate(chip: *mut snd_usb_audio, samplerate_id: u8);

    pub fn snd_usb_mixer_rc_memory_change(mixer: *mut usb_mixer_interface, unitid: i32);

    pub fn snd_usb_mixer_fu_apply_quirk(
        mixer: *mut usb_mixer_interface,
        cval: *mut usb_mixer_elem_info,
        unitid: i32,
        kctl: *mut snd_kcontrol,
    );

    pub fn snd_usb_mixer_resume_quirk(mixer: *mut usb_mixer_interface);
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
