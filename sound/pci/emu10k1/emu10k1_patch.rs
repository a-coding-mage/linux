// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *  Patch transfer callback for Emu10k1
 *
 *  Copyright (C) 2000 Takashi iwai <tiwai@suse.de>
 */
/*
 * All the code for loading in a patch.  There is very little that is
 * chip specific here.  Just the actual writing to the board.
 */

// Rust translation of dependency intent from:
// #include "emu10k1_synth_local.h"

use core::ffi::{c_int, c_long, c_uint, c_void};

const BLANK_LOOP_START: c_int = 4;
const BLANK_LOOP_END: c_int = 8;
const BLANK_LOOP_SIZE: c_int = 12;
const BLANK_HEAD_SIZE: c_int = 3;

type u8 = u8;
type u32 = u32;

#[repr(C)]
pub struct snd_emux {
    pub hw: *mut snd_emu10k1,
}

#[repr(C)]
pub struct snd_sf_sample {
    pub v: snd_soundfont_sample_info,
    pub block: *mut snd_util_memblk,
}

#[repr(C)]
pub struct snd_soundfont_sample_info {
    pub sample: c_uint,
    pub start: c_int,
    pub end: c_int,
    pub loopstart: c_int,
    pub loopend: c_int,
    pub size: c_int,
    pub mode_flags: c_uint,
    pub truesize: c_int,
}

#[repr(C)]
pub struct snd_util_memhdr {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_util_memblk {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_emu10k1 {
    pub card: *mut snd_card,
}

#[repr(C)]
pub struct snd_card {
    pub dev: *mut device,
}

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

extern "C" {
    static SNDRV_SFNT_SAMPLE_BIDIR_LOOP: c_uint;
    static SNDRV_SFNT_SAMPLE_REVERSE_LOOP: c_uint;
    static SNDRV_SFNT_SAMPLE_8BITS: c_uint;
    static SNDRV_SFNT_SAMPLE_UNSIGNED: c_uint;
    static SNDRV_SFNT_SAMPLE_NO_BLANK: c_uint;
    static SNDRV_SFNT_SAMPLE_SINGLESHOT: c_uint;

    static EINVAL: c_int;
    static ENOSPC: c_int;
    static EFAULT: c_int;

    fn snd_BUG_ON(cond: bool) -> bool;
    fn dev_warn(dev: *mut device, fmt: *const i8, ...);
    fn dev_dbg(dev: *mut device, fmt: *const i8, ...);
    fn snd_emu10k1_synth_alloc(emu: *mut snd_emu10k1, size: c_int) -> *mut snd_util_memblk;
    fn snd_emu10k1_synth_free(emu: *mut snd_emu10k1, block: *mut snd_util_memblk);
    fn snd_emu10k1_synth_memset(
        emu: *mut snd_emu10k1,
        block: *mut snd_util_memblk,
        offset: c_int,
        size: c_int,
        fill: u8,
    );
    fn snd_emu10k1_synth_copy_from_user(
        emu: *mut snd_emu10k1,
        block: *mut snd_util_memblk,
        offset: c_int,
        data: *const c_void,
        size: c_int,
        xor: u32,
    ) -> c_int;
}

/*
 * allocate a sample block and copy data from userspace
 */
#[no_mangle]
pub unsafe extern "C" fn snd_emu10k1_sample_new(
    rec: *mut snd_emux,
    sp: *mut snd_sf_sample,
    hdr: *mut snd_util_memhdr,
    mut data: *const c_void,
    _count: c_long,
) -> c_int {
    let fill: u8;
    let xor: u32;
    let shift: c_int;
    let mut offset: c_int;
    let mut truesize: c_int;
    let mut size: c_int;
    let blocksize: c_int;
    let loop_start: c_int;
    let loop_end: c_int;
    let loop_size: c_int;
    let data_end: c_int;
    let mut unroll: c_int;
    let emu: *mut snd_emu10k1;

    emu = (*rec).hw;
    if snd_BUG_ON(sp.is_null() || hdr.is_null()) {
        return -EINVAL;
    }

    if (*sp).v.mode_flags & (SNDRV_SFNT_SAMPLE_BIDIR_LOOP | SNDRV_SFNT_SAMPLE_REVERSE_LOOP) != 0 {
        /* should instead return -ENOTSUPP; but compatibility */
        dev_warn(
            (*(*emu).card).dev,
            b"Emu10k1 wavetable patch %d with unsupported loop feature\n\0".as_ptr() as *const i8,
            (*sp).v.sample,
        );
    }

    if (*sp).v.mode_flags & SNDRV_SFNT_SAMPLE_8BITS != 0 {
        shift = 0;
        fill = 0x80;
        xor = if (*sp).v.mode_flags & SNDRV_SFNT_SAMPLE_UNSIGNED != 0 {
            0
        } else {
            0x80808080
        };
    } else {
        shift = 1;
        fill = 0;
        xor = if (*sp).v.mode_flags & SNDRV_SFNT_SAMPLE_UNSIGNED != 0 {
            0x80008000
        } else {
            0
        };
    }

    /* compute true data size to be loaded */
    truesize = (*sp).v.size + BLANK_HEAD_SIZE;
    if (*sp).v.mode_flags & SNDRV_SFNT_SAMPLE_NO_BLANK != 0 {
        truesize += BLANK_LOOP_SIZE;
        /* if no blank loop is attached in the sample, add it */
        if (*sp).v.mode_flags & SNDRV_SFNT_SAMPLE_SINGLESHOT != 0 {
            (*sp).v.loopstart = (*sp).v.end + BLANK_LOOP_START;
            (*sp).v.loopend = (*sp).v.end + BLANK_LOOP_END;
        }
    }

    loop_start = (*sp).v.loopstart;
    loop_end = (*sp).v.loopend;
    loop_size = loop_end - loop_start;
    if loop_size == 0 {
        return -EINVAL;
    }
    data_end = (*sp).v.end;

    /* recalculate offset */
    (*sp).v.start += BLANK_HEAD_SIZE;
    (*sp).v.end += BLANK_HEAD_SIZE;
    (*sp).v.loopstart += BLANK_HEAD_SIZE;
    (*sp).v.loopend += BLANK_HEAD_SIZE;

    // Automatic pre-filling of the cache does not work in the presence
    // of loops (*), and we don't want to fill it manually, as that is
    // fiddly and slow. So we unroll the loop until the loop end is
    // beyond the cache size.
    // (*) Strictly speaking, a single iteration is supported (that's
    // how it works when the playback engine runs), but handling this
    // special case is not worth it.
    unroll = 0;
    while (*sp).v.loopend < 64 {
        truesize += loop_size;
        (*sp).v.loopstart += loop_size;
        (*sp).v.loopend += loop_size;
        (*sp).v.end += loop_size;
        unroll += 1;
    }

    /* try to allocate a memory block */
    blocksize = truesize << shift;
    (*sp).block = snd_emu10k1_synth_alloc(emu, blocksize);
    if (*sp).block.is_null() {
        dev_dbg(
            (*(*emu).card).dev,
            b"synth malloc failed (size=%d)\n\0".as_ptr() as *const i8,
            blocksize,
        );
        /* not ENOMEM (for compatibility with OSS) */
        return -ENOSPC;
    }
    /* set the total size */
    (*sp).v.truesize = blocksize;

    /* write blank samples at head */
    offset = 0;
    size = BLANK_HEAD_SIZE << shift;
    snd_emu10k1_synth_memset(emu, (*sp).block, offset, size, fill);
    offset += size;

    /* copy provided samples */
    if unroll != 0 && loop_end <= data_end {
        size = loop_end << shift;
        if snd_emu10k1_synth_copy_from_user(emu, (*sp).block, offset, data, size, xor) != 0 {
            snd_emu10k1_synth_free(emu, (*sp).block);
            (*sp).block = core::ptr::null_mut();
            return -EFAULT;
        }
        offset += size;

        data = (data as *const u8).offset((loop_start << shift) as isize) as *const c_void;
        while {
            unroll -= 1;
            unroll > 0
        } {
            size = loop_size << shift;
            if snd_emu10k1_synth_copy_from_user(emu, (*sp).block, offset, data, size, xor) != 0 {
                snd_emu10k1_synth_free(emu, (*sp).block);
                (*sp).block = core::ptr::null_mut();
                return -EFAULT;
            }
            offset += size;
        }

        size = (data_end - loop_start) << shift;
    } else {
        size = data_end << shift;
    }
    if snd_emu10k1_synth_copy_from_user(emu, (*sp).block, offset, data, size, xor) != 0 {
        snd_emu10k1_synth_free(emu, (*sp).block);
        (*sp).block = core::ptr::null_mut();
        return -EFAULT;
    }
    offset += size;

    /* clear rest of samples (if any) */
    if offset < blocksize {
        snd_emu10k1_synth_memset(emu, (*sp).block, offset, blocksize - offset, fill);
    }

    return 0;
}

/*
 * free a sample block
 */
#[no_mangle]
pub unsafe extern "C" fn snd_emu10k1_sample_free(
    rec: *mut snd_emux,
    sp: *mut snd_sf_sample,
    hdr: *mut snd_util_memhdr,
) -> c_int {
    let emu: *mut snd_emu10k1;

    emu = (*rec).hw;
    if snd_BUG_ON(sp.is_null() || hdr.is_null()) {
        return -EINVAL;
    }

    if !(*sp).block.is_null() {
        snd_emu10k1_synth_free(emu, (*sp).block);
        (*sp).block = core::ptr::null_mut();
    }
    return 0;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
