// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (c) 2021 The Linux Foundation. All rights reserved.
 *
 * lpass-cdc-dma.c -- ALSA SoC CDC DMA CPU DAI driver for QTi LPASS
 */

// C dependencies:
// dt-bindings/sound/qcom,lpass.h
// linux/clk.h
// linux/module.h
// linux/export.h
// sound/soc.h
// sound/soc-dai.h
// lpass-lpaif-reg.h
// lpass.h

const CODEC_MEM_HZ_NORMAL: core::ffi::c_ulong = 153600000;

#[repr(C)]
enum codec_dma_interfaces {
    LPASS_CDC_DMA_INTERFACE1 = 1,
    LPASS_CDC_DMA_INTERFACE2,
    LPASS_CDC_DMA_INTERFACE3,
    LPASS_CDC_DMA_INTERFACE4,
    LPASS_CDC_DMA_INTERFACE5,
    LPASS_CDC_DMA_INTERFACE6,
    LPASS_CDC_DMA_INTERFACE7,
    LPASS_CDC_DMA_INTERFACE8,
    LPASS_CDC_DMA_INTERFACE9,
    LPASS_CDC_DMA_INTERFACE10,
}

extern "C" {
    static LPASS_CDC_DMA_RX0: core::ffi::c_int;
    static LPASS_CDC_DMA_RX1: core::ffi::c_int;
    static LPASS_CDC_DMA_RX2: core::ffi::c_int;
    static LPASS_CDC_DMA_RX3: core::ffi::c_int;
    static LPASS_CDC_DMA_RX4: core::ffi::c_int;
    static LPASS_CDC_DMA_RX5: core::ffi::c_int;
    static LPASS_CDC_DMA_RX6: core::ffi::c_int;
    static LPASS_CDC_DMA_RX7: core::ffi::c_int;
    static LPASS_CDC_DMA_RX8: core::ffi::c_int;
    static LPASS_CDC_DMA_RX9: core::ffi::c_int;
    static LPASS_CDC_DMA_TX0: core::ffi::c_int;
    static LPASS_CDC_DMA_TX1: core::ffi::c_int;
    static LPASS_CDC_DMA_TX2: core::ffi::c_int;
    static LPASS_CDC_DMA_TX3: core::ffi::c_int;
    static LPASS_CDC_DMA_TX4: core::ffi::c_int;
    static LPASS_CDC_DMA_TX5: core::ffi::c_int;
    static LPASS_CDC_DMA_TX6: core::ffi::c_int;
    static LPASS_CDC_DMA_TX7: core::ffi::c_int;
    static LPASS_CDC_DMA_TX8: core::ffi::c_int;
    static LPASS_CDC_DMA_VA_TX0: core::ffi::c_int;
    static LPASS_CDC_DMA_VA_TX1: core::ffi::c_int;
    static LPASS_CDC_DMA_VA_TX2: core::ffi::c_int;
    static LPASS_CDC_DMA_VA_TX3: core::ffi::c_int;
    static LPASS_CDC_DMA_VA_TX4: core::ffi::c_int;
    static LPASS_CDC_DMA_VA_TX5: core::ffi::c_int;
    static LPASS_CDC_DMA_VA_TX6: core::ffi::c_int;
    static LPASS_CDC_DMA_VA_TX7: core::ffi::c_int;
    static LPASS_CDC_DMA_VA_TX8: core::ffi::c_int;
    static EINVAL: core::ffi::c_int;
    static LPAIF_DMACTL_ENABLE_ON: core::ffi::c_uint;
    static LPAIF_DMACTL_ENABLE_OFF: core::ffi::c_uint;
    static LPASS_CDC_DMA_INTF_ONE_CHANNEL: core::ffi::c_uint;
    static LPASS_CDC_DMA_INTF_TWO_CHANNEL: core::ffi::c_uint;
    static LPASS_CDC_DMA_INTF_FOUR_CHANNEL: core::ffi::c_uint;
    static LPASS_CDC_DMA_INTF_SIX_CHANNEL: core::ffi::c_uint;
    static LPASS_CDC_DMA_INTF_EIGHT_CHANNEL: core::ffi::c_uint;
    static SNDRV_PCM_TRIGGER_START: core::ffi::c_int;
    static SNDRV_PCM_TRIGGER_RESUME: core::ffi::c_int;
    static SNDRV_PCM_TRIGGER_PAUSE_RELEASE: core::ffi::c_int;
    static SNDRV_PCM_TRIGGER_STOP: core::ffi::c_int;
    static SNDRV_PCM_TRIGGER_SUSPEND: core::ffi::c_int;
    static SNDRV_PCM_TRIGGER_PAUSE_PUSH: core::ffi::c_int;
}

#[repr(C)]
pub struct snd_pcm_substream {
    pub runtime: *mut snd_pcm_runtime,
}

#[repr(C)]
pub struct snd_pcm_runtime {
    pub private_data: *mut core::ffi::c_void,
}

#[repr(C)]
pub struct snd_soc_dai {
    pub id: core::ffi::c_int,
    pub driver: *mut snd_soc_dai_driver,
}

#[repr(C)]
pub struct snd_soc_dai_driver {
    pub id: core::ffi::c_uint,
}

#[repr(C)]
pub struct snd_soc_pcm_runtime {
    pub dev: *mut device,
}

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct lpass_data {
    pub rxtx_rd_dmactl: *mut lpaif_dmactl,
    pub rxtx_wr_dmactl: *mut lpaif_dmactl,
    pub va_wr_dmactl: *mut lpaif_dmactl,
    pub variant: *const lpass_variant,
    pub codec_mem0: *mut clk,
    pub va_mem0: *mut clk,
}

#[repr(C)]
pub struct lpass_pcm_data {
    pub dma_ch: core::ffi::c_int,
}

#[repr(C)]
pub struct lpass_variant {
    pub rxtx_wrdma_channel_start: core::ffi::c_int,
    pub va_wrdma_channel_start: core::ffi::c_int,
}

#[repr(C)]
pub struct lpaif_dmactl {
    pub codec_intf: *mut regmap_field,
    pub codec_fs_sel: *mut regmap_field,
    pub codec_fs_delay: *mut regmap_field,
    pub codec_pack: *mut regmap_field,
    pub codec_enable: *mut regmap_field,
    pub codec_channel: *mut regmap_field,
}

#[repr(C)]
pub struct regmap_field {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_pcm_hw_params {
    _private: [u8; 0],
}

#[repr(C)]
pub struct clk {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_dai_ops {
    pub startup: Option<
        unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_soc_dai) -> core::ffi::c_int,
    >,
    pub shutdown: Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_soc_dai)>,
    pub hw_params: Option<
        unsafe extern "C" fn(
            *mut snd_pcm_substream,
            *mut snd_pcm_hw_params,
            *mut snd_soc_dai,
        ) -> core::ffi::c_int,
    >,
    pub trigger: Option<
        unsafe extern "C" fn(
            *mut snd_pcm_substream,
            core::ffi::c_int,
            *mut snd_soc_dai,
        ) -> core::ffi::c_int,
    >,
}

extern "C" {
    fn snd_soc_substream_to_rtd(substream: *mut snd_pcm_substream) -> *mut snd_soc_pcm_runtime;
    fn snd_soc_rtd_to_cpu(
        rtd: *mut snd_soc_pcm_runtime,
        num: core::ffi::c_int,
    ) -> *mut snd_soc_dai;
    fn snd_soc_dai_get_drvdata(dai: *mut snd_soc_dai) -> *mut lpass_data;
    fn dev_err(dev: *mut device, fmt: *const core::ffi::c_char, ...);
    fn regmap_fields_write(
        fields: *mut regmap_field,
        id: core::ffi::c_int,
        val: core::ffi::c_uint,
    ) -> core::ffi::c_int;
    fn clk_set_rate(clk: *mut clk, rate: core::ffi::c_ulong) -> core::ffi::c_int;
    fn clk_prepare_enable(clk: *mut clk) -> core::ffi::c_int;
    fn clk_disable_unprepare(clk: *mut clk);
    fn params_channels(params: *mut snd_pcm_hw_params) -> core::ffi::c_uint;
}

#[inline]
unsafe fn c_range(
    value: core::ffi::c_uint,
    start: core::ffi::c_int,
    end: core::ffi::c_int,
) -> bool {
    value >= start as core::ffi::c_uint && value <= end as core::ffi::c_uint
}

#[inline]
unsafe fn c_range_i(
    value: core::ffi::c_int,
    start: core::ffi::c_int,
    end: core::ffi::c_int,
) -> bool {
    value >= start && value <= end
}

unsafe extern "C" fn __lpass_get_dmactl_handle(
    substream: *mut snd_pcm_substream,
    dai: *mut snd_soc_dai,
    dmactl: *mut *mut lpaif_dmactl,
    id: *mut core::ffi::c_int,
) {
    let soc_runtime = snd_soc_substream_to_rtd(substream);
    let cpu_dai = snd_soc_rtd_to_cpu(soc_runtime, 0);
    let drvdata = snd_soc_dai_get_drvdata(dai);
    let rt = (*substream).runtime;
    let pcm_data = (*rt).private_data as *mut lpass_pcm_data;
    let v = (*drvdata).variant;
    let dai_id = (*(*cpu_dai).driver).id;

    if c_range(dai_id, LPASS_CDC_DMA_RX0, LPASS_CDC_DMA_RX9) {
        *dmactl = (*drvdata).rxtx_rd_dmactl;
        *id = (*pcm_data).dma_ch;
    } else if c_range(dai_id, LPASS_CDC_DMA_TX0, LPASS_CDC_DMA_TX8) {
        *dmactl = (*drvdata).rxtx_wr_dmactl;
        *id = (*pcm_data)
            .dma_ch
            .wrapping_sub((*v).rxtx_wrdma_channel_start);
    } else if c_range(dai_id, LPASS_CDC_DMA_VA_TX0, LPASS_CDC_DMA_VA_TX8) {
        *dmactl = (*drvdata).va_wr_dmactl;
        *id = (*pcm_data)
            .dma_ch
            .wrapping_sub((*v).va_wrdma_channel_start);
    } else {
        dev_err(
            (*soc_runtime).dev,
            b"invalid dai id for dma ctl: %d\n\0".as_ptr() as *const core::ffi::c_char,
            dai_id,
        );
    }
}

unsafe extern "C" fn __lpass_get_codec_dma_intf_type(
    dai_id: core::ffi::c_int,
) -> core::ffi::c_int {
    let ret: core::ffi::c_int;

    if dai_id == LPASS_CDC_DMA_RX0 || dai_id == LPASS_CDC_DMA_TX0 || dai_id == LPASS_CDC_DMA_VA_TX0
    {
        ret = codec_dma_interfaces::LPASS_CDC_DMA_INTERFACE1 as core::ffi::c_int;
    } else if dai_id == LPASS_CDC_DMA_RX1
        || dai_id == LPASS_CDC_DMA_TX1
        || dai_id == LPASS_CDC_DMA_VA_TX1
    {
        ret = codec_dma_interfaces::LPASS_CDC_DMA_INTERFACE2 as core::ffi::c_int;
    } else if dai_id == LPASS_CDC_DMA_RX2
        || dai_id == LPASS_CDC_DMA_TX2
        || dai_id == LPASS_CDC_DMA_VA_TX2
    {
        ret = codec_dma_interfaces::LPASS_CDC_DMA_INTERFACE3 as core::ffi::c_int;
    } else if dai_id == LPASS_CDC_DMA_RX3
        || dai_id == LPASS_CDC_DMA_TX3
        || dai_id == LPASS_CDC_DMA_VA_TX3
    {
        ret = codec_dma_interfaces::LPASS_CDC_DMA_INTERFACE4 as core::ffi::c_int;
    } else if dai_id == LPASS_CDC_DMA_RX4
        || dai_id == LPASS_CDC_DMA_TX4
        || dai_id == LPASS_CDC_DMA_VA_TX4
    {
        ret = codec_dma_interfaces::LPASS_CDC_DMA_INTERFACE5 as core::ffi::c_int;
    } else if dai_id == LPASS_CDC_DMA_RX5
        || dai_id == LPASS_CDC_DMA_TX5
        || dai_id == LPASS_CDC_DMA_VA_TX5
    {
        ret = codec_dma_interfaces::LPASS_CDC_DMA_INTERFACE6 as core::ffi::c_int;
    } else if dai_id == LPASS_CDC_DMA_RX6
        || dai_id == LPASS_CDC_DMA_TX6
        || dai_id == LPASS_CDC_DMA_VA_TX6
    {
        ret = codec_dma_interfaces::LPASS_CDC_DMA_INTERFACE7 as core::ffi::c_int;
    } else if dai_id == LPASS_CDC_DMA_RX7
        || dai_id == LPASS_CDC_DMA_TX7
        || dai_id == LPASS_CDC_DMA_VA_TX7
    {
        ret = codec_dma_interfaces::LPASS_CDC_DMA_INTERFACE8 as core::ffi::c_int;
    } else if dai_id == LPASS_CDC_DMA_RX8
        || dai_id == LPASS_CDC_DMA_TX8
        || dai_id == LPASS_CDC_DMA_VA_TX8
    {
        ret = codec_dma_interfaces::LPASS_CDC_DMA_INTERFACE9 as core::ffi::c_int;
    } else if dai_id == LPASS_CDC_DMA_RX9 {
        ret = codec_dma_interfaces::LPASS_CDC_DMA_INTERFACE10 as core::ffi::c_int;
    } else {
        ret = -EINVAL;
    }
    ret
}

unsafe extern "C" fn __lpass_platform_codec_intf_init(
    dai: *mut snd_soc_dai,
    substream: *mut snd_pcm_substream,
) -> core::ffi::c_int {
    let soc_runtime = snd_soc_substream_to_rtd(substream);
    let cpu_dai = snd_soc_rtd_to_cpu(soc_runtime, 0);
    let mut dmactl: *mut lpaif_dmactl = core::ptr::null_mut();
    let dev = (*soc_runtime).dev;
    let mut ret: core::ffi::c_int;
    let mut id: core::ffi::c_int = 0;
    let codec_intf: core::ffi::c_int;
    let dai_id = (*(*cpu_dai).driver).id;

    codec_intf = __lpass_get_codec_dma_intf_type(dai_id as core::ffi::c_int);
    if codec_intf < 0 {
        dev_err(
            dev,
            b"failed to get codec_intf: %d\n\0".as_ptr() as *const core::ffi::c_char,
            codec_intf,
        );
        return codec_intf;
    }

    __lpass_get_dmactl_handle(substream, dai, &mut dmactl, &mut id);
    if dmactl.is_null() {
        return -EINVAL;
    }

    ret = regmap_fields_write((*dmactl).codec_intf, id, codec_intf as core::ffi::c_uint);
    if ret != 0 {
        dev_err(
            dev,
            b"error writing to dmactl codec_intf reg field: %d\n\0".as_ptr()
                as *const core::ffi::c_char,
            ret,
        );
        return ret;
    }
    ret = regmap_fields_write((*dmactl).codec_fs_sel, id, 0x0);
    if ret != 0 {
        dev_err(
            dev,
            b"error writing to dmactl codec_fs_sel reg field: %d\n\0".as_ptr()
                as *const core::ffi::c_char,
            ret,
        );
        return ret;
    }
    ret = regmap_fields_write((*dmactl).codec_fs_delay, id, 0x0);
    if ret != 0 {
        dev_err(
            dev,
            b"error writing to dmactl codec_fs_delay reg field: %d\n\0".as_ptr()
                as *const core::ffi::c_char,
            ret,
        );
        return ret;
    }
    ret = regmap_fields_write((*dmactl).codec_pack, id, 0x1);
    if ret != 0 {
        dev_err(
            dev,
            b"error writing to dmactl codec_pack reg field: %d\n\0".as_ptr()
                as *const core::ffi::c_char,
            ret,
        );
        return ret;
    }
    ret = regmap_fields_write((*dmactl).codec_enable, id, LPAIF_DMACTL_ENABLE_ON);
    if ret != 0 {
        dev_err(
            dev,
            b"error writing to dmactl codec_enable reg field: %d\n\0".as_ptr()
                as *const core::ffi::c_char,
            ret,
        );
        return ret;
    }
    0
}

unsafe extern "C" fn lpass_cdc_dma_daiops_startup(
    substream: *mut snd_pcm_substream,
    dai: *mut snd_soc_dai,
) -> core::ffi::c_int {
    let drvdata = snd_soc_dai_get_drvdata(dai);
    let soc_runtime = snd_soc_substream_to_rtd(substream);

    if c_range_i((*dai).id, LPASS_CDC_DMA_RX0, LPASS_CDC_DMA_RX9)
        || c_range_i((*dai).id, LPASS_CDC_DMA_TX0, LPASS_CDC_DMA_TX8)
    {
        clk_set_rate((*drvdata).codec_mem0, CODEC_MEM_HZ_NORMAL);
        clk_prepare_enable((*drvdata).codec_mem0);
    } else if c_range_i((*dai).id, LPASS_CDC_DMA_VA_TX0, LPASS_CDC_DMA_VA_TX0) {
        clk_set_rate((*drvdata).va_mem0, CODEC_MEM_HZ_NORMAL);
        clk_prepare_enable((*drvdata).va_mem0);
    } else {
        dev_err(
            (*soc_runtime).dev,
            b"%s: invalid  interface: %d\n\0".as_ptr() as *const core::ffi::c_char,
            b"lpass_cdc_dma_daiops_startup\0".as_ptr() as *const core::ffi::c_char,
            (*dai).id,
        );
    }
    0
}

unsafe extern "C" fn lpass_cdc_dma_daiops_shutdown(
    substream: *mut snd_pcm_substream,
    dai: *mut snd_soc_dai,
) {
    let drvdata = snd_soc_dai_get_drvdata(dai);
    let soc_runtime = snd_soc_substream_to_rtd(substream);

    if c_range_i((*dai).id, LPASS_CDC_DMA_RX0, LPASS_CDC_DMA_RX9)
        || c_range_i((*dai).id, LPASS_CDC_DMA_TX0, LPASS_CDC_DMA_TX8)
    {
        clk_disable_unprepare((*drvdata).codec_mem0);
    } else if c_range_i((*dai).id, LPASS_CDC_DMA_VA_TX0, LPASS_CDC_DMA_VA_TX0) {
        clk_disable_unprepare((*drvdata).va_mem0);
    } else {
        dev_err(
            (*soc_runtime).dev,
            b"%s: invalid  interface: %d\n\0".as_ptr() as *const core::ffi::c_char,
            b"lpass_cdc_dma_daiops_shutdown\0".as_ptr() as *const core::ffi::c_char,
            (*dai).id,
        );
    }
}

unsafe extern "C" fn lpass_cdc_dma_daiops_hw_params(
    substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
    dai: *mut snd_soc_dai,
) -> core::ffi::c_int {
    let soc_runtime = snd_soc_substream_to_rtd(substream);
    let mut dmactl: *mut lpaif_dmactl = core::ptr::null_mut();
    let regval: core::ffi::c_uint;
    let channels = params_channels(params);
    let ret: core::ffi::c_int;
    let mut id: core::ffi::c_int = 0;

    match channels {
        1 => {
            regval = LPASS_CDC_DMA_INTF_ONE_CHANNEL;
        }
        2 => {
            regval = LPASS_CDC_DMA_INTF_TWO_CHANNEL;
        }
        4 => {
            regval = LPASS_CDC_DMA_INTF_FOUR_CHANNEL;
        }
        6 => {
            regval = LPASS_CDC_DMA_INTF_SIX_CHANNEL;
        }
        8 => {
            regval = LPASS_CDC_DMA_INTF_EIGHT_CHANNEL;
        }
        _ => {
            dev_err(
                (*soc_runtime).dev,
                b"invalid PCM config\n\0".as_ptr() as *const core::ffi::c_char,
            );
            return -EINVAL;
        }
    }

    __lpass_get_dmactl_handle(substream, dai, &mut dmactl, &mut id);
    if dmactl.is_null() {
        return -EINVAL;
    }

    ret = regmap_fields_write((*dmactl).codec_channel, id, regval);
    if ret != 0 {
        dev_err(
            (*soc_runtime).dev,
            b"error writing to dmactl codec_channel reg field: %d\n\0".as_ptr()
                as *const core::ffi::c_char,
            ret,
        );
        return ret;
    }
    0
}

unsafe extern "C" fn lpass_cdc_dma_daiops_trigger(
    substream: *mut snd_pcm_substream,
    cmd: core::ffi::c_int,
    dai: *mut snd_soc_dai,
) -> core::ffi::c_int {
    let soc_runtime = snd_soc_substream_to_rtd(substream);
    let mut dmactl: *mut lpaif_dmactl = core::ptr::null_mut();
    let mut ret: core::ffi::c_int = 0;
    let mut id: core::ffi::c_int = 0;

    if cmd == SNDRV_PCM_TRIGGER_START
        || cmd == SNDRV_PCM_TRIGGER_RESUME
        || cmd == SNDRV_PCM_TRIGGER_PAUSE_RELEASE
    {
        __lpass_platform_codec_intf_init(dai, substream);
    } else if cmd == SNDRV_PCM_TRIGGER_STOP
        || cmd == SNDRV_PCM_TRIGGER_SUSPEND
        || cmd == SNDRV_PCM_TRIGGER_PAUSE_PUSH
    {
        __lpass_get_dmactl_handle(substream, dai, &mut dmactl, &mut id);
        if dmactl.is_null() {
            return -EINVAL;
        }

        ret = regmap_fields_write((*dmactl).codec_enable, id, LPAIF_DMACTL_ENABLE_OFF);
        if ret != 0 {
            dev_err(
                (*soc_runtime).dev,
                b"error writing to dmactl codec_enable reg: %d\n\0".as_ptr()
                    as *const core::ffi::c_char,
                ret,
            );
            return ret;
        }
    } else {
        ret = -EINVAL;
        dev_err(
            (*soc_runtime).dev,
            b"%s: invalid %d interface\n\0".as_ptr() as *const core::ffi::c_char,
            b"lpass_cdc_dma_daiops_trigger\0".as_ptr() as *const core::ffi::c_char,
            cmd,
        );
    }
    ret
}

#[no_mangle]
pub static asoc_qcom_lpass_cdc_dma_dai_ops: snd_soc_dai_ops = snd_soc_dai_ops {
    startup: Some(lpass_cdc_dma_daiops_startup),
    shutdown: Some(lpass_cdc_dma_daiops_shutdown),
    hw_params: Some(lpass_cdc_dma_daiops_hw_params),
    trigger: Some(lpass_cdc_dma_daiops_trigger),
};

// EXPORT_SYMBOL_GPL(asoc_qcom_lpass_cdc_dma_dai_ops);

// MODULE_DESCRIPTION("QTi LPASS CDC DMA Driver");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
