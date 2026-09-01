// SPDX-License-Identifier: LGPL-2.1-or-later
/************************************************************************

This file is part of Echo Digital Audio's generic driver library.
Copyright Echo Digital Audio Corporation (c) 1998 - 2005
All rights reserved
www.echoaudio.com

 Translation from C++ and adaptation for use in ALSA-Driver
 were made by Giuliano Pochini <pochini@shiny.it>

*************************************************************************/

unsafe fn set_sample_rate(chip: *mut echoaudio, rate: u32) -> i32 {
    let clock: u32;
    let mut control_reg: u32;
    let old_control_reg: u32;

    if wait_handshake(chip) != 0 {
        return -EIO;
    }

    old_control_reg = le32_to_cpu((*(*chip).comm_page).control_register);
    control_reg = old_control_reg & !INDIGO_EXPRESS_CLOCK_MASK;

    match rate {
        32000 => {
            clock = INDIGO_EXPRESS_32000;
        }
        44100 => {
            clock = INDIGO_EXPRESS_44100;
        }
        48000 => {
            clock = INDIGO_EXPRESS_48000;
        }
        64000 => {
            clock = INDIGO_EXPRESS_32000 | INDIGO_EXPRESS_DOUBLE_SPEED;
        }
        88200 => {
            clock = INDIGO_EXPRESS_44100 | INDIGO_EXPRESS_DOUBLE_SPEED;
        }
        96000 => {
            clock = INDIGO_EXPRESS_48000 | INDIGO_EXPRESS_DOUBLE_SPEED;
        }
        _ => {
            return -EINVAL;
        }
    }

    control_reg |= clock;
    if control_reg != old_control_reg {
        dev_dbg(
            (*(*chip).card).dev,
            c"set_sample_rate: %d clock %d\n".as_ptr(),
            rate,
            clock,
        );
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

    if snd_BUG_ON(
        pipe as u32 >= num_pipes_out(chip) ||
        output as u32 >= num_busses_out(chip),
    ) != 0 {
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
        c"set_vmixer_gain: pipe %d, out %d = %d\n".as_ptr(),
        pipe as i32,
        output as i32,
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



unsafe fn detect_input_clocks(chip: *const echoaudio) -> u32 {
    ECHO_CLOCK_BIT_INTERNAL
}



/* The IndigoIO has no ASIC. Just do nothing */
unsafe fn load_asic(chip: *mut echoaudio) -> i32 {
    0
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
