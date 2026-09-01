// SPDX-License-Identifier: GPL-2.0-only
//
// Copyright(c) 2021-2022 Intel Corporation
//
// Author: Cezary Rojewski <cezary.rojewski@intel.com>
//

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]

use core::ffi::c_void;

type u8 = ::core::primitive::u8;
type u32 = ::core::primitive::u32;
type c_int = ::core::ffi::c_int;
type c_uint = ::core::ffi::c_uint;
type c_ulong = ::core::ffi::c_ulong;
type dma_addr_t = usize;
type phys_addr_t = usize;
type __le32 = u32;

// Dependencies originally provided by:
// <linux/pci.h>, <sound/hda_register.h>, <sound/hdaudio_ext.h>,
// "cldma.h", and "registers.h".
#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct hdac_bus {
    pub dev: *mut device,
}

#[repr(C)]
pub struct work_struct {
    _private: [u8; 0],
}

#[repr(C)]
pub struct delayed_work {
    pub work: work_struct,
}

#[repr(C)]
pub struct completion {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_dma_buffer {
    pub area: *mut u8,
    pub addr: dma_addr_t,
}

/* Stream Registers */
const AZX_CL_SD_BASE: c_uint = 0x80;
const AZX_SD_CTL_STRM_MASK: c_uint = GENMASK(23, 20);
const AZX_SD_BDLPL_BDLPLBA_MASK: c_uint = GENMASK(31, 7);

#[inline]
unsafe fn AZX_SD_CTL_STRM(s: *mut hda_cldma) -> c_uint {
    (((*s).stream_tag as c_uint) << 20) & AZX_SD_CTL_STRM_MASK
}

#[inline]
fn AZX_SD_BDLPL_BDLPLBA(lb: u32) -> u32 {
    lb & AZX_SD_BDLPL_BDLPLBA_MASK
}

/* Software Position Based FIFO Capability Registers */
const AZX_CL_SPBFCS: c_uint = 0x20;
const AZX_REG_CL_SPBFCTL: c_uint = AZX_CL_SPBFCS + 0x4;
const AZX_REG_CL_SD_SPIB: c_uint = AZX_CL_SPBFCS + 0x8;

const AVS_CL_OP_INTERVAL_US: c_uint = 3;
const AVS_CL_OP_TIMEOUT_US: c_uint = 300;
const AVS_CL_IOC_TIMEOUT_MS: c_uint = 300;
const AVS_CL_STREAM_INDEX: c_uint = 0;

#[repr(C)]
pub struct hda_cldma {
    pub dev: *mut device,
    pub bus: *mut hdac_bus,
    pub dsp_ba: *mut c_void,

    pub buffer_size: c_uint,
    pub num_periods: c_uint,
    pub stream_tag: u8,
    pub sd_addr: *mut c_void,

    pub dmab_data: snd_dma_buffer,
    pub dmab_bdl: snd_dma_buffer,
    pub memcpy_work: delayed_work,
    pub completion: completion,

    /* runtime */
    pub position: *mut c_void,
    pub remaining: c_uint,
    pub sd_status: c_uint,
}

extern "C" {
    fn memcpy(dest: *mut c_void, src: *const c_void, n: usize) -> *mut c_void;

    fn hda_cldma_start(cl: *mut hda_cldma) -> c_int;

    fn snd_hdac_stream_readl(cl: *mut hda_cldma, reg: c_uint) -> c_uint;
    fn snd_hdac_stream_writel(cl: *mut hda_cldma, reg: c_uint, value: c_uint);
    fn snd_hdac_stream_writeb(cl: *mut hda_cldma, reg: c_uint, value: c_uint);
    fn snd_hdac_stream_readb(cl: *mut hda_cldma, reg: c_uint) -> c_uint;
    fn snd_hdac_stream_updateb(cl: *mut hda_cldma, reg: c_uint, mask: c_uint, value: c_uint);
    fn snd_hdac_stream_updatel(cl: *mut hda_cldma, reg: c_uint, mask: c_uint, value: c_uint);
    fn snd_hdac_adsp_updatel(cl: *mut hda_cldma, reg: c_uint, mask: c_uint, value: c_uint);

    fn wait_for_completion_timeout(x: *mut completion, timeout: c_ulong) -> c_ulong;
    fn msecs_to_jiffies(m: c_uint) -> c_ulong;
    fn reinit_completion(x: *mut completion);
    fn complete(x: *mut completion);
    fn schedule_delayed_work(work: *mut delayed_work, delay: c_ulong) -> bool;
    fn cancel_delayed_work_sync(work: *mut delayed_work) -> bool;

    fn snd_sgbuf_get_addr(dmab: *mut snd_dma_buffer, offset: c_int) -> phys_addr_t;
    fn snd_sgbuf_get_chunk_size(dmab: *mut snd_dma_buffer, offset: c_int, size: u32) -> c_int;
    fn snd_dma_alloc_pages(
        type_: c_int,
        device: *mut device,
        size: c_uint,
        dmab: *mut snd_dma_buffer,
    ) -> c_int;
    fn snd_dma_free_pages(dmab: *mut snd_dma_buffer);

    fn dev_err(dev: *mut device, fmt: *const u8, ...);
    fn dev_dbg(dev: *mut device, fmt: *const u8, ...);
}

extern "C" {
    static SD_CTL: c_uint;
    static SD_STS: c_uint;
    static SD_BDLPL: c_uint;
    static SD_BDLPU: c_uint;
    static SD_CBL: c_uint;
    static SD_LVI: c_uint;
    static CL_SD_SPIB: c_uint;
    static CL_SPBFCTL: c_uint;
    static SD_INT_MASK: c_uint;
    static SD_CTL_DMA_START: c_uint;
    static SD_CTL_STREAM_RESET: c_uint;
    static SD_INT_COMPLETE: c_uint;
    static AVS_ADSP_REG_ADSPIC: c_uint;
    static AVS_ADSP_ADSPIC_CLDMA: c_uint;
    static SNDRV_DMA_TYPE_DEV_SG: c_int;
    static SNDRV_DMA_TYPE_DEV: c_int;
    static BDL_SIZE: c_uint;
}

const fn GENMASK(h: c_uint, l: c_uint) -> c_uint {
    ((!0u32) << l) & ((!0u32) >> (31 - h))
}

#[inline]
fn lower_32_bits(n: usize) -> u32 {
    n as u32
}

#[inline]
fn upper_32_bits(n: usize) -> u32 {
    (n >> 32) as u32
}

#[inline]
fn cpu_to_le32(x: u32) -> __le32 {
    x.to_le()
}

unsafe fn container_of_memcpy_work(work: *mut work_struct) -> *mut hda_cldma {
    (work as *mut u8).sub(core::mem::offset_of!(hda_cldma, memcpy_work)) as *mut hda_cldma
}

unsafe extern "C" fn cldma_memcpy_work(work: *mut work_struct) {
    let cl: *mut hda_cldma = container_of_memcpy_work(work);
    let mut ret: c_int;

    ret = hda_cldma_start(cl);
    if ret < 0 {
        dev_err((*cl).dev, c"cldma set RUN failed: %d\n".as_ptr() as *const u8, ret);
        return;
    }

    loop {
        ret = wait_for_completion_timeout(
            &mut (*cl).completion,
            msecs_to_jiffies(AVS_CL_IOC_TIMEOUT_MS),
        ) as c_int;
        if ret == 0 {
            dev_err((*cl).dev, c"cldma IOC timeout\n".as_ptr() as *const u8);
            break;
        }

        if ((*cl).sd_status & SD_INT_COMPLETE) == 0 {
            dev_err(
                (*cl).dev,
                c"cldma transfer error, SD status: 0x%08x\n".as_ptr() as *const u8,
                (*cl).sd_status,
            );
            break;
        }

        if (*cl).remaining == 0 {
            break;
        }

        reinit_completion(&mut (*cl).completion);
        hda_cldma_fill(cl);
        /* enable CLDMA interrupt */
        snd_hdac_adsp_updatel(
            cl,
            AVS_ADSP_REG_ADSPIC,
            AVS_ADSP_ADSPIC_CLDMA,
            AVS_ADSP_ADSPIC_CLDMA,
        );
    }
}

// Static initialization in C used __DELAYED_WORK_INITIALIZER and
// COMPLETION_INITIALIZER, which are kernel macro initializers supplied by
// external headers.
extern "C" {
    pub static mut code_loader: hda_cldma;
}

pub unsafe extern "C" fn hda_cldma_fill(cl: *mut hda_cldma) {
    let mut size: c_uint;
    let mut offset: c_uint;

    if (*cl).remaining > (*cl).buffer_size {
        size = (*cl).buffer_size;
    } else {
        size = (*cl).remaining;
    }

    offset = snd_hdac_stream_readl(cl, CL_SD_SPIB);
    if offset + size > (*cl).buffer_size {
        let ss: c_uint;

        ss = (*cl).buffer_size - offset;
        memcpy(
            (*cl).dmab_data.area.add(offset as usize) as *mut c_void,
            (*cl).position,
            ss as usize,
        );
        offset = 0;
        size -= ss;
        (*cl).position = ((*cl).position as *mut u8).add(ss as usize) as *mut c_void;
        (*cl).remaining -= ss;
    }

    memcpy(
        (*cl).dmab_data.area.add(offset as usize) as *mut c_void,
        (*cl).position,
        size as usize,
    );
    (*cl).position = ((*cl).position as *mut u8).add(size as usize) as *mut c_void;
    (*cl).remaining -= size;

    snd_hdac_stream_writel(cl, CL_SD_SPIB, offset + size);
}

pub unsafe extern "C" fn hda_cldma_transfer(cl: *mut hda_cldma, start_delay: c_ulong) {
    if (*cl).remaining == 0 {
        return;
    }

    reinit_completion(&mut (*cl).completion);
    /* fill buffer with the first chunk before scheduling run */
    hda_cldma_fill(cl);

    schedule_delayed_work(&mut (*cl).memcpy_work, start_delay);
}

pub unsafe extern "C" fn hda_cldma_start(cl: *mut hda_cldma) -> c_int {
    let mut reg: c_uint = 0;

    /* enable interrupts */
    snd_hdac_adsp_updatel(
        cl,
        AVS_ADSP_REG_ADSPIC,
        AVS_ADSP_ADSPIC_CLDMA,
        AVS_ADSP_ADSPIC_CLDMA,
    );
    snd_hdac_stream_updateb(
        cl,
        SD_CTL,
        SD_INT_MASK | SD_CTL_DMA_START,
        SD_INT_MASK | SD_CTL_DMA_START,
    );

    /* await DMA engine start */
    snd_hdac_stream_readb_poll(
        cl,
        SD_CTL,
        &mut reg,
        (reg & SD_CTL_DMA_START) != 0,
        AVS_CL_OP_INTERVAL_US,
        AVS_CL_OP_TIMEOUT_US,
    )
}

pub unsafe extern "C" fn hda_cldma_stop(cl: *mut hda_cldma) -> c_int {
    let mut reg: c_uint = 0;
    let ret: c_int;

    /* disable interrupts */
    snd_hdac_adsp_updatel(cl, AVS_ADSP_REG_ADSPIC, AVS_ADSP_ADSPIC_CLDMA, 0);
    snd_hdac_stream_updateb(cl, SD_CTL, SD_INT_MASK | SD_CTL_DMA_START, 0);

    /* await DMA engine stop */
    ret = snd_hdac_stream_readb_poll(
        cl,
        SD_CTL,
        &mut reg,
        (reg & SD_CTL_DMA_START) == 0,
        AVS_CL_OP_INTERVAL_US,
        AVS_CL_OP_TIMEOUT_US,
    );
    cancel_delayed_work_sync(&mut (*cl).memcpy_work);

    ret
}

pub unsafe extern "C" fn hda_cldma_reset(cl: *mut hda_cldma) -> c_int {
    let mut reg: c_uint = 0;
    let mut ret: c_int;

    ret = hda_cldma_stop(cl);
    if ret < 0 {
        dev_err((*cl).dev, c"cldma stop failed: %d\n".as_ptr() as *const u8, ret);
        return ret;
    }

    snd_hdac_stream_updateb(cl, SD_CTL, SD_CTL_STREAM_RESET, SD_CTL_STREAM_RESET);
    ret = snd_hdac_stream_readb_poll(
        cl,
        SD_CTL,
        &mut reg,
        (reg & SD_CTL_STREAM_RESET) != 0,
        AVS_CL_OP_INTERVAL_US,
        AVS_CL_OP_TIMEOUT_US,
    );
    if ret < 0 {
        dev_err((*cl).dev, c"cldma set SRST failed: %d\n".as_ptr() as *const u8, ret);
        return ret;
    }

    snd_hdac_stream_updateb(cl, SD_CTL, SD_CTL_STREAM_RESET, 0);
    ret = snd_hdac_stream_readb_poll(
        cl,
        SD_CTL,
        &mut reg,
        (reg & SD_CTL_STREAM_RESET) == 0,
        AVS_CL_OP_INTERVAL_US,
        AVS_CL_OP_TIMEOUT_US,
    );
    if ret < 0 {
        dev_err((*cl).dev, c"cldma unset SRST failed: %d\n".as_ptr() as *const u8, ret);
        return ret;
    }

    0
}

pub unsafe extern "C" fn hda_cldma_set_data(cl: *mut hda_cldma, data: *mut c_void, size: c_uint) {
    /* setup runtime */
    (*cl).position = data;
    (*cl).remaining = size;
}

unsafe fn cldma_setup_bdle(cl: *mut hda_cldma, bdle_size: u32) {
    let dmab: *mut snd_dma_buffer = &mut (*cl).dmab_data;
    let mut bdl: *mut __le32 = (*cl).dmab_bdl.area as *mut __le32;
    let mut remaining: c_int = (*cl).buffer_size as c_int;
    let mut offset: c_int = 0;

    (*cl).num_periods = 0;

    while remaining > 0 {
        let addr: phys_addr_t;
        let chunk: c_int;

        addr = snd_sgbuf_get_addr(dmab, offset);
        *bdl.add(0) = cpu_to_le32(lower_32_bits(addr));
        *bdl.add(1) = cpu_to_le32(upper_32_bits(addr));
        chunk = snd_sgbuf_get_chunk_size(dmab, offset, bdle_size);
        *bdl.add(2) = cpu_to_le32(chunk as u32);

        remaining -= chunk;
        /* set IOC only for the last entry */
        *bdl.add(3) = if remaining > 0 { 0 } else { cpu_to_le32(0x01) };

        bdl = bdl.add(4);
        offset += chunk;
        (*cl).num_periods += 1;
    }
}

pub unsafe extern "C" fn hda_cldma_setup(cl: *mut hda_cldma) {
    let bdl_addr: dma_addr_t = (*cl).dmab_bdl.addr;

    cldma_setup_bdle(cl, (*cl).buffer_size / 2);

    snd_hdac_stream_writel(
        cl,
        SD_BDLPL,
        AZX_SD_BDLPL_BDLPLBA(lower_32_bits(bdl_addr)),
    );
    snd_hdac_stream_writel(cl, SD_BDLPU, upper_32_bits(bdl_addr));

    snd_hdac_stream_writel(cl, SD_CBL, (*cl).buffer_size);
    snd_hdac_stream_writeb(cl, SD_LVI, (*cl).num_periods - 1);

    snd_hdac_stream_updatel(cl, SD_CTL, AZX_SD_CTL_STRM_MASK, AZX_SD_CTL_STRM(cl));
    /* enable spib */
    snd_hdac_stream_writel(cl, CL_SPBFCTL, 1);
}

pub unsafe extern "C" fn hda_cldma_interrupt(cl: *mut hda_cldma) {
    /* disable CLDMA interrupt */
    snd_hdac_adsp_updatel(cl, AVS_ADSP_REG_ADSPIC, AVS_ADSP_ADSPIC_CLDMA, 0);

    (*cl).sd_status = snd_hdac_stream_readb(cl, SD_STS);
    dev_dbg(
        (*cl).dev,
        c"%s sd_status: 0x%08x\n".as_ptr() as *const u8,
        c"hda_cldma_interrupt".as_ptr(),
        (*cl).sd_status,
    );

    complete(&mut (*cl).completion);
}

pub unsafe extern "C" fn hda_cldma_init(
    cl: *mut hda_cldma,
    bus: *mut hdac_bus,
    dsp_ba: *mut c_void,
    buffer_size: c_uint,
) -> c_int {
    let mut ret: c_int;

    ret = snd_dma_alloc_pages(SNDRV_DMA_TYPE_DEV_SG, (*bus).dev, buffer_size, &mut (*cl).dmab_data);
    if ret < 0 {
        return ret;
    }

    ret = snd_dma_alloc_pages(SNDRV_DMA_TYPE_DEV, (*bus).dev, BDL_SIZE, &mut (*cl).dmab_bdl);
    if ret < 0 {
        snd_dma_free_pages(&mut (*cl).dmab_data);
        return ret;
    }

    (*cl).dev = (*bus).dev;
    (*cl).bus = bus;
    (*cl).dsp_ba = dsp_ba;
    (*cl).buffer_size = buffer_size;
    (*cl).sd_addr = (dsp_ba as *mut u8).add(AZX_CL_SD_BASE as usize) as *mut c_void;

    0
}

pub unsafe extern "C" fn hda_cldma_free(cl: *mut hda_cldma) {
    snd_dma_free_pages(&mut (*cl).dmab_data);
    snd_dma_free_pages(&mut (*cl).dmab_bdl);
}

unsafe fn snd_hdac_stream_readb_poll(
    cl: *mut hda_cldma,
    reg: c_uint,
    val: *mut c_uint,
    cond: bool,
    sleep_us: c_uint,
    timeout_us: c_uint,
) -> c_int {
    // Macro dependency from the original C environment. This declaration-shaped
    // body preserves the call site and parameters without supplying the kernel
    // polling implementation in this isolated translation.
    let _ = (cl, reg, val, cond, sleep_us, timeout_us);
    unimplemented!()
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
