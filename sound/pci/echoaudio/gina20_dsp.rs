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

use core::ffi::{c_char, c_int};

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_card {
    pub dev: *mut device,
}

#[repr(C)]
pub struct comm_page {
    pub status_clocks: u32,
    pub sample_rate: u32,
    pub gd_clock_state: u8,
    pub gd_spdif_status: u8,
    pub gd_resampler_state: u8,
    pub line_in_level: *mut c_int,
    pub flags: u32,
}

#[repr(C)]
pub struct echoaudio {
    pub card: *mut snd_card,
    pub comm_page: *mut comm_page,
    pub device_id: u16,
    pub subdevice_id: u16,
    pub bad_board: bool,
    pub dsp_code_to_load: c_int,
    pub spdif_status: u8,
    pub clock_state: u8,
    pub asic_loaded: bool,
    pub input_clock_types: u32,
    pub professional_spdif: bool,
    pub sample_rate: u32,
    pub input_clock: u16,
    pub input_gain: *mut c_int,
}

unsafe extern "C" {
    fn snd_BUG_ON(condition: bool) -> bool;
    fn init_dsp_comm_page(chip: *mut echoaudio) -> c_int;
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn load_firmware(chip: *mut echoaudio) -> c_int;
    fn init_line_levels(chip: *mut echoaudio) -> c_int;
    fn le32_to_cpu(value: u32) -> u32;
    fn cpu_to_le32(value: u32) -> u32;
    fn wait_handshake(chip: *mut echoaudio) -> c_int;
    fn clear_handshake(chip: *mut echoaudio);
    fn send_vector(chip: *mut echoaudio, vector: c_int) -> c_int;
    fn num_busses_in(chip: *mut echoaudio) -> u16;
}

unsafe fn set_professional_spdif(chip: *mut echoaudio, prof: c_char) -> c_int {
    if prof != 0 {
        (*(*chip).comm_page).flags |= cpu_to_le32(DSP_FLAG_PROFESSIONAL_SPDIF);
    } else {
        (*(*chip).comm_page).flags &= !cpu_to_le32(DSP_FLAG_PROFESSIONAL_SPDIF);
    }
    (*chip).professional_spdif = prof != 0;
    update_flags(chip)
}

unsafe fn update_flags(chip: *mut echoaudio) -> c_int {
    if wait_handshake(chip) != 0 {
        return -EIO;
    }
    clear_handshake(chip);
    send_vector(chip, DSP_VC_UPDATE_FLAGS)
}

unsafe fn init_hw(chip: *mut echoaudio, device_id: u16, subdevice_id: u16) -> c_int {
    let mut err: c_int;

    if snd_BUG_ON((subdevice_id & 0xfff0) != GINA20) {
        return -ENODEV;
    }

    err = init_dsp_comm_page(chip);
    if err != 0 {
        dev_err(
            (*(*chip).card).dev,
            b"init_hw - could not initialize DSP comm page\n\0".as_ptr() as *const c_char,
        );
        return err;
    }

    (*chip).device_id = device_id;
    (*chip).subdevice_id = subdevice_id;
    (*chip).bad_board = true;
    (*chip).dsp_code_to_load = FW_GINA20_DSP;
    (*chip).spdif_status = GD_SPDIF_STATUS_UNDEF;
    (*chip).clock_state = GD_CLOCK_UNDEF;
    /* Since this card has no ASIC, mark it as loaded so everything
       works OK */
    (*chip).asic_loaded = true;
    (*chip).input_clock_types = ECHO_CLOCK_BIT_INTERNAL | ECHO_CLOCK_BIT_SPDIF;

    err = load_firmware(chip);
    if err < 0 {
        return err;
    }
    (*chip).bad_board = false;

    err
}

unsafe fn set_mixer_defaults(chip: *mut echoaudio) -> c_int {
    (*chip).professional_spdif = false;
    init_line_levels(chip)
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

    clock_bits
}

/* The Gina20 has no ASIC. Just do nothing */
unsafe fn load_asic(_chip: *mut echoaudio) -> c_int {
    0
}

unsafe fn set_sample_rate(chip: *mut echoaudio, rate: u32) -> c_int {
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

unsafe fn set_input_clock(chip: *mut echoaudio, clock: u16) -> c_int {
    match clock {
        ECHO_CLOCK_INTERNAL => {
            /* Reset the audio state to unknown (just in case) */
            (*chip).clock_state = GD_CLOCK_UNDEF;
            (*chip).spdif_status = GD_SPDIF_STATUS_UNDEF;
            set_sample_rate(chip, (*chip).sample_rate);
            (*chip).input_clock = clock;
        }
        ECHO_CLOCK_SPDIF => {
            (*(*chip).comm_page).gd_clock_state = GD_CLOCK_SPDIFIN;
            (*(*chip).comm_page).gd_spdif_status = GD_SPDIF_STATUS_NOCHANGE;
            clear_handshake(chip);
            send_vector(chip, DSP_VC_SET_GD_AUDIO_STATE);
            (*chip).clock_state = GD_CLOCK_SPDIFIN;
            (*chip).input_clock = clock;
        }
        _ => {
            return -EINVAL;
        }
    }

    0
}

/* Set input bus gain (one unit is 0.5dB !) */
unsafe fn set_input_gain(chip: *mut echoaudio, input: u16, mut gain: c_int) -> c_int {
    if snd_BUG_ON(input >= num_busses_in(chip)) {
        return -EINVAL;
    }

    if wait_handshake(chip) != 0 {
        return -EIO;
    }

    *(*chip).input_gain.add(input as usize) = gain;
    gain += GL20_INPUT_GAIN_MAGIC_NUMBER;
    *(*(*chip).comm_page).line_in_level.add(input as usize) = gain;
    0
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
