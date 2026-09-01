// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (c) 2020, The Linux Foundation. All rights reserved.
 *
 * lpass-sc7180.c -- ALSA SoC platform-machine driver for QTi LPASS
 */

// C dependencies:
// linux/clk.h, linux/device.h, linux/err.h, linux/kernel.h, linux/module.h,
// linux/of.h, linux/platform_device.h, linux/pm.h,
// dt-bindings/sound/sc7180-lpass.h, sound/pcm.h, sound/soc.h,
// lpass-lpaif-reg.h, lpass.h

use core::ffi::{c_char, c_int, c_uint};

extern "C" {
    static asoc_qcom_lpass_cpu_dai_ops: snd_soc_dai_ops;
    static asoc_qcom_lpass_cpu_dai_ops2: snd_soc_dai_ops;
    static asoc_qcom_lpass_hdmi_dai_ops: snd_soc_dai_ops;

    fn find_first_zero_bit(addr: *const c_ulong, size: c_uint) -> c_ulong;
    fn find_next_zero_bit(addr: *const c_ulong, size: c_uint, offset: c_uint) -> c_ulong;
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

static mut sc7180_lpass_cpu_dai_driver: [snd_soc_dai_driver; 3] = [
    snd_soc_dai_driver {
        id: MI2S_PRIMARY,
        name: b"Primary MI2S\0".as_ptr() as *const c_char,
        playback: snd_soc_pcm_stream {
            stream_name: b"Primary Playback\0".as_ptr() as *const c_char,
            formats: SNDRV_PCM_FMTBIT_S16,
            rates: SNDRV_PCM_RATE_48000,
            rate_min: 48000,
            rate_max: 48000,
            channels_min: 2,
            channels_max: 2,
        },
        capture: snd_soc_pcm_stream {
            stream_name: b"Primary Capture\0".as_ptr() as *const c_char,
            formats: SNDRV_PCM_FMTBIT_S16 | SNDRV_PCM_FMTBIT_S32,
            rates: SNDRV_PCM_RATE_48000,
            rate_min: 48000,
            rate_max: 48000,
            channels_min: 2,
            channels_max: 2,
        },
        ops: unsafe { &asoc_qcom_lpass_cpu_dai_ops },
    },
    snd_soc_dai_driver {
        id: MI2S_SECONDARY,
        name: b"Secondary MI2S\0".as_ptr() as *const c_char,
        playback: snd_soc_pcm_stream {
            stream_name: b"Secondary Playback\0".as_ptr() as *const c_char,
            formats: SNDRV_PCM_FMTBIT_S16,
            rates: SNDRV_PCM_RATE_48000,
            rate_min: 48000,
            rate_max: 48000,
            channels_min: 2,
            channels_max: 2,
        },
        ops: unsafe { &asoc_qcom_lpass_cpu_dai_ops2 },
        ..unsafe { core::mem::zeroed() }
    },
    snd_soc_dai_driver {
        id: LPASS_DP_RX,
        name: b"Hdmi\0".as_ptr() as *const c_char,
        playback: snd_soc_pcm_stream {
            stream_name: b"Hdmi Playback\0".as_ptr() as *const c_char,
            formats: SNDRV_PCM_FMTBIT_S24,
            rates: SNDRV_PCM_RATE_48000,
            rate_min: 48000,
            rate_max: 48000,
            channels_min: 2,
            channels_max: 2,
        },
        ops: unsafe { &asoc_qcom_lpass_hdmi_dai_ops },
        ..unsafe { core::mem::zeroed() }
    },
];

unsafe extern "C" fn sc7180_lpass_alloc_dma_channel(
    drvdata: *mut lpass_data,
    direction: c_int,
    dai_id: c_uint,
) -> c_int {
    let v: *const lpass_variant = (*drvdata).variant;
    let mut chan: c_int = 0;

    if dai_id == LPASS_DP_RX {
        if direction == SNDRV_PCM_STREAM_PLAYBACK {
            chan = find_first_zero_bit(&(*drvdata).hdmi_dma_ch_bit_map, (*v).hdmi_rdma_channels)
                as c_int;

            if chan >= (*v).hdmi_rdma_channels as c_int {
                return -EBUSY;
            }
        }
        set_bit(chan, &mut (*drvdata).hdmi_dma_ch_bit_map);
    } else {
        if direction == SNDRV_PCM_STREAM_PLAYBACK {
            chan = find_first_zero_bit(&(*drvdata).dma_ch_bit_map, (*v).rdma_channels) as c_int;

            if chan >= (*v).rdma_channels as c_int {
                return -EBUSY;
            }
        } else {
            chan = find_next_zero_bit(
                &(*drvdata).dma_ch_bit_map,
                (*v).wrdma_channel_start + (*v).wrdma_channels,
                (*v).wrdma_channel_start,
            ) as c_int;

            if chan >= ((*v).wrdma_channel_start + (*v).wrdma_channels) as c_int {
                return -EBUSY;
            }
        }

        set_bit(chan, &mut (*drvdata).dma_ch_bit_map);
    }
    chan
}

unsafe extern "C" fn sc7180_lpass_free_dma_channel(
    drvdata: *mut lpass_data,
    chan: c_int,
    dai_id: c_uint,
) -> c_int {
    if dai_id == LPASS_DP_RX {
        clear_bit(chan, &mut (*drvdata).hdmi_dma_ch_bit_map);
    } else {
        clear_bit(chan, &mut (*drvdata).dma_ch_bit_map);
    }

    0
}

unsafe extern "C" fn sc7180_lpass_init(pdev: *mut platform_device) -> c_int {
    let drvdata: *mut lpass_data = platform_get_drvdata(pdev);
    let variant: *const lpass_variant = (*drvdata).variant;
    let dev: *mut device = &mut (*pdev).dev;
    let mut ret: c_int;
    let mut i: c_int;

    (*drvdata).clks = devm_kcalloc(
        dev,
        (*variant).num_clks as usize,
        core::mem::size_of::<clk_bulk_data>(),
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
        dev_err(dev, b"Failed to get clocks %d\n\0".as_ptr() as *const c_char, ret);
        return ret;
    }

    ret = clk_bulk_prepare_enable((*drvdata).num_clks, (*drvdata).clks);
    if ret != 0 {
        dev_err(dev, b"sc7180 clk_enable failed\n\0".as_ptr() as *const c_char);
        return ret;
    }

    0
}

unsafe extern "C" fn sc7180_lpass_exit(pdev: *mut platform_device) -> c_int {
    let drvdata: *mut lpass_data = platform_get_drvdata(pdev);

    clk_bulk_disable_unprepare((*drvdata).num_clks, (*drvdata).clks);
    0
}

unsafe extern "C" fn sc7180_lpass_dev_resume(dev: *mut device) -> c_int {
    let drvdata: *mut lpass_data = dev_get_drvdata(dev);

    clk_bulk_prepare_enable((*drvdata).num_clks, (*drvdata).clks)
}

unsafe extern "C" fn sc7180_lpass_dev_suspend(dev: *mut device) -> c_int {
    let drvdata: *mut lpass_data = dev_get_drvdata(dev);

    clk_bulk_disable_unprepare((*drvdata).num_clks, (*drvdata).clks);
    0
}

static sc7180_lpass_pm_ops: dev_pm_ops = dev_pm_ops {
    // SYSTEM_SLEEP_PM_OPS(sc7180_lpass_dev_suspend, sc7180_lpass_dev_resume)
    suspend: Some(sc7180_lpass_dev_suspend),
    resume: Some(sc7180_lpass_dev_resume),
};

static sc7180_clk_name: [*const c_char; 3] = [
    b"pcnoc-sway-clk\0".as_ptr() as *const c_char,
    b"audio-core\0".as_ptr() as *const c_char,
    b"pcnoc-mport-clk\0".as_ptr() as *const c_char,
];

static sc7180_dai_osr_clk_names: [*const c_char; 2] = [
    b"mclk0\0".as_ptr() as *const c_char,
    b"null\0".as_ptr() as *const c_char,
];

static sc7180_dai_bit_clk_names: [*const c_char; 2] = [
    b"mi2s-bit-clk0\0".as_ptr() as *const c_char,
    b"mi2s-bit-clk1\0".as_ptr() as *const c_char,
];

static sc7180_data: lpass_variant = lpass_variant {
    i2sctrl_reg_base: 0x1000,
    i2sctrl_reg_stride: 0x1000,
    i2s_ports: 3,
    irq_reg_base: 0x9000,
    irq_reg_stride: 0x1000,
    irq_ports: 3,
    rdma_reg_base: 0xC000,
    rdma_reg_stride: 0x1000,
    rdma_channels: 5,
    hdmi_rdma_reg_base: 0x64000,
    hdmi_rdma_reg_stride: 0x1000,
    hdmi_rdma_channels: 4,
    dmactl_audif_start: 1,
    wrdma_reg_base: 0x18000,
    wrdma_reg_stride: 0x1000,
    wrdma_channel_start: 5,
    wrdma_channels: 4,

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

    clk_name: sc7180_clk_name.as_ptr(),
    num_clks: 3,
    dai_driver: unsafe { sc7180_lpass_cpu_dai_driver.as_ptr() as *mut snd_soc_dai_driver },
    num_dai: sc7180_lpass_cpu_dai_driver.len(),
    dai_osr_clk_names: sc7180_dai_osr_clk_names.as_ptr(),
    dai_bit_clk_names: sc7180_dai_bit_clk_names.as_ptr(),
    init: Some(sc7180_lpass_init),
    exit: Some(sc7180_lpass_exit),
    alloc_dma_channel: Some(sc7180_lpass_alloc_dma_channel),
    free_dma_channel: Some(sc7180_lpass_free_dma_channel),
};

static sc7180_lpass_cpu_device_id: [of_device_id; 2] = [
    of_device_id {
        compatible: b"qcom,sc7180-lpass-cpu\0".as_ptr() as *const c_char,
        data: &sc7180_data as *const lpass_variant as *const core::ffi::c_void,
    },
    of_device_id {
        ..unsafe { core::mem::zeroed() }
    },
];

// MODULE_DEVICE_TABLE(of, sc7180_lpass_cpu_device_id);

static mut sc7180_lpass_cpu_platform_driver: platform_driver = platform_driver {
    driver: device_driver {
        name: b"sc7180-lpass-cpu\0".as_ptr() as *const c_char,
        of_match_table: sc7180_lpass_cpu_device_id.as_ptr(),
        pm: &sc7180_lpass_pm_ops,
    },
    probe: Some(asoc_qcom_lpass_cpu_platform_probe),
    remove: Some(asoc_qcom_lpass_cpu_platform_remove),
    shutdown: Some(asoc_qcom_lpass_cpu_platform_shutdown),
};

// module_platform_driver(sc7180_lpass_cpu_platform_driver);
// MODULE_DESCRIPTION("SC7180 LPASS CPU DRIVER");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
