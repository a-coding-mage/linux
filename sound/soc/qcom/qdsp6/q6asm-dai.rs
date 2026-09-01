// SPDX-License-Identifier: GPL-2.0
// Copyright (c) 2011-2017, The Linux Foundation. All rights reserved.
// Copyright (c) 2018, Linaro Limited

// Rust translation of soc/qcom/qdsp6/q6asm-dai.c.
// C include dependencies intentionally remain external to this translation:
// dt-bindings/sound/qcom,q6asm.h, Linux device/platform/ASoC/PCM/compress,
// q6asm.h, q6routing.h, and q6dsp-errno.h.

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_longlong, c_uint, c_ulonglong, c_void};
use core::mem::{size_of, zeroed};
use core::ptr::{null, null_mut};

type u32 = u32;
type size_t = usize;
type phys_addr_t = u64;
type snd_pcm_uframes_t = u64;
type bool_ = bool;

const DRV_NAME: &[u8] = b"q6asm-fe-dai\0";

const PLAYBACK_MIN_NUM_PERIODS: u32 = 2;
const PLAYBACK_MAX_NUM_PERIODS: u32 = 8;
const PLAYBACK_MAX_PERIOD_SIZE: u32 = 65536;
const PLAYBACK_MIN_PERIOD_SIZE: u32 = 128;
const CAPTURE_MIN_NUM_PERIODS: u32 = 2;
const CAPTURE_MAX_NUM_PERIODS: u32 = 8;
const CAPTURE_MAX_PERIOD_SIZE: u32 = 4096;
const CAPTURE_MIN_PERIOD_SIZE: u32 = 320;
const SID_MASK_DEFAULT: c_longlong = 0xF;

/* Default values used if user space does not set */
const COMPR_PLAYBACK_MIN_FRAGMENT_SIZE: u32 = 8 * 1024;
const COMPR_PLAYBACK_MAX_FRAGMENT_SIZE: u32 = 128 * 1024;
const COMPR_PLAYBACK_MIN_NUM_FRAGMENTS: u32 = 4;
const COMPR_PLAYBACK_MAX_NUM_FRAGMENTS: u32 = 16 * 4;

const ALAC_CH_LAYOUT_MONO: u32 = (101 << 16) | 1;
const ALAC_CH_LAYOUT_STEREO: u32 = (101 << 16) | 2;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
enum stream_state {
    Q6ASM_STREAM_IDLE = 0,
    Q6ASM_STREAM_STOPPED,
    Q6ASM_STREAM_RUNNING,
}

#[repr(C)]
struct q6asm_dai_rtd {
    substream: *mut snd_pcm_substream,
    cstream: *mut snd_compr_stream,
    codec: snd_codec,
    dma_buffer: snd_dma_buffer,
    lock: spinlock_t,
    phys: phys_addr_t,
    pcm_size: c_uint,
    pcm_count: c_uint,
    periods: c_uint,
    bytes_sent: u64,
    bytes_received: u64,
    copied_total: u64,
    bits_per_sample: u16,
    queue_ptr: snd_pcm_uframes_t,
    source: u16, /* Encoding source bit mask */
    audio_client: *mut audio_client,
    next_track_stream_id: u32,
    next_track: bool_,
    stream_id: u32,
    session_id: u16,
    state: stream_state,
    initial_samples_drop: u32,
    trailing_samples_drop: u32,
    notify_on_drain: bool_,
}

#[repr(C)]
struct q6asm_dai_data {
    dais: *mut snd_soc_dai_driver,
    num_dais: c_int,
    sid: c_longlong,
}

static q6asm_dai_hardware_capture: snd_pcm_hardware = snd_pcm_hardware {
    info: SNDRV_PCM_INFO_MMAP | SNDRV_PCM_INFO_BATCH |
        SNDRV_PCM_INFO_BLOCK_TRANSFER |
        SNDRV_PCM_INFO_NO_REWINDS | SNDRV_PCM_INFO_SYNC_APPLPTR |
        SNDRV_PCM_INFO_MMAP_VALID |
        SNDRV_PCM_INFO_INTERLEAVED |
        SNDRV_PCM_INFO_PAUSE | SNDRV_PCM_INFO_RESUME,
    formats: SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S24_LE,
    rates: SNDRV_PCM_RATE_8000_48000,
    rate_min: 8000,
    rate_max: 48000,
    channels_min: 1,
    channels_max: 4,
    buffer_bytes_max: CAPTURE_MAX_NUM_PERIODS * CAPTURE_MAX_PERIOD_SIZE,
    period_bytes_min: CAPTURE_MIN_PERIOD_SIZE,
    period_bytes_max: CAPTURE_MAX_PERIOD_SIZE,
    periods_min: CAPTURE_MIN_NUM_PERIODS,
    periods_max: CAPTURE_MAX_NUM_PERIODS,
    fifo_size: 0,
};

static q6asm_dai_hardware_playback: snd_pcm_hardware = snd_pcm_hardware {
    info: SNDRV_PCM_INFO_MMAP | SNDRV_PCM_INFO_BATCH |
        SNDRV_PCM_INFO_BLOCK_TRANSFER |
        SNDRV_PCM_INFO_MMAP_VALID |
        SNDRV_PCM_INFO_NO_REWINDS | SNDRV_PCM_INFO_SYNC_APPLPTR |
        SNDRV_PCM_INFO_INTERLEAVED |
        SNDRV_PCM_INFO_PAUSE | SNDRV_PCM_INFO_RESUME,
    formats: SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S24_LE,
    rates: SNDRV_PCM_RATE_8000_192000,
    rate_min: 8000,
    rate_max: 192000,
    channels_min: 1,
    channels_max: 8,
    buffer_bytes_max: PLAYBACK_MAX_NUM_PERIODS * PLAYBACK_MAX_PERIOD_SIZE,
    period_bytes_min: PLAYBACK_MIN_PERIOD_SIZE,
    period_bytes_max: PLAYBACK_MAX_PERIOD_SIZE,
    periods_min: PLAYBACK_MIN_NUM_PERIODS,
    periods_max: PLAYBACK_MAX_NUM_PERIODS,
    fifo_size: 0,
};

const fn Q6ASM_FEDAI_DRIVER(num: usize, id: c_int) -> snd_soc_dai_driver {
    snd_soc_dai_driver {
        playback: snd_soc_pcm_stream {
            stream_name: MULTIMEDIA_PLAYBACK_NAMES[num].as_ptr() as *const c_char,
            rates: SNDRV_PCM_RATE_8000_48000 | SNDRV_PCM_RATE_12000 |
                SNDRV_PCM_RATE_24000 | SNDRV_PCM_RATE_88200 |
                SNDRV_PCM_RATE_96000 | SNDRV_PCM_RATE_176400 |
                SNDRV_PCM_RATE_192000,
            formats: SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S24_LE,
            channels_min: 1,
            channels_max: 8,
            rate_min: 8000,
            rate_max: 192000,
        },
        capture: snd_soc_pcm_stream {
            stream_name: MULTIMEDIA_CAPTURE_NAMES[num].as_ptr() as *const c_char,
            rates: SNDRV_PCM_RATE_8000_48000 | SNDRV_PCM_RATE_12000 |
                SNDRV_PCM_RATE_24000,
            formats: SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S24_LE,
            channels_min: 1,
            channels_max: 4,
            rate_min: 8000,
            rate_max: 48000,
        },
        name: MULTIMEDIA_NAMES[num].as_ptr() as *const c_char,
        id,
        ops: null(),
    }
}

static q6asm_compr_caps: snd_compr_codec_caps = snd_compr_codec_caps {
    num_descriptors: 1,
    descriptor: [snd_codec_desc {
        max_ch: 2,
        sample_rates: [8000, 11025, 12000, 16000, 22050, 24000, 32000, 44100,
            48000, 88200, 96000, 176400, 192000],
        num_sample_rates: 13,
        bit_rate: [320, 128],
        num_bitrates: 2,
        profiles: 0,
        modes: SND_AUDIOCHANMODE_MP3_STEREO,
        formats: 0,
    }],
    codec: 0,
};

unsafe extern "C" fn event_handler(opcode: u32, _token: u32, _payload: *mut c_void, priv_: *mut c_void) {
    let prtd = priv_ as *mut q6asm_dai_rtd;
    let substream = (*prtd).substream;

    match opcode {
        ASM_CLIENT_EVENT_CMD_RUN_DONE => {}
        ASM_CLIENT_EVENT_CMD_EOS_DONE => {}
        ASM_CLIENT_EVENT_DATA_WRITE_DONE => {
            snd_pcm_period_elapsed(substream);
        }
        ASM_CLIENT_EVENT_DATA_READ_DONE => {
            snd_pcm_period_elapsed(substream);
            if (*prtd).state == stream_state::Q6ASM_STREAM_RUNNING {
                q6asm_read((*prtd).audio_client, (*prtd).stream_id);
            }
        }
        _ => {}
    }
}

unsafe extern "C" fn q6asm_dai_prepare(component: *mut snd_soc_component,
                                       substream: *mut snd_pcm_substream) -> c_int {
    let runtime = (*substream).runtime;
    let soc_prtd = snd_soc_substream_to_rtd(substream);
    let prtd = (*runtime).private_data as *mut q6asm_dai_rtd;
    let pdata: *mut q6asm_dai_data;
    let dev = (*component).dev;
    let mut ret: c_int;

    pdata = snd_soc_component_get_drvdata(component) as *mut q6asm_dai_data;
    if pdata.is_null() {
        return -EINVAL;
    }

    if prtd.is_null() || (*prtd).audio_client.is_null() {
        dev_err(dev, c"%s: private data null or audio client freed\n".as_ptr(), c"q6asm_dai_prepare".as_ptr());
        return -EINVAL;
    }

    (*prtd).pcm_count = snd_pcm_lib_period_bytes(substream);
    /* rate and channels are sent to audio driver */
    if (*prtd).state == stream_state::Q6ASM_STREAM_RUNNING {
        /* clear the previous setup if any  */
        ret = q6asm_cmd((*prtd).audio_client, (*prtd).stream_id, CMD_CLOSE);
        if ret < 0 {
            dev_err(dev, c"Failed to close q6asm stream %d\n".as_ptr(), (*prtd).stream_id);
            return ret;
        }

        ret = q6asm_unmap_memory_regions((*substream).stream, (*prtd).audio_client);
        if ret < 0 {
            dev_err(dev, c"Failed to unmap memory regions for q6asm stream %d\n".as_ptr(), (*prtd).stream_id);
            return ret;
        }

        q6routing_stream_close((*(*soc_prtd).dai_link).id, (*substream).stream);
        (*prtd).state = stream_state::Q6ASM_STREAM_STOPPED;
    }

    ret = q6asm_map_memory_regions((*substream).stream, (*prtd).audio_client,
                                   (*prtd).phys, (*prtd).pcm_size / (*prtd).periods,
                                   (*prtd).periods);
    if ret < 0 {
        dev_err(dev, c"Audio Start: Buffer Allocation failed rc = %d\n".as_ptr(), ret);
        return -ENOMEM;
    }

    if (*substream).stream == SNDRV_PCM_STREAM_PLAYBACK {
        ret = q6asm_open_write((*prtd).audio_client, (*prtd).stream_id,
                               FORMAT_LINEAR_PCM, 0, (*prtd).bits_per_sample, false);
    } else if (*substream).stream == SNDRV_PCM_STREAM_CAPTURE {
        ret = q6asm_open_read((*prtd).audio_client, (*prtd).stream_id,
                              FORMAT_LINEAR_PCM, (*prtd).bits_per_sample);
    } else {
        ret = 0;
    }

    if ret < 0 {
        dev_err(dev, c"%s: q6asm_open_write failed\n".as_ptr(), c"q6asm_dai_prepare".as_ptr());
        q6asm_unmap_memory_regions((*substream).stream, (*prtd).audio_client);
        return ret;
    }

    (*prtd).session_id = q6asm_get_session_id((*prtd).audio_client);
    ret = q6routing_stream_open((*(*soc_prtd).dai_link).id, LEGACY_PCM_MODE,
                                (*prtd).session_id, (*substream).stream);
    if ret != 0 {
        dev_err(dev, c"%s: stream reg failed ret:%d\n".as_ptr(), c"q6asm_dai_prepare".as_ptr(), ret);
        q6asm_cmd((*prtd).audio_client, (*prtd).stream_id, CMD_CLOSE);
        q6asm_unmap_memory_regions((*substream).stream, (*prtd).audio_client);
        return ret;
    }

    if (*substream).stream == SNDRV_PCM_STREAM_PLAYBACK {
        ret = q6asm_media_format_block_multi_ch_pcm((*prtd).audio_client, (*prtd).stream_id,
                                                    (*runtime).rate, (*runtime).channels,
                                                    null_mut(), (*prtd).bits_per_sample);
    } else if (*substream).stream == SNDRV_PCM_STREAM_CAPTURE {
        ret = q6asm_enc_cfg_blk_pcm_format_support((*prtd).audio_client, (*prtd).stream_id,
                                                   (*runtime).rate, (*runtime).channels,
                                                   (*prtd).bits_per_sample);
        for _i in 0..(*runtime).periods {
            q6asm_read((*prtd).audio_client, (*prtd).stream_id);
        }
    } else {
        ret = 0;
    }

    if ret < 0 {
        dev_info(dev, c"%s: CMD Format block failed\n".as_ptr(), c"q6asm_dai_prepare".as_ptr());
    } else {
        (*prtd).state = stream_state::Q6ASM_STREAM_RUNNING;
    }
    ret
}

unsafe extern "C" fn q6asm_dai_ack(component: *mut snd_soc_component,
                                   substream: *mut snd_pcm_substream) -> c_int {
    let runtime = (*substream).runtime;
    let prtd = (*runtime).private_data as *mut q6asm_dai_rtd;
    let mut ret: c_int = 0;
    if (*substream).stream == SNDRV_PCM_STREAM_PLAYBACK &&
        (*prtd).state == stream_state::Q6ASM_STREAM_RUNNING {
        let avail_periods = ((*(*runtime).control).appl_ptr - (*prtd).queue_ptr) / (*runtime).period_size;
        for _i in 0..avail_periods {
            ret = q6asm_write_async((*prtd).audio_client, (*prtd).stream_id,
                                    (*prtd).pcm_count, 0, 0, 0);
            if ret < 0 {
                dev_err((*component).dev, c"Error queuing playback buffer %d\n".as_ptr(), ret);
                return ret;
            }
            (*prtd).queue_ptr += (*runtime).period_size;
        }
    }
    ret
}

unsafe extern "C" fn q6asm_dai_trigger(_component: *mut snd_soc_component,
                                       substream: *mut snd_pcm_substream,
                                       cmd: c_int) -> c_int {
    let runtime = (*substream).runtime;
    let prtd = (*runtime).private_data as *mut q6asm_dai_rtd;
    match cmd {
        SNDRV_PCM_TRIGGER_START | SNDRV_PCM_TRIGGER_RESUME | SNDRV_PCM_TRIGGER_PAUSE_RELEASE =>
            q6asm_run_nowait((*prtd).audio_client, (*prtd).stream_id, 0, 0, 0),
        SNDRV_PCM_TRIGGER_STOP =>
            q6asm_cmd_nowait((*prtd).audio_client, (*prtd).stream_id, CMD_EOS),
        SNDRV_PCM_TRIGGER_SUSPEND | SNDRV_PCM_TRIGGER_PAUSE_PUSH =>
            q6asm_cmd_nowait((*prtd).audio_client, (*prtd).stream_id, CMD_PAUSE),
        _ => -EINVAL,
    }
}

unsafe extern "C" fn q6asm_dai_open(component: *mut snd_soc_component,
                                    substream: *mut snd_pcm_substream) -> c_int {
    let runtime = (*substream).runtime;
    let soc_prtd = snd_soc_substream_to_rtd(substream);
    let cpu_dai = snd_soc_rtd_to_cpu(soc_prtd, 0);
    let dev = (*component).dev;
    let mut ret: c_int = 0;
    let stream_id = (*(*cpu_dai).driver).id;
    let pdata = snd_soc_component_get_drvdata(component) as *mut q6asm_dai_data;
    if pdata.is_null() {
        dev_err(dev, c"Drv data not found ..\n".as_ptr());
        return -EINVAL;
    }

    let prtd = kzalloc(size_of::<q6asm_dai_rtd>(), GFP_KERNEL) as *mut q6asm_dai_rtd;
    if prtd.is_null() {
        return -ENOMEM;
    }
    (*prtd).substream = substream;
    (*prtd).audio_client = q6asm_audio_client_alloc(dev, Some(event_handler), prtd as *mut c_void,
                                                    stream_id, LEGACY_PCM_MODE);
    if IS_ERR((*prtd).audio_client as *const c_void) {
        dev_info(dev, c"%s: Could not allocate memory\n".as_ptr(), c"q6asm_dai_open".as_ptr());
        ret = PTR_ERR((*prtd).audio_client as *const c_void);
        kfree(prtd as *mut c_void);
        return ret;
    }

    /* DSP expects stream id from 1 */
    (*prtd).stream_id = 1;

    if (*substream).stream == SNDRV_PCM_STREAM_PLAYBACK {
        (*runtime).hw = q6asm_dai_hardware_playback;
    } else if (*substream).stream == SNDRV_PCM_STREAM_CAPTURE {
        (*runtime).hw = q6asm_dai_hardware_capture;
    }

    /* Ensure that buffer size is a multiple of period size */
    ret = snd_pcm_hw_constraint_integer(runtime, SNDRV_PCM_HW_PARAM_PERIODS);
    if ret < 0 {
        dev_info(dev, c"snd_pcm_hw_constraint_integer failed\n".as_ptr());
    }

    if (*substream).stream == SNDRV_PCM_STREAM_PLAYBACK {
        ret = snd_pcm_hw_constraint_minmax(runtime, SNDRV_PCM_HW_PARAM_BUFFER_BYTES,
            PLAYBACK_MIN_NUM_PERIODS * PLAYBACK_MIN_PERIOD_SIZE,
            PLAYBACK_MAX_NUM_PERIODS * PLAYBACK_MAX_PERIOD_SIZE);
        if ret < 0 {
            dev_err(dev, c"constraint for buffer bytes min max ret = %d\n".as_ptr(), ret);
        }
    }

    ret = snd_pcm_hw_constraint_step(runtime, 0, SNDRV_PCM_HW_PARAM_PERIOD_SIZE, 480);
    if ret < 0 {
        dev_err(dev, c"constraint for period bytes step ret = %d\n".as_ptr(), ret);
    }
    ret = snd_pcm_hw_constraint_step(runtime, 0, SNDRV_PCM_HW_PARAM_BUFFER_SIZE, 480);
    if ret < 0 {
        dev_err(dev, c"constraint for buffer bytes step ret = %d\n".as_ptr(), ret);
    }

    (*runtime).private_data = prtd as *mut c_void;

    if (*substream).stream == SNDRV_PCM_STREAM_PLAYBACK {
        snd_soc_set_runtime_hwparams(substream, &q6asm_dai_hardware_playback);
        (*runtime).dma_bytes = q6asm_dai_hardware_playback.buffer_bytes_max;
    } else {
        snd_soc_set_runtime_hwparams(substream, &q6asm_dai_hardware_capture);
        (*runtime).dma_bytes = q6asm_dai_hardware_capture.buffer_bytes_max;
    }

    if (*pdata).sid < 0 {
        (*prtd).phys = (*substream).dma_buffer.addr;
    } else {
        (*prtd).phys = (*substream).dma_buffer.addr | (((*pdata).sid as u64) << 32);
    }
    0
}

unsafe extern "C" fn q6asm_dai_close(_component: *mut snd_soc_component,
                                     substream: *mut snd_pcm_substream) -> c_int {
    let runtime = (*substream).runtime;
    let soc_prtd = snd_soc_substream_to_rtd(substream);
    let prtd = (*runtime).private_data as *mut q6asm_dai_rtd;
    if !(*prtd).audio_client.is_null() {
        if (*prtd).state == stream_state::Q6ASM_STREAM_RUNNING {
            q6asm_cmd((*prtd).audio_client, (*prtd).stream_id, CMD_CLOSE);
            q6asm_unmap_memory_regions((*substream).stream, (*prtd).audio_client);
        }
        q6asm_audio_client_free((*prtd).audio_client);
        (*prtd).audio_client = null_mut();
    }
    q6routing_stream_close((*(*soc_prtd).dai_link).id, (*substream).stream);
    kfree(prtd as *mut c_void);
    0
}

unsafe extern "C" fn q6asm_dai_pointer(_component: *mut snd_soc_component,
                                       substream: *mut snd_pcm_substream) -> snd_pcm_uframes_t {
    let runtime = (*substream).runtime;
    let prtd = (*runtime).private_data as *mut q6asm_dai_rtd;
    let ptr = q6asm_get_hw_pointer((*prtd).audio_client, (*substream).stream) * (*runtime).period_size;
    if ptr != 0 { ptr - 1 } else { 0 }
}

unsafe extern "C" fn q6asm_dai_hw_params(_component: *mut snd_soc_component,
                                         substream: *mut snd_pcm_substream,
                                         params: *mut snd_pcm_hw_params) -> c_int {
    let runtime = (*substream).runtime;
    let prtd = (*runtime).private_data as *mut q6asm_dai_rtd;
    (*prtd).pcm_size = params_buffer_bytes(params);
    (*prtd).periods = params_periods(params);
    match params_format(params) {
        SNDRV_PCM_FORMAT_S16_LE => (*prtd).bits_per_sample = 16,
        SNDRV_PCM_FORMAT_S24_LE => (*prtd).bits_per_sample = 24,
        _ => {}
    }
    0
}

unsafe extern "C" fn compress_event_handler(opcode: u32, token: u32,
                                            _payload: *mut c_void, priv_: *mut c_void) {
    let prtd = priv_ as *mut q6asm_dai_rtd;
    let substream = (*prtd).cstream;
    let mut wflags: u32 = 0;
    let mut is_last_buffer = false;

    spin_lock_irqsave(&mut (*prtd).lock);
    match opcode {
        ASM_CLIENT_EVENT_CMD_RUN_DONE => {
            if (*prtd).bytes_sent == 0 {
                q6asm_stream_remove_initial_silence((*prtd).audio_client, (*prtd).stream_id,
                                                    (*prtd).initial_samples_drop);
                q6asm_write_async((*prtd).audio_client, (*prtd).stream_id, (*prtd).pcm_count, 0, 0, 0);
                (*prtd).bytes_sent += (*prtd).pcm_count as u64;
            }
        }
        ASM_CLIENT_EVENT_CMD_EOS_DONE => {
            if (*prtd).notify_on_drain {
                if (*substream).partial_drain {
                    /*
                     * Close old stream and make it stale, switch
                     * the active stream now!
                     */
                    q6asm_cmd_nowait((*prtd).audio_client, (*prtd).stream_id, CMD_CLOSE);
                    /*
                     * vaild stream ids start from 1, So we are
                     * toggling this between 1 and 2.
                     */
                    (*prtd).stream_id = if (*prtd).stream_id == 1 { 2 } else { 1 };
                }
                snd_compr_drain_notify((*prtd).cstream);
                (*prtd).notify_on_drain = false;
            }
        }
        ASM_CLIENT_EVENT_DATA_WRITE_DONE => {
            let bytes_written = token >> ASM_WRITE_TOKEN_LEN_SHIFT;
            (*prtd).copied_total += bytes_written as u64;
            snd_compr_fragment_elapsed(substream);
            if (*prtd).state != stream_state::Q6ASM_STREAM_RUNNING {
                spin_unlock_irqrestore(&mut (*prtd).lock);
                return;
            }
            let avail = (*prtd).bytes_received - (*prtd).bytes_sent;
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
                    wflags |= ASM_LAST_BUFFER_FLAG;
                    q6asm_stream_remove_trailing_silence((*prtd).audio_client, (*prtd).stream_id,
                                                         (*prtd).trailing_samples_drop);
                }
                q6asm_write_async((*prtd).audio_client, (*prtd).stream_id, bytes_to_write, 0, 0, wflags);
                (*prtd).bytes_sent += bytes_to_write as u64;
            }
            if (*prtd).notify_on_drain && is_last_buffer {
                q6asm_cmd_nowait((*prtd).audio_client, (*prtd).stream_id, CMD_EOS);
            }
        }
        _ => {}
    }
    spin_unlock_irqrestore(&mut (*prtd).lock);
}

/* Remaining driver operations translated directly from C. */

unsafe extern "C" fn q6asm_dai_compr_open(component: *mut snd_soc_component,
                                          stream: *mut snd_compr_stream) -> c_int {
    let rtd = (*stream).private_data as *mut snd_soc_pcm_runtime;
    let runtime = (*stream).runtime;
    let cpu_dai = snd_soc_rtd_to_cpu(rtd, 0);
    let dev = (*component).dev;
    let stream_id = (*(*cpu_dai).driver).id;
    let pdata = snd_soc_component_get_drvdata(component) as *mut q6asm_dai_data;
    if pdata.is_null() {
        dev_err(dev, c"Drv data not found ..\n".as_ptr());
        return -EINVAL;
    }
    let prtd = kzalloc(size_of::<q6asm_dai_rtd>(), GFP_KERNEL) as *mut q6asm_dai_rtd;
    if prtd.is_null() {
        return -ENOMEM;
    }
    (*prtd).stream_id = 1;
    (*prtd).cstream = stream;
    (*prtd).audio_client = q6asm_audio_client_alloc(dev, Some(compress_event_handler),
                                                    prtd as *mut c_void, stream_id, LEGACY_PCM_MODE);
    if IS_ERR((*prtd).audio_client as *const c_void) {
        dev_err(dev, c"Could not allocate memory\n".as_ptr());
        let ret = PTR_ERR((*prtd).audio_client as *const c_void);
        kfree(prtd as *mut c_void);
        return ret;
    }
    let size = COMPR_PLAYBACK_MAX_FRAGMENT_SIZE * COMPR_PLAYBACK_MAX_NUM_FRAGMENTS;
    let mut ret = snd_dma_alloc_pages(SNDRV_DMA_TYPE_DEV, dev, size as usize, &mut (*prtd).dma_buffer);
    if ret != 0 {
        dev_err(dev, c"Cannot allocate buffer(s)\n".as_ptr());
        q6asm_audio_client_free((*prtd).audio_client);
        kfree(prtd as *mut c_void);
        return ret;
    }
    if (*pdata).sid < 0 {
        (*prtd).phys = (*prtd).dma_buffer.addr;
    } else {
        (*prtd).phys = (*prtd).dma_buffer.addr | (((*pdata).sid as u64) << 32);
    }
    snd_compr_set_runtime_buffer(stream, &mut (*prtd).dma_buffer);
    spin_lock_init(&mut (*prtd).lock);
    (*runtime).private_data = prtd as *mut c_void;
    ret = 0;
    ret
}

unsafe extern "C" fn q6asm_dai_compr_free(_component: *mut snd_soc_component,
                                          stream: *mut snd_compr_stream) -> c_int {
    let runtime = (*stream).runtime;
    let prtd = (*runtime).private_data as *mut q6asm_dai_rtd;
    let rtd = (*stream).private_data as *mut snd_soc_pcm_runtime;
    if !(*prtd).audio_client.is_null() {
        if (*prtd).state == stream_state::Q6ASM_STREAM_RUNNING {
            q6asm_cmd((*prtd).audio_client, (*prtd).stream_id, CMD_CLOSE);
            if (*prtd).next_track_stream_id != 0 {
                q6asm_cmd((*prtd).audio_client, (*prtd).next_track_stream_id, CMD_CLOSE);
            }
            q6asm_unmap_memory_regions((*stream).direction, (*prtd).audio_client);
        }
        snd_dma_free_pages(&mut (*prtd).dma_buffer);
        q6asm_audio_client_free((*prtd).audio_client);
        (*prtd).audio_client = null_mut();
    }
    q6routing_stream_close((*(*rtd).dai_link).id, (*stream).direction);
    kfree(prtd as *mut c_void);
    0
}

unsafe extern "C" fn __q6asm_dai_compr_set_codec_params(component: *mut snd_soc_component,
    stream: *mut snd_compr_stream, codec: *mut snd_codec, stream_id: c_int) -> c_int {
    let runtime = (*stream).runtime;
    let prtd = (*runtime).private_data as *mut q6asm_dai_rtd;
    let dev = (*component).dev;
    let codec_options = &mut (*prtd).codec.options as *mut snd_codec_options;
    core::ptr::copy_nonoverlapping(codec, &mut (*prtd).codec, 1);
    match (*codec).id {
        SND_AUDIOCODEC_FLAC => {
            let mut flac_cfg: q6asm_flac_cfg = zeroed();
            let flac = &(*codec_options).flac_d;
            flac_cfg.ch_cfg = (*codec).ch_in;
            flac_cfg.sample_rate = (*codec).sample_rate;
            flac_cfg.stream_info_present = 1;
            flac_cfg.sample_size = flac.sample_size;
            flac_cfg.min_blk_size = flac.min_blk_size;
            flac_cfg.max_blk_size = flac.max_blk_size;
            flac_cfg.max_frame_size = flac.max_frame_size;
            flac_cfg.min_frame_size = flac.min_frame_size;
            let ret = q6asm_stream_media_format_block_flac((*prtd).audio_client, stream_id, &mut flac_cfg);
            if ret < 0 {
                dev_err(dev, c"FLAC CMD Format block failed:%d\n".as_ptr(), ret);
                return -EIO;
            }
        }
        SND_AUDIOCODEC_WMA => {
            let mut wma_cfg: q6asm_wma_cfg = zeroed();
            let mut wma_v9: c_uint = 0;
            let wma = &(*codec_options).wma_d;
            wma_cfg.sample_rate = (*codec).sample_rate;
            wma_cfg.num_channels = (*codec).ch_in;
            wma_cfg.bytes_per_sec = (*codec).bit_rate / 8;
            wma_cfg.block_align = (*codec).align;
            wma_cfg.bits_per_sample = (*prtd).bits_per_sample;
            wma_cfg.enc_options = wma.encoder_option;
            wma_cfg.adv_enc_options = wma.adv_encoder_option;
            wma_cfg.adv_enc_options2 = wma.adv_encoder_option2;
            if wma_cfg.num_channels == 1 {
                wma_cfg.channel_mask = 4; /* Mono Center */
            } else if wma_cfg.num_channels == 2 {
                wma_cfg.channel_mask = 3; /* Stereo FL/FR */
            } else {
                return -EINVAL;
            }
            match (*codec).profile {
                SND_AUDIOPROFILE_WMA9 => { wma_cfg.fmtag = 0x161; wma_v9 = 1; }
                SND_AUDIOPROFILE_WMA10 => wma_cfg.fmtag = 0x166,
                SND_AUDIOPROFILE_WMA9_PRO => wma_cfg.fmtag = 0x162,
                SND_AUDIOPROFILE_WMA9_LOSSLESS => wma_cfg.fmtag = 0x163,
                SND_AUDIOPROFILE_WMA10_LOSSLESS => wma_cfg.fmtag = 0x167,
                _ => {
                    dev_err(dev, c"Unknown WMA profile:%x\n".as_ptr(), (*codec).profile);
                    return -EIO;
                }
            }
            let ret = if wma_v9 != 0 {
                q6asm_stream_media_format_block_wma_v9((*prtd).audio_client, stream_id, &mut wma_cfg)
            } else {
                q6asm_stream_media_format_block_wma_v10((*prtd).audio_client, stream_id, &mut wma_cfg)
            };
            if ret < 0 {
                dev_err(dev, c"WMA9 CMD failed:%d\n".as_ptr(), ret);
                return -EIO;
            }
        }
        SND_AUDIOCODEC_ALAC => {
            let mut alac_cfg: q6asm_alac_cfg = zeroed();
            let alac = &(*codec_options).alac_d;
            alac_cfg.sample_rate = (*codec).sample_rate;
            alac_cfg.avg_bit_rate = (*codec).bit_rate;
            alac_cfg.bit_depth = (*prtd).bits_per_sample;
            alac_cfg.num_channels = (*codec).ch_in;
            alac_cfg.frame_length = alac.frame_length;
            alac_cfg.pb = alac.pb;
            alac_cfg.mb = alac.mb;
            alac_cfg.kb = alac.kb;
            alac_cfg.max_run = alac.max_run;
            alac_cfg.compatible_version = alac.compatible_version;
            alac_cfg.max_frame_bytes = alac.max_frame_bytes;
            match (*codec).ch_in {
                1 => alac_cfg.channel_layout_tag = ALAC_CH_LAYOUT_MONO,
                2 => alac_cfg.channel_layout_tag = ALAC_CH_LAYOUT_STEREO,
                _ => {}
            }
            let ret = q6asm_stream_media_format_block_alac((*prtd).audio_client, stream_id, &mut alac_cfg);
            if ret < 0 {
                dev_err(dev, c"ALAC CMD Format block failed:%d\n".as_ptr(), ret);
                return -EIO;
            }
        }
        SND_AUDIOCODEC_APE => {
            let mut ape_cfg: q6asm_ape_cfg = zeroed();
            let ape = &(*codec_options).ape_d;
            ape_cfg.sample_rate = (*codec).sample_rate;
            ape_cfg.num_channels = (*codec).ch_in;
            ape_cfg.bits_per_sample = (*prtd).bits_per_sample;
            ape_cfg.compatible_version = ape.compatible_version;
            ape_cfg.compression_level = ape.compression_level;
            ape_cfg.format_flags = ape.format_flags;
            ape_cfg.blocks_per_frame = ape.blocks_per_frame;
            ape_cfg.final_frame_blocks = ape.final_frame_blocks;
            ape_cfg.total_frames = ape.total_frames;
            ape_cfg.seek_table_present = ape.seek_table_present;
            let ret = q6asm_stream_media_format_block_ape((*prtd).audio_client, stream_id, &mut ape_cfg);
            if ret < 0 {
                dev_err(dev, c"APE CMD Format block failed:%d\n".as_ptr(), ret);
                return -EIO;
            }
        }
        _ => {}
    }
    0
}

unsafe extern "C" fn q6asm_dai_compr_set_params(component: *mut snd_soc_component,
    stream: *mut snd_compr_stream, params: *mut snd_compr_params) -> c_int {
    let runtime = (*stream).runtime;
    let prtd = (*runtime).private_data as *mut q6asm_dai_rtd;
    let rtd = (*stream).private_data as *mut snd_soc_pcm_runtime;
    let dir = (*stream).direction;
    let dev = (*component).dev;
    let pdata = snd_soc_component_get_drvdata(component) as *mut q6asm_dai_data;
    if pdata.is_null() { return -EINVAL; }
    if prtd.is_null() || (*prtd).audio_client.is_null() {
        dev_err(dev, c"private data null or audio client freed\n".as_ptr());
        return -EINVAL;
    }
    (*prtd).periods = (*runtime).fragments;
    (*prtd).pcm_count = (*runtime).fragment_size;
    (*prtd).pcm_size = (*runtime).fragments * (*runtime).fragment_size;
    (*prtd).bits_per_sample = 16;
    let mut ret: c_int;
    if dir == SND_COMPRESS_PLAYBACK {
        ret = q6asm_open_write((*prtd).audio_client, (*prtd).stream_id, (*params).codec.id,
                               (*params).codec.profile, (*prtd).bits_per_sample, true);
        if ret < 0 {
            dev_err(dev, c"q6asm_open_write failed\n".as_ptr());
            return ret;
        }
    }
    (*prtd).session_id = q6asm_get_session_id((*prtd).audio_client);
    ret = q6routing_stream_open((*(*rtd).dai_link).id, LEGACY_PCM_MODE, (*prtd).session_id, dir);
    if ret != 0 {
        dev_err(dev, c"Stream reg failed ret:%d\n".as_ptr(), ret);
        q6asm_cmd((*prtd).audio_client, (*prtd).stream_id, CMD_CLOSE);
        return ret;
    }
    ret = __q6asm_dai_compr_set_codec_params(component, stream, &mut (*params).codec, (*prtd).stream_id as c_int);
    if ret != 0 {
        dev_err(dev, c"codec param setup failed ret:%d\n".as_ptr(), ret);
        q6routing_stream_close((*(*rtd).dai_link).id, dir);
        q6asm_cmd((*prtd).audio_client, (*prtd).stream_id, CMD_CLOSE);
        return ret;
    }
    ret = q6asm_map_memory_regions(dir, (*prtd).audio_client, (*prtd).phys,
                                   (*prtd).pcm_size / (*prtd).periods, (*prtd).periods);
    if ret < 0 {
        dev_err(dev, c"Buffer Mapping failed ret:%d\n".as_ptr(), ret);
        q6routing_stream_close((*(*rtd).dai_link).id, dir);
        q6asm_cmd((*prtd).audio_client, (*prtd).stream_id, CMD_CLOSE);
        return -ENOMEM;
    }
    (*prtd).state = stream_state::Q6ASM_STREAM_RUNNING;
    0
}

unsafe extern "C" fn q6asm_dai_compr_set_metadata(component: *mut snd_soc_component,
    stream: *mut snd_compr_stream, metadata: *mut snd_compr_metadata) -> c_int {
    let runtime = (*stream).runtime;
    let prtd = (*runtime).private_data as *mut q6asm_dai_rtd;
    let mut ret: c_int = 0;
    match (*metadata).key {
        SNDRV_COMPRESS_ENCODER_PADDING => (*prtd).trailing_samples_drop = (*metadata).value[0],
        SNDRV_COMPRESS_ENCODER_DELAY => {
            (*prtd).initial_samples_drop = (*metadata).value[0];
            if (*prtd).next_track_stream_id != 0 {
                ret = q6asm_open_write((*prtd).audio_client, (*prtd).next_track_stream_id,
                                       (*prtd).codec.id, (*prtd).codec.profile,
                                       (*prtd).bits_per_sample, true);
                if ret < 0 {
                    dev_err((*component).dev, c"q6asm_open_write failed\n".as_ptr());
                    return ret;
                }
                ret = __q6asm_dai_compr_set_codec_params(component, stream, &mut (*prtd).codec,
                                                         (*prtd).next_track_stream_id as c_int);
                if ret < 0 {
                    dev_err((*component).dev, c"q6asm_open_write failed\n".as_ptr());
                    return ret;
                }
                ret = q6asm_stream_remove_initial_silence((*prtd).audio_client,
                    (*prtd).next_track_stream_id, (*prtd).initial_samples_drop);
                (*prtd).next_track_stream_id = 0;
            }
        }
        _ => ret = -EINVAL,
    }
    ret
}

unsafe extern "C" fn q6asm_dai_compr_trigger(_component: *mut snd_soc_component,
    stream: *mut snd_compr_stream, cmd: c_int) -> c_int {
    let runtime = (*stream).runtime;
    let prtd = (*runtime).private_data as *mut q6asm_dai_rtd;
    match cmd {
        SNDRV_PCM_TRIGGER_START | SNDRV_PCM_TRIGGER_RESUME | SNDRV_PCM_TRIGGER_PAUSE_RELEASE =>
            q6asm_run_nowait((*prtd).audio_client, (*prtd).stream_id, 0, 0, 0),
        SNDRV_PCM_TRIGGER_STOP =>
            q6asm_cmd_nowait((*prtd).audio_client, (*prtd).stream_id, CMD_EOS),
        SNDRV_PCM_TRIGGER_SUSPEND | SNDRV_PCM_TRIGGER_PAUSE_PUSH =>
            q6asm_cmd_nowait((*prtd).audio_client, (*prtd).stream_id, CMD_PAUSE),
        SND_COMPR_TRIGGER_NEXT_TRACK => {
            (*prtd).next_track = true;
            (*prtd).next_track_stream_id = if (*prtd).stream_id == 1 { 2 } else { 1 };
            0
        }
        SND_COMPR_TRIGGER_DRAIN | SND_COMPR_TRIGGER_PARTIAL_DRAIN => {
            (*prtd).notify_on_drain = true;
            0
        }
        _ => -EINVAL,
    }
}

unsafe extern "C" fn q6asm_dai_compr_pointer(_component: *mut snd_soc_component,
    stream: *mut snd_compr_stream, tstamp: *mut snd_compr_tstamp64) -> c_int {
    let runtime = (*stream).runtime;
    let prtd = (*runtime).private_data as *mut q6asm_dai_rtd;
    spin_lock_irqsave(&mut (*prtd).lock);
    (*tstamp).copied_total = (*prtd).copied_total;
    let mut temp_copied_total = (*tstamp).copied_total;
    (*tstamp).byte_offset = do_div(&mut temp_copied_total, (*prtd).pcm_size);
    spin_unlock_irqrestore(&mut (*prtd).lock);
    0
}

unsafe extern "C" fn q6asm_compr_copy(_component: *mut snd_soc_component,
    stream: *mut snd_compr_stream, buf: *mut c_char, count: size_t) -> c_int {
    let runtime = (*stream).runtime;
    let prtd = (*runtime).private_data as *mut q6asm_dai_rtd;
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
    let dstn = ((*prtd).dma_buffer.area as *mut u8).add(app_pointer as usize) as *mut c_void;
    if count < ((*prtd).pcm_size - app_pointer) as usize {
        if copy_from_user(dstn, buf as *const c_void, count) != 0 { return -EFAULT; }
    } else {
        let copy = ((*prtd).pcm_size - app_pointer) as usize;
        if copy_from_user(dstn, buf as *const c_void, copy) != 0 { return -EFAULT; }
        if copy_from_user((*prtd).dma_buffer.area, buf.add(copy) as *const c_void, count - copy) != 0 {
            return -EFAULT;
        }
    }
    spin_lock_irqsave(&mut (*prtd).lock);
    let bytes_in_flight = (*prtd).bytes_received - (*prtd).copied_total;
    if (*prtd).next_track {
        (*prtd).next_track = false;
        (*prtd).copied_total = ALIGN((*prtd).copied_total, (*prtd).pcm_count as u64);
        (*prtd).bytes_sent = ALIGN((*prtd).bytes_sent, (*prtd).pcm_count as u64);
    }
    (*prtd).bytes_received = bytes_received + count as u64;
    /* Kick off the data to dsp if its starving!! */
    if (*prtd).state == stream_state::Q6ASM_STREAM_RUNNING && bytes_in_flight == 0 {
        let mut bytes_to_write = (*prtd).pcm_count;
        let avail = (*prtd).bytes_received - (*prtd).bytes_sent;
        if avail < (*prtd).pcm_count as u64 {
            bytes_to_write = avail as u32;
        }
        q6asm_write_async((*prtd).audio_client, (*prtd).stream_id, bytes_to_write, 0, 0, wflags);
        (*prtd).bytes_sent += bytes_to_write as u64;
    }
    spin_unlock_irqrestore(&mut (*prtd).lock);
    count as c_int
}

unsafe extern "C" fn q6asm_dai_compr_mmap(component: *mut snd_soc_component,
    stream: *mut snd_compr_stream, vma: *mut vm_area_struct) -> c_int {
    let runtime = (*stream).runtime;
    let prtd = (*runtime).private_data as *mut q6asm_dai_rtd;
    dma_mmap_coherent((*component).dev, vma, (*prtd).dma_buffer.area,
                      (*prtd).dma_buffer.addr, (*prtd).dma_buffer.bytes)
}

unsafe extern "C" fn q6asm_dai_compr_get_caps(_component: *mut snd_soc_component,
    _stream: *mut snd_compr_stream, caps: *mut snd_compr_caps) -> c_int {
    (*caps).direction = SND_COMPRESS_PLAYBACK;
    (*caps).min_fragment_size = COMPR_PLAYBACK_MIN_FRAGMENT_SIZE;
    (*caps).max_fragment_size = COMPR_PLAYBACK_MAX_FRAGMENT_SIZE;
    (*caps).min_fragments = COMPR_PLAYBACK_MIN_NUM_FRAGMENTS;
    (*caps).max_fragments = COMPR_PLAYBACK_MAX_NUM_FRAGMENTS;
    (*caps).num_codecs = 5;
    (*caps).codecs[0] = SND_AUDIOCODEC_MP3;
    (*caps).codecs[1] = SND_AUDIOCODEC_FLAC;
    (*caps).codecs[2] = SND_AUDIOCODEC_WMA;
    (*caps).codecs[3] = SND_AUDIOCODEC_ALAC;
    (*caps).codecs[4] = SND_AUDIOCODEC_APE;
    0
}

unsafe extern "C" fn q6asm_dai_compr_get_codec_caps(_component: *mut snd_soc_component,
    _stream: *mut snd_compr_stream, codec: *mut snd_compr_codec_caps) -> c_int {
    match (*codec).codec {
        SND_AUDIOCODEC_MP3 => *codec = q6asm_compr_caps,
        _ => {}
    }
    0
}

static q6asm_dai_compress_ops: snd_compress_ops = snd_compress_ops {
    open: Some(q6asm_dai_compr_open),
    free: Some(q6asm_dai_compr_free),
    set_params: Some(q6asm_dai_compr_set_params),
    set_metadata: Some(q6asm_dai_compr_set_metadata),
    pointer: Some(q6asm_dai_compr_pointer),
    trigger: Some(q6asm_dai_compr_trigger),
    get_caps: Some(q6asm_dai_compr_get_caps),
    get_codec_caps: Some(q6asm_dai_compr_get_codec_caps),
    mmap: Some(q6asm_dai_compr_mmap),
    copy: Some(q6asm_compr_copy),
};

unsafe extern "C" fn q6asm_dai_pcm_new(component: *mut snd_soc_component,
                                       rtd: *mut snd_soc_pcm_runtime) -> c_int {
    let pcm = (*rtd).pcm;
    let size = q6asm_dai_hardware_playback.buffer_bytes_max as size_t;
    snd_pcm_set_fixed_buffer_all(pcm, SNDRV_DMA_TYPE_DEV, (*component).dev, size)
}

static q6asm_dapm_widgets: [snd_soc_dapm_widget; 16] = [
    SND_SOC_DAPM_AIF_IN(c"MM_DL1".as_ptr(), c"MultiMedia1 Playback".as_ptr(), 0, SND_SOC_NOPM, 0, 0),
    SND_SOC_DAPM_AIF_IN(c"MM_DL2".as_ptr(), c"MultiMedia2 Playback".as_ptr(), 0, SND_SOC_NOPM, 0, 0),
    SND_SOC_DAPM_AIF_IN(c"MM_DL3".as_ptr(), c"MultiMedia3 Playback".as_ptr(), 0, SND_SOC_NOPM, 0, 0),
    SND_SOC_DAPM_AIF_IN(c"MM_DL4".as_ptr(), c"MultiMedia4 Playback".as_ptr(), 0, SND_SOC_NOPM, 0, 0),
    SND_SOC_DAPM_AIF_IN(c"MM_DL5".as_ptr(), c"MultiMedia5 Playback".as_ptr(), 0, SND_SOC_NOPM, 0, 0),
    SND_SOC_DAPM_AIF_IN(c"MM_DL6".as_ptr(), c"MultiMedia6 Playback".as_ptr(), 0, SND_SOC_NOPM, 0, 0),
    SND_SOC_DAPM_AIF_IN(c"MM_DL7".as_ptr(), c"MultiMedia7 Playback".as_ptr(), 0, SND_SOC_NOPM, 0, 0),
    SND_SOC_DAPM_AIF_IN(c"MM_DL8".as_ptr(), c"MultiMedia8 Playback".as_ptr(), 0, SND_SOC_NOPM, 0, 0),
    SND_SOC_DAPM_AIF_OUT(c"MM_UL1".as_ptr(), c"MultiMedia1 Capture".as_ptr(), 0, SND_SOC_NOPM, 0, 0),
    SND_SOC_DAPM_AIF_OUT(c"MM_UL2".as_ptr(), c"MultiMedia2 Capture".as_ptr(), 0, SND_SOC_NOPM, 0, 0),
    SND_SOC_DAPM_AIF_OUT(c"MM_UL3".as_ptr(), c"MultiMedia3 Capture".as_ptr(), 0, SND_SOC_NOPM, 0, 0),
    SND_SOC_DAPM_AIF_OUT(c"MM_UL4".as_ptr(), c"MultiMedia4 Capture".as_ptr(), 0, SND_SOC_NOPM, 0, 0),
    SND_SOC_DAPM_AIF_OUT(c"MM_UL5".as_ptr(), c"MultiMedia5 Capture".as_ptr(), 0, SND_SOC_NOPM, 0, 0),
    SND_SOC_DAPM_AIF_OUT(c"MM_UL6".as_ptr(), c"MultiMedia6 Capture".as_ptr(), 0, SND_SOC_NOPM, 0, 0),
    SND_SOC_DAPM_AIF_OUT(c"MM_UL7".as_ptr(), c"MultiMedia7 Capture".as_ptr(), 0, SND_SOC_NOPM, 0, 0),
    SND_SOC_DAPM_AIF_OUT(c"MM_UL8".as_ptr(), c"MultiMedia8 Capture".as_ptr(), 0, SND_SOC_NOPM, 0, 0),
];

static q6asm_fe_dai_component: snd_soc_component_driver = snd_soc_component_driver {
    name: DRV_NAME.as_ptr() as *const c_char,
    open: Some(q6asm_dai_open),
    hw_params: Some(q6asm_dai_hw_params),
    close: Some(q6asm_dai_close),
    prepare: Some(q6asm_dai_prepare),
    trigger: Some(q6asm_dai_trigger),
    ack: Some(q6asm_dai_ack),
    pointer: Some(q6asm_dai_pointer),
    pcm_new: Some(q6asm_dai_pcm_new),
    compress_ops: &q6asm_dai_compress_ops,
    dapm_widgets: q6asm_dapm_widgets.as_ptr(),
    num_dapm_widgets: q6asm_dapm_widgets.len() as c_uint,
    legacy_dai_naming: 1,
};

static mut q6asm_fe_dais_template: [snd_soc_dai_driver; 8] = [
    Q6ASM_FEDAI_DRIVER(0, MSM_FRONTEND_DAI_MULTIMEDIA1),
    Q6ASM_FEDAI_DRIVER(1, MSM_FRONTEND_DAI_MULTIMEDIA2),
    Q6ASM_FEDAI_DRIVER(2, MSM_FRONTEND_DAI_MULTIMEDIA3),
    Q6ASM_FEDAI_DRIVER(3, MSM_FRONTEND_DAI_MULTIMEDIA4),
    Q6ASM_FEDAI_DRIVER(4, MSM_FRONTEND_DAI_MULTIMEDIA5),
    Q6ASM_FEDAI_DRIVER(5, MSM_FRONTEND_DAI_MULTIMEDIA6),
    Q6ASM_FEDAI_DRIVER(6, MSM_FRONTEND_DAI_MULTIMEDIA7),
    Q6ASM_FEDAI_DRIVER(7, MSM_FRONTEND_DAI_MULTIMEDIA8),
];

static q6asm_dai_ops: snd_soc_dai_ops = snd_soc_dai_ops {
    compress_new: Some(snd_soc_new_compress),
};

unsafe extern "C" fn of_q6asm_parse_dai_data(dev: *mut device,
                                             pdata: *mut q6asm_dai_data) -> c_int {
    let mut empty_stream: snd_soc_pcm_stream = zeroed();
    let mut idx: c_int = 0;
    (*pdata).num_dais = of_get_child_count((*dev).of_node);
    if (*pdata).num_dais == 0 {
        dev_err(dev, c"No dais found in DT\n".as_ptr());
        return -EINVAL;
    }
    (*pdata).dais = devm_kcalloc(dev, (*pdata).num_dais as usize,
                                 size_of::<snd_soc_dai_driver>(), GFP_KERNEL) as *mut snd_soc_dai_driver;
    if (*pdata).dais.is_null() {
        return -ENOMEM;
    }
    memset(&mut empty_stream as *mut _ as *mut c_void, 0, size_of::<snd_soc_pcm_stream>());

    let mut node = of_get_next_child((*dev).of_node, null_mut());
    while !node.is_null() {
        let mut id: c_int = 0;
        let mut dir: c_int = 0;
        let ret = of_property_read_u32(node, c"reg".as_ptr(), &mut id);
        if ret != 0 || id >= MAX_SESSIONS || id < 0 {
            dev_err(dev, c"valid dai id not found:%d\n".as_ptr(), ret);
            node = of_get_next_child((*dev).of_node, node);
            continue;
        }
        let dai_drv = (*pdata).dais.add(idx as usize);
        idx += 1;
        *dai_drv = q6asm_fe_dais_template[id as usize];
        let ret2 = of_property_read_u32(node, c"direction".as_ptr(), &mut dir);
        if ret2 != 0 {
            node = of_get_next_child((*dev).of_node, node);
            continue;
        }
        if dir == Q6ASM_DAI_RX {
            (*dai_drv).capture = empty_stream;
        } else if dir == Q6ASM_DAI_TX {
            (*dai_drv).playback = empty_stream;
        }
        if of_property_read_bool(node, c"is-compress-dai".as_ptr()) {
            (*dai_drv).ops = &q6asm_dai_ops;
        }
        node = of_get_next_child((*dev).of_node, node);
    }
    0
}

unsafe extern "C" fn q6asm_dai_probe(pdev: *mut platform_device) -> c_int {
    let dev = &mut (*pdev).dev as *mut device;
    let node = (*dev).of_node;
    let mut args: of_phandle_args = zeroed();
    let pdata = devm_kzalloc(dev, size_of::<q6asm_dai_data>(), GFP_KERNEL) as *mut q6asm_dai_data;
    if pdata.is_null() {
        return -ENOMEM;
    }
    let mut rc = of_parse_phandle_with_fixed_args(node, c"iommus".as_ptr(), 1, 0, &mut args);
    if rc < 0 {
        (*pdata).sid = -1;
    } else {
        (*pdata).sid = (args.args[0] as c_longlong) & SID_MASK_DEFAULT;
    }
    dev_set_drvdata(dev, pdata as *mut c_void);
    rc = of_q6asm_parse_dai_data(dev, pdata);
    if rc != 0 {
        return rc;
    }
    devm_snd_soc_register_component(dev, &q6asm_fe_dai_component,
                                    (*pdata).dais, (*pdata).num_dais)
}

/* CONFIG_OF: of_device_id table and module device table. */
static q6asm_dai_device_id: [of_device_id; 2] = [
    of_device_id { compatible: c"qcom,q6asm-dais".as_ptr() },
    of_device_id { compatible: null() },
];

static mut q6asm_dai_platform_driver: platform_driver = platform_driver {
    driver: device_driver {
        name: c"q6asm-dai".as_ptr(),
        of_match_table: q6asm_dai_device_id.as_ptr(),
    },
    probe: Some(q6asm_dai_probe),
};

module_platform_driver!(q6asm_dai_platform_driver);
MODULE_DEVICE_TABLE!(of, q6asm_dai_device_id);
MODULE_DESCRIPTION!("Q6ASM dai driver");
MODULE_LICENSE!("GPL v2");

/* Opaque and external dependency surface from the original C includes. */
#[repr(C)] struct snd_pcm_substream { runtime: *mut snd_pcm_runtime, stream: c_int, dma_buffer: snd_dma_buffer }
#[repr(C)] struct snd_compr_stream { private_data: *mut c_void, runtime: *mut snd_compr_runtime, direction: c_int, partial_drain: bool_ }
#[repr(C)] struct snd_compr_runtime { private_data: *mut c_void, fragments: u32, fragment_size: u32 }
#[repr(C)] struct snd_codec { id: c_int, profile: c_int, ch_in: u32, sample_rate: u32, bit_rate: u32, align: u32, options: snd_codec_options }
#[repr(C)] struct snd_codec_options { flac_d: snd_dec_flac, wma_d: snd_dec_wma, alac_d: snd_dec_alac, ape_d: snd_dec_ape }
#[repr(C)] struct snd_dec_flac { sample_size: u32, min_blk_size: u32, max_blk_size: u32, max_frame_size: u32, min_frame_size: u32 }
#[repr(C)] struct snd_dec_wma { encoder_option: u32, adv_encoder_option: u32, adv_encoder_option2: u32 }
#[repr(C)] struct snd_dec_alac { frame_length: u32, pb: u8, mb: u8, kb: u8, max_run: u8, compatible_version: u32, max_frame_bytes: u32 }
#[repr(C)] struct snd_dec_ape { compatible_version: u32, compression_level: u32, format_flags: u32, blocks_per_frame: u32, final_frame_blocks: u32, total_frames: u32, seek_table_present: u32 }
#[repr(C)] #[derive(Copy, Clone)] struct snd_dma_buffer { addr: u64, area: *mut c_void, bytes: usize }
#[repr(C)] struct spinlock_t { _priv: [u8; 0] }
#[repr(C)] struct audio_client { _priv: [u8; 0] }
#[repr(C)] struct snd_pcm_runtime { private_data: *mut c_void, hw: snd_pcm_hardware, rate: u32, channels: u32, periods: u32, period_size: u64, control: *mut snd_pcm_mmap_control, dma_bytes: u32 }
#[repr(C)] struct snd_pcm_mmap_control { appl_ptr: u64 }
#[repr(C)] #[derive(Copy, Clone)] struct snd_pcm_hardware { info: u32, formats: u32, rates: u32, rate_min: u32, rate_max: u32, channels_min: u32, channels_max: u32, buffer_bytes_max: u32, period_bytes_min: u32, period_bytes_max: u32, periods_min: u32, periods_max: u32, fifo_size: u32 }
#[repr(C)] struct snd_soc_component { dev: *mut device }
#[repr(C)] struct snd_soc_pcm_runtime { dai_link: *mut snd_soc_dai_link, pcm: *mut snd_pcm }
#[repr(C)] struct snd_soc_dai_link { id: c_int }
#[repr(C)] struct snd_soc_dai { driver: *mut snd_soc_dai_driver }
#[repr(C)] #[derive(Copy, Clone)] struct snd_soc_pcm_stream { stream_name: *const c_char, rates: u32, formats: u32, channels_min: u32, channels_max: u32, rate_min: u32, rate_max: u32 }
#[repr(C)] #[derive(Copy, Clone)] struct snd_soc_dai_driver { playback: snd_soc_pcm_stream, capture: snd_soc_pcm_stream, name: *const c_char, id: c_int, ops: *const snd_soc_dai_ops }
#[repr(C)] struct snd_soc_dai_ops { compress_new: Option<unsafe extern "C" fn()> }
#[repr(C)] struct snd_compress_ops { open: Option<unsafe extern "C" fn(*mut snd_soc_component, *mut snd_compr_stream) -> c_int>, free: Option<unsafe extern "C" fn(*mut snd_soc_component, *mut snd_compr_stream) -> c_int>, set_params: Option<unsafe extern "C" fn(*mut snd_soc_component, *mut snd_compr_stream, *mut snd_compr_params) -> c_int>, set_metadata: Option<unsafe extern "C" fn(*mut snd_soc_component, *mut snd_compr_stream, *mut snd_compr_metadata) -> c_int>, pointer: Option<unsafe extern "C" fn(*mut snd_soc_component, *mut snd_compr_stream, *mut snd_compr_tstamp64) -> c_int>, trigger: Option<unsafe extern "C" fn(*mut snd_soc_component, *mut snd_compr_stream, c_int) -> c_int>, get_caps: Option<unsafe extern "C" fn(*mut snd_soc_component, *mut snd_compr_stream, *mut snd_compr_caps) -> c_int>, get_codec_caps: Option<unsafe extern "C" fn(*mut snd_soc_component, *mut snd_compr_stream, *mut snd_compr_codec_caps) -> c_int>, mmap: Option<unsafe extern "C" fn(*mut snd_soc_component, *mut snd_compr_stream, *mut vm_area_struct) -> c_int>, copy: Option<unsafe extern "C" fn(*mut snd_soc_component, *mut snd_compr_stream, *mut c_char, size_t) -> c_int> }
#[repr(C)] struct snd_soc_component_driver { name: *const c_char, open: Option<unsafe extern "C" fn(*mut snd_soc_component, *mut snd_pcm_substream) -> c_int>, hw_params: Option<unsafe extern "C" fn(*mut snd_soc_component, *mut snd_pcm_substream, *mut snd_pcm_hw_params) -> c_int>, close: Option<unsafe extern "C" fn(*mut snd_soc_component, *mut snd_pcm_substream) -> c_int>, prepare: Option<unsafe extern "C" fn(*mut snd_soc_component, *mut snd_pcm_substream) -> c_int>, trigger: Option<unsafe extern "C" fn(*mut snd_soc_component, *mut snd_pcm_substream, c_int) -> c_int>, ack: Option<unsafe extern "C" fn(*mut snd_soc_component, *mut snd_pcm_substream) -> c_int>, pointer: Option<unsafe extern "C" fn(*mut snd_soc_component, *mut snd_pcm_substream) -> snd_pcm_uframes_t>, pcm_new: Option<unsafe extern "C" fn(*mut snd_soc_component, *mut snd_soc_pcm_runtime) -> c_int>, compress_ops: *const snd_compress_ops, dapm_widgets: *const snd_soc_dapm_widget, num_dapm_widgets: c_uint, legacy_dai_naming: c_uint }
#[repr(C)] #[derive(Copy, Clone)] struct snd_soc_dapm_widget { _priv: [u8; 0] }
#[repr(C)] struct snd_pcm { _priv: [u8; 0] }
#[repr(C)] struct snd_pcm_hw_params { _priv: [u8; 0] }
#[repr(C)] struct snd_compr_params { codec: snd_codec }
#[repr(C)] struct snd_compr_metadata { key: c_int, value: [u32; 8] }
#[repr(C)] struct snd_compr_tstamp64 { copied_total: u64, byte_offset: u32 }
#[repr(C)] struct snd_compr_caps { direction: c_int, min_fragment_size: u32, max_fragment_size: u32, min_fragments: u32, max_fragments: u32, num_codecs: u32, codecs: [c_int; 32] }
#[repr(C)] #[derive(Copy, Clone)] struct snd_codec_desc { max_ch: u32, sample_rates: [u32; 13], num_sample_rates: u32, bit_rate: [u32; 2], num_bitrates: u32, profiles: u32, modes: u32, formats: u32 }
#[repr(C)] #[derive(Copy, Clone)] struct snd_compr_codec_caps { num_descriptors: u32, descriptor: [snd_codec_desc; 1], codec: c_int }
#[repr(C)] struct q6asm_flac_cfg { ch_cfg: u32, sample_rate: u32, stream_info_present: u32, sample_size: u32, min_blk_size: u32, max_blk_size: u32, max_frame_size: u32, min_frame_size: u32 }
#[repr(C)] struct q6asm_wma_cfg { sample_rate: u32, num_channels: u32, bytes_per_sec: u32, block_align: u32, bits_per_sample: u16, enc_options: u32, adv_enc_options: u32, adv_enc_options2: u32, channel_mask: u32, fmtag: u32 }
#[repr(C)] struct q6asm_alac_cfg { sample_rate: u32, avg_bit_rate: u32, bit_depth: u16, num_channels: u32, frame_length: u32, pb: u8, mb: u8, kb: u8, max_run: u8, compatible_version: u32, max_frame_bytes: u32, channel_layout_tag: u32 }
#[repr(C)] struct q6asm_ape_cfg { sample_rate: u32, num_channels: u32, bits_per_sample: u16, compatible_version: u32, compression_level: u32, format_flags: u32, blocks_per_frame: u32, final_frame_blocks: u32, total_frames: u32, seek_table_present: u32 }
#[repr(C)] struct vm_area_struct { _priv: [u8; 0] }
#[repr(C)] struct device { of_node: *mut device_node }
#[repr(C)] struct device_node { _priv: [u8; 0] }
#[repr(C)] struct of_phandle_args { args: [u32; 8] }
#[repr(C)] struct platform_device { dev: device }
#[repr(C)] struct of_device_id { compatible: *const c_char }
#[repr(C)] struct device_driver { name: *const c_char, of_match_table: *const of_device_id }
#[repr(C)] struct platform_driver { driver: device_driver, probe: Option<unsafe extern "C" fn(*mut platform_device) -> c_int> }

static MULTIMEDIA_NAMES: [&[u8]; 8] = [b"MultiMedia1\0", b"MultiMedia2\0", b"MultiMedia3\0", b"MultiMedia4\0", b"MultiMedia5\0", b"MultiMedia6\0", b"MultiMedia7\0", b"MultiMedia8\0"];
static MULTIMEDIA_PLAYBACK_NAMES: [&[u8]; 8] = [b"MultiMedia1 Playback\0", b"MultiMedia2 Playback\0", b"MultiMedia3 Playback\0", b"MultiMedia4 Playback\0", b"MultiMedia5 Playback\0", b"MultiMedia6 Playback\0", b"MultiMedia7 Playback\0", b"MultiMedia8 Playback\0"];
static MULTIMEDIA_CAPTURE_NAMES: [&[u8]; 8] = [b"MultiMedia1 Capture\0", b"MultiMedia2 Capture\0", b"MultiMedia3 Capture\0", b"MultiMedia4 Capture\0", b"MultiMedia5 Capture\0", b"MultiMedia6 Capture\0", b"MultiMedia7 Capture\0", b"MultiMedia8 Capture\0"];

extern "C" {
    fn snd_pcm_period_elapsed(substream: *mut snd_pcm_substream);
    fn q6asm_read(ac: *mut audio_client, stream_id: u32) -> c_int;
    fn snd_soc_substream_to_rtd(substream: *mut snd_pcm_substream) -> *mut snd_soc_pcm_runtime;
    fn snd_soc_component_get_drvdata(component: *mut snd_soc_component) -> *mut c_void;
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn dev_info(dev: *mut device, fmt: *const c_char, ...);
    fn q6asm_cmd(ac: *mut audio_client, stream_id: u32, cmd: c_int) -> c_int;
    fn q6asm_cmd_nowait(ac: *mut audio_client, stream_id: u32, cmd: c_int) -> c_int;
    fn q6asm_unmap_memory_regions(dir: c_int, ac: *mut audio_client) -> c_int;
    fn q6routing_stream_close(id: c_int, dir: c_int);
    fn q6asm_map_memory_regions(dir: c_int, ac: *mut audio_client, phys: phys_addr_t, period_sz: u32, periods: u32) -> c_int;
    fn q6asm_open_write(ac: *mut audio_client, stream_id: u32, fmt: c_int, profile: c_int, bits: u16, compr: bool_) -> c_int;
    fn q6asm_open_read(ac: *mut audio_client, stream_id: u32, fmt: c_int, bits: u16) -> c_int;
    fn q6asm_get_session_id(ac: *mut audio_client) -> u16;
    fn q6routing_stream_open(id: c_int, mode: c_int, session_id: u16, dir: c_int) -> c_int;
    fn q6asm_media_format_block_multi_ch_pcm(ac: *mut audio_client, stream_id: u32, rate: u32, channels: u32, map: *mut c_void, bits: u16) -> c_int;
    fn q6asm_enc_cfg_blk_pcm_format_support(ac: *mut audio_client, stream_id: u32, rate: u32, channels: u32, bits: u16) -> c_int;
    fn snd_pcm_lib_period_bytes(substream: *mut snd_pcm_substream) -> u32;
    fn q6asm_write_async(ac: *mut audio_client, stream_id: u32, count: u32, a: c_int, b: c_int, flags: u32) -> c_int;
    fn snd_soc_rtd_to_cpu(rtd: *mut snd_soc_pcm_runtime, n: c_int) -> *mut snd_soc_dai;
    fn kzalloc(size: usize, flags: c_int) -> *mut c_void;
    fn kfree(p: *mut c_void);
    fn q6asm_audio_client_alloc(dev: *mut device, cb: Option<unsafe extern "C" fn(u32, u32, *mut c_void, *mut c_void)>, priv_: *mut c_void, stream_id: c_int, mode: c_int) -> *mut audio_client;
    fn IS_ERR(p: *const c_void) -> bool_;
    fn PTR_ERR(p: *const c_void) -> c_int;
    fn snd_pcm_hw_constraint_integer(runtime: *mut snd_pcm_runtime, param: c_int) -> c_int;
    fn snd_pcm_hw_constraint_minmax(runtime: *mut snd_pcm_runtime, param: c_int, min: u32, max: u32) -> c_int;
    fn snd_pcm_hw_constraint_step(runtime: *mut snd_pcm_runtime, cond: c_int, param: c_int, step: u32) -> c_int;
    fn snd_soc_set_runtime_hwparams(substream: *mut snd_pcm_substream, hw: *const snd_pcm_hardware);
    fn q6asm_get_hw_pointer(ac: *mut audio_client, stream: c_int) -> u64;
    fn params_buffer_bytes(params: *mut snd_pcm_hw_params) -> u32;
    fn params_periods(params: *mut snd_pcm_hw_params) -> u32;
    fn params_format(params: *mut snd_pcm_hw_params) -> c_int;
    fn spin_lock_irqsave(lock: *mut spinlock_t);
    fn spin_unlock_irqrestore(lock: *mut spinlock_t);
    fn q6asm_stream_remove_initial_silence(ac: *mut audio_client, stream_id: u32, samples: u32) -> c_int;
    fn snd_compr_drain_notify(stream: *mut snd_compr_stream);
    fn snd_compr_fragment_elapsed(stream: *mut snd_compr_stream);
    fn q6asm_stream_remove_trailing_silence(ac: *mut audio_client, stream_id: u32, samples: u32) -> c_int;
    fn snd_dma_alloc_pages(typ: c_int, dev: *mut device, size: usize, buf: *mut snd_dma_buffer) -> c_int;
    fn snd_dma_free_pages(buf: *mut snd_dma_buffer);
    fn snd_compr_set_runtime_buffer(stream: *mut snd_compr_stream, buf: *mut snd_dma_buffer);
    fn spin_lock_init(lock: *mut spinlock_t);
    fn q6asm_stream_media_format_block_flac(ac: *mut audio_client, stream_id: c_int, cfg: *mut q6asm_flac_cfg) -> c_int;
    fn q6asm_stream_media_format_block_wma_v9(ac: *mut audio_client, stream_id: c_int, cfg: *mut q6asm_wma_cfg) -> c_int;
    fn q6asm_stream_media_format_block_wma_v10(ac: *mut audio_client, stream_id: c_int, cfg: *mut q6asm_wma_cfg) -> c_int;
    fn q6asm_stream_media_format_block_alac(ac: *mut audio_client, stream_id: c_int, cfg: *mut q6asm_alac_cfg) -> c_int;
    fn q6asm_stream_media_format_block_ape(ac: *mut audio_client, stream_id: c_int, cfg: *mut q6asm_ape_cfg) -> c_int;
    fn copy_from_user(dst: *mut c_void, src: *const c_void, n: usize) -> c_int;
    fn dma_mmap_coherent(dev: *mut device, vma: *mut vm_area_struct, area: *mut c_void, addr: u64, bytes: usize) -> c_int;
    fn snd_pcm_set_fixed_buffer_all(pcm: *mut snd_pcm, typ: c_int, dev: *mut device, size: usize) -> c_int;
    fn devm_kcalloc(dev: *mut device, n: usize, size: usize, flags: c_int) -> *mut c_void;
    fn memset(s: *mut c_void, c: c_int, n: usize) -> *mut c_void;
    fn of_get_child_count(node: *mut device_node) -> c_int;
    fn of_get_next_child(parent: *mut device_node, prev: *mut device_node) -> *mut device_node;
    fn of_property_read_u32(node: *mut device_node, name: *const c_char, out: *mut c_int) -> c_int;
    fn of_property_read_bool(node: *mut device_node, name: *const c_char) -> bool_;
    fn devm_kzalloc(dev: *mut device, size: usize, flags: c_int) -> *mut c_void;
    fn of_parse_phandle_with_fixed_args(node: *mut device_node, name: *const c_char, cells: c_int, index: c_int, out: *mut of_phandle_args) -> c_int;
    fn dev_set_drvdata(dev: *mut device, data: *mut c_void);
    fn devm_snd_soc_register_component(dev: *mut device, component: *const snd_soc_component_driver, dais: *mut snd_soc_dai_driver, num_dais: c_int) -> c_int;
    fn snd_soc_new_compress();
}

fn do_div(value: &mut u64, base: u32) -> u32 {
    let rem = (*value % base as u64) as u32;
    *value /= base as u64;
    rem
}

fn ALIGN(value: u64, align: u64) -> u64 {
    (value + align - 1) & !(align - 1)
}

extern "Rust" {
    static SNDRV_PCM_INFO_MMAP: u32;
    static SNDRV_PCM_INFO_BATCH: u32;
    static SNDRV_PCM_INFO_BLOCK_TRANSFER: u32;
    static SNDRV_PCM_INFO_NO_REWINDS: u32;
    static SNDRV_PCM_INFO_SYNC_APPLPTR: u32;
    static SNDRV_PCM_INFO_MMAP_VALID: u32;
    static SNDRV_PCM_INFO_INTERLEAVED: u32;
    static SNDRV_PCM_INFO_PAUSE: u32;
    static SNDRV_PCM_INFO_RESUME: u32;
    static SNDRV_PCM_FMTBIT_S16_LE: u32;
    static SNDRV_PCM_FMTBIT_S24_LE: u32;
    static SNDRV_PCM_RATE_8000_48000: u32;
    static SNDRV_PCM_RATE_8000_192000: u32;
    static SNDRV_PCM_RATE_12000: u32;
    static SNDRV_PCM_RATE_24000: u32;
    static SNDRV_PCM_RATE_88200: u32;
    static SNDRV_PCM_RATE_96000: u32;
    static SNDRV_PCM_RATE_176400: u32;
    static SNDRV_PCM_RATE_192000: u32;
}

const EINVAL: c_int = 22;
const ENOMEM: c_int = 12;
const EIO: c_int = 5;
const EFAULT: c_int = 14;
const GFP_KERNEL: c_int = 0;
const SNDRV_PCM_STREAM_PLAYBACK: c_int = 0;
const SNDRV_PCM_STREAM_CAPTURE: c_int = 1;
const SNDRV_PCM_TRIGGER_START: c_int = 0;
const SNDRV_PCM_TRIGGER_STOP: c_int = 1;
const SNDRV_PCM_TRIGGER_PAUSE_PUSH: c_int = 3;
const SNDRV_PCM_TRIGGER_PAUSE_RELEASE: c_int = 4;
const SNDRV_PCM_TRIGGER_SUSPEND: c_int = 5;
const SNDRV_PCM_TRIGGER_RESUME: c_int = 6;
const SNDRV_PCM_HW_PARAM_PERIODS: c_int = 0;
const SNDRV_PCM_HW_PARAM_BUFFER_BYTES: c_int = 1;
const SNDRV_PCM_HW_PARAM_PERIOD_SIZE: c_int = 2;
const SNDRV_PCM_HW_PARAM_BUFFER_SIZE: c_int = 3;
const SNDRV_PCM_FORMAT_S16_LE: c_int = 2;
const SNDRV_PCM_FORMAT_S24_LE: c_int = 6;
const SNDRV_DMA_TYPE_DEV: c_int = 0;
const SND_COMPRESS_PLAYBACK: c_int = 0;
const SND_COMPR_TRIGGER_NEXT_TRACK: c_int = 10;
const SND_COMPR_TRIGGER_DRAIN: c_int = 11;
const SND_COMPR_TRIGGER_PARTIAL_DRAIN: c_int = 12;
const SND_AUDIOCODEC_MP3: c_int = 1;
const SND_AUDIOCODEC_FLAC: c_int = 2;
const SND_AUDIOCODEC_WMA: c_int = 3;
const SND_AUDIOCODEC_ALAC: c_int = 4;
const SND_AUDIOCODEC_APE: c_int = 5;
const SND_AUDIOPROFILE_WMA9: c_int = 1;
const SND_AUDIOPROFILE_WMA10: c_int = 2;
const SND_AUDIOPROFILE_WMA9_PRO: c_int = 3;
const SND_AUDIOPROFILE_WMA9_LOSSLESS: c_int = 4;
const SND_AUDIOPROFILE_WMA10_LOSSLESS: c_int = 5;
const SND_AUDIOCHANMODE_MP3_STEREO: u32 = 2;
const SNDRV_COMPRESS_ENCODER_PADDING: c_int = 1;
const SNDRV_COMPRESS_ENCODER_DELAY: c_int = 2;
const ASM_CLIENT_EVENT_CMD_RUN_DONE: u32 = 1;
const ASM_CLIENT_EVENT_CMD_EOS_DONE: u32 = 2;
const ASM_CLIENT_EVENT_DATA_WRITE_DONE: u32 = 3;
const ASM_CLIENT_EVENT_DATA_READ_DONE: u32 = 4;
const ASM_WRITE_TOKEN_LEN_SHIFT: u32 = 16;
const ASM_LAST_BUFFER_FLAG: u32 = 1;
const CMD_CLOSE: c_int = 1;
const CMD_EOS: c_int = 2;
const CMD_PAUSE: c_int = 3;
const FORMAT_LINEAR_PCM: c_int = 0;
const LEGACY_PCM_MODE: c_int = 0;
const SND_SOC_NOPM: c_int = -1;
const MAX_SESSIONS: c_int = 8;
const Q6ASM_DAI_RX: c_int = 0;
const Q6ASM_DAI_TX: c_int = 1;
const MSM_FRONTEND_DAI_MULTIMEDIA1: c_int = 0;
const MSM_FRONTEND_DAI_MULTIMEDIA2: c_int = 1;
const MSM_FRONTEND_DAI_MULTIMEDIA3: c_int = 2;
const MSM_FRONTEND_DAI_MULTIMEDIA4: c_int = 3;
const MSM_FRONTEND_DAI_MULTIMEDIA5: c_int = 4;
const MSM_FRONTEND_DAI_MULTIMEDIA6: c_int = 5;
const MSM_FRONTEND_DAI_MULTIMEDIA7: c_int = 6;
const MSM_FRONTEND_DAI_MULTIMEDIA8: c_int = 7;

const fn SND_SOC_DAPM_AIF_IN(_name: *const c_char, _stream: *const c_char, _slot: c_int, _reg: c_int, _shift: c_int, _invert: c_int) -> snd_soc_dapm_widget {
    snd_soc_dapm_widget { _priv: [] }
}
const fn SND_SOC_DAPM_AIF_OUT(_name: *const c_char, _stream: *const c_char, _slot: c_int, _reg: c_int, _shift: c_int, _invert: c_int) -> snd_soc_dapm_widget {
    snd_soc_dapm_widget { _priv: [] }
}

macro_rules! module_platform_driver { ($driver:ident) => {}; }
macro_rules! MODULE_DEVICE_TABLE { ($table:ident, $id:ident) => {}; }
macro_rules! MODULE_DESCRIPTION { ($desc:expr) => {}; }
macro_rules! MODULE_LICENSE { ($license:expr) => {}; }

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
