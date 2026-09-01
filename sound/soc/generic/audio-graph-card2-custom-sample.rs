// SPDX-License-Identifier: GPL-2.0
//
// audio-graph-card2-custom-sample.c
//
// Copyright (C) 2020 Renesas Electronics Corp.
// Copyright (C) 2020 Kuninori Morimoto <kuninori.morimoto.gx@renesas.com>
//
// C dependencies:
// #include <linux/device.h>
// #include <linux/module.h>
// #include <linux/platform_device.h>
// #include <sound/graph_card.h>

use core::ffi::{c_char, c_int, c_void};
use core::mem::size_of;
use core::ptr;

const GFP_KERNEL: c_int = 0;
const ENOMEM: c_int = 12;

#[repr(C)]
pub struct device {
	_private: [u8; 0],
}

#[repr(C)]
pub struct device_node {
	_private: [u8; 0],
}

#[repr(C)]
pub struct link_info {
	_private: [u8; 0],
}

#[repr(C)]
pub struct snd_pcm_substream {
	_private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_card {
	pub name: *const c_char,
	pub probe: Option<unsafe extern "C" fn(*mut snd_soc_card) -> c_int>,
}

#[repr(C)]
pub struct simple_util_priv {
	pub ops: *const snd_soc_ops,
	pub snd_card: snd_soc_card,
}

#[repr(C)]
pub struct snd_soc_pcm_runtime {
	pub card: *mut snd_soc_card,
}

#[repr(C)]
pub struct snd_soc_ops {
	pub startup: Option<unsafe extern "C" fn(*mut snd_pcm_substream) -> c_int>,
	pub shutdown: Option<unsafe extern "C" fn(*mut snd_pcm_substream)>,
	pub hw_params: Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut c_void) -> c_int>,
}

#[repr(C)]
pub struct graph2_custom_hooks {
	pub hook_pre: Option<unsafe extern "C" fn(*mut simple_util_priv) -> c_int>,
	pub hook_post: Option<unsafe extern "C" fn(*mut simple_util_priv) -> c_int>,
	pub custom_normal:
		Option<unsafe extern "C" fn(*mut simple_util_priv, *mut device_node, *mut link_info) -> c_int>,
	pub custom_dpcm:
		Option<unsafe extern "C" fn(*mut simple_util_priv, *mut device_node, *mut link_info) -> c_int>,
	pub custom_c2c:
		Option<unsafe extern "C" fn(*mut simple_util_priv, *mut device_node, *mut link_info) -> c_int>,
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
pub struct driver_private {
	pub name: *const c_char,
	pub of_match_table: *const of_device_id,
}

#[repr(C)]
pub struct platform_driver {
	pub driver: driver_private,
	pub probe: Option<unsafe extern "C" fn(*mut platform_device) -> c_int>,
	pub remove: Option<unsafe extern "C" fn(*mut platform_device) -> c_int>,
}

unsafe extern "C" {
	fn snd_soc_card_get_drvdata(card: *mut snd_soc_card) -> *mut simple_util_priv;
	fn simple_priv_to_dev(priv_: *mut simple_util_priv) -> *mut device;
	fn graph_util_card_probe(card: *mut snd_soc_card) -> c_int;
	fn dev_info(dev: *mut device, fmt: *const c_char, ...);
	fn simple_priv_to_card(priv_: *mut simple_util_priv) -> *mut snd_soc_card;
	fn audio_graph2_link_normal(
		priv_: *mut simple_util_priv,
		lnk: *mut device_node,
		li: *mut link_info,
	) -> c_int;
	fn audio_graph2_link_dpcm(
		priv_: *mut simple_util_priv,
		lnk: *mut device_node,
		li: *mut link_info,
	) -> c_int;
	fn audio_graph2_link_c2c(
		priv_: *mut simple_util_priv,
		lnk: *mut device_node,
		li: *mut link_info,
	) -> c_int;
	fn snd_soc_substream_to_rtd(substream: *mut snd_pcm_substream) -> *mut snd_soc_pcm_runtime;
	fn simple_util_startup(substream: *mut snd_pcm_substream) -> c_int;
	fn simple_util_shutdown(substream: *mut snd_pcm_substream);
	fn simple_util_hw_params(substream: *mut snd_pcm_substream, params: *mut c_void) -> c_int;
	fn devm_kzalloc(dev: *mut device, size: usize, flags: c_int) -> *mut c_void;
	fn audio_graph2_parse_of(
		priv_: *mut simple_util_priv,
		dev: *mut device,
		hooks: *mut graph2_custom_hooks,
	) -> c_int;
	fn simple_util_remove(pdev: *mut platform_device) -> c_int;
}

/*
 * Custom driver can have own priv
 * which includes simple_util_priv.
 */
#[repr(C)]
pub struct custom_priv {
	pub simple_priv: simple_util_priv,

	/* custom driver's own params */
	pub custom_params: c_int,
}

/* You can get custom_priv from simple_priv */
/* #define simple_to_custom(simple) container_of((simple), struct custom_priv, simple_priv) */
unsafe fn simple_to_custom(simple: *mut simple_util_priv) -> *mut custom_priv {
	simple.cast::<custom_priv>()
}

unsafe extern "C" fn custom_card_probe(card: *mut snd_soc_card) -> c_int {
	let simple_priv = snd_soc_card_get_drvdata(card);
	let custom_priv = simple_to_custom(simple_priv);
	let dev = simple_priv_to_dev(simple_priv);

	dev_info(dev, b"custom probe\n\0".as_ptr().cast::<c_char>());

	(*custom_priv).custom_params = 1;

	/* you can use generic probe function */
	graph_util_card_probe(card)
}

unsafe extern "C" fn custom_hook_pre(priv_: *mut simple_util_priv) -> c_int {
	let dev = simple_priv_to_dev(priv_);

	/* You can custom before parsing */
	dev_info(
		dev,
		b"hook : %s\n\0".as_ptr().cast::<c_char>(),
		b"custom_hook_pre\0".as_ptr().cast::<c_char>(),
	);

	0
}

unsafe extern "C" fn custom_hook_post(priv_: *mut simple_util_priv) -> c_int {
	let dev = simple_priv_to_dev(priv_);
	let card: *mut snd_soc_card;

	/* You can custom after parsing */
	dev_info(
		dev,
		b"hook : %s\n\0".as_ptr().cast::<c_char>(),
		b"custom_hook_post\0".as_ptr().cast::<c_char>(),
	);

	/* overwrite .probe sample */
	card = simple_priv_to_card(priv_);
	(*card).probe = Some(custom_card_probe);

	0
}

unsafe extern "C" fn custom_normal(
	priv_: *mut simple_util_priv,
	lnk: *mut device_node,
	li: *mut link_info,
) -> c_int {
	let dev = simple_priv_to_dev(priv_);

	/*
	 * You can custom Normal parsing
	 * before/affter audio_graph2_link_normal()
	 */
	dev_info(
		dev,
		b"hook : %s\n\0".as_ptr().cast::<c_char>(),
		b"custom_normal\0".as_ptr().cast::<c_char>(),
	);

	audio_graph2_link_normal(priv_, lnk, li)
}

unsafe extern "C" fn custom_dpcm(
	priv_: *mut simple_util_priv,
	lnk: *mut device_node,
	li: *mut link_info,
) -> c_int {
	let dev = simple_priv_to_dev(priv_);

	/*
	 * You can custom DPCM parsing
	 * before/affter audio_graph2_link_dpcm()
	 */
	dev_info(
		dev,
		b"hook : %s\n\0".as_ptr().cast::<c_char>(),
		b"custom_dpcm\0".as_ptr().cast::<c_char>(),
	);

	audio_graph2_link_dpcm(priv_, lnk, li)
}

unsafe extern "C" fn custom_c2c(
	priv_: *mut simple_util_priv,
	lnk: *mut device_node,
	li: *mut link_info,
) -> c_int {
	let dev = simple_priv_to_dev(priv_);

	/*
	 * You can custom Codec2Codec parsing
	 * before/affter audio_graph2_link_c2c()
	 */
	dev_info(
		dev,
		b"hook : %s\n\0".as_ptr().cast::<c_char>(),
		b"custom_c2c\0".as_ptr().cast::<c_char>(),
	);

	audio_graph2_link_c2c(priv_, lnk, li)
}

/*
 * audio-graph-card2 has many hooks for your customizing.
 */
static mut custom_hooks: graph2_custom_hooks = graph2_custom_hooks {
	hook_pre: Some(custom_hook_pre),
	hook_post: Some(custom_hook_post),
	custom_normal: Some(custom_normal),
	custom_dpcm: Some(custom_dpcm),
	custom_c2c: Some(custom_c2c),
};

unsafe extern "C" fn custom_startup(substream: *mut snd_pcm_substream) -> c_int {
	let rtd = snd_soc_substream_to_rtd(substream);
	let priv_ = snd_soc_card_get_drvdata((*rtd).card);
	let dev = simple_priv_to_dev(priv_);

	dev_info(dev, b"custom startup\n\0".as_ptr().cast::<c_char>());

	simple_util_startup(substream)
}

/* You can use custom ops */
static custom_ops: snd_soc_ops = snd_soc_ops {
	startup: Some(custom_startup),
	shutdown: Some(simple_util_shutdown),
	hw_params: Some(simple_util_hw_params),
};

unsafe extern "C" fn custom_probe(pdev: *mut platform_device) -> c_int {
	let custom_priv: *mut custom_priv;
	let simple_priv: *mut simple_util_priv;
	let dev = ptr::addr_of_mut!((*pdev).dev);
	let ret: c_int;

	custom_priv = devm_kzalloc(dev, size_of::<custom_priv>(), GFP_KERNEL).cast::<custom_priv>();
	if custom_priv.is_null() {
		return -ENOMEM;
	}

	simple_priv = ptr::addr_of_mut!((*custom_priv).simple_priv);
	(*simple_priv).ops = ptr::addr_of!(custom_ops);

	/* "audio-graph-card2-custom-sample" is too long */
	(*simple_priv).snd_card.name = b"card2-custom\0".as_ptr().cast::<c_char>();

	/* use audio-graph-card2 parsing with own custom hooks */
	ret = audio_graph2_parse_of(simple_priv, dev, ptr::addr_of_mut!(custom_hooks));
	if ret < 0 {
		return ret;
	}

	/* customize more if needed */

	0
}

static custom_of_match: [of_device_id; 2] = [
	of_device_id {
		compatible: b"audio-graph-card2-custom-sample\0".as_ptr().cast::<c_char>(),
	},
	of_device_id {
		compatible: ptr::null(),
	},
];
/* MODULE_DEVICE_TABLE(of, custom_of_match); */

static mut custom_card: platform_driver = platform_driver {
	driver: driver_private {
		name: b"audio-graph-card2-custom-sample\0".as_ptr().cast::<c_char>(),
		of_match_table: custom_of_match.as_ptr(),
	},
	probe: Some(custom_probe),
	remove: Some(simple_util_remove),
};
/* module_platform_driver(custom_card); */

/* MODULE_ALIAS("platform:asoc-audio-graph-card2-custom-sample"); */
/* MODULE_LICENSE("GPL v2"); */
/* MODULE_DESCRIPTION("ASoC Audio Graph Card2 Custom Sample"); */
/* MODULE_AUTHOR("Kuninori Morimoto <kuninori.morimoto.gx@renesas.com>"); */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
