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



/* These functions are common for all "3G" cards */


unsafe fn check_asic_status(chip: *mut echoaudio) -> c_int {
    let mut box_status: u32;

    if wait_handshake(chip) != 0 {
        return -EIO;
    }

    (*(*chip).comm_page).ext_box_status = cpu_to_le32(E3G_ASIC_NOT_LOADED);
    (*chip).asic_loaded = false;
    clear_handshake(chip);
    send_vector(chip, DSP_VC_TEST_ASIC);

    if wait_handshake(chip) != 0 {
        (*chip).dsp_code = core::ptr::null_mut();
        return -EIO;
    }

    box_status = le32_to_cpu((*(*chip).comm_page).ext_box_status);
    dev_dbg((*(*chip).card).dev, c"box_status=%x\n".as_ptr(), box_status);
    if box_status == E3G_ASIC_NOT_LOADED {
        return -ENODEV;
    }

    (*chip).asic_loaded = true;
    (box_status & E3G_BOX_TYPE_MASK) as c_int
}



#[inline]
unsafe fn get_frq_reg(chip: *mut echoaudio) -> u32 {
    le32_to_cpu((*(*chip).comm_page).e3g_frq_register)
}



/* Most configuration of 3G cards is accomplished by writing the control
register. write_control_reg sends the new control register value to the DSP. */
unsafe fn write_control_reg(chip: *mut echoaudio, ctl: u32, frq: u32, force: c_char) -> c_int {
    let ctl_reg: __le32;
    let frq_reg: __le32;

    if wait_handshake(chip) != 0 {
        return -EIO;
    }

    dev_dbg(
        (*(*chip).card).dev,
        c"WriteControlReg: Setting 0x%x, 0x%x\n".as_ptr(),
        ctl,
        frq,
    );

    ctl_reg = cpu_to_le32(ctl);
    frq_reg = cpu_to_le32(frq);

    if ctl_reg != (*(*chip).comm_page).control_register ||
        frq_reg != (*(*chip).comm_page).e3g_frq_register ||
        force != 0
    {
        (*(*chip).comm_page).e3g_frq_register = frq_reg;
        (*(*chip).comm_page).control_register = ctl_reg;
        clear_handshake(chip);
        return send_vector(chip, DSP_VC_WRITE_CONTROL_REG);
    }

    dev_dbg((*(*chip).card).dev, c"WriteControlReg: not written, no change\n".as_ptr());
    0
}



/* Set the digital mode - currently for Gina24, Layla24, Mona, 3G */
unsafe fn set_digital_mode(chip: *mut echoaudio, mode: u8) -> c_int {
    let previous_mode: u8;
    let err: c_int;
    let mut i: c_int;
    let mut o: c_int;

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
     * then make sure all output, input and monitor levels are
     * updated by the DSP comm object. */
    if err >= 0 && previous_mode != mode &&
        (previous_mode == DIGITAL_MODE_ADAT || mode == DIGITAL_MODE_ADAT)
    {
        /* C source uses guard(spinlock_irq)(&chip->lock); */
        let _guard = spinlock_irq_guard(&mut (*chip).lock);
        o = 0;
        while o < num_busses_out(chip) {
            i = 0;
            while i < num_busses_in(chip) {
                set_monitor_gain(chip, o, i, (*chip).monitor_gain[o as usize][i as usize]);
                i += 1;
            }
            o += 1;
        }

        /* If ECHOCARD_HAS_INPUT_GAIN is enabled in the card build:
         * for (i = 0; i < num_busses_in(chip); i++)
         *     set_input_gain(chip, i, chip->input_gain[i]);
         * update_input_line_level(chip);
         */
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
    }

    err
}



unsafe fn set_spdif_bits(chip: *mut echoaudio, mut control_reg: u32, rate: u32) -> u32 {
    control_reg &= E3G_SPDIF_FORMAT_CLEAR_MASK;

    match rate {
        32000 => {
            control_reg |= E3G_SPDIF_SAMPLE_RATE0 | E3G_SPDIF_SAMPLE_RATE1;
        }
        44100 => {
            if (*chip).professional_spdif != 0 {
                control_reg |= E3G_SPDIF_SAMPLE_RATE0;
            }
        }
        48000 => {
            control_reg |= E3G_SPDIF_SAMPLE_RATE1;
        }
        _ => {}
    }

    if (*chip).professional_spdif != 0 {
        control_reg |= E3G_SPDIF_PRO_MODE;
    }

    if (*chip).non_audio_spdif != 0 {
        control_reg |= E3G_SPDIF_NOT_AUDIO;
    }

    control_reg |= E3G_SPDIF_24_BIT | E3G_SPDIF_TWO_CHANNEL |
        E3G_SPDIF_COPY_PERMIT;

    control_reg
}



/* Set the S/PDIF output format */
unsafe fn set_professional_spdif(chip: *mut echoaudio, prof: c_char) -> c_int {
    let mut control_reg: u32;

    control_reg = le32_to_cpu((*(*chip).comm_page).control_register);
    (*chip).professional_spdif = prof;
    control_reg = set_spdif_bits(chip, control_reg, (*chip).sample_rate);
    write_control_reg(chip, control_reg, get_frq_reg(chip), 0)
}



/* detect_input_clocks() returns a bitmask consisting of all the input clocks
currently connected to the hardware; this changes as the user connects and
disconnects clock inputs. You should use this information to determine which
clocks the user is allowed to select. */
unsafe fn detect_input_clocks(chip: *const echoaudio) -> u32 {
    let clocks_from_dsp: u32;
    let mut clock_bits: u32;

    /* Map the DSP clock detect bits to the generic driver clock
     * detect bits */
    clocks_from_dsp = le32_to_cpu((*(*chip).comm_page).status_clocks);

    clock_bits = ECHO_CLOCK_BIT_INTERNAL;

    if clocks_from_dsp & E3G_CLOCK_DETECT_BIT_WORD != 0 {
        clock_bits |= ECHO_CLOCK_BIT_WORD;
    }

    match (*chip).digital_mode {
        DIGITAL_MODE_SPDIF_RCA | DIGITAL_MODE_SPDIF_OPTICAL => {
            if clocks_from_dsp & E3G_CLOCK_DETECT_BIT_SPDIF != 0 {
                clock_bits |= ECHO_CLOCK_BIT_SPDIF;
            }
        }
        DIGITAL_MODE_ADAT => {
            if clocks_from_dsp & E3G_CLOCK_DETECT_BIT_ADAT != 0 {
                clock_bits |= ECHO_CLOCK_BIT_ADAT;
            }
        }
        _ => {}
    }

    clock_bits
}



unsafe fn load_asic(chip: *mut echoaudio) -> c_int {
    let box_type: c_int;
    let mut err: c_int;

    if (*chip).asic_loaded {
        return 0;
    }

    /* Give the DSP a few milliseconds to settle down */
    mdelay(2);

    err = load_asic_generic(chip, DSP_FNC_LOAD_3G_ASIC, FW_3G_ASIC);
    if err < 0 {
        return err;
    }

    (*chip).asic_code = FW_3G_ASIC;

    /* Now give the new ASIC some time to set up */
    msleep(1000);
    /* See if it worked */
    box_type = check_asic_status(chip);

    /* Set up the control register if the load succeeded -
     * 48 kHz, internal clock, S/PDIF RCA mode */
    if box_type >= 0 {
        err = write_control_reg(chip, E3G_48KHZ, E3G_FREQ_REG_DEFAULT, true as c_char);
        if err < 0 {
            return err;
        }
    }

    box_type
}


unsafe fn set_sample_rate(chip: *mut echoaudio, rate: u32) -> c_int {
    let mut control_reg: u32;
    let mut clock: u32;
    let mut base_rate: u32;
    let mut frq_reg: u32;

    /* Only set the clock for internal mode. */
    if (*chip).input_clock != ECHO_CLOCK_INTERNAL {
        dev_warn(
            (*(*chip).card).dev,
            c"Cannot set sample rate - clock not set to CLK_CLOCKININTERNAL\n".as_ptr(),
        );
        /* Save the rate anyhow */
        (*(*chip).comm_page).sample_rate = cpu_to_le32(rate);
        (*chip).sample_rate = rate;
        set_input_clock(chip, (*chip).input_clock);
        return 0;
    }

    if snd_BUG_ON(rate >= 50000 && (*chip).digital_mode == DIGITAL_MODE_ADAT) != 0 {
        return -EINVAL;
    }

    control_reg = le32_to_cpu((*(*chip).comm_page).control_register);
    control_reg &= E3G_CLOCK_CLEAR_MASK;

    match rate {
        96000 => {
            clock = E3G_96KHZ;
        }
        88200 => {
            clock = E3G_88KHZ;
        }
        48000 => {
            clock = E3G_48KHZ;
        }
        44100 => {
            clock = E3G_44KHZ;
        }
        32000 => {
            clock = E3G_32KHZ;
        }
        _ => {
            clock = E3G_CONTINUOUS_CLOCK;
            if rate > 50000 {
                clock |= E3G_DOUBLE_SPEED_MODE;
            }
        }
    }

    control_reg |= clock;
    control_reg = set_spdif_bits(chip, control_reg, rate);

    base_rate = rate;
    if base_rate > 50000 {
        base_rate /= 2;
    }
    if base_rate < 32000 {
        base_rate = 32000;
    }

    frq_reg = E3G_MAGIC_NUMBER / base_rate - 2;
    if frq_reg > E3G_FREQ_REG_MAX {
        frq_reg = E3G_FREQ_REG_MAX;
    }

    (*(*chip).comm_page).sample_rate = cpu_to_le32(rate); /* ignored by the DSP */
    (*chip).sample_rate = rate;
    dev_dbg(
        (*(*chip).card).dev,
        c"SetSampleRate: %d clock %x\n".as_ptr(),
        rate,
        control_reg,
    );

    /* Tell the DSP about it - DSP reads both control reg & freq reg */
    write_control_reg(chip, control_reg, frq_reg, 0)
}



/* Set the sample clock source to internal, S/PDIF, ADAT */
unsafe fn set_input_clock(chip: *mut echoaudio, clock: u16) -> c_int {
    let mut control_reg: u32;
    let clocks_from_dsp: u32;


    /* Mask off the clock select bits */
    control_reg = le32_to_cpu((*(*chip).comm_page).control_register) &
        E3G_CLOCK_CLEAR_MASK;
    clocks_from_dsp = le32_to_cpu((*(*chip).comm_page).status_clocks);

    match clock {
        ECHO_CLOCK_INTERNAL => {
            (*chip).input_clock = ECHO_CLOCK_INTERNAL;
            return set_sample_rate(chip, (*chip).sample_rate);
        }
        ECHO_CLOCK_SPDIF => {
            if (*chip).digital_mode == DIGITAL_MODE_ADAT {
                return -EAGAIN;
            }
            control_reg |= E3G_SPDIF_CLOCK;
            if clocks_from_dsp & E3G_CLOCK_DETECT_BIT_SPDIF96 != 0 {
                control_reg |= E3G_DOUBLE_SPEED_MODE;
            } else {
                control_reg &= !E3G_DOUBLE_SPEED_MODE;
            }
        }
        ECHO_CLOCK_ADAT => {
            if (*chip).digital_mode != DIGITAL_MODE_ADAT {
                return -EAGAIN;
            }
            control_reg |= E3G_ADAT_CLOCK;
            control_reg &= !E3G_DOUBLE_SPEED_MODE;
        }
        ECHO_CLOCK_WORD => {
            control_reg |= E3G_WORD_CLOCK;
            if clocks_from_dsp & E3G_CLOCK_DETECT_BIT_WORD96 != 0 {
                control_reg |= E3G_DOUBLE_SPEED_MODE;
            } else {
                control_reg &= !E3G_DOUBLE_SPEED_MODE;
            }
        }
        _ => {
            dev_err(
                (*(*chip).card).dev,
                c"Input clock 0x%x not supported for Echo3G\n".as_ptr(),
                clock,
            );
            return -EINVAL;
        }
    }

    (*chip).input_clock = clock;
    write_control_reg(chip, control_reg, get_frq_reg(chip), 1)
}



unsafe fn dsp_set_digital_mode(chip: *mut echoaudio, mode: u8) -> c_int {
    let mut control_reg: u32;
    let err: c_int;
    let mut incompatible_clock: c_int;

    /* Set clock to "internal" if it's not compatible with the new mode */
    incompatible_clock = false as c_int;
    match mode {
        DIGITAL_MODE_SPDIF_OPTICAL | DIGITAL_MODE_SPDIF_RCA => {
            if (*chip).input_clock == ECHO_CLOCK_ADAT {
                incompatible_clock = true as c_int;
            }
        }
        DIGITAL_MODE_ADAT => {
            if (*chip).input_clock == ECHO_CLOCK_SPDIF {
                incompatible_clock = true as c_int;
            }
        }
        _ => {
            dev_err(
                (*(*chip).card).dev,
                c"Digital mode not supported: %d\n".as_ptr(),
                mode,
            );
            return -EINVAL;
        }
    }

    /* C source uses guard(spinlock_irq)(&chip->lock); */
    let _guard = spinlock_irq_guard(&mut (*chip).lock);

    if incompatible_clock != 0 {
        (*chip).sample_rate = 48000;
        set_input_clock(chip, ECHO_CLOCK_INTERNAL);
    }

    /* Clear the current digital mode */
    control_reg = le32_to_cpu((*(*chip).comm_page).control_register);
    control_reg &= E3G_DIGITAL_MODE_CLEAR_MASK;

    /* Tweak the control reg */
    match mode {
        DIGITAL_MODE_SPDIF_OPTICAL => {
            control_reg |= E3G_SPDIF_OPTICAL_MODE;
        }
        DIGITAL_MODE_SPDIF_RCA => {
            /* E3G_SPDIF_OPTICAL_MODE bit cleared */
        }
        DIGITAL_MODE_ADAT => {
            control_reg |= E3G_ADAT_MODE;
            control_reg &= !E3G_DOUBLE_SPEED_MODE; /* @@ useless */
        }
        _ => {}
    }

    err = write_control_reg(chip, control_reg, get_frq_reg(chip), 1);
    if err < 0 {
        return err;
    }
    (*chip).digital_mode = mode;

    dev_dbg((*(*chip).card).dev, c"set_digital_mode(%d)\n".as_ptr(), (*chip).digital_mode);
    incompatible_clock
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
