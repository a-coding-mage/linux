// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * ALSA Soc PCM3008 codec support
 *
 * Author:	Hugo Villeneuve
 * Copyright (C) 2008 Lyrtech inc
 *
 * Based on AC97 Soc codec, original copyright follow:
 * Copyright 2005 Wolfson Microelectronics PLC.
 *
 * Generic PCM3008 support.
 */

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::mem::size_of;
use core::ptr;

#[repr(C)]
pub struct gpio_desc {
	_private: [u8; 0],
}

#[repr(C)]
pub struct snd_kcontrol {
	_private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_dapm_context {
	_private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_dapm_widget {
	pub dapm: *mut snd_soc_dapm_context,
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
pub struct snd_soc_pcm_stream {
	pub stream_name: *const c_char,
	pub channels_min: c_uint,
	pub channels_max: c_uint,
	pub rates: c_uint,
	pub formats: u64,
}

#[repr(C)]
pub struct snd_soc_dai_driver {
	pub name: *const c_char,
	pub playback: snd_soc_pcm_stream,
	pub capture: snd_soc_pcm_stream,
}

#[repr(C)]
pub struct snd_soc_dapm_widget_def {
	pub _private: [usize; 8],
}

#[repr(C)]
pub struct snd_soc_dapm_route {
	pub sink: *const c_char,
	pub control: *const c_char,
	pub source: *const c_char,
}

#[repr(C)]
pub struct snd_soc_component_driver {
	pub dapm_widgets: *const snd_soc_dapm_widget_def,
	pub num_dapm_widgets: c_uint,
	pub dapm_routes: *const snd_soc_dapm_route,
	pub num_dapm_routes: c_uint,
	pub idle_bias_on: c_uint,
	pub use_pmdown_time: c_uint,
	pub endianness: c_uint,
}

#[repr(C)]
pub struct platform_driver_inner {
	pub name: *const c_char,
}

#[repr(C)]
pub struct platform_driver {
	pub probe: Option<unsafe extern "C" fn(*mut platform_device) -> c_int>,
	pub driver: platform_driver_inner,
}

#[repr(C)]
pub struct pcm3008 {
	pub dem0_pin: *mut gpio_desc,
	pub dem1_pin: *mut gpio_desc,
	pub pdad_pin: *mut gpio_desc,
	pub pdda_pin: *mut gpio_desc,
}

unsafe extern "C" {
	fn snd_soc_dapm_to_component(dapm: *mut snd_soc_dapm_context) -> *mut snd_soc_component;
	fn gpiod_set_value_cansleep(desc: *mut gpio_desc, value: c_int);
	fn devm_kzalloc(dev: *mut device, size: usize, flags: c_uint) -> *mut c_void;
	fn platform_set_drvdata(pdev: *mut platform_device, data: *mut c_void);
	fn devm_gpiod_get(dev: *mut device, con_id: *const c_char, flags: c_uint) -> *mut gpio_desc;
	fn devm_snd_soc_register_component(
		dev: *mut device,
		component_driver: *const snd_soc_component_driver,
		dai_drv: *mut snd_soc_dai_driver,
		num_dai: c_int,
	) -> c_int;
}

const ENOMEM: c_int = 12;
const GFP_KERNEL: c_uint = 0;
const GPIOD_OUT_LOW: c_uint = 0;
const GPIOD_OUT_HIGH: c_uint = 1;
const SND_SOC_NOPM: c_int = -1;
const SND_SOC_DAPM_PRE_PMU: c_int = 0x1;
const SND_SOC_DAPM_POST_PMD: c_int = 0x2;
const SNDRV_PCM_RATE_32000: c_uint = 0x0000_0400;
const SNDRV_PCM_RATE_44100: c_uint = 0x0000_4000;
const SNDRV_PCM_RATE_48000: c_uint = 0x0000_8000;
const SNDRV_PCM_FMTBIT_S16_LE: u64 = 1 << 2;

const PCM3008_RATES: c_uint =
	SNDRV_PCM_RATE_32000 | SNDRV_PCM_RATE_44100 | SNDRV_PCM_RATE_48000;

#[inline]
unsafe fn SND_SOC_DAPM_EVENT_ON(event: c_int) -> c_int {
	if event == SND_SOC_DAPM_PRE_PMU {
		1
	} else {
		0
	}
}

#[inline]
unsafe fn IS_ERR<T>(ptr: *mut T) -> bool {
	(ptr as isize) < 0 && (ptr as isize) >= -4095
}

#[inline]
unsafe fn PTR_ERR<T>(ptr: *mut T) -> c_int {
	ptr as isize as c_int
}

unsafe extern "C" fn pcm3008_dac_ev(
	w: *mut snd_soc_dapm_widget,
	_kcontrol: *mut snd_kcontrol,
	event: c_int,
) -> c_int {
	let component: *mut snd_soc_component = snd_soc_dapm_to_component((*w).dapm);
	let pcm: *mut pcm3008 = (*(*component).dev).platform_data as *mut pcm3008;

	gpiod_set_value_cansleep((*pcm).pdda_pin, SND_SOC_DAPM_EVENT_ON(event));

	0
}

unsafe extern "C" fn pcm3008_adc_ev(
	w: *mut snd_soc_dapm_widget,
	_kcontrol: *mut snd_kcontrol,
	event: c_int,
) -> c_int {
	let component: *mut snd_soc_component = snd_soc_dapm_to_component((*w).dapm);
	let pcm: *mut pcm3008 = (*(*component).dev).platform_data as *mut pcm3008;

	gpiod_set_value_cansleep((*pcm).pdad_pin, SND_SOC_DAPM_EVENT_ON(event));

	0
}

/* SND_SOC_DAPM_* widget macro initializers from the C source are dependency-provided. */
static pcm3008_dapm_widgets: [snd_soc_dapm_widget_def; 6] = [
	snd_soc_dapm_widget_def { _private: [0; 8] }, /* SND_SOC_DAPM_INPUT("VINL") */
	snd_soc_dapm_widget_def { _private: [0; 8] }, /* SND_SOC_DAPM_INPUT("VINR") */
	snd_soc_dapm_widget_def { _private: [0; 8] }, /* SND_SOC_DAPM_DAC_E("DAC", NULL, SND_SOC_NOPM, 0, 0, pcm3008_dac_ev, SND_SOC_DAPM_PRE_PMU | SND_SOC_DAPM_POST_PMD) */
	snd_soc_dapm_widget_def { _private: [0; 8] }, /* SND_SOC_DAPM_ADC_E("ADC", NULL, SND_SOC_NOPM, 0, 0, pcm3008_adc_ev, SND_SOC_DAPM_PRE_PMU | SND_SOC_DAPM_POST_PMD) */
	snd_soc_dapm_widget_def { _private: [0; 8] }, /* SND_SOC_DAPM_OUTPUT("VOUTL") */
	snd_soc_dapm_widget_def { _private: [0; 8] }, /* SND_SOC_DAPM_OUTPUT("VOUTR") */
];

static pcm3008_dapm_routes: [snd_soc_dapm_route; 6] = [
	snd_soc_dapm_route {
		sink: b"PCM3008 Capture\0".as_ptr() as *const c_char,
		control: ptr::null(),
		source: b"ADC\0".as_ptr() as *const c_char,
	},
	snd_soc_dapm_route {
		sink: b"ADC\0".as_ptr() as *const c_char,
		control: ptr::null(),
		source: b"VINL\0".as_ptr() as *const c_char,
	},
	snd_soc_dapm_route {
		sink: b"ADC\0".as_ptr() as *const c_char,
		control: ptr::null(),
		source: b"VINR\0".as_ptr() as *const c_char,
	},
	snd_soc_dapm_route {
		sink: b"DAC\0".as_ptr() as *const c_char,
		control: ptr::null(),
		source: b"PCM3008 Playback\0".as_ptr() as *const c_char,
	},
	snd_soc_dapm_route {
		sink: b"VOUTL\0".as_ptr() as *const c_char,
		control: ptr::null(),
		source: b"DAC\0".as_ptr() as *const c_char,
	},
	snd_soc_dapm_route {
		sink: b"VOUTR\0".as_ptr() as *const c_char,
		control: ptr::null(),
		source: b"DAC\0".as_ptr() as *const c_char,
	},
];

static mut pcm3008_dai: snd_soc_dai_driver = snd_soc_dai_driver {
	name: b"pcm3008-hifi\0".as_ptr() as *const c_char,
	playback: snd_soc_pcm_stream {
		stream_name: b"PCM3008 Playback\0".as_ptr() as *const c_char,
		channels_min: 1,
		channels_max: 2,
		rates: PCM3008_RATES,
		formats: SNDRV_PCM_FMTBIT_S16_LE,
	},
	capture: snd_soc_pcm_stream {
		stream_name: b"PCM3008 Capture\0".as_ptr() as *const c_char,
		channels_min: 1,
		channels_max: 2,
		rates: PCM3008_RATES,
		formats: SNDRV_PCM_FMTBIT_S16_LE,
	},
};

static soc_component_dev_pcm3008: snd_soc_component_driver = snd_soc_component_driver {
	dapm_widgets: pcm3008_dapm_widgets.as_ptr(),
	num_dapm_widgets: pcm3008_dapm_widgets.len() as c_uint,
	dapm_routes: pcm3008_dapm_routes.as_ptr(),
	num_dapm_routes: pcm3008_dapm_routes.len() as c_uint,
	idle_bias_on: 1,
	use_pmdown_time: 1,
	endianness: 1,
};

unsafe extern "C" fn pcm3008_codec_probe(pdev: *mut platform_device) -> c_int {
	let dev: *mut device = &mut (*pdev).dev;
	let pcm: *mut pcm3008;

	pcm = devm_kzalloc(dev, size_of::<pcm3008>(), GFP_KERNEL) as *mut pcm3008;
	if pcm.is_null() {
		return -ENOMEM;
	}
	platform_set_drvdata(pdev, pcm as *mut c_void);

	/* DEM1  DEM0  DE-EMPHASIS_MODE
	 * Low   Low   De-emphasis 44.1 kHz ON
	 * Low   High  De-emphasis OFF
	 * High  Low   De-emphasis 48 kHz ON
	 * High  High  De-emphasis 32 kHz ON
	 */

	/* Configure DEM0 GPIO (turning OFF DAC De-emphasis). */
	(*pcm).dem0_pin = devm_gpiod_get(dev, b"dem0\0".as_ptr() as *const c_char, GPIOD_OUT_HIGH);
	if IS_ERR((*pcm).dem0_pin) {
		return PTR_ERR((*pcm).dem0_pin);
	}

	/* Configure DEM1 GPIO (turning OFF DAC De-emphasis). */
	(*pcm).dem1_pin = devm_gpiod_get(dev, b"dem1\0".as_ptr() as *const c_char, GPIOD_OUT_LOW);
	if IS_ERR((*pcm).dem1_pin) {
		return PTR_ERR((*pcm).dem1_pin);
	}

	/* Configure PDAD GPIO. */
	(*pcm).pdad_pin = devm_gpiod_get(dev, b"pdad\0".as_ptr() as *const c_char, GPIOD_OUT_LOW);
	if IS_ERR((*pcm).pdad_pin) {
		return PTR_ERR((*pcm).pdad_pin);
	}

	/* Configure PDDA GPIO. */
	(*pcm).pdda_pin = devm_gpiod_get(dev, b"pdda\0".as_ptr() as *const c_char, GPIOD_OUT_LOW);
	if IS_ERR((*pcm).pdda_pin) {
		return PTR_ERR((*pcm).pdda_pin);
	}

	devm_snd_soc_register_component(dev, &soc_component_dev_pcm3008, &raw mut pcm3008_dai, 1)
}

/* MODULE_ALIAS("platform:pcm3008-codec"); */

static mut pcm3008_codec_driver: platform_driver = platform_driver {
	probe: Some(pcm3008_codec_probe),
	driver: platform_driver_inner {
		name: b"pcm3008-codec\0".as_ptr() as *const c_char,
	},
};

/* module_platform_driver(pcm3008_codec_driver); */

/* MODULE_DESCRIPTION("Soc PCM3008 driver"); */
/* MODULE_AUTHOR("Hugo Villeneuve"); */
/* MODULE_LICENSE("GPL"); */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
