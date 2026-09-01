// SPDX-License-Identifier: GPL-2.0-only
/*
 * helper functions for HDMI models (Xonar HDAV1.3/HDAV1.3 Slim)
 *
 * Copyright (c) Clemens Ladisch <clemens@ladisch.de>
 */

/* Dependencies from the original C includes:
 * <linux/pci.h>, <linux/delay.h>, <sound/asoundef.h>, <sound/control.h>,
 * <sound/core.h>, <sound/pcm.h>, <sound/pcm_params.h>, <sound/tlv.h>,
 * and "xonar.h".
 */

use core::ffi::{c_char, c_uint};

unsafe fn hdmi_write_command(
    chip: *mut oxygen,
    command: u8,
    count: c_uint,
    params: *const u8,
) {
    let mut i: c_uint;
    let mut checksum: u8;

    oxygen_write_uart(chip, 0xfb);
    oxygen_write_uart(chip, 0xef);
    oxygen_write_uart(chip, command);
    oxygen_write_uart(chip, count as u8);
    i = 0;
    while i < count {
        oxygen_write_uart(chip, *params.add(i as usize));
        i += 1;
    }
    checksum = 0xfbu8
        .wrapping_add(0xef)
        .wrapping_add(command)
        .wrapping_add(count as u8);
    i = 0;
    while i < count {
        checksum = checksum.wrapping_add(*params.add(i as usize));
        i += 1;
    }
    oxygen_write_uart(chip, checksum);
}

unsafe fn xonar_hdmi_init_commands(chip: *mut oxygen, hdmi: *mut xonar_hdmi) {
    let mut param: u8;

    oxygen_reset_uart(chip);
    param = 0;
    hdmi_write_command(chip, 0x61, 1, &param);
    param = 1;
    hdmi_write_command(chip, 0x74, 1, &param);
    hdmi_write_command(chip, 0x54, 5, (*hdmi).params.as_ptr());
}

pub unsafe fn xonar_hdmi_init(chip: *mut oxygen, hdmi: *mut xonar_hdmi) {
    (*hdmi).params[1] = IEC958_AES3_CON_FS_48000;
    (*hdmi).params[4] = 1;
    xonar_hdmi_init_commands(chip, hdmi);
}

pub unsafe fn xonar_hdmi_cleanup(chip: *mut oxygen) {
    let param: u8 = 0;

    hdmi_write_command(chip, 0x74, 1, &param);
}

pub unsafe fn xonar_hdmi_resume(chip: *mut oxygen, hdmi: *mut xonar_hdmi) {
    xonar_hdmi_init_commands(chip, hdmi);
}

pub unsafe fn xonar_hdmi_pcm_hardware_filter(
    channel: c_uint,
    hardware: *mut snd_pcm_hardware,
) {
    if channel == PCM_MULTICH {
        (*hardware).rates = SNDRV_PCM_RATE_44100
            | SNDRV_PCM_RATE_48000
            | SNDRV_PCM_RATE_96000
            | SNDRV_PCM_RATE_192000;
        (*hardware).rate_min = 44100;
    }
}

pub unsafe fn xonar_set_hdmi_params(
    chip: *mut oxygen,
    hdmi: *mut xonar_hdmi,
    params: *mut snd_pcm_hw_params,
) {
    (*hdmi).params[0] = 0; /* 1 = non-audio */
    match params_rate(params) {
        44100 => {
            (*hdmi).params[1] = IEC958_AES3_CON_FS_44100;
        }
        48000 => {
            (*hdmi).params[1] = IEC958_AES3_CON_FS_48000;
        }
        192000 => {
            (*hdmi).params[1] = IEC958_AES3_CON_FS_192000;
        }
        _ => {
            /* 96000 */
            (*hdmi).params[1] = IEC958_AES3_CON_FS_96000;
        }
    }
    (*hdmi).params[2] = (params_channels(params) / 2 - 1) as u8;
    if params_format(params) == SNDRV_PCM_FORMAT_S16_LE {
        (*hdmi).params[3] = 0;
    } else {
        (*hdmi).params[3] = 0xc0;
    }
    (*hdmi).params[4] = 1; /* ? */
    hdmi_write_command(chip, 0x54, 5, (*hdmi).params.as_ptr());
}

pub unsafe fn xonar_hdmi_uart_input(chip: *mut oxygen) {
    if (*chip).uart_input_count >= 2
        && (*chip).uart_input[((*chip).uart_input_count - 2) as usize] == b'O'
        && (*chip).uart_input[((*chip).uart_input_count - 1) as usize] == b'K'
    {
        dev_dbg(
            (*(*chip).card).dev,
            b"message from HDMI chip received:\n\0".as_ptr() as *const c_char,
        );
        print_hex_dump_bytes(
            b"\0".as_ptr() as *const c_char,
            DUMP_PREFIX_OFFSET,
            (*chip).uart_input.as_ptr(),
            (*chip).uart_input_count,
        );
        (*chip).uart_input_count = 0;
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
