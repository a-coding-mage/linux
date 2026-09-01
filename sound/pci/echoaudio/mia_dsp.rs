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

// C include dependencies and external driver symbols are expected to be
// supplied by the surrounding translation unit/module.

unsafe extern "C" {
    fn init_dsp_comm_page(chip: *mut echoaudio) -> ::core::ffi::c_int;
    fn load_firmware(chip: *mut echoaudio) -> ::core::ffi::c_int;
    fn init_line_levels(chip: *mut echoaudio) -> ::core::ffi::c_int;
    fn wait_handshake(chip: *mut echoaudio) -> ::core::ffi::c_int;
    fn clear_handshake(chip: *mut echoaudio);
    fn send_vector(chip: *mut echoaudio, vector: u32) -> ::core::ffi::c_int;
    fn num_pipes_out(chip: *mut echoaudio) -> u16;
    fn num_busses_out(chip: *mut echoaudio) -> u16;
    fn le32_to_cpu(value: u32) -> u32;
    fn cpu_to_le32(value: u32) -> u32;
    fn snd_BUG_ON(condition: bool) -> ::core::ffi::c_int;
    fn dev_err(dev: *mut ::core::ffi::c_void, fmt: *const ::core::ffi::c_char, ...);
    fn dev_dbg(dev: *mut ::core::ffi::c_void, fmt: *const ::core::ffi::c_char, ...);
}

unsafe fn init_hw(
    chip: *mut echoaudio,
    device_id: u16,
    subdevice_id: u16,
) -> ::core::ffi::c_int {
    let mut err: ::core::ffi::c_int;

    if snd_BUG_ON((subdevice_id & 0xfff0) != MIA) != 0 {
        return -ENODEV;
    }

    err = init_dsp_comm_page(chip);
    if err != 0 {
        dev_err(
            (*(*chip).card).dev,
            b"init_hw - could not initialize DSP comm page\n\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return err;
    }

    (*chip).device_id = device_id;
    (*chip).subdevice_id = subdevice_id;
    (*chip).bad_board = true;
    (*chip).dsp_code_to_load = FW_MIA_DSP;
    /* Since this card has no ASIC, mark it as loaded so everything
       works OK */
    (*chip).asic_loaded = true;
    if (subdevice_id & 0x000f) == MIA_MIDI_REV {
        (*chip).has_midi = true;
    }
    (*chip).input_clock_types = ECHO_CLOCK_BIT_INTERNAL | ECHO_CLOCK_BIT_SPDIF;

    err = load_firmware(chip);
    if err < 0 {
        return err;
    }
    (*chip).bad_board = false;

    return err;
}

unsafe fn set_mixer_defaults(chip: *mut echoaudio) -> ::core::ffi::c_int {
    return init_line_levels(chip);
}

unsafe fn detect_input_clocks(chip: *const echoaudio) -> u32 {
    let clocks_from_dsp: u32;
    let mut clock_bits: u32;

    /* Map the DSP clock detect bits to the generic driver clock
       detect bits */
    clocks_from_dsp = le32_to_cpu((*(*chip).comm_page).status_clocks);

    clock_bits = ECHO_CLOCK_BIT_INTERNAL;

    if (clocks_from_dsp & GLDM_CLOCK_DETECT_BIT_SPDIF) != 0 {
        clock_bits |= ECHO_CLOCK_BIT_SPDIF;
    }

    return clock_bits;
}

/* The Mia has no ASIC. Just do nothing */
unsafe fn load_asic(_chip: *mut echoaudio) -> ::core::ffi::c_int {
    return 0;
}

unsafe fn set_sample_rate(chip: *mut echoaudio, rate: u32) -> ::core::ffi::c_int {
    let mut control_reg: u32;

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
                b"set_sample_rate: %d invalid!\n\0".as_ptr() as *const ::core::ffi::c_char,
                rate,
            );
            return -EINVAL;
        }
    }

    /* Override the clock setting if this Mia is set to S/PDIF clock */
    if (*chip).input_clock == ECHO_CLOCK_SPDIF {
        control_reg |= MIA_SPDIF;
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
    return 0;
}

unsafe fn set_input_clock(chip: *mut echoaudio, clock: u16) -> ::core::ffi::c_int {
    dev_dbg(
        (*(*chip).card).dev,
        b"set_input_clock(%d)\n\0".as_ptr() as *const ::core::ffi::c_char,
        clock as ::core::ffi::c_int,
    );
    if snd_BUG_ON(clock != ECHO_CLOCK_INTERNAL && clock != ECHO_CLOCK_SPDIF) != 0 {
        return -EINVAL;
    }

    (*chip).input_clock = clock;
    return set_sample_rate(chip, (*chip).sample_rate);
}

/* This function routes the sound from a virtual channel to a real output */
unsafe fn set_vmixer_gain(
    chip: *mut echoaudio,
    output: u16,
    pipe: u16,
    gain: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let index: ::core::ffi::c_int;

    if snd_BUG_ON(pipe >= num_pipes_out(chip) || output >= num_busses_out(chip)) != 0 {
        return -EINVAL;
    }

    if wait_handshake(chip) != 0 {
        return -EIO;
    }

    (*chip).vmixer_gain[output as usize][pipe as usize] = gain;
    index = (output as ::core::ffi::c_int) * (num_pipes_out(chip) as ::core::ffi::c_int)
        + pipe as ::core::ffi::c_int;
    (*(*chip).comm_page).vmixer[index as usize] = gain;

    dev_dbg(
        (*(*chip).card).dev,
        b"set_vmixer_gain: pipe %d, out %d = %d\n\0".as_ptr() as *const ::core::ffi::c_char,
        pipe as ::core::ffi::c_int,
        output as ::core::ffi::c_int,
        gain,
    );
    return 0;
}

/* Tell the DSP to read and update virtual mixer levels in comm page. */
unsafe fn update_vmixer_level(chip: *mut echoaudio) -> ::core::ffi::c_int {
    if wait_handshake(chip) != 0 {
        return -EIO;
    }
    clear_handshake(chip);
    return send_vector(chip, DSP_VC_SET_VMIXER_GAIN);
}

/* Tell the DSP to reread the flags from the comm page */
unsafe fn update_flags(chip: *mut echoaudio) -> ::core::ffi::c_int {
    if wait_handshake(chip) != 0 {
        return -EIO;
    }
    clear_handshake(chip);
    return send_vector(chip, DSP_VC_UPDATE_FLAGS);
}

unsafe fn set_professional_spdif(
    chip: *mut echoaudio,
    prof: ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    dev_dbg(
        (*(*chip).card).dev,
        b"set_professional_spdif %d\n\0".as_ptr() as *const ::core::ffi::c_char,
        prof as ::core::ffi::c_int,
    );
    if prof != 0 {
        (*(*chip).comm_page).flags |= cpu_to_le32(DSP_FLAG_PROFESSIONAL_SPDIF);
    } else {
        (*(*chip).comm_page).flags &= !cpu_to_le32(DSP_FLAG_PROFESSIONAL_SPDIF);
    }
    (*chip).professional_spdif = prof;
    return update_flags(chip);
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
