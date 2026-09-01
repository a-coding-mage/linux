// SPDX-License-Identifier: GPL-2.0+
//
// AMD ALSA SoC PCM Driver
//
// Copyright 2016 Advanced Micro Devices, Inc.

// C source included:
// <linux/platform_device.h>, <linux/module.h>, <linux/err.h>, <linux/io.h>,
// <sound/pcm_params.h>, <sound/soc.h>, <sound/soc-dai.h>,
// <linux/dma-mapping.h>, and "acp3x.h".

const DRV_NAME: &[u8] = b"acp3x_i2s_playcap\0";

type u16 = ::core::ffi::c_ushort;
type u32 = ::core::ffi::c_uint;
type u64 = ::core::ffi::c_ulonglong;

extern "C" {
    static SND_SOC_DAIFMT_FORMAT_MASK: ::core::ffi::c_uint;
    static SND_SOC_DAIFMT_I2S: ::core::ffi::c_int;
    static SND_SOC_DAIFMT_DSP_A: ::core::ffi::c_int;
    static TDM_DISABLE: u32;
    static TDM_ENABLE: u32;
    static EINVAL: ::core::ffi::c_int;
    static ENOMEM: ::core::ffi::c_int;
    static ENODEV: ::core::ffi::c_int;
    static SLOT_WIDTH_8: ::core::ffi::c_int;
    static SLOT_WIDTH_16: ::core::ffi::c_int;
    static SLOT_WIDTH_24: ::core::ffi::c_int;
    static SLOT_WIDTH_32: ::core::ffi::c_int;
    static FRM_LEN: u32;
    static SNDRV_PCM_STREAM_PLAYBACK: ::core::ffi::c_int;
    static SNDRV_PCM_FORMAT_U8: ::core::ffi::c_int;
    static SNDRV_PCM_FORMAT_S8: ::core::ffi::c_int;
    static SNDRV_PCM_FORMAT_S16_LE: ::core::ffi::c_int;
    static SNDRV_PCM_FORMAT_S24_LE: ::core::ffi::c_int;
    static SNDRV_PCM_FORMAT_S32_LE: ::core::ffi::c_int;
    static I2S_BT_INSTANCE: u32;
    static I2S_SP_INSTANCE: u32;
    static mmACP_BTTDM_ITER: u32;
    static mmACP_BTTDM_TXFRMT: u32;
    static mmACP_I2STDM_ITER: u32;
    static mmACP_I2STDM_TXFRMT: u32;
    static mmACP_BTTDM_IRER: u32;
    static mmACP_BTTDM_RXFRMT: u32;
    static mmACP_I2STDM_IRER: u32;
    static mmACP_I2STDM_RXFRMT: u32;
    static ACP3x_ITER_IRER_SAMP_LEN_MASK: u32;
    static SNDRV_PCM_TRIGGER_START: ::core::ffi::c_int;
    static SNDRV_PCM_TRIGGER_RESUME: ::core::ffi::c_int;
    static SNDRV_PCM_TRIGGER_PAUSE_RELEASE: ::core::ffi::c_int;
    static SNDRV_PCM_TRIGGER_STOP: ::core::ffi::c_int;
    static SNDRV_PCM_TRIGGER_SUSPEND: ::core::ffi::c_int;
    static SNDRV_PCM_TRIGGER_PAUSE_PUSH: ::core::ffi::c_int;
    static mmACP_BT_TX_INTR_WATERMARK_SIZE: u32;
    static mmACP_BT_TX_RINGBUFSIZE: u32;
    static mmACP_I2S_TX_INTR_WATERMARK_SIZE: u32;
    static mmACP_I2S_TX_RINGBUFSIZE: u32;
    static mmACP_BT_RX_INTR_WATERMARK_SIZE: u32;
    static mmACP_BT_RX_RINGBUFSIZE: u32;
    static mmACP_I2S_RX_INTR_WATERMARK_SIZE: u32;
    static mmACP_I2S_RX_RINGBUFSIZE: u32;
    static mmACP_BTTDM_IER: u32;
    static mmACP_I2STDM_IER: u32;
    static SND_SOC_POSSIBLE_DAIFMT_I2S: u64;
    static SND_SOC_POSSIBLE_DAIFMT_DSP_A: u64;
    static SNDRV_PCM_RATE_8000_96000: u32;
    static SNDRV_PCM_FMTBIT_S16_LE: u64;
    static SNDRV_PCM_FMTBIT_S8: u64;
    static SNDRV_PCM_FMTBIT_U8: u64;
    static SNDRV_PCM_FMTBIT_S32_LE: u64;
    static SNDRV_PCM_RATE_8000_48000: u32;
    static GFP_KERNEL: gfp_t;
    static IORESOURCE_MEM: ::core::ffi::c_uint;

    fn snd_soc_dai_get_drvdata(dai: *mut snd_soc_dai) -> *mut ::core::ffi::c_void;
    fn snd_soc_substream_to_rtd(substream: *mut snd_pcm_substream) -> *mut snd_soc_pcm_runtime;
    fn snd_soc_card_get_drvdata(card: *mut snd_soc_card) -> *mut ::core::ffi::c_void;
    fn params_format(params: *mut snd_pcm_hw_params) -> ::core::ffi::c_int;
    fn rv_readl(addr: *mut ::core::ffi::c_void) -> u32;
    fn rv_writel(val: u32, addr: *mut ::core::ffi::c_void);
    fn frames_to_bytes(runtime: *mut snd_pcm_runtime, frames: snd_pcm_uframes_t) -> u32;
    fn acp_get_byte_count(rtd: *mut i2s_stream_instance, stream: ::core::ffi::c_int) -> u64;
    fn devm_kzalloc(dev: *mut device, size: usize, flags: gfp_t) -> *mut ::core::ffi::c_void;
    fn platform_get_resource(
        pdev: *mut platform_device,
        resource: ::core::ffi::c_uint,
        num: ::core::ffi::c_uint,
    ) -> *mut resource;
    fn devm_ioremap(dev: *mut device, offset: resource_size_t, size: resource_size_t)
        -> *mut ::core::ffi::c_void;
    fn resource_size(res: *mut resource) -> resource_size_t;
    fn dev_err(dev: *mut device, fmt: *const ::core::ffi::c_char, ...);
    fn dev_set_drvdata(dev: *mut device, data: *mut ::core::ffi::c_void);
    fn devm_snd_soc_register_component(
        dev: *mut device,
        component_driver: *const snd_soc_component_driver,
        dai_drv: *mut snd_soc_dai_driver,
        num_dai: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
}

#[repr(C)]
pub struct snd_soc_dai {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_pcm_hw_params {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_card {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_pcm_runtime {
    pub private_data: *mut ::core::ffi::c_void,
    pub period_size: snd_pcm_uframes_t,
    pub buffer_size: snd_pcm_uframes_t,
}

#[repr(C)]
pub struct snd_pcm_substream {
    pub runtime: *mut snd_pcm_runtime,
    pub stream: ::core::ffi::c_int,
}

#[repr(C)]
pub struct snd_soc_pcm_runtime {
    pub card: *mut snd_soc_card,
}

#[repr(C)]
pub struct acp3x_platform_info {
    pub play_i2s_instance: u32,
    pub cap_i2s_instance: u32,
}

#[repr(C)]
pub struct i2s_dev_data {
    pub acp3x_base: *mut ::core::ffi::c_void,
    pub i2s_irq: resource_size_t,
    pub tdm_mode: u32,
    pub tdm_fmt: u32,
}

#[repr(C)]
pub struct i2s_stream_instance {
    pub acp3x_base: *mut ::core::ffi::c_void,
    pub i2s_instance: u32,
    pub xfer_resolution: u32,
    pub bytescount: u64,
}

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct platform_device {
    pub dev: device,
}

#[repr(C)]
pub struct resource {
    pub start: resource_size_t,
}

type gfp_t = ::core::ffi::c_uint;
type resource_size_t = ::core::ffi::c_ulong;
type snd_pcm_uframes_t = ::core::ffi::c_ulong;

type HwParamsFn = unsafe extern "C" fn(
    *mut snd_pcm_substream,
    *mut snd_pcm_hw_params,
    *mut snd_soc_dai,
) -> ::core::ffi::c_int;
type TriggerFn =
    unsafe extern "C" fn(*mut snd_pcm_substream, ::core::ffi::c_int, *mut snd_soc_dai)
        -> ::core::ffi::c_int;
type SetFmtFn = unsafe extern "C" fn(*mut snd_soc_dai, ::core::ffi::c_uint) -> ::core::ffi::c_int;
type SetTdmSlotFn = unsafe extern "C" fn(
    *mut snd_soc_dai,
    u32,
    u32,
    ::core::ffi::c_int,
    ::core::ffi::c_int,
) -> ::core::ffi::c_int;

#[repr(C)]
pub struct snd_soc_dai_ops {
    pub hw_params: Option<HwParamsFn>,
    pub trigger: Option<TriggerFn>,
    pub set_fmt: Option<SetFmtFn>,
    pub set_tdm_slot: Option<SetTdmSlotFn>,
    pub auto_selectable_formats: *const u64,
    pub num_auto_selectable_formats: ::core::ffi::c_int,
}

#[repr(C)]
pub struct snd_soc_component_driver {
    pub name: *const ::core::ffi::c_char,
    pub legacy_dai_naming: ::core::ffi::c_int,
}

#[repr(C)]
pub struct snd_soc_pcm_stream {
    pub rates: u32,
    pub formats: u64,
    pub channels_min: ::core::ffi::c_uint,
    pub channels_max: ::core::ffi::c_uint,
    pub rate_min: ::core::ffi::c_uint,
    pub rate_max: ::core::ffi::c_uint,
}

#[repr(C)]
pub struct snd_soc_dai_driver {
    pub playback: snd_soc_pcm_stream,
    pub capture: snd_soc_pcm_stream,
    pub ops: *const snd_soc_dai_ops,
}

type ProbeFn = unsafe extern "C" fn(*mut platform_device) -> ::core::ffi::c_int;

#[repr(C)]
pub struct device_driver {
    pub name: *const ::core::ffi::c_char,
}

#[repr(C)]
pub struct platform_driver {
    pub probe: Option<ProbeFn>,
    pub driver: device_driver,
}

#[inline]
unsafe fn BIT(nr: u32) -> u32 {
    1u32 << nr
}

#[inline]
unsafe fn acp3x_addr(base: *mut ::core::ffi::c_void, reg: u32) -> *mut ::core::ffi::c_void {
    (base as *mut u8).add(reg as usize) as *mut ::core::ffi::c_void
}

unsafe extern "C" fn acp3x_i2s_set_fmt(
    cpu_dai: *mut snd_soc_dai,
    fmt: ::core::ffi::c_uint,
) -> ::core::ffi::c_int {
    let adata: *mut i2s_dev_data;
    let mode: ::core::ffi::c_int;

    adata = snd_soc_dai_get_drvdata(cpu_dai) as *mut i2s_dev_data;
    mode = (fmt & SND_SOC_DAIFMT_FORMAT_MASK) as ::core::ffi::c_int;
    if mode == SND_SOC_DAIFMT_I2S {
        (*adata).tdm_mode = TDM_DISABLE;
    } else if mode == SND_SOC_DAIFMT_DSP_A {
        (*adata).tdm_mode = TDM_ENABLE;
    } else {
        return -EINVAL;
    }
    0
}

unsafe extern "C" fn acp3x_i2s_set_tdm_slot(
    cpu_dai: *mut snd_soc_dai,
    _tx_mask: u32,
    _rx_mask: u32,
    slots: ::core::ffi::c_int,
    slot_width: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let adata: *mut i2s_dev_data;
    let frm_len: u32;
    let slot_len: u16;

    adata = snd_soc_dai_get_drvdata(cpu_dai) as *mut i2s_dev_data;

    /* These values are as per Hardware Spec */
    if slot_width == SLOT_WIDTH_8 {
        slot_len = 8;
    } else if slot_width == SLOT_WIDTH_16 {
        slot_len = 16;
    } else if slot_width == SLOT_WIDTH_24 {
        slot_len = 24;
    } else if slot_width == SLOT_WIDTH_32 {
        slot_len = 0;
    } else {
        return -EINVAL;
    }
    frm_len = FRM_LEN | ((slots as u32) << 15) | ((slot_len as u32) << 18);
    (*adata).tdm_fmt = frm_len;
    0
}

unsafe extern "C" fn acp3x_i2s_hwparams(
    substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
    dai: *mut snd_soc_dai,
) -> ::core::ffi::c_int {
    let rtd: *mut i2s_stream_instance;
    let prtd: *mut snd_soc_pcm_runtime;
    let card: *mut snd_soc_card;
    let pinfo: *mut acp3x_platform_info;
    let adata: *mut i2s_dev_data;
    let mut val: u32;
    let reg_val: u32;
    let frmt_reg: u32;

    prtd = snd_soc_substream_to_rtd(substream);
    rtd = (*(*substream).runtime).private_data as *mut i2s_stream_instance;
    card = (*prtd).card;
    adata = snd_soc_dai_get_drvdata(dai) as *mut i2s_dev_data;
    pinfo = snd_soc_card_get_drvdata(card) as *mut acp3x_platform_info;
    if !pinfo.is_null() {
        if (*substream).stream == SNDRV_PCM_STREAM_PLAYBACK {
            (*rtd).i2s_instance = (*pinfo).play_i2s_instance;
        } else {
            (*rtd).i2s_instance = (*pinfo).cap_i2s_instance;
        }
    }

    /* These values are as per Hardware Spec */
    let format = params_format(params);
    if format == SNDRV_PCM_FORMAT_U8 || format == SNDRV_PCM_FORMAT_S8 {
        (*rtd).xfer_resolution = 0x0;
    } else if format == SNDRV_PCM_FORMAT_S16_LE {
        (*rtd).xfer_resolution = 0x02;
    } else if format == SNDRV_PCM_FORMAT_S24_LE {
        (*rtd).xfer_resolution = 0x04;
    } else if format == SNDRV_PCM_FORMAT_S32_LE {
        (*rtd).xfer_resolution = 0x05;
    } else {
        return -EINVAL;
    }
    if (*substream).stream == SNDRV_PCM_STREAM_PLAYBACK {
        if (*rtd).i2s_instance == I2S_BT_INSTANCE {
            reg_val = mmACP_BTTDM_ITER;
            frmt_reg = mmACP_BTTDM_TXFRMT;
        } else {
            reg_val = mmACP_I2STDM_ITER;
            frmt_reg = mmACP_I2STDM_TXFRMT;
        }
    } else if (*rtd).i2s_instance == I2S_BT_INSTANCE {
        reg_val = mmACP_BTTDM_IRER;
        frmt_reg = mmACP_BTTDM_RXFRMT;
    } else {
        reg_val = mmACP_I2STDM_IRER;
        frmt_reg = mmACP_I2STDM_RXFRMT;
    }
    if (*adata).tdm_mode != 0 {
        val = rv_readl(acp3x_addr((*rtd).acp3x_base, reg_val));
        rv_writel(val | 0x2, acp3x_addr((*rtd).acp3x_base, reg_val));
        rv_writel((*adata).tdm_fmt, acp3x_addr((*rtd).acp3x_base, frmt_reg));
    }
    val = rv_readl(acp3x_addr((*rtd).acp3x_base, reg_val));
    val &= !ACP3x_ITER_IRER_SAMP_LEN_MASK;
    val = val | ((*rtd).xfer_resolution << 3);
    rv_writel(val, acp3x_addr((*rtd).acp3x_base, reg_val));
    0
}

unsafe extern "C" fn acp3x_i2s_trigger(
    substream: *mut snd_pcm_substream,
    cmd: ::core::ffi::c_int,
    _dai: *mut snd_soc_dai,
) -> ::core::ffi::c_int {
    let rtd: *mut i2s_stream_instance;
    let mut val: u32;
    let period_bytes: u32;
    let reg_val: u32;
    let ier_val: u32;
    let water_val: u32;
    let buf_size: u32;
    let buf_reg: u32;
    let ret: ::core::ffi::c_int;

    rtd = (*(*substream).runtime).private_data as *mut i2s_stream_instance;
    period_bytes = frames_to_bytes((*substream).runtime, (*(*substream).runtime).period_size);
    buf_size = frames_to_bytes((*substream).runtime, (*(*substream).runtime).buffer_size);
    if cmd == SNDRV_PCM_TRIGGER_START
        || cmd == SNDRV_PCM_TRIGGER_RESUME
        || cmd == SNDRV_PCM_TRIGGER_PAUSE_RELEASE
    {
        (*rtd).bytescount = acp_get_byte_count(rtd, (*substream).stream);
        if (*substream).stream == SNDRV_PCM_STREAM_PLAYBACK {
            if (*rtd).i2s_instance == I2S_BT_INSTANCE {
                water_val = mmACP_BT_TX_INTR_WATERMARK_SIZE;
                reg_val = mmACP_BTTDM_ITER;
                ier_val = mmACP_BTTDM_IER;
                buf_reg = mmACP_BT_TX_RINGBUFSIZE;
            } else {
                water_val = mmACP_I2S_TX_INTR_WATERMARK_SIZE;
                reg_val = mmACP_I2STDM_ITER;
                ier_val = mmACP_I2STDM_IER;
                buf_reg = mmACP_I2S_TX_RINGBUFSIZE;
            }
        } else if (*rtd).i2s_instance == I2S_BT_INSTANCE {
            water_val = mmACP_BT_RX_INTR_WATERMARK_SIZE;
            reg_val = mmACP_BTTDM_IRER;
            ier_val = mmACP_BTTDM_IER;
            buf_reg = mmACP_BT_RX_RINGBUFSIZE;
        } else {
            water_val = mmACP_I2S_RX_INTR_WATERMARK_SIZE;
            reg_val = mmACP_I2STDM_IRER;
            ier_val = mmACP_I2STDM_IER;
            buf_reg = mmACP_I2S_RX_RINGBUFSIZE;
        }
        rv_writel(period_bytes, acp3x_addr((*rtd).acp3x_base, water_val));
        rv_writel(buf_size, acp3x_addr((*rtd).acp3x_base, buf_reg));
        val = rv_readl(acp3x_addr((*rtd).acp3x_base, reg_val));
        val = val | BIT(0);
        rv_writel(val, acp3x_addr((*rtd).acp3x_base, reg_val));
        rv_writel(1, acp3x_addr((*rtd).acp3x_base, ier_val));
        ret = 0;
    } else if cmd == SNDRV_PCM_TRIGGER_STOP
        || cmd == SNDRV_PCM_TRIGGER_SUSPEND
        || cmd == SNDRV_PCM_TRIGGER_PAUSE_PUSH
    {
        if (*substream).stream == SNDRV_PCM_STREAM_PLAYBACK {
            if (*rtd).i2s_instance == I2S_BT_INSTANCE {
                reg_val = mmACP_BTTDM_ITER;
            } else {
                reg_val = mmACP_I2STDM_ITER;
            }
        } else if (*rtd).i2s_instance == I2S_BT_INSTANCE {
            reg_val = mmACP_BTTDM_IRER;
        } else {
            reg_val = mmACP_I2STDM_IRER;
        }
        val = rv_readl(acp3x_addr((*rtd).acp3x_base, reg_val));
        val = val & !BIT(0);
        rv_writel(val, acp3x_addr((*rtd).acp3x_base, reg_val));

        if (rv_readl(acp3x_addr((*rtd).acp3x_base, mmACP_BTTDM_ITER)) & BIT(0)) == 0
            && (rv_readl(acp3x_addr((*rtd).acp3x_base, mmACP_BTTDM_IRER)) & BIT(0)) == 0
        {
            rv_writel(0, acp3x_addr((*rtd).acp3x_base, mmACP_BTTDM_IER));
        }
        if (rv_readl(acp3x_addr((*rtd).acp3x_base, mmACP_I2STDM_ITER)) & BIT(0)) == 0
            && (rv_readl(acp3x_addr((*rtd).acp3x_base, mmACP_I2STDM_IRER)) & BIT(0)) == 0
        {
            rv_writel(0, acp3x_addr((*rtd).acp3x_base, mmACP_I2STDM_IER));
        }
        ret = 0;
    } else {
        ret = -EINVAL;
    }

    ret
}

static acp3x_i2s_selectable_formats: u64 =
    unsafe { SND_SOC_POSSIBLE_DAIFMT_I2S | SND_SOC_POSSIBLE_DAIFMT_DSP_A };

static acp3x_i2s_dai_ops: snd_soc_dai_ops = snd_soc_dai_ops {
    hw_params: Some(acp3x_i2s_hwparams),
    trigger: Some(acp3x_i2s_trigger),
    set_fmt: Some(acp3x_i2s_set_fmt),
    set_tdm_slot: Some(acp3x_i2s_set_tdm_slot),
    auto_selectable_formats: unsafe { &acp3x_i2s_selectable_formats as *const u64 },
    num_auto_selectable_formats: 1,
};

static acp3x_dai_component: snd_soc_component_driver = snd_soc_component_driver {
    name: DRV_NAME.as_ptr() as *const ::core::ffi::c_char,
    legacy_dai_naming: 1,
};

static mut acp3x_i2s_dai: snd_soc_dai_driver = snd_soc_dai_driver {
    playback: snd_soc_pcm_stream {
        rates: unsafe { SNDRV_PCM_RATE_8000_96000 },
        formats: unsafe {
            SNDRV_PCM_FMTBIT_S16_LE
                | SNDRV_PCM_FMTBIT_S8
                | SNDRV_PCM_FMTBIT_U8
                | SNDRV_PCM_FMTBIT_S32_LE
        },
        channels_min: 2,
        channels_max: 8,
        rate_min: 8000,
        rate_max: 96000,
    },
    capture: snd_soc_pcm_stream {
        rates: unsafe { SNDRV_PCM_RATE_8000_48000 },
        formats: unsafe {
            SNDRV_PCM_FMTBIT_S16_LE
                | SNDRV_PCM_FMTBIT_S8
                | SNDRV_PCM_FMTBIT_U8
                | SNDRV_PCM_FMTBIT_S32_LE
        },
        channels_min: 2,
        channels_max: 2,
        rate_min: 8000,
        rate_max: 48000,
    },
    ops: unsafe { &acp3x_i2s_dai_ops as *const snd_soc_dai_ops },
};

unsafe extern "C" fn acp3x_dai_probe(pdev: *mut platform_device) -> ::core::ffi::c_int {
    let res: *mut resource;
    let adata: *mut i2s_dev_data;
    let ret: ::core::ffi::c_int;

    adata = devm_kzalloc(
        &mut (*pdev).dev,
        ::core::mem::size_of::<i2s_dev_data>(),
        GFP_KERNEL,
    ) as *mut i2s_dev_data;
    if adata.is_null() {
        return -ENOMEM;
    }

    res = platform_get_resource(pdev, IORESOURCE_MEM, 0);
    if res.is_null() {
        dev_err(
            &mut (*pdev).dev,
            b"IORESOURCE_MEM FAILED\n\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return -ENOMEM;
    }
    (*adata).acp3x_base = devm_ioremap(&mut (*pdev).dev, (*res).start, resource_size(res));
    if (*adata).acp3x_base.is_null() {
        return -ENOMEM;
    }

    (*adata).i2s_irq = (*res).start;
    dev_set_drvdata(&mut (*pdev).dev, adata as *mut ::core::ffi::c_void);
    ret = devm_snd_soc_register_component(
        &mut (*pdev).dev,
        &acp3x_dai_component,
        &mut acp3x_i2s_dai,
        1,
    );
    if ret != 0 {
        dev_err(
            &mut (*pdev).dev,
            b"Fail to register acp i2s dai\n\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return -ENODEV;
    }
    0
}

static mut acp3x_dai_driver: platform_driver = platform_driver {
    probe: Some(acp3x_dai_probe),
    driver: device_driver {
        name: b"acp3x_i2s_playcap\0".as_ptr() as *const ::core::ffi::c_char,
    },
};

// module_platform_driver(acp3x_dai_driver);
// MODULE_AUTHOR("Vishnuvardhanrao.Ravulapati@amd.com");
// MODULE_DESCRIPTION("AMD ACP 3.x PCM Driver");
// MODULE_LICENSE("GPL v2");
// MODULE_ALIAS("platform:" DRV_NAME);

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
