// SPDX-License-Identifier: GPL-2.0-or-later
/*
 */

/*
 * Vortex PCM ALSA driver.
 *
 * Supports ADB and WT DMA. Unfortunately, WT channels do not run yet.
 * It remains stuck,and DMA transfers do not happen.
 */

// Dependencies from the original C includes:
// <sound/asoundef.h>, <linux/time.h>, <sound/core.h>,
// <sound/pcm.h>, <sound/pcm_params.h>, and "au88x0.h".

unsafe fn VORTEX_PCM_TYPE(x: *mut snd_pcm) -> *mut u8 {
    unsafe { (*x).name.as_mut_ptr().add(40) }
}

/* hardware definition */
static snd_vortex_playback_hw_adb: snd_pcm_hardware = snd_pcm_hardware {
    info: SNDRV_PCM_INFO_MMAP |
        /* SNDRV_PCM_INFO_RESUME | */
        SNDRV_PCM_INFO_PAUSE | SNDRV_PCM_INFO_INTERLEAVED |
        SNDRV_PCM_INFO_MMAP_VALID,
    formats: SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_U8 |
        SNDRV_PCM_FMTBIT_MU_LAW | SNDRV_PCM_FMTBIT_A_LAW,
    rates: SNDRV_PCM_RATE_CONTINUOUS,
    rate_min: 5000,
    rate_max: 48000,
    channels_min: 1,
    channels_max: 2,
    buffer_bytes_max: 0x10000,
    period_bytes_min: 0x20,
    period_bytes_max: 0x1000,
    periods_min: 2,
    periods_max: 1024,
};

// Original C condition: #ifndef CHIP_AU8820
#[cfg(not(CHIP_AU8820))]
static snd_vortex_playback_hw_a3d: snd_pcm_hardware = snd_pcm_hardware {
    info: SNDRV_PCM_INFO_MMAP |
        /* SNDRV_PCM_INFO_RESUME | */
        SNDRV_PCM_INFO_PAUSE | SNDRV_PCM_INFO_INTERLEAVED |
        SNDRV_PCM_INFO_MMAP_VALID,
    formats: SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_U8 |
        SNDRV_PCM_FMTBIT_MU_LAW | SNDRV_PCM_FMTBIT_A_LAW,
    rates: SNDRV_PCM_RATE_CONTINUOUS,
    rate_min: 5000,
    rate_max: 48000,
    channels_min: 1,
    channels_max: 1,
    buffer_bytes_max: 0x10000,
    period_bytes_min: 0x100,
    period_bytes_max: 0x1000,
    periods_min: 2,
    periods_max: 64,
};

static snd_vortex_playback_hw_spdif: snd_pcm_hardware = snd_pcm_hardware {
    info: SNDRV_PCM_INFO_MMAP |
        /* SNDRV_PCM_INFO_RESUME | */
        SNDRV_PCM_INFO_PAUSE | SNDRV_PCM_INFO_INTERLEAVED |
        SNDRV_PCM_INFO_MMAP_VALID,
    formats: SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_U8 |
        SNDRV_PCM_FMTBIT_IEC958_SUBFRAME_LE | SNDRV_PCM_FMTBIT_MU_LAW |
        SNDRV_PCM_FMTBIT_A_LAW,
    rates: SNDRV_PCM_RATE_32000 | SNDRV_PCM_RATE_44100 | SNDRV_PCM_RATE_48000,
    rate_min: 32000,
    rate_max: 48000,
    channels_min: 1,
    channels_max: 2,
    buffer_bytes_max: 0x10000,
    period_bytes_min: 0x100,
    period_bytes_max: 0x1000,
    periods_min: 2,
    periods_max: 64,
};

// Original C condition: #ifndef CHIP_AU8810
#[cfg(not(CHIP_AU8810))]
static snd_vortex_playback_hw_wt: snd_pcm_hardware = snd_pcm_hardware {
    info: SNDRV_PCM_INFO_MMAP |
        SNDRV_PCM_INFO_INTERLEAVED |
        SNDRV_PCM_INFO_BLOCK_TRANSFER | SNDRV_PCM_INFO_MMAP_VALID,
    formats: SNDRV_PCM_FMTBIT_S16_LE,
    rates: SNDRV_PCM_RATE_8000_48000 | SNDRV_PCM_RATE_CONTINUOUS, // SNDRV_PCM_RATE_48000,
    rate_min: 8000,
    rate_max: 48000,
    channels_min: 1,
    channels_max: 2,
    buffer_bytes_max: 0x10000,
    period_bytes_min: 0x0400,
    period_bytes_max: 0x1000,
    periods_min: 2,
    periods_max: 64,
};

// Original C condition: #ifdef CHIP_AU8830
#[cfg(CHIP_AU8830)]
static au8830_channels: [u32; 3] = [1, 2, 4];

#[cfg(CHIP_AU8830)]
static hw_constraints_au8830_channels: snd_pcm_hw_constraint_list = snd_pcm_hw_constraint_list {
    count: au8830_channels.len() as u32,
    list: au8830_channels.as_ptr(),
    mask: 0,
};

unsafe fn vortex_notify_pcm_vol_change(card: *mut snd_card, kctl: *mut snd_kcontrol, activate: i32) {
    unsafe {
        if activate != 0 {
            (*kctl).vd[0].access &= !SNDRV_CTL_ELEM_ACCESS_INACTIVE;
        } else {
            (*kctl).vd[0].access |= SNDRV_CTL_ELEM_ACCESS_INACTIVE;
        }
        snd_ctl_notify(
            card,
            SNDRV_CTL_EVENT_MASK_VALUE | SNDRV_CTL_EVENT_MASK_INFO,
            &mut (*kctl).id,
        );
    }
}

/* open callback */
unsafe fn snd_vortex_pcm_open(substream: *mut snd_pcm_substream) -> i32 {
    unsafe {
        let vortex: *mut vortex_t = snd_pcm_substream_chip(substream);
        let runtime: *mut snd_pcm_runtime = (*substream).runtime;
        let mut err: i32;

        /* Force equal size periods */
        err = snd_pcm_hw_constraint_integer(runtime, SNDRV_PCM_HW_PARAM_PERIODS);
        if err < 0 {
            return err;
        }
        /* Avoid PAGE_SIZE boundary to fall inside of a period. */
        err = snd_pcm_hw_constraint_pow2(runtime, 0, SNDRV_PCM_HW_PARAM_PERIOD_BYTES);
        if err < 0 {
            return err;
        }

        snd_pcm_hw_constraint_step(runtime, 0, SNDRV_PCM_HW_PARAM_BUFFER_BYTES, 64);

        if *VORTEX_PCM_TYPE((*substream).pcm) as i32 != VORTEX_PCM_WT {
            #[cfg(not(CHIP_AU8820))]
            {
                if *VORTEX_PCM_TYPE((*substream).pcm) as i32 == VORTEX_PCM_A3D {
                    (*runtime).hw = snd_vortex_playback_hw_a3d;
                }
            }
            if *VORTEX_PCM_TYPE((*substream).pcm) as i32 == VORTEX_PCM_SPDIF {
                (*runtime).hw = snd_vortex_playback_hw_spdif;
                match (*vortex).spdif_sr {
                    32000 => (*runtime).hw.rates = SNDRV_PCM_RATE_32000,
                    44100 => (*runtime).hw.rates = SNDRV_PCM_RATE_44100,
                    48000 => (*runtime).hw.rates = SNDRV_PCM_RATE_48000,
                    _ => {}
                }
            }
            if *VORTEX_PCM_TYPE((*substream).pcm) as i32 == VORTEX_PCM_ADB ||
                *VORTEX_PCM_TYPE((*substream).pcm) as i32 == VORTEX_PCM_I2S
            {
                (*runtime).hw = snd_vortex_playback_hw_adb;
            }
            #[cfg(CHIP_AU8830)]
            {
                if (*substream).stream == SNDRV_PCM_STREAM_PLAYBACK &&
                    VORTEX_IS_QUAD(vortex) != 0 &&
                    *VORTEX_PCM_TYPE((*substream).pcm) as i32 == VORTEX_PCM_ADB
                {
                    (*runtime).hw.channels_max = 4;
                    snd_pcm_hw_constraint_list(
                        runtime,
                        0,
                        SNDRV_PCM_HW_PARAM_CHANNELS,
                        &hw_constraints_au8830_channels,
                    );
                }
            }
            (*(*substream).runtime).private_data = core::ptr::null_mut();
        }
        #[cfg(not(CHIP_AU8810))]
        else {
            (*runtime).hw = snd_vortex_playback_hw_wt;
            (*(*substream).runtime).private_data = core::ptr::null_mut();
        }
        0
    }
}

/* close callback */
unsafe fn snd_vortex_pcm_close(substream: *mut snd_pcm_substream) -> i32 {
    unsafe {
        //vortex_t *chip = snd_pcm_substream_chip(substream);
        let stream: *mut stream_t = (*(*substream).runtime).private_data as *mut stream_t;

        // the hardware-specific codes will be here
        if !stream.is_null() {
            (*stream).substream = core::ptr::null_mut();
            (*stream).nr_ch = 0;
        }
        (*(*substream).runtime).private_data = core::ptr::null_mut();
        0
    }
}

/* hw_params callback */
unsafe fn snd_vortex_pcm_hw_params(
    substream: *mut snd_pcm_substream,
    hw_params: *mut snd_pcm_hw_params,
) -> i32 {
    unsafe {
        let chip: *mut vortex_t = snd_pcm_substream_chip(substream);
        let mut stream: *mut stream_t = (*(*substream).runtime).private_data as *mut stream_t;

        /*
           pr_info( "Vortex: periods %d, period_bytes %d, channels = %d\n", params_periods(hw_params),
           params_period_bytes(hw_params), params_channels(hw_params));
         */
        spin_lock_irq(&mut (*chip).lock);
        // Make audio routes and config buffer DMA.
        if *VORTEX_PCM_TYPE((*substream).pcm) as i32 != VORTEX_PCM_WT {
            let dma: i32;
            let r#type: i32 = *VORTEX_PCM_TYPE((*substream).pcm) as i32;
            /* Dealloc any routes. */
            if !stream.is_null() {
                vortex_adb_allocroute(
                    chip,
                    (*stream).dma,
                    (*stream).nr_ch,
                    (*stream).dir,
                    (*stream).r#type,
                    (*substream).number,
                );
            }
            /* Alloc routes. */
            dma = vortex_adb_allocroute(
                chip,
                -1,
                params_channels(hw_params),
                (*substream).stream,
                r#type,
                (*substream).number,
            );
            if dma < 0 {
                spin_unlock_irq(&mut (*chip).lock);
                return dma;
            }
            stream = &mut (*chip).dma_adb[dma as usize];
            (*(*substream).runtime).private_data = stream as *mut _;
            (*stream).substream = substream;
            /* Setup Buffers. */
            vortex_adbdma_setbuffers(
                chip,
                dma,
                params_period_bytes(hw_params),
                params_periods(hw_params),
            );
            if *VORTEX_PCM_TYPE((*substream).pcm) as i32 == VORTEX_PCM_ADB {
                (*chip).pcm_vol[(*substream).number as usize].active = 1;
                vortex_notify_pcm_vol_change(
                    (*chip).card,
                    (*chip).pcm_vol[(*substream).number as usize].kctl,
                    1,
                );
            }
        }
        #[cfg(not(CHIP_AU8810))]
        else {
            /* if (stream != NULL)
               vortex_wt_allocroute(chip, substream->number, 0); */
            vortex_wt_allocroute(chip, (*substream).number, params_channels(hw_params));
            stream = &mut (*chip).dma_wt[(*substream).number as usize];
            (*(*substream).runtime).private_data = stream as *mut _;
            (*stream).dma = (*substream).number;
            (*stream).substream = substream;
            vortex_wtdma_setbuffers(
                chip,
                (*substream).number,
                params_period_bytes(hw_params),
                params_periods(hw_params),
            );
        }
        spin_unlock_irq(&mut (*chip).lock);
        0
    }
}

/* hw_free callback */
unsafe fn snd_vortex_pcm_hw_free(substream: *mut snd_pcm_substream) -> i32 {
    unsafe {
        let chip: *mut vortex_t = snd_pcm_substream_chip(substream);
        let stream: *mut stream_t = (*(*substream).runtime).private_data as *mut stream_t;

        spin_lock_irq(&mut (*chip).lock);
        // Delete audio routes.
        if *VORTEX_PCM_TYPE((*substream).pcm) as i32 != VORTEX_PCM_WT {
            if !stream.is_null() {
                if *VORTEX_PCM_TYPE((*substream).pcm) as i32 == VORTEX_PCM_ADB {
                    (*chip).pcm_vol[(*substream).number as usize].active = 0;
                    vortex_notify_pcm_vol_change(
                        (*chip).card,
                        (*chip).pcm_vol[(*substream).number as usize].kctl,
                        0,
                    );
                }
                vortex_adb_allocroute(
                    chip,
                    (*stream).dma,
                    (*stream).nr_ch,
                    (*stream).dir,
                    (*stream).r#type,
                    (*substream).number,
                );
            }
        }
        #[cfg(not(CHIP_AU8810))]
        else {
            if !stream.is_null() {
                vortex_wt_allocroute(chip, (*stream).dma, 0);
            }
        }
        (*(*substream).runtime).private_data = core::ptr::null_mut();
        spin_unlock_irq(&mut (*chip).lock);

        0
    }
}

/* prepare callback */
unsafe fn snd_vortex_pcm_prepare(substream: *mut snd_pcm_substream) -> i32 {
    unsafe {
        let chip: *mut vortex_t = snd_pcm_substream_chip(substream);
        let runtime: *mut snd_pcm_runtime = (*substream).runtime;
        let stream: *mut stream_t = (*(*substream).runtime).private_data as *mut stream_t;
        let dma: i32 = (*stream).dma;
        let fmt: i32;
        let dir: i32;

        // set up the hardware with the current configuration.
        if (*substream).stream == SNDRV_PCM_STREAM_PLAYBACK {
            dir = 1;
        } else {
            dir = 0;
        }
        fmt = vortex_alsafmt_aspfmt((*runtime).format, chip);
        spin_lock_irq(&mut (*chip).lock);
        if *VORTEX_PCM_TYPE((*substream).pcm) as i32 != VORTEX_PCM_WT {
            vortex_adbdma_setmode(
                chip,
                dma,
                1,
                dir,
                fmt,
                if (*runtime).channels == 1 { 0 } else { 1 },
                0,
            );
            vortex_adbdma_setstartbuffer(chip, dma, 0);
            if *VORTEX_PCM_TYPE((*substream).pcm) as i32 != VORTEX_PCM_SPDIF {
                vortex_adb_setsrc(chip, dma, (*runtime).rate, dir);
            }
        }
        #[cfg(not(CHIP_AU8810))]
        else {
            vortex_wtdma_setmode(chip, dma, 1, fmt, 0, 0);
            // FIXME: Set rate (i guess using vortex_wt_writereg() somehow).
            vortex_wtdma_setstartbuffer(chip, dma, 0);
        }
        spin_unlock_irq(&mut (*chip).lock);
        0
    }
}

/* trigger callback */
unsafe fn snd_vortex_pcm_trigger(substream: *mut snd_pcm_substream, cmd: i32) -> i32 {
    unsafe {
        let chip: *mut vortex_t = snd_pcm_substream_chip(substream);
        let stream: *mut stream_t = (*(*substream).runtime).private_data as *mut stream_t;
        let dma: i32 = (*stream).dma;

        spin_lock(&mut (*chip).lock);
        match cmd {
            SNDRV_PCM_TRIGGER_START => {
                // do something to start the PCM engine
                //printk(KERN_INFO "vortex: start %d\n", dma);
                (*stream).fifo_enabled = 1;
                if *VORTEX_PCM_TYPE((*substream).pcm) as i32 != VORTEX_PCM_WT {
                    vortex_adbdma_resetup(chip, dma);
                    vortex_adbdma_startfifo(chip, dma);
                }
                #[cfg(not(CHIP_AU8810))]
                else {
                    dev_info((*(*chip).card).dev, c"wt start %d\n".as_ptr(), dma);
                    vortex_wtdma_startfifo(chip, dma);
                }
            }
            SNDRV_PCM_TRIGGER_STOP => {
                // do something to stop the PCM engine
                //printk(KERN_INFO "vortex: stop %d\n", dma);
                (*stream).fifo_enabled = 0;
                if *VORTEX_PCM_TYPE((*substream).pcm) as i32 != VORTEX_PCM_WT {
                    vortex_adbdma_stopfifo(chip, dma);
                }
                #[cfg(not(CHIP_AU8810))]
                else {
                    dev_info((*(*chip).card).dev, c"wt stop %d\n".as_ptr(), dma);
                    vortex_wtdma_stopfifo(chip, dma);
                }
            }
            SNDRV_PCM_TRIGGER_PAUSE_PUSH => {
                //printk(KERN_INFO "vortex: pause %d\n", dma);
                if *VORTEX_PCM_TYPE((*substream).pcm) as i32 != VORTEX_PCM_WT {
                    vortex_adbdma_pausefifo(chip, dma);
                }
                #[cfg(not(CHIP_AU8810))]
                else {
                    vortex_wtdma_pausefifo(chip, dma);
                }
            }
            SNDRV_PCM_TRIGGER_PAUSE_RELEASE => {
                //printk(KERN_INFO "vortex: resume %d\n", dma);
                if *VORTEX_PCM_TYPE((*substream).pcm) as i32 != VORTEX_PCM_WT {
                    vortex_adbdma_resumefifo(chip, dma);
                }
                #[cfg(not(CHIP_AU8810))]
                else {
                    vortex_wtdma_resumefifo(chip, dma);
                }
            }
            _ => {
                spin_unlock(&mut (*chip).lock);
                return -EINVAL;
            }
        }
        spin_unlock(&mut (*chip).lock);
        0
    }
}

/* pointer callback */
unsafe fn snd_vortex_pcm_pointer(substream: *mut snd_pcm_substream) -> snd_pcm_uframes_t {
    unsafe {
        let chip: *mut vortex_t = snd_pcm_substream_chip(substream);
        let stream: *mut stream_t = (*(*substream).runtime).private_data as *mut stream_t;
        let dma: i32 = (*stream).dma;
        let mut current_ptr: snd_pcm_uframes_t = 0;

        spin_lock(&mut (*chip).lock);
        if *VORTEX_PCM_TYPE((*substream).pcm) as i32 != VORTEX_PCM_WT {
            current_ptr = vortex_adbdma_getlinearpos(chip, dma);
        }
        #[cfg(not(CHIP_AU8810))]
        else {
            current_ptr = vortex_wtdma_getlinearpos(chip, dma);
        }
        //printk(KERN_INFO "vortex: pointer = 0x%x\n", current_ptr);
        spin_unlock(&mut (*chip).lock);
        current_ptr = bytes_to_frames((*substream).runtime, current_ptr);
        if current_ptr >= (*(*substream).runtime).buffer_size {
            current_ptr = 0;
        }
        current_ptr
    }
}

/* operators */
static snd_vortex_playback_ops: snd_pcm_ops = snd_pcm_ops {
    open: Some(snd_vortex_pcm_open),
    close: Some(snd_vortex_pcm_close),
    hw_params: Some(snd_vortex_pcm_hw_params),
    hw_free: Some(snd_vortex_pcm_hw_free),
    prepare: Some(snd_vortex_pcm_prepare),
    trigger: Some(snd_vortex_pcm_trigger),
    pointer: Some(snd_vortex_pcm_pointer),
};

/*
*  definitions of capture are omitted here...
*/

static vortex_pcm_prettyname: [*const i8; VORTEX_PCM_LAST as usize] = [
    concat!(CARD_NAME, " ADB\0").as_ptr() as *const i8,
    concat!(CARD_NAME, " SPDIF\0").as_ptr() as *const i8,
    concat!(CARD_NAME, " A3D\0").as_ptr() as *const i8,
    concat!(CARD_NAME, " WT\0").as_ptr() as *const i8,
    concat!(CARD_NAME, " I2S\0").as_ptr() as *const i8,
];

static vortex_pcm_name: [*const i8; VORTEX_PCM_LAST as usize] = [
    c"adb".as_ptr(),
    c"spdif".as_ptr(),
    c"a3d".as_ptr(),
    c"wt".as_ptr(),
    c"i2s".as_ptr(),
];

/* SPDIF kcontrol */

unsafe fn snd_vortex_spdif_info(
    _kcontrol: *mut snd_kcontrol,
    uinfo: *mut snd_ctl_elem_info,
) -> i32 {
    unsafe {
        (*uinfo).type_ = SNDRV_CTL_ELEM_TYPE_IEC958;
        (*uinfo).count = 1;
        0
    }
}

unsafe fn snd_vortex_spdif_mask_get(
    _kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> i32 {
    unsafe {
        (*ucontrol).value.iec958.status[0] = 0xff;
        (*ucontrol).value.iec958.status[1] = 0xff;
        (*ucontrol).value.iec958.status[2] = 0xff;
        (*ucontrol).value.iec958.status[3] = IEC958_AES3_CON_FS;
        0
    }
}

unsafe fn snd_vortex_spdif_get(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> i32 {
    unsafe {
        let vortex: *mut vortex_t = snd_kcontrol_chip(kcontrol);
        (*ucontrol).value.iec958.status[0] = 0x00;
        (*ucontrol).value.iec958.status[1] =
            IEC958_AES1_CON_ORIGINAL | IEC958_AES1_CON_DIGDIGCONV_ID;
        (*ucontrol).value.iec958.status[2] = 0x00;
        match (*vortex).spdif_sr {
            32000 => (*ucontrol).value.iec958.status[3] = IEC958_AES3_CON_FS_32000,
            44100 => (*ucontrol).value.iec958.status[3] = IEC958_AES3_CON_FS_44100,
            48000 => (*ucontrol).value.iec958.status[3] = IEC958_AES3_CON_FS_48000,
            _ => {}
        }
        0
    }
}

unsafe fn snd_vortex_spdif_put(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> i32 {
    unsafe {
        let vortex: *mut vortex_t = snd_kcontrol_chip(kcontrol);
        let mut spdif_sr: i32 = 48000;
        match (*ucontrol).value.iec958.status[3] & IEC958_AES3_CON_FS {
            IEC958_AES3_CON_FS_32000 => spdif_sr = 32000,
            IEC958_AES3_CON_FS_44100 => spdif_sr = 44100,
            IEC958_AES3_CON_FS_48000 => spdif_sr = 48000,
            _ => {}
        }
        if spdif_sr == (*vortex).spdif_sr {
            return 0;
        }
        (*vortex).spdif_sr = spdif_sr;
        vortex_spdif_init(vortex, (*vortex).spdif_sr, 1);
        1
    }
}

/* spdif controls */
static snd_vortex_mixer_spdif: [snd_kcontrol_new; 2] = [
    snd_kcontrol_new {
        iface: SNDRV_CTL_ELEM_IFACE_PCM,
        name: SNDRV_CTL_NAME_IEC958_DEFAULT,
        info: Some(snd_vortex_spdif_info),
        get: Some(snd_vortex_spdif_get),
        put: Some(snd_vortex_spdif_put),
        ..snd_kcontrol_new::default()
    },
    snd_kcontrol_new {
        access: SNDRV_CTL_ELEM_ACCESS_READ,
        iface: SNDRV_CTL_ELEM_IFACE_PCM,
        name: SNDRV_CTL_NAME_IEC958_CON_MASK,
        info: Some(snd_vortex_spdif_info),
        get: Some(snd_vortex_spdif_mask_get),
        ..snd_kcontrol_new::default()
    },
];

/* subdevice PCM Volume control */

unsafe fn snd_vortex_pcm_vol_info(
    kcontrol: *mut snd_kcontrol,
    uinfo: *mut snd_ctl_elem_info,
) -> i32 {
    unsafe {
        let vortex: *mut vortex_t = snd_kcontrol_chip(kcontrol);
        (*uinfo).type_ = SNDRV_CTL_ELEM_TYPE_INTEGER;
        (*uinfo).count = if VORTEX_IS_QUAD(vortex) != 0 { 4 } else { 2 };
        (*uinfo).value.integer.min = -128;
        (*uinfo).value.integer.max = 32;
        0
    }
}

unsafe fn snd_vortex_pcm_vol_get(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> i32 {
    unsafe {
        let mut i: i32;
        let vortex: *mut vortex_t = snd_kcontrol_chip(kcontrol);
        let subdev: i32 = (*kcontrol).id.subdevice;
        let p: *mut pcm_vol = &mut (*vortex).pcm_vol[subdev as usize];
        let max_chn: i32 = if VORTEX_IS_QUAD(vortex) != 0 { 4 } else { 2 };
        i = 0;
        while i < max_chn {
            (*ucontrol).value.integer.value[i as usize] = (*p).vol[i as usize] as _;
            i += 1;
        }
        0
    }
}

unsafe fn snd_vortex_pcm_vol_put(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> i32 {
    unsafe {
        let mut i: i32;
        let mut changed: i32 = 0;
        let mixin: i32;
        let vol: u8;
        let vortex: *mut vortex_t = snd_kcontrol_chip(kcontrol);
        let subdev: i32 = (*kcontrol).id.subdevice;
        let p: *mut pcm_vol = &mut (*vortex).pcm_vol[subdev as usize];
        let max_chn: i32 = if VORTEX_IS_QUAD(vortex) != 0 { 4 } else { 2 };
        i = 0;
        while i < max_chn {
            if (*p).vol[i as usize] != (*ucontrol).value.integer.value[i as usize] as _ {
                (*p).vol[i as usize] = (*ucontrol).value.integer.value[i as usize] as _;
                if (*p).active != 0 {
                    mixin = match (*vortex).dma_adb[(*p).dma as usize].nr_ch {
                        1 => (*p).mixin[0],
                        4 => (*p).mixin[i as usize],
                        _ => (*p).mixin[if i < 2 { i as usize } else { (i - 2) as usize }],
                    };
                    vol = (*p).vol[i as usize] as u8;
                    vortex_mix_setinputvolumebyte(
                        vortex,
                        (*vortex).mixplayb[i as usize],
                        mixin,
                        vol,
                    );
                }
                changed = 1;
            }
            i += 1;
        }
        changed
    }
}

static vortex_pcm_vol_db_scale: [u32; 2] = DECLARE_TLV_DB_MINMAX!(-9600, 2400);

static snd_vortex_pcm_vol: snd_kcontrol_new = snd_kcontrol_new {
    iface: SNDRV_CTL_ELEM_IFACE_PCM,
    name: c"PCM Playback Volume".as_ptr(),
    access: SNDRV_CTL_ELEM_ACCESS_READWRITE |
        SNDRV_CTL_ELEM_ACCESS_TLV_READ |
        SNDRV_CTL_ELEM_ACCESS_INACTIVE,
    info: Some(snd_vortex_pcm_vol_info),
    get: Some(snd_vortex_pcm_vol_get),
    put: Some(snd_vortex_pcm_vol_put),
    tlv: snd_kcontrol_new_tlv {
        p: vortex_pcm_vol_db_scale.as_ptr(),
    },
    ..snd_kcontrol_new::default()
};

/* create a pcm device */
unsafe fn snd_vortex_new_pcm(chip: *mut vortex_t, idx: i32, nr: i32) -> i32 {
    unsafe {
        let mut pcm: *mut snd_pcm = core::ptr::null_mut();
        let mut kctl: *mut snd_kcontrol;
        let mut i: i32;
        let mut err: i32;
        let nr_capt: i32;

        if chip.is_null() || idx < 0 || idx >= VORTEX_PCM_LAST {
            return -ENODEV;
        }

        /* idx indicates which kind of PCM device. ADB, SPDIF, I2S and A3D share the
         * same dma engine. WT uses it own separate dma engine which can't capture. */
        if idx == VORTEX_PCM_ADB {
            nr_capt = nr;
        } else {
            nr_capt = 0;
        }
        err = snd_pcm_new(
            (*chip).card,
            vortex_pcm_prettyname[idx as usize],
            idx,
            nr,
            nr_capt,
            &mut pcm,
        );
        if err < 0 {
            return err;
        }
        snprintf(
            (*pcm).name.as_mut_ptr(),
            core::mem::size_of_val(&(*pcm).name),
            c"%s %s".as_ptr(),
            CARD_NAME_SHORT,
            vortex_pcm_name[idx as usize],
        );
        (*chip).pcm[idx as usize] = pcm;
        // This is an evil hack, but it saves a lot of duplicated code.
        *VORTEX_PCM_TYPE(pcm) = idx as u8;
        (*pcm).private_data = chip as *mut _;
        /* set operators */
        snd_pcm_set_ops(pcm, SNDRV_PCM_STREAM_PLAYBACK, &snd_vortex_playback_ops);
        if idx == VORTEX_PCM_ADB {
            snd_pcm_set_ops(pcm, SNDRV_PCM_STREAM_CAPTURE, &snd_vortex_playback_ops);
        }

        /* pre-allocation of Scatter-Gather buffers */

        snd_pcm_set_managed_buffer_all(
            pcm,
            SNDRV_DMA_TYPE_DEV_SG,
            &mut (*(*chip).pci_dev).dev,
            0x10000,
            0x10000,
        );

        match *VORTEX_PCM_TYPE(pcm) as i32 {
            VORTEX_PCM_ADB => {
                err = snd_pcm_add_chmap_ctls(
                    pcm,
                    SNDRV_PCM_STREAM_PLAYBACK,
                    snd_pcm_std_chmaps,
                    if VORTEX_IS_QUAD(chip) != 0 { 4 } else { 2 },
                    0,
                    core::ptr::null_mut(),
                );
                if err < 0 {
                    return err;
                }
                err = snd_pcm_add_chmap_ctls(
                    pcm,
                    SNDRV_PCM_STREAM_CAPTURE,
                    snd_pcm_std_chmaps,
                    2,
                    0,
                    core::ptr::null_mut(),
                );
                if err < 0 {
                    return err;
                }
            }
            #[cfg(CHIP_AU8830)]
            VORTEX_PCM_A3D => {
                err = snd_pcm_add_chmap_ctls(
                    pcm,
                    SNDRV_PCM_STREAM_PLAYBACK,
                    snd_pcm_std_chmaps,
                    1,
                    0,
                    core::ptr::null_mut(),
                );
                if err < 0 {
                    return err;
                }
            }
            _ => {}
        }

        if *VORTEX_PCM_TYPE(pcm) as i32 == VORTEX_PCM_SPDIF {
            i = 0;
            while i < snd_vortex_mixer_spdif.len() as i32 {
                kctl = snd_ctl_new1(&snd_vortex_mixer_spdif[i as usize], chip as *mut _);
                if kctl.is_null() {
                    return -ENOMEM;
                }
                err = snd_ctl_add((*chip).card, kctl);
                if err < 0 {
                    return err;
                }
                i += 1;
            }
        }
        if *VORTEX_PCM_TYPE(pcm) as i32 == VORTEX_PCM_ADB {
            i = 0;
            while i < NR_PCM {
                (*chip).pcm_vol[i as usize].active = 0;
                (*chip).pcm_vol[i as usize].dma = -1;
                kctl = snd_ctl_new1(&snd_vortex_pcm_vol, chip as *mut _);
                if kctl.is_null() {
                    return -ENOMEM;
                }
                (*chip).pcm_vol[i as usize].kctl = kctl;
                (*kctl).id.device = 0;
                (*kctl).id.subdevice = i;
                err = snd_ctl_add((*chip).card, kctl);
                if err < 0 {
                    return err;
                }
                i += 1;
            }
        }
        0
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
