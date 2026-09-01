// SPDX-License-Identifier: (GPL-2.0-only OR BSD-3-Clause)
//
// This file is provided under a dual BSD/GPLv2 license.  When using or
// redistributing this file, you may do so under either license.
//
// Copyright(c) 2018-2022 Intel Corporation
//

use core::ffi::{c_char, c_int, c_uint, c_void};

type u8 = u8;
type u32 = u32;
type PhysAddrT = usize;
type Le32 = u32;

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct firmware {
    pub size: usize,
    pub data: *const u8,
}

#[repr(C)]
pub struct snd_dma_buffer {
    pub dev: *mut c_void,
    pub area: *mut c_void,
    pub addr: usize,
    pub bytes: usize,
}

#[repr(C)]
pub struct sof_intel_dsp_desc {
    pub init_core_mask: c_uint,
    pub ipc_ctl: c_uint,
    pub rom_status_reg: c_uint,
    pub rom_init_timeout: c_uint,
}

#[repr(C)]
pub struct sof_intel_hda_dev {
    pub desc: *const sof_intel_dsp_desc,
    pub code_loading: c_int,
    pub waitq: c_void,
}

#[repr(C)]
pub struct snd_sof_pdata {
    pub hw_pdata: *mut sof_intel_hda_dev,
}

#[repr(C)]
pub struct snd_sof_basefw {
    pub fw: *const firmware,
    pub payload_offset: usize,
}

#[repr(C)]
pub struct snd_sof_dev {
    pub dev: *mut device,
    pub pdata: *mut snd_sof_pdata,
    pub basefw: snd_sof_basefw,
}

unsafe extern "C" {
    static HDA_DSP_BAR: c_uint;
    static HDA_DSP_REG_ADSPIC: c_uint;
    static HDA_DSP_ADSPIC_CL_DMA: u32;
    static HDA_DSP_ADSPIC_IPC: u32;
    static HDA_DSP_REG_HIPCCTL_DONE: u32;
    static HDA_DSP_REG_HIPCCTL_BUSY: u32;
    static HDA_DSP_REG_POLL_INTERVAL_US: c_uint;
    static HDA_DSP_BASEFW_TIMEOUT_US: c_uint;
    static HDA_DSP_SRAM_REG_ROM_ERROR: c_uint;
    static SOF_DBG_DUMP_PCI: u32;
    static SOF_DBG_DUMP_MBOX: u32;
    static SOF_HDA_ADSP_LOADER_BASE: c_int;
    static SOF_HDA_ADSP_REG_SD_CTL: c_int;
    static SOF_HDA_ADSP_REG_SD_BDLPL: c_int;
    static SOF_HDA_ADSP_REG_SD_BDLPU: c_int;
    static SOF_HDA_ADSP_REG_SD_CBL: c_int;
    static SOF_HDA_ADSP_REG_SD_LVI: c_int;
    static SOF_DSP_REG_CL_SPBFIFO: c_int;
    static SOF_HDA_ADSP_REG_CL_SPBFIFO_SPBFCCTL: c_int;
    static SOF_HDA_ADSP_REG_CL_SPBFIFO_SPIB: c_int;
    static SOF_HDA_ADSP_REG_SD_STS: c_int;
    static FSR_STATE_INIT_DONE: u32;
    static FSR_STATE_ROM_BASEFW_ENTERED: u32;
    static SNDRV_DMA_TYPE_DEV: c_int;
    static USEC_PER_MSEC: c_uint;
    static EIO: c_int;
    static EINVAL: c_int;
    static PAGE_SIZE: usize;

    fn snd_sof_dsp_update_bits(sdev: *mut snd_sof_dev, bar: c_uint, offset: c_uint,
                               mask: u32, value: u32) -> c_int;
    fn snd_sof_dsp_write(sdev: *mut snd_sof_dev, bar: c_uint, offset: c_uint, value: u32);
    fn snd_sof_dsp_read(sdev: *mut snd_sof_dev, bar: c_uint, offset: c_uint) -> u32;
    fn snd_sof_dsp_read_poll_timeout(sdev: *mut snd_sof_dev, bar: c_uint, offset: c_uint,
                                     val: *mut c_uint, cond_state: u32,
                                     sleep_us: c_uint, timeout_us: c_uint) -> c_int;
    fn snd_sof_dsp_dbg_dump(sdev: *mut snd_sof_dev, msg: *const c_char, flags: u32);
    fn snd_dma_alloc_pages(t: c_int, dev: *mut device, size: usize,
                           dmab: *mut snd_dma_buffer) -> c_int;
    fn snd_dma_free_pages(dmab: *mut snd_dma_buffer);
    fn hda_dsp_core_is_enabled(sdev: *mut snd_sof_dev, mask: c_uint) -> bool;
    fn hda_dsp_core_stall_reset(sdev: *mut snd_sof_dev, mask: c_uint) -> c_int;
    fn hda_dsp_core_run(sdev: *mut snd_sof_dev, mask: c_uint) -> c_int;
    fn hda_dsp_core_reset_power_down(sdev: *mut snd_sof_dev, mask: c_uint) -> c_int;
    fn hda_dsp_enable_core(sdev: *mut snd_sof_dev, mask: c_uint) -> c_int;
    fn virt_to_phys(ptr: *mut c_void) -> PhysAddrT;
    fn cpu_to_le32(x: u32) -> Le32;
    fn udelay(usecs: c_uint);
    fn msecs_to_jiffies(msecs: c_uint) -> usize;
    fn wait_event_timeout(waitq: *mut c_void, condition: bool, timeout: usize) -> c_int;
    fn memcpy(dst: *mut c_void, src: *const c_void, n: usize) -> *mut c_void;
    fn FSR_TO_STATE_CODE(status: c_uint) -> u32;
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn dev_dbg(dev: *mut device, fmt: *const c_char, ...);
}

const HDA_SKL_WAIT_TIMEOUT: c_uint = 500; /* 500 msec */

#[inline]
unsafe fn HDA_SKL_CLDMA_MAX_BUFFER_SIZE() -> usize {
    32usize.wrapping_mul(PAGE_SIZE)
}

/* Stream Reset */
const HDA_CL_SD_CTL_SRST_SHIFT: u32 = 0;
#[inline]
fn HDA_CL_SD_CTL_SRST(x: u32) -> u32 {
    ((x & 0x1) << HDA_CL_SD_CTL_SRST_SHIFT)
}

/* Stream Run */
const HDA_CL_SD_CTL_RUN_SHIFT: u32 = 1;
#[inline]
fn HDA_CL_SD_CTL_RUN(x: u32) -> u32 {
    ((x & 0x1) << HDA_CL_SD_CTL_RUN_SHIFT)
}

/* Interrupt On Completion Enable */
const HDA_CL_SD_CTL_IOCE_SHIFT: u32 = 2;
#[inline]
fn HDA_CL_SD_CTL_IOCE(x: u32) -> u32 {
    ((x & 0x1) << HDA_CL_SD_CTL_IOCE_SHIFT)
}

/* FIFO Error Interrupt Enable */
const HDA_CL_SD_CTL_FEIE_SHIFT: u32 = 3;
#[inline]
fn HDA_CL_SD_CTL_FEIE(x: u32) -> u32 {
    ((x & 0x1) << HDA_CL_SD_CTL_FEIE_SHIFT)
}

/* Descriptor Error Interrupt Enable */
const HDA_CL_SD_CTL_DEIE_SHIFT: u32 = 4;
#[inline]
fn HDA_CL_SD_CTL_DEIE(x: u32) -> u32 {
    ((x & 0x1) << HDA_CL_SD_CTL_DEIE_SHIFT)
}

/* FIFO Limit Change */
const HDA_CL_SD_CTL_FIFOLC_SHIFT: u32 = 5;
#[inline]
fn HDA_CL_SD_CTL_FIFOLC(x: u32) -> u32 {
    ((x & 0x1) << HDA_CL_SD_CTL_FIFOLC_SHIFT)
}

/* Stripe Control */
const HDA_CL_SD_CTL_STRIPE_SHIFT: u32 = 16;
#[inline]
fn HDA_CL_SD_CTL_STRIPE(x: u32) -> u32 {
    ((x & 0x3) << HDA_CL_SD_CTL_STRIPE_SHIFT)
}

/* Traffic Priority */
const HDA_CL_SD_CTL_TP_SHIFT: u32 = 18;
#[inline]
fn HDA_CL_SD_CTL_TP(x: u32) -> u32 {
    ((x & 0x1) << HDA_CL_SD_CTL_TP_SHIFT)
}

/* Bidirectional Direction Control */
const HDA_CL_SD_CTL_DIR_SHIFT: u32 = 19;
#[inline]
fn HDA_CL_SD_CTL_DIR(x: u32) -> u32 {
    ((x & 0x1) << HDA_CL_SD_CTL_DIR_SHIFT)
}

/* Stream Number */
const HDA_CL_SD_CTL_STRM_SHIFT: u32 = 20;
#[inline]
fn HDA_CL_SD_CTL_STRM(x: u32) -> u32 {
    ((x & 0xf) << HDA_CL_SD_CTL_STRM_SHIFT)
}

#[inline]
fn HDA_CL_SD_CTL_INT(x: u32) -> u32 {
    HDA_CL_SD_CTL_IOCE(x) | HDA_CL_SD_CTL_FEIE(x) | HDA_CL_SD_CTL_DEIE(x)
}

#[inline]
fn HDA_CL_SD_CTL_INT_MASK() -> u32 {
    HDA_CL_SD_CTL_IOCE(1) | HDA_CL_SD_CTL_FEIE(1) | HDA_CL_SD_CTL_DEIE(1)
}

const DMA_ADDRESS_128_BITS_ALIGNMENT: u32 = 7;
#[inline]
fn BDL_ALIGN(x: u32) -> u32 {
    x >> DMA_ADDRESS_128_BITS_ALIGNMENT
}

/* Buffer Descriptor List Lower Base Address */
const HDA_CL_SD_BDLPLBA_SHIFT: u32 = 7;
const HDA_CL_SD_BDLPLBA_MASK: u32 = 0xffff_ff80;
#[inline]
fn lower_32_bits(x: usize) -> u32 {
    x as u32
}
#[inline]
fn upper_32_bits(x: usize) -> u32 {
    (x >> 32) as u32
}
#[inline]
fn HDA_CL_SD_BDLPLBA(x: usize) -> u32 {
    (BDL_ALIGN(lower_32_bits(x)) << HDA_CL_SD_BDLPLBA_SHIFT) & HDA_CL_SD_BDLPLBA_MASK
}

/* Buffer Descriptor List Upper Base Address */
#[inline]
fn HDA_CL_SD_BDLPUBA(x: usize) -> u32 {
    upper_32_bits(x)
}

/* Software Position in Buffer Enable */
const HDA_CL_SPBFIFO_SPBFCCTL_SPIBE_SHIFT: u32 = 0;
const HDA_CL_SPBFIFO_SPBFCCTL_SPIBE_MASK: u32 = 1 << HDA_CL_SPBFIFO_SPBFCCTL_SPIBE_SHIFT;

#[inline]
fn HDA_CL_SPBFIFO_SPBFCCTL_SPIBE(x: u32) -> u32 {
    (x << HDA_CL_SPBFIFO_SPBFCCTL_SPIBE_SHIFT) & HDA_CL_SPBFIFO_SPBFCCTL_SPIBE_MASK
}

const HDA_CL_DMA_SD_INT_COMPLETE: u8 = 0x4;

unsafe fn cl_skl_cldma_setup_bdle(sdev: *mut snd_sof_dev,
                                  dmab_data: *mut snd_dma_buffer,
                                  bdlp: *mut *mut Le32, size: c_int,
                                  with_ioc: c_int) -> c_int {
    let addr: PhysAddrT = virt_to_phys((*dmab_data).area);
    let bdl: *mut Le32 = *bdlp;

    /*
     * This code is simplified by using one fragment of physical memory and assuming
     * all the code fits. This could be improved with scatter-gather but the firmware
     * size is limited by DSP memory anyways
     */
    *bdl.add(0) = cpu_to_le32(lower_32_bits(addr));
    *bdl.add(1) = cpu_to_le32(upper_32_bits(addr));
    *bdl.add(2) = cpu_to_le32(size as u32);
    *bdl.add(3) = if with_ioc == 0 { 0 } else { cpu_to_le32(0x01) };

    1 /* one fragment */
}

unsafe fn cl_skl_cldma_stream_run(sdev: *mut snd_sof_dev, enable: bool) {
    let sd_offset: c_int = SOF_HDA_ADSP_LOADER_BASE;
    let mut val: u8;
    let mut retries: c_int;
    let run: u32 = if enable { 0x1 } else { 0 };

    snd_sof_dsp_update_bits(sdev, HDA_DSP_BAR,
                            (sd_offset + SOF_HDA_ADSP_REG_SD_CTL) as c_uint,
                            HDA_CL_SD_CTL_RUN(1), HDA_CL_SD_CTL_RUN(run));

    retries = 300;
    loop {
        udelay(3);

        /* waiting for hardware to report the stream Run bit set */
        val = snd_sof_dsp_read(sdev, HDA_DSP_BAR,
                               (sd_offset + SOF_HDA_ADSP_REG_SD_CTL) as c_uint) as u8;
        val &= HDA_CL_SD_CTL_RUN(1) as u8;
        if enable && val != 0 {
            break;
        } else if !enable && val == 0 {
            break;
        }
        retries -= 1;
        if retries == 0 {
            break;
        }
    }

    if retries == 0 {
        dev_err((*sdev).dev, b"%s: failed to set Run bit=%d enable=%d\n\0".as_ptr() as *const c_char,
                b"cl_skl_cldma_stream_run\0".as_ptr() as *const c_char, val as c_int, enable as c_int);
    }
}

unsafe fn cl_skl_cldma_stream_clear(sdev: *mut snd_sof_dev) {
    let sd_offset: c_int = SOF_HDA_ADSP_LOADER_BASE;

    /* make sure Run bit is cleared before setting stream register */
    cl_skl_cldma_stream_run(sdev, false);

    /* Disable the Interrupt On Completion, FIFO Error Interrupt,
     * Descriptor Error Interrupt and set the cldma stream number to 0.
     */
    snd_sof_dsp_update_bits(sdev, HDA_DSP_BAR,
                            (sd_offset + SOF_HDA_ADSP_REG_SD_CTL) as c_uint,
                            HDA_CL_SD_CTL_INT_MASK(), HDA_CL_SD_CTL_INT(0));
    snd_sof_dsp_update_bits(sdev, HDA_DSP_BAR,
                            (sd_offset + SOF_HDA_ADSP_REG_SD_CTL) as c_uint,
                            HDA_CL_SD_CTL_STRM(0xf), HDA_CL_SD_CTL_STRM(0));

    snd_sof_dsp_write(sdev, HDA_DSP_BAR,
                      (sd_offset + SOF_HDA_ADSP_REG_SD_BDLPL) as c_uint, HDA_CL_SD_BDLPLBA(0));
    snd_sof_dsp_write(sdev, HDA_DSP_BAR,
                      (sd_offset + SOF_HDA_ADSP_REG_SD_BDLPU) as c_uint, 0);

    /* Set the Cyclic Buffer Length to 0. */
    snd_sof_dsp_write(sdev, HDA_DSP_BAR,
                      (sd_offset + SOF_HDA_ADSP_REG_SD_CBL) as c_uint, 0);
    /* Set the Last Valid Index. */
    snd_sof_dsp_write(sdev, HDA_DSP_BAR,
                      (sd_offset + SOF_HDA_ADSP_REG_SD_LVI) as c_uint, 0);
}

unsafe fn cl_skl_cldma_setup_spb(sdev: *mut snd_sof_dev,
                                 size: c_uint, enable: bool) {
    let sd_offset: c_int = SOF_DSP_REG_CL_SPBFIFO;

    if enable {
        snd_sof_dsp_update_bits(sdev, HDA_DSP_BAR,
                                (sd_offset + SOF_HDA_ADSP_REG_CL_SPBFIFO_SPBFCCTL) as c_uint,
                                HDA_CL_SPBFIFO_SPBFCCTL_SPIBE_MASK,
                                HDA_CL_SPBFIFO_SPBFCCTL_SPIBE(1));
    }

    snd_sof_dsp_write(sdev, HDA_DSP_BAR,
                      (sd_offset + SOF_HDA_ADSP_REG_CL_SPBFIFO_SPIB) as c_uint, size);
}

unsafe fn cl_skl_cldma_set_intr(sdev: *mut snd_sof_dev, enable: bool) {
    let val: u32 = if enable { HDA_DSP_ADSPIC_CL_DMA } else { 0 };

    snd_sof_dsp_update_bits(sdev, HDA_DSP_BAR, HDA_DSP_REG_ADSPIC,
                            HDA_DSP_ADSPIC_CL_DMA, val);
}

unsafe fn cl_skl_cldma_cleanup_spb(sdev: *mut snd_sof_dev) {
    let sd_offset: c_int = SOF_DSP_REG_CL_SPBFIFO;

    snd_sof_dsp_update_bits(sdev, HDA_DSP_BAR,
                            (sd_offset + SOF_HDA_ADSP_REG_CL_SPBFIFO_SPBFCCTL) as c_uint,
                            HDA_CL_SPBFIFO_SPBFCCTL_SPIBE_MASK,
                            HDA_CL_SPBFIFO_SPBFCCTL_SPIBE(0));

    snd_sof_dsp_write(sdev, HDA_DSP_BAR,
                      (sd_offset + SOF_HDA_ADSP_REG_CL_SPBFIFO_SPIB) as c_uint, 0);
}

unsafe fn cl_skl_cldma_setup_controller(sdev: *mut snd_sof_dev,
                                        dmab_bdl: *mut snd_dma_buffer,
                                        max_size: c_uint, count: u32) {
    let sd_offset: c_int = SOF_HDA_ADSP_LOADER_BASE;

    /* Clear the stream first and then set it. */
    cl_skl_cldma_stream_clear(sdev);

    /* setting the stream register */
    snd_sof_dsp_write(sdev, HDA_DSP_BAR,
                      (sd_offset + SOF_HDA_ADSP_REG_SD_BDLPL) as c_uint,
                      HDA_CL_SD_BDLPLBA((*dmab_bdl).addr));
    snd_sof_dsp_write(sdev, HDA_DSP_BAR,
                      (sd_offset + SOF_HDA_ADSP_REG_SD_BDLPU) as c_uint,
                      HDA_CL_SD_BDLPUBA((*dmab_bdl).addr));

    /* Set the Cyclic Buffer Length. */
    snd_sof_dsp_write(sdev, HDA_DSP_BAR,
                      (sd_offset + SOF_HDA_ADSP_REG_SD_CBL) as c_uint, max_size);
    /* Set the Last Valid Index. */
    snd_sof_dsp_write(sdev, HDA_DSP_BAR,
                      (sd_offset + SOF_HDA_ADSP_REG_SD_LVI) as c_uint, count.wrapping_sub(1));

    /* Set the Interrupt On Completion, FIFO Error Interrupt,
     * Descriptor Error Interrupt and the cldma stream number.
     */
    snd_sof_dsp_update_bits(sdev, HDA_DSP_BAR,
                            (sd_offset + SOF_HDA_ADSP_REG_SD_CTL) as c_uint,
                            HDA_CL_SD_CTL_INT_MASK(), HDA_CL_SD_CTL_INT(1));
    snd_sof_dsp_update_bits(sdev, HDA_DSP_BAR,
                            (sd_offset + SOF_HDA_ADSP_REG_SD_CTL) as c_uint,
                            HDA_CL_SD_CTL_STRM(0xf),
                            HDA_CL_SD_CTL_STRM(1));
}

unsafe fn cl_stream_prepare_skl(sdev: *mut snd_sof_dev,
                                dmab: *mut snd_dma_buffer,
                                dmab_bdl: *mut snd_dma_buffer) -> c_int {
    let bufsize: c_uint = HDA_SKL_CLDMA_MAX_BUFFER_SIZE() as c_uint;
    let mut bdl: *mut Le32;
    let frags: c_int;
    let mut ret: c_int;

    ret = snd_dma_alloc_pages(SNDRV_DMA_TYPE_DEV, (*sdev).dev, bufsize as usize, dmab);
    if ret < 0 {
        dev_err((*sdev).dev, b"%s: failed to alloc fw buffer: %x\n\0".as_ptr() as *const c_char,
                b"cl_stream_prepare_skl\0".as_ptr() as *const c_char, ret);
        return ret;
    }

    ret = snd_dma_alloc_pages(SNDRV_DMA_TYPE_DEV, (*sdev).dev, bufsize as usize, dmab_bdl);
    if ret < 0 {
        dev_err((*sdev).dev, b"%s: failed to alloc blde: %x\n\0".as_ptr() as *const c_char,
                b"cl_stream_prepare_skl\0".as_ptr() as *const c_char, ret);
        snd_dma_free_pages(dmab);
        return ret;
    }

    bdl = (*dmab_bdl).area as *mut Le32;
    frags = cl_skl_cldma_setup_bdle(sdev, dmab, &mut bdl, bufsize as c_int, 1);
    cl_skl_cldma_setup_controller(sdev, dmab_bdl, bufsize, frags as u32);

    ret
}

unsafe fn cl_cleanup_skl(sdev: *mut snd_sof_dev,
                         dmab: *mut snd_dma_buffer,
                         dmab_bdl: *mut snd_dma_buffer) {
    cl_skl_cldma_cleanup_spb(sdev);
    cl_skl_cldma_stream_clear(sdev);
    snd_dma_free_pages(dmab);
    snd_dma_free_pages(dmab_bdl);
}

unsafe fn cl_dsp_init_skl(sdev: *mut snd_sof_dev,
                          dmab: *mut snd_dma_buffer,
                          dmab_bdl: *mut snd_dma_buffer) -> c_int {
    let hda: *mut sof_intel_hda_dev = (*(*sdev).pdata).hw_pdata;
    let chip: *const sof_intel_dsp_desc = (*hda).desc;
    let mut status: c_uint = 0;
    let flags: u32;
    let mut ret: c_int;

    /* check if the init_core is already enabled, if yes, reset and make it run,
     * if not, powerdown and enable it again.
     */
    if hda_dsp_core_is_enabled(sdev, (*chip).init_core_mask) {
        /* if enabled, reset it, and run the init_core. */
        ret = hda_dsp_core_stall_reset(sdev, (*chip).init_core_mask);
        if ret < 0 {
            flags = SOF_DBG_DUMP_PCI | SOF_DBG_DUMP_MBOX;
            snd_sof_dsp_dbg_dump(sdev, b"Boot failed\n\0".as_ptr() as *const c_char, flags);
            cl_cleanup_skl(sdev, dmab, dmab_bdl);
            hda_dsp_core_reset_power_down(sdev, (*chip).init_core_mask);
            return ret;
        }

        ret = hda_dsp_core_run(sdev, (*chip).init_core_mask);
        if ret < 0 {
            dev_err((*sdev).dev, b"%s: dsp core start failed %d\n\0".as_ptr() as *const c_char,
                    b"cl_dsp_init_skl\0".as_ptr() as *const c_char, ret);
            flags = SOF_DBG_DUMP_PCI | SOF_DBG_DUMP_MBOX;
            snd_sof_dsp_dbg_dump(sdev, b"Boot failed\n\0".as_ptr() as *const c_char, flags);
            cl_cleanup_skl(sdev, dmab, dmab_bdl);
            hda_dsp_core_reset_power_down(sdev, (*chip).init_core_mask);
            return ret;
        }
    } else {
        /* if not enabled, power down it first and then powerup and run
         * the init_core.
         */
        ret = hda_dsp_core_reset_power_down(sdev, (*chip).init_core_mask);
        if ret < 0 {
            dev_err((*sdev).dev, b"%s: dsp core0 disable fail: %d\n\0".as_ptr() as *const c_char,
                    b"cl_dsp_init_skl\0".as_ptr() as *const c_char, ret);
            flags = SOF_DBG_DUMP_PCI | SOF_DBG_DUMP_MBOX;
            snd_sof_dsp_dbg_dump(sdev, b"Boot failed\n\0".as_ptr() as *const c_char, flags);
            cl_cleanup_skl(sdev, dmab, dmab_bdl);
            hda_dsp_core_reset_power_down(sdev, (*chip).init_core_mask);
            return ret;
        }
        ret = hda_dsp_enable_core(sdev, (*chip).init_core_mask);
        if ret < 0 {
            dev_err((*sdev).dev, b"%s: dsp core0 enable fail: %d\n\0".as_ptr() as *const c_char,
                    b"cl_dsp_init_skl\0".as_ptr() as *const c_char, ret);
            flags = SOF_DBG_DUMP_PCI | SOF_DBG_DUMP_MBOX;
            snd_sof_dsp_dbg_dump(sdev, b"Boot failed\n\0".as_ptr() as *const c_char, flags);
            cl_cleanup_skl(sdev, dmab, dmab_bdl);
            hda_dsp_core_reset_power_down(sdev, (*chip).init_core_mask);
            return ret;
        }
    }

    /* prepare DMA for code loader stream */
    ret = cl_stream_prepare_skl(sdev, dmab, dmab_bdl);
    if ret < 0 {
        dev_err((*sdev).dev, b"%s: dma prepare fw loading err: %x\n\0".as_ptr() as *const c_char,
                b"cl_dsp_init_skl\0".as_ptr() as *const c_char, ret);
        return ret;
    }

    /* enable the interrupt */
    snd_sof_dsp_update_bits(sdev, HDA_DSP_BAR, HDA_DSP_REG_ADSPIC,
                            HDA_DSP_ADSPIC_IPC, HDA_DSP_ADSPIC_IPC);

    /* enable IPC DONE interrupt */
    snd_sof_dsp_update_bits(sdev, HDA_DSP_BAR, (*chip).ipc_ctl,
                            HDA_DSP_REG_HIPCCTL_DONE,
                            HDA_DSP_REG_HIPCCTL_DONE);

    /* enable IPC BUSY interrupt */
    snd_sof_dsp_update_bits(sdev, HDA_DSP_BAR, (*chip).ipc_ctl,
                            HDA_DSP_REG_HIPCCTL_BUSY,
                            HDA_DSP_REG_HIPCCTL_BUSY);

    /* polling the ROM init status information. */
    ret = snd_sof_dsp_read_poll_timeout(sdev, HDA_DSP_BAR,
                                        (*chip).rom_status_reg, &mut status,
                                        (FSR_TO_STATE_CODE(status) == FSR_STATE_INIT_DONE) as u32,
                                        HDA_DSP_REG_POLL_INTERVAL_US,
                                        (*chip).rom_init_timeout.wrapping_mul(USEC_PER_MSEC));
    if ret < 0 {
        flags = SOF_DBG_DUMP_PCI | SOF_DBG_DUMP_MBOX;
        snd_sof_dsp_dbg_dump(sdev, b"Boot failed\n\0".as_ptr() as *const c_char, flags);
        cl_cleanup_skl(sdev, dmab, dmab_bdl);
        hda_dsp_core_reset_power_down(sdev, (*chip).init_core_mask);
        return ret;
    }

    ret
}

unsafe fn cl_skl_cldma_fill_buffer(sdev: *mut snd_sof_dev,
                                   dmab: *mut snd_dma_buffer,
                                   bufsize: c_uint,
                                   copysize: c_uint,
                                   curr_pos: *const c_void,
                                   intr_enable: bool) {
    let hda: *mut sof_intel_hda_dev = (*(*sdev).pdata).hw_pdata;

    /* copy the image into the buffer with the maximum buffer size. */
    let size: c_uint = if bufsize == copysize { bufsize } else { copysize };

    memcpy((*dmab).area, curr_pos, size as usize);

    /* Set the wait condition for every load. */
    (*hda).code_loading = 1;

    /* Set the interrupt. */
    if intr_enable {
        cl_skl_cldma_set_intr(sdev, true);
    }

    /* Set the SPB. */
    cl_skl_cldma_setup_spb(sdev, size, true);

    /* Trigger the code loading stream. */
    cl_skl_cldma_stream_run(sdev, true);
}

unsafe fn cl_skl_cldma_wait_interruptible(sdev: *mut snd_sof_dev,
                                          intr_wait: bool) -> c_int {
    let hda: *mut sof_intel_hda_dev = (*(*sdev).pdata).hw_pdata;
    let chip: *const sof_intel_dsp_desc = (*hda).desc;
    let sd_offset: c_int = SOF_HDA_ADSP_LOADER_BASE;
    let cl_dma_intr_status: u8;

    /*
     * Wait for CLDMA interrupt to inform the binary segment transfer is
     * complete.
     */
    if wait_event_timeout(&mut (*hda).waitq as *mut c_void, (*hda).code_loading == 0,
                          msecs_to_jiffies(HDA_SKL_WAIT_TIMEOUT)) == 0 {
        dev_err((*sdev).dev, b"cldma copy timeout\n\0".as_ptr() as *const c_char);
        dev_err((*sdev).dev, b"ROM code=%#x: FW status=%#x\n\0".as_ptr() as *const c_char,
                snd_sof_dsp_read(sdev, HDA_DSP_BAR, HDA_DSP_SRAM_REG_ROM_ERROR),
                snd_sof_dsp_read(sdev, HDA_DSP_BAR, (*chip).rom_status_reg));
        return -EIO;
    }

    /* now check DMA interrupt status */
    cl_dma_intr_status = snd_sof_dsp_read(sdev, HDA_DSP_BAR,
                                          (sd_offset + SOF_HDA_ADSP_REG_SD_STS) as c_uint) as u8;

    if (cl_dma_intr_status & HDA_CL_DMA_SD_INT_COMPLETE) == 0 {
        dev_err((*sdev).dev, b"cldma copy failed\n\0".as_ptr() as *const c_char);
        return -EIO;
    }

    dev_dbg((*sdev).dev, b"cldma buffer copy complete\n\0".as_ptr() as *const c_char);
    0
}

unsafe fn cl_skl_cldma_copy_to_buf(sdev: *mut snd_sof_dev,
                                   dmab: *mut snd_dma_buffer,
                                   bin: *const c_void,
                                   total_size: u32, bufsize: u32) -> c_int {
    let mut bytes_left: c_uint = total_size;
    let mut curr_pos: *const c_void = bin;
    let mut ret: c_int;

    if total_size <= 0 {
        return -EINVAL;
    }

    while bytes_left > 0 {
        if bytes_left > bufsize {
            dev_dbg((*sdev).dev, b"cldma copy %#x bytes\n\0".as_ptr() as *const c_char, bufsize);

            cl_skl_cldma_fill_buffer(sdev, dmab, bufsize, bufsize, curr_pos, true);

            ret = cl_skl_cldma_wait_interruptible(sdev, false);
            if ret < 0 {
                dev_err((*sdev).dev,
                        b"%s: fw failed to load. %#x bytes remaining\n\0".as_ptr() as *const c_char,
                        b"cl_skl_cldma_copy_to_buf\0".as_ptr() as *const c_char, bytes_left);
                return ret;
            }

            bytes_left = bytes_left.wrapping_sub(bufsize);
            curr_pos = (curr_pos as *const u8).add(bufsize as usize) as *const c_void;
        } else {
            dev_dbg((*sdev).dev, b"cldma copy %#x bytes\n\0".as_ptr() as *const c_char, bytes_left);

            cl_skl_cldma_set_intr(sdev, false);
            cl_skl_cldma_fill_buffer(sdev, dmab, bufsize, bytes_left, curr_pos, false);
            return 0;
        }
    }

    bytes_left as c_int
}

unsafe fn cl_copy_fw_skl(sdev: *mut snd_sof_dev,
                         dmab: *mut snd_dma_buffer) -> c_int {
    let fw: *const firmware = (*sdev).basefw.fw;
    let mut stripped_firmware: firmware = firmware {
        size: 0,
        data: core::ptr::null(),
    };
    let bufsize: c_uint = HDA_SKL_CLDMA_MAX_BUFFER_SIZE() as c_uint;
    let mut ret: c_int;

    stripped_firmware.data = (*fw).data.add((*sdev).basefw.payload_offset);
    stripped_firmware.size = (*fw).size.wrapping_sub((*sdev).basefw.payload_offset);

    dev_dbg((*sdev).dev, b"firmware size: %#zx buffer size %#x\n\0".as_ptr() as *const c_char,
            (*fw).size, bufsize);

    ret = cl_skl_cldma_copy_to_buf(sdev, dmab, stripped_firmware.data as *const c_void,
                                   stripped_firmware.size as u32, bufsize);
    if ret < 0 {
        dev_err((*sdev).dev, b"%s: fw copy failed %d\n\0".as_ptr() as *const c_char,
                b"cl_copy_fw_skl\0".as_ptr() as *const c_char, ret);
    }

    ret
}

pub unsafe fn hda_dsp_cl_boot_firmware_skl(sdev: *mut snd_sof_dev) -> c_int {
    let hda: *mut sof_intel_hda_dev = (*(*sdev).pdata).hw_pdata;
    let chip: *const sof_intel_dsp_desc = (*hda).desc;
    let mut dmab_bdl: snd_dma_buffer = core::mem::zeroed();
    let mut dmab: snd_dma_buffer = core::mem::zeroed();
    let mut reg: c_uint = 0;
    let flags: u32;
    let mut ret: c_int;

    ret = cl_dsp_init_skl(sdev, &mut dmab, &mut dmab_bdl);

    /* retry enabling core and ROM load. seemed to help */
    if ret < 0 {
        ret = cl_dsp_init_skl(sdev, &mut dmab, &mut dmab_bdl);
        if ret < 0 {
            dev_err((*sdev).dev, b"Error code=%#x: FW status=%#x\n\0".as_ptr() as *const c_char,
                    snd_sof_dsp_read(sdev, HDA_DSP_BAR, HDA_DSP_SRAM_REG_ROM_ERROR),
                    snd_sof_dsp_read(sdev, HDA_DSP_BAR, (*chip).rom_status_reg));
            dev_err((*sdev).dev, b"Core En/ROM load fail:%d\n\0".as_ptr() as *const c_char, ret);
            return ret;
        }
    }

    dev_dbg((*sdev).dev, b"ROM init successful\n\0".as_ptr() as *const c_char);

    /* at this point DSP ROM has been initialized and should be ready for
     * code loading and firmware boot
     */
    ret = cl_copy_fw_skl(sdev, &mut dmab);
    if ret < 0 {
        dev_err((*sdev).dev, b"%s: load firmware failed : %d\n\0".as_ptr() as *const c_char,
                b"hda_dsp_cl_boot_firmware_skl\0".as_ptr() as *const c_char, ret);
        flags = SOF_DBG_DUMP_PCI | SOF_DBG_DUMP_MBOX;

        snd_sof_dsp_dbg_dump(sdev, b"Boot failed\n\0".as_ptr() as *const c_char, flags);

        /* power down DSP */
        hda_dsp_core_reset_power_down(sdev, (*chip).init_core_mask);
        cl_skl_cldma_stream_run(sdev, false);
        cl_cleanup_skl(sdev, &mut dmab, &mut dmab_bdl);

        dev_err((*sdev).dev, b"%s: load fw failed err: %d\n\0".as_ptr() as *const c_char,
                b"hda_dsp_cl_boot_firmware_skl\0".as_ptr() as *const c_char, ret);
        return ret;
    }

    ret = snd_sof_dsp_read_poll_timeout(sdev, HDA_DSP_BAR,
                                        (*chip).rom_status_reg, &mut reg,
                                        (FSR_TO_STATE_CODE(reg) == FSR_STATE_ROM_BASEFW_ENTERED) as u32,
                                        HDA_DSP_REG_POLL_INTERVAL_US,
                                        HDA_DSP_BASEFW_TIMEOUT_US);

    dev_dbg((*sdev).dev, b"Firmware download successful, booting...\n\0".as_ptr() as *const c_char);

    cl_skl_cldma_stream_run(sdev, false);
    cl_cleanup_skl(sdev, &mut dmab, &mut dmab_bdl);

    if ret == 0 {
        return (*chip).init_core_mask as c_int;
    }

    ret
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
