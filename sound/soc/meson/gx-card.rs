// SPDX-License-Identifier: (GPL-2.0 OR MIT)
//
// Copyright (c) 2020 BayLibre, SAS.
// Author: Jerome Brunet <jbrunet@baylibre.com>

// C dependencies:
// #include <linux/module.h>
// #include <linux/of_platform.h>
// #include <sound/soc.h>
// #include <sound/soc-dai.h>
// #include "meson-card.h"

use core::ffi::{c_char, c_int, c_uint, c_void};

const GFP_KERNEL: c_uint = 0;
const ENOMEM: c_int = 12;
const SNDRV_PCM_FMTBIT_S24_LE: u64 = 1 << 6;

#[repr(C)]
pub struct device {
	_private: [u8; 0],
}

#[repr(C)]
pub struct device_node {
	_private: [u8; 0],
}

#[repr(C)]
pub struct snd_pcm_substream {
	_private: [u8; 0],
}

#[repr(C)]
pub struct snd_pcm_hw_params {
	_private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_pcm_stream {
	pub formats: u64,
	pub rate_min: c_uint,
	pub rate_max: c_uint,
	pub channels_min: c_uint,
	pub channels_max: c_uint,
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
pub struct snd_soc_dai_link_component {
	pub of_node: *mut device_node,
	pub dai_name: *const c_char,
}

#[repr(C)]
pub struct snd_soc_dai_link {
	pub cpus: *mut snd_soc_dai_link_component,
	pub num_cpus: c_uint,
	pub ops: *const snd_soc_ops,
	pub dai_fmt: c_uint,
	pub c2c_params: *const snd_soc_pcm_stream,
	pub num_c2c_params: c_uint,
	pub no_pcm: c_uint,
}

#[repr(C)]
pub struct snd_soc_card {
	pub dev: *mut device,
	pub dai_link: *mut snd_soc_dai_link,
}

#[repr(C)]
pub struct snd_soc_pcm_runtime {
	pub card: *mut snd_soc_card,
	pub id: c_int,
}

#[repr(C)]
pub struct meson_card {
	pub link_data: *mut *mut c_void,
}

#[repr(C)]
pub struct meson_card_match_data {
	pub add_link: Option<
		unsafe extern "C" fn(
			card: *mut snd_soc_card,
			np: *mut device_node,
			index: *mut c_int,
		) -> c_int,
	>,
}

#[repr(C)]
pub struct of_device_id {
	pub compatible: *const c_char,
	pub data: *const c_void,
}

#[repr(C)]
pub struct platform_driver_inner {
	pub name: *const c_char,
	pub of_match_table: *const of_device_id,
}

#[repr(C)]
pub struct platform_driver {
	pub probe: Option<unsafe extern "C" fn(*mut c_void) -> c_int>,
	pub remove: Option<unsafe extern "C" fn(*mut c_void) -> c_int>,
	pub driver: platform_driver_inner,
}

#[repr(C)]
struct gx_dai_link_i2s_data {
	mclk_fs: c_uint,
}

unsafe extern "C" {
	fn snd_soc_substream_to_rtd(substream: *mut snd_pcm_substream) -> *mut snd_soc_pcm_runtime;
	fn snd_soc_card_get_drvdata(card: *mut snd_soc_card) -> *mut c_void;
	fn meson_card_i2s_set_sysclk(
		substream: *mut snd_pcm_substream,
		params: *mut snd_pcm_hw_params,
		mclk_fs: c_uint,
	) -> c_int;
	fn devm_kzalloc(dev: *mut device, size: usize, flags: c_uint) -> *mut c_void;
	fn meson_card_parse_daifmt(node: *mut device_node, cpu_node: *mut device_node) -> c_uint;
	fn of_property_read_u32(
		np: *mut device_node,
		propname: *const c_char,
		out_value: *mut c_uint,
	) -> c_int;
	fn of_device_is_compatible(device: *mut device_node, compat: *const c_char) -> c_int;
	fn strstr(haystack: *const c_char, needle: *const c_char) -> *mut c_char;
	fn meson_card_parse_dai(
		card: *mut snd_soc_card,
		np: *mut device_node,
		dlc: *mut snd_soc_dai_link_component,
	) -> c_int;
	fn meson_card_set_fe_link(
		card: *mut snd_soc_card,
		link: *mut snd_soc_dai_link,
		np: *mut device_node,
		is_playback: bool,
	) -> c_int;
	fn meson_card_set_be_link(
		card: *mut snd_soc_card,
		link: *mut snd_soc_dai_link,
		np: *mut device_node,
	) -> c_int;
	fn meson_card_probe(pdev: *mut c_void) -> c_int;
	fn meson_card_remove(pdev: *mut c_void) -> c_int;
}

/*
 * Base params for the codec to codec links
 * Those will be over-written by the CPU side of the link
 */
static codec_params: snd_soc_pcm_stream = snd_soc_pcm_stream {
	formats: SNDRV_PCM_FMTBIT_S24_LE,
	rate_min: 5525,
	rate_max: 192000,
	channels_min: 1,
	channels_max: 8,
};

unsafe extern "C" fn gx_card_i2s_be_hw_params(
	substream: *mut snd_pcm_substream,
	params: *mut snd_pcm_hw_params,
) -> c_int {
	let rtd = snd_soc_substream_to_rtd(substream);
	let priv_data = snd_soc_card_get_drvdata((*rtd).card);
	let priv_ = priv_data as *mut meson_card;
	let be = *(*priv_).link_data.offset((*rtd).id as isize) as *mut gx_dai_link_i2s_data;

	meson_card_i2s_set_sysclk(substream, params, (*be).mclk_fs)
}

static gx_card_i2s_be_ops: snd_soc_ops = snd_soc_ops {
	hw_params: Some(gx_card_i2s_be_hw_params),
};

unsafe extern "C" fn gx_card_parse_i2s(
	card: *mut snd_soc_card,
	node: *mut device_node,
	index: *mut c_int,
) -> c_int {
	let priv_data = snd_soc_card_get_drvdata(card);
	let priv_ = priv_data as *mut meson_card;
	let link = (*card).dai_link.offset(*index as isize);
	let be: *mut gx_dai_link_i2s_data;
	let dev = (*card).dev;

	/* Allocate i2s link parameters */
	be = devm_kzalloc(
		dev,
		core::mem::size_of::<gx_dai_link_i2s_data>(),
		GFP_KERNEL,
	) as *mut gx_dai_link_i2s_data;
	if be.is_null() {
		return -ENOMEM;
	}
	*(*priv_).link_data.offset(*index as isize) = be as *mut c_void;

	/* Setup i2s link */
	(*link).ops = &gx_card_i2s_be_ops;
	(*link).dai_fmt = meson_card_parse_daifmt(node, (*(*link).cpus).of_node);

	of_property_read_u32(node, c"mclk-fs".as_ptr(), &mut (*be).mclk_fs);

	0
}

unsafe extern "C" fn gx_card_cpu_identify(
	c: *mut snd_soc_dai_link_component,
	match_: *mut c_char,
) -> c_int {
	// C source used: of_device_is_compatible(c->of_node, DT_PREFIX "aiu")
	// DT_PREFIX is provided by an included dependency, so the concatenated
	// compatible string is kept as an external dependency here.
	unsafe extern "C" {
		static DT_PREFIX_AIU: c_char;
	}

	if of_device_is_compatible((*c).of_node, &DT_PREFIX_AIU as *const c_char) != 0 {
		if !strstr((*c).dai_name, match_ as *const c_char).is_null() {
			return 1;
		}
	}

	/* dai not matched */
	0
}

unsafe extern "C" fn gx_card_add_link(
	card: *mut snd_soc_card,
	np: *mut device_node,
	index: *mut c_int,
) -> c_int {
	let dai_link = (*card).dai_link.offset(*index as isize);
	let cpu: *mut snd_soc_dai_link_component;
	let dev = (*card).dev;
	let mut ret: c_int;

	cpu = devm_kzalloc(
		dev,
		core::mem::size_of::<snd_soc_dai_link_component>(),
		GFP_KERNEL,
	) as *mut snd_soc_dai_link_component;
	if cpu.is_null() {
		return -ENOMEM;
	}

	(*dai_link).cpus = cpu;
	(*dai_link).num_cpus = 1;

	ret = meson_card_parse_dai(card, np, (*dai_link).cpus);
	if ret != 0 {
		return ret;
	}

	if gx_card_cpu_identify((*dai_link).cpus, c"FIFO".as_ptr() as *mut c_char) != 0 {
		return meson_card_set_fe_link(card, dai_link, np, true);
	}

	ret = meson_card_set_be_link(card, dai_link, np);
	if ret != 0 {
		return ret;
	}

	/* Or apply codec to codec params if necessary */
	if gx_card_cpu_identify((*dai_link).cpus, c"CODEC CTRL".as_ptr() as *mut c_char) != 0 {
		(*dai_link).c2c_params = &codec_params;
		(*dai_link).num_c2c_params = 1;
	} else {
		(*dai_link).no_pcm = 1;
		/* Check if the cpu is the i2s encoder and parse i2s data */
		if gx_card_cpu_identify((*dai_link).cpus, c"I2S Encoder".as_ptr() as *mut c_char) != 0 {
			ret = gx_card_parse_i2s(card, np, index);
		}
	}

	ret
}

static gx_card_match_data: meson_card_match_data = meson_card_match_data {
	add_link: Some(gx_card_add_link),
};

static gx_card_of_match: [of_device_id; 2] = [
	of_device_id {
		compatible: c"amlogic,gx-sound-card".as_ptr(),
		data: &gx_card_match_data as *const meson_card_match_data as *const c_void,
	},
	of_device_id {
		compatible: core::ptr::null(),
		data: core::ptr::null(),
	},
];
// MODULE_DEVICE_TABLE(of, gx_card_of_match);

static mut gx_card_pdrv: platform_driver = platform_driver {
	probe: Some(meson_card_probe),
	remove: Some(meson_card_remove),
	driver: platform_driver_inner {
		name: c"gx-sound-card".as_ptr(),
		of_match_table: gx_card_of_match.as_ptr(),
	},
};
// module_platform_driver(gx_card_pdrv);

// MODULE_DESCRIPTION("Amlogic GX ALSA machine driver");
// MODULE_AUTHOR("Jerome Brunet <jbrunet@baylibre.com>");
// MODULE_LICENSE("GPL v2");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
