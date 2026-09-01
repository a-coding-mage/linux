// SPDX-License-Identifier: GPL-2.0
// Copyright (c) 2021, Linaro Limited

// Translated from the implementation source. C include dependencies are expected
// to be provided by the surrounding kernel Rust binding environment.

use core::ffi::c_void;
use core::mem::size_of;
use core::ptr;

const DRV_NAME: &[u8] = b"q6apm-dai\0";
const POS_BUFFER_BYTES: u32 = 4096;

const PLAYBACK_MIN_NUM_PERIODS: u32 = 2;
const PLAYBACK_MAX_NUM_PERIODS: u32 = 8;
const PLAYBACK_MAX_PERIOD_SIZE: u32 = 65536;
const PLAYBACK_MIN_PERIOD_SIZE: u32 = 128;
const CAPTURE_MIN_NUM_PERIODS: u32 = 2;
const CAPTURE_MAX_NUM_PERIODS: u32 = 8;
const CAPTURE_MAX_PERIOD_SIZE: u32 = 65536;
const CAPTURE_MIN_PERIOD_SIZE: u32 = 6144;
const BUFFER_BYTES_MAX: u32 = PLAYBACK_MAX_NUM_PERIODS * PLAYBACK_MAX_PERIOD_SIZE;
const BUFFER_BYTES_MIN: u32 = PLAYBACK_MIN_NUM_PERIODS * PLAYBACK_MIN_PERIOD_SIZE;
const COMPR_PLAYBACK_MAX_FRAGMENT_SIZE: u32 = 128 * 1024;
const COMPR_PLAYBACK_MAX_NUM_FRAGMENTS: u32 = 16 * 4;
const COMPR_PLAYBACK_MIN_FRAGMENT_SIZE: u32 = 8 * 1024;
const COMPR_PLAYBACK_MIN_NUM_FRAGMENTS: u32 = 4;
const SID_MASK_DEFAULT: u32 = 0xF;

static q6apm_compr_caps: snd_compr_codec_caps = snd_compr_codec_caps {
    num_descriptors: 1,
    descriptor: {
        let mut descriptor = [snd_codec_desc::zeroed(); SND_MAX_CODEC_DESCRIPTORS];
        descriptor[0].max_ch = 2;
        descriptor[0].sample_rates = [
            8000, 11025, 12000, 16000, 22050, 24000, 32000, 44100, 48000, 88200, 96000, 176400,
            192000,
        ];
        descriptor[0].num_sample_rates = 13;
        descriptor[0].bit_rate[0] = 320;
        descriptor[0].bit_rate[1] = 128;
        descriptor[0].num_bitrates = 2;
        descriptor[0].profiles = 0;
        descriptor[0].modes = SND_AUDIOCHANMODE_MP3_STEREO;
        descriptor[0].formats = 0;
        descriptor
    },
};

#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq)]
enum stream_state {
    Q6APM_STREAM_IDLE = 0,
    Q6APM_STREAM_STOPPED,
    Q6APM_STREAM_RUNNING,
}

#[repr(C)]
struct q6apm_dai_rtd {
    substream: *mut snd_pcm_substream,
    cstream: *mut snd_compr_stream,
    codec: snd_codec,
    codec_param: snd_compr_params,
    dma_buffer: snd_dma_buffer,
    pos_buffer: *mut sh_mem_pull_push_mode_position_buffer,
    last_pos_index: u32,
    phys: phys_addr_t,
    pos_phys: phys_addr_t,
    pcm_size: u32,
    push_pull_size: u32,
    pcm_count: u32,
    periods: u32,
    bytes_sent: u64,
    bytes_received: u64,
    copied_total: u64,
    bits_per_sample: u16,
    queue_ptr: snd_pcm_uframes_t,
    next_track: bool,
    state: stream_state,
    graph: *mut q6apm_graph,
    lock: spinlock_t,
    notify_on_drain: bool,
}

#[repr(C)]
struct q6apm_dai_data {
    sid: i64,
}

static q6apm_dai_hardware_capture: snd_pcm_hardware = snd_pcm_hardware {
    info: SNDRV_PCM_INFO_MMAP
        | SNDRV_PCM_INFO_BLOCK_TRANSFER
        | SNDRV_PCM_INFO_MMAP_VALID
        | SNDRV_PCM_INFO_INTERLEAVED
        | SNDRV_PCM_INFO_PAUSE
        | SNDRV_PCM_INFO_RESUME
        | SNDRV_PCM_INFO_NO_REWINDS
        | SNDRV_PCM_INFO_SYNC_APPLPTR
        | SNDRV_PCM_INFO_BATCH,
    formats: SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S24_LE,
    rates: SNDRV_PCM_RATE_8000_48000,
    rate_min: 8000,
    rate_max: 48000,
    channels_min: 2,
    channels_max: 4,
    buffer_bytes_max: CAPTURE_MAX_NUM_PERIODS * CAPTURE_MAX_PERIOD_SIZE,
    period_bytes_min: CAPTURE_MIN_PERIOD_SIZE,
    period_bytes_max: CAPTURE_MAX_PERIOD_SIZE,
    periods_min: CAPTURE_MIN_NUM_PERIODS,
    periods_max: CAPTURE_MAX_NUM_PERIODS,
    fifo_size: 0,
};

static q6apm_dai_hardware_playback: snd_pcm_hardware = snd_pcm_hardware {
    info: SNDRV_PCM_INFO_MMAP
        | SNDRV_PCM_INFO_BLOCK_TRANSFER
        | SNDRV_PCM_INFO_MMAP_VALID
        | SNDRV_PCM_INFO_INTERLEAVED
        | SNDRV_PCM_INFO_PAUSE
        | SNDRV_PCM_INFO_RESUME
        | SNDRV_PCM_INFO_NO_REWINDS
        | SNDRV_PCM_INFO_SYNC_APPLPTR
        | SNDRV_PCM_INFO_BATCH,
    formats: SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S24_LE,
    rates: SNDRV_PCM_RATE_8000_192000,
    rate_min: 8000,
    rate_max: 192000,
    channels_min: 2,
    channels_max: 8,
    buffer_bytes_max: PLAYBACK_MAX_NUM_PERIODS * PLAYBACK_MAX_PERIOD_SIZE,
    period_bytes_min: PLAYBACK_MIN_PERIOD_SIZE,
    period_bytes_max: PLAYBACK_MAX_PERIOD_SIZE,
    periods_min: PLAYBACK_MIN_NUM_PERIODS,
    periods_max: PLAYBACK_MAX_NUM_PERIODS,
    fifo_size: 0,
};

unsafe extern "C" fn event_handler(
    opcode: u32,
    _token: u32,
    _payload: *mut c_void,
    priv_: *mut c_void,
) {
    let prtd = priv_ as *mut q6apm_dai_rtd;
    let substream = (*prtd).substream;

    match opcode {
        APM_CLIENT_EVENT_WATERMARK_EVENT => snd_pcm_period_elapsed(substream),
        APM_CLIENT_EVENT_CMD_EOS_DONE => (*prtd).state = stream_state::Q6APM_STREAM_STOPPED,
        APM_CLIENT_EVENT_DATA_WRITE_DONE => {
            snd_pcm_period_elapsed(substream);
        }
        APM_CLIENT_EVENT_DATA_READ_DONE => {
            snd_pcm_period_elapsed(substream);
            if (*prtd).state == stream_state::Q6APM_STREAM_RUNNING {
                q6apm_read((*prtd).graph);
            }
        }
        _ => {}
    }
}

unsafe extern "C" fn event_handler_compr(
    opcode: u32,
    token: u32,
    _payload: *mut c_void,
    priv_: *mut c_void,
) {
    let prtd = priv_ as *mut q6apm_dai_rtd;
    let substream = (*prtd).cstream;
    let mut wflags: u32 = 0;
    let mut is_last_buffer = false;

    let _guard = spinlock_irqsave_guard(&mut (*prtd).lock);
    match opcode {
        APM_CLIENT_EVENT_CMD_EOS_DONE => {
            if (*prtd).notify_on_drain {
                snd_compr_drain_notify((*prtd).cstream);
                (*prtd).notify_on_drain = false;
            } else {
                (*prtd).state = stream_state::Q6APM_STREAM_STOPPED;
            }
        }
        APM_CLIENT_EVENT_DATA_WRITE_DONE => {
            let bytes_written: u32 = token >> APM_WRITE_TOKEN_LEN_SHIFT;
            (*prtd).copied_total = (*prtd).copied_total.wrapping_add(bytes_written as u64);
            snd_compr_fragment_elapsed(substream);

            if (*prtd).state != stream_state::Q6APM_STREAM_RUNNING {
                return;
            }

            let avail = (*prtd).bytes_received.wrapping_sub((*prtd).bytes_sent);
            let bytes_to_write: u32 = if avail > (*prtd).pcm_count as u64 {
                (*prtd).pcm_count
            } else {
                if (*substream).partial_drain || (*prtd).notify_on_drain {
                    is_last_buffer = true;
                }
                avail as u32
            };

            if bytes_to_write != 0 {
                if (*substream).partial_drain && is_last_buffer {
                    wflags |= APM_LAST_BUFFER_FLAG;
                }

                q6apm_write_async((*prtd).graph, bytes_to_write, 0, 0, wflags);
                (*prtd).bytes_sent = (*prtd).bytes_sent.wrapping_add(bytes_to_write as u64);

                if (*prtd).notify_on_drain && is_last_buffer {
                    audioreach_shared_memory_send_eos((*prtd).graph);
                }
            }
        }
        _ => {}
    }
}

unsafe extern "C" fn q6apm_dai_prepare(
    component: *mut snd_soc_component,
    substream: *mut snd_pcm_substream,
) -> i32 {
    let runtime = (*substream).runtime;
    let prtd = (*runtime).private_data as *mut q6apm_dai_rtd;
    let mut cfg: audioreach_module_config = core::mem::zeroed();
    let dev = (*component).dev;
    let pdata = snd_soc_component_get_drvdata(component) as *mut q6apm_dai_data;

    if pdata.is_null() {
        return -EINVAL;
    }

    if prtd.is_null() || (*prtd).graph.is_null() {
        dev_err(dev, c"%s: private data null or audio client freed\n".as_ptr(), __func__());
        return -EINVAL;
    }

    cfg.direction = (*substream).stream;
    cfg.sample_rate = (*runtime).rate;
    cfg.num_channels = (*runtime).channels;
    cfg.bit_width = (*prtd).bits_per_sample;
    cfg.fmt = SND_AUDIOCODEC_PCM;
    audioreach_set_default_channel_mapping(cfg.channel_map.as_mut_ptr(), (*runtime).channels);
    if (*prtd).state as u32 != 0 {
        /* clear the previous setup if any  */
        q6apm_graph_stop((*prtd).graph);
        q6apm_free_fragments((*prtd).graph, (*substream).stream);
    }

    (*prtd).last_pos_index = 0;
    (*prtd).pcm_count = snd_pcm_lib_period_bytes(substream);
    if q6apm_is_graph_in_push_pull_mode((*prtd).graph) {
        if (*prtd).pcm_size != (*prtd).push_pull_size {
            let mut ret = q6apm_push_pull_config(
                (*prtd).graph,
                (*prtd).phys,
                (*prtd).pos_phys,
                (*prtd).pcm_size,
            );
            if ret < 0 {
                dev_err(dev, c"Push/Pull config failed rc = %d\n".as_ptr(), ret);
                return ret;
            }

            ret = q6apm_register_watermark_event(
                (*prtd).graph,
                (*prtd).pcm_size / (*prtd).periods,
                (*prtd).periods,
            );
            if ret < 0 {
                dev_err(dev, c"WaterMark event config failed rc = %d\n".as_ptr(), ret);
                return ret;
            }
            (*prtd).push_pull_size = (*prtd).pcm_size;
        }
    } else {
        let ret = q6apm_alloc_fragments(
            (*prtd).graph,
            (*substream).stream,
            (*prtd).phys,
            (*prtd).pcm_size / (*prtd).periods,
            (*prtd).periods,
        );
        if ret < 0 {
            dev_err(dev, c"Audio Start: Buffer Allocation failed rc = %d\n".as_ptr(), ret);
            return ret;
        }
    }

    let mut ret = q6apm_graph_media_format_pcm((*prtd).graph, &mut cfg);
    if ret < 0 {
        dev_err(dev, c"%s: CMD Format block failed\n".as_ptr(), __func__());
        return ret;
    }

    /* rate and channels are sent to audio driver */
    ret = q6apm_graph_media_format_shmem((*prtd).graph, &mut cfg);
    if ret < 0 {
        dev_err(dev, c"Failed to set media format %d\n".as_ptr(), ret);
        return ret;
    }

    ret = q6apm_graph_prepare((*prtd).graph);
    if ret != 0 {
        dev_err(dev, c"Failed to prepare Graph %d\n".as_ptr(), ret);
        return ret;
    }

    ret = q6apm_graph_start((*prtd).graph);
    if ret != 0 {
        dev_err(dev, c"Failed to Start Graph %d\n".as_ptr(), ret);
        return ret;
    }
    if !q6apm_is_graph_in_push_pull_mode((*prtd).graph) {
        if (*substream).stream == SNDRV_PCM_STREAM_CAPTURE {
            /* Queue the buffers for Capture ONLY after graph is started */
            let mut i = 0;
            while i < (*runtime).periods {
                q6apm_read((*prtd).graph);
                i += 1;
            }
        }
    }

    /* Now that graph as been prepared and started update the internal state accordingly */
    (*prtd).state = stream_state::Q6APM_STREAM_RUNNING;

    0
}

unsafe extern "C" fn q6apm_dai_ack(
    component: *mut snd_soc_component,
    substream: *mut snd_pcm_substream,
) -> i32 {
    let runtime = (*substream).runtime;
    let prtd = (*runtime).private_data as *mut q6apm_dai_rtd;
    let mut ret = 0;

    if q6apm_is_graph_in_push_pull_mode((*prtd).graph) {
        return 0;
    }

    if (*substream).stream == SNDRV_PCM_STREAM_PLAYBACK {
        let avail_periods =
            ((*(*runtime).control).appl_ptr - (*prtd).queue_ptr) / (*runtime).period_size;
        let mut i = 0;
        while i < avail_periods {
            ret = q6apm_write_async((*prtd).graph, (*prtd).pcm_count, 0, 0, NO_TIMESTAMP);
            if ret < 0 {
                dev_err((*component).dev, c"Error queuing playback buffer %d\n".as_ptr(), ret);
                return ret;
            }
            (*prtd).queue_ptr += (*runtime).period_size;
            i += 1;
        }
    }

    ret
}

unsafe extern "C" fn q6apm_dai_trigger(
    _component: *mut snd_soc_component,
    substream: *mut snd_pcm_substream,
    cmd: i32,
) -> i32 {
    let runtime = (*substream).runtime;
    let prtd = (*runtime).private_data as *mut q6apm_dai_rtd;
    let mut ret = 0;

    match cmd {
        SNDRV_PCM_TRIGGER_START | SNDRV_PCM_TRIGGER_RESUME | SNDRV_PCM_TRIGGER_PAUSE_RELEASE => {}
        SNDRV_PCM_TRIGGER_STOP => {
            /* TODO support be handled via SoftPause Module */
            (*prtd).state = stream_state::Q6APM_STREAM_STOPPED;
            (*prtd).queue_ptr = 0;
            (*prtd).last_pos_index = 0;
        }
        SNDRV_PCM_TRIGGER_SUSPEND | SNDRV_PCM_TRIGGER_PAUSE_PUSH => {}
        _ => ret = -EINVAL,
    }

    ret
}

unsafe extern "C" fn q6apm_dai_open(
    component: *mut snd_soc_component,
    substream: *mut snd_pcm_substream,
) -> i32 {
    let runtime = (*substream).runtime;
    let soc_prtd = snd_soc_substream_to_rtd(substream);
    let cpu_dai = snd_soc_rtd_to_cpu(soc_prtd, 0);
    let dev = (*component).dev;
    let graph_id = (*(*cpu_dai).driver).id;
    let pdata = snd_soc_component_get_drvdata(component) as *mut q6apm_dai_data;

    if pdata.is_null() {
        dev_err(dev, c"Drv data not found ..\n".as_ptr());
        return -EINVAL;
    }

    let prtd = kzalloc(size_of::<q6apm_dai_rtd>(), GFP_KERNEL) as *mut q6apm_dai_rtd;
    if prtd.is_null() {
        return -ENOMEM;
    }

    spin_lock_init(&mut (*prtd).lock);
    (*prtd).substream = substream;
    (*prtd).graph = q6apm_graph_open(
        dev,
        Some(event_handler),
        prtd as *mut c_void,
        graph_id,
        (*substream).stream,
    );
    if IS_ERR((*prtd).graph as *const c_void) {
        dev_err(dev, c"%s: Could not allocate memory\n".as_ptr(), __func__());
        let ret = PTR_ERR((*prtd).graph as *const c_void);
        kfree(prtd as *mut c_void);
        return ret;
    }

    if (*substream).stream == SNDRV_PCM_STREAM_PLAYBACK {
        (*runtime).hw = q6apm_dai_hardware_playback;
    } else if (*substream).stream == SNDRV_PCM_STREAM_CAPTURE {
        (*runtime).hw = q6apm_dai_hardware_capture;
    }

    /* Ensure that buffer size is a multiple of period size */
    let mut ret = snd_pcm_hw_constraint_integer(runtime, SNDRV_PCM_HW_PARAM_PERIODS);
    if ret < 0 {
        dev_err(dev, c"snd_pcm_hw_constraint_integer failed\n".as_ptr());
        kfree(prtd as *mut c_void);
        return ret;
    }

    if (*substream).stream == SNDRV_PCM_STREAM_PLAYBACK {
        ret = snd_pcm_hw_constraint_minmax(
            runtime,
            SNDRV_PCM_HW_PARAM_BUFFER_BYTES,
            BUFFER_BYTES_MIN,
            BUFFER_BYTES_MAX,
        );
        if ret < 0 {
            dev_err(dev, c"constraint for buffer bytes min max ret = %d\n".as_ptr(), ret);
            kfree(prtd as *mut c_void);
            return ret;
        }
    }

    /* setup 10ms latency to accommodate DSP restrictions */
    ret = snd_pcm_hw_constraint_step(runtime, 0, SNDRV_PCM_HW_PARAM_PERIOD_SIZE, 480);
    if ret < 0 {
        dev_err(dev, c"constraint for period bytes step ret = %d\n".as_ptr(), ret);
        kfree(prtd as *mut c_void);
        return ret;
    }

    ret = snd_pcm_hw_constraint_step(runtime, 0, SNDRV_PCM_HW_PARAM_BUFFER_SIZE, 480);
    if ret < 0 {
        dev_err(dev, c"constraint for buffer bytes step ret = %d\n".as_ptr(), ret);
        kfree(prtd as *mut c_void);
        return ret;
    }

    (*runtime).private_data = prtd as *mut c_void;
    (*runtime).dma_bytes = BUFFER_BYTES_MAX;
    if (*pdata).sid < 0 {
        (*prtd).phys = (*substream).dma_buffer.addr;
    } else {
        (*prtd).phys = (*substream).dma_buffer.addr | (((*pdata).sid as phys_addr_t) << 32);
    }

    if q6apm_is_graph_in_push_pull_mode((*prtd).graph) {
        let pos_buffer: *mut c_void;

        (*prtd).pos_phys = (*prtd).phys + BUFFER_BYTES_MAX as phys_addr_t;
        pos_buffer = (*substream).dma_buffer.area.add(BUFFER_BYTES_MAX as usize) as *mut c_void;
        (*prtd).pos_buffer = pos_buffer as *mut sh_mem_pull_push_mode_position_buffer;
    }

    0
}

unsafe extern "C" fn q6apm_dai_close(
    _component: *mut snd_soc_component,
    substream: *mut snd_pcm_substream,
) -> i32 {
    let runtime = (*substream).runtime;
    let prtd = (*runtime).private_data as *mut q6apm_dai_rtd;

    if (*prtd).state as u32 != 0 {
        /* only stop graph that is started */
        q6apm_graph_stop((*prtd).graph);
        q6apm_free_fragments((*prtd).graph, (*substream).stream);
    }

    q6apm_graph_close((*prtd).graph);
    (*prtd).graph = ptr::null_mut();
    kfree(prtd as *mut c_void);
    (*runtime).private_data = ptr::null_mut();

    0
}

unsafe extern "C" fn q6apm_dai_pointer(
    _component: *mut snd_soc_component,
    substream: *mut snd_pcm_substream,
) -> snd_pcm_uframes_t {
    let runtime = (*substream).runtime;
    let prtd = (*runtime).private_data as *mut q6apm_dai_rtd;

    if q6apm_is_graph_in_push_pull_mode((*prtd).graph) {
        let mut retries = 10;
        let mut index: u32;
        let mut fc1: u32;
        let mut fc2: u32;

        /* index is valid if frame_counter does not change while reading. */
        loop {
            fc1 = READ_ONCE(&(*(*prtd).pos_buffer).frame_counter);
            index = READ_ONCE(&(*(*prtd).pos_buffer).index);
            fc2 = READ_ONCE(&(*(*prtd).pos_buffer).frame_counter);
            if !(fc1 != fc2 && {
                retries -= 1;
                retries != 0
            }) {
                break;
            }
        }

        if fc1 != fc2 {
            index = (*prtd).last_pos_index;
        } else {
            (*prtd).last_pos_index = index;
        }

        return bytes_to_frames(runtime, index);
    }

    let ptr = q6apm_get_hw_pointer((*prtd).graph, (*substream).stream) * (*runtime).period_size;
    if ptr != 0 {
        return ptr - 1;
    }

    0
}

unsafe extern "C" fn q6apm_dai_hw_params(
    _component: *mut snd_soc_component,
    substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
) -> i32 {
    let runtime = (*substream).runtime;
    let prtd = (*runtime).private_data as *mut q6apm_dai_rtd;

    (*prtd).pcm_size = params_buffer_bytes(params);
    (*prtd).periods = params_periods(params);

    match params_format(params) {
        SNDRV_PCM_FORMAT_S16_LE => (*prtd).bits_per_sample = 16,
        SNDRV_PCM_FORMAT_S24_LE => (*prtd).bits_per_sample = 24,
        _ => return -EINVAL,
    }

    0
}

unsafe extern "C" fn q6apm_dai_memory_map(
    component: *mut snd_soc_component,
    substream: *mut snd_pcm_substream,
    graph_id: i32,
    is_push_pull: bool,
) -> i32 {
    let dev = (*component).dev;
    let pdata = snd_soc_component_get_drvdata(component) as *mut q6apm_dai_data;
    let mut phys: phys_addr_t;

    if pdata.is_null() {
        dev_err((*component).dev, c"Drv data not found ..\n".as_ptr());
        return -EINVAL;
    }

    if (*pdata).sid < 0 {
        phys = (*substream).dma_buffer.addr;
    } else {
        phys = (*substream).dma_buffer.addr | (((*pdata).sid as phys_addr_t) << 32);
    }

    let mut ret = q6apm_map_memory_fixed_region(dev, graph_id, phys, BUFFER_BYTES_MAX);
    if ret < 0 {
        dev_err(dev, c"Audio Start: Buffer Allocation failed rc = %d\n".as_ptr(), ret);
    }

    if is_push_pull {
        if (*pdata).sid < 0 {
            phys = (*substream).dma_buffer.addr + BUFFER_BYTES_MAX as phys_addr_t;
        } else {
            phys = ((*substream).dma_buffer.addr + BUFFER_BYTES_MAX as phys_addr_t)
                | (((*pdata).sid as phys_addr_t) << 32);
        }

        ret = q6apm_map_pos_buffer(dev, graph_id, phys, POS_BUFFER_BYTES);
        if ret < 0 {
            dev_err(dev, c"Audio Start: Buffer Allocation failed rc = %d\n".as_ptr(), ret);
        }
    } else {
    }

    ret
}

unsafe extern "C" fn q6apm_dai_pcm_new(
    component: *mut snd_soc_component,
    rtd: *mut snd_soc_pcm_runtime,
) -> i32 {
    let cpu_dai = snd_soc_rtd_to_cpu(rtd, 0);
    let pcm = (*rtd).pcm;
    /*
     * Allocate one extra page as a workaround for a DSP bug where 32-bit
     * address arithmetic can overflow when the buffer is placed near the
     * end of the addressable range.
     */
    let mut size = BUFFER_BYTES_MAX + PAGE_SIZE;
    let graph_id = (*(*cpu_dai).driver).id;
    let mut substream: *mut snd_pcm_substream = ptr::null_mut();

    /* Note: DSP backend dais are uni-directional ONLY(either playback or capture) */
    if !(*pcm).streams[SNDRV_PCM_STREAM_PLAYBACK as usize].substream.is_null() {
        substream = (*pcm).streams[SNDRV_PCM_STREAM_PLAYBACK as usize].substream;
    } else if !(*pcm).streams[SNDRV_PCM_STREAM_CAPTURE as usize].substream.is_null() {
        substream = (*pcm).streams[SNDRV_PCM_STREAM_CAPTURE as usize].substream;
    }

    if !substream.is_null() {
        let is_push_pull =
            q6apm_is_graph_in_push_pull_mode_from_id((*component).dev, graph_id, (*substream).stream);
        if is_push_pull {
            size += POS_BUFFER_BYTES;
        }

        let mut ret = snd_pcm_set_fixed_buffer_all(pcm, SNDRV_DMA_TYPE_DEV, (*component).dev, size);
        if ret != 0 {
            return ret;
        }

        ret = q6apm_dai_memory_map(component, substream, graph_id, is_push_pull);
        if ret != 0 {
            return ret;
        }
    }

    0
}

unsafe extern "C" fn q6apm_dai_memory_unmap(
    component: *mut snd_soc_component,
    substream: *mut snd_pcm_substream,
) {
    let soc_prtd = snd_soc_substream_to_rtd(substream);
    if soc_prtd.is_null() {
        return;
    }

    let cpu_dai = snd_soc_rtd_to_cpu(soc_prtd, 0);
    if cpu_dai.is_null() {
        return;
    }

    let graph_id = (*(*cpu_dai).driver).id;
    q6apm_unmap_memory_fixed_region((*component).dev, graph_id);

    if q6apm_is_graph_in_push_pull_mode_from_id((*component).dev, graph_id, (*substream).stream) {
        q6apm_unmap_pos_buffer((*component).dev, graph_id);
    }
}

unsafe extern "C" fn q6apm_dai_pcm_free(
    component: *mut snd_soc_component,
    pcm: *mut snd_pcm,
) {
    let mut substream = (*pcm).streams[SNDRV_PCM_STREAM_CAPTURE as usize].substream;
    if !substream.is_null() {
        q6apm_dai_memory_unmap(component, substream);
    }

    substream = (*pcm).streams[SNDRV_PCM_STREAM_PLAYBACK as usize].substream;
    if !substream.is_null() {
        q6apm_dai_memory_unmap(component, substream);
    }
}

unsafe extern "C" fn q6apm_dai_compr_open(
    component: *mut snd_soc_component,
    stream: *mut snd_compr_stream,
) -> i32 {
    let rtd = (*stream).private_data as *mut snd_soc_pcm_runtime;
    let cpu_dai = snd_soc_rtd_to_cpu(rtd, 0);
    let runtime = (*stream).runtime;
    let dev = (*component).dev;
    let graph_id = (*(*cpu_dai).driver).id;
    let pdata = snd_soc_component_get_drvdata(component) as *mut q6apm_dai_data;
    if pdata.is_null() {
        return -EINVAL;
    }

    let prtd = kzalloc(size_of::<q6apm_dai_rtd>(), GFP_KERNEL) as *mut q6apm_dai_rtd;
    if prtd.is_null() {
        return -ENOMEM;
    }

    (*prtd).cstream = stream;
    (*prtd).graph = q6apm_graph_open(
        dev,
        Some(event_handler_compr),
        prtd as *mut c_void,
        graph_id,
        SNDRV_PCM_STREAM_PLAYBACK,
    );
    if IS_ERR((*prtd).graph as *const c_void) {
        let ret = PTR_ERR((*prtd).graph as *const c_void);
        kfree(prtd as *mut c_void);
        return ret;
    }

    (*runtime).private_data = prtd as *mut c_void;
    (*runtime).dma_bytes = BUFFER_BYTES_MAX;
    let size = COMPR_PLAYBACK_MAX_FRAGMENT_SIZE * COMPR_PLAYBACK_MAX_NUM_FRAGMENTS;
    let ret = snd_dma_alloc_pages(SNDRV_DMA_TYPE_DEV, dev, size, &mut (*prtd).dma_buffer);
    if ret != 0 {
        return ret;
    }

    if (*pdata).sid < 0 {
        (*prtd).phys = (*prtd).dma_buffer.addr;
    } else {
        (*prtd).phys = (*prtd).dma_buffer.addr | (((*pdata).sid as phys_addr_t) << 32);
    }

    snd_compr_set_runtime_buffer(stream, &mut (*prtd).dma_buffer);
    spin_lock_init(&mut (*prtd).lock);

    q6apm_enable_compress_module(dev, (*prtd).graph, true);
    0
}

unsafe extern "C" fn q6apm_dai_compr_free(
    component: *mut snd_soc_component,
    stream: *mut snd_compr_stream,
) -> i32 {
    let runtime = (*stream).runtime;
    let prtd = (*runtime).private_data as *mut q6apm_dai_rtd;

    q6apm_graph_stop((*prtd).graph);
    q6apm_free_fragments((*prtd).graph, SNDRV_PCM_STREAM_PLAYBACK);
    q6apm_unmap_memory_fixed_region((*component).dev, (*(*prtd).graph).id);
    q6apm_graph_close((*prtd).graph);
    snd_dma_free_pages(&mut (*prtd).dma_buffer);
    (*prtd).graph = ptr::null_mut();
    kfree(prtd as *mut c_void);
    (*runtime).private_data = ptr::null_mut();

    0
}

unsafe extern "C" fn q6apm_dai_compr_get_caps(
    _component: *mut snd_soc_component,
    _stream: *mut snd_compr_stream,
    caps: *mut snd_compr_caps,
) -> i32 {
    (*caps).direction = SND_COMPRESS_PLAYBACK;
    (*caps).min_fragment_size = COMPR_PLAYBACK_MIN_FRAGMENT_SIZE;
    (*caps).max_fragment_size = COMPR_PLAYBACK_MAX_FRAGMENT_SIZE;
    (*caps).min_fragments = COMPR_PLAYBACK_MIN_NUM_FRAGMENTS;
    (*caps).max_fragments = COMPR_PLAYBACK_MAX_NUM_FRAGMENTS;
    (*caps).num_codecs = 4;
    (*caps).codecs[0] = SND_AUDIOCODEC_MP3;
    (*caps).codecs[1] = SND_AUDIOCODEC_AAC;
    (*caps).codecs[2] = SND_AUDIOCODEC_FLAC;
    (*caps).codecs[3] = SND_AUDIOCODEC_OPUS_RAW;

    0
}

unsafe extern "C" fn q6apm_dai_compr_get_codec_caps(
    _component: *mut snd_soc_component,
    _stream: *mut snd_compr_stream,
    codec: *mut snd_compr_codec_caps,
) -> i32 {
    match (*codec).codec {
        SND_AUDIOCODEC_MP3 => *codec = q6apm_compr_caps,
        _ => {}
    }

    0
}

unsafe extern "C" fn q6apm_dai_compr_pointer(
    _component: *mut snd_soc_component,
    stream: *mut snd_compr_stream,
    tstamp: *mut snd_compr_tstamp64,
) -> i32 {
    let runtime = (*stream).runtime;
    let prtd = (*runtime).private_data as *mut q6apm_dai_rtd;

    let _guard = spinlock_irqsave_guard(&mut (*prtd).lock);
    (*tstamp).copied_total = (*prtd).copied_total;
    let mut temp_copied_total = (*tstamp).copied_total;
    (*tstamp).byte_offset = do_div(&mut temp_copied_total, (*prtd).pcm_size);

    0
}

unsafe extern "C" fn q6apm_dai_compr_trigger(
    _component: *mut snd_soc_component,
    stream: *mut snd_compr_stream,
    cmd: i32,
) -> i32 {
    let runtime = (*stream).runtime;
    let prtd = (*runtime).private_data as *mut q6apm_dai_rtd;
    let mut ret = 0;

    match cmd {
        SNDRV_PCM_TRIGGER_START | SNDRV_PCM_TRIGGER_RESUME | SNDRV_PCM_TRIGGER_PAUSE_RELEASE => {
            ret = q6apm_write_async((*prtd).graph, (*prtd).pcm_count, 0, 0, NO_TIMESTAMP);
        }
        SNDRV_PCM_TRIGGER_STOP => {}
        SNDRV_PCM_TRIGGER_SUSPEND | SNDRV_PCM_TRIGGER_PAUSE_PUSH => {}
        SND_COMPR_TRIGGER_NEXT_TRACK => (*prtd).next_track = true,
        SND_COMPR_TRIGGER_DRAIN | SND_COMPR_TRIGGER_PARTIAL_DRAIN => {
            (*prtd).notify_on_drain = true;
        }
        _ => ret = -EINVAL,
    }

    ret
}

unsafe extern "C" fn q6apm_dai_compr_ack(
    _component: *mut snd_soc_component,
    stream: *mut snd_compr_stream,
    count: usize,
) -> i32 {
    let runtime = (*stream).runtime;
    let prtd = (*runtime).private_data as *mut q6apm_dai_rtd;

    let _guard = spinlock_irqsave_guard(&mut (*prtd).lock);
    (*prtd).bytes_received = (*prtd).bytes_received.wrapping_add(count as u64);

    count as i32
}

unsafe extern "C" fn q6apm_dai_compr_set_params(
    component: *mut snd_soc_component,
    stream: *mut snd_compr_stream,
    params: *mut snd_compr_params,
) -> i32 {
    let runtime = (*stream).runtime;
    let prtd = (*runtime).private_data as *mut q6apm_dai_rtd;
    let pdata = snd_soc_component_get_drvdata(component) as *mut q6apm_dai_data;
    let mut cfg: audioreach_module_config = core::mem::zeroed();
    let codec = &mut (*params).codec as *mut snd_codec;
    let dir = (*stream).direction;

    if pdata.is_null() {
        return -EINVAL;
    }

    (*prtd).periods = (*runtime).fragments;
    (*prtd).pcm_count = (*runtime).fragment_size;
    (*prtd).pcm_size = (*runtime).fragments * (*runtime).fragment_size;
    (*prtd).bits_per_sample = 16;

    if (*prtd).next_track != true {
        ptr::copy_nonoverlapping(codec, &mut (*prtd).codec, 1);

        let mut ret = q6apm_set_real_module_id((*component).dev, (*prtd).graph, (*codec).id);
        if ret != 0 {
            return ret;
        }

        cfg.direction = dir;
        cfg.sample_rate = (*codec).sample_rate;
        cfg.num_channels = 2;
        cfg.bit_width = (*prtd).bits_per_sample;
        cfg.fmt = (*codec).id;
        audioreach_set_default_channel_mapping(cfg.channel_map.as_mut_ptr(), cfg.num_channels);
        ptr::copy_nonoverlapping(codec, &mut cfg.codec, 1);

        ret = q6apm_graph_media_format_shmem((*prtd).graph, &mut cfg);
        if ret < 0 {
            return ret;
        }

        ret = q6apm_graph_media_format_pcm((*prtd).graph, &mut cfg);
        if ret != 0 {
            return ret;
        }

        ret = q6apm_alloc_fragments(
            (*prtd).graph,
            SNDRV_PCM_STREAM_PLAYBACK,
            (*prtd).phys,
            (*prtd).pcm_size / (*prtd).periods,
            (*prtd).periods,
        );
        if ret < 0 {
            return -ENOMEM;
        }

        ret = q6apm_graph_prepare((*prtd).graph);
        if ret != 0 {
            return ret;
        }

        ret = q6apm_graph_start((*prtd).graph);
        if ret != 0 {
            return ret;
        }
    } else {
        cfg.direction = dir;
        cfg.sample_rate = (*codec).sample_rate;
        cfg.num_channels = 2;
        cfg.bit_width = (*prtd).bits_per_sample;
        cfg.fmt = (*codec).id;
        ptr::copy_nonoverlapping(codec, &mut cfg.codec, 1);

        let ret = audioreach_compr_set_param((*prtd).graph, &mut cfg);
        if ret < 0 {
            return ret;
        }
    }
    (*prtd).state = stream_state::Q6APM_STREAM_RUNNING;

    0
}

unsafe extern "C" fn q6apm_dai_compr_set_metadata(
    component: *mut snd_soc_component,
    stream: *mut snd_compr_stream,
    metadata: *mut snd_compr_metadata,
) -> i32 {
    let runtime = (*stream).runtime;
    let prtd = (*runtime).private_data as *mut q6apm_dai_rtd;
    let mut ret = 0;

    match (*metadata).key {
        SNDRV_COMPRESS_ENCODER_PADDING => {
            q6apm_remove_trailing_silence((*component).dev, (*prtd).graph, (*metadata).value[0]);
        }
        SNDRV_COMPRESS_ENCODER_DELAY => {
            q6apm_remove_initial_silence((*component).dev, (*prtd).graph, (*metadata).value[0]);
        }
        _ => ret = -EINVAL,
    }

    ret
}

unsafe extern "C" fn q6apm_dai_compr_mmap(
    component: *mut snd_soc_component,
    stream: *mut snd_compr_stream,
    vma: *mut vm_area_struct,
) -> i32 {
    let runtime = (*stream).runtime;
    let prtd = (*runtime).private_data as *mut q6apm_dai_rtd;
    let dev = (*component).dev;

    dma_mmap_coherent(
        dev,
        vma,
        (*prtd).dma_buffer.area,
        (*prtd).dma_buffer.addr,
        (*prtd).dma_buffer.bytes,
    )
}

unsafe extern "C" fn q6apm_compr_copy(
    _component: *mut snd_soc_component,
    stream: *mut snd_compr_stream,
    buf: *mut c_char,
    count: usize,
) -> i32 {
    let runtime = (*stream).runtime;
    let prtd = (*runtime).private_data as *mut q6apm_dai_rtd;
    let wflags: u32 = 0;
    let mut bytes_received = (*prtd).bytes_received;
    let mut temp_bytes_received = bytes_received;

    /**
     * Make sure that next track data pointer is aligned at 32 bit boundary
     * This is a Mandatory requirement from DSP data buffers alignment
     */
    if (*prtd).next_track {
        bytes_received = ALIGN((*prtd).bytes_received, (*prtd).pcm_count as u64);
        temp_bytes_received = bytes_received;
    }

    let app_pointer = do_div(&mut temp_bytes_received, (*prtd).pcm_size);
    let dstn = (*prtd).dma_buffer.area.add(app_pointer as usize);

    if count < ((*prtd).pcm_size - app_pointer) as usize {
        if copy_from_user(dstn, buf as *const c_void, count) != 0 {
            return -EFAULT;
        }
    } else {
        let copy = ((*prtd).pcm_size - app_pointer) as usize;
        if copy_from_user(dstn, buf as *const c_void, copy) != 0 {
            return -EFAULT;
        }
        if copy_from_user(
            (*prtd).dma_buffer.area,
            buf.add(copy) as *const c_void,
            count - copy,
        ) != 0
        {
            return -EFAULT;
        }
    }

    let _guard = spinlock_irqsave_guard(&mut (*prtd).lock);
    let mut bytes_in_flight = (*prtd).bytes_received.wrapping_sub((*prtd).copied_total);

    if (*prtd).next_track {
        (*prtd).next_track = false;
        (*prtd).copied_total = ALIGN((*prtd).copied_total, (*prtd).pcm_count as u64);
        (*prtd).bytes_sent = ALIGN((*prtd).bytes_sent, (*prtd).pcm_count as u64);
    }

    (*prtd).bytes_received = bytes_received.wrapping_add(count as u64);

    /* Kick off the data to dsp if its starving!! */
    if (*prtd).state == stream_state::Q6APM_STREAM_RUNNING && bytes_in_flight == 0 {
        let mut bytes_to_write = (*prtd).pcm_count;
        let avail = (*prtd).bytes_received.wrapping_sub((*prtd).bytes_sent);

        if avail < (*prtd).pcm_count as u64 {
            bytes_to_write = avail as u32;
        }

        q6apm_write_async((*prtd).graph, bytes_to_write, 0, 0, wflags);
        (*prtd).bytes_sent = (*prtd).bytes_sent.wrapping_add(bytes_to_write as u64);
    }

    count as i32
}

static q6apm_dai_compress_ops: snd_compress_ops = snd_compress_ops {
    open: Some(q6apm_dai_compr_open),
    free: Some(q6apm_dai_compr_free),
    get_caps: Some(q6apm_dai_compr_get_caps),
    get_codec_caps: Some(q6apm_dai_compr_get_codec_caps),
    pointer: Some(q6apm_dai_compr_pointer),
    trigger: Some(q6apm_dai_compr_trigger),
    ack: Some(q6apm_dai_compr_ack),
    set_params: Some(q6apm_dai_compr_set_params),
    set_metadata: Some(q6apm_dai_compr_set_metadata),
    mmap: Some(q6apm_dai_compr_mmap),
    copy: Some(q6apm_compr_copy),
};

static q6apm_fe_dai_component: snd_soc_component_driver = snd_soc_component_driver {
    name: DRV_NAME.as_ptr() as *const c_char,
    open: Some(q6apm_dai_open),
    close: Some(q6apm_dai_close),
    prepare: Some(q6apm_dai_prepare),
    pcm_new: Some(q6apm_dai_pcm_new),
    pcm_free: Some(q6apm_dai_pcm_free),
    hw_params: Some(q6apm_dai_hw_params),
    pointer: Some(q6apm_dai_pointer),
    trigger: Some(q6apm_dai_trigger),
    ack: Some(q6apm_dai_ack),
    compress_ops: &q6apm_dai_compress_ops,
    use_dai_pcm_id: true,
    remove_order: SND_SOC_COMP_ORDER_EARLY,
};

unsafe extern "C" fn q6apm_dai_probe(pdev: *mut platform_device) -> i32 {
    let dev = &mut (*pdev).dev as *mut device;
    let node = (*dev).of_node;
    let pdata = devm_kzalloc(dev, size_of::<q6apm_dai_data>(), GFP_KERNEL) as *mut q6apm_dai_data;
    let mut args: of_phandle_args = core::mem::zeroed();

    if pdata.is_null() {
        return -ENOMEM;
    }

    let rc = of_parse_phandle_with_fixed_args(node, c"iommus".as_ptr(), 1, 0, &mut args);
    if rc < 0 {
        (*pdata).sid = -1;
    } else {
        (*pdata).sid = (args.args[0] & SID_MASK_DEFAULT) as i64;
    }

    dev_set_drvdata(dev, pdata as *mut c_void);

    devm_snd_soc_register_component(dev, &q6apm_fe_dai_component, ptr::null_mut(), 0)
}

// CONFIG_OF: Open Firmware device match table.
static q6apm_dai_device_id: [of_device_id; 2] = [
    of_device_id {
        compatible: c"qcom,q6apm-dais".as_ptr(),
        ..of_device_id::zeroed()
    },
    of_device_id::zeroed(),
];
// MODULE_DEVICE_TABLE(of, q6apm_dai_device_id);

static mut q6apm_dai_platform_driver: platform_driver = platform_driver {
    driver: device_driver {
        name: c"q6apm-dai".as_ptr(),
        of_match_table: of_match_ptr(q6apm_dai_device_id.as_ptr()),
        ..device_driver::zeroed()
    },
    probe: Some(q6apm_dai_probe),
    ..platform_driver::zeroed()
};

// module_platform_driver(q6apm_dai_platform_driver);
module_platform_driver!(q6apm_dai_platform_driver);

module_description!("Q6APM dai driver");
module_license!("GPL");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
