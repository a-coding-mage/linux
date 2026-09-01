// SPDX-License-Identifier: GPL-2.0
/*
 * mt8188-mt6359.rs  --  MT8188-MT6359 ALSA SoC machine driver
 *
 * Copyright (c) 2022 MediaTek Inc.
 * Author: Trevor Wu <trevor.wu@mediatek.com>
 */

// Translated from C implementation source. Kernel, ALSA SoC, codec, and
// MediaTek helper declarations are expected to be supplied by surrounding
// bindings/modules corresponding to the original includes.

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::ptr::{null, null_mut};

const CKSYS_AUD_TOP_CFG: c_uint = 0x032c;
const RG_TEST_ON: c_uint = BIT!(0);
const RG_TEST_TYPE: c_uint = BIT!(2);
const CKSYS_AUD_TOP_MON: c_uint = 0x0330;
const TEST_MISO_COUNT_1: c_uint = GENMASK!(3, 0);
const TEST_MISO_COUNT_2: c_uint = GENMASK!(7, 4);
const TEST_MISO_DONE_1: c_uint = BIT!(28);
const TEST_MISO_DONE_2: c_uint = BIT!(29);

const NAU8825_HS_PRESENT: c_uint = BIT!(0);
const RT5682S_HS_PRESENT: c_uint = BIT!(1);
const ES8326_HS_PRESENT: c_uint = BIT!(2);
const MAX98390_TWO_AMP: c_uint = BIT!(3);

/*
 * Maxim MAX98390
 */
const MAX98390_CODEC_DAI: *const c_char = c"max98390-aif1".as_ptr();
const MAX98390_DEV0_NAME: *const c_char = c"max98390.0-0038".as_ptr(); /* rear right */
const MAX98390_DEV1_NAME: *const c_char = c"max98390.0-0039".as_ptr(); /* rear left */
const MAX98390_DEV2_NAME: *const c_char = c"max98390.0-003a".as_ptr(); /* front right */
const MAX98390_DEV3_NAME: *const c_char = c"max98390.0-003b".as_ptr(); /* front left */

/*
 * Nau88l25
 */
const NAU8825_CODEC_DAI: *const c_char = c"nau8825-hifi".as_ptr();

/*
 * ES8326
 */
const ES8326_CODEC_DAI: *const c_char = c"ES8326 HiFi".as_ptr();

const SOF_DMA_DL2: *const c_char = c"SOF_DMA_DL2".as_ptr();
const SOF_DMA_DL3: *const c_char = c"SOF_DMA_DL3".as_ptr();
const SOF_DMA_UL4: *const c_char = c"SOF_DMA_UL4".as_ptr();
const SOF_DMA_UL5: *const c_char = c"SOF_DMA_UL5".as_ptr();

const RT5682S_CODEC_DAI: *const c_char = c"rt5682s-aif1".as_ptr();

/* FE */
SND_SOC_DAILINK_DEFS!(playback2,
		     DAILINK_COMP_ARRAY!(COMP_CPU!(c"DL2")),
		     DAILINK_COMP_ARRAY!(COMP_DUMMY!()),
		     DAILINK_COMP_ARRAY!(COMP_EMPTY!()));

SND_SOC_DAILINK_DEFS!(playback3,
		     DAILINK_COMP_ARRAY!(COMP_CPU!(c"DL3")),
		     DAILINK_COMP_ARRAY!(COMP_DUMMY!()),
		     DAILINK_COMP_ARRAY!(COMP_EMPTY!()));

SND_SOC_DAILINK_DEFS!(playback6,
		     DAILINK_COMP_ARRAY!(COMP_CPU!(c"DL6")),
		     DAILINK_COMP_ARRAY!(COMP_DUMMY!()),
		     DAILINK_COMP_ARRAY!(COMP_EMPTY!()));

SND_SOC_DAILINK_DEFS!(playback7,
		     DAILINK_COMP_ARRAY!(COMP_CPU!(c"DL7")),
		     DAILINK_COMP_ARRAY!(COMP_DUMMY!()),
		     DAILINK_COMP_ARRAY!(COMP_EMPTY!()));

SND_SOC_DAILINK_DEFS!(playback8,
		     DAILINK_COMP_ARRAY!(COMP_CPU!(c"DL8")),
		     DAILINK_COMP_ARRAY!(COMP_DUMMY!()),
		     DAILINK_COMP_ARRAY!(COMP_EMPTY!()));

SND_SOC_DAILINK_DEFS!(playback10,
		     DAILINK_COMP_ARRAY!(COMP_CPU!(c"DL10")),
		     DAILINK_COMP_ARRAY!(COMP_DUMMY!()),
		     DAILINK_COMP_ARRAY!(COMP_EMPTY!()));

SND_SOC_DAILINK_DEFS!(playback11,
		     DAILINK_COMP_ARRAY!(COMP_CPU!(c"DL11")),
		     DAILINK_COMP_ARRAY!(COMP_DUMMY!()),
		     DAILINK_COMP_ARRAY!(COMP_EMPTY!()));

SND_SOC_DAILINK_DEFS!(capture1,
		     DAILINK_COMP_ARRAY!(COMP_CPU!(c"UL1")),
		     DAILINK_COMP_ARRAY!(COMP_DUMMY!()),
		     DAILINK_COMP_ARRAY!(COMP_EMPTY!()));

SND_SOC_DAILINK_DEFS!(capture2,
		     DAILINK_COMP_ARRAY!(COMP_CPU!(c"UL2")),
		     DAILINK_COMP_ARRAY!(COMP_DUMMY!()),
		     DAILINK_COMP_ARRAY!(COMP_EMPTY!()));

SND_SOC_DAILINK_DEFS!(capture3,
		     DAILINK_COMP_ARRAY!(COMP_CPU!(c"UL3")),
		     DAILINK_COMP_ARRAY!(COMP_DUMMY!()),
		     DAILINK_COMP_ARRAY!(COMP_EMPTY!()));

SND_SOC_DAILINK_DEFS!(capture4,
		     DAILINK_COMP_ARRAY!(COMP_CPU!(c"UL4")),
		     DAILINK_COMP_ARRAY!(COMP_DUMMY!()),
		     DAILINK_COMP_ARRAY!(COMP_EMPTY!()));

SND_SOC_DAILINK_DEFS!(capture5,
		     DAILINK_COMP_ARRAY!(COMP_CPU!(c"UL5")),
		     DAILINK_COMP_ARRAY!(COMP_DUMMY!()),
		     DAILINK_COMP_ARRAY!(COMP_EMPTY!()));

SND_SOC_DAILINK_DEFS!(capture6,
		     DAILINK_COMP_ARRAY!(COMP_CPU!(c"UL6")),
		     DAILINK_COMP_ARRAY!(COMP_DUMMY!()),
		     DAILINK_COMP_ARRAY!(COMP_EMPTY!()));

SND_SOC_DAILINK_DEFS!(capture8,
		     DAILINK_COMP_ARRAY!(COMP_CPU!(c"UL8")),
		     DAILINK_COMP_ARRAY!(COMP_DUMMY!()),
		     DAILINK_COMP_ARRAY!(COMP_EMPTY!()));

SND_SOC_DAILINK_DEFS!(capture9,
		     DAILINK_COMP_ARRAY!(COMP_CPU!(c"UL9")),
		     DAILINK_COMP_ARRAY!(COMP_DUMMY!()),
		     DAILINK_COMP_ARRAY!(COMP_EMPTY!()));

SND_SOC_DAILINK_DEFS!(capture10,
		     DAILINK_COMP_ARRAY!(COMP_CPU!(c"UL10")),
		     DAILINK_COMP_ARRAY!(COMP_DUMMY!()),
		     DAILINK_COMP_ARRAY!(COMP_EMPTY!()));

/* BE */
SND_SOC_DAILINK_DEFS!(dl_src,
		     DAILINK_COMP_ARRAY!(COMP_CPU!(c"DL_SRC")),
		     DAILINK_COMP_ARRAY!(COMP_CODEC!(c"mt6359-sound",
						   c"mt6359-snd-codec-aif1")),
		     DAILINK_COMP_ARRAY!(COMP_EMPTY!()));

SND_SOC_DAILINK_DEFS!(DMIC_BE,
		     DAILINK_COMP_ARRAY!(COMP_CPU!(c"DMIC")),
		     DAILINK_COMP_ARRAY!(COMP_DUMMY!()),
		     DAILINK_COMP_ARRAY!(COMP_EMPTY!()));

SND_SOC_DAILINK_DEFS!(dptx,
		     DAILINK_COMP_ARRAY!(COMP_CPU!(c"DPTX")),
		     DAILINK_COMP_ARRAY!(COMP_DUMMY!()),
		     DAILINK_COMP_ARRAY!(COMP_EMPTY!()));

SND_SOC_DAILINK_DEFS!(etdm1_in,
		     DAILINK_COMP_ARRAY!(COMP_CPU!(c"ETDM1_IN")),
		     DAILINK_COMP_ARRAY!(COMP_DUMMY!()),
		     DAILINK_COMP_ARRAY!(COMP_EMPTY!()));

SND_SOC_DAILINK_DEFS!(etdm2_in,
		     DAILINK_COMP_ARRAY!(COMP_CPU!(c"ETDM2_IN")),
		     DAILINK_COMP_ARRAY!(COMP_DUMMY!()),
		     DAILINK_COMP_ARRAY!(COMP_EMPTY!()));

SND_SOC_DAILINK_DEFS!(etdm1_out,
		     DAILINK_COMP_ARRAY!(COMP_CPU!(c"ETDM1_OUT")),
		     DAILINK_COMP_ARRAY!(COMP_DUMMY!()),
		     DAILINK_COMP_ARRAY!(COMP_EMPTY!()));

SND_SOC_DAILINK_DEFS!(etdm2_out,
		     DAILINK_COMP_ARRAY!(COMP_CPU!(c"ETDM2_OUT")),
		     DAILINK_COMP_ARRAY!(COMP_DUMMY!()),
		     DAILINK_COMP_ARRAY!(COMP_EMPTY!()));

SND_SOC_DAILINK_DEFS!(etdm3_out,
		     DAILINK_COMP_ARRAY!(COMP_CPU!(c"ETDM3_OUT")),
		     DAILINK_COMP_ARRAY!(COMP_DUMMY!()),
		     DAILINK_COMP_ARRAY!(COMP_EMPTY!()));

SND_SOC_DAILINK_DEFS!(pcm1,
		     DAILINK_COMP_ARRAY!(COMP_CPU!(c"PCM1")),
		     DAILINK_COMP_ARRAY!(COMP_DUMMY!()),
		     DAILINK_COMP_ARRAY!(COMP_EMPTY!()));

SND_SOC_DAILINK_DEFS!(ul_src,
		     DAILINK_COMP_ARRAY!(COMP_CPU!(c"UL_SRC")),
		     DAILINK_COMP_ARRAY!(COMP_CODEC!(c"mt6359-sound",
						   c"mt6359-snd-codec-aif1")),
		     DAILINK_COMP_ARRAY!(COMP_EMPTY!()));

SND_SOC_DAILINK_DEFS!(AFE_SOF_DL2,
		     DAILINK_COMP_ARRAY!(COMP_CPU!(c"SOF_DL2")),
		     DAILINK_COMP_ARRAY!(COMP_DUMMY!()),
		     DAILINK_COMP_ARRAY!(COMP_EMPTY!()));

SND_SOC_DAILINK_DEFS!(AFE_SOF_DL3,
		     DAILINK_COMP_ARRAY!(COMP_CPU!(c"SOF_DL3")),
		     DAILINK_COMP_ARRAY!(COMP_DUMMY!()),
		     DAILINK_COMP_ARRAY!(COMP_EMPTY!()));

SND_SOC_DAILINK_DEFS!(AFE_SOF_UL4,
		     DAILINK_COMP_ARRAY!(COMP_CPU!(c"SOF_UL4")),
		     DAILINK_COMP_ARRAY!(COMP_DUMMY!()),
		     DAILINK_COMP_ARRAY!(COMP_EMPTY!()));

SND_SOC_DAILINK_DEFS!(AFE_SOF_UL5,
		     DAILINK_COMP_ARRAY!(COMP_CPU!(c"SOF_UL5")),
		     DAILINK_COMP_ARRAY!(COMP_DUMMY!()),
		     DAILINK_COMP_ARRAY!(COMP_EMPTY!()));

static_ref! {
static G_SOF_CONN_STREAMS: [sof_conn_stream; 4] = [
	sof_conn_stream { sof_link: c"AFE_SOF_DL2".as_ptr(), sof_dma: SOF_DMA_DL2, stream_dir: SNDRV_PCM_STREAM_PLAYBACK },
	sof_conn_stream { sof_link: c"AFE_SOF_DL3".as_ptr(), sof_dma: SOF_DMA_DL3, stream_dir: SNDRV_PCM_STREAM_PLAYBACK },
	sof_conn_stream { sof_link: c"AFE_SOF_UL4".as_ptr(), sof_dma: SOF_DMA_UL4, stream_dir: SNDRV_PCM_STREAM_CAPTURE },
	sof_conn_stream { sof_link: c"AFE_SOF_UL5".as_ptr(), sof_dma: SOF_DMA_UL5, stream_dir: SNDRV_PCM_STREAM_CAPTURE },
];
}

#[repr(C)]
enum mt8188_jacks {
	MT8188_JACK_HEADSET,
	MT8188_JACK_DP,
	MT8188_JACK_HDMI,
	MT8188_JACK_MAX,
}

static_mut! {
static mut MT8188_HDMI_JACK_PINS: [snd_soc_jack_pin; 1] = [
	snd_soc_jack_pin { pin: c"HDMI".as_ptr(), mask: SND_JACK_AVOUT },
];
static mut MT8188_DP_JACK_PINS: [snd_soc_jack_pin; 1] = [
	snd_soc_jack_pin { pin: c"DP".as_ptr(), mask: SND_JACK_AVOUT },
];
static mut NAU8825_JACK_PINS: [snd_soc_jack_pin; 2] = [
	snd_soc_jack_pin { pin: c"Headphone Jack".as_ptr(), mask: SND_JACK_HEADPHONE },
	snd_soc_jack_pin { pin: c"Headset Mic".as_ptr(), mask: SND_JACK_MICROPHONE },
];
static mut MT8188_HEADSET_JACK_PINS: [snd_soc_jack_pin; 2] = [
	snd_soc_jack_pin { pin: c"Headphone".as_ptr(), mask: SND_JACK_HEADPHONE },
	snd_soc_jack_pin { pin: c"Headset Mic".as_ptr(), mask: SND_JACK_MICROPHONE },
];
}

static_ref! {
static MT8188_DUMB_SPK_CONTROLS: [snd_kcontrol_new; 1] = [SOC_DAPM_PIN_SWITCH!(c"Ext Spk")];
static MT8188_DUMB_SPK_WIDGETS: [snd_soc_dapm_widget; 1] = [SND_SOC_DAPM_SPK!(c"Ext Spk", null())];
static MT8188_DUAL_SPK_CONTROLS: [snd_kcontrol_new; 2] = [SOC_DAPM_PIN_SWITCH!(c"Left Spk"), SOC_DAPM_PIN_SWITCH!(c"Right Spk")];
static MT8188_DUAL_SPK_WIDGETS: [snd_soc_dapm_widget; 2] = [SND_SOC_DAPM_SPK!(c"Left Spk", null()), SND_SOC_DAPM_SPK!(c"Right Spk", null())];
static MT8188_REAR_SPK_CONTROLS: [snd_kcontrol_new; 2] = [SOC_DAPM_PIN_SWITCH!(c"Rear Left Spk"), SOC_DAPM_PIN_SWITCH!(c"Rear Right Spk")];
static MT8188_REAR_SPK_WIDGETS: [snd_soc_dapm_widget; 2] = [SND_SOC_DAPM_SPK!(c"Rear Left Spk", null()), SND_SOC_DAPM_SPK!(c"Rear Right Spk", null())];

static MT8188_MT6359_WIDGETS: [snd_soc_dapm_widget; 13] = [
	SND_SOC_DAPM_HP!(c"Headphone", null()),
	SND_SOC_DAPM_MIC!(c"Headset Mic", null()),
	SND_SOC_DAPM_MIC!(c"AP DMIC", null()),
	SND_SOC_DAPM_SINK!(c"HDMI"),
	SND_SOC_DAPM_SINK!(c"DP"),
	SND_SOC_DAPM_MIXER!(SOF_DMA_DL2, SND_SOC_NOPM, 0, 0, null(), 0),
	SND_SOC_DAPM_MIXER!(SOF_DMA_DL3, SND_SOC_NOPM, 0, 0, null(), 0),
	SND_SOC_DAPM_MIXER!(SOF_DMA_UL4, SND_SOC_NOPM, 0, 0, null(), 0),
	SND_SOC_DAPM_MIXER!(SOF_DMA_UL5, SND_SOC_NOPM, 0, 0, null(), 0),

	/* dynamic pinctrl */
	SND_SOC_DAPM_PINCTRL!(c"ETDM_SPK_PIN", c"aud_etdm_spk_on", c"aud_etdm_spk_off"),
	SND_SOC_DAPM_PINCTRL!(c"ETDM_HP_PIN", c"aud_etdm_hp_on", c"aud_etdm_hp_off"),
	SND_SOC_DAPM_PINCTRL!(c"MTKAIF_PIN", c"aud_mtkaif_on", c"aud_mtkaif_off"),
];

static MT8188_MT6359_CONTROLS: [snd_kcontrol_new; 2] = [
	SOC_DAPM_PIN_SWITCH!(c"Headphone"),
	SOC_DAPM_PIN_SWITCH!(c"Headset Mic"),
];

static MT8188_NAU8825_WIDGETS: [snd_soc_dapm_widget; 1] = [SND_SOC_DAPM_HP!(c"Headphone Jack", null())];
static MT8188_NAU8825_CONTROLS: [snd_kcontrol_new; 1] = [SOC_DAPM_PIN_SWITCH!(c"Headphone Jack")];

static MT8188_MT6359_ROUTES: [snd_soc_dapm_route; 8] = [
	/* SOF Uplink */
	snd_soc_dapm_route { sink: SOF_DMA_UL4, control: null(), source: c"O034".as_ptr() },
	snd_soc_dapm_route { sink: SOF_DMA_UL4, control: null(), source: c"O035".as_ptr() },
	snd_soc_dapm_route { sink: SOF_DMA_UL5, control: null(), source: c"O036".as_ptr() },
	snd_soc_dapm_route { sink: SOF_DMA_UL5, control: null(), source: c"O037".as_ptr() },
	/* SOF Downlink */
	snd_soc_dapm_route { sink: c"I070".as_ptr(), control: null(), source: SOF_DMA_DL2 },
	snd_soc_dapm_route { sink: c"I071".as_ptr(), control: null(), source: SOF_DMA_DL2 },
	snd_soc_dapm_route { sink: c"I020".as_ptr(), control: null(), source: SOF_DMA_DL3 },
	snd_soc_dapm_route { sink: c"I021".as_ptr(), control: null(), source: SOF_DMA_DL3 },
];
}

unsafe extern "C" fn mt8188_mt6359_mtkaif_calibration(rtd: *mut snd_soc_pcm_runtime) -> c_int {
	let cmpnt_afe = snd_soc_rtdcom_lookup(rtd, AFE_PCM_NAME);
	let cmpnt_codec = (*snd_soc_rtd_to_codec(rtd, 0)).component;
	let mut pin_w: *mut snd_soc_dapm_widget = null_mut();
	let mut w: *mut snd_soc_dapm_widget;
	let afe: *mut mtk_base_afe;
	let afe_priv: *mut mt8188_afe_private;
	let param: *mut mtkaif_param;
	let chosen_phase_1: c_int;
	let chosen_phase_2: c_int;
	let mut prev_cycle_1: c_int = 0;
	let mut prev_cycle_2: c_int = 0;
	let mut test_done_1: u8;
	let mut test_done_2: u8;
	let mut cycle_1: c_int;
	let mut cycle_2: c_int;
	let mut mtkaif_chosen_phase = [-1i32; MT8188_MTKAIF_MISO_NUM as usize];
	let mut mtkaif_phase_cycle = [0i32; MT8188_MTKAIF_MISO_NUM as usize];
	let mtkaif_calibration_num_phase: c_int;
	let mut mtkaif_calibration_ok: bool;
	let mut monitor: u32 = 0;
	let mut counter: c_int;
	let mut phase: c_int;
	let mut i: c_int;

	if cmpnt_afe.is_null() {
		return -EINVAL;
	}

	afe = snd_soc_component_get_drvdata(cmpnt_afe) as *mut mtk_base_afe;
	afe_priv = (*afe).platform_priv as *mut mt8188_afe_private;
	param = &mut (*afe_priv).mtkaif_params;

	dev_dbg!((*afe).dev, c"%s(), start\n", __func__!());

	(*param).mtkaif_calibration_ok = false;
	i = 0;
	while i < MT8188_MTKAIF_MISO_NUM {
		(*param).mtkaif_chosen_phase[i as usize] = -1;
		(*param).mtkaif_phase_cycle[i as usize] = 0;
		mtkaif_chosen_phase[i as usize] = -1;
		mtkaif_phase_cycle[i as usize] = 0;
		i += 1;
	}

	if IS_ERR((*afe_priv).topckgen as *const c_void) {
		dev_info!((*afe).dev, c"%s() Cannot find topckgen controller\n", __func__!());
		return 0;
	}

	for_each_card_widgets!((*rtd).card, w, {
		if strcmp((*w).name, c"MTKAIF_PIN".as_ptr()) == 0 {
			pin_w = w;
			break;
		}
	});

	if !pin_w.is_null() {
		snd_soc_dapm_pinctrl_event(pin_w, null_mut(), SND_SOC_DAPM_PRE_PMU);
	} else {
		dev_dbg!((*afe).dev, c"%s(), no pinmux widget, please check if default on\n", __func__!());
	}

	pm_runtime_get_sync((*afe).dev);
	mt6359_mtkaif_calibration_enable(cmpnt_codec);

	/* set test type to synchronizer pulse */
	regmap_write((*afe_priv).topckgen, CKSYS_AUD_TOP_CFG, RG_TEST_TYPE);
	mtkaif_calibration_num_phase = 42;	/* mt6359: 0 ~ 42 */
	mtkaif_calibration_ok = true;

	phase = 0;
	while phase <= mtkaif_calibration_num_phase && mtkaif_calibration_ok {
		mt6359_set_mtkaif_calibration_phase(cmpnt_codec, phase, phase, phase);

		regmap_set_bits((*afe_priv).topckgen, CKSYS_AUD_TOP_CFG, RG_TEST_ON);

		test_done_1 = 0;
		test_done_2 = 0;

		cycle_1 = -1;
		cycle_2 = -1;

		counter = 0;
		while (test_done_1 & test_done_2) == 0 {
			regmap_read((*afe_priv).topckgen, CKSYS_AUD_TOP_MON, &mut monitor);
			test_done_1 = FIELD_GET!(TEST_MISO_DONE_1, monitor) as u8;
			test_done_2 = FIELD_GET!(TEST_MISO_DONE_2, monitor) as u8;

			if test_done_1 == 1 {
				cycle_1 = FIELD_GET!(TEST_MISO_COUNT_1, monitor) as c_int;
			}

			if test_done_2 == 1 {
				cycle_2 = FIELD_GET!(TEST_MISO_COUNT_2, monitor) as c_int;
			}

			/* handle if never test done */
			counter += 1;
			if counter > 10000 {
				dev_err!((*afe).dev, c"%s(), test fail, cycle_1 %d, cycle_2 %d, monitor 0x%x\n",
					__func__!(), cycle_1, cycle_2, monitor);
				mtkaif_calibration_ok = false;
				break;
			}
		}

		if phase == 0 {
			prev_cycle_1 = cycle_1;
			prev_cycle_2 = cycle_2;
		}

		if cycle_1 != prev_cycle_1 &&
		    mtkaif_chosen_phase[MT8188_MTKAIF_MISO_0 as usize] < 0 {
			mtkaif_chosen_phase[MT8188_MTKAIF_MISO_0 as usize] = phase - 1;
			mtkaif_phase_cycle[MT8188_MTKAIF_MISO_0 as usize] = prev_cycle_1;
		}

		if cycle_2 != prev_cycle_2 &&
		    mtkaif_chosen_phase[MT8188_MTKAIF_MISO_1 as usize] < 0 {
			mtkaif_chosen_phase[MT8188_MTKAIF_MISO_1 as usize] = phase - 1;
			mtkaif_phase_cycle[MT8188_MTKAIF_MISO_1 as usize] = prev_cycle_2;
		}

		regmap_clear_bits((*afe_priv).topckgen, CKSYS_AUD_TOP_CFG, RG_TEST_ON);

		if mtkaif_chosen_phase[MT8188_MTKAIF_MISO_0 as usize] >= 0 &&
		   mtkaif_chosen_phase[MT8188_MTKAIF_MISO_1 as usize] >= 0 {
			break;
		}
		phase += 1;
	}

	if mtkaif_chosen_phase[MT8188_MTKAIF_MISO_0 as usize] < 0 {
		mtkaif_calibration_ok = false;
		chosen_phase_1 = 0;
	} else {
		chosen_phase_1 = mtkaif_chosen_phase[MT8188_MTKAIF_MISO_0 as usize];
	}

	if mtkaif_chosen_phase[MT8188_MTKAIF_MISO_1 as usize] < 0 {
		mtkaif_calibration_ok = false;
		chosen_phase_2 = 0;
	} else {
		chosen_phase_2 = mtkaif_chosen_phase[MT8188_MTKAIF_MISO_1 as usize];
	}

	mt6359_set_mtkaif_calibration_phase(cmpnt_codec, chosen_phase_1, chosen_phase_2, 0);

	mt6359_mtkaif_calibration_disable(cmpnt_codec);
	pm_runtime_put((*afe).dev);

	(*param).mtkaif_calibration_ok = mtkaif_calibration_ok;
	(*param).mtkaif_chosen_phase[MT8188_MTKAIF_MISO_0 as usize] = chosen_phase_1;
	(*param).mtkaif_chosen_phase[MT8188_MTKAIF_MISO_1 as usize] = chosen_phase_2;

	i = 0;
	while i < MT8188_MTKAIF_MISO_NUM {
		(*param).mtkaif_phase_cycle[i as usize] = mtkaif_phase_cycle[i as usize];
		i += 1;
	}

	if !pin_w.is_null() {
		snd_soc_dapm_pinctrl_event(pin_w, null_mut(), SND_SOC_DAPM_POST_PMD);
	}

	dev_dbg!((*afe).dev, c"%s(), end, calibration ok %d\n",
		__func__!(), (*param).mtkaif_calibration_ok as c_int);

	0
}

unsafe extern "C" fn mt8188_mt6359_accdet_init(rtd: *mut snd_soc_pcm_runtime) -> c_int {
	let soc_card_data = snd_soc_card_get_drvdata((*rtd).card) as *mut mtk_soc_card_data;
	let jack = &mut (*(*soc_card_data).card_data).jacks[mt8188_jacks::MT8188_JACK_HEADSET as usize] as *mut snd_soc_jack;
	let mut ret: c_int;

	if (*soc_card_data).accdet.is_null() {
		return 0;
	}

	ret = snd_soc_card_jack_new_pins((*rtd).card, c"Headset Jack".as_ptr(),
				   SND_JACK_HEADSET | SND_JACK_BTN_0 |
				   SND_JACK_BTN_1 | SND_JACK_BTN_2 |
				   SND_JACK_BTN_3,
				   jack, MT8188_HEADSET_JACK_PINS.as_mut_ptr(),
				   ARRAY_SIZE!(MT8188_HEADSET_JACK_PINS));
	if ret != 0 {
		dev_err!((*rtd).dev, c"Headset Jack create failed: %d\n", ret);
		return ret;
	}

	ret = mt6359_accdet_enable_jack_detect((*soc_card_data).accdet, jack);
	if ret != 0 {
		dev_err!((*rtd).dev, c"Headset Jack enable failed: %d\n", ret);
		return ret;
	}

	0
}

unsafe extern "C" fn mt8188_mt6359_init(rtd: *mut snd_soc_pcm_runtime) -> c_int {
	let cmpnt_codec = (*snd_soc_rtd_to_codec(rtd, 0)).component;

	/* set mtkaif protocol */
	mt6359_set_mtkaif_protocol(cmpnt_codec, MT6359_MTKAIF_PROTOCOL_2_CLK_P2);

	/* mtkaif calibration */
	mt8188_mt6359_mtkaif_calibration(rtd);

	mt8188_mt6359_accdet_init(rtd);

	0
}

const DAI_LINK_DL2_FE: usize = 0;
const DAI_LINK_DL3_FE: usize = 1;
const DAI_LINK_DL6_FE: usize = 2;
const DAI_LINK_DL7_FE: usize = 3;
const DAI_LINK_DL8_FE: usize = 4;
const DAI_LINK_DL10_FE: usize = 5;
const DAI_LINK_DL11_FE: usize = 6;
const DAI_LINK_UL1_FE: usize = 7;
const DAI_LINK_UL2_FE: usize = 8;
const DAI_LINK_UL3_FE: usize = 9;
const DAI_LINK_UL4_FE: usize = 10;
const DAI_LINK_UL5_FE: usize = 11;
const DAI_LINK_UL6_FE: usize = 12;
const DAI_LINK_UL8_FE: usize = 13;
const DAI_LINK_UL9_FE: usize = 14;
const DAI_LINK_UL10_FE: usize = 15;
const DAI_LINK_DL_SRC_BE: usize = 16;
const DAI_LINK_DMIC_BE: usize = 17;
const DAI_LINK_DPTX_BE: usize = 18;
const DAI_LINK_ETDM1_IN_BE: usize = 19;
const DAI_LINK_ETDM2_IN_BE: usize = 20;
const DAI_LINK_ETDM1_OUT_BE: usize = 21;
const DAI_LINK_ETDM2_OUT_BE: usize = 22;
const DAI_LINK_ETDM3_OUT_BE: usize = 23;
const DAI_LINK_PCM1_BE: usize = 24;
const DAI_LINK_UL_SRC_BE: usize = 25;
const DAI_LINK_REGULAR_LAST: usize = DAI_LINK_UL_SRC_BE;
const DAI_LINK_SOF_START: usize = 26;
const DAI_LINK_SOF_DL2_BE: usize = DAI_LINK_SOF_START;
const DAI_LINK_SOF_DL3_BE: usize = 27;
const DAI_LINK_SOF_UL4_BE: usize = 28;
const DAI_LINK_SOF_UL5_BE: usize = 29;
const DAI_LINK_SOF_END: usize = DAI_LINK_SOF_UL5_BE;

const DAI_LINK_REGULAR_NUM: usize = DAI_LINK_REGULAR_LAST + 1;

unsafe extern "C" fn mt8188_dptx_hw_params(substream: *mut snd_pcm_substream,
				 params: *mut snd_pcm_hw_params) -> c_int {
	let rtd = snd_soc_substream_to_rtd(substream);
	let rate: c_uint = params_rate(params);
	let mclk_fs_ratio: c_uint = 256;
	let mclk_fs: c_uint = rate.wrapping_mul(mclk_fs_ratio);
	let dai = snd_soc_rtd_to_cpu(rtd, 0);

	snd_soc_dai_set_sysclk(dai, 0, mclk_fs, SND_SOC_CLOCK_OUT)
}

static_ref! {
static MT8188_DPTX_OPS: snd_soc_ops = snd_soc_ops {
	hw_params: Some(mt8188_dptx_hw_params),
	..ZEROED!()
};
}

unsafe extern "C" fn mt8188_dptx_hw_params_fixup(_rtd: *mut snd_soc_pcm_runtime,
				       params: *mut snd_pcm_hw_params) -> c_int {
	/* fix BE i2s format to 32bit, clean param mask first */
	snd_mask_reset_range(hw_param_mask(params, SNDRV_PCM_HW_PARAM_FORMAT),
			     0, SNDRV_PCM_FORMAT_LAST);

	params_set_format(params, SNDRV_PCM_FORMAT_S32_LE);

	0
}

unsafe extern "C" fn mt8188_hdmi_codec_init(rtd: *mut snd_soc_pcm_runtime) -> c_int {
	let soc_card_data = snd_soc_card_get_drvdata((*rtd).card) as *mut mtk_soc_card_data;
	let jack = &mut (*(*soc_card_data).card_data).jacks[mt8188_jacks::MT8188_JACK_HDMI as usize] as *mut snd_soc_jack;
	let component = (*snd_soc_rtd_to_codec(rtd, 0)).component;
	let mut ret: c_int = 0;

	ret = snd_soc_card_jack_new_pins((*rtd).card, c"HDMI Jack".as_ptr(),
					 SND_JACK_AVOUT, jack,
					 MT8188_HDMI_JACK_PINS.as_mut_ptr(),
					 ARRAY_SIZE!(MT8188_HDMI_JACK_PINS));
	if ret != 0 {
		dev_err!((*rtd).dev, c"%s, new jack failed: %d\n", __func__!(), ret);
		return ret;
	}

	ret = snd_soc_component_set_jack(component, jack, null_mut());
	if ret != 0 {
		dev_err!((*rtd).dev, c"%s, set jack failed on %s (ret=%d)\n",
			__func__!(), (*component).name, ret);
		return ret;
	}

	0
}

unsafe extern "C" fn mt8188_dptx_codec_init(rtd: *mut snd_soc_pcm_runtime) -> c_int {
	let soc_card_data = snd_soc_card_get_drvdata((*rtd).card) as *mut mtk_soc_card_data;
	let jack = &mut (*(*soc_card_data).card_data).jacks[mt8188_jacks::MT8188_JACK_DP as usize] as *mut snd_soc_jack;
	let component = (*snd_soc_rtd_to_codec(rtd, 0)).component;
	let mut ret: c_int = 0;

	ret = snd_soc_card_jack_new_pins((*rtd).card, c"DP Jack".as_ptr(), SND_JACK_AVOUT,
					 jack, MT8188_DP_JACK_PINS.as_mut_ptr(),
					 ARRAY_SIZE!(MT8188_DP_JACK_PINS));
	if ret != 0 {
		dev_err!((*rtd).dev, c"%s, new jack failed: %d\n", __func__!(), ret);
		return ret;
	}

	ret = snd_soc_component_set_jack(component, jack, null_mut());
	if ret != 0 {
		dev_err!((*rtd).dev, c"%s, set jack failed on %s (ret=%d)\n",
			__func__!(), (*component).name, ret);
		return ret;
	}

	0
}

unsafe extern "C" fn mt8188_dumb_amp_init(rtd: *mut snd_soc_pcm_runtime) -> c_int {
	let card = (*rtd).card;
	let dapm = snd_soc_card_to_dapm(card);
	let mut ret: c_int = 0;

	ret = snd_soc_dapm_new_controls(dapm, MT8188_DUMB_SPK_WIDGETS.as_ptr(),
					ARRAY_SIZE!(MT8188_DUMB_SPK_WIDGETS));
	if ret != 0 {
		dev_err!((*rtd).dev, c"unable to add Dumb Speaker dapm, ret %d\n", ret);
		return ret;
	}

	ret = snd_soc_add_card_controls(card, MT8188_DUMB_SPK_CONTROLS.as_ptr(),
					ARRAY_SIZE!(MT8188_DUMB_SPK_CONTROLS));
	if ret != 0 {
		dev_err!((*rtd).dev, c"unable to add Dumb card controls, ret %d\n", ret);
		return ret;
	}

	0
}

unsafe extern "C" fn mt8188_max98390_hw_params(substream: *mut snd_pcm_substream,
				     params: *mut snd_pcm_hw_params) -> c_int {
	let rtd = snd_soc_substream_to_rtd(substream);
	let bit_width: c_uint = params_width(params);
	let cpu_dai = snd_soc_rtd_to_cpu(rtd, 0);
	let mut codec_dai: *mut snd_soc_dai;
	let mut i: c_int;

	snd_soc_dai_set_tdm_slot(cpu_dai, 0xf, 0xf, 4, bit_width);

	for_each_rtd_codec_dais!(rtd, i, codec_dai, {
		if strcmp((*(*codec_dai).component).name, MAX98390_DEV0_NAME) == 0 {
			snd_soc_dai_set_tdm_slot(codec_dai, 0x8, 0x3, 4, bit_width);
		}

		if strcmp((*(*codec_dai).component).name, MAX98390_DEV1_NAME) == 0 {
			snd_soc_dai_set_tdm_slot(codec_dai, 0x4, 0x3, 4, bit_width);
		}

		if strcmp((*(*codec_dai).component).name, MAX98390_DEV2_NAME) == 0 {
			snd_soc_dai_set_tdm_slot(codec_dai, 0x2, 0x3, 4, bit_width);
		}

		if strcmp((*(*codec_dai).component).name, MAX98390_DEV3_NAME) == 0 {
			snd_soc_dai_set_tdm_slot(codec_dai, 0x1, 0x3, 4, bit_width);
		}
	});
	0
}

static_ref! {
static MT8188_MAX98390_OPS: snd_soc_ops = snd_soc_ops {
	hw_params: Some(mt8188_max98390_hw_params),
	..ZEROED!()
};
}

unsafe extern "C" fn mt8188_max98390_codec_init(rtd: *mut snd_soc_pcm_runtime) -> c_int {
	let card = (*rtd).card;
	let dapm = snd_soc_card_to_dapm(card);
	let mut ret: c_int;

	/* add regular speakers dapm route */
	ret = snd_soc_dapm_new_controls(dapm, MT8188_DUAL_SPK_WIDGETS.as_ptr(),
					ARRAY_SIZE!(MT8188_DUAL_SPK_WIDGETS));
	if ret != 0 {
		dev_err!((*rtd).dev, c"unable to add Left/Right Speaker widget, ret %d\n", ret);
		return ret;
	}

	ret = snd_soc_add_card_controls(card, MT8188_DUAL_SPK_CONTROLS.as_ptr(),
					ARRAY_SIZE!(MT8188_DUAL_SPK_CONTROLS));
	if ret != 0 {
		dev_err!((*rtd).dev, c"unable to add Left/Right card controls, ret %d\n", ret);
		return ret;
	}

	if (*(*rtd).dai_link).num_codecs <= 2 {
		return 0;
	}

	/* add widgets/controls/dapm for rear speakers */
	ret = snd_soc_dapm_new_controls(dapm, MT8188_REAR_SPK_WIDGETS.as_ptr(),
					ARRAY_SIZE!(MT8188_REAR_SPK_WIDGETS));
	if ret != 0 {
		dev_err!((*rtd).dev, c"unable to add Rear Speaker widget, ret %d\n", ret);
		/* Don't need to add routes if widget addition failed */
		return ret;
	}

	ret = snd_soc_add_card_controls(card, MT8188_REAR_SPK_CONTROLS.as_ptr(),
					ARRAY_SIZE!(MT8188_REAR_SPK_CONTROLS));
	if ret != 0 {
		dev_err!((*rtd).dev, c"unable to add Rear card controls, ret %d\n", ret);
		return ret;
	}

	0
}

unsafe extern "C" fn mt8188_headset_codec_init(rtd: *mut snd_soc_pcm_runtime) -> c_int {
	let card = (*rtd).card;
	let dapm = snd_soc_card_to_dapm(card);
	let soc_card_data = snd_soc_card_get_drvdata((*rtd).card) as *mut mtk_soc_card_data;
	let jack = &mut (*(*soc_card_data).card_data).jacks[mt8188_jacks::MT8188_JACK_HEADSET as usize] as *mut snd_soc_jack;
	let component = (*snd_soc_rtd_to_codec(rtd, 0)).component;
	let card_data = (*soc_card_data).card_data;
	let mut ret: c_int;

	ret = snd_soc_dapm_new_controls(dapm, MT8188_NAU8825_WIDGETS.as_ptr(),
					ARRAY_SIZE!(MT8188_NAU8825_WIDGETS));
	if ret != 0 {
		dev_err!((*rtd).dev, c"unable to add nau8825 card widget, ret %d\n", ret);
		return ret;
	}

	ret = snd_soc_add_card_controls(card, MT8188_NAU8825_CONTROLS.as_ptr(),
					ARRAY_SIZE!(MT8188_NAU8825_CONTROLS));
	if ret != 0 {
		dev_err!((*rtd).dev, c"unable to add nau8825 card controls, ret %d\n", ret);
		return ret;
	}

	ret = snd_soc_card_jack_new_pins((*rtd).card, c"Headset Jack".as_ptr(),
					 SND_JACK_HEADSET | SND_JACK_BTN_0 |
					 SND_JACK_BTN_1 | SND_JACK_BTN_2 |
					 SND_JACK_BTN_3,
					 jack,
					 NAU8825_JACK_PINS.as_mut_ptr(),
					 ARRAY_SIZE!(NAU8825_JACK_PINS));
	if ret != 0 {
		dev_err!((*rtd).dev, c"Headset Jack creation failed: %d\n", ret);
		return ret;
	}

	if ((*card_data).flags & ES8326_HS_PRESENT) != 0 {
		snd_jack_set_key((*jack).jack, SND_JACK_BTN_0, KEY_PLAYPAUSE);
		snd_jack_set_key((*jack).jack, SND_JACK_BTN_1, KEY_VOLUMEUP);
		snd_jack_set_key((*jack).jack, SND_JACK_BTN_2, KEY_VOLUMEDOWN);
		snd_jack_set_key((*jack).jack, SND_JACK_BTN_3, KEY_VOICECOMMAND);
	} else {
		snd_jack_set_key((*jack).jack, SND_JACK_BTN_0, KEY_PLAYPAUSE);
		snd_jack_set_key((*jack).jack, SND_JACK_BTN_1, KEY_VOICECOMMAND);
		snd_jack_set_key((*jack).jack, SND_JACK_BTN_2, KEY_VOLUMEUP);
		snd_jack_set_key((*jack).jack, SND_JACK_BTN_3, KEY_VOLUMEDOWN);
	}

	ret = snd_soc_component_set_jack(component, jack, null_mut());

	if ret != 0 {
		dev_err!((*rtd).dev, c"Headset Jack call-back failed: %d\n", ret);
		return ret;
	}

	0
}

unsafe extern "C" fn mt8188_headset_codec_exit(rtd: *mut snd_soc_pcm_runtime) {
	let component = (*snd_soc_rtd_to_codec(rtd, 0)).component;

	snd_soc_component_set_jack(component, null_mut(), null_mut());
}

unsafe extern "C" fn mt8188_nau8825_hw_params(substream: *mut snd_pcm_substream,
				    params: *mut snd_pcm_hw_params) -> c_int {
	let rtd = snd_soc_substream_to_rtd(substream);
	let codec_dai = snd_soc_rtd_to_codec(rtd, 0);
	let rate: c_uint = params_rate(params);
	let bit_width: c_uint = params_width(params);
	let clk_freq: c_int;
	let mut ret: c_int;

	clk_freq = rate.wrapping_mul(2).wrapping_mul(bit_width) as c_int;

	/* Configure clock for codec */
	ret = snd_soc_dai_set_sysclk(codec_dai, NAU8825_CLK_FLL_BLK, 0,
				     SND_SOC_CLOCK_IN);
	if ret < 0 {
		dev_err!((*codec_dai).dev, c"can't set BCLK clock %d\n", ret);
		return ret;
	}

	/* Configure pll for codec */
	ret = snd_soc_dai_set_pll(codec_dai, 0, 0, clk_freq as c_uint,
				  params_rate(params).wrapping_mul(256));
	if ret < 0 {
		dev_err!((*codec_dai).dev, c"can't set BCLK: %d\n", ret);
		return ret;
	}

	0
}

static_ref! {
static MT8188_NAU8825_OPS: snd_soc_ops = snd_soc_ops {
	hw_params: Some(mt8188_nau8825_hw_params),
	..ZEROED!()
};
}

unsafe extern "C" fn mt8188_rt5682s_i2s_hw_params(substream: *mut snd_pcm_substream,
					params: *mut snd_pcm_hw_params) -> c_int {
	let rtd = snd_soc_substream_to_rtd(substream);
	let card = (*rtd).card;
	let cpu_dai = snd_soc_rtd_to_cpu(rtd, 0);
	let codec_dai = snd_soc_rtd_to_codec(rtd, 0);
	let rate: c_uint = params_rate(params);
	let bitwidth: c_int;
	let mut ret: c_int;

	bitwidth = snd_pcm_format_width(params_format(params));
	if bitwidth < 0 {
		dev_err!((*card).dev, c"invalid bit width: %d\n", bitwidth);
		return bitwidth;
	}

	ret = snd_soc_dai_set_tdm_slot(codec_dai, 0x00, 0x0, 0x2, bitwidth as c_uint);
	if ret != 0 {
		dev_err!((*card).dev, c"failed to set tdm slot\n");
		return ret;
	}

	ret = snd_soc_dai_set_pll(codec_dai, RT5682_PLL1, RT5682_PLL1_S_BCLK1,
				  rate.wrapping_mul(32), rate.wrapping_mul(512));
	if ret != 0 {
		dev_err!((*card).dev, c"failed to set pll\n");
		return ret;
	}

	ret = snd_soc_dai_set_sysclk(codec_dai, RT5682_SCLK_S_PLL1,
				     rate.wrapping_mul(512), SND_SOC_CLOCK_IN);
	if ret != 0 {
		dev_err!((*card).dev, c"failed to set sysclk\n");
		return ret;
	}

	snd_soc_dai_set_sysclk(cpu_dai, 0, rate.wrapping_mul(128),
				      SND_SOC_CLOCK_OUT)
}

static_ref! {
static MT8188_RT5682S_I2S_OPS: snd_soc_ops = snd_soc_ops {
	hw_params: Some(mt8188_rt5682s_i2s_hw_params),
	..ZEROED!()
};
}

unsafe extern "C" fn mt8188_sof_be_hw_params(substream: *mut snd_pcm_substream,
				   _params: *mut snd_pcm_hw_params) -> c_int {
	let rtd = snd_soc_substream_to_rtd(substream);
	let mut cmpnt_afe: *mut snd_soc_component = null_mut();
	let mut runtime: *mut snd_soc_pcm_runtime;

	/* find afe component */
	for_each_card_rtds!((*rtd).card, runtime, {
		cmpnt_afe = snd_soc_rtdcom_lookup(runtime, AFE_PCM_NAME);
		if !cmpnt_afe.is_null() {
			break;
		}
	});

	if !cmpnt_afe.is_null() && !pm_runtime_active((*cmpnt_afe).dev) {
		dev_err!((*rtd).dev, c"afe pm runtime is not active!!\n");
		return -EINVAL;
	}

	0
}

static_ref! {
static MT8188_SOF_BE_OPS: snd_soc_ops = snd_soc_ops {
	hw_params: Some(mt8188_sof_be_hw_params),
	..ZEROED!()
};
}

unsafe extern "C" fn mt8188_es8326_hw_params(substream: *mut snd_pcm_substream,
				 params: *mut snd_pcm_hw_params) -> c_int {
	let rtd = snd_soc_substream_to_rtd(substream);
	let cpu_dai = snd_soc_rtd_to_cpu(rtd, 0);
	let codec_dai = snd_soc_rtd_to_codec(rtd, 0);
	let rate: c_uint = params_rate(params);
	let mut ret: c_int;

	/* Configure MCLK for codec */
	ret = snd_soc_dai_set_sysclk(codec_dai, 0, rate.wrapping_mul(256), SND_SOC_CLOCK_IN);
	if ret < 0 {
		dev_err!((*codec_dai).dev, c"can't set MCLK %d\n", ret);
		return ret;
	}

	/* Configure MCLK for cpu */
	snd_soc_dai_set_sysclk(cpu_dai, 0, rate.wrapping_mul(256), SND_SOC_CLOCK_OUT)
}

static_ref! {
static MT8188_ES8326_OPS: snd_soc_ops = snd_soc_ops {
	hw_params: Some(mt8188_es8326_hw_params),
	..ZEROED!()
};
}

static_mut! {
static mut MT8188_MT6359_DAI_LINKS: [snd_soc_dai_link; 30] = [
	/* FE */
	[DAI_LINK_DL2_FE] snd_soc_dai_link { name: c"DL2_FE".as_ptr(), stream_name: c"DL2 Playback".as_ptr(), trigger: [SND_SOC_DPCM_TRIGGER_POST, SND_SOC_DPCM_TRIGGER_POST], dynamic: 1, playback_only: 1, dpcm_merged_chan: 1, dpcm_merged_rate: 1, dpcm_merged_format: 1, SND_SOC_DAILINK_REG!(playback2), ..ZEROED!() },
	[DAI_LINK_DL3_FE] snd_soc_dai_link { name: c"DL3_FE".as_ptr(), stream_name: c"DL3 Playback".as_ptr(), trigger: [SND_SOC_DPCM_TRIGGER_POST, SND_SOC_DPCM_TRIGGER_POST], dynamic: 1, playback_only: 1, dpcm_merged_chan: 1, dpcm_merged_rate: 1, dpcm_merged_format: 1, SND_SOC_DAILINK_REG!(playback3), ..ZEROED!() },
	[DAI_LINK_DL6_FE] snd_soc_dai_link { name: c"DL6_FE".as_ptr(), stream_name: c"DL6 Playback".as_ptr(), trigger: [SND_SOC_DPCM_TRIGGER_POST, SND_SOC_DPCM_TRIGGER_POST], dynamic: 1, playback_only: 1, dpcm_merged_chan: 1, dpcm_merged_rate: 1, dpcm_merged_format: 1, SND_SOC_DAILINK_REG!(playback6), ..ZEROED!() },
	[DAI_LINK_DL7_FE] snd_soc_dai_link { name: c"DL7_FE".as_ptr(), stream_name: c"DL7 Playback".as_ptr(), trigger: [SND_SOC_DPCM_TRIGGER_PRE, SND_SOC_DPCM_TRIGGER_PRE], dynamic: 1, playback_only: 1, SND_SOC_DAILINK_REG!(playback7), ..ZEROED!() },
	[DAI_LINK_DL8_FE] snd_soc_dai_link { name: c"DL8_FE".as_ptr(), stream_name: c"DL8 Playback".as_ptr(), trigger: [SND_SOC_DPCM_TRIGGER_POST, SND_SOC_DPCM_TRIGGER_POST], dynamic: 1, playback_only: 1, SND_SOC_DAILINK_REG!(playback8), ..ZEROED!() },
	[DAI_LINK_DL10_FE] snd_soc_dai_link { name: c"DL10_FE".as_ptr(), stream_name: c"DL10 Playback".as_ptr(), trigger: [SND_SOC_DPCM_TRIGGER_POST, SND_SOC_DPCM_TRIGGER_POST], dynamic: 1, playback_only: 1, SND_SOC_DAILINK_REG!(playback10), ..ZEROED!() },
	[DAI_LINK_DL11_FE] snd_soc_dai_link { name: c"DL11_FE".as_ptr(), stream_name: c"DL11 Playback".as_ptr(), trigger: [SND_SOC_DPCM_TRIGGER_POST, SND_SOC_DPCM_TRIGGER_POST], dynamic: 1, playback_only: 1, SND_SOC_DAILINK_REG!(playback11), ..ZEROED!() },
	[DAI_LINK_UL1_FE] snd_soc_dai_link { name: c"UL1_FE".as_ptr(), stream_name: c"UL1 Capture".as_ptr(), trigger: [SND_SOC_DPCM_TRIGGER_PRE, SND_SOC_DPCM_TRIGGER_PRE], dynamic: 1, capture_only: 1, SND_SOC_DAILINK_REG!(capture1), ..ZEROED!() },
	[DAI_LINK_UL2_FE] snd_soc_dai_link { name: c"UL2_FE".as_ptr(), stream_name: c"UL2 Capture".as_ptr(), trigger: [SND_SOC_DPCM_TRIGGER_POST, SND_SOC_DPCM_TRIGGER_POST], dynamic: 1, capture_only: 1, SND_SOC_DAILINK_REG!(capture2), ..ZEROED!() },
	[DAI_LINK_UL3_FE] snd_soc_dai_link { name: c"UL3_FE".as_ptr(), stream_name: c"UL3 Capture".as_ptr(), trigger: [SND_SOC_DPCM_TRIGGER_POST, SND_SOC_DPCM_TRIGGER_POST], dynamic: 1, capture_only: 1, SND_SOC_DAILINK_REG!(capture3), ..ZEROED!() },
	[DAI_LINK_UL4_FE] snd_soc_dai_link { name: c"UL4_FE".as_ptr(), stream_name: c"UL4 Capture".as_ptr(), trigger: [SND_SOC_DPCM_TRIGGER_POST, SND_SOC_DPCM_TRIGGER_POST], dynamic: 1, capture_only: 1, dpcm_merged_chan: 1, dpcm_merged_rate: 1, dpcm_merged_format: 1, SND_SOC_DAILINK_REG!(capture4), ..ZEROED!() },
	[DAI_LINK_UL5_FE] snd_soc_dai_link { name: c"UL5_FE".as_ptr(), stream_name: c"UL5 Capture".as_ptr(), trigger: [SND_SOC_DPCM_TRIGGER_POST, SND_SOC_DPCM_TRIGGER_POST], dynamic: 1, capture_only: 1, dpcm_merged_chan: 1, dpcm_merged_rate: 1, dpcm_merged_format: 1, SND_SOC_DAILINK_REG!(capture5), ..ZEROED!() },
	[DAI_LINK_UL6_FE] snd_soc_dai_link { name: c"UL6_FE".as_ptr(), stream_name: c"UL6 Capture".as_ptr(), trigger: [SND_SOC_DPCM_TRIGGER_PRE, SND_SOC_DPCM_TRIGGER_PRE], dynamic: 1, capture_only: 1, SND_SOC_DAILINK_REG!(capture6), ..ZEROED!() },
	[DAI_LINK_UL8_FE] snd_soc_dai_link { name: c"UL8_FE".as_ptr(), stream_name: c"UL8 Capture".as_ptr(), trigger: [SND_SOC_DPCM_TRIGGER_POST, SND_SOC_DPCM_TRIGGER_POST], dynamic: 1, capture_only: 1, SND_SOC_DAILINK_REG!(capture8), ..ZEROED!() },
	[DAI_LINK_UL9_FE] snd_soc_dai_link { name: c"UL9_FE".as_ptr(), stream_name: c"UL9 Capture".as_ptr(), trigger: [SND_SOC_DPCM_TRIGGER_POST, SND_SOC_DPCM_TRIGGER_POST], dynamic: 1, capture_only: 1, SND_SOC_DAILINK_REG!(capture9), ..ZEROED!() },
	[DAI_LINK_UL10_FE] snd_soc_dai_link { name: c"UL10_FE".as_ptr(), stream_name: c"UL10 Capture".as_ptr(), trigger: [SND_SOC_DPCM_TRIGGER_POST, SND_SOC_DPCM_TRIGGER_POST], dynamic: 1, capture_only: 1, SND_SOC_DAILINK_REG!(capture10), ..ZEROED!() },
	/* BE */
	[DAI_LINK_DL_SRC_BE] snd_soc_dai_link { name: c"DL_SRC_BE".as_ptr(), no_pcm: 1, playback_only: 1, SND_SOC_DAILINK_REG!(dl_src), ..ZEROED!() },
	[DAI_LINK_DMIC_BE] snd_soc_dai_link { name: c"DMIC_BE".as_ptr(), no_pcm: 1, capture_only: 1, ignore_suspend: 1, SND_SOC_DAILINK_REG!(DMIC_BE), ..ZEROED!() },
	[DAI_LINK_DPTX_BE] snd_soc_dai_link { name: c"DPTX_BE".as_ptr(), ops: &MT8188_DPTX_OPS, be_hw_params_fixup: Some(mt8188_dptx_hw_params_fixup), no_pcm: 1, playback_only: 1, SND_SOC_DAILINK_REG!(dptx), ..ZEROED!() },
	[DAI_LINK_ETDM1_IN_BE] snd_soc_dai_link { name: c"ETDM1_IN_BE".as_ptr(), no_pcm: 1, dai_fmt: SND_SOC_DAIFMT_I2S | SND_SOC_DAIFMT_NB_NF | SND_SOC_DAIFMT_CBP_CFP, capture_only: 1, ignore_suspend: 1, SND_SOC_DAILINK_REG!(etdm1_in), ..ZEROED!() },
	[DAI_LINK_ETDM2_IN_BE] snd_soc_dai_link { name: c"ETDM2_IN_BE".as_ptr(), no_pcm: 1, dai_fmt: SND_SOC_DAIFMT_I2S | SND_SOC_DAIFMT_NB_NF | SND_SOC_DAIFMT_CBP_CFP, capture_only: 1, SND_SOC_DAILINK_REG!(etdm2_in), ..ZEROED!() },
	[DAI_LINK_ETDM1_OUT_BE] snd_soc_dai_link { name: c"ETDM1_OUT_BE".as_ptr(), no_pcm: 1, dai_fmt: SND_SOC_DAIFMT_I2S | SND_SOC_DAIFMT_NB_NF | SND_SOC_DAIFMT_CBC_CFC, playback_only: 1, SND_SOC_DAILINK_REG!(etdm1_out), ..ZEROED!() },
	[DAI_LINK_ETDM2_OUT_BE] snd_soc_dai_link { name: c"ETDM2_OUT_BE".as_ptr(), no_pcm: 1, dai_fmt: SND_SOC_DAIFMT_I2S | SND_SOC_DAIFMT_NB_NF | SND_SOC_DAIFMT_CBC_CFC, playback_only: 1, SND_SOC_DAILINK_REG!(etdm2_out), ..ZEROED!() },
	[DAI_LINK_ETDM3_OUT_BE] snd_soc_dai_link { name: c"ETDM3_OUT_BE".as_ptr(), no_pcm: 1, dai_fmt: SND_SOC_DAIFMT_I2S | SND_SOC_DAIFMT_NB_NF | SND_SOC_DAIFMT_CBC_CFC, playback_only: 1, SND_SOC_DAILINK_REG!(etdm3_out), ..ZEROED!() },
	[DAI_LINK_PCM1_BE] snd_soc_dai_link { name: c"PCM1_BE".as_ptr(), no_pcm: 1, dai_fmt: SND_SOC_DAIFMT_I2S | SND_SOC_DAIFMT_NB_NF | SND_SOC_DAIFMT_CBC_CFC, SND_SOC_DAILINK_REG!(pcm1), ..ZEROED!() },
	[DAI_LINK_UL_SRC_BE] snd_soc_dai_link { name: c"UL_SRC_BE".as_ptr(), no_pcm: 1, capture_only: 1, SND_SOC_DAILINK_REG!(ul_src), ..ZEROED!() },

	/* SOF BE */
	[DAI_LINK_SOF_DL2_BE] snd_soc_dai_link { name: c"AFE_SOF_DL2".as_ptr(), no_pcm: 1, playback_only: 1, ops: &MT8188_SOF_BE_OPS, SND_SOC_DAILINK_REG!(AFE_SOF_DL2), ..ZEROED!() },
	[DAI_LINK_SOF_DL3_BE] snd_soc_dai_link { name: c"AFE_SOF_DL3".as_ptr(), no_pcm: 1, playback_only: 1, ops: &MT8188_SOF_BE_OPS, SND_SOC_DAILINK_REG!(AFE_SOF_DL3), ..ZEROED!() },
	[DAI_LINK_SOF_UL4_BE] snd_soc_dai_link { name: c"AFE_SOF_UL4".as_ptr(), no_pcm: 1, capture_only: 1, ops: &MT8188_SOF_BE_OPS, SND_SOC_DAILINK_REG!(AFE_SOF_UL4), ..ZEROED!() },
	[DAI_LINK_SOF_UL5_BE] snd_soc_dai_link { name: c"AFE_SOF_UL5".as_ptr(), no_pcm: 1, capture_only: 1, ops: &MT8188_SOF_BE_OPS, SND_SOC_DAILINK_REG!(AFE_SOF_UL5), ..ZEROED!() },
];
}

unsafe extern "C" fn mt8188_fixup_controls(card: *mut snd_soc_card) {
	let soc_card_data = snd_soc_card_get_drvdata(card) as *mut mtk_soc_card_data;
	let card_data = (*soc_card_data).card_data;
	let mut kctl: *mut snd_kcontrol;

	if ((*card_data).flags & (NAU8825_HS_PRESENT | RT5682S_HS_PRESENT | ES8326_HS_PRESENT)) != 0 {
		let mut w: *mut snd_soc_dapm_widget;
		let mut next_w: *mut snd_soc_dapm_widget;

		for_each_card_widgets_safe!(card, w, next_w, {
			if strcmp((*w).name, c"Headphone".as_ptr()) != 0 {
				continue;
			}

			snd_soc_dapm_free_widget(w);
		});

		kctl = snd_ctl_find_id_mixer((*card).snd_card, c"Headphone Switch".as_ptr());
		if !kctl.is_null() {
			snd_ctl_remove((*card).snd_card, kctl);
		} else {
			dev_warn!((*card).dev, c"Cannot find ctl : Headphone Switch\n");
		}
	}
}

static_mut! {
static mut MT8188_MT6359_SOC_CARD: snd_soc_card = snd_soc_card {
	owner: THIS_MODULE,
	dai_link: MT8188_MT6359_DAI_LINKS.as_mut_ptr(),
	num_links: ARRAY_SIZE!(MT8188_MT6359_DAI_LINKS),
	dapm_widgets: MT8188_MT6359_WIDGETS.as_ptr(),
	num_dapm_widgets: ARRAY_SIZE!(MT8188_MT6359_WIDGETS),
	dapm_routes: MT8188_MT6359_ROUTES.as_ptr(),
	num_dapm_routes: ARRAY_SIZE!(MT8188_MT6359_ROUTES),
	controls: MT8188_MT6359_CONTROLS.as_ptr(),
	num_controls: ARRAY_SIZE!(MT8188_MT6359_CONTROLS),
	fixup_controls: Some(mt8188_fixup_controls),
	..ZEROED!()
};
}

unsafe extern "C" fn mt8188_mt6359_soc_card_probe(soc_card_data: *mut mtk_soc_card_data, legacy: bool) -> c_int {
	let card_data = (*soc_card_data).card_data;
	let card = (*(*soc_card_data).card_data).card;
	let mut dai_link: *mut snd_soc_dai_link;
	let mut init_mt6359 = false;
	let mut init_es8326 = false;
	let mut init_nau8825 = false;
	let mut init_rt5682s = false;
	let mut init_max98390 = false;
	let mut init_dumb = false;
	let mut i: c_int;

	if legacy {
		return -EINVAL;
	}

	for_each_card_prelinks!(card, i, dai_link, {
		if strcmp((*dai_link).name, c"DPTX_BE".as_ptr()) == 0 {
			if (*dai_link).num_codecs != 0 &&
			    !snd_soc_dlc_is_dummy((*dai_link).codecs) {
				(*dai_link).init = Some(mt8188_dptx_codec_init);
			}
		} else if strcmp((*dai_link).name, c"ETDM3_OUT_BE".as_ptr()) == 0 {
			if (*dai_link).num_codecs != 0 &&
			    !snd_soc_dlc_is_dummy((*dai_link).codecs) {
				(*dai_link).init = Some(mt8188_hdmi_codec_init);
			}
		} else if strcmp((*dai_link).name, c"DL_SRC_BE".as_ptr()) == 0 ||
			   strcmp((*dai_link).name, c"UL_SRC_BE".as_ptr()) == 0 {
			if !init_mt6359 {
				(*dai_link).init = Some(mt8188_mt6359_init);
				init_mt6359 = true;
			}
		} else if strcmp((*dai_link).name, c"ETDM1_OUT_BE".as_ptr()) == 0 ||
			   strcmp((*dai_link).name, c"ETDM2_OUT_BE".as_ptr()) == 0 ||
			   strcmp((*dai_link).name, c"ETDM1_IN_BE".as_ptr()) == 0 ||
			   strcmp((*dai_link).name, c"ETDM2_IN_BE".as_ptr()) == 0 {
			if (*dai_link).num_codecs == 0 {
				continue;
			}

			if strcmp((*(*dai_link).codecs).dai_name, MAX98390_CODEC_DAI) == 0 {
				/*
				 * The TDM protocol settings with fixed 4 slots are defined in
				 * mt8188_max98390_ops. Two amps is I2S mode,
				 * SOC and codec don't require TDM settings.
				 */
				if ((*card_data).flags & MAX98390_TWO_AMP) == 0 {
					(*dai_link).ops = &MT8188_MAX98390_OPS;
				}
				if !init_max98390 {
					(*dai_link).init = Some(mt8188_max98390_codec_init);
					init_max98390 = true;
				}
			} else if strcmp((*(*dai_link).codecs).dai_name, NAU8825_CODEC_DAI) == 0 {
				(*dai_link).ops = &MT8188_NAU8825_OPS;
				if !init_nau8825 {
					(*dai_link).init = Some(mt8188_headset_codec_init);
					(*dai_link).exit = Some(mt8188_headset_codec_exit);
					init_nau8825 = true;
				}
			} else if strcmp((*(*dai_link).codecs).dai_name, RT5682S_CODEC_DAI) == 0 {
				(*dai_link).ops = &MT8188_RT5682S_I2S_OPS;
				if !init_rt5682s {
					(*dai_link).init = Some(mt8188_headset_codec_init);
					(*dai_link).exit = Some(mt8188_headset_codec_exit);
					init_rt5682s = true;
				}
			} else if strcmp((*(*dai_link).codecs).dai_name, ES8326_CODEC_DAI) == 0 {
				(*dai_link).ops = &MT8188_ES8326_OPS;
				if !init_es8326 {
					(*dai_link).init = Some(mt8188_headset_codec_init);
					(*dai_link).exit = Some(mt8188_headset_codec_exit);
					init_es8326 = true;
				}
			} else {
				if !snd_soc_dlc_is_dummy((*dai_link).codecs) {
					if !init_dumb {
						(*dai_link).init = Some(mt8188_dumb_amp_init);
						init_dumb = true;
					}
				}
			}
		}
	});

	0
}

static_ref! {
static MT8188_SOF_PRIV: mtk_sof_priv = mtk_sof_priv {
	conn_streams: G_SOF_CONN_STREAMS.as_ptr(),
	num_streams: ARRAY_SIZE!(G_SOF_CONN_STREAMS),
	..ZEROED!()
};

static MT8188_EVB_PLATFORM_CARD_DATA: mtk_platform_card_data = mtk_platform_card_data {
	card: &raw mut MT8188_MT6359_SOC_CARD,
	num_jacks: mt8188_jacks::MT8188_JACK_MAX as c_int,
	..ZEROED!()
};

static MT8188_EVB_CARD: mtk_soundcard_pdata = mtk_soundcard_pdata {
	card_name: c"mt8188_mt6359".as_ptr(),
	card_data: &MT8188_EVB_PLATFORM_CARD_DATA,
	sof_priv: &MT8188_SOF_PRIV,
	soc_probe: Some(mt8188_mt6359_soc_card_probe),
	..ZEROED!()
};

static MT8188_NAU8825_PLATFORM_CARD_DATA: mtk_platform_card_data = mtk_platform_card_data {
	card: &raw mut MT8188_MT6359_SOC_CARD,
	num_jacks: mt8188_jacks::MT8188_JACK_MAX as c_int,
	flags: NAU8825_HS_PRESENT,
	..ZEROED!()
};

static MT8188_NAU8825_CARD: mtk_soundcard_pdata = mtk_soundcard_pdata {
	card_name: c"mt8188_nau8825".as_ptr(),
	card_data: &MT8188_NAU8825_PLATFORM_CARD_DATA,
	sof_priv: &MT8188_SOF_PRIV,
	soc_probe: Some(mt8188_mt6359_soc_card_probe),
	..ZEROED!()
};

static MT8188_RT5682S_PLATFORM_CARD_DATA: mtk_platform_card_data = mtk_platform_card_data {
	card: &raw mut MT8188_MT6359_SOC_CARD,
	num_jacks: mt8188_jacks::MT8188_JACK_MAX as c_int,
	flags: RT5682S_HS_PRESENT | MAX98390_TWO_AMP,
	..ZEROED!()
};

static MT8188_RT5682S_CARD: mtk_soundcard_pdata = mtk_soundcard_pdata {
	card_name: c"mt8188_rt5682s".as_ptr(),
	card_data: &MT8188_RT5682S_PLATFORM_CARD_DATA,
	sof_priv: &MT8188_SOF_PRIV,
	soc_probe: Some(mt8188_mt6359_soc_card_probe),
	..ZEROED!()
};

static MT8188_ES8326_PLATFORM_CARD_DATA: mtk_platform_card_data = mtk_platform_card_data {
	card: &raw mut MT8188_MT6359_SOC_CARD,
	num_jacks: mt8188_jacks::MT8188_JACK_MAX as c_int,
	flags: ES8326_HS_PRESENT | MAX98390_TWO_AMP,
	..ZEROED!()
};

static MT8188_ES8326_CARD: mtk_soundcard_pdata = mtk_soundcard_pdata {
	card_name: c"mt8188_es8326".as_ptr(),
	card_data: &MT8188_ES8326_PLATFORM_CARD_DATA,
	sof_priv: &MT8188_SOF_PRIV,
	soc_probe: Some(mt8188_mt6359_soc_card_probe),
	..ZEROED!()
};

static MT8188_MT6359_DT_MATCH: [of_device_id; 5] = [
	of_device_id { compatible: c"mediatek,mt8188-mt6359-evb".as_ptr(), data: &MT8188_EVB_CARD as *const _ as *const c_void, ..ZEROED!() },
	of_device_id { compatible: c"mediatek,mt8188-nau8825".as_ptr(), data: &MT8188_NAU8825_CARD as *const _ as *const c_void, ..ZEROED!() },
	of_device_id { compatible: c"mediatek,mt8188-rt5682s".as_ptr(), data: &MT8188_RT5682S_CARD as *const _ as *const c_void, ..ZEROED!() },
	of_device_id { compatible: c"mediatek,mt8188-es8326".as_ptr(), data: &MT8188_ES8326_CARD as *const _ as *const c_void, ..ZEROED!() },
	of_device_id { /* sentinel */ ..ZEROED!() },
];
}
MODULE_DEVICE_TABLE!(of, MT8188_MT6359_DT_MATCH);

static_mut! {
static mut MT8188_MT6359_DRIVER: platform_driver = platform_driver {
	driver: device_driver {
		name: c"mt8188_mt6359".as_ptr(),
		of_match_table: MT8188_MT6359_DT_MATCH.as_ptr(),
		pm: &snd_soc_pm_ops,
		..ZEROED!()
	},
	probe: Some(mtk_soundcard_common_probe),
	..ZEROED!()
};
}

module_platform_driver!(MT8188_MT6359_DRIVER);

/* Module information */
MODULE_DESCRIPTION!(c"MT8188-MT6359 ALSA SoC machine driver");
MODULE_AUTHOR!(c"Trevor Wu <trevor.wu@mediatek.com>");
MODULE_LICENSE!(c"GPL");
MODULE_ALIAS!(c"mt8188 mt6359 soc card");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
