// SPDX-License-Identifier: GPL-2.0
//
// Copyright(c) 2021-2022 Intel Corporation
//
// Author: Cezary Rojewski <cezary.rojewski@intel.com>
//

// C includes translated as external dependency intent:
// #include <sound/soc.h>
// #include <sound/hda_codec.h>
// #include "hda.h"

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::mem::MaybeUninit;

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_dai {
    pub dev: *mut device,
}

#[repr(C)]
pub struct snd_pcm_substream {
    pub stream: c_int,
    pub runtime: *mut snd_pcm_runtime,
}

#[repr(C)]
pub struct snd_pcm_runtime {
    pub format: c_uint,
    pub subformat: c_uint,
    pub channels: c_uint,
    pub rate: c_uint,
    pub private_data: *mut c_void,
}

#[repr(C)]
pub struct hdac_stream {
    pub stream_tag: c_uint,
}

#[repr(C)]
pub struct hda_codec_core {
    pub vendor_id: c_uint,
}

#[repr(C)]
pub struct hda_codec {
    pub core: hda_codec_core,
}

#[repr(C)]
pub struct hda_pcm_stream_ops {
    pub open: unsafe extern "C" fn(
        *mut hda_pcm_stream,
        *mut hda_codec,
        *mut snd_pcm_substream,
    ) -> c_int,
    pub close: unsafe extern "C" fn(
        *mut hda_pcm_stream,
        *mut hda_codec,
        *mut snd_pcm_substream,
    ) -> c_int,
}

#[repr(C)]
pub struct hda_pcm_stream {
    pub ops: hda_pcm_stream_ops,
    pub maxbps: c_uint,
}

#[repr(C)]
pub struct hda_pcm {
    pub name: *const c_char,
    pub stream: [hda_pcm_stream; 2],
}

#[repr(C)]
pub struct snd_soc_dai_ops {
    pub startup:
        Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_soc_dai) -> c_int>,
    pub shutdown: Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_soc_dai)>,
    pub hw_free:
        Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_soc_dai) -> c_int>,
    pub prepare:
        Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_soc_dai) -> c_int>,
}

unsafe extern "C" {
    fn dev_to_hda_codec(dev: *mut device) -> *mut hda_codec;
    fn snd_soc_dai_get_dma_data(
        dai: *mut snd_soc_dai,
        substream: *mut snd_pcm_substream,
    ) -> *mut hda_pcm_stream;
    fn snd_hda_codec_pcm_get(pcm: *mut hda_pcm);
    fn snd_hda_codec_pcm_put(pcm: *mut hda_pcm);
    fn snd_hda_codec_cleanup(
        codec: *mut hda_codec,
        stream_info: *mut hda_pcm_stream,
        substream: *mut snd_pcm_substream,
    );
    fn snd_hdac_stream_format_bits(format: c_uint, subformat: c_uint, maxbps: c_uint) -> c_uint;
    fn snd_hdac_stream_format(channels: c_uint, bits: c_uint, rate: c_uint) -> c_uint;
    fn snd_hda_codec_prepare(
        codec: *mut hda_codec,
        stream_info: *mut hda_pcm_stream,
        stream_tag: c_uint,
        format: c_uint,
        substream: *mut snd_pcm_substream,
    ) -> c_int;

    fn dev_dbg(dev: *mut device, fmt: *const c_char, ...);
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
}

const fn offset_of_hda_pcm_stream() -> usize {
    let uninit = MaybeUninit::<hda_pcm>::uninit();
    let base = uninit.as_ptr();
    unsafe { (&raw const (*base).stream) as usize - base as usize }
}

unsafe fn hda_pcm_from_stream(stream_info: *mut hda_pcm_stream, stream: c_int) -> *mut hda_pcm {
    let offset = offset_of_hda_pcm_stream()
        + (stream as usize).wrapping_mul(core::mem::size_of::<hda_pcm_stream>());
    (stream_info as *mut u8).wrapping_sub(offset) as *mut hda_pcm
}

unsafe extern "C" fn hda_codec_dai_startup(
    substream: *mut snd_pcm_substream,
    dai: *mut snd_soc_dai,
) -> c_int {
    let stream_info: *mut hda_pcm_stream;
    let codec: *mut hda_codec;
    let pcm: *mut hda_pcm;
    let ret: c_int;

    codec = dev_to_hda_codec((*dai).dev);
    stream_info = snd_soc_dai_get_dma_data(dai, substream);
    pcm = hda_pcm_from_stream(stream_info, (*substream).stream);

    dev_dbg(
        (*dai).dev,
        c"open stream codec: %08x, info: %p, pcm: %p %s substream: %p\n".as_ptr(),
        (*codec).core.vendor_id,
        stream_info,
        pcm,
        (*pcm).name,
        substream,
    );

    snd_hda_codec_pcm_get(pcm);

    ret = ((*stream_info).ops.open)(stream_info, codec, substream);
    if ret < 0 {
        dev_err(
            (*dai).dev,
            c"codec open failed: %d\n".as_ptr(),
            ret,
        );
        snd_hda_codec_pcm_put(pcm);
        return ret;
    }

    0
}

unsafe extern "C" fn hda_codec_dai_shutdown(
    substream: *mut snd_pcm_substream,
    dai: *mut snd_soc_dai,
) {
    let stream_info: *mut hda_pcm_stream;
    let codec: *mut hda_codec;
    let pcm: *mut hda_pcm;
    let ret: c_int;

    codec = dev_to_hda_codec((*dai).dev);
    stream_info = snd_soc_dai_get_dma_data(dai, substream);
    pcm = hda_pcm_from_stream(stream_info, (*substream).stream);

    dev_dbg(
        (*dai).dev,
        c"close stream codec: %08x, info: %p, pcm: %p %s substream: %p\n".as_ptr(),
        (*codec).core.vendor_id,
        stream_info,
        pcm,
        (*pcm).name,
        substream,
    );

    ret = ((*stream_info).ops.close)(stream_info, codec, substream);
    if ret < 0 {
        dev_err(
            (*dai).dev,
            c"codec close failed: %d\n".as_ptr(),
            ret,
        );
    }

    snd_hda_codec_pcm_put(pcm);
}

unsafe extern "C" fn hda_codec_dai_hw_free(
    substream: *mut snd_pcm_substream,
    dai: *mut snd_soc_dai,
) -> c_int {
    let stream_info: *mut hda_pcm_stream;
    let codec: *mut hda_codec;

    codec = dev_to_hda_codec((*dai).dev);
    stream_info = snd_soc_dai_get_dma_data(dai, substream);

    snd_hda_codec_cleanup(codec, stream_info, substream);

    0
}

unsafe extern "C" fn hda_codec_dai_prepare(
    substream: *mut snd_pcm_substream,
    dai: *mut snd_soc_dai,
) -> c_int {
    let runtime: *mut snd_pcm_runtime = (*substream).runtime;
    let stream_info: *mut hda_pcm_stream;
    let stream: *mut hdac_stream;
    let codec: *mut hda_codec;
    let format: c_uint;
    let bits: c_uint;
    let ret: c_int;

    codec = dev_to_hda_codec((*dai).dev);
    stream = (*(*substream).runtime).private_data as *mut hdac_stream;
    stream_info = snd_soc_dai_get_dma_data(dai, substream);

    bits = snd_hdac_stream_format_bits((*runtime).format, (*runtime).subformat, (*stream_info).maxbps);
    format = snd_hdac_stream_format((*runtime).channels, bits, (*runtime).rate);

    ret = snd_hda_codec_prepare(codec, stream_info, (*stream).stream_tag, format, substream);
    if ret < 0 {
        dev_err(
            (*dai).dev,
            c"codec prepare failed: %d\n".as_ptr(),
            ret,
        );
        return ret;
    }

    0
}

#[unsafe(no_mangle)]
pub static snd_soc_hda_codec_dai_ops: snd_soc_dai_ops = snd_soc_dai_ops {
    startup: Some(hda_codec_dai_startup),
    shutdown: Some(hda_codec_dai_shutdown),
    hw_free: Some(hda_codec_dai_hw_free),
    prepare: Some(hda_codec_dai_prepare),
};

// EXPORT_SYMBOL_GPL(snd_soc_hda_codec_dai_ops);

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
