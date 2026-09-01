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

// Static forward declarations in the original C source:
// load_asic, dsp_set_digital_mode, set_digital_mode, check_asic_status,
// set_sample_rate, set_input_clock, set_professional_spdif,
// set_phantom_power, write_control_reg.

// Original C source included <linux/interrupt.h> for local_irq_enable().

unsafe fn init_hw(chip: *mut echoaudio, device_id: u16, subdevice_id: u16) -> i32 {
    let mut err: i32;

    local_irq_enable();
    if snd_BUG_ON((subdevice_id & 0xfff0) != ECHO3G) != 0 {
        return -ENODEV;
    }

    err = init_dsp_comm_page(chip);
    if err != 0 {
        dev_err(
            (*(*chip).card).dev,
            b"init_hw - could not initialize DSP comm page\n\0".as_ptr() as *const _,
        );
        return err;
    }

    (*(*chip).comm_page).e3g_frq_register =
        cpu_to_le32((E3G_MAGIC_NUMBER / 48000) - 2);
    (*chip).device_id = device_id;
    (*chip).subdevice_id = subdevice_id;
    (*chip).bad_board = true;
    (*chip).has_midi = true;
    (*chip).dsp_code_to_load = FW_ECHO3G_DSP;

    /*
     * Load the DSP code and the ASIC on the PCI card and get
     * what type of external box is attached
     */
    err = load_firmware(chip);

    if err < 0 {
        return err;
    } else if err == E3G_GINA3G_BOX_TYPE {
        (*chip).input_clock_types =
            ECHO_CLOCK_BIT_INTERNAL | ECHO_CLOCK_BIT_SPDIF | ECHO_CLOCK_BIT_ADAT;
        (*chip).card_name = b"Gina3G\0".as_ptr() as *const _;
        (*chip).bx_digital_out = 6;
        (*chip).px_digital_out = (*chip).bx_digital_out;
        (*chip).bx_analog_in = 14;
        (*chip).px_analog_in = (*chip).bx_analog_in;
        (*chip).bx_digital_in = 16;
        (*chip).px_digital_in = (*chip).bx_digital_in;
        (*chip).bx_num = 24;
        (*chip).px_num = (*chip).bx_num;
        (*chip).has_phantom_power = true;
        (*chip).hasnt_input_nominal_level = true;
    } else if err == E3G_LAYLA3G_BOX_TYPE {
        (*chip).input_clock_types = ECHO_CLOCK_BIT_INTERNAL
            | ECHO_CLOCK_BIT_SPDIF
            | ECHO_CLOCK_BIT_ADAT
            | ECHO_CLOCK_BIT_WORD;
        (*chip).card_name = b"Layla3G\0".as_ptr() as *const _;
        (*chip).bx_digital_out = 8;
        (*chip).px_digital_out = (*chip).bx_digital_out;
        (*chip).bx_analog_in = 16;
        (*chip).px_analog_in = (*chip).bx_analog_in;
        (*chip).bx_digital_in = 24;
        (*chip).px_digital_in = (*chip).bx_digital_in;
        (*chip).bx_num = 32;
        (*chip).px_num = (*chip).bx_num;
    } else {
        return -ENODEV;
    }

    (*chip).digital_modes = ECHOCAPS_HAS_DIGITAL_MODE_SPDIF_RCA
        | ECHOCAPS_HAS_DIGITAL_MODE_SPDIF_OPTICAL
        | ECHOCAPS_HAS_DIGITAL_MODE_ADAT;

    err
}

unsafe fn set_mixer_defaults(chip: *mut echoaudio) -> i32 {
    (*chip).digital_mode = DIGITAL_MODE_SPDIF_RCA;
    (*chip).professional_spdif = false;
    (*chip).non_audio_spdif = false;
    (*chip).bad_board = false;
    (*chip).phantom_power = false;
    init_line_levels(chip)
}

unsafe fn set_phantom_power(chip: *mut echoaudio, on: core::ffi::c_char) -> i32 {
    let mut control_reg: u32 = le32_to_cpu((*(*chip).comm_page).control_register);

    if on != 0 {
        control_reg |= E3G_PHANTOM_POWER;
    } else {
        control_reg &= !E3G_PHANTOM_POWER;
    }

    (*chip).phantom_power = on;
    write_control_reg(
        chip,
        control_reg,
        le32_to_cpu((*(*chip).comm_page).e3g_frq_register),
        0,
    )
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
