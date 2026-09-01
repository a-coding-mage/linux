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

unsafe extern "C" {
    fn write_control_reg(chip: *mut echoaudio, value: u32, force: i8) -> i32;
    fn set_input_clock(chip: *mut echoaudio, clock: u16) -> i32;
    fn set_professional_spdif(chip: *mut echoaudio, prof: i8) -> i32;
    fn set_digital_mode(chip: *mut echoaudio, mode: u8) -> i32;
    fn load_asic_generic(chip: *mut echoaudio, cmd: u32, asic: i16) -> i32;
    fn check_asic_status(chip: *mut echoaudio) -> i32;
}

unsafe fn init_hw(chip: *mut echoaudio, device_id: u16, subdevice_id: u16) -> i32 {
    let mut err: i32;

    if snd_BUG_ON!((subdevice_id & 0xfff0) != MONA) {
        return -ENODEV;
    }

    err = init_dsp_comm_page(chip);
    if err != 0 {
        dev_err!(
            (*(*chip).card).dev,
            "init_hw - could not initialize DSP comm page\n"
        );
        return err;
    }

    (*chip).device_id = device_id;
    (*chip).subdevice_id = subdevice_id;
    (*chip).bad_board = true;
    (*chip).input_clock_types = ECHO_CLOCK_BIT_INTERNAL
        | ECHO_CLOCK_BIT_SPDIF
        | ECHO_CLOCK_BIT_WORD
        | ECHO_CLOCK_BIT_ADAT;
    (*chip).digital_modes = ECHOCAPS_HAS_DIGITAL_MODE_SPDIF_RCA
        | ECHOCAPS_HAS_DIGITAL_MODE_SPDIF_OPTICAL
        | ECHOCAPS_HAS_DIGITAL_MODE_ADAT;

    /* Mona comes in both '301 and '361 flavors */
    if (*chip).device_id == DEVICE_ID_56361 {
        (*chip).dsp_code_to_load = FW_MONA_361_DSP;
    } else {
        (*chip).dsp_code_to_load = FW_MONA_301_DSP;
    }

    err = load_firmware(chip);
    if err < 0 {
        return err;
    }
    (*chip).bad_board = false;

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

    /* Map the DSP clock detect bits to the generic driver clock
       detect bits */
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

/* Mona has an ASIC on the PCI card and another ASIC in the external box;
both need to be loaded. */
unsafe fn load_asic(chip: *mut echoaudio) -> i32 {
    let mut control_reg: u32;
    let mut err: i32;
    let asic: i16;

    if (*chip).asic_loaded {
        return 0;
    }

    mdelay(10);

    if (*chip).device_id == DEVICE_ID_56361 {
        asic = FW_MONA_361_1_ASIC48;
    } else {
        asic = FW_MONA_301_1_ASIC48;
    }

    err = load_asic_generic(chip, DSP_FNC_LOAD_MONA_PCI_CARD_ASIC, asic);
    if err < 0 {
        return err;
    }

    (*chip).asic_code = asic;
    mdelay(10);

    /* Do the external one */
    err = load_asic_generic(chip, DSP_FNC_LOAD_MONA_EXTERNAL_ASIC, FW_MONA_2_ASIC);
    if err < 0 {
        return err;
    }

    mdelay(10);
    err = check_asic_status(chip);

    /* Set up the control register if the load succeeded -
       48 kHz, internal clock, S/PDIF RCA mode */
    if err == 0 {
        control_reg = GML_CONVERTER_ENABLE | GML_48KHZ;
        err = write_control_reg(chip, control_reg, true as i8);
    }

    err
}

/* Depending on what digital mode you want, Mona needs different ASICs
loaded.  This function checks the ASIC needed for the new mode and sees
if it matches the one already loaded. */
unsafe fn switch_asic(chip: *mut echoaudio, double_speed: i8) -> i32 {
    let err: i32;
    let asic: i16;

    /* Check the clock detect bits to see if this is
    a single-speed clock or a double-speed clock; load
    a new ASIC if necessary. */
    if (*chip).device_id == DEVICE_ID_56361 {
        if double_speed != 0 {
            asic = FW_MONA_361_1_ASIC96;
        } else {
            asic = FW_MONA_361_1_ASIC48;
        }
    } else if double_speed != 0 {
        asic = FW_MONA_301_1_ASIC96;
    } else {
        asic = FW_MONA_301_1_ASIC48;
    }

    if asic != (*chip).asic_code {
        /* Load the desired ASIC */
        err = load_asic_generic(chip, DSP_FNC_LOAD_MONA_PCI_CARD_ASIC, asic);
        if err < 0 {
            return err;
        }
        (*chip).asic_code = asic;
    }

    0
}

unsafe fn set_sample_rate(chip: *mut echoaudio, rate: u32) -> i32 {
    let mut control_reg: u32;
    let mut clock: u32;
    let asic: i16;
    let mut force_write: i8;

    /* Only set the clock for internal mode. */
    if (*chip).input_clock != ECHO_CLOCK_INTERNAL {
        dev_dbg!(
            (*(*chip).card).dev,
            "Cannot set sample rate - clock not set to CLK_CLOCKININTERNAL\n"
        );
        /* Save the rate anyhow */
        (*(*chip).comm_page).sample_rate = cpu_to_le32(rate);
        (*chip).sample_rate = rate;
        return 0;
    }

    /* Now, check to see if the required ASIC is loaded */
    if rate >= 88200 {
        if (*chip).digital_mode == DIGITAL_MODE_ADAT {
            return -EINVAL;
        }
        if (*chip).device_id == DEVICE_ID_56361 {
            asic = FW_MONA_361_1_ASIC96;
        } else {
            asic = FW_MONA_301_1_ASIC96;
        }
    } else if (*chip).device_id == DEVICE_ID_56361 {
        asic = FW_MONA_361_1_ASIC48;
    } else {
        asic = FW_MONA_301_1_ASIC48;
    }

    force_write = 0;
    if asic != (*chip).asic_code {
        let err: i32;
        /* Load the desired ASIC (load_asic_generic() can sleep) */
        spin_unlock_irq(&mut (*chip).lock);
        err = load_asic_generic(chip, DSP_FNC_LOAD_MONA_PCI_CARD_ASIC, asic);
        spin_lock_irq(&mut (*chip).lock);

        if err < 0 {
            return err;
        }
        (*chip).asic_code = asic;
        force_write = 1;
    }

    /* Compute the new control register value */
    clock = 0;
    control_reg = le32_to_cpu((*(*chip).comm_page).control_register);
    control_reg &= GML_CLOCK_CLEAR_MASK;
    control_reg &= GML_SPDIF_RATE_CLEAR_MASK;

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
            dev_err!((*(*chip).card).dev, "set_sample_rate: %d invalid!\n", rate);
            return -EINVAL;
        }
    }

    control_reg |= clock;

    (*(*chip).comm_page).sample_rate = cpu_to_le32(rate); /* ignored by the DSP */
    (*chip).sample_rate = rate;
    dev_dbg!(
        (*(*chip).card).dev,
        "set_sample_rate: %d clock %d\n",
        rate,
        clock
    );

    write_control_reg(chip, control_reg, force_write)
}

unsafe fn set_input_clock(chip: *mut echoaudio, clock: u16) -> i32 {
    let mut control_reg: u32;
    let clocks_from_dsp: u32;
    let mut err: i32;

    /* Mask off the clock select bits */
    control_reg = le32_to_cpu((*(*chip).comm_page).control_register) & GML_CLOCK_CLEAR_MASK;
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
            spin_unlock_irq(&mut (*chip).lock);
            err = switch_asic(
                chip,
                (clocks_from_dsp & GML_CLOCK_DETECT_BIT_SPDIF96) as i8,
            );
            spin_lock_irq(&mut (*chip).lock);
            if err < 0 {
                return err;
            }
            control_reg |= GML_SPDIF_CLOCK;
            if (clocks_from_dsp & GML_CLOCK_DETECT_BIT_SPDIF96) != 0 {
                control_reg |= GML_DOUBLE_SPEED_MODE;
            } else {
                control_reg &= !GML_DOUBLE_SPEED_MODE;
            }
        }
        ECHO_CLOCK_WORD => {
            spin_unlock_irq(&mut (*chip).lock);
            err = switch_asic(
                chip,
                (clocks_from_dsp & GML_CLOCK_DETECT_BIT_WORD96) as i8,
            );
            spin_lock_irq(&mut (*chip).lock);
            if err < 0 {
                return err;
            }
            control_reg |= GML_WORD_CLOCK;
            if (clocks_from_dsp & GML_CLOCK_DETECT_BIT_WORD96) != 0 {
                control_reg |= GML_DOUBLE_SPEED_MODE;
            } else {
                control_reg &= !GML_DOUBLE_SPEED_MODE;
            }
        }
        ECHO_CLOCK_ADAT => {
            dev_dbg!((*(*chip).card).dev, "Set Mona clock to ADAT\n");
            if (*chip).digital_mode != DIGITAL_MODE_ADAT {
                return -EAGAIN;
            }
            control_reg |= GML_ADAT_CLOCK;
            control_reg &= !GML_DOUBLE_SPEED_MODE;
        }
        _ => {
            dev_err!(
                (*(*chip).card).dev,
                "Input clock 0x%x not supported for Mona\n",
                clock
            );
            return -EINVAL;
        }
    }

    (*chip).input_clock = clock;
    write_control_reg(chip, control_reg, true as i8)
}

unsafe fn dsp_set_digital_mode(chip: *mut echoaudio, mode: u8) -> i32 {
    let mut control_reg: u32;
    let err: i32;
    let mut incompatible_clock: i32;

    /* Set clock to "internal" if it's not compatible with the new mode */
    incompatible_clock = false as i32;
    match mode {
        DIGITAL_MODE_SPDIF_OPTICAL | DIGITAL_MODE_SPDIF_RCA => {
            if (*chip).input_clock == ECHO_CLOCK_ADAT {
                incompatible_clock = true as i32;
            }
        }
        DIGITAL_MODE_ADAT => {
            if (*chip).input_clock == ECHO_CLOCK_SPDIF {
                incompatible_clock = true as i32;
            }
        }
        _ => {
            dev_err!(
                (*(*chip).card).dev,
                "Digital mode not supported: %d\n",
                mode
            );
            return -EINVAL;
        }
    }

    let _guard = guard_spinlock_irq(&mut (*chip).lock);

    if incompatible_clock != 0 {
        /* Switch to 48KHz, internal */
        (*chip).sample_rate = 48000;
        set_input_clock(chip, ECHO_CLOCK_INTERNAL);
    }

    /* Clear the current digital mode */
    control_reg = le32_to_cpu((*(*chip).comm_page).control_register);
    control_reg &= GML_DIGITAL_MODE_CLEAR_MASK;

    /* Tweak the control reg */
    match mode {
        DIGITAL_MODE_SPDIF_OPTICAL => {
            control_reg |= GML_SPDIF_OPTICAL_MODE;
        }
        DIGITAL_MODE_SPDIF_RCA => {
            /* GML_SPDIF_OPTICAL_MODE bit cleared */
        }
        DIGITAL_MODE_ADAT => {
            /* If the current ASIC is the 96KHz ASIC, switch the ASIC
               and set to 48 KHz */
            if (*chip).asic_code == FW_MONA_361_1_ASIC96
                || (*chip).asic_code == FW_MONA_301_1_ASIC96
            {
                set_sample_rate(chip, 48000);
            }
            control_reg |= GML_ADAT_MODE;
            control_reg &= !GML_DOUBLE_SPEED_MODE;
        }
        _ => {}
    }

    err = write_control_reg(chip, control_reg, false as i8);
    if err < 0 {
        return err;
    }
    (*chip).digital_mode = mode;

    dev_dbg!((*(*chip).card).dev, "set_digital_mode to %d\n", mode);
    incompatible_clock
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
