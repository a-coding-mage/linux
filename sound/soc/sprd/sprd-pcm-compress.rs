// SPDX-License-Identifier: GPL-2.0
// Copyright (C) 2019 Spreadtrum Communications Inc.

// Rust translation of soc/sprd/sprd-pcm-compress.c.
// Kernel, ALSA, DMA engine, and local sprd-pcm-dma.h dependencies are expected
// to be provided by the surrounding repository bindings.

use core::ffi::{c_char, c_int, c_ulong, c_void};
use core::mem::size_of;
use core::ptr;

const SPRD_COMPR_DMA_CHANS: usize = 2;

/* Default values if userspace does not set */
const SPRD_COMPR_MIN_FRAGMENT_SIZE: usize = SZ_8K;
const SPRD_COMPR_MAX_FRAGMENT_SIZE: usize = SZ_128K;
const SPRD_COMPR_MIN_NUM_FRAGMENTS: u32 = 4;
const SPRD_COMPR_MAX_NUM_FRAGMENTS: u32 = 64;

/* DSP FIFO size */
const SPRD_COMPR_MCDT_EMPTY_WMK: c_int = 0;
const SPRD_COMPR_MCDT_FIFO_SIZE: c_int = 512;

/* Stage 0 IRAM buffer size definition */
const SPRD_COMPR_IRAM_BUF_SIZE: usize = SZ_32K;
const SPRD_COMPR_IRAM_INFO_SIZE: usize = size_of::<sprd_compr_playinfo>();
const SPRD_COMPR_IRAM_LINKLIST_SIZE: usize = 1024 - SPRD_COMPR_IRAM_INFO_SIZE;
const SPRD_COMPR_IRAM_SIZE: usize =
    SPRD_COMPR_IRAM_BUF_SIZE + SPRD_COMPR_IRAM_INFO_SIZE + SPRD_COMPR_IRAM_LINKLIST_SIZE;

/* Stage 1 DDR buffer size definition */
const SPRD_COMPR_AREA_BUF_SIZE: usize = SZ_2M;
const SPRD_COMPR_AREA_LINKLIST_SIZE: usize = 1024;
const SPRD_COMPR_AREA_SIZE: usize = SPRD_COMPR_AREA_BUF_SIZE + SPRD_COMPR_AREA_LINKLIST_SIZE;

type dma_cookie_t = c_int;
type dma_addr_t = usize;
type size_t = usize;

#[repr(C)]
pub struct dma_chan {
    pub device: *mut dma_device,
}

#[repr(C)]
pub struct dma_device {
    pub device_prep_slave_sg: Option<
        unsafe extern "C" fn(
            *mut dma_chan,
            *mut scatterlist,
            c_int,
            dma_transfer_direction,
            c_ulong,
            *mut sprd_dma_linklist,
        ) -> *mut dma_async_tx_descriptor,
    >,
}

#[repr(C)]
pub struct dma_async_tx_descriptor {
    pub callback: Option<unsafe extern "C" fn(*mut c_void)>,
    pub callback_param: *mut c_void,
}

#[repr(C)]
pub struct scatterlist {
    _private: [u8; 0],
}

#[repr(C)]
pub struct dma_slave_config {
    pub src_maxburst: c_int,
    pub src_addr_width: dma_slave_buswidth,
    pub dst_addr_width: dma_slave_buswidth,
    pub src_addr: dma_addr_t,
    pub dst_addr: dma_addr_t,
}

#[repr(C)]
pub struct sprd_dma_linklist {
    pub virt_addr: c_ulong,
    pub phy_addr: dma_addr_t,
}

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_component {
    pub dev: *mut device,
}

#[repr(C)]
pub struct snd_soc_pcm_runtime {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_compr_stream {
    pub runtime: *mut snd_compr_runtime,
    pub private_data: *mut c_void,
    pub direction: c_int,
}

#[repr(C)]
pub struct snd_compr_runtime {
    pub private_data: *mut c_void,
    pub fragment_size: c_int,
}

#[repr(C)]
pub struct snd_compr_params {
    pub buffer: snd_compr_buffer,
    pub codec: snd_codec,
    pub no_wake_mode: bool,
}

#[repr(C)]
pub struct snd_compr_buffer {
    pub fragment_size: c_int,
    pub fragments: c_int,
}

#[repr(C)]
pub struct snd_codec {
    pub sample_rate: c_int,
    pub bit_rate: c_int,
    pub id: c_int,
}

#[repr(C)]
pub struct snd_dma_buffer {
    pub addr: dma_addr_t,
    pub area: *mut u8,
    pub bytes: c_int,
}

#[repr(C)]
pub struct snd_compr_tstamp64 {
    pub copied_total: u64,
    pub pcm_io_frames: u64,
}

#[repr(C)]
pub struct snd_compr_caps {
    pub direction: c_int,
    pub min_fragment_size: usize,
    pub max_fragment_size: usize,
    pub min_fragments: u32,
    pub max_fragments: u32,
    pub num_codecs: u32,
    pub codecs: [c_int; 32],
}

#[repr(C)]
pub struct snd_compr_codec_caps {
    pub codec: c_int,
    pub num_descriptors: c_int,
    pub descriptor: [snd_codec_desc; 32],
}

#[repr(C)]
pub struct snd_codec_desc {
    pub max_ch: c_int,
    pub bit_rate: [c_int; 32],
    pub num_bitrates: c_int,
    pub profiles: c_int,
    pub modes: c_int,
    pub formats: c_int,
}

#[repr(C)]
pub struct sprd_compr_dma {
    pub chan: *mut dma_chan,
    pub desc: *mut dma_async_tx_descriptor,
    pub cookie: dma_cookie_t,
    pub phys: dma_addr_t,
    pub virt: *mut c_void,
    pub trans_len: c_int,
}

/*
 * The Spreadtrum Audio compress offload mode will use 2-stage DMA transfer to
 * save power. That means we can request 2 dma channels, one for source channel,
 * and another one for destination channel. Once the source channel's transaction
 * is done, it will trigger the destination channel's transaction automatically
 * by hardware signal.
 *
 * For 2-stage DMA transfer, we can allocate 2 buffers: IRAM buffer (always
 * power-on) and DDR buffer. The source channel will transfer data from IRAM
 * buffer to the DSP fifo to decoding/encoding, once IRAM buffer is empty by
 * transferring done, the destination channel will start to transfer data from
 * DDR buffer to IRAM buffer.
 *
 * Since the DSP fifo is only 512B, IRAM buffer is allocated by 32K, and DDR
 * buffer is larger to 2M. That means only the IRAM 32k data is transferred
 * done, we can wake up the AP system to transfer data from DDR to IRAM, and
 * other time the AP system can be suspended to save power.
 */
#[repr(C)]
pub struct sprd_compr_stream {
    pub cstream: *mut snd_compr_stream,
    pub compr_ops: *mut sprd_compr_ops,
    pub dma: [sprd_compr_dma; SPRD_COMPR_DMA_CHANS],

    /* DMA engine channel number */
    pub num_channels: c_int,

    /* Stage 0 IRAM buffer */
    pub iram_buffer: snd_dma_buffer,
    /* Stage 1 DDR buffer */
    pub compr_buffer: snd_dma_buffer,

    /* DSP play information IRAM buffer */
    pub info_phys: dma_addr_t,
    pub info_area: *mut c_void,
    pub info_size: c_int,

    /* Data size copied to IRAM buffer */
    pub copied_total: u64,
    /* Total received data size from userspace */
    pub received_total: u64,
    /* Stage 0 IRAM buffer received data size */
    pub received_stage0: c_int,
    /* Stage 1 DDR buffer received data size */
    pub received_stage1: c_int,
    /* Stage 1 DDR buffer pointer */
    pub stage1_pointer: c_int,
}

#[repr(C)]
pub struct sprd_compr_callback {
    pub drain_notify: Option<unsafe extern "C" fn(*mut c_void)>,
    pub drain_data: *mut c_void,
}

#[repr(C)]
pub struct sprd_compr_params {
    pub direction: c_int,
    pub sample_rate: c_int,
    pub channels: c_int,
    pub info_phys: dma_addr_t,
    pub info_size: c_int,
    pub rate: c_int,
    pub format: c_int,
}

#[repr(C)]
pub struct sprd_compr_data {
    pub dma_params: *mut sprd_pcm_dma_params,
    pub ops: *mut sprd_compr_ops,
}

#[repr(C)]
pub struct sprd_pcm_dma_params {
    pub chan_name: [*const c_char; SPRD_COMPR_DMA_CHANS],
    pub dev_phys: [dma_addr_t; SPRD_COMPR_DMA_CHANS],
}

#[repr(C)]
pub struct sprd_compr_playinfo {
    pub current_data_offset: u64,
}

#[repr(C)]
pub struct sprd_compr_ops {
    pub open: Option<unsafe extern "C" fn(c_int, *mut sprd_compr_callback) -> c_int>,
    pub close: Option<unsafe extern "C" fn(c_int)>,
    pub set_params: Option<unsafe extern "C" fn(c_int, *mut sprd_compr_params) -> c_int>,
    pub start: Option<unsafe extern "C" fn(c_int) -> c_int>,
    pub stop: Option<unsafe extern "C" fn(c_int) -> c_int>,
    pub pause: Option<unsafe extern "C" fn(c_int) -> c_int>,
    pub pause_release: Option<unsafe extern "C" fn(c_int) -> c_int>,
    pub drain: Option<unsafe extern "C" fn(u64) -> c_int>,
}

#[repr(C)]
pub struct snd_compress_ops {
    pub open: Option<
        unsafe extern "C" fn(*mut snd_soc_component, *mut snd_compr_stream) -> c_int,
    >,
    pub free: Option<
        unsafe extern "C" fn(*mut snd_soc_component, *mut snd_compr_stream) -> c_int,
    >,
    pub set_params: Option<
        unsafe extern "C" fn(
            *mut snd_soc_component,
            *mut snd_compr_stream,
            *mut snd_compr_params,
        ) -> c_int,
    >,
    pub trigger: Option<
        unsafe extern "C" fn(*mut snd_soc_component, *mut snd_compr_stream, c_int) -> c_int,
    >,
    pub pointer: Option<
        unsafe extern "C" fn(
            *mut snd_soc_component,
            *mut snd_compr_stream,
            *mut snd_compr_tstamp64,
        ) -> c_int,
    >,
    pub copy: Option<
        unsafe extern "C" fn(
            *mut snd_soc_component,
            *mut snd_compr_stream,
            *mut c_char,
            size_t,
        ) -> c_int,
    >,
    pub get_caps: Option<
        unsafe extern "C" fn(
            *mut snd_soc_component,
            *mut snd_compr_stream,
            *mut snd_compr_caps,
        ) -> c_int,
    >,
    pub get_codec_caps: Option<
        unsafe extern "C" fn(
            *mut snd_soc_component,
            *mut snd_compr_stream,
            *mut snd_compr_codec_caps,
        ) -> c_int,
    >,
}

extern "C" {
    static SZ_8K: usize;
    static SZ_128K: usize;
    static SZ_32K: usize;
    static SZ_2M: usize;

    static DMA_SLAVE_BUSWIDTH_4_BYTES: dma_slave_buswidth;
    static DMA_SLAVE_BUSWIDTH_2_BYTES: dma_slave_buswidth;
    static DMA_MEM_TO_DEV: dma_transfer_direction;
    static DMA_DEV_TO_MEM: dma_transfer_direction;

    static SND_COMPRESS_PLAYBACK: c_int;
    static SNDRV_PCM_TRIGGER_START: c_int;
    static SNDRV_PCM_TRIGGER_STOP: c_int;
    static SNDRV_PCM_TRIGGER_SUSPEND: c_int;
    static SNDRV_PCM_TRIGGER_PAUSE_PUSH: c_int;
    static SNDRV_PCM_TRIGGER_RESUME: c_int;
    static SNDRV_PCM_TRIGGER_PAUSE_RELEASE: c_int;
    static SND_COMPR_TRIGGER_PARTIAL_DRAIN: c_int;
    static SND_COMPR_TRIGGER_DRAIN: c_int;
    static SND_AUDIOCODEC_MP3: c_int;
    static SND_AUDIOCODEC_AAC: c_int;
    static SND_AUDIOCHANMODE_MP3_STEREO: c_int;
    static SNDRV_DMA_TYPE_DEV_IRAM: c_int;
    static SNDRV_DMA_TYPE_DEV: c_int;
    static GFP_KERNEL: c_int;
    static EINVAL: c_int;
    static ENODEV: c_int;
    static ENOMEM: c_int;
    static EFAULT: c_int;

    fn DMA_BIT_MASK(nr: c_int) -> u64;
    fn SPRD_DMA_FLAGS(a: c_int, b: c_int, c: c_int, d: c_int) -> c_ulong;
    static SPRD_DMA_SRC_CHN1: c_int;
    static SPRD_DMA_DST_CHN1: c_int;
    static SPRD_DMA_TRANS_DONE_TRG: c_int;
    static SPRD_DMA_FRAG_REQ: c_int;
    static SPRD_DMA_TRANS_INT: c_int;

    fn memset(s: *mut c_void, c: c_int, n: size_t) -> *mut c_void;
    fn copy_from_user(to: *mut c_void, from: *const c_char, n: size_t) -> c_ulong;
    fn kzalloc_objs(obj_size: size_t, num: c_int) -> *mut scatterlist;
    fn kfree(p: *mut c_void);
    fn devm_kzalloc(dev: *mut device, size: size_t, flags: c_int) -> *mut c_void;
    fn devm_kfree(dev: *mut device, p: *mut c_void);
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);

    fn snd_compr_drain_notify(cstream: *mut snd_compr_stream);
    fn snd_compr_fragment_elapsed(cstream: *mut snd_compr_stream);
    fn snd_soc_rtd_to_cpu(rtd: *mut snd_soc_pcm_runtime, num: c_int) -> *mut c_void;
    fn snd_soc_dai_get_drvdata(cpu: *mut c_void) -> *mut c_void;
    fn dma_coerce_mask_and_coherent(dev: *mut device, mask: u64) -> c_int;
    fn snd_dma_alloc_pages(
        ty: c_int,
        dev: *mut device,
        size: usize,
        dmab: *mut snd_dma_buffer,
    ) -> c_int;
    fn snd_dma_free_pages(dmab: *mut snd_dma_buffer);
    fn dma_request_slave_channel(dev: *mut device, name: *const c_char) -> *mut dma_chan;
    fn dma_release_channel(chan: *mut dma_chan);
    fn sg_init_table(sgl: *mut scatterlist, nents: c_int);
    fn sg_dma_len(sg: *mut scatterlist) -> *mut c_int;
    fn sg_dma_address(sg: *mut scatterlist) -> *mut dma_addr_t;
    fn dmaengine_slave_config(chan: *mut dma_chan, config: *mut dma_slave_config) -> c_int;
    fn dmaengine_submit(desc: *mut dma_async_tx_descriptor) -> dma_cookie_t;
    fn dma_submit_error(cookie: dma_cookie_t) -> c_int;
    fn dma_async_issue_pending(chan: *mut dma_chan);
    fn dmaengine_terminate_async(chan: *mut dma_chan);
    fn dmaengine_pause(chan: *mut dma_chan);
    fn dmaengine_resume(chan: *mut dma_chan);
}

type dma_transfer_direction = c_int;
type dma_slave_buswidth = c_int;

unsafe extern "C" fn sprd_platform_compr_trigger(
    component: *mut snd_soc_component,
    cstream: *mut snd_compr_stream,
    cmd: c_int,
) -> c_int {
    let runtime = (*cstream).runtime;
    let stream = (*runtime).private_data as *mut sprd_compr_stream;
    let dev = (*component).dev;
    let channels = (*stream).num_channels;
    let mut ret: c_int = 0;
    let stream_id = (*cstream).direction;

    if (*cstream).direction != SND_COMPRESS_PLAYBACK {
        dev_err(dev, b"unsupported compress direction\n\0".as_ptr() as *const c_char);
        return -EINVAL;
    }

    if cmd == SNDRV_PCM_TRIGGER_START {
        let mut i = channels - 1;
        while i >= 0 {
            let dma = &mut (*stream).dma[i as usize] as *mut sprd_compr_dma;

            if !(*dma).desc.is_null() {
                (*dma).cookie = dmaengine_submit((*dma).desc);
                ret = dma_submit_error((*dma).cookie);
                if ret != 0 {
                    dev_err(
                        dev,
                        b"failed to submit request: %d\n\0".as_ptr() as *const c_char,
                        ret,
                    );
                    return ret;
                }
            }

            if i == 0 {
                break;
            }
            i -= 1;
        }

        let mut i = channels - 1;
        while i >= 0 {
            let dma = &mut (*stream).dma[i as usize] as *mut sprd_compr_dma;

            if !(*dma).chan.is_null() {
                dma_async_issue_pending((*dma).chan);
            }

            if i == 0 {
                break;
            }
            i -= 1;
        }

        ret = ((*(*stream).compr_ops).start.unwrap())(stream_id);
    } else if cmd == SNDRV_PCM_TRIGGER_STOP {
        let mut i = channels - 1;
        while i >= 0 {
            let dma = &mut (*stream).dma[i as usize] as *mut sprd_compr_dma;

            if !(*dma).chan.is_null() {
                dmaengine_terminate_async((*dma).chan);
            }

            if i == 0 {
                break;
            }
            i -= 1;
        }

        (*stream).copied_total = 0;
        (*stream).stage1_pointer = 0;
        (*stream).received_total = 0;
        (*stream).received_stage0 = 0;
        (*stream).received_stage1 = 0;

        ret = ((*(*stream).compr_ops).stop.unwrap())(stream_id);
    } else if cmd == SNDRV_PCM_TRIGGER_SUSPEND || cmd == SNDRV_PCM_TRIGGER_PAUSE_PUSH {
        let mut i = channels - 1;
        while i >= 0 {
            let dma = &mut (*stream).dma[i as usize] as *mut sprd_compr_dma;

            if !(*dma).chan.is_null() {
                dmaengine_pause((*dma).chan);
            }

            if i == 0 {
                break;
            }
            i -= 1;
        }

        ret = ((*(*stream).compr_ops).pause.unwrap())(stream_id);
    } else if cmd == SNDRV_PCM_TRIGGER_RESUME || cmd == SNDRV_PCM_TRIGGER_PAUSE_RELEASE {
        let mut i = channels - 1;
        while i >= 0 {
            let dma = &mut (*stream).dma[i as usize] as *mut sprd_compr_dma;

            if !(*dma).chan.is_null() {
                dmaengine_resume((*dma).chan);
            }

            if i == 0 {
                break;
            }
            i -= 1;
        }

        ret = ((*(*stream).compr_ops).pause_release.unwrap())(stream_id);
    } else if cmd == SND_COMPR_TRIGGER_PARTIAL_DRAIN || cmd == SND_COMPR_TRIGGER_DRAIN {
        ret = ((*(*stream).compr_ops).drain.unwrap())((*stream).received_total);
    } else {
        ret = -EINVAL;
    }

    ret
}

unsafe extern "C" fn sprd_platform_compr_drain_notify(arg: *mut c_void) {
    let cstream = arg as *mut snd_compr_stream;
    let runtime = (*cstream).runtime;
    let stream = (*runtime).private_data as *mut sprd_compr_stream;

    memset(
        (*stream).info_area,
        0,
        size_of::<sprd_compr_playinfo>(),
    );

    snd_compr_drain_notify(cstream);
}

unsafe extern "C" fn sprd_platform_compr_dma_complete(data: *mut c_void) {
    let cstream = data as *mut snd_compr_stream;
    let runtime = (*cstream).runtime;
    let stream = (*runtime).private_data as *mut sprd_compr_stream;
    let dma = &mut (*stream).dma[1] as *mut sprd_compr_dma;

    /* Update data size copied to IRAM buffer */
    (*stream).copied_total = (*stream).copied_total.wrapping_add((*dma).trans_len as u64);
    if (*stream).copied_total > (*stream).received_total {
        (*stream).copied_total = (*stream).received_total;
    }

    snd_compr_fragment_elapsed(cstream);
}

unsafe extern "C" fn sprd_platform_compr_dma_config(
    component: *mut snd_soc_component,
    cstream: *mut snd_compr_stream,
    params: *mut snd_compr_params,
    channel: c_int,
) -> c_int {
    let runtime = (*cstream).runtime;
    let stream = (*runtime).private_data as *mut sprd_compr_stream;
    let rtd = (*cstream).private_data as *mut snd_soc_pcm_runtime;
    let dev = (*component).dev;
    let data = snd_soc_dai_get_drvdata(snd_soc_rtd_to_cpu(rtd, 0)) as *mut sprd_compr_data;
    let dma_params = (*data).dma_params;
    let dma = &mut (*stream).dma[channel as usize] as *mut sprd_compr_dma;
    let mut config: dma_slave_config = core::mem::zeroed();
    let mut link: sprd_dma_linklist = core::mem::zeroed();
    let dir: dma_transfer_direction;
    let mut sg: *mut scatterlist;
    let sgt: *mut scatterlist;
    let bus_width: dma_slave_buswidth;
    let period: c_int;
    let period_cnt: c_int;
    let sg_num: c_int = 2;
    let src_addr: dma_addr_t;
    let dst_addr: dma_addr_t;
    let flags: c_ulong;
    let mut ret: c_int;

    if dma_params.is_null() {
        dev_err(dev, b"no dma parameters setting\n\0".as_ptr() as *const c_char);
        return -EINVAL;
    }

    (*dma).chan = dma_request_slave_channel(dev, (*dma_params).chan_name[channel as usize]);
    if (*dma).chan.is_null() {
        dev_err(dev, b"failed to request dma channel\n\0".as_ptr() as *const c_char);
        return -ENODEV;
    }

    sg = kzalloc_objs(size_of::<scatterlist>(), sg_num);
    sgt = sg;
    if sg.is_null() {
        ret = -ENOMEM;
        dma_release_channel((*dma).chan);
        return ret;
    }

    match channel {
        0 => {
            bus_width = DMA_SLAVE_BUSWIDTH_4_BYTES;
            period = (SPRD_COMPR_MCDT_FIFO_SIZE - SPRD_COMPR_MCDT_EMPTY_WMK) * 4;
            period_cnt = (*params).buffer.fragment_size / period;
            src_addr = (*stream).iram_buffer.addr;
            dst_addr = (*dma_params).dev_phys[channel as usize];
            flags = SPRD_DMA_FLAGS(
                SPRD_DMA_SRC_CHN1,
                SPRD_DMA_TRANS_DONE_TRG,
                SPRD_DMA_FRAG_REQ,
                SPRD_DMA_TRANS_INT,
            );
        }
        1 => {
            bus_width = DMA_SLAVE_BUSWIDTH_2_BYTES;
            period = (*params).buffer.fragment_size;
            period_cnt = (*params).buffer.fragments;
            src_addr = (*stream).compr_buffer.addr;
            dst_addr = (*stream).iram_buffer.addr;
            flags = SPRD_DMA_FLAGS(
                SPRD_DMA_DST_CHN1,
                SPRD_DMA_TRANS_DONE_TRG,
                SPRD_DMA_FRAG_REQ,
                SPRD_DMA_TRANS_INT,
            );
        }
        _ => {
            ret = -EINVAL;
            kfree(sg as *mut c_void);
            dma_release_channel((*dma).chan);
            return ret;
        }
    }

    (*dma).trans_len = period * period_cnt;

    config.src_maxburst = period;
    config.src_addr_width = bus_width;
    config.dst_addr_width = bus_width;
    if (*cstream).direction == SND_COMPRESS_PLAYBACK {
        config.src_addr = src_addr;
        config.dst_addr = dst_addr;
        dir = DMA_MEM_TO_DEV;
    } else {
        config.src_addr = dst_addr;
        config.dst_addr = src_addr;
        dir = DMA_DEV_TO_MEM;
    }

    sg_init_table(sgt, sg_num);
    let mut j = 0;
    let mut cur = sgt;
    while j < sg_num {
        *sg_dma_len(cur) = (*dma).trans_len;
        *sg_dma_address(cur) = dst_addr;
        cur = cur.add(1);
        j += 1;
    }

    /*
     * Configure the link-list address for the DMA engine link-list
     * mode.
     */
    link.virt_addr = (*dma).virt as c_ulong;
    link.phy_addr = (*dma).phys;

    ret = dmaengine_slave_config((*dma).chan, &mut config);
    if ret != 0 {
        dev_err(
            dev,
            b"failed to set slave configuration: %d\n\0".as_ptr() as *const c_char,
            ret,
        );
        kfree(sg as *mut c_void);
        dma_release_channel((*dma).chan);
        return ret;
    }

    /*
     * We configure the DMA request mode, interrupt mode, channel
     * mode and channel trigger mode by the flags.
     */
    (*dma).desc = ((*(*(*dma).chan).device).device_prep_slave_sg.unwrap())(
        (*dma).chan,
        sg,
        sg_num,
        dir,
        flags,
        &mut link,
    ));
    if (*dma).desc.is_null() {
        dev_err(dev, b"failed to prepare slave sg\n\0".as_ptr() as *const c_char);
        ret = -ENOMEM;
        kfree(sg as *mut c_void);
        dma_release_channel((*dma).chan);
        return ret;
    }

    /* Only channel 1 transfer can wake up the AP system. */
    if !(*params).no_wake_mode && channel == 1 {
        (*(*dma).desc).callback = Some(sprd_platform_compr_dma_complete);
        (*(*dma).desc).callback_param = cstream as *mut c_void;
    }

    kfree(sg as *mut c_void);

    0
}

unsafe extern "C" fn sprd_platform_compr_set_params(
    component: *mut snd_soc_component,
    cstream: *mut snd_compr_stream,
    params: *mut snd_compr_params,
) -> c_int {
    let runtime = (*cstream).runtime;
    let stream = (*runtime).private_data as *mut sprd_compr_stream;
    let dev = (*component).dev;
    let mut compr_params: sprd_compr_params = core::mem::zeroed();
    let mut ret: c_int;

    /*
     * Configure the DMA engine 2-stage transfer mode. Channel 1 set as the
     * destination channel, and channel 0 set as the source channel, that
     * means once the source channel's transaction is done, it will trigger
     * the destination channel's transaction automatically.
     */
    ret = sprd_platform_compr_dma_config(component, cstream, params, 1);
    if ret != 0 {
        dev_err(
            dev,
            b"failed to config stage 1 DMA: %d\n\0".as_ptr() as *const c_char,
            ret,
        );
        return ret;
    }

    ret = sprd_platform_compr_dma_config(component, cstream, params, 0);
    if ret != 0 {
        dev_err(
            dev,
            b"failed to config stage 0 DMA: %d\n\0".as_ptr() as *const c_char,
            ret,
        );
        dma_release_channel((*stream).dma[1].chan);
        return ret;
    }

    compr_params.direction = (*cstream).direction;
    compr_params.sample_rate = (*params).codec.sample_rate;
    compr_params.channels = (*stream).num_channels;
    compr_params.info_phys = (*stream).info_phys;
    compr_params.info_size = (*stream).info_size;
    compr_params.rate = (*params).codec.bit_rate;
    compr_params.format = (*params).codec.id;

    ret = ((*(*stream).compr_ops).set_params.unwrap())((*cstream).direction, &mut compr_params);
    if ret != 0 {
        dev_err(
            dev,
            b"failed to set parameters: %d\n\0".as_ptr() as *const c_char,
            ret,
        );
        dma_release_channel((*stream).dma[0].chan);
        dma_release_channel((*stream).dma[1].chan);
        return ret;
    }

    0
}

unsafe extern "C" fn sprd_platform_compr_open(
    component: *mut snd_soc_component,
    cstream: *mut snd_compr_stream,
) -> c_int {
    let runtime = (*cstream).runtime;
    let rtd = (*cstream).private_data as *mut snd_soc_pcm_runtime;
    let dev = (*component).dev;
    let data = snd_soc_dai_get_drvdata(snd_soc_rtd_to_cpu(rtd, 0)) as *mut sprd_compr_data;
    let stream: *mut sprd_compr_stream;
    let mut cb: sprd_compr_callback = core::mem::zeroed();
    let stream_id = (*cstream).direction;
    let mut ret: c_int;

    ret = dma_coerce_mask_and_coherent(dev, DMA_BIT_MASK(32));
    if ret != 0 {
        return ret;
    }

    stream = devm_kzalloc(dev, size_of::<sprd_compr_stream>(), GFP_KERNEL) as *mut sprd_compr_stream;
    if stream.is_null() {
        return -ENOMEM;
    }

    (*stream).cstream = cstream;
    (*stream).num_channels = 2;
    (*stream).compr_ops = (*data).ops;

    /*
     * Allocate the stage 0 IRAM buffer size, including the DMA 0
     * link-list size and play information of DSP address size.
     */
    ret = snd_dma_alloc_pages(
        SNDRV_DMA_TYPE_DEV_IRAM,
        dev,
        SPRD_COMPR_IRAM_SIZE,
        &mut (*stream).iram_buffer,
    );
    if ret < 0 {
        devm_kfree(dev, stream as *mut c_void);
        return ret;
    }

    /* Use to save link-list configuration for DMA 0. */
    (*stream).dma[0].virt = (*stream).iram_buffer.area.add(SPRD_COMPR_IRAM_SIZE) as *mut c_void;
    (*stream).dma[0].phys = (*stream).iram_buffer.addr + SPRD_COMPR_IRAM_SIZE;

    /* Use to update the current data offset of DSP. */
    (*stream).info_phys =
        (*stream).iram_buffer.addr + SPRD_COMPR_IRAM_SIZE + SPRD_COMPR_IRAM_LINKLIST_SIZE;
    (*stream).info_area = (*stream)
        .iram_buffer
        .area
        .add(SPRD_COMPR_IRAM_SIZE + SPRD_COMPR_IRAM_LINKLIST_SIZE)
        as *mut c_void;
    (*stream).info_size = SPRD_COMPR_IRAM_INFO_SIZE as c_int;

    /*
     * Allocate the stage 1 DDR buffer size, including the DMA 1 link-list
     * size.
     */
    ret = snd_dma_alloc_pages(
        SNDRV_DMA_TYPE_DEV,
        dev,
        SPRD_COMPR_AREA_SIZE,
        &mut (*stream).compr_buffer,
    );
    if ret < 0 {
        snd_dma_free_pages(&mut (*stream).iram_buffer);
        devm_kfree(dev, stream as *mut c_void);
        return ret;
    }

    /* Use to save link-list configuration for DMA 1. */
    (*stream).dma[1].virt = (*stream).compr_buffer.area.add(SPRD_COMPR_AREA_SIZE) as *mut c_void;
    (*stream).dma[1].phys = (*stream).compr_buffer.addr + SPRD_COMPR_AREA_SIZE;

    cb.drain_notify = Some(sprd_platform_compr_drain_notify);
    cb.drain_data = cstream as *mut c_void;
    ret = ((*(*stream).compr_ops).open.unwrap())(stream_id, &mut cb);
    if ret != 0 {
        dev_err(
            dev,
            b"failed to open compress platform: %d\n\0".as_ptr() as *const c_char,
            ret,
        );
        snd_dma_free_pages(&mut (*stream).compr_buffer);
        snd_dma_free_pages(&mut (*stream).iram_buffer);
        devm_kfree(dev, stream as *mut c_void);
        return ret;
    }

    (*runtime).private_data = stream as *mut c_void;
    0
}

unsafe extern "C" fn sprd_platform_compr_free(
    component: *mut snd_soc_component,
    cstream: *mut snd_compr_stream,
) -> c_int {
    let runtime = (*cstream).runtime;
    let stream = (*runtime).private_data as *mut sprd_compr_stream;
    let dev = (*component).dev;
    let stream_id = (*cstream).direction;
    let mut i: c_int = 0;

    while i < (*stream).num_channels {
        let dma = &mut (*stream).dma[i as usize] as *mut sprd_compr_dma;

        if !(*dma).chan.is_null() {
            dma_release_channel((*dma).chan);
            (*dma).chan = ptr::null_mut();
        }

        i += 1;
    }

    snd_dma_free_pages(&mut (*stream).compr_buffer);
    snd_dma_free_pages(&mut (*stream).iram_buffer);

    ((*(*stream).compr_ops).close.unwrap())(stream_id);

    devm_kfree(dev, stream as *mut c_void);
    0
}

unsafe extern "C" fn sprd_platform_compr_pointer(
    _component: *mut snd_soc_component,
    cstream: *mut snd_compr_stream,
    tstamp: *mut snd_compr_tstamp64,
) -> c_int {
    let runtime = (*cstream).runtime;
    let stream = (*runtime).private_data as *mut sprd_compr_stream;
    let info = (*stream).info_area as *mut sprd_compr_playinfo;

    (*tstamp).copied_total = (*stream).copied_total;
    (*tstamp).pcm_io_frames = (*info).current_data_offset;

    0
}

unsafe extern "C" fn sprd_platform_compr_copy(
    _component: *mut snd_soc_component,
    cstream: *mut snd_compr_stream,
    mut buf: *mut c_char,
    count: size_t,
) -> c_int {
    let runtime = (*cstream).runtime;
    let stream = (*runtime).private_data as *mut sprd_compr_stream;
    let mut avail_bytes: c_int;
    let mut data_count: c_int = count as c_int;
    let mut dst: *mut c_void;

    /*
     * We usually set fragment size as 32K, and the stage 0 IRAM buffer
     * size is 32K too. So if now the received data size of the stage 0
     * IRAM buffer is less than 32K, that means we have some available
     * spaces for the stage 0 IRAM buffer.
     */
    if (*stream).received_stage0 < (*runtime).fragment_size {
        avail_bytes = (*runtime).fragment_size - (*stream).received_stage0;
        dst = (*stream)
            .iram_buffer
            .area
            .add((*stream).received_stage0 as usize) as *mut c_void;

        if avail_bytes >= data_count {
            /*
             * Copy data to the stage 0 IRAM buffer directly if
             * spaces are enough.
             */
            if copy_from_user(dst, buf, data_count as size_t) != 0 {
                return -EFAULT;
            }

            (*stream).received_stage0 += data_count;
            (*stream).copied_total = (*stream).copied_total.wrapping_add(data_count as u64);
            (*stream).received_total = (*stream).received_total.wrapping_add(count as u64);
            return count as c_int;
        } else {
            /*
             * If the data count is larger than the available spaces
             * of the stage 0 IRAM buffer, we should copy one
             * partial data to the stage 0 IRAM buffer, and copy
             * the left to the stage 1 DDR buffer.
             */
            if copy_from_user(dst, buf, avail_bytes as size_t) != 0 {
                return -EFAULT;
            }

            data_count -= avail_bytes;
            (*stream).received_stage0 += avail_bytes;
            (*stream).copied_total = (*stream).copied_total.wrapping_add(avail_bytes as u64);
            buf = buf.add(avail_bytes as usize);
        }
    }

    /*
     * Copy data to the stage 1 DDR buffer if no spaces for the stage 0 IRAM
     * buffer.
     */
    dst = (*stream)
        .compr_buffer
        .area
        .add((*stream).stage1_pointer as usize) as *mut c_void;
    if data_count < (*stream).compr_buffer.bytes - (*stream).stage1_pointer {
        if copy_from_user(dst, buf, data_count as size_t) != 0 {
            return -EFAULT;
        }

        (*stream).stage1_pointer += data_count;
    } else {
        avail_bytes = (*stream).compr_buffer.bytes - (*stream).stage1_pointer;

        if copy_from_user(dst, buf, avail_bytes as size_t) != 0 {
            return -EFAULT;
        }

        if copy_from_user(
            (*stream).compr_buffer.area as *mut c_void,
            buf.add(avail_bytes as usize),
            (data_count - avail_bytes) as size_t,
        ) != 0
        {
            return -EFAULT;
        }

        (*stream).stage1_pointer = data_count - avail_bytes;
    }

    (*stream).received_stage1 += data_count;

    /* Update the copied data size. */
    (*stream).received_total = (*stream).received_total.wrapping_add(count as u64);
    count as c_int
}

unsafe extern "C" fn sprd_platform_compr_get_caps(
    _component: *mut snd_soc_component,
    cstream: *mut snd_compr_stream,
    caps: *mut snd_compr_caps,
) -> c_int {
    (*caps).direction = (*cstream).direction;
    (*caps).min_fragment_size = SPRD_COMPR_MIN_FRAGMENT_SIZE;
    (*caps).max_fragment_size = SPRD_COMPR_MAX_FRAGMENT_SIZE;
    (*caps).min_fragments = SPRD_COMPR_MIN_NUM_FRAGMENTS;
    (*caps).max_fragments = SPRD_COMPR_MAX_NUM_FRAGMENTS;
    (*caps).num_codecs = 2;
    (*caps).codecs[0] = SND_AUDIOCODEC_MP3;
    (*caps).codecs[1] = SND_AUDIOCODEC_AAC;

    0
}

unsafe extern "C" fn sprd_platform_compr_get_codec_caps(
    _component: *mut snd_soc_component,
    _cstream: *mut snd_compr_stream,
    codec: *mut snd_compr_codec_caps,
) -> c_int {
    if (*codec).codec == SND_AUDIOCODEC_MP3 {
        (*codec).num_descriptors = 2;
        (*codec).descriptor[0].max_ch = 2;
        (*codec).descriptor[0].bit_rate[0] = 320;
        (*codec).descriptor[0].bit_rate[1] = 128;
        (*codec).descriptor[0].num_bitrates = 2;
        (*codec).descriptor[0].profiles = 0;
        (*codec).descriptor[0].modes = SND_AUDIOCHANMODE_MP3_STEREO;
        (*codec).descriptor[0].formats = 0;
    } else if (*codec).codec == SND_AUDIOCODEC_AAC {
        (*codec).num_descriptors = 2;
        (*codec).descriptor[1].max_ch = 2;
        (*codec).descriptor[1].bit_rate[0] = 320;
        (*codec).descriptor[1].bit_rate[1] = 128;
        (*codec).descriptor[1].num_bitrates = 2;
        (*codec).descriptor[1].profiles = 0;
        (*codec).descriptor[1].modes = 0;
        (*codec).descriptor[1].formats = 0;
    } else {
        return -EINVAL;
    }

    0
}

#[no_mangle]
pub static sprd_platform_compress_ops: snd_compress_ops = snd_compress_ops {
    open: Some(sprd_platform_compr_open),
    free: Some(sprd_platform_compr_free),
    set_params: Some(sprd_platform_compr_set_params),
    trigger: Some(sprd_platform_compr_trigger),
    pointer: Some(sprd_platform_compr_pointer),
    copy: Some(sprd_platform_compr_copy),
    get_caps: Some(sprd_platform_compr_get_caps),
    get_codec_caps: Some(sprd_platform_compr_get_codec_caps),
};

// MODULE_DESCRIPTION("Spreadtrum ASoC Compress Platform Driver");
// MODULE_LICENSE("GPL v2");
// MODULE_ALIAS("platform:compress-platform");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
