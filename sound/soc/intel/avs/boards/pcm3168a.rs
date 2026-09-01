// SPDX-License-Identifier: GPL-2.0-only
//
// Copyright(c) 2024-2025 Intel Corporation
//
// Author: Cezary Rojewski <cezary.rojewski@intel.com>
//

// C dependencies translated as external Rust dependencies:
// linux/module.h, linux/platform_device.h, sound/core.h, sound/pcm.h,
// sound/pcm_params.h, sound/soc.h, sound/soc-acpi.h, ../utils.h

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::mem::size_of;
use core::ptr::{copy_nonoverlapping, null};

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
pub struct snd_soc_pcm_runtime {
	_private: [u8; 0],
}

#[repr(C)]
pub struct snd_pcm_hw_params {
	_private: [u8; 0],
}

#[repr(C)]
pub struct snd_mask {
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
pub struct snd_soc_acpi_mach {
	pub pdata: *mut avs_mach_pdata,
}

#[repr(C)]
pub struct avs_mach_pdata {
	pub obsolete_card_names: bool,
}

#[repr(C)]
pub struct snd_soc_dai_link_component {
	pub name: *const c_char,
}

#[repr(C)]
pub struct snd_soc_dai_link {
	pub name: *const c_char,
	pub cpus: *mut snd_soc_dai_link_component,
	pub num_cpus: c_uint,
	pub codecs: *mut snd_soc_dai_link_component,
	pub num_codecs: c_uint,
	pub platforms: *mut snd_soc_dai_link_component,
	pub num_platforms: c_uint,
	pub dai_fmt: c_uint,
	pub be_hw_params_fixup:
		Option<unsafe extern "C" fn(*mut snd_soc_pcm_runtime, *mut snd_pcm_hw_params) -> c_int>,
	pub nonatomic: c_uint,
	pub no_pcm: c_uint,
}

#[repr(C)]
pub struct snd_soc_card {
	pub name: *const c_char,
	pub driver_name: *const c_char,
	pub long_name: *const c_char,
	pub dev: *mut device,
	pub owner: *mut c_void,
	pub dai_link: *mut snd_soc_dai_link,
	pub num_links: c_int,
	pub dapm_widgets: *const snd_soc_dapm_widget,
	pub num_dapm_widgets: c_uint,
	pub dapm_routes: *const snd_soc_dapm_route,
	pub num_dapm_routes: c_uint,
	pub fully_routed: bool,
}

#[repr(C)]
pub struct platform_device_id {
	pub name: [c_char; 20],
}

#[repr(C)]
pub struct dev_pm_ops {
	_private: [u8; 0],
}

#[repr(C)]
pub struct platform_driver_inner {
	pub name: *const c_char,
	pub pm: *const dev_pm_ops,
}

#[repr(C)]
pub struct platform_driver {
	pub probe: Option<unsafe extern "C" fn(*mut platform_device) -> c_int>,
	pub driver: platform_driver_inner,
	pub id_table: *const platform_device_id,
}

unsafe extern "C" {
	static mut THIS_MODULE: *mut c_void;
	static snd_soc_pm_ops: dev_pm_ops;

	fn hw_param_mask(params: *mut snd_pcm_hw_params, var: c_int) -> *mut snd_mask;
	fn snd_mask_none(mask: *mut snd_mask);
	fn snd_mask_set_format(mask: *mut snd_mask, format: c_int);
	fn devm_kcalloc(dev: *mut device, n: usize, size: usize, flags: c_uint) -> *mut c_void;
	fn devm_kzalloc(dev: *mut device, size: usize, flags: c_uint) -> *mut c_void;
	fn dev_name(dev: *mut device) -> *const c_char;
	fn dev_get_platdata(dev: *mut device) -> *mut c_void;
	fn devm_snd_soc_register_deferrable_card(dev: *mut device, card: *mut snd_soc_card) -> c_int;
}

const GFP_KERNEL: c_uint = 0;
const ENOMEM: c_int = 12;
const SNDRV_PCM_HW_PARAM_FORMAT: c_int = 0;
const SNDRV_PCM_FORMAT_S24_LE: c_int = 0;
const SND_SOC_DAIFMT_I2S: c_uint = 0;
const SND_SOC_DAIFMT_NB_NF: c_uint = 0;
const SND_SOC_DAIFMT_CBP_CFP: c_uint = 0;

static CARD_WIDGETS: [snd_soc_dapm_widget; 7] = [
	SND_SOC_DAPM_HP!(c"CPB Stereo HP 1".as_ptr(), None),
	SND_SOC_DAPM_HP!(c"CPB Stereo HP 2".as_ptr(), None),
	SND_SOC_DAPM_HP!(c"CPB Stereo HP 3".as_ptr(), None),
	SND_SOC_DAPM_LINE!(c"CPB Line Out".as_ptr(), None),
	SND_SOC_DAPM_MIC!(c"CPB Stereo Mic 1".as_ptr(), None),
	SND_SOC_DAPM_MIC!(c"CPB Stereo Mic 2".as_ptr(), None),
	SND_SOC_DAPM_LINE!(c"CPB Line In".as_ptr(), None),
];

static CARD_ROUTES: [snd_soc_dapm_route; 14] = [
	snd_soc_dapm_route { sink: c"CPB Stereo HP 1".as_ptr(), control: null(), source: c"AOUT1L".as_ptr() },
	snd_soc_dapm_route { sink: c"CPB Stereo HP 1".as_ptr(), control: null(), source: c"AOUT1R".as_ptr() },
	snd_soc_dapm_route { sink: c"CPB Stereo HP 2".as_ptr(), control: null(), source: c"AOUT2L".as_ptr() },
	snd_soc_dapm_route { sink: c"CPB Stereo HP 2".as_ptr(), control: null(), source: c"AOUT2R".as_ptr() },
	snd_soc_dapm_route { sink: c"CPB Stereo HP 3".as_ptr(), control: null(), source: c"AOUT3L".as_ptr() },
	snd_soc_dapm_route { sink: c"CPB Stereo HP 3".as_ptr(), control: null(), source: c"AOUT3R".as_ptr() },
	snd_soc_dapm_route { sink: c"CPB Line Out".as_ptr(), control: null(), source: c"AOUT4L".as_ptr() },
	snd_soc_dapm_route { sink: c"CPB Line Out".as_ptr(), control: null(), source: c"AOUT4R".as_ptr() },

	snd_soc_dapm_route { sink: c"AIN1L".as_ptr(), control: null(), source: c"CPB Stereo Mic 1".as_ptr() },
	snd_soc_dapm_route { sink: c"AIN1R".as_ptr(), control: null(), source: c"CPB Stereo Mic 1".as_ptr() },
	snd_soc_dapm_route { sink: c"AIN2L".as_ptr(), control: null(), source: c"CPB Stereo Mic 2".as_ptr() },
	snd_soc_dapm_route { sink: c"AIN2R".as_ptr(), control: null(), source: c"CPB Stereo Mic 2".as_ptr() },
	snd_soc_dapm_route { sink: c"AIN3L".as_ptr(), control: null(), source: c"CPB Line In".as_ptr() },
	snd_soc_dapm_route { sink: c"AIN3R".as_ptr(), control: null(), source: c"CPB Line In".as_ptr() },
];

unsafe extern "C" fn avs_pcm3168a_be_fixup(
	runtime: *mut snd_soc_pcm_runtime,
	params: *mut snd_pcm_hw_params,
) -> c_int {
	let fmt: *mut snd_mask = unsafe { hw_param_mask(params, SNDRV_PCM_HW_PARAM_FORMAT) };

	/* Set SSP to 24 bit. */
	unsafe {
		snd_mask_none(fmt);
		snd_mask_set_format(fmt, SNDRV_PCM_FORMAT_S24_LE);
	}

	0
}

SND_SOC_DAILINK_DEF!(
	pcm3168a_dac,
	DAILINK_COMP_ARRAY!(COMP_CODEC!(c"i2c-PCM3168A:00".as_ptr(), c"pcm3168a-dac".as_ptr()))
);
SND_SOC_DAILINK_DEF!(
	pcm3168a_adc,
	DAILINK_COMP_ARRAY!(COMP_CODEC!(c"i2c-PCM3168A:00".as_ptr(), c"pcm3168a-adc".as_ptr()))
);
SND_SOC_DAILINK_DEF!(cpu_ssp0, DAILINK_COMP_ARRAY!(COMP_CPU!(c"SSP0 Pin".as_ptr())));
SND_SOC_DAILINK_DEF!(cpu_ssp2, DAILINK_COMP_ARRAY!(COMP_CPU!(c"SSP2 Pin".as_ptr())));

unsafe extern "C" fn avs_create_dai_links(
	dev: *mut device,
	links: *mut *mut snd_soc_dai_link,
	num_links: *mut c_int,
) -> c_int {
	let platform: *mut snd_soc_dai_link_component;
	let dl: *mut snd_soc_dai_link;
	let num_dl: c_int = 2;

	dl = unsafe {
		devm_kcalloc(
			dev,
			num_dl as usize,
			size_of::<snd_soc_dai_link>(),
			GFP_KERNEL,
		) as *mut snd_soc_dai_link
	};
	platform = unsafe {
		devm_kzalloc(dev, size_of::<snd_soc_dai_link_component>(), GFP_KERNEL)
			as *mut snd_soc_dai_link_component
	};
	if dl.is_null() || platform.is_null() {
		return -ENOMEM;
	}

	unsafe {
		(*platform).name = dev_name(dev);
		(*dl.add(0)).num_cpus = 1;
		(*dl.add(0)).num_codecs = 1;
		(*dl.add(0)).platforms = platform;
		(*dl.add(0)).num_platforms = 1;
		(*dl.add(0)).dai_fmt = SND_SOC_DAIFMT_I2S | SND_SOC_DAIFMT_NB_NF | SND_SOC_DAIFMT_CBP_CFP;
		(*dl.add(0)).be_hw_params_fixup = Some(avs_pcm3168a_be_fixup);
		(*dl.add(0)).nonatomic = 1;
		(*dl.add(0)).no_pcm = 1;
		copy_nonoverlapping(dl.add(0), dl.add(1), 1);

		(*dl.add(0)).name = c"SSP0-Codec-dac".as_ptr();
		(*dl.add(0)).cpus = cpu_ssp0;
		(*dl.add(0)).codecs = pcm3168a_dac;
		(*dl.add(1)).name = c"SSP2-Codec-adc".as_ptr();
		(*dl.add(1)).cpus = cpu_ssp2;
		(*dl.add(1)).codecs = pcm3168a_adc;

		*links = dl;
		*num_links = num_dl;
	}
	0
}

unsafe extern "C" fn avs_pcm3168a_probe(pdev: *mut platform_device) -> c_int {
	let mach: *mut snd_soc_acpi_mach;
	let pdata: *mut avs_mach_pdata;
	let dev: *mut device = unsafe { &mut (*pdev).dev };
	let card: *mut snd_soc_card;
	let ret: c_int;

	mach = unsafe { dev_get_platdata(dev) as *mut snd_soc_acpi_mach };
	pdata = unsafe { (*mach).pdata };

	card = unsafe { devm_kzalloc(dev, size_of::<snd_soc_card>(), GFP_KERNEL) as *mut snd_soc_card };
	if card.is_null() {
		return -ENOMEM;
	}

	ret = unsafe { avs_create_dai_links(dev, &mut (*card).dai_link, &mut (*card).num_links) };
	if ret != 0 {
		return ret;
	}

	unsafe {
		if (*pdata).obsolete_card_names {
			(*card).name = c"avs_pcm3168a".as_ptr();
		} else {
			(*card).driver_name = c"avs_pcm3168a".as_ptr();
			(*card).name = c"AVS I2S PCM3168A".as_ptr();
			(*card).long_name = (*card).name;
		}
		(*card).dev = dev;
		(*card).owner = THIS_MODULE;
		(*card).dapm_widgets = CARD_WIDGETS.as_ptr();
		(*card).num_dapm_widgets = CARD_WIDGETS.len() as c_uint;
		(*card).dapm_routes = CARD_ROUTES.as_ptr();
		(*card).num_dapm_routes = CARD_ROUTES.len() as c_uint;
		(*card).fully_routed = true;

		devm_snd_soc_register_deferrable_card(dev, card)
	}
}

static AVS_PCM3168A_DRIVER_IDS: [platform_device_id; 2] = [
	platform_device_id {
		name: [
			b'a' as c_char, b'v' as c_char, b's' as c_char, b'_' as c_char,
			b'p' as c_char, b'c' as c_char, b'm' as c_char, b'3' as c_char,
			b'1' as c_char, b'6' as c_char, b'8' as c_char, b'a' as c_char,
			0, 0, 0, 0, 0, 0, 0, 0,
		],
	},
	platform_device_id { name: [0; 20] },
];
MODULE_DEVICE_TABLE!(platform, AVS_PCM3168A_DRIVER_IDS);

static mut AVS_PCM3168A_DRIVER: platform_driver = platform_driver {
	probe: Some(avs_pcm3168a_probe),
	driver: platform_driver_inner {
		name: c"avs_pcm3168a".as_ptr(),
		pm: unsafe { &snd_soc_pm_ops },
	},
	id_table: AVS_PCM3168A_DRIVER_IDS.as_ptr(),
};

module_platform_driver!(AVS_PCM3168A_DRIVER);

MODULE_DESCRIPTION!(c"Intel pcm3168a machine driver".as_ptr());
MODULE_AUTHOR!(c"Cezary Rojewski <cezary.rojewski@intel.com>".as_ptr());
MODULE_LICENSE!(c"GPL".as_ptr());

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
