/* SPDX-License-Identifier: GPL-2.0-or-later */

/*
 *  Routines for control of TEA6330T circuit.
 *  Sound fader control circuit for car radios.
 */

/* Dependency supplied by sound/i2c.h in the original source. */

#[repr(C)]
pub struct snd_i2c_bus {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_card {
    _private: [u8; 0],
}

extern "C" {
    pub fn snd_tea6330t_detect(bus: *mut snd_i2c_bus, equalizer: ::std::os::raw::c_int)
        -> ::std::os::raw::c_int;
    pub fn snd_tea6330t_update_mixer(
        card: *mut snd_card,
        bus: *mut snd_i2c_bus,
        equalizer: ::std::os::raw::c_int,
        fader: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
    pub fn snd_tea6330t_restore_mixer(bus: *mut snd_i2c_bus) -> ::std::os::raw::c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
