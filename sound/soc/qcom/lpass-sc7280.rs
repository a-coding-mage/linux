// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (c) 2020-2021, The Linux Foundation. All rights reserved.
 *
 * lpass-sc7180.c -- ALSA SoC platform-machine driver for QTi LPASS
 */

// Dependencies in the original C source:
// linux/module.h, sound/pcm.h, sound/soc.h, linux/pm.h,
// dt-bindings/sound/sc7180-lpass.h, lpass-lpaif-reg.h, lpass.h.

extern "C" {
    static asoc_qcom_lpass_cpu_dai_ops: snd_soc_dai_ops;
    static asoc_qcom_lpass_hdmi_dai_ops: snd_soc_dai_ops;
    static asoc_qcom_lpass_cdc_dma_dai_ops: snd_soc_dai_ops;

    fn find_first_zero_bit(addr: *const c_ulong, size: c_ulong) -> c_ulong;
    fn find_next_zero_bit(addr: *const c_ulong, size: c_ulong, offset: c_ulong) -> c_ulong;
    fn set_bit(nr: c_int, addr: *mut c_ulong);
    fn clear_bit(nr: c_int, addr: *mut c_ulong);

    fn platform_get_drvdata(pdev: *mut platform_device) -> *mut lpass_data;
    fn dev_get_drvdata(dev: *mut device) -> *mut lpass_data;
    fn devm_kcalloc(
        dev: *mut device,
        n: usize,
        size: usize,
        flags: gfp_t,
    ) -> *mut clk_bulk_data;
    fn devm_clk_bulk_get(dev: *mut device, num_clks: c_int, clks: *mut clk_bulk_data) -> c_int;
    fn clk_bulk_prepare_enable(num_clks: c_int, clks: *mut clk_bulk_data) -> c_int;
    fn clk_bulk_disable_unprepare(num_clks: c_int, clks: *mut clk_bulk_data);
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);

    fn asoc_qcom_lpass_cpu_platform_probe(pdev: *mut platform_device) -> c_int;
    fn asoc_qcom_lpass_cpu_platform_remove(pdev: *mut platform_device) -> c_int;
    fn asoc_qcom_lpass_cpu_platform_shutdown(pdev: *mut platform_device);
}

use core::ffi::{c_char, c_int, c_ulong, c_uint, c_void};
use core::mem::size_of;
use core::ptr;

type gfp_t = c_uint;

const GFP_KERNEL: gfp_t = 0;
const ENOMEM: c_int = 12;
const EBUSY: c_int = 16;

static mut SC7280_LPASS_CPU_DAI_DRIVER: [snd_soc_dai_driver; 6] = unsafe {
    [
        snd_soc_dai_driver {
            id: MI2S_PRIMARY,
            name: c"Primary MI2S".as_ptr(),
            playback: snd_soc_pcm_stream {
                stream_name: c"Primary Playback".as_ptr(),
                formats: SNDRV_PCM_FMTBIT_S16,
                rates: SNDRV_PCM_RATE_48000,
                rate_min: 48000,
                rate_max: 48000,
                channels_min: 2,
                channels_max: 2,
            },
            capture: snd_soc_pcm_stream {
                stream_name: c"Primary Capture".as_ptr(),
                formats: SNDRV_PCM_FMTBIT_S16 | SNDRV_PCM_FMTBIT_S32,
                rates: SNDRV_PCM_RATE_48000,
                rate_min: 48000,
                rate_max: 48000,
                channels_min: 2,
                channels_max: 2,
            },
            ops: &asoc_qcom_lpass_cpu_dai_ops,
        },
        snd_soc_dai_driver {
            id: MI2S_SECONDARY,
            name: c"Secondary MI2S".as_ptr(),
            playback: snd_soc_pcm_stream {
                stream_name: c"Secondary MI2S Playback".as_ptr(),
                formats: SNDRV_PCM_FMTBIT_S16,
                rates: SNDRV_PCM_RATE_48000,
                rate_min: 48000,
                rate_max: 48000,
                channels_min: 2,
                channels_max: 2,
            },
            capture: snd_soc_pcm_stream::zeroed(),
            ops: &asoc_qcom_lpass_cpu_dai_ops,
        },
        snd_soc_dai_driver {
            id: LPASS_DP_RX,
            name: c"Hdmi".as_ptr(),
            playback: snd_soc_pcm_stream {
                stream_name: c"DP Playback".as_ptr(),
                formats: SNDRV_PCM_FMTBIT_S24,
                rates: SNDRV_PCM_RATE_48000,
                rate_min: 48000,
                rate_max: 48000,
                channels_min: 2,
                channels_max: 2,
            },
            capture: snd_soc_pcm_stream::zeroed(),
            ops: &asoc_qcom_lpass_hdmi_dai_ops,
        },
        snd_soc_dai_driver {
            id: LPASS_CDC_DMA_RX0,
            name: c"CDC DMA RX".as_ptr(),
            playback: snd_soc_pcm_stream {
                stream_name: c"WCD Playback".as_ptr(),
                formats: SNDRV_PCM_FMTBIT_S16,
                rates: SNDRV_PCM_RATE_48000,
                rate_min: 48000,
                rate_max: 48000,
                channels_min: 2,
                channels_max: 2,
            },
            capture: snd_soc_pcm_stream::zeroed(),
            ops: &asoc_qcom_lpass_cdc_dma_dai_ops,
        },
        snd_soc_dai_driver {
            id: LPASS_CDC_DMA_TX3,
            name: c"CDC DMA TX".as_ptr(),
            playback: snd_soc_pcm_stream::zeroed(),
            capture: snd_soc_pcm_stream {
                stream_name: c"WCD Capture".as_ptr(),
                formats: SNDRV_PCM_FMTBIT_S16,
                rates: SNDRV_PCM_RATE_48000,
                rate_min: 48000,
                rate_max: 48000,
                channels_min: 1,
                channels_max: 1,
            },
            ops: &asoc_qcom_lpass_cdc_dma_dai_ops,
        },
        snd_soc_dai_driver {
            id: LPASS_CDC_DMA_VA_TX0,
            name: c"CDC DMA VA".as_ptr(),
            playback: snd_soc_pcm_stream::zeroed(),
            capture: snd_soc_pcm_stream {
                stream_name: c"DMIC Capture".as_ptr(),
                formats: SNDRV_PCM_FMTBIT_S16,
                rates: SNDRV_PCM_RATE_48000,
                rate_min: 48000,
                rate_max: 48000,
                channels_min: 2,
                channels_max: 4,
            },
            ops: &asoc_qcom_lpass_cdc_dma_dai_ops,
        },
    ]
};

unsafe extern "C" fn sc7280_lpass_alloc_dma_channel(
    drvdata: *mut lpass_data,
    direction: c_int,
    dai_id: c_uint,
) -> c_int {
    let v: *const lpass_variant = (*drvdata).variant;
    let mut chan: c_int = 0;

    match dai_id {
        MI2S_PRIMARY..=MI2S_QUINARY => {
            if direction == SNDRV_PCM_STREAM_PLAYBACK {
                chan = find_first_zero_bit(
                    &(*drvdata).dma_ch_bit_map as *const c_ulong,
                    (*v).rdma_channels as c_ulong,
                ) as c_int;

                if chan >= (*v).rdma_channels {
                    return -EBUSY;
                }
            } else {
                chan = find_next_zero_bit(
                    &(*drvdata).dma_ch_bit_map as *const c_ulong,
                    ((*v).wrdma_channel_start + (*v).wrdma_channels) as c_ulong,
                    (*v).wrdma_channel_start as c_ulong,
                ) as c_int;

                if chan >= (*v).wrdma_channel_start + (*v).wrdma_channels {
                    return -EBUSY;
                }
            }
            set_bit(chan, &mut (*drvdata).dma_ch_bit_map as *mut c_ulong);
        }
        LPASS_DP_RX => {
            chan = find_first_zero_bit(
                &(*drvdata).hdmi_dma_ch_bit_map as *const c_ulong,
                (*v).hdmi_rdma_channels as c_ulong,
            ) as c_int;
            if chan >= (*v).hdmi_rdma_channels {
                return -EBUSY;
            }
            set_bit(chan, &mut (*drvdata).hdmi_dma_ch_bit_map as *mut c_ulong);
        }
        LPASS_CDC_DMA_RX0..=LPASS_CDC_DMA_RX9 => {
            chan = find_first_zero_bit(
                &(*drvdata).rxtx_dma_ch_bit_map as *const c_ulong,
                (*v).rxtx_rdma_channels as c_ulong,
            ) as c_int;
            if chan >= (*v).rxtx_rdma_channels {
                return -EBUSY;
            }
        }
        LPASS_CDC_DMA_TX0..=LPASS_CDC_DMA_TX8 => {
            chan = find_next_zero_bit(
                &(*drvdata).rxtx_dma_ch_bit_map as *const c_ulong,
                ((*v).rxtx_wrdma_channel_start + (*v).rxtx_wrdma_channels) as c_ulong,
                (*v).rxtx_wrdma_channel_start as c_ulong,
            ) as c_int;
            if chan >= (*v).rxtx_wrdma_channel_start + (*v).rxtx_wrdma_channels {
                return -EBUSY;
            }
            set_bit(chan, &mut (*drvdata).rxtx_dma_ch_bit_map as *mut c_ulong);
        }
        LPASS_CDC_DMA_VA_TX0..=LPASS_CDC_DMA_VA_TX8 => {
            chan = find_next_zero_bit(
                &(*drvdata).va_dma_ch_bit_map as *const c_ulong,
                ((*v).va_wrdma_channel_start + (*v).va_wrdma_channels) as c_ulong,
                (*v).va_wrdma_channel_start as c_ulong,
            ) as c_int;
            if chan >= (*v).va_wrdma_channel_start + (*v).va_wrdma_channels {
                return -EBUSY;
            }
            set_bit(chan, &mut (*drvdata).va_dma_ch_bit_map as *mut c_ulong);
        }
        _ => {}
    }

    chan
}

unsafe extern "C" fn sc7280_lpass_free_dma_channel(
    drvdata: *mut lpass_data,
    chan: c_int,
    dai_id: c_uint,
) -> c_int {
    match dai_id {
        MI2S_PRIMARY..=MI2S_QUINARY => {
            clear_bit(chan, &mut (*drvdata).dma_ch_bit_map as *mut c_ulong);
        }
        LPASS_DP_RX => {
            clear_bit(chan, &mut (*drvdata).hdmi_dma_ch_bit_map as *mut c_ulong);
        }
        LPASS_CDC_DMA_RX0..=LPASS_CDC_DMA_RX9 | LPASS_CDC_DMA_TX0..=LPASS_CDC_DMA_TX8 => {
            clear_bit(chan, &mut (*drvdata).rxtx_dma_ch_bit_map as *mut c_ulong);
        }
        LPASS_CDC_DMA_VA_TX0..=LPASS_CDC_DMA_VA_TX8 => {
            clear_bit(chan, &mut (*drvdata).va_dma_ch_bit_map as *mut c_ulong);
        }
        _ => {}
    }

    0
}

unsafe extern "C" fn sc7280_lpass_init(pdev: *mut platform_device) -> c_int {
    let drvdata: *mut lpass_data = platform_get_drvdata(pdev);
    let variant: *const lpass_variant = (*drvdata).variant;
    let dev: *mut device = &mut (*pdev).dev;
    let mut ret: c_int;
    let mut i: c_int;

    (*drvdata).clks = devm_kcalloc(
        dev,
        (*variant).num_clks as usize,
        size_of::<clk_bulk_data>(),
        GFP_KERNEL,
    );
    if (*drvdata).clks.is_null() {
        return -ENOMEM;
    }

    (*drvdata).num_clks = (*variant).num_clks;

    i = 0;
    while i < (*drvdata).num_clks {
        (*(*drvdata).clks.add(i as usize)).id = *(*variant).clk_name.add(i as usize);
        i += 1;
    }

    ret = devm_clk_bulk_get(dev, (*drvdata).num_clks, (*drvdata).clks);
    if ret != 0 {
        dev_err(dev, c"Failed to get clocks %d\n".as_ptr(), ret);
        return ret;
    }

    ret = clk_bulk_prepare_enable((*drvdata).num_clks, (*drvdata).clks);
    if ret != 0 {
        dev_err(dev, c"sc7280 clk_enable failed\n".as_ptr());
        return ret;
    }

    0
}

unsafe extern "C" fn sc7280_lpass_exit(pdev: *mut platform_device) -> c_int {
    let drvdata: *mut lpass_data = platform_get_drvdata(pdev);

    clk_bulk_disable_unprepare((*drvdata).num_clks, (*drvdata).clks);
    0
}

unsafe extern "C" fn sc7280_lpass_dev_resume(dev: *mut device) -> c_int {
    let drvdata: *mut lpass_data = dev_get_drvdata(dev);

    clk_bulk_prepare_enable((*drvdata).num_clks, (*drvdata).clks)
}

unsafe extern "C" fn sc7280_lpass_dev_suspend(dev: *mut device) -> c_int {
    let drvdata: *mut lpass_data = dev_get_drvdata(dev);

    clk_bulk_disable_unprepare((*drvdata).num_clks, (*drvdata).clks);
    0
}

static SC7280_LPASS_PM_OPS: dev_pm_ops = dev_pm_ops {
    // SYSTEM_SLEEP_PM_OPS(sc7280_lpass_dev_suspend, sc7280_lpass_dev_resume)
    suspend: Some(sc7280_lpass_dev_suspend),
    resume: Some(sc7280_lpass_dev_resume),
};

static SC7280_CLK_NAME: [*const c_char; 1] = [c"core_cc_sysnoc_mport_core".as_ptr()];
static SC7280_DAI_OSR_CLK_NAMES: [*const c_char; 2] =
    [c"audio_cc_ext_mclk0".as_ptr(), c"null".as_ptr()];
static SC7280_DAI_BIT_CLK_NAMES: [*const c_char; 2] =
    [c"core_cc_ext_if0_ibit".as_ptr(), c"core_cc_ext_if1_ibit".as_ptr()];

static SC7280_DATA: lpass_variant = unsafe {
    lpass_variant {
        i2sctrl_reg_base: 0x1000,
        i2sctrl_reg_stride: 0x1000,
        i2s_ports: 3,
        irq_reg_base: 0x9000,
        irq_reg_stride: 0x1000,
        irq_ports: 3,
        rdma_reg_base: 0xC000,
        rdma_reg_stride: 0x1000,
        rdma_channels: 5,
        rxtx_rdma_reg_base: 0xC000,
        rxtx_rdma_reg_stride: 0x1000,
        rxtx_rdma_channels: 8,
        hdmi_rdma_reg_base: 0x64000,
        hdmi_rdma_reg_stride: 0x1000,
        hdmi_rdma_channels: 4,
        dmactl_audif_start: 1,
        wrdma_reg_base: 0x18000,
        wrdma_reg_stride: 0x1000,
        wrdma_channel_start: 5,
        wrdma_channels: 4,
        rxtx_irq_reg_base: 0x9000,
        rxtx_irq_reg_stride: 0x1000,
        rxtx_irq_ports: 3,
        rxtx_wrdma_reg_base: 0x18000,
        rxtx_wrdma_reg_stride: 0x1000,
        rxtx_wrdma_channel_start: 5,
        rxtx_wrdma_channels: 6,
        va_wrdma_reg_base: 0x18000,
        va_wrdma_reg_stride: 0x1000,
        va_wrdma_channel_start: 5,
        va_wrdma_channels: 3,
        va_irq_reg_base: 0x9000,
        va_irq_reg_stride: 0x1000,
        va_irq_ports: 3,

        loopback: REG_FIELD_ID(0x1000, 17, 17, 3, 0x1000),
        spken: REG_FIELD_ID(0x1000, 16, 16, 3, 0x1000),
        spkmode: REG_FIELD_ID(0x1000, 11, 15, 3, 0x1000),
        spkmono: REG_FIELD_ID(0x1000, 10, 10, 3, 0x1000),
        micen: REG_FIELD_ID(0x1000, 9, 9, 3, 0x1000),
        micmode: REG_FIELD_ID(0x1000, 4, 8, 3, 0x1000),
        micmono: REG_FIELD_ID(0x1000, 3, 3, 3, 0x1000),
        wssrc: REG_FIELD_ID(0x1000, 2, 2, 3, 0x1000),
        bitwidth: REG_FIELD_ID(0x1000, 0, 1, 3, 0x1000),

        rdma_dyncclk: REG_FIELD_ID(0xC000, 21, 21, 5, 0x1000),
        rdma_bursten: REG_FIELD_ID(0xC000, 20, 20, 5, 0x1000),
        rdma_wpscnt: REG_FIELD_ID(0xC000, 16, 19, 5, 0x1000),
        rdma_intf: REG_FIELD_ID(0xC000, 12, 15, 5, 0x1000),
        rdma_fifowm: REG_FIELD_ID(0xC000, 1, 5, 5, 0x1000),
        rdma_enable: REG_FIELD_ID(0xC000, 0, 0, 5, 0x1000),

        wrdma_dyncclk: REG_FIELD_ID(0x18000, 22, 22, 4, 0x1000),
        wrdma_bursten: REG_FIELD_ID(0x18000, 21, 21, 4, 0x1000),
        wrdma_wpscnt: REG_FIELD_ID(0x18000, 17, 20, 4, 0x1000),
        wrdma_intf: REG_FIELD_ID(0x18000, 12, 16, 4, 0x1000),
        wrdma_fifowm: REG_FIELD_ID(0x18000, 1, 5, 4, 0x1000),
        wrdma_enable: REG_FIELD_ID(0x18000, 0, 0, 4, 0x1000),

        rxtx_rdma_enable: REG_FIELD_ID(0xC000, 0, 0, 7, 0x1000),
        rxtx_rdma_fifowm: REG_FIELD_ID(0xC000, 1, 11, 7, 0x1000),
        rxtx_rdma_intf: REG_FIELD_ID(0xC000, 12, 15, 7, 0x1000),
        rxtx_rdma_wpscnt: REG_FIELD_ID(0xC000, 16, 19, 7, 0x1000),
        rxtx_rdma_bursten: REG_FIELD_ID(0xC000, 20, 20, 7, 0x1000),
        rxtx_rdma_dyncclk: REG_FIELD_ID(0xC000, 21, 21, 7, 0x1000),

        rxtx_rdma_codec_ch: REG_FIELD_ID(0xC050, 0, 7, 7, 0x1000),
        rxtx_rdma_codec_intf: REG_FIELD_ID(0xC050, 16, 19, 7, 0x1000),
        rxtx_rdma_codec_fs_delay: REG_FIELD_ID(0xC050, 21, 24, 7, 0x1000),
        rxtx_rdma_codec_fs_sel: REG_FIELD_ID(0xC050, 25, 27, 7, 0x1000),
        rxtx_rdma_codec_pack: REG_FIELD_ID(0xC050, 29, 29, 5, 0x1000),
        rxtx_rdma_codec_enable: REG_FIELD_ID(0xC050, 30, 30, 7, 0x1000),

        rxtx_wrdma_enable: REG_FIELD_ID(0x18000, 0, 0, 5, 0x1000),
        rxtx_wrdma_fifowm: REG_FIELD_ID(0x18000, 1, 11, 5, 0x1000),
        rxtx_wrdma_intf: REG_FIELD_ID(0x18000, 12, 16, 5, 0x1000),
        rxtx_wrdma_wpscnt: REG_FIELD_ID(0x18000, 17, 20, 5, 0x1000),
        rxtx_wrdma_bursten: REG_FIELD_ID(0x18000, 21, 21, 5, 0x1000),
        rxtx_wrdma_dyncclk: REG_FIELD_ID(0x18000, 22, 22, 5, 0x1000),

        rxtx_wrdma_codec_ch: REG_FIELD_ID(0x18050, 0, 7, 5, 0x1000),
        rxtx_wrdma_codec_intf: REG_FIELD_ID(0x18050, 16, 19, 5, 0x1000),
        rxtx_wrdma_codec_fs_delay: REG_FIELD_ID(0x18050, 21, 24, 5, 0x1000),
        rxtx_wrdma_codec_fs_sel: REG_FIELD_ID(0x18050, 25, 27, 5, 0x1000),
        rxtx_wrdma_codec_pack: REG_FIELD_ID(0x18050, 29, 29, 5, 0x1000),
        rxtx_wrdma_codec_enable: REG_FIELD_ID(0x18050, 30, 30, 5, 0x1000),

        va_wrdma_enable: REG_FIELD_ID(0x18000, 0, 0, 5, 0x1000),
        va_wrdma_fifowm: REG_FIELD_ID(0x18000, 1, 11, 5, 0x1000),
        va_wrdma_intf: REG_FIELD_ID(0x18000, 12, 16, 5, 0x1000),
        va_wrdma_wpscnt: REG_FIELD_ID(0x18000, 17, 20, 5, 0x1000),
        va_wrdma_bursten: REG_FIELD_ID(0x18000, 21, 21, 5, 0x1000),
        va_wrdma_dyncclk: REG_FIELD_ID(0x18000, 22, 22, 5, 0x1000),

        va_wrdma_codec_ch: REG_FIELD_ID(0x18050, 0, 7, 5, 0x1000),
        va_wrdma_codec_intf: REG_FIELD_ID(0x18050, 16, 19, 5, 0x1000),
        va_wrdma_codec_fs_delay: REG_FIELD_ID(0x18050, 21, 24, 5, 0x1000),
        va_wrdma_codec_fs_sel: REG_FIELD_ID(0x18050, 25, 27, 5, 0x1000),
        va_wrdma_codec_pack: REG_FIELD_ID(0x18050, 29, 29, 5, 0x1000),
        va_wrdma_codec_enable: REG_FIELD_ID(0x18050, 30, 30, 5, 0x1000),

        hdmi_tx_ctl_addr: 0x1000,
        hdmi_legacy_addr: 0x1008,
        hdmi_vbit_addr: 0x610c0,
        hdmi_ch_lsb_addr: 0x61048,
        hdmi_ch_msb_addr: 0x6104c,
        ch_stride: 0x8,
        hdmi_parity_addr: 0x61034,
        hdmi_dmactl_addr: 0x61038,
        hdmi_dma_stride: 0x4,
        hdmi_DP_addr: 0x610c8,
        hdmi_sstream_addr: 0x6101c,
        hdmi_irq_reg_base: 0x63000,
        hdmi_irq_ports: 1,

        hdmi_rdma_dyncclk: REG_FIELD_ID(0x64000, 14, 14, 4, 0x1000),
        hdmi_rdma_bursten: REG_FIELD_ID(0x64000, 13, 13, 4, 0x1000),
        hdmi_rdma_burst8: REG_FIELD_ID(0x64000, 15, 15, 4, 0x1000),
        hdmi_rdma_burst16: REG_FIELD_ID(0x64000, 16, 16, 4, 0x1000),
        hdmi_rdma_dynburst: REG_FIELD_ID(0x64000, 18, 18, 4, 0x1000),
        hdmi_rdma_wpscnt: REG_FIELD_ID(0x64000, 10, 12, 4, 0x1000),
        hdmi_rdma_fifowm: REG_FIELD_ID(0x64000, 1, 5, 4, 0x1000),
        hdmi_rdma_enable: REG_FIELD_ID(0x64000, 0, 0, 4, 0x1000),

        sstream_en: REG_FIELD(0x6101c, 0, 0),
        dma_sel: REG_FIELD(0x6101c, 1, 2),
        auto_bbit_en: REG_FIELD(0x6101c, 3, 3),
        layout: REG_FIELD(0x6101c, 4, 4),
        layout_sp: REG_FIELD(0x6101c, 5, 8),
        set_sp_on_en: REG_FIELD(0x6101c, 10, 10),
        dp_audio: REG_FIELD(0x6101c, 11, 11),
        dp_staffing_en: REG_FIELD(0x6101c, 12, 12),
        dp_sp_b_hw_en: REG_FIELD(0x6101c, 13, 13),

        mute: REG_FIELD(0x610c8, 0, 0),
        as_sdp_cc: REG_FIELD(0x610c8, 1, 3),
        as_sdp_ct: REG_FIELD(0x610c8, 4, 7),
        aif_db4: REG_FIELD(0x610c8, 8, 15),
        frequency: REG_FIELD(0x610c8, 16, 21),
        mst_index: REG_FIELD(0x610c8, 28, 29),
        dptx_index: REG_FIELD(0x610c8, 30, 31),

        soft_reset: REG_FIELD(0x1000, 31, 31),
        force_reset: REG_FIELD(0x1000, 30, 30),

        use_hw_chs: REG_FIELD(0x61038, 0, 0),
        use_hw_usr: REG_FIELD(0x61038, 1, 1),
        hw_chs_sel: REG_FIELD(0x61038, 2, 4),
        hw_usr_sel: REG_FIELD(0x61038, 5, 6),

        replace_vbit: REG_FIELD(0x610c0, 0, 0),
        vbit_stream: REG_FIELD(0x610c0, 1, 1),

        legacy_en: REG_FIELD(0x1008, 0, 0),
        calc_en: REG_FIELD(0x61034, 0, 0),
        lsb_bits: REG_FIELD(0x61048, 0, 31),
        msb_bits: REG_FIELD(0x6104c, 0, 31),

        clk_name: SC7280_CLK_NAME.as_ptr(),
        num_clks: 1,

        dai_driver: SC7280_LPASS_CPU_DAI_DRIVER.as_ptr() as *mut snd_soc_dai_driver,
        num_dai: ARRAY_SIZE_SC7280_LPASS_CPU_DAI_DRIVER,
        dai_osr_clk_names: SC7280_DAI_OSR_CLK_NAMES.as_ptr(),
        dai_bit_clk_names: SC7280_DAI_BIT_CLK_NAMES.as_ptr(),
        init: Some(sc7280_lpass_init),
        exit: Some(sc7280_lpass_exit),
        alloc_dma_channel: Some(sc7280_lpass_alloc_dma_channel),
        free_dma_channel: Some(sc7280_lpass_free_dma_channel),
    }
};

const ARRAY_SIZE_SC7280_LPASS_CPU_DAI_DRIVER: c_int = 6;

static SC7280_LPASS_CPU_DEVICE_ID: [of_device_id; 2] = [
    of_device_id {
        compatible: c"qcom,sc7280-lpass-cpu".as_ptr(),
        data: &SC7280_DATA as *const lpass_variant as *const c_void,
    },
    of_device_id {
        compatible: ptr::null(),
        data: ptr::null(),
    },
];
// MODULE_DEVICE_TABLE(of, sc7280_lpass_cpu_device_id);

static mut SC7280_LPASS_CPU_PLATFORM_DRIVER: platform_driver = platform_driver {
    driver: device_driver {
        name: c"sc7280-lpass-cpu".as_ptr(),
        of_match_table: SC7280_LPASS_CPU_DEVICE_ID.as_ptr(),
        pm: &SC7280_LPASS_PM_OPS,
    },
    probe: Some(asoc_qcom_lpass_cpu_platform_probe),
    remove: Some(asoc_qcom_lpass_cpu_platform_remove),
    shutdown: Some(asoc_qcom_lpass_cpu_platform_shutdown),
};

// module_platform_driver(sc7280_lpass_cpu_platform_driver);
// MODULE_DESCRIPTION("SC7280 LPASS CPU DRIVER");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
