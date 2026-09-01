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

// Dependencies supplied by the surrounding driver:
// struct echoaudio, u16, u32, INDIGO_IO, ENODEV, EIO, EINVAL,
// FW_INDIGO_IO_DSP, ECHO_CLOCK_BIT_INTERNAL, DSP_VC_UPDATE_CLOCKS,
// DSP_VC_SET_VMIXER_GAIN, snd_BUG_ON, init_dsp_comm_page, dev_err,
// load_firmware, init_line_levels, wait_handshake, cpu_to_le32,
// clear_handshake, send_vector, num_pipes_out, num_busses_out, dev_dbg.

unsafe fn init_hw(chip: *mut echoaudio, device_id: u16, subdevice_id: u16) -> i32 {
    let mut err: i32;

    if snd_BUG_ON((subdevice_id & 0xfff0) != INDIGO_IO) {
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
    (*chip).dsp_code_to_load = FW_INDIGO_IO_DSP;
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

unsafe fn detect_input_clocks(chip: *const echoaudio) -> u32 {
    ECHO_CLOCK_BIT_INTERNAL
}

/* The IndigoIO has no ASIC. Just do nothing */
unsafe fn load_asic(chip: *mut echoaudio) -> i32 {
    0
}

unsafe fn set_sample_rate(chip: *mut echoaudio, rate: u32) -> i32 {
    if wait_handshake(chip) != 0 {
        return -EIO;
    }

    (*chip).sample_rate = rate;
    (*(*chip).comm_page).sample_rate = cpu_to_le32(rate);
    clear_handshake(chip);
    send_vector(chip, DSP_VC_UPDATE_CLOCKS)
}

/* This function routes the sound from a virtual channel to a real output */
unsafe fn set_vmixer_gain(chip: *mut echoaudio, output: u16, pipe: u16, gain: i32) -> i32 {
    let index: i32;

    if snd_BUG_ON(
        pipe >= num_pipes_out(chip) as u16 || output >= num_busses_out(chip) as u16,
    ) {
        return -EINVAL;
    }

    if wait_handshake(chip) != 0 {
        return -EIO;
    }

    (*chip).vmixer_gain[output as usize][pipe as usize] = gain;
    index = output as i32 * num_pipes_out(chip) + pipe as i32;
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
