// SPDX-License-Identifier: GPL-2.0

#[repr(C)]
pub struct snd_usb_power_domain {
    pub pd_id: i32,              // UAC3 Power Domain ID
    pub pd_d1d0_rec: i32,        // D1 to D0 recovery time
    pub pd_d2d0_rec: i32,        // D2 to D0 recovery time
    pub ctrl_iface: *mut usb_host_interface, // Control interface
}

pub const UAC3_PD_STATE_D0: i32 = 0;
pub const UAC3_PD_STATE_D1: i32 = 1;
pub const UAC3_PD_STATE_D2: i32 = 2;

extern "C" {
    pub fn snd_usb_power_domain_set(
        chip: *mut snd_usb_audio,
        pd: *mut snd_usb_power_domain,
        state: u8,
    ) -> i32;

    pub fn snd_usb_find_power_domain(
        ctrl_iface: *mut usb_host_interface,
        id: u8,
    ) -> *mut snd_usb_power_domain;

    pub fn snd_usb_autoresume(chip: *mut snd_usb_audio) -> i32;

    pub fn snd_usb_autosuspend(chip: *mut snd_usb_audio);
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
