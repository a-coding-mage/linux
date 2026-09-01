// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (c) 2010-2011,2013-2015 The Linux Foundation. All rights reserved.
 *
 * lpass-apq8016.c -- ALSA SoC CPU DAI driver for APQ8016 LPASS
 */

// Dependencies translated from:
// <linux/clk.h>, <linux/device.h>, <linux/err.h>, <linux/kernel.h>,
// <linux/module.h>, <linux/of.h>, <linux/platform_device.h>,
// <sound/pcm.h>, <sound/pcm_params.h>, <sound/soc.h>, <sound/soc-dai.h>,
// <dt-bindings/sound/apq8016-lpass.h>, "lpass-lpaif-reg.h", "lpass.h".

use core::ffi::{c_char, c_int, c_long, c_uint};

static mut apq8016_lpass_cpu_dai_driver: [snd_soc_dai_driver; 4] = [
    snd_soc_dai_driver {
        id: MI2S_PRIMARY,
        name: c"Primary MI2S".as_ptr(),
        playback: snd_soc_pcm_stream {
            stream_name: c"Primary Playback".as_ptr(),
            formats: SNDRV_PCM_FMTBIT_S16 | SNDRV_PCM_FMTBIT_S24 | SNDRV_PCM_FMTBIT_S32,
            rates: SNDRV_PCM_RATE_8000
                | SNDRV_PCM_RATE_16000
                | SNDRV_PCM_RATE_32000
                | SNDRV_PCM_RATE_48000
                | SNDRV_PCM_RATE_96000,
            rate_min: 8000,
            rate_max: 96000,
            channels_min: 1,
            channels_max: 8,
            ..Default::default()
        },
        ops: unsafe { &asoc_qcom_lpass_cpu_dai_ops },
        ..Default::default()
    },
    snd_soc_dai_driver {
        id: MI2S_SECONDARY,
        name: c"Secondary MI2S".as_ptr(),
        playback: snd_soc_pcm_stream {
            stream_name: c"Secondary Playback".as_ptr(),
            formats: SNDRV_PCM_FMTBIT_S16 | SNDRV_PCM_FMTBIT_S24 | SNDRV_PCM_FMTBIT_S32,
            rates: SNDRV_PCM_RATE_8000
                | SNDRV_PCM_RATE_16000
                | SNDRV_PCM_RATE_32000
                | SNDRV_PCM_RATE_48000
                | SNDRV_PCM_RATE_96000,
            rate_min: 8000,
            rate_max: 96000,
            channels_min: 1,
            channels_max: 8,
            ..Default::default()
        },
        ops: unsafe { &asoc_qcom_lpass_cpu_dai_ops },
        ..Default::default()
    },
    snd_soc_dai_driver {
        id: MI2S_TERTIARY,
        name: c"Tertiary MI2S".as_ptr(),
        capture: snd_soc_pcm_stream {
            stream_name: c"Tertiary Capture".as_ptr(),
            formats: SNDRV_PCM_FMTBIT_S16 | SNDRV_PCM_FMTBIT_S24 | SNDRV_PCM_FMTBIT_S32,
            rates: SNDRV_PCM_RATE_8000
                | SNDRV_PCM_RATE_16000
                | SNDRV_PCM_RATE_32000
                | SNDRV_PCM_RATE_48000
                | SNDRV_PCM_RATE_96000,
            rate_min: 8000,
            rate_max: 96000,
            channels_min: 1,
            channels_max: 8,
            ..Default::default()
        },
        ops: unsafe { &asoc_qcom_lpass_cpu_dai_ops },
        ..Default::default()
    },
    snd_soc_dai_driver {
        id: MI2S_QUATERNARY,
        name: c"Quatenary MI2S".as_ptr(),
        playback: snd_soc_pcm_stream {
            stream_name: c"Quatenary Playback".as_ptr(),
            formats: SNDRV_PCM_FMTBIT_S16 | SNDRV_PCM_FMTBIT_S24 | SNDRV_PCM_FMTBIT_S32,
            rates: SNDRV_PCM_RATE_8000
                | SNDRV_PCM_RATE_16000
                | SNDRV_PCM_RATE_32000
                | SNDRV_PCM_RATE_48000
                | SNDRV_PCM_RATE_96000,
            rate_min: 8000,
            rate_max: 96000,
            channels_min: 1,
            channels_max: 8,
            ..Default::default()
        },
        capture: snd_soc_pcm_stream {
            stream_name: c"Quatenary Capture".as_ptr(),
            formats: SNDRV_PCM_FMTBIT_S16 | SNDRV_PCM_FMTBIT_S24 | SNDRV_PCM_FMTBIT_S32,
            rates: SNDRV_PCM_RATE_8000
                | SNDRV_PCM_RATE_16000
                | SNDRV_PCM_RATE_32000
                | SNDRV_PCM_RATE_48000
                | SNDRV_PCM_RATE_96000,
            rate_min: 8000,
            rate_max: 96000,
            channels_min: 1,
            channels_max: 8,
            ..Default::default()
        },
        ops: unsafe { &asoc_qcom_lpass_cpu_dai_ops },
        ..Default::default()
    },
];

unsafe extern "C" fn apq8016_lpass_alloc_dma_channel(
    drvdata: *mut lpass_data,
    direction: c_int,
    dai_id: c_uint,
) -> c_int {
    let v: *const lpass_variant = (*drvdata).variant;
    let mut chan: c_int = 0;

    if direction == SNDRV_PCM_STREAM_PLAYBACK {
        chan = find_first_zero_bit(
            &mut (*drvdata).dma_ch_bit_map,
            (*v).rdma_channels,
        ) as c_int;

        if chan >= (*v).rdma_channels as c_int {
            return -EBUSY;
        }
    } else {
        chan = find_next_zero_bit(
            &mut (*drvdata).dma_ch_bit_map,
            (*v).wrdma_channel_start + (*v).wrdma_channels,
            (*v).wrdma_channel_start,
        ) as c_int;

        if chan >= ((*v).wrdma_channel_start + (*v).wrdma_channels) as c_int {
            return -EBUSY;
        }
    }

    set_bit(chan, &mut (*drvdata).dma_ch_bit_map);

    chan
}

unsafe extern "C" fn apq8016_lpass_free_dma_channel(
    drvdata: *mut lpass_data,
    chan: c_int,
    dai_id: c_uint,
) -> c_int {
    clear_bit(chan, &mut (*drvdata).dma_ch_bit_map);

    0
}

unsafe extern "C" fn apq8016_lpass_init(pdev: *mut platform_device) -> c_int {
    let drvdata: *mut lpass_data = platform_get_drvdata(pdev);
    let variant: *const lpass_variant = (*drvdata).variant;
    let dev: *mut device = &mut (*pdev).dev;
    let mut ret: c_int;
    let mut i: c_int;

    (*drvdata).clks = devm_kcalloc(
        dev,
        (*variant).num_clks,
        core::mem::size_of_val(&*(*drvdata).clks),
        GFP_KERNEL,
    );
    if (*drvdata).clks.is_null() {
        return -ENOMEM;
    }
    (*drvdata).num_clks = (*variant).num_clks;

    i = 0;
    while i < (*drvdata).num_clks as c_int {
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
        dev_err(dev, c"apq8016 clk_enable failed\n".as_ptr());
        return ret;
    }

    (*drvdata).ahbix_clk = devm_clk_get(dev, c"ahbix-clk".as_ptr());
    if IS_ERR((*drvdata).ahbix_clk) {
        dev_err(
            dev,
            c"error getting ahbix-clk: %ld\n".as_ptr(),
            PTR_ERR((*drvdata).ahbix_clk) as c_long,
        );
        ret = PTR_ERR((*drvdata).ahbix_clk) as c_int;
        clk_bulk_disable_unprepare((*drvdata).num_clks, (*drvdata).clks);
        return ret;
    }

    ret = clk_set_rate((*drvdata).ahbix_clk, LPASS_AHBIX_CLOCK_FREQUENCY);
    if ret != 0 {
        dev_err(dev, c"error setting rate on ahbix_clk: %d\n".as_ptr(), ret);
        clk_bulk_disable_unprepare((*drvdata).num_clks, (*drvdata).clks);
        return ret;
    }
    dev_dbg(
        dev,
        c"set ahbix_clk rate to %lu\n".as_ptr(),
        clk_get_rate((*drvdata).ahbix_clk),
    );

    ret = clk_prepare_enable((*drvdata).ahbix_clk);
    if ret != 0 {
        dev_err(dev, c"error enabling ahbix_clk: %d\n".as_ptr(), ret);
        clk_bulk_disable_unprepare((*drvdata).num_clks, (*drvdata).clks);
        return ret;
    }

    0
}

unsafe extern "C" fn apq8016_lpass_exit(pdev: *mut platform_device) -> c_int {
    let drvdata: *mut lpass_data = platform_get_drvdata(pdev);

    clk_bulk_disable_unprepare((*drvdata).num_clks, (*drvdata).clks);
    clk_disable_unprepare((*drvdata).ahbix_clk);

    0
}

static apq8016_clk_name: [*const c_char; 2] = [
    c"pcnoc-mport-clk".as_ptr(),
    c"pcnoc-sway-clk".as_ptr(),
];

static apq8016_dai_osr_clk_names: [*const c_char; 4] = [
    c"mi2s-osr-clk0".as_ptr(),
    c"mi2s-osr-clk1".as_ptr(),
    c"mi2s-osr-clk2".as_ptr(),
    c"mi2s-osr-clk3".as_ptr(),
];

static apq8016_dai_bit_clk_names: [*const c_char; 4] = [
    c"mi2s-bit-clk0".as_ptr(),
    c"mi2s-bit-clk1".as_ptr(),
    c"mi2s-bit-clk2".as_ptr(),
    c"mi2s-bit-clk3".as_ptr(),
];

static apq8016_data: lpass_variant = lpass_variant {
    i2sctrl_reg_base: 0x1000,
    i2sctrl_reg_stride: 0x1000,
    i2s_ports: 4,
    irq_reg_base: 0x6000,
    irq_reg_stride: 0x1000,
    irq_ports: 3,
    rdma_reg_base: 0x8400,
    rdma_reg_stride: 0x1000,
    rdma_channels: 2,
    dmactl_audif_start: 1,
    wrdma_reg_base: 0xB000,
    wrdma_reg_stride: 0x1000,
    wrdma_channel_start: 5,
    wrdma_channels: 2,

    loopback: REG_FIELD_ID!(0x1000, 15, 15, 4, 0x1000),
    spken: REG_FIELD_ID!(0x1000, 14, 14, 4, 0x1000),
    spkmode: REG_FIELD_ID!(0x1000, 10, 13, 4, 0x1000),
    spkmono: REG_FIELD_ID!(0x1000, 9, 9, 4, 0x1000),
    micen: REG_FIELD_ID!(0x1000, 8, 8, 4, 0x1000),
    micmode: REG_FIELD_ID!(0x1000, 4, 7, 4, 0x1000),
    micmono: REG_FIELD_ID!(0x1000, 3, 3, 4, 0x1000),
    wssrc: REG_FIELD_ID!(0x1000, 2, 2, 4, 0x1000),
    bitwidth: REG_FIELD_ID!(0x1000, 0, 1, 4, 0x1000),

    rdma_dyncclk: REG_FIELD_ID!(0x8400, 12, 12, 2, 0x1000),
    rdma_bursten: REG_FIELD_ID!(0x8400, 11, 11, 2, 0x1000),
    rdma_wpscnt: REG_FIELD_ID!(0x8400, 8, 10, 2, 0x1000),
    rdma_intf: REG_FIELD_ID!(0x8400, 4, 7, 2, 0x1000),
    rdma_fifowm: REG_FIELD_ID!(0x8400, 1, 3, 2, 0x1000),
    rdma_enable: REG_FIELD_ID!(0x8400, 0, 0, 2, 0x1000),

    wrdma_dyncclk: REG_FIELD_ID!(0xB000, 12, 12, 2, 0x1000),
    wrdma_bursten: REG_FIELD_ID!(0xB000, 11, 11, 2, 0x1000),
    wrdma_wpscnt: REG_FIELD_ID!(0xB000, 8, 10, 2, 0x1000),
    wrdma_intf: REG_FIELD_ID!(0xB000, 4, 7, 2, 0x1000),
    wrdma_fifowm: REG_FIELD_ID!(0xB000, 1, 3, 2, 0x1000),
    wrdma_enable: REG_FIELD_ID!(0xB000, 0, 0, 2, 0x1000),

    clk_name: apq8016_clk_name.as_ptr(),
    num_clks: 2,
    dai_driver: unsafe { apq8016_lpass_cpu_dai_driver.as_mut_ptr() },
    num_dai: apq8016_lpass_cpu_dai_driver.len(),
    dai_osr_clk_names: apq8016_dai_osr_clk_names.as_ptr(),
    dai_bit_clk_names: apq8016_dai_bit_clk_names.as_ptr(),
    init: Some(apq8016_lpass_init),
    exit: Some(apq8016_lpass_exit),
    alloc_dma_channel: Some(apq8016_lpass_alloc_dma_channel),
    free_dma_channel: Some(apq8016_lpass_free_dma_channel),
    ..Default::default()
};

#[used]
static apq8016_lpass_cpu_device_id: [of_device_id; 3] = [
    of_device_id {
        compatible: c"qcom,lpass-cpu-apq8016".as_ptr(),
        data: &apq8016_data as *const lpass_variant as *const core::ffi::c_void,
        ..Default::default()
    },
    of_device_id {
        compatible: c"qcom,apq8016-lpass-cpu".as_ptr(),
        data: &apq8016_data as *const lpass_variant as *const core::ffi::c_void,
        ..Default::default()
    },
    of_device_id {
        ..Default::default()
    },
];

// MODULE_DEVICE_TABLE(of, apq8016_lpass_cpu_device_id);

static mut apq8016_lpass_cpu_platform_driver: platform_driver = platform_driver {
    driver: device_driver {
        name: c"apq8016-lpass-cpu".as_ptr(),
        of_match_table: of_match_ptr(apq8016_lpass_cpu_device_id.as_ptr()),
        ..Default::default()
    },
    probe: Some(asoc_qcom_lpass_cpu_platform_probe),
    remove: Some(asoc_qcom_lpass_cpu_platform_remove),
    ..Default::default()
};

module_platform_driver!(apq8016_lpass_cpu_platform_driver);

// MODULE_DESCRIPTION("APQ8016 LPASS CPU Driver");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
