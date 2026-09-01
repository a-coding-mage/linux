// SPDX-License-Identifier: GPL-2.0-only
/***************************************************************************

   Copyright Echo Digital Audio Corporation (c) 1998 - 2004
   All rights reserved
   www.echoaudio.com

   This file is part of Echo Digital Audio's generic driver library.
   *************************************************************************

 Translation from C++ and adaptation for use in ALSA-Driver
 were made by Giuliano Pochini <pochini@shiny.it>

****************************************************************************/

unsafe fn init_hw(chip: *mut echoaudio, device_id: u16, subdevice_id: u16) -> i32 {
    let mut err: i32;

    if snd_BUG_ON((subdevice_id & 0xfff0) != DARLA20) {
        return -ENODEV;
    }

    err = init_dsp_comm_page(chip);
    if err != 0 {
        dev_err!(
            (*(*chip).card).dev,
            "init_hw: could not initialize DSP comm page\n"
        );
        return err;
    }

    (*chip).device_id = device_id;
    (*chip).subdevice_id = subdevice_id;
    (*chip).bad_board = true;
    (*chip).dsp_code_to_load = FW_DARLA20_DSP;
    (*chip).spdif_status = GD_SPDIF_STATUS_UNDEF;
    (*chip).clock_state = GD_CLOCK_UNDEF;
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

unsafe fn set_mixer_defaults(chip: *mut echoaudio) -> i32 {
    init_line_levels(chip)
}

/* The Darla20 has no external clock sources */
unsafe fn detect_input_clocks(chip: *const echoaudio) -> u32 {
    ECHO_CLOCK_BIT_INTERNAL
}

/* The Darla20 has no ASIC. Just do nothing */
unsafe fn load_asic(chip: *mut echoaudio) -> i32 {
    0
}

unsafe fn set_sample_rate(chip: *mut echoaudio, rate: u32) -> i32 {
    let mut clock_state: u8;
    let mut spdif_status: u8;

    if wait_handshake(chip) != 0 {
        return -EIO;
    }

    match rate {
        44100 => {
            clock_state = GD_CLOCK_44;
            spdif_status = GD_SPDIF_STATUS_44;
        }
        48000 => {
            clock_state = GD_CLOCK_48;
            spdif_status = GD_SPDIF_STATUS_48;
        }
        _ => {
            clock_state = GD_CLOCK_NOCHANGE;
            spdif_status = GD_SPDIF_STATUS_NOCHANGE;
        }
    }

    if (*chip).clock_state == clock_state {
        clock_state = GD_CLOCK_NOCHANGE;
    }
    if spdif_status == (*chip).spdif_status {
        spdif_status = GD_SPDIF_STATUS_NOCHANGE;
    }

    (*(*chip).comm_page).sample_rate = cpu_to_le32(rate);
    (*(*chip).comm_page).gd_clock_state = clock_state;
    (*(*chip).comm_page).gd_spdif_status = spdif_status;
    (*(*chip).comm_page).gd_resampler_state = 3; /* magic number - should always be 3 */

    /* Save the new audio state if it changed */
    if clock_state != GD_CLOCK_NOCHANGE {
        (*chip).clock_state = clock_state;
    }
    if spdif_status != GD_SPDIF_STATUS_NOCHANGE {
        (*chip).spdif_status = spdif_status;
    }
    (*chip).sample_rate = rate;

    clear_handshake(chip);
    send_vector(chip, DSP_VC_SET_GD_AUDIO_STATE)
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
