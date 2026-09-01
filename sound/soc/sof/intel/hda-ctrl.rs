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
 */

// Dependencies translated from:
// <linux/module.h>
// <sound/hdaudio_ext.h>
// <sound/hda_register.h>
// <sound/hda_component.h>
// <sound/hda-mlink.h>
// "../ops.h"
// "hda.h"

use core::ffi::{c_char, c_int, c_ulong, c_void};

#[repr(C)]
pub struct snd_sof_dev {
    pub dev: *mut c_void,
    pub pdata: *mut sof_dev_desc,
    pub bar: [*mut c_void; HDA_DSP_MAX_BAR],
}

#[repr(C)]
pub struct sof_dev_desc {
    pub hw_pdata: *mut c_void,
}

#[repr(C)]
pub struct sof_intel_hda_dev {
    pub link_dma_active_sdw_mask: [u32; SOF_HDA_LINK_DMA_MASK_SIZE],
    pub link_dma_active_multi_mask: [u32; SOF_HDA_LINK_DMA_MASK_SIZE],
    pub link_dma_out_hda_used_mask: u32,
    pub l1_disabled: bool,
}

#[repr(C)]
pub struct hdac_bus {
    pub dev: *mut c_void,
    pub ppcap: *mut c_void,
    pub spbcap: *mut c_void,
    pub drsmcap: *mut c_void,
    pub gtscap: *mut c_void,
    pub mlcap: *mut c_void,
    pub remap_addr: *mut c_void,
    pub chip_init: bool,
    pub stream_list: list_head,
    pub codec_mask: u32,
    pub use_posbuf: bool,
    pub posbuf: hdac_bus_posbuf,
}

#[repr(C)]
pub struct hdac_bus_posbuf {
    pub addr: u64,
}

#[repr(C)]
pub struct hdac_stream {
    pub list: list_head,
}

#[repr(C)]
pub struct list_head {
    pub next: *mut list_head,
    pub prev: *mut list_head,
}

pub const EIO: c_int = 5;

pub const HDA_DSP_MAX_BAR: usize = 8;
pub const HDA_DSP_HDA_BAR: u32 = 0;
pub const HDA_DSP_PP_BAR: usize = 1;
pub const HDA_DSP_SPIB_BAR: usize = 2;
pub const HDA_DSP_DRSM_BAR: usize = 3;

pub const SOF_HDA_LINK_DMA_MASK_SIZE: usize = 4;

extern "C" {
    static mut jiffies: c_ulong;

    static HDA_DSP_CTRL_RESET_TIMEOUT: c_ulong;
    static SOF_HDA_GCTL: u32;
    static SOF_HDA_GCTL_RESET: u32;
    static SOF_HDA_LLCH: u32;
    static SOF_HDA_CAP_NEXT_MASK: u32;
    static SOF_HDA_CAP_ID_MASK: u32;
    static SOF_HDA_CAP_ID_OFF: u32;
    static SOF_HDA_PP_CAP_ID: u32;
    static SOF_HDA_SPIB_CAP_ID: u32;
    static SOF_HDA_DRSM_CAP_ID: u32;
    static SOF_HDA_GTS_CAP_ID: u32;
    static SOF_HDA_ML_CAP_ID: u32;
    static SOF_HDA_MAX_CAPS: c_int;
    static SOF_HDA_REG_PP_PPCTL: u32;
    static SOF_HDA_PPCTL_GPROCEN: u32;
    static SOF_HDA_PPCTL_PIE: u32;
    static PCI_CGCTL: u32;
    static PCI_CGCTL_MISCBDCGE_MASK: u32;
    static PCI_CGCTL_ADSPDCGE: u32;
    static HDA_VS_INTEL_EM2: u32;
    static HDA_VS_INTEL_EM2_L1SEN: u32;
    static PCI_PGCTL: u32;
    static PCI_PGCTL_ADSPPGD: u32;
    static SOF_HDA_WAKESTS: u32;
    static SOF_HDA_WAKESTS_INT_MASK: u32;
    static GCTL: u32;
    static AZX_GCTL_UNSOL: u32;
    static SOF_HDA_ADSP_REG_SD_STS: u32;
    static SOF_HDA_CL_DMA_SD_INT_MASK: u32;
    static SOF_HDA_INTSTS: u32;
    static SOF_HDA_INT_CTRL_EN: u32;
    static SOF_HDA_INT_ALL_STREAM: u32;
    static SOF_HDA_INTCTL: u32;
    static SOF_HDA_INT_GLOBAL_EN: u32;
    static SOF_HDA_ADSP_DPLBASE: u32;
    static SOF_HDA_ADSP_DPUBASE: u32;
    static SOF_HDA_ADSP_REG_SD_CTL: u32;

    fn msecs_to_jiffies(msecs: c_ulong) -> c_ulong;
    fn time_before(a: c_ulong, b: c_ulong) -> bool;
    fn usleep_range(min: c_ulong, max: c_ulong);
    fn snd_sof_dsp_update_bits(sdev: *mut snd_sof_dev, bar: u32, offset: u32, mask: u32, value: u32);
    fn snd_sof_dsp_read(sdev: *mut snd_sof_dev, bar: u32, offset: u32) -> u32;
    fn snd_sof_dsp_write(sdev: *mut snd_sof_dev, bar: u32, offset: u32, value: u32);
    fn snd_sof_pci_update_bits(sdev: *mut snd_sof_dev, offset: u32, mask: u32, value: u32);
    fn sof_to_bus(sdev: *mut snd_sof_dev) -> *mut hdac_bus;
    fn bus_to_sof_hda(bus: *mut hdac_bus) -> *mut sof_intel_hda_dev;
    fn hda_codec_set_codec_wakeup(sdev: *mut snd_sof_dev, enable: bool);
    fn snd_hdac_chip_updatel(bus: *mut hdac_bus, reg: u32, mask: u32, value: u32);
    fn hda_bus_ml_init(bus: *mut hdac_bus) -> c_int;
    fn hda_codec_detect_mask(sdev: *mut snd_sof_dev);
    fn hda_codec_rirb_status_clear(sdev: *mut snd_sof_dev);
    fn hda_codec_init_cmd_io(sdev: *mut snd_sof_dev);
    fn hda_bus_ml_reset_losidv(bus: *mut hdac_bus);
    fn hda_codec_stop_cmd_io(sdev: *mut snd_sof_dev);
    fn SOF_STREAM_SD_OFFSET(stream: *mut hdac_stream) -> c_int;
    fn dev_err(dev: *mut c_void, fmt: *const c_char, ...);
    fn dev_dbg(dev: *mut c_void, fmt: *const c_char, ...);
}

unsafe fn upper_32_bits(n: u64) -> u32 {
    (n >> 32) as u32
}

unsafe fn list_entry_hdac_stream(_head: *mut list_head) -> *mut hdac_stream {
    // TODO: requires the external kernel container_of/list_entry definition.
    core::ptr::null_mut()
}

/*
 * HDA Operations.
 */

#[no_mangle]
pub unsafe extern "C" fn hda_dsp_ctrl_link_reset(sdev: *mut snd_sof_dev, reset: bool) -> c_int {
    let timeout: c_ulong;
    let mut gctl: u32 = 0;
    let val: u32;

    /* 0 to enter reset and 1 to exit reset */
    val = if reset { 0 } else { SOF_HDA_GCTL_RESET };

    /* enter/exit HDA controller reset */
    snd_sof_dsp_update_bits(
        sdev,
        HDA_DSP_HDA_BAR,
        SOF_HDA_GCTL,
        SOF_HDA_GCTL_RESET,
        val,
    );

    /* wait to enter/exit reset */
    timeout = jiffies.wrapping_add(msecs_to_jiffies(HDA_DSP_CTRL_RESET_TIMEOUT));
    while time_before(jiffies, timeout) {
        gctl = snd_sof_dsp_read(sdev, HDA_DSP_HDA_BAR, SOF_HDA_GCTL);
        if (gctl & SOF_HDA_GCTL_RESET) == val {
            return 0;
        }
        usleep_range(500, 1000);
    }

    /* enter/exit reset failed */
    dev_err(
        (*sdev).dev,
        b"error: failed to %s HDA controller gctl 0x%x\n\0".as_ptr() as *const c_char,
        if reset {
            b"reset\0".as_ptr() as *const c_char
        } else {
            b"ready\0".as_ptr() as *const c_char
        },
        gctl,
    );
    -EIO
}

#[no_mangle]
pub unsafe extern "C" fn hda_dsp_ctrl_get_caps(sdev: *mut snd_sof_dev) -> c_int {
    let bus: *mut hdac_bus = sof_to_bus(sdev);
    let mut cap: u32;
    let mut offset: u32;
    let feature: u32;
    let mut count: c_int = 0;
    let mut ret: c_int;

    /*
     * On some devices, one reset cycle is necessary before reading
     * capabilities
     */
    ret = hda_dsp_ctrl_link_reset(sdev, true);
    if ret < 0 {
        return ret;
    }
    ret = hda_dsp_ctrl_link_reset(sdev, false);
    if ret < 0 {
        return ret;
    }

    offset = snd_sof_dsp_read(sdev, HDA_DSP_HDA_BAR, SOF_HDA_LLCH);

    loop {
        dev_dbg(
            (*sdev).dev,
            b"checking for capabilities at offset 0x%x\n\0".as_ptr() as *const c_char,
            offset & SOF_HDA_CAP_NEXT_MASK,
        );

        cap = snd_sof_dsp_read(sdev, HDA_DSP_HDA_BAR, offset);

        if cap == u32::MAX {
            dev_dbg((*bus).dev, b"Invalid capability reg read\n\0".as_ptr() as *const c_char);
            break;
        }

        feature = (cap & SOF_HDA_CAP_ID_MASK) >> SOF_HDA_CAP_ID_OFF;

        if feature == SOF_HDA_PP_CAP_ID {
            dev_dbg(
                (*sdev).dev,
                b"found DSP capability at 0x%x\n\0".as_ptr() as *const c_char,
                offset,
            );
            (*bus).ppcap = ((*bus).remap_addr as *mut u8).add(offset as usize) as *mut c_void;
            (*sdev).bar[HDA_DSP_PP_BAR] = (*bus).ppcap;
        } else if feature == SOF_HDA_SPIB_CAP_ID {
            dev_dbg(
                (*sdev).dev,
                b"found SPIB capability at 0x%x\n\0".as_ptr() as *const c_char,
                offset,
            );
            (*bus).spbcap = ((*bus).remap_addr as *mut u8).add(offset as usize) as *mut c_void;
            (*sdev).bar[HDA_DSP_SPIB_BAR] = (*bus).spbcap;
        } else if feature == SOF_HDA_DRSM_CAP_ID {
            dev_dbg(
                (*sdev).dev,
                b"found DRSM capability at 0x%x\n\0".as_ptr() as *const c_char,
                offset,
            );
            (*bus).drsmcap = ((*bus).remap_addr as *mut u8).add(offset as usize) as *mut c_void;
            (*sdev).bar[HDA_DSP_DRSM_BAR] = (*bus).drsmcap;
        } else if feature == SOF_HDA_GTS_CAP_ID {
            dev_dbg(
                (*sdev).dev,
                b"found GTS capability at 0x%x\n\0".as_ptr() as *const c_char,
                offset,
            );
            (*bus).gtscap = ((*bus).remap_addr as *mut u8).add(offset as usize) as *mut c_void;
        } else if feature == SOF_HDA_ML_CAP_ID {
            dev_dbg(
                (*sdev).dev,
                b"found ML capability at 0x%x\n\0".as_ptr() as *const c_char,
                offset,
            );
            (*bus).mlcap = ((*bus).remap_addr as *mut u8).add(offset as usize) as *mut c_void;
        } else {
            dev_dbg(
                (*sdev).dev,
                b"found capability %d at 0x%x\n\0".as_ptr() as *const c_char,
                feature,
                offset,
            );
        }

        offset = cap & SOF_HDA_CAP_NEXT_MASK;
        let continue_loop = count <= SOF_HDA_MAX_CAPS && offset != 0;
        count += 1;
        if !continue_loop {
            break;
        }
    }

    0
}
// EXPORT_SYMBOL_NS(hda_dsp_ctrl_get_caps, "SND_SOC_SOF_INTEL_HDA_COMMON");

#[no_mangle]
pub unsafe extern "C" fn hda_dsp_ctrl_ppcap_enable(sdev: *mut snd_sof_dev, enable: bool) {
    let val: u32 = if enable { SOF_HDA_PPCTL_GPROCEN } else { 0 };

    snd_sof_dsp_update_bits(
        sdev,
        HDA_DSP_PP_BAR as u32,
        SOF_HDA_REG_PP_PPCTL,
        SOF_HDA_PPCTL_GPROCEN,
        val,
    );
}
// EXPORT_SYMBOL_NS(hda_dsp_ctrl_ppcap_enable, "SND_SOC_SOF_INTEL_HDA_COMMON");

#[no_mangle]
pub unsafe extern "C" fn hda_dsp_ctrl_ppcap_int_enable(sdev: *mut snd_sof_dev, enable: bool) {
    let val: u32 = if enable { SOF_HDA_PPCTL_PIE } else { 0 };

    snd_sof_dsp_update_bits(
        sdev,
        HDA_DSP_PP_BAR as u32,
        SOF_HDA_REG_PP_PPCTL,
        SOF_HDA_PPCTL_PIE,
        val,
    );
}
// EXPORT_SYMBOL_NS(hda_dsp_ctrl_ppcap_int_enable, "SND_SOC_SOF_INTEL_HDA_COMMON");

#[no_mangle]
pub unsafe extern "C" fn hda_dsp_ctrl_misc_clock_gating(sdev: *mut snd_sof_dev, enable: bool) {
    let val: u32 = if enable { PCI_CGCTL_MISCBDCGE_MASK } else { 0 };

    snd_sof_pci_update_bits(sdev, PCI_CGCTL, PCI_CGCTL_MISCBDCGE_MASK, val);
}

/*
 * enable/disable audio dsp clock gating and power gating bits.
 * This allows the HW to opportunistically power and clock gate
 * the audio dsp when it is idle
 */
#[no_mangle]
pub unsafe extern "C" fn hda_dsp_ctrl_clock_power_gating(
    sdev: *mut snd_sof_dev,
    enable: bool,
) -> c_int {
    let hda: *mut sof_intel_hda_dev = (*(*sdev).pdata).hw_pdata as *mut sof_intel_hda_dev;
    let mut val: u32;

    /* enable/disable audio dsp clock gating */
    val = if enable { PCI_CGCTL_ADSPDCGE } else { 0 };
    snd_sof_pci_update_bits(sdev, PCI_CGCTL, PCI_CGCTL_ADSPDCGE, val);

    /* disable the DMI link when requested. But enable only if it wasn't disabled previously */
    val = if enable { HDA_VS_INTEL_EM2_L1SEN } else { 0 };
    if !enable || !(*hda).l1_disabled {
        snd_sof_dsp_update_bits(
            sdev,
            HDA_DSP_HDA_BAR,
            HDA_VS_INTEL_EM2,
            HDA_VS_INTEL_EM2_L1SEN,
            val,
        );
    }

    /* enable/disable audio dsp power gating */
    val = if enable { 0 } else { PCI_PGCTL_ADSPPGD };
    snd_sof_pci_update_bits(sdev, PCI_PGCTL, PCI_PGCTL_ADSPPGD, val);

    0
}
// EXPORT_SYMBOL_NS(hda_dsp_ctrl_clock_power_gating, "SND_SOC_SOF_INTEL_HDA_COMMON");

#[no_mangle]
pub unsafe extern "C" fn hda_dsp_ctrl_init_chip(
    sdev: *mut snd_sof_dev,
    detect_codec: bool,
) -> c_int {
    let bus: *mut hdac_bus = sof_to_bus(sdev);
    let sof_hda: *mut sof_intel_hda_dev = bus_to_sof_hda(bus);
    let mut stream: *mut hdac_stream;
    let mut sd_offset: c_int;
    let mut ret: c_int = 0;
    let gctl: u32;

    if (*bus).chip_init {
        return 0;
    }

    /*
     * The controller reset clears the ACE2+ link DMA stream allocation
     * constraints; reset the masks to reflect this.
     */
    core::ptr::write_bytes(
        (*sof_hda).link_dma_active_sdw_mask.as_mut_ptr(),
        0,
        (*sof_hda).link_dma_active_sdw_mask.len(),
    );
    core::ptr::write_bytes(
        (*sof_hda).link_dma_active_multi_mask.as_mut_ptr(),
        0,
        (*sof_hda).link_dma_active_multi_mask.len(),
    );
    (*sof_hda).link_dma_out_hda_used_mask = 0;

    hda_codec_set_codec_wakeup(sdev, true);

    hda_dsp_ctrl_misc_clock_gating(sdev, false);

    /* clear WAKE_STS if not in reset */
    gctl = snd_sof_dsp_read(sdev, HDA_DSP_HDA_BAR, SOF_HDA_GCTL);
    if (gctl & SOF_HDA_GCTL_RESET) != 0 {
        snd_sof_dsp_write(
            sdev,
            HDA_DSP_HDA_BAR,
            SOF_HDA_WAKESTS,
            SOF_HDA_WAKESTS_INT_MASK,
        );
    }

    /* reset HDA controller */
    ret = hda_dsp_ctrl_link_reset(sdev, true);
    if ret < 0 {
        dev_err(
            (*sdev).dev,
            b"error: failed to reset HDA controller\n\0".as_ptr() as *const c_char,
        );
        hda_dsp_ctrl_misc_clock_gating(sdev, true);
        hda_codec_set_codec_wakeup(sdev, false);
        return ret;
    }

    usleep_range(500, 1000);

    /* exit HDA controller reset */
    ret = hda_dsp_ctrl_link_reset(sdev, false);
    if ret < 0 {
        dev_err(
            (*sdev).dev,
            b"error: failed to exit HDA controller reset\n\0".as_ptr() as *const c_char,
        );
        hda_dsp_ctrl_misc_clock_gating(sdev, true);
        hda_codec_set_codec_wakeup(sdev, false);
        return ret;
    }
    usleep_range(1000, 1200);

    /* Accept unsolicited responses */
    snd_hdac_chip_updatel(bus, GCTL, AZX_GCTL_UNSOL, AZX_GCTL_UNSOL);

    /* Perform a one-time enumeration of the Multi-Link capability */
    ret = hda_bus_ml_init(bus);
    if ret < 0 {
        dev_err(
            (*sdev).dev,
            b"%s: failed to enumerate multi-links\n\0".as_ptr() as *const c_char,
            b"hda_dsp_ctrl_init_chip\0".as_ptr() as *const c_char,
        );
        hda_dsp_ctrl_misc_clock_gating(sdev, true);
        hda_codec_set_codec_wakeup(sdev, false);
        return ret;
    }

    if detect_codec {
        hda_codec_detect_mask(sdev);
    }

    /* clear stream status */
    // list_for_each_entry(stream, &bus->stream_list, list)
    let mut pos = (*bus).stream_list.next;
    while pos != &mut (*bus).stream_list as *mut list_head {
        stream = list_entry_hdac_stream(pos);
        sd_offset = SOF_STREAM_SD_OFFSET(stream);
        snd_sof_dsp_write(
            sdev,
            HDA_DSP_HDA_BAR,
            (sd_offset as u32).wrapping_add(SOF_HDA_ADSP_REG_SD_STS),
            SOF_HDA_CL_DMA_SD_INT_MASK,
        );
        pos = (*pos).next;
    }

    /* clear WAKESTS */
    snd_sof_dsp_write(sdev, HDA_DSP_HDA_BAR, SOF_HDA_WAKESTS, (*bus).codec_mask);

    hda_codec_rirb_status_clear(sdev);

    /* clear interrupt status register */
    snd_sof_dsp_write(
        sdev,
        HDA_DSP_HDA_BAR,
        SOF_HDA_INTSTS,
        SOF_HDA_INT_CTRL_EN | SOF_HDA_INT_ALL_STREAM,
    );

    hda_codec_init_cmd_io(sdev);

    /* enable CIE and GIE interrupts */
    snd_sof_dsp_update_bits(
        sdev,
        HDA_DSP_HDA_BAR,
        SOF_HDA_INTCTL,
        SOF_HDA_INT_CTRL_EN | SOF_HDA_INT_GLOBAL_EN,
        SOF_HDA_INT_CTRL_EN | SOF_HDA_INT_GLOBAL_EN,
    );

    /* program the position buffer */
    if (*bus).use_posbuf && (*bus).posbuf.addr != 0 {
        snd_sof_dsp_write(
            sdev,
            HDA_DSP_HDA_BAR,
            SOF_HDA_ADSP_DPLBASE,
            (*bus).posbuf.addr as u32,
        );
        snd_sof_dsp_write(
            sdev,
            HDA_DSP_HDA_BAR,
            SOF_HDA_ADSP_DPUBASE,
            upper_32_bits((*bus).posbuf.addr),
        );
    }

    hda_bus_ml_reset_losidv(bus);

    (*bus).chip_init = true;

    hda_dsp_ctrl_misc_clock_gating(sdev, true);

    hda_codec_set_codec_wakeup(sdev, false);

    ret
}
// EXPORT_SYMBOL_NS(hda_dsp_ctrl_init_chip, "SND_SOC_SOF_INTEL_HDA_COMMON");

#[no_mangle]
pub unsafe extern "C" fn hda_dsp_ctrl_stop_chip(sdev: *mut snd_sof_dev) {
    let bus: *mut hdac_bus = sof_to_bus(sdev);
    let mut stream: *mut hdac_stream;
    let mut sd_offset: c_int;

    if !(*bus).chip_init {
        return;
    }

    /* disable interrupts in stream descriptor */
    // list_for_each_entry(stream, &bus->stream_list, list)
    let mut pos = (*bus).stream_list.next;
    while pos != &mut (*bus).stream_list as *mut list_head {
        stream = list_entry_hdac_stream(pos);
        sd_offset = SOF_STREAM_SD_OFFSET(stream);
        snd_sof_dsp_update_bits(
            sdev,
            HDA_DSP_HDA_BAR,
            (sd_offset as u32).wrapping_add(SOF_HDA_ADSP_REG_SD_CTL),
            SOF_HDA_CL_DMA_SD_INT_MASK,
            0,
        );
        pos = (*pos).next;
    }

    /* disable SIE for all streams */
    snd_sof_dsp_update_bits(
        sdev,
        HDA_DSP_HDA_BAR,
        SOF_HDA_INTCTL,
        SOF_HDA_INT_ALL_STREAM,
        0,
    );

    /* disable controller CIE and GIE */
    snd_sof_dsp_update_bits(
        sdev,
        HDA_DSP_HDA_BAR,
        SOF_HDA_INTCTL,
        SOF_HDA_INT_CTRL_EN | SOF_HDA_INT_GLOBAL_EN,
        0,
    );

    /* clear stream status */
    // list_for_each_entry(stream, &bus->stream_list, list)
    pos = (*bus).stream_list.next;
    while pos != &mut (*bus).stream_list as *mut list_head {
        stream = list_entry_hdac_stream(pos);
        sd_offset = SOF_STREAM_SD_OFFSET(stream);
        snd_sof_dsp_write(
            sdev,
            HDA_DSP_HDA_BAR,
            (sd_offset as u32).wrapping_add(SOF_HDA_ADSP_REG_SD_STS),
            SOF_HDA_CL_DMA_SD_INT_MASK,
        );
        pos = (*pos).next;
    }

    /* clear WAKESTS */
    snd_sof_dsp_write(
        sdev,
        HDA_DSP_HDA_BAR,
        SOF_HDA_WAKESTS,
        SOF_HDA_WAKESTS_INT_MASK,
    );

    hda_codec_rirb_status_clear(sdev);

    /* clear interrupt status register */
    snd_sof_dsp_write(
        sdev,
        HDA_DSP_HDA_BAR,
        SOF_HDA_INTSTS,
        SOF_HDA_INT_CTRL_EN | SOF_HDA_INT_ALL_STREAM,
    );

    hda_codec_stop_cmd_io(sdev);

    /* disable position buffer */
    if (*bus).use_posbuf && (*bus).posbuf.addr != 0 {
        snd_sof_dsp_write(sdev, HDA_DSP_HDA_BAR, SOF_HDA_ADSP_DPLBASE, 0);
        snd_sof_dsp_write(sdev, HDA_DSP_HDA_BAR, SOF_HDA_ADSP_DPUBASE, 0);
    }

    (*bus).chip_init = false;
}

// MODULE_LICENSE("Dual BSD/GPL");
// MODULE_DESCRIPTION("SOF helpers for HDaudio platforms");
// MODULE_IMPORT_NS("SND_SOC_SOF_HDA_MLINK");
// MODULE_IMPORT_NS("SND_SOC_SOF_HDA_AUDIO_CODEC");
// MODULE_IMPORT_NS("SND_SOC_SOF_HDA_AUDIO_CODEC_I915");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
