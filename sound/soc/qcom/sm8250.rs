// SPDX-License-Identifier: GPL-2.0
// Copyright (c) 2020, Linaro Limited

// C include dependencies:
// dt-bindings/sound/qcom,q6afe.h, linux/module.h, linux/platform_device.h,
// sound/soc.h, sound/soc-dapm.h, sound/pcm.h, sound/pcm_params.h,
// linux/soundwire/sdw.h, sound/jack.h, linux/input-event-codes.h,
// "qdsp6/q6afe.h", "common.h", "usb_offload_utils.h", "sdw.h"

const MI2S_BCLK_RATE: u32 = 1536000;

#[repr(C)]
struct sm8250_snd_data {
    stream_prepared: [bool; AFE_PORT_MAX],
    card: *mut snd_soc_card,
    jack: snd_soc_jack,
    usb_offload_jack: snd_soc_jack,
    usb_offload_jack_setup: bool,
    dp_jack: snd_soc_jack,
    jack_setup: bool,
}

unsafe extern "C" {
    static mut THIS_MODULE: *mut module;
    static snd_sm8250_dt_match: [of_device_id; 8];

    fn snd_soc_card_get_drvdata(card: *mut snd_soc_card) -> *mut c_void;
    fn snd_soc_card_set_drvdata(card: *mut snd_soc_card, data: *mut c_void);
    fn snd_soc_rtd_to_cpu(rtd: *mut snd_soc_pcm_runtime, num: u32) -> *mut snd_soc_dai;
    fn snd_soc_rtd_to_codec(rtd: *mut snd_soc_pcm_runtime, num: u32) -> *mut snd_soc_dai;
    fn snd_soc_substream_to_rtd(substream: *mut snd_pcm_substream) -> *mut snd_soc_pcm_runtime;
    fn snd_soc_dai_set_sysclk(
        dai: *mut snd_soc_dai,
        clk_id: i32,
        freq: u32,
        dir: i32,
    ) -> i32;
    fn snd_soc_dai_set_fmt(dai: *mut snd_soc_dai, fmt: u32) -> i32;
    fn hw_param_interval(params: *mut snd_pcm_hw_params, var: i32) -> *mut snd_interval;
    fn hw_param_mask(params: *mut snd_pcm_hw_params, var: i32) -> *mut snd_mask;
    fn snd_mask_set_format(mask: *mut snd_mask, format: i32);
    fn qcom_snd_dp_jack_setup(
        rtd: *mut snd_soc_pcm_runtime,
        jack: *mut snd_soc_jack,
        hdmi_pcm_id: i32,
    ) -> i32;
    fn qcom_snd_usb_offload_jack_setup(
        rtd: *mut snd_soc_pcm_runtime,
        jack: *mut snd_soc_jack,
        jack_setup: *mut bool,
    ) -> i32;
    fn qcom_snd_usb_offload_jack_remove(
        rtd: *mut snd_soc_pcm_runtime,
        jack_setup: *mut bool,
    );
    fn qcom_snd_wcd_jack_setup(
        rtd: *mut snd_soc_pcm_runtime,
        jack: *mut snd_soc_jack,
        jack_setup: *mut bool,
    ) -> i32;
    fn qcom_snd_sdw_startup(substream: *mut snd_pcm_substream) -> i32;
    fn qcom_snd_sdw_shutdown(substream: *mut snd_pcm_substream);
    fn qcom_snd_sdw_prepare(substream: *mut snd_pcm_substream, prepared: *mut bool) -> i32;
    fn qcom_snd_sdw_hw_free(substream: *mut snd_pcm_substream, prepared: *mut bool) -> i32;
    fn devm_kzalloc(dev: *mut device, size: usize, flags: gfp_t) -> *mut c_void;
    fn dev_set_drvdata(dev: *mut device, data: *mut c_void);
    fn qcom_snd_parse_of(card: *mut snd_soc_card) -> i32;
    fn of_device_get_match_data(dev: *mut device) -> *const c_void;
    fn devm_snd_soc_register_card(dev: *mut device, card: *mut snd_soc_card) -> i32;
}

unsafe extern "C" fn sm8250_snd_init(rtd: *mut snd_soc_pcm_runtime) -> i32 {
    let data = snd_soc_card_get_drvdata((*rtd).card) as *mut sm8250_snd_data;
    let cpu_dai = snd_soc_rtd_to_cpu(rtd, 0);

    match (*cpu_dai).id {
        DISPLAY_PORT_RX => qcom_snd_dp_jack_setup(rtd, &mut (*data).dp_jack, 0),
        USB_RX => qcom_snd_usb_offload_jack_setup(
            rtd,
            &mut (*data).usb_offload_jack,
            &mut (*data).usb_offload_jack_setup,
        ),
        _ => qcom_snd_wcd_jack_setup(rtd, &mut (*data).jack, &mut (*data).jack_setup),
    }
}

unsafe extern "C" fn sm8250_snd_exit(rtd: *mut snd_soc_pcm_runtime) {
    let data = snd_soc_card_get_drvdata((*rtd).card) as *mut sm8250_snd_data;
    let cpu_dai = snd_soc_rtd_to_cpu(rtd, 0);

    if (*cpu_dai).id == USB_RX {
        qcom_snd_usb_offload_jack_remove(rtd, &mut (*data).usb_offload_jack_setup);
    }
}

unsafe extern "C" fn sm8250_be_hw_params_fixup(
    _rtd: *mut snd_soc_pcm_runtime,
    params: *mut snd_pcm_hw_params,
) -> i32 {
    let rate = hw_param_interval(params, SNDRV_PCM_HW_PARAM_RATE);
    let channels = hw_param_interval(params, SNDRV_PCM_HW_PARAM_CHANNELS);
    let fmt = hw_param_mask(params, SNDRV_PCM_HW_PARAM_FORMAT);

    (*rate).max = 48000;
    (*rate).min = (*rate).max;
    (*channels).max = 2;
    (*channels).min = (*channels).max;
    snd_mask_set_format(fmt, SNDRV_PCM_FORMAT_S16_LE);

    0
}

unsafe extern "C" fn sm8250_snd_startup(substream: *mut snd_pcm_substream) -> i32 {
    let fmt: u32 = SND_SOC_DAIFMT_BP_FP;
    let mut codec_dai_fmt: u32 = SND_SOC_DAIFMT_BC_FC;
    let rtd = snd_soc_substream_to_rtd(substream);
    let cpu_dai = snd_soc_rtd_to_cpu(rtd, 0);
    let codec_dai = snd_soc_rtd_to_codec(rtd, 0);

    match (*cpu_dai).id {
        PRIMARY_MI2S_RX => {
            codec_dai_fmt |= SND_SOC_DAIFMT_NB_NF | SND_SOC_DAIFMT_I2S;
            snd_soc_dai_set_sysclk(
                cpu_dai,
                Q6AFE_LPASS_CLK_ID_PRI_MI2S_IBIT,
                MI2S_BCLK_RATE,
                SNDRV_PCM_STREAM_PLAYBACK,
            );
            snd_soc_dai_set_fmt(cpu_dai, fmt);
            snd_soc_dai_set_fmt(codec_dai, codec_dai_fmt);
        }
        SECONDARY_MI2S_RX => {
            codec_dai_fmt |= SND_SOC_DAIFMT_NB_NF | SND_SOC_DAIFMT_I2S;
            snd_soc_dai_set_sysclk(
                cpu_dai,
                Q6AFE_LPASS_CLK_ID_SEC_MI2S_IBIT,
                MI2S_BCLK_RATE,
                SNDRV_PCM_STREAM_PLAYBACK,
            );
            snd_soc_dai_set_fmt(cpu_dai, fmt);
            snd_soc_dai_set_fmt(codec_dai, codec_dai_fmt);
        }
        TERTIARY_MI2S_RX => {
            codec_dai_fmt |= SND_SOC_DAIFMT_NB_NF | SND_SOC_DAIFMT_I2S;
            snd_soc_dai_set_sysclk(
                cpu_dai,
                Q6AFE_LPASS_CLK_ID_TER_MI2S_IBIT,
                MI2S_BCLK_RATE,
                SNDRV_PCM_STREAM_PLAYBACK,
            );
            snd_soc_dai_set_fmt(cpu_dai, fmt);
            snd_soc_dai_set_fmt(codec_dai, codec_dai_fmt);
        }
        QUINARY_MI2S_RX => {
            codec_dai_fmt |= SND_SOC_DAIFMT_NB_NF | SND_SOC_DAIFMT_I2S;
            snd_soc_dai_set_sysclk(
                cpu_dai,
                Q6AFE_LPASS_CLK_ID_QUI_MI2S_IBIT,
                MI2S_BCLK_RATE,
                SNDRV_PCM_STREAM_PLAYBACK,
            );
            snd_soc_dai_set_fmt(cpu_dai, fmt);
            snd_soc_dai_set_fmt(codec_dai, codec_dai_fmt);
        }
        LPI_MI2S_RX_0 => {
            codec_dai_fmt |= SND_SOC_DAIFMT_NB_NF | SND_SOC_DAIFMT_I2S;
            snd_soc_dai_set_sysclk(
                cpu_dai,
                Q6AFE_LPASS_CLK_ID_INT0_MI2S_IBIT,
                MI2S_BCLK_RATE,
                SNDRV_PCM_STREAM_PLAYBACK,
            );
            snd_soc_dai_set_fmt(cpu_dai, fmt);
            snd_soc_dai_set_fmt(codec_dai, codec_dai_fmt);
        }
        LPI_MI2S_TX_3 => {
            codec_dai_fmt |= SND_SOC_DAIFMT_NB_NF | SND_SOC_DAIFMT_I2S;
            snd_soc_dai_set_sysclk(
                cpu_dai,
                Q6AFE_LPASS_CLK_ID_INT3_MI2S_IBIT,
                MI2S_BCLK_RATE,
                SNDRV_PCM_STREAM_CAPTURE,
            );
            snd_soc_dai_set_fmt(cpu_dai, fmt);
            snd_soc_dai_set_fmt(codec_dai, codec_dai_fmt);
        }
        _ => {}
    }

    qcom_snd_sdw_startup(substream)
}

unsafe extern "C" fn sm8250_snd_prepare(substream: *mut snd_pcm_substream) -> i32 {
    let rtd = snd_soc_substream_to_rtd(substream);
    let cpu_dai = snd_soc_rtd_to_cpu(rtd, 0);
    let data = snd_soc_card_get_drvdata((*rtd).card) as *mut sm8250_snd_data;

    qcom_snd_sdw_prepare(
        substream,
        &mut (*data).stream_prepared[(*cpu_dai).id as usize],
    )
}

unsafe extern "C" fn sm8250_snd_hw_free(substream: *mut snd_pcm_substream) -> i32 {
    let rtd = snd_soc_substream_to_rtd(substream);
    let data = snd_soc_card_get_drvdata((*rtd).card) as *mut sm8250_snd_data;
    let cpu_dai = snd_soc_rtd_to_cpu(rtd, 0);

    qcom_snd_sdw_hw_free(
        substream,
        &mut (*data).stream_prepared[(*cpu_dai).id as usize],
    )
}

static sm8250_be_ops: snd_soc_ops = snd_soc_ops {
    startup: Some(sm8250_snd_startup),
    shutdown: Some(qcom_snd_sdw_shutdown),
    hw_free: Some(sm8250_snd_hw_free),
    prepare: Some(sm8250_snd_prepare),
};

unsafe fn sm8250_add_be_ops(card: *mut snd_soc_card) {
    let mut link: *mut snd_soc_dai_link;
    let mut i: i32 = 0;

    // C source used for_each_card_prelinks(card, i, link).
    while i < (*card).num_links {
        link = (*card).dai_link.add(i as usize);
        if (*link).no_pcm == 1 {
            (*link).init = Some(sm8250_snd_init);
            (*link).exit = Some(sm8250_snd_exit);
            (*link).be_hw_params_fixup = Some(sm8250_be_hw_params_fixup);
            (*link).ops = &sm8250_be_ops;
        }
        i += 1;
    }
}

unsafe extern "C" fn sm8250_platform_probe(pdev: *mut platform_device) -> i32 {
    let mut card: *mut snd_soc_card;
    let mut data: *mut sm8250_snd_data;
    let dev: *mut device = &mut (*pdev).dev;
    let mut ret: i32;

    card = devm_kzalloc(dev, core::mem::size_of::<snd_soc_card>(), GFP_KERNEL) as *mut snd_soc_card;
    if card.is_null() {
        return -ENOMEM;
    }

    (*card).owner = THIS_MODULE;
    /* Allocate the private data */
    data = devm_kzalloc(
        dev,
        core::mem::size_of::<sm8250_snd_data>(),
        GFP_KERNEL,
    ) as *mut sm8250_snd_data;
    if data.is_null() {
        return -ENOMEM;
    }

    (*card).dev = dev;
    dev_set_drvdata(dev, card as *mut c_void);
    snd_soc_card_set_drvdata(card, data as *mut c_void);
    ret = qcom_snd_parse_of(card);
    if ret != 0 {
        return ret;
    }

    (*card).driver_name = of_device_get_match_data(dev) as *const c_char;
    sm8250_add_be_ops(card);
    devm_snd_soc_register_card(dev, card)
}

static snd_sm8250_dt_match: [of_device_id; 8] = [
    of_device_id {
        compatible: c_str!("fairphone,fp4-sndcard"),
        data: c_str!("sm7225") as *const c_void,
        ..Default::default()
    },
    of_device_id {
        compatible: c_str!("fairphone,fp5-sndcard"),
        data: c_str!("qcm6490") as *const c_void,
        ..Default::default()
    },
    of_device_id {
        compatible: c_str!("qcom,qrb2210-sndcard"),
        data: c_str!("qcm2290") as *const c_void,
        ..Default::default()
    },
    of_device_id {
        compatible: c_str!("qcom,qrb4210-rb2-sndcard"),
        data: c_str!("sm4250") as *const c_void,
        ..Default::default()
    },
    of_device_id {
        compatible: c_str!("qcom,qrb5165-rb5-sndcard"),
        data: c_str!("sm8250") as *const c_void,
        ..Default::default()
    },
    of_device_id {
        compatible: c_str!("qcom,sdm660-sndcard"),
        data: c_str!("sdm660") as *const c_void,
        ..Default::default()
    },
    of_device_id {
        compatible: c_str!("qcom,sm8250-sndcard"),
        data: c_str!("sm8250") as *const c_void,
        ..Default::default()
    },
    of_device_id::default(),
];

module_device_table!(of, snd_sm8250_dt_match);

static mut snd_sm8250_driver: platform_driver = platform_driver {
    probe: Some(sm8250_platform_probe),
    driver: device_driver {
        name: c_str!("snd-sm8250"),
        of_match_table: snd_sm8250_dt_match.as_ptr(),
        ..Default::default()
    },
    ..Default::default()
};

module_platform_driver!(snd_sm8250_driver);
module_author!("Srinivas Kandagatla <srinivas.kandagatla@linaro.org");
module_description!("SM8250 ASoC Machine Driver");
module_license!("GPL");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
