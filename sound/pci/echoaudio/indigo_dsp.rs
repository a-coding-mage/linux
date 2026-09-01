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

unsafe fn set_vmixer_gain(chip: *mut echoaudio, output: u16, pipe: u16, gain: i32) -> i32;
unsafe fn update_vmixer_level(chip: *mut echoaudio) -> i32;

unsafe fn init_hw(chip: *mut echoaudio, device_id: u16, subdevice_id: u16) -> i32 {
    let mut err: i32;

    if snd_BUG_ON((subdevice_id & 0xfff0) != INDIGO) {
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
    (*chip).dsp_code_to_load = FW_INDIGO_DSP;
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

unsafe fn detect_input_clocks(_chip: *const echoaudio) -> u32 {
    ECHO_CLOCK_BIT_INTERNAL
}

/* The Indigo has no ASIC. Just do nothing */
unsafe fn load_asic(_chip: *mut echoaudio) -> i32 {
    0
}

unsafe fn set_sample_rate(chip: *mut echoaudio, rate: u32) -> i32 {
    let control_reg: u32;

    match rate {
        96000 => {
            control_reg = MIA_96000;
        }
        88200 => {
            control_reg = MIA_88200;
        }
        48000 => {
            control_reg = MIA_48000;
        }
        44100 => {
            control_reg = MIA_44100;
        }
        32000 => {
            control_reg = MIA_32000;
        }
        _ => {
            dev_err(
                (*(*chip).card).dev,
                "set_sample_rate: %d invalid!\n",
                rate,
            );
            return -EINVAL;
        }
    }

    /* Set the control register if it has changed */
    if control_reg != le32_to_cpu((*(*chip).comm_page).control_register) {
        if wait_handshake(chip) != 0 {
            return -EIO;
        }

        (*(*chip).comm_page).sample_rate = cpu_to_le32(rate); /* ignored by the DSP */
        (*(*chip).comm_page).control_register = cpu_to_le32(control_reg);
        (*chip).sample_rate = rate;

        clear_handshake(chip);
        return send_vector(chip, DSP_VC_UPDATE_CLOCKS);
    }
    0
}

/* This function routes the sound from a virtual channel to a real output */
unsafe fn set_vmixer_gain(chip: *mut echoaudio, output: u16, pipe: u16, gain: i32) -> i32 {
    let index: i32;

    if snd_BUG_ON(pipe >= num_pipes_out(chip) || output >= num_busses_out(chip)) {
        return -EINVAL;
    }

    if wait_handshake(chip) != 0 {
        return -EIO;
    }

    (*chip).vmixer_gain[output as usize][pipe as usize] = gain;
    index = output as i32 * num_pipes_out(chip) as i32 + pipe as i32;
    (*(*chip).comm_page).vmixer[index as usize] = gain;

    dev_dbg(
        (*(*chip).card).dev,
        "set_vmixer_gain: pipe %d, out %d = %d\n",
        pipe,
        output,
        gain,
    );
    0
}

/* Tell the DSP to read and update virtual mixer levels in comm page. */
unsafe fn update_vmixer_level(chip: *mut echoaudio) -> i32 {
    if wait_handshake(chip) != 0 {
        return -EIO;
    }
    clear_handshake(chip);
    send_vector(chip, DSP_VC_SET_VMIXER_GAIN)
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
