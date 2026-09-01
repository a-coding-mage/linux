// SPDX-License-Identifier: GPL-2.0+
//
// siu_pcm.c - ALSA driver for Renesas SH7343, SH7722 SIU peripheral.
//
// Copyright (C) 2009-2010 Guennadi Liakhovetski <g.liakhovetski@gmx.de>
// Copyright (C) 2006 Carlos Munoz <carlos@kenati.com>

// C dependencies:
// linux/delay.h, linux/dma-mapping.h, linux/dmaengine.h, linux/interrupt.h,
// linux/module.h, linux/platform_device.h, sound/control.h, sound/core.h,
// sound/pcm.h, sound/pcm_params.h, sound/soc.h, asm/siu.h, "siu.h".

const DRV_NAME: *const core::ffi::c_char = b"siu-i2s\0".as_ptr() as *const core::ffi::c_char;

#[inline]
const fn GET_MAX_PERIODS(buf_bytes: u32, period_bytes: u32) -> u32 {
    buf_bytes / period_bytes
}

#[inline]
const fn PERIOD_OFFSET(buf_addr: dma_addr_t, period_num: u32, period_bytes: u32) -> dma_addr_t {
    buf_addr.wrapping_add((period_num as dma_addr_t).wrapping_mul(period_bytes as dma_addr_t))
}

const RWF_STM_RD: u32 = 0x01; /* Read in progress */
const RWF_STM_WT: u32 = 0x02; /* Write in progress */

type c_int = core::ffi::c_int;
type c_uint = core::ffi::c_uint;
type c_ulong = core::ffi::c_ulong;
type c_void = core::ffi::c_void;
type bool_t = bool;
type u32 = core::ffi::c_uint;
type size_t = usize;
type dma_addr_t = usize;
type dma_cookie_t = c_int;
type snd_pcm_sframes_t = isize;
type snd_pcm_uframes_t = usize;

#[repr(C)]
pub struct siu_port {
    pub playback: siu_stream,
    pub capture: siu_stream,
    pub stfifo: u32,
    pub pcm: *mut snd_pcm,
}

#[repr(C)]
pub struct siu_stream {
    pub rw_flg: u32,
    pub cur_period: u32,
    pub buf_bytes: u32,
    pub period_bytes: u32,
    pub cookie: dma_cookie_t,
    pub substream: *mut snd_pcm_substream,
    pub chan: *mut dma_chan,
    pub tx_desc: *mut dma_async_tx_descriptor,
    pub work: work_struct,
    pub param: sh_dmae_slave,
    pub format: c_int,
    pub xfer_cnt: snd_pcm_sframes_t,
}

#[repr(C)]
pub struct siu_info {
    pub reg: *mut u32,
    pub port_id: u32,
}

#[repr(C)]
pub struct siu_platform {
    pub dma_slave_tx_b: c_int,
    pub dma_slave_tx_a: c_int,
    pub dma_slave_rx_b: c_int,
    pub dma_slave_rx_a: c_int,
}

#[repr(C)]
pub struct sh_dmae_slave {
    pub shdma_slave: shdma_slave,
}

#[repr(C)]
pub struct shdma_slave {
    pub slave_id: c_int,
}

#[repr(C)]
pub struct dma_chan {
    pub private: *mut c_void,
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
pub struct work_struct {
    _private: [u8; 0],
}

#[repr(C)]
pub struct workqueue_struct {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_component {
    pub dev: *mut device,
}

#[repr(C)]
pub struct snd_pcm_substream {
    pub pcm: *mut snd_pcm,
    pub runtime: *mut snd_pcm_runtime,
    pub stream: c_int,
}

#[repr(C)]
pub struct snd_pcm {
    pub card: *mut snd_card,
}

#[repr(C)]
pub struct snd_card {
    pub dev: *mut device,
}

#[repr(C)]
pub struct device {
    pub platform_data: *mut c_void,
}

#[repr(C)]
pub struct snd_pcm_runtime {
    pub dma_addr: dma_addr_t,
    pub channels: c_uint,
    pub format: c_int,
}

#[repr(C)]
pub struct snd_soc_pcm_runtime {
    pub card: *mut snd_soc_card,
    pub pcm: *mut snd_pcm,
}

#[repr(C)]
pub struct snd_soc_card {
    pub snd_card: *mut snd_card,
}

#[repr(C)]
pub struct platform_device {
    pub id: c_int,
}

#[repr(C)]
pub struct dma_cap_mask_t {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_component_driver {
    pub name: *const core::ffi::c_char,
    pub open: Option<unsafe extern "C" fn(*mut snd_soc_component, *mut snd_pcm_substream) -> c_int>,
    pub close: Option<unsafe extern "C" fn(*mut snd_soc_component, *mut snd_pcm_substream) -> c_int>,
    pub prepare: Option<unsafe extern "C" fn(*mut snd_soc_component, *mut snd_pcm_substream) -> c_int>,
    pub trigger: Option<unsafe extern "C" fn(*mut snd_soc_component, *mut snd_pcm_substream, c_int) -> c_int>,
    pub pointer: Option<unsafe extern "C" fn(*mut snd_soc_component, *mut snd_pcm_substream) -> snd_pcm_uframes_t>,
    pub pcm_new: Option<unsafe extern "C" fn(*mut snd_soc_component, *mut snd_soc_pcm_runtime) -> c_int>,
    pub pcm_free: Option<unsafe extern "C" fn(*mut snd_soc_component, *mut snd_pcm)>,
    pub legacy_dai_naming: c_int,
}

const EPERM: c_int = 1;
const ENOMEM: c_int = 12;
const EBUSY: c_int = 16;
const EINVAL: c_int = 22;
const SIU_STFIFO: isize = 0;
const SIU_EVNTC: isize = 0;
const SIU_SBFSTS: isize = 0;
const SIU_PORT_NUM: usize = 2;
const SNDRV_PCM_STREAM_PLAYBACK: c_int = 0;
const SNDRV_PCM_STREAM_CAPTURE: c_int = 1;
const SNDRV_PCM_TRIGGER_START: c_int = 0;
const SNDRV_PCM_TRIGGER_STOP: c_int = 1;
const SNDRV_DMA_TYPE_DEV: c_int = 0;
const SIU_BUFFER_BYTES_MAX: size_t = 0;
const DMA_SLAVE: c_int = 0;
const DMA_MEM_TO_DEV: c_int = 0;
const DMA_DEV_TO_MEM: c_int = 1;
const DMA_PREP_INTERRUPT: c_int = 1;
const DMA_CTRL_ACK: c_int = 2;

static mut siu_ports: [*mut siu_port; SIU_PORT_NUM] = [core::ptr::null_mut(); SIU_PORT_NUM];

unsafe extern "C" {
    static mut siu_i2s_data: *mut siu_info;
    static mut system_highpri_wq: *mut workqueue_struct;

    fn siu_read32(addr: *mut u32) -> u32;
    fn siu_write32(addr: *mut u32, val: u32);
    fn queue_work(wq: *mut workqueue_struct, work: *mut work_struct) -> bool_t;
    fn snd_pcm_period_elapsed(substream: *mut snd_pcm_substream);
    fn sg_init_table(sg: *mut scatterlist, nents: c_uint);
    fn sg_set_page(sg: *mut scatterlist, page: *mut c_void, len: size_t, offset: c_uint);
    fn pfn_to_page(pfn: dma_addr_t) -> *mut c_void;
    fn PFN_DOWN(addr: dma_addr_t) -> dma_addr_t;
    fn offset_in_page(addr: dma_addr_t) -> c_uint;
    fn sg_dma_len(sg: *mut scatterlist) -> *mut c_uint;
    fn sg_dma_address(sg: *mut scatterlist) -> *mut dma_addr_t;
    fn dmaengine_prep_slave_sg(
        chan: *mut dma_chan,
        sg: *mut scatterlist,
        sg_len: c_uint,
        direction: c_int,
        flags: c_int,
    ) -> *mut dma_async_tx_descriptor;
    fn dmaengine_submit(desc: *mut dma_async_tx_descriptor) -> dma_cookie_t;
    fn dma_async_issue_pending(chan: *mut dma_chan);
    fn dma_cap_zero(mask: *mut dma_cap_mask_t);
    fn dma_cap_set(cap: c_int, mask: *mut dma_cap_mask_t);
    fn dma_request_channel(
        mask: dma_cap_mask_t,
        filter: Option<unsafe extern "C" fn(*mut dma_chan, *mut c_void) -> bool_t>,
        param: *mut c_void,
    ) -> *mut dma_chan;
    fn dma_release_channel(chan: *mut dma_chan);
    fn snd_pcm_lib_buffer_bytes(ss: *mut snd_pcm_substream) -> u32;
    fn snd_pcm_lib_period_bytes(ss: *mut snd_pcm_substream) -> u32;
    fn bytes_to_frames(rt: *mut snd_pcm_runtime, bytes: size_t) -> snd_pcm_sframes_t;
    fn siu_port_info(ss: *mut snd_pcm_substream) -> *mut siu_port;
    fn to_platform_device(dev: *mut device) -> *mut platform_device;
    fn siu_init_port(i: c_int, port_info: *mut *mut siu_port, card: *mut snd_card) -> c_int;
    fn snd_pcm_set_managed_buffer_all(
        pcm: *mut snd_pcm,
        typ: c_int,
        dev: *mut device,
        min: size_t,
        max: size_t,
    );
    fn INIT_WORK(work: *mut work_struct, func: unsafe extern "C" fn(*mut work_struct));
    fn cancel_work_sync(work: *mut work_struct) -> bool_t;
    fn siu_free_port(port_info: *mut siu_port);
    fn pr_debug(fmt: *const core::ffi::c_char, ...);
    fn dev_dbg(dev: *mut device, fmt: *const core::ffi::c_char, ...);
    fn dev_err(dev: *mut device, fmt: *const core::ffi::c_char, ...);
    fn dev_warn(dev: *mut device, fmt: *const core::ffi::c_char, ...);
    fn dev_info(dev: *mut device, fmt: *const core::ffi::c_char, ...);
}

/* transfersize is number of u32 dma transfers per period */
unsafe extern "C" fn siu_pcm_stmwrite_stop(port_info: *mut siu_port) -> c_int {
    let info: *mut siu_info = siu_i2s_data;
    let base: *mut u32 = (*info).reg;
    let siu_stream: *mut siu_stream = &mut (*port_info).playback;
    let mut stfifo: u32;

    if (*siu_stream).rw_flg == 0 {
        return -EPERM;
    }

    /* output FIFO disable */
    stfifo = siu_read32(base.offset(SIU_STFIFO));
    siu_write32(base.offset(SIU_STFIFO), stfifo & !0x0c180c18);
    pr_debug(
        b"%s: STFIFO %x -> %x\n\0".as_ptr() as *const core::ffi::c_char,
        b"siu_pcm_stmwrite_stop\0".as_ptr(),
        stfifo,
        stfifo & !0x0c180c18,
    );

    /* during stmwrite clear */
    (*siu_stream).rw_flg = 0;

    0
}

unsafe extern "C" fn siu_pcm_stmwrite_start(port_info: *mut siu_port) -> c_int {
    let siu_stream: *mut siu_stream = &mut (*port_info).playback;

    if (*siu_stream).rw_flg != 0 {
        return -EPERM;
    }

    /* Current period in buffer */
    (*port_info).playback.cur_period = 0;

    /* during stmwrite flag set */
    (*siu_stream).rw_flg = RWF_STM_WT;

    /* DMA transfer start */
    queue_work(system_highpri_wq, &mut (*siu_stream).work);

    0
}

unsafe extern "C" fn siu_dma_tx_complete(arg: *mut c_void) {
    let siu_stream: *mut siu_stream = arg as *mut siu_stream;

    if (*siu_stream).rw_flg == 0 {
        return;
    }

    /* Update completed period count */
    (*siu_stream).cur_period = (*siu_stream).cur_period.wrapping_add(1);
    if (*siu_stream).cur_period >= GET_MAX_PERIODS((*siu_stream).buf_bytes, (*siu_stream).period_bytes) {
        (*siu_stream).cur_period = 0;
    }

    pr_debug(
        b"%s: done period #%d (%u/%u bytes), cookie %d\n\0".as_ptr() as *const core::ffi::c_char,
        b"siu_dma_tx_complete\0".as_ptr(),
        (*siu_stream).cur_period,
        (*siu_stream).cur_period.wrapping_mul((*siu_stream).period_bytes),
        (*siu_stream).buf_bytes,
        (*siu_stream).cookie,
    );

    queue_work(system_highpri_wq, &mut (*siu_stream).work);

    /* Notify alsa: a period is done */
    snd_pcm_period_elapsed((*siu_stream).substream);
}

unsafe extern "C" fn siu_pcm_wr_set(port_info: *mut siu_port, buff: dma_addr_t, size: u32) -> c_int {
    let info: *mut siu_info = siu_i2s_data;
    let base: *mut u32 = (*info).reg;
    let siu_stream: *mut siu_stream = &mut (*port_info).playback;
    let substream: *mut snd_pcm_substream = (*siu_stream).substream;
    let dev: *mut device = (*(*(*(*substream).pcm).card)).dev;
    let mut desc: *mut dma_async_tx_descriptor;
    let mut cookie: dma_cookie_t;
    let mut sg: scatterlist = core::mem::zeroed();
    let mut stfifo: u32;

    sg_init_table(&mut sg, 1);
    sg_set_page(&mut sg, pfn_to_page(PFN_DOWN(buff)), size as size_t, offset_in_page(buff));
    *sg_dma_len(&mut sg) = size;
    *sg_dma_address(&mut sg) = buff;

    desc = dmaengine_prep_slave_sg(
        (*siu_stream).chan,
        &mut sg,
        1,
        DMA_MEM_TO_DEV,
        DMA_PREP_INTERRUPT | DMA_CTRL_ACK,
    );
    if desc.is_null() {
        dev_err(dev, b"Failed to allocate a dma descriptor\n\0".as_ptr() as *const core::ffi::c_char);
        return -ENOMEM;
    }

    (*desc).callback = Some(siu_dma_tx_complete);
    (*desc).callback_param = siu_stream as *mut c_void;
    cookie = dmaengine_submit(desc);
    if cookie < 0 {
        dev_err(dev, b"Failed to submit a dma transfer\n\0".as_ptr() as *const core::ffi::c_char);
        return cookie;
    }

    (*siu_stream).tx_desc = desc;
    (*siu_stream).cookie = cookie;

    dma_async_issue_pending((*siu_stream).chan);

    /* only output FIFO enable */
    stfifo = siu_read32(base.offset(SIU_STFIFO));
    siu_write32(base.offset(SIU_STFIFO), stfifo | ((*port_info).stfifo & 0x0c180c18));
    dev_dbg(
        dev,
        b"%s: STFIFO %x -> %x\n\0".as_ptr() as *const core::ffi::c_char,
        b"siu_pcm_wr_set\0".as_ptr(),
        stfifo,
        stfifo | ((*port_info).stfifo & 0x0c180c18),
    );

    0
}

unsafe extern "C" fn siu_pcm_rd_set(port_info: *mut siu_port, buff: dma_addr_t, size: size_t) -> c_int {
    let info: *mut siu_info = siu_i2s_data;
    let base: *mut u32 = (*info).reg;
    let siu_stream: *mut siu_stream = &mut (*port_info).capture;
    let substream: *mut snd_pcm_substream = (*siu_stream).substream;
    let dev: *mut device = (*(*(*(*substream).pcm).card)).dev;
    let mut desc: *mut dma_async_tx_descriptor;
    let mut cookie: dma_cookie_t;
    let mut sg: scatterlist = core::mem::zeroed();
    let mut stfifo: u32;

    dev_dbg(
        dev,
        b"%s: %u@%llx\n\0".as_ptr() as *const core::ffi::c_char,
        b"siu_pcm_rd_set\0".as_ptr(),
        size as c_uint,
        buff as u64,
    );

    sg_init_table(&mut sg, 1);
    sg_set_page(&mut sg, pfn_to_page(PFN_DOWN(buff)), size, offset_in_page(buff));
    *sg_dma_len(&mut sg) = size as c_uint;
    *sg_dma_address(&mut sg) = buff;

    desc = dmaengine_prep_slave_sg(
        (*siu_stream).chan,
        &mut sg,
        1,
        DMA_DEV_TO_MEM,
        DMA_PREP_INTERRUPT | DMA_CTRL_ACK,
    );
    if desc.is_null() {
        dev_err(dev, b"Failed to allocate dma descriptor\n\0".as_ptr() as *const core::ffi::c_char);
        return -ENOMEM;
    }

    (*desc).callback = Some(siu_dma_tx_complete);
    (*desc).callback_param = siu_stream as *mut c_void;
    cookie = dmaengine_submit(desc);
    if cookie < 0 {
        dev_err(dev, b"Failed to submit dma descriptor\n\0".as_ptr() as *const core::ffi::c_char);
        return cookie;
    }

    (*siu_stream).tx_desc = desc;
    (*siu_stream).cookie = cookie;

    dma_async_issue_pending((*siu_stream).chan);

    /* only input FIFO enable */
    stfifo = siu_read32(base.offset(SIU_STFIFO));
    siu_write32(base.offset(SIU_STFIFO), siu_read32(base.offset(SIU_STFIFO)) | ((*port_info).stfifo & 0x13071307));
    dev_dbg(
        dev,
        b"%s: STFIFO %x -> %x\n\0".as_ptr() as *const core::ffi::c_char,
        b"siu_pcm_rd_set\0".as_ptr(),
        stfifo,
        stfifo | ((*port_info).stfifo & 0x13071307),
    );

    0
}

unsafe extern "C" fn siu_io_work(work: *mut work_struct) {
    let siu_stream: *mut siu_stream = container_of_siu_stream_work(work);
    let substream: *mut snd_pcm_substream = (*siu_stream).substream;
    let dev: *mut device = (*(*(*(*substream).pcm).card)).dev;
    let rt: *mut snd_pcm_runtime = (*substream).runtime;
    let port_info: *mut siu_port = siu_port_info(substream);

    dev_dbg(
        dev,
        b"%s: flags %x\n\0".as_ptr() as *const core::ffi::c_char,
        b"siu_io_work\0".as_ptr(),
        (*siu_stream).rw_flg,
    );

    if (*siu_stream).rw_flg == 0 {
        dev_dbg(
            dev,
            b"%s: stream inactive\n\0".as_ptr() as *const core::ffi::c_char,
            b"siu_io_work\0".as_ptr(),
        );
        return;
    }

    if (*substream).stream == SNDRV_PCM_STREAM_CAPTURE {
        let buff: dma_addr_t;
        let count: size_t;

        buff = PERIOD_OFFSET((*rt).dma_addr, (*siu_stream).cur_period, (*siu_stream).period_bytes);
        count = (*siu_stream).period_bytes as size_t;

        /* DMA transfer start */
        siu_pcm_rd_set(port_info, buff, count);
    } else {
        siu_pcm_wr_set(
            port_info,
            PERIOD_OFFSET((*rt).dma_addr, (*siu_stream).cur_period, (*siu_stream).period_bytes),
            (*siu_stream).period_bytes,
        );
    }
}

// Rust equivalent of container_of(work, struct siu_stream, work).
unsafe fn container_of_siu_stream_work(work: *mut work_struct) -> *mut siu_stream {
    (work as *mut u8).offset(-(core::mem::offset_of!(siu_stream, work) as isize)) as *mut siu_stream
}

/* Capture */
unsafe extern "C" fn siu_pcm_stmread_start(port_info: *mut siu_port) -> c_int {
    let siu_stream: *mut siu_stream = &mut (*port_info).capture;

    if (*siu_stream).xfer_cnt > 0x1000000 {
        return -EINVAL;
    }
    if (*siu_stream).rw_flg != 0 {
        return -EPERM;
    }

    /* Current period in buffer */
    (*siu_stream).cur_period = 0;

    /* during stmread flag set */
    (*siu_stream).rw_flg = RWF_STM_RD;

    queue_work(system_highpri_wq, &mut (*siu_stream).work);

    0
}

unsafe extern "C" fn siu_pcm_stmread_stop(port_info: *mut siu_port) -> c_int {
    let info: *mut siu_info = siu_i2s_data;
    let base: *mut u32 = (*info).reg;
    let siu_stream: *mut siu_stream = &mut (*port_info).capture;
    let dev: *mut device = (*(*(*(*(*siu_stream).substream).pcm).card)).dev;
    let mut stfifo: u32;

    if (*siu_stream).rw_flg == 0 {
        return -EPERM;
    }

    /* input FIFO disable */
    stfifo = siu_read32(base.offset(SIU_STFIFO));
    siu_write32(base.offset(SIU_STFIFO), stfifo & !0x13071307);
    dev_dbg(
        dev,
        b"%s: STFIFO %x -> %x\n\0".as_ptr() as *const core::ffi::c_char,
        b"siu_pcm_stmread_stop\0".as_ptr(),
        stfifo,
        stfifo & !0x13071307,
    );

    /* during stmread flag clear */
    (*siu_stream).rw_flg = 0;

    0
}

unsafe extern "C" fn filter(chan: *mut dma_chan, secondary: *mut c_void) -> bool_t {
    let param: *mut sh_dmae_slave = secondary as *mut sh_dmae_slave;

    pr_debug(
        b"%s: secondary ID %d\n\0".as_ptr() as *const core::ffi::c_char,
        b"filter\0".as_ptr(),
        (*param).shdma_slave.slave_id,
    );

    (*chan).private = &mut (*param).shdma_slave as *mut shdma_slave as *mut c_void;
    true
}

unsafe extern "C" fn siu_pcm_open(component: *mut snd_soc_component, ss: *mut snd_pcm_substream) -> c_int {
    /* Playback / Capture */
    let pdata: *mut siu_platform = (*(*component).dev).platform_data as *mut siu_platform;
    let info: *mut siu_info = siu_i2s_data;
    let port_info: *mut siu_port = siu_port_info(ss);
    let siu_stream: *mut siu_stream;
    let port: u32 = (*info).port_id;
    let dev: *mut device = (*(*(*ss).pcm).card).dev;
    let mut mask: dma_cap_mask_t = core::mem::zeroed();
    let param: *mut sh_dmae_slave;

    dma_cap_zero(&mut mask);
    dma_cap_set(DMA_SLAVE, &mut mask);

    dev_dbg(
        dev,
        b"%s, port=%d@%p\n\0".as_ptr() as *const core::ffi::c_char,
        b"siu_pcm_open\0".as_ptr(),
        port,
        port_info,
    );

    if (*ss).stream == SNDRV_PCM_STREAM_PLAYBACK {
        siu_stream = &mut (*port_info).playback;
        param = &mut (*siu_stream).param;
        (*param).shdma_slave.slave_id = if port != 0 { (*pdata).dma_slave_tx_b } else { (*pdata).dma_slave_tx_a };
    } else {
        siu_stream = &mut (*port_info).capture;
        param = &mut (*siu_stream).param;
        (*param).shdma_slave.slave_id = if port != 0 { (*pdata).dma_slave_rx_b } else { (*pdata).dma_slave_rx_a };
    }

    /* Get DMA channel */
    (*siu_stream).chan = dma_request_channel(mask, Some(filter), param as *mut c_void);
    if (*siu_stream).chan.is_null() {
        dev_err(dev, b"DMA channel allocation failed!\n\0".as_ptr() as *const core::ffi::c_char);
        return -EBUSY;
    }

    (*siu_stream).substream = ss;

    0
}

unsafe extern "C" fn siu_pcm_close(_component: *mut snd_soc_component, ss: *mut snd_pcm_substream) -> c_int {
    let info: *mut siu_info = siu_i2s_data;
    let dev: *mut device = (*(*(*ss).pcm).card).dev;
    let port_info: *mut siu_port = siu_port_info(ss);
    let siu_stream: *mut siu_stream;

    dev_dbg(
        dev,
        b"%s: port=%d\n\0".as_ptr() as *const core::ffi::c_char,
        b"siu_pcm_close\0".as_ptr(),
        (*info).port_id,
    );

    if (*ss).stream == SNDRV_PCM_STREAM_PLAYBACK {
        siu_stream = &mut (*port_info).playback;
    } else {
        siu_stream = &mut (*port_info).capture;
    }

    dma_release_channel((*siu_stream).chan);
    (*siu_stream).chan = core::ptr::null_mut();

    (*siu_stream).substream = core::ptr::null_mut();

    0
}

unsafe extern "C" fn siu_pcm_prepare(_component: *mut snd_soc_component, ss: *mut snd_pcm_substream) -> c_int {
    let info: *mut siu_info = siu_i2s_data;
    let port_info: *mut siu_port = siu_port_info(ss);
    let dev: *mut device = (*(*(*ss).pcm).card).dev;
    let rt: *mut snd_pcm_runtime;
    let siu_stream: *mut siu_stream;
    let xfer_cnt: snd_pcm_sframes_t;

    if (*ss).stream == SNDRV_PCM_STREAM_PLAYBACK {
        siu_stream = &mut (*port_info).playback;
    } else {
        siu_stream = &mut (*port_info).capture;
    }

    rt = (*(*siu_stream).substream).runtime;

    (*siu_stream).buf_bytes = snd_pcm_lib_buffer_bytes(ss);
    (*siu_stream).period_bytes = snd_pcm_lib_period_bytes(ss);

    dev_dbg(
        dev,
        b"%s: port=%d, %d channels, period=%u bytes\n\0".as_ptr() as *const core::ffi::c_char,
        b"siu_pcm_prepare\0".as_ptr(),
        (*info).port_id,
        (*rt).channels,
        (*siu_stream).period_bytes,
    );

    /* We only support buffers that are multiples of the period */
    if (*siu_stream).buf_bytes % (*siu_stream).period_bytes != 0 {
        dev_err(
            dev,
            b"%s() - buffer=%d not multiple of period=%d\n\0".as_ptr() as *const core::ffi::c_char,
            b"siu_pcm_prepare\0".as_ptr(),
            (*siu_stream).buf_bytes,
            (*siu_stream).period_bytes,
        );
        return -EINVAL;
    }

    xfer_cnt = bytes_to_frames(rt, (*siu_stream).period_bytes as size_t);
    if xfer_cnt == 0 || xfer_cnt > 0x1000000 {
        return -EINVAL;
    }

    (*siu_stream).format = (*rt).format;
    (*siu_stream).xfer_cnt = xfer_cnt;

    dev_dbg(
        dev,
        b"port=%d buf=%lx buf_bytes=%d period_bytes=%d format=%d channels=%d xfer_cnt=%d\n\0".as_ptr() as *const core::ffi::c_char,
        (*info).port_id,
        (*rt).dma_addr as c_ulong,
        (*siu_stream).buf_bytes,
        (*siu_stream).period_bytes,
        (*siu_stream).format,
        (*rt).channels,
        xfer_cnt as c_int,
    );

    0
}

unsafe extern "C" fn siu_pcm_trigger(_component: *mut snd_soc_component, ss: *mut snd_pcm_substream, cmd: c_int) -> c_int {
    let info: *mut siu_info = siu_i2s_data;
    let dev: *mut device = (*(*(*ss).pcm).card).dev;
    let port_info: *mut siu_port = siu_port_info(ss);
    let ret: c_int;

    dev_dbg(
        dev,
        b"%s: port=%d@%p, cmd=%d\n\0".as_ptr() as *const core::ffi::c_char,
        b"siu_pcm_trigger\0".as_ptr(),
        (*info).port_id,
        port_info,
        cmd,
    );

    match cmd {
        SNDRV_PCM_TRIGGER_START => {
            if (*ss).stream == SNDRV_PCM_STREAM_PLAYBACK {
                ret = siu_pcm_stmwrite_start(port_info);
            } else {
                ret = siu_pcm_stmread_start(port_info);
            }

            if ret < 0 {
                dev_warn(
                    dev,
                    b"%s: start failed on port=%d\n\0".as_ptr() as *const core::ffi::c_char,
                    b"siu_pcm_trigger\0".as_ptr(),
                    (*info).port_id,
                );
            }
        }
        SNDRV_PCM_TRIGGER_STOP => {
            if (*ss).stream == SNDRV_PCM_STREAM_PLAYBACK {
                siu_pcm_stmwrite_stop(port_info);
            } else {
                siu_pcm_stmread_stop(port_info);
            }
            ret = 0;
        }
        _ => {
            dev_err(
                dev,
                b"%s() unsupported cmd=%d\n\0".as_ptr() as *const core::ffi::c_char,
                b"siu_pcm_trigger\0".as_ptr(),
                cmd,
            );
            ret = -EINVAL;
        }
    }

    ret
}

/*
 * So far only resolution of one period is supported, subject to extending the
 * dmangine API
 */
unsafe extern "C" fn siu_pcm_pointer_dma(
    _component: *mut snd_soc_component,
    ss: *mut snd_pcm_substream,
) -> snd_pcm_uframes_t {
    let dev: *mut device = (*(*(*ss).pcm).card).dev;
    let info: *mut siu_info = siu_i2s_data;
    let base: *mut u32 = (*info).reg;
    let port_info: *mut siu_port = siu_port_info(ss);
    let rt: *mut snd_pcm_runtime = (*ss).runtime;
    let mut ptr: size_t;
    let siu_stream: *mut siu_stream;

    if (*ss).stream == SNDRV_PCM_STREAM_PLAYBACK {
        siu_stream = &mut (*port_info).playback;
    } else {
        siu_stream = &mut (*port_info).capture;
    }

    /*
     * ptr is the offset into the buffer where the dma is currently at. We
     * check if the dma buffer has just wrapped.
     */
    ptr = PERIOD_OFFSET((*rt).dma_addr, (*siu_stream).cur_period, (*siu_stream).period_bytes)
        .wrapping_sub((*rt).dma_addr) as size_t;

    dev_dbg(
        dev,
        b"%s: port=%d, events %x, FSTS %x, xferred %u/%u, cookie %d\n\0".as_ptr() as *const core::ffi::c_char,
        b"siu_pcm_pointer_dma\0".as_ptr(),
        (*info).port_id,
        siu_read32(base.offset(SIU_EVNTC)),
        siu_read32(base.offset(SIU_SBFSTS)),
        ptr as c_uint,
        (*siu_stream).buf_bytes,
        (*siu_stream).cookie,
    );

    if ptr >= (*siu_stream).buf_bytes as size_t {
        ptr = 0;
    }

    bytes_to_frames((*ss).runtime, ptr) as snd_pcm_uframes_t
}

unsafe extern "C" fn siu_pcm_new(_component: *mut snd_soc_component, rtd: *mut snd_soc_pcm_runtime) -> c_int {
    let card: *mut snd_card = (*(*rtd).card).snd_card;
    let pcm: *mut snd_pcm = (*rtd).pcm;
    let info: *mut siu_info = siu_i2s_data;
    let pdev: *mut platform_device = to_platform_device((*card).dev);
    let mut ret: c_int;
    let mut i: c_int;

    /* pdev->id selects between SIUA and SIUB */
    if (*pdev).id < 0 || (*pdev).id >= SIU_PORT_NUM as c_int {
        return -EINVAL;
    }

    (*info).port_id = (*pdev).id as u32;

    /*
     * While the siu has 2 ports, only one port can be on at a time (only 1
     * SPB). So far all the boards using the siu had only one of the ports
     * wired to a codec. To simplify things, we only register one port with
     * alsa. In case both ports are needed, it should be changed here
     */
    i = (*pdev).id;
    while i < (*pdev).id + 1 {
        let port_info: *mut *mut siu_port = &mut siu_ports[i as usize];

        ret = siu_init_port(i, port_info, card);
        if ret < 0 {
            return ret;
        }

        snd_pcm_set_managed_buffer_all(
            pcm,
            SNDRV_DMA_TYPE_DEV,
            (*card).dev,
            SIU_BUFFER_BYTES_MAX,
            SIU_BUFFER_BYTES_MAX,
        );

        (**port_info).pcm = pcm;

        /* IO works */
        INIT_WORK(&mut (**port_info).playback.work, siu_io_work);
        INIT_WORK(&mut (**port_info).capture.work, siu_io_work);

        i += 1;
    }

    dev_info((*card).dev, b"SuperH SIU driver initialized.\n\0".as_ptr() as *const core::ffi::c_char);
    0
}

unsafe extern "C" fn siu_pcm_free(_component: *mut snd_soc_component, pcm: *mut snd_pcm) {
    let pdev: *mut platform_device = to_platform_device((*(*pcm).card).dev);
    let port_info: *mut siu_port = siu_ports[(*pdev).id as usize];

    cancel_work_sync(&mut (*port_info).capture.work);
    cancel_work_sync(&mut (*port_info).playback.work);

    siu_free_port(port_info);

    dev_dbg(
        (*(*pcm).card).dev,
        b"%s\n\0".as_ptr() as *const core::ffi::c_char,
        b"siu_pcm_free\0".as_ptr(),
    );
}

#[unsafe(no_mangle)]
pub static siu_component: snd_soc_component_driver = snd_soc_component_driver {
    name: DRV_NAME,
    open: Some(siu_pcm_open),
    close: Some(siu_pcm_close),
    prepare: Some(siu_pcm_prepare),
    trigger: Some(siu_pcm_trigger),
    pointer: Some(siu_pcm_pointer_dma),
    pcm_new: Some(siu_pcm_new),
    pcm_free: Some(siu_pcm_free),
    legacy_dai_naming: 1,
};

// EXPORT_SYMBOL_GPL(siu_component);

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
