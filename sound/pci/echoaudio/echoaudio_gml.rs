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


/* These functions are common for Gina24, Layla24 and Mona cards */


/* ASIC status check - some cards have one or two ASICs that need to be
loaded.  Once that load is complete, this function is called to see if
the load was successful.
If this load fails, it does not necessarily mean that the hardware is
defective - the external box may be disconnected or turned off. */
unsafe fn check_asic_status(chip: *mut echoaudio) -> i32 {
    let mut asic_status: u32 = 0;

    send_vector(chip, DSP_VC_TEST_ASIC);

    /* The DSP will return a value to indicate whether or not the
       ASIC is currently loaded */
    if read_dsp(chip, &mut asic_status) < 0 {
        dev_err(
            (*(*chip).card).dev,
            "check_asic_status: failed on read_dsp\n",
        );
        (*chip).asic_loaded = false;
        return -EIO;
    }

    (*chip).asic_loaded = asic_status == ASIC_ALREADY_LOADED;
    if (*chip).asic_loaded {
        0
    } else {
        -EIO
    }
}



/* Most configuration of Gina24, Layla24, or Mona is accomplished by writing
the control register.  write_control_reg sends the new control register
value to the DSP. */
unsafe fn write_control_reg(chip: *mut echoaudio, mut value: u32, force: i8) -> i32 {
    let reg_value: __le32;

    /* Handle the digital input auto-mute */
    if (*chip).digital_in_automute != 0 {
        value |= GML_DIGITAL_IN_AUTO_MUTE;
    } else {
        value &= !GML_DIGITAL_IN_AUTO_MUTE;
    }

    dev_dbg((*(*chip).card).dev, "write_control_reg: 0x%x\n", value);

    /* Write the control register */
    reg_value = cpu_to_le32(value);
    if reg_value != (*(*chip).comm_page).control_register || force != 0 {
        if wait_handshake(chip) != 0 {
            return -EIO;
        }
        (*(*chip).comm_page).control_register = reg_value;
        clear_handshake(chip);
        return send_vector(chip, DSP_VC_WRITE_CONTROL_REG);
    }
    0
}



/* Gina24, Layla24, and Mona support digital input auto-mute.  If the digital
input auto-mute is enabled, the DSP will only enable the digital inputs if
the card is syncing to a valid clock on the ADAT or S/PDIF inputs.
If the auto-mute is disabled, the digital inputs are enabled regardless of
what the input clock is set or what is connected. */
unsafe fn set_input_auto_mute(chip: *mut echoaudio, automute: i32) -> i32 {
    dev_dbg((*(*chip).card).dev, "set_input_auto_mute %d\n", automute);

    (*chip).digital_in_automute = automute;

    /* Re-set the input clock to the current value - indirectly causes
    the auto-mute flag to be sent to the DSP */
    set_input_clock(chip, (*chip).input_clock)
}



/* S/PDIF coax / S/PDIF optical / ADAT - switch */
unsafe fn set_digital_mode(chip: *mut echoaudio, mode: u8) -> i32 {
    let previous_mode: u8;
    let err: i32;
    let mut i: i32;
    let mut o: i32;

    if (*chip).bad_board {
        return -EIO;
    }

    /* All audio channels must be closed before changing the digital mode */
    if snd_BUG_ON((*chip).pipe_alloc_mask) != 0 {
        return -EAGAIN;
    }

    if snd_BUG_ON(((*chip).digital_modes & (1 << mode)) == 0) != 0 {
        return -EINVAL;
    }

    previous_mode = (*chip).digital_mode;
    err = dsp_set_digital_mode(chip, mode);

    /* If we successfully changed the digital mode from or to ADAT,
       then make sure all output, input and monitor levels are
       updated by the DSP comm object. */
    if err >= 0
        && previous_mode != mode
        && (previous_mode == DIGITAL_MODE_ADAT || mode == DIGITAL_MODE_ADAT)
    {
        spin_lock_irq(&mut (*chip).lock);
        o = 0;
        while o < num_busses_out(chip) {
            i = 0;
            while i < num_busses_in(chip) {
                set_monitor_gain(
                    chip,
                    o,
                    i,
                    (*chip).monitor_gain[o as usize][i as usize],
                );
                i += 1;
            }
            o += 1;
        }

        #[cfg(ECHOCARD_HAS_INPUT_GAIN)]
        {
            i = 0;
            while i < num_busses_in(chip) {
                set_input_gain(chip, i, (*chip).input_gain[i as usize]);
                i += 1;
            }
            update_input_line_level(chip);
        }

        o = 0;
        while o < num_busses_out(chip) {
            set_output_gain(chip, o, (*chip).output_gain[o as usize]);
            o += 1;
        }
        update_output_line_level(chip);
        spin_unlock_irq(&mut (*chip).lock);
    }

    err
}



/* Set the S/PDIF output format */
unsafe fn set_professional_spdif(chip: *mut echoaudio, prof: i8) -> i32 {
    let mut control_reg: u32;
    let err: i32;

    /* Clear the current S/PDIF flags */
    control_reg = le32_to_cpu((*(*chip).comm_page).control_register);
    control_reg &= GML_SPDIF_FORMAT_CLEAR_MASK;

    /* Set the new S/PDIF flags depending on the mode */
    control_reg |= GML_SPDIF_TWO_CHANNEL | GML_SPDIF_24_BIT | GML_SPDIF_COPY_PERMIT;
    if prof != 0 {
        /* Professional mode */
        control_reg |= GML_SPDIF_PRO_MODE;

        match (*chip).sample_rate {
            32000 => {
                control_reg |= GML_SPDIF_SAMPLE_RATE0 | GML_SPDIF_SAMPLE_RATE1;
            }
            44100 => {
                control_reg |= GML_SPDIF_SAMPLE_RATE0;
            }
            48000 => {
                control_reg |= GML_SPDIF_SAMPLE_RATE1;
            }
            _ => {}
        }
    } else {
        /* Consumer mode */
        match (*chip).sample_rate {
            32000 => {
                control_reg |= GML_SPDIF_SAMPLE_RATE0 | GML_SPDIF_SAMPLE_RATE1;
            }
            48000 => {
                control_reg |= GML_SPDIF_SAMPLE_RATE1;
            }
            _ => {}
        }
    }

    err = write_control_reg(chip, control_reg, false as i8);
    if err != 0 {
        return err;
    }
    (*chip).professional_spdif = prof;
    dev_dbg(
        (*(*chip).card).dev,
        "set_professional_spdif to %s\n",
        if prof != 0 { "Professional" } else { "Consumer" },
    );
    0
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
