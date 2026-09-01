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

    if snd_BUG_ON((subdevice_id & 0xfff0) != DARLA24) {
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
    (*chip).dsp_code_to_load = FW_DARLA24_DSP;
    /* Since this card has no ASIC, mark it as loaded so everything
       works OK */
    (*chip).asic_loaded = true;
    (*chip).input_clock_types = ECHO_CLOCK_BIT_INTERNAL | ECHO_CLOCK_BIT_ESYNC;

    err = load_firmware(chip);
    if err < 0 {
        return err;
    }
    (*chip).bad_board = false;

    return err;
}

unsafe fn set_mixer_defaults(chip: *mut echoaudio) -> i32 {
    return init_line_levels(chip);
}

unsafe fn detect_input_clocks(chip: *const echoaudio) -> u32 {
    let clocks_from_dsp: u32;
    let mut clock_bits: u32;

    /* Map the DSP clock detect bits to the generic driver clock
       detect bits */
    clocks_from_dsp = le32_to_cpu((*(*chip).comm_page).status_clocks);

    clock_bits = ECHO_CLOCK_BIT_INTERNAL;

    if (clocks_from_dsp & GLDM_CLOCK_DETECT_BIT_ESYNC) != 0 {
        clock_bits |= ECHO_CLOCK_BIT_ESYNC;
    }

    return clock_bits;
}

/* The Darla24 has no ASIC. Just do nothing */
unsafe fn load_asic(_chip: *mut echoaudio) -> i32 {
    return 0;
}

unsafe fn set_sample_rate(chip: *mut echoaudio, rate: u32) -> i32 {
    let mut clock: u8;

    match rate {
        96000 => {
            clock = GD24_96000;
        }
        88200 => {
            clock = GD24_88200;
        }
        48000 => {
            clock = GD24_48000;
        }
        44100 => {
            clock = GD24_44100;
        }
        32000 => {
            clock = GD24_32000;
        }
        22050 => {
            clock = GD24_22050;
        }
        16000 => {
            clock = GD24_16000;
        }
        11025 => {
            clock = GD24_11025;
        }
        8000 => {
            clock = GD24_8000;
        }
        _ => {
            dev_err!(
                (*(*chip).card).dev,
                "set_sample_rate: Error, invalid sample rate %d\n",
                rate
            );
            return -EINVAL;
        }
    }

    if wait_handshake(chip) != 0 {
        return -EIO;
    }

    dev_dbg!(
        (*(*chip).card).dev,
        "set_sample_rate: %d clock %d\n",
        rate,
        clock
    );
    (*chip).sample_rate = rate;

    /* Override the sample rate if this card is set to Echo sync. */
    if (*chip).input_clock == ECHO_CLOCK_ESYNC {
        clock = GD24_EXT_SYNC;
    }

    (*(*chip).comm_page).sample_rate = cpu_to_le32(rate); /* ignored by the DSP ? */
    (*(*chip).comm_page).gd_clock_state = clock;
    clear_handshake(chip);
    return send_vector(chip, DSP_VC_SET_GD_AUDIO_STATE);
}

unsafe fn set_input_clock(chip: *mut echoaudio, clock: u16) -> i32 {
    if snd_BUG_ON(clock != ECHO_CLOCK_INTERNAL && clock != ECHO_CLOCK_ESYNC) {
        return -EINVAL;
    }
    (*chip).input_clock = clock;
    return set_sample_rate(chip, (*chip).sample_rate);
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
