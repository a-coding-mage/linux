// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *  Copyright (c) by Jaroslav Kysela <perex@perex.cz>
 *                   Uros Bizjak <uros@kss-loka.si>
 *
 *  Routines for control of 8-bit SoundBlaster cards and clones
 *  Please note: I don't have access to old SB8 soundcards.
 *
 * --
 *
 * Thu Apr 29 20:36:17 BST 1999 George David Morrison <gdm@gedamo.demon.co.uk>
 *   DSP can't respond to commands whilst in "high speed" mode. Caused
 *   glitching during playback. Fixed.
 *
 * Wed Jul 12 22:02:55 CEST 2000 Uros Bizjak <uros@kss-loka.si>
 *   Cleaned up and rewrote lowlevel routines.
 */

use core::ffi::{c_char, c_int, c_uint, c_void};

/* MODULE_AUTHOR("Jaroslav Kysela <perex@perex.cz>, Uros Bizjak <uros@kss-loka.si>"); */
/* MODULE_DESCRIPTION("Routines for control of 8-bit SoundBlaster cards and clones"); */
/* MODULE_LICENSE("GPL"); */

pub const SB8_CLOCK: c_uint = 1000000;

#[inline]
pub const fn SB8_DEN(v: c_uint) -> c_uint {
    (SB8_CLOCK + v / 2) / v
}

#[inline]
pub const fn SB8_RATE(v: c_uint) -> c_uint {
    SB8_CLOCK / SB8_DEN(v)
}

pub type size_t = usize;
pub type snd_pcm_uframes_t = usize;
pub type irqreturn_t = c_uint;

#[repr(C)]
pub struct snd_ratnum {
    pub num: c_uint,
    pub den_min: c_uint,
    pub den_max: c_uint,
    pub den_step: c_uint,
}

#[repr(C)]
pub struct snd_pcm_hw_constraint_ratnums {
    pub nrats: c_uint,
    pub rats: *const snd_ratnum,
}

#[repr(C)]
pub struct snd_interval {
    pub min: c_uint,
    pub max: c_uint,
}

#[repr(C)]
pub struct snd_pcm_hardware {
    pub info: c_uint,
    pub formats: c_uint,
    pub rates: c_uint,
    pub rate_min: c_uint,
    pub rate_max: c_uint,
    pub channels_min: c_uint,
    pub channels_max: c_uint,
    pub buffer_bytes_max: size_t,
    pub period_bytes_min: size_t,
    pub period_bytes_max: size_t,
    pub periods_min: c_uint,
    pub periods_max: c_uint,
    pub fifo_size: size_t,
}

#[repr(C)]
pub struct snd_pcm_hw_params {
    pub rate_num: c_uint,
    pub rate_den: c_uint,
}

#[repr(C)]
pub struct snd_pcm_hw_rule {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_pcm_runtime {
    pub channels: c_uint,
    pub format: c_uint,
    pub rate: c_uint,
    pub rate_den: c_uint,
    pub dma_area: *mut u8,
    pub dma_addr: usize,
    pub hw: snd_pcm_hardware,
}

#[repr(C)]
pub struct snd_pcm_substream {
    pub runtime: *mut snd_pcm_runtime,
    pub stream: c_int,
}

#[repr(C)]
pub struct snd_card {
    pub dev: *mut c_void,
}

#[repr(C)]
pub struct snd_pcm {
    pub name: [c_char; 80],
    pub info_flags: c_uint,
    pub private_data: *mut c_void,
}

#[repr(C)]
pub struct snd_pcm_ops {
    pub open: Option<unsafe extern "C" fn(*mut snd_pcm_substream) -> c_int>,
    pub close: Option<unsafe extern "C" fn(*mut snd_pcm_substream) -> c_int>,
    pub prepare: Option<unsafe extern "C" fn(*mut snd_pcm_substream) -> c_int>,
    pub trigger: Option<unsafe extern "C" fn(*mut snd_pcm_substream, c_int) -> c_int>,
    pub pointer: Option<unsafe extern "C" fn(*mut snd_pcm_substream) -> snd_pcm_uframes_t>,
}

#[repr(C)]
pub struct snd_sb {
    pub hardware: c_int,
    pub mode: c_uint,
    pub playback_format: u8,
    pub capture_format: u8,
    pub dma16: c_int,
    pub dma8: c_int,
    pub p_dma_size: size_t,
    pub p_period_size: size_t,
    pub c_dma_size: size_t,
    pub c_period_size: size_t,
    pub reg_lock: c_void,
    pub mixer_lock: c_void,
    pub open_lock: c_void,
    pub force_mode16: c_uint,
    pub playback_substream: *mut snd_pcm_substream,
    pub capture_substream: *mut snd_pcm_substream,
    pub open: c_uint,
    pub card: *mut snd_card,
    pub version: c_uint,
}

unsafe extern "C" {
    static SB_HW_JAZZ16: c_int;
    static SB_HW_PRO: c_int;
    static SB_HW_201: c_int;
    static SB_HW_20: c_int;
    static SB_HW_10: c_int;

    static SNDRV_PCM_FORMAT_S16_LE: c_uint;
    static SNDRV_PCM_FMTBIT_U8: c_uint;
    static SNDRV_PCM_FMTBIT_S16_LE: c_uint;
    static SNDRV_PCM_INFO_MMAP: c_uint;
    static SNDRV_PCM_INFO_INTERLEAVED: c_uint;
    static SNDRV_PCM_INFO_MMAP_VALID: c_uint;
    static SNDRV_PCM_INFO_HALF_DUPLEX: c_uint;
    static SNDRV_PCM_RATE_CONTINUOUS: c_uint;
    static SNDRV_PCM_RATE_8000: c_uint;
    static SNDRV_PCM_RATE_11025: c_uint;
    static SNDRV_PCM_RATE_22050: c_uint;
    static SNDRV_PCM_RATE_8000_48000: c_uint;
    static SNDRV_PCM_STREAM_PLAYBACK: c_int;
    static SNDRV_PCM_STREAM_CAPTURE: c_int;
    static SNDRV_PCM_HW_PARAM_CHANNELS: c_int;
    static SNDRV_PCM_HW_PARAM_RATE: c_int;
    static SNDRV_PCM_HW_PARAM_BUFFER_BYTES: c_int;
    static SNDRV_PCM_HW_PARAM_PERIOD_BYTES: c_int;
    static SNDRV_PCM_TRIGGER_START: c_int;
    static SNDRV_PCM_TRIGGER_STOP: c_int;

    static SB_MODE_CAPTURE_16: c_uint;
    static SB_MODE_PLAYBACK_16: c_uint;
    static SB_MODE_PLAYBACK_8: c_uint;
    static SB_MODE_CAPTURE_8: c_uint;
    static SB_MODE_PLAYBACK: c_uint;
    static SB_MODE_CAPTURE: c_uint;
    static SB_OPEN_PCM: c_uint;

    static SB_DSP_LO_OUTPUT_AUTO: u8;
    static SB_DSP_HI_OUTPUT_AUTO: u8;
    static SB_DSP_OUTPUT: u8;
    static SB_DSP_LO_INPUT_AUTO: u8;
    static SB_DSP_HI_INPUT_AUTO: u8;
    static SB_DSP_INPUT: u8;
    static SB_DSP_STEREO_16BIT: u8;
    static SB_DSP_MONO_16BIT: u8;
    static SB_DSP_STEREO_8BIT: u8;
    static SB_DSP_MONO_8BIT: u8;
    static SB_DSP_SPEAKER_ON: u8;
    static SB_DSP_SPEAKER_OFF: u8;
    static SB_DSP_STEREO_SW: u8;
    static SB_DSP_DMA8_EXIT: u8;
    static SB_DSP_SAMPLE_RATE: u8;
    static SB_DSP_PLAYBACK_FILT: u8;
    static SB_DSP_CAPTURE_FILT: u8;
    static SB_DSP_BLOCK_SIZE: u8;
    static SB_DSP_DMA8_OFF: u8;

    static DMA_MODE_WRITE: c_uint;
    static DMA_MODE_READ: c_uint;
    static DMA_AUTOINIT: c_uint;
    static SNDRV_DMA_TYPE_DEV: c_int;
    static IRQ_HANDLED: irqreturn_t;

    fn hw_param_interval(params: *mut snd_pcm_hw_params, var: c_int) -> *mut snd_interval;
    fn snd_interval_ratnum(
        i: *mut snd_interval,
        rats_count: c_uint,
        rats: *const snd_ratnum,
        num: *mut c_uint,
        den: *mut c_uint,
    ) -> c_int;
    fn snd_interval_refine(i: *mut snd_interval, v: *const snd_interval) -> c_int;
    fn snd_pcm_substream_chip(substream: *mut snd_pcm_substream) -> *mut snd_sb;
    fn snd_BUG_ON(cond: bool) -> bool;
    fn snd_sbdsp_command(chip: *mut snd_sb, val: u8) -> c_int;
    fn snd_sbmixer_read(chip: *mut snd_sb, reg: u8) -> c_uint;
    fn snd_sbmixer_write(chip: *mut snd_sb, reg: u8, val: c_uint);
    fn snd_dma_program(dma: c_int, addr: usize, size: size_t, mode: c_uint);
    fn snd_pcm_lib_buffer_bytes(substream: *mut snd_pcm_substream) -> size_t;
    fn snd_pcm_lib_period_bytes(substream: *mut snd_pcm_substream) -> size_t;
    fn snd_sbdsp_reset(chip: *mut snd_sb);
    fn snd_sb_ack_8bit(chip: *mut snd_sb);
    fn snd_pcm_period_elapsed(substream: *mut snd_pcm_substream);
    fn snd_dma_pointer(dma: c_int, size: size_t) -> size_t;
    fn bytes_to_frames(runtime: *mut snd_pcm_runtime, bytes: size_t) -> snd_pcm_uframes_t;
    fn snd_pcm_hw_rule_add(
        runtime: *mut snd_pcm_runtime,
        cond: c_uint,
        var: c_int,
        func: unsafe extern "C" fn(*mut snd_pcm_hw_params, *mut snd_pcm_hw_rule) -> c_int,
        private: *mut c_void,
        ...
    ) -> c_int;
    fn snd_pcm_hw_constraint_ratnums(
        runtime: *mut snd_pcm_runtime,
        cond: c_uint,
        var: c_int,
        r: *const snd_pcm_hw_constraint_ratnums,
    ) -> c_int;
    fn snd_pcm_hw_constraint_step(
        runtime: *mut snd_pcm_runtime,
        cond: c_uint,
        var: c_int,
        step: c_uint,
    ) -> c_int;
    fn snd_pcm_new(
        card: *mut snd_card,
        id: *const c_char,
        device: c_int,
        playback_count: c_int,
        capture_count: c_int,
        rpcm: *mut *mut snd_pcm,
    ) -> c_int;
    fn sprintf(s: *mut c_char, format: *const c_char, ...) -> c_int;
    fn snd_pcm_set_ops(pcm: *mut snd_pcm, direction: c_int, ops: *const snd_pcm_ops);
    fn snd_pcm_set_managed_buffer_all(
        pcm: *mut snd_pcm,
        type_: c_int,
        data: *mut c_void,
        size: size_t,
        max: size_t,
    );
}

static clock: snd_ratnum = snd_ratnum {
    num: SB8_CLOCK,
    den_min: 1,
    den_max: 256,
    den_step: 1,
};

static hw_constraints_clock: snd_pcm_hw_constraint_ratnums = snd_pcm_hw_constraint_ratnums {
    nrats: 1,
    rats: &clock,
};

static stereo_clocks: [snd_ratnum; 2] = [
    snd_ratnum {
        num: SB8_CLOCK,
        den_min: SB8_DEN(22050),
        den_max: SB8_DEN(22050),
        den_step: 1,
    },
    snd_ratnum {
        num: SB8_CLOCK,
        den_min: SB8_DEN(11025),
        den_max: SB8_DEN(11025),
        den_step: 1,
    },
];

unsafe extern "C" fn snd_sb8_hw_constraint_rate_channels(
    params: *mut snd_pcm_hw_params,
    _rule: *mut snd_pcm_hw_rule,
) -> c_int {
    let c = hw_param_interval(params, SNDRV_PCM_HW_PARAM_CHANNELS);
    if (*c).min > 1 {
        let mut num: c_uint = 0;
        let mut den: c_uint = 0;
        let err = snd_interval_ratnum(
            hw_param_interval(params, SNDRV_PCM_HW_PARAM_RATE),
            2,
            stereo_clocks.as_ptr(),
            &mut num,
            &mut den,
        );
        if err >= 0 && den != 0 {
            (*params).rate_num = num;
            (*params).rate_den = den;
        }
        return err;
    }
    0
}

unsafe extern "C" fn snd_sb8_hw_constraint_channels_rate(
    params: *mut snd_pcm_hw_params,
    _rule: *mut snd_pcm_hw_rule,
) -> c_int {
    let r = hw_param_interval(params, SNDRV_PCM_HW_PARAM_RATE);
    if (*r).min > SB8_RATE(22050) || (*r).max <= SB8_RATE(11025) {
        let t = snd_interval { min: 1, max: 1 };
        return snd_interval_refine(hw_param_interval(params, SNDRV_PCM_HW_PARAM_CHANNELS), &t);
    }
    0
}

unsafe extern "C" fn snd_sb8_playback_prepare(substream: *mut snd_pcm_substream) -> c_int {
    let chip = snd_pcm_substream_chip(substream);
    let runtime = (*substream).runtime;
    let mut mixreg: c_uint = 0;
    let rate: c_uint;
    let size: c_uint;
    let mut count: c_uint;
    let format: u8;
    let stereo: bool = (*runtime).channels > 1;
    let dma: c_int;

    rate = (*runtime).rate;
    if (*chip).hardware == SB_HW_JAZZ16 {
        if (*runtime).format == SNDRV_PCM_FORMAT_S16_LE {
            if ((*chip).mode & SB_MODE_CAPTURE_16) != 0 {
                return -16;
            } else {
                (*chip).mode |= SB_MODE_PLAYBACK_16;
            }
        }
        (*chip).playback_format = SB_DSP_LO_OUTPUT_AUTO;
    } else if (*chip).hardware == SB_HW_PRO {
        if (*runtime).channels > 1 {
            if snd_BUG_ON(rate != SB8_RATE(11025) && rate != SB8_RATE(22050)) {
                return -22;
            }
            (*chip).playback_format = SB_DSP_HI_OUTPUT_AUTO;
        } else if rate > 23000 {
            (*chip).playback_format = SB_DSP_HI_OUTPUT_AUTO;
        } else {
            (*chip).playback_format = SB_DSP_LO_OUTPUT_AUTO;
        }
    } else if (*chip).hardware == SB_HW_201 {
        if rate > 23000 {
            (*chip).playback_format = SB_DSP_HI_OUTPUT_AUTO;
        } else {
            (*chip).playback_format = SB_DSP_LO_OUTPUT_AUTO;
        }
    } else if (*chip).hardware == SB_HW_20 {
        (*chip).playback_format = SB_DSP_LO_OUTPUT_AUTO;
    } else if (*chip).hardware == SB_HW_10 {
        (*chip).playback_format = SB_DSP_OUTPUT;
    } else {
        return -22;
    }

    if ((*chip).mode & SB_MODE_PLAYBACK_16) != 0 {
        format = if stereo { SB_DSP_STEREO_16BIT } else { SB_DSP_MONO_16BIT };
        dma = (*chip).dma16;
    } else {
        format = if stereo { SB_DSP_STEREO_8BIT } else { SB_DSP_MONO_8BIT };
        (*chip).mode |= SB_MODE_PLAYBACK_8;
        dma = (*chip).dma8;
    }
    size = snd_pcm_lib_buffer_bytes(substream) as c_uint;
    (*chip).p_dma_size = size as size_t;
    count = snd_pcm_lib_period_bytes(substream) as c_uint;
    (*chip).p_period_size = count as size_t;
    /* C scoped_guard(spinlock_irqsave, &chip->reg_lock) */
    snd_sbdsp_command(chip, SB_DSP_SPEAKER_ON);
    if (*chip).hardware == SB_HW_JAZZ16 {
        snd_sbdsp_command(chip, format);
    } else if stereo {
        /* set playback stereo mode */
        /* C scoped_guard(spinlock, &chip->mixer_lock) */
        mixreg = snd_sbmixer_read(chip, SB_DSP_STEREO_SW);
        snd_sbmixer_write(chip, SB_DSP_STEREO_SW, mixreg | 0x02);

        /* Soundblaster hardware programming reference guide, 3-23 */
        snd_sbdsp_command(chip, SB_DSP_DMA8_EXIT);
        *(*runtime).dma_area.add(0) = 0x80;
        snd_dma_program(dma, (*runtime).dma_addr, 1, DMA_MODE_WRITE);
        /* force interrupt */
        snd_sbdsp_command(chip, SB_DSP_OUTPUT);
        snd_sbdsp_command(chip, 0);
        snd_sbdsp_command(chip, 0);
    }
    snd_sbdsp_command(chip, SB_DSP_SAMPLE_RATE);
    if stereo {
        snd_sbdsp_command(chip, (256 - (*runtime).rate_den / 2) as u8);
        /* C scoped_guard(spinlock, &chip->mixer_lock) */
        /* save output filter status and turn it off */
        mixreg = snd_sbmixer_read(chip, SB_DSP_PLAYBACK_FILT);
        snd_sbmixer_write(chip, SB_DSP_PLAYBACK_FILT, mixreg | 0x20);
        /* just use force_mode16 for temporary storate... */
        (*chip).force_mode16 = mixreg;
    } else {
        snd_sbdsp_command(chip, (256 - (*runtime).rate_den) as u8);
    }
    if (*chip).playback_format != SB_DSP_OUTPUT {
        if ((*chip).mode & SB_MODE_PLAYBACK_16) != 0 {
            count /= 2;
        }
        count = count.wrapping_sub(1);
        snd_sbdsp_command(chip, SB_DSP_BLOCK_SIZE);
        snd_sbdsp_command(chip, (count & 0xff) as u8);
        snd_sbdsp_command(chip, (count >> 8) as u8);
    }
    snd_dma_program(dma, (*runtime).dma_addr, size as size_t, DMA_MODE_WRITE | DMA_AUTOINIT);
    0
}

unsafe extern "C" fn snd_sb8_playback_trigger(
    substream: *mut snd_pcm_substream,
    cmd: c_int,
) -> c_int {
    let chip = snd_pcm_substream_chip(substream);
    let mut count: c_uint;

    /* C guard(spinlock_irqsave)(&chip->reg_lock); */
    if cmd == SNDRV_PCM_TRIGGER_START {
        snd_sbdsp_command(chip, (*chip).playback_format);
        if (*chip).playback_format == SB_DSP_OUTPUT {
            count = ((*chip).p_period_size - 1) as c_uint;
            snd_sbdsp_command(chip, (count & 0xff) as u8);
            snd_sbdsp_command(chip, (count >> 8) as u8);
        }
    } else if cmd == SNDRV_PCM_TRIGGER_STOP {
        if (*chip).playback_format == SB_DSP_HI_OUTPUT_AUTO {
            let runtime = (*substream).runtime;
            snd_sbdsp_reset(chip);
            if (*runtime).channels > 1 {
                /* C guard(spinlock)(&chip->mixer_lock); */
                /* restore output filter and set hardware to mono mode */
                snd_sbmixer_write(chip, SB_DSP_STEREO_SW, (*chip).force_mode16 & !0x02);
            }
        } else {
            snd_sbdsp_command(chip, SB_DSP_DMA8_OFF);
        }
        snd_sbdsp_command(chip, SB_DSP_SPEAKER_OFF);
    }
    0
}

unsafe extern "C" fn snd_sb8_capture_prepare(substream: *mut snd_pcm_substream) -> c_int {
    let chip = snd_pcm_substream_chip(substream);
    let runtime = (*substream).runtime;
    let mut mixreg: c_uint = 0;
    let rate: c_uint;
    let size: c_uint;
    let mut count: c_uint;
    let format: u8;
    let stereo: bool = (*runtime).channels > 1;
    let dma: c_int;

    rate = (*runtime).rate;
    if (*chip).hardware == SB_HW_JAZZ16 {
        if (*runtime).format == SNDRV_PCM_FORMAT_S16_LE {
            if ((*chip).mode & SB_MODE_PLAYBACK_16) != 0 {
                return -16;
            } else {
                (*chip).mode |= SB_MODE_CAPTURE_16;
            }
        }
        (*chip).capture_format = SB_DSP_LO_INPUT_AUTO;
    } else if (*chip).hardware == SB_HW_PRO {
        if (*runtime).channels > 1 {
            if snd_BUG_ON(rate != SB8_RATE(11025) && rate != SB8_RATE(22050)) {
                return -22;
            }
            (*chip).capture_format = SB_DSP_HI_INPUT_AUTO;
        } else {
            (*chip).capture_format = if rate > 23000 {
                SB_DSP_HI_INPUT_AUTO
            } else {
                SB_DSP_LO_INPUT_AUTO
            };
        }
    } else if (*chip).hardware == SB_HW_201 {
        if rate > 13000 {
            (*chip).capture_format = SB_DSP_HI_INPUT_AUTO;
        } else {
            (*chip).capture_format = SB_DSP_LO_INPUT_AUTO;
        }
    } else if (*chip).hardware == SB_HW_20 {
        (*chip).capture_format = SB_DSP_LO_INPUT_AUTO;
    } else if (*chip).hardware == SB_HW_10 {
        (*chip).capture_format = SB_DSP_INPUT;
    } else {
        return -22;
    }
    if ((*chip).mode & SB_MODE_CAPTURE_16) != 0 {
        format = if stereo { SB_DSP_STEREO_16BIT } else { SB_DSP_MONO_16BIT };
        dma = (*chip).dma16;
    } else {
        format = if stereo { SB_DSP_STEREO_8BIT } else { SB_DSP_MONO_8BIT };
        (*chip).mode |= SB_MODE_CAPTURE_8;
        dma = (*chip).dma8;
    }
    size = snd_pcm_lib_buffer_bytes(substream) as c_uint;
    (*chip).c_dma_size = size as size_t;
    count = snd_pcm_lib_period_bytes(substream) as c_uint;
    (*chip).c_period_size = count as size_t;
    /* C scoped_guard(spinlock_irqsave, &chip->reg_lock) */
    snd_sbdsp_command(chip, SB_DSP_SPEAKER_OFF);
    if (*chip).hardware == SB_HW_JAZZ16 {
        snd_sbdsp_command(chip, format);
    } else if stereo {
        snd_sbdsp_command(chip, SB_DSP_STEREO_8BIT);
    }
    snd_sbdsp_command(chip, SB_DSP_SAMPLE_RATE);
    if stereo {
        snd_sbdsp_command(chip, (256 - (*runtime).rate_den / 2) as u8);
        /* C scoped_guard(spinlock, &chip->mixer_lock) */
        /* save input filter status and turn it off */
        mixreg = snd_sbmixer_read(chip, SB_DSP_CAPTURE_FILT);
        snd_sbmixer_write(chip, SB_DSP_CAPTURE_FILT, mixreg | 0x20);
        /* just use force_mode16 for temporary storate... */
        (*chip).force_mode16 = mixreg;
    } else {
        snd_sbdsp_command(chip, (256 - (*runtime).rate_den) as u8);
    }
    if (*chip).capture_format != SB_DSP_INPUT {
        if ((*chip).mode & SB_MODE_PLAYBACK_16) != 0 {
            count /= 2;
        }
        count = count.wrapping_sub(1);
        snd_sbdsp_command(chip, SB_DSP_BLOCK_SIZE);
        snd_sbdsp_command(chip, (count & 0xff) as u8);
        snd_sbdsp_command(chip, (count >> 8) as u8);
    }
    snd_dma_program(dma, (*runtime).dma_addr, size as size_t, DMA_MODE_READ | DMA_AUTOINIT);
    0
}

unsafe extern "C" fn snd_sb8_capture_trigger(
    substream: *mut snd_pcm_substream,
    cmd: c_int,
) -> c_int {
    let chip = snd_pcm_substream_chip(substream);
    let mut count: c_uint;

    /* C guard(spinlock_irqsave)(&chip->reg_lock); */
    if cmd == SNDRV_PCM_TRIGGER_START {
        snd_sbdsp_command(chip, (*chip).capture_format);
        if (*chip).capture_format == SB_DSP_INPUT {
            count = ((*chip).c_period_size - 1) as c_uint;
            snd_sbdsp_command(chip, (count & 0xff) as u8);
            snd_sbdsp_command(chip, (count >> 8) as u8);
        }
    } else if cmd == SNDRV_PCM_TRIGGER_STOP {
        if (*chip).capture_format == SB_DSP_HI_INPUT_AUTO {
            let runtime = (*substream).runtime;
            snd_sbdsp_reset(chip);
            if (*runtime).channels > 1 {
                /* restore input filter status */
                /* C scoped_guard(spinlock, &chip->mixer_lock) */
                snd_sbmixer_write(chip, SB_DSP_CAPTURE_FILT, (*chip).force_mode16);
                /* set hardware to mono mode */
                snd_sbdsp_command(chip, SB_DSP_MONO_8BIT);
            }
        } else {
            snd_sbdsp_command(chip, SB_DSP_DMA8_OFF);
        }
        snd_sbdsp_command(chip, SB_DSP_SPEAKER_OFF);
    }
    0
}

#[no_mangle]
pub unsafe extern "C" fn snd_sb8dsp_interrupt(chip: *mut snd_sb) -> irqreturn_t {
    let substream: *mut snd_pcm_substream;

    snd_sb_ack_8bit(chip);
    if (*chip).mode == SB_MODE_PLAYBACK_16 {
        /* ok.. playback is active */
        if (*chip).hardware != SB_HW_JAZZ16 {
            return IRQ_HANDLED;
        }
        substream = (*chip).playback_substream;
        if (*chip).playback_format == SB_DSP_OUTPUT {
            snd_sb8_playback_trigger(substream, SNDRV_PCM_TRIGGER_START);
        }
        snd_pcm_period_elapsed(substream);
    } else if (*chip).mode == SB_MODE_PLAYBACK_8 {
        substream = (*chip).playback_substream;
        if (*chip).playback_format == SB_DSP_OUTPUT {
            snd_sb8_playback_trigger(substream, SNDRV_PCM_TRIGGER_START);
        }
        snd_pcm_period_elapsed(substream);
    } else if (*chip).mode == SB_MODE_CAPTURE_16 {
        if (*chip).hardware != SB_HW_JAZZ16 {
            return IRQ_HANDLED;
        }
        substream = (*chip).capture_substream;
        if (*chip).capture_format == SB_DSP_INPUT {
            snd_sb8_capture_trigger(substream, SNDRV_PCM_TRIGGER_START);
        }
        snd_pcm_period_elapsed(substream);
    } else if (*chip).mode == SB_MODE_CAPTURE_8 {
        substream = (*chip).capture_substream;
        if (*chip).capture_format == SB_DSP_INPUT {
            snd_sb8_capture_trigger(substream, SNDRV_PCM_TRIGGER_START);
        }
        snd_pcm_period_elapsed(substream);
    }
    IRQ_HANDLED
}

unsafe extern "C" fn snd_sb8_playback_pointer(
    substream: *mut snd_pcm_substream,
) -> snd_pcm_uframes_t {
    let chip = snd_pcm_substream_chip(substream);
    let ptr: size_t;
    let dma: c_int;

    if ((*chip).mode & SB_MODE_PLAYBACK_8) != 0 {
        dma = (*chip).dma8;
    } else if ((*chip).mode & SB_MODE_PLAYBACK_16) != 0 {
        dma = (*chip).dma16;
    } else {
        return 0;
    }
    ptr = snd_dma_pointer(dma, (*chip).p_dma_size);
    bytes_to_frames((*substream).runtime, ptr)
}

unsafe extern "C" fn snd_sb8_capture_pointer(
    substream: *mut snd_pcm_substream,
) -> snd_pcm_uframes_t {
    let chip = snd_pcm_substream_chip(substream);
    let ptr: size_t;
    let dma: c_int;

    if ((*chip).mode & SB_MODE_CAPTURE_8) != 0 {
        dma = (*chip).dma8;
    } else if ((*chip).mode & SB_MODE_CAPTURE_16) != 0 {
        dma = (*chip).dma16;
    } else {
        return 0;
    }
    ptr = snd_dma_pointer(dma, (*chip).c_dma_size);
    bytes_to_frames((*substream).runtime, ptr)
}

/*

 */

static snd_sb8_playback: snd_pcm_hardware = snd_pcm_hardware {
    info: 0,              /* filled in snd_sb8_playback_hw() */
    formats: 0,           /* filled in snd_sb8_playback_hw() */
    rates: 0,             /* filled in snd_sb8_playback_hw() */
    rate_min: 4000,
    rate_max: 23000,
    channels_min: 1,
    channels_max: 1,
    buffer_bytes_max: 65536,
    period_bytes_min: 64,
    period_bytes_max: 65536,
    periods_min: 1,
    periods_max: 1024,
    fifo_size: 0,
};

unsafe fn snd_sb8_playback_hw() -> snd_pcm_hardware {
    snd_pcm_hardware {
        info: SNDRV_PCM_INFO_MMAP | SNDRV_PCM_INFO_INTERLEAVED | SNDRV_PCM_INFO_MMAP_VALID,
        formats: SNDRV_PCM_FMTBIT_U8,
        rates: SNDRV_PCM_RATE_CONTINUOUS
            | SNDRV_PCM_RATE_8000
            | SNDRV_PCM_RATE_11025
            | SNDRV_PCM_RATE_22050,
        ..snd_sb8_playback
    }
}

static snd_sb8_capture: snd_pcm_hardware = snd_pcm_hardware {
    info: 0,              /* filled in snd_sb8_capture_hw() */
    formats: 0,           /* filled in snd_sb8_capture_hw() */
    rates: 0,             /* filled in snd_sb8_capture_hw() */
    rate_min: 4000,
    rate_max: 13000,
    channels_min: 1,
    channels_max: 1,
    buffer_bytes_max: 65536,
    period_bytes_min: 64,
    period_bytes_max: 65536,
    periods_min: 1,
    periods_max: 1024,
    fifo_size: 0,
};

unsafe fn snd_sb8_capture_hw() -> snd_pcm_hardware {
    snd_pcm_hardware {
        info: SNDRV_PCM_INFO_MMAP | SNDRV_PCM_INFO_INTERLEAVED | SNDRV_PCM_INFO_MMAP_VALID,
        formats: SNDRV_PCM_FMTBIT_U8,
        rates: SNDRV_PCM_RATE_CONTINUOUS | SNDRV_PCM_RATE_8000 | SNDRV_PCM_RATE_11025,
        ..snd_sb8_capture
    }
}

/*
 *
 */

unsafe extern "C" fn snd_sb8_open(substream: *mut snd_pcm_substream) -> c_int {
    let chip = snd_pcm_substream_chip(substream);
    let runtime = (*substream).runtime;

    /* C scoped_guard(spinlock_irqsave, &chip->open_lock) */
    if (*chip).open != 0 {
        return -11;
    }
    (*chip).open |= SB_OPEN_PCM;
    if (*substream).stream == SNDRV_PCM_STREAM_PLAYBACK {
        (*chip).playback_substream = substream;
        (*runtime).hw = snd_sb8_playback_hw();
    } else {
        (*chip).capture_substream = substream;
        (*runtime).hw = snd_sb8_capture_hw();
    }
    if (*chip).hardware == SB_HW_JAZZ16 {
        if (*chip).dma16 == 5 || (*chip).dma16 == 7 {
            (*runtime).hw.formats |= SNDRV_PCM_FMTBIT_S16_LE;
        }
        (*runtime).hw.rates |= SNDRV_PCM_RATE_8000_48000;
        (*runtime).hw.rate_min = 4000;
        (*runtime).hw.rate_max = 50000;
        (*runtime).hw.channels_max = 2;
    } else if (*chip).hardware == SB_HW_PRO {
        (*runtime).hw.rate_max = 44100;
        (*runtime).hw.channels_max = 2;
        snd_pcm_hw_rule_add(
            runtime,
            0,
            SNDRV_PCM_HW_PARAM_RATE,
            snd_sb8_hw_constraint_rate_channels,
            core::ptr::null_mut(),
            SNDRV_PCM_HW_PARAM_CHANNELS,
            SNDRV_PCM_HW_PARAM_RATE,
            -1,
        );
        snd_pcm_hw_rule_add(
            runtime,
            0,
            SNDRV_PCM_HW_PARAM_CHANNELS,
            snd_sb8_hw_constraint_channels_rate,
            core::ptr::null_mut(),
            SNDRV_PCM_HW_PARAM_RATE,
            -1,
        );
    } else if (*chip).hardware == SB_HW_201 {
        if (*substream).stream == SNDRV_PCM_STREAM_PLAYBACK {
            (*runtime).hw.rate_max = 44100;
        } else {
            (*runtime).hw.rate_max = 15000;
        }
    }
    snd_pcm_hw_constraint_ratnums(runtime, 0, SNDRV_PCM_HW_PARAM_RATE, &hw_constraints_clock);
    if (*chip).dma8 > 3 || (*chip).dma16 >= 0 {
        snd_pcm_hw_constraint_step(runtime, 0, SNDRV_PCM_HW_PARAM_BUFFER_BYTES, 2);
        snd_pcm_hw_constraint_step(runtime, 0, SNDRV_PCM_HW_PARAM_PERIOD_BYTES, 2);
        (*runtime).hw.buffer_bytes_max = 128 * 1024 * 1024;
        (*runtime).hw.period_bytes_max = 128 * 1024 * 1024;
    }
    0
}

unsafe extern "C" fn snd_sb8_close(substream: *mut snd_pcm_substream) -> c_int {
    let chip = snd_pcm_substream_chip(substream);

    (*chip).playback_substream = core::ptr::null_mut();
    (*chip).capture_substream = core::ptr::null_mut();
    /* C guard(spinlock_irqsave)(&chip->open_lock); */
    (*chip).open &= !SB_OPEN_PCM;
    if (*substream).stream == SNDRV_PCM_STREAM_PLAYBACK {
        (*chip).mode &= !SB_MODE_PLAYBACK;
    } else {
        (*chip).mode &= !SB_MODE_CAPTURE;
    }
    0
}

/*
 *  Initialization part
 */

static snd_sb8_playback_ops: snd_pcm_ops = snd_pcm_ops {
    open: Some(snd_sb8_open),
    close: Some(snd_sb8_close),
    prepare: Some(snd_sb8_playback_prepare),
    trigger: Some(snd_sb8_playback_trigger),
    pointer: Some(snd_sb8_playback_pointer),
};

static snd_sb8_capture_ops: snd_pcm_ops = snd_pcm_ops {
    open: Some(snd_sb8_open),
    close: Some(snd_sb8_close),
    prepare: Some(snd_sb8_capture_prepare),
    trigger: Some(snd_sb8_capture_trigger),
    pointer: Some(snd_sb8_capture_pointer),
};

#[no_mangle]
pub unsafe extern "C" fn snd_sb8dsp_pcm(chip: *mut snd_sb, device: c_int) -> c_int {
    let card = (*chip).card;
    let mut pcm: *mut snd_pcm = core::ptr::null_mut();
    let mut err: c_int;
    let mut max_prealloc: size_t = 64 * 1024;

    err = snd_pcm_new(card, b"SB8 DSP\0".as_ptr() as *const c_char, device, 1, 1, &mut pcm);
    if err < 0 {
        return err;
    }
    sprintf(
        (*pcm).name.as_mut_ptr(),
        b"DSP v%i.%i\0".as_ptr() as *const c_char,
        (*chip).version >> 8,
        (*chip).version & 0xff,
    );
    (*pcm).info_flags = SNDRV_PCM_INFO_HALF_DUPLEX;
    (*pcm).private_data = chip as *mut c_void;

    snd_pcm_set_ops(pcm, SNDRV_PCM_STREAM_PLAYBACK, &snd_sb8_playback_ops);
    snd_pcm_set_ops(pcm, SNDRV_PCM_STREAM_CAPTURE, &snd_sb8_capture_ops);

    if (*chip).dma8 > 3 || (*chip).dma16 >= 0 {
        max_prealloc = 128 * 1024;
    }
    snd_pcm_set_managed_buffer_all(pcm, SNDRV_DMA_TYPE_DEV, (*card).dev, 64 * 1024, max_prealloc);

    0
}

/* EXPORT_SYMBOL(snd_sb8dsp_pcm); */
/* EXPORT_SYMBOL(snd_sb8dsp_interrupt); */
/* sb8_midi.c */
/* EXPORT_SYMBOL(snd_sb8dsp_midi_interrupt); */
/* EXPORT_SYMBOL(snd_sb8dsp_midi); */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
