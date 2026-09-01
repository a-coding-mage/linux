// SPDX-License-Identifier: GPL-2.0-only
/****************************************************************************

   Copyright Echo Digital Audio Corporation (c) 1998 - 2004
   All rights reserved
   www.echoaudio.com

   This file is part of Echo Digital Audio's generic driver library.
   *************************************************************************

 Translation from C++ and adaptation for use in ALSA-Driver
 were made by Giuliano Pochini <pochini@shiny.it>

****************************************************************************/

use crate::*;

unsafe extern "C" {
    fn read_dsp(chip: *mut echoaudio, data: *mut u32) -> i32;
    fn load_asic_generic(chip: *mut echoaudio, cmd: u32, asic: i16) -> i32;
}

unsafe fn init_hw(chip: *mut echoaudio, device_id: u16, subdevice_id: u16) -> i32 {
    let mut err: i32;

    if snd_BUG_ON(((subdevice_id as u32) & 0xfff0) != LAYLA20) {
        return -ENODEV;
    }

    err = init_dsp_comm_page(chip);
    if err != 0 {
        dev_err(
            (*(*chip).card).dev,
            "init_hw - could not initialize DSP comm page\n",
        );
        return err;
    }

    (*chip).device_id = device_id;
    (*chip).subdevice_id = subdevice_id;
    (*chip).bad_board = true;
    (*chip).has_midi = true;
    (*chip).dsp_code_to_load = FW_LAYLA20_DSP;
    (*chip).input_clock_types = ECHO_CLOCK_BIT_INTERNAL
        | ECHO_CLOCK_BIT_SPDIF
        | ECHO_CLOCK_BIT_WORD
        | ECHO_CLOCK_BIT_SUPER;
    (*chip).output_clock_types = ECHO_CLOCK_BIT_WORD | ECHO_CLOCK_BIT_SUPER;

    err = load_firmware(chip);
    if err < 0 {
        return err;
    }
    (*chip).bad_board = false;

    err
}

unsafe fn set_mixer_defaults(chip: *mut echoaudio) -> i32 {
    (*chip).professional_spdif = false;
    init_line_levels(chip)
}

unsafe fn detect_input_clocks(chip: *const echoaudio) -> u32 {
    let clocks_from_dsp: u32;
    let mut clock_bits: u32;

    /* Map the DSP clock detect bits to the generic driver clock detect bits */
    clocks_from_dsp = le32_to_cpu((*(*chip).comm_page).status_clocks);

    clock_bits = ECHO_CLOCK_BIT_INTERNAL;

    if (clocks_from_dsp & GLDM_CLOCK_DETECT_BIT_SPDIF) != 0 {
        clock_bits |= ECHO_CLOCK_BIT_SPDIF;
    }

    if (clocks_from_dsp & GLDM_CLOCK_DETECT_BIT_WORD) != 0 {
        if (clocks_from_dsp & GLDM_CLOCK_DETECT_BIT_SUPER) != 0 {
            clock_bits |= ECHO_CLOCK_BIT_SUPER;
        } else {
            clock_bits |= ECHO_CLOCK_BIT_WORD;
        }
    }

    clock_bits
}

/* ASIC status check - some cards have one or two ASICs that need to be
loaded.  Once that load is complete, this function is called to see if
the load was successful.
If this load fails, it does not necessarily mean that the hardware is
defective - the external box may be disconnected or turned off.
This routine sometimes fails for Layla20; for Layla20, the loop runs
5 times and succeeds if it wins on three of the loops. */
unsafe fn check_asic_status(chip: *mut echoaudio) -> i32 {
    let mut asic_status: u32 = 0;
    let mut goodcnt: i32;
    let mut i: i32;

    (*chip).asic_loaded = false;
    goodcnt = 0;
    i = 0;
    while i < 5 {
        send_vector(chip, DSP_VC_TEST_ASIC);

        /* The DSP will return a value to indicate whether or not
           the ASIC is currently loaded */
        if read_dsp(chip, &mut asic_status) < 0 {
            dev_err(
                (*(*chip).card).dev,
                "check_asic_status: failed on read_dsp\n",
            );
            return -EIO;
        }

        if asic_status == ASIC_ALREADY_LOADED {
            goodcnt += 1;
            if goodcnt == 3 {
                (*chip).asic_loaded = true;
                return 0;
            }
        }
        i += 1;
    }
    -EIO
}

/* Layla20 has an ASIC in the external box */
unsafe fn load_asic(chip: *mut echoaudio) -> i32 {
    let err: i32;

    if (*chip).asic_loaded {
        return 0;
    }

    err = load_asic_generic(chip, DSP_FNC_LOAD_LAYLA_ASIC, FW_LAYLA20_ASIC);
    if err < 0 {
        return err;
    }

    /* Check if ASIC is alive and well. */
    check_asic_status(chip)
}

unsafe fn set_sample_rate(chip: *mut echoaudio, rate: u32) -> i32 {
    if snd_BUG_ON(rate < 8000 || rate > 50000) {
        return -EINVAL;
    }

    /* Only set the clock for internal mode. Do not return failure,
       simply treat it as a non-event. */
    if (*chip).input_clock != ECHO_CLOCK_INTERNAL {
        dev_warn(
            (*(*chip).card).dev,
            "Cannot set sample rate - clock not set to CLK_CLOCKININTERNAL\n",
        );
        (*(*chip).comm_page).sample_rate = cpu_to_le32(rate);
        (*chip).sample_rate = rate;
        return 0;
    }

    if wait_handshake(chip) != 0 {
        return -EIO;
    }

    dev_dbg((*(*chip).card).dev, "set_sample_rate(%d)\n", rate);
    (*chip).sample_rate = rate;
    (*(*chip).comm_page).sample_rate = cpu_to_le32(rate);
    clear_handshake(chip);
    send_vector(chip, DSP_VC_SET_LAYLA_SAMPLE_RATE)
}

unsafe fn set_input_clock(chip: *mut echoaudio, clock_source: u16) -> i32 {
    let mut clock: u16;
    let mut rate: u32;

    rate = 0;
    match clock_source {
        ECHO_CLOCK_INTERNAL => {
            rate = (*chip).sample_rate;
            clock = LAYLA20_CLOCK_INTERNAL;
        }
        ECHO_CLOCK_SPDIF => {
            clock = LAYLA20_CLOCK_SPDIF;
        }
        ECHO_CLOCK_WORD => {
            clock = LAYLA20_CLOCK_WORD;
        }
        ECHO_CLOCK_SUPER => {
            clock = LAYLA20_CLOCK_SUPER;
        }
        _ => {
            dev_err(
                (*(*chip).card).dev,
                "Input clock 0x%x not supported for Layla24\n",
                clock_source,
            );
            return -EINVAL;
        }
    }
    (*chip).input_clock = clock_source;

    (*(*chip).comm_page).input_clock = cpu_to_le16(clock);
    clear_handshake(chip);
    send_vector(chip, DSP_VC_UPDATE_CLOCKS);

    if rate != 0 {
        set_sample_rate(chip, rate);
    }

    0
}

unsafe fn set_output_clock(chip: *mut echoaudio, mut clock: u16) -> i32 {
    match clock {
        ECHO_CLOCK_SUPER => {
            clock = LAYLA20_OUTPUT_CLOCK_SUPER;
        }
        ECHO_CLOCK_WORD => {
            clock = LAYLA20_OUTPUT_CLOCK_WORD;
        }
        _ => {
            dev_err((*(*chip).card).dev, "set_output_clock wrong clock\n");
            return -EINVAL;
        }
    }

    if wait_handshake(chip) != 0 {
        return -EIO;
    }

    (*(*chip).comm_page).output_clock = cpu_to_le16(clock);
    (*chip).output_clock = clock;
    clear_handshake(chip);
    send_vector(chip, DSP_VC_UPDATE_CLOCKS)
}

/* Set input bus gain (one unit is 0.5dB !) */
unsafe fn set_input_gain(chip: *mut echoaudio, input: u16, mut gain: i32) -> i32 {
    if snd_BUG_ON(input as u32 >= num_busses_in(chip)) {
        return -EINVAL;
    }

    if wait_handshake(chip) != 0 {
        return -EIO;
    }

    (*chip).input_gain[input as usize] = gain;
    gain += GL20_INPUT_GAIN_MAGIC_NUMBER;
    (*(*chip).comm_page).line_in_level[input as usize] = gain;
    0
}

/* Tell the DSP to reread the flags from the comm page */
unsafe fn update_flags(chip: *mut echoaudio) -> i32 {
    if wait_handshake(chip) != 0 {
        return -EIO;
    }
    clear_handshake(chip);
    send_vector(chip, DSP_VC_UPDATE_FLAGS)
}

unsafe fn set_professional_spdif(chip: *mut echoaudio, prof: i8) -> i32 {
    if prof != 0 {
        (*(*chip).comm_page).flags |= cpu_to_le32(DSP_FLAG_PROFESSIONAL_SPDIF);
    } else {
        (*(*chip).comm_page).flags &= !cpu_to_le32(DSP_FLAG_PROFESSIONAL_SPDIF);
    }
    (*chip).professional_spdif = prof != 0;
    update_flags(chip)
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
