// SPDX-License-Identifier: (GPL-2.0-only OR BSD-3-Clause)
//
// This file is provided under a dual BSD/GPLv2 license.  When using or
// redistributing this file, you may do so under either license.
//
// Copyright(c) 2025 Intel Corporation.
//

/*
 * Hardware interface for SoundWire BPT support with HDA DMA
 */

// C dependencies:
// <linux/lcm.h>
// <sound/hdaudio_ext.h>
// <sound/hda-mlink.h>
// <sound/hda-sdw-bpt.h>
// <sound/sof.h>
// <sound/sof/ipc4/header.h>
// ../ops.h
// ../sof-priv.h
// ../ipc4-priv.h
// hda.h

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};
use core::mem::size_of;

type u32 = u32;
type bool_ = bool;
type snd_pcm_uframes_t = c_ulong;

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_dma_buffer {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_sof_pdata {
    pub ipc_type: c_int,
}

#[repr(C)]
pub struct snd_sof_dev {
    pub private: *mut c_void,
    pub pdata: *mut snd_sof_pdata,
    pub dev: *mut device,
    pub ipc: *mut c_void,
    pub dspless_mode_selected: bool_,
}

#[repr(C)]
pub struct sof_ipc4_fw_data {
    pub num_playback_streams: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sof_ipc4_msg_data {
    pub data: [u32; 0],
}

#[repr(C)]
pub struct sof_ipc4_msg {
    pub primary: u32,
    pub extension: u32,
}

#[repr(C)]
pub struct hdac_stream {
    pub index: c_int,
    pub direction: c_int,
    pub stream_tag: c_int,
}

#[repr(C)]
pub struct hdac_ext_stream {
    pub hstream: hdac_stream,
}

#[repr(C)]
pub struct hdac_bus {
    _private: [u8; 0],
}

#[repr(C)]
pub struct hdac_ext_link {
    _private: [u8; 0],
}

#[repr(C)]
pub struct completion {
    _private: [u8; 0],
}

#[repr(C)]
pub struct sof_intel_hda_stream {
    pub hext_stream: hdac_ext_stream,
    pub ioc: completion,
}

const EOPNOTSUPP: c_int = 95;
const EINVAL: c_int = 22;
const ETIMEDOUT: c_int = 110;

const SOF_IPC_TYPE_4: c_int = 4;
const SOF_IPC4_PIPE_RUNNING: c_int = 1;
const SOF_IPC4_PIPE_PAUSED: c_int = 2;
const SOF_IPC4_PIPE_RESET: c_int = 3;

const SOF_IPC4_GLB_CHAIN_DMA: u32 = 0;
const SOF_IPC4_MSG_REQUEST: u32 = 0;
const SOF_IPC4_FW_GEN_MSG: u32 = 0;
const SOF_IPC4_GLB_CHAIN_DMA_ALLOCATE_MASK: u32 = 0;
const SOF_IPC4_GLB_CHAIN_DMA_ENABLE_MASK: u32 = 0;

const SNDRV_PCM_STREAM_PLAYBACK: c_int = 0;
const SNDRV_PCM_STREAM_CAPTURE: c_int = 1;
const SNDRV_PCM_TRIGGER_START: c_int = 0;
const SNDRV_PCM_TRIGGER_STOP: c_int = 1;

const HDA_CL_STREAM_FORMAT: c_uint = 0;
const AC_FMT_MULT_SHIFT: c_uint = 0;
const HDA_DSP_PP_BAR: c_uint = 0;
const SOF_HDA_REG_PP_PPCTL: c_uint = 0;

const BPT_FREQUENCY: c_uint = 192000; /* The max rate defined in rate_bits[] hdac_device.c */
const BPT_MULTIPLIER: c_uint = (BPT_FREQUENCY / 48000) - 1;
const BPT_CHAIN_DMA_FIFO_MS: c_uint = 10;

unsafe extern "C" {
    fn dev_get_drvdata(dev: *mut device) -> *mut c_void;
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn dev_dbg(dev: *mut device, fmt: *const c_char, ...);
    fn sof_ipc_tx_message_no_reply(ipc: *mut c_void, msg: *mut sof_ipc4_msg, msg_bytes: c_uint)
        -> c_int;
    fn snd_sof_boot_dsp_firmware(sdev: *mut snd_sof_dev) -> c_int;
    fn hda_data_stream_prepare(
        dev: *mut device,
        format: c_uint,
        size: u32,
        dmab: *mut snd_dma_buffer,
        no_irq: bool_,
        direction: c_int,
        can_sleep: bool_,
        force: bool_,
    ) -> *mut hdac_ext_stream;
    fn hda_data_stream_cleanup(
        dev: *mut device,
        dmab: *mut snd_dma_buffer,
        no_irq: bool_,
        stream: *mut hdac_ext_stream,
        can_sleep: bool_,
        force: bool_,
    ) -> c_int;
    fn snd_sof_dsp_update_bits(
        sdev: *mut snd_sof_dev,
        bar: c_uint,
        offset: c_uint,
        mask: u32,
        value: u32,
    ) -> c_int;
    fn snd_hdac_ext_stream_reset(stream: *mut hdac_ext_stream);
    fn snd_hdac_ext_stream_setup(stream: *mut hdac_ext_stream, format: c_uint);
    fn snd_hdac_ext_stream_start(stream: *mut hdac_ext_stream);
    fn snd_hdac_ext_stream_clear(stream: *mut hdac_ext_stream);
    fn sof_to_bus(sdev: *mut snd_sof_dev) -> *mut hdac_bus;
    fn hdac_bus_eml_sdw_get_hlink(bus: *mut hdac_bus) -> *mut hdac_ext_link;
    fn snd_hdac_ext_bus_link_set_stream_id(hlink: *mut hdac_ext_link, stream_tag: c_int);
    fn snd_hdac_ext_bus_link_clear_stream_id(hlink: *mut hdac_ext_link, stream_tag: c_int);
    fn hda_cl_trigger(dev: *mut device, stream: *mut hdac_ext_stream, cmd: c_int) -> c_int;
    fn lcm(a: c_uint, b: c_uint) -> c_uint;
    fn hdac_bus_eml_sdw_map_stream_ch(
        bus: *mut hdac_bus,
        link_id: c_int,
        pdi_id: c_int,
        ch_mask: c_uint,
        stream_id: c_int,
        direction: c_int,
    ) -> c_int;
    fn wait_for_completion_timeout(x: *mut completion, timeout: c_ulong) -> c_ulong;
    fn msecs_to_jiffies(m: c_uint) -> c_ulong;
    fn hda_dsp_stream_get_position(
        stream: *mut hdac_stream,
        direction: c_int,
        can_sleep: bool_,
    ) -> snd_pcm_uframes_t;
    fn usleep_range(min: c_ulong, max: c_ulong);
}

#[inline]
const fn BIT(n: c_int) -> u32 {
    1u32 << (n as u32)
}

#[inline]
const fn DIV_ROUND_UP(n: c_uint, d: c_uint) -> c_uint {
    (n + d - 1) / d
}

#[inline]
const fn GENMASK(h: c_uint, l: c_uint) -> c_uint {
    let all = !0u32;
    (all << l) & (all >> (31 - h))
}

#[inline]
unsafe fn IS_ERR(ptr: *mut hdac_ext_stream) -> bool {
    (ptr as isize) < 0 && (ptr as isize) >= -4095
}

#[inline]
unsafe fn PTR_ERR(ptr: *mut hdac_ext_stream) -> c_int {
    ptr as isize as c_int
}

#[inline]
unsafe fn hdac_stream(stream: *mut hdac_ext_stream) -> *mut hdac_stream {
    &mut (*stream).hstream
}

#[inline]
const fn SOF_IPC4_MSG_TYPE_SET(x: u32) -> u32 {
    x
}

#[inline]
const fn SOF_IPC4_MSG_DIR(x: u32) -> u32 {
    x
}

#[inline]
const fn SOF_IPC4_MSG_TARGET(x: u32) -> u32 {
    x
}

#[inline]
const fn SOF_IPC4_GLB_CHAIN_DMA_HOST_ID(x: c_int) -> u32 {
    x as u32
}

#[inline]
const fn SOF_IPC4_GLB_CHAIN_DMA_LINK_ID(x: c_int) -> u32 {
    x as u32
}

#[inline]
const fn SOF_IPC4_GLB_EXT_CHAIN_DMA_FIFO_SIZE(x: usize) -> u32 {
    x as u32
}

/*
 * This routine is directly inspired by sof_ipc4_chain_dma_trigger(),
 * with major simplifications since there are no pipelines defined
 * and no dependency on ALSA hw_params
 */
unsafe fn chain_dma_trigger(
    sdev: *mut snd_sof_dev,
    stream_tag: c_uint,
    direction: c_int,
    state: c_int,
) -> c_int {
    let ipc4_data = (*sdev).private as *mut sof_ipc4_fw_data;
    let allocate: bool_;
    let enable: bool_;
    let set_fifo_size: bool_;
    let mut msg = sof_ipc4_msg {
        primary: 0,
        extension: 0,
    };
    let mut dma_id: c_int;

    if (*(*sdev).pdata).ipc_type != SOF_IPC_TYPE_4 {
        return -EOPNOTSUPP;
    }

    match state {
        SOF_IPC4_PIPE_RUNNING => {
            /* Allocate and start the chain */
            allocate = true;
            enable = true;
            set_fifo_size = true;
        }
        SOF_IPC4_PIPE_PAUSED => {
            /* Stop the chain */
            allocate = true;
            enable = false;
            set_fifo_size = false;
        }
        SOF_IPC4_PIPE_RESET => {
            /* Deallocate chain resources and remove the chain */
            allocate = false;
            enable = false;
            set_fifo_size = false;
        }
        _ => {
            dev_err((*sdev).dev, c"Unexpected state %d".as_ptr(), state);
            return -EINVAL;
        }
    }

    msg.primary = SOF_IPC4_MSG_TYPE_SET(SOF_IPC4_GLB_CHAIN_DMA);
    msg.primary |= SOF_IPC4_MSG_DIR(SOF_IPC4_MSG_REQUEST);
    msg.primary |= SOF_IPC4_MSG_TARGET(SOF_IPC4_FW_GEN_MSG);

    /* for BPT/BRA we can use the same stream tag for host and link */
    dma_id = stream_tag as c_int - 1;
    if direction == SNDRV_PCM_STREAM_CAPTURE {
        dma_id += (*ipc4_data).num_playback_streams;
    }

    msg.primary |= SOF_IPC4_GLB_CHAIN_DMA_HOST_ID(dma_id);
    msg.primary |= SOF_IPC4_GLB_CHAIN_DMA_LINK_ID(dma_id);

    /* For BPT/BRA we use 32 bits so SCS is not set */

    /* CHAIN DMA needs at least 2ms */
    if set_fifo_size {
        msg.extension |= SOF_IPC4_GLB_EXT_CHAIN_DMA_FIFO_SIZE(
            (BPT_FREQUENCY / 1000 * BPT_CHAIN_DMA_FIFO_MS) as usize * size_of::<u32>(),
        );
    }

    if allocate {
        msg.primary |= SOF_IPC4_GLB_CHAIN_DMA_ALLOCATE_MASK;
    }

    if enable {
        msg.primary |= SOF_IPC4_GLB_CHAIN_DMA_ENABLE_MASK;
    }

    sof_ipc_tx_message_no_reply((*sdev).ipc, &mut msg, 0)
}

unsafe fn hda_sdw_bpt_dma_prepare(
    dev: *mut device,
    sdw_bpt_stream: *mut *mut hdac_ext_stream,
    dmab_bdl: *mut snd_dma_buffer,
    bpt_num_bytes: u32,
    num_channels: c_uint,
    direction: c_int,
) -> c_int {
    let sdev = dev_get_drvdata(dev) as *mut snd_sof_dev;
    let bpt_stream: *mut hdac_ext_stream;
    let mut format = HDA_CL_STREAM_FORMAT;

    if !(*sdev).dspless_mode_selected {
        let ret: c_int;

        /*
         * Make sure that the DSP is booted up, which might not be the
         * case if the on-demand DSP boot is used
         */
        ret = snd_sof_boot_dsp_firmware(sdev);
        if ret != 0 {
            return ret;
        }
    }
    /*
     * the baseline format needs to be adjusted to
     * bandwidth requirements
     */
    format |= num_channels - 1;
    format |= BPT_MULTIPLIER << AC_FMT_MULT_SHIFT;

    dev_dbg(dev, c"direction %d format_val %#x\n".as_ptr(), direction, format);

    bpt_stream = hda_data_stream_prepare(
        dev,
        format,
        bpt_num_bytes,
        dmab_bdl,
        false,
        direction,
        false,
        true,
    );
    if IS_ERR(bpt_stream) {
        dev_err(
            (*sdev).dev,
            c"%s: SDW BPT DMA prepare failed: dir %d\n".as_ptr(),
            c"hda_sdw_bpt_dma_prepare".as_ptr(),
            direction,
        );
        return PTR_ERR(bpt_stream);
    }
    *sdw_bpt_stream = bpt_stream;

    if !(*sdev).dspless_mode_selected {
        let hstream: *mut hdac_stream;
        let mask: u32;

        /* decouple host and link DMA if the DSP is used */
        hstream = &mut (*bpt_stream).hstream;
        mask = BIT((*hstream).index);

        snd_sof_dsp_update_bits(sdev, HDA_DSP_PP_BAR, SOF_HDA_REG_PP_PPCTL, mask, mask);

        snd_hdac_ext_stream_reset(bpt_stream);

        snd_hdac_ext_stream_setup(bpt_stream, format);
    }

    if (*hdac_stream(bpt_stream)).direction == SNDRV_PCM_STREAM_PLAYBACK {
        let bus = sof_to_bus(sdev);
        let hlink: *mut hdac_ext_link;
        let stream_tag: c_int;

        stream_tag = (*hdac_stream(bpt_stream)).stream_tag;
        hlink = hdac_bus_eml_sdw_get_hlink(bus);

        snd_hdac_ext_bus_link_set_stream_id(hlink, stream_tag);
    }
    0
}

unsafe fn hda_sdw_bpt_dma_deprepare(
    dev: *mut device,
    sdw_bpt_stream: *mut hdac_ext_stream,
    dmab_bdl: *mut snd_dma_buffer,
) -> c_int {
    let sdev = dev_get_drvdata(dev) as *mut snd_sof_dev;
    let hstream: *mut hdac_stream;
    let mask: u32;
    let mut ret: c_int;

    ret = hda_data_stream_cleanup((*sdev).dev, dmab_bdl, false, sdw_bpt_stream, false, true);
    if ret < 0 {
        dev_err(
            (*sdev).dev,
            c"%s: SDW BPT DMA cleanup failed\n".as_ptr(),
            c"hda_sdw_bpt_dma_deprepare".as_ptr(),
        );
        return ret;
    }

    if (*hdac_stream(sdw_bpt_stream)).direction == SNDRV_PCM_STREAM_PLAYBACK {
        let bus = sof_to_bus(sdev);
        let hlink: *mut hdac_ext_link;
        let stream_tag: c_int;

        stream_tag = (*hdac_stream(sdw_bpt_stream)).stream_tag;
        hlink = hdac_bus_eml_sdw_get_hlink(bus);

        snd_hdac_ext_bus_link_clear_stream_id(hlink, stream_tag);
    }

    if !(*sdev).dspless_mode_selected {
        /* Release CHAIN_DMA resources */
        ret = chain_dma_trigger(
            sdev,
            (*hdac_stream(sdw_bpt_stream)).stream_tag as c_uint,
            (*hdac_stream(sdw_bpt_stream)).direction,
            SOF_IPC4_PIPE_RESET,
        );
        if ret < 0 {
            dev_err(
                (*sdev).dev,
                c"%s: chain_dma_trigger PIPE_RESET failed: %d\n".as_ptr(),
                c"hda_sdw_bpt_dma_deprepare".as_ptr(),
                ret,
            );
        }

        /* couple host and link DMA */
        hstream = &mut (*sdw_bpt_stream).hstream;
        mask = BIT((*hstream).index);

        snd_sof_dsp_update_bits(sdev, HDA_DSP_PP_BAR, SOF_HDA_REG_PP_PPCTL, mask, 0);
    }

    0
}

unsafe fn hda_sdw_bpt_dma_enable(
    dev: *mut device,
    sdw_bpt_stream: *mut hdac_ext_stream,
) -> c_int {
    let sdev = dev_get_drvdata(dev) as *mut snd_sof_dev;
    let mut ret: c_int;

    ret = hda_cl_trigger((*sdev).dev, sdw_bpt_stream, SNDRV_PCM_TRIGGER_START);
    if ret < 0 {
        dev_err(
            (*sdev).dev,
            c"%s: SDW BPT DMA trigger start failed\n".as_ptr(),
            c"hda_sdw_bpt_dma_enable".as_ptr(),
        );
    }

    if !(*sdev).dspless_mode_selected {
        /* the chain DMA needs to be programmed before the DMAs */
        ret = chain_dma_trigger(
            sdev,
            (*hdac_stream(sdw_bpt_stream)).stream_tag as c_uint,
            (*hdac_stream(sdw_bpt_stream)).direction,
            SOF_IPC4_PIPE_RUNNING,
        );
        if ret < 0 {
            dev_err(
                (*sdev).dev,
                c"%s: chain_dma_trigger failed: %d\n".as_ptr(),
                c"hda_sdw_bpt_dma_enable".as_ptr(),
                ret,
            );
            hda_cl_trigger((*sdev).dev, sdw_bpt_stream, SNDRV_PCM_TRIGGER_STOP);
            return ret;
        }
        snd_hdac_ext_stream_start(sdw_bpt_stream);
    }

    ret
}

unsafe fn hda_sdw_bpt_dma_disable(
    dev: *mut device,
    sdw_bpt_stream: *mut hdac_ext_stream,
) -> c_int {
    let sdev = dev_get_drvdata(dev) as *mut snd_sof_dev;
    let mut ret: c_int;

    if !(*sdev).dspless_mode_selected {
        snd_hdac_ext_stream_clear(sdw_bpt_stream);

        ret = chain_dma_trigger(
            sdev,
            (*hdac_stream(sdw_bpt_stream)).stream_tag as c_uint,
            (*hdac_stream(sdw_bpt_stream)).direction,
            SOF_IPC4_PIPE_PAUSED,
        );
        if ret < 0 {
            dev_err(
                (*sdev).dev,
                c"%s: chain_dma_trigger PIPE_PAUSED failed: %d\n".as_ptr(),
                c"hda_sdw_bpt_dma_disable".as_ptr(),
                ret,
            );
        }
    }

    ret = hda_cl_trigger((*sdev).dev, sdw_bpt_stream, SNDRV_PCM_TRIGGER_STOP);
    if ret < 0 {
        dev_err(
            (*sdev).dev,
            c"%s: SDW BPT DMA trigger stop failed\n".as_ptr(),
            c"hda_sdw_bpt_dma_disable".as_ptr(),
        );
    }

    ret
}

const FIFO_ALIGNMENT: c_uint = 64;

#[unsafe(no_mangle)]
pub unsafe extern "C" fn hda_sdw_bpt_get_buf_size_alignment(dma_bandwidth: c_uint) -> c_uint {
    let num_channels = DIV_ROUND_UP(dma_bandwidth, BPT_FREQUENCY * 32);
    let data_block = num_channels * 4;
    let alignment = lcm(data_block, FIFO_ALIGNMENT);

    alignment
}
// EXPORT_SYMBOL_NS(hda_sdw_bpt_get_buf_size_alignment, "SND_SOC_SOF_INTEL_HDA_SDW_BPT");

#[unsafe(no_mangle)]
pub unsafe extern "C" fn hda_sdw_bpt_open(
    dev: *mut device,
    link_id: c_int,
    bpt_tx_stream: *mut *mut hdac_ext_stream,
    dmab_tx_bdl: *mut snd_dma_buffer,
    bpt_tx_num_bytes: u32,
    tx_dma_bandwidth: u32,
    bpt_rx_stream: *mut *mut hdac_ext_stream,
    dmab_rx_bdl: *mut snd_dma_buffer,
    bpt_rx_num_bytes: u32,
    rx_dma_bandwidth: u32,
) -> c_int {
    let sdev = dev_get_drvdata(dev) as *mut snd_sof_dev;
    let num_channels_tx: c_uint;
    let num_channels_rx: c_uint;
    let mut ret1: c_int;
    let mut ret: c_int;

    num_channels_tx = DIV_ROUND_UP(tx_dma_bandwidth, BPT_FREQUENCY * 32);

    ret = hda_sdw_bpt_dma_prepare(
        dev,
        bpt_tx_stream,
        dmab_tx_bdl,
        bpt_tx_num_bytes,
        num_channels_tx,
        SNDRV_PCM_STREAM_PLAYBACK,
    );
    if ret < 0 {
        dev_err(
            dev,
            c"%s: hda_sdw_bpt_dma_prepare failed for TX: %d\n".as_ptr(),
            c"hda_sdw_bpt_open".as_ptr(),
            ret,
        );
        return ret;
    }

    num_channels_rx = DIV_ROUND_UP(rx_dma_bandwidth, BPT_FREQUENCY * 32);

    ret = hda_sdw_bpt_dma_prepare(
        dev,
        bpt_rx_stream,
        dmab_rx_bdl,
        bpt_rx_num_bytes,
        num_channels_rx,
        SNDRV_PCM_STREAM_CAPTURE,
    );
    if ret < 0 {
        dev_err(
            dev,
            c"%s: hda_sdw_bpt_dma_prepare failed for RX: %d\n".as_ptr(),
            c"hda_sdw_bpt_open".as_ptr(),
            ret,
        );

        ret1 = hda_sdw_bpt_dma_deprepare(dev, *bpt_tx_stream, dmab_tx_bdl);
        if ret1 < 0 {
            dev_err(
                dev,
                c"%s: hda_sdw_bpt_dma_deprepare failed for TX: %d\n".as_ptr(),
                c"hda_sdw_bpt_open".as_ptr(),
                ret1,
            );
        }
        return ret;
    }

    /* we need to map the channels in PCMSyCM registers */
    ret = hdac_bus_eml_sdw_map_stream_ch(
        sof_to_bus(sdev),
        link_id,
        0, /* cpu_dai->id -> PDI0 */
        GENMASK(num_channels_tx - 1, 0),
        (*hdac_stream(*bpt_tx_stream)).stream_tag,
        SNDRV_PCM_STREAM_PLAYBACK,
    );
    if ret < 0 {
        dev_err(
            dev,
            c"%s: hdac_bus_eml_sdw_map_stream_ch failed for TX: %d\n".as_ptr(),
            c"hda_sdw_bpt_open".as_ptr(),
            ret,
        );
        return close(dev, link_id, bpt_tx_stream, dmab_tx_bdl, bpt_rx_stream, dmab_rx_bdl, ret);
    }

    ret = hdac_bus_eml_sdw_map_stream_ch(
        sof_to_bus(sdev),
        link_id,
        1, /* cpu_dai->id -> PDI1 */
        GENMASK(num_channels_rx - 1, 0),
        (*hdac_stream(*bpt_rx_stream)).stream_tag,
        SNDRV_PCM_STREAM_CAPTURE,
    );
    if ret == 0 {
        return 0;
    }

    dev_err(
        dev,
        c"%s: hdac_bus_eml_sdw_map_stream_ch failed for RX: %d\n".as_ptr(),
        c"hda_sdw_bpt_open".as_ptr(),
        ret,
    );

    close(dev, link_id, bpt_tx_stream, dmab_tx_bdl, bpt_rx_stream, dmab_rx_bdl, ret)
}
// EXPORT_SYMBOL_NS(hda_sdw_bpt_open, "SND_SOC_SOF_INTEL_HDA_SDW_BPT");

unsafe fn close(
    dev: *mut device,
    link_id: c_int,
    bpt_tx_stream: *mut *mut hdac_ext_stream,
    dmab_tx_bdl: *mut snd_dma_buffer,
    bpt_rx_stream: *mut *mut hdac_ext_stream,
    dmab_rx_bdl: *mut snd_dma_buffer,
    ret: c_int,
) -> c_int {
    let ret1 = hda_sdw_bpt_close(
        dev,
        link_id,
        *bpt_tx_stream,
        dmab_tx_bdl,
        *bpt_rx_stream,
        dmab_rx_bdl,
    );
    if ret1 < 0 {
        dev_err(
            dev,
            c"%s: hda_sdw_bpt_close failed: %d\n".as_ptr(),
            c"hda_sdw_bpt_open".as_ptr(),
            ret1,
        );
    }

    ret
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn hda_sdw_bpt_send_async(
    dev: *mut device,
    bpt_tx_stream: *mut hdac_ext_stream,
    bpt_rx_stream: *mut hdac_ext_stream,
) -> c_int {
    let ret1: c_int;
    let mut ret: c_int;

    ret = hda_sdw_bpt_dma_enable(dev, bpt_tx_stream);
    if ret < 0 {
        dev_err(
            dev,
            c"%s: hda_sdw_bpt_dma_enable failed for TX: %d\n".as_ptr(),
            c"hda_sdw_bpt_send_async".as_ptr(),
            ret,
        );
        return ret;
    }

    ret = hda_sdw_bpt_dma_enable(dev, bpt_rx_stream);
    if ret < 0 {
        dev_err(
            dev,
            c"%s: hda_sdw_bpt_dma_enable failed for RX: %d\n".as_ptr(),
            c"hda_sdw_bpt_send_async".as_ptr(),
            ret,
        );

        ret1 = hda_sdw_bpt_dma_disable(dev, bpt_tx_stream);
        if ret1 < 0 {
            dev_err(
                dev,
                c"%s: hda_sdw_bpt_dma_disable failed for TX: %d\n".as_ptr(),
                c"hda_sdw_bpt_send_async".as_ptr(),
                ret1,
            );
        }
    }

    ret
}
// EXPORT_SYMBOL_NS(hda_sdw_bpt_send_async, "SND_SOC_SOF_INTEL_HDA_SDW_BPT");

/*
 * 3s is several orders of magnitude larger than what is needed for a
 * typical firmware download.
 */
const HDA_BPT_IOC_TIMEOUT_MS: c_uint = 3000;

#[unsafe(no_mangle)]
pub unsafe extern "C" fn hda_sdw_bpt_wait(
    dev: *mut device,
    bpt_tx_stream: *mut hdac_ext_stream,
    bpt_rx_stream: *mut hdac_ext_stream,
) -> c_int {
    let hda_tx_stream: *mut sof_intel_hda_stream;
    let hda_rx_stream: *mut sof_intel_hda_stream;
    let mut tx_position: snd_pcm_uframes_t;
    let mut rx_position: snd_pcm_uframes_t;
    let time_tx_left: c_ulong;
    let time_rx_left: c_ulong;
    let mut ret: c_int = 0;
    let mut ret1: c_int;
    let mut i: c_int;

    hda_tx_stream = bpt_tx_stream as *mut sof_intel_hda_stream;
    hda_rx_stream = bpt_rx_stream as *mut sof_intel_hda_stream;

    time_tx_left = wait_for_completion_timeout(
        &mut (*hda_tx_stream).ioc,
        msecs_to_jiffies(HDA_BPT_IOC_TIMEOUT_MS),
    );
    if time_tx_left == 0 {
        tx_position =
            hda_dsp_stream_get_position(hdac_stream(bpt_tx_stream), SNDRV_PCM_STREAM_PLAYBACK, false);
        dev_err(
            dev,
            c"%s: SDW BPT TX DMA did not complete: %ld\n".as_ptr(),
            c"hda_sdw_bpt_wait".as_ptr(),
            tx_position,
        );
        ret = -ETIMEDOUT;
        return dma_disable(dev, bpt_rx_stream, bpt_tx_stream, ret);
    }

    /* Make sure the DMA is flushed */
    i = 0;
    loop {
        tx_position =
            hda_dsp_stream_get_position(hdac_stream(bpt_tx_stream), SNDRV_PCM_STREAM_PLAYBACK, false);
        usleep_range(1000, 1010);
        i += 1;
        if !(tx_position != 0 && i < HDA_BPT_IOC_TIMEOUT_MS as c_int) {
            break;
        }
    }
    if tx_position != 0 {
        dev_err(
            dev,
            c"%s: SDW BPT TX DMA position %ld was not cleared\n".as_ptr(),
            c"hda_sdw_bpt_wait".as_ptr(),
            tx_position,
        );
        ret = -ETIMEDOUT;
        return dma_disable(dev, bpt_rx_stream, bpt_tx_stream, ret);
    }

    /* the wait should be minimal here */
    time_rx_left = wait_for_completion_timeout(
        &mut (*hda_rx_stream).ioc,
        msecs_to_jiffies(HDA_BPT_IOC_TIMEOUT_MS),
    );
    if time_rx_left == 0 {
        rx_position =
            hda_dsp_stream_get_position(hdac_stream(bpt_rx_stream), SNDRV_PCM_STREAM_CAPTURE, false);
        dev_err(
            dev,
            c"%s: SDW BPT RX DMA did not complete: %ld\n".as_ptr(),
            c"hda_sdw_bpt_wait".as_ptr(),
            rx_position,
        );
        ret = -ETIMEDOUT;
        return dma_disable(dev, bpt_rx_stream, bpt_tx_stream, ret);
    }

    /* Make sure the DMA is flushed */
    i = 0;
    loop {
        rx_position =
            hda_dsp_stream_get_position(hdac_stream(bpt_rx_stream), SNDRV_PCM_STREAM_CAPTURE, false);
        usleep_range(1000, 1010);
        i += 1;
        if !(rx_position != 0 && i < HDA_BPT_IOC_TIMEOUT_MS as c_int) {
            break;
        }
    }
    if rx_position != 0 {
        dev_err(
            dev,
            c"%s: SDW BPT RX DMA position %ld was not cleared\n".as_ptr(),
            c"hda_sdw_bpt_wait".as_ptr(),
            rx_position,
        );
        ret = -ETIMEDOUT;
        return dma_disable(dev, bpt_rx_stream, bpt_tx_stream, ret);
    }

    ret1 = hda_sdw_bpt_dma_disable(dev, bpt_rx_stream);
    if ret == 0 {
        ret = ret1;
    }

    ret1 = hda_sdw_bpt_dma_disable(dev, bpt_tx_stream);
    if ret == 0 {
        ret = ret1;
    }

    ret
}
// EXPORT_SYMBOL_NS(hda_sdw_bpt_wait, "SND_SOC_SOF_INTEL_HDA_SDW_BPT");

unsafe fn dma_disable(
    dev: *mut device,
    bpt_rx_stream: *mut hdac_ext_stream,
    bpt_tx_stream: *mut hdac_ext_stream,
    mut ret: c_int,
) -> c_int {
    let mut ret1 = hda_sdw_bpt_dma_disable(dev, bpt_rx_stream);
    if ret == 0 {
        ret = ret1;
    }

    ret1 = hda_sdw_bpt_dma_disable(dev, bpt_tx_stream);
    if ret == 0 {
        ret = ret1;
    }

    ret
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn hda_sdw_bpt_close(
    dev: *mut device,
    link_id: c_int,
    bpt_tx_stream: *mut hdac_ext_stream,
    dmab_tx_bdl: *mut snd_dma_buffer,
    bpt_rx_stream: *mut hdac_ext_stream,
    dmab_rx_bdl: *mut snd_dma_buffer,
) -> c_int {
    let sdev = dev_get_drvdata(dev) as *mut snd_sof_dev;
    let mut ret: c_int;
    let mut ret1: c_int;

    /*
     * In the case of SoundWire we need to reset the PCMSyCM registers.
     * Need to continue depreparing the DMA buffers even if this fails.
     */
    ret = hdac_bus_eml_sdw_map_stream_ch(
        sof_to_bus(sdev),
        link_id,
        0, /* PDI0 */
        0,
        0,
        SNDRV_PCM_STREAM_PLAYBACK,
    );
    if ret < 0 {
        dev_err(
            dev,
            c"%s: hdac_bus_eml_sdw_map_stream_ch failed %d for PDI0\n".as_ptr(),
            c"hda_sdw_bpt_close".as_ptr(),
            ret,
        );
    }

    ret1 = hdac_bus_eml_sdw_map_stream_ch(
        sof_to_bus(sdev),
        link_id,
        1, /* PDI1 */
        0,
        0,
        SNDRV_PCM_STREAM_CAPTURE,
    );
    if ret1 < 0 {
        dev_err(
            dev,
            c"%s: hdac_bus_eml_sdw_map_stream_ch failed %d for PDI1\n".as_ptr(),
            c"hda_sdw_bpt_close".as_ptr(),
            ret1,
        );
        if ret == 0 {
            ret = ret1;
        }
    }

    ret1 = hda_sdw_bpt_dma_deprepare(dev, bpt_rx_stream, dmab_rx_bdl);
    if ret == 0 {
        ret = ret1;
    }

    ret1 = hda_sdw_bpt_dma_deprepare(dev, bpt_tx_stream, dmab_tx_bdl);
    if ret == 0 {
        ret = ret1;
    }

    ret
}
// EXPORT_SYMBOL_NS(hda_sdw_bpt_close, "SND_SOC_SOF_INTEL_HDA_SDW_BPT");

// MODULE_LICENSE("Dual BSD/GPL");
// MODULE_DESCRIPTION("SOF helpers for HDaudio SoundWire BPT");
// MODULE_IMPORT_NS("SND_SOC_SOF_INTEL_HDA_COMMON");
// MODULE_IMPORT_NS("SND_SOC_SOF_HDA_MLINK");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
