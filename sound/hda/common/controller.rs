// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *
 *  Implementation of primary alsa driver code base for Intel HD Audio.
 *
 *  Copyright(c) 2004 Intel Corporation
 *
 *  Copyright (c) 2004 Takashi Iwai <tiwai@suse.de>
 *                     PeiSen Hou <pshou@realtek.com.tw>
 */

// Dependencies from:
// linux/clocksource.h, linux/delay.h, linux/interrupt.h, linux/kernel.h,
// linux/module.h, linux/pm_runtime.h, linux/slab.h, asm/tsc.h,
// sound/core.h, sound/initval.h, sound/pcm_params.h,
// hda_controller.h, hda_local.h, controller_trace.h.

/* DSP lock helpers */
// CONFIG_SND_HDA_DSP_LOADER:
// guard_dsp_lock(dev) maps to guard(snd_hdac_dsp_lock)(azx_stream(dev)).
// Otherwise it is a no-op.
unsafe fn guard_dsp_lock(_dev: *mut azx_dev) {}

unsafe fn dsp_is_locked(dev: *mut azx_dev) -> bool {
    snd_hdac_stream_is_locked(azx_stream(dev))
}

/* assign a stream for the PCM */
unsafe fn azx_assign_device(chip: *mut azx, substream: *mut snd_pcm_substream) -> *mut azx_dev {
    let s: *mut hdac_stream;

    s = snd_hdac_stream_assign(azx_bus(chip), substream);
    if s.is_null() {
        return core::ptr::null_mut();
    }
    stream_to_azx_dev(s)
}

/* release the assigned stream */
unsafe fn azx_release_device(azx_dev: *mut azx_dev) {
    snd_hdac_stream_release(azx_stream(azx_dev));
}

unsafe fn to_hda_pcm_stream(substream: *mut snd_pcm_substream) -> *mut hda_pcm_stream {
    let apcm: *mut azx_pcm = snd_pcm_substream_chip(substream);
    &mut (*(*apcm).info).stream[(*substream).stream as usize]
}

unsafe fn azx_adjust_codec_delay(substream: *mut snd_pcm_substream, nsec: u64) -> u64 {
    let apcm: *mut azx_pcm = snd_pcm_substream_chip(substream);
    let hinfo: *mut hda_pcm_stream = to_hda_pcm_stream(substream);
    let codec_frames: u64;
    let codec_nsecs: u64;

    if (*hinfo).ops.get_delay.is_none() {
        return nsec;
    }

    codec_frames = ((*hinfo).ops.get_delay.unwrap())(hinfo, (*apcm).codec, substream) as u64;
    codec_nsecs = div_u64(codec_frames.wrapping_mul(1000000000u64), (*(*substream).runtime).rate as u64);

    if (*substream).stream == SNDRV_PCM_STREAM_CAPTURE {
        return nsec.wrapping_add(codec_nsecs);
    }

    if nsec > codec_nsecs { nsec - codec_nsecs } else { 0 }
}

/*
 * PCM ops
 */

unsafe fn azx_pcm_close(substream: *mut snd_pcm_substream) -> i32 {
    let apcm: *mut azx_pcm = snd_pcm_substream_chip(substream);
    let hinfo: *mut hda_pcm_stream = to_hda_pcm_stream(substream);
    let chip: *mut azx = (*apcm).chip;
    let azx_dev: *mut azx_dev = get_azx_dev(substream);

    trace_azx_pcm_close(chip, azx_dev);
    mutex_lock(&mut (*chip).open_mutex);
    if (*(*chip).ops).pcm_close.is_some() {
        ((*(*chip).ops).pcm_close.unwrap())(chip, azx_dev);
    }
    azx_release_device(azx_dev);
    if (*hinfo).ops.close.is_some() {
        ((*hinfo).ops.close.unwrap())(hinfo, (*apcm).codec, substream);
    }
    snd_hda_power_down((*apcm).codec);
    mutex_unlock(&mut (*chip).open_mutex);
    snd_hda_codec_pcm_put((*apcm).info);
    0
}

unsafe fn azx_pcm_hw_params(
    substream: *mut snd_pcm_substream,
    hw_params: *mut snd_pcm_hw_params,
) -> i32 {
    let apcm: *mut azx_pcm = snd_pcm_substream_chip(substream);
    let chip: *mut azx = (*apcm).chip;
    let azx_dev: *mut azx_dev = get_azx_dev(substream);
    let hdas: *mut hdac_stream = azx_stream(azx_dev);

    trace_azx_pcm_hw_params(chip, azx_dev);
    guard_dsp_lock(azx_dev);
    if dsp_is_locked(azx_dev) {
        return -EBUSY;
    }

    /* Set up BDLEs here, return -ENOMEM if too many BDLEs are required */
    (*hdas).bufsize = params_buffer_bytes(hw_params);
    (*hdas).period_bytes = params_period_bytes(hw_params);
    (*hdas).format_val = 0;
    (*hdas).no_period_wakeup =
        ((*hw_params).info & SNDRV_PCM_INFO_NO_PERIOD_WAKEUP) != 0 &&
        ((*hw_params).flags & SNDRV_PCM_HW_PARAMS_NO_PERIOD_WAKEUP) != 0;
    if snd_hdac_stream_setup_periods(hdas) < 0 {
        return -ENOMEM;
    }

    0
}

unsafe fn azx_pcm_hw_free(substream: *mut snd_pcm_substream) -> i32 {
    let apcm: *mut azx_pcm = snd_pcm_substream_chip(substream);
    let azx_dev: *mut azx_dev = get_azx_dev(substream);
    let hinfo: *mut hda_pcm_stream = to_hda_pcm_stream(substream);

    /* reset BDL address */
    guard_dsp_lock(azx_dev);
    if !dsp_is_locked(azx_dev) {
        snd_hdac_stream_cleanup(azx_stream(azx_dev));
    }

    snd_hda_codec_cleanup((*apcm).codec, hinfo, substream);

    (*azx_stream(azx_dev)).prepared = 0;
    0
}

unsafe fn azx_pcm_prepare(substream: *mut snd_pcm_substream) -> i32 {
    let apcm: *mut azx_pcm = snd_pcm_substream_chip(substream);
    let chip: *mut azx = (*apcm).chip;
    let azx_dev: *mut azx_dev = get_azx_dev(substream);
    let hinfo: *mut hda_pcm_stream = to_hda_pcm_stream(substream);
    let runtime: *mut snd_pcm_runtime = (*substream).runtime;
    let mut format_val: u32;
    let mut stream_tag: u32;
    let bits: u32;
    let mut err: i32;
    let spdif: *mut hda_spdif_out = snd_hda_spdif_out_of_nid((*apcm).codec, (*hinfo).nid);
    let ctls: u16 = if !spdif.is_null() { (*spdif).ctls } else { 0 };

    trace_azx_pcm_prepare(chip, azx_dev);
    guard_dsp_lock(azx_dev);
    if dsp_is_locked(azx_dev) {
        return -EBUSY;
    }

    snd_hdac_stream_reset(azx_stream(azx_dev));
    bits = snd_hdac_stream_format_bits((*runtime).format, SNDRV_PCM_SUBFORMAT_STD, (*hinfo).maxbps);

    format_val = snd_hdac_spdif_stream_format((*runtime).channels, bits, (*runtime).rate, ctls);
    if format_val == 0 {
        dev_err((*(*chip).card).dev, c"invalid format_val, rate=%d, ch=%d, format=%d\n".as_ptr(),
                (*runtime).rate, (*runtime).channels, (*runtime).format);
        return -EINVAL;
    }

    err = snd_hdac_stream_set_params(azx_stream(azx_dev), format_val);
    if err < 0 {
        return err;
    }

    snd_hdac_stream_setup(azx_stream(azx_dev), false);

    stream_tag = (*azx_dev).core.stream_tag;
    /* CA-IBG chips need the playback stream starting from 1 */
    if ((*chip).driver_caps & AZX_DCAPS_CTX_WORKAROUND) != 0 &&
       stream_tag > (*chip).capture_streams {
        stream_tag -= (*chip).capture_streams;
    }
    err = snd_hda_codec_prepare((*apcm).codec, hinfo, stream_tag,
                                (*azx_dev).core.format_val, substream);
    if err < 0 {
        return err;
    }

    (*azx_stream(azx_dev)).prepared = 1;
    0
}

unsafe fn azx_pcm_trigger(substream: *mut snd_pcm_substream, cmd: i32) -> i32 {
    let apcm: *mut azx_pcm = snd_pcm_substream_chip(substream);
    let chip: *mut azx = (*apcm).chip;
    let bus: *mut hdac_bus = azx_bus(chip);
    let mut azx_dev: *mut azx_dev;
    let mut s: *mut snd_pcm_substream;
    let hstr: *mut hdac_stream;
    let start: bool;
    let mut sbits: i32 = 0;
    let sync_reg: i32;

    azx_dev = get_azx_dev(substream);
    trace_azx_pcm_trigger(chip, azx_dev, cmd);

    hstr = azx_stream(azx_dev);
    if ((*chip).driver_caps & AZX_DCAPS_OLD_SSYNC) != 0 {
        sync_reg = AZX_REG_OLD_SSYNC;
    } else {
        sync_reg = AZX_REG_SSYNC;
    }

    if dsp_is_locked(azx_dev) || (*hstr).prepared == 0 {
        return -EPIPE;
    }

    match cmd {
        SNDRV_PCM_TRIGGER_START |
        SNDRV_PCM_TRIGGER_PAUSE_RELEASE |
        SNDRV_PCM_TRIGGER_RESUME => start = true,
        SNDRV_PCM_TRIGGER_PAUSE_PUSH |
        SNDRV_PCM_TRIGGER_SUSPEND |
        SNDRV_PCM_TRIGGER_STOP => start = false,
        _ => return -EINVAL,
    }

    snd_pcm_group_for_each_entry!(s, substream, {
        if (*(*s).pcm).card != (*(*substream).pcm).card {
            continue;
        }
        azx_dev = get_azx_dev(s);
        sbits |= 1 << (*azx_dev).core.index;
        snd_pcm_trigger_done(s, substream);
    });

    spin_lock(&mut (*bus).reg_lock);
    /* first, set SYNC bits of corresponding streams */
    snd_hdac_stream_sync_trigger(hstr, true, sbits, sync_reg);

    snd_pcm_group_for_each_entry!(s, substream, {
        if (*(*s).pcm).card != (*(*substream).pcm).card {
            continue;
        }
        azx_dev = get_azx_dev(s);
        if start {
            (*azx_dev).insufficient = 1;
            snd_hdac_stream_start(azx_stream(azx_dev));
        } else {
            snd_hdac_stream_stop(azx_stream(azx_dev));
        }
    });
    spin_unlock(&mut (*bus).reg_lock);

    snd_hdac_stream_sync(hstr, start, sbits);

    spin_lock(&mut (*bus).reg_lock);
    /* reset SYNC bits */
    snd_hdac_stream_sync_trigger(hstr, false, sbits, sync_reg);
    snd_hdac_stream_timecounter_init(hstr, sbits, start);
    spin_unlock(&mut (*bus).reg_lock);
    0
}

pub unsafe extern "C" fn azx_get_pos_lpib(chip: *mut azx, azx_dev: *mut azx_dev) -> u32 {
    let _ = chip;
    snd_hdac_stream_get_pos_lpib(azx_stream(azx_dev))
}

pub unsafe extern "C" fn azx_get_pos_posbuf(chip: *mut azx, azx_dev: *mut azx_dev) -> u32 {
    let _ = chip;
    snd_hdac_stream_get_pos_posbuf(azx_stream(azx_dev))
}

pub unsafe extern "C" fn azx_get_position(chip: *mut azx, azx_dev: *mut azx_dev) -> u32 {
    let substream: *mut snd_pcm_substream = (*azx_dev).core.substream;
    let mut pos: u32;
    let stream: i32 = (*substream).stream;
    let mut delay: i32 = 0;

    if (*chip).get_position[stream as usize].is_some() {
        pos = ((*chip).get_position[stream as usize].unwrap())(chip, azx_dev);
    } else {
        /* use the position buffer as default */
        pos = azx_get_pos_posbuf(chip, azx_dev);
    }

    if pos >= (*azx_dev).core.bufsize {
        pos = 0;
    }

    if !(*substream).runtime.is_null() {
        let apcm: *mut azx_pcm = snd_pcm_substream_chip(substream);
        let hinfo: *mut hda_pcm_stream = to_hda_pcm_stream(substream);

        if (*chip).get_delay[stream as usize].is_some() {
            delay += ((*chip).get_delay[stream as usize].unwrap())(chip, azx_dev, pos);
        }
        if (*hinfo).ops.get_delay.is_some() {
            delay += ((*hinfo).ops.get_delay.unwrap())(hinfo, (*apcm).codec, substream);
        }
        (*(*substream).runtime).delay = delay;
    }

    trace_azx_get_position(chip, azx_dev, pos, delay);
    pos
}

unsafe fn azx_pcm_pointer(substream: *mut snd_pcm_substream) -> snd_pcm_uframes_t {
    let apcm: *mut azx_pcm = snd_pcm_substream_chip(substream);
    let chip: *mut azx = (*apcm).chip;
    let azx_dev: *mut azx_dev = get_azx_dev(substream);
    bytes_to_frames((*substream).runtime, azx_get_position(chip, azx_dev))
}

/*
 * azx_scale64: Scale base by mult/div while not overflowing sanely
 *
 * Derived from scale64_check_overflow in kernel/time/timekeeping.c
 *
 * The tmestamps for a 48Khz stream can overflow after (2^64/10^9)/48K which
 * is about 384307 ie ~4.5 days.
 *
 * This scales the calculation so that overflow will happen but after 2^64 /
 * 48000 secs, which is pretty large!
 *
 * In caln below:
 *	base may overflow, but since there isn't any additional division
 *	performed on base it's OK
 *	rem can't overflow because both are 32-bit values
 */

// CONFIG_X86
unsafe fn azx_scale64(mut base: u64, num: u32, den: u32) -> u64 {
    let mut rem: u64;

    rem = do_div(&mut base, den);

    base = base.wrapping_mul(num as u64);
    rem = rem.wrapping_mul(num as u64);

    do_div(&mut rem, den);

    base.wrapping_add(rem)
}

// CONFIG_X86 implementation; without CONFIG_X86 this function returns -ENXIO.
unsafe fn azx_get_sync_time(
    device: *mut ktime_t,
    system: *mut system_counterval_t,
    ctx: *mut core::ffi::c_void,
) -> i32 {
    let substream: *mut snd_pcm_substream = ctx as *mut snd_pcm_substream;
    let azx_dev: *mut azx_dev = get_azx_dev(substream);
    let apcm: *mut azx_pcm = snd_pcm_substream_chip(substream);
    let chip: *mut azx = (*apcm).chip;
    let runtime: *mut snd_pcm_runtime;
    let mut ll_counter: u64;
    let ll_counter_l: u64;
    let ll_counter_h: u64;
    let mut tsc_counter: u64;
    let tsc_counter_l: u64;
    let tsc_counter_h: u64;
    let wallclk_ctr: u32;
    let wallclk_cycles: u32;
    let direction: bool;
    let mut dma_select: u32;
    let mut timeout: u32;
    let mut retry_count: u32 = 0;

    runtime = (*substream).runtime;

    if (*substream).stream == SNDRV_PCM_STREAM_PLAYBACK {
        direction = true;
    } else {
        direction = false;
    }

    /* 0th stream tag is not used, so DMA ch 0 is for 1st stream tag */
    loop {
        timeout = 100;
        dma_select = ((direction as u32) << GTSCC_CDMAS_DMA_DIR_SHIFT) |
                     ((*azx_dev).core.stream_tag - 1);
        snd_hdac_chip_writel(azx_bus(chip), GTSCC, dma_select);

        /* Enable the capture */
        snd_hdac_chip_updatel(azx_bus(chip), GTSCC, 0, GTSCC_TSCCI_MASK);

        while timeout != 0 {
            if (snd_hdac_chip_readl(azx_bus(chip), GTSCC) & GTSCC_TSCCD_MASK) != 0 {
                break;
            }

            timeout -= 1;
        }

        if timeout == 0 {
            dev_err((*(*chip).card).dev, c"GTSCC capture Timedout!\n".as_ptr());
            return -EIO;
        }

        /* Read wall clock counter */
        wallclk_ctr = snd_hdac_chip_readl(azx_bus(chip), WALFCC);

        /* Read TSC counter */
        tsc_counter_l = snd_hdac_chip_readl(azx_bus(chip), TSCCL) as u64;
        tsc_counter_h = snd_hdac_chip_readl(azx_bus(chip), TSCCU) as u64;

        /* Read Link counter */
        ll_counter_l = snd_hdac_chip_readl(azx_bus(chip), LLPCL) as u64;
        ll_counter_h = snd_hdac_chip_readl(azx_bus(chip), LLPCU) as u64;

        /* Ack: registers read done */
        snd_hdac_chip_writel(azx_bus(chip), GTSCC, GTSCC_TSCCD_SHIFT);

        tsc_counter = (tsc_counter_h << TSCCU_CCU_SHIFT) | tsc_counter_l;

        ll_counter = (ll_counter_h << LLPC_CCU_SHIFT) | ll_counter_l;
        wallclk_cycles = wallclk_ctr & WALFCC_CIF_MASK;

        /*
         * An error occurs near frame "rollover". The clocks in
         * frame value indicates whether this error may have
         * occurred. Here we use the value of 10 i.e.,
         * HDA_MAX_CYCLE_OFFSET
         */
        if wallclk_cycles < HDA_MAX_CYCLE_VALUE - HDA_MAX_CYCLE_OFFSET &&
           wallclk_cycles > HDA_MAX_CYCLE_OFFSET {
            break;
        }

        /*
         * Sleep before we read again, else we may again get
         * value near to MAX_CYCLE. Try to sleep for different
         * amount of time so we dont hit the same number again
         */
        udelay(retry_count);
        retry_count += 1;

        if retry_count == HDA_MAX_CYCLE_READ_RETRY {
            break;
        }
    }

    if retry_count == HDA_MAX_CYCLE_READ_RETRY {
        dev_err_ratelimited((*(*chip).card).dev, c"Error in WALFCC cycle count\n".as_ptr());
        return -EIO;
    }

    *device = ns_to_ktime(azx_scale64(ll_counter, NSEC_PER_SEC, (*runtime).rate));
    *device = ktime_add_ns(*device, ((wallclk_cycles as u64 * NSEC_PER_SEC as u64) /
               (((HDA_MAX_CYCLE_VALUE + 1) as u64) * (*runtime).rate as u64)) as u64);

    (*system).cycles = tsc_counter;
    (*system).cs_id = CSID_X86_ART;

    0
}

unsafe fn azx_get_crosststamp(
    substream: *mut snd_pcm_substream,
    xtstamp: *mut system_device_crosststamp,
) -> i32 {
    get_device_system_crosststamp(Some(azx_get_sync_time), substream as *mut core::ffi::c_void,
                                  core::ptr::null_mut(), xtstamp)
}

unsafe fn is_link_time_supported(
    runtime: *mut snd_pcm_runtime,
    ts: *mut snd_pcm_audio_tstamp_config,
) -> bool {
    if ((*runtime).hw.info & SNDRV_PCM_INFO_HAS_LINK_SYNCHRONIZED_ATIME) != 0 {
        if (*ts).type_requested == SNDRV_PCM_AUDIO_TSTAMP_TYPE_LINK_SYNCHRONIZED {
            return true;
        }
    }

    false
}

unsafe fn azx_get_time_info(
    substream: *mut snd_pcm_substream,
    system_ts: *mut timespec64,
    audio_ts: *mut timespec64,
    audio_tstamp_config: *mut snd_pcm_audio_tstamp_config,
    audio_tstamp_report: *mut snd_pcm_audio_tstamp_report,
) -> i32 {
    let mut xtstamp: system_device_crosststamp = system_device_crosststamp { clock_id: CLOCK_REALTIME, ..core::mem::zeroed() };
    let azx_dev: *mut azx_dev = get_azx_dev(substream);
    let runtime: *mut snd_pcm_runtime = (*substream).runtime;
    let ret: i32;
    let mut nsec: u64;

    if ((*(*substream).runtime).hw.info & SNDRV_PCM_INFO_HAS_LINK_ATIME) != 0 &&
       (*audio_tstamp_config).type_requested == SNDRV_PCM_AUDIO_TSTAMP_TYPE_LINK {
        snd_pcm_gettime((*substream).runtime, system_ts);

        nsec = timecounter_read(&mut (*azx_dev).core.tc);
        if (*audio_tstamp_config).report_delay != 0 {
            nsec = azx_adjust_codec_delay(substream, nsec);
        }

        *audio_ts = ns_to_timespec64(nsec);

        (*audio_tstamp_report).actual_type = SNDRV_PCM_AUDIO_TSTAMP_TYPE_LINK;
        (*audio_tstamp_report).accuracy_report = 1; /* rest of structure is valid */
        (*audio_tstamp_report).accuracy = 42; /* 24 MHz WallClock == 42ns resolution */
    } else if is_link_time_supported(runtime, audio_tstamp_config) {
        ret = azx_get_crosststamp(substream, &mut xtstamp);
        if ret != 0 {
            return ret;
        }

        match (*runtime).tstamp_type {
            SNDRV_PCM_TSTAMP_TYPE_MONOTONIC => return -EINVAL,
            SNDRV_PCM_TSTAMP_TYPE_MONOTONIC_RAW => {
                *system_ts = ktime_to_timespec64(xtstamp.sys_monoraw);
            }
            _ => {
                *system_ts = ktime_to_timespec64(xtstamp.sys_systime);
            }
        }

        *audio_ts = ktime_to_timespec64(xtstamp.device);

        (*audio_tstamp_report).actual_type =
            SNDRV_PCM_AUDIO_TSTAMP_TYPE_LINK_SYNCHRONIZED;
        (*audio_tstamp_report).accuracy_report = 1;
        /* 24 MHz WallClock == 42ns resolution */
        (*audio_tstamp_report).accuracy = 42;
    } else {
        (*audio_tstamp_report).actual_type = SNDRV_PCM_AUDIO_TSTAMP_TYPE_DEFAULT;
    }

    0
}

static azx_pcm_hw: snd_pcm_hardware = snd_pcm_hardware {
    info: SNDRV_PCM_INFO_MMAP |
          SNDRV_PCM_INFO_INTERLEAVED |
          SNDRV_PCM_INFO_BLOCK_TRANSFER |
          SNDRV_PCM_INFO_MMAP_VALID |
          /* No full-resume yet implemented */
          /* SNDRV_PCM_INFO_RESUME |*/
          SNDRV_PCM_INFO_PAUSE |
          SNDRV_PCM_INFO_SYNC_START |
          SNDRV_PCM_INFO_HAS_WALL_CLOCK | /* legacy */
          SNDRV_PCM_INFO_HAS_LINK_ATIME |
          SNDRV_PCM_INFO_NO_PERIOD_WAKEUP,
    formats: SNDRV_PCM_FMTBIT_S16_LE,
    rates: SNDRV_PCM_RATE_48000,
    rate_min: 48000,
    rate_max: 48000,
    channels_min: 2,
    channels_max: 2,
    buffer_bytes_max: AZX_MAX_BUF_SIZE,
    period_bytes_min: 128,
    period_bytes_max: AZX_MAX_BUF_SIZE / 2,
    periods_min: 2,
    periods_max: AZX_MAX_FRAG,
    fifo_size: 0,
};

unsafe fn azx_pcm_open(substream: *mut snd_pcm_substream) -> i32 {
    let apcm: *mut azx_pcm = snd_pcm_substream_chip(substream);
    let hinfo: *mut hda_pcm_stream = to_hda_pcm_stream(substream);
    let chip: *mut azx = (*apcm).chip;
    let mut azx_dev: *mut azx_dev;
    let runtime: *mut snd_pcm_runtime = (*substream).runtime;
    let mut err: i32;
    let buff_step: i32;

    snd_hda_codec_pcm_get((*apcm).info);
    mutex_lock(&mut (*chip).open_mutex);
    azx_dev = azx_assign_device(chip, substream);
    trace_azx_pcm_open(chip, azx_dev);
    if azx_dev.is_null() {
        err = -EBUSY;
        goto_unlock!(chip, apcm, err);
    }
    (*runtime).private_data = azx_dev as *mut core::ffi::c_void;

    (*runtime).hw = azx_pcm_hw;
    if (*chip).gts_present {
        (*runtime).hw.info |= SNDRV_PCM_INFO_HAS_LINK_SYNCHRONIZED_ATIME;
    }
    (*runtime).hw.channels_min = (*hinfo).channels_min;
    (*runtime).hw.channels_max = (*hinfo).channels_max;
    (*runtime).hw.formats = (*hinfo).formats;
    (*runtime).hw.rates = (*hinfo).rates;
    snd_pcm_limit_hw_rates(runtime);
    snd_pcm_hw_constraint_integer(runtime, SNDRV_PCM_HW_PARAM_PERIODS);

    /* avoid wrap-around with wall-clock */
    snd_pcm_hw_constraint_minmax(runtime, SNDRV_PCM_HW_PARAM_BUFFER_TIME, 20, 178000000);

    if (*chip).align_buffer_size {
        /* constrain buffer sizes to be multiple of 128
           bytes. This is more efficient in terms of memory
           access but isn't required by the HDA spec and
           prevents users from specifying exact period/buffer
           sizes. For example for 44.1kHz, a period size set
           to 20ms will be rounded to 19.59ms. */
        buff_step = 128;
    } else {
        /* Don't enforce steps on buffer sizes, still need to
           be multiple of 4 bytes (HDA spec). Tested on Intel
           HDA controllers, may not work on all devices where
           option needs to be disabled */
        buff_step = 4;
    }

    snd_pcm_hw_constraint_step(runtime, 0, SNDRV_PCM_HW_PARAM_BUFFER_BYTES, buff_step);
    snd_pcm_hw_constraint_step(runtime, 0, SNDRV_PCM_HW_PARAM_PERIOD_BYTES, buff_step);
    snd_hda_power_up((*apcm).codec);
    if (*hinfo).ops.open.is_some() {
        err = ((*hinfo).ops.open.unwrap())(hinfo, (*apcm).codec, substream);
    } else {
        err = -ENODEV;
    }
    if err < 0 {
        azx_release_device(azx_dev);
        snd_hda_power_down((*apcm).codec);
        mutex_unlock(&mut (*chip).open_mutex);
        snd_hda_codec_pcm_put((*apcm).info);
        return err;
    }
    snd_pcm_limit_hw_rates(runtime);
    /* sanity check */
    if snd_BUG_ON((*runtime).hw.channels_min == 0) ||
       snd_BUG_ON((*runtime).hw.channels_max == 0) ||
       snd_BUG_ON((*runtime).hw.formats == 0) ||
       snd_BUG_ON((*runtime).hw.rates == 0) {
        azx_release_device(azx_dev);
        if (*hinfo).ops.close.is_some() {
            ((*hinfo).ops.close.unwrap())(hinfo, (*apcm).codec, substream);
        }
        err = -EINVAL;
        snd_hda_power_down((*apcm).codec);
        mutex_unlock(&mut (*chip).open_mutex);
        snd_hda_codec_pcm_put((*apcm).info);
        return err;
    }

    /* disable LINK_ATIME timestamps for capture streams
       until we figure out how to handle digital inputs */
    if (*substream).stream == SNDRV_PCM_STREAM_CAPTURE {
        (*runtime).hw.info &= !SNDRV_PCM_INFO_HAS_WALL_CLOCK; /* legacy */
        (*runtime).hw.info &= !SNDRV_PCM_INFO_HAS_LINK_ATIME;
    }

    snd_pcm_set_sync(substream);
    mutex_unlock(&mut (*chip).open_mutex);
    0
}

static azx_pcm_ops: snd_pcm_ops = snd_pcm_ops {
    open: Some(azx_pcm_open),
    close: Some(azx_pcm_close),
    hw_params: Some(azx_pcm_hw_params),
    hw_free: Some(azx_pcm_hw_free),
    prepare: Some(azx_pcm_prepare),
    trigger: Some(azx_pcm_trigger),
    pointer: Some(azx_pcm_pointer),
    get_time_info: Some(azx_get_time_info),
};

unsafe fn azx_pcm_free(pcm: *mut snd_pcm) {
    let apcm: *mut azx_pcm = (*pcm).private_data as *mut azx_pcm;
    if !apcm.is_null() {
        list_del(&mut (*apcm).list);
        (*(*apcm).info).pcm = core::ptr::null_mut();
        kfree(apcm as *mut core::ffi::c_void);
    }
}

const MAX_PREALLOC_SIZE: u32 = 32 * 1024 * 1024;

pub unsafe extern "C" fn snd_hda_attach_pcm_stream(
    _bus: *mut hda_bus,
    codec: *mut hda_codec,
    cpcm: *mut hda_pcm,
) -> i32 {
    let bus: *mut hdac_bus = &mut (*_bus).core;
    let chip: *mut azx = bus_to_azx(bus);
    let mut pcm: *mut snd_pcm = core::ptr::null_mut();
    let mut apcm: *mut azx_pcm;
    let pcm_dev: i32 = (*cpcm).device;
    let mut size: u32;
    let mut s: i32;
    let mut err: i32;
    let mut type_: i32 = SNDRV_DMA_TYPE_DEV_SG;

    list_for_each_entry!(apcm, &mut (*chip).pcm_list, list, {
        if (*(*apcm).pcm).device == pcm_dev {
            dev_err((*(*chip).card).dev, c"PCM %d already exists\n".as_ptr(), pcm_dev);
            return -EBUSY;
        }
    });
    err = snd_pcm_new((*chip).card, (*cpcm).name, pcm_dev,
                      (*cpcm).stream[SNDRV_PCM_STREAM_PLAYBACK as usize].substreams,
                      (*cpcm).stream[SNDRV_PCM_STREAM_CAPTURE as usize].substreams,
                      &mut pcm);
    if err < 0 {
        return err;
    }
    strscpy((*pcm).name.as_mut_ptr(), (*cpcm).name, core::mem::size_of_val(&(*pcm).name));
    apcm = kzalloc_obj::<azx_pcm>();
    if apcm.is_null() {
        snd_device_free((*chip).card, pcm as *mut core::ffi::c_void);
        return -ENOMEM;
    }
    (*apcm).chip = chip;
    (*apcm).pcm = pcm;
    (*apcm).codec = codec;
    (*apcm).info = cpcm;
    (*pcm).private_data = apcm as *mut core::ffi::c_void;
    (*pcm).private_free = Some(azx_pcm_free);
    if (*cpcm).pcm_type == HDA_PCM_TYPE_MODEM {
        (*pcm).dev_class = SNDRV_PCM_CLASS_MODEM;
    }
    list_add_tail(&mut (*apcm).list, &mut (*chip).pcm_list);
    (*cpcm).pcm = pcm;
    s = 0;
    while s < 2 {
        if (*cpcm).stream[s as usize].substreams != 0 {
            snd_pcm_set_ops(pcm, s, &azx_pcm_ops);
        }
        s += 1;
    }
    /* buffer pre-allocation */
    size = CONFIG_SND_HDA_PREALLOC_SIZE * 1024;
    if size > MAX_PREALLOC_SIZE {
        size = MAX_PREALLOC_SIZE;
    }
    if (*chip).uc_buffer {
        type_ = SNDRV_DMA_TYPE_DEV_WC_SG;
    }
    snd_pcm_set_managed_buffer_all(pcm, type_, (*(*chip).card).dev, size, MAX_PREALLOC_SIZE);
    0
}

unsafe fn azx_command_addr(cmd: u32) -> u32 {
    let mut addr: u32 = cmd >> 28;

    if addr >= AZX_MAX_CODECS {
        snd_BUG();
        addr = 0;
    }

    addr
}

/* receive a response */
unsafe fn azx_rirb_get_response(bus: *mut hdac_bus, addr: u32, res: *mut u32) -> i32 {
    let chip: *mut azx = bus_to_azx(bus);
    let hbus: *mut hda_bus = &mut (*chip).bus;
    let mut err: i32;

    loop {
        err = snd_hdac_bus_get_response(bus, addr, res);
        if err == 0 {
            return 0;
        }

        if (*hbus).no_response_fallback {
            return -EIO;
        }

        if !(*bus).polling_mode {
            dev_warn((*(*chip).card).dev,
                     c"azx_get_response timeout, switching to polling mode: last cmd=0x%08x\n".as_ptr(),
                     (*bus).last_cmd[addr as usize]);
            (*bus).polling_mode = true;
            continue;
        }

        if (*chip).msi {
            dev_warn((*(*chip).card).dev,
                     c"No response from codec, disabling MSI: last cmd=0x%08x\n".as_ptr(),
                     (*bus).last_cmd[addr as usize]);
            if (*(*chip).ops).disable_msi_reset_irq.is_some() &&
               ((*(*chip).ops).disable_msi_reset_irq.unwrap())(chip) < 0 {
                return -EIO;
            }
            continue;
        }

        if (*chip).probing {
            /* If this critical timeout happens during the codec probing
             * phase, this is likely an access to a non-existing codec
             * slot.  Better to return an error and reset the system.
             */
            return -EIO;
        }

        /* no fallback mechanism? */
        if !(*chip).fallback_to_single_cmd {
            return -EIO;
        }

        /* a fatal communication error; need either to reset or to fallback
         * to the single_cmd mode
         */
        if (*hbus).allow_bus_reset && !(*hbus).response_reset && !(*hbus).in_reset {
            (*hbus).response_reset = true;
            dev_err((*(*chip).card).dev,
                    c"No response from codec, resetting bus: last cmd=0x%08x\n".as_ptr(),
                    (*bus).last_cmd[addr as usize]);
            return -EAGAIN; /* give a chance to retry */
        }

        dev_err((*(*chip).card).dev,
                c"azx_get_response timeout, switching to single_cmd mode: last cmd=0x%08x\n".as_ptr(),
                (*bus).last_cmd[addr as usize]);
        (*chip).single_cmd = true;
        (*hbus).response_reset = false;
        snd_hdac_bus_stop_cmd_io(bus);
        return -EIO;
    }
}

/*
 * Use the single immediate command instead of CORB/RIRB for simplicity
 *
 * Note: according to Intel, this is not preferred use.  The command was
 *       intended for the BIOS only, and may get confused with unsolicited
 *       responses.  So, we shouldn't use it for normal operation from the
 *       driver.
 *       I left the codes, however, for debugging/testing purposes.
 */

/* receive a response */
unsafe fn azx_single_wait_for_response(chip: *mut azx, addr: u32) -> i32 {
    let mut timeout: i32 = 50;

    while timeout != 0 {
        timeout -= 1;
        /* check IRV busy bit */
        if (azx_readw(chip, IRS) & AZX_IRS_VALID) != 0 {
            /* reuse rirb.res as the response return value */
            (*azx_bus(chip)).rirb.res[addr as usize] = azx_readl(chip, IR);
            return 0;
        }
        udelay(1);
    }
    if printk_ratelimit() {
        dev_dbg((*(*chip).card).dev, c"get_response timeout: IRS=0x%x\n".as_ptr(),
                azx_readw(chip, IRS));
    }
    (*azx_bus(chip)).rirb.res[addr as usize] = -1i32 as u32;
    -EIO
}

/* send a command */
unsafe fn azx_single_send_cmd(bus: *mut hdac_bus, val: u32) -> i32 {
    let chip: *mut azx = bus_to_azx(bus);
    let addr: u32 = azx_command_addr(val);
    let mut timeout: i32 = 50;

    (*bus).last_cmd[azx_command_addr(val) as usize] = val;
    while timeout != 0 {
        timeout -= 1;
        /* check ICB busy bit */
        if (azx_readw(chip, IRS) & AZX_IRS_BUSY) == 0 {
            /* Clear IRV valid bit */
            azx_writew(chip, IRS, azx_readw(chip, IRS) | AZX_IRS_VALID);
            azx_writel(chip, IC, val);
            azx_writew(chip, IRS, azx_readw(chip, IRS) | AZX_IRS_BUSY);
            return azx_single_wait_for_response(chip, addr);
        }
        udelay(1);
    }
    if printk_ratelimit() {
        dev_dbg((*(*chip).card).dev,
                c"send_cmd timeout: IRS=0x%x, val=0x%x\n".as_ptr(),
                azx_readw(chip, IRS), val);
    }
    -EIO
}

/* receive a response */
unsafe fn azx_single_get_response(bus: *mut hdac_bus, addr: u32, res: *mut u32) -> i32 {
    if !res.is_null() {
        *res = (*bus).rirb.res[addr as usize];
    }
    0
}

/*
 * The below are the main callbacks from hda_codec.
 *
 * They are just the skeleton to call sub-callbacks according to the
 * current setting of chip->single_cmd.
 */

/* send a command */
unsafe fn azx_send_cmd(bus: *mut hdac_bus, val: u32) -> i32 {
    let chip: *mut azx = bus_to_azx(bus);

    if (*chip).disabled {
        return 0;
    }
    if (*chip).single_cmd || (*bus).use_pio_for_commands {
        azx_single_send_cmd(bus, val)
    } else {
        snd_hdac_bus_send_cmd(bus, val)
    }
}

/* get a response */
unsafe fn azx_get_response(bus: *mut hdac_bus, addr: u32, res: *mut u32) -> i32 {
    let chip: *mut azx = bus_to_azx(bus);

    if (*chip).disabled {
        return 0;
    }
    if (*chip).single_cmd || (*bus).use_pio_for_commands {
        azx_single_get_response(bus, addr, res)
    } else {
        azx_rirb_get_response(bus, addr, res)
    }
}

static bus_core_ops: hdac_bus_ops = hdac_bus_ops {
    command: Some(azx_send_cmd),
    get_response: Some(azx_get_response),
};

// CONFIG_SND_HDA_DSP_LOADER
/*
 * DSP loading code (e.g. for CA0132)
 */

/* use the first stream for loading DSP */
unsafe fn azx_get_dsp_loader_dev(chip: *mut azx) -> *mut azx_dev {
    let bus: *mut hdac_bus = azx_bus(chip);
    let mut s: *mut hdac_stream;

    list_for_each_entry!(s, &mut (*bus).stream_list, list, {
        if (*s).index == (*chip).playback_index_offset {
            return stream_to_azx_dev(s);
        }
    });

    core::ptr::null_mut()
}

pub unsafe extern "C" fn snd_hda_codec_load_dsp_prepare(
    codec: *mut hda_codec,
    format: u32,
    byte_size: u32,
    bufp: *mut snd_dma_buffer,
) -> i32 {
    let bus: *mut hdac_bus = &mut (*(*codec).bus).core;
    let chip: *mut azx = bus_to_azx(bus);
    let azx_dev: *mut azx_dev;
    let hstr: *mut hdac_stream;
    let mut saved: bool = false;
    let err: i32;

    azx_dev = azx_get_dsp_loader_dev(chip);
    hstr = azx_stream(azx_dev);
    spin_lock_irq(&mut (*bus).reg_lock);
    if (*hstr).opened {
        (*chip).saved_azx_dev = *azx_dev;
        saved = true;
    }
    spin_unlock_irq(&mut (*bus).reg_lock);

    err = snd_hdac_dsp_prepare(hstr, format, byte_size, bufp);
    if err < 0 {
        spin_lock_irq(&mut (*bus).reg_lock);
        if saved {
            *azx_dev = (*chip).saved_azx_dev;
        }
        spin_unlock_irq(&mut (*bus).reg_lock);
        return err;
    }

    (*hstr).prepared = 0;
    err
}

pub unsafe extern "C" fn snd_hda_codec_load_dsp_trigger(codec: *mut hda_codec, start: bool) {
    let bus: *mut hdac_bus = &mut (*(*codec).bus).core;
    let chip: *mut azx = bus_to_azx(bus);
    let azx_dev: *mut azx_dev = azx_get_dsp_loader_dev(chip);

    snd_hdac_dsp_trigger(azx_stream(azx_dev), start);
}

pub unsafe extern "C" fn snd_hda_codec_load_dsp_cleanup(
    codec: *mut hda_codec,
    dmab: *mut snd_dma_buffer,
) {
    let bus: *mut hdac_bus = &mut (*(*codec).bus).core;
    let chip: *mut azx = bus_to_azx(bus);
    let azx_dev: *mut azx_dev = azx_get_dsp_loader_dev(chip);
    let hstr: *mut hdac_stream = azx_stream(azx_dev);

    if (*dmab).area.is_null() || !(*hstr).locked {
        return;
    }

    snd_hdac_dsp_cleanup(hstr, dmab);
    spin_lock_irq(&mut (*bus).reg_lock);
    if (*hstr).opened {
        *azx_dev = (*chip).saved_azx_dev;
    }
    (*hstr).locked = false;
    spin_unlock_irq(&mut (*bus).reg_lock);
}

/*
 * reset and start the controller registers
 */
pub unsafe extern "C" fn azx_init_chip(chip: *mut azx, full_reset: bool) {
    if snd_hdac_bus_init_chip(azx_bus(chip), full_reset) {
        /* correct RINTCNT for CXT */
        if ((*chip).driver_caps & AZX_DCAPS_CTX_WORKAROUND) != 0 {
            azx_writew(chip, RINTCNT, 0xc0);
        }
    }
}

pub unsafe extern "C" fn azx_stop_all_streams(chip: *mut azx) {
    let bus: *mut hdac_bus = azx_bus(chip);

    snd_hdac_stop_streams(bus);
}

pub unsafe extern "C" fn azx_stop_chip(chip: *mut azx) {
    snd_hdac_bus_stop_chip(azx_bus(chip));
}

/*
 * interrupt handler
 */
unsafe fn stream_update(bus: *mut hdac_bus, s: *mut hdac_stream) {
    let chip: *mut azx = bus_to_azx(bus);
    let azx_dev: *mut azx_dev = stream_to_azx_dev(s);

    /* check whether this IRQ is really acceptable */
    if (*(*chip).ops).position_check.is_none() ||
       ((*(*chip).ops).position_check.unwrap())(chip, azx_dev) {
        spin_unlock(&mut (*bus).reg_lock);
        snd_pcm_period_elapsed((*azx_stream(azx_dev)).substream);
        spin_lock(&mut (*bus).reg_lock);
    }
}

pub unsafe extern "C" fn azx_interrupt(_irq: i32, dev_id: *mut core::ffi::c_void) -> irqreturn_t {
    let chip: *mut azx = dev_id as *mut azx;
    let bus: *mut hdac_bus = azx_bus(chip);
    let mut status: u32;
    let mut active: bool;
    let mut handled: bool = false;
    let mut repeat: i32 = 0; /* count for avoiding endless loop */

    if azx_has_pm_runtime(chip) {
        if !pm_runtime_active((*(*chip).card).dev) {
            return IRQ_NONE;
        }
    }

    spin_lock(&mut (*bus).reg_lock);

    if (*chip).disabled {
        spin_unlock(&mut (*bus).reg_lock);
        return IRQ_NONE;
    }

    loop {
        status = azx_readl(chip, INTSTS);
        if status == 0 || status == 0xffffffff {
            break;
        }

        handled = true;
        active = false;
        if snd_hdac_bus_handle_stream_irq(bus, status, Some(stream_update)) {
            active = true;
        }

        status = azx_readb(chip, RIRBSTS) as u32;
        if (status & RIRB_INT_MASK) != 0 {
            /*
             * Clearing the interrupt status here ensures that no
             * interrupt gets masked after the RIRB wp is read in
             * snd_hdac_bus_update_rirb. This avoids a possible
             * race condition where codec response in RIRB may
             * remain unserviced by IRQ, eventually falling back
             * to polling mode in azx_rirb_get_response.
             */
            azx_writeb(chip, RIRBSTS, RIRB_INT_MASK);
            active = true;
            if (status & RIRB_INT_RESPONSE) != 0 {
                if ((*chip).driver_caps & AZX_DCAPS_CTX_WORKAROUND) != 0 {
                    udelay(80);
                }
                snd_hdac_bus_update_rirb(bus);
            }
        }

        repeat += 1;
        if !(active && repeat < 10) {
            break;
        }
    }

    spin_unlock(&mut (*bus).reg_lock);
    IRQ_RETVAL(handled)
}

/*
 * Codec initerface
 */

/*
 * Probe the given codec address
 */
unsafe fn probe_codec(chip: *mut azx, addr: i32) -> i32 {
    let cmd: u32 = ((addr as u32) << 28) | (AC_NODE_ROOT << 20) |
        (AC_VERB_PARAMETERS << 8) | AC_PAR_VENDOR_ID;
    let bus: *mut hdac_bus = azx_bus(chip);
    let mut err: i32;
    let mut res: u32 = -1i32 as u32;

    mutex_lock(&mut (*bus).cmd_mutex);
    (*chip).probing = true;
    azx_send_cmd(bus, cmd);
    err = azx_get_response(bus, addr as u32, &mut res);
    (*chip).probing = false;
    mutex_unlock(&mut (*bus).cmd_mutex);
    if err < 0 || res == -1i32 as u32 {
        return -EIO;
    }
    dev_dbg((*(*chip).card).dev, c"codec #%d probed OK\n".as_ptr(), addr);
    0
}

pub unsafe extern "C" fn snd_hda_bus_reset(bus: *mut hda_bus) {
    let chip: *mut azx = bus_to_azx(&mut (*bus).core);

    (*bus).in_reset = true;
    azx_stop_chip(chip);
    azx_init_chip(chip, true);
    if (*bus).core.chip_init {
        snd_hda_bus_reset_codecs(bus);
    }
    (*bus).in_reset = false;
}

/* HD-audio bus initialization */
pub unsafe extern "C" fn azx_bus_init(chip: *mut azx, model: *const core::ffi::c_char) -> i32 {
    let bus: *mut hda_bus = &mut (*chip).bus;
    let mut err: i32;

    err = snd_hdac_bus_init(&mut (*bus).core, (*(*chip).card).dev, &bus_core_ops);
    if err < 0 {
        return err;
    }

    (*bus).card = (*chip).card;
    mutex_init(&mut (*bus).prepare_mutex);
    (*bus).pci = (*chip).pci;
    (*bus).modelname = model;
    (*bus).mixer_assigned = -1;
    (*bus).core.snoop = azx_snoop(chip);
    if (*chip).get_position[0] != Some(azx_get_pos_lpib) ||
       (*chip).get_position[1] != Some(azx_get_pos_lpib) {
        (*bus).core.use_posbuf = true;
    }
    (*bus).core.bdl_pos_adj = (*chip).bdl_pos_adj;
    if ((*chip).driver_caps & AZX_DCAPS_CORBRP_SELF_CLEAR) != 0 {
        (*bus).core.corbrp_self_clear = true;
    }

    if ((*chip).driver_caps & AZX_DCAPS_4K_BDLE_BOUNDARY) != 0 {
        (*bus).core.align_bdle_4k = true;
    }

    if ((*chip).driver_caps & AZX_DCAPS_PIO_COMMANDS) != 0 {
        (*bus).core.use_pio_for_commands = true;
    }

    /* enable sync_write flag for stable communication as default */
    (*bus).core.sync_write = true;

    0
}

/* Probe codecs */
pub unsafe extern "C" fn azx_probe_codecs(chip: *mut azx, mut max_slots: u32) -> i32 {
    let bus: *mut hdac_bus = azx_bus(chip);
    let mut c: i32;
    let mut codecs: i32;
    let mut err: i32;

    codecs = 0;
    if max_slots == 0 {
        max_slots = AZX_DEFAULT_CODECS;
    }

    /* First try to probe all given codec slots */
    c = 0;
    while c < max_slots as i32 {
        if (((*bus).codec_mask & (1 << c)) & (*chip).codec_probe_mask) != 0 {
            if probe_codec(chip, c) < 0 {
                /* Some BIOSen give you wrong codec addresses
                 * that don't exist
                 */
                dev_warn((*(*chip).card).dev,
                         c"Codec #%d probe error; disabling it...\n".as_ptr(), c);
                (*bus).codec_mask &= !(1 << c);
                /* no codecs */
                if (*bus).codec_mask == 0 {
                    break;
                }
                /* More badly, accessing to a non-existing
                 * codec often screws up the controller chip,
                 * and disturbs the further communications.
                 * Thus if an error occurs during probing,
                 * better to reset the controller chip to
                 * get back to the sanity state.
                 */
                azx_stop_chip(chip);
                azx_init_chip(chip, true);
            }
        }
        c += 1;
    }

    /* Then create codec instances */
    c = 0;
    while c < max_slots as i32 {
        if (((*bus).codec_mask & (1 << c)) & (*chip).codec_probe_mask) != 0 {
            let mut codec: *mut hda_codec = core::ptr::null_mut();
            err = snd_hda_codec_new(&mut (*chip).bus, (*chip).card, c, &mut codec);
            if err < 0 {
                c += 1;
                continue;
            }
            (*codec).jackpoll_interval = (*chip).jackpoll_interval;
            (*codec).beep_mode = (*chip).beep_mode;
            (*codec).ctl_dev_id = (*chip).ctl_dev_id;
            codecs += 1;
        }
        c += 1;
    }
    if codecs == 0 {
        dev_err((*(*chip).card).dev, c"no codecs initialized\n".as_ptr());
        return -ENXIO;
    }
    0
}

/* configure each codec instance */
pub unsafe extern "C" fn azx_codec_configure(chip: *mut azx) -> i32 {
    let mut codec: *mut hda_codec;
    let mut next: *mut hda_codec;
    let mut success: i32 = 0;

    list_for_each_codec!(codec, &mut (*chip).bus, {
        if snd_hda_codec_configure(codec) == 0 {
            success += 1;
        }
    });

    if success != 0 {
        /* unregister failed codecs if any codec has been probed */
        list_for_each_codec_safe!(codec, next, &mut (*chip).bus, {
            if !(*codec).configured {
                codec_err(codec, c"Unable to configure, disabling\n".as_ptr());
                snd_hdac_device_unregister(&mut (*codec).core);
            }
        });
    }

    if success != 0 { 0 } else { -ENODEV }
}

pub unsafe extern "C" fn azx_add_stream(chip: *mut azx, azx_dev: *mut azx_dev, idx: i32, tag: i32) {
    snd_hdac_stream_init(azx_bus(chip), azx_stream(azx_dev), idx,
                         azx_stream_direction(chip, idx), tag);
}

/* initialize SD streams */
pub unsafe extern "C" fn azx_init_streams(chip: *mut azx) -> i32 {
    let mut i: i32;

    /* initialize each stream (aka device)
     * assign the starting bdl address to each stream (device)
     * and initialize
     */
    i = 0;
    while i < (*chip).num_streams {
        let azx_dev: *mut azx_dev = kzalloc_obj::<azx_dev>();

        if azx_dev.is_null() {
            return -ENOMEM;
        }
        azx_add_stream(chip, azx_dev, i, i + 1);
        i += 1;
    }

    0
}

pub unsafe extern "C" fn azx_free_streams(chip: *mut azx) {
    let bus: *mut hdac_bus = azx_bus(chip);
    let mut s: *mut hdac_stream;

    while !list_empty(&mut (*bus).stream_list) {
        s = list_first_entry!(&mut (*bus).stream_list, hdac_stream, list);
        list_del(&mut (*s).list);
        kfree(stream_to_azx_dev(s) as *mut core::ffi::c_void);
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
