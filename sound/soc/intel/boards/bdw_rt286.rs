// SPDX-License-Identifier: GPL-2.0-only
/*
 * Sound card driver for Intel Broadwell Wildcat Point with Realtek 286
 *
 * Copyright (C) 2013, Intel Corporation
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::ptr;

#[repr(C)]
pub struct module {
	_private: [u8; 0],
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
pub struct snd_soc_component {
	_private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_jack {
	_private: [u8; 0],
}

#[repr(C)]
pub struct snd_kcontrol_new {
	_private: [u8; 0],
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
pub struct snd_soc_jack_pin {
	pub pin: *const c_char,
	pub mask: c_int,
}

#[repr(C)]
pub struct snd_soc_pcm_runtime {
	pub card: *mut snd_soc_card,
	pub dev: *mut device,
}

#[repr(C)]
pub struct snd_soc_dai {
	pub component: *mut snd_soc_component,
}

#[repr(C)]
pub struct snd_pcm_hw_params {
	_private: [u8; 0],
}

#[repr(C)]
pub struct snd_pcm_substream {
	_private: [u8; 0],
}

#[repr(C)]
pub struct snd_interval {
	pub min: c_uint,
	pub max: c_uint,
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
pub struct snd_soc_acpi_mach_params {
	pub platform: *const c_char,
}

#[repr(C)]
pub struct snd_soc_acpi_mach {
	pub mach_params: snd_soc_acpi_mach_params,
}

#[repr(C)]
pub struct snd_soc_dai_link {
	pub name: *const c_char,
	pub stream_name: *const c_char,
	pub id: c_int,
	pub nonatomic: c_uint,
	pub dynamic: c_uint,
	pub trigger: [c_int; 2],
	pub playback_only: c_uint,
	pub capture_only: c_uint,
	pub no_pcm: c_uint,
	pub init: Option<unsafe extern "C" fn(rtd: *mut snd_soc_pcm_runtime) -> c_int>,
	pub exit: Option<unsafe extern "C" fn(rtd: *mut snd_soc_pcm_runtime)>,
	pub dai_fmt: c_uint,
	pub ignore_pmdown_time: c_uint,
	pub be_hw_params_fixup: Option<
		unsafe extern "C" fn(
			rtd: *mut snd_soc_pcm_runtime,
			params: *mut snd_pcm_hw_params,
		) -> c_int,
	>,
	pub ops: *const snd_soc_ops,
	/* SND_SOC_DAILINK_REG(...) supplies CPU, codec, and platform component arrays. */
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
	pub probe: Option<unsafe extern "C" fn(pdev: *mut platform_device) -> c_int>,
	pub driver: device_driver,
}

#[repr(C)]
pub struct snd_soc_card {
	pub owner: *mut module,
	pub suspend_pre: Option<unsafe extern "C" fn(card: *mut snd_soc_card) -> c_int>,
	pub resume_post: Option<unsafe extern "C" fn(card: *mut snd_soc_card) -> c_int>,
	pub dai_link: *mut snd_soc_dai_link,
	pub num_links: c_int,
	pub controls: *const snd_kcontrol_new,
	pub num_controls: c_int,
	pub dapm_widgets: *const snd_soc_dapm_widget,
	pub num_dapm_widgets: c_int,
	pub dapm_routes: *const snd_soc_dapm_route,
	pub num_dapm_routes: c_int,
	pub fully_routed: bool,
	pub dev: *mut device,
	pub name: *const c_char,
	pub driver_name: *const c_char,
}

unsafe extern "C" {
	static mut THIS_MODULE: *mut module;
	static snd_soc_pm_ops: dev_pm_ops;

	static SND_JACK_MICROPHONE: c_int;
	static SND_JACK_HEADPHONE: c_int;
	static SND_JACK_HEADSET: c_int;
	static SND_JACK_BTN_0: c_int;
	static SNDRV_PCM_HW_PARAM_CHANNELS: c_int;
	static SNDRV_PCM_HW_PARAM_RATE: c_int;
	static SNDRV_PCM_FORMAT_S16_LE: c_int;
	static RT286_SCLK_S_PLL: c_int;
	static SND_SOC_CLOCK_IN: c_int;
	static SND_SOC_DPCM_TRIGGER_POST: c_int;
	static SND_SOC_DAIFMT_I2S: c_uint;
	static SND_SOC_DAIFMT_NB_NF: c_uint;
	static SND_SOC_DAIFMT_CBC_CFC: c_uint;

	fn snd_soc_rtd_to_codec(rtd: *mut snd_soc_pcm_runtime, num: c_int) -> *mut snd_soc_dai;
	fn snd_soc_card_jack_new_pins(
		card: *mut snd_soc_card,
		id: *const c_char,
		r#type: c_int,
		jack: *mut snd_soc_jack,
		pins: *mut snd_soc_jack_pin,
		num_pins: c_uint,
	) -> c_int;
	fn snd_soc_component_set_jack(
		component: *mut snd_soc_component,
		jack: *mut snd_soc_jack,
		data: *mut c_void,
	) -> c_int;
	fn hw_param_interval(
		params: *mut snd_pcm_hw_params,
		var: c_int,
	) -> *mut snd_interval;
	fn params_set_format(params: *mut snd_pcm_hw_params, format: c_int);
	fn snd_soc_substream_to_rtd(substream: *mut snd_pcm_substream) -> *mut snd_soc_pcm_runtime;
	fn snd_soc_dai_set_sysclk(
		dai: *mut snd_soc_dai,
		clk_id: c_int,
		freq: c_uint,
		dir: c_int,
	) -> c_int;
	fn dev_err(dev: *mut device, fmt: *const c_char, ...) -> c_int;
	fn snd_soc_card_get_codec_dai(
		card: *mut snd_soc_card,
		dai_name: *const c_char,
	) -> *mut snd_soc_dai;
	fn dev_get_platdata(dev: *mut device) -> *mut c_void;
	fn snd_soc_fixup_dai_links_platform_name(
		card: *mut snd_soc_card,
		platform_name: *const c_char,
	) -> c_int;
	fn snd_soc_acpi_sof_parent(dev: *mut device) -> bool;
	fn devm_snd_soc_register_card(dev: *mut device, card: *mut snd_soc_card) -> c_int;
}

static mut card_headset: snd_soc_jack = snd_soc_jack { _private: [] };

static mut card_headset_pins: [snd_soc_jack_pin; 2] = [
	snd_soc_jack_pin {
		pin: c"Mic Jack".as_ptr(),
		mask: unsafe { SND_JACK_MICROPHONE },
	},
	snd_soc_jack_pin {
		pin: c"Headphone Jack".as_ptr(),
		mask: unsafe { SND_JACK_HEADPHONE },
	},
];

/* SOC_DAPM_PIN_SWITCH("Speaker"), SOC_DAPM_PIN_SWITCH("Headphone Jack") */
static card_controls: [snd_kcontrol_new; 2] = [
	snd_kcontrol_new { _private: [] },
	snd_kcontrol_new { _private: [] },
];

/*
 * SND_SOC_DAPM_HP("Headphone Jack", NULL), SND_SOC_DAPM_SPK("Speaker", NULL),
 * SND_SOC_DAPM_MIC("Mic Jack", NULL), SND_SOC_DAPM_MIC("DMIC1", NULL),
 * SND_SOC_DAPM_MIC("DMIC2", NULL), SND_SOC_DAPM_LINE("Line Jack", NULL)
 */
static card_widgets: [snd_soc_dapm_widget; 6] = [
	snd_soc_dapm_widget { _private: [] },
	snd_soc_dapm_widget { _private: [] },
	snd_soc_dapm_widget { _private: [] },
	snd_soc_dapm_widget { _private: [] },
	snd_soc_dapm_widget { _private: [] },
	snd_soc_dapm_widget { _private: [] },
];

static card_routes: [snd_soc_dapm_route; 10] = [
	snd_soc_dapm_route { sink: c"Speaker".as_ptr(), control: ptr::null(), source: c"SPOR".as_ptr() },
	snd_soc_dapm_route { sink: c"Speaker".as_ptr(), control: ptr::null(), source: c"SPOL".as_ptr() },
	snd_soc_dapm_route { sink: c"Headphone Jack".as_ptr(), control: ptr::null(), source: c"HPO Pin".as_ptr() },
	snd_soc_dapm_route { sink: c"MIC1".as_ptr(), control: ptr::null(), source: c"Mic Jack".as_ptr() },
	snd_soc_dapm_route { sink: c"LINE1".as_ptr(), control: ptr::null(), source: c"Line Jack".as_ptr() },
	snd_soc_dapm_route { sink: c"DMIC1 Pin".as_ptr(), control: ptr::null(), source: c"DMIC1".as_ptr() },
	snd_soc_dapm_route { sink: c"DMIC2 Pin".as_ptr(), control: ptr::null(), source: c"DMIC2".as_ptr() },
	/* CODEC BE connections */
	snd_soc_dapm_route { sink: c"SSP0 CODEC IN".as_ptr(), control: ptr::null(), source: c"AIF1 Capture".as_ptr() },
	snd_soc_dapm_route { sink: c"AIF1 Playback".as_ptr(), control: ptr::null(), source: c"SSP0 CODEC OUT".as_ptr() },
];

unsafe extern "C" fn codec_link_init(rtd: *mut snd_soc_pcm_runtime) -> c_int {
	let codec: *mut snd_soc_component = (*snd_soc_rtd_to_codec(rtd, 0)).component;
	let ret: c_int;

	ret = snd_soc_card_jack_new_pins(
		(*rtd).card,
		c"Headset".as_ptr(),
		SND_JACK_HEADSET | SND_JACK_BTN_0,
		&raw mut card_headset,
		card_headset_pins.as_mut_ptr(),
		card_headset_pins.len() as c_uint,
	);
	if ret != 0 {
		return ret;
	}

	snd_soc_component_set_jack(codec, &raw mut card_headset, ptr::null_mut())
}

unsafe extern "C" fn codec_link_exit(rtd: *mut snd_soc_pcm_runtime) {
	let codec: *mut snd_soc_component = (*snd_soc_rtd_to_codec(rtd, 0)).component;

	snd_soc_component_set_jack(codec, ptr::null_mut(), ptr::null_mut());
}

unsafe extern "C" fn codec_link_hw_params_fixup(
	_rtd: *mut snd_soc_pcm_runtime,
	params: *mut snd_pcm_hw_params,
) -> c_int {
	let channels: *mut snd_interval = hw_param_interval(params, SNDRV_PCM_HW_PARAM_CHANNELS);
	let rate: *mut snd_interval = hw_param_interval(params, SNDRV_PCM_HW_PARAM_RATE);

	/* The ADSP will convert the FE rate to 48kHz, stereo. */
	(*rate).max = 48000;
	(*rate).min = (*rate).max;
	(*channels).max = 2;
	(*channels).min = (*channels).max;
	/* Set SSP0 to 16 bit. */
	params_set_format(params, SNDRV_PCM_FORMAT_S16_LE);

	0
}

unsafe extern "C" fn codec_link_hw_params(
	substream: *mut snd_pcm_substream,
	_params: *mut snd_pcm_hw_params,
) -> c_int {
	let rtd: *mut snd_soc_pcm_runtime = snd_soc_substream_to_rtd(substream);
	let codec_dai: *mut snd_soc_dai = snd_soc_rtd_to_codec(rtd, 0);
	let ret: c_int;

	ret = snd_soc_dai_set_sysclk(codec_dai, RT286_SCLK_S_PLL, 24000000, SND_SOC_CLOCK_IN);
	if ret < 0 {
		dev_err((*rtd).dev, c"set codec sysclk failed: %d\n".as_ptr(), ret);
		return ret;
	}

	ret
}

static codec_link_ops: snd_soc_ops = snd_soc_ops {
	hw_params: Some(codec_link_hw_params),
};

/*
 * SND_SOC_DAILINK_DEF(system, DAILINK_COMP_ARRAY(COMP_CPU("System Pin")));
 * SND_SOC_DAILINK_DEF(offload0, DAILINK_COMP_ARRAY(COMP_CPU("Offload0 Pin")));
 * SND_SOC_DAILINK_DEF(offload1, DAILINK_COMP_ARRAY(COMP_CPU("Offload1 Pin")));
 * SND_SOC_DAILINK_DEF(loopback, DAILINK_COMP_ARRAY(COMP_CPU("Loopback Pin")));
 *
 * SND_SOC_DAILINK_DEF(dummy, DAILINK_COMP_ARRAY(COMP_DUMMY()));
 * SND_SOC_DAILINK_DEF(platform, DAILINK_COMP_ARRAY(COMP_PLATFORM("haswell-pcm-audio")));
 * SND_SOC_DAILINK_DEF(codec, DAILINK_COMP_ARRAY(COMP_CODEC("i2c-INT343A:00", "rt286-aif1")));
 * SND_SOC_DAILINK_DEF(ssp0_port, DAILINK_COMP_ARRAY(COMP_CPU("ssp0-port")));
 */

static mut card_dai_links: [snd_soc_dai_link; 5] = [
	/* Front End DAI links */
	snd_soc_dai_link {
		name: c"System PCM".as_ptr(),
		stream_name: c"System Playback/Capture".as_ptr(),
		id: 0,
		nonatomic: 1,
		dynamic: 1,
		trigger: unsafe { [SND_SOC_DPCM_TRIGGER_POST, SND_SOC_DPCM_TRIGGER_POST] },
		playback_only: 0,
		capture_only: 0,
		no_pcm: 0,
		init: None,
		exit: None,
		dai_fmt: 0,
		ignore_pmdown_time: 0,
		be_hw_params_fixup: None,
		ops: ptr::null(),
		/* SND_SOC_DAILINK_REG(system, dummy, platform) */
	},
	snd_soc_dai_link {
		name: c"Offload0".as_ptr(),
		stream_name: c"Offload0 Playback".as_ptr(),
		id: 0,
		nonatomic: 1,
		dynamic: 1,
		trigger: unsafe { [SND_SOC_DPCM_TRIGGER_POST, SND_SOC_DPCM_TRIGGER_POST] },
		playback_only: 1,
		capture_only: 0,
		no_pcm: 0,
		init: None,
		exit: None,
		dai_fmt: 0,
		ignore_pmdown_time: 0,
		be_hw_params_fixup: None,
		ops: ptr::null(),
		/* SND_SOC_DAILINK_REG(offload0, dummy, platform) */
	},
	snd_soc_dai_link {
		name: c"Offload1".as_ptr(),
		stream_name: c"Offload1 Playback".as_ptr(),
		id: 0,
		nonatomic: 1,
		dynamic: 1,
		trigger: unsafe { [SND_SOC_DPCM_TRIGGER_POST, SND_SOC_DPCM_TRIGGER_POST] },
		playback_only: 1,
		capture_only: 0,
		no_pcm: 0,
		init: None,
		exit: None,
		dai_fmt: 0,
		ignore_pmdown_time: 0,
		be_hw_params_fixup: None,
		ops: ptr::null(),
		/* SND_SOC_DAILINK_REG(offload1, dummy, platform) */
	},
	snd_soc_dai_link {
		name: c"Loopback PCM".as_ptr(),
		stream_name: c"Loopback".as_ptr(),
		id: 0,
		nonatomic: 1,
		dynamic: 1,
		trigger: unsafe { [SND_SOC_DPCM_TRIGGER_POST, SND_SOC_DPCM_TRIGGER_POST] },
		playback_only: 0,
		capture_only: 1,
		no_pcm: 0,
		init: None,
		exit: None,
		dai_fmt: 0,
		ignore_pmdown_time: 0,
		be_hw_params_fixup: None,
		ops: ptr::null(),
		/* SND_SOC_DAILINK_REG(loopback, dummy, platform) */
	},
	/* Back End DAI links */
	snd_soc_dai_link {
		/* SSP0 - Codec */
		name: c"Codec".as_ptr(),
		stream_name: ptr::null(),
		id: 0,
		nonatomic: 1,
		dynamic: 0,
		trigger: [0, 0],
		playback_only: 0,
		capture_only: 0,
		no_pcm: 1,
		init: Some(codec_link_init),
		exit: Some(codec_link_exit),
		dai_fmt: unsafe { SND_SOC_DAIFMT_I2S | SND_SOC_DAIFMT_NB_NF | SND_SOC_DAIFMT_CBC_CFC },
		ignore_pmdown_time: 1,
		be_hw_params_fixup: Some(codec_link_hw_params_fixup),
		ops: &codec_link_ops,
		/* SND_SOC_DAILINK_REG(ssp0_port, codec, platform) */
	},
];

unsafe extern "C" fn card_suspend_pre(card: *mut snd_soc_card) -> c_int {
	let codec_dai: *mut snd_soc_dai = snd_soc_card_get_codec_dai(card, c"rt286-aif1".as_ptr());

	if codec_dai.is_null() {
		return 0;
	}

	snd_soc_component_set_jack((*codec_dai).component, ptr::null_mut(), ptr::null_mut())
}

unsafe extern "C" fn card_resume_post(card: *mut snd_soc_card) -> c_int {
	let codec_dai: *mut snd_soc_dai = snd_soc_card_get_codec_dai(card, c"rt286-aif1".as_ptr());

	if codec_dai.is_null() {
		return 0;
	}

	snd_soc_component_set_jack((*codec_dai).component, &raw mut card_headset, ptr::null_mut())
}

static mut bdw_rt286_card: snd_soc_card = snd_soc_card {
	owner: unsafe { THIS_MODULE },
	suspend_pre: Some(card_suspend_pre),
	resume_post: Some(card_resume_post),
	dai_link: unsafe { card_dai_links.as_mut_ptr() },
	num_links: 5,
	controls: card_controls.as_ptr(),
	num_controls: 2,
	dapm_widgets: card_widgets.as_ptr(),
	num_dapm_widgets: 6,
	dapm_routes: card_routes.as_ptr(),
	num_dapm_routes: 10,
	fully_routed: true,
	dev: ptr::null_mut(),
	name: ptr::null(),
	driver_name: ptr::null(),
};

/* Use space before codec name to simplify card ID, and simplify driver name. */
const SOF_CARD_NAME: *const c_char = c"bdw rt286".as_ptr(); /* card name will be 'sof-bdw rt286' */
const SOF_DRIVER_NAME: *const c_char = c"SOF".as_ptr();

const CARD_NAME: *const c_char = c"broadwell-rt286".as_ptr();

unsafe extern "C" fn bdw_rt286_probe(pdev: *mut platform_device) -> c_int {
	let mach: *mut snd_soc_acpi_mach;
	let dev: *mut device = &raw mut (*pdev).dev;
	let ret: c_int;

	bdw_rt286_card.dev = dev;
	mach = dev_get_platdata(dev) as *mut snd_soc_acpi_mach;

	ret = snd_soc_fixup_dai_links_platform_name(&raw mut bdw_rt286_card, (*mach).mach_params.platform);
	if ret != 0 {
		return ret;
	}

	if snd_soc_acpi_sof_parent(dev) {
		bdw_rt286_card.name = SOF_CARD_NAME;
		bdw_rt286_card.driver_name = SOF_DRIVER_NAME;
	} else {
		bdw_rt286_card.name = CARD_NAME;
	}

	devm_snd_soc_register_card(dev, &raw mut bdw_rt286_card)
}

static mut bdw_rt286_driver: platform_driver = platform_driver {
	probe: Some(bdw_rt286_probe),
	driver: device_driver {
		name: c"bdw_rt286".as_ptr(),
		pm: unsafe { &snd_soc_pm_ops },
	},
};

/* module_platform_driver(bdw_rt286_driver) */

/* MODULE_AUTHOR("Liam Girdwood, Xingchao Wang"); */
/* MODULE_DESCRIPTION("Sound card driver for Intel Broadwell Wildcat Point with Realtek 286"); */
/* MODULE_LICENSE("GPL"); */
/* MODULE_ALIAS("platform:bdw_rt286"); */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
