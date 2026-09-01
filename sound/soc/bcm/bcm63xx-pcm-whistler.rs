// SPDX-License-Identifier: GPL-2.0-or-later
// linux/sound/bcm/bcm63xx-pcm-whistler.c
// BCM63xx whistler pcm interface
// Copyright (c) 2020 Broadcom Corporation
// Author: Kevin-Ke Li <kevin-ke.li@broadcom.com>

// C dependencies originally included:
// <linux/dma-mapping.h>, <linux/io.h>, <linux/irq.h>, <linux/module.h>,
// <sound/pcm_params.h>, <linux/regmap.h>, <linux/of_device.h>,
// <sound/soc.h>, "bcm63xx-i2s.h".

use core::ffi::{c_char, c_int, c_uint, c_void};

type dma_addr_t = usize;
type snd_pcm_uframes_t = usize;
type irqreturn_t = c_uint;

#[repr(C)]
pub struct device {
    pub of_node: *mut c_void,
}

#[repr(C)]
pub struct platform_device {
    pub dev: device,
}

#[repr(C)]
pub struct regmap {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_pcm_hardware {
    pub info: c_uint,
    pub formats: u64,
    pub period_bytes_max: usize,
    pub periods_min: c_uint,
    pub periods_max: c_uint,
    pub buffer_bytes_max: usize,
    pub fifo_size: c_uint,
}

#[repr(C)]
pub struct snd_soc_component {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_pcm_hw_params {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_dai {
    pub dev: *mut device,
}

#[repr(C)]
pub struct snd_soc_pcm_runtime {
    pub pcm: *mut snd_pcm,
}

#[repr(C)]
pub struct snd_pcm_runtime {
    pub hw: snd_pcm_hardware,
    pub dma_addr: dma_addr_t,
    pub dma_area: *mut u8,
    pub dma_bytes: dma_addr_t,
    pub buffer_size: snd_pcm_uframes_t,
    pub private_data: *mut c_void,
}

#[repr(C)]
pub struct snd_pcm_substream {
    pub stream: c_int,
    pub runtime: *mut snd_pcm_runtime,
}

#[repr(C)]
pub struct snd_pcm_str {
    pub substream: *mut snd_pcm_substream,
}

#[repr(C)]
pub struct snd_card {
    pub dev: *mut device,
}

#[repr(C)]
pub struct snd_pcm {
    pub card: *mut snd_card,
    pub streams: [snd_pcm_str; 2],
}

#[repr(C)]
pub struct snd_soc_component_driver {
    pub open: Option<
        unsafe extern "C" fn(*mut snd_soc_component, *mut snd_pcm_substream) -> c_int,
    >,
    pub close: Option<
        unsafe extern "C" fn(*mut snd_soc_component, *mut snd_pcm_substream) -> c_int,
    >,
    pub hw_params: Option<
        unsafe extern "C" fn(
            *mut snd_soc_component,
            *mut snd_pcm_substream,
            *mut snd_pcm_hw_params,
        ) -> c_int,
    >,
    pub hw_free: Option<
        unsafe extern "C" fn(*mut snd_soc_component, *mut snd_pcm_substream) -> c_int,
    >,
    pub prepare: Option<
        unsafe extern "C" fn(*mut snd_soc_component, *mut snd_pcm_substream) -> c_int,
    >,
    pub trigger: Option<
        unsafe extern "C" fn(*mut snd_soc_component, *mut snd_pcm_substream, c_int) -> c_int,
    >,
    pub pointer: Option<
        unsafe extern "C" fn(*mut snd_soc_component, *mut snd_pcm_substream) -> snd_pcm_uframes_t,
    >,
    pub pcm_new:
        Option<unsafe extern "C" fn(*mut snd_soc_component, *mut snd_soc_pcm_runtime) -> c_int>,
}

#[repr(C)]
pub struct bcm_i2s_priv {
    pub regmap_i2s: *mut regmap,
    pub capture_substream: *mut snd_pcm_substream,
    pub play_substream: *mut snd_pcm_substream,
}

#[repr(C)]
struct i2s_dma_desc {
    dma_area: *mut u8,
    dma_addr: dma_addr_t,
    dma_len: c_uint,
}

#[repr(C)]
struct bcm63xx_runtime_data {
    dma_len: c_int,
    dma_addr: dma_addr_t,
    dma_addr_next: dma_addr_t,
}

unsafe extern "C" {
    static SNDRV_PCM_INFO_MMAP: c_uint;
    static SNDRV_PCM_INFO_MMAP_VALID: c_uint;
    static SNDRV_PCM_INFO_INTERLEAVED: c_uint;
    static SNDRV_PCM_INFO_PAUSE: c_uint;
    static SNDRV_PCM_INFO_RESUME: c_uint;
    static SNDRV_PCM_FMTBIT_S32_LE: u64;
    static PAGE_SIZE: usize;

    static SNDRV_PCM_STREAM_PLAYBACK: c_int;
    static SNDRV_PCM_STREAM_CAPTURE: c_int;
    static SNDRV_PCM_TRIGGER_START: c_int;
    static SNDRV_PCM_TRIGGER_STOP: c_int;
    static SNDRV_PCM_TRIGGER_SUSPEND: c_int;
    static SNDRV_PCM_TRIGGER_PAUSE_PUSH: c_int;
    static SNDRV_PCM_HW_PARAM_PERIOD_BYTES: c_int;
    static SNDRV_PCM_HW_PARAM_BUFFER_BYTES: c_int;
    static SNDRV_PCM_HW_PARAM_PERIODS: c_int;
    static SNDRV_DMA_TYPE_DEV_WC: c_int;

    static I2S_TX_IRQ_EN: c_uint;
    static I2S_TX_DESC_OFF_INTR_EN: c_uint;
    static I2S_TX_CFG: c_uint;
    static I2S_TX_ENABLE_MASK: c_uint;
    static I2S_TX_ENABLE: c_uint;
    static I2S_RX_IRQ_EN: c_uint;
    static I2S_RX_DESC_OFF_INTR_EN_MSK: c_uint;
    static I2S_RX_DESC_OFF_INTR_EN: c_uint;
    static I2S_RX_CFG: c_uint;
    static I2S_RX_ENABLE_MASK: c_uint;
    static I2S_RX_ENABLE: c_uint;
    static I2S_TX_DESC_IFF_LEN: c_uint;
    static I2S_TX_DESC_IFF_ADDR: c_uint;
    static I2S_RX_DESC_IFF_LEN: c_uint;
    static I2S_RX_DESC_IFF_ADDR: c_uint;
    static I2S_RX_IRQ_CTL: c_uint;
    static I2S_RX_DESC_OFF_LEVEL_MASK: c_uint;
    static I2S_RX_DESC_OFF_LEVEL_SHIFT: c_uint;
    static I2S_RX_DESC_OFF_ADDR: c_uint;
    static I2S_RX_DESC_OFF_LEN: c_uint;
    static I2S_RX_DESC_IFF_LEVEL_MASK: c_uint;
    static I2S_RX_DESC_IFF_LEVEL_SHIFT: c_uint;
    static I2S_DESC_FIFO_DEPTH: c_uint;
    static I2S_RX_INTR_MASK: c_uint;
    static I2S_TX_IRQ_CTL: c_uint;
    static I2S_TX_DESC_OFF_INTR_EN_MSK: c_uint;
    static I2S_TX_DESC_OFF_LEVEL_MASK: c_uint;
    static I2S_TX_DESC_OFF_LEVEL_SHIFT: c_uint;
    static I2S_TX_DESC_OFF_ADDR: c_uint;
    static I2S_TX_DESC_OFF_LEN: c_uint;
    static I2S_TX_DESC_IFF_LEVEL_MASK: c_uint;
    static I2S_TX_DESC_IFF_LEVEL_SHIFT: c_uint;
    static I2S_TX_INTR_MASK: c_uint;
    static DMA_BIT_MASK_32: u64;
    static IRQ_HANDLED: irqreturn_t;
    static GFP_NOWAIT: c_uint;
    static ENOMEM: c_int;
    static EINVAL: c_int;

    fn kzalloc(size: usize, flags: c_uint) -> *mut c_void;
    fn kfree(ptr: *mut c_void);
    fn snd_soc_substream_to_rtd(substream: *mut snd_pcm_substream) -> *mut snd_soc_pcm_runtime;
    fn snd_soc_rtd_to_cpu(rtd: *mut snd_soc_pcm_runtime, num: c_int) -> *mut snd_soc_dai;
    fn snd_soc_dai_set_dma_data(
        dai: *mut snd_soc_dai,
        substream: *mut snd_pcm_substream,
        data: *mut c_void,
    );
    fn snd_soc_dai_get_dma_data(
        dai: *mut snd_soc_dai,
        substream: *mut snd_pcm_substream,
    ) -> *mut c_void;
    fn dev_get_drvdata(dev: *mut device) -> *mut c_void;
    fn regmap_update_bits(
        map: *mut regmap,
        reg: c_uint,
        mask: c_uint,
        val: c_uint,
    ) -> c_int;
    fn regmap_write(map: *mut regmap, reg: c_uint, val: dma_addr_t) -> c_int;
    fn regmap_read(map: *mut regmap, reg: c_uint, val: *mut c_uint) -> c_int;
    fn snd_pcm_lib_period_bytes(substream: *mut snd_pcm_substream) -> c_uint;
    fn bytes_to_frames(runtime: *mut snd_pcm_runtime, bytes: dma_addr_t) -> snd_pcm_uframes_t;
    fn snd_pcm_hw_constraint_step(
        runtime: *mut snd_pcm_runtime,
        cond: c_uint,
        var: c_int,
        step: c_uint,
    ) -> c_int;
    fn snd_pcm_hw_constraint_integer(runtime: *mut snd_pcm_runtime, var: c_int) -> c_int;
    fn snd_pcm_period_elapsed(substream: *mut snd_pcm_substream);
    fn of_dma_configure(dev: *mut device, np: *mut c_void, force_dma: c_int) -> c_int;
    fn dma_coerce_mask_and_coherent(dev: *mut device, mask: u64) -> c_int;
    fn snd_pcm_set_fixed_buffer_all(
        pcm: *mut snd_pcm,
        ty: c_int,
        dev: *mut device,
        size: usize,
    ) -> c_int;
    fn platform_get_irq(pdev: *mut platform_device, num: c_uint) -> c_int;
    fn devm_request_irq(
        dev: *mut device,
        irq: c_int,
        handler: unsafe extern "C" fn(c_int, *mut c_void) -> irqreturn_t,
        flags: c_uint,
        name: *const c_char,
        dev_id: *mut c_void,
    ) -> c_int;
    fn irq_get_trigger_type(irq: c_int) -> c_uint;
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn devm_snd_soc_register_component(
        dev: *mut device,
        cmpnt_drv: *const snd_soc_component_driver,
        dai_drv: *mut c_void,
        num_dai: c_int,
    ) -> c_int;
}

unsafe fn kzalloc_obj<T>(flags: c_uint) -> *mut T {
    unsafe { kzalloc(core::mem::size_of::<T>(), flags) as *mut T }
}

static mut bcm63xx_pcm_hardware: snd_pcm_hardware = snd_pcm_hardware {
    info: unsafe {
        SNDRV_PCM_INFO_MMAP
            | SNDRV_PCM_INFO_MMAP_VALID
            | SNDRV_PCM_INFO_INTERLEAVED
            | SNDRV_PCM_INFO_PAUSE
            | SNDRV_PCM_INFO_RESUME
    },
    formats: unsafe { SNDRV_PCM_FMTBIT_S32_LE }, /* support S32 only */
    period_bytes_max: 8192 - 32,
    periods_min: 1,
    periods_max: unsafe { (PAGE_SIZE / core::mem::size_of::<i2s_dma_desc>()) as c_uint },
    buffer_bytes_max: 128 * 1024,
    fifo_size: 32,
};

unsafe extern "C" fn bcm63xx_pcm_hw_params(
    _component: *mut snd_soc_component,
    substream: *mut snd_pcm_substream,
    _params: *mut snd_pcm_hw_params,
) -> c_int {
    let dma_desc: *mut i2s_dma_desc;
    let rtd: *mut snd_soc_pcm_runtime = unsafe { snd_soc_substream_to_rtd(substream) };

    dma_desc = unsafe { kzalloc_obj::<i2s_dma_desc>(GFP_NOWAIT) };
    if dma_desc.is_null() {
        return unsafe { -ENOMEM };
    }

    unsafe {
        snd_soc_dai_set_dma_data(
            snd_soc_rtd_to_cpu(rtd, 0),
            substream,
            dma_desc as *mut c_void,
        );
    }

    0
}

unsafe extern "C" fn bcm63xx_pcm_hw_free(
    _component: *mut snd_soc_component,
    substream: *mut snd_pcm_substream,
) -> c_int {
    let dma_desc: *mut i2s_dma_desc;
    let rtd: *mut snd_soc_pcm_runtime = unsafe { snd_soc_substream_to_rtd(substream) };

    dma_desc = unsafe {
        snd_soc_dai_get_dma_data(snd_soc_rtd_to_cpu(rtd, 0), substream) as *mut i2s_dma_desc
    };
    unsafe { kfree(dma_desc as *mut c_void) };

    0
}

unsafe extern "C" fn bcm63xx_pcm_trigger(
    _component: *mut snd_soc_component,
    substream: *mut snd_pcm_substream,
    cmd: c_int,
) -> c_int {
    let mut ret: c_int = 0;
    let rtd: *mut snd_soc_pcm_runtime;
    let i2s_priv: *mut bcm_i2s_priv;
    let regmap_i2s: *mut regmap;

    rtd = unsafe { snd_soc_substream_to_rtd(substream) };
    i2s_priv =
        unsafe { dev_get_drvdata((*snd_soc_rtd_to_cpu(rtd, 0)).dev) as *mut bcm_i2s_priv };
    regmap_i2s = unsafe { (*i2s_priv).regmap_i2s };

    if unsafe { (*substream).stream == SNDRV_PCM_STREAM_PLAYBACK } {
        if cmd == unsafe { SNDRV_PCM_TRIGGER_START } {
            unsafe {
                regmap_update_bits(
                    regmap_i2s,
                    I2S_TX_IRQ_EN,
                    I2S_TX_DESC_OFF_INTR_EN,
                    I2S_TX_DESC_OFF_INTR_EN,
                );
                regmap_update_bits(
                    regmap_i2s,
                    I2S_TX_CFG,
                    I2S_TX_ENABLE_MASK,
                    I2S_TX_ENABLE,
                );
            }
        } else if cmd == unsafe { SNDRV_PCM_TRIGGER_STOP }
            || cmd == unsafe { SNDRV_PCM_TRIGGER_SUSPEND }
            || cmd == unsafe { SNDRV_PCM_TRIGGER_PAUSE_PUSH }
        {
            unsafe {
                regmap_write(regmap_i2s, I2S_TX_IRQ_EN, 0);
                regmap_update_bits(regmap_i2s, I2S_TX_CFG, I2S_TX_ENABLE_MASK, 0);
            }
        } else {
            ret = unsafe { -EINVAL };
        }
    } else if cmd == unsafe { SNDRV_PCM_TRIGGER_START } {
        unsafe {
            regmap_update_bits(
                regmap_i2s,
                I2S_RX_IRQ_EN,
                I2S_RX_DESC_OFF_INTR_EN_MSK,
                I2S_RX_DESC_OFF_INTR_EN,
            );
            regmap_update_bits(regmap_i2s, I2S_RX_CFG, I2S_RX_ENABLE_MASK, I2S_RX_ENABLE);
        }
    } else if cmd == unsafe { SNDRV_PCM_TRIGGER_STOP }
        || cmd == unsafe { SNDRV_PCM_TRIGGER_SUSPEND }
        || cmd == unsafe { SNDRV_PCM_TRIGGER_PAUSE_PUSH }
    {
        unsafe {
            regmap_update_bits(regmap_i2s, I2S_RX_IRQ_EN, I2S_RX_DESC_OFF_INTR_EN_MSK, 0);
            regmap_update_bits(regmap_i2s, I2S_RX_CFG, I2S_RX_ENABLE_MASK, 0);
        }
    } else {
        ret = unsafe { -EINVAL };
    }
    ret
}

unsafe extern "C" fn bcm63xx_pcm_prepare(
    _component: *mut snd_soc_component,
    substream: *mut snd_pcm_substream,
) -> c_int {
    let dma_desc: *mut i2s_dma_desc;
    let regmap_i2s: *mut regmap;
    let i2s_priv: *mut bcm_i2s_priv;
    let rtd: *mut snd_soc_pcm_runtime = unsafe { snd_soc_substream_to_rtd(substream) };
    let runtime: *mut snd_pcm_runtime = unsafe { (*substream).runtime };
    let regaddr_desclen: c_uint;
    let regaddr_descaddr: c_uint;

    dma_desc = unsafe {
        snd_soc_dai_get_dma_data(snd_soc_rtd_to_cpu(rtd, 0), substream) as *mut i2s_dma_desc
    };
    unsafe {
        (*dma_desc).dma_len = snd_pcm_lib_period_bytes(substream);
        (*dma_desc).dma_addr = (*runtime).dma_addr;
        (*dma_desc).dma_area = (*runtime).dma_area;
    }

    if unsafe { (*substream).stream == SNDRV_PCM_STREAM_PLAYBACK } {
        regaddr_desclen = unsafe { I2S_TX_DESC_IFF_LEN };
        regaddr_descaddr = unsafe { I2S_TX_DESC_IFF_ADDR };
    } else {
        regaddr_desclen = unsafe { I2S_RX_DESC_IFF_LEN };
        regaddr_descaddr = unsafe { I2S_RX_DESC_IFF_ADDR };
    }

    i2s_priv =
        unsafe { dev_get_drvdata((*snd_soc_rtd_to_cpu(rtd, 0)).dev) as *mut bcm_i2s_priv };
    regmap_i2s = unsafe { (*i2s_priv).regmap_i2s };

    unsafe {
        regmap_write(regmap_i2s, regaddr_desclen, (*dma_desc).dma_len as dma_addr_t);
        regmap_write(regmap_i2s, regaddr_descaddr, (*dma_desc).dma_addr);
    }

    0
}

unsafe extern "C" fn bcm63xx_pcm_pointer(
    _component: *mut snd_soc_component,
    substream: *mut snd_pcm_substream,
) -> snd_pcm_uframes_t {
    let x: snd_pcm_uframes_t;
    let runtime = unsafe { (*substream).runtime };
    let prtd: *mut bcm63xx_runtime_data =
        unsafe { (*runtime).private_data as *mut bcm63xx_runtime_data };

    if unsafe { (*prtd).dma_addr_next == 0 } {
        unsafe {
            (*prtd).dma_addr_next = (*runtime).dma_addr;
        }
    }

    x = unsafe { bytes_to_frames(runtime, (*prtd).dma_addr_next - (*runtime).dma_addr) };

    if x == unsafe { (*runtime).buffer_size } {
        0
    } else {
        x
    }
}

unsafe extern "C" fn bcm63xx_pcm_open(
    _component: *mut snd_soc_component,
    substream: *mut snd_pcm_substream,
) -> c_int {
    let mut ret: c_int = 0;
    let runtime: *mut snd_pcm_runtime = unsafe { (*substream).runtime };
    let prtd: *mut bcm63xx_runtime_data;

    unsafe {
        (*runtime).hw = bcm63xx_pcm_hardware;
    }
    ret = unsafe { snd_pcm_hw_constraint_step(runtime, 0, SNDRV_PCM_HW_PARAM_PERIOD_BYTES, 32) };
    if ret != 0 {
        return ret;
    }

    ret = unsafe { snd_pcm_hw_constraint_step(runtime, 0, SNDRV_PCM_HW_PARAM_BUFFER_BYTES, 32) };
    if ret != 0 {
        return ret;
    }

    ret = unsafe { snd_pcm_hw_constraint_integer(runtime, SNDRV_PCM_HW_PARAM_PERIODS) };
    if ret < 0 {
        return ret;
    }

    ret = unsafe { -ENOMEM };
    prtd = unsafe { kzalloc_obj::<bcm63xx_runtime_data>(0) };
    if prtd.is_null() {
        return ret;
    }

    unsafe {
        (*runtime).private_data = prtd as *mut c_void;
    }
    0
}

unsafe extern "C" fn bcm63xx_pcm_close(
    _component: *mut snd_soc_component,
    substream: *mut snd_pcm_substream,
) -> c_int {
    let runtime: *mut snd_pcm_runtime = unsafe { (*substream).runtime };
    let prtd: *mut bcm63xx_runtime_data =
        unsafe { (*runtime).private_data as *mut bcm63xx_runtime_data };

    unsafe { kfree(prtd as *mut c_void) };
    0
}

unsafe extern "C" fn i2s_dma_isr(_irq: c_int, bcm_i2s_priv: *mut c_void) -> irqreturn_t {
    let mut availdepth: c_uint;
    let mut ifflevel: c_uint;
    let mut offlevel: c_uint;
    let mut int_status: c_uint = 0;
    let mut val_1: c_uint = 0;
    let mut val_2: c_uint = 0;
    let mut prtd: *mut bcm63xx_runtime_data;
    let mut substream: *mut snd_pcm_substream;
    let mut runtime: *mut snd_pcm_runtime;
    let regmap_i2s: *mut regmap;
    let mut dma_desc: *mut i2s_dma_desc;
    let mut rtd: *mut snd_soc_pcm_runtime;
    let i2s_priv: *mut bcm_i2s_priv;

    i2s_priv = bcm_i2s_priv as *mut bcm_i2s_priv;
    regmap_i2s = unsafe { (*i2s_priv).regmap_i2s };

    /* rx */
    unsafe {
        regmap_read(regmap_i2s, I2S_RX_IRQ_CTL, &mut int_status);
    }

    if unsafe { int_status & I2S_RX_DESC_OFF_INTR_EN_MSK } != 0 {
        substream = unsafe { (*i2s_priv).capture_substream };
        runtime = unsafe { (*substream).runtime };
        rtd = unsafe { snd_soc_substream_to_rtd(substream) };
        prtd = unsafe { (*runtime).private_data as *mut bcm63xx_runtime_data };
        dma_desc = unsafe {
            snd_soc_dai_get_dma_data(snd_soc_rtd_to_cpu(rtd, 0), substream) as *mut i2s_dma_desc
        };

        offlevel = unsafe {
            (int_status & I2S_RX_DESC_OFF_LEVEL_MASK) >> I2S_RX_DESC_OFF_LEVEL_SHIFT
        };
        let mut val_read = false;
        while offlevel != 0 {
            unsafe {
                regmap_read(regmap_i2s, I2S_RX_DESC_OFF_ADDR, &mut val_1);
                regmap_read(regmap_i2s, I2S_RX_DESC_OFF_LEN, &mut val_2);
            }
            val_read = true;
            offlevel -= 1;
        }
        if val_read {
            unsafe {
                (*prtd).dma_addr_next = val_1.wrapping_add(val_2) as dma_addr_t;
            }
        }

        ifflevel = unsafe {
            (int_status & I2S_RX_DESC_IFF_LEVEL_MASK) >> I2S_RX_DESC_IFF_LEVEL_SHIFT
        };

        availdepth = unsafe { I2S_DESC_FIFO_DEPTH - ifflevel };
        while availdepth != 0 {
            unsafe {
                (*dma_desc).dma_addr += snd_pcm_lib_period_bytes(substream) as dma_addr_t;
                (*dma_desc).dma_area =
                    (*dma_desc).dma_area.add(snd_pcm_lib_period_bytes(substream) as usize);
                if (*dma_desc).dma_addr - (*runtime).dma_addr >= (*runtime).dma_bytes {
                    (*dma_desc).dma_addr = (*runtime).dma_addr;
                    (*dma_desc).dma_area = (*runtime).dma_area;
                }

                (*prtd).dma_addr = (*dma_desc).dma_addr;
                regmap_write(
                    regmap_i2s,
                    I2S_RX_DESC_IFF_LEN,
                    snd_pcm_lib_period_bytes(substream) as dma_addr_t,
                );
                regmap_write(regmap_i2s, I2S_RX_DESC_IFF_ADDR, (*dma_desc).dma_addr);
            }
            availdepth -= 1;
        }

        unsafe {
            snd_pcm_period_elapsed(substream);
        }

        /* Clear interrupt by writing 0 */
        unsafe {
            regmap_update_bits(regmap_i2s, I2S_RX_IRQ_CTL, I2S_RX_INTR_MASK, 0);
        }
    }

    /* tx */
    unsafe {
        regmap_read(regmap_i2s, I2S_TX_IRQ_CTL, &mut int_status);
    }

    if unsafe { int_status & I2S_TX_DESC_OFF_INTR_EN_MSK } != 0 {
        substream = unsafe { (*i2s_priv).play_substream };
        runtime = unsafe { (*substream).runtime };
        rtd = unsafe { snd_soc_substream_to_rtd(substream) };
        prtd = unsafe { (*runtime).private_data as *mut bcm63xx_runtime_data };
        dma_desc = unsafe {
            snd_soc_dai_get_dma_data(snd_soc_rtd_to_cpu(rtd, 0), substream) as *mut i2s_dma_desc
        };

        offlevel = unsafe {
            (int_status & I2S_TX_DESC_OFF_LEVEL_MASK) >> I2S_TX_DESC_OFF_LEVEL_SHIFT
        };
        while offlevel != 0 {
            unsafe {
                regmap_read(regmap_i2s, I2S_TX_DESC_OFF_ADDR, &mut val_1);
                regmap_read(regmap_i2s, I2S_TX_DESC_OFF_LEN, &mut val_2);
                (*prtd).dma_addr_next = val_1.wrapping_add(val_2) as dma_addr_t;
            }
            offlevel -= 1;
        }

        ifflevel = unsafe {
            (int_status & I2S_TX_DESC_IFF_LEVEL_MASK) >> I2S_TX_DESC_IFF_LEVEL_SHIFT
        };
        availdepth = unsafe { I2S_DESC_FIFO_DEPTH - ifflevel };

        while availdepth != 0 {
            unsafe {
                (*dma_desc).dma_addr += snd_pcm_lib_period_bytes(substream) as dma_addr_t;
                (*dma_desc).dma_area =
                    (*dma_desc).dma_area.add(snd_pcm_lib_period_bytes(substream) as usize);

                if (*dma_desc).dma_addr - (*runtime).dma_addr >= (*runtime).dma_bytes {
                    (*dma_desc).dma_addr = (*runtime).dma_addr;
                    (*dma_desc).dma_area = (*runtime).dma_area;
                }

                (*prtd).dma_addr = (*dma_desc).dma_addr;
                regmap_write(
                    regmap_i2s,
                    I2S_TX_DESC_IFF_LEN,
                    snd_pcm_lib_period_bytes(substream) as dma_addr_t,
                );
                regmap_write(regmap_i2s, I2S_TX_DESC_IFF_ADDR, (*dma_desc).dma_addr);
            }
            availdepth -= 1;
        }

        unsafe {
            snd_pcm_period_elapsed(substream);
        }

        /* Clear interrupt by writing 0 */
        unsafe {
            regmap_update_bits(regmap_i2s, I2S_TX_IRQ_CTL, I2S_TX_INTR_MASK, 0);
        }
    }

    unsafe { IRQ_HANDLED }
}

unsafe extern "C" fn bcm63xx_soc_pcm_new(
    _component: *mut snd_soc_component,
    rtd: *mut snd_soc_pcm_runtime,
) -> c_int {
    let pcm: *mut snd_pcm = unsafe { (*rtd).pcm };
    let i2s_priv: *mut bcm_i2s_priv;
    let mut ret: c_int;

    i2s_priv =
        unsafe { dev_get_drvdata((*snd_soc_rtd_to_cpu(rtd, 0)).dev) as *mut bcm_i2s_priv };

    ret = unsafe { of_dma_configure((*(*pcm).card).dev, (*(*(*pcm).card).dev).of_node, 1) };
    if ret != 0 {
        return ret;
    }

    ret = unsafe { dma_coerce_mask_and_coherent((*(*pcm).card).dev, DMA_BIT_MASK_32) };
    if ret != 0 {
        return ret;
    }

    if unsafe { !(*pcm).streams[SNDRV_PCM_STREAM_PLAYBACK as usize].substream.is_null() } {
        unsafe {
            (*i2s_priv).play_substream =
                (*pcm).streams[SNDRV_PCM_STREAM_PLAYBACK as usize].substream;
        }
    }
    if unsafe { !(*pcm).streams[SNDRV_PCM_STREAM_CAPTURE as usize].substream.is_null() } {
        unsafe {
            (*i2s_priv).capture_substream =
                (*pcm).streams[SNDRV_PCM_STREAM_CAPTURE as usize].substream;
        }
    }

    unsafe {
        snd_pcm_set_fixed_buffer_all(
            pcm,
            SNDRV_DMA_TYPE_DEV_WC,
            (*(*pcm).card).dev,
            bcm63xx_pcm_hardware.buffer_bytes_max,
        )
    }
}

static bcm63xx_soc_platform: snd_soc_component_driver = snd_soc_component_driver {
    open: Some(bcm63xx_pcm_open),
    close: Some(bcm63xx_pcm_close),
    hw_params: Some(bcm63xx_pcm_hw_params),
    hw_free: Some(bcm63xx_pcm_hw_free),
    prepare: Some(bcm63xx_pcm_prepare),
    trigger: Some(bcm63xx_pcm_trigger),
    pointer: Some(bcm63xx_pcm_pointer),
    pcm_new: Some(bcm63xx_soc_pcm_new),
};

#[unsafe(no_mangle)]
pub unsafe extern "C" fn bcm63xx_soc_platform_probe(
    pdev: *mut platform_device,
    i2s_priv: *mut bcm_i2s_priv,
) -> c_int {
    let mut ret: c_int;

    ret = unsafe { platform_get_irq(pdev, 0) };
    if ret < 0 {
        return ret;
    }

    ret = unsafe {
        devm_request_irq(
            &mut (*pdev).dev,
            ret,
            i2s_dma_isr,
            irq_get_trigger_type(ret),
            b"i2s_dma\0".as_ptr() as *const c_char,
            i2s_priv as *mut c_void,
        )
    };
    if ret != 0 {
        unsafe {
            dev_err(
                &mut (*pdev).dev,
                b"i2s_init: failed to request interrupt.ret=%d\n\0".as_ptr() as *const c_char,
                ret,
            );
        }
        return ret;
    }

    unsafe {
        devm_snd_soc_register_component(
            &mut (*pdev).dev,
            &bcm63xx_soc_platform,
            core::ptr::null_mut(),
            0,
        )
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn bcm63xx_soc_platform_remove(_pdev: *mut platform_device) -> c_int {
    0
}

// MODULE_AUTHOR("Kevin,Li <kevin-ke.li@broadcom.com>");
// MODULE_DESCRIPTION("Broadcom DSL XPON ASOC PCM Interface");
// MODULE_LICENSE("GPL v2");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
