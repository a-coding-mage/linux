// SPDX-License-Identifier: LGPL-2.1-or-later
/************************************************************************

This file is part of Echo Digital Audio's generic driver library.
Copyright Echo Digital Audio Corporation (c) 1998 - 2005
All rights reserved
www.echoaudio.com

 Translation from C++ and adaptation for use in ALSA-Driver
 were made by Giuliano Pochini <pochini@shiny.it>

*************************************************************************/

use core::ffi::{c_char, c_int, c_void};

type u16 = u16;

#[repr(C)]
pub struct snd_card {
    pub dev: *mut c_void,
}

#[repr(C)]
pub struct echoaudio {
    pub card: *mut snd_card,
    pub device_id: u16,
    pub subdevice_id: u16,
    pub bad_board: bool,
    pub dsp_code_to_load: c_int,
    pub asic_loaded: bool,
    pub input_clock_types: c_int,
}

unsafe extern "C" {
    fn init_dsp_comm_page(chip: *mut echoaudio) -> c_int;
    fn load_firmware(chip: *mut echoaudio) -> c_int;
    fn init_line_levels(chip: *mut echoaudio) -> c_int;

    fn snd_BUG_ON(condition: bool) -> bool;
    fn dev_err(dev: *mut c_void, fmt: *const c_char, ...);

    static INDIGO_DJX: u16;
    static FW_INDIGO_DJX_DSP: c_int;
    static ECHO_CLOCK_BIT_INTERNAL: c_int;
    static ENODEV: c_int;
}

unsafe fn update_vmixer_level(chip: *mut echoaudio) -> c_int;
unsafe fn set_vmixer_gain(chip: *mut echoaudio, output: u16, pipe: u16, gain: c_int) -> c_int;

unsafe fn init_hw(chip: *mut echoaudio, device_id: u16, subdevice_id: u16) -> c_int {
    let mut err: c_int;

    if snd_BUG_ON((subdevice_id & 0xfff0) != INDIGO_DJX) {
        return -ENODEV;
    }

    err = init_dsp_comm_page(chip);
    if err < 0 {
        dev_err(
            (*(*chip).card).dev,
            b"init_hw - could not initialize DSP comm page\n\0".as_ptr() as *const c_char,
        );
        return err;
    }

    (*chip).device_id = device_id;
    (*chip).subdevice_id = subdevice_id;
    (*chip).bad_board = true;
    (*chip).dsp_code_to_load = FW_INDIGO_DJX_DSP;
    /* Since this card has no ASIC, mark it as loaded so everything
       works OK */
    (*chip).asic_loaded = true;
    (*chip).input_clock_types = ECHO_CLOCK_BIT_INTERNAL;

    err = load_firmware(chip);
    if err < 0 {
        return err;
    }
    (*chip).bad_board = false;

    err
}

unsafe fn set_mixer_defaults(chip: *mut echoaudio) -> c_int {
    init_line_levels(chip)
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
