// SPDX-License-Identifier: (GPL-2.0-only OR BSD-3-Clause)
//
// This file is provided under a dual BSD/GPLv2 license.  When using or
// redistributing this file, you may do so under either license.
//
// Copyright(c) 2018 Intel Corporation
//
// Authors: Liam Girdwood <liam.r.girdwood@linux.intel.com>
//	    Ranjani Sridharan <ranjani.sridharan@linux.intel.com>
//	    Rander Wang <rander.wang@intel.com>
//          Keyon Jie <yang.jie@linux.intel.com>
//

/*
 * Hardware interface for generic Intel audio DSP HDA IP
 *
 * C dependencies removed from executable Rust:
 * <sound/hdaudio_ext.h>, <sound/hda_register.h>, <sound/sof.h>,
 * <trace/events/sof_intel.h>, "../ops.h", "../sof-audio.h",
 * "../ipc4-priv.h", "hda.h".
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_uint, c_void};

type u32 = u32;
type u64 = u64;
type dma_addr_t = u64;
type snd_pcm_uframes_t = u64;
type irqreturn_t = c_uint;

extern "C" {
    static mut sof_hda_position_quirk: c_int;

    static SOF_HDA_POSITION_QUIRK_USE_DPIB_REGISTERS: c_int;
    static SOF_HDA_POSITION_QUIRK_USE_SKYLAKE_LEGACY: c_int;
    static SOF_HDA_POSITION_QUIRK_USE_DPIB_DDR_UPDATE: c_int;
    static SNDRV_PCM_STREAM_PLAYBACK: c_int;
    static SNDRV_PCM_STREAM_CAPTURE: c_int;
    static SNDRV_PCM_TRIGGER_PAUSE_RELEASE: c_int;
    static SNDRV_PCM_TRIGGER_START: c_int;
    static SNDRV_PCM_TRIGGER_PAUSE_PUSH: c_int;
    static SNDRV_PCM_TRIGGER_SUSPEND: c_int;
    static SNDRV_PCM_TRIGGER_STOP: c_int;
    static SOF_INTEL_ACE_1_0: c_uint;
    static SOF_HDA_STREAM_DMI_L1_COMPATIBLE: u32;
    static SOF_INTEL_PROCEN_FMT_QUIRK: u32;
    static SOF_IPC_TYPE_4: c_int;
    static HDA_DSP_MAX_BDL_ENTRIES: c_int;
    static HDA_DSP_SPIB_BAR: usize;
    static HDA_DSP_HDA_BAR: usize;
    static HDA_DSP_PP_BAR: usize;
    static SOF_HDA_ADSP_REG_CL_SPBFIFO_SPBFCCTL: u32;
    static HDA_VS_INTEL_EM2: u32;
    static HDA_VS_INTEL_EM2_L1SEN: u32;
    static HDA_DSP_STREAM_RESET_TIMEOUT: c_int;
    static SOF_STREAM_SD_OFFSET_CRST: u32;
    static SOF_HDA_SD_CTL_DMA_START: u32;
    static SOF_HDA_CL_DMA_SD_INT_MASK: u32;
    static SOF_HDA_INTCTL: u32;
    static HDA_DSP_REG_POLL_INTERVAL_US: c_uint;
    static HDA_DSP_STREAM_RUN_TIMEOUT: c_uint;
    static SOF_HDA_ADSP_REG_SD_STS: c_int;
    static SOF_HDA_ADSP_REG_SD_BDLPL: c_int;
    static SOF_HDA_ADSP_REG_SD_BDLPU: c_int;
    static SOF_HDA_ADSP_REG_SD_CBL: c_int;
    static SOF_HDA_ADSP_REG_SD_LVI: c_int;
    static SOF_HDA_REG_PP_PPCTL: u32;
    static HDA_VS_INTEL_LTRP: u32;
    static HDA_VS_INTEL_LTRP_GB_MASK: u8;
    static SOF_HDA_CL_SD_CTL_STREAM_TAG_MASK: u32;
    static SOF_HDA_CL_SD_CTL_STREAM_TAG_SHIFT: c_int;
    static SOF_HDA_ADSP_REG_SD_FORMAT: c_int;
    static SOF_HDA_ADSP_DPLBASE: u32;
    static SOF_HDA_ADSP_DPUBASE: u32;
    static SOF_HDA_ADSP_DPLBASE_ENABLE: u32;
    static SOF_HDA_ADSP_REG_SD_FIFOSIZE: c_int;
    static SOF_HDA_SD_FIFOSIZE_FIFOS_MASK: u32;
    static SOF_HDA_INTSTS: u32;
    static SOF_HDA_CL_DMA_SD_INT_COMPLETE: u32;
    static AZX_INT_CTRL_EN: u32;
    static IRQ_HANDLED: irqreturn_t;
    static SOF_HDA_GCAP: u32;
    static SOF_HDA_PLAYBACK_STREAMS: c_int;
    static SOF_HDA_CAPTURE_STREAMS: c_int;
    static SNDRV_DMA_TYPE_DEV: c_int;
    static SNDRV_DMA_TYPE_DEV_SG: c_int;
    static SOF_HDA_DPIB_ENTRY_SIZE: c_int;
    static PAGE_SIZE: usize;
    static GFP_KERNEL: c_uint;
    static HDA_DSP_BDL_SIZE: usize;
    static SOF_HDA_PPHC_BASE: usize;
    static SOF_HDA_PPHC_INTERVAL: usize;
    static SOF_HDA_PPLC_BASE: usize;
    static SOF_HDA_PPLC_MULTI: usize;
    static SOF_HDA_PPLC_INTERVAL: usize;
    static SOF_HDA_SPIB_BASE: usize;
    static SOF_HDA_SPIB_INTERVAL: usize;
    static SOF_HDA_SPIB_SPIB: usize;
    static SOF_HDA_SPIB_MAXFIFO: usize;
    static AZX_REG_VS_SDXDPIB_XBASE: u32;
    static AZX_REG_VS_SDXDPIB_XINTERVAL: u32;
    static AZX_REG_PPLCLLPL: usize;
    static AZX_REG_PPLCLLPU: usize;
    static AZX_REG_PPHCLDPL: usize;
    static AZX_REG_PPHCLDPU: usize;
    static HDA_DSP_SPIB_DISABLE: c_int;
    static HDA_DSP_SPIB_ENABLE: c_int;
    static EINVAL: c_int;
    static ENODEV: c_int;
    static ENOMEM: c_int;
    static ETIMEDOUT: c_int;
}

const HDA_LTRP_GB_VALUE_US: u8 = 95;

#[repr(C)] pub struct device { _private: [u8; 0] }
#[repr(C)] pub struct list_head { pub next: *mut list_head, pub prev: *mut list_head }
#[repr(C)] pub struct snd_dma_buffer { pub area: *mut c_void, pub addr: dma_addr_t, pub bytes: usize }
#[repr(C)] pub struct snd_pcm_hw_params { _private: [u8; 0] }
#[repr(C)] pub struct snd_soc_component { _private: [u8; 0] }
#[repr(C)] pub struct snd_soc_dai { _private: [u8; 0] }
#[repr(C)] pub struct snd_soc_dpcm { pub fe: *mut snd_soc_pcm_runtime, pub be: *mut snd_soc_pcm_runtime }
#[repr(C)] pub struct snd_soc_dai_link { pub name: *const c_char }
#[repr(C)] pub struct snd_soc_pcm_runtime { pub dai_link: *mut snd_soc_dai_link }
#[repr(C)] pub struct snd_compr_stream { pub private_data: *mut snd_soc_pcm_runtime, pub direction: c_int }
#[repr(C)] pub struct snd_pcm_runtime { pub private_data: *mut c_void }
#[repr(C)] pub struct snd_pcm_substream { pub runtime: *mut snd_pcm_runtime, pub stream: c_int }
#[repr(C)] pub struct pci_dev { pub dev: device }
#[repr(C)] pub struct sof_intel_dsp_desc { pub hw_ip_version: c_uint, pub quirks: u32 }
#[repr(C)] pub struct snd_sof_pdata { pub hw_pdata: *mut sof_intel_hda_dev, pub ipc_type: c_int }
#[repr(C)] pub struct snd_sof_dev {
    pub dev: *mut device,
    pub pdata: *mut snd_sof_pdata,
    pub private: *mut c_void,
    pub bar: [usize; 8],
    pub dspless_mode_selected: bool,
}
#[repr(C)] pub struct sof_ipc4_fw_data { pub num_playback_streams: c_int, pub num_capture_streams: c_int }
#[repr(C)] pub struct sof_intel_hda_dev { pub no_ipc_position: bool, pub l1_disabled: bool, pub stream_max: c_int }
#[repr(C)] pub struct hdac_bus {
    pub reg_lock: c_void,
    pub stream_list: list_head,
    pub align_bdle_4k: bool,
    pub use_posbuf: bool,
    pub posbuf: snd_dma_buffer,
    pub rb: snd_dma_buffer,
    pub dev: *mut device,
}
#[repr(C)] pub struct hdac_stream {
    pub list: list_head,
    pub bus: *mut hdac_bus,
    pub sd_addr: usize,
    pub spib_addr: usize,
    pub fifo_addr: usize,
    pub direction: c_int,
    pub stream_tag: c_int,
    pub index: c_int,
    pub opened: bool,
    pub running: bool,
    pub period_bytes: c_int,
    pub bufsize: u32,
    pub frags: c_int,
    pub no_period_wakeup: bool,
    pub bdl: snd_dma_buffer,
    pub posbuf: *mut u32,
    pub substream: *mut snd_pcm_substream,
    pub cstream: *mut snd_compr_stream,
    pub sd_int_sta_mask: u32,
    pub format_val: u32,
    pub fifo_size: u32,
    pub curr_pos: u64,
}
#[repr(C)] pub struct hdac_ext_stream {
    pub hstream: hdac_stream,
    pub link_locked: bool,
    pub pphc_addr: usize,
    pub pplc_addr: usize,
    pub pplcllpl: u32,
    pub pplcllpu: u32,
}
#[repr(C)] pub struct completion { _private: [u8; 0] }
#[repr(C)] pub struct sof_intel_hda_stream {
    pub hext_stream: hdac_ext_stream,
    pub sdev: *mut snd_sof_dev,
    pub ioc: completion,
    pub host_reserved: bool,
    pub flags: u32,
}
#[repr(C)] pub struct sof_intel_dsp_bdl { pub addr_l: u32, pub addr_h: u32, pub size: u32, pub ioc: u32 }

extern "C" {
    fn sof_to_bus(sdev: *mut snd_sof_dev) -> *mut hdac_bus;
    fn bus_to_sof_hda(bus: *mut hdac_bus) -> *mut sof_intel_hda_dev;
    fn get_chip_info(pdata: *mut snd_sof_pdata) -> *const sof_intel_dsp_desc;
    fn stream_to_hdac_ext_stream(s: *mut hdac_stream) -> *mut hdac_ext_stream;
    fn hstream_to_sof_hda_stream(s: *mut hdac_ext_stream) -> *mut sof_intel_hda_stream;
    fn hdac_stream(s: *mut hdac_ext_stream) -> *mut hdac_stream;
    fn snd_soc_substream_to_rtd(substream: *mut snd_pcm_substream) -> *mut snd_soc_pcm_runtime;
    fn snd_soc_rtd_to_cpu(rtd: *mut snd_soc_pcm_runtime, num: c_int) -> *mut snd_soc_dai;
    fn snd_soc_dai_get_dma_data(dai: *mut snd_soc_dai, substream: *mut snd_pcm_substream) -> *mut hdac_ext_stream;
    fn snd_sgbuf_get_addr(dmab: *mut snd_dma_buffer, offset: c_int) -> dma_addr_t;
    fn snd_sgbuf_get_chunk_size(dmab: *mut snd_dma_buffer, offset: c_int, size: c_int) -> c_int;
    fn snd_sof_dsp_update_bits(sdev: *mut snd_sof_dev, bar: usize, offset: u32, mask: u32, value: u32);
    fn snd_sof_dsp_update8(sdev: *mut snd_sof_dev, bar: usize, offset: u32, mask: u8, value: u8);
    fn snd_sof_dsp_read(sdev: *mut snd_sof_dev, bar: usize, offset: u32) -> u32;
    fn snd_sof_dsp_write(sdev: *mut snd_sof_dev, bar: usize, offset: u32, value: u32);
    fn sof_io_write(sdev: *mut snd_sof_dev, addr: usize, value: u32);
    fn snd_hdac_ext_stream_release(stream: *mut hdac_ext_stream, ty: c_int);
    fn snd_hdac_stream_get_pos_posbuf(hstream: *mut hdac_stream) -> snd_pcm_uframes_t;
    fn snd_sof_pcm_period_elapsed(substream: *mut snd_pcm_substream);
    fn snd_compr_fragment_elapsed(cstream: *mut snd_compr_stream);
    fn hda_codec_check_rirb_status(sdev: *mut snd_sof_dev) -> bool;
    fn snd_dma_alloc_pages(ty: c_int, dev: *mut device, size: usize, dmab: *mut snd_dma_buffer) -> c_int;
    fn snd_dma_free_pages(dmab: *mut snd_dma_buffer);
    fn devm_kzalloc(dev: *mut device, size: usize, flags: c_uint) -> *mut c_void;
    fn devm_kfree(dev: *mut device, p: *mut c_void);
    fn init_completion(x: *mut completion);
    fn complete(x: *mut completion);
    fn dev_get_drvdata(dev: *mut device) -> *mut snd_sof_dev;
    fn to_pci_dev(dev: *mut device) -> *mut pci_dev;
    fn snd_pcm_direction_name(direction: c_int) -> *const c_char;
    fn kasprintf(flags: c_uint, fmt: *const c_char, ...) -> *mut c_char;
    fn kfree(p: *mut c_void);
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn dev_err_once(dev: *mut device, fmt: *const c_char, ...);
    fn dev_dbg(dev: *mut device, fmt: *const c_char, ...);
    fn udelay(usecs: c_uint);
    fn usleep_range(min: c_uint, max: c_uint);
    fn readb(addr: usize) -> u8;
    fn writeb(value: u8, addr: usize);
    fn readl(addr: usize) -> u32;
    fn list_add_tail(new: *mut list_head, head: *mut list_head);
    fn list_del(entry: *mut list_head);
    fn trace_sof_intel_hda_dsp_check_stream_irq(sdev: *mut snd_sof_dev, status: u32);
    fn trace_sof_intel_hda_dsp_stream_status(dev: *mut device, s: *mut hdac_stream, status: u32);
}

#[inline] unsafe fn BIT(n: c_int) -> u32 { 1u32 << n }
#[inline] unsafe fn lower_32_bits(x: u64) -> u32 { x as u32 }
#[inline] unsafe fn upper_32_bits(x: u64) -> u32 { (x >> 32) as u32 }
#[inline] unsafe fn cpu_to_le32(x: u32) -> u32 { x.to_le() }
#[inline] unsafe fn SOF_STREAM_SD_OFFSET(hstream: *mut hdac_stream) -> c_int { (*hstream).index * 0x20 + 0x80 }
#[inline] unsafe fn ERR_PTR<T>(err: c_int) -> *mut T { err as isize as *mut T }
#[inline] unsafe fn merge_u64(u32_u: u32, u32_l: u32) -> u64 { ((u32_u as u64) << 32) | (u32_l as u64) }

unsafe fn for_each_stream<F: FnMut(*mut hdac_stream)>(bus: *mut hdac_bus, mut f: F) {
    let head = &mut (*bus).stream_list as *mut list_head;
    let mut pos = (*head).next;
    while !pos.is_null() && pos != head {
        f(pos as *mut hdac_stream);
        pos = (*pos).next;
    }
}

unsafe fn for_each_stream_safe<F: FnMut(*mut hdac_stream)>(bus: *mut hdac_bus, mut f: F) {
    let head = &mut (*bus).stream_list as *mut list_head;
    let mut pos = (*head).next;
    while !pos.is_null() && pos != head {
        let next = (*pos).next;
        f(pos as *mut hdac_stream);
        pos = next;
    }
}

static Playback: &[u8] = b"Playback\0";
static Capture: &[u8] = b"Capture\0";

unsafe fn hda_hstream_direction_str(hstream: *mut hdac_stream) -> *const c_char {
    if (*hstream).direction == SNDRV_PCM_STREAM_PLAYBACK {
        Playback.as_ptr() as *const c_char
    } else {
        Capture.as_ptr() as *const c_char
    }
}

unsafe fn hda_hstream_dbg_get_stream_info_str(hstream: *mut hdac_stream) -> *mut c_char {
    let rtd: *mut snd_soc_pcm_runtime;

    if !(*hstream).substream.is_null() {
        rtd = snd_soc_substream_to_rtd((*hstream).substream);
    } else if !(*hstream).cstream.is_null() {
        rtd = (*(*hstream).cstream).private_data;
    } else {
        return kasprintf(
            GFP_KERNEL,
            b"-- (%s, stream_tag: %u)\0".as_ptr() as *const c_char,
            hda_hstream_direction_str(hstream),
            (*hstream).stream_tag as c_uint,
        );
    }

    kasprintf(
        GFP_KERNEL,
        b"dai_link \"%s\" (%s, stream_tag: %u)\0".as_ptr() as *const c_char,
        (*(*rtd).dai_link).name,
        hda_hstream_direction_str(hstream),
        (*hstream).stream_tag as c_uint,
    )
}

unsafe fn hda_setup_bdle(
    sdev: *mut snd_sof_dev,
    dmab: *mut snd_dma_buffer,
    hstream: *mut hdac_stream,
    bdlp: *mut *mut sof_intel_dsp_bdl,
    mut offset: c_int,
    mut size: c_int,
    ioc: c_int,
) -> c_int {
    let bus = sof_to_bus(sdev);
    let mut bdl = *bdlp;

    while size > 0 {
        if (*hstream).frags >= HDA_DSP_MAX_BDL_ENTRIES {
            dev_err((*sdev).dev, b"error: stream frags exceeded\n\0".as_ptr() as *const c_char);
            return -EINVAL;
        }

        let addr = snd_sgbuf_get_addr(dmab, offset);
        (*bdl).addr_l = cpu_to_le32(lower_32_bits(addr));
        (*bdl).addr_h = cpu_to_le32(upper_32_bits(addr));
        let mut chunk = snd_sgbuf_get_chunk_size(dmab, offset, size);
        if (*bus).align_bdle_4k {
            let remain = 0x1000 - (offset & 0xfff);
            if chunk > remain {
                chunk = remain;
            }
        }
        (*bdl).size = cpu_to_le32(chunk as u32);
        size -= chunk;
        (*bdl).ioc = if size != 0 || ioc == 0 { 0 } else { cpu_to_le32(0x01) };
        bdl = bdl.add(1);
        (*hstream).frags += 1;
        offset += chunk;
    }

    *bdlp = bdl;
    offset
}

pub unsafe extern "C" fn hda_dsp_stream_setup_bdl(
    sdev: *mut snd_sof_dev,
    dmab: *mut snd_dma_buffer,
    hstream: *mut hdac_stream,
) -> c_int {
    let hda = (*(*sdev).pdata).hw_pdata;
    let mut period_bytes = (*hstream).period_bytes;
    dev_dbg((*sdev).dev, b"period_bytes: %#x, bufsize: %#x\n\0".as_ptr() as *const c_char, period_bytes, (*hstream).bufsize);

    if period_bytes == 0 {
        let chunk_size = snd_sgbuf_get_chunk_size(dmab, 0, (*hstream).bufsize as c_int);
        period_bytes = (*hstream).bufsize as c_int;
        if chunk_size == (*hstream).bufsize as c_int {
            period_bytes /= 2;
        }
    }

    let mut periods = (*hstream).bufsize as c_int / period_bytes;
    dev_dbg((*sdev).dev, b"periods: %d\n\0".as_ptr() as *const c_char, periods);

    let remain = (*hstream).bufsize as c_int % period_bytes;
    if remain != 0 {
        periods += 1;
    }

    let mut bdl = (*hstream).bdl.area as *mut sof_intel_dsp_bdl;
    let mut offset = 0;
    (*hstream).frags = 0;

    let ioc = if (*hda).no_ipc_position {
        (!(*hstream).no_period_wakeup) as c_int
    } else {
        0
    };

    for i in 0..periods {
        if i == periods - 1 && remain != 0 {
            offset = hda_setup_bdle(sdev, dmab, hstream, &mut bdl, offset, remain, 0);
        } else {
            offset = hda_setup_bdle(sdev, dmab, hstream, &mut bdl, offset, period_bytes, ioc);
        }
    }

    offset
}

pub unsafe extern "C" fn hda_dsp_stream_spib_config(
    sdev: *mut snd_sof_dev,
    hext_stream: *mut hdac_ext_stream,
    enable: c_int,
    size: u32,
) -> c_int {
    let hstream = &mut (*hext_stream).hstream as *mut hdac_stream;
    if (*sdev).bar[HDA_DSP_SPIB_BAR] == 0 {
        dev_err((*sdev).dev, b"error: address of spib capability is NULL\n\0".as_ptr() as *const c_char);
        return -EINVAL;
    }

    let mask = 1u32 << (*hstream).index;
    if enable == 0 {
        sof_io_write(sdev, (*hstream).spib_addr, 0);
    }
    snd_sof_dsp_update_bits(
        sdev,
        HDA_DSP_SPIB_BAR,
        SOF_HDA_ADSP_REG_CL_SPBFIFO_SPBFCCTL,
        mask,
        (enable as u32) << (*hstream).index,
    );
    if enable != 0 {
        sof_io_write(sdev, (*hstream).spib_addr, size);
    }
    0
}

unsafe fn _hda_dsp_stream_get(
    sdev: *mut snd_sof_dev,
    direction: c_int,
    flags: u32,
    pair: bool,
) -> *mut hdac_ext_stream {
    let chip_info = get_chip_info((*sdev).pdata);
    let hda = (*(*sdev).pdata).hw_pdata;
    let bus = sof_to_bus(sdev);
    let mut hext_stream: *mut hdac_ext_stream = core::ptr::null_mut();

    for_each_stream(bus, |s| {
        if hext_stream.is_null() && (*s).direction == direction && !(*s).opened {
            let hs = stream_to_hdac_ext_stream(s);
            let hda_stream = hstream_to_sof_hda_stream(hs);
            if (*hda_stream).host_reserved {
                return;
            }
            if pair && (*hs).link_locked {
                return;
            }
            (*s).opened = true;
            if pair {
                (*hs).link_locked = true;
            }
            hext_stream = hs;
        }
    });

    if hext_stream.is_null() {
        dev_err((*sdev).dev, b"error: no free %s streams\n\0".as_ptr() as *const c_char, snd_pcm_direction_name(direction));
        return hext_stream;
    }

    let hda_stream = hstream_to_sof_hda_stream(hext_stream);
    (*hda_stream).flags = flags;

    if (*chip_info).hw_ip_version < SOF_INTEL_ACE_1_0
        && (flags & SOF_HDA_STREAM_DMI_L1_COMPATIBLE) == 0
    {
        snd_sof_dsp_update_bits(sdev, HDA_DSP_HDA_BAR, HDA_VS_INTEL_EM2, HDA_VS_INTEL_EM2_L1SEN, 0);
        (*hda).l1_disabled = true;
    }

    hext_stream
}

pub unsafe extern "C" fn hda_dsp_stream_get(sdev: *mut snd_sof_dev, direction: c_int, flags: u32) -> *mut hdac_ext_stream {
    _hda_dsp_stream_get(sdev, direction, flags, false)
}

pub unsafe extern "C" fn hda_dsp_stream_pair_get(sdev: *mut snd_sof_dev, direction: c_int, flags: u32) -> *mut hdac_ext_stream {
    _hda_dsp_stream_get(sdev, direction, flags, true)
}

unsafe fn _hda_dsp_stream_put(sdev: *mut snd_sof_dev, direction: c_int, stream_tag: c_int, pair: bool) -> c_int {
    let chip_info = get_chip_info((*sdev).pdata);
    let hda = (*(*sdev).pdata).hw_pdata;
    let bus = sof_to_bus(sdev);
    let mut link_stream: *mut hdac_ext_stream = core::ptr::null_mut();
    let mut dmi_l1_enable = true;
    let mut found = false;

    for_each_stream(bus, |s| {
        let hext_stream = stream_to_hdac_ext_stream(s);
        let hda_stream = hstream_to_sof_hda_stream(hext_stream);
        if !(*s).opened {
            return;
        }
        if (*s).direction == direction && (*s).stream_tag == stream_tag {
            (*s).opened = false;
            found = true;
            if pair {
                link_stream = hext_stream;
            }
        } else if ((*hda_stream).flags & SOF_HDA_STREAM_DMI_L1_COMPATIBLE) == 0 {
            dmi_l1_enable = false;
        }
    });

    if (*chip_info).hw_ip_version < SOF_INTEL_ACE_1_0 && dmi_l1_enable {
        snd_sof_dsp_update_bits(sdev, HDA_DSP_HDA_BAR, HDA_VS_INTEL_EM2, HDA_VS_INTEL_EM2_L1SEN, HDA_VS_INTEL_EM2_L1SEN);
        (*hda).l1_disabled = false;
    }

    if !found {
        dev_err((*sdev).dev, b"%s: stream_tag %d not opened!\n\0".as_ptr() as *const c_char, b"_hda_dsp_stream_put\0".as_ptr(), stream_tag);
        return -ENODEV;
    }

    if pair {
        snd_hdac_ext_stream_release(link_stream, 0);
    }

    0
}

pub unsafe extern "C" fn hda_dsp_stream_put(sdev: *mut snd_sof_dev, direction: c_int, stream_tag: c_int) -> c_int {
    _hda_dsp_stream_put(sdev, direction, stream_tag, false)
}

pub unsafe extern "C" fn hda_dsp_stream_pair_put(sdev: *mut snd_sof_dev, direction: c_int, stream_tag: c_int) -> c_int {
    _hda_dsp_stream_put(sdev, direction, stream_tag, true)
}

unsafe fn hda_dsp_stream_reset(sdev: *mut snd_sof_dev, hstream: *mut hdac_stream) -> c_int {
    let sd_offset = SOF_STREAM_SD_OFFSET(hstream) as u32;
    let mut timeout = HDA_DSP_STREAM_RESET_TIMEOUT;

    snd_sof_dsp_update_bits(sdev, HDA_DSP_HDA_BAR, sd_offset, SOF_STREAM_SD_OFFSET_CRST, SOF_STREAM_SD_OFFSET_CRST);
    loop {
        let val = snd_sof_dsp_read(sdev, HDA_DSP_HDA_BAR, sd_offset);
        if (val & SOF_STREAM_SD_OFFSET_CRST) != 0 {
            break;
        }
        timeout -= 1;
        if timeout == 0 {
            break;
        }
    }
    if timeout == 0 {
        dev_err((*sdev).dev, b"timeout waiting for stream reset\n\0".as_ptr() as *const c_char);
        return -ETIMEDOUT;
    }

    timeout = HDA_DSP_STREAM_RESET_TIMEOUT;
    snd_sof_dsp_update_bits(sdev, HDA_DSP_HDA_BAR, sd_offset, SOF_STREAM_SD_OFFSET_CRST, 0);
    udelay(3);
    loop {
        let val = snd_sof_dsp_read(sdev, HDA_DSP_HDA_BAR, sd_offset);
        if (val & SOF_STREAM_SD_OFFSET_CRST) == 0 {
            break;
        }
        timeout -= 1;
        if timeout == 0 {
            break;
        }
    }
    if timeout == 0 {
        dev_err((*sdev).dev, b"timeout waiting for stream to exit reset\n\0".as_ptr() as *const c_char);
        return -ETIMEDOUT;
    }

    0
}

pub unsafe extern "C" fn hda_dsp_stream_trigger(sdev: *mut snd_sof_dev, hext_stream: *mut hdac_ext_stream, cmd: c_int) -> c_int {
    let hstream = &mut (*hext_stream).hstream as *mut hdac_stream;
    let sd_offset = SOF_STREAM_SD_OFFSET(hstream) as u32;
    let dma_start = SOF_HDA_SD_CTL_DMA_START;
    let mut ret = 0;

    if cmd == SNDRV_PCM_TRIGGER_PAUSE_RELEASE || cmd == SNDRV_PCM_TRIGGER_START {
        if cmd == SNDRV_PCM_TRIGGER_PAUSE_RELEASE && !(*sdev).dspless_mode_selected {
            return ret;
        }
        if !(*hstream).running {
            snd_sof_dsp_update_bits(sdev, HDA_DSP_HDA_BAR, SOF_HDA_INTCTL, 1u32 << (*hstream).index, 1u32 << (*hstream).index);
            snd_sof_dsp_update_bits(sdev, HDA_DSP_HDA_BAR, sd_offset, SOF_HDA_SD_CTL_DMA_START | SOF_HDA_CL_DMA_SD_INT_MASK, SOF_HDA_SD_CTL_DMA_START | SOF_HDA_CL_DMA_SD_INT_MASK);
            let mut timeout = HDA_DSP_STREAM_RUN_TIMEOUT;
            while timeout != 0 {
                let run = snd_sof_dsp_read(sdev, HDA_DSP_HDA_BAR, sd_offset);
                if (run & dma_start) == dma_start {
                    break;
                }
                timeout -= 1;
            }
            if timeout == 0 { ret = -ETIMEDOUT; } else { (*hstream).running = true; }
        }
    } else if cmd == SNDRV_PCM_TRIGGER_PAUSE_PUSH || cmd == SNDRV_PCM_TRIGGER_SUSPEND || cmd == SNDRV_PCM_TRIGGER_STOP {
        if cmd == SNDRV_PCM_TRIGGER_PAUSE_PUSH && !(*sdev).dspless_mode_selected {
            return ret;
        }
        snd_sof_dsp_update_bits(sdev, HDA_DSP_HDA_BAR, sd_offset, SOF_HDA_SD_CTL_DMA_START | SOF_HDA_CL_DMA_SD_INT_MASK, 0);
        let mut timeout = HDA_DSP_STREAM_RUN_TIMEOUT;
        while timeout != 0 {
            let run = snd_sof_dsp_read(sdev, HDA_DSP_HDA_BAR, sd_offset);
            if (run & dma_start) == 0 {
                break;
            }
            timeout -= 1;
        }
        if timeout == 0 {
            ret = -ETIMEDOUT;
        } else {
            snd_sof_dsp_write(sdev, HDA_DSP_HDA_BAR, sd_offset + SOF_HDA_ADSP_REG_SD_STS as u32, SOF_HDA_CL_DMA_SD_INT_MASK);
            (*hstream).running = false;
            snd_sof_dsp_update_bits(sdev, HDA_DSP_HDA_BAR, SOF_HDA_INTCTL, 1u32 << (*hstream).index, 0);
        }
    } else {
        dev_err((*sdev).dev, b"error: unknown command: %d\n\0".as_ptr() as *const c_char, cmd);
        return -EINVAL;
    }

    if ret < 0 {
        let stream_name = hda_hstream_dbg_get_stream_info_str(hstream);
        dev_err((*sdev).dev, b"%s: cmd %d on %s: timeout on STREAM_SD_OFFSET read\n\0".as_ptr() as *const c_char, b"hda_dsp_stream_trigger\0".as_ptr(), cmd, if !stream_name.is_null() { stream_name } else { b"unknown stream\0".as_ptr() as *mut c_char });
        kfree(stream_name as *mut c_void);
    }
    ret
}

pub unsafe extern "C" fn hda_dsp_iccmax_stream_hw_params(sdev: *mut snd_sof_dev, hext_stream: *mut hdac_ext_stream, dmab: *mut snd_dma_buffer, _params: *mut snd_pcm_hw_params) -> c_int {
    if hext_stream.is_null() {
        dev_err((*sdev).dev, b"error: no stream available\n\0".as_ptr() as *const c_char);
        return -ENODEV;
    }
    let hstream = &mut (*hext_stream).hstream as *mut hdac_stream;
    let sd_offset = SOF_STREAM_SD_OFFSET(hstream) as u32;
    let mask = 1u32 << (*hstream).index;

    if dmab.is_null() {
        dev_err((*sdev).dev, b"error: no dma buffer allocated!\n\0".as_ptr() as *const c_char);
        return -ENODEV;
    }
    if !(*hstream).posbuf.is_null() {
        *(*hstream).posbuf = 0;
    }
    snd_sof_dsp_write(sdev, HDA_DSP_HDA_BAR, sd_offset + SOF_HDA_ADSP_REG_SD_BDLPL as u32, 0);
    snd_sof_dsp_write(sdev, HDA_DSP_HDA_BAR, sd_offset + SOF_HDA_ADSP_REG_SD_BDLPU as u32, 0);
    (*hstream).frags = 0;
    let ret = hda_dsp_stream_setup_bdl(sdev, dmab, hstream);
    if ret < 0 {
        dev_err((*sdev).dev, b"error: set up of BDL failed\n\0".as_ptr() as *const c_char);
        return ret;
    }
    snd_sof_dsp_write(sdev, HDA_DSP_HDA_BAR, sd_offset + SOF_HDA_ADSP_REG_SD_BDLPL as u32, (*hstream).bdl.addr as u32);
    snd_sof_dsp_write(sdev, HDA_DSP_HDA_BAR, sd_offset + SOF_HDA_ADSP_REG_SD_BDLPU as u32, upper_32_bits((*hstream).bdl.addr));
    snd_sof_dsp_write(sdev, HDA_DSP_HDA_BAR, sd_offset + SOF_HDA_ADSP_REG_SD_CBL as u32, (*hstream).bufsize);
    snd_sof_dsp_update_bits(sdev, HDA_DSP_HDA_BAR, sd_offset + SOF_HDA_ADSP_REG_SD_LVI as u32, 0xffff, ((*hstream).frags - 1) as u32);
    snd_sof_dsp_update_bits(sdev, HDA_DSP_PP_BAR, SOF_HDA_REG_PP_PPCTL, mask, mask);
    snd_sof_dsp_update8(sdev, HDA_DSP_HDA_BAR, HDA_VS_INTEL_LTRP, HDA_VS_INTEL_LTRP_GB_MASK, HDA_LTRP_GB_VALUE_US);
    snd_sof_dsp_update_bits(sdev, HDA_DSP_HDA_BAR, sd_offset, SOF_HDA_SD_CTL_DMA_START, SOF_HDA_SD_CTL_DMA_START);
    0
}

pub unsafe extern "C" fn hda_dsp_stream_hw_params(sdev: *mut snd_sof_dev, hext_stream: *mut hdac_ext_stream, dmab: *mut snd_dma_buffer, _params: *mut snd_pcm_hw_params) -> c_int {
    let chip = get_chip_info((*sdev).pdata);
    let bus = sof_to_bus(sdev);
    if hext_stream.is_null() {
        dev_err((*sdev).dev, b"error: no stream available\n\0".as_ptr() as *const c_char);
        return -ENODEV;
    }
    if dmab.is_null() {
        dev_err((*sdev).dev, b"error: no dma buffer allocated!\n\0".as_ptr() as *const c_char);
        return -ENODEV;
    }
    let hstream = &mut (*hext_stream).hstream as *mut hdac_stream;
    let sd_offset = SOF_STREAM_SD_OFFSET(hstream) as u32;
    let mask = BIT((*hstream).index);
    if !(*sdev).dspless_mode_selected {
        snd_sof_dsp_update_bits(sdev, HDA_DSP_PP_BAR, SOF_HDA_REG_PP_PPCTL, mask, mask);
    }
    snd_sof_dsp_update_bits(sdev, HDA_DSP_HDA_BAR, sd_offset, SOF_HDA_CL_DMA_SD_INT_MASK | SOF_HDA_SD_CTL_DMA_START, 0);
    (*hstream).running = false;
    snd_sof_dsp_update_bits(sdev, HDA_DSP_HDA_BAR, sd_offset + SOF_HDA_ADSP_REG_SD_STS as u32, SOF_HDA_CL_DMA_SD_INT_MASK, SOF_HDA_CL_DMA_SD_INT_MASK);
    let ret_reset = hda_dsp_stream_reset(sdev, hstream);
    if ret_reset < 0 { return ret_reset; }
    if !(*hstream).posbuf.is_null() { *(*hstream).posbuf = 0; }
    snd_sof_dsp_write(sdev, HDA_DSP_HDA_BAR, sd_offset + SOF_HDA_ADSP_REG_SD_BDLPL as u32, 0);
    snd_sof_dsp_write(sdev, HDA_DSP_HDA_BAR, sd_offset + SOF_HDA_ADSP_REG_SD_BDLPU as u32, 0);
    snd_sof_dsp_update_bits(sdev, HDA_DSP_HDA_BAR, sd_offset, SOF_HDA_CL_DMA_SD_INT_MASK | SOF_HDA_SD_CTL_DMA_START, 0);
    snd_sof_dsp_update_bits(sdev, HDA_DSP_HDA_BAR, sd_offset + SOF_HDA_ADSP_REG_SD_STS as u32, SOF_HDA_CL_DMA_SD_INT_MASK, SOF_HDA_CL_DMA_SD_INT_MASK);
    (*hstream).frags = 0;
    let ret = hda_dsp_stream_setup_bdl(sdev, dmab, hstream);
    if ret < 0 {
        dev_err((*sdev).dev, b"error: set up of BDL failed\n\0".as_ptr() as *const c_char);
        return ret;
    }
    snd_sof_dsp_update_bits(sdev, HDA_DSP_HDA_BAR, sd_offset, SOF_HDA_CL_SD_CTL_STREAM_TAG_MASK, ((*hstream).stream_tag as u32) << SOF_HDA_CL_SD_CTL_STREAM_TAG_SHIFT);
    snd_sof_dsp_write(sdev, HDA_DSP_HDA_BAR, sd_offset + SOF_HDA_ADSP_REG_SD_CBL as u32, (*hstream).bufsize);
    if !(*sdev).dspless_mode_selected && ((*chip).quirks & SOF_INTEL_PROCEN_FMT_QUIRK) != 0 {
        snd_sof_dsp_update_bits(sdev, HDA_DSP_PP_BAR, SOF_HDA_REG_PP_PPCTL, mask, 0);
    }
    snd_sof_dsp_update_bits(sdev, HDA_DSP_HDA_BAR, sd_offset + SOF_HDA_ADSP_REG_SD_FORMAT as u32, 0xffff, (*hstream).format_val);
    if !(*sdev).dspless_mode_selected && ((*chip).quirks & SOF_INTEL_PROCEN_FMT_QUIRK) != 0 {
        snd_sof_dsp_update_bits(sdev, HDA_DSP_PP_BAR, SOF_HDA_REG_PP_PPCTL, mask, mask);
    }
    snd_sof_dsp_update_bits(sdev, HDA_DSP_HDA_BAR, sd_offset + SOF_HDA_ADSP_REG_SD_LVI as u32, 0xffff, ((*hstream).frags - 1) as u32);
    snd_sof_dsp_write(sdev, HDA_DSP_HDA_BAR, sd_offset + SOF_HDA_ADSP_REG_SD_BDLPL as u32, (*hstream).bdl.addr as u32);
    snd_sof_dsp_write(sdev, HDA_DSP_HDA_BAR, sd_offset + SOF_HDA_ADSP_REG_SD_BDLPU as u32, upper_32_bits((*hstream).bdl.addr));
    if (*bus).use_posbuf && (*bus).posbuf.addr != 0 && (snd_sof_dsp_read(sdev, HDA_DSP_HDA_BAR, SOF_HDA_ADSP_DPLBASE) & SOF_HDA_ADSP_DPLBASE_ENABLE) == 0 {
        snd_sof_dsp_write(sdev, HDA_DSP_HDA_BAR, SOF_HDA_ADSP_DPUBASE, upper_32_bits((*bus).posbuf.addr));
        snd_sof_dsp_write(sdev, HDA_DSP_HDA_BAR, SOF_HDA_ADSP_DPLBASE, ((*bus).posbuf.addr as u32) | SOF_HDA_ADSP_DPLBASE_ENABLE);
    }
    snd_sof_dsp_update_bits(sdev, HDA_DSP_HDA_BAR, sd_offset, SOF_HDA_CL_DMA_SD_INT_MASK, SOF_HDA_CL_DMA_SD_INT_MASK);
    if (*hstream).direction == SNDRV_PCM_STREAM_PLAYBACK {
        (*hstream).fifo_size = snd_sof_dsp_read(sdev, HDA_DSP_HDA_BAR, sd_offset + SOF_HDA_ADSP_REG_SD_FIFOSIZE as u32);
        (*hstream).fifo_size &= SOF_HDA_SD_FIFOSIZE_FIFOS_MASK;
        (*hstream).fifo_size += 1;
    } else {
        (*hstream).fifo_size = 0;
    }
    ret
}

pub unsafe extern "C" fn hda_dsp_stream_hw_free(sdev: *mut snd_sof_dev, substream: *mut snd_pcm_substream) -> c_int {
    let hstream = (*(*substream).runtime).private_data as *mut hdac_stream;
    let hext_stream = stream_to_hdac_ext_stream(hstream);
    let ret = hda_dsp_stream_reset(sdev, hstream);
    if ret < 0 { return ret; }
    if !(*sdev).dspless_mode_selected {
        let mask = BIT((*hstream).index);
        if !(*hext_stream).link_locked {
            snd_sof_dsp_update_bits(sdev, HDA_DSP_PP_BAR, SOF_HDA_REG_PP_PPCTL, mask, 0);
        }
    }
    hda_dsp_stream_spib_config(sdev, hext_stream, HDA_DSP_SPIB_DISABLE, 0);
    (*hstream).substream = core::ptr::null_mut();
    0
}

pub unsafe extern "C" fn hda_dsp_check_stream_irq(sdev: *mut snd_sof_dev) -> bool {
    let status = snd_sof_dsp_read(sdev, HDA_DSP_HDA_BAR, SOF_HDA_INTSTS);
    trace_sof_intel_hda_dsp_check_stream_irq(sdev, status);
    status != 0xffffffff
}

unsafe fn hda_dsp_compr_bytes_transferred(hstream: *mut hdac_stream, direction: c_int) {
    let buffer_size = (*hstream).bufsize as u64;
    let prev_pos = (*hstream).curr_pos % buffer_size;
    let pos = hda_dsp_stream_get_position(hstream, direction, false);
    let num_bytes = if pos < prev_pos { (buffer_size - prev_pos) + pos } else { pos - prev_pos };
    (*hstream).curr_pos = (*hstream).curr_pos.wrapping_add(num_bytes);
}

unsafe fn hda_dsp_stream_check(bus: *mut hdac_bus, status: u32) -> bool {
    let sof_hda = bus_to_sof_hda(bus);
    let mut active = false;
    for_each_stream(bus, |s| {
        if (status & BIT((*s).index)) != 0 && (*s).opened {
            let sd_status = readb((*s).sd_addr + SOF_HDA_ADSP_REG_SD_STS as usize) as u32;
            trace_sof_intel_hda_dsp_stream_status((*bus).dev, s, sd_status);
            writeb(sd_status as u8, (*s).sd_addr + SOF_HDA_ADSP_REG_SD_STS as usize);
            active = true;
            if !(*s).running { return; }
            if (sd_status & SOF_HDA_CL_DMA_SD_INT_COMPLETE) == 0 { return; }
            if (*s).substream.is_null() && (*s).cstream.is_null() {
                let hext_stream = stream_to_hdac_ext_stream(s);
                let hda_stream = hstream_to_sof_hda_stream(hext_stream);
                complete(&mut (*hda_stream).ioc);
                return;
            }
            if !(*s).substream.is_null() && (*sof_hda).no_ipc_position {
                snd_sof_pcm_period_elapsed((*s).substream);
            } else if !(*s).cstream.is_null() {
                hda_dsp_compr_bytes_transferred(s, (*(*s).cstream).direction);
                snd_compr_fragment_elapsed((*s).cstream);
            }
        }
    });
    active
}

pub unsafe extern "C" fn hda_dsp_stream_threaded_handler(_irq: c_int, context: *mut c_void) -> irqreturn_t {
    let sdev = context as *mut snd_sof_dev;
    let bus = sof_to_bus(sdev);
    let mut active = true;
    let mut i = 0;
    while i < 10 && active {
        let status = snd_sof_dsp_read(sdev, HDA_DSP_HDA_BAR, SOF_HDA_INTSTS);
        active = hda_dsp_stream_check(bus, status);
        if (status & AZX_INT_CTRL_EN) != 0 {
            active |= hda_codec_check_rirb_status(sdev);
        }
        i += 1;
    }
    IRQ_HANDLED
}

pub unsafe extern "C" fn hda_dsp_stream_init(sdev: *mut snd_sof_dev) -> c_int {
    let bus = sof_to_bus(sdev);
    let pci = to_pci_dev((*sdev).dev);
    let sof_hda = bus_to_sof_hda(bus);
    let gcap = snd_sof_dsp_read(sdev, HDA_DSP_HDA_BAR, SOF_HDA_GCAP);
    dev_dbg((*sdev).dev, b"hda global caps = 0x%x\n\0".as_ptr() as *const c_char, gcap);
    let num_capture = ((gcap >> 8) & 0x0f) as c_int;
    let num_playback = ((gcap >> 12) & 0x0f) as c_int;
    let num_total = num_playback + num_capture;
    dev_dbg((*sdev).dev, b"detected %d playback and %d capture streams\n\0".as_ptr() as *const c_char, num_playback, num_capture);
    if num_playback >= SOF_HDA_PLAYBACK_STREAMS {
        dev_err((*sdev).dev, b"error: too many playback streams %d\n\0".as_ptr() as *const c_char, num_playback);
        return -EINVAL;
    }
    if num_capture >= SOF_HDA_CAPTURE_STREAMS {
        dev_err((*sdev).dev, b"error: too many capture streams %d\n\0".as_ptr() as *const c_char, num_capture);
        return -EINVAL;
    }
    let mut ret = snd_dma_alloc_pages(SNDRV_DMA_TYPE_DEV, &mut (*pci).dev, (SOF_HDA_DPIB_ENTRY_SIZE * num_total) as usize, &mut (*bus).posbuf);
    if ret < 0 {
        dev_err((*sdev).dev, b"error: posbuffer dma alloc failed\n\0".as_ptr() as *const c_char);
        return -ENOMEM;
    }
    ret = snd_dma_alloc_pages(SNDRV_DMA_TYPE_DEV, &mut (*pci).dev, PAGE_SIZE, &mut (*bus).rb);
    if ret < 0 {
        dev_err((*sdev).dev, b"error: RB alloc failed\n\0".as_ptr() as *const c_char);
        return -ENOMEM;
    }
    for i in 0..num_total {
        let hda_stream = devm_kzalloc((*sdev).dev, core::mem::size_of::<sof_intel_hda_stream>(), GFP_KERNEL) as *mut sof_intel_hda_stream;
        if hda_stream.is_null() { return -ENOMEM; }
        (*hda_stream).sdev = sdev;
        init_completion(&mut (*hda_stream).ioc);
        let hext_stream = &mut (*hda_stream).hext_stream as *mut hdac_ext_stream;
        if (*sdev).bar[HDA_DSP_PP_BAR] != 0 {
            (*hext_stream).pphc_addr = (*sdev).bar[HDA_DSP_PP_BAR] + SOF_HDA_PPHC_BASE + SOF_HDA_PPHC_INTERVAL * i as usize;
            (*hext_stream).pplc_addr = (*sdev).bar[HDA_DSP_PP_BAR] + SOF_HDA_PPLC_BASE + SOF_HDA_PPLC_MULTI * num_total as usize + SOF_HDA_PPLC_INTERVAL * i as usize;
        }
        let hstream = &mut (*hext_stream).hstream as *mut hdac_stream;
        if (*sdev).bar[HDA_DSP_SPIB_BAR] != 0 {
            (*hstream).spib_addr = (*sdev).bar[HDA_DSP_SPIB_BAR] + SOF_HDA_SPIB_BASE + SOF_HDA_SPIB_INTERVAL * i as usize + SOF_HDA_SPIB_SPIB;
            (*hstream).fifo_addr = (*sdev).bar[HDA_DSP_SPIB_BAR] + SOF_HDA_SPIB_BASE + SOF_HDA_SPIB_INTERVAL * i as usize + SOF_HDA_SPIB_MAXFIFO;
        }
        (*hstream).bus = bus;
        (*hstream).sd_int_sta_mask = 1u32 << i;
        (*hstream).index = i;
        let sd_offset = SOF_STREAM_SD_OFFSET(hstream);
        (*hstream).sd_addr = (*sdev).bar[HDA_DSP_HDA_BAR] + sd_offset as usize;
        (*hstream).opened = false;
        (*hstream).running = false;
        if i < num_capture {
            (*hstream).stream_tag = i + 1;
            (*hstream).direction = SNDRV_PCM_STREAM_CAPTURE;
        } else {
            (*hstream).stream_tag = i - num_capture + 1;
            (*hstream).direction = SNDRV_PCM_STREAM_PLAYBACK;
        }
        ret = snd_dma_alloc_pages(SNDRV_DMA_TYPE_DEV, &mut (*pci).dev, HDA_DSP_BDL_SIZE, &mut (*hstream).bdl);
        if ret < 0 {
            dev_err((*sdev).dev, b"error: stream bdl dma alloc failed\n\0".as_ptr() as *const c_char);
            return -ENOMEM;
        }
        (*hstream).posbuf = ((*bus).posbuf.area as usize + ((*hstream).index as usize) * 8) as *mut u32;
        list_add_tail(&mut (*hstream).list, &mut (*bus).stream_list);
    }
    (*sof_hda).stream_max = num_total;
    if (*(*sdev).pdata).ipc_type == SOF_IPC_TYPE_4 {
        let ipc4_data = (*sdev).private as *mut sof_ipc4_fw_data;
        (*ipc4_data).num_playback_streams = num_playback;
        (*ipc4_data).num_capture_streams = num_capture;
    }
    0
}

pub unsafe extern "C" fn hda_dsp_stream_free(sdev: *mut snd_sof_dev) {
    let bus = sof_to_bus(sdev);
    if !(*bus).posbuf.area.is_null() {
        snd_dma_free_pages(&mut (*bus).posbuf);
    }
    if !(*bus).rb.area.is_null() {
        snd_dma_free_pages(&mut (*bus).rb);
    }
    for_each_stream_safe(bus, |s| {
        if !(*s).bdl.area.is_null() {
            snd_dma_free_pages(&mut (*s).bdl);
        }
        list_del(&mut (*s).list);
        let hext_stream = stream_to_hdac_ext_stream(s);
        let hda_stream = hstream_to_sof_hda_stream(hext_stream);
        devm_kfree((*sdev).dev, hda_stream as *mut c_void);
    });
}

pub unsafe extern "C" fn hda_dsp_stream_get_position(hstream: *mut hdac_stream, direction: c_int, can_sleep: bool) -> snd_pcm_uframes_t {
    let hext_stream = stream_to_hdac_ext_stream(hstream);
    let hda_stream = hstream_to_sof_hda_stream(hext_stream);
    let sdev = (*hda_stream).sdev;
    let mut pos: snd_pcm_uframes_t;
    if sof_hda_position_quirk == SOF_HDA_POSITION_QUIRK_USE_SKYLAKE_LEGACY {
        if direction == SNDRV_PCM_STREAM_PLAYBACK {
            pos = snd_sof_dsp_read(sdev, HDA_DSP_HDA_BAR, AZX_REG_VS_SDXDPIB_XBASE + AZX_REG_VS_SDXDPIB_XINTERVAL * (*hstream).index as u32) as snd_pcm_uframes_t;
        } else {
            if can_sleep { usleep_range(20, 21); }
            snd_sof_dsp_read(sdev, HDA_DSP_HDA_BAR, AZX_REG_VS_SDXDPIB_XBASE + AZX_REG_VS_SDXDPIB_XINTERVAL * (*hstream).index as u32);
            pos = snd_hdac_stream_get_pos_posbuf(hstream);
        }
    } else if sof_hda_position_quirk == SOF_HDA_POSITION_QUIRK_USE_DPIB_REGISTERS {
        pos = snd_sof_dsp_read(sdev, HDA_DSP_HDA_BAR, AZX_REG_VS_SDXDPIB_XBASE + AZX_REG_VS_SDXDPIB_XINTERVAL * (*hstream).index as u32) as snd_pcm_uframes_t;
    } else if sof_hda_position_quirk == SOF_HDA_POSITION_QUIRK_USE_DPIB_DDR_UPDATE {
        pos = snd_hdac_stream_get_pos_posbuf(hstream);
    } else {
        dev_err_once((*sdev).dev, b"hda_position_quirk value %d not supported\n\0".as_ptr() as *const c_char, sof_hda_position_quirk);
        pos = 0;
    }
    if pos >= (*hstream).bufsize as u64 {
        pos = 0;
    }
    pos
}

pub unsafe extern "C" fn hda_dsp_get_stream_llp(_sdev: *mut snd_sof_dev, _component: *mut snd_soc_component, substream: *mut snd_pcm_substream) -> u64 {
    let rtd = snd_soc_substream_to_rtd(substream);
    let mut be_rtd: *mut snd_soc_pcm_runtime = core::ptr::null_mut();
    /*
     * for_each_dpcm_be(rtd, substream->stream, dpcm) is supplied externally in C.
     * File-local Rust cannot expand it without the dependency-provided list shape.
     */
    let _ = rtd;
    if be_rtd.is_null() {
        return 0;
    }
    let cpu_dai = snd_soc_rtd_to_cpu(be_rtd, 0);
    if cpu_dai.is_null() {
        return 0;
    }
    let hext_stream = snd_soc_dai_get_dma_data(cpu_dai, substream);
    if hext_stream.is_null() {
        return 0;
    }
    let llp_l = readl((*hext_stream).pplc_addr + AZX_REG_PPLCLLPL);
    let llp_u = readl((*hext_stream).pplc_addr + AZX_REG_PPLCLLPU);
    if (*hext_stream).pplcllpl != 0 || (*hext_stream).pplcllpu != 0 {
        return merge_u64(llp_u, llp_l).wrapping_sub(merge_u64((*hext_stream).pplcllpu, (*hext_stream).pplcllpl));
    }
    merge_u64(llp_u, llp_l)
}

pub unsafe extern "C" fn hda_dsp_get_stream_ldp(_sdev: *mut snd_sof_dev, _component: *mut snd_soc_component, substream: *mut snd_pcm_substream) -> u64 {
    let hstream = (*(*substream).runtime).private_data as *mut hdac_stream;
    let hext_stream = stream_to_hdac_ext_stream(hstream);
    let ldp_l = readl((*hext_stream).pphc_addr + AZX_REG_PPHCLDPL);
    let ldp_u = readl((*hext_stream).pphc_addr + AZX_REG_PPHCLDPU);
    ((ldp_u as u64) << 32) | ldp_l as u64
}

pub unsafe extern "C" fn hda_data_stream_prepare(
    dev: *mut device,
    format: c_uint,
    size: c_uint,
    dmab: *mut snd_dma_buffer,
    persistent_buffer: bool,
    direction: c_int,
    is_iccmax: bool,
    pair: bool,
) -> *mut hdac_ext_stream {
    let sdev = dev_get_drvdata(dev);
    let hext_stream = if pair { hda_dsp_stream_pair_get(sdev, direction, 0) } else { hda_dsp_stream_get(sdev, direction, 0) };
    if hext_stream.is_null() {
        dev_err((*sdev).dev, b"%s: no stream available\n\0".as_ptr() as *const c_char, b"hda_data_stream_prepare\0".as_ptr());
        return ERR_PTR(-ENODEV);
    }
    let hstream = &mut (*hext_stream).hstream as *mut hdac_stream;
    (*hstream).substream = core::ptr::null_mut();
    let mut ret: c_int;
    if !persistent_buffer || (*dmab).area.is_null() {
        ret = snd_dma_alloc_pages(SNDRV_DMA_TYPE_DEV_SG, dev, size as usize, dmab);
        if ret < 0 {
            dev_err((*sdev).dev, b"%s: memory alloc failed: %d\n\0".as_ptr() as *const c_char, b"hda_data_stream_prepare\0".as_ptr(), ret);
            if pair { hda_dsp_stream_pair_put(sdev, direction, (*hstream).stream_tag); } else { hda_dsp_stream_put(sdev, direction, (*hstream).stream_tag); }
            return ERR_PTR(ret);
        }
    }
    (*hstream).period_bytes = 0;
    (*hstream).format_val = format;
    (*hstream).bufsize = size;
    if is_iccmax {
        ret = hda_dsp_iccmax_stream_hw_params(sdev, hext_stream, dmab, core::ptr::null_mut());
        if ret < 0 {
            dev_err((*sdev).dev, b"%s: iccmax stream prepare failed: %d\n\0".as_ptr() as *const c_char, b"hda_data_stream_prepare\0".as_ptr(), ret);
            snd_dma_free_pages(dmab);
            (*dmab).area = core::ptr::null_mut();
            (*dmab).bytes = 0;
            (*hstream).bufsize = 0;
            (*hstream).format_val = 0;
            if pair { hda_dsp_stream_pair_put(sdev, direction, (*hstream).stream_tag); } else { hda_dsp_stream_put(sdev, direction, (*hstream).stream_tag); }
            return ERR_PTR(ret);
        }
    } else {
        ret = hda_dsp_stream_hw_params(sdev, hext_stream, dmab, core::ptr::null_mut());
        if ret < 0 {
            dev_err((*sdev).dev, b"%s: hdac prepare failed: %d\n\0".as_ptr() as *const c_char, b"hda_data_stream_prepare\0".as_ptr(), ret);
            snd_dma_free_pages(dmab);
            (*dmab).area = core::ptr::null_mut();
            (*dmab).bytes = 0;
            (*hstream).bufsize = 0;
            (*hstream).format_val = 0;
            if pair { hda_dsp_stream_pair_put(sdev, direction, (*hstream).stream_tag); } else { hda_dsp_stream_put(sdev, direction, (*hstream).stream_tag); }
            return ERR_PTR(ret);
        }
        hda_dsp_stream_spib_config(sdev, hext_stream, HDA_DSP_SPIB_ENABLE, size);
    }
    hext_stream
}

pub unsafe extern "C" fn hda_data_stream_cleanup(
    dev: *mut device,
    dmab: *mut snd_dma_buffer,
    persistent_buffer: bool,
    hext_stream: *mut hdac_ext_stream,
    is_iccmax: bool,
    pair: bool,
) -> c_int {
    let sdev = dev_get_drvdata(dev);
    let hstream = hdac_stream(hext_stream);
    let sd_offset = SOF_STREAM_SD_OFFSET(hstream) as u32;
    let mut ret = 0;
    if !is_iccmax {
        ret = hda_dsp_stream_spib_config(sdev, hext_stream, HDA_DSP_SPIB_DISABLE, 0);
    }
    if (*hstream).direction == SNDRV_PCM_STREAM_CAPTURE {
        snd_sof_dsp_update_bits(sdev, HDA_DSP_HDA_BAR, sd_offset, SOF_HDA_SD_CTL_DMA_START, 0);
    }
    if pair {
        hda_dsp_stream_pair_put(sdev, (*hstream).direction, (*hstream).stream_tag);
    } else {
        hda_dsp_stream_put(sdev, (*hstream).direction, (*hstream).stream_tag);
    }
    (*hstream).running = false;
    (*hstream).substream = core::ptr::null_mut();
    snd_sof_dsp_write(sdev, HDA_DSP_HDA_BAR, sd_offset + SOF_HDA_ADSP_REG_SD_BDLPL as u32, 0);
    snd_sof_dsp_write(sdev, HDA_DSP_HDA_BAR, sd_offset + SOF_HDA_ADSP_REG_SD_BDLPU as u32, 0);
    snd_sof_dsp_write(sdev, HDA_DSP_HDA_BAR, sd_offset, 0);
    if !persistent_buffer {
        snd_dma_free_pages(dmab);
        (*dmab).area = core::ptr::null_mut();
        (*dmab).bytes = 0;
        (*hstream).bufsize = 0;
        (*hstream).format_val = 0;
    }
    ret
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
