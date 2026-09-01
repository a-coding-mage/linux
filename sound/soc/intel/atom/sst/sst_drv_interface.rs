// SPDX-License-Identifier: GPL-2.0-only
/*
 *  sst_drv_interface.c - Intel SST Driver for audio engine
 *
 *  Copyright (C) 2008-14 Intel Corp
 *  Authors:	Vinod Koul <vinod.koul@intel.com>
 *		Harsha Priya <priya.harsha@intel.com>
 *		Dharageswari R <dharageswari.r@intel.com)
 * ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
 *
 * ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
 */

/* Translated from the isolated C implementation source.  Types, constants,
 * helper functions, and kernel facilities referenced from the original include
 * files are expected to be supplied by the surrounding Rust translation.
 */

const NUM_CODEC: i32 = 2;
const MIN_FRAGMENT: i32 = 2;
const MAX_FRAGMENT: i32 = 4;
const MIN_FRAGMENT_SIZE: i32 = 50 * 1024;
const MAX_FRAGMENT_SIZE: i32 = 1024 * 1024;

#[inline]
const fn SST_GET_BYTES_PER_SAMPLE(pcm_wd_sz: u32) -> u32 {
    (((pcm_wd_sz + 15) >> 4) << 1)
}

/* Original C used CONFIG_PM:
 *   #ifdef CONFIG_PM
 *   #define GET_USAGE_COUNT(dev) (atomic_read(&dev->power.usage_count))
 *   #else
 *   #define GET_USAGE_COUNT(dev) 1
 *   #endif
 */
#[cfg(CONFIG_PM)]
unsafe fn GET_USAGE_COUNT(dev: *mut device) -> i32 {
    atomic_read(&mut (*dev).power.usage_count)
}

#[cfg(not(CONFIG_PM))]
unsafe fn GET_USAGE_COUNT(_dev: *mut device) -> i32 {
    1
}

pub unsafe extern "C" fn free_stream_context(ctx: *mut intel_sst_drv, str_id: u32) -> i32 {
    let mut ret: i32 = 0;
    let stream: *mut stream_info = get_stream_info(ctx, str_id);

    if !stream.is_null() {
        /* str_id is valid, so stream is alloacted */
        ret = sst_free_stream(ctx, str_id);
        if ret != 0 {
            sst_clean_stream(&mut *(*ctx).streams.add(str_id as usize));
        }
        return ret;
    } else {
        dev_err(
            (*ctx).dev,
            c"we tried to free stream context %d which was freed!!!\n".as_ptr(),
            str_id,
        );
    }
    ret
}

/*
 * sst_get_sfreq - this function returns the frequency of the stream
 *
 * @str_param : stream params
 */
pub unsafe extern "C" fn sst_get_sfreq(str_param: *mut snd_sst_params) -> i32 {
    match (*str_param).codec {
        SST_CODEC_TYPE_PCM => (*str_param).sparams.uc.pcm_params.sfreq,
        SST_CODEC_TYPE_AAC => (*str_param).sparams.uc.aac_params.externalsr,
        SST_CODEC_TYPE_MP3 => 0,
        _ => -EINVAL,
    }
}

/*
 * sst_get_num_channel - get number of channels for the stream
 *
 * @str_param : stream params
 */
pub unsafe extern "C" fn sst_get_num_channel(str_param: *mut snd_sst_params) -> i32 {
    match (*str_param).codec {
        SST_CODEC_TYPE_PCM => (*str_param).sparams.uc.pcm_params.num_chan,
        SST_CODEC_TYPE_MP3 => (*str_param).sparams.uc.mp3_params.num_chan,
        SST_CODEC_TYPE_AAC => (*str_param).sparams.uc.aac_params.num_chan,
        _ => -EINVAL,
    }
}

/*
 * sst_get_stream - this function prepares for stream allocation
 *
 * @str_param : stream param
 */
pub unsafe extern "C" fn sst_get_stream(
    ctx: *mut intel_sst_drv,
    str_param: *mut snd_sst_params,
) -> i32 {
    let retval: i32;
    let str_info: *mut stream_info;

    /* stream is not allocated, we are allocating */
    retval = ((*(*ctx).ops).alloc_stream.unwrap())(ctx, str_param);
    if retval <= 0 {
        return -EIO;
    }
    /* store sampling freq */
    str_info = &mut *(*ctx).streams.add(retval as usize);
    (*str_info).sfreq = sst_get_sfreq(str_param);

    retval
}

unsafe extern "C" fn sst_power_control(dev: *mut device, state: bool) -> i32 {
    let ctx: *mut intel_sst_drv = dev_get_drvdata(dev);
    let mut ret: i32 = 0;
    let mut usage_count: i32;

    if state {
        ret = pm_runtime_resume_and_get(dev);
        usage_count = GET_USAGE_COUNT(dev);
        dev_dbg((*ctx).dev, c"Enable: pm usage count: %d\n".as_ptr(), usage_count);
        if ret < 0 {
            dev_err((*ctx).dev, c"Runtime get failed with err: %d\n".as_ptr(), ret);
            return ret;
        }
        if (*ctx).sst_state == SST_RESET && usage_count == 1 {
            ret = sst_load_fw(ctx);
            if ret != 0 {
                dev_err(dev, c"FW download fail %d\n".as_ptr(), ret);
                sst_set_fw_state_locked(ctx, SST_RESET);
                ret = sst_pm_runtime_put(ctx);
            }
        }
    } else {
        usage_count = GET_USAGE_COUNT(dev);
        dev_dbg((*ctx).dev, c"Disable: pm usage count: %d\n".as_ptr(), usage_count);
        return sst_pm_runtime_put(ctx);
    }
    ret
}

/*
 * sst_open_pcm_stream - Open PCM interface
 *
 * @str_param: parameters of pcm stream
 *
 * This function is called by MID sound card driver to open
 * a new pcm interface
 */
unsafe extern "C" fn sst_open_pcm_stream(
    dev: *mut device,
    str_param: *mut snd_sst_params,
) -> i32 {
    let retval: i32;
    let ctx: *mut intel_sst_drv = dev_get_drvdata(dev);

    if str_param.is_null() {
        return -EINVAL;
    }

    retval = sst_get_stream(ctx, str_param);
    if retval > 0 {
        (*ctx).stream_cnt += 1;
    } else {
        dev_err((*ctx).dev, c"sst_get_stream returned err %d\n".as_ptr(), retval);
    }

    retval
}

unsafe extern "C" fn sst_cdev_open(
    dev: *mut device,
    str_params: *mut snd_sst_params,
    cb: *mut sst_compress_cb,
) -> i32 {
    let mut str_id: i32;
    let retval: i32;
    let stream: *mut stream_info;
    let ctx: *mut intel_sst_drv = dev_get_drvdata(dev);

    retval = pm_runtime_resume_and_get((*ctx).dev);
    if retval < 0 {
        return retval;
    }

    str_id = sst_get_stream(ctx, str_params);
    if str_id > 0 {
        dev_dbg(dev, c"stream allocated in sst_cdev_open %d\n".as_ptr(), str_id);
        stream = &mut *(*ctx).streams.add(str_id as usize);
        (*stream).compr_cb = (*cb).compr_cb;
        (*stream).compr_cb_param = (*cb).param;
        (*stream).drain_notify = (*cb).drain_notify;
        (*stream).drain_cb_param = (*cb).drain_cb_param;
    } else {
        dev_err(dev, c"stream encountered error during alloc %d\n".as_ptr(), str_id);
        str_id = -EINVAL;
        sst_pm_runtime_put(ctx);
    }
    str_id
}

unsafe extern "C" fn sst_cdev_close(dev: *mut device, str_id: u32) -> i32 {
    let retval: i32;
    let stream: *mut stream_info;
    let ctx: *mut intel_sst_drv = dev_get_drvdata(dev);

    stream = get_stream_info(ctx, str_id);
    if stream.is_null() {
        dev_err(dev, c"stream info is NULL for str %d!!!\n".as_ptr(), str_id);
        return -EINVAL;
    }

    retval = sst_free_stream(ctx, str_id);
    (*stream).compr_cb_param = core::ptr::null_mut();
    (*stream).compr_cb = None;

    if retval != 0 {
        dev_err(dev, c"free stream returned err %d\n".as_ptr(), retval);
    }

    dev_dbg(dev, c"End\n".as_ptr());
    retval
}

unsafe extern "C" fn sst_cdev_ack(dev: *mut device, str_id: u32, bytes: c_ulong) -> i32 {
    let stream: *mut stream_info;
    let mut fw_tstamp: snd_sst_tstamp = core::mem::zeroed();
    let offset: i32;
    let addr: *mut c_void;
    let ctx: *mut intel_sst_drv = dev_get_drvdata(dev);

    stream = get_stream_info(ctx, str_id);
    if stream.is_null() {
        return -EINVAL;
    }

    /* update bytes sent */
    (*stream).cumm_bytes = (*stream).cumm_bytes.wrapping_add(bytes);
    dev_dbg(
        dev,
        c"bytes copied %d inc by %ld\n".as_ptr(),
        (*stream).cumm_bytes,
        bytes,
    );

    addr = ((*ctx).mailbox.add((*ctx).tstamp as usize) as *mut c_void)
        .byte_add(str_id as usize * core::mem::size_of_val(&fw_tstamp));

    memcpy_fromio(
        &mut fw_tstamp as *mut _ as *mut c_void,
        addr,
        core::mem::size_of_val(&fw_tstamp),
    );

    fw_tstamp.bytes_copied = (*stream).cumm_bytes as _;
    dev_dbg(
        dev,
        c"bytes sent to fw %llu inc by %ld\n".as_ptr(),
        fw_tstamp.bytes_copied,
        bytes,
    );

    offset = offset_of!(snd_sst_tstamp, bytes_copied) as i32;
    sst_shim_write(addr, offset, fw_tstamp.bytes_copied);
    0
}

unsafe extern "C" fn sst_cdev_set_metadata(
    dev: *mut device,
    str_id: u32,
    metadata: *mut snd_compr_metadata,
) -> i32 {
    let retval: i32;
    let str_info: *mut stream_info;
    let ctx: *mut intel_sst_drv = dev_get_drvdata(dev);

    dev_dbg(dev, c"set metadata for stream %d\n".as_ptr(), str_id);

    str_info = get_stream_info(ctx, str_id);
    if str_info.is_null() {
        return -EINVAL;
    }

    dev_dbg(dev, c"pipe id = %d\n".as_ptr(), (*str_info).pipe_id);
    retval = sst_prepare_and_post_msg(
        ctx,
        (*str_info).task_id,
        IPC_CMD,
        IPC_IA_SET_STREAM_PARAMS_MRFLD,
        (*str_info).pipe_id,
        core::mem::size_of::<snd_compr_metadata>() as _,
        metadata as *mut c_void,
        core::ptr::null_mut(),
        true,
        true,
        true,
        false,
    );

    retval
}

unsafe extern "C" fn sst_cdev_stream_pause(dev: *mut device, str_id: u32) -> i32 {
    let ctx: *mut intel_sst_drv = dev_get_drvdata(dev);

    sst_pause_stream(ctx, str_id)
}

unsafe extern "C" fn sst_cdev_stream_pause_release(dev: *mut device, str_id: u32) -> i32 {
    let ctx: *mut intel_sst_drv = dev_get_drvdata(dev);

    sst_resume_stream(ctx, str_id)
}

unsafe extern "C" fn sst_cdev_stream_start(dev: *mut device, str_id: u32) -> i32 {
    let str_info: *mut stream_info;
    let ctx: *mut intel_sst_drv = dev_get_drvdata(dev);

    str_info = get_stream_info(ctx, str_id);
    if str_info.is_null() {
        return -EINVAL;
    }
    (*str_info).prev = (*str_info).status;
    (*str_info).status = STREAM_RUNNING;
    sst_start_stream(ctx, str_id)
}

unsafe extern "C" fn sst_cdev_stream_drop(dev: *mut device, str_id: u32) -> i32 {
    let ctx: *mut intel_sst_drv = dev_get_drvdata(dev);

    sst_drop_stream(ctx, str_id)
}

unsafe extern "C" fn sst_cdev_stream_drain(dev: *mut device, str_id: u32) -> i32 {
    let ctx: *mut intel_sst_drv = dev_get_drvdata(dev);

    sst_drain_stream(ctx, str_id, false)
}

unsafe extern "C" fn sst_cdev_stream_partial_drain(dev: *mut device, str_id: u32) -> i32 {
    let ctx: *mut intel_sst_drv = dev_get_drvdata(dev);

    sst_drain_stream(ctx, str_id, true)
}

unsafe extern "C" fn sst_cdev_tstamp(
    dev: *mut device,
    str_id: u32,
    tstamp: *mut snd_compr_tstamp64,
) -> i32 {
    let mut fw_tstamp: snd_sst_tstamp = core::mem::zeroed();
    let stream: *mut stream_info;
    let ctx: *mut intel_sst_drv = dev_get_drvdata(dev);
    let addr: *mut c_void;

    addr = ((*ctx).mailbox.add((*ctx).tstamp as usize) as *mut c_void)
        .byte_add(str_id as usize * core::mem::size_of_val(&fw_tstamp));

    memcpy_fromio(
        &mut fw_tstamp as *mut _ as *mut c_void,
        addr,
        core::mem::size_of_val(&fw_tstamp),
    );

    stream = get_stream_info(ctx, str_id);
    if stream.is_null() {
        return -EINVAL;
    }
    dev_dbg(
        dev,
        c"rb_counter %llu in bytes\n".as_ptr(),
        fw_tstamp.ring_buffer_counter,
    );

    (*tstamp).copied_total = fw_tstamp.ring_buffer_counter;
    (*tstamp).pcm_frames = fw_tstamp.frames_decoded;
    (*tstamp).pcm_io_frames = div_u64(
        fw_tstamp.hardware_counter,
        (*stream).num_ch as u64 * SST_GET_BYTES_PER_SAMPLE(24) as u64,
    );
    (*tstamp).sampling_rate = fw_tstamp.sampling_frequency;

    dev_dbg(dev, c"PCM  = %llu\n".as_ptr(), (*tstamp).pcm_io_frames);
    dev_dbg(
        dev,
        c"Ptr Query on strid = %d  copied_total %llu, decodec %llu\n".as_ptr(),
        str_id,
        (*tstamp).copied_total,
        (*tstamp).pcm_frames,
    );
    dev_dbg(dev, c"rendered %llu\n".as_ptr(), (*tstamp).pcm_io_frames);

    0
}

unsafe extern "C" fn sst_cdev_caps(caps: *mut snd_compr_caps) -> i32 {
    (*caps).num_codecs = NUM_CODEC as _;
    (*caps).min_fragment_size = MIN_FRAGMENT_SIZE as _; /* 50KB */
    (*caps).max_fragment_size = MAX_FRAGMENT_SIZE as _; /* 1024KB */
    (*caps).min_fragments = MIN_FRAGMENT as _;
    (*caps).max_fragments = MAX_FRAGMENT as _;
    (*caps).codecs[0] = SND_AUDIOCODEC_MP3;
    (*caps).codecs[1] = SND_AUDIOCODEC_AAC;
    0
}

static caps_mp3: snd_compr_codec_caps = snd_compr_codec_caps {
    num_descriptors: 1,
    descriptor: {
        let mut descriptor = [snd_codec_desc::ZERO; MAX_NUM_CODEC_DESCRIPTORS];
        descriptor[0].max_ch = 2;
        descriptor[0].sample_rates[0] = 48000;
        descriptor[0].sample_rates[1] = 44100;
        descriptor[0].sample_rates[2] = 32000;
        descriptor[0].sample_rates[3] = 16000;
        descriptor[0].sample_rates[4] = 8000;
        descriptor[0].num_sample_rates = 5;
        descriptor[0].bit_rate[0] = 320;
        descriptor[0].bit_rate[1] = 192;
        descriptor[0].num_bitrates = 2;
        descriptor[0].profiles = 0;
        descriptor[0].modes = SND_AUDIOCHANMODE_MP3_STEREO;
        descriptor[0].formats = 0;
        descriptor
    },
    ..snd_compr_codec_caps::ZERO
};

static caps_aac: snd_compr_codec_caps = snd_compr_codec_caps {
    num_descriptors: 2,
    descriptor: {
        let mut descriptor = [snd_codec_desc::ZERO; MAX_NUM_CODEC_DESCRIPTORS];
        descriptor[1].max_ch = 2;
        descriptor[0].sample_rates[0] = 48000;
        descriptor[0].sample_rates[1] = 44100;
        descriptor[0].sample_rates[2] = 32000;
        descriptor[0].sample_rates[3] = 16000;
        descriptor[0].sample_rates[4] = 8000;
        descriptor[0].num_sample_rates = 5;
        descriptor[1].bit_rate[0] = 320;
        descriptor[1].bit_rate[1] = 192;
        descriptor[1].num_bitrates = 2;
        descriptor[1].profiles = 0;
        descriptor[1].modes = 0;
        descriptor[1].formats = SND_AUDIOSTREAMFORMAT_MP4ADTS | SND_AUDIOSTREAMFORMAT_RAW;
        descriptor
    },
    ..snd_compr_codec_caps::ZERO
};

unsafe extern "C" fn sst_cdev_codec_caps(codec: *mut snd_compr_codec_caps) -> i32 {
    if (*codec).codec == SND_AUDIOCODEC_MP3 {
        *codec = caps_mp3;
    } else if (*codec).codec == SND_AUDIOCODEC_AAC {
        *codec = caps_aac;
    } else {
        return -EINVAL;
    }

    0
}

/*
 * sst_close_pcm_stream - Close PCM interface
 *
 * @str_id: stream id to be closed
 *
 * This function is called by MID sound card driver to close
 * an existing pcm interface
 */
unsafe extern "C" fn sst_close_pcm_stream(dev: *mut device, str_id: u32) -> i32 {
    let stream: *mut stream_info;
    let retval: i32;
    let ctx: *mut intel_sst_drv = dev_get_drvdata(dev);

    stream = get_stream_info(ctx, str_id);
    if stream.is_null() {
        dev_err((*ctx).dev, c"stream info is NULL for str %d!!!\n".as_ptr(), str_id);
        return -EINVAL;
    }

    retval = free_stream_context(ctx, str_id);
    (*stream).pcm_substream = core::ptr::null_mut();
    (*stream).status = STREAM_UN_INIT;
    (*stream).period_elapsed = None;
    (*ctx).stream_cnt -= 1;

    if retval != 0 {
        dev_err((*ctx).dev, c"free stream returned err %d\n".as_ptr(), retval);
    }

    dev_dbg((*ctx).dev, c"Exit\n".as_ptr());
    0
}

#[inline]
unsafe extern "C" fn sst_calc_tstamp(
    ctx: *mut intel_sst_drv,
    info: *mut pcm_stream_info,
    substream: *mut snd_pcm_substream,
    fw_tstamp: *mut snd_sst_tstamp,
) -> i32 {
    let delay_bytes: usize;
    let delay_frames: usize;
    let buffer_sz: usize;
    let mut pointer_bytes: u32 = 0;
    let pointer_samples: u32;

    dev_dbg(
        (*ctx).dev,
        c"mrfld ring_buffer_counter %llu in bytes\n".as_ptr(),
        (*fw_tstamp).ring_buffer_counter,
    );
    dev_dbg(
        (*ctx).dev,
        c"mrfld hardware_counter %llu in bytes\n".as_ptr(),
        (*fw_tstamp).hardware_counter,
    );
    if (*substream).stream == SNDRV_PCM_STREAM_PLAYBACK {
        delay_bytes = ((*fw_tstamp).ring_buffer_counter - (*fw_tstamp).hardware_counter) as usize;
    } else {
        delay_bytes = ((*fw_tstamp).hardware_counter - (*fw_tstamp).ring_buffer_counter) as usize;
    }
    delay_frames = bytes_to_frames((*substream).runtime, delay_bytes);
    buffer_sz = snd_pcm_lib_buffer_bytes(substream);
    div_u64_rem(
        (*fw_tstamp).ring_buffer_counter,
        buffer_sz as u32,
        &mut pointer_bytes,
    );
    pointer_samples = bytes_to_samples((*substream).runtime, pointer_bytes);

    dev_dbg((*ctx).dev, c"pcm delay %zu in bytes\n".as_ptr(), delay_bytes);

    (*info).buffer_ptr = pointer_samples / (*(*substream).runtime).channels;

    (*info).pcm_delay = delay_frames as _;
    dev_dbg(
        (*ctx).dev,
        c"buffer ptr %llu pcm_delay rep: %llu\n".as_ptr(),
        (*info).buffer_ptr,
        (*info).pcm_delay,
    );
    0
}

unsafe extern "C" fn sst_read_timestamp(
    dev: *mut device,
    info: *mut pcm_stream_info,
) -> i32 {
    let stream: *mut stream_info;
    let substream: *mut snd_pcm_substream;
    let mut fw_tstamp: snd_sst_tstamp = core::mem::zeroed();
    let str_id: u32;
    let ctx: *mut intel_sst_drv = dev_get_drvdata(dev);
    let addr: *mut c_void;

    str_id = (*info).str_id;
    stream = get_stream_info(ctx, str_id);
    if stream.is_null() {
        return -EINVAL;
    }

    if (*stream).pcm_substream.is_null() {
        return -EINVAL;
    }
    substream = (*stream).pcm_substream;

    addr = ((*ctx).mailbox.add((*ctx).tstamp as usize) as *mut c_void)
        .byte_add(str_id as usize * core::mem::size_of_val(&fw_tstamp));

    memcpy_fromio(
        &mut fw_tstamp as *mut _ as *mut c_void,
        addr,
        core::mem::size_of_val(&fw_tstamp),
    );

    sst_calc_tstamp(ctx, info, substream, &mut fw_tstamp)
}

unsafe extern "C" fn sst_stream_start(dev: *mut device, str_id: i32) -> i32 {
    let str_info: *mut stream_info;
    let ctx: *mut intel_sst_drv = dev_get_drvdata(dev);

    if (*ctx).sst_state != SST_FW_RUNNING {
        return 0;
    }

    str_info = get_stream_info(ctx, str_id as u32);
    if str_info.is_null() {
        return -EINVAL;
    }
    (*str_info).prev = (*str_info).status;
    (*str_info).status = STREAM_RUNNING;
    sst_start_stream(ctx, str_id as u32);

    0
}

unsafe extern "C" fn sst_stream_drop(dev: *mut device, str_id: i32) -> i32 {
    let str_info: *mut stream_info;
    let ctx: *mut intel_sst_drv = dev_get_drvdata(dev);

    if (*ctx).sst_state != SST_FW_RUNNING {
        return 0;
    }

    str_info = get_stream_info(ctx, str_id as u32);
    if str_info.is_null() {
        return -EINVAL;
    }
    (*str_info).prev = STREAM_UN_INIT;
    (*str_info).status = STREAM_INIT;
    sst_drop_stream(ctx, str_id as u32)
}

unsafe extern "C" fn sst_stream_pause(dev: *mut device, str_id: i32) -> i32 {
    let str_info: *mut stream_info;
    let ctx: *mut intel_sst_drv = dev_get_drvdata(dev);

    if (*ctx).sst_state != SST_FW_RUNNING {
        return 0;
    }

    str_info = get_stream_info(ctx, str_id as u32);
    if str_info.is_null() {
        return -EINVAL;
    }

    sst_pause_stream(ctx, str_id as u32)
}

unsafe extern "C" fn sst_stream_resume(dev: *mut device, str_id: i32) -> i32 {
    let str_info: *mut stream_info;
    let ctx: *mut intel_sst_drv = dev_get_drvdata(dev);

    if (*ctx).sst_state != SST_FW_RUNNING {
        return 0;
    }

    str_info = get_stream_info(ctx, str_id as u32);
    if str_info.is_null() {
        return -EINVAL;
    }
    sst_resume_stream(ctx, str_id as u32)
}

unsafe extern "C" fn sst_stream_init(
    dev: *mut device,
    str_info: *mut pcm_stream_info,
) -> i32 {
    let mut str_id: i32 = 0;
    let stream: *mut stream_info;
    let ctx: *mut intel_sst_drv = dev_get_drvdata(dev);

    str_id = (*str_info).str_id as i32;

    if (*ctx).sst_state != SST_FW_RUNNING {
        return 0;
    }

    stream = get_stream_info(ctx, str_id as u32);
    if stream.is_null() {
        return -EINVAL;
    }

    dev_dbg((*ctx).dev, c"setting the period ptrs\n".as_ptr());
    (*stream).pcm_substream = (*str_info).arg;
    (*stream).period_elapsed = (*str_info).period_elapsed;
    (*stream).sfreq = (*str_info).sfreq;
    (*stream).prev = (*stream).status;
    (*stream).status = STREAM_INIT;
    dev_dbg(
        (*ctx).dev,
        c"pcm_substream %p, period_elapsed %p, sfreq %d, status %d\n".as_ptr(),
        (*stream).pcm_substream,
        (*stream).period_elapsed,
        (*stream).sfreq,
        (*stream).status,
    );

    0
}

/*
 * sst_set_byte_stream - Set generic params
 *
 * @cmd: control cmd to be set
 * @arg: command argument
 *
 * This function is called by MID sound card driver to configure
 * SST runtime params.
 */
unsafe extern "C" fn sst_send_byte_stream(
    dev: *mut device,
    bytes: *mut snd_sst_bytes_v2,
) -> i32 {
    let mut ret_val: i32 = 0;
    let ctx: *mut intel_sst_drv = dev_get_drvdata(dev);

    if bytes.is_null() {
        return -EINVAL;
    }
    ret_val = pm_runtime_resume_and_get((*ctx).dev);
    if ret_val < 0 {
        return ret_val;
    }

    ret_val = sst_send_byte_stream_mrfld(ctx, bytes);
    sst_pm_runtime_put(ctx);

    ret_val
}

static mut pcm_ops: sst_ops = sst_ops {
    open: Some(sst_open_pcm_stream),
    stream_init: Some(sst_stream_init),
    stream_start: Some(sst_stream_start),
    stream_drop: Some(sst_stream_drop),
    stream_pause: Some(sst_stream_pause),
    stream_pause_release: Some(sst_stream_resume),
    stream_read_tstamp: Some(sst_read_timestamp),
    send_byte_stream: Some(sst_send_byte_stream),
    close: Some(sst_close_pcm_stream),
    power: Some(sst_power_control),
};

static mut compr_ops: compress_sst_ops = compress_sst_ops {
    open: Some(sst_cdev_open),
    close: Some(sst_cdev_close),
    stream_pause: Some(sst_cdev_stream_pause),
    stream_pause_release: Some(sst_cdev_stream_pause_release),
    stream_start: Some(sst_cdev_stream_start),
    stream_drop: Some(sst_cdev_stream_drop),
    stream_drain: Some(sst_cdev_stream_drain),
    stream_partial_drain: Some(sst_cdev_stream_partial_drain),
    tstamp: Some(sst_cdev_tstamp),
    ack: Some(sst_cdev_ack),
    get_caps: Some(sst_cdev_caps),
    get_codec_caps: Some(sst_cdev_codec_caps),
    set_metadata: Some(sst_cdev_set_metadata),
    power: Some(sst_power_control),
};

static mut sst_dsp_device: sst_device = sst_device {
    name: c"Intel(R) SST LPE".as_ptr(),
    dev: core::ptr::null_mut(),
    ops: unsafe { &mut pcm_ops },
    compr_ops: unsafe { &mut compr_ops },
};

/*
 * sst_register - function to register DSP
 *
 * This functions registers DSP with the platform driver
 */
pub unsafe extern "C" fn sst_register(dev: *mut device) -> i32 {
    let ret_val: i32;

    sst_dsp_device.dev = dev;
    ret_val = sst_register_dsp(&mut sst_dsp_device);
    if ret_val != 0 {
        dev_err(dev, c"Unable to register DSP with platform driver\n".as_ptr());
    }

    ret_val
}

pub unsafe extern "C" fn sst_unregister(_dev: *mut device) -> i32 {
    sst_unregister_dsp(&mut sst_dsp_device)
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
