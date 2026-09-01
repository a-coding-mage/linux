// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *  Patch routines for the emu8000 (AWE32/64)
 *
 *  Copyright (C) 1999 Steve Ratcliffe
 *  Copyright (C) 1999-2000 Takashi Iwai <tiwai@suse.de>
 */

// C dependencies: "emu8000_local.h", <linux/sched/signal.h>,
// <linux/uaccess.h>, <linux/moduleparam.h>

static mut emu8000_reset_addr: i32 = 0;
// module_param(emu8000_reset_addr, int, 0444);
// MODULE_PARM_DESC(emu8000_reset_addr, "reset write address at each time (makes slowdown)");

/*
 * Open up channels.
 */
unsafe fn snd_emu8000_open_dma(emu: *mut snd_emu8000, write: i32) -> i32 {
    let mut i: i32;

    /* reserve all 30 voices for loading */
    i = 0;
    while i < EMU8000_DRAM_VOICES {
        snd_emux_lock_voice((*emu).emu, i);
        snd_emu8000_dma_chan(emu, i, write);
        i += 1;
    }

    /* assign voice 31 and 32 to ROM */
    EMU8000_VTFT_WRITE(emu, 30, 0);
    EMU8000_PSST_WRITE(emu, 30, 0x1d8);
    EMU8000_CSL_WRITE(emu, 30, 0x1e0);
    EMU8000_CCCA_WRITE(emu, 30, 0x1d8);
    EMU8000_VTFT_WRITE(emu, 31, 0);
    EMU8000_PSST_WRITE(emu, 31, 0x1d8);
    EMU8000_CSL_WRITE(emu, 31, 0x1e0);
    EMU8000_CCCA_WRITE(emu, 31, 0x1d8);

    0
}

/*
 * Close all dram channels.
 */
unsafe fn snd_emu8000_close_dma(emu: *mut snd_emu8000) {
    let mut i: i32;

    i = 0;
    while i < EMU8000_DRAM_VOICES {
        snd_emu8000_dma_chan(emu, i, EMU8000_RAM_CLOSE);
        snd_emux_unlock_voice((*emu).emu, i);
        i += 1;
    }
}

/*
 */

const BLANK_LOOP_START: i32 = 4;
const BLANK_LOOP_END: i32 = 8;
const BLANK_LOOP_SIZE: i32 = 12;
const BLANK_HEAD_SIZE: i32 = 48;

/*
 * Read a word from userland, taking care of conversions from
 * 8bit samples etc.
 */
unsafe fn read_word(buf: *const core::ffi::c_void, offset: i32, mode: i32) -> u16 {
    let mut c: u16;
    if (mode & SNDRV_SFNT_SAMPLE_8BITS) != 0 {
        let mut cc: u8 = 0;
        get_user_u8(&mut cc, (buf as *const u8).offset(offset as isize));
        c = (cc as u16) << 8; /* convert 8bit -> 16bit */
    } else {
        // Original C uses the SNDRV_LITTLE_ENDIAN build-time condition here.
        #[cfg(target_endian = "little")]
        {
            c = 0;
            get_user_u16(&mut c, (buf as *const u16).byte_offset(offset as isize));
        }
        #[cfg(not(target_endian = "little"))]
        {
            let mut cc: u16 = 0;
            get_user_u16(&mut cc, (buf as *const u16).byte_offset(offset as isize));
            c = swab16(cc);
        }
    }
    if (mode & SNDRV_SFNT_SAMPLE_UNSIGNED) != 0 {
        c ^= 0x8000; /* unsigned -> signed */
    }
    c
}

/*
 */
unsafe fn snd_emu8000_write_wait(emu: *mut snd_emu8000) {
    while (EMU8000_SMALW_READ(emu) & 0x80000000) != 0 {
        schedule_timeout_interruptible(1);
        if signal_pending(current) != 0 {
            break;
        }
    }
}

/*
 * write sample word data
 *
 * You should not have to keep resetting the address each time
 * as the chip is supposed to step on the next address automatically.
 * It mostly does, but during writes of some samples at random it
 * completely loses words (every one in 16 roughly but with no
 * obvious pattern).
 *
 * This is therefore much slower than need be, but is at least
 * working.
 */
unsafe fn write_word(emu: *mut snd_emu8000, offset: *mut i32, data: u16) {
    if emu8000_reset_addr != 0 {
        if emu8000_reset_addr > 1 {
            snd_emu8000_write_wait(emu);
        }
        EMU8000_SMALW_WRITE(emu, *offset);
    }
    EMU8000_SMLD_WRITE(emu, data);
    *offset += 1;
}

/*
 * Write the sample to EMU800 memory.  This routine is invoked out of
 * the generic soundfont routines as a callback.
 */
pub unsafe extern "C" fn snd_emu8000_sample_new(
    rec: *mut snd_emux,
    sp: *mut snd_sf_sample,
    hdr: *mut snd_util_memhdr,
    data: *const core::ffi::c_void,
    count: i64,
) -> i32 {
    let mut i: i32;
    let mut rc: i32;
    let mut offset: i32;
    let mut truesize: i32;
    let mut dram_offset: i32;
    let dram_start: i32;
    let emu: *mut snd_emu8000;

    emu = (*rec).hw as *mut snd_emu8000;
    if snd_BUG_ON(sp.is_null()) != 0 {
        return -EINVAL;
    }

    /* compute true data size to be loaded */
    truesize = (*sp).v.size;
    if ((*sp).v.mode_flags & (SNDRV_SFNT_SAMPLE_BIDIR_LOOP | SNDRV_SFNT_SAMPLE_REVERSE_LOOP)) != 0 {
        truesize += (*sp).v.loopend - (*sp).v.loopstart;
    }
    if ((*sp).v.mode_flags & SNDRV_SFNT_SAMPLE_NO_BLANK) != 0 {
        truesize += BLANK_LOOP_SIZE;
    }

    (*sp).block = snd_util_mem_alloc(hdr, truesize * 2);
    if (*sp).block == core::ptr::null_mut() {
        /* not ENOMEM (for compatibility) */
        return -ENOSPC;
    }

    if ((*sp).v.mode_flags & SNDRV_SFNT_SAMPLE_8BITS) != 0 {
        if access_ok(data, (*sp).v.size as usize) == 0 {
            return -EFAULT;
        }
    } else if access_ok(data, ((*sp).v.size * 2) as usize) == 0 {
        return -EFAULT;
    }

    /* dram position (in word) -- mem_offset is byte */
    dram_offset = EMU8000_DRAM_OFFSET + ((*(*sp).block).offset >> 1);
    dram_start = dram_offset;

    /* set the total size (store onto obsolete checksum value) */
    (*sp).v.truesize = truesize * 2; /* in bytes */

    snd_emux_terminate_all((*emu).emu);
    rc = snd_emu8000_open_dma(emu, EMU8000_RAM_WRITE);
    if rc != 0 {
        return rc;
    }

    /* Set the address to start writing at */
    snd_emu8000_write_wait(emu);
    EMU8000_SMALW_WRITE(emu, dram_offset);

    /*snd_emu8000_init_fm(emu);*/

    // Original C has this block disabled with #if 0.
    /*
    if (*(*sp).block).offset == 0 {
        i = 0;
        while i < BLANK_HEAD_SIZE {
            write_word(emu, &mut dram_offset, 0);
            i += 1;
        }
    }
    */

    offset = 0;
    i = 0;
    while i < (*sp).v.size {
        let mut s: u16;

        s = read_word(data, offset, (*sp).v.mode_flags);
        offset += 1;
        write_word(emu, &mut dram_offset, s);

        /* we may take too long time in this loop.
         * so give controls back to kernel if needed.
         */
        cond_resched();

        if i == (*sp).v.loopend
            && ((*sp).v.mode_flags
                & (SNDRV_SFNT_SAMPLE_BIDIR_LOOP | SNDRV_SFNT_SAMPLE_REVERSE_LOOP))
                != 0
        {
            let looplen: i32 = (*sp).v.loopend - (*sp).v.loopstart;
            let mut k: i32;

            /* copy reverse loop */
            k = 1;
            while k <= looplen {
                s = read_word(data, offset - k, (*sp).v.mode_flags);
                write_word(emu, &mut dram_offset, s);
                k += 1;
            }
            if ((*sp).v.mode_flags & SNDRV_SFNT_SAMPLE_BIDIR_LOOP) != 0 {
                (*sp).v.loopend += looplen;
            } else {
                (*sp).v.loopstart += looplen;
                (*sp).v.loopend += looplen;
            }
            (*sp).v.end += looplen;
        }
        i += 1;
    }

    /* if no blank loop is attached in the sample, add it */
    if ((*sp).v.mode_flags & SNDRV_SFNT_SAMPLE_NO_BLANK) != 0 {
        i = 0;
        while i < BLANK_LOOP_SIZE {
            write_word(emu, &mut dram_offset, 0);
            i += 1;
        }
        if ((*sp).v.mode_flags & SNDRV_SFNT_SAMPLE_SINGLESHOT) != 0 {
            (*sp).v.loopstart = (*sp).v.end + BLANK_LOOP_START;
            (*sp).v.loopend = (*sp).v.end + BLANK_LOOP_END;
        }
    }

    /* add dram offset */
    (*sp).v.start += dram_start;
    (*sp).v.end += dram_start;
    (*sp).v.loopstart += dram_start;
    (*sp).v.loopend += dram_start;

    snd_emu8000_close_dma(emu);
    snd_emu8000_init_fm(emu);

    0
}

/*
 * free a sample block
 */
pub unsafe extern "C" fn snd_emu8000_sample_free(
    rec: *mut snd_emux,
    sp: *mut snd_sf_sample,
    hdr: *mut snd_util_memhdr,
) -> i32 {
    if !(*sp).block.is_null() {
        snd_util_mem_free(hdr, (*sp).block);
        (*sp).block = core::ptr::null_mut();
    }
    0
}

/*
 * sample_reset callback - terminate voices
 */
pub unsafe extern "C" fn snd_emu8000_sample_reset(rec: *mut snd_emux) {
    snd_emux_terminate_all(rec);
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
