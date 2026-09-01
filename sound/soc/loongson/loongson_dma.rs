// SPDX-License-Identifier: GPL-2.0
//
// Loongson ALSA SoC Platform (DMA) driver
//
// Copyright (C) 2023 Loongson Technology Corporation Limited
// Author: Yingkun Meng <mengyingkun@loongson.cn>
//         Binbin ZHou <zhoubinbin@loongson.cn>
//

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]

use core::ffi::c_void;
use core::mem::size_of;
use core::ptr;

/* Dependencies from:
 * linux/module.h, linux/io-64-nonatomic-lo-hi.h, linux/delay.h,
 * linux/pm_runtime.h, linux/dma-mapping.h, sound/soc.h, sound/pcm.h,
 * sound/pcm_params.h, and loongson_i2s.h.
 */

type u32 = u32;
type u64 = u64;
type size_t = usize;
type dma_addr_t = u64;
type snd_pcm_uframes_t = u64;
type irqreturn_t = i32;

const fn BIT(nr: u32) -> u64 {
    1u64 << nr
}

/* Internal DMA dma_order Register */
const DMA_ORDER_STOP: u64 = BIT(4); /* DMA stop */
const DMA_ORDER_START: u64 = BIT(3); /* DMA start */
const DMA_ORDER_ASK_VALID: u64 = BIT(2); /* DMA ask valid flag */
const DMA_ORDER_AXI_UNCO: u64 = BIT(1); /* Uncache access */
const DMA_ORDER_ADDR_64: u64 = BIT(0); /* 64bits address support */

const DMA_ORDER_ASK_MASK: u64 = !0x1fu64; /* Ask addr mask */
const DMA_ORDER_CTRL_MASK: u64 = 0x0fu64; /* Control mask  */

/*
 * Internal DMA registers descriptor.
 */
#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct loongson_idma_desc {
    pub order: u32,       /* Next descriptor address register */
    pub saddr: u32,       /* Source address register */
    pub daddr: u32,       /* Device address register */
    pub length: u32,      /* Total length register */
    pub step_length: u32, /* Memory stride register */
    pub step_times: u32,  /* Repeat time register */
    pub cmd: u32,         /* Command register */
    pub stats: u32,       /* Status register */
    pub order_hi: u32,    /* Next descriptor high address register */
    pub saddr_hi: u32,    /* High source address register */
    pub res: [u32; 6],    /* Reserved */
}

#[repr(C)]
pub struct loongson_runtime_data {
    pub dma_data: *mut loongson_idma_data,

    pub dma_desc_arr: *mut loongson_idma_desc,
    pub dma_desc_arr_phy: dma_addr_t,
    pub dma_desc_arr_size: i32,

    pub dma_pos_desc: *mut loongson_idma_desc,
    pub dma_pos_desc_phy: dma_addr_t,
}

static loongson_idma_hardware: snd_pcm_hardware = snd_pcm_hardware {
    info: SNDRV_PCM_INFO_MMAP
        | SNDRV_PCM_INFO_INTERLEAVED
        | SNDRV_PCM_INFO_MMAP_VALID
        | SNDRV_PCM_INFO_RESUME
        | SNDRV_PCM_INFO_PAUSE,
    formats: SNDRV_PCM_FMTBIT_S8
        | SNDRV_PCM_FMTBIT_S16_LE
        | SNDRV_PCM_FMTBIT_S20_3LE
        | SNDRV_PCM_FMTBIT_S24_LE,
    period_bytes_min: 128,
    period_bytes_max: 128 * 1024,
    periods_min: 1,
    periods_max: PAGE_SIZE / size_of::<loongson_idma_desc>(),
    buffer_bytes_max: 1024 * 1024,
};

unsafe fn dma_desc_save(prtd: *mut loongson_runtime_data) -> *mut loongson_idma_desc {
    let order_reg: *mut c_void = (*(*prtd).dma_data).order_addr;
    let mut val: u64;

    val = (*prtd).dma_pos_desc_phy as u64 & DMA_ORDER_ASK_MASK;
    val |= readq(order_reg) & DMA_ORDER_CTRL_MASK;
    val |= DMA_ORDER_ASK_VALID;
    writeq(val, order_reg);

    while readl(order_reg) as u64 & DMA_ORDER_ASK_VALID != 0 {
        udelay(2);
    }

    (*prtd).dma_pos_desc
}

unsafe extern "C" fn loongson_idma_pcm_trigger(
    component: *mut snd_soc_component,
    substream: *mut snd_pcm_substream,
    cmd: i32,
) -> i32 {
    let prtd: *mut loongson_runtime_data = (*(*substream).runtime).private_data as *mut loongson_runtime_data;
    let dev: *mut device = (*(*(*substream).pcm).card).dev;
    let order_reg: *mut c_void = (*(*prtd).dma_data).order_addr;
    let mut val: u64;

    match cmd {
        SNDRV_PCM_TRIGGER_START | SNDRV_PCM_TRIGGER_RESUME | SNDRV_PCM_TRIGGER_PAUSE_RELEASE => {
            val = (*prtd).dma_pos_desc_phy & DMA_ORDER_ASK_MASK;
            if (*dev).coherent_dma_mask == DMA_BIT_MASK(64) {
                val |= DMA_ORDER_ADDR_64;
            } else {
                val &= !DMA_ORDER_ADDR_64;
            }
            val |= readq(order_reg) & DMA_ORDER_CTRL_MASK;
            val |= DMA_ORDER_START;
            writeq(val, order_reg);

            while readl(order_reg) as u64 & DMA_ORDER_START != 0 {
                udelay(2);
            }
        }
        SNDRV_PCM_TRIGGER_STOP | SNDRV_PCM_TRIGGER_SUSPEND | SNDRV_PCM_TRIGGER_PAUSE_PUSH => {
            dma_desc_save(prtd);

            /* dma stop */
            val = readq(order_reg) | DMA_ORDER_STOP;
            writeq(val, order_reg);
            udelay(1000);
        }
        _ => {
            dev_err(dev, b"Invalid pcm trigger operation\n\0".as_ptr());
            return -EINVAL;
        }
    }

    0
}

unsafe extern "C" fn loongson_idma_pcm_hw_params(
    component: *mut snd_soc_component,
    substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
) -> i32 {
    let runtime: *mut snd_pcm_runtime = (*substream).runtime;
    let dev: *mut device = (*(*(*substream).pcm).card).dev;
    let prtd: *mut loongson_runtime_data = (*runtime).private_data as *mut loongson_runtime_data;
    let buf_len: size_t = params_buffer_bytes(params);
    let period_len: size_t = params_period_bytes(params);
    let mut order_addr: dma_addr_t;
    let mut mem_addr: dma_addr_t;
    let mut desc: *mut loongson_idma_desc;
    let num_periods: u32;
    let mut i: i32;

    if buf_len % period_len != 0 {
        dev_err(dev, b"buf len not multiply of period len\n\0".as_ptr());
        return -EINVAL;
    }

    num_periods = (buf_len / period_len) as u32;
    if num_periods == 0 || num_periods > (*prtd).dma_desc_arr_size as u32 {
        dev_err(dev, b"dma data too small or too big\n\0".as_ptr());
        return -EINVAL;
    }

    snd_pcm_set_runtime_buffer(substream, &mut (*substream).dma_buffer);
    (*runtime).dma_bytes = buf_len;

    /* initialize dma descriptor array */
    mem_addr = (*runtime).dma_addr;
    order_addr = (*prtd).dma_desc_arr_phy;
    i = 0;
    while i < num_periods as i32 {
        desc = (*prtd).dma_desc_arr.add(i as usize);

        /* next descriptor physical address */
        order_addr = order_addr.wrapping_add(size_of::<loongson_idma_desc>() as dma_addr_t);
        (*desc).order = lower_32_bits(order_addr | BIT(0)) as u32;
        (*desc).order_hi = upper_32_bits(order_addr) as u32;

        (*desc).saddr = lower_32_bits(mem_addr) as u32;
        (*desc).saddr_hi = upper_32_bits(mem_addr) as u32;
        (*desc).daddr = (*(*prtd).dma_data).dev_addr;

        (*desc).cmd = BIT(0) as u32;
        if (*substream).stream == SNDRV_PCM_STREAM_PLAYBACK {
            (*desc).cmd |= BIT(12) as u32;
        }

        (*desc).length = (period_len >> 2) as u32;
        (*desc).step_length = 0;
        (*desc).step_times = 1;

        mem_addr = mem_addr.wrapping_add(period_len as dma_addr_t);
        i += 1;
    }
    desc = (*prtd).dma_desc_arr.add((num_periods - 1) as usize);
    (*desc).order = lower_32_bits((*prtd).dma_desc_arr_phy | BIT(0)) as u32;
    (*desc).order_hi = upper_32_bits((*prtd).dma_desc_arr_phy) as u32;

    /* init position descriptor */
    ptr::write((*prtd).dma_pos_desc, ptr::read((*prtd).dma_desc_arr));

    0
}

unsafe extern "C" fn loongson_idma_pcm_pointer(
    component: *mut snd_soc_component,
    substream: *mut snd_pcm_substream,
) -> snd_pcm_uframes_t {
    let runtime: *mut snd_pcm_runtime = (*substream).runtime;
    let dev: *mut device = (*(*(*substream).pcm).card).dev;
    let prtd: *mut loongson_runtime_data = (*runtime).private_data as *mut loongson_runtime_data;
    let desc: *mut loongson_idma_desc;
    let mut x: snd_pcm_uframes_t;
    let addr: u64;

    desc = dma_desc_save(prtd);
    addr = ((*desc).saddr_hi as u64) << 32 | (*desc).saddr as u64;

    if addr < (*runtime).dma_addr || addr > (*runtime).dma_addr + (*runtime).dma_bytes as u64 {
        dev_warn(dev, b"WARNING! dma_addr:0x%llx\n\0".as_ptr(), addr);
        x = 0;
    } else {
        x = bytes_to_frames(runtime, addr - (*runtime).dma_addr);
        if x == (*runtime).buffer_size {
            x = 0;
        }
    }

    x
}

unsafe extern "C" fn loongson_idma_pcm_dma_irq(
    irq: i32,
    devid: *mut c_void,
) -> irqreturn_t {
    let substream: *mut snd_pcm_substream = devid as *mut snd_pcm_substream;

    snd_pcm_period_elapsed(substream);
    IRQ_HANDLED
}

unsafe extern "C" fn loongson_idma_pcm_open(
    component: *mut snd_soc_component,
    substream: *mut snd_pcm_substream,
) -> i32 {
    let runtime: *mut snd_pcm_runtime = (*substream).runtime;
    let rtd: *mut snd_soc_pcm_runtime = snd_soc_substream_to_rtd(substream);
    let card: *mut snd_card = (*(*substream).pcm).card;
    let mut prtd: *mut loongson_runtime_data;
    let dma_data: *mut loongson_idma_data;

    /*
     * For mysterious reasons (and despite what the manual says)
     * playback samples are lost if the DMA count is not a multiple
     * of the DMA burst size.  Let's add a rule to enforce that.
     */
    snd_pcm_hw_constraint_step(runtime, 0, SNDRV_PCM_HW_PARAM_PERIOD_BYTES, 128);
    snd_pcm_hw_constraint_step(runtime, 0, SNDRV_PCM_HW_PARAM_BUFFER_BYTES, 128);
    snd_pcm_hw_constraint_integer((*substream).runtime, SNDRV_PCM_HW_PARAM_PERIODS);
    snd_soc_set_runtime_hwparams(substream, &loongson_idma_hardware);

    prtd = kzalloc_obj::<loongson_runtime_data>();
    if prtd.is_null() {
        return -ENOMEM;
    }

    (*prtd).dma_desc_arr = dma_alloc_coherent(
        (*card).dev,
        PAGE_SIZE,
        &mut (*prtd).dma_desc_arr_phy,
        GFP_KERNEL,
    ) as *mut loongson_idma_desc;
    if (*prtd).dma_desc_arr.is_null() {
        kfree(prtd as *mut c_void);
        return -ENOMEM;
    }

    (*prtd).dma_desc_arr_size = (PAGE_SIZE / size_of::<loongson_idma_desc>()) as i32;

    (*prtd).dma_pos_desc = dma_alloc_coherent(
        (*card).dev,
        size_of::<loongson_idma_desc>(),
        &mut (*prtd).dma_pos_desc_phy,
        GFP_KERNEL,
    ) as *mut loongson_idma_desc;
    if (*prtd).dma_pos_desc.is_null() {
        dma_free_coherent(
            (*card).dev,
            PAGE_SIZE,
            (*prtd).dma_desc_arr as *mut c_void,
            (*prtd).dma_desc_arr_phy,
        );
        kfree(prtd as *mut c_void);
        return -ENOMEM;
    }

    dma_data = snd_soc_dai_get_dma_data(snd_soc_rtd_to_cpu(rtd, 0), substream);
    (*prtd).dma_data = dma_data;

    (*(*substream).runtime).private_data = prtd as *mut c_void;

    0
}

unsafe extern "C" fn loongson_idma_pcm_close(
    component: *mut snd_soc_component,
    substream: *mut snd_pcm_substream,
) -> i32 {
    let card: *mut snd_card = (*(*substream).pcm).card;
    let prtd: *mut loongson_runtime_data = (*(*substream).runtime).private_data as *mut loongson_runtime_data;

    dma_free_coherent(
        (*card).dev,
        PAGE_SIZE,
        (*prtd).dma_desc_arr as *mut c_void,
        (*prtd).dma_desc_arr_phy,
    );

    dma_free_coherent(
        (*card).dev,
        size_of::<loongson_idma_desc>(),
        (*prtd).dma_pos_desc as *mut c_void,
        (*prtd).dma_pos_desc_phy,
    );

    kfree(prtd as *mut c_void);
    0
}

unsafe extern "C" fn loongson_idma_pcm_mmap(
    component: *mut snd_soc_component,
    substream: *mut snd_pcm_substream,
    vma: *mut vm_area_struct,
) -> i32 {
    remap_pfn_range(
        vma,
        (*vma).vm_start,
        (*substream).dma_buffer.addr >> PAGE_SHIFT,
        (*vma).vm_end - (*vma).vm_start,
        (*vma).vm_page_prot,
    )
}

unsafe extern "C" fn loongson_idma_pcm_new(
    component: *mut snd_soc_component,
    rtd: *mut snd_soc_pcm_runtime,
) -> i32 {
    let card: *mut snd_card = (*(*rtd).card).snd_card;
    let mut substream: *mut snd_pcm_substream;
    let mut dma_data: *mut loongson_idma_data;
    let mut i: u32 = 0;
    let mut ret: i32;

    while i < SNDRV_PCM_STREAM_LAST + 1 {
        substream = (*(*rtd).pcm).streams[i as usize].substream;
        if substream.is_null() {
            i += 1;
            continue;
        }

        dma_data = snd_soc_dai_get_dma_data(snd_soc_rtd_to_cpu(rtd, 0), substream);
        ret = devm_request_irq(
            (*card).dev,
            (*dma_data).irq,
            Some(loongson_idma_pcm_dma_irq),
            IRQF_TRIGGER_HIGH,
            LS_I2S_DRVNAME,
            substream as *mut c_void,
        );
        if ret < 0 {
            dev_err((*card).dev, b"request irq for DMA failed\n\0".as_ptr());
            return ret;
        }

        i += 1;
    }

    snd_pcm_set_fixed_buffer_all(
        (*rtd).pcm,
        SNDRV_DMA_TYPE_DEV,
        (*card).dev,
        loongson_idma_hardware.buffer_bytes_max,
    )
}

/* Internal DMA component */
#[no_mangle]
pub static loongson_i2s_idma_component: snd_soc_component_driver = snd_soc_component_driver {
    name: LS_I2S_DRVNAME,
    open: Some(loongson_idma_pcm_open),
    close: Some(loongson_idma_pcm_close),
    hw_params: Some(loongson_idma_pcm_hw_params),
    trigger: Some(loongson_idma_pcm_trigger),
    pointer: Some(loongson_idma_pcm_pointer),
    mmap: Some(loongson_idma_pcm_mmap),
    pcm_new: Some(loongson_idma_pcm_new),
};
/* EXPORT_SYMBOL_GPL(loongson_i2s_idma_component); */

static loongson_edma_hardware: snd_pcm_hardware = snd_pcm_hardware {
    info: SNDRV_PCM_INFO_MMAP
        | SNDRV_PCM_INFO_INTERLEAVED
        | SNDRV_PCM_INFO_MMAP_VALID
        | SNDRV_PCM_INFO_RESUME
        | SNDRV_PCM_INFO_PAUSE,
    formats: SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S20_3LE | SNDRV_PCM_FMTBIT_S24_LE,
    period_bytes_min: 128,
    period_bytes_max: 128 * 1024,
    periods_min: 1,
    periods_max: 64,
    buffer_bytes_max: 1024 * 1024,
};

#[no_mangle]
pub static loongson_dmaengine_pcm_config: snd_dmaengine_pcm_config = snd_dmaengine_pcm_config {
    pcm_hardware: &loongson_edma_hardware,
    prepare_slave_config: Some(snd_dmaengine_pcm_prepare_slave_config),
    prealloc_buffer_size: 128 * 1024,
};
/* EXPORT_SYMBOL_GPL(loongson_dmaengine_pcm_config); */

/* External DMA component */
unsafe extern "C" fn loongson_edma_pcm_open(
    component: *mut snd_soc_component,
    substream: *mut snd_pcm_substream,
) -> i32 {
    let runtime: *mut snd_pcm_runtime = (*substream).runtime;

    if (*(*substream).pcm).device & 1 != 0 {
        (*runtime).hw.info &= !SNDRV_PCM_INFO_INTERLEAVED;
        (*runtime).hw.info |= SNDRV_PCM_INFO_NONINTERLEAVED;
    }

    if (*(*substream).pcm).device & 2 != 0 {
        (*runtime).hw.info &= !(SNDRV_PCM_INFO_MMAP | SNDRV_PCM_INFO_MMAP_VALID);
    }
    /*
     * For mysterious reasons (and despite what the manual says)
     * playback samples are lost if the DMA count is not a multiple
     * of the DMA burst size.  Let's add a rule to enforce that.
     */
    snd_pcm_hw_constraint_step(runtime, 0, SNDRV_PCM_HW_PARAM_PERIOD_BYTES, 128);
    snd_pcm_hw_constraint_step(runtime, 0, SNDRV_PCM_HW_PARAM_BUFFER_BYTES, 128);
    snd_pcm_hw_constraint_integer((*substream).runtime, SNDRV_PCM_HW_PARAM_PERIODS);

    0
}

#[no_mangle]
pub static loongson_i2s_edma_component: snd_soc_component_driver = snd_soc_component_driver {
    name: LS_I2S_DRVNAME,
    open: Some(loongson_edma_pcm_open),
};
/* EXPORT_SYMBOL_GPL(loongson_i2s_edma_component); */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
