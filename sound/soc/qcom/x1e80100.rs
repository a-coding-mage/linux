// SPDX-License-Identifier: GPL-2.0
// Copyright (c) 2023, Linaro Limited

// Translated from C implementation source. External kernel, ASoC, QDSP6, and
// SoundWire symbols are declared here as dependencies supplied by other files.

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::mem::size_of;
use core::ptr::{addr_of_mut, null, null_mut};

const AFE_PORT_MAX: usize = 0; // TODO: supplied by dt-bindings/sound/qcom,q6afe.h
const GFP_KERNEL: c_uint = 0; // TODO: supplied by linux headers
const ENOMEM: c_int = 12;
const EINVAL: c_int = 22;
const SNDRV_PCM_HW_PARAM_RATE: c_int = 0; // TODO: supplied by sound/pcm.h
const SNDRV_PCM_HW_PARAM_CHANNELS: c_int = 0; // TODO: supplied by sound/pcm.h

const WSA_CODEC_DMA_RX_0: c_int = 0; // TODO: supplied by q6afe/q6dsp headers
const WSA_CODEC_DMA_RX_1: c_int = 0; // TODO: supplied by q6afe/q6dsp headers
const DISPLAY_PORT_RX_0: c_int = 0; // TODO: supplied by q6afe/q6dsp headers
const DISPLAY_PORT_RX_1: c_int = 0; // TODO: supplied by q6afe/q6dsp headers
const DISPLAY_PORT_RX_7: c_int = 0; // TODO: supplied by q6afe/q6dsp headers
const TX_CODEC_DMA_TX_0: c_int = 0; // TODO: supplied by q6afe/q6dsp headers
const TX_CODEC_DMA_TX_1: c_int = 0; // TODO: supplied by q6afe/q6dsp headers
const TX_CODEC_DMA_TX_2: c_int = 0; // TODO: supplied by q6afe/q6dsp headers
const TX_CODEC_DMA_TX_3: c_int = 0; // TODO: supplied by q6afe/q6dsp headers

const PCM_CHANNEL_FC: c_uint = 0; // TODO: supplied by sound/pcm.h
const PCM_CHANNEL_FL: c_uint = 0; // TODO: supplied by sound/pcm.h
const PCM_CHANNEL_FR: c_uint = 0; // TODO: supplied by sound/pcm.h
const PCM_CHANNEL_LB: c_uint = 0; // TODO: supplied by sound/pcm.h
const PCM_CHANNEL_RB: c_uint = 0; // TODO: supplied by sound/pcm.h

#[repr(C)]
struct snd_soc_card {
    owner: *mut c_void,
    dev: *mut device,
    driver_name: *const c_void,
}

#[repr(C)]
struct snd_soc_jack {
    _private: [u8; 0],
}

#[repr(C)]
struct snd_soc_pcm_runtime {
    card: *mut snd_soc_card,
}

#[repr(C)]
struct snd_soc_dai {
    id: c_int,
}

#[repr(C)]
struct snd_pcm_runtime {
    channels: c_uint,
}

#[repr(C)]
struct snd_pcm_substream {
    runtime: *mut snd_pcm_runtime,
}

#[repr(C)]
struct snd_pcm_hw_params {
    _private: [u8; 0],
}

#[repr(C)]
struct snd_interval {
    min: c_uint,
    max: c_uint,
}

#[repr(C)]
struct snd_soc_ops {
    startup: Option<unsafe extern "C" fn(*mut snd_pcm_substream) -> c_int>,
    shutdown: Option<unsafe extern "C" fn(*mut snd_pcm_substream)>,
    hw_free: Option<unsafe extern "C" fn(*mut snd_pcm_substream) -> c_int>,
    prepare: Option<unsafe extern "C" fn(*mut snd_pcm_substream) -> c_int>,
}

#[repr(C)]
struct snd_soc_dai_link {
    no_pcm: c_int,
    init: Option<unsafe extern "C" fn(*mut snd_soc_pcm_runtime) -> c_int>,
    be_hw_params_fixup:
        Option<unsafe extern "C" fn(*mut snd_soc_pcm_runtime, *mut snd_pcm_hw_params) -> c_int>,
    ops: *const snd_soc_ops,
}

#[repr(C)]
struct device {
    _private: [u8; 0],
}

#[repr(C)]
struct platform_device {
    dev: device,
}

#[repr(C)]
struct of_device_id {
    compatible: *const c_char,
    data: *const c_void,
}

#[repr(C)]
struct device_driver {
    name: *const c_char,
    of_match_table: *const of_device_id,
}

#[repr(C)]
struct platform_driver {
    probe: Option<unsafe extern "C" fn(*mut platform_device) -> c_int>,
    driver: device_driver,
}

#[repr(C)]
struct x1e80100_snd_data {
    stream_prepared: [bool; AFE_PORT_MAX],
    card: *mut snd_soc_card,
    jack: snd_soc_jack,
    dp_jack: [snd_soc_jack; 8],
    jack_setup: bool,
}

unsafe extern "C" {
    static mut THIS_MODULE: *mut c_void;

    fn snd_soc_card_get_drvdata(card: *mut snd_soc_card) -> *mut c_void;
    fn snd_soc_card_set_drvdata(card: *mut snd_soc_card, data: *mut c_void);
    fn snd_soc_rtd_to_cpu(rtd: *mut snd_soc_pcm_runtime, index: c_int) -> *mut snd_soc_dai;
    fn snd_soc_substream_to_rtd(substream: *mut snd_pcm_substream) -> *mut snd_soc_pcm_runtime;
    fn snd_soc_limit_volume(card: *mut snd_soc_card, name: *const c_char, max: c_int) -> c_int;
    fn snd_soc_dai_set_channel_map(
        dai: *mut snd_soc_dai,
        tx_num: c_uint,
        tx_slot: *mut c_uint,
        rx_num: c_uint,
        rx_slot: *mut c_uint,
    ) -> c_int;
    fn hw_param_interval(
        params: *mut snd_pcm_hw_params,
        var: c_int,
    ) -> *mut snd_interval;

    fn qcom_snd_dp_jack_setup(
        rtd: *mut snd_soc_pcm_runtime,
        jack: *mut snd_soc_jack,
        pcm_id: c_int,
    ) -> c_int;
    fn qcom_snd_wcd_jack_setup(
        rtd: *mut snd_soc_pcm_runtime,
        jack: *mut snd_soc_jack,
        jack_setup: *mut bool,
    ) -> c_int;
    fn qcom_snd_sdw_prepare(
        substream: *mut snd_pcm_substream,
        stream_prepared: *mut bool,
    ) -> c_int;
    fn qcom_snd_sdw_hw_free(
        substream: *mut snd_pcm_substream,
        stream_prepared: *mut bool,
    ) -> c_int;
    fn qcom_snd_sdw_startup(substream: *mut snd_pcm_substream) -> c_int;
    fn qcom_snd_sdw_shutdown(substream: *mut snd_pcm_substream);
    fn qcom_snd_parse_of(card: *mut snd_soc_card) -> c_int;

    fn devm_kzalloc(dev: *mut device, size: usize, flags: c_uint) -> *mut c_void;
    fn dev_set_drvdata(dev: *mut device, data: *mut c_void);
    fn of_device_get_match_data(dev: *mut device) -> *const c_void;
    fn devm_snd_soc_register_card(dev: *mut device, card: *mut snd_soc_card) -> c_int;
}

unsafe extern "C" fn x1e80100_snd_init(rtd: *mut snd_soc_pcm_runtime) -> c_int {
    let data = snd_soc_card_get_drvdata((*rtd).card) as *mut x1e80100_snd_data;
    let cpu_dai = snd_soc_rtd_to_cpu(rtd, 0);
    let card = (*rtd).card;
    let mut dp_jack: *mut snd_soc_jack = null_mut();
    let mut dp_pcm_id: c_int = 0;

    match (*cpu_dai).id {
        WSA_CODEC_DMA_RX_0 | WSA_CODEC_DMA_RX_1 => {
            /*
             * Set limit of -3 dB on Digital Volume and 0 dB on PA Volume
             * to reduce the risk of speaker damage until we have active
             * speaker protection in place.
             */
            snd_soc_limit_volume(card, c"WSA WSA_RX0 Digital Volume".as_ptr(), 81);
            snd_soc_limit_volume(card, c"WSA WSA_RX1 Digital Volume".as_ptr(), 81);
            snd_soc_limit_volume(card, c"WSA2 WSA_RX0 Digital Volume".as_ptr(), 81);
            snd_soc_limit_volume(card, c"WSA2 WSA_RX1 Digital Volume".as_ptr(), 81);
            snd_soc_limit_volume(card, c"SpkrLeft PA Volume".as_ptr(), 6);
            snd_soc_limit_volume(card, c"SpkrRight PA Volume".as_ptr(), 6);
            snd_soc_limit_volume(card, c"WooferLeft PA Volume".as_ptr(), 6);
            snd_soc_limit_volume(card, c"TweeterLeft PA Volume".as_ptr(), 6);
            snd_soc_limit_volume(card, c"WooferRight PA Volume".as_ptr(), 6);
            snd_soc_limit_volume(card, c"TweeterRight PA Volume".as_ptr(), 6);
        }
        DISPLAY_PORT_RX_0 => {
            dp_pcm_id = 0;
            dp_jack = addr_of_mut!((*data).dp_jack[dp_pcm_id as usize]);
        }
        id if id >= DISPLAY_PORT_RX_1 && id <= DISPLAY_PORT_RX_7 => {
            dp_pcm_id = (*cpu_dai).id - DISPLAY_PORT_RX_1 + 1;
            dp_jack = addr_of_mut!((*data).dp_jack[dp_pcm_id as usize]);
        }
        _ => {}
    }

    if !dp_jack.is_null() {
        return qcom_snd_dp_jack_setup(rtd, dp_jack, dp_pcm_id);
    }

    qcom_snd_wcd_jack_setup(
        rtd,
        addr_of_mut!((*data).jack),
        addr_of_mut!((*data).jack_setup),
    )
}

unsafe extern "C" fn x1e80100_be_hw_params_fixup(
    rtd: *mut snd_soc_pcm_runtime,
    params: *mut snd_pcm_hw_params,
) -> c_int {
    let cpu_dai = snd_soc_rtd_to_cpu(rtd, 0);
    let rate = hw_param_interval(params, SNDRV_PCM_HW_PARAM_RATE);
    let channels = hw_param_interval(params, SNDRV_PCM_HW_PARAM_CHANNELS);

    (*rate).max = 48000;
    (*rate).min = (*rate).max;
    match (*cpu_dai).id {
        TX_CODEC_DMA_TX_0 | TX_CODEC_DMA_TX_1 | TX_CODEC_DMA_TX_2 | TX_CODEC_DMA_TX_3 => {
            (*channels).min = 1;
        }
        _ => {}
    }

    0
}

unsafe fn x1e80100_snd_hw_map_channels(ch_map: *mut c_uint, num: c_int) -> c_int {
    match num {
        1 => {
            *ch_map.add(0) = PCM_CHANNEL_FC;
        }
        2 => {
            *ch_map.add(0) = PCM_CHANNEL_FL;
            *ch_map.add(1) = PCM_CHANNEL_FR;
        }
        3 => {
            *ch_map.add(0) = PCM_CHANNEL_FL;
            *ch_map.add(1) = PCM_CHANNEL_FR;
            *ch_map.add(2) = PCM_CHANNEL_FC;
        }
        4 => {
            *ch_map.add(0) = PCM_CHANNEL_FL;
            *ch_map.add(1) = PCM_CHANNEL_LB;
            *ch_map.add(2) = PCM_CHANNEL_FR;
            *ch_map.add(3) = PCM_CHANNEL_RB;
        }
        _ => {
            return -EINVAL;
        }
    }

    0
}

unsafe extern "C" fn x1e80100_snd_prepare(substream: *mut snd_pcm_substream) -> c_int {
    let rtd = snd_soc_substream_to_rtd(substream);
    let cpu_dai = snd_soc_rtd_to_cpu(rtd, 0);
    let data = snd_soc_card_get_drvdata((*rtd).card) as *mut x1e80100_snd_data;
    let channels = (*(*substream).runtime).channels;
    let mut rx_slot: [c_uint; 4] = [0; 4];
    let mut ret: c_int;

    match (*cpu_dai).id {
        WSA_CODEC_DMA_RX_0 | WSA_CODEC_DMA_RX_1 => {
            ret = x1e80100_snd_hw_map_channels(rx_slot.as_mut_ptr(), channels as c_int);
            if ret != 0 {
                return ret;
            }

            ret = snd_soc_dai_set_channel_map(
                cpu_dai,
                0,
                null_mut(),
                channels,
                rx_slot.as_mut_ptr(),
            );
            if ret != 0 {
                return ret;
            }
        }
        _ => {}
    }

    qcom_snd_sdw_prepare(
        substream,
        addr_of_mut!((*data).stream_prepared[(*cpu_dai).id as usize]),
    )
}

unsafe extern "C" fn x1e80100_snd_hw_free(substream: *mut snd_pcm_substream) -> c_int {
    let rtd = snd_soc_substream_to_rtd(substream);
    let data = snd_soc_card_get_drvdata((*rtd).card) as *mut x1e80100_snd_data;
    let cpu_dai = snd_soc_rtd_to_cpu(rtd, 0);

    qcom_snd_sdw_hw_free(
        substream,
        addr_of_mut!((*data).stream_prepared[(*cpu_dai).id as usize]),
    )
}

static x1e80100_be_ops: snd_soc_ops = snd_soc_ops {
    startup: Some(qcom_snd_sdw_startup),
    shutdown: Some(qcom_snd_sdw_shutdown),
    hw_free: Some(x1e80100_snd_hw_free),
    prepare: Some(x1e80100_snd_prepare),
};

unsafe fn x1e80100_add_be_ops(card: *mut snd_soc_card) {
    let mut link: *mut snd_soc_dai_link;
    let mut i: c_int;

    // for_each_card_prelinks(card, i, link)
    todo!("iterate card prelinks supplied by sound/soc.h");
    if (*link).no_pcm == 1 {
        (*link).init = Some(x1e80100_snd_init);
        (*link).be_hw_params_fixup = Some(x1e80100_be_hw_params_fixup);
        (*link).ops = &x1e80100_be_ops;
    }
}

unsafe extern "C" fn x1e80100_platform_probe(pdev: *mut platform_device) -> c_int {
    let mut card: *mut snd_soc_card;
    let mut data: *mut x1e80100_snd_data;
    let dev = addr_of_mut!((*pdev).dev);
    let mut ret: c_int;

    card = devm_kzalloc(dev, size_of::<snd_soc_card>(), GFP_KERNEL) as *mut snd_soc_card;
    if card.is_null() {
        return -ENOMEM;
    }
    /* Allocate the private data */
    data = devm_kzalloc(
        dev,
        size_of::<x1e80100_snd_data>(),
        GFP_KERNEL,
    ) as *mut x1e80100_snd_data;
    if data.is_null() {
        return -ENOMEM;
    }

    (*card).owner = THIS_MODULE;
    (*card).dev = dev;
    dev_set_drvdata(dev, card as *mut c_void);
    snd_soc_card_set_drvdata(card, data as *mut c_void);

    ret = qcom_snd_parse_of(card);
    if ret != 0 {
        return ret;
    }

    (*card).driver_name = of_device_get_match_data(dev);
    x1e80100_add_be_ops(card);

    devm_snd_soc_register_card(dev, card)
}

static snd_x1e80100_dt_match: [of_device_id; 3] = [
    of_device_id {
        compatible: c"qcom,x1e80100-sndcard".as_ptr(),
        data: c"x1e80100".as_ptr() as *const c_void,
    },
    of_device_id {
        compatible: c"qcom,glymur-sndcard".as_ptr(),
        data: c"glymur".as_ptr() as *const c_void,
    },
    of_device_id {
        compatible: null(),
        data: null(),
    },
];
// MODULE_DEVICE_TABLE(of, snd_x1e80100_dt_match);

static mut snd_x1e80100_driver: platform_driver = platform_driver {
    probe: Some(x1e80100_platform_probe),
    driver: device_driver {
        name: c"snd-x1e80100".as_ptr(),
        of_match_table: snd_x1e80100_dt_match.as_ptr(),
    },
};
// module_platform_driver(snd_x1e80100_driver);
// MODULE_AUTHOR("Srinivas Kandagatla <srinivas.kandagatla@linaro.org");
// MODULE_AUTHOR("Krzysztof Kozlowski <krzysztof.kozlowski@linaro.org>");
// MODULE_DESCRIPTION("Qualcomm X1E80100 ASoC Machine Driver");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
