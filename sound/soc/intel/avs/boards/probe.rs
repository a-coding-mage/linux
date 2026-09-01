// SPDX-License-Identifier: GPL-2.0-only
//
// Copyright(c) 2021-2022 Intel Corporation
//
// Authors: Cezary Rojewski <cezary.rojewski@intel.com>
//          Amadeusz Slawinski <amadeuszx.slawinski@linux.intel.com>
//

#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]
#![allow(dead_code)]

use core::ffi::{c_char, c_int, c_void};
use core::mem::size_of;
use core::ptr;

const ENOMEM: c_int = 12;
const GFP_KERNEL: c_uint = 0;

type c_uint = u32;

#[repr(C)]
pub struct device {
	_private: [u8; 0],
}

#[repr(C)]
pub struct module {
	_private: [u8; 0],
}

#[repr(C)]
pub struct dev_pm_ops {
	_private: [u8; 0],
}

#[repr(C)]
pub struct platform_device {
	pub dev: device,
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
	pub cpus: *mut snd_soc_dai_link_component,
	pub num_cpus: c_uint,
	pub codecs: *const snd_soc_dai_link_component,
	pub num_codecs: c_uint,
	pub platforms: *mut snd_soc_dai_link_component,
	pub num_platforms: c_uint,
	pub nonatomic: c_uint,
}

#[repr(C)]
pub struct snd_soc_card {
	pub name: *const c_char,
	pub long_name: *const c_char,
	pub driver_name: *const c_char,
	pub dai_link: *mut snd_soc_dai_link,
	pub num_links: c_int,
	pub dev: *mut device,
	pub owner: *mut module,
	pub fully_routed: bool,
}

#[repr(C)]
pub struct platform_device_id {
	pub name: [c_char; 20],
	pub driver_data: usize,
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
	pub id_table: *const platform_device_id,
}

unsafe extern "C" {
	static snd_soc_dummy_dlc: snd_soc_dai_link_component;
	static snd_soc_pm_ops: dev_pm_ops;
	static mut THIS_MODULE: module;

	fn devm_kzalloc(dev: *mut device, size: usize, flags: c_uint) -> *mut c_void;
	fn dev_name(dev: *const device) -> *const c_char;
	fn devm_snd_soc_register_deferrable_card(dev: *mut device, card: *mut snd_soc_card) -> c_int;
}

unsafe extern "C" fn avs_create_dai_links(
	dev: *mut device,
	links: *mut *mut snd_soc_dai_link,
	num_links: *mut c_int,
) -> c_int {
	let dl: *mut snd_soc_dai_link;

	dl = devm_kzalloc(dev, size_of::<snd_soc_dai_link>(), GFP_KERNEL) as *mut snd_soc_dai_link;
	if dl.is_null() {
		return -ENOMEM;
	}

	(*dl).cpus = devm_kzalloc(
		dev,
		size_of::<snd_soc_dai_link_component>(),
		GFP_KERNEL,
	) as *mut snd_soc_dai_link_component;
	(*dl).platforms = devm_kzalloc(
		dev,
		size_of::<snd_soc_dai_link_component>(),
		GFP_KERNEL,
	) as *mut snd_soc_dai_link_component;
	if (*dl).cpus.is_null() || (*dl).platforms.is_null() {
		return -ENOMEM;
	}

	(*dl).name = b"Compress Probe Capture\0".as_ptr() as *const c_char;
	(*(*dl).cpus).dai_name = b"Probe Extraction CPU DAI\0".as_ptr() as *const c_char;
	(*dl).num_cpus = 1;
	(*dl).codecs = &snd_soc_dummy_dlc;
	(*dl).num_codecs = 1;
	(*(*dl).platforms).name = dev_name(dev);
	(*dl).num_platforms = 1;
	(*dl).nonatomic = 1;

	*links = dl;
	*num_links = 1;
	return 0;
}

unsafe extern "C" fn avs_probe_mb_probe(pdev: *mut platform_device) -> c_int {
	let dev: *mut device = &mut (*pdev).dev;
	let card: *mut snd_soc_card;
	let ret: c_int;

	card = devm_kzalloc(dev, size_of::<snd_soc_card>(), GFP_KERNEL) as *mut snd_soc_card;
	if card.is_null() {
		return -ENOMEM;
	}

	ret = avs_create_dai_links(dev, &mut (*card).dai_link, &mut (*card).num_links);
	if ret != 0 {
		return ret;
	}

	(*card).driver_name = b"avs_probe_mb\0".as_ptr() as *const c_char;
	(*card).name = b"AVS PROBE\0".as_ptr() as *const c_char;
	(*card).long_name = (*card).name;
	(*card).dev = dev;
	(*card).owner = &mut THIS_MODULE;
	(*card).fully_routed = true;

	return devm_snd_soc_register_deferrable_card(dev, card);
}

static avs_probe_mb_driver_ids: [platform_device_id; 2] = [
	platform_device_id {
		name: [
			b'a' as c_char,
			b'v' as c_char,
			b's' as c_char,
			b'_' as c_char,
			b'p' as c_char,
			b'r' as c_char,
			b'o' as c_char,
			b'b' as c_char,
			b'e' as c_char,
			b'_' as c_char,
			b'm' as c_char,
			b'b' as c_char,
			0,
			0,
			0,
			0,
			0,
			0,
			0,
			0,
		],
		driver_data: 0,
	},
	platform_device_id {
		name: [0; 20],
		driver_data: 0,
	},
];
// MODULE_DEVICE_TABLE(platform, avs_probe_mb_driver_ids);

static mut avs_probe_mb_driver: platform_driver = platform_driver {
	probe: Some(avs_probe_mb_probe),
	driver: device_driver {
		name: b"avs_probe_mb\0".as_ptr() as *const c_char,
		pm: unsafe { &snd_soc_pm_ops },
	},
	id_table: avs_probe_mb_driver_ids.as_ptr(),
};

// module_platform_driver(avs_probe_mb_driver);

// MODULE_DESCRIPTION("Intel probe machine driver");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
