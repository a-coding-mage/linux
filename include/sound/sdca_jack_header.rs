/* SPDX-License-Identifier: GPL-2.0 */
/*
 * The MIPI SDCA specification is available for public downloads at
 * https://www.mipi.org/mipi-sdca-v1-0-download
 *
 * Copyright (C) 2025 Cirrus Logic, Inc. and
 *                    Cirrus Logic International Semiconductor Ltd.
 */

// C header guard: __SDCA_JACK_H__

// Opaque types supplied by other translation units.
#[repr(C)]
pub struct sdca_interrupt {
    _private: [u8; 0],
}

#[repr(C)]
pub struct sdca_interrupt_info {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_kcontrol {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_jack {
    _private: [u8; 0],
}

/**
 * struct jack_state - Jack state structure to keep data between interrupts
 * @kctl: Pointer to the ALSA control attached to this jack
 * @jack: Pointer to the ASoC jack struct for this jack
 * @mask: Possible reported jack status bits for this jack
 */
#[repr(C)]
pub struct jack_state {
    pub kctl: *mut snd_kcontrol,
    pub jack: *mut snd_soc_jack,

    pub mask: u32,
}

unsafe extern "C" {
    pub fn sdca_jack_alloc_state(interrupt: *mut sdca_interrupt) -> i32;
    pub fn sdca_jack_init_state(interrupt: *mut sdca_interrupt) -> i32;
    pub fn sdca_jack_free_state(interrupt: *mut sdca_interrupt);

    pub fn sdca_jack_process(interrupt: *mut sdca_interrupt) -> i32;
    pub fn sdca_jack_set_jack(
        info: *mut sdca_interrupt_info,
        jack: *mut snd_soc_jack,
    ) -> i32;
    pub fn sdca_jack_report(interrupt: *mut sdca_interrupt) -> i32;
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
