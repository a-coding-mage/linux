// SPDX-License-Identifier: GPL-2.0-only
/*
 * ASoC machine driver for Intel Broadwell platforms with RT5650 codec
 *
 * Copyright 2019, The Chromium OS Authors.  All rights reserved.
 */

// C dependencies translated as external symbols/types:
// linux/delay.h, linux/gpio/consumer.h, linux/module.h,
// linux/platform_device.h, sound/core.h, sound/jack.h, sound/pcm.h,
// sound/pcm_params.h, sound/soc.h, sound/soc-acpi.h,
// ../../codecs/rt5645.h

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::ptr;

#[repr(C)]
pub struct gpio_desc {
	_private: [u8; 0],
}

#[repr(C)]
pub struct device {
	pub platform_data: *mut c_void,
}

#[repr(C)]
pub struct platform_device {
	pub dev: device,
}

#[repr(C)]
pub struct snd_soc_component {
	pub dev: *mut device,
}

#[repr(C)]
pub struct snd_soc_dai {
	pub component: *mut snd_soc_component,
}

#[repr(C)]
pub struct snd_soc_card {
	pub name: *const c_char,
	pub driver_name: *const c_char,
	pub owner: *mut c_void,
	pub dai_link: *mut snd_soc_dai_link,
	pub num_links: c_uint,
	pub dapm_widgets: *const snd_soc_dapm_widget,
	pub num_dapm_widgets: c_uint,
	pub dapm_routes: *const snd_soc_dapm_route,
	pub num_dapm_routes: c_uint,
	pub controls: *const snd_kcontrol_new,
	pub num_controls: c_uint,
	pub fully_routed: bool,
	pub dev: *mut device,
}

#[repr(C)]
pub struct snd_soc_pcm_runtime {
	pub dev: *mut device,
	pub card: *mut snd_soc_card,
}

#[repr(C)]
pub struct snd_interval {
	pub min: c_uint,
	pub max: c_uint,
}

#[repr(C)]
pub struct snd_mask {
	_private: [u8; 0],
}

#[repr(C)]
pub struct snd_pcm_hw_params {
	_private: [u8; 0],
}

#[repr(C)]
pub struct snd_pcm_hardware {
	pub channels_max: c_uint,
}

#[repr(C)]
pub struct snd_pcm_runtime {
	pub hw: snd_pcm_hardware,
}

#[repr(C)]
pub struct snd_pcm_substream {
	pub runtime: *mut snd_pcm_runtime,
	pub stream: c_int,
}

#[repr(C)]
pub struct snd_pcm_hw_constraint_list {
	pub count: c_uint,
	pub list: *const c_uint,
	pub mask: c_uint,
}

#[repr(C)]
pub struct snd_soc_jack {
	_private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_jack_pin {
	pub pin: *const c_char,
	pub mask: c_int,
}

#[repr(C)]
pub struct snd_soc_dapm_widget {
	_private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_dapm_route {
	pub sink: *const c_char,
	pub control: *const c_char,
	pub source: *const c_char,
}

#[repr(C)]
pub struct snd_kcontrol_new {
	_private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_ops {
	pub startup: Option<unsafe extern "C" fn(*mut snd_pcm_substream) -> c_int>,
	pub hw_params:
		Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_pcm_hw_params) -> c_int>,
}

#[repr(C)]
pub struct snd_soc_dai_link_component {
	pub name: *const c_char,
	pub dai_name: *const c_char,
}

#[repr(C)]
pub struct snd_soc_dai_link {
	pub name: *const c_char,
	pub stream_name: *const c_char,
	pub id: c_int,
	pub nonatomic: c_uint,
	pub dynamic: c_uint,
	pub no_pcm: c_uint,
	pub dai_fmt: c_uint,
	pub ignore_pmdown_time: c_uint,
	pub ops: *const snd_soc_ops,
	pub trigger: [c_int; 2],
	pub be_hw_params_fixup:
		Option<unsafe extern "C" fn(*mut snd_soc_pcm_runtime, *mut snd_pcm_hw_params) -> c_int>,
	pub init: Option<unsafe extern "C" fn(*mut snd_soc_pcm_runtime) -> c_int>,
	pub cpus: *mut snd_soc_dai_link_component,
	pub num_cpus: c_uint,
	pub codecs: *mut snd_soc_dai_link_component,
	pub num_codecs: c_uint,
	pub platforms: *mut snd_soc_dai_link_component,
	pub num_platforms: c_uint,
}

#[repr(C)]
pub struct snd_soc_acpi_mach_params {
	pub platform: *const c_char,
}

#[repr(C)]
pub struct snd_soc_acpi_mach {
	pub mach_params: snd_soc_acpi_mach_params,
}

#[repr(C)]
pub struct dev_pm_ops {
	_private: [u8; 0],
}

#[repr(C)]
pub struct device_driver {
	pub name: *const c_char,
	pub pm: *const dev_pm_ops,
}

#[repr(C)]
pub struct platform_driver {
	pub probe: Option<unsafe extern "C" fn(*mut platform_device) -> c_int>,
	pub driver: device_driver,
}

#[repr(C)]
struct bdw_rt5650_priv {
	gpio_hp_en: *mut gpio_desc,
	component: *mut snd_soc_component,
}

const SNDRV_PCM_HW_PARAM_RATE: c_int = 0;
const SNDRV_PCM_HW_PARAM_CHANNELS: c_int = 1;
const SNDRV_PCM_HW_PARAM_FORMAT: c_int = 2;
const SNDRV_PCM_FORMAT_S24_LE: c_int = 6;
const SNDRV_PCM_STREAM_CAPTURE: c_int = 1;
const SND_JACK_HEADPHONE: c_int = 0x0001;
const SND_JACK_MICROPHONE: c_int = 0x0002;
const RT5645_PLL1_S_MCLK: c_int = 0;
const RT5645_SCLK_S_PLL1: c_int = 1;
const SND_SOC_CLOCK_IN: c_int = 0;
const RT5645_DA_STEREO_FILTER: c_uint = 1 << 0;
const RT5645_DA_MONO_L_FILTER: c_uint = 1 << 1;
const RT5645_DA_MONO_R_FILTER: c_uint = 1 << 2;
const RT5645_AD_STEREO_FILTER: c_uint = 1 << 3;
const RT5645_AD_MONO_L_FILTER: c_uint = 1 << 4;
const RT5645_AD_MONO_R_FILTER: c_uint = 1 << 5;
const RT5645_CLK_SEL_I2S1_ASRC: c_uint = 1;
const SND_SOC_DPCM_TRIGGER_POST: c_int = 1;
const SND_SOC_DAIFMT_DSP_B: c_uint = 0x0004;
const SND_SOC_DAIFMT_NB_NF: c_uint = 0x0000;
const SND_SOC_DAIFMT_CBC_CFC: c_uint = 0x0040;
const GFP_KERNEL: c_uint = 0;
const ENOMEM: c_int = 12;

unsafe extern "C" {
	static mut THIS_MODULE: *mut c_void;
	static snd_soc_pm_ops: dev_pm_ops;

	fn hw_param_interval(params: *mut snd_pcm_hw_params, var: c_int) -> *mut snd_interval;
	fn hw_param_mask(params: *mut snd_pcm_hw_params, var: c_int) -> *mut snd_mask;
	fn snd_mask_set_format(mask: *mut snd_mask, val: c_int);
	fn snd_soc_substream_to_rtd(substream: *mut snd_pcm_substream) -> *mut snd_soc_pcm_runtime;
	fn snd_soc_rtd_to_codec(rtd: *mut snd_soc_pcm_runtime, num: c_int) -> *mut snd_soc_dai;
	fn snd_soc_dai_set_pll(
		dai: *mut snd_soc_dai,
		pll_id: c_int,
		source: c_int,
		freq_in: c_uint,
		freq_out: c_uint,
	) -> c_int;
	fn snd_soc_dai_set_sysclk(
		dai: *mut snd_soc_dai,
		clk_id: c_int,
		freq: c_uint,
		dir: c_int,
	) -> c_int;
	fn snd_pcm_hw_constraint_list(
		runtime: *mut snd_pcm_runtime,
		cond: c_uint,
		var: c_int,
		l: *const snd_pcm_hw_constraint_list,
	) -> c_int;
	fn snd_soc_card_get_drvdata(card: *mut snd_soc_card) -> *mut c_void;
	fn rt5645_sel_asrc_clk_src(
		component: *mut snd_soc_component,
		filter_mask: c_uint,
		clk_src: c_uint,
	);
	fn snd_soc_dai_set_tdm_slot(
		dai: *mut snd_soc_dai,
		tx_mask: c_uint,
		rx_mask: c_uint,
		slots: c_int,
		slot_width: c_int,
	) -> c_int;
	fn snd_soc_card_jack_new_pins(
		card: *mut snd_soc_card,
		id: *const c_char,
		r#type: c_int,
		jack: *mut snd_soc_jack,
		pins: *mut snd_soc_jack_pin,
		num_pins: c_uint,
	) -> c_int;
	fn rt5645_set_jack_detect(
		component: *mut snd_soc_component,
		hp_jack: *mut snd_soc_jack,
		mic_jack: *mut snd_soc_jack,
		btn_jack: *mut snd_soc_jack,
	);
	fn dev_err(dev: *mut device, fmt: *const c_char, ...);
	fn devm_kzalloc(dev: *mut device, size: usize, gfp: c_uint) -> *mut c_void;
	fn snd_soc_fixup_dai_links_platform_name(
		card: *mut snd_soc_card,
		platform_name: *const c_char,
	) -> c_int;
	fn snd_soc_acpi_sof_parent(dev: *mut device) -> bool;
	fn snd_soc_card_set_drvdata(card: *mut snd_soc_card, data: *mut c_void);
	fn devm_snd_soc_register_card(dev: *mut device, card: *mut snd_soc_card) -> c_int;
}

// Macro-created widgets from SND_SOC_DAPM_HP/SPK/MIC.
static bdw_rt5650_widgets: [snd_soc_dapm_widget; 5] = [
	/* SND_SOC_DAPM_HP("Headphone", NULL) */
	snd_soc_dapm_widget { _private: [] },
	/* SND_SOC_DAPM_SPK("Speaker", NULL) */
	snd_soc_dapm_widget { _private: [] },
	/* SND_SOC_DAPM_MIC("Headset Mic", NULL) */
	snd_soc_dapm_widget { _private: [] },
	/* SND_SOC_DAPM_MIC("DMIC Pair1", NULL) */
	snd_soc_dapm_widget { _private: [] },
	/* SND_SOC_DAPM_MIC("DMIC Pair2", NULL) */
	snd_soc_dapm_widget { _private: [] },
];

static bdw_rt5650_map: [snd_soc_dapm_route; 16] = [
	/* Speakers */
	snd_soc_dapm_route { sink: c"Speaker".as_ptr(), control: ptr::null(), source: c"SPOL".as_ptr() },
	snd_soc_dapm_route { sink: c"Speaker".as_ptr(), control: ptr::null(), source: c"SPOR".as_ptr() },

	/* Headset jack connectors */
	snd_soc_dapm_route { sink: c"Headphone".as_ptr(), control: ptr::null(), source: c"HPOL".as_ptr() },
	snd_soc_dapm_route { sink: c"Headphone".as_ptr(), control: ptr::null(), source: c"HPOR".as_ptr() },
	snd_soc_dapm_route { sink: c"IN1P".as_ptr(), control: ptr::null(), source: c"Headset Mic".as_ptr() },
	snd_soc_dapm_route { sink: c"IN1N".as_ptr(), control: ptr::null(), source: c"Headset Mic".as_ptr() },

	/* Digital MICs
	 * DMIC Pair1 are the two DMICs connected on the DMICN1 connector.
	 * DMIC Pair2 are the two DMICs connected on the DMICN2 connector.
	 * Facing the camera, DMIC Pair1 are on the left side, DMIC Pair2
	 * are on the right side.
	 */
	snd_soc_dapm_route { sink: c"DMIC L1".as_ptr(), control: ptr::null(), source: c"DMIC Pair1".as_ptr() },
	snd_soc_dapm_route { sink: c"DMIC R1".as_ptr(), control: ptr::null(), source: c"DMIC Pair1".as_ptr() },
	snd_soc_dapm_route { sink: c"DMIC L2".as_ptr(), control: ptr::null(), source: c"DMIC Pair2".as_ptr() },
	snd_soc_dapm_route { sink: c"DMIC R2".as_ptr(), control: ptr::null(), source: c"DMIC Pair2".as_ptr() },

	/* CODEC BE connections */
	snd_soc_dapm_route { sink: c"SSP0 CODEC IN".as_ptr(), control: ptr::null(), source: c"AIF1 Capture".as_ptr() },
	snd_soc_dapm_route { sink: c"AIF1 Playback".as_ptr(), control: ptr::null(), source: c"SSP0 CODEC OUT".as_ptr() },
];

// Macro-created controls from SOC_DAPM_PIN_SWITCH.
static bdw_rt5650_controls: [snd_kcontrol_new; 5] = [
	/* SOC_DAPM_PIN_SWITCH("Speaker") */
	snd_kcontrol_new { _private: [] },
	/* SOC_DAPM_PIN_SWITCH("Headphone") */
	snd_kcontrol_new { _private: [] },
	/* SOC_DAPM_PIN_SWITCH("Headset Mic") */
	snd_kcontrol_new { _private: [] },
	/* SOC_DAPM_PIN_SWITCH("DMIC Pair1") */
	snd_kcontrol_new { _private: [] },
	/* SOC_DAPM_PIN_SWITCH("DMIC Pair2") */
	snd_kcontrol_new { _private: [] },
];

static mut headphone_jack: snd_soc_jack = snd_soc_jack { _private: [] };
static mut mic_jack: snd_soc_jack = snd_soc_jack { _private: [] };

static mut headphone_jack_pin: snd_soc_jack_pin = snd_soc_jack_pin {
	pin: c"Headphone".as_ptr(),
	mask: SND_JACK_HEADPHONE,
};

static mut mic_jack_pin: snd_soc_jack_pin = snd_soc_jack_pin {
	pin: c"Headset Mic".as_ptr(),
	mask: SND_JACK_MICROPHONE,
};

unsafe extern "C" fn broadwell_ssp0_fixup(
	_rtd: *mut snd_soc_pcm_runtime,
	params: *mut snd_pcm_hw_params,
) -> c_int {
	let rate: *mut snd_interval = hw_param_interval(params, SNDRV_PCM_HW_PARAM_RATE);
	let chan: *mut snd_interval = hw_param_interval(params, SNDRV_PCM_HW_PARAM_CHANNELS);

	/* The ADSP will convert the FE rate to 48k, max 4-channels */
	(*rate).max = 48000;
	(*rate).min = (*rate).max;
	(*chan).min = 2;
	(*chan).max = 4;

	/* set SSP0 to 24 bit */
	snd_mask_set_format(
		hw_param_mask(params, SNDRV_PCM_HW_PARAM_FORMAT),
		SNDRV_PCM_FORMAT_S24_LE,
	);

	0
}

unsafe extern "C" fn bdw_rt5650_hw_params(
	substream: *mut snd_pcm_substream,
	_params: *mut snd_pcm_hw_params,
) -> c_int {
	let rtd: *mut snd_soc_pcm_runtime = snd_soc_substream_to_rtd(substream);
	let codec_dai: *mut snd_soc_dai = snd_soc_rtd_to_codec(rtd, 0);
	let mut ret: c_int;

	/* Workaround: set codec PLL to 19.2MHz that PLL source is
	 * from MCLK(24MHz) to conform 2.4MHz DMIC clock.
	 */
	ret = snd_soc_dai_set_pll(codec_dai, 0, RT5645_PLL1_S_MCLK, 24000000, 19200000);
	if ret < 0 {
		dev_err((*rtd).dev, c"can't set codec pll: %d\n".as_ptr(), ret);
		return ret;
	}

	/* The actual MCLK freq is 24MHz. The codec is told that MCLK is
	 * 24.576MHz to satisfy the requirement of rl6231_get_clk_info.
	 * ASRC is enabled on AD and DA filters to ensure good audio quality.
	 */
	ret = snd_soc_dai_set_sysclk(codec_dai, RT5645_SCLK_S_PLL1, 24576000, SND_SOC_CLOCK_IN);
	if ret < 0 {
		dev_err((*rtd).dev, c"can't set codec sysclk configuration\n".as_ptr());
		return ret;
	}

	ret
}

static bdw_rt5650_ops: snd_soc_ops = snd_soc_ops {
	startup: None,
	hw_params: Some(bdw_rt5650_hw_params),
};

static channels: [c_uint; 2] = [2, 4];

static constraints_channels: snd_pcm_hw_constraint_list = snd_pcm_hw_constraint_list {
	count: channels.len() as c_uint,
	list: channels.as_ptr(),
	mask: 0,
};

unsafe extern "C" fn bdw_rt5650_fe_startup(substream: *mut snd_pcm_substream) -> c_int {
	let runtime: *mut snd_pcm_runtime = (*substream).runtime;

	/* Board supports stereo and quad configurations for capture */
	if (*substream).stream != SNDRV_PCM_STREAM_CAPTURE {
		return 0;
	}

	(*runtime).hw.channels_max = 4;
	snd_pcm_hw_constraint_list(
		runtime,
		0,
		SNDRV_PCM_HW_PARAM_CHANNELS,
		&constraints_channels,
	)
}

static bdw_rt5650_fe_ops: snd_soc_ops = snd_soc_ops {
	startup: Some(bdw_rt5650_fe_startup),
	hw_params: None,
};

unsafe extern "C" fn bdw_rt5650_init(rtd: *mut snd_soc_pcm_runtime) -> c_int {
	let bdw_rt5650: *mut bdw_rt5650_priv =
		snd_soc_card_get_drvdata((*rtd).card) as *mut bdw_rt5650_priv;
	let codec_dai: *mut snd_soc_dai = snd_soc_rtd_to_codec(rtd, 0);
	let component: *mut snd_soc_component = (*codec_dai).component;
	let mut ret: c_int;

	/* Enable codec ASRC function for Stereo DAC/Stereo1 ADC/DMIC/I2S1.
	 * The ASRC clock source is clk_i2s1_asrc.
	 */
	rt5645_sel_asrc_clk_src(
		component,
		RT5645_DA_STEREO_FILTER
			| RT5645_DA_MONO_L_FILTER
			| RT5645_DA_MONO_R_FILTER
			| RT5645_AD_STEREO_FILTER
			| RT5645_AD_MONO_L_FILTER
			| RT5645_AD_MONO_R_FILTER,
		RT5645_CLK_SEL_I2S1_ASRC,
	);

	/* TDM 4 slots 24 bit, set Rx & Tx bitmask to 4 active slots */
	ret = snd_soc_dai_set_tdm_slot(codec_dai, 0xF, 0xF, 4, 24);

	if ret < 0 {
		dev_err((*rtd).dev, c"can't set codec TDM slot %d\n".as_ptr(), ret);
		return ret;
	}

	/* Create and initialize headphone jack */
	if snd_soc_card_jack_new_pins(
		(*rtd).card,
		c"Headphone Jack".as_ptr(),
		SND_JACK_HEADPHONE,
		&raw mut headphone_jack,
		&raw mut headphone_jack_pin,
		1,
	) != 0
	{
		dev_err((*component).dev, c"Can't create headphone jack\n".as_ptr());
	}

	/* Create and initialize mic jack */
	if snd_soc_card_jack_new_pins(
		(*rtd).card,
		c"Mic Jack".as_ptr(),
		SND_JACK_MICROPHONE,
		&raw mut mic_jack,
		&raw mut mic_jack_pin,
		1,
	) != 0
	{
		dev_err((*component).dev, c"Can't create mic jack\n".as_ptr());
	}

	rt5645_set_jack_detect(
		component,
		&raw mut headphone_jack,
		&raw mut mic_jack,
		ptr::null_mut(),
	);

	(*bdw_rt5650).component = component;

	0
}

/* broadwell digital audio interface glue - connects codec <--> CPU */
static mut dummy: [snd_soc_dai_link_component; 1] = [snd_soc_dai_link_component {
	name: ptr::null(),
	dai_name: ptr::null(),
}];

static mut fe: [snd_soc_dai_link_component; 1] = [snd_soc_dai_link_component {
	name: c"System Pin".as_ptr(),
	dai_name: ptr::null(),
}];

static mut platform: [snd_soc_dai_link_component; 1] = [snd_soc_dai_link_component {
	name: c"haswell-pcm-audio".as_ptr(),
	dai_name: ptr::null(),
}];

static mut be: [snd_soc_dai_link_component; 1] = [snd_soc_dai_link_component {
	name: c"i2c-10EC5650:00".as_ptr(),
	dai_name: c"rt5645-aif1".as_ptr(),
}];

static mut ssp0_port: [snd_soc_dai_link_component; 1] = [snd_soc_dai_link_component {
	name: c"ssp0-port".as_ptr(),
	dai_name: ptr::null(),
}];

static mut bdw_rt5650_dais: [snd_soc_dai_link; 2] = [
	/* Front End DAI links */
	snd_soc_dai_link {
		name: c"System PCM".as_ptr(),
		stream_name: c"System Playback".as_ptr(),
		id: 0,
		nonatomic: 1,
		dynamic: 1,
		no_pcm: 0,
		dai_fmt: 0,
		ignore_pmdown_time: 0,
		ops: &bdw_rt5650_fe_ops,
		trigger: [
			SND_SOC_DPCM_TRIGGER_POST,
			SND_SOC_DPCM_TRIGGER_POST,
		],
		be_hw_params_fixup: None,
		init: None,
		cpus: unsafe { fe.as_mut_ptr() },
		num_cpus: 1,
		codecs: unsafe { dummy.as_mut_ptr() },
		num_codecs: 1,
		platforms: unsafe { platform.as_mut_ptr() },
		num_platforms: 1,
	},

	/* Back End DAI links */
	snd_soc_dai_link {
		/* SSP0 - Codec */
		name: c"Codec".as_ptr(),
		stream_name: ptr::null(),
		id: 0,
		nonatomic: 1,
		dynamic: 0,
		no_pcm: 1,
		dai_fmt: SND_SOC_DAIFMT_DSP_B | SND_SOC_DAIFMT_NB_NF | SND_SOC_DAIFMT_CBC_CFC,
		ignore_pmdown_time: 1,
		be_hw_params_fixup: Some(broadwell_ssp0_fixup),
		ops: &bdw_rt5650_ops,
		init: Some(bdw_rt5650_init),
		cpus: unsafe { ssp0_port.as_mut_ptr() },
		num_cpus: 1,
		codecs: unsafe { be.as_mut_ptr() },
		num_codecs: 1,
		platforms: unsafe { platform.as_mut_ptr() },
		num_platforms: 1,
	},
];

/* use space before codec name to simplify card ID, and simplify driver name */
const SOF_CARD_NAME: *const c_char = c"bdw rt5650".as_ptr(); /* card name will be 'sof-bdw rt5650' */
const SOF_DRIVER_NAME: *const c_char = c"SOF".as_ptr();

const CARD_NAME: *const c_char = c"bdw-rt5650".as_ptr();
const DRIVER_NAME: *const c_char = ptr::null(); /* card name will be used for driver name */

/* ASoC machine driver for Broadwell DSP + RT5650 */
static mut bdw_rt5650_card: snd_soc_card = snd_soc_card {
	name: CARD_NAME,
	driver_name: DRIVER_NAME,
	owner: unsafe { THIS_MODULE },
	dai_link: unsafe { bdw_rt5650_dais.as_mut_ptr() },
	num_links: 2,
	dapm_widgets: bdw_rt5650_widgets.as_ptr(),
	num_dapm_widgets: 5,
	dapm_routes: bdw_rt5650_map.as_ptr(),
	num_dapm_routes: 16,
	controls: bdw_rt5650_controls.as_ptr(),
	num_controls: 5,
	fully_routed: true,
	dev: ptr::null_mut(),
};

unsafe extern "C" fn bdw_rt5650_probe(pdev: *mut platform_device) -> c_int {
	let bdw_rt5650: *mut bdw_rt5650_priv;
	let mach: *mut snd_soc_acpi_mach;
	let mut ret: c_int;

	bdw_rt5650_card.dev = &raw mut (*pdev).dev;

	/* Allocate driver private struct */
	bdw_rt5650 = devm_kzalloc(
		&raw mut (*pdev).dev,
		core::mem::size_of::<bdw_rt5650_priv>(),
		GFP_KERNEL,
	) as *mut bdw_rt5650_priv;
	if bdw_rt5650.is_null() {
		return -ENOMEM;
	}

	/* override platform name, if required */
	mach = (*pdev).dev.platform_data as *mut snd_soc_acpi_mach;
	ret = snd_soc_fixup_dai_links_platform_name(
		&raw mut bdw_rt5650_card,
		(*mach).mach_params.platform,
	);

	if ret != 0 {
		return ret;
	}

	/* set card and driver name */
	if snd_soc_acpi_sof_parent(&raw mut (*pdev).dev) {
		bdw_rt5650_card.name = SOF_CARD_NAME;
		bdw_rt5650_card.driver_name = SOF_DRIVER_NAME;
	} else {
		bdw_rt5650_card.name = CARD_NAME;
		bdw_rt5650_card.driver_name = DRIVER_NAME;
	}

	snd_soc_card_set_drvdata(&raw mut bdw_rt5650_card, bdw_rt5650 as *mut c_void);

	devm_snd_soc_register_card(&raw mut (*pdev).dev, &raw mut bdw_rt5650_card)
}

static mut bdw_rt5650_audio: platform_driver = platform_driver {
	probe: Some(bdw_rt5650_probe),
	driver: device_driver {
		name: c"bdw-rt5650".as_ptr(),
		pm: unsafe { &snd_soc_pm_ops },
	},
};

// module_platform_driver(bdw_rt5650_audio)

/* Module information */
// MODULE_AUTHOR("Ben Zhang <benzh@chromium.org>");
// MODULE_DESCRIPTION("Intel Broadwell RT5650 machine driver");
// MODULE_LICENSE("GPL v2");
// MODULE_ALIAS("platform:bdw-rt5650");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
