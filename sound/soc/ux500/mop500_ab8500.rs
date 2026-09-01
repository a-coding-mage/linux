// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) ST-Ericsson SA 2012
 *
 * Author: Ola Lilja <ola.o.lilja@stericsson.com>,
 *         Kristoffer Karlsson <kristoffer.karlsson@stericsson.com>
 *         for ST-Ericsson.
 */

// Linux kernel headers: linux/module.h, linux/device.h, linux/io.h, linux/clk.h, linux/mutex.h
// ALSA/sound headers: sound/soc.h, sound/soc-dapm.h, sound/pcm.h, sound/pcm_params.h
// Project headers: ux500_pcm.h, ux500_msp_dai.h, mop500_ab8500.h, ../codecs/ab8500-codec.h

const TX_SLOT_MONO: u16 = 0x0008;
const TX_SLOT_STEREO: u16 = 0x000a;
const RX_SLOT_MONO: u16 = 0x0001;
const RX_SLOT_STEREO: u16 = 0x0003;
const TX_SLOT_8CH: u16 = 0x00FF;
const RX_SLOT_8CH: u16 = 0x00FF;

const DEF_TX_SLOTS: u16 = TX_SLOT_STEREO;
const DEF_RX_SLOTS: u16 = RX_SLOT_MONO;

const DRIVERMODE_NORMAL: i32 = 0;
const DRIVERMODE_CODEC_ONLY: i32 = 1;

// Slot configuration
static mut tx_slots: u32 = DEF_TX_SLOTS as u32;
static mut rx_slots: u32 = DEF_RX_SLOTS as u32;

// Configuration consistency parameters
// Mutex: mop500_ab8500_params_lock (to be implemented by external code)
static mut mop500_ab8500_usage: usize = 0;
static mut mop500_ab8500_rate: i32 = 0;
static mut mop500_ab8500_channels: i32 = 0;

// Clocks
const ENUM_MCLK: &[&str] = &["SYSCLK", "ULPCLK"];

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Mclk {
    MclkSysclk = 0,
    MclkUlpclk = 1,
}

// SOC_ENUM_SINGLE_EXT_DECL equivalent (macro-generated data structure, preserved as comment)
// soc_enum_mclk: contains enum_mclk array reference and callback info

// Private data for machine-part MOP500<->AB8500
#[repr(C)]
pub struct Mop500Ab8500Drvdata {
    // Clocks
    pub mclk_sel: Mclk,
    pub clk_ptr_intclk: *mut core::ffi::c_void,
    pub clk_ptr_sysclk: *mut core::ffi::c_void,
    pub clk_ptr_ulpclk: *mut core::ffi::c_void,
}

#[inline]
fn get_mclk_str(mclk_sel: Mclk) -> &'static str {
    match mclk_sel {
        Mclk::MclkSysclk => "SYSCLK",
        Mclk::MclkUlpclk => "ULPCLK",
    }
}

fn mop500_ab8500_set_mclk(
    dev: *mut core::ffi::c_void,
    drvdata: *mut Mop500Ab8500Drvdata,
) -> i32 {
    let status: i32;
    let clk_ptr: *mut core::ffi::c_void;

    unsafe {
        if (*drvdata).clk_ptr_intclk as *const _ == core::ptr::null() {
            // dev_err macro call equivalent
            return -5; // -EIO
        }

        clk_ptr = match (*drvdata).mclk_sel {
            Mclk::MclkSysclk => (*drvdata).clk_ptr_sysclk,
            Mclk::MclkUlpclk => (*drvdata).clk_ptr_ulpclk,
        };

        if clk_ptr as *const _ == core::ptr::null() {
            // dev_err macro call equivalent
            return -22; // -EINVAL
        }

        // Call to external function clk_set_parent
        // status = clk_set_parent((*drvdata).clk_ptr_intclk, clk_ptr);
        status = 0; // Placeholder for external function call

        if status != 0 {
            // dev_err macro call
        } else {
            // dev_dbg macro call
        }
    }

    status
}

// Control-events

fn mclk_input_control_get(
    kcontrol: *mut core::ffi::c_void,
    ucontrol: *mut core::ffi::c_void,
) -> i32 {
    // External function calls needed:
    // struct snd_soc_card *card = snd_kcontrol_chip(kcontrol);
    // struct mop500_ab8500_drvdata *drvdata = snd_soc_card_get_drvdata(card);
    // ucontrol->value.enumerated.item[0] = drvdata->mclk_sel;

    0
}

fn mclk_input_control_put(
    kcontrol: *mut core::ffi::c_void,
    ucontrol: *mut core::ffi::c_void,
) -> i32 {
    // External function calls needed:
    // struct snd_soc_card *card = snd_kcontrol_chip(kcontrol);
    // struct mop500_ab8500_drvdata *drvdata = snd_soc_card_get_drvdata(card);
    // unsigned int val = ucontrol->value.enumerated.item[0];

    // if (val > (unsigned int)MCLK_ULPCLK)
    //     return -EINVAL;
    // if (drvdata->mclk_sel == val)
    //     return 0;
    //
    // drvdata->mclk_sel = val;
    //
    // return 1;

    0
}

// Controls
// mop500_ab8500_ctrls array: macro-generated control structures
// SOC_ENUM_EXT("Master Clock Select", soc_enum_mclk, mclk_input_control_get, mclk_input_control_put),
// SOC_DAPM_PIN_SWITCH entries for various audio pins

// ASoC

fn mop500_ab8500_startup(substream: *mut core::ffi::c_void) -> i32 {
    // struct snd_soc_pcm_runtime *rtd = snd_soc_substream_to_rtd(substream);
    // return mop500_ab8500_set_mclk(rtd->card->dev, snd_soc_card_get_drvdata(rtd->card));

    0
}

fn mop500_ab8500_shutdown(substream: *mut core::ffi::c_void) {
    // struct snd_soc_pcm_runtime *rtd = snd_soc_substream_to_rtd(substream);
    // struct device *dev = rtd->card->dev;
    //
    // dev_dbg(dev, "%s: Enter\n", __func__);
    //
    // if (substream->stream == SNDRV_PCM_STREAM_PLAYBACK)
    //     tx_slots = DEF_TX_SLOTS;
    // else
    //     rx_slots = DEF_RX_SLOTS;
}

fn mop500_ab8500_hw_params(
    substream: *mut core::ffi::c_void,
    params: *mut core::ffi::c_void,
) -> i32 {
    // struct snd_soc_pcm_runtime *rtd = snd_soc_substream_to_rtd(substream);
    // struct snd_soc_dai *codec_dai = snd_soc_rtd_to_codec(rtd, 0);
    // struct snd_soc_dai *cpu_dai = snd_soc_rtd_to_cpu(rtd, 0);
    // struct device *dev = rtd->card->dev;
    // unsigned int fmt;
    // int channels, ret = 0, driver_mode, slots;
    // unsigned int sw_codec, sw_cpu;
    // bool is_playback;
    //
    // dev_dbg(dev, "%s: Enter\n", __func__);
    //
    // dev_dbg(dev, "%s: substream->pcm->name = %s\n"
    //     "substream->pcm->id = %s.\n"
    //     "substream->name = %s.\n"
    //     "substream->number = %d.\n",
    //     __func__,
    //     substream->pcm->name,
    //     substream->pcm->id,
    //     substream->name,
    //     substream->number);
    //
    // Ensure configuration consistency between DAIs
    // scoped_guard(mutex, &mop500_ab8500_params_lock) {
    //     if (mop500_ab8500_usage) {
    //         if (mop500_ab8500_rate != params_rate(params) ||
    //             mop500_ab8500_channels != params_channels(params)) {
    //             return -EBUSY;
    //         }
    //     } else {
    //         mop500_ab8500_rate = params_rate(params);
    //         mop500_ab8500_channels = params_channels(params);
    //     }
    //     __set_bit(cpu_dai->id, &mop500_ab8500_usage);
    // }
    //
    // channels = params_channels(params);
    //
    // switch (params_format(params)) {
    // case SNDRV_PCM_FORMAT_S32_LE:
    //     sw_cpu = 32;
    //     break;
    //
    // case SNDRV_PCM_FORMAT_S16_LE:
    //     sw_cpu = 16;
    //     break;
    //
    // default:
    //     return -EINVAL;
    // }
    //
    // Setup codec depending on driver-mode
    // if (channels == 8)
    //     driver_mode = DRIVERMODE_CODEC_ONLY;
    // else
    //     driver_mode = DRIVERMODE_NORMAL;
    //
    // Setup format
    // if (driver_mode == DRIVERMODE_NORMAL) {
    //     fmt = SND_SOC_DAIFMT_DSP_A |
    //         SND_SOC_DAIFMT_CBP_CFP |
    //         SND_SOC_DAIFMT_NB_NF |
    //         SND_SOC_DAIFMT_CONT;
    // } else {
    //     fmt = SND_SOC_DAIFMT_DSP_A |
    //         SND_SOC_DAIFMT_CBP_CFP |
    //         SND_SOC_DAIFMT_NB_NF |
    //         SND_SOC_DAIFMT_GATED;
    // }
    //
    // ret = snd_soc_runtime_set_dai_fmt(rtd, fmt);
    // if (ret)
    //     return ret;
    //
    // Setup TDM-slots
    // is_playback = (substream->stream == SNDRV_PCM_STREAM_PLAYBACK);
    // switch (channels) {
    // case 1:
    //     slots = 16;
    //     tx_slots = (is_playback) ? TX_SLOT_MONO : 0;
    //     rx_slots = (is_playback) ? 0 : RX_SLOT_MONO;
    //     break;
    // case 2:
    //     slots = 16;
    //     tx_slots = (is_playback) ? TX_SLOT_STEREO : 0;
    //     rx_slots = (is_playback) ? 0 : RX_SLOT_STEREO;
    //     break;
    // case 8:
    //     slots = 16;
    //     tx_slots = (is_playback) ? TX_SLOT_8CH : 0;
    //     rx_slots = (is_playback) ? 0 : RX_SLOT_8CH;
    //     break;
    // default:
    //     return -EINVAL;
    // }
    //
    // if (driver_mode == DRIVERMODE_NORMAL)
    //     sw_codec = sw_cpu;
    // else
    //     sw_codec = 20;
    //
    // ret = snd_soc_dai_set_tdm_slot(cpu_dai, tx_slots, rx_slots, slots, sw_cpu);
    // if (ret)
    //     return ret;
    //
    // ret = snd_soc_dai_set_tdm_slot(codec_dai, tx_slots, rx_slots, slots, sw_codec);
    // if (ret)
    //     return ret;
    //
    // return 0;

    0
}

fn mop500_ab8500_hw_free(substream: *mut core::ffi::c_void) -> i32 {
    // struct snd_soc_pcm_runtime *rtd = snd_soc_substream_to_rtd(substream);
    // struct snd_soc_dai *cpu_dai = snd_soc_rtd_to_cpu(rtd, 0);
    //
    // guard(mutex)(&mop500_ab8500_params_lock);
    // __clear_bit(cpu_dai->id, &mop500_ab8500_usage);
    //
    // return 0;

    0
}

// snd_soc_ops structure array
pub struct SndSocOps {
    pub hw_params: Option<fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> i32>,
    pub hw_free: Option<fn(*mut core::ffi::c_void) -> i32>,
    pub startup: Option<fn(*mut core::ffi::c_void) -> i32>,
    pub shutdown: Option<fn(*mut core::ffi::c_void)>,
}

pub const MOP500_AB8500_OPS: &[SndSocOps] = &[SndSocOps {
    hw_params: Some(mop500_ab8500_hw_params),
    hw_free: Some(mop500_ab8500_hw_free),
    startup: Some(mop500_ab8500_startup),
    shutdown: Some(mop500_ab8500_shutdown),
}];

pub fn mop500_ab8500_machine_init(rtd: *mut core::ffi::c_void) -> i32 {
    // struct snd_soc_dapm_context *dapm = snd_soc_card_to_dapm(rtd->card);
    // struct device *dev = rtd->card->dev;
    // struct mop500_ab8500_drvdata *drvdata;
    // int ret;
    //
    // dev_dbg(dev, "%s Enter.\n", __func__);
    //
    // Create driver private-data struct
    // drvdata = devm_kzalloc(dev, sizeof(struct mop500_ab8500_drvdata), GFP_KERNEL);
    //
    // if (!drvdata)
    //     return -ENOMEM;
    //
    // snd_soc_card_set_drvdata(rtd->card, drvdata);
    //
    // Setup clocks
    // drvdata->clk_ptr_sysclk = clk_get(dev, "sysclk");
    // if (IS_ERR(drvdata->clk_ptr_sysclk))
    //     dev_warn(dev, "%s: WARNING: clk_get failed for 'sysclk'!\n", __func__);
    // drvdata->clk_ptr_ulpclk = clk_get(dev, "ulpclk");
    // if (IS_ERR(drvdata->clk_ptr_ulpclk))
    //     dev_warn(dev, "%s: WARNING: clk_get failed for 'ulpclk'!\n", __func__);
    // drvdata->clk_ptr_intclk = clk_get(dev, "intclk");
    // if (IS_ERR(drvdata->clk_ptr_intclk))
    //     dev_warn(dev, "%s: WARNING: clk_get failed for 'intclk'!\n", __func__);
    //
    // Set intclk default parent to ulpclk
    // drvdata->mclk_sel = MCLK_ULPCLK;
    // ret = mop500_ab8500_set_mclk(dev, drvdata);
    // if (ret < 0)
    //     dev_warn(dev, "%s: WARNING: mop500_ab8500_set_mclk!\n", __func__);
    //
    // drvdata->mclk_sel = MCLK_ULPCLK;
    //
    // Add controls
    // ret = snd_soc_add_card_controls(rtd->card, mop500_ab8500_ctrls,
    //         ARRAY_SIZE(mop500_ab8500_ctrls));
    // if (ret < 0) {
    //     pr_err("%s: Failed to add machine-controls (%d)!\n", __func__, ret);
    //     return ret;
    // }
    //
    // ret = snd_soc_dapm_disable_pin(dapm, "Earpiece");
    // ret |= snd_soc_dapm_disable_pin(dapm, "Speaker Left");
    // ret |= snd_soc_dapm_disable_pin(dapm, "Speaker Right");
    // ret |= snd_soc_dapm_disable_pin(dapm, "LineOut Left");
    // ret |= snd_soc_dapm_disable_pin(dapm, "LineOut Right");
    // ret |= snd_soc_dapm_disable_pin(dapm, "Vibra 1");
    // ret |= snd_soc_dapm_disable_pin(dapm, "Vibra 2");
    // ret |= snd_soc_dapm_disable_pin(dapm, "Mic 1");
    // ret |= snd_soc_dapm_disable_pin(dapm, "Mic 2");
    // ret |= snd_soc_dapm_disable_pin(dapm, "LineIn Left");
    // ret |= snd_soc_dapm_disable_pin(dapm, "LineIn Right");
    // ret |= snd_soc_dapm_disable_pin(dapm, "DMic 1");
    // ret |= snd_soc_dapm_disable_pin(dapm, "DMic 2");
    // ret |= snd_soc_dapm_disable_pin(dapm, "DMic 3");
    // ret |= snd_soc_dapm_disable_pin(dapm, "DMic 4");
    // ret |= snd_soc_dapm_disable_pin(dapm, "DMic 5");
    // ret |= snd_soc_dapm_disable_pin(dapm, "DMic 6");
    //
    // return ret;

    0
}

pub fn mop500_ab8500_remove(card: *mut core::ffi::c_void) {
    // struct mop500_ab8500_drvdata *drvdata = snd_soc_card_get_drvdata(card);
    //
    // clk_put(drvdata->clk_ptr_sysclk);
    // clk_put(drvdata->clk_ptr_ulpclk);
    // clk_put(drvdata->clk_ptr_intclk);
    //
    // snd_soc_card_set_drvdata(card, NULL);
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
