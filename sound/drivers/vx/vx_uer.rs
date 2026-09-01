// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Driver for Digigram VX soundcards
 *
 * IEC958 stuff
 *
 * Copyright (c) 2002 by Takashi Iwai <tiwai@suse.de>
 */

use crate::*;

/*
 * Original C dependencies:
 * #include <linux/delay.h>
 * #include <sound/core.h>
 * #include <sound/vx_core.h>
 * #include "vx_cmd.h"
 */

/*
 * vx_modify_board_clock - tell the board that its clock has been modified
 * @sync: DSP needs to resynchronize its FIFO
 */
unsafe fn vx_modify_board_clock(chip: *mut vx_core, sync: c_int) -> c_int {
    let mut rmh: vx_rmh = core::mem::zeroed();

    vx_init_rmh(&mut rmh, CMD_MODIFY_CLOCK);
    /* Ask the DSP to resynchronize its FIFO. */
    if sync != 0 {
        rmh.Cmd[0] |= CMD_MODIFY_CLOCK_S_BIT;
    }
    vx_send_msg(chip, &mut rmh)
}

/*
 * vx_modify_board_inputs - resync audio inputs
 */
unsafe fn vx_modify_board_inputs(chip: *mut vx_core) -> c_int {
    let mut rmh: vx_rmh = core::mem::zeroed();

    vx_init_rmh(&mut rmh, CMD_RESYNC_AUDIO_INPUTS);
    rmh.Cmd[0] |= 1 << 0; /* reference: AUDIO 0 */
    vx_send_msg(chip, &mut rmh)
}

/*
 * vx_read_one_cbit - read one bit from UER config
 * @index: the bit index
 * returns 0 or 1.
 */
unsafe fn vx_read_one_cbit(chip: *mut vx_core, index: c_int) -> c_int {
    let val: c_int;

    /* TODO: preserve C guard(mutex)(&chip->lock) with the repository lock guard. */
    if (*chip).type_ >= VX_TYPE_VXPOCKET {
        vx_outb(chip, CSUER, 1); /* read */
        vx_outb(chip, RUER, index & XX_UER_CBITS_OFFSET_MASK);
        val = (vx_inb(chip, RUER) >> 7) & 0x01;
    } else {
        vx_outl(chip, CSUER, 1); /* read */
        vx_outl(chip, RUER, index & XX_UER_CBITS_OFFSET_MASK);
        val = (vx_inl(chip, RUER) >> 7) & 0x01;
    }
    val
}

/*
 * vx_write_one_cbit - write one bit to UER config
 * @index: the bit index
 * @val: bit value, 0 or 1
 */
unsafe fn vx_write_one_cbit(chip: *mut vx_core, index: c_int, mut val: c_int) {
    val = if val != 0 { 1 } else { 0 }; /* 0 or 1 */
    /* TODO: preserve C guard(mutex)(&chip->lock) with the repository lock guard. */
    if vx_is_pcmcia(chip) != 0 {
        vx_outb(chip, CSUER, 0); /* write */
        vx_outb(
            chip,
            RUER,
            (val << 7) | (index & XX_UER_CBITS_OFFSET_MASK),
        );
    } else {
        vx_outl(chip, CSUER, 0); /* write */
        vx_outl(
            chip,
            RUER,
            (val << 7) | (index & XX_UER_CBITS_OFFSET_MASK),
        );
    }
}

/*
 * vx_read_uer_status - read the current UER status
 * @mode: pointer to store the UER mode, VX_UER_MODE_XXX
 *
 * returns the frequency of UER, or 0 if not sync,
 * or a negative error code.
 */
unsafe fn vx_read_uer_status(chip: *mut vx_core, mode: *mut c_uint) -> c_int {
    let val: c_int;
    let mut freq: c_int;

    /* Default values */
    freq = 0;

    /* Read UER status */
    if vx_is_pcmcia(chip) != 0 {
        val = vx_inb(chip, CSUER);
    } else {
        val = vx_inl(chip, CSUER);
    }
    if val < 0 {
        return val;
    }
    /* If clock is present, read frequency */
    if (val & VX_SUER_CLOCK_PRESENT_MASK) != 0 {
        match val & VX_SUER_FREQ_MASK {
            VX_SUER_FREQ_32KHz_MASK => {
                freq = 32000;
            }
            VX_SUER_FREQ_44KHz_MASK => {
                freq = 44100;
            }
            VX_SUER_FREQ_48KHz_MASK => {
                freq = 48000;
            }
            _ => {}
        }
    }
    if (val & VX_SUER_DATA_PRESENT_MASK) != 0 {
        /* bit 0 corresponds to consumer/professional bit */
        *mode = if vx_read_one_cbit(chip, 0) != 0 {
            VX_UER_MODE_PROFESSIONAL
        } else {
            VX_UER_MODE_CONSUMER
        };
    } else {
        *mode = VX_UER_MODE_NOT_PRESENT;
    }

    freq
}

/*
 * compute the sample clock value from frequency
 *
 * The formula is as follows:
 *
 *    HexFreq = (dword) ((double) ((double) 28224000 / (double) Frequency))
 *    switch ( HexFreq & 0x00000F00 )
 *    case 0x00000100: ;
 *    case 0x00000200:
 *    case 0x00000300: HexFreq -= 0x00000201 ;
 *    case 0x00000400:
 *    case 0x00000500:
 *    case 0x00000600:
 *    case 0x00000700: HexFreq = (dword) (((double) 28224000 / (double) (Frequency*2)) - 1)
 *    default        : HexFreq = (dword) ((double) 28224000 / (double) (Frequency*4)) - 0x000001FF
 */

unsafe fn vx_calc_clock_from_freq(_chip: *mut vx_core, freq: c_int) -> c_int {
    let mut hexfreq: c_int;

    if snd_BUG_ON((freq <= 0) as c_int) != 0 {
        return 0;
    }

    hexfreq = (28224000 * 10) / freq;
    hexfreq = (hexfreq + 5) / 10;

    /* max freq = 55125 Hz */
    if snd_BUG_ON((hexfreq <= 0x00000200) as c_int) != 0 {
        return 0;
    }

    if hexfreq <= 0x03ff {
        return hexfreq - 0x00000201;
    }
    if hexfreq <= 0x07ff {
        return (hexfreq / 2) - 1;
    }
    if hexfreq <= 0x0fff {
        return (hexfreq / 4) + 0x000001ff;
    }

    0x5fe /* min freq = 6893 Hz */
}

/*
 * vx_change_clock_source - change the clock source
 * @source: the new source
 */
unsafe fn vx_change_clock_source(chip: *mut vx_core, source: c_int) {
    /* we mute DAC to prevent clicks */
    vx_toggle_dac_mute(chip, 1);
    /* TODO: preserve C scoped_guard(mutex, &chip->lock) with the repository lock guard. */
    ((*(*chip).ops).set_clock_source).expect("set_clock_source")(chip, source);
    (*chip).clock_source = source;
    /* unmute */
    vx_toggle_dac_mute(chip, 0);
}

/*
 * set the internal clock
 */
#[no_mangle]
pub unsafe extern "C" fn vx_set_internal_clock(chip: *mut vx_core, freq: c_uint) {
    let clock: c_int;

    /* Get real clock value */
    clock = vx_calc_clock_from_freq(chip, freq as c_int);
    dev_dbg(
        (*(*chip).card).dev,
        c"set internal clock to 0x%x from freq %d\n".as_ptr(),
        clock,
        freq,
    );
    /* TODO: preserve C guard(mutex)(&chip->lock) with the repository lock guard. */
    if vx_is_pcmcia(chip) != 0 {
        vx_outb(chip, HIFREQ, (clock >> 8) & 0x0f);
        vx_outb(chip, LOFREQ, clock & 0xff);
    } else {
        vx_outl(chip, HIFREQ, (clock >> 8) & 0x0f);
        vx_outl(chip, LOFREQ, clock & 0xff);
    }
}

/*
 * set the iec958 status bits
 * @bits: 32-bit status bits
 */
#[no_mangle]
pub unsafe extern "C" fn vx_set_iec958_status(chip: *mut vx_core, bits: c_uint) {
    let mut i: c_int;

    if ((*chip).chip_status & VX_STAT_IS_STALE) != 0 {
        return;
    }

    i = 0;
    while i < 32 {
        vx_write_one_cbit(chip, i, (bits & (1u32 << i)) as c_int);
        i += 1;
    }
}

/*
 * vx_set_clock - change the clock and audio source if necessary
 */
#[no_mangle]
pub unsafe extern "C" fn vx_set_clock(chip: *mut vx_core, freq: c_uint) -> c_int {
    let mut src_changed: c_int = 0;

    if ((*chip).chip_status & VX_STAT_IS_STALE) != 0 {
        return 0;
    }

    /* change the audio source if possible */
    vx_sync_audio_source(chip);

    if (*chip).clock_mode == VX_CLOCK_MODE_EXTERNAL
        || ((*chip).clock_mode == VX_CLOCK_MODE_AUTO
            && (*chip).audio_source == VX_AUDIO_SRC_DIGITAL)
    {
        if (*chip).clock_source != UER_SYNC {
            vx_change_clock_source(chip, UER_SYNC);
            mdelay(6);
            src_changed = 1;
        }
    } else if (*chip).clock_mode == VX_CLOCK_MODE_INTERNAL
        || ((*chip).clock_mode == VX_CLOCK_MODE_AUTO
            && (*chip).audio_source != VX_AUDIO_SRC_DIGITAL)
    {
        if (*chip).clock_source != INTERNAL_QUARTZ {
            vx_change_clock_source(chip, INTERNAL_QUARTZ);
            src_changed = 1;
        }
        if (*chip).freq == freq {
            return 0;
        }
        vx_set_internal_clock(chip, freq);
        if src_changed != 0 {
            vx_modify_board_inputs(chip);
        }
    }
    if (*chip).freq == freq {
        return 0;
    }
    (*chip).freq = freq;
    vx_modify_board_clock(chip, 1);
    0
}

/*
 * vx_change_frequency - called from interrupt handler
 */
#[no_mangle]
pub unsafe extern "C" fn vx_change_frequency(chip: *mut vx_core) -> c_int {
    let freq: c_int;

    if ((*chip).chip_status & VX_STAT_IS_STALE) != 0 {
        return 0;
    }

    if (*chip).clock_source == INTERNAL_QUARTZ {
        return 0;
    }
    /*
     * Read the real UER board frequency
     */
    freq = vx_read_uer_status(chip, &mut (*chip).uer_detected);
    if freq < 0 {
        return freq;
    }
    /*
     * The frequency computed by the DSP is good and
     * is different from the previous computed.
     */
    if freq == 48000 || freq == 44100 || freq == 32000 {
        (*chip).freq_detected = freq as c_uint;
    }

    0
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
