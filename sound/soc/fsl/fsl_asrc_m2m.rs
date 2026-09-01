// SPDX-License-Identifier: GPL-2.0
//
// Copyright (C) 2014-2016 Freescale Semiconductor, Inc.
// Copyright (C) 2019-2024 NXP
//
// Freescale ASRC Memory to Memory (M2M) driver

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};
use core::{mem, ptr};

type u8 = u8;
type u16 = u16;
type u32 = u32;
type dma_addr_t = usize;
type snd_pcm_format_t = c_int;

const IN: usize = 0;
const OUT: usize = 1;
const GFP_KERNEL: c_uint = 0;
const EINVAL: c_int = 22;
const ENOMEM: c_int = 12;
const ETIMEDOUT: c_int = 110;
const EBUSY: c_int = 16;
const HZ: c_long = 100;
const SNDRV_DMA_TYPE_DEV: c_int = 0;
const DMA_PREP_INTERRUPT: c_ulong = 1;
const O_RDWR: c_uint = 0o2;
const SND_AUDIOCODEC_PCM: c_uint = 0;
const SND_COMPRESS_ACCEL: c_int = 0;
const SNDRV_DEFAULT_IDX1: c_int = -1;
static SNDRV_DEFAULT_STR1: *const c_char = ptr::null();
static THIS_MODULE: *mut c_void = ptr::null_mut();
const PAIR_CTX_NUM: usize = 4;

/* Maximum output and capture buffer size */
const ASRC_M2M_BUFFER_SIZE: c_uint = 512 * 1024;

/* Maximum output and capture period size */
const ASRC_M2M_PERIOD_SIZE: c_uint = 48 * 1024;

type c_long = i64;

#[repr(C)]
struct device {
    _private: [u8; 0],
}

#[repr(C)]
struct platform_device {
    dev: device,
}

#[repr(C)]
struct completion {
    _private: [u8; 0],
}

#[repr(C)]
struct regmap {
    _private: [u8; 0],
}

#[repr(C)]
struct dma_chan {
    _private: [u8; 0],
}

#[repr(C)]
struct dma_async_tx_descriptor {
    chan: *mut dma_chan,
    callback: Option<unsafe extern "C" fn(*mut c_void)>,
    callback_param: *mut c_void,
}

#[repr(C)]
struct snd_dma_buffer {
    area: *mut c_void,
    addr: dma_addr_t,
    bytes: usize,
}

#[repr(C)]
struct snd_compr_runtime {
    private_data: *mut c_void,
}

#[repr(C)]
struct snd_compr_stream {
    private_data: *mut c_void,
    runtime: *mut snd_compr_runtime,
}

#[repr(C)]
struct snd_compr_task_runtime {
    input_size: c_uint,
    output_size: c_uint,
    input: *mut dma_buf,
    output: *mut dma_buf,
}

#[repr(C)]
struct snd_compr_params {
    codec: snd_codec,
    buffer: snd_compr_buffer,
}

#[repr(C)]
struct snd_compr_buffer {
    fragment_size: c_uint,
}

#[repr(C)]
struct snd_codec {
    format: snd_pcm_format_t,
    pcm_format: snd_pcm_format_t,
    sample_rate: c_uint,
    options: snd_codec_options,
    ch_in: c_uint,
    ch_out: c_uint,
}

#[repr(C)]
struct snd_codec_options {
    src_d: snd_codec_src,
}

#[repr(C)]
struct snd_codec_src {
    out_sample_rate: c_uint,
}

#[repr(C)]
struct snd_compr_caps {
    num_codecs: c_uint,
    min_fragment_size: c_uint,
    max_fragment_size: c_uint,
    min_fragments: c_uint,
    max_fragments: c_uint,
    codecs: [c_uint; 32],
}

#[repr(C)]
struct snd_codec_desc {
    max_ch: c_uint,
    sample_rates: [u32; 32],
    num_sample_rates: c_uint,
    formats: snd_pcm_format_t,
    pcm_formats: c_uint,
    src: snd_codec_src_caps,
}

#[repr(C)]
struct snd_codec_src_caps {
    out_sample_rate_min: c_uint,
    out_sample_rate_max: c_uint,
}

#[repr(C)]
struct snd_compr_codec_caps {
    codec: c_uint,
    num_descriptors: c_uint,
    descriptor: [snd_codec_desc; 32],
}

#[repr(C)]
struct snd_compr_ops {
    open: Option<unsafe extern "C" fn(*mut snd_compr_stream) -> c_int>,
    free: Option<unsafe extern "C" fn(*mut snd_compr_stream) -> c_int>,
    set_params: Option<unsafe extern "C" fn(*mut snd_compr_stream, *mut snd_compr_params) -> c_int>,
    get_caps: Option<unsafe extern "C" fn(*mut snd_compr_stream, *mut snd_compr_caps) -> c_int>,
    get_codec_caps: Option<unsafe extern "C" fn(*mut snd_compr_stream, *mut snd_compr_codec_caps) -> c_int>,
    task_create: Option<unsafe extern "C" fn(*mut snd_compr_stream, *mut snd_compr_task_runtime) -> c_int>,
    task_start: Option<unsafe extern "C" fn(*mut snd_compr_stream, *mut snd_compr_task_runtime) -> c_int>,
    task_stop: Option<unsafe extern "C" fn(*mut snd_compr_stream, *mut snd_compr_task_runtime) -> c_int>,
    task_free: Option<unsafe extern "C" fn(*mut snd_compr_stream, *mut snd_compr_task_runtime) -> c_int>,
}

#[repr(C)]
struct snd_card {
    driver: [c_char; 16],
    shortname: [c_char; 32],
    longname: [c_char; 80],
}

#[repr(C)]
struct snd_compr {
    ops: *const snd_compr_ops,
    private_data: *mut c_void,
}

#[repr(C)]
struct fsl_asrc_m2m_cap {
    fmt_in: c_uint,
    fmt_out: c_uint,
    rate_in_count: c_int,
    rate_out_count: c_int,
    rate_in: [u32; 32],
    rate_out: [u32; 32],
    chan_min: c_uint,
    chan_max: c_uint,
}

type asrc_pair_index = c_int;

#[repr(C)]
struct fsl_asrc_pair {
    complete: [completion; 2],
    private: *mut c_void,
    asrc: *mut fsl_asrc,
    index: asrc_pair_index,
    sample_format: [snd_pcm_format_t; 2],
    rate: [c_uint; 2],
    channels: c_uint,
    buf_len: [c_uint; 2],
    dma_buffer: [snd_dma_buffer; 2],
    dma_chan: [*mut dma_chan; 2],
    desc: [*mut dma_async_tx_descriptor; 2],
    ratio_mod_flag: bool,
    ratio_mod: c_uint,
}

#[repr(C)]
struct fsl_asrc {
    pdev: *mut platform_device,
    regmap: *mut regmap,
    paddr: u32,
    pair_priv_size: usize,
    card: *mut snd_card,
    pair: [*mut fsl_asrc_pair; PAIR_CTX_NUM],
    get_output_fifo_size: Option<unsafe extern "C" fn(*mut fsl_asrc_pair) -> u32>,
    get_fifo_addr: Option<unsafe extern "C" fn(c_int, asrc_pair_index) -> u32>,
    m2m_get_maxburst: Option<unsafe extern "C" fn(c_int, *mut fsl_asrc_pair) -> c_uint>,
    m2m_set_ratio_mod: Option<unsafe extern "C" fn(*mut fsl_asrc_pair, c_uint)>,
    start_before_dma: bool,
    m2m_start: Option<unsafe extern "C" fn(*mut fsl_asrc_pair)>,
    m2m_output_ready: Option<unsafe extern "C" fn(*mut fsl_asrc_pair)>,
    m2m_calc_out_len: Option<unsafe extern "C" fn(*mut fsl_asrc_pair, c_uint) -> c_uint>,
    m2m_get_cap: Option<unsafe extern "C" fn(*mut fsl_asrc_m2m_cap) -> c_int>,
    request_pair: Option<unsafe extern "C" fn(c_uint, *mut fsl_asrc_pair) -> c_int>,
    m2m_prepare: Option<unsafe extern "C" fn(*mut fsl_asrc_pair) -> c_int>,
    get_dma_channel: Option<unsafe extern "C" fn(*mut fsl_asrc_pair, c_int) -> *mut dma_chan>,
    m2m_unprepare: Option<unsafe extern "C" fn(*mut fsl_asrc_pair)>,
    release_pair: Option<unsafe extern "C" fn(*mut fsl_asrc_pair)>,
    m2m_stop: Option<unsafe extern "C" fn(*mut fsl_asrc_pair)>,
    m2m_pair_suspend: Option<unsafe extern "C" fn(*mut fsl_asrc_pair)>,
    m2m_pair_resume: Option<unsafe extern "C" fn(*mut fsl_asrc_pair)>,
}

#[repr(C)]
struct scatterlist {
    dma_address: dma_addr_t,
    dma_len: c_uint,
}

#[repr(C)]
struct sg_table {
    _private: [u8; 0],
}

#[repr(C)]
struct vm_area_struct {
    _private: [u8; 0],
}

#[repr(C)]
struct dma_buf {
    priv_: *mut c_void,
}

#[repr(C)]
struct dma_buf_attachment {
    dmabuf: *mut dma_buf,
    dev: *mut device,
}

#[repr(C)]
struct dma_buf_export_info {
    ops: *const dma_buf_ops,
    size: usize,
    flags: c_uint,
    priv_: *mut c_void,
}

#[repr(C)]
struct dma_buf_ops {
    mmap: Option<unsafe extern "C" fn(*mut dma_buf, *mut vm_area_struct) -> c_int>,
    map_dma_buf: Option<unsafe extern "C" fn(*mut dma_buf_attachment, dma_data_direction) -> *mut sg_table>,
    unmap_dma_buf: Option<unsafe extern "C" fn(*mut dma_buf_attachment, *mut sg_table, dma_data_direction)>,
    release: Option<unsafe extern "C" fn(*mut dma_buf)>,
}

type dma_data_direction = c_int;

#[repr(C)]
#[derive(Copy, Clone)]
enum dma_slave_buswidth {
    DMA_SLAVE_BUSWIDTH_1_BYTE = 1,
    DMA_SLAVE_BUSWIDTH_2_BYTES = 2,
    DMA_SLAVE_BUSWIDTH_3_BYTES = 3,
    DMA_SLAVE_BUSWIDTH_4_BYTES = 4,
}

const DMA_MEM_TO_DEV: c_int = 1;
const DMA_DEV_TO_MEM: c_int = 2;

#[repr(C)]
struct dma_slave_config {
    direction: c_int,
    dst_addr: u32,
    dst_addr_width: dma_slave_buswidth,
    dst_maxburst: c_uint,
    src_addr: u32,
    src_addr_width: dma_slave_buswidth,
    src_maxburst: c_uint,
}

unsafe extern "C" {
    fn complete(x: *mut completion);
    fn reinit_completion(x: *mut completion);
    fn init_completion(x: *mut completion);
    fn completion_done(x: *mut completion) -> bool;
    fn wait_for_completion_interruptible_timeout(x: *mut completion, timeout: c_long) -> c_long;
    fn snd_pcm_format_physical_width(format: snd_pcm_format_t) -> c_uint;
    fn pcm_format_to_bits(format: snd_pcm_format_t) -> c_uint;
    fn regmap_read(map: *mut regmap, reg: u32, val: *mut u32) -> c_int;
    fn dmaengine_slave_config(chan: *mut dma_chan, config: *mut dma_slave_config) -> c_int;
    fn dmaengine_prep_slave_sg(
        chan: *mut dma_chan,
        sg: *mut scatterlist,
        sg_len: c_uint,
        direction: c_int,
        flags: c_ulong,
    ) -> *mut dma_async_tx_descriptor;
    fn dmaengine_submit(desc: *mut dma_async_tx_descriptor);
    fn dma_async_issue_pending(chan: *mut dma_chan);
    fn dmaengine_terminate_all(chan: *mut dma_chan);
    fn dma_release_channel(chan: *mut dma_chan);
    fn kzalloc(size: usize, flags: c_uint) -> *mut c_void;
    fn kmalloc(size: usize, flags: c_uint) -> *mut c_void;
    fn kfree(ptr: *mut c_void);
    fn snd_dma_alloc_pages(typ: c_int, dev: *mut device, size: c_int, dmab: *mut snd_dma_buffer) -> c_int;
    fn snd_dma_free_pages(dmab: *mut snd_dma_buffer);
    fn pm_runtime_get_sync(dev: *mut device) -> c_int;
    fn pm_runtime_put_sync(dev: *mut device) -> c_int;
    fn snd_dma_buffer_mmap(dmab: *mut snd_dma_buffer, vma: *mut vm_area_struct) -> c_int;
    fn dma_get_sgtable(dev: *mut device, sgt: *mut sg_table, cpu_addr: *mut c_void, dma_addr: dma_addr_t, size: usize) -> c_int;
    fn dma_map_sgtable(dev: *mut device, sgt: *mut sg_table, direction: dma_data_direction, attrs: c_ulong) -> c_int;
    fn dma_unmap_sgtable(dev: *mut device, sgt: *mut sg_table, direction: dma_data_direction, attrs: c_ulong);
    fn sg_free_table(sgt: *mut sg_table);
    fn sg_init_table(sg: *mut scatterlist, nents: c_uint);
    fn dma_buf_export(info: *mut dma_buf_export_info) -> *mut dma_buf;
    fn IS_ERR(ptr: *const c_void) -> bool;
    fn PTR_ERR(ptr: *const c_void) -> c_int;
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn devm_kzalloc(dev: *mut device, size: usize, flags: c_uint) -> *mut c_void;
    fn snd_card_new(dev: *mut device, idx: c_int, id: *const c_char, module: *mut c_void, extra: c_int, card: *mut *mut snd_card) -> c_int;
    fn snd_compress_new(card: *mut snd_card, device: c_int, dir: c_int, id: *const c_char, compr: *mut snd_compr) -> c_int;
    fn snd_card_register(card: *mut snd_card) -> c_int;
    fn snd_card_free(card: *mut snd_card);
    fn strscpy(dst: *mut c_char, src: *const c_char, size: usize) -> isize;
    fn memcpy(dst: *mut c_void, src: *const c_void, n: usize) -> *mut c_void;
}

fn dir_str(dir: c_int) -> *const c_char {
    if dir == IN as c_int {
        c"in".as_ptr()
    } else {
        c"out".as_ptr()
    }
}

fn rounddown(x: c_uint, y: c_uint) -> c_uint {
    x / y * y
}

/* dma complete callback */
unsafe extern "C" fn asrc_input_dma_callback(data: *mut c_void) {
    let pair = data as *mut fsl_asrc_pair;

    unsafe { complete(&mut (*pair).complete[IN]) };
}

/* dma complete callback */
unsafe extern "C" fn asrc_output_dma_callback(data: *mut c_void) {
    let pair = data as *mut fsl_asrc_pair;

    unsafe { complete(&mut (*pair).complete[OUT]) };
}

/**
 *asrc_read_last_fifo: read all the remaining data from FIFO
 *@pair: Structure pointer of fsl_asrc_pair
 *@dma_vaddr: virtual address of capture buffer
 *@length: payload length of capture buffer
 */
unsafe fn asrc_read_last_fifo(pair: *mut fsl_asrc_pair, dma_vaddr: *mut c_void, length: *mut u32) {
    let asrc = unsafe { (*pair).asrc };
    let index = unsafe { (*pair).index };
    let mut i: u32;
    let mut reg: u32 = 0;
    let mut size: u32;
    let mut t_size: u32 = 0;
    let width: u32;
    let mut reg32: *mut u32 = ptr::null_mut();
    let mut reg16: *mut u16 = ptr::null_mut();
    let mut reg24: *mut u8 = ptr::null_mut();

    width = unsafe { snd_pcm_format_physical_width((*pair).sample_format[OUT]) };
    if width == 32 {
        reg32 = unsafe { (dma_vaddr as *mut u8).add(*length as usize) as *mut u32 };
    } else if width == 16 {
        reg16 = unsafe { (dma_vaddr as *mut u8).add(*length as usize) as *mut u16 };
    } else {
        reg24 = unsafe { (dma_vaddr as *mut u8).add(*length as usize) };
    }

    loop {
        size = unsafe { ((*asrc).get_output_fifo_size.unwrap())(pair) };
        if unsafe { size + *length > ASRC_M2M_BUFFER_SIZE } {
            break;
        }

        i = 0;
        while unsafe { i < size * (*pair).channels } {
            unsafe {
                regmap_read((*asrc).regmap, ((*asrc).get_fifo_addr.unwrap())(OUT as c_int, index), &mut reg);
                if !reg32.is_null() {
                    *reg32 = reg;
                    reg32 = reg32.add(1);
                } else if !reg16.is_null() {
                    *reg16 = reg as u16;
                    reg16 = reg16.add(1);
                } else {
                    *reg24 = reg as u8;
                    reg24 = reg24.add(1);
                    *reg24 = (reg >> 8) as u8;
                    reg24 = reg24.add(1);
                    *reg24 = (reg >> 16) as u8;
                    reg24 = reg24.add(1);
                }
            }
            i += 1;
        }
        t_size += size;

        /* In case there is data left in FIFO */
        if size == 0 {
            break;
        }
    }

    /* Update payload length */
    unsafe {
        if !reg32.is_null() {
            *length += t_size * (*pair).channels * 4;
        } else if !reg16.is_null() {
            *length += t_size * (*pair).channels * 2;
        } else {
            *length += t_size * (*pair).channels * 3;
        }
    }
}

/* config dma channel */
unsafe fn asrc_dmaconfig(
    pair: *mut fsl_asrc_pair,
    chan: *mut dma_chan,
    dma_addr: u32,
    buf_addr: dma_addr_t,
    buf_len: u32,
    dir: c_int,
    width: c_int,
) -> c_int {
    let asrc = unsafe { (*pair).asrc };
    let dev = unsafe { &mut (*(*asrc).pdev).dev as *mut device };
    let mut slave_config: dma_slave_config = unsafe { mem::zeroed() };
    let buswidth: dma_slave_buswidth;
    let mut sg_len: c_uint;
    let max_period_size: c_uint;
    let sg: *mut scatterlist;
    let mut ret: c_int;
    let mut i: c_uint;

    match width {
        8 => buswidth = dma_slave_buswidth::DMA_SLAVE_BUSWIDTH_1_BYTE,
        16 => buswidth = dma_slave_buswidth::DMA_SLAVE_BUSWIDTH_2_BYTES,
        24 => buswidth = dma_slave_buswidth::DMA_SLAVE_BUSWIDTH_3_BYTES,
        32 => buswidth = dma_slave_buswidth::DMA_SLAVE_BUSWIDTH_4_BYTES,
        _ => {
            unsafe { dev_err(dev, c"invalid word width\n".as_ptr()) };
            return -EINVAL;
        }
    }

    unsafe { ptr::write_bytes(&mut slave_config as *mut dma_slave_config, 0, 1) };
    if dir == IN as c_int {
        slave_config.direction = DMA_MEM_TO_DEV;
        slave_config.dst_addr = dma_addr;
        slave_config.dst_addr_width = buswidth;
        slave_config.dst_maxburst = unsafe { ((*asrc).m2m_get_maxburst.unwrap())(IN as c_int, pair) };
    } else {
        slave_config.direction = DMA_DEV_TO_MEM;
        slave_config.src_addr = dma_addr;
        slave_config.src_addr_width = buswidth;
        slave_config.src_maxburst = unsafe { ((*asrc).m2m_get_maxburst.unwrap())(OUT as c_int, pair) };
    }

    ret = unsafe { dmaengine_slave_config(chan, &mut slave_config) };
    if ret != 0 {
        unsafe {
            dev_err(
                dev,
                c"failed to config dmaengine for %s task: %d\n".as_ptr(),
                dir_str(dir),
                ret,
            )
        };
        return -EINVAL;
    }

    max_period_size = unsafe { rounddown(ASRC_M2M_PERIOD_SIZE, width as c_uint * (*pair).channels / 8) };
    /* scatter gather mode */
    sg_len = buf_len / max_period_size;
    if buf_len % max_period_size != 0 {
        sg_len += 1;
    }

    sg = unsafe { kmalloc(mem::size_of::<scatterlist>() * sg_len as usize, GFP_KERNEL) as *mut scatterlist };
    if sg.is_null() {
        return -ENOMEM;
    }

    unsafe { sg_init_table(sg, sg_len) };
    i = 0;
    while i < sg_len - 1 {
        unsafe {
            (*sg.add(i as usize)).dma_address = buf_addr + i as usize * max_period_size as usize;
            (*sg.add(i as usize)).dma_len = max_period_size;
        }
        i += 1;
    }
    unsafe {
        (*sg.add(i as usize)).dma_address = buf_addr + i as usize * max_period_size as usize;
        (*sg.add(i as usize)).dma_len = buf_len - i * max_period_size;

        (*pair).desc[dir as usize] = dmaengine_prep_slave_sg(
            chan,
            sg,
            sg_len,
            slave_config.direction,
            DMA_PREP_INTERRUPT,
        );
        kfree(sg as *mut c_void);
        if (*pair).desc[dir as usize].is_null() {
            dev_err(dev, c"failed to prepare dmaengine for %s task\n".as_ptr(), dir_str(dir));
            return -EINVAL;
        }

        (*(*pair).desc[dir as usize]).callback = if dir == IN as c_int {
            Some(asrc_input_dma_callback)
        } else {
            Some(asrc_output_dma_callback)
        };
        (*(*pair).desc[dir as usize]).callback_param = pair as *mut c_void;
    }

    0
}

/* main function of converter */
unsafe fn asrc_m2m_device_run(pair: *mut fsl_asrc_pair, task: *mut snd_compr_task_runtime) -> c_int {
    let asrc = unsafe { (*pair).asrc };
    let dev = unsafe { &mut (*(*asrc).pdev).dev as *mut device };
    let index = unsafe { (*pair).index };
    let src_buf: *mut snd_dma_buffer;
    let dst_buf: *mut snd_dma_buffer;
    let in_buf_len: c_uint;
    let mut out_dma_len: c_uint;
    let mut width: c_uint;
    let mut fifo_addr: u32;
    let mut ret: c_int = 0;

    /* set ratio mod */
    unsafe {
        if let Some(m2m_set_ratio_mod) = (*asrc).m2m_set_ratio_mod {
            if (*pair).ratio_mod_flag {
                m2m_set_ratio_mod(pair, (*pair).ratio_mod);
                (*pair).ratio_mod_flag = false;
            }
        }
    }

    src_buf = unsafe { &mut (*pair).dma_buffer[IN] };
    dst_buf = unsafe { &mut (*pair).dma_buffer[OUT] };

    width = unsafe { snd_pcm_format_physical_width((*pair).sample_format[IN]) };
    fifo_addr = unsafe { (*asrc).paddr + ((*asrc).get_fifo_addr.unwrap())(IN as c_int, index) };

    in_buf_len = unsafe { (*task).input_size };

    if unsafe {
        in_buf_len < width * (*pair).channels / 8
            || in_buf_len > ASRC_M2M_BUFFER_SIZE
            || in_buf_len % (width * (*pair).channels / 8) != 0
    } {
        unsafe { dev_err(dev, c"out buffer size is error: [%d]\n".as_ptr(), in_buf_len) };
        ret = -EINVAL;
        return ret;
    }

    /* dma config for output dma channel */
    ret = unsafe {
        asrc_dmaconfig(
            pair,
            (*pair).dma_chan[IN],
            fifo_addr,
            (*src_buf).addr,
            in_buf_len,
            IN as c_int,
            width as c_int,
        )
    };
    if ret != 0 {
        unsafe { dev_err(dev, c"out dma config error\n".as_ptr()) };
        return ret;
    }

    width = unsafe { snd_pcm_format_physical_width((*pair).sample_format[OUT]) };
    fifo_addr = unsafe { (*asrc).paddr + ((*asrc).get_fifo_addr.unwrap())(OUT as c_int, index) };
    out_dma_len = unsafe { ((*asrc).m2m_calc_out_len.unwrap())(pair, in_buf_len) };
    if out_dma_len > 0 && out_dma_len <= ASRC_M2M_BUFFER_SIZE {
        /* dma config for capture dma channel */
        ret = unsafe {
            asrc_dmaconfig(
                pair,
                (*pair).dma_chan[OUT],
                fifo_addr,
                (*dst_buf).addr,
                out_dma_len,
                OUT as c_int,
                width as c_int,
            )
        };
        if ret != 0 {
            unsafe { dev_err(dev, c"cap dma config error\n".as_ptr()) };
            return ret;
        }
    } else if out_dma_len > ASRC_M2M_BUFFER_SIZE {
        unsafe { dev_err(dev, c"cap buffer size error\n".as_ptr()) };
        ret = -EINVAL;
        return ret;
    }

    unsafe {
        reinit_completion(&mut (*pair).complete[IN]);
        reinit_completion(&mut (*pair).complete[OUT]);

        if (*asrc).start_before_dma {
            ((*asrc).m2m_start.unwrap())(pair);
        }

        /* Submit DMA request */
        dmaengine_submit((*pair).desc[IN]);
        dma_async_issue_pending((*(*pair).desc[IN]).chan);
        if out_dma_len > 0 {
            if (*asrc).start_before_dma {
                if let Some(m2m_output_ready) = (*asrc).m2m_output_ready {
                    m2m_output_ready(pair);
                }
            }
            dmaengine_submit((*pair).desc[OUT]);
            dma_async_issue_pending((*(*pair).desc[OUT]).chan);
        }

        if !(*asrc).start_before_dma {
            ((*asrc).m2m_start.unwrap())(pair);
        }

        if wait_for_completion_interruptible_timeout(&mut (*pair).complete[IN], 10 * HZ) == 0 {
            dev_err(dev, c"out DMA task timeout\n".as_ptr());
            ret = -ETIMEDOUT;
            return ret;
        }

        if out_dma_len > 0 {
            if wait_for_completion_interruptible_timeout(&mut (*pair).complete[OUT], 10 * HZ) == 0 {
                dev_err(dev, c"cap DMA task timeout\n".as_ptr());
                ret = -ETIMEDOUT;
                return ret;
            }
        }

        /* read the last words from FIFO */
        asrc_read_last_fifo(pair, (*dst_buf).area, &mut out_dma_len);
        /* update payload length for capture */
        (*task).output_size = out_dma_len;
    }

    ret
}

unsafe extern "C" fn fsl_asrc_m2m_comp_open(stream: *mut snd_compr_stream) -> c_int {
    let asrc = unsafe { (*stream).private_data as *mut fsl_asrc };
    let runtime = unsafe { (*stream).runtime };
    let dev = unsafe { &mut (*(*asrc).pdev).dev as *mut device };
    let pair: *mut fsl_asrc_pair;
    let size: c_int;
    let mut ret: c_int;

    pair = unsafe { kzalloc(mem::size_of::<fsl_asrc_pair>() + (*asrc).pair_priv_size, GFP_KERNEL) as *mut fsl_asrc_pair };
    if pair.is_null() {
        return -ENOMEM;
    }

    unsafe {
        (*pair).private = (pair as *mut u8).add(mem::size_of::<fsl_asrc_pair>()) as *mut c_void;
        (*pair).asrc = asrc;

        init_completion(&mut (*pair).complete[IN]);
        init_completion(&mut (*pair).complete[OUT]);

        (*runtime).private_data = pair as *mut c_void;
    }

    size = ASRC_M2M_BUFFER_SIZE as c_int;
    ret = unsafe { snd_dma_alloc_pages(SNDRV_DMA_TYPE_DEV, dev, size, &mut (*pair).dma_buffer[IN]) };
    if ret != 0 {
        unsafe { kfree(pair as *mut c_void) };
        return ret;
    }

    ret = unsafe { snd_dma_alloc_pages(SNDRV_DMA_TYPE_DEV, dev, size, &mut (*pair).dma_buffer[OUT]) };
    if ret != 0 {
        unsafe {
            snd_dma_free_pages(&mut (*pair).dma_buffer[IN]);
            kfree(pair as *mut c_void);
        }
        return ret;
    }

    ret = unsafe { pm_runtime_get_sync(dev) };
    if ret < 0 {
        unsafe {
            dev_err(dev, c"Failed to power up asrc\n".as_ptr());
            snd_dma_free_pages(&mut (*pair).dma_buffer[OUT]);
            snd_dma_free_pages(&mut (*pair).dma_buffer[IN]);
            kfree(pair as *mut c_void);
        }
        return ret;
    }

    0
}

unsafe extern "C" fn fsl_asrc_m2m_comp_release(stream: *mut snd_compr_stream) -> c_int {
    let asrc = unsafe { (*stream).private_data as *mut fsl_asrc };
    let runtime = unsafe { (*stream).runtime };
    let pair = unsafe { (*runtime).private_data as *mut fsl_asrc_pair };
    let dev = unsafe { &mut (*(*asrc).pdev).dev as *mut device };

    unsafe {
        pm_runtime_put_sync(dev);

        snd_dma_free_pages(&mut (*pair).dma_buffer[IN]);
        snd_dma_free_pages(&mut (*pair).dma_buffer[OUT]);

        kfree((*runtime).private_data);
    }

    0
}

unsafe extern "C" fn fsl_asrc_m2m_comp_set_params(
    stream: *mut snd_compr_stream,
    params: *mut snd_compr_params,
) -> c_int {
    let asrc = unsafe { (*stream).private_data as *mut fsl_asrc };
    let runtime = unsafe { (*stream).runtime };
    let pair = unsafe { (*runtime).private_data as *mut fsl_asrc_pair };
    let mut cap: fsl_asrc_m2m_cap = unsafe { mem::zeroed() };
    let mut ret: c_int;
    let mut i: c_int;

    ret = unsafe { ((*asrc).m2m_get_cap.unwrap())(&mut cap) };
    if ret != 0 {
        return -EINVAL;
    }

    if unsafe { pcm_format_to_bits((*params).codec.format) & cap.fmt_in != 0 } {
        unsafe { (*pair).sample_format[IN] = (*params).codec.format };
    } else {
        return -EINVAL;
    }

    if unsafe { pcm_format_to_bits((*params).codec.pcm_format) & cap.fmt_out != 0 } {
        unsafe { (*pair).sample_format[OUT] = (*params).codec.pcm_format };
    } else {
        return -EINVAL;
    }

    /* check input rate is in scope */
    i = 0;
    while i < cap.rate_in_count {
        if unsafe { (*params).codec.sample_rate == cap.rate_in[i as usize] } {
            unsafe { (*pair).rate[IN] = (*params).codec.sample_rate };
            break;
        }
        i += 1;
    }
    if i == cap.rate_in_count {
        return -EINVAL;
    }

    /* check output rate is in scope */
    i = 0;
    while i < cap.rate_out_count {
        if unsafe { (*params).codec.options.src_d.out_sample_rate == cap.rate_out[i as usize] } {
            unsafe { (*pair).rate[OUT] = (*params).codec.options.src_d.out_sample_rate };
            break;
        }
        i += 1;
    }
    if i == cap.rate_out_count {
        return -EINVAL;
    }

    if unsafe {
        (*params).codec.ch_in != (*params).codec.ch_out
            || (*params).codec.ch_in < cap.chan_min
            || (*params).codec.ch_in > cap.chan_max
    } {
        return -EINVAL;
    }

    unsafe {
        (*pair).channels = (*params).codec.ch_in;
        (*pair).buf_len[IN] = (*params).buffer.fragment_size;
        (*pair).buf_len[OUT] = (*params).buffer.fragment_size;
    }

    0
}

unsafe extern "C" fn fsl_asrc_m2m_mmap(dmabuf: *mut dma_buf, vma: *mut vm_area_struct) -> c_int {
    let dmab = unsafe { (*dmabuf).priv_ as *mut snd_dma_buffer };

    unsafe { snd_dma_buffer_mmap(dmab, vma) }
}

unsafe extern "C" fn fsl_asrc_m2m_map_dma_buf(
    attachment: *mut dma_buf_attachment,
    direction: dma_data_direction,
) -> *mut sg_table {
    let dmab = unsafe { (*(*attachment).dmabuf).priv_ as *mut snd_dma_buffer };
    let sgt: *mut sg_table;

    sgt = unsafe { kmalloc(mem::size_of::<sg_table>(), GFP_KERNEL) as *mut sg_table };
    if sgt.is_null() {
        return ptr::null_mut();
    }

    if unsafe { dma_get_sgtable((*attachment).dev, sgt, (*dmab).area, (*dmab).addr, (*dmab).bytes) < 0 } {
        unsafe {
            sg_free_table(sgt);
            kfree(sgt as *mut c_void);
        }
        return ptr::null_mut();
    }

    if unsafe { dma_map_sgtable((*attachment).dev, sgt, direction, 0) != 0 } {
        unsafe {
            sg_free_table(sgt);
            kfree(sgt as *mut c_void);
        }
        return ptr::null_mut();
    }

    sgt
}

unsafe extern "C" fn fsl_asrc_m2m_unmap_dma_buf(
    attachment: *mut dma_buf_attachment,
    table: *mut sg_table,
    direction: dma_data_direction,
) {
    unsafe { dma_unmap_sgtable((*attachment).dev, table, direction, 0) };
}

unsafe extern "C" fn fsl_asrc_m2m_release(_dmabuf: *mut dma_buf) {
    /* buffer is released by fsl_asrc_m2m_comp_release() */
}

static fsl_asrc_m2m_dma_buf_ops: dma_buf_ops = dma_buf_ops {
    mmap: Some(fsl_asrc_m2m_mmap),
    map_dma_buf: Some(fsl_asrc_m2m_map_dma_buf),
    unmap_dma_buf: Some(fsl_asrc_m2m_unmap_dma_buf),
    release: Some(fsl_asrc_m2m_release),
};

unsafe extern "C" fn fsl_asrc_m2m_comp_task_create(
    stream: *mut snd_compr_stream,
    task: *mut snd_compr_task_runtime,
) -> c_int {
    let mut exp_info_in: dma_buf_export_info = unsafe { mem::zeroed() };
    let mut exp_info_out: dma_buf_export_info = unsafe { mem::zeroed() };
    let asrc = unsafe { (*stream).private_data as *mut fsl_asrc };
    let runtime = unsafe { (*stream).runtime };
    let pair = unsafe { (*runtime).private_data as *mut fsl_asrc_pair };
    let dev = unsafe { &mut (*(*asrc).pdev).dev as *mut device };
    let mut ret: c_int;

    exp_info_in.ops = &fsl_asrc_m2m_dma_buf_ops;
    exp_info_in.size = ASRC_M2M_BUFFER_SIZE as usize;
    exp_info_in.flags = O_RDWR;
    exp_info_in.priv_ = unsafe { &mut (*pair).dma_buffer[IN] as *mut snd_dma_buffer as *mut c_void };
    unsafe {
        (*task).input = dma_buf_export(&mut exp_info_in);
        if IS_ERR((*task).input as *const c_void) {
            ret = PTR_ERR((*task).input as *const c_void);
            return ret;
        }
    }

    exp_info_out.ops = &fsl_asrc_m2m_dma_buf_ops;
    exp_info_out.size = ASRC_M2M_BUFFER_SIZE as usize;
    exp_info_out.flags = O_RDWR;
    exp_info_out.priv_ = unsafe { &mut (*pair).dma_buffer[OUT] as *mut snd_dma_buffer as *mut c_void };
    unsafe {
        (*task).output = dma_buf_export(&mut exp_info_out);
        if IS_ERR((*task).output as *const c_void) {
            ret = PTR_ERR((*task).output as *const c_void);
            return ret;
        }
    }

    /* Request asrc pair/context */
    ret = unsafe { ((*asrc).request_pair.unwrap())((*pair).channels, pair) };
    if ret != 0 {
        unsafe { dev_err(dev, c"failed to request pair: %d\n".as_ptr(), ret) };
        return ret;
    }

    ret = unsafe { ((*asrc).m2m_prepare.unwrap())(pair) };
    if ret != 0 {
        unsafe {
            dev_err(dev, c"failed to start pair part one: %d\n".as_ptr(), ret);
            ((*asrc).release_pair.unwrap())(pair);
        }
        return ret;
    }

    /* Request dma channels */
    unsafe {
        (*pair).dma_chan[IN] = ((*asrc).get_dma_channel.unwrap())(pair, IN as c_int);
        if (*pair).dma_chan[IN].is_null() {
            dev_err(dev, c"[ctx%d] failed to get input DMA channel\n".as_ptr(), (*pair).index);
            ret = -EBUSY;
            if let Some(m2m_unprepare) = (*asrc).m2m_unprepare {
                m2m_unprepare(pair);
            }
            ((*asrc).release_pair.unwrap())(pair);
            return ret;
        }

        (*pair).dma_chan[OUT] = ((*asrc).get_dma_channel.unwrap())(pair, OUT as c_int);
        if (*pair).dma_chan[OUT].is_null() {
            dev_err(dev, c"[ctx%d] failed to get output DMA channel\n".as_ptr(), (*pair).index);
            ret = -EBUSY;
            dma_release_channel((*pair).dma_chan[IN]);
            if let Some(m2m_unprepare) = (*asrc).m2m_unprepare {
                m2m_unprepare(pair);
            }
            ((*asrc).release_pair.unwrap())(pair);
            return ret;
        }
    }

    0
}

unsafe extern "C" fn fsl_asrc_m2m_comp_task_start(
    stream: *mut snd_compr_stream,
    task: *mut snd_compr_task_runtime,
) -> c_int {
    let runtime = unsafe { (*stream).runtime };
    let pair = unsafe { (*runtime).private_data as *mut fsl_asrc_pair };

    unsafe { asrc_m2m_device_run(pair, task) }
}

unsafe extern "C" fn fsl_asrc_m2m_comp_task_stop(
    _stream: *mut snd_compr_stream,
    _task: *mut snd_compr_task_runtime,
) -> c_int {
    0
}

unsafe extern "C" fn fsl_asrc_m2m_comp_task_free(
    stream: *mut snd_compr_stream,
    _task: *mut snd_compr_task_runtime,
) -> c_int {
    let asrc = unsafe { (*stream).private_data as *mut fsl_asrc };
    let runtime = unsafe { (*stream).runtime };
    let pair = unsafe { (*runtime).private_data as *mut fsl_asrc_pair };

    /* Stop & release pair/context */
    unsafe {
        if let Some(m2m_stop) = (*asrc).m2m_stop {
            m2m_stop(pair);
        }

        if let Some(m2m_unprepare) = (*asrc).m2m_unprepare {
            m2m_unprepare(pair);
        }
        ((*asrc).release_pair.unwrap())(pair);

        /* Release dma channel */
        if !(*pair).dma_chan[IN].is_null() {
            dma_release_channel((*pair).dma_chan[IN]);
        }
        if !(*pair).dma_chan[OUT].is_null() {
            dma_release_channel((*pair).dma_chan[OUT]);
        }
    }

    0
}

unsafe extern "C" fn fsl_asrc_m2m_get_caps(
    _cstream: *mut snd_compr_stream,
    caps: *mut snd_compr_caps,
) -> c_int {
    unsafe {
        (*caps).num_codecs = 1;
        (*caps).min_fragment_size = 4096;
        (*caps).max_fragment_size = 4096;
        (*caps).min_fragments = 1;
        (*caps).max_fragments = 1;
        (*caps).codecs[0] = SND_AUDIOCODEC_PCM;
    }

    0
}

unsafe fn fsl_asrc_m2m_fill_codec_caps(
    asrc: *mut fsl_asrc,
    codec: *mut snd_compr_codec_caps,
) -> c_int {
    let mut cap: fsl_asrc_m2m_cap = unsafe { mem::zeroed() };
    let mut k: snd_pcm_format_t = 0;
    let mut j: c_int = 0;
    let mut ret: c_int;

    ret = unsafe { ((*asrc).m2m_get_cap.unwrap())(&mut cap) };
    if ret != 0 {
        return -EINVAL;
    }

    /* pcm_for_each_format(k) */
    while k < 64 {
        if unsafe { pcm_format_to_bits(k) & cap.fmt_in != 0 } {
            unsafe {
                (*codec).descriptor[j as usize].max_ch = cap.chan_max;
                memcpy(
                    (*codec).descriptor[j as usize].sample_rates.as_mut_ptr() as *mut c_void,
                    cap.rate_in.as_ptr() as *const c_void,
                    cap.rate_in_count as usize * mem::size_of::<u32>(),
                );
                (*codec).descriptor[j as usize].num_sample_rates = cap.rate_in_count as c_uint;
                (*codec).descriptor[j as usize].formats = k;
                (*codec).descriptor[j as usize].pcm_formats = cap.fmt_out;
                (*codec).descriptor[j as usize].src.out_sample_rate_min = cap.rate_out[0];
                (*codec).descriptor[j as usize].src.out_sample_rate_max =
                    cap.rate_out[(cap.rate_out_count - 1) as usize];
                j += 1;
            }
        }
        k += 1;
    }

    unsafe {
        (*codec).codec = SND_AUDIOCODEC_PCM;
        (*codec).num_descriptors = j as c_uint;
    }
    0
}

unsafe extern "C" fn fsl_asrc_m2m_get_codec_caps(
    stream: *mut snd_compr_stream,
    codec: *mut snd_compr_codec_caps,
) -> c_int {
    let asrc = unsafe { (*stream).private_data as *mut fsl_asrc };

    unsafe { fsl_asrc_m2m_fill_codec_caps(asrc, codec) }
}

static mut fsl_asrc_m2m_compr_ops: snd_compr_ops = snd_compr_ops {
    open: Some(fsl_asrc_m2m_comp_open),
    free: Some(fsl_asrc_m2m_comp_release),
    set_params: Some(fsl_asrc_m2m_comp_set_params),
    get_caps: Some(fsl_asrc_m2m_get_caps),
    get_codec_caps: Some(fsl_asrc_m2m_get_codec_caps),
    task_create: Some(fsl_asrc_m2m_comp_task_create),
    task_start: Some(fsl_asrc_m2m_comp_task_start),
    task_stop: Some(fsl_asrc_m2m_comp_task_stop),
    task_free: Some(fsl_asrc_m2m_comp_task_free),
};

#[unsafe(no_mangle)]
pub unsafe extern "C" fn fsl_asrc_m2m_suspend(asrc: *mut fsl_asrc) -> c_int {
    let mut pair: *mut fsl_asrc_pair;
    let mut i: c_int;

    i = 0;
    while i < PAIR_CTX_NUM as c_int {
        unsafe {
            pair = (*asrc).pair[i as usize];
            if pair.is_null() || (*pair).dma_buffer[IN].area.is_null() || (*pair).dma_buffer[OUT].area.is_null() {
                i += 1;
                continue;
            }
            if !completion_done(&mut (*pair).complete[IN]) {
                if !(*pair).dma_chan[IN].is_null() {
                    dmaengine_terminate_all((*pair).dma_chan[IN]);
                }
                asrc_input_dma_callback(pair as *mut c_void);
            }
            if !completion_done(&mut (*pair).complete[OUT]) {
                if !(*pair).dma_chan[OUT].is_null() {
                    dmaengine_terminate_all((*pair).dma_chan[OUT]);
                }
                asrc_output_dma_callback(pair as *mut c_void);
            }

            if let Some(m2m_pair_suspend) = (*asrc).m2m_pair_suspend {
                m2m_pair_suspend(pair);
            }
        }
        i += 1;
    }

    0
}
/* EXPORT_SYMBOL_GPL(fsl_asrc_m2m_suspend); */

#[unsafe(no_mangle)]
pub unsafe extern "C" fn fsl_asrc_m2m_resume(asrc: *mut fsl_asrc) -> c_int {
    let mut pair: *mut fsl_asrc_pair;
    let mut i: c_int;

    i = 0;
    while i < PAIR_CTX_NUM as c_int {
        unsafe {
            pair = (*asrc).pair[i as usize];
            if pair.is_null() {
                i += 1;
                continue;
            }
            if let Some(m2m_pair_resume) = (*asrc).m2m_pair_resume {
                m2m_pair_resume(pair);
            }
        }
        i += 1;
    }

    0
}
/* EXPORT_SYMBOL_GPL(fsl_asrc_m2m_resume); */

#[unsafe(no_mangle)]
pub unsafe extern "C" fn fsl_asrc_m2m_init(asrc: *mut fsl_asrc) -> c_int {
    let dev = unsafe { &mut (*(*asrc).pdev).dev as *mut device };
    let mut card: *mut snd_card = ptr::null_mut();
    let compr: *mut snd_compr;
    let mut ret: c_int;

    ret = unsafe { snd_card_new(dev, SNDRV_DEFAULT_IDX1, SNDRV_DEFAULT_STR1, THIS_MODULE, 0, &mut card) };
    if ret < 0 {
        return ret;
    }

    unsafe {
        strscpy((*card).driver.as_mut_ptr(), c"fsl-asrc-m2m".as_ptr(), (*card).driver.len());
        strscpy((*card).shortname.as_mut_ptr(), c"ASRC-M2M".as_ptr(), (*card).shortname.len());
        strscpy((*card).longname.as_mut_ptr(), c"ASRC-M2M".as_ptr(), (*card).shortname.len());

        (*asrc).card = card;

        compr = devm_kzalloc(dev, mem::size_of::<snd_compr>(), GFP_KERNEL) as *mut snd_compr;
        if compr.is_null() {
            ret = -ENOMEM;
            snd_card_free(card);
            return ret;
        }

        (*compr).ops = &raw const fsl_asrc_m2m_compr_ops;
        (*compr).private_data = asrc as *mut c_void;

        ret = snd_compress_new(card, 0, SND_COMPRESS_ACCEL, c"ASRC M2M".as_ptr(), compr);
        if ret < 0 {
            snd_card_free(card);
            return ret;
        }

        ret = snd_card_register(card);
        if ret < 0 {
            snd_card_free(card);
            return ret;
        }
    }

    0
}
/* EXPORT_SYMBOL_GPL(fsl_asrc_m2m_init); */

#[unsafe(no_mangle)]
pub unsafe extern "C" fn fsl_asrc_m2m_exit(asrc: *mut fsl_asrc) {
    let card = unsafe { (*asrc).card };

    unsafe { snd_card_free(card) };
}
/* EXPORT_SYMBOL_GPL(fsl_asrc_m2m_exit); */

/* MODULE_IMPORT_NS("DMA_BUF"); */
/* MODULE_AUTHOR("Shengjiu Wang <Shengjiu.Wang@nxp.com>"); */
/* MODULE_DESCRIPTION("Freescale ASRC M2M driver"); */
/* MODULE_LICENSE("GPL"); */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
