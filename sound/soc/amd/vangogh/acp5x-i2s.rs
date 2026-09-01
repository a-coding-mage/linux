// SPDX-License-Identifier: GPL-2.0+
//
// AMD ALSA SoC PCM Driver
//
// Copyright (C) 2021 Advanced Micro Devices, Inc. All rights reserved.

// C dependencies: linux/platform_device.h, linux/module.h, linux/err.h,
// linux/io.h, sound/pcm_params.h, sound/soc.h, sound/soc-dai.h,
// linux/dma-mapping.h, and "acp5x.h".

const DRV_NAME: &[u8] = b"acp5x_i2s_playcap\0";

type u16 = core::ffi::c_ushort;
type u32 = core::ffi::c_uint;
type u64 = core::ffi::c_ulonglong;

extern "C" {
    static SND_SOC_DAIFMT_FORMAT_MASK: core::ffi::c_uint;
    static SND_SOC_DAIFMT_I2S: core::ffi::c_uint;
    static SND_SOC_DAIFMT_DSP_A: core::ffi::c_uint;
    static SND_SOC_DAIFMT_CLOCK_PROVIDER_MASK: core::ffi::c_uint;
    static SND_SOC_DAIFMT_BP_FP: core::ffi::c_uint;
    static SND_SOC_DAIFMT_BC_FC: core::ffi::c_uint;
    static TDM_DISABLE: core::ffi::c_int;
    static TDM_ENABLE: core::ffi::c_int;
    static I2S_MASTER_MODE_ENABLE: core::ffi::c_int;
    static I2S_MASTER_MODE_DISABLE: core::ffi::c_int;
    static EINVAL: core::ffi::c_int;
    static ENOMEM: core::ffi::c_int;
    static SLOT_WIDTH_8: core::ffi::c_int;
    static SLOT_WIDTH_16: core::ffi::c_int;
    static SLOT_WIDTH_24: core::ffi::c_int;
    static SLOT_WIDTH_32: core::ffi::c_int;
    static FRM_LEN: u32;
    static SNDRV_PCM_STREAM_PLAYBACK: core::ffi::c_int;
    static SNDRV_PCM_FORMAT_U8: core::ffi::c_int;
    static SNDRV_PCM_FORMAT_S8: core::ffi::c_int;
    static SNDRV_PCM_FORMAT_S16_LE: core::ffi::c_int;
    static SNDRV_PCM_FORMAT_S24_LE: core::ffi::c_int;
    static SNDRV_PCM_FORMAT_S32_LE: core::ffi::c_int;
    static I2S_HS_INSTANCE: core::ffi::c_int;
    static I2S_SP_INSTANCE: core::ffi::c_int;
    static ACP_HSTDM_ITER: u32;
    static ACP_HSTDM_TXFRMT: u32;
    static ACP_I2STDM_ITER: u32;
    static ACP_I2STDM_TXFRMT: u32;
    static ACP_HSTDM_IRER: u32;
    static ACP_HSTDM_RXFRMT: u32;
    static ACP_I2STDM_IRER: u32;
    static ACP_I2STDM_RXFRMT: u32;
    static ACP5x_ITER_IRER_SAMP_LEN_MASK: u32;
    static SNDRV_PCM_TRIGGER_START: core::ffi::c_int;
    static SNDRV_PCM_TRIGGER_RESUME: core::ffi::c_int;
    static SNDRV_PCM_TRIGGER_PAUSE_RELEASE: core::ffi::c_int;
    static SNDRV_PCM_TRIGGER_STOP: core::ffi::c_int;
    static SNDRV_PCM_TRIGGER_SUSPEND: core::ffi::c_int;
    static SNDRV_PCM_TRIGGER_PAUSE_PUSH: core::ffi::c_int;
    static ACP_HS_TX_INTR_WATERMARK_SIZE: u32;
    static ACP_HSTDM_IER: u32;
    static ACP_HS_TX_RINGBUFSIZE: u32;
    static ACP_I2S_TX_INTR_WATERMARK_SIZE: u32;
    static ACP_I2STDM_IER: u32;
    static ACP_I2S_TX_RINGBUFSIZE: u32;
    static ACP_HS_RX_INTR_WATERMARK_SIZE: u32;
    static ACP_HS_RX_RINGBUFSIZE: u32;
    static ACP_I2S_RX_INTR_WATERMARK_SIZE: u32;
    static ACP_I2S_RX_RINGBUFSIZE: u32;
    static SND_SOC_POSSIBLE_DAIFMT_I2S: u64;
    static SND_SOC_POSSIBLE_DAIFMT_DSP_A: u64;
    static SNDRV_PCM_RATE_8000_96000: u32;
    static SNDRV_PCM_FMTBIT_S16_LE: u64;
    static SNDRV_PCM_FMTBIT_S8: u64;
    static SNDRV_PCM_FMTBIT_U8: u64;
    static SNDRV_PCM_FMTBIT_S32_LE: u64;
    static GFP_KERNEL: u32;
    static IORESOURCE_MEM: u32;

    fn snd_soc_dai_get_drvdata(cpu_dai: *mut snd_soc_dai) -> *mut core::ffi::c_void;
    fn snd_soc_substream_to_rtd(substream: *mut snd_pcm_substream) -> *mut snd_soc_pcm_runtime;
    fn snd_soc_card_get_drvdata(card: *mut snd_soc_card) -> *mut core::ffi::c_void;
    fn params_format(params: *mut snd_pcm_hw_params) -> core::ffi::c_int;
    fn params_rate(params: *mut snd_pcm_hw_params) -> core::ffi::c_uint;
    fn acp_readl(addr: *mut core::ffi::c_void) -> u32;
    fn acp_writel(val: u32, addr: *mut core::ffi::c_void);
    fn frames_to_bytes(runtime: *mut snd_pcm_runtime, frames: u64) -> u32;
    fn acp_get_byte_count(rtd: *mut i2s_stream_instance, stream: core::ffi::c_int) -> u64;
    fn acp5x_set_i2s_clk(adata: *mut i2s_dev_data, rtd: *mut i2s_stream_instance);
    fn devm_kzalloc(dev: *mut device, size: usize, flags: u32) -> *mut core::ffi::c_void;
    fn platform_get_resource(
        pdev: *mut platform_device,
        ty: u32,
        num: core::ffi::c_uint,
    ) -> *mut resource;
    fn dev_err(dev: *mut device, fmt: *const core::ffi::c_char, ...);
    fn resource_size(res: *mut resource) -> usize;
    fn devm_ioremap(dev: *mut device, offset: u64, size: usize) -> *mut core::ffi::c_void;
    fn dev_set_drvdata(dev: *mut device, data: *mut core::ffi::c_void);
    fn devm_snd_soc_register_component(
        dev: *mut device,
        cmpnt_drv: *const snd_soc_component_driver,
        dai_drv: *mut snd_soc_dai_driver,
        num_dai: core::ffi::c_int,
    ) -> core::ffi::c_int;
}

#[repr(C)]
struct snd_soc_dai {
    _private: [u8; 0],
}

#[repr(C)]
struct snd_pcm_hw_params {
    _private: [u8; 0],
}

#[repr(C)]
struct snd_soc_card {
    _private: [u8; 0],
}

#[repr(C)]
struct device {
    _private: [u8; 0],
}

#[repr(C)]
struct snd_pcm_runtime {
    private_data: *mut core::ffi::c_void,
    period_size: u64,
    buffer_size: u64,
}

#[repr(C)]
struct snd_pcm_substream {
    runtime: *mut snd_pcm_runtime,
    stream: core::ffi::c_int,
}

#[repr(C)]
struct snd_soc_pcm_runtime {
    card: *mut snd_soc_card,
}

#[repr(C)]
struct acp5x_platform_info {
    play_i2s_instance: core::ffi::c_int,
    cap_i2s_instance: core::ffi::c_int,
}

#[repr(C)]
struct i2s_dev_data {
    acp5x_base: *mut core::ffi::c_void,
    tdm_mode: core::ffi::c_int,
    master_mode: core::ffi::c_int,
    tdm_fmt: u32,
}

#[repr(C)]
struct i2s_stream_instance {
    acp5x_base: *mut core::ffi::c_void,
    i2s_instance: core::ffi::c_int,
    xfer_resolution: u32,
    lrclk_div: u32,
    bclk_div: u32,
    bytescount: u64,
}

#[repr(C)]
struct snd_soc_dai_ops {
    hw_params: Option<
        unsafe extern "C" fn(
            *mut snd_pcm_substream,
            *mut snd_pcm_hw_params,
            *mut snd_soc_dai,
        ) -> core::ffi::c_int,
    >,
    trigger:
        Option<unsafe extern "C" fn(*mut snd_pcm_substream, core::ffi::c_int, *mut snd_soc_dai) -> core::ffi::c_int>,
    set_fmt: Option<unsafe extern "C" fn(*mut snd_soc_dai, core::ffi::c_uint) -> core::ffi::c_int>,
    set_tdm_slot: Option<
        unsafe extern "C" fn(
            *mut snd_soc_dai,
            u32,
            u32,
            core::ffi::c_int,
            core::ffi::c_int,
        ) -> core::ffi::c_int,
    >,
    auto_selectable_formats: *const u64,
    num_auto_selectable_formats: core::ffi::c_int,
}

#[repr(C)]
struct snd_soc_component_driver {
    name: *const core::ffi::c_char,
    legacy_dai_naming: core::ffi::c_int,
}

#[repr(C)]
struct snd_soc_pcm_stream {
    rates: u32,
    formats: u64,
    channels_min: core::ffi::c_uint,
    channels_max: core::ffi::c_uint,
    rate_min: core::ffi::c_uint,
    rate_max: core::ffi::c_uint,
}

#[repr(C)]
struct snd_soc_dai_driver {
    playback: snd_soc_pcm_stream,
    capture: snd_soc_pcm_stream,
    ops: *const snd_soc_dai_ops,
}

#[repr(C)]
struct resource {
    start: u64,
}

#[repr(C)]
struct platform_device {
    dev: device,
}

#[repr(C)]
struct driver {
    name: *const core::ffi::c_char,
}

#[repr(C)]
struct platform_driver {
    probe: Option<unsafe extern "C" fn(*mut platform_device) -> core::ffi::c_int>,
    driver: driver,
}

const fn BIT(nr: u32) -> u32 {
    1u32 << nr
}

unsafe fn ptr_add(base: *mut core::ffi::c_void, offset: u32) -> *mut core::ffi::c_void {
    (base as *mut u8).add(offset as usize) as *mut core::ffi::c_void
}

unsafe extern "C" fn acp5x_i2s_set_fmt(
    cpu_dai: *mut snd_soc_dai,
    fmt: core::ffi::c_uint,
) -> core::ffi::c_int {
    let adata: *mut i2s_dev_data;
    let mut mode: core::ffi::c_int;

    adata = snd_soc_dai_get_drvdata(cpu_dai) as *mut i2s_dev_data;
    mode = (fmt & SND_SOC_DAIFMT_FORMAT_MASK) as core::ffi::c_int;
    if mode == SND_SOC_DAIFMT_I2S as core::ffi::c_int {
        (*adata).tdm_mode = TDM_DISABLE;
    } else if mode == SND_SOC_DAIFMT_DSP_A as core::ffi::c_int {
        (*adata).tdm_mode = TDM_ENABLE;
    } else {
        return -EINVAL;
    }
    mode = (fmt & SND_SOC_DAIFMT_CLOCK_PROVIDER_MASK) as core::ffi::c_int;
    if mode == SND_SOC_DAIFMT_BP_FP as core::ffi::c_int {
        (*adata).master_mode = I2S_MASTER_MODE_ENABLE;
    } else if mode == SND_SOC_DAIFMT_BC_FC as core::ffi::c_int {
        (*adata).master_mode = I2S_MASTER_MODE_DISABLE;
    }
    0
}

unsafe extern "C" fn acp5x_i2s_set_tdm_slot(
    cpu_dai: *mut snd_soc_dai,
    _tx_mask: u32,
    _rx_mask: u32,
    slots: core::ffi::c_int,
    slot_width: core::ffi::c_int,
) -> core::ffi::c_int {
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

unsafe extern "C" fn acp5x_i2s_hwparams(
    substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
    dai: *mut snd_soc_dai,
) -> core::ffi::c_int {
    let rtd: *mut i2s_stream_instance;
    let prtd: *mut snd_soc_pcm_runtime;
    let card: *mut snd_soc_card;
    let pinfo: *mut acp5x_platform_info;
    let adata: *mut i2s_dev_data;

    let mut val: u32;
    let reg_val: u32;
    let frmt_reg: u32;
    let mut lrclk_div_val: u32;
    let mut bclk_div_val: u32;

    lrclk_div_val = 0;
    bclk_div_val = 0;
    prtd = snd_soc_substream_to_rtd(substream);
    rtd = (*(*substream).runtime).private_data as *mut i2s_stream_instance;
    card = (*prtd).card;
    adata = snd_soc_dai_get_drvdata(dai) as *mut i2s_dev_data;
    pinfo = snd_soc_card_get_drvdata(card) as *mut acp5x_platform_info;
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
        if (*rtd).i2s_instance == I2S_HS_INSTANCE {
            reg_val = ACP_HSTDM_ITER;
            frmt_reg = ACP_HSTDM_TXFRMT;
        } else {
            reg_val = ACP_I2STDM_ITER;
            frmt_reg = ACP_I2STDM_TXFRMT;
        }
    } else if (*rtd).i2s_instance == I2S_HS_INSTANCE {
        reg_val = ACP_HSTDM_IRER;
        frmt_reg = ACP_HSTDM_RXFRMT;
    } else {
        reg_val = ACP_I2STDM_IRER;
        frmt_reg = ACP_I2STDM_RXFRMT;
    }
    if (*adata).tdm_mode != 0 {
        val = acp_readl(ptr_add((*rtd).acp5x_base, reg_val));
        acp_writel(val | 0x2, ptr_add((*rtd).acp5x_base, reg_val));
        acp_writel((*adata).tdm_fmt, ptr_add((*rtd).acp5x_base, frmt_reg));
    }
    val = acp_readl(ptr_add((*rtd).acp5x_base, reg_val));
    val &= !ACP5x_ITER_IRER_SAMP_LEN_MASK;
    val = val | ((*rtd).xfer_resolution << 3);
    acp_writel(val, ptr_add((*rtd).acp5x_base, reg_val));

    if (*adata).master_mode != 0 {
        let format = params_format(params);
        if format == SNDRV_PCM_FORMAT_S16_LE {
            match params_rate(params) {
                8000 => bclk_div_val = 768,
                16000 => bclk_div_val = 384,
                24000 => bclk_div_val = 256,
                32000 => bclk_div_val = 192,
                44100 | 48000 => bclk_div_val = 128,
                88200 | 96000 => bclk_div_val = 64,
                192000 => bclk_div_val = 32,
                _ => return -EINVAL,
            }
            lrclk_div_val = 32;
        } else if format == SNDRV_PCM_FORMAT_S32_LE {
            match params_rate(params) {
                8000 => bclk_div_val = 384,
                16000 => bclk_div_val = 192,
                24000 => bclk_div_val = 128,
                32000 => bclk_div_val = 96,
                44100 | 48000 => bclk_div_val = 64,
                88200 | 96000 => bclk_div_val = 32,
                192000 => bclk_div_val = 16,
                _ => return -EINVAL,
            }
            lrclk_div_val = 64;
        } else {
            return -EINVAL;
        }
        (*rtd).lrclk_div = lrclk_div_val;
        (*rtd).bclk_div = bclk_div_val;
    }
    0
}

unsafe extern "C" fn acp5x_i2s_trigger(
    substream: *mut snd_pcm_substream,
    cmd: core::ffi::c_int,
    dai: *mut snd_soc_dai,
) -> core::ffi::c_int {
    let rtd: *mut i2s_stream_instance;
    let adata: *mut i2s_dev_data;
    let mut val: u32;
    let period_bytes: u32;
    let reg_val: u32;
    let ier_val: u32;
    let water_val: u32;
    let buf_size: u32;
    let buf_reg: u32;
    let ret: core::ffi::c_int;

    adata = snd_soc_dai_get_drvdata(dai) as *mut i2s_dev_data;
    rtd = (*(*substream).runtime).private_data as *mut i2s_stream_instance;
    period_bytes = frames_to_bytes((*substream).runtime, (*(*substream).runtime).period_size);
    buf_size = frames_to_bytes((*substream).runtime, (*(*substream).runtime).buffer_size);
    if cmd == SNDRV_PCM_TRIGGER_START
        || cmd == SNDRV_PCM_TRIGGER_RESUME
        || cmd == SNDRV_PCM_TRIGGER_PAUSE_RELEASE
    {
        (*rtd).bytescount = acp_get_byte_count(rtd, (*substream).stream);
        if (*substream).stream == SNDRV_PCM_STREAM_PLAYBACK {
            if (*rtd).i2s_instance == I2S_HS_INSTANCE {
                water_val = ACP_HS_TX_INTR_WATERMARK_SIZE;
                reg_val = ACP_HSTDM_ITER;
                ier_val = ACP_HSTDM_IER;
                buf_reg = ACP_HS_TX_RINGBUFSIZE;
            } else {
                water_val = ACP_I2S_TX_INTR_WATERMARK_SIZE;
                reg_val = ACP_I2STDM_ITER;
                ier_val = ACP_I2STDM_IER;
                buf_reg = ACP_I2S_TX_RINGBUFSIZE;
            }
        } else if (*rtd).i2s_instance == I2S_HS_INSTANCE {
            water_val = ACP_HS_RX_INTR_WATERMARK_SIZE;
            reg_val = ACP_HSTDM_IRER;
            ier_val = ACP_HSTDM_IER;
            buf_reg = ACP_HS_RX_RINGBUFSIZE;
        } else {
            water_val = ACP_I2S_RX_INTR_WATERMARK_SIZE;
            reg_val = ACP_I2STDM_IRER;
            ier_val = ACP_I2STDM_IER;
            buf_reg = ACP_I2S_RX_RINGBUFSIZE;
        }
        acp_writel(period_bytes, ptr_add((*rtd).acp5x_base, water_val));
        acp_writel(buf_size, ptr_add((*rtd).acp5x_base, buf_reg));
        if (*adata).master_mode != 0 {
            acp5x_set_i2s_clk(adata, rtd);
        }
        val = acp_readl(ptr_add((*rtd).acp5x_base, reg_val));
        val = val | BIT(0);
        acp_writel(val, ptr_add((*rtd).acp5x_base, reg_val));
        acp_writel(1, ptr_add((*rtd).acp5x_base, ier_val));
        ret = 0;
    } else if cmd == SNDRV_PCM_TRIGGER_STOP
        || cmd == SNDRV_PCM_TRIGGER_SUSPEND
        || cmd == SNDRV_PCM_TRIGGER_PAUSE_PUSH
    {
        if (*substream).stream == SNDRV_PCM_STREAM_PLAYBACK {
            if (*rtd).i2s_instance == I2S_HS_INSTANCE {
                reg_val = ACP_HSTDM_ITER;
            } else {
                reg_val = ACP_I2STDM_ITER;
            }
        } else if (*rtd).i2s_instance == I2S_HS_INSTANCE {
            reg_val = ACP_HSTDM_IRER;
        } else {
            reg_val = ACP_I2STDM_IRER;
        }
        val = acp_readl(ptr_add((*rtd).acp5x_base, reg_val));
        val = val & !BIT(0);
        acp_writel(val, ptr_add((*rtd).acp5x_base, reg_val));

        if (acp_readl(ptr_add((*rtd).acp5x_base, ACP_HSTDM_ITER)) & BIT(0)) == 0
            && (acp_readl(ptr_add((*rtd).acp5x_base, ACP_HSTDM_IRER)) & BIT(0)) == 0
        {
            acp_writel(0, ptr_add((*rtd).acp5x_base, ACP_HSTDM_IER));
        }
        if (acp_readl(ptr_add((*rtd).acp5x_base, ACP_I2STDM_ITER)) & BIT(0)) == 0
            && (acp_readl(ptr_add((*rtd).acp5x_base, ACP_I2STDM_IRER)) & BIT(0)) == 0
        {
            acp_writel(0, ptr_add((*rtd).acp5x_base, ACP_I2STDM_IER));
        }
        ret = 0;
    } else {
        ret = -EINVAL;
    }
    ret
}

static mut acp5x_i2s_selectable_formats: u64 = 0;

static acp5x_i2s_dai_ops: snd_soc_dai_ops = snd_soc_dai_ops {
    hw_params: Some(acp5x_i2s_hwparams),
    trigger: Some(acp5x_i2s_trigger),
    set_fmt: Some(acp5x_i2s_set_fmt),
    set_tdm_slot: Some(acp5x_i2s_set_tdm_slot),
    auto_selectable_formats: unsafe { &acp5x_i2s_selectable_formats as *const u64 },
    num_auto_selectable_formats: 1,
};

static acp5x_dai_component: snd_soc_component_driver = snd_soc_component_driver {
    name: b"acp5x-i2s\0".as_ptr() as *const core::ffi::c_char,
    legacy_dai_naming: 1,
};

static mut acp5x_i2s_dai: snd_soc_dai_driver = snd_soc_dai_driver {
    playback: snd_soc_pcm_stream {
        rates: 0,
        formats: 0,
        channels_min: 2,
        channels_max: 2,
        rate_min: 8000,
        rate_max: 96000,
    },
    capture: snd_soc_pcm_stream {
        rates: 0,
        formats: 0,
        channels_min: 2,
        channels_max: 2,
        rate_min: 8000,
        rate_max: 96000,
    },
    ops: &acp5x_i2s_dai_ops as *const snd_soc_dai_ops,
};

unsafe extern "C" fn acp5x_dai_probe(pdev: *mut platform_device) -> core::ffi::c_int {
    let res: *mut resource;
    let adata: *mut i2s_dev_data;
    let ret: core::ffi::c_int;

    adata = devm_kzalloc(
        &mut (*pdev).dev,
        core::mem::size_of::<i2s_dev_data>(),
        GFP_KERNEL,
    ) as *mut i2s_dev_data;
    if adata.is_null() {
        return -ENOMEM;
    }

    res = platform_get_resource(pdev, IORESOURCE_MEM, 0);
    if res.is_null() {
        dev_err(&mut (*pdev).dev, b"IORESOURCE_MEM FAILED\n\0".as_ptr() as *const core::ffi::c_char);
        return -ENOMEM;
    }
    (*adata).acp5x_base = devm_ioremap(&mut (*pdev).dev, (*res).start, resource_size(res));
    if (*adata).acp5x_base.is_null() {
        return -ENOMEM;
    }

    (*adata).master_mode = I2S_MASTER_MODE_ENABLE;
    dev_set_drvdata(&mut (*pdev).dev, adata as *mut core::ffi::c_void);
    acp5x_i2s_selectable_formats = SND_SOC_POSSIBLE_DAIFMT_I2S | SND_SOC_POSSIBLE_DAIFMT_DSP_A;
    acp5x_i2s_dai.playback.rates = SNDRV_PCM_RATE_8000_96000;
    acp5x_i2s_dai.playback.formats =
        SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S8 | SNDRV_PCM_FMTBIT_U8 | SNDRV_PCM_FMTBIT_S32_LE;
    acp5x_i2s_dai.capture.rates = SNDRV_PCM_RATE_8000_96000;
    acp5x_i2s_dai.capture.formats =
        SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S8 | SNDRV_PCM_FMTBIT_U8 | SNDRV_PCM_FMTBIT_S32_LE;
    ret = devm_snd_soc_register_component(
        &mut (*pdev).dev,
        &acp5x_dai_component,
        &mut acp5x_i2s_dai,
        1,
    );
    if ret != 0 {
        dev_err(
            &mut (*pdev).dev,
            b"Fail to register acp i2s dai\n\0".as_ptr() as *const core::ffi::c_char,
        );
    }
    ret
}

static acp5x_dai_driver: platform_driver = platform_driver {
    probe: Some(acp5x_dai_probe),
    driver: driver {
        name: b"acp5x_i2s_playcap\0".as_ptr() as *const core::ffi::c_char,
    },
};

// module_platform_driver(acp5x_dai_driver);
// MODULE_AUTHOR("Vijendar.Mukunda@amd.com");
// MODULE_DESCRIPTION("AMD ACP5.x CPU DAI Driver");
// MODULE_ALIAS("platform:" DRV_NAME);
// MODULE_LICENSE("GPL v2");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
