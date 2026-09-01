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

extern "C" {
    fn write_control_reg(chip: *mut echoaudio, value: u32, force: i8) -> i32;
    fn set_input_clock(chip: *mut echoaudio, clock: u16) -> i32;
    fn set_professional_spdif(chip: *mut echoaudio, prof: i8) -> i32;
    fn set_digital_mode(chip: *mut echoaudio, mode: u8) -> i32;
    fn load_asic_generic(chip: *mut echoaudio, cmd: u32, asic: i16) -> i32;
    fn check_asic_status(chip: *mut echoaudio) -> i32;
}

unsafe fn init_hw(chip: *mut echoaudio, device_id: u16, subdevice_id: u16) -> i32 {
    let mut err: i32;

    if snd_BUG_ON(((subdevice_id as i32) & 0xfff0) != LAYLA24) {
        return -ENODEV;
    }

    err = init_dsp_comm_page(chip);
    if err != 0 {
        dev_err(
            (*(*chip).card).dev,
            c"init_hw - could not initialize DSP comm page\n".as_ptr(),
        );
        return err;
    }

    (*chip).device_id = device_id;
    (*chip).subdevice_id = subdevice_id;
    (*chip).bad_board = true;
    (*chip).has_midi = true;
    (*chip).dsp_code_to_load = FW_LAYLA24_DSP;
    (*chip).input_clock_types =
        ECHO_CLOCK_BIT_INTERNAL | ECHO_CLOCK_BIT_SPDIF | ECHO_CLOCK_BIT_WORD | ECHO_CLOCK_BIT_ADAT;
    (*chip).digital_modes = ECHOCAPS_HAS_DIGITAL_MODE_SPDIF_RCA
        | ECHOCAPS_HAS_DIGITAL_MODE_SPDIF_OPTICAL
        | ECHOCAPS_HAS_DIGITAL_MODE_ADAT;

    err = load_firmware(chip);
    if err < 0 {
        return err;
    }
    (*chip).bad_board = false;

    err = init_line_levels(chip);
    if err < 0 {
        return err;
    }

    err
}

unsafe fn set_mixer_defaults(chip: *mut echoaudio) -> i32 {
    (*chip).digital_mode = DIGITAL_MODE_SPDIF_RCA;
    (*chip).professional_spdif = false;
    (*chip).digital_in_automute = true;
    init_line_levels(chip)
}

unsafe fn detect_input_clocks(chip: *const echoaudio) -> u32 {
    let clocks_from_dsp: u32;
    let mut clock_bits: u32;

    /* Map the DSP clock detect bits to the generic driver clock detect bits */
    clocks_from_dsp = le32_to_cpu((*(*chip).comm_page).status_clocks);

    clock_bits = ECHO_CLOCK_BIT_INTERNAL;

    if (clocks_from_dsp & GML_CLOCK_DETECT_BIT_SPDIF) != 0 {
        clock_bits |= ECHO_CLOCK_BIT_SPDIF;
    }

    if (clocks_from_dsp & GML_CLOCK_DETECT_BIT_ADAT) != 0 {
        clock_bits |= ECHO_CLOCK_BIT_ADAT;
    }

    if (clocks_from_dsp & GML_CLOCK_DETECT_BIT_WORD) != 0 {
        clock_bits |= ECHO_CLOCK_BIT_WORD;
    }

    clock_bits
}

/* Layla24 has an ASIC on the PCI card and another ASIC in the external box;
both need to be loaded. */
unsafe fn load_asic(chip: *mut echoaudio) -> i32 {
    let mut err: i32;

    if (*chip).asic_loaded {
        return 1;
    }

    /* Give the DSP a few milliseconds to settle down */
    mdelay(10);

    /* Load the ASIC for the PCI card */
    err = load_asic_generic(
        chip,
        DSP_FNC_LOAD_LAYLA24_PCI_CARD_ASIC,
        FW_LAYLA24_1_ASIC,
    );
    if err < 0 {
        return err;
    }

    (*chip).asic_code = FW_LAYLA24_2S_ASIC;

    /* Now give the new ASIC a little time to set up */
    mdelay(10);

    /* Do the external one */
    err = load_asic_generic(
        chip,
        DSP_FNC_LOAD_LAYLA24_EXTERNAL_ASIC,
        FW_LAYLA24_2S_ASIC,
    );
    if err < 0 {
        return err;
    }

    /* Now give the external ASIC a little time to set up */
    mdelay(10);

    /* See if it worked */
    err = check_asic_status(chip);

    /* Set up the control register if the load succeeded -
       48 kHz, internal clock, S/PDIF RCA mode */
    if err == 0 {
        err = write_control_reg(chip, GML_CONVERTER_ENABLE | GML_48KHZ, true as i8);
    }

    err
}

unsafe fn set_sample_rate(chip: *mut echoaudio, rate: u32) -> i32 {
    let mut control_reg: u32;
    let mut clock: u32;
    let mut base_rate: u32;

    if snd_BUG_ON(rate >= 50000 && (*chip).digital_mode == DIGITAL_MODE_ADAT) {
        return -EINVAL;
    }

    /* Only set the clock for internal mode. */
    if (*chip).input_clock != ECHO_CLOCK_INTERNAL {
        dev_warn(
            (*(*chip).card).dev,
            c"Cannot set sample rate - clock not set to CLK_CLOCKININTERNAL\n".as_ptr(),
        );
        /* Save the rate anyhow */
        (*(*chip).comm_page).sample_rate = cpu_to_le32(rate);
        (*chip).sample_rate = rate;
        return 0;
    }

    /* Get the control register & clear the appropriate bits */
    control_reg = le32_to_cpu((*(*chip).comm_page).control_register);
    control_reg &= GML_CLOCK_CLEAR_MASK & GML_SPDIF_RATE_CLEAR_MASK;

    clock = 0;

    match rate {
        96000 => {
            clock = GML_96KHZ;
        }
        88200 => {
            clock = GML_88KHZ;
        }
        48000 => {
            clock = GML_48KHZ | GML_SPDIF_SAMPLE_RATE1;
        }
        44100 => {
            clock = GML_44KHZ;
            /* Professional mode */
            if (control_reg & GML_SPDIF_PRO_MODE) != 0 {
                clock |= GML_SPDIF_SAMPLE_RATE0;
            }
        }
        32000 => {
            clock = GML_32KHZ | GML_SPDIF_SAMPLE_RATE0 | GML_SPDIF_SAMPLE_RATE1;
        }
        22050 => {
            clock = GML_22KHZ;
        }
        16000 => {
            clock = GML_16KHZ;
        }
        11025 => {
            clock = GML_11KHZ;
        }
        8000 => {
            clock = GML_8KHZ;
        }
        _ => {
            /* If this is a non-standard rate, then the driver needs to
            use Layla24's special "continuous frequency" mode */
            clock = LAYLA24_CONTINUOUS_CLOCK;
            if rate > 50000 {
                base_rate = rate >> 1;
                control_reg |= GML_DOUBLE_SPEED_MODE;
            } else {
                base_rate = rate;
            }

            if base_rate < 25000 {
                base_rate = 25000;
            }

            if wait_handshake(chip) != 0 {
                return -EIO;
            }

            (*(*chip).comm_page).sample_rate =
                cpu_to_le32(LAYLA24_MAGIC_NUMBER / base_rate - 2);

            clear_handshake(chip);
            send_vector(chip, DSP_VC_SET_LAYLA24_FREQUENCY_REG);
        }
    }

    control_reg |= clock;

    (*(*chip).comm_page).sample_rate = cpu_to_le32(rate); /* ignored by the DSP ? */
    (*chip).sample_rate = rate;
    dev_dbg(
        (*(*chip).card).dev,
        c"set_sample_rate: %d clock %d\n".as_ptr(),
        rate,
        control_reg,
    );

    write_control_reg(chip, control_reg, false as i8)
}

unsafe fn set_input_clock(chip: *mut echoaudio, clock: u16) -> i32 {
    let mut control_reg: u32;
    let clocks_from_dsp: u32;

    /* Mask off the clock select bits */
    control_reg = le32_to_cpu((*(*chip).comm_page).control_register) & GML_CLOCK_CLEAR_MASK;
    clocks_from_dsp = le32_to_cpu((*(*chip).comm_page).status_clocks);

    /* Pick the new clock */
    match clock {
        ECHO_CLOCK_INTERNAL => {
            (*chip).input_clock = ECHO_CLOCK_INTERNAL;
            return set_sample_rate(chip, (*chip).sample_rate);
        }
        ECHO_CLOCK_SPDIF => {
            if (*chip).digital_mode == DIGITAL_MODE_ADAT {
                return -EAGAIN;
            }
            control_reg |= GML_SPDIF_CLOCK;
            /* Layla24 doesn't support 96KHz S/PDIF */
            control_reg &= !GML_DOUBLE_SPEED_MODE;
        }
        ECHO_CLOCK_WORD => {
            control_reg |= GML_WORD_CLOCK;
            if (clocks_from_dsp & GML_CLOCK_DETECT_BIT_WORD96) != 0 {
                control_reg |= GML_DOUBLE_SPEED_MODE;
            } else {
                control_reg &= !GML_DOUBLE_SPEED_MODE;
            }
        }
        ECHO_CLOCK_ADAT => {
            if (*chip).digital_mode != DIGITAL_MODE_ADAT {
                return -EAGAIN;
            }
            control_reg |= GML_ADAT_CLOCK;
            control_reg &= !GML_DOUBLE_SPEED_MODE;
        }
        _ => {
            dev_err(
                (*(*chip).card).dev,
                c"Input clock 0x%x not supported for Layla24\n".as_ptr(),
                clock as i32,
            );
            return -EINVAL;
        }
    }

    (*chip).input_clock = clock;
    write_control_reg(chip, control_reg, true as i8)
}

/* Depending on what digital mode you want, Layla24 needs different ASICs
loaded.  This function checks the ASIC needed for the new mode and sees
if it matches the one already loaded. */
unsafe fn switch_asic(chip: *mut echoaudio, asic: i16) -> i32 {
    let monitors: *mut s8;

    /*  Check to see if this is already loaded */
    if asic != (*chip).asic_code {
        monitors = kmemdup(
            (*(*chip).comm_page).monitors.as_mut_ptr() as *const _,
            MONITOR_ARRAY_SIZE,
            GFP_KERNEL,
        ) as *mut s8;
        if monitors.is_null() {
            return -ENOMEM;
        }

        memset(
            (*(*chip).comm_page).monitors.as_mut_ptr() as *mut _,
            ECHOGAIN_MUTED,
            MONITOR_ARRAY_SIZE,
        );

        /* Load the desired ASIC */
        if load_asic_generic(chip, DSP_FNC_LOAD_LAYLA24_EXTERNAL_ASIC, asic) < 0 {
            memcpy(
                (*(*chip).comm_page).monitors.as_mut_ptr() as *mut _,
                monitors as *const _,
                MONITOR_ARRAY_SIZE,
            );
            kfree(monitors as *const _);
            return -EIO;
        }
        (*chip).asic_code = asic;
        memcpy(
            (*(*chip).comm_page).monitors.as_mut_ptr() as *mut _,
            monitors as *const _,
            MONITOR_ARRAY_SIZE,
        );
        kfree(monitors as *const _);
    }

    0
}

unsafe fn dsp_set_digital_mode(chip: *mut echoaudio, mode: u8) -> i32 {
    let mut control_reg: u32;
    let err: i32;
    let mut incompatible_clock: i32;
    let asic: i16;

    /* Set clock to "internal" if it's not compatible with the new mode */
    incompatible_clock = false as i32;
    match mode {
        DIGITAL_MODE_SPDIF_OPTICAL | DIGITAL_MODE_SPDIF_RCA => {
            if (*chip).input_clock == ECHO_CLOCK_ADAT {
                incompatible_clock = true as i32;
            }
            asic = FW_LAYLA24_2S_ASIC;
        }
        DIGITAL_MODE_ADAT => {
            if (*chip).input_clock == ECHO_CLOCK_SPDIF {
                incompatible_clock = true as i32;
            }
            asic = FW_LAYLA24_2A_ASIC;
        }
        _ => {
            dev_err(
                (*(*chip).card).dev,
                c"Digital mode not supported: %d\n".as_ptr(),
                mode as i32,
            );
            return -EINVAL;
        }
    }

    if incompatible_clock != 0 {
        /* Switch to 48KHz, internal */
        (*chip).sample_rate = 48000;
        guard_spinlock_irq(&mut (*chip).lock);
        set_input_clock(chip, ECHO_CLOCK_INTERNAL);
    }

    /* switch_asic() can sleep */
    if switch_asic(chip, asic) < 0 {
        return -EIO;
    }

    guard_spinlock_irq(&mut (*chip).lock);

    /* Tweak the control register */
    control_reg = le32_to_cpu((*(*chip).comm_page).control_register);
    control_reg &= GML_DIGITAL_MODE_CLEAR_MASK;

    match mode {
        DIGITAL_MODE_SPDIF_OPTICAL => {
            control_reg |= GML_SPDIF_OPTICAL_MODE;
        }
        DIGITAL_MODE_SPDIF_RCA => {
            /* GML_SPDIF_OPTICAL_MODE bit cleared */
        }
        DIGITAL_MODE_ADAT => {
            control_reg |= GML_ADAT_MODE;
            control_reg &= !GML_DOUBLE_SPEED_MODE;
        }
        _ => {}
    }

    err = write_control_reg(chip, control_reg, true as i8);
    if err < 0 {
        return err;
    }
    (*chip).digital_mode = mode;

    dev_dbg(
        (*(*chip).card).dev,
        c"set_digital_mode to %d\n".as_ptr(),
        mode as i32,
    );
    incompatible_clock
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
