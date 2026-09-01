// SPDX-License-Identifier: LGPL-2.1-or-later
/************************************************************************

This file is part of Echo Digital Audio's generic driver library.
Copyright Echo Digital Audio Corporation (c) 1998 - 2005
All rights reserved
www.echoaudio.com

 Translation from C++ and adaptation for use in ALSA-Driver
 were made by Giuliano Pochini <pochini@shiny.it>

*************************************************************************/

unsafe extern "C" {
    fn update_vmixer_level(chip: *mut echoaudio) -> ::core::ffi::c_int;
    fn set_vmixer_gain(
        chip: *mut echoaudio,
        output: u16,
        pipe: u16,
        gain: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
}

unsafe fn init_hw(
    chip: *mut echoaudio,
    device_id: u16,
    subdevice_id: u16,
) -> ::core::ffi::c_int {
    let mut err: ::core::ffi::c_int;

    if snd_BUG_ON(((subdevice_id as ::core::ffi::c_int) & 0xfff0) != INDIGO_IOX) != 0 {
        return -ENODEV;
    }

    err = init_dsp_comm_page(chip);
    if err < 0 {
        dev_err(
            (*(*chip).card).dev,
            c"init_hw - could not initialize DSP comm page\n".as_ptr(),
        );
        return err;
    }

    (*chip).device_id = device_id;
    (*chip).subdevice_id = subdevice_id;
    (*chip).bad_board = true;
    (*chip).dsp_code_to_load = FW_INDIGO_IOX_DSP;
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

unsafe fn set_mixer_defaults(chip: *mut echoaudio) -> ::core::ffi::c_int {
    init_line_levels(chip)
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
