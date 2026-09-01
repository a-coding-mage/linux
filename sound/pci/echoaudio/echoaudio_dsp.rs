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

use crate::*;

// C preprocessor check preserved from source:
// #if PAGE_SIZE < 4096
// #error PAGE_SIZE is < 4k
// #endif

unsafe fn restore_dsp_settings(chip: *mut echoaudio) -> i32;

/* Some vector commands involve the DSP reading or writing data to and from the
comm page; if you send one of these commands to the DSP, it will complete the
command and then write a non-zero value to the Handshake field in the
comm page.  This function waits for the handshake to show up. */
unsafe fn wait_handshake(chip: *mut echoaudio) -> i32 {
    let mut i: i32;

    /* Wait up to 20ms for the handshake from the DSP */
    i = 0;
    while i < HANDSHAKE_TIMEOUT {
        /* Look for the handshake value */
        barrier();
        if (*(*chip).comm_page).handshake != 0 {
            return 0;
        }
        udelay(1);
        i += 1;
    }

    dev_err((*(*chip).card).dev, c_str!("wait_handshake(): Timeout waiting for DSP\n"));
    -EBUSY
}

/* Much of the interaction between the DSP and the driver is done via vector
commands; send_vector writes a vector command to the DSP.  Typically, this
causes the DSP to read or write fields in the comm page.
PCI posting is not required thanks to the handshake logic. */
unsafe fn send_vector(chip: *mut echoaudio, command: u32) -> i32 {
    let mut i: i32;

    wmb(); /* Flush all pending writes before sending the command */

    /* Wait up to 100ms for the "vector busy" bit to be off */
    i = 0;
    while i < VECTOR_BUSY_TIMEOUT {
        if (get_dsp_register(chip, CHI32_VECTOR_REG) & CHI32_VECTOR_BUSY) == 0 {
            set_dsp_register(chip, CHI32_VECTOR_REG, command);
            /*if (i)  DE_ACT(("send_vector time: %d\n", i));*/
            return 0;
        }
        udelay(1);
        i += 1;
    }

    dev_err((*(*chip).card).dev, c_str!("timeout on send_vector\n"));
    -EBUSY
}

/* write_dsp writes a 32-bit value to the DSP; this is used almost
exclusively for loading the DSP. */
unsafe fn write_dsp(chip: *mut echoaudio, data: u32) -> i32 {
    let mut status: u32;
    let mut i: u32;

    i = 0;
    while i < 10000000 {
        /* timeout = 10s */
        status = get_dsp_register(chip, CHI32_STATUS_REG);
        if (status & CHI32_STATUS_HOST_WRITE_EMPTY) != 0 {
            set_dsp_register(chip, CHI32_DATA_REG, data);
            wmb(); /* write it immediately */
            return 0;
        }
        udelay(1);
        cond_resched();
        i += 1;
    }

    (*chip).bad_board = true; /* Set true until DSP re-loaded */
    dev_dbg((*(*chip).card).dev, c_str!("write_dsp: Set bad_board to true\n"));
    -EIO
}

/* read_dsp reads a 32-bit value from the DSP; this is used almost
exclusively for loading the DSP and checking the status of the ASIC. */
unsafe fn read_dsp(chip: *mut echoaudio, data: *mut u32) -> i32 {
    let mut status: u32;
    let mut i: u32;

    i = 0;
    while i < READ_DSP_TIMEOUT {
        status = get_dsp_register(chip, CHI32_STATUS_REG);
        if (status & CHI32_STATUS_HOST_READ_FULL) != 0 {
            *data = get_dsp_register(chip, CHI32_DATA_REG);
            return 0;
        }
        udelay(1);
        cond_resched();
        i += 1;
    }

    (*chip).bad_board = true; /* Set true until DSP re-loaded */
    dev_err((*(*chip).card).dev, c_str!("read_dsp: Set bad_board to true\n"));
    -EIO
}

/****************************************************************************
    Firmware loading functions
 ****************************************************************************/

/* This function is used to read back the serial number from the DSP;
this is triggered by the SET_COMMPAGE_ADDR command.
Only some early Echogals products have serial numbers in the ROM;
the serial number is not used, but you still need to do this as
part of the DSP load process. */
unsafe fn read_sn(chip: *mut echoaudio) -> i32 {
    let mut i: i32;
    let mut sn: [u32; 6] = [0; 6];

    i = 0;
    while i < 5 {
        if read_dsp(chip, &mut sn[i as usize]) != 0 {
            dev_err((*(*chip).card).dev, c_str!("Failed to read serial number\n"));
            return -EIO;
        }
        i += 1;
    }
    dev_dbg(
        (*(*chip).card).dev,
        c_str!("Read serial number %08x %08x %08x %08x %08x\n"),
        sn[0],
        sn[1],
        sn[2],
        sn[3],
        sn[4],
    );
    0
}

// #ifndef ECHOCARD_HAS_ASIC
/* This card has no ASIC, just return ok */
unsafe fn check_asic_status(chip: *mut echoaudio) -> i32 {
    (*chip).asic_loaded = true;
    0
}
// #endif /* !ECHOCARD_HAS_ASIC */

// #ifdef ECHOCARD_HAS_ASIC
/* Load ASIC code - done after the DSP is loaded */
unsafe fn load_asic_generic(chip: *mut echoaudio, cmd: u32, asic: i16) -> i32 {
    let mut fw: *const firmware = core::ptr::null();
    let mut err: i32;
    let mut i: u32;
    let size: u32;
    let code: *mut u8;

    err = get_firmware(&mut fw, chip, asic);
    if err < 0 {
        dev_warn((*(*chip).card).dev, c_str!("Firmware not found !\n"));
        return err;
    }

    code = (*fw).data as *mut u8;
    size = (*fw).size as u32;

    /* Send the "Here comes the ASIC" command */
    if write_dsp(chip, cmd) < 0 {
        dev_err((*(*chip).card).dev, c_str!("failed on write_dsp\n"));
        free_firmware(fw, chip);
        return -EIO;
    }

    /* Write length of ASIC file in bytes */
    if write_dsp(chip, size) < 0 {
        dev_err((*(*chip).card).dev, c_str!("failed on write_dsp\n"));
        free_firmware(fw, chip);
        return -EIO;
    }

    i = 0;
    while i < size {
        if write_dsp(chip, *code.add(i as usize) as u32) < 0 {
            dev_err((*(*chip).card).dev, c_str!("failed on write_dsp\n"));
            free_firmware(fw, chip);
            return -EIO;
        }
        i += 1;
    }

    free_firmware(fw, chip);
    0
}
// #endif /* ECHOCARD_HAS_ASIC */

// #ifdef DSP_56361
/* Install the resident loader for 56361 DSPs;  The resident loader is on
the EPROM on the board for 56301 DSP. The resident loader is a tiny little
program that is used to load the real DSP code. */
unsafe fn install_resident_loader(chip: *mut echoaudio) -> i32 {
    let mut address: u32;
    let mut index: i32;
    let words: i32;
    let mut i: i32;
    let code: *mut u16;
    let mut status: u32;
    let mut fw: *const firmware = core::ptr::null();

    /* 56361 cards only!  This check is required by the old 56301-based
    Mona and Gina24 */
    if (*chip).device_id != DEVICE_ID_56361 {
        return 0;
    }

    /* Look to see if the resident loader is present.  If the resident
    loader is already installed, host flag 5 will be on. */
    status = get_dsp_register(chip, CHI32_STATUS_REG);
    if (status & CHI32_STATUS_REG_HF5) != 0 {
        dev_dbg(
            (*(*chip).card).dev,
            c_str!("Resident loader already installed; status is 0x%x\n"),
            status,
        );
        return 0;
    }

    i = get_firmware(&mut fw, chip, FW_361_LOADER);
    if i < 0 {
        dev_warn((*(*chip).card).dev, c_str!("Firmware not found !\n"));
        return i;
    }

    /* The DSP code is an array of 16 bit words.  The array is divided up
    into sections.  The first word of each section is the size in words,
    followed by the section type.
    Since DSP addresses and data are 24 bits wide, they each take up two
    16 bit words in the array.
    This is a lot like the other loader loop, but it's not a loop, you
    don't write the memory type, and you don't write a zero at the end. */

    /* Set DSP format bits for 24 bit mode */
    set_dsp_register(
        chip,
        CHI32_CONTROL_REG,
        get_dsp_register(chip, CHI32_CONTROL_REG) | 0x900,
    );

    code = (*fw).data as *mut u16;

    /* Skip the header section; the first word in the array is the size
    of the first section, so the first real section of code is pointed
    to by Code[0]. */
    index = *code.add(0) as i32;

    /* Skip the section size, LRS block type, and DSP memory type */
    index += 3;

    /* Get the number of DSP words to write */
    words = *code.add(index as usize) as i32;
    index += 1;

    /* Get the DSP address for this block; 24 bits, so build from two words */
    address = ((*code.add(index as usize) as u32) << 16) + *code.add(index as usize + 1) as u32;
    index += 2;

    /* Write the count to the DSP */
    if write_dsp(chip, words as u32) != 0 {
        dev_err((*(*chip).card).dev, c_str!("install_resident_loader: Failed to write word count!\n"));
        free_firmware(fw, chip);
        return -EIO;
    }
    /* Write the DSP address */
    if write_dsp(chip, address) != 0 {
        dev_err((*(*chip).card).dev, c_str!("install_resident_loader: Failed to write DSP address!\n"));
        free_firmware(fw, chip);
        return -EIO;
    }
    /* Write out this block of code to the DSP */
    i = 0;
    while i < words {
        let data: u32;

        data = ((*code.add(index as usize) as u32) << 16) + *code.add(index as usize + 1) as u32;
        if write_dsp(chip, data) != 0 {
            dev_err((*(*chip).card).dev, c_str!("install_resident_loader: Failed to write DSP code\n"));
            free_firmware(fw, chip);
            return -EIO;
        }
        index += 2;
        i += 1;
    }

    /* Wait for flag 5 to come up */
    i = 0;
    while i < 200 {
        /* Timeout is 50us * 200 = 10ms */
        udelay(50);
        status = get_dsp_register(chip, CHI32_STATUS_REG);
        if (status & CHI32_STATUS_REG_HF5) != 0 {
            break;
        }
        i += 1;
    }

    if i == 200 {
        dev_err((*(*chip).card).dev, c_str!("Resident loader failed to set HF5\n"));
        free_firmware(fw, chip);
        return -EIO;
    }

    dev_dbg((*(*chip).card).dev, c_str!("Resident loader successfully installed\n"));
    free_firmware(fw, chip);
    0
}
// #endif /* DSP_56361 */

unsafe fn load_dsp(chip: *mut echoaudio, code: *mut u16) -> i32 {
    let mut address: u32;
    let mut data: u32;
    let mut index: i32;
    let mut words: i32;
    let mut i: i32;

    if (*chip).dsp_code == code {
        dev_warn((*(*chip).card).dev, c_str!("DSP is already loaded!\n"));
        return 0;
    }
    (*chip).bad_board = true; /* Set true until DSP loaded */
    (*chip).dsp_code = core::ptr::null_mut(); /* Current DSP code not loaded */
    (*chip).asic_loaded = false; /* Loading the DSP code will reset the ASIC */

    dev_dbg((*(*chip).card).dev, c_str!("load_dsp: Set bad_board to true\n"));

    /* If this board requires a resident loader, install it. */
    // #ifdef DSP_56361
    i = install_resident_loader(chip);
    if i < 0 {
        return i;
    }
    // #endif

    /* Send software reset command */
    if send_vector(chip, DSP_VC_RESET) < 0 {
        dev_err((*(*chip).card).dev, c_str!("LoadDsp: send_vector DSP_VC_RESET failed, Critical Failure\n"));
        return -EIO;
    }
    /* Delay 10us */
    udelay(10);

    /* Wait 10ms for HF3 to indicate that software reset is complete */
    i = 0;
    while i < 1000 {
        /* Timeout is 10us * 1000 = 10ms */
        if (get_dsp_register(chip, CHI32_STATUS_REG) & CHI32_STATUS_REG_HF3) != 0 {
            break;
        }
        udelay(10);
        i += 1;
    }

    if i == 1000 {
        dev_err((*(*chip).card).dev, c_str!("load_dsp: Timeout waiting for CHI32_STATUS_REG_HF3\n"));
        return -EIO;
    }

    /* Set DSP format bits for 24 bit mode now that soft reset is done */
    set_dsp_register(
        chip,
        CHI32_CONTROL_REG,
        get_dsp_register(chip, CHI32_CONTROL_REG) | 0x900,
    );

    /* Main loader loop */

    index = *code.add(0) as i32;
    loop {
        let block_type: i32;
        let mem_type: i32;

        /* Total Block Size */
        index += 1;

        /* Block Type */
        block_type = *code.add(index as usize) as i32;
        if block_type == 4 {
            /* We're finished */
            break;
        }

        index += 1;

        /* Memory Type  P=0,X=1,Y=2 */
        mem_type = *code.add(index as usize) as i32;
        index += 1;

        /* Block Code Size */
        words = *code.add(index as usize) as i32;
        index += 1;
        if words == 0 {
            /* We're finished */
            break;
        }

        /* Start Address */
        address = ((*code.add(index as usize) as u32) << 16) + *code.add(index as usize + 1) as u32;
        index += 2;

        if write_dsp(chip, words as u32) < 0 {
            dev_err((*(*chip).card).dev, c_str!("load_dsp: failed to write number of DSP words\n"));
            return -EIO;
        }
        if write_dsp(chip, address) < 0 {
            dev_err((*(*chip).card).dev, c_str!("load_dsp: failed to write DSP address\n"));
            return -EIO;
        }
        if write_dsp(chip, mem_type as u32) < 0 {
            dev_err((*(*chip).card).dev, c_str!("load_dsp: failed to write DSP memory type\n"));
            return -EIO;
        }
        /* Code */
        i = 0;
        while i < words {
            data = ((*code.add(index as usize) as u32) << 16) + *code.add(index as usize + 1) as u32;
            if write_dsp(chip, data) < 0 {
                dev_err((*(*chip).card).dev, c_str!("load_dsp: failed to write DSP data\n"));
                return -EIO;
            }
            index += 2;
            i += 1;
        }
    }

    if write_dsp(chip, 0) < 0 {
        /* We're done!!! */
        dev_err((*(*chip).card).dev, c_str!("load_dsp: Failed to write final zero\n"));
        return -EIO;
    }
    udelay(10);

    i = 0;
    while i < 5000 {
        /* Timeout is 100us * 5000 = 500ms */
        /* Wait for flag 4 - indicates that the DSP loaded OK */
        if (get_dsp_register(chip, CHI32_STATUS_REG) & CHI32_STATUS_REG_HF4) != 0 {
            set_dsp_register(
                chip,
                CHI32_CONTROL_REG,
                get_dsp_register(chip, CHI32_CONTROL_REG) & !0x1b00,
            );

            if write_dsp(chip, DSP_FNC_SET_COMMPAGE_ADDR) < 0 {
                dev_err((*(*chip).card).dev, c_str!("load_dsp: Failed to write DSP_FNC_SET_COMMPAGE_ADDR\n"));
                return -EIO;
            }

            if write_dsp(chip, (*chip).comm_page_phys) < 0 {
                dev_err((*(*chip).card).dev, c_str!("load_dsp: Failed to write comm page address\n"));
                return -EIO;
            }

            /* Get the serial number via slave mode.
            This is triggered by the SET_COMMPAGE_ADDR command.
            We don't actually use the serial number but we have to
            get it as part of the DSP init voodoo. */
            if read_sn(chip) < 0 {
                dev_err((*(*chip).card).dev, c_str!("load_dsp: Failed to read serial number\n"));
                return -EIO;
            }

            (*chip).dsp_code = code; /* Show which DSP code loaded */
            (*chip).bad_board = false; /* DSP OK */
            return 0;
        }
        udelay(100);
        i += 1;
    }

    dev_err((*(*chip).card).dev, c_str!("load_dsp: DSP load timed out waiting for HF4\n"));
    -EIO
}

/* load_firmware takes care of loading the DSP and any ASIC code. */
unsafe fn load_firmware(chip: *mut echoaudio) -> i32 {
    let mut fw: *const firmware = core::ptr::null();
    let mut box_type: i32;
    let mut err: i32;

    if snd_BUG_ON((*chip).comm_page.is_null()) != 0 {
        return -EPERM;
    }

    /* See if the ASIC is present and working - only if the DSP is already loaded */
    if !(*chip).dsp_code.is_null() {
        box_type = check_asic_status(chip);
        if box_type >= 0 {
            return box_type;
        }
        /* ASIC check failed; force the DSP to reload */
        (*chip).dsp_code = core::ptr::null_mut();
    }

    err = get_firmware(&mut fw, chip, (*chip).dsp_code_to_load);
    if err < 0 {
        return err;
    }
    err = load_dsp(chip, (*fw).data as *mut u16);
    free_firmware(fw, chip);
    if err < 0 {
        return err;
    }

    box_type = load_asic(chip);
    if box_type < 0 {
        return box_type; /* error */
    }

    box_type
}

/****************************************************************************
    Mixer functions
 ****************************************************************************/

// #if defined(ECHOCARD_HAS_INPUT_NOMINAL_LEVEL) || defined(ECHOCARD_HAS_OUTPUT_NOMINAL_LEVEL)
/* Set the nominal level for an input or output bus (true = -10dBV, false = +4dBu) */
unsafe fn set_nominal_level(chip: *mut echoaudio, index: u16, consumer: i8) -> i32 {
    if snd_BUG_ON(index as u32 >= num_busses_out(chip) + num_busses_in(chip)) != 0 {
        return -EINVAL;
    }

    /* Wait for the handshake (OK even if ASIC is not loaded) */
    if wait_handshake(chip) != 0 {
        return -EIO;
    }

    (*chip).nominal_level[index as usize] = consumer;

    if consumer != 0 {
        (*(*chip).comm_page).nominal_level_mask |= cpu_to_le32(1u32 << index);
    } else {
        (*(*chip).comm_page).nominal_level_mask &= !cpu_to_le32(1u32 << index);
    }

    0
}
// #endif /* ECHOCARD_HAS_*_NOMINAL_LEVEL */

/* Set the gain for a single physical output channel (dB). */
unsafe fn set_output_gain(chip: *mut echoaudio, channel: u16, gain: i8) -> i32 {
    if snd_BUG_ON(channel as u32 >= num_busses_out(chip)) != 0 {
        return -EINVAL;
    }

    if wait_handshake(chip) != 0 {
        return -EIO;
    }

    /* Save the new value */
    (*chip).output_gain[channel as usize] = gain;
    (*(*chip).comm_page).line_out_level[channel as usize] = gain;
    0
}

// #ifdef ECHOCARD_HAS_MONITOR
/* Set the monitor level from an input bus to an output bus. */
unsafe fn set_monitor_gain(chip: *mut echoaudio, output: u16, input: u16, gain: i8) -> i32 {
    if snd_BUG_ON(output as u32 >= num_busses_out(chip) || input as u32 >= num_busses_in(chip)) != 0 {
        return -EINVAL;
    }

    if wait_handshake(chip) != 0 {
        return -EIO;
    }

    (*chip).monitor_gain[output as usize][input as usize] = gain;
    (*(*chip).comm_page).monitors[monitor_index(chip, output, input) as usize] = gain;
    0
}
// #endif /* ECHOCARD_HAS_MONITOR */

/* Tell the DSP to read and update output, nominal & monitor levels in comm page. */
unsafe fn update_output_line_level(chip: *mut echoaudio) -> i32 {
    if wait_handshake(chip) != 0 {
        return -EIO;
    }
    clear_handshake(chip);
    send_vector(chip, DSP_VC_UPDATE_OUTVOL)
}

/* Tell the DSP to read and update input levels in comm page */
unsafe fn update_input_line_level(chip: *mut echoaudio) -> i32 {
    if wait_handshake(chip) != 0 {
        return -EIO;
    }
    clear_handshake(chip);
    send_vector(chip, DSP_VC_UPDATE_INGAIN)
}

/* set_meters_on turns the meters on or off.  If meters are turned on, the DSP
will write the meter and clock detect values to the comm page at about 30Hz */
unsafe fn set_meters_on(chip: *mut echoaudio, on: i8) {
    if on != 0 && (*chip).meters_enabled == 0 {
        send_vector(chip, DSP_VC_METERS_ON);
        (*chip).meters_enabled = 1;
    } else if on == 0 && (*chip).meters_enabled != 0 {
        send_vector(chip, DSP_VC_METERS_OFF);
        (*chip).meters_enabled = 0;
        memset(
            (*(*chip).comm_page).vu_meter.as_mut_ptr() as *mut i8,
            ECHOGAIN_MUTED,
            DSP_MAXPIPES,
        );
        memset(
            (*(*chip).comm_page).peak_meter.as_mut_ptr() as *mut i8,
            ECHOGAIN_MUTED,
            DSP_MAXPIPES,
        );
    }
}

/* Fill out an the given array using the current values in the comm page.
Meters are written in the comm page by the DSP in this order:
 Output busses
 Input busses
 Output pipes (vmixer cards only)

This function assumes there are no more than 16 in/out busses or pipes
Meters is an array [3][16][2] of long. */
unsafe fn get_audio_meters(chip: *mut echoaudio, meters: *mut c_long) {
    let mut i: u32;
    let mut m: u32;
    let mut n: u32;

    i = 0;
    while i < 96 {
        *meters.add(i as usize) = 0;
        i += 1;
    }

    m = 0;
    n = 0;
    i = 0;
    while i < num_busses_out(chip) {
        *meters.add(n as usize) = (*(*chip).comm_page).vu_meter[m as usize] as c_long;
        n += 1;
        *meters.add(n as usize) = (*(*chip).comm_page).peak_meter[m as usize] as c_long;
        n += 1;
        i += 1;
        m += 1;
    }

    // #ifdef ECHOCARD_ECHO3G
    m = E3G_MAX_OUTPUTS; /* Skip unused meters */
    // #endif

    n = 32;
    i = 0;
    while i < num_busses_in(chip) {
        *meters.add(n as usize) = (*(*chip).comm_page).vu_meter[m as usize] as c_long;
        n += 1;
        *meters.add(n as usize) = (*(*chip).comm_page).peak_meter[m as usize] as c_long;
        n += 1;
        i += 1;
        m += 1;
    }
    // #ifdef ECHOCARD_HAS_VMIXER
    n = 64;
    i = 0;
    while i < num_pipes_out(chip) {
        *meters.add(n as usize) = (*(*chip).comm_page).vu_meter[m as usize] as c_long;
        n += 1;
        *meters.add(n as usize) = (*(*chip).comm_page).peak_meter[m as usize] as c_long;
        n += 1;
        i += 1;
        m += 1;
    }
    // #endif
}

unsafe fn restore_dsp_settings(chip: *mut echoaudio) -> i32 {
    let mut i: i32;
    let mut o: i32;
    let mut err: i32;

    err = check_asic_status(chip);
    if err < 0 {
        return err;
    }

    /* Gina20/Darla20 only. Should be harmless for other cards. */
    (*(*chip).comm_page).gd_clock_state = GD_CLOCK_UNDEF;
    (*(*chip).comm_page).gd_spdif_status = GD_SPDIF_STATUS_UNDEF;
    (*(*chip).comm_page).handshake = cpu_to_le32(0xffffffff);

    /* Restore output busses */
    i = 0;
    while i < num_busses_out(chip) as i32 {
        err = set_output_gain(chip, i as u16, (*chip).output_gain[i as usize]);
        if err < 0 {
            return err;
        }
        i += 1;
    }

    // #ifdef ECHOCARD_HAS_VMIXER
    i = 0;
    while i < num_pipes_out(chip) as i32 {
        o = 0;
        while o < num_busses_out(chip) as i32 {
            err = set_vmixer_gain(chip, o as u16, i as u16, (*chip).vmixer_gain[o as usize][i as usize]);
            if err < 0 {
                return err;
            }
            o += 1;
        }
        i += 1;
    }
    if update_vmixer_level(chip) < 0 {
        return -EIO;
    }
    // #endif /* ECHOCARD_HAS_VMIXER */

    // #ifdef ECHOCARD_HAS_MONITOR
    o = 0;
    while o < num_busses_out(chip) as i32 {
        i = 0;
        while i < num_busses_in(chip) as i32 {
            err = set_monitor_gain(chip, o as u16, i as u16, (*chip).monitor_gain[o as usize][i as usize]);
            if err < 0 {
                return err;
            }
            i += 1;
        }
        o += 1;
    }
    // #endif /* ECHOCARD_HAS_MONITOR */

    // #ifdef ECHOCARD_HAS_INPUT_GAIN
    i = 0;
    while i < num_busses_in(chip) as i32 {
        err = set_input_gain(chip, i as u16, (*chip).input_gain[i as usize]);
        if err < 0 {
            return err;
        }
        i += 1;
    }
    // #endif /* ECHOCARD_HAS_INPUT_GAIN */

    err = update_output_line_level(chip);
    if err < 0 {
        return err;
    }

    err = update_input_line_level(chip);
    if err < 0 {
        return err;
    }

    err = set_sample_rate(chip, (*chip).sample_rate);
    if err < 0 {
        return err;
    }

    if (*chip).meters_enabled != 0 {
        err = send_vector(chip, DSP_VC_METERS_ON);
        if err < 0 {
            return err;
        }
    }

    // #ifdef ECHOCARD_HAS_DIGITAL_MODE_SWITCH
    if set_digital_mode(chip, (*chip).digital_mode) < 0 {
        return -EIO;
    }
    // #endif

    // #ifdef ECHOCARD_HAS_DIGITAL_IO
    if set_professional_spdif(chip, (*chip).professional_spdif) < 0 {
        return -EIO;
    }
    // #endif

    // #ifdef ECHOCARD_HAS_PHANTOM_POWER
    if set_phantom_power(chip, (*chip).phantom_power) < 0 {
        return -EIO;
    }
    // #endif

    // #ifdef ECHOCARD_HAS_EXTERNAL_CLOCK
    /* set_input_clock() also restores automute setting */
    if set_input_clock(chip, (*chip).input_clock) < 0 {
        return -EIO;
    }
    // #endif

    // #ifdef ECHOCARD_HAS_OUTPUT_CLOCK_SWITCH
    if set_output_clock(chip, (*chip).output_clock) < 0 {
        return -EIO;
    }
    // #endif

    if wait_handshake(chip) < 0 {
        return -EIO;
    }
    clear_handshake(chip);
    if send_vector(chip, DSP_VC_UPDATE_FLAGS) < 0 {
        return -EIO;
    }

    0
}

/****************************************************************************
    Transport functions
 ****************************************************************************/

/* set_audio_format() sets the format of the audio data in host memory for
this pipe.  Note that _MS_ (mono-to-stereo) playback modes are not used by ALSA
but they are here because they are just mono while capturing */
unsafe fn set_audio_format(chip: *mut echoaudio, pipe_index: u16, format: *const audioformat) {
    let mut dsp_format: u16;

    dsp_format = DSP_AUDIOFORM_SS_16LE;

    /* Look for super-interleave (no big-endian and 8 bits) */
    if (*format).interleave > 2 {
        match (*format).bits_per_sample {
            16 => {
                dsp_format = DSP_AUDIOFORM_SUPER_INTERLEAVE_16LE;
            }
            24 => {
                dsp_format = DSP_AUDIOFORM_SUPER_INTERLEAVE_24LE;
            }
            32 => {
                dsp_format = DSP_AUDIOFORM_SUPER_INTERLEAVE_32LE;
            }
            _ => {}
        }
        dsp_format |= (*format).interleave as u16;
    } else if (*format).data_are_bigendian != 0 {
        /* For big-endian data, only 32 bit samples are supported */
        match (*format).interleave {
            1 => {
                dsp_format = DSP_AUDIOFORM_MM_32BE;
            }
            // #ifdef ECHOCARD_HAS_STEREO_BIG_ENDIAN32
            2 => {
                dsp_format = DSP_AUDIOFORM_SS_32BE;
            }
            // #endif
            _ => {}
        }
    } else if (*format).interleave == 1 && (*format).bits_per_sample == 32 && (*format).mono_to_stereo == 0 {
        /* 32 bit little-endian mono->mono case */
        dsp_format = DSP_AUDIOFORM_MM_32LE;
    } else {
        /* Handle the other little-endian formats */
        match (*format).bits_per_sample {
            8 => {
                if (*format).interleave == 2 {
                    dsp_format = DSP_AUDIOFORM_SS_8;
                } else {
                    dsp_format = DSP_AUDIOFORM_MS_8;
                }
            }
            24 => {
                if (*format).interleave == 2 {
                    dsp_format = DSP_AUDIOFORM_SS_24LE;
                } else {
                    dsp_format = DSP_AUDIOFORM_MS_24LE;
                }
            }
            32 => {
                if (*format).interleave == 2 {
                    dsp_format = DSP_AUDIOFORM_SS_32LE;
                } else {
                    dsp_format = DSP_AUDIOFORM_MS_32LE;
                }
            }
            _ => {
                if (*format).interleave == 2 {
                    dsp_format = DSP_AUDIOFORM_SS_16LE;
                } else {
                    dsp_format = DSP_AUDIOFORM_MS_16LE;
                }
            }
        }
    }
    dev_dbg((*(*chip).card).dev, c_str!("set_audio_format[%d] = %x\n"), pipe_index, dsp_format);
    (*(*chip).comm_page).audio_format[pipe_index as usize] = cpu_to_le16(dsp_format);
}

/* start_transport starts transport for a set of pipes.
The bits 1 in channel_mask specify what pipes to start. Only the bit of the
first channel must be set, regardless its interleave.
Same thing for pause_ and stop_ -trasport below. */
unsafe fn start_transport(chip: *mut echoaudio, channel_mask: u32, _cyclic_mask: u32) -> i32 {
    if wait_handshake(chip) != 0 {
        return -EIO;
    }

    (*(*chip).comm_page).cmd_start |= cpu_to_le32(channel_mask);

    if (*(*chip).comm_page).cmd_start != 0 {
        clear_handshake(chip);
        send_vector(chip, DSP_VC_START_TRANSFER);
        if wait_handshake(chip) != 0 {
            return -EIO;
        }
        /* Keep track of which pipes are transporting */
        (*chip).active_mask |= channel_mask;
        (*(*chip).comm_page).cmd_start = 0;
        return 0;
    }

    dev_err((*(*chip).card).dev, c_str!("start_transport: No pipes to start!\n"));
    -EINVAL
}

unsafe fn pause_transport(chip: *mut echoaudio, channel_mask: u32) -> i32 {
    if wait_handshake(chip) != 0 {
        return -EIO;
    }

    (*(*chip).comm_page).cmd_stop |= cpu_to_le32(channel_mask);
    (*(*chip).comm_page).cmd_reset = 0;
    if (*(*chip).comm_page).cmd_stop != 0 {
        clear_handshake(chip);
        send_vector(chip, DSP_VC_STOP_TRANSFER);
        if wait_handshake(chip) != 0 {
            return -EIO;
        }
        /* Keep track of which pipes are transporting */
        (*chip).active_mask &= !channel_mask;
        (*(*chip).comm_page).cmd_stop = 0;
        (*(*chip).comm_page).cmd_reset = 0;
        return 0;
    }

    dev_dbg((*(*chip).card).dev, c_str!("pause_transport: No pipes to stop!\n"));
    0
}

unsafe fn stop_transport(chip: *mut echoaudio, channel_mask: u32) -> i32 {
    if wait_handshake(chip) != 0 {
        return -EIO;
    }

    (*(*chip).comm_page).cmd_stop |= cpu_to_le32(channel_mask);
    (*(*chip).comm_page).cmd_reset |= cpu_to_le32(channel_mask);
    if (*(*chip).comm_page).cmd_reset != 0 {
        clear_handshake(chip);
        send_vector(chip, DSP_VC_STOP_TRANSFER);
        if wait_handshake(chip) != 0 {
            return -EIO;
        }
        /* Keep track of which pipes are transporting */
        (*chip).active_mask &= !channel_mask;
        (*(*chip).comm_page).cmd_stop = 0;
        (*(*chip).comm_page).cmd_reset = 0;
        return 0;
    }

    dev_dbg((*(*chip).card).dev, c_str!("stop_transport: No pipes to stop!\n"));
    0
}

unsafe fn is_pipe_allocated(chip: *mut echoaudio, pipe_index: u16) -> i32 {
    ((*chip).pipe_alloc_mask & (1u32 << pipe_index)) as i32
}

/* Stops everything and turns off the DSP. All pipes should be already
stopped and unallocated. */
unsafe fn rest_in_peace(chip: *mut echoaudio) -> i32 {
    /* Stops all active pipes (just to be sure) */
    stop_transport(chip, (*chip).active_mask);

    set_meters_on(chip, false as i8);

    // #ifdef ECHOCARD_HAS_MIDI
    enable_midi_input(chip, false as i8);
    // #endif

    /* Go to sleep */
    if !(*chip).dsp_code.is_null() {
        /* Make load_firmware do a complete reload */
        (*chip).dsp_code = core::ptr::null_mut();
        /* Put the DSP to sleep */
        return send_vector(chip, DSP_VC_GO_COMATOSE);
    }
    0
}

/* Fills the comm page with default values */
unsafe fn init_dsp_comm_page(chip: *mut echoaudio) -> i32 {
    /* Check if the compiler added extra padding inside the structure */
    if offset_of!(comm_page, midi_output) != 0xbe0 {
        dev_err((*(*chip).card).dev, c_str!("init_dsp_comm_page() - Invalid struct comm_page structure\n"));
        return -EPERM;
    }

    /* Init all the basic stuff */
    (*chip).card_name = ECHOCARD_NAME;
    (*chip).bad_board = true; /* Set true until DSP loaded */
    (*chip).dsp_code = core::ptr::null_mut(); /* Current DSP code not loaded */
    (*chip).asic_loaded = false;
    memset((*chip).comm_page as *mut c_void, 0, core::mem::size_of::<comm_page>());

    /* Init the comm page */
    (*(*chip).comm_page).comm_size = cpu_to_le32(core::mem::size_of::<comm_page>() as u32);
    (*(*chip).comm_page).handshake = cpu_to_le32(0xffffffff);
    (*(*chip).comm_page).midi_out_free_count = cpu_to_le32(DSP_MIDI_OUT_FIFO_SIZE);
    (*(*chip).comm_page).sample_rate = cpu_to_le32(44100);

    /* Set line levels so we don't blast any inputs on startup */
    memset(
        (*(*chip).comm_page).monitors.as_mut_ptr() as *mut c_void,
        ECHOGAIN_MUTED,
        MONITOR_ARRAY_SIZE,
    );
    memset(
        (*(*chip).comm_page).vmixer.as_mut_ptr() as *mut c_void,
        ECHOGAIN_MUTED,
        VMIXER_ARRAY_SIZE,
    );

    0
}

/* This function initializes the chip structure with default values, ie. all
 * muted and internal clock source. Then it copies the settings to the DSP.
 * This MUST be called after the DSP is up and running !
 */
unsafe fn init_line_levels(chip: *mut echoaudio) -> i32 {
    memset((*chip).output_gain.as_mut_ptr() as *mut c_void, ECHOGAIN_MUTED, core::mem::size_of_val(&(*chip).output_gain));
    memset((*chip).input_gain.as_mut_ptr() as *mut c_void, ECHOGAIN_MUTED, core::mem::size_of_val(&(*chip).input_gain));
    memset((*chip).monitor_gain.as_mut_ptr() as *mut c_void, ECHOGAIN_MUTED, core::mem::size_of_val(&(*chip).monitor_gain));
    memset((*chip).vmixer_gain.as_mut_ptr() as *mut c_void, ECHOGAIN_MUTED, core::mem::size_of_val(&(*chip).vmixer_gain));
    (*chip).input_clock = ECHO_CLOCK_INTERNAL;
    (*chip).output_clock = ECHO_CLOCK_WORD;
    (*chip).sample_rate = 44100;
    restore_dsp_settings(chip)
}

/* This is low level part of the interrupt handler.
It returns -1 if the IRQ is not ours, or N>=0 if it is, where N is the number
of midi data in the input queue. */
unsafe fn service_irq(chip: *mut echoaudio) -> i32 {
    let mut st: i32;

    /* Read the DSP status register and see if this DSP generated this interrupt */
    if (get_dsp_register(chip, CHI32_STATUS_REG) & CHI32_STATUS_IRQ) != 0 {
        st = 0;
        // #ifdef ECHOCARD_HAS_MIDI
        /* Get and parse midi data if present */
        if (*(*chip).comm_page).midi_input[0] != 0 {
            /* The count is at index 0 */
            st = midi_service_irq(chip); /* Returns how many midi bytes we received */
        }
        // #endif
        /* Clear the hardware interrupt */
        (*(*chip).comm_page).midi_input[0] = 0;
        send_vector(chip, DSP_VC_ACK_INT);
        return st;
    }
    -1
}

/******************************************************************************
    Functions for opening and closing pipes
 ******************************************************************************/

/* allocate_pipes is used to reserve audio pipes for your exclusive use.
The call will fail if some pipes are already allocated. */
unsafe fn allocate_pipes(
    chip: *mut echoaudio,
    pipe: *mut audiopipe,
    pipe_index: i32,
    interleave: i32,
) -> i32 {
    let mut i: i32;
    let mut channel_mask: u32;

    dev_dbg((*(*chip).card).dev, c_str!("allocate_pipes: ch=%d int=%d\n"), pipe_index, interleave);

    if (*chip).bad_board {
        return -EIO;
    }

    channel_mask = 0;
    i = 0;
    while i < interleave {
        channel_mask |= 1u32 << (pipe_index + i);
        i += 1;
    }
    if ((*chip).pipe_alloc_mask & channel_mask) != 0 {
        dev_err((*(*chip).card).dev, c_str!("allocate_pipes: channel already open\n"));
        return -EAGAIN;
    }

    (*(*chip).comm_page).position[pipe_index as usize] = 0;
    (*chip).pipe_alloc_mask |= channel_mask;
    /* This driver uses cyclic buffers only */
    (*chip).pipe_cyclic_mask |= channel_mask;
    (*pipe).index = pipe_index;
    (*pipe).interleave = interleave;
    (*pipe).state = PIPE_STATE_STOPPED;

    /* The counter register is where the DSP writes the 32 bit DMA
    position for a pipe.  The DSP is constantly updating this value as
    it moves data. The DMA counter is in units of bytes, not samples. */
    (*pipe).dma_counter = &mut (*(*chip).comm_page).position[pipe_index as usize] as *mut __le32;
    *(*pipe).dma_counter = 0;
    pipe_index
}

unsafe fn free_pipes(chip: *mut echoaudio, pipe: *mut audiopipe) -> i32 {
    let mut channel_mask: u32;
    let mut i: i32;

    if snd_BUG_ON(is_pipe_allocated(chip, (*pipe).index as u16) == 0) != 0 {
        return -EINVAL;
    }
    if snd_BUG_ON((*pipe).state != PIPE_STATE_STOPPED) != 0 {
        return -EINVAL;
    }

    channel_mask = 0;
    i = 0;
    while i < (*pipe).interleave {
        channel_mask |= 1u32 << ((*pipe).index + i);
        i += 1;
    }

    (*chip).pipe_alloc_mask &= !channel_mask;
    (*chip).pipe_cyclic_mask &= !channel_mask;
    0
}

/******************************************************************************
    Functions for managing the scatter-gather list
******************************************************************************/

unsafe fn sglist_init(chip: *mut echoaudio, pipe: *mut audiopipe) -> i32 {
    (*pipe).sglist_head = 0;
    memset((*pipe).sgpage.area as *mut c_void, 0, PAGE_SIZE);
    (*(*chip).comm_page).sglist_addr[(*pipe).index as usize].addr = cpu_to_le32((*pipe).sgpage.addr);
    0
}

unsafe fn sglist_add_mapping(
    chip: *mut echoaudio,
    pipe: *mut audiopipe,
    address: dma_addr_t,
    length: usize,
) -> i32 {
    let head: i32 = (*pipe).sglist_head;
    let list: *mut sg_entry = (*pipe).sgpage.area as *mut sg_entry;

    if head < MAX_SGLIST_ENTRIES - 1 {
        (*list.add(head as usize)).addr = cpu_to_le32(address);
        (*list.add(head as usize)).size = cpu_to_le32(length as u32);
        (*pipe).sglist_head += 1;
    } else {
        dev_err((*(*chip).card).dev, c_str!("SGlist: too many fragments\n"));
        return -ENOMEM;
    }
    0
}

unsafe fn sglist_add_irq(chip: *mut echoaudio, pipe: *mut audiopipe) -> i32 {
    sglist_add_mapping(chip, pipe, 0, 0)
}

unsafe fn sglist_wrap(chip: *mut echoaudio, pipe: *mut audiopipe) -> i32 {
    sglist_add_mapping(chip, pipe, (*pipe).sgpage.addr, 0)
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
