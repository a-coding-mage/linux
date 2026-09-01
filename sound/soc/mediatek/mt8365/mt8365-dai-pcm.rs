// SPDX-License-Identifier: GPL-2.0
/*
 * MediaTek 8365 ALSA SoC Audio DAI PCM Control
 *
 * Copyright (c) 2024 MediaTek Inc.
 * Authors: Jia Zeng <jia.zeng@mediatek.com>
 *          Alexandre Mergnat <amergnat@baylibre.com>
 */

// Dependencies from the original C includes:
// <linux/bitops.h>, <linux/regmap.h>, <sound/pcm_params.h>,
// "mt8365-afe-clk.h", and "mt8365-afe-common.h".

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::mem::size_of;
use core::ptr;

#[repr(C)]
struct mt8365_pcm_intf_data {
    slave_mode: bool,
    lrck_inv: bool,
    bck_inv: bool,
    format: c_uint,
}

#[repr(C)]
struct list_head {
    next: *mut list_head,
    prev: *mut list_head,
}

#[repr(C)]
struct mtk_base_afe {
    regmap: *mut c_void,
    platform_priv: *mut mt8365_afe_private,
    dev: *mut c_void,
    sub_dais: list_head,
}

#[repr(C)]
struct mt8365_afe_private {
    dai_priv: [*mut c_void; 0],
}

#[repr(C)]
struct mtk_base_afe_dai {
    list: list_head,
    dai_drivers: *mut snd_soc_dai_driver,
    num_dai_drivers: c_uint,
    dapm_widgets: *const snd_soc_dapm_widget,
    num_dapm_widgets: c_uint,
    dapm_routes: *const snd_soc_dapm_route,
    num_dapm_routes: c_uint,
}

#[repr(C)]
struct snd_pcm_runtime {
    rate: c_uint,
}

#[repr(C)]
struct snd_pcm_substream {
    runtime: *mut snd_pcm_runtime,
}

#[repr(C)]
struct snd_soc_dai {
    symmetric_sample_bits: c_uint,
}

#[repr(C)]
struct snd_soc_dai_ops {
    startup: Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_soc_dai) -> c_int>,
    shutdown: Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_soc_dai)>,
    prepare: Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_soc_dai) -> c_int>,
    set_fmt: Option<unsafe extern "C" fn(*mut snd_soc_dai, c_uint) -> c_int>,
}

#[repr(C)]
struct snd_soc_pcm_stream {
    stream_name: *const c_char,
    channels_min: c_uint,
    channels_max: c_uint,
    rates: c_uint,
    formats: u64,
}

#[repr(C)]
struct snd_soc_dai_driver {
    name: *const c_char,
    id: c_int,
    playback: snd_soc_pcm_stream,
    capture: snd_soc_pcm_stream,
    ops: *const snd_soc_dai_ops,
    symmetric_rate: c_uint,
    symmetric_sample_bits: c_uint,
}

#[repr(C)]
struct snd_soc_dapm_widget {
    _private: [u8; 0],
}

#[repr(C)]
struct snd_soc_dapm_route {
    sink: *const c_char,
    control: *const c_char,
    source: *const c_char,
}

unsafe extern "C" {
    static PCM_INTF_CON1: c_uint;
    static PCM_INTF_CON1_EN: c_uint;
    static PCM_INTF_CON1_MASTER_MODE: c_uint;
    static PCM_INTF_CON1_BYPASS_ASRC: c_uint;
    static PCM_INTF_CON1_SYNC_OUT_INV: c_uint;
    static PCM_INTF_CON1_BCLK_OUT_INV: c_uint;
    static PCM_INTF_CON1_SLAVE_MODE: c_uint;
    static PCM_INTF_CON1_SYNC_IN_INV: c_uint;
    static PCM_INTF_CON1_BCLK_IN_INV: c_uint;
    static PCM_INTF_CON1_FORMAT_MASK: c_uint;
    static MT8365_PCM_FORMAT_PCMA: c_uint;
    static MT8365_PCM_FORMAT_PCMB: c_uint;
    static PCM_INTF_CON1_FS_48K: c_uint;
    static PCM_INTF_CON1_FS_32K: c_uint;
    static PCM_INTF_CON1_FS_16K: c_uint;
    static PCM_INTF_CON1_FS_8K: c_uint;
    static PCM_INTF_CON1_24BIT: c_uint;
    static PCM_INTF_CON1_64BCK: c_uint;
    static PCM_INTF_CON1_16BIT: c_uint;
    static PCM_INTF_CON1_32BCK: c_uint;
    static PCM_INTF_CON1_EXT_MODEM: c_uint;
    static PCM_INTF_CON1_CONFIG_MASK: c_uint;
    static MT8365_AFE_IO_PCM1: usize;
    static SND_SOC_DAIFMT_FORMAT_MASK: c_uint;
    static SND_SOC_DAIFMT_I2S: c_uint;
    static MT8365_PCM_FORMAT_I2S: c_uint;
    static SND_SOC_DAIFMT_INV_MASK: c_uint;
    static SND_SOC_DAIFMT_NB_NF: c_uint;
    static SND_SOC_DAIFMT_NB_IF: c_uint;
    static SND_SOC_DAIFMT_IB_NF: c_uint;
    static SND_SOC_DAIFMT_IB_IF: c_uint;
    static SND_SOC_DAIFMT_MASTER_MASK: c_uint;
    static SND_SOC_DAIFMT_CBP_CFP: c_uint;
    static SND_SOC_DAIFMT_CBC_CFC: c_uint;
    static SNDRV_PCM_STREAM_PLAYBACK: c_int;
    static SNDRV_PCM_STREAM_CAPTURE: c_int;
    static SNDRV_PCM_RATE_8000: c_uint;
    static SNDRV_PCM_RATE_16000: c_uint;
    static SNDRV_PCM_RATE_32000: c_uint;
    static SNDRV_PCM_RATE_48000: c_uint;
    static SNDRV_PCM_FMTBIT_S16_LE: u64;
    static SNDRV_PCM_FMTBIT_S32_LE: u64;
    static GFP_KERNEL: c_uint;

    fn regmap_update_bits(
        map: *mut c_void,
        reg: c_uint,
        mask: c_uint,
        val: c_uint,
    ) -> c_int;
    fn snd_soc_dai_get_drvdata(dai: *mut snd_soc_dai) -> *mut mtk_base_afe;
    fn snd_soc_dai_active(dai: *mut snd_soc_dai) -> c_int;
    fn snd_soc_dai_stream_active(dai: *mut snd_soc_dai, stream: c_int) -> c_uint;
    fn snd_pcm_stream_str(substream: *mut snd_pcm_substream) -> *const c_char;
    fn mt8365_afe_enable_main_clk(afe: *mut mtk_base_afe) -> c_int;
    fn mt8365_afe_disable_main_clk(afe: *mut mtk_base_afe);
    fn dev_info(dev: *mut c_void, fmt: *const c_char, ...) -> c_int;
    fn devm_kzalloc(dev: *mut c_void, size: usize, flags: c_uint) -> *mut c_void;
    fn list_add(new: *mut list_head, head: *mut list_head);
    fn FIELD_PREP(mask: c_uint, val: c_uint) -> c_uint;
    fn PCM_INTF_CON1_SYNC_LEN(len: c_uint) -> c_uint;
}

const EINVAL: c_int = 22;
const ENOMEM: c_int = 12;

/* DAI Drivers */

unsafe extern "C" fn mt8365_dai_enable_pcm1(afe: *mut mtk_base_afe) {
    unsafe {
        regmap_update_bits(
            (*afe).regmap,
            PCM_INTF_CON1,
            PCM_INTF_CON1_EN,
            PCM_INTF_CON1_EN,
        );
    }
}

unsafe extern "C" fn mt8365_dai_disable_pcm1(afe: *mut mtk_base_afe) {
    unsafe {
        regmap_update_bits((*afe).regmap, PCM_INTF_CON1, PCM_INTF_CON1_EN, 0x0);
    }
}

unsafe extern "C" fn mt8365_dai_configure_pcm1(
    substream: *mut snd_pcm_substream,
    dai: *mut snd_soc_dai,
) -> c_int {
    unsafe {
        let afe: *mut mtk_base_afe = snd_soc_dai_get_drvdata(dai);
        let afe_priv: *mut mt8365_afe_private = (*afe).platform_priv;
        let pcm_priv: *mut mt8365_pcm_intf_data =
            (*afe_priv).dai_priv[MT8365_AFE_IO_PCM1] as *mut mt8365_pcm_intf_data;
        let slave_mode: bool = (*pcm_priv).slave_mode;
        let lrck_inv: bool = (*pcm_priv).lrck_inv;
        let bck_inv: bool = (*pcm_priv).bck_inv;
        let fmt: c_uint = (*pcm_priv).format;
        let bit_width: c_uint = (*dai).symmetric_sample_bits;
        let mut val: c_uint = 0;

        if !slave_mode {
            val |= PCM_INTF_CON1_MASTER_MODE | PCM_INTF_CON1_BYPASS_ASRC;

            if lrck_inv {
                val |= PCM_INTF_CON1_SYNC_OUT_INV;
            }
            if bck_inv {
                val |= PCM_INTF_CON1_BCLK_OUT_INV;
            }
        } else {
            val |= PCM_INTF_CON1_SLAVE_MODE;

            if lrck_inv {
                val |= PCM_INTF_CON1_SYNC_IN_INV;
            }
            if bck_inv {
                val |= PCM_INTF_CON1_BCLK_IN_INV;
            }

            /* TODO: add asrc setting */
        }

        val |= FIELD_PREP(PCM_INTF_CON1_FORMAT_MASK, fmt);

        if fmt == MT8365_PCM_FORMAT_PCMA || fmt == MT8365_PCM_FORMAT_PCMB {
            val |= PCM_INTF_CON1_SYNC_LEN(1);
        } else {
            val |= PCM_INTF_CON1_SYNC_LEN(bit_width);
        }

        match (*(*substream).runtime).rate {
            48000 => {
                val |= PCM_INTF_CON1_FS_48K;
            }
            32000 => {
                val |= PCM_INTF_CON1_FS_32K;
            }
            16000 => {
                val |= PCM_INTF_CON1_FS_16K;
            }
            8000 => {
                val |= PCM_INTF_CON1_FS_8K;
            }
            _ => {
                return -EINVAL;
            }
        }

        if bit_width > 16 {
            val |= PCM_INTF_CON1_24BIT | PCM_INTF_CON1_64BCK;
        } else {
            val |= PCM_INTF_CON1_16BIT | PCM_INTF_CON1_32BCK;
        }

        val |= PCM_INTF_CON1_EXT_MODEM;

        regmap_update_bits((*afe).regmap, PCM_INTF_CON1, PCM_INTF_CON1_CONFIG_MASK, val);

        0
    }
}

unsafe extern "C" fn mt8365_dai_pcm1_startup(
    _substream: *mut snd_pcm_substream,
    dai: *mut snd_soc_dai,
) -> c_int {
    unsafe {
        let afe: *mut mtk_base_afe = snd_soc_dai_get_drvdata(dai);

        if snd_soc_dai_active(dai) != 0 {
            return 0;
        }

        mt8365_afe_enable_main_clk(afe);

        0
    }
}

unsafe extern "C" fn mt8365_dai_pcm1_shutdown(
    _substream: *mut snd_pcm_substream,
    dai: *mut snd_soc_dai,
) {
    unsafe {
        let afe: *mut mtk_base_afe = snd_soc_dai_get_drvdata(dai);

        if snd_soc_dai_active(dai) != 0 {
            return;
        }

        mt8365_dai_disable_pcm1(afe);
        mt8365_afe_disable_main_clk(afe);
    }
}

unsafe extern "C" fn mt8365_dai_pcm1_prepare(
    substream: *mut snd_pcm_substream,
    dai: *mut snd_soc_dai,
) -> c_int {
    unsafe {
        let afe: *mut mtk_base_afe = snd_soc_dai_get_drvdata(dai);
        let ret: c_int;

        if snd_soc_dai_stream_active(dai, SNDRV_PCM_STREAM_PLAYBACK)
            + snd_soc_dai_stream_active(dai, SNDRV_PCM_STREAM_CAPTURE)
            > 1
        {
            dev_info(
                (*afe).dev,
                c"%s '%s' active(%u-%u) already\n".as_ptr(),
                c"mt8365_dai_pcm1_prepare".as_ptr(),
                snd_pcm_stream_str(substream),
                snd_soc_dai_stream_active(dai, SNDRV_PCM_STREAM_PLAYBACK),
                snd_soc_dai_stream_active(dai, SNDRV_PCM_STREAM_CAPTURE),
            );
            return 0;
        }

        ret = mt8365_dai_configure_pcm1(substream, dai);
        if ret != 0 {
            return ret;
        }

        mt8365_dai_enable_pcm1(afe);

        0
    }
}

unsafe extern "C" fn mt8365_dai_pcm1_set_fmt(dai: *mut snd_soc_dai, fmt: c_uint) -> c_int {
    unsafe {
        let afe: *mut mtk_base_afe = snd_soc_dai_get_drvdata(dai);
        let afe_priv: *mut mt8365_afe_private = (*afe).platform_priv;
        let pcm_priv: *mut mt8365_pcm_intf_data =
            (*afe_priv).dai_priv[MT8365_AFE_IO_PCM1] as *mut mt8365_pcm_intf_data;

        match fmt & SND_SOC_DAIFMT_FORMAT_MASK {
            x if x == SND_SOC_DAIFMT_I2S => {
                (*pcm_priv).format = MT8365_PCM_FORMAT_I2S;
            }
            _ => {
                return -EINVAL;
            }
        }

        match fmt & SND_SOC_DAIFMT_INV_MASK {
            x if x == SND_SOC_DAIFMT_NB_NF => {
                (*pcm_priv).bck_inv = false;
                (*pcm_priv).lrck_inv = false;
            }
            x if x == SND_SOC_DAIFMT_NB_IF => {
                (*pcm_priv).bck_inv = false;
                (*pcm_priv).lrck_inv = true;
            }
            x if x == SND_SOC_DAIFMT_IB_NF => {
                (*pcm_priv).bck_inv = true;
                (*pcm_priv).lrck_inv = false;
            }
            x if x == SND_SOC_DAIFMT_IB_IF => {
                (*pcm_priv).bck_inv = true;
                (*pcm_priv).lrck_inv = true;
            }
            _ => {
                return -EINVAL;
            }
        }

        match fmt & SND_SOC_DAIFMT_MASTER_MASK {
            x if x == SND_SOC_DAIFMT_CBP_CFP => {
                (*pcm_priv).slave_mode = true;
            }
            x if x == SND_SOC_DAIFMT_CBC_CFC => {
                (*pcm_priv).slave_mode = false;
            }
            _ => {
                return -EINVAL;
            }
        }

        0
    }
}

static mt8365_dai_pcm1_ops: snd_soc_dai_ops = snd_soc_dai_ops {
    startup: Some(mt8365_dai_pcm1_startup),
    shutdown: Some(mt8365_dai_pcm1_shutdown),
    prepare: Some(mt8365_dai_pcm1_prepare),
    set_fmt: Some(mt8365_dai_pcm1_set_fmt),
};

static mut mtk_dai_pcm_driver: [snd_soc_dai_driver; 1] = [snd_soc_dai_driver {
    name: c"PCM1".as_ptr(),
    id: unsafe { MT8365_AFE_IO_PCM1 as c_int },
    playback: snd_soc_pcm_stream {
        stream_name: c"PCM1 Playback".as_ptr(),
        channels_min: 1,
        channels_max: 2,
        rates: unsafe {
            SNDRV_PCM_RATE_8000
                | SNDRV_PCM_RATE_16000
                | SNDRV_PCM_RATE_32000
                | SNDRV_PCM_RATE_48000
        },
        formats: unsafe { SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S32_LE },
    },
    capture: snd_soc_pcm_stream {
        stream_name: c"PCM1 Capture".as_ptr(),
        channels_min: 1,
        channels_max: 2,
        rates: unsafe {
            SNDRV_PCM_RATE_8000
                | SNDRV_PCM_RATE_16000
                | SNDRV_PCM_RATE_32000
                | SNDRV_PCM_RATE_48000
        },
        formats: unsafe { SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S32_LE },
    },
    ops: &mt8365_dai_pcm1_ops,
    symmetric_rate: 1,
    symmetric_sample_bits: 1,
}];

/* DAI widget */

// SND_SOC_DAPM_OUTPUT("PCM1 Out")
// SND_SOC_DAPM_INPUT("PCM1 In")
static mtk_dai_pcm_widgets: [snd_soc_dapm_widget; 2] = [
    snd_soc_dapm_widget { _private: [] },
    snd_soc_dapm_widget { _private: [] },
];

/* DAI route */

static mtk_dai_pcm_routes: [snd_soc_dapm_route; 6] = [
    snd_soc_dapm_route {
        sink: c"PCM1 Playback".as_ptr(),
        control: ptr::null(),
        source: c"O07".as_ptr(),
    },
    snd_soc_dapm_route {
        sink: c"PCM1 Playback".as_ptr(),
        control: ptr::null(),
        source: c"O08".as_ptr(),
    },
    snd_soc_dapm_route {
        sink: c"PCM1 Out".as_ptr(),
        control: ptr::null(),
        source: c"PCM1 Playback".as_ptr(),
    },
    snd_soc_dapm_route {
        sink: c"I09".as_ptr(),
        control: ptr::null(),
        source: c"PCM1 Capture".as_ptr(),
    },
    snd_soc_dapm_route {
        sink: c"I22".as_ptr(),
        control: ptr::null(),
        source: c"PCM1 Capture".as_ptr(),
    },
    snd_soc_dapm_route {
        sink: c"PCM1 Capture".as_ptr(),
        control: ptr::null(),
        source: c"PCM1 In".as_ptr(),
    },
];

unsafe extern "C" fn init_pcmif_priv_data(afe: *mut mtk_base_afe) -> c_int {
    unsafe {
        let afe_priv: *mut mt8365_afe_private = (*afe).platform_priv;
        let pcmif_priv: *mut mt8365_pcm_intf_data;

        pcmif_priv = devm_kzalloc(
            (*afe).dev,
            size_of::<mt8365_pcm_intf_data>(),
            GFP_KERNEL,
        ) as *mut mt8365_pcm_intf_data;
        if pcmif_priv.is_null() {
            return -ENOMEM;
        }

        (*afe_priv).dai_priv[MT8365_AFE_IO_PCM1] = pcmif_priv as *mut c_void;
        0
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn mt8365_dai_pcm_register(afe: *mut mtk_base_afe) -> c_int {
    unsafe {
        let dai: *mut mtk_base_afe_dai;

        dai = devm_kzalloc((*afe).dev, size_of::<mtk_base_afe_dai>(), GFP_KERNEL)
            as *mut mtk_base_afe_dai;
        if dai.is_null() {
            return -ENOMEM;
        }

        list_add(&mut (*dai).list, &mut (*afe).sub_dais);
        (*dai).dai_drivers = mtk_dai_pcm_driver.as_mut_ptr();
        (*dai).num_dai_drivers = mtk_dai_pcm_driver.len() as c_uint;
        (*dai).dapm_widgets = mtk_dai_pcm_widgets.as_ptr();
        (*dai).num_dapm_widgets = mtk_dai_pcm_widgets.len() as c_uint;
        (*dai).dapm_routes = mtk_dai_pcm_routes.as_ptr();
        (*dai).num_dapm_routes = mtk_dai_pcm_routes.len() as c_uint;
        init_pcmif_priv_data(afe)
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
