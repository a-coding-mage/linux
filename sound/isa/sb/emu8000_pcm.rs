// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * pcm emulation on emu8000 wavetable
 *
 *  Copyright (C) 2002 Takashi Iwai <tiwai@suse.de>
 */

// Dependencies from the original C file:
// "emu8000_local.h", <linux/sched/signal.h>, <linux/init.h>,
// <linux/slab.h>, <sound/initval.h>, and <sound/pcm.h>.

/*
 * define the following if you want to use this pcm with non-interleaved mode
 */
/* #define USE_NONINTERLEAVE */

/* NOTE: for using the non-interleaved mode with alsa-lib, you have to set
 * mmap_emulation flag to 1 in your .asoundrc, such like
 *
 *	pcm.emu8k {
 *		type plug
 *		slave.pcm {
 *			type hw
 *			card 0
 *			device 1
 *			mmap_emulation 1
 *		}
 *	}
 *
 * besides, for the time being, the non-interleaved mode doesn't work well on
 * alsa-lib...
 */

use core::ffi::{c_int, c_uint, c_ulong, c_void};
use core::ptr;

#[repr(C)]
pub struct snd_emu8k_pcm {
    emu: *mut snd_emu8000,
    substream: *mut snd_pcm_substream,

    allocated_bytes: c_uint,
    block: *mut snd_util_memblk,
    offset: c_uint,
    buf_size: c_uint,
    period_size: c_uint,
    loop_start: [c_uint; 2],
    pitch: c_uint,
    panning: [c_int; 2],
    last_ptr: c_int,
    period_pos: c_int,
    voices: c_int,
    dram_opened: c_uint,
    running: c_uint,
    timer_running: c_uint,
    timer: timer_list,
    timer_lock: spinlock_t,
}

const LOOP_BLANK_SIZE: c_uint = 8;

/*
 * open up channels for the simultaneous data transfer and playback
 */
unsafe fn emu8k_open_dram_for_pcm(emu: *mut snd_emu8000, channels: c_int) -> c_int {
    let mut i: c_int;

    /* reserve up to 2 voices for playback */
    snd_emux_lock_voice((*emu).emu, 0);
    if channels > 1 {
        snd_emux_lock_voice((*emu).emu, 1);
    }

    /* reserve 28 voices for loading */
    i = channels + 1;
    while i < EMU8000_DRAM_VOICES {
        let mut mode: c_uint = EMU8000_RAM_WRITE;
        snd_emux_lock_voice((*emu).emu, i);
        // #ifndef USE_NONINTERLEAVE
        if channels > 1 && (i & 1) != 0 {
            mode |= EMU8000_RAM_RIGHT;
        }
        // #endif
        snd_emu8000_dma_chan(emu, i, mode);
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
 */
unsafe fn snd_emu8000_write_wait(emu: *mut snd_emu8000, can_schedule: c_int) {
    while (EMU8000_SMALW_READ(emu) & 0x80000000) != 0 {
        if can_schedule != 0 {
            schedule_timeout_interruptible(1);
            if signal_pending(current) != 0 {
                break;
            }
        }
    }
}

/*
 * close all channels
 */
unsafe fn emu8k_close_dram(emu: *mut snd_emu8000) {
    let mut i: c_int = 0;

    while i < 2 {
        snd_emux_unlock_voice((*emu).emu, i);
        i += 1;
    }
    while i < EMU8000_DRAM_VOICES {
        snd_emu8000_dma_chan(emu, i, EMU8000_RAM_CLOSE);
        snd_emux_unlock_voice((*emu).emu, i);
        i += 1;
    }
}

/*
 * convert Hz to AWE32 rate offset (see emux/soundfont.c)
 */

const OFFSET_SAMPLERATE: c_int = 1011119; /* base = 44100 */
const SAMPLERATE_RATIO: c_int = 4096;

unsafe fn calc_rate_offset(hz: c_int) -> c_int {
    snd_sf_linear_to_log(hz, OFFSET_SAMPLERATE, SAMPLERATE_RATIO)
}

/*
 */

static emu8k_pcm_hw: snd_pcm_hardware = snd_pcm_hardware {
    // #ifdef USE_NONINTERLEAVE: SNDRV_PCM_INFO_NONINTERLEAVED
    info: SNDRV_PCM_INFO_INTERLEAVED,
    formats: SNDRV_PCM_FMTBIT_S16_LE,
    rates: SNDRV_PCM_RATE_CONTINUOUS | SNDRV_PCM_RATE_8000_48000,
    rate_min: 4000,
    rate_max: 48000,
    channels_min: 1,
    channels_max: 2,
    buffer_bytes_max: 128 * 1024,
    period_bytes_min: 1024,
    period_bytes_max: 128 * 1024,
    periods_min: 2,
    periods_max: 1024,
    fifo_size: 0,
};

/*
 * get the current position at the given channel from CCCA register
 */
unsafe fn emu8k_get_curpos(rec: *mut snd_emu8k_pcm, ch: c_int) -> c_int {
    let mut val: c_int = (EMU8000_CCCA_READ((*rec).emu, ch) & 0xfffffff) as c_int;
    val -= ((*rec).loop_start[ch as usize] - 1) as c_int;
    val
}

/*
 * timer interrupt handler
 * check the current position and update the period if necessary.
 */
unsafe extern "C" fn emu8k_pcm_timer_func(t: *mut timer_list) {
    let rec: *mut snd_emu8k_pcm = timer_container_of(t);
    let mut ptr_: c_int;
    let mut delta: c_int;
    let mut period_elapsed: bool = false;

    spin_lock(&mut (*rec).timer_lock);
    /* update the current pointer */
    ptr_ = emu8k_get_curpos(rec, 0);
    if ptr_ < (*rec).last_ptr {
        delta = ptr_ + (*rec).buf_size as c_int - (*rec).last_ptr;
    } else {
        delta = ptr_ - (*rec).last_ptr;
    }
    (*rec).period_pos += delta;
    (*rec).last_ptr = ptr_;

    /* reprogram timer */
    mod_timer(&mut (*rec).timer, jiffies + 1);

    /* update period */
    if (*rec).period_pos >= (*rec).period_size as c_int {
        (*rec).period_pos %= (*rec).period_size as c_int;
        period_elapsed = true;
    }
    spin_unlock(&mut (*rec).timer_lock);

    if period_elapsed {
        snd_pcm_period_elapsed((*rec).substream);
    }
}

/*
 * open pcm
 * creating an instance here
 */
unsafe extern "C" fn emu8k_pcm_open(subs: *mut snd_pcm_substream) -> c_int {
    let emu: *mut snd_emu8000 = snd_pcm_substream_chip(subs);
    let rec: *mut snd_emu8k_pcm;
    let runtime: *mut snd_pcm_runtime = (*subs).runtime;

    rec = kzalloc(core::mem::size_of::<snd_emu8k_pcm>(), GFP_KERNEL) as *mut snd_emu8k_pcm;
    if rec.is_null() {
        return -ENOMEM;
    }

    (*rec).emu = emu;
    (*rec).substream = subs;
    (*runtime).private_data = rec as *mut c_void;

    spin_lock_init(&mut (*rec).timer_lock);
    timer_setup(&mut (*rec).timer, Some(emu8k_pcm_timer_func), 0);

    (*runtime).hw = emu8k_pcm_hw;
    (*runtime).hw.buffer_bytes_max = (*emu).mem_size - LOOP_BLANK_SIZE * 3;
    (*runtime).hw.period_bytes_max = (*runtime).hw.buffer_bytes_max / 2;

    /* use timer to update periods.. (specified in msec) */
    snd_pcm_hw_constraint_minmax(
        runtime,
        SNDRV_PCM_HW_PARAM_PERIOD_TIME,
        DIV_ROUND_UP(1000000, HZ),
        UINT_MAX,
    );

    0
}

unsafe extern "C" fn emu8k_pcm_close(subs: *mut snd_pcm_substream) -> c_int {
    let rec: *mut snd_emu8k_pcm = (*(*subs).runtime).private_data as *mut snd_emu8k_pcm;
    kfree(rec as *mut c_void);
    (*(*subs).runtime).private_data = ptr::null_mut();
    0
}

/*
 * calculate pitch target
 */
fn calc_pitch_target(pitch: c_int) -> c_int {
    let mut ptarget: c_int = 1 << (pitch >> 12);
    if (pitch & 0x800) != 0 {
        ptarget += (ptarget * 0x102e) / 0x2710;
    }
    if (pitch & 0x400) != 0 {
        ptarget += (ptarget * 0x764) / 0x2710;
    }
    if (pitch & 0x200) != 0 {
        ptarget += (ptarget * 0x389) / 0x2710;
    }
    ptarget += ptarget >> 1;
    if ptarget > 0xffff {
        ptarget = 0xffff;
    }
    ptarget
}

/*
 * set up the voice
 */
unsafe fn setup_voice(rec: *mut snd_emu8k_pcm, ch: c_int) {
    let hw: *mut snd_emu8000 = (*rec).emu;
    let mut temp: c_uint;

    /* channel to be silent and idle */
    EMU8000_DCYSUSV_WRITE(hw, ch, 0x0080);
    EMU8000_VTFT_WRITE(hw, ch, 0x0000FFFF);
    EMU8000_CVCF_WRITE(hw, ch, 0x0000FFFF);
    EMU8000_PTRX_WRITE(hw, ch, 0);
    EMU8000_CPF_WRITE(hw, ch, 0);

    /* pitch offset */
    EMU8000_IP_WRITE(hw, ch, (*rec).pitch);
    /* set envelope parameters */
    EMU8000_ENVVAL_WRITE(hw, ch, 0x8000);
    EMU8000_ATKHLD_WRITE(hw, ch, 0x7f7f);
    EMU8000_DCYSUS_WRITE(hw, ch, 0x7f7f);
    EMU8000_ENVVOL_WRITE(hw, ch, 0x8000);
    EMU8000_ATKHLDV_WRITE(hw, ch, 0x7f7f);
    /* decay/sustain parameter for volume envelope is used
       for triggerg the voice */
    /* modulation envelope heights */
    EMU8000_PEFE_WRITE(hw, ch, 0x0);
    /* lfo1/2 delay */
    EMU8000_LFO1VAL_WRITE(hw, ch, 0x8000);
    EMU8000_LFO2VAL_WRITE(hw, ch, 0x8000);
    /* lfo1 pitch & cutoff shift */
    EMU8000_FMMOD_WRITE(hw, ch, 0);
    /* lfo1 volume & freq */
    EMU8000_TREMFRQ_WRITE(hw, ch, 0);
    /* lfo2 pitch & freq */
    EMU8000_FM2FRQ2_WRITE(hw, ch, 0);
    /* pan & loop start */
    temp = (*rec).panning[ch as usize] as c_uint;
    temp = (temp << 24) | ((*rec).loop_start[ch as usize] - 1);
    EMU8000_PSST_WRITE(hw, ch, temp);
    /* chorus & loop end (chorus 8bit, MSB) */
    temp = 0; // chorus
    temp = (temp << 24) | ((*rec).loop_start[ch as usize] + (*rec).buf_size - 1);
    EMU8000_CSL_WRITE(hw, ch, temp);
    /* Q & current address (Q 4bit value, MSB) */
    temp = 0; // filterQ
    temp = (temp << 28) | ((*rec).loop_start[ch as usize] - 1);
    EMU8000_CCCA_WRITE(hw, ch, temp);
    /* clear unknown registers */
    EMU8000_00A0_WRITE(hw, ch, 0);
    EMU8000_0080_WRITE(hw, ch, 0);
}

/*
 * trigger the voice
 */
unsafe fn start_voice(rec: *mut snd_emu8k_pcm, ch: c_int) {
    let hw: *mut snd_emu8000 = (*rec).emu;
    let mut temp: c_uint;
    let aux: c_uint;
    let pt: c_int = calc_pitch_target((*rec).pitch as c_int);

    /* cutoff and volume */
    EMU8000_IFATN_WRITE(hw, ch, 0xff00);
    EMU8000_VTFT_WRITE(hw, ch, 0xffff);
    EMU8000_CVCF_WRITE(hw, ch, 0xffff);
    /* trigger envelope */
    EMU8000_DCYSUSV_WRITE(hw, ch, 0x7f7f);
    /* set reverb and pitch target */
    temp = 0; // reverb
    if (*rec).panning[ch as usize] == 0 {
        aux = 0xff;
    } else {
        aux = (-(*rec).panning[ch as usize] & 0xff) as c_uint;
    }
    temp = (temp << 8) | ((pt as c_uint) << 16) | aux;
    EMU8000_PTRX_WRITE(hw, ch, temp);
    EMU8000_CPF_WRITE(hw, ch, (pt as c_uint) << 16);

    /* start timer */
    spin_lock_irqsave(&mut (*rec).timer_lock);
    if (*rec).timer_running == 0 {
        mod_timer(&mut (*rec).timer, jiffies + 1);
        (*rec).timer_running = 1;
    }
    spin_unlock_irqrestore(&mut (*rec).timer_lock);
}

/*
 * stop the voice immediately
 */
unsafe fn stop_voice(rec: *mut snd_emu8k_pcm, ch: c_int) {
    let hw: *mut snd_emu8000 = (*rec).emu;

    EMU8000_DCYSUSV_WRITE(hw, ch, 0x807F);

    /* stop timer */
    spin_lock_irqsave(&mut (*rec).timer_lock);
    if (*rec).timer_running != 0 {
        timer_delete(&mut (*rec).timer);
        (*rec).timer_running = 0;
    }
    spin_unlock_irqrestore(&mut (*rec).timer_lock);
}

unsafe extern "C" fn emu8k_pcm_trigger(subs: *mut snd_pcm_substream, cmd: c_int) -> c_int {
    let rec: *mut snd_emu8k_pcm = (*(*subs).runtime).private_data as *mut snd_emu8k_pcm;
    let mut ch: c_int;

    match cmd {
        SNDRV_PCM_TRIGGER_START => {
            ch = 0;
            while ch < (*rec).voices {
                start_voice(rec, ch);
                ch += 1;
            }
            (*rec).running = 1;
        }
        SNDRV_PCM_TRIGGER_STOP => {
            (*rec).running = 0;
            ch = 0;
            while ch < (*rec).voices {
                stop_voice(rec, ch);
                ch += 1;
            }
        }
        _ => return -EINVAL,
    }
    0
}

/*
 * copy / silence ops
 */

/*
 * this macro should be inserted in the copy/silence loops
 * to reduce the latency.  without this, the system will hang up
 * during the whole loop.
 */
unsafe fn CHECK_SCHEDULER() -> c_int {
    cond_resched();
    if signal_pending(current) != 0 {
        return -EAGAIN;
    }
    0
}

unsafe fn GET_VAL(sval: *mut c_ushort, iter: *mut iov_iter) -> c_int {
    if iter.is_null() {
        *sval = 0;
    } else if copy_from_iter(sval as *mut c_void, 2, iter) != 2 {
        return -EFAULT;
    }
    0
}

type c_ushort = u16;

// #ifdef USE_NONINTERLEAVE
// The original non-interleaved implementation is preserved here as conditional
// intent. The active translation follows the default interleaved branch below.

unsafe fn LOOP_WRITE(
    rec: *mut snd_emu8k_pcm,
    mut pos: c_ulong,
    iter: *mut iov_iter,
    mut count: c_ulong,
) -> c_int {
    let emu: *mut snd_emu8000 = (*rec).emu;
    snd_emu8000_write_wait(emu, 1);
    EMU8000_SMALW_WRITE(emu, pos as c_uint + (*rec).loop_start[0]);
    if (*rec).voices > 1 {
        EMU8000_SMARW_WRITE(emu, pos as c_uint + (*rec).loop_start[1]);
    }
    while count > 0 {
        let mut sval: c_ushort = 0;
        let mut err: c_int;
        err = CHECK_SCHEDULER();
        if err != 0 {
            return err;
        }
        err = GET_VAL(&mut sval, iter);
        if err != 0 {
            return err;
        }
        EMU8000_SMLD_WRITE(emu, sval);
        if (*rec).voices > 1 {
            err = CHECK_SCHEDULER();
            if err != 0 {
                return err;
            }
            err = GET_VAL(&mut sval, iter);
            if err != 0 {
                return err;
            }
            EMU8000_SMRD_WRITE(emu, sval);
        }
        count -= 1;
    }
    0
}

/*
 * copy the interleaved data can be done easily by using
 * DMA "left" and "right" channels on emu8k engine.
 */
unsafe extern "C" fn emu8k_pcm_copy(
    subs: *mut snd_pcm_substream,
    _voice: c_int,
    mut pos: c_ulong,
    src: *mut iov_iter,
    mut count: c_ulong,
) -> c_int {
    let rec: *mut snd_emu8k_pcm = (*(*subs).runtime).private_data as *mut snd_emu8k_pcm;

    /* convert to frames */
    pos = bytes_to_frames((*subs).runtime, pos);
    count = bytes_to_frames((*subs).runtime, count);
    LOOP_WRITE(rec, pos, src, count);
    0
}

unsafe extern "C" fn emu8k_pcm_silence(
    subs: *mut snd_pcm_substream,
    _voice: c_int,
    mut pos: c_ulong,
    mut count: c_ulong,
) -> c_int {
    let rec: *mut snd_emu8k_pcm = (*(*subs).runtime).private_data as *mut snd_emu8k_pcm;

    /* convert to frames */
    pos = bytes_to_frames((*subs).runtime, pos);
    count = bytes_to_frames((*subs).runtime, count);
    LOOP_WRITE(rec, pos, ptr::null_mut(), count);
    0
}

/*
 * allocate a memory block
 */
unsafe extern "C" fn emu8k_pcm_hw_params(
    subs: *mut snd_pcm_substream,
    hw_params: *mut snd_pcm_hw_params,
) -> c_int {
    let rec: *mut snd_emu8k_pcm = (*(*subs).runtime).private_data as *mut snd_emu8k_pcm;

    if !(*rec).block.is_null() {
        /* reallocation - release the old block */
        snd_util_mem_free((*(*rec).emu).memhdr, (*rec).block);
        (*rec).block = ptr::null_mut();
    }

    (*rec).allocated_bytes = params_buffer_bytes(hw_params) + LOOP_BLANK_SIZE * 4;
    (*rec).block = snd_util_mem_alloc((*(*rec).emu).memhdr, (*rec).allocated_bytes);
    if (*rec).block.is_null() {
        return -ENOMEM;
    }
    (*rec).offset = EMU8000_DRAM_OFFSET + ((*(*rec).block).offset >> 1); /* in word */
    /* at least dma_bytes must be set for non-interleaved mode */
    (*subs).dma_buffer.bytes = params_buffer_bytes(hw_params);

    0
}

/*
 * free the memory block
 */
unsafe extern "C" fn emu8k_pcm_hw_free(subs: *mut snd_pcm_substream) -> c_int {
    let rec: *mut snd_emu8k_pcm = (*(*subs).runtime).private_data as *mut snd_emu8k_pcm;

    if !(*rec).block.is_null() {
        let mut ch: c_int;
        ch = 0;
        while ch < (*rec).voices {
            stop_voice(rec, ch); // to be sure
            ch += 1;
        }
        if (*rec).dram_opened != 0 {
            emu8k_close_dram((*rec).emu);
        }
        snd_util_mem_free((*(*rec).emu).memhdr, (*rec).block);
        (*rec).block = ptr::null_mut();
    }
    0
}

/*
 */
unsafe extern "C" fn emu8k_pcm_prepare(subs: *mut snd_pcm_substream) -> c_int {
    let rec: *mut snd_emu8k_pcm = (*(*subs).runtime).private_data as *mut snd_emu8k_pcm;

    (*rec).pitch = (0xe000 + calc_rate_offset((*(*subs).runtime).rate)) as c_uint;
    (*rec).last_ptr = 0;
    (*rec).period_pos = 0;

    (*rec).buf_size = (*(*subs).runtime).buffer_size;
    (*rec).period_size = (*(*subs).runtime).period_size;
    (*rec).voices = (*(*subs).runtime).channels;
    (*rec).loop_start[0] = (*rec).offset + LOOP_BLANK_SIZE;
    if (*rec).voices > 1 {
        (*rec).loop_start[1] = (*rec).loop_start[0] + (*rec).buf_size + LOOP_BLANK_SIZE;
    }
    if (*rec).voices > 1 {
        (*rec).panning[0] = 0xff;
        (*rec).panning[1] = 0x00;
    } else {
        (*rec).panning[0] = 0x80;
    }

    if (*rec).dram_opened == 0 {
        let mut err: c_int;
        let mut i: c_int;
        let mut ch: c_int;

        snd_emux_terminate_all((*(*rec).emu).emu);
        err = emu8k_open_dram_for_pcm((*rec).emu, (*rec).voices);
        if err != 0 {
            return err;
        }
        (*rec).dram_opened = 1;

        /* clear loop blanks */
        snd_emu8000_write_wait((*rec).emu, 0);
        EMU8000_SMALW_WRITE((*rec).emu, (*rec).offset);
        i = 0;
        while i < LOOP_BLANK_SIZE as c_int {
            EMU8000_SMLD_WRITE((*rec).emu, 0);
            i += 1;
        }
        ch = 0;
        while ch < (*rec).voices {
            EMU8000_SMALW_WRITE((*rec).emu, (*rec).loop_start[ch as usize] + (*rec).buf_size);
            i = 0;
            while i < LOOP_BLANK_SIZE as c_int {
                EMU8000_SMLD_WRITE((*rec).emu, 0);
                i += 1;
            }
            ch += 1;
        }
    }

    setup_voice(rec, 0);
    if (*rec).voices > 1 {
        setup_voice(rec, 1);
    }
    0
}

unsafe extern "C" fn emu8k_pcm_pointer(subs: *mut snd_pcm_substream) -> snd_pcm_uframes_t {
    let rec: *mut snd_emu8k_pcm = (*(*subs).runtime).private_data as *mut snd_emu8k_pcm;
    if (*rec).running != 0 {
        return emu8k_get_curpos(rec, 0) as snd_pcm_uframes_t;
    }
    0
}

static emu8k_pcm_ops: snd_pcm_ops = snd_pcm_ops {
    open: Some(emu8k_pcm_open),
    close: Some(emu8k_pcm_close),
    hw_params: Some(emu8k_pcm_hw_params),
    hw_free: Some(emu8k_pcm_hw_free),
    prepare: Some(emu8k_pcm_prepare),
    trigger: Some(emu8k_pcm_trigger),
    pointer: Some(emu8k_pcm_pointer),
    copy: Some(emu8k_pcm_copy),
    fill_silence: Some(emu8k_pcm_silence),
};

unsafe extern "C" fn snd_emu8000_pcm_free(pcm: *mut snd_pcm) {
    let emu: *mut snd_emu8000 = (*pcm).private_data as *mut snd_emu8000;
    (*emu).pcm = ptr::null_mut();
}

#[no_mangle]
pub unsafe extern "C" fn snd_emu8000_pcm_new(
    card: *mut snd_card,
    emu: *mut snd_emu8000,
    index: c_int,
) -> c_int {
    let mut pcm: *mut snd_pcm = ptr::null_mut();
    let mut err: c_int;

    err = snd_pcm_new(card, c"Emu8000 PCM".as_ptr(), index, 1, 0, &mut pcm);
    if err < 0 {
        return err;
    }
    (*pcm).private_data = emu as *mut c_void;
    (*pcm).private_free = Some(snd_emu8000_pcm_free);
    snd_pcm_set_ops(pcm, SNDRV_PCM_STREAM_PLAYBACK, &emu8k_pcm_ops);
    (*emu).pcm = pcm;

    snd_device_register(card, pcm as *mut c_void);

    0
}


// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
