// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *  Copyright (c) by Jaroslav Kysela <perex@perex.cz>
 *  Routines for control of 16-bit SoundBlaster cards and clones
 *  Note: This is very ugly hardware which uses one 8-bit DMA channel and
 *        second 16-bit DMA channel. Unfortunately 8-bit DMA channel can't
 *        transfer 16-bit samples and 16-bit DMA channels can't transfer
 *        8-bit samples. This make full duplex more complicated than
 *        can be... People, don't buy these soundcards for full 16-bit
 *        duplex!!!
 *  Note: 16-bit wide is assigned to first direction which made request.
 *        With full duplex - playback is preferred with abstract layer.
 *
 *  Note: Some chip revisions have hardware bug. Changing capture
 *        channel from full-duplex 8bit DMA to 16bit DMA will block
 *        16bit DMA transfers from DSP chip (capture) until 8bit transfer
 *        to DSP chip (playback) starts. This bug can be avoided with
 *        "16bit DMA Allocation" setting set to Playback or Capture.
 */

// Dependencies from the original C includes:
// linux/io.h, asm/dma.h, linux/init.h, linux/time.h, linux/module.h,
// sound/core.h, sound/sb.h, sound/sb16_csp.h, sound/mpu401.h,
// sound/control.h, sound/info.h.

fn runtime_format_bits(runtime: *mut snd_pcm_runtime) -> u32 {
    unsafe { pcm_format_to_bits((*runtime).format) as u32 }
}

// Original C condition: #ifdef CONFIG_SND_SB16_CSP
#[cfg(CONFIG_SND_SB16_CSP)]
unsafe fn snd_sb16_csp_playback_prepare(chip: *mut snd_sb, runtime: *mut snd_pcm_runtime) {
    if (*chip).hardware == SB_HW_16CSP {
        let csp: *mut snd_sb_csp = (*chip).csp;

        if (*csp).running & SNDRV_SB_CSP_ST_LOADED != 0 {
            /* manually loaded codec */
            let mut start_csp = false;
            if ((*csp).mode & SNDRV_SB_CSP_MODE_DSP_WRITE != 0)
                && runtime_format_bits(runtime) == (*csp).acc_format
            {
                /* Supported runtime PCM format for playback */
                if ((*csp).ops.csp_use)(csp) == 0 {
                    /* If CSP was successfully acquired */
                    start_csp = true;
                }
            } else if ((*csp).mode & SNDRV_SB_CSP_MODE_QSOUND != 0) && (*csp).q_enabled != 0 {
                /* QSound decoder is loaded and enabled */
                if runtime_format_bits(runtime)
                    & (SNDRV_PCM_FMTBIT_S8
                        | SNDRV_PCM_FMTBIT_U8
                        | SNDRV_PCM_FMTBIT_S16_LE
                        | SNDRV_PCM_FMTBIT_U16_LE)
                    != 0
                {
                    /* Only for simple PCM formats */
                    if ((*csp).ops.csp_use)(csp) == 0 {
                        /* If CSP was successfully acquired */
                        start_csp = true;
                    }
                }
            }
            if start_csp {
                if ((*csp).ops.csp_start)(
                    csp,
                    if (*chip).mode & SB_MODE_PLAYBACK_16 != 0 {
                        SNDRV_SB_CSP_SAMPLE_16BIT
                    } else {
                        SNDRV_SB_CSP_SAMPLE_8BIT
                    },
                    if (*runtime).channels > 1 {
                        SNDRV_SB_CSP_STEREO
                    } else {
                        SNDRV_SB_CSP_MONO
                    },
                ) != 0
                {
                    /* Failed, release CSP */
                    ((*csp).ops.csp_unuse)(csp);
                } else {
                    /* Success, CSP acquired and running */
                    (*chip).open = SNDRV_SB_CSP_MODE_DSP_WRITE;
                }
            }
        } else if ((*csp).ops.csp_use)(csp) == 0 {
            /* Acquire CSP and try to autoload hardware codec */
            if ((*csp).ops.csp_autoload)(csp, (*runtime).format, SNDRV_SB_CSP_MODE_DSP_WRITE) != 0 {
                /* Unsupported format, release CSP */
                ((*csp).ops.csp_unuse)(csp);
            } else {
                /* Try to start CSP */
                if ((*csp).ops.csp_start)(
                    csp,
                    if (*chip).mode & SB_MODE_PLAYBACK_16 != 0 {
                        SNDRV_SB_CSP_SAMPLE_16BIT
                    } else {
                        SNDRV_SB_CSP_SAMPLE_8BIT
                    },
                    if (*runtime).channels > 1 {
                        SNDRV_SB_CSP_STEREO
                    } else {
                        SNDRV_SB_CSP_MONO
                    },
                ) != 0
                {
                    /* Failed, release CSP */
                    ((*csp).ops.csp_unuse)(csp);
                } else {
                    /* Success, CSP acquired and running */
                    (*chip).open = SNDRV_SB_CSP_MODE_DSP_WRITE;
                }
            }
        }
    }
}

#[cfg(CONFIG_SND_SB16_CSP)]
unsafe fn snd_sb16_csp_capture_prepare(chip: *mut snd_sb, runtime: *mut snd_pcm_runtime) {
    if (*chip).hardware == SB_HW_16CSP {
        let csp: *mut snd_sb_csp = (*chip).csp;

        if (*csp).running & SNDRV_SB_CSP_ST_LOADED != 0 {
            /* manually loaded codec */
            let mut start_csp = false;
            if ((*csp).mode & SNDRV_SB_CSP_MODE_DSP_READ != 0)
                && runtime_format_bits(runtime) == (*csp).acc_format
            {
                /* Supported runtime PCM format for capture */
                if ((*csp).ops.csp_use)(csp) == 0 {
                    /* If CSP was successfully acquired */
                    start_csp = true;
                }
            }
            if start_csp {
                if ((*csp).ops.csp_start)(
                    csp,
                    if (*chip).mode & SB_MODE_CAPTURE_16 != 0 {
                        SNDRV_SB_CSP_SAMPLE_16BIT
                    } else {
                        SNDRV_SB_CSP_SAMPLE_8BIT
                    },
                    if (*runtime).channels > 1 {
                        SNDRV_SB_CSP_STEREO
                    } else {
                        SNDRV_SB_CSP_MONO
                    },
                ) != 0
                {
                    /* Failed, release CSP */
                    ((*csp).ops.csp_unuse)(csp);
                } else {
                    /* Success, CSP acquired and running */
                    (*chip).open = SNDRV_SB_CSP_MODE_DSP_READ;
                }
            }
        } else if ((*csp).ops.csp_use)(csp) == 0 {
            /* Acquire CSP and try to autoload hardware codec */
            if ((*csp).ops.csp_autoload)(csp, (*runtime).format, SNDRV_SB_CSP_MODE_DSP_READ) != 0 {
                /* Unsupported format, release CSP */
                ((*csp).ops.csp_unuse)(csp);
            } else {
                /* Try to start CSP */
                if ((*csp).ops.csp_start)(
                    csp,
                    if (*chip).mode & SB_MODE_CAPTURE_16 != 0 {
                        SNDRV_SB_CSP_SAMPLE_16BIT
                    } else {
                        SNDRV_SB_CSP_SAMPLE_8BIT
                    },
                    if (*runtime).channels > 1 {
                        SNDRV_SB_CSP_STEREO
                    } else {
                        SNDRV_SB_CSP_MONO
                    },
                ) != 0
                {
                    /* Failed, release CSP */
                    ((*csp).ops.csp_unuse)(csp);
                } else {
                    /* Success, CSP acquired and running */
                    (*chip).open = SNDRV_SB_CSP_MODE_DSP_READ;
                }
            }
        }
    }
}

#[cfg(CONFIG_SND_SB16_CSP)]
unsafe fn snd_sb16_csp_update(chip: *mut snd_sb) {
    if (*chip).hardware == SB_HW_16CSP {
        let csp: *mut snd_sb_csp = (*chip).csp;

        if (*csp).qpos_changed != 0 {
            guard_spinlock(&mut (*chip).reg_lock);
            ((*csp).ops.csp_qsound_transfer)(csp);
        }
    }
}

#[cfg(CONFIG_SND_SB16_CSP)]
unsafe fn snd_sb16_csp_playback_open(chip: *mut snd_sb, runtime: *mut snd_pcm_runtime) {
    /* CSP decoders (QSound excluded) support only 16bit transfers */
    if (*chip).hardware == SB_HW_16CSP {
        let csp: *mut snd_sb_csp = (*chip).csp;

        if (*csp).running & SNDRV_SB_CSP_ST_LOADED != 0 {
            /* manually loaded codec */
            if (*csp).mode & SNDRV_SB_CSP_MODE_DSP_WRITE != 0 {
                (*runtime).hw.formats |= (*csp).acc_format;
            }
        } else {
            /* autoloaded codecs */
            (*runtime).hw.formats |=
                SNDRV_PCM_FMTBIT_MU_LAW | SNDRV_PCM_FMTBIT_A_LAW | SNDRV_PCM_FMTBIT_IMA_ADPCM;
        }
    }
}

#[cfg(CONFIG_SND_SB16_CSP)]
unsafe fn snd_sb16_csp_playback_close(chip: *mut snd_sb) {
    if ((*chip).hardware == SB_HW_16CSP) && ((*chip).open == SNDRV_SB_CSP_MODE_DSP_WRITE) {
        let csp: *mut snd_sb_csp = (*chip).csp;

        if ((*csp).ops.csp_stop)(csp) == 0 {
            ((*csp).ops.csp_unuse)(csp);
            (*chip).open = 0;
        }
    }
}

#[cfg(CONFIG_SND_SB16_CSP)]
unsafe fn snd_sb16_csp_capture_open(chip: *mut snd_sb, runtime: *mut snd_pcm_runtime) {
    /* CSP coders support only 16bit transfers */
    if (*chip).hardware == SB_HW_16CSP {
        let csp: *mut snd_sb_csp = (*chip).csp;

        if (*csp).running & SNDRV_SB_CSP_ST_LOADED != 0 {
            /* manually loaded codec */
            if (*csp).mode & SNDRV_SB_CSP_MODE_DSP_READ != 0 {
                (*runtime).hw.formats |= (*csp).acc_format;
            }
        } else {
            /* autoloaded codecs */
            (*runtime).hw.formats |=
                SNDRV_PCM_FMTBIT_MU_LAW | SNDRV_PCM_FMTBIT_A_LAW | SNDRV_PCM_FMTBIT_IMA_ADPCM;
        }
    }
}

#[cfg(CONFIG_SND_SB16_CSP)]
unsafe fn snd_sb16_csp_capture_close(chip: *mut snd_sb) {
    if ((*chip).hardware == SB_HW_16CSP) && ((*chip).open == SNDRV_SB_CSP_MODE_DSP_READ) {
        let csp: *mut snd_sb_csp = (*chip).csp;

        if ((*csp).ops.csp_stop)(csp) == 0 {
            ((*csp).ops.csp_unuse)(csp);
            (*chip).open = 0;
        }
    }
}

#[cfg(not(CONFIG_SND_SB16_CSP))]
unsafe fn snd_sb16_csp_playback_prepare(_chip: *mut snd_sb, _runtime: *mut snd_pcm_runtime) {}
#[cfg(not(CONFIG_SND_SB16_CSP))]
unsafe fn snd_sb16_csp_capture_prepare(_chip: *mut snd_sb, _runtime: *mut snd_pcm_runtime) {}
#[cfg(not(CONFIG_SND_SB16_CSP))]
unsafe fn snd_sb16_csp_update(_chip: *mut snd_sb) {}
#[cfg(not(CONFIG_SND_SB16_CSP))]
unsafe fn snd_sb16_csp_playback_open(_chip: *mut snd_sb, _runtime: *mut snd_pcm_runtime) {}
#[cfg(not(CONFIG_SND_SB16_CSP))]
unsafe fn snd_sb16_csp_playback_close(_chip: *mut snd_sb) {}
#[cfg(not(CONFIG_SND_SB16_CSP))]
unsafe fn snd_sb16_csp_capture_open(_chip: *mut snd_sb, _runtime: *mut snd_pcm_runtime) {}
#[cfg(not(CONFIG_SND_SB16_CSP))]
unsafe fn snd_sb16_csp_capture_close(_chip: *mut snd_sb) {}

unsafe fn snd_sb16_setup_rate(chip: *mut snd_sb, rate: u16, channel: c_int) {
    guard_spinlock_irqsave(&mut (*chip).reg_lock);
    if (*chip).mode
        & (if channel == SNDRV_PCM_STREAM_PLAYBACK {
            SB_MODE_PLAYBACK_16
        } else {
            SB_MODE_CAPTURE_16
        })
        != 0
    {
        snd_sb_ack_16bit(chip);
    } else {
        snd_sb_ack_8bit(chip);
    }
    if (*chip).mode & SB_RATE_LOCK == 0 {
        (*chip).locked_rate = rate;
        snd_sbdsp_command(chip, SB_DSP_SAMPLE_RATE_IN);
        snd_sbdsp_command(chip, (rate >> 8) as c_int);
        snd_sbdsp_command(chip, (rate & 0xff) as c_int);
        snd_sbdsp_command(chip, SB_DSP_SAMPLE_RATE_OUT);
        snd_sbdsp_command(chip, (rate >> 8) as c_int);
        snd_sbdsp_command(chip, (rate & 0xff) as c_int);
    }
}

unsafe fn snd_sb16_playback_prepare(substream: *mut snd_pcm_substream) -> c_int {
    let chip: *mut snd_sb = snd_pcm_substream_chip(substream);
    let runtime: *mut snd_pcm_runtime = (*substream).runtime;
    let format: u8;
    let mut size: u32;
    let mut count: u32;
    let dma: u32;

    snd_sb16_csp_playback_prepare(chip, runtime);
    if snd_pcm_format_unsigned((*runtime).format) > 0 {
        format = if (*runtime).channels > 1 {
            SB_DSP4_MODE_UNS_STEREO
        } else {
            SB_DSP4_MODE_UNS_MONO
        };
    } else {
        format = if (*runtime).channels > 1 {
            SB_DSP4_MODE_SIGN_STEREO
        } else {
            SB_DSP4_MODE_SIGN_MONO
        };
    }

    snd_sb16_setup_rate(chip, (*runtime).rate, SNDRV_PCM_STREAM_PLAYBACK);
    size = snd_pcm_lib_buffer_bytes(substream);
    (*chip).p_dma_size = size;
    dma = if (*chip).mode & SB_MODE_PLAYBACK_8 != 0 {
        (*chip).dma8 as u32
    } else {
        (*chip).dma16 as u32
    };
    snd_dma_program(dma, (*runtime).dma_addr, size, DMA_MODE_WRITE | DMA_AUTOINIT);

    count = snd_pcm_lib_period_bytes(substream);
    guard_spinlock_irqsave(&mut (*chip).reg_lock);
    if (*chip).mode & SB_MODE_PLAYBACK_16 != 0 {
        count >>= 1;
        count = count.wrapping_sub(1);
        snd_sbdsp_command(chip, SB_DSP4_OUT16_AI);
        snd_sbdsp_command(chip, format as c_int);
        snd_sbdsp_command(chip, (count & 0xff) as c_int);
        snd_sbdsp_command(chip, (count >> 8) as c_int);
        snd_sbdsp_command(chip, SB_DSP_DMA16_OFF);
    } else {
        count = count.wrapping_sub(1);
        snd_sbdsp_command(chip, SB_DSP4_OUT8_AI);
        snd_sbdsp_command(chip, format as c_int);
        snd_sbdsp_command(chip, (count & 0xff) as c_int);
        snd_sbdsp_command(chip, (count >> 8) as c_int);
        snd_sbdsp_command(chip, SB_DSP_DMA8_OFF);
    }
    0
}

unsafe fn snd_sb16_playback_trigger(substream: *mut snd_pcm_substream, cmd: c_int) -> c_int {
    let chip: *mut snd_sb = snd_pcm_substream_chip(substream);

    guard_spinlock(&mut (*chip).reg_lock);
    match cmd {
        SNDRV_PCM_TRIGGER_START | SNDRV_PCM_TRIGGER_RESUME => {
            (*chip).mode |= SB_RATE_LOCK_PLAYBACK;
            snd_sbdsp_command(
                chip,
                if (*chip).mode & SB_MODE_PLAYBACK_16 != 0 {
                    SB_DSP_DMA16_ON
                } else {
                    SB_DSP_DMA8_ON
                },
            );
        }
        SNDRV_PCM_TRIGGER_STOP | SNDRV_PCM_TRIGGER_SUSPEND => {
            snd_sbdsp_command(
                chip,
                if (*chip).mode & SB_MODE_PLAYBACK_16 != 0 {
                    SB_DSP_DMA16_OFF
                } else {
                    SB_DSP_DMA8_OFF
                },
            );
            /* next two lines are needed for some types of DSP4 (SB AWE 32 - 4.13) */
            if (*chip).mode & SB_RATE_LOCK_CAPTURE != 0 {
                snd_sbdsp_command(
                    chip,
                    if (*chip).mode & SB_MODE_CAPTURE_16 != 0 {
                        SB_DSP_DMA16_ON
                    } else {
                        SB_DSP_DMA8_ON
                    },
                );
            }
            (*chip).mode &= !SB_RATE_LOCK_PLAYBACK;
        }
        _ => return -EINVAL,
    }
    0
}

unsafe fn snd_sb16_capture_prepare(substream: *mut snd_pcm_substream) -> c_int {
    let chip: *mut snd_sb = snd_pcm_substream_chip(substream);
    let runtime: *mut snd_pcm_runtime = (*substream).runtime;
    let format: u8;
    let mut size: u32;
    let mut count: u32;
    let dma: u32;

    snd_sb16_csp_capture_prepare(chip, runtime);
    if snd_pcm_format_unsigned((*runtime).format) > 0 {
        format = if (*runtime).channels > 1 {
            SB_DSP4_MODE_UNS_STEREO
        } else {
            SB_DSP4_MODE_UNS_MONO
        };
    } else {
        format = if (*runtime).channels > 1 {
            SB_DSP4_MODE_SIGN_STEREO
        } else {
            SB_DSP4_MODE_SIGN_MONO
        };
    }
    snd_sb16_setup_rate(chip, (*runtime).rate, SNDRV_PCM_STREAM_CAPTURE);
    size = snd_pcm_lib_buffer_bytes(substream);
    (*chip).c_dma_size = size;
    dma = if (*chip).mode & SB_MODE_CAPTURE_8 != 0 {
        (*chip).dma8 as u32
    } else {
        (*chip).dma16 as u32
    };
    snd_dma_program(dma, (*runtime).dma_addr, size, DMA_MODE_READ | DMA_AUTOINIT);

    count = snd_pcm_lib_period_bytes(substream);
    guard_spinlock_irqsave(&mut (*chip).reg_lock);
    if (*chip).mode & SB_MODE_CAPTURE_16 != 0 {
        count >>= 1;
        count = count.wrapping_sub(1);
        snd_sbdsp_command(chip, SB_DSP4_IN16_AI);
        snd_sbdsp_command(chip, format as c_int);
        snd_sbdsp_command(chip, (count & 0xff) as c_int);
        snd_sbdsp_command(chip, (count >> 8) as c_int);
        snd_sbdsp_command(chip, SB_DSP_DMA16_OFF);
    } else {
        count = count.wrapping_sub(1);
        snd_sbdsp_command(chip, SB_DSP4_IN8_AI);
        snd_sbdsp_command(chip, format as c_int);
        snd_sbdsp_command(chip, (count & 0xff) as c_int);
        snd_sbdsp_command(chip, (count >> 8) as c_int);
        snd_sbdsp_command(chip, SB_DSP_DMA8_OFF);
    }
    0
}

unsafe fn snd_sb16_capture_trigger(substream: *mut snd_pcm_substream, cmd: c_int) -> c_int {
    let chip: *mut snd_sb = snd_pcm_substream_chip(substream);

    guard_spinlock(&mut (*chip).reg_lock);
    match cmd {
        SNDRV_PCM_TRIGGER_START | SNDRV_PCM_TRIGGER_RESUME => {
            (*chip).mode |= SB_RATE_LOCK_CAPTURE;
            snd_sbdsp_command(
                chip,
                if (*chip).mode & SB_MODE_CAPTURE_16 != 0 {
                    SB_DSP_DMA16_ON
                } else {
                    SB_DSP_DMA8_ON
                },
            );
        }
        SNDRV_PCM_TRIGGER_STOP | SNDRV_PCM_TRIGGER_SUSPEND => {
            snd_sbdsp_command(
                chip,
                if (*chip).mode & SB_MODE_CAPTURE_16 != 0 {
                    SB_DSP_DMA16_OFF
                } else {
                    SB_DSP_DMA8_OFF
                },
            );
            /* next two lines are needed for some types of DSP4 (SB AWE 32 - 4.13) */
            if (*chip).mode & SB_RATE_LOCK_PLAYBACK != 0 {
                snd_sbdsp_command(
                    chip,
                    if (*chip).mode & SB_MODE_PLAYBACK_16 != 0 {
                        SB_DSP_DMA16_ON
                    } else {
                        SB_DSP_DMA8_ON
                    },
                );
            }
            (*chip).mode &= !SB_RATE_LOCK_CAPTURE;
        }
        _ => return -EINVAL,
    }
    0
}

pub unsafe extern "C" fn snd_sb16dsp_interrupt(irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    let chip: *mut snd_sb = dev_id as *mut snd_sb;
    let status: u8;
    let mut ok: c_int;

    {
        let _guard = scoped_guard_spinlock(&mut (*chip).mixer_lock);
        status = snd_sbmixer_read(chip, SB_DSP4_IRQSTATUS);
    }
    if (status & SB_IRQTYPE_MPUIN != 0) && (*chip).rmidi_callback.is_some() {
        ((*chip).rmidi_callback.unwrap())(irq, (*(*chip).rmidi).private_data);
    }
    if status & SB_IRQTYPE_8BIT != 0 {
        ok = 0;
        if (*chip).mode & SB_MODE_PLAYBACK_8 != 0 {
            snd_pcm_period_elapsed((*chip).playback_substream);
            snd_sb16_csp_update(chip);
            ok += 1;
        }
        if (*chip).mode & SB_MODE_CAPTURE_8 != 0 {
            snd_pcm_period_elapsed((*chip).capture_substream);
            ok += 1;
        }
        {
            let _guard = scoped_guard_spinlock(&mut (*chip).reg_lock);
            if ok == 0 {
                snd_sbdsp_command(chip, SB_DSP_DMA8_OFF);
            }
            snd_sb_ack_8bit(chip);
        }
    }
    if status & SB_IRQTYPE_16BIT != 0 {
        ok = 0;
        if (*chip).mode & SB_MODE_PLAYBACK_16 != 0 {
            snd_pcm_period_elapsed((*chip).playback_substream);
            snd_sb16_csp_update(chip);
            ok += 1;
        }
        if (*chip).mode & SB_MODE_CAPTURE_16 != 0 {
            snd_pcm_period_elapsed((*chip).capture_substream);
            ok += 1;
        }
        {
            let _guard = scoped_guard_spinlock(&mut (*chip).reg_lock);
            if ok == 0 {
                snd_sbdsp_command(chip, SB_DSP_DMA16_OFF);
            }
            snd_sb_ack_16bit(chip);
        }
    }
    IRQ_HANDLED
}

/*

 */

unsafe fn snd_sb16_playback_pointer(substream: *mut snd_pcm_substream) -> snd_pcm_uframes_t {
    let chip: *mut snd_sb = snd_pcm_substream_chip(substream);
    let dma: u32;
    let ptr: usize;

    dma = if (*chip).mode & SB_MODE_PLAYBACK_8 != 0 {
        (*chip).dma8 as u32
    } else {
        (*chip).dma16 as u32
    };
    ptr = snd_dma_pointer(dma, (*chip).p_dma_size);
    bytes_to_frames((*substream).runtime, ptr)
}

unsafe fn snd_sb16_capture_pointer(substream: *mut snd_pcm_substream) -> snd_pcm_uframes_t {
    let chip: *mut snd_sb = snd_pcm_substream_chip(substream);
    let dma: u32;
    let ptr: usize;

    dma = if (*chip).mode & SB_MODE_CAPTURE_8 != 0 {
        (*chip).dma8 as u32
    } else {
        (*chip).dma16 as u32
    };
    ptr = snd_dma_pointer(dma, (*chip).c_dma_size);
    bytes_to_frames((*substream).runtime, ptr)
}

/*

 */

static snd_sb16_playback: snd_pcm_hardware = snd_pcm_hardware {
    info: SNDRV_PCM_INFO_MMAP | SNDRV_PCM_INFO_INTERLEAVED | SNDRV_PCM_INFO_MMAP_VALID,
    formats: 0,
    rates: SNDRV_PCM_RATE_CONTINUOUS | SNDRV_PCM_RATE_8000_44100,
    rate_min: 4000,
    rate_max: 44100,
    channels_min: 1,
    channels_max: 2,
    buffer_bytes_max: 128 * 1024,
    period_bytes_min: 64,
    period_bytes_max: 128 * 1024,
    periods_min: 1,
    periods_max: 1024,
    fifo_size: 0,
};

static snd_sb16_capture: snd_pcm_hardware = snd_pcm_hardware {
    info: SNDRV_PCM_INFO_MMAP | SNDRV_PCM_INFO_INTERLEAVED | SNDRV_PCM_INFO_MMAP_VALID,
    formats: 0,
    rates: SNDRV_PCM_RATE_CONTINUOUS | SNDRV_PCM_RATE_8000_44100,
    rate_min: 4000,
    rate_max: 44100,
    channels_min: 1,
    channels_max: 2,
    buffer_bytes_max: 128 * 1024,
    period_bytes_min: 64,
    period_bytes_max: 128 * 1024,
    periods_min: 1,
    periods_max: 1024,
    fifo_size: 0,
};

/*
 *  open/close
 */

unsafe fn snd_sb16_playback_open(substream: *mut snd_pcm_substream) -> c_int {
    let chip: *mut snd_sb = snd_pcm_substream_chip(substream);
    let runtime: *mut snd_pcm_runtime = (*substream).runtime;

    guard_spinlock_irqsave(&mut (*chip).open_lock);
    if (*chip).mode & SB_MODE_PLAYBACK != 0 {
        return -EAGAIN;
    }
    (*runtime).hw = snd_sb16_playback;

    if (*chip).force_mode16 & SB_MODE_CAPTURE_16 == 0
        && (*chip).dma16 >= 0
        && (*chip).mode & SB_MODE_CAPTURE_16 == 0
    {
        (*chip).mode |= SB_MODE_PLAYBACK_16;
        (*runtime).hw.formats = SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_U16_LE;
        /* Vibra16X hack */
        if (*chip).dma16 <= 3 {
            (*runtime).hw.buffer_bytes_max = 64 * 1024;
            (*runtime).hw.period_bytes_max = 64 * 1024;
        } else {
            snd_sb16_csp_playback_open(chip, runtime);
        }
    } else if (*chip).dma8 >= 0 && (*chip).mode & SB_MODE_CAPTURE_8 == 0 {
        (*chip).mode |= SB_MODE_PLAYBACK_8;
        /* DSP v 4.xx can transfer 16bit data through 8bit DMA channel, SBHWPG 2-7 */
        if (*chip).dma16 < 0 {
            (*runtime).hw.formats = SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_U16_LE;
            (*chip).mode |= SB_MODE_PLAYBACK_16;
        } else {
            (*runtime).hw.formats = SNDRV_PCM_FMTBIT_U8 | SNDRV_PCM_FMTBIT_S8;
        }
        (*runtime).hw.buffer_bytes_max = 64 * 1024;
        (*runtime).hw.period_bytes_max = 64 * 1024;
    } else {
        return -EAGAIN;
    }

    if (*chip).hardware == SB_HW_ALS100 {
        (*runtime).hw.rate_max = 48000;
    }
    if (*chip).hardware == SB_HW_CS5530 {
        (*runtime).hw.buffer_bytes_max = 32 * 1024;
        (*runtime).hw.periods_min = 2;
        (*runtime).hw.rate_min = 44100;
    }
    if (*chip).mode & SB_RATE_LOCK != 0 {
        (*runtime).hw.rate_max = (*chip).locked_rate;
        (*runtime).hw.rate_min = (*runtime).hw.rate_max;
    }
    (*chip).playback_substream = substream;
    0
}

unsafe fn snd_sb16_playback_close(substream: *mut snd_pcm_substream) -> c_int {
    let chip: *mut snd_sb = snd_pcm_substream_chip(substream);

    snd_sb16_csp_playback_close(chip);
    guard_spinlock_irqsave(&mut (*chip).open_lock);
    (*chip).playback_substream = core::ptr::null_mut();
    (*chip).mode &= !SB_MODE_PLAYBACK;
    0
}

unsafe fn snd_sb16_capture_open(substream: *mut snd_pcm_substream) -> c_int {
    let chip: *mut snd_sb = snd_pcm_substream_chip(substream);
    let runtime: *mut snd_pcm_runtime = (*substream).runtime;

    guard_spinlock_irqsave(&mut (*chip).open_lock);
    if (*chip).mode & SB_MODE_CAPTURE != 0 {
        return -EAGAIN;
    }
    (*runtime).hw = snd_sb16_capture;

    if (*chip).force_mode16 & SB_MODE_PLAYBACK_16 == 0
        && (*chip).dma16 >= 0
        && (*chip).mode & SB_MODE_PLAYBACK_16 == 0
    {
        (*chip).mode |= SB_MODE_CAPTURE_16;
        (*runtime).hw.formats = SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_U16_LE;
        /* Vibra16X hack */
        if (*chip).dma16 <= 3 {
            (*runtime).hw.buffer_bytes_max = 64 * 1024;
            (*runtime).hw.period_bytes_max = 64 * 1024;
        } else {
            snd_sb16_csp_capture_open(chip, runtime);
        }
    } else if (*chip).dma8 >= 0 && (*chip).mode & SB_MODE_PLAYBACK_8 == 0 {
        (*chip).mode |= SB_MODE_CAPTURE_8;
        /* DSP v 4.xx can transfer 16bit data through 8bit DMA channel, SBHWPG 2-7 */
        if (*chip).dma16 < 0 {
            (*runtime).hw.formats = SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_U16_LE;
            (*chip).mode |= SB_MODE_CAPTURE_16;
        } else {
            (*runtime).hw.formats = SNDRV_PCM_FMTBIT_U8 | SNDRV_PCM_FMTBIT_S8;
        }
        (*runtime).hw.buffer_bytes_max = 64 * 1024;
        (*runtime).hw.period_bytes_max = 64 * 1024;
    } else {
        return -EAGAIN;
    }

    if (*chip).hardware == SB_HW_ALS100 {
        (*runtime).hw.rate_max = 48000;
    }
    if (*chip).hardware == SB_HW_CS5530 {
        (*runtime).hw.buffer_bytes_max = 32 * 1024;
        (*runtime).hw.periods_min = 2;
        (*runtime).hw.rate_min = 44100;
    }
    if (*chip).mode & SB_RATE_LOCK != 0 {
        (*runtime).hw.rate_max = (*chip).locked_rate;
        (*runtime).hw.rate_min = (*runtime).hw.rate_max;
    }
    (*chip).capture_substream = substream;
    0
}

unsafe fn snd_sb16_capture_close(substream: *mut snd_pcm_substream) -> c_int {
    let chip: *mut snd_sb = snd_pcm_substream_chip(substream);

    snd_sb16_csp_capture_close(chip);
    guard_spinlock_irqsave(&mut (*chip).open_lock);
    (*chip).capture_substream = core::ptr::null_mut();
    (*chip).mode &= !SB_MODE_CAPTURE;
    0
}

/*
 *  DMA control interface
 */

unsafe fn snd_sb16_set_dma_mode(chip: *mut snd_sb, what: c_int) -> c_int {
    if (*chip).dma8 < 0 || (*chip).dma16 < 0 {
        if snd_BUG_ON(what) != 0 {
            return -EINVAL;
        }
        return 0;
    }
    if what == 0 {
        (*chip).force_mode16 = 0;
    } else if what == 1 {
        (*chip).force_mode16 = SB_MODE_PLAYBACK_16;
    } else if what == 2 {
        (*chip).force_mode16 = SB_MODE_CAPTURE_16;
    } else {
        return -EINVAL;
    }
    0
}

unsafe fn snd_sb16_get_dma_mode(chip: *mut snd_sb) -> c_int {
    if (*chip).dma8 < 0 || (*chip).dma16 < 0 {
        return 0;
    }
    match (*chip).force_mode16 {
        SB_MODE_PLAYBACK_16 => 1,
        SB_MODE_CAPTURE_16 => 2,
        _ => 0,
    }
}

unsafe fn snd_sb16_dma_control_info(
    _kcontrol: *mut snd_kcontrol,
    uinfo: *mut snd_ctl_elem_info,
) -> c_int {
    static texts: [*const c_char; 3] = [
        b"Auto\0".as_ptr() as *const c_char,
        b"Playback\0".as_ptr() as *const c_char,
        b"Capture\0".as_ptr() as *const c_char,
    ];

    snd_ctl_enum_info(uinfo, 1, 3, texts.as_ptr())
}

unsafe fn snd_sb16_dma_control_get(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let chip: *mut snd_sb = snd_kcontrol_chip(kcontrol);

    guard_spinlock_irqsave(&mut (*chip).reg_lock);
    (*ucontrol).value.enumerated.item[0] = snd_sb16_get_dma_mode(chip) as _;
    0
}

unsafe fn snd_sb16_dma_control_put(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let chip: *mut snd_sb = snd_kcontrol_chip(kcontrol);
    let nval: u8;
    let oval: u8;
    let change: c_int;

    if (*chip).mode & (SB_MODE_PLAYBACK | SB_MODE_CAPTURE) != 0 {
        return -EBUSY;
    }

    nval = (*ucontrol).value.enumerated.item[0] as u8;
    if nval > 2 {
        return -EINVAL;
    }
    {
        let _guard = scoped_guard_spinlock_irqsave(&mut (*chip).reg_lock);
        oval = snd_sb16_get_dma_mode(chip) as u8;
        change = (nval != oval) as c_int;
        snd_sb16_set_dma_mode(chip, nval as c_int);
    }
    if change != 0 {
        snd_dma_disable((*chip).dma8);
        snd_dma_disable((*chip).dma16);
    }
    change
}

static snd_sb16_dma_control: snd_kcontrol_new = snd_kcontrol_new {
    iface: SNDRV_CTL_ELEM_IFACE_CARD,
    name: b"16-bit DMA Allocation\0".as_ptr() as *const c_char,
    info: Some(snd_sb16_dma_control_info),
    get: Some(snd_sb16_dma_control_get),
    put: Some(snd_sb16_dma_control_put),
};

/*
 *  Initialization part
 */

pub unsafe extern "C" fn snd_sb16dsp_configure(chip: *mut snd_sb) -> c_int {
    let mut irqreg: u8 = 0;
    let mut dmareg: u8 = 0;
    let mut mpureg: u8;
    let realirq: u8;
    let realdma: u8;
    let realmpureg: u8;
    /* note: mpu register should be present only on SB16 Vibra soundcards */

    {
        let _guard = scoped_guard_spinlock_irqsave(&mut (*chip).mixer_lock);
        mpureg = snd_sbmixer_read(chip, SB_DSP4_MPUSETUP) & !0x06;
    }
    match (*chip).irq {
        2 | 9 => {
            irqreg |= SB_IRQSETUP_IRQ9;
        }
        5 => {
            irqreg |= SB_IRQSETUP_IRQ5;
        }
        7 => {
            irqreg |= SB_IRQSETUP_IRQ7;
        }
        10 => {
            irqreg |= SB_IRQSETUP_IRQ10;
        }
        _ => return -EINVAL,
    }
    if (*chip).dma8 >= 0 {
        match (*chip).dma8 {
            0 => {
                dmareg |= SB_DMASETUP_DMA0;
            }
            1 => {
                dmareg |= SB_DMASETUP_DMA1;
            }
            3 => {
                dmareg |= SB_DMASETUP_DMA3;
            }
            _ => return -EINVAL,
        }
    }
    if (*chip).dma16 >= 0 && (*chip).dma16 != (*chip).dma8 {
        match (*chip).dma16 {
            5 => {
                dmareg |= SB_DMASETUP_DMA5;
            }
            6 => {
                dmareg |= SB_DMASETUP_DMA6;
            }
            7 => {
                dmareg |= SB_DMASETUP_DMA7;
            }
            _ => return -EINVAL,
        }
    }
    match (*chip).mpu_port {
        0x300 => {
            mpureg |= 0x04;
        }
        0x330 => {
            mpureg |= 0x00;
        }
        _ => {
            mpureg |= 0x02; /* disable MPU */
        }
    }

    {
        let _guard = scoped_guard_spinlock_irqsave(&mut (*chip).mixer_lock);
        snd_sbmixer_write(chip, SB_DSP4_IRQSETUP, irqreg);
        realirq = snd_sbmixer_read(chip, SB_DSP4_IRQSETUP);

        snd_sbmixer_write(chip, SB_DSP4_DMASETUP, dmareg);
        realdma = snd_sbmixer_read(chip, SB_DSP4_DMASETUP);

        snd_sbmixer_write(chip, SB_DSP4_MPUSETUP, mpureg);
        realmpureg = snd_sbmixer_read(chip, SB_DSP4_MPUSETUP);
    }
    if ((!realirq) & irqreg != 0) || ((!realdma) & dmareg != 0) {
        dev_err(
            (*(*chip).card).dev,
            b"SB16 [0x%lx]: unable to set DMA & IRQ (PnP device?)\n\0".as_ptr() as *const c_char,
            (*chip).port,
        );
        dev_err(
            (*(*chip).card).dev,
            b"SB16 [0x%lx]: wanted: irqreg=0x%x, dmareg=0x%x, mpureg = 0x%x\n\0".as_ptr()
                as *const c_char,
            (*chip).port,
            realirq,
            realdma,
            realmpureg,
        );
        dev_err(
            (*(*chip).card).dev,
            b"SB16 [0x%lx]:    got: irqreg=0x%x, dmareg=0x%x, mpureg = 0x%x\n\0".as_ptr()
                as *const c_char,
            (*chip).port,
            irqreg,
            dmareg,
            mpureg,
        );
        return -ENODEV;
    }
    0
}

static snd_sb16_playback_ops: snd_pcm_ops = snd_pcm_ops {
    open: Some(snd_sb16_playback_open),
    close: Some(snd_sb16_playback_close),
    prepare: Some(snd_sb16_playback_prepare),
    trigger: Some(snd_sb16_playback_trigger),
    pointer: Some(snd_sb16_playback_pointer),
};

static snd_sb16_capture_ops: snd_pcm_ops = snd_pcm_ops {
    open: Some(snd_sb16_capture_open),
    close: Some(snd_sb16_capture_close),
    prepare: Some(snd_sb16_capture_prepare),
    trigger: Some(snd_sb16_capture_trigger),
    pointer: Some(snd_sb16_capture_pointer),
};

pub unsafe extern "C" fn snd_sb16dsp_pcm(chip: *mut snd_sb, device: c_int) -> c_int {
    let card: *mut snd_card = (*chip).card;
    let mut pcm: *mut snd_pcm = core::ptr::null_mut();
    let err: c_int;

    err = snd_pcm_new(card, b"SB16 DSP\0".as_ptr() as *const c_char, device, 1, 1, &mut pcm);
    if err < 0 {
        return err;
    }
    sprintf(
        (*pcm).name.as_mut_ptr(),
        b"DSP v%i.%i\0".as_ptr() as *const c_char,
        (*chip).version >> 8,
        (*chip).version & 0xff,
    );
    (*pcm).info_flags = SNDRV_PCM_INFO_JOINT_DUPLEX;
    (*pcm).private_data = chip as *mut c_void;
    (*chip).pcm = pcm;

    snd_pcm_set_ops(pcm, SNDRV_PCM_STREAM_PLAYBACK, &snd_sb16_playback_ops);
    snd_pcm_set_ops(pcm, SNDRV_PCM_STREAM_CAPTURE, &snd_sb16_capture_ops);

    if (*chip).dma16 >= 0 && (*chip).dma8 != (*chip).dma16 {
        snd_ctl_add(card, snd_ctl_new1(&snd_sb16_dma_control, chip as *mut c_void));
    } else {
        (*pcm).info_flags = SNDRV_PCM_INFO_HALF_DUPLEX;
    }

    snd_pcm_set_managed_buffer_all(
        pcm,
        SNDRV_DMA_TYPE_DEV,
        (*card).dev,
        64 * 1024,
        128 * 1024,
    );
    0
}

pub unsafe extern "C" fn snd_sb16dsp_get_pcm_ops(direction: c_int) -> *const snd_pcm_ops {
    if direction == SNDRV_PCM_STREAM_PLAYBACK {
        &snd_sb16_playback_ops
    } else {
        &snd_sb16_capture_ops
    }
}

// EXPORT_SYMBOL(snd_sb16dsp_pcm);
// EXPORT_SYMBOL(snd_sb16dsp_get_pcm_ops);
// EXPORT_SYMBOL(snd_sb16dsp_configure);
// EXPORT_SYMBOL(snd_sb16dsp_interrupt);

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
