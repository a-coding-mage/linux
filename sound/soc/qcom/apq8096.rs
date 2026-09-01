// SPDX-License-Identifier: GPL-2.0
// Copyright (c) 2018, Linaro Limited

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::mem::size_of;
use core::ptr;

const SLIM_MAX_TX_PORTS: usize = 16;
const SLIM_MAX_RX_PORTS: usize = 16;
const WCD9335_DEFAULT_MCLK_RATE: c_uint = 9600000;

const SNDRV_PCM_HW_PARAM_RATE: c_int = 0;
const SNDRV_PCM_HW_PARAM_CHANNELS: c_int = 0;
const SNDRV_PCM_STREAM_PLAYBACK: c_int = 0;
const ENOTSUPP: c_int = 524;
const ENOMEM: c_int = 12;
const GFP_KERNEL: c_uint = 0;

#[repr(C)]
pub struct snd_interval {
    pub min: c_uint,
    pub max: c_uint,
}

#[repr(C)]
pub struct snd_pcm_hw_params {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_pcm_substream {
    pub stream: c_int,
}

#[repr(C)]
pub struct snd_soc_pcm_runtime {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_dai {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_ops {
    pub hw_params: Option<
        unsafe extern "C" fn(
            substream: *mut snd_pcm_substream,
            params: *mut snd_pcm_hw_params,
        ) -> c_int,
    >,
}

#[repr(C)]
pub struct snd_soc_dai_link {
    pub no_pcm: c_int,
    pub be_hw_params_fixup: Option<
        unsafe extern "C" fn(
            rtd: *mut snd_soc_pcm_runtime,
            params: *mut snd_pcm_hw_params,
        ) -> c_int,
    >,
    pub init: Option<unsafe extern "C" fn(rtd: *mut snd_soc_pcm_runtime) -> c_int>,
    pub ops: *const snd_soc_ops,
}

#[repr(C)]
pub struct snd_soc_card {
    pub driver_name: *const c_char,
    pub dev: *mut device,
    pub owner: *mut module,
    pub num_links: c_int,
    pub dai_link: *mut snd_soc_dai_link,
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
pub struct of_device_id {
    pub compatible: *const c_char,
}

#[repr(C)]
pub struct device_driver {
    pub name: *const c_char,
    pub of_match_table: *const of_device_id,
}

#[repr(C)]
pub struct platform_driver {
    pub probe: Option<unsafe extern "C" fn(pdev: *mut platform_device) -> c_int>,
    pub driver: device_driver,
}

#[repr(C)]
pub struct module {
    _private: [u8; 0],
}

unsafe extern "C" {
    static mut THIS_MODULE: *mut module;

    fn hw_param_interval(
        params: *mut snd_pcm_hw_params,
        var: c_int,
    ) -> *mut snd_interval;
    fn snd_soc_substream_to_rtd(
        substream: *mut snd_pcm_substream,
    ) -> *mut snd_soc_pcm_runtime;
    fn snd_soc_rtd_to_codec(
        rtd: *mut snd_soc_pcm_runtime,
        num: c_int,
    ) -> *mut snd_soc_dai;
    fn snd_soc_rtd_to_cpu(
        rtd: *mut snd_soc_pcm_runtime,
        num: c_int,
    ) -> *mut snd_soc_dai;
    fn snd_soc_dai_get_channel_map(
        dai: *mut snd_soc_dai,
        tx_num: *mut c_uint,
        tx_slot: *mut c_uint,
        rx_num: *mut c_uint,
        rx_slot: *mut c_uint,
    ) -> c_int;
    fn snd_soc_dai_set_channel_map(
        dai: *mut snd_soc_dai,
        tx_num: c_uint,
        tx_slot: *mut c_uint,
        rx_num: c_uint,
        rx_slot: *mut c_uint,
    ) -> c_int;
    fn snd_soc_dai_set_sysclk(
        dai: *mut snd_soc_dai,
        clk_id: c_int,
        freq: c_uint,
        dir: c_int,
    ) -> c_int;
    fn devm_kzalloc(dev: *mut device, size: usize, flags: c_uint) -> *mut c_void;
    fn dev_set_drvdata(dev: *mut device, data: *mut c_void);
    fn qcom_snd_parse_of(card: *mut snd_soc_card) -> c_int;
    fn devm_snd_soc_register_card(dev: *mut device, card: *mut snd_soc_card) -> c_int;
    fn pr_err(fmt: *const c_char, ...);
}

unsafe extern "C" fn apq8096_be_hw_params_fixup(
    _rtd: *mut snd_soc_pcm_runtime,
    params: *mut snd_pcm_hw_params,
) -> c_int {
    unsafe {
        let rate = hw_param_interval(params, SNDRV_PCM_HW_PARAM_RATE);
        let channels = hw_param_interval(params, SNDRV_PCM_HW_PARAM_CHANNELS);

        (*rate).max = 48000;
        (*rate).min = (*rate).max;
        (*channels).max = 2;
        (*channels).min = (*channels).max;

        0
    }
}

unsafe extern "C" fn msm_snd_hw_params(
    substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
) -> c_int {
    unsafe {
        let rtd = snd_soc_substream_to_rtd(substream);
        let codec_dai = snd_soc_rtd_to_codec(rtd, 0);
        let cpu_dai = snd_soc_rtd_to_cpu(rtd, 0);
        let mut rx_ch: [c_uint; SLIM_MAX_RX_PORTS] = [0; SLIM_MAX_RX_PORTS];
        let mut tx_ch: [c_uint; SLIM_MAX_TX_PORTS] = [0; SLIM_MAX_TX_PORTS];
        let mut rx_ch_cnt: c_uint = 0;
        let mut tx_ch_cnt: c_uint = 0;
        let mut ret: c_int = 0;

        ret = snd_soc_dai_get_channel_map(
            codec_dai,
            &mut tx_ch_cnt,
            tx_ch.as_mut_ptr(),
            &mut rx_ch_cnt,
            rx_ch.as_mut_ptr(),
        );
        if ret != 0 && ret != -ENOTSUPP {
            pr_err(c"failed to get codec chan map, err:%d\n".as_ptr(), ret);
            return ret;
        } else if ret == -ENOTSUPP {
            return 0;
        }

        if (*substream).stream == SNDRV_PCM_STREAM_PLAYBACK {
            ret = snd_soc_dai_set_channel_map(
                cpu_dai,
                0,
                ptr::null_mut(),
                rx_ch_cnt,
                rx_ch.as_mut_ptr(),
            );
        } else {
            ret = snd_soc_dai_set_channel_map(
                cpu_dai,
                tx_ch_cnt,
                tx_ch.as_mut_ptr(),
                0,
                ptr::null_mut(),
            );
        }
        if ret != 0 && ret != -ENOTSUPP {
            pr_err(c"Failed to set cpu chan map, err:%d\n".as_ptr(), ret);
        } else if ret == -ENOTSUPP {
            ret = 0;
        }

        ret
    }
}

static APQ8096_OPS: snd_soc_ops = snd_soc_ops {
    hw_params: Some(msm_snd_hw_params),
};

unsafe extern "C" fn apq8096_init(rtd: *mut snd_soc_pcm_runtime) -> c_int {
    unsafe {
        let codec_dai = snd_soc_rtd_to_codec(rtd, 0);

        /*
         * Codec SLIMBUS configuration
         * RX1, RX2, RX3, RX4, RX5, RX6, RX7, RX8, RX9, RX10, RX11, RX12, RX13
         * TX1, TX2, TX3, TX4, TX5, TX6, TX7, TX8, TX9, TX10, TX11, TX12, TX13
         * TX14, TX15, TX16
         */
        let mut rx_ch: [c_uint; SLIM_MAX_RX_PORTS] = [
            144, 145, 146, 147, 148, 149, 150, 151, 152, 153, 154, 155, 156, 0, 0, 0,
        ];
        let mut tx_ch: [c_uint; SLIM_MAX_TX_PORTS] = [
            128, 129, 130, 131, 132, 133, 134, 135, 136, 137, 138, 139, 140, 141, 142, 143,
        ];

        snd_soc_dai_set_channel_map(
            codec_dai,
            tx_ch.len() as c_uint,
            tx_ch.as_mut_ptr(),
            rx_ch.len() as c_uint,
            rx_ch.as_mut_ptr(),
        );

        snd_soc_dai_set_sysclk(
            codec_dai,
            0,
            WCD9335_DEFAULT_MCLK_RATE,
            SNDRV_PCM_STREAM_PLAYBACK,
        );

        0
    }
}

unsafe fn apq8096_add_be_ops(card: *mut snd_soc_card) {
    unsafe {
        let mut i: c_int = 0;

        while i < (*card).num_links {
            let link = (*card).dai_link.add(i as usize);
            if (*link).no_pcm == 1 {
                (*link).be_hw_params_fixup = Some(apq8096_be_hw_params_fixup);
                (*link).init = Some(apq8096_init);
                (*link).ops = &APQ8096_OPS;
            }
            i += 1;
        }
    }
}

unsafe extern "C" fn apq8096_platform_probe(pdev: *mut platform_device) -> c_int {
    unsafe {
        let card: *mut snd_soc_card;
        let dev = &mut (*pdev).dev as *mut device;
        let mut ret: c_int;

        card = devm_kzalloc(dev, size_of::<snd_soc_card>(), GFP_KERNEL) as *mut snd_soc_card;
        if card.is_null() {
            return -ENOMEM;
        }

        (*card).driver_name = c"apq8096".as_ptr();
        (*card).dev = dev;
        (*card).owner = THIS_MODULE;
        dev_set_drvdata(dev, card as *mut c_void);
        ret = qcom_snd_parse_of(card);
        if ret != 0 {
            return ret;
        }

        apq8096_add_be_ops(card);
        devm_snd_soc_register_card(dev, card)
    }
}

static MSM_SND_APQ8096_DT_MATCH: [of_device_id; 2] = [
    of_device_id {
        compatible: c"qcom,apq8096-sndcard".as_ptr(),
    },
    of_device_id {
        compatible: ptr::null(),
    },
];

// MODULE_DEVICE_TABLE(of, msm_snd_apq8096_dt_match);

static mut MSM_SND_APQ8096_DRIVER: platform_driver = platform_driver {
    probe: Some(apq8096_platform_probe),
    driver: device_driver {
        name: c"msm-snd-apq8096".as_ptr(),
        of_match_table: MSM_SND_APQ8096_DT_MATCH.as_ptr(),
    },
};

// module_platform_driver(msm_snd_apq8096_driver);
// MODULE_AUTHOR("Srinivas Kandagatla <srinivas.kandagatla@linaro.org");
// MODULE_DESCRIPTION("APQ8096 ASoC Machine Driver");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
