// SPDX-License-Identifier: GPL-2.0-only
/*
 *  sst_mfld_platform.c - Intel MID Platform driver
 *
 *  Copyright (C) 2010-2014 Intel Corp
 *  Author: Vinod Koul <vinod.koul@intel.com>
 *  ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
 *
 * ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
 */

use core::ffi::{c_int, c_ulong, c_void};
use core::mem::size_of;
use core::ptr;

/* Includes in the C source:
 * <linux/slab.h>, <linux/io.h>, <linux/module.h>, <sound/core.h>,
 * <sound/pcm.h>, <sound/pcm_params.h>, <sound/soc.h>,
 * <sound/compress_driver.h>, <asm/div64.h>, "sst-mfld-platform.h"
 */

const ENOMEM: c_int = 12;
const ENODEV: c_int = 19;
const EINVAL: c_int = 22;

const SST_PLATFORM_INIT: c_int = 0;
const SND_AUDIOCODEC_MP3: c_int = 1;
const SND_AUDIOCODEC_AAC: c_int = 2;
const SND_AUDIOSTREAMFORMAT_MP4ADTS: c_int = 1;
const SND_AUDIOSTREAMFORMAT_RAW: c_int = 2;
const SST_CODEC_TYPE_MP3: c_int = 1;
const SST_CODEC_TYPE_AAC: c_int = 2;
const AAC_BIT_STREAM_ADTS: c_int = 1;
const AAC_BIT_STREAM_RAW: c_int = 2;
const SNDRV_PCM_TRIGGER_START: c_int = 0;
const SNDRV_PCM_TRIGGER_STOP: c_int = 1;
const SNDRV_PCM_TRIGGER_PAUSE_PUSH: c_int = 3;
const SNDRV_PCM_TRIGGER_PAUSE_RELEASE: c_int = 4;
const SND_COMPR_TRIGGER_DRAIN: c_int = 7;
const SND_COMPR_TRIGGER_PARTIAL_DRAIN: c_int = 8;

#[repr(C)]
pub struct snd_soc_component {
    _private: [u8; 0],
}

#[repr(C)]
pub struct module {
    _private: [u8; 0],
}

#[repr(C)]
pub struct device_driver {
    pub owner: *mut module,
}

#[repr(C)]
pub struct device {
    pub driver: *mut device_driver,
}

#[repr(C)]
pub struct snd_compr_stream {
    pub runtime: *mut snd_compr_runtime,
}

#[repr(C)]
pub struct snd_compr_runtime {
    pub private_data: *mut c_void,
    pub buffer: *mut c_void,
    pub buffer_size: usize,
    pub fragment_size: usize,
}

#[repr(C)]
pub struct snd_codec {
    pub id: c_int,
    pub ch_in: c_int,
    pub format: c_int,
    pub sample_rate: c_int,
}

#[repr(C)]
pub struct snd_compr_params {
    pub codec: snd_codec,
}

#[repr(C)]
pub struct snd_compr_tstamp64 {
    pub copied_total: u64,
    pub byte_offset: u32,
}

#[repr(C)]
pub struct snd_compr_caps {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_compr_codec_caps {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_compr_metadata {
    _private: [u8; 0],
}

#[repr(C)]
pub struct sst_data {
    pub dev: *mut device,
    pub compr_ops: *mut sst_compr_ops,
}

#[repr(C)]
pub struct sst_runtime_stream {
    pub status_lock: spinlock_t,
    pub compr_ops: *mut sst_compr_ops,
    pub id: c_int,
    pub bytes_written: usize,
}

#[repr(C)]
pub struct spinlock_t {
    _private: [u8; 0],
}

#[repr(C)]
pub struct sst_compr_ops {
    pub power: Option<unsafe extern "C" fn(*mut device, bool)>,
    pub close: Option<unsafe extern "C" fn(*mut device, c_int) -> c_int>,
    pub open: Option<unsafe extern "C" fn(*mut device, *mut snd_sst_params, *mut sst_compress_cb) -> c_int>,
    pub stream_start: Option<unsafe extern "C" fn(*mut device, c_int) -> c_int>,
    pub stream_drop: Option<unsafe extern "C" fn(*mut device, c_int) -> c_int>,
    pub stream_drain: Option<unsafe extern "C" fn(*mut device, c_int) -> c_int>,
    pub stream_partial_drain: Option<unsafe extern "C" fn(*mut device, c_int) -> c_int>,
    pub stream_pause: Option<unsafe extern "C" fn(*mut device, c_int) -> c_int>,
    pub stream_pause_release: Option<unsafe extern "C" fn(*mut device, c_int) -> c_int>,
    pub tstamp: Option<unsafe extern "C" fn(*mut device, c_int, *mut snd_compr_tstamp64)>,
    pub ack: Option<unsafe extern "C" fn(*mut device, c_int, c_ulong)>,
    pub get_caps: Option<unsafe extern "C" fn(*mut snd_compr_caps) -> c_int>,
    pub get_codec_caps: Option<unsafe extern "C" fn(*mut snd_compr_codec_caps) -> c_int>,
    pub set_metadata: Option<unsafe extern "C" fn(*mut device, c_int, *mut snd_compr_metadata) -> c_int>,
}

#[repr(C)]
pub struct snd_sst_params {
    pub codec: c_int,
    pub sparams: sst_stream_params,
    pub aparams: sst_alloc_params,
}

#[repr(C)]
pub struct sst_stream_params {
    pub uc: sst_stream_union,
}

#[repr(C)]
pub union sst_stream_union {
    pub mp3_params: sst_mp3_params,
    pub aac_params: sst_aac_params,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sst_mp3_params {
    pub num_chan: c_int,
    pub pcm_wd_sz: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sst_aac_params {
    pub num_chan: c_int,
    pub pcm_wd_sz: c_int,
    pub bs_format: c_int,
    pub externalsr: c_int,
}

#[repr(C)]
pub struct sst_alloc_params {
    pub ring_buf_info: [sst_ring_buf_info; 1],
    pub sg_count: c_int,
    pub frag_size: usize,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sst_ring_buf_info {
    pub addr: usize,
    pub size: usize,
}

#[repr(C)]
pub struct sst_compress_cb {
    pub param: *mut c_void,
    pub compr_cb: Option<unsafe extern "C" fn(*mut c_void)>,
    pub drain_cb_param: *mut c_void,
    pub drain_notify: Option<unsafe extern "C" fn(*mut c_void)>,
}

#[repr(C)]
pub struct snd_compress_ops {
    pub open: Option<unsafe extern "C" fn(*mut snd_soc_component, *mut snd_compr_stream) -> c_int>,
    pub free: Option<unsafe extern "C" fn(*mut snd_soc_component, *mut snd_compr_stream) -> c_int>,
    pub set_params: Option<unsafe extern "C" fn(*mut snd_soc_component, *mut snd_compr_stream, *mut snd_compr_params) -> c_int>,
    pub set_metadata: Option<unsafe extern "C" fn(*mut snd_soc_component, *mut snd_compr_stream, *mut snd_compr_metadata) -> c_int>,
    pub trigger: Option<unsafe extern "C" fn(*mut snd_soc_component, *mut snd_compr_stream, c_int) -> c_int>,
    pub pointer: Option<unsafe extern "C" fn(*mut snd_soc_component, *mut snd_compr_stream, *mut snd_compr_tstamp64) -> c_int>,
    pub ack: Option<unsafe extern "C" fn(*mut snd_soc_component, *mut snd_compr_stream, usize) -> c_int>,
    pub get_caps: Option<unsafe extern "C" fn(*mut snd_soc_component, *mut snd_compr_stream, *mut snd_compr_caps) -> c_int>,
    pub get_codec_caps: Option<unsafe extern "C" fn(*mut snd_soc_component, *mut snd_compr_stream, *mut snd_compr_codec_caps) -> c_int>,
}

unsafe extern "C" {
    static mut sst: *mut sst_data;

    fn pr_debug(fmt: *const u8, ...);
    fn pr_err(fmt: *const u8, ...);
    fn kzalloc(size: usize, flags: c_int) -> *mut c_void;
    fn kfree(ptr: *mut c_void);
    fn spin_lock_init(lock: *mut spinlock_t);
    fn try_module_get(module: *mut module) -> bool;
    fn module_put(module: *mut module);
    fn snd_compr_fragment_elapsed(cstream: *mut snd_compr_stream);
    fn snd_compr_drain_notify(cstream: *mut snd_compr_stream);
    fn snd_soc_component_get_drvdata(component: *mut snd_soc_component) -> *mut sst_data;
    fn sst_set_stream_status(stream: *mut sst_runtime_stream, status: c_int);
    fn sst_fill_stream_params(
        cstream: *mut snd_compr_stream,
        ctx: *mut sst_data,
        str_params: *mut snd_sst_params,
        compressed: bool,
    ) -> c_int;
    fn virt_to_phys(address: *mut c_void) -> usize;
}

/* compress stream operations */
unsafe extern "C" fn sst_compr_fragment_elapsed(arg: *mut c_void) {
    let cstream: *mut snd_compr_stream = arg as *mut snd_compr_stream;

    pr_debug(c"fragment elapsed by driver\n".as_ptr() as *const u8);
    if !cstream.is_null() {
        snd_compr_fragment_elapsed(cstream);
    }
}

unsafe extern "C" fn sst_drain_notify(arg: *mut c_void) {
    let cstream: *mut snd_compr_stream = arg as *mut snd_compr_stream;

    pr_debug(c"drain notify by driver\n".as_ptr() as *const u8);
    if !cstream.is_null() {
        snd_compr_drain_notify(cstream);
    }
}

unsafe extern "C" fn sst_platform_compr_open(
    _component: *mut snd_soc_component,
    cstream: *mut snd_compr_stream,
) -> c_int {
    let ret_val: c_int;
    let runtime: *mut snd_compr_runtime = (*cstream).runtime;
    let stream: *mut sst_runtime_stream;

    stream = kzalloc(size_of::<sst_runtime_stream>(), 0) as *mut sst_runtime_stream;
    if stream.is_null() {
        return -ENOMEM;
    }

    spin_lock_init(ptr::addr_of_mut!((*stream).status_lock));

    /* get the sst ops */
    if sst.is_null()
        || (*sst).dev.is_null()
        || (*(*sst).dev).driver.is_null()
        || !try_module_get((*(*(*sst).dev).driver).owner)
    {
        pr_err(c"no device available to run\n".as_ptr() as *const u8);
        ret_val = -ENODEV;
        kfree(stream as *mut c_void);
        return ret_val;
    }
    (*stream).compr_ops = (*sst).compr_ops;
    (*stream).id = 0;

    /* Turn on LPE */
    if let Some(power) = (*(*sst).compr_ops).power {
        power((*sst).dev, true);
    }

    sst_set_stream_status(stream, SST_PLATFORM_INIT);
    (*runtime).private_data = stream as *mut c_void;
    0
}

unsafe extern "C" fn sst_platform_compr_free(
    _component: *mut snd_soc_component,
    cstream: *mut snd_compr_stream,
) -> c_int {
    let stream: *mut sst_runtime_stream;
    let mut ret_val: c_int = 0;
    let str_id: c_int;

    stream = (*(*cstream).runtime).private_data as *mut sst_runtime_stream;
    /* Turn off LPE */
    if let Some(power) = (*(*sst).compr_ops).power {
        power((*sst).dev, false);
    }

    /*need to check*/
    str_id = (*stream).id;
    if str_id != 0 {
        if let Some(close) = (*(*stream).compr_ops).close {
            ret_val = close((*sst).dev, str_id);
        }
    }
    module_put((*(*(*sst).dev).driver).owner);
    kfree(stream as *mut c_void);
    pr_debug(c"%s: %d\n".as_ptr() as *const u8, c"sst_platform_compr_free".as_ptr(), ret_val);
    0
}

unsafe extern "C" fn sst_platform_compr_set_params(
    component: *mut snd_soc_component,
    cstream: *mut snd_compr_stream,
    params: *mut snd_compr_params,
) -> c_int {
    let stream: *mut sst_runtime_stream;
    let mut retval: c_int;
    let mut str_params: snd_sst_params = core::mem::zeroed();
    let mut cb: sst_compress_cb = core::mem::zeroed();
    let ctx: *mut sst_data = snd_soc_component_get_drvdata(component);

    stream = (*(*cstream).runtime).private_data as *mut sst_runtime_stream;
    /* construct fw structure for this*/

    /* fill the device type and stream id to pass to SST driver */
    retval = sst_fill_stream_params(cstream, ctx, ptr::addr_of_mut!(str_params), true);
    pr_debug(c"compr_set_params: fill stream params ret_val = 0x%x\n".as_ptr() as *const u8, retval);
    if retval < 0 {
        return retval;
    }

    match (*params).codec.id {
        SND_AUDIOCODEC_MP3 => {
            str_params.codec = SST_CODEC_TYPE_MP3;
            str_params.sparams.uc.mp3_params.num_chan = (*params).codec.ch_in;
            str_params.sparams.uc.mp3_params.pcm_wd_sz = 16;
        }

        SND_AUDIOCODEC_AAC => {
            str_params.codec = SST_CODEC_TYPE_AAC;
            str_params.sparams.uc.aac_params.num_chan = (*params).codec.ch_in;
            str_params.sparams.uc.aac_params.pcm_wd_sz = 16;
            if (*params).codec.format == SND_AUDIOSTREAMFORMAT_MP4ADTS {
                str_params.sparams.uc.aac_params.bs_format = AAC_BIT_STREAM_ADTS;
            } else if (*params).codec.format == SND_AUDIOSTREAMFORMAT_RAW {
                str_params.sparams.uc.aac_params.bs_format = AAC_BIT_STREAM_RAW;
            } else {
                pr_err(c"Undefined format%d\n".as_ptr() as *const u8, (*params).codec.format);
                return -EINVAL;
            }
            str_params.sparams.uc.aac_params.externalsr = (*params).codec.sample_rate;
        }

        _ => {
            pr_err(c"codec not supported, id =%d\n".as_ptr() as *const u8, (*params).codec.id);
            return -EINVAL;
        }
    }

    str_params.aparams.ring_buf_info[0].addr = virt_to_phys((*(*cstream).runtime).buffer);
    str_params.aparams.ring_buf_info[0].size = (*(*cstream).runtime).buffer_size;
    str_params.aparams.sg_count = 1;
    str_params.aparams.frag_size = (*(*cstream).runtime).fragment_size;

    cb.param = cstream as *mut c_void;
    cb.compr_cb = Some(sst_compr_fragment_elapsed);
    cb.drain_cb_param = cstream as *mut c_void;
    cb.drain_notify = Some(sst_drain_notify);

    if let Some(open) = (*(*stream).compr_ops).open {
        retval = open((*sst).dev, ptr::addr_of_mut!(str_params), ptr::addr_of_mut!(cb));
    }
    if retval < 0 {
        pr_err(c"stream allocation failed %d\n".as_ptr() as *const u8, retval);
        return retval;
    }

    (*stream).id = retval;
    0
}

unsafe extern "C" fn sst_platform_compr_trigger(
    _component: *mut snd_soc_component,
    cstream: *mut snd_compr_stream,
    cmd: c_int,
) -> c_int {
    let stream: *mut sst_runtime_stream = (*(*cstream).runtime).private_data as *mut sst_runtime_stream;

    match cmd {
        SNDRV_PCM_TRIGGER_START => {
            if let Some(stream_start) = (*(*stream).compr_ops).stream_start {
                return stream_start((*sst).dev, (*stream).id);
            }
        }
        SNDRV_PCM_TRIGGER_STOP => {
            if let Some(stream_drop) = (*(*stream).compr_ops).stream_drop {
                return stream_drop((*sst).dev, (*stream).id);
            }
        }
        SND_COMPR_TRIGGER_DRAIN => {
            if let Some(stream_drain) = (*(*stream).compr_ops).stream_drain {
                return stream_drain((*sst).dev, (*stream).id);
            }
        }
        SND_COMPR_TRIGGER_PARTIAL_DRAIN => {
            if let Some(stream_partial_drain) = (*(*stream).compr_ops).stream_partial_drain {
                return stream_partial_drain((*sst).dev, (*stream).id);
            }
        }
        SNDRV_PCM_TRIGGER_PAUSE_PUSH => {
            if let Some(stream_pause) = (*(*stream).compr_ops).stream_pause {
                return stream_pause((*sst).dev, (*stream).id);
            }
        }
        SNDRV_PCM_TRIGGER_PAUSE_RELEASE => {
            if let Some(stream_pause_release) = (*(*stream).compr_ops).stream_pause_release {
                return stream_pause_release((*sst).dev, (*stream).id);
            }
        }
        _ => {}
    }
    -EINVAL
}

unsafe extern "C" fn sst_platform_compr_pointer(
    _component: *mut snd_soc_component,
    cstream: *mut snd_compr_stream,
    tstamp: *mut snd_compr_tstamp64,
) -> c_int {
    let stream: *mut sst_runtime_stream;
    let temp_copied_total: u64 = (*tstamp).copied_total;

    stream = (*(*cstream).runtime).private_data as *mut sst_runtime_stream;
    if let Some(tstamp_fn) = (*(*stream).compr_ops).tstamp {
        tstamp_fn((*sst).dev, (*stream).id, tstamp);
    }
    (*tstamp).byte_offset = (temp_copied_total % (*(*cstream).runtime).buffer_size as u64) as u32;
    pr_debug(c"calc bytes offset/copied bytes as %u\n".as_ptr() as *const u8, (*tstamp).byte_offset);
    0
}

unsafe extern "C" fn sst_platform_compr_ack(
    _component: *mut snd_soc_component,
    cstream: *mut snd_compr_stream,
    bytes: usize,
) -> c_int {
    let stream: *mut sst_runtime_stream;

    stream = (*(*cstream).runtime).private_data as *mut sst_runtime_stream;
    if let Some(ack) = (*(*stream).compr_ops).ack {
        ack((*sst).dev, (*stream).id, bytes as c_ulong);
    }
    (*stream).bytes_written = (*stream).bytes_written.wrapping_add(bytes);

    0
}

unsafe extern "C" fn sst_platform_compr_get_caps(
    _component: *mut snd_soc_component,
    cstream: *mut snd_compr_stream,
    caps: *mut snd_compr_caps,
) -> c_int {
    let stream: *mut sst_runtime_stream = (*(*cstream).runtime).private_data as *mut sst_runtime_stream;

    if let Some(get_caps) = (*(*stream).compr_ops).get_caps {
        return get_caps(caps);
    }
    -EINVAL
}

unsafe extern "C" fn sst_platform_compr_get_codec_caps(
    _component: *mut snd_soc_component,
    cstream: *mut snd_compr_stream,
    codec: *mut snd_compr_codec_caps,
) -> c_int {
    let stream: *mut sst_runtime_stream = (*(*cstream).runtime).private_data as *mut sst_runtime_stream;

    if let Some(get_codec_caps) = (*(*stream).compr_ops).get_codec_caps {
        return get_codec_caps(codec);
    }
    -EINVAL
}

unsafe extern "C" fn sst_platform_compr_set_metadata(
    _component: *mut snd_soc_component,
    cstream: *mut snd_compr_stream,
    metadata: *mut snd_compr_metadata,
) -> c_int {
    let stream: *mut sst_runtime_stream = (*(*cstream).runtime).private_data as *mut sst_runtime_stream;

    if let Some(set_metadata) = (*(*stream).compr_ops).set_metadata {
        return set_metadata((*sst).dev, (*stream).id, metadata);
    }
    -EINVAL
}

pub static sst_platform_compress_ops: snd_compress_ops = snd_compress_ops {
    open: Some(sst_platform_compr_open),
    free: Some(sst_platform_compr_free),
    set_params: Some(sst_platform_compr_set_params),
    set_metadata: Some(sst_platform_compr_set_metadata),
    trigger: Some(sst_platform_compr_trigger),
    pointer: Some(sst_platform_compr_pointer),
    ack: Some(sst_platform_compr_ack),
    get_caps: Some(sst_platform_compr_get_caps),
    get_codec_caps: Some(sst_platform_compr_get_codec_caps),
};

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
