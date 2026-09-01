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

// Dependencies supplied by the surrounding driver: echoaudio, comm page layout,
// firmware ids, clock/digital mode constants, endian helpers, logging helpers,
// locking helpers, and DSP/ASIC support routines.

unsafe extern "C" {
    fn write_control_reg(chip: *mut echoaudio, value: u32, force: core::ffi::c_char) -> core::ffi::c_int;
    fn set_professional_spdif(chip: *mut echoaudio, prof: core::ffi::c_char) -> core::ffi::c_int;
    fn set_digital_mode(chip: *mut echoaudio, mode: u8) -> core::ffi::c_int;
    fn load_asic_generic(chip: *mut echoaudio, cmd: u32, asic: core::ffi::c_short) -> core::ffi::c_int;
    fn check_asic_status(chip: *mut echoaudio) -> core::ffi::c_int;
}

unsafe fn init_hw(
    chip: *mut echoaudio,
    device_id: u16,
    subdevice_id: u16,
) -> core::ffi::c_int {
    let mut err: core::ffi::c_int;

    if snd_BUG_ON(((subdevice_id as core::ffi::c_int) & 0xfff0) != GINA24) {
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
    (*chip).input_clock_types = ECHO_CLOCK_BIT_INTERNAL
        | ECHO_CLOCK_BIT_SPDIF
        | ECHO_CLOCK_BIT_ESYNC
        | ECHO_CLOCK_BIT_ESYNC96
        | ECHO_CLOCK_BIT_ADAT;

    /* Gina24 comes in both '301 and '361 flavors */
    if (*chip).device_id == DEVICE_ID_56361 {
        (*chip).dsp_code_to_load = FW_GINA24_361_DSP;
        (*chip).digital_modes = ECHOCAPS_HAS_DIGITAL_MODE_SPDIF_RCA
            | ECHOCAPS_HAS_DIGITAL_MODE_SPDIF_OPTICAL
            | ECHOCAPS_HAS_DIGITAL_MODE_ADAT;
    } else {
        (*chip).dsp_code_to_load = FW_GINA24_301_DSP;
        (*chip).digital_modes = ECHOCAPS_HAS_DIGITAL_MODE_SPDIF_RCA
            | ECHOCAPS_HAS_DIGITAL_MODE_SPDIF_OPTICAL
            | ECHOCAPS_HAS_DIGITAL_MODE_ADAT
            | ECHOCAPS_HAS_DIGITAL_MODE_SPDIF_CDROM;
    }

    err = load_firmware(chip);
    if err < 0 {
        return err;
    }
    (*chip).bad_board = false;

    err
}

unsafe fn set_mixer_defaults(chip: *mut echoaudio) -> core::ffi::c_int {
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

    if (clocks_from_dsp & GML_CLOCK_DETECT_BIT_ESYNC) != 0 {
        clock_bits |= ECHO_CLOCK_BIT_ESYNC | ECHO_CLOCK_BIT_ESYNC96;
    }

    clock_bits
}

/* Gina24 has an ASIC on the PCI card which must be loaded for anything
interesting to happen. */
unsafe fn load_asic(chip: *mut echoaudio) -> core::ffi::c_int {
    let mut control_reg: u32;
    let mut err: core::ffi::c_int;
    let asic: core::ffi::c_short;

    if (*chip).asic_loaded {
        return 1;
    }

    /* Give the DSP a few milliseconds to settle down */
    mdelay(10);

    /* Pick the correct ASIC for '301 or '361 Gina24 */
    if (*chip).device_id == DEVICE_ID_56361 {
        asic = FW_GINA24_361_ASIC;
    } else {
        asic = FW_GINA24_301_ASIC;
    }

    err = load_asic_generic(chip, DSP_FNC_LOAD_GINA24_ASIC, asic);
    if err < 0 {
        return err;
    }

    (*chip).asic_code = asic;

    /* Now give the new ASIC a little time to set up */
    mdelay(10);
    /* See if it worked */
    err = check_asic_status(chip);

    /* Set up the control register if the load succeeded -
       48 kHz, internal clock, S/PDIF RCA mode */
    if err == 0 {
        control_reg = GML_CONVERTER_ENABLE | GML_48KHZ;
        err = write_control_reg(chip, control_reg, true as core::ffi::c_char);
    }
    err
}

unsafe fn set_sample_rate(chip: *mut echoaudio, rate: u32) -> core::ffi::c_int {
    let mut control_reg: u32;
    let mut clock: u32;

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

    clock = 0;

    control_reg = le32_to_cpu((*(*chip).comm_page).control_register);
    control_reg &= GML_CLOCK_CLEAR_MASK & GML_SPDIF_RATE_CLEAR_MASK;

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
            /* Professional mode ? */
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
            dev_err(
                (*(*chip).card).dev,
                c"set_sample_rate: %d invalid!\n".as_ptr(),
                rate,
            );
            return -EINVAL;
        }
    }

    control_reg |= clock;

    (*(*chip).comm_page).sample_rate = cpu_to_le32(rate); /* ignored by the DSP */
    (*chip).sample_rate = rate;
    dev_dbg(
        (*(*chip).card).dev,
        c"set_sample_rate: %d clock %d\n".as_ptr(),
        rate,
        clock,
    );

    write_control_reg(chip, control_reg, false as core::ffi::c_char)
}

unsafe fn set_input_clock(chip: *mut echoaudio, clock: u16) -> core::ffi::c_int {
    let mut control_reg: u32;
    let clocks_from_dsp: u32;

    /* Mask off the clock select bits */
    control_reg =
        le32_to_cpu((*(*chip).comm_page).control_register) & GML_CLOCK_CLEAR_MASK;
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
            control_reg |= GML_SPDIF_CLOCK;
            if (clocks_from_dsp & GML_CLOCK_DETECT_BIT_SPDIF96) != 0 {
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
        ECHO_CLOCK_ESYNC => {
            control_reg |= GML_ESYNC_CLOCK;
            control_reg &= !GML_DOUBLE_SPEED_MODE;
        }
        ECHO_CLOCK_ESYNC96 => {
            control_reg |= GML_ESYNC_CLOCK | GML_DOUBLE_SPEED_MODE;
        }
        _ => {
            dev_err(
                (*(*chip).card).dev,
                c"Input clock 0x%x not supported for Gina24\n".as_ptr(),
                clock,
            );
            return -EINVAL;
        }
    }

    (*chip).input_clock = clock;
    write_control_reg(chip, control_reg, true as core::ffi::c_char)
}

unsafe fn dsp_set_digital_mode(chip: *mut echoaudio, mode: u8) -> core::ffi::c_int {
    let mut control_reg: u32;
    let err: core::ffi::c_int;
    let mut incompatible_clock: core::ffi::c_int;

    /* Set clock to "internal" if it's not compatible with the new mode */
    incompatible_clock = false as core::ffi::c_int;
    match mode {
        DIGITAL_MODE_SPDIF_OPTICAL | DIGITAL_MODE_SPDIF_CDROM | DIGITAL_MODE_SPDIF_RCA => {
            if (*chip).input_clock == ECHO_CLOCK_ADAT {
                incompatible_clock = true as core::ffi::c_int;
            }
        }
        DIGITAL_MODE_ADAT => {
            if (*chip).input_clock == ECHO_CLOCK_SPDIF {
                incompatible_clock = true as core::ffi::c_int;
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

    let _guard = spinlock_irq_guard(&mut (*chip).lock);

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
        DIGITAL_MODE_SPDIF_CDROM => {
            /* '361 Gina24 cards do not have the S/PDIF CD-ROM mode */
            if (*chip).device_id == DEVICE_ID_56301 {
                control_reg |= GML_SPDIF_CDROM_MODE;
            }
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

    err = write_control_reg(chip, control_reg, true as core::ffi::c_char);
    if err < 0 {
        return err;
    }
    (*chip).digital_mode = mode;

    dev_dbg(
        (*(*chip).card).dev,
        c"set_digital_mode to %d\n".as_ptr(),
        (*chip).digital_mode,
    );
    incompatible_clock
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
