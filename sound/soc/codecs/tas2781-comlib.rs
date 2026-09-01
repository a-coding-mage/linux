// SPDX-License-Identifier: GPL-2.0
//
// TAS2563/TAS2781 Common functions for HDA and ASoC Audio drivers
//
// Copyright 2023 - 2025 Texas Instruments, Inc.
//
// Author: Shenghao Ding <shenghao-ding@ti.com>

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_uint, c_ushort, c_void};
use core::ptr;

const EINVAL: c_int = 22;

#[repr(C)]
pub struct device {
	_private: [u8; 0],
}

#[repr(C)]
pub struct regmap {
	_private: [u8; 0],
}

#[repr(C)]
pub struct mutex {
	_private: [u8; 0],
}

#[repr(C)]
pub struct tasdev_blk {
	pub data: *mut c_void,
}

#[repr(C)]
pub struct tasdevice_data {
	pub dev_blks: *mut tasdev_blk,
	pub nr_blk: c_uint,
}

#[repr(C)]
pub struct tasdevice_prog {
	pub dev_data: tasdevice_data,
}

#[repr(C)]
pub struct tasdevice_config {
	pub dev_data: tasdevice_data,
}

#[repr(C)]
pub struct tasdevice_fw {
	pub programs: *mut tasdevice_prog,
	pub nr_programs: c_ushort,
	pub configs: *mut tasdevice_config,
	pub nr_configurations: c_ushort,
}

#[repr(C)]
pub struct tasdevice_priv {
	pub ndev: c_ushort,
	pub regmap: *mut regmap,
	pub change_chn_book:
		unsafe extern "C" fn(*mut tasdevice_priv, c_ushort, c_uint) -> c_int,
	pub dev: *mut device,
	pub fmw: *mut tasdevice_fw,
	pub codec_lock: mutex,
}

unsafe extern "C" {
	fn TASDEVICE_BOOK_ID(reg: c_uint) -> c_uint;
	fn TASDEVICE_PGRG(reg: c_uint) -> c_uint;
	fn regmap_read(map: *mut regmap, reg: c_uint, val: *mut c_uint) -> c_int;
	fn regmap_bulk_read(
		map: *mut regmap,
		reg: c_uint,
		data: *mut u8,
		len: c_uint,
	) -> c_int;
	fn regmap_write(map: *mut regmap, reg: c_uint, value: c_uint) -> c_int;
	fn regmap_bulk_write(
		map: *mut regmap,
		reg: c_uint,
		data: *mut u8,
		len: c_uint,
	) -> c_int;
	fn dev_err(dev: *mut device, fmt: *const c_char, ...);
	fn kfree(ptr: *const c_void);
	fn mutex_destroy(lock: *mut mutex);
}

const TASDEVICE_DEV_READ_E_FMT: &[u8] = b"%s, E=%d\n\0";
const TASDEVICE_DEV_READ_NO_CHANNEL_FMT: &[u8] = b"%s, no such channel(%d)\n\0";
const TASDEVICE_DEV_READ_FUNC: &[u8] = b"tasdevice_dev_read\0";
const TASDEVICE_DEV_BULK_READ_FUNC: &[u8] = b"tasdevice_dev_bulk_read\0";
const TASDEVICE_DEV_WRITE_FUNC: &[u8] = b"tasdevice_dev_write\0";
const TASDEVICE_DEV_BULK_WRITE_FUNC: &[u8] = b"tasdevice_dev_bulk_write\0";

#[unsafe(no_mangle)]
pub unsafe extern "C" fn tasdevice_dev_read(
	tas_priv: *mut tasdevice_priv,
	chn: c_ushort,
	reg: c_uint,
	val: *mut c_uint,
) -> c_int {
	let mut ret: c_int = 0;

	if chn < (*tas_priv).ndev {
		let map: *mut regmap = (*tas_priv).regmap;

		ret = ((*tas_priv).change_chn_book)(tas_priv, chn, TASDEVICE_BOOK_ID(reg));
		if ret < 0 {
			return ret;
		}

		ret = regmap_read(map, TASDEVICE_PGRG(reg), val);
		if ret < 0 {
			dev_err(
				(*tas_priv).dev,
				TASDEVICE_DEV_READ_E_FMT.as_ptr() as *const c_char,
				TASDEVICE_DEV_READ_FUNC.as_ptr() as *const c_char,
				ret,
			);
		}
	} else {
		ret = -EINVAL;
		dev_err(
			(*tas_priv).dev,
			TASDEVICE_DEV_READ_NO_CHANNEL_FMT.as_ptr() as *const c_char,
			TASDEVICE_DEV_READ_FUNC.as_ptr() as *const c_char,
			chn as c_int,
		);
	}

	ret
}

// EXPORT_SYMBOL_GPL(tasdevice_dev_read);

#[unsafe(no_mangle)]
pub unsafe extern "C" fn tasdevice_dev_bulk_read(
	tas_priv: *mut tasdevice_priv,
	chn: c_ushort,
	reg: c_uint,
	data: *mut u8,
	len: c_uint,
) -> c_int {
	let mut ret: c_int = 0;

	if chn < (*tas_priv).ndev {
		let map: *mut regmap = (*tas_priv).regmap;

		ret = ((*tas_priv).change_chn_book)(tas_priv, chn, TASDEVICE_BOOK_ID(reg));
		if ret < 0 {
			return ret;
		}

		ret = regmap_bulk_read(map, TASDEVICE_PGRG(reg), data, len);
		if ret < 0 {
			dev_err(
				(*tas_priv).dev,
				TASDEVICE_DEV_READ_E_FMT.as_ptr() as *const c_char,
				TASDEVICE_DEV_BULK_READ_FUNC.as_ptr() as *const c_char,
				ret,
			);
		}
	} else {
		dev_err(
			(*tas_priv).dev,
			TASDEVICE_DEV_READ_NO_CHANNEL_FMT.as_ptr() as *const c_char,
			TASDEVICE_DEV_BULK_READ_FUNC.as_ptr() as *const c_char,
			chn as c_int,
		);
	}

	ret
}

// EXPORT_SYMBOL_GPL(tasdevice_dev_bulk_read);

#[unsafe(no_mangle)]
pub unsafe extern "C" fn tasdevice_dev_write(
	tas_priv: *mut tasdevice_priv,
	chn: c_ushort,
	reg: c_uint,
	value: c_uint,
) -> c_int {
	let mut ret: c_int = 0;

	if chn < (*tas_priv).ndev {
		let map: *mut regmap = (*tas_priv).regmap;

		ret = ((*tas_priv).change_chn_book)(tas_priv, chn, TASDEVICE_BOOK_ID(reg));
		if ret < 0 {
			return ret;
		}

		ret = regmap_write(map, TASDEVICE_PGRG(reg), value);
		if ret < 0 {
			dev_err(
				(*tas_priv).dev,
				TASDEVICE_DEV_READ_E_FMT.as_ptr() as *const c_char,
				TASDEVICE_DEV_WRITE_FUNC.as_ptr() as *const c_char,
				ret,
			);
		}
	} else {
		ret = -EINVAL;
		dev_err(
			(*tas_priv).dev,
			TASDEVICE_DEV_READ_NO_CHANNEL_FMT.as_ptr() as *const c_char,
			TASDEVICE_DEV_WRITE_FUNC.as_ptr() as *const c_char,
			chn as c_int,
		);
	}

	ret
}

// EXPORT_SYMBOL_GPL(tasdevice_dev_write);

#[unsafe(no_mangle)]
pub unsafe extern "C" fn tasdevice_dev_bulk_write(
	tas_priv: *mut tasdevice_priv,
	chn: c_ushort,
	reg: c_uint,
	data: *mut u8,
	len: c_uint,
) -> c_int {
	let mut ret: c_int = 0;

	if chn < (*tas_priv).ndev {
		let map: *mut regmap = (*tas_priv).regmap;

		ret = ((*tas_priv).change_chn_book)(tas_priv, chn, TASDEVICE_BOOK_ID(reg));
		if ret < 0 {
			return ret;
		}

		ret = regmap_bulk_write(map, TASDEVICE_PGRG(reg), data, len);
		if ret < 0 {
			dev_err(
				(*tas_priv).dev,
				TASDEVICE_DEV_READ_E_FMT.as_ptr() as *const c_char,
				TASDEVICE_DEV_BULK_WRITE_FUNC.as_ptr() as *const c_char,
				ret,
			);
		}
	} else {
		ret = -EINVAL;
		dev_err(
			(*tas_priv).dev,
			TASDEVICE_DEV_READ_NO_CHANNEL_FMT.as_ptr() as *const c_char,
			TASDEVICE_DEV_BULK_WRITE_FUNC.as_ptr() as *const c_char,
			chn as c_int,
		);
	}

	ret
}

// EXPORT_SYMBOL_GPL(tasdevice_dev_bulk_write);

unsafe fn tasdev_dsp_prog_blk_remove(prog: *mut tasdevice_prog) {
	let tas_dt: *mut tasdevice_data;
	let mut blk: *mut tasdev_blk;
	let mut i: c_uint;

	if prog.is_null() {
		return;
	}

	tas_dt = &mut (*prog).dev_data;

	if (*tas_dt).dev_blks.is_null() {
		return;
	}

	i = 0;
	while i < (*tas_dt).nr_blk {
		blk = (*tas_dt).dev_blks.add(i as usize);
		kfree((*blk).data as *const c_void);
		i = i.wrapping_add(1);
	}
	kfree((*tas_dt).dev_blks as *const c_void);
}

unsafe fn tasdev_dsp_prog_remove(prog: *mut tasdevice_prog, nr: c_ushort) {
	let mut i: c_int;

	i = 0;
	while i < nr as c_int {
		tasdev_dsp_prog_blk_remove(prog.add(i as usize));
		i += 1;
	}
	kfree(prog as *const c_void);
}

unsafe fn tasdev_dsp_cfg_blk_remove(cfg: *mut tasdevice_config) {
	let tas_dt: *mut tasdevice_data;
	let mut blk: *mut tasdev_blk;
	let mut i: c_uint;

	if !cfg.is_null() {
		tas_dt = &mut (*cfg).dev_data;

		if (*tas_dt).dev_blks.is_null() {
			return;
		}

		i = 0;
		while i < (*tas_dt).nr_blk {
			blk = (*tas_dt).dev_blks.add(i as usize);
			kfree((*blk).data as *const c_void);
			i = i.wrapping_add(1);
		}
		kfree((*tas_dt).dev_blks as *const c_void);
	}
}

unsafe fn tasdev_dsp_cfg_remove(config: *mut tasdevice_config, nr: c_ushort) {
	let mut i: c_int;

	i = 0;
	while i < nr as c_int {
		tasdev_dsp_cfg_blk_remove(config.add(i as usize));
		i += 1;
	}
	kfree(config as *const c_void);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn tasdevice_dsp_remove(context: *mut c_void) {
	let tas_dev: *mut tasdevice_priv = context as *mut tasdevice_priv;
	let tas_fmw: *mut tasdevice_fw = (*tas_dev).fmw;

	if (*tas_dev).fmw.is_null() {
		return;
	}

	if !(*tas_fmw).programs.is_null() {
		tasdev_dsp_prog_remove((*tas_fmw).programs, (*tas_fmw).nr_programs);
	}
	if !(*tas_fmw).configs.is_null() {
		tasdev_dsp_cfg_remove((*tas_fmw).configs, (*tas_fmw).nr_configurations);
	}
	kfree(tas_fmw as *const c_void);
	(*tas_dev).fmw = ptr::null_mut();
}

// EXPORT_SYMBOL_GPL(tasdevice_dsp_remove);

#[unsafe(no_mangle)]
pub unsafe extern "C" fn tasdevice_remove(tas_priv: *mut tasdevice_priv) {
	mutex_destroy(&mut (*tas_priv).codec_lock);
}

// EXPORT_SYMBOL_GPL(tasdevice_remove);

// MODULE_DESCRIPTION("TAS2781 common library");
// MODULE_AUTHOR("Shenghao Ding, TI, <shenghao-ding@ti.com>");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
