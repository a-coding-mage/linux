// SPDX-License-Identifier: (GPL-2.0-only OR BSD-3-Clause)
//
// This file is provided under a dual BSD/GPLv2 license.  When using or
// redistributing this file, you may do so under either license.
//
// Copyright(c) 2023 Intel Corporation
//

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::mem;
use core::ptr;

type u32 = c_uint;
type bool_ = bool;
type sof_ipc_type = c_int;

const GFP_KERNEL: c_uint = 0;
const ENOMEM: c_int = 12;
const EINVAL: c_int = 22;
const SOF_IPC_TYPE_3: sof_ipc_type = 3;
const SOF_IPC_TYPE_4: sof_ipc_type = 4;
const SOF_IPC_TYPE_COUNT: c_int = 5;

/* Build-time Kconfig condition from C:
 * IS_ENABLED(CONFIG_SND_SOC_SOF_ALLOW_FALLBACK_TO_NEWER_IPC_VERSION)
 */
const CONFIG_SND_SOC_SOF_ALLOW_FALLBACK_TO_NEWER_IPC_VERSION_ENABLED: bool = false;

#[repr(C)]
pub struct device {
	_private: [u8; 0],
}

#[repr(C)]
pub struct firmware {
	pub size: usize,
	pub data: *const u8,
}

#[repr(C)]
pub struct sof_loadable_file_profile {
	pub fw_path: *const c_char,
	pub fw_name: *const c_char,
	pub fw_path_postfix: *const c_char,
	pub fw_lib_path: *const c_char,
	pub fw_lib_path_postfix: *const c_char,
	pub tplg_path: *const c_char,
	pub tplg_name: *const c_char,
	pub ipc_type: sof_ipc_type,
}

#[repr(C)]
pub struct snd_sof_dev {
	pub dev: *mut device,
	pub pdata: *mut snd_sof_pdata,
}

#[repr(C)]
pub struct snd_sof_pdata {
	pub desc: *const sof_dev_desc,
	pub tplg_filename: *const c_char,
	pub machine: *mut snd_soc_acpi_mach,
	pub disable_function_topology: bool_,
}

#[repr(C)]
pub struct snd_soc_acpi_mach {
	pub get_function_tplg_files: *const c_void,
}

#[repr(C)]
pub struct sof_dev_desc {
	pub ops: *const sof_ops,
	pub default_fw_path: [*const c_char; SOF_IPC_TYPE_COUNT as usize],
	pub default_fw_filename: [*const c_char; SOF_IPC_TYPE_COUNT as usize],
	pub default_lib_path: [*const c_char; SOF_IPC_TYPE_COUNT as usize],
	pub default_tplg_path: [*const c_char; SOF_IPC_TYPE_COUNT as usize],
	pub ipc_supported_mask: c_uint,
}

#[repr(C)]
pub struct sof_ops {
	pub load_firmware: Option<unsafe extern "C" fn(*mut snd_sof_dev) -> c_int>,
}

unsafe extern "C" {
	static SOF_EXT_MAN_MAGIC_NUMBER: u32;
	static SOF_EXT_MAN4_MAGIC_NUMBER: u32;

	static snd_sof_load_firmware_raw: unsafe extern "C" fn(*mut snd_sof_dev) -> c_int;
	static snd_sof_load_firmware_memcpy: unsafe extern "C" fn(*mut snd_sof_dev) -> c_int;

	fn kasprintf(gfp: c_uint, fmt: *const c_char, ...) -> *mut c_char;
	fn devm_kasprintf(dev: *mut device, gfp: c_uint, fmt: *const c_char, ...) -> *mut c_char;
	fn kfree(ptr: *const c_void);
	fn devm_kfree(dev: *mut device, ptr: *const c_void);
	fn firmware_request_nowarn(fw: *mut *const firmware, name: *const c_char, device: *mut device) -> c_int;
	fn release_firmware(fw: *const firmware);
	fn strstr(haystack: *const c_char, needle: *const c_char) -> *mut c_char;
	fn memset(s: *mut c_void, c: c_int, n: usize) -> *mut c_void;
	fn dev_dbg(dev: *mut device, fmt: *const c_char, ...);
	fn dev_err(dev: *mut device, fmt: *const c_char, ...);
	fn dev_info(dev: *mut device, fmt: *const c_char, ...);
}

#[inline]
fn BIT(nr: c_int) -> c_uint {
	1u32.wrapping_shl(nr as u32)
}

unsafe fn sof_test_firmware_file(
	dev: *mut device,
	profile: *mut sof_loadable_file_profile,
	ipc_type_to_adjust: *mut sof_ipc_type,
) -> c_int {
	let mut fw_ipc_type: sof_ipc_type;
	let magic: *const u32;
	let ret: c_int;

	let fw_filename = kasprintf(
		GFP_KERNEL,
		b"%s/%s\0".as_ptr() as *const c_char,
		(*profile).fw_path,
		(*profile).fw_name,
	);
	if fw_filename.is_null() {
		return -ENOMEM;
	}

	let mut fw: *const firmware = ptr::null();
	ret = firmware_request_nowarn(&mut fw, fw_filename, dev);
	if ret < 0 {
		dev_dbg(
			dev,
			b"Failed to open firmware file: %s\n\0".as_ptr() as *const c_char,
			fw_filename,
		);
		kfree(fw_filename as *const c_void);
		return ret;
	}

	/* firmware file exists, check the magic number */
	magic = (*fw).data as *const u32;
	if *magic == SOF_EXT_MAN_MAGIC_NUMBER {
		fw_ipc_type = SOF_IPC_TYPE_3;
	} else if *magic == SOF_EXT_MAN4_MAGIC_NUMBER {
		fw_ipc_type = SOF_IPC_TYPE_4;
	} else {
		dev_err(
			dev,
			b"Invalid firmware magic: %#x\n\0".as_ptr() as *const c_char,
			*magic,
		);
		release_firmware(fw);
		kfree(fw_filename as *const c_void);
		return -EINVAL;
	}

	if !ipc_type_to_adjust.is_null() {
		*ipc_type_to_adjust = fw_ipc_type;
	} else if fw_ipc_type != (*profile).ipc_type {
		dev_err(
			dev,
			b"ipc type mismatch between %s and expected: %d vs %d\n\0".as_ptr() as *const c_char,
			fw_filename,
			fw_ipc_type,
			(*profile).ipc_type,
		);
		release_firmware(fw);
		kfree(fw_filename as *const c_void);
		return -EINVAL;
	}

	release_firmware(fw);
	kfree(fw_filename as *const c_void);
	return 0;
}

unsafe fn sof_test_topology_file(dev: *mut device, profile: *mut sof_loadable_file_profile) -> c_int {
	let mut fw: *const firmware = ptr::null();
	let tplg_filename: *mut c_char;
	let ret: c_int;

	if (*profile).tplg_path.is_null() || (*profile).tplg_name.is_null() {
		return 0;
	}

	/* Dummy topology does not exist and should not be used */
	if !strstr((*profile).tplg_name, b"dummy\0".as_ptr() as *const c_char).is_null() {
		return 0;
	}

	tplg_filename = kasprintf(
		GFP_KERNEL,
		b"%s/%s\0".as_ptr() as *const c_char,
		(*profile).tplg_path,
		(*profile).tplg_name,
	);
	if tplg_filename.is_null() {
		return -ENOMEM;
	}

	ret = firmware_request_nowarn(&mut fw, tplg_filename, dev);
	if ret == 0 {
		release_firmware(fw);
	} else {
		dev_dbg(
			dev,
			b"Failed to open topology file: %s\n\0".as_ptr() as *const c_char,
			tplg_filename,
		);
	}

	kfree(tplg_filename as *const c_void);

	return ret;
}

unsafe fn sof_platform_uses_generic_loader(sdev: *mut snd_sof_dev) -> bool {
	let load_firmware = (*(*(*(*sdev).pdata).desc).ops).load_firmware;

	load_firmware == Some(snd_sof_load_firmware_raw)
		|| load_firmware == Some(snd_sof_load_firmware_memcpy)
}

unsafe fn sof_file_profile_for_ipc_type(
	sdev: *mut snd_sof_dev,
	mut ipc_type: sof_ipc_type,
	desc: *const sof_dev_desc,
	base_profile: *mut sof_loadable_file_profile,
	out_profile: *mut sof_loadable_file_profile,
) -> c_int {
	let plat_data: *mut snd_sof_pdata = (*sdev).pdata;
	let mut fw_lib_path_allocated = false;
	let dev: *mut device = (*sdev).dev;
	let mut fw_path_allocated = false;
	let mut ret: c_int = 0;

	/* firmware path */
	if !(*base_profile).fw_path.is_null() {
		(*out_profile).fw_path = (*base_profile).fw_path;
	} else if !(*base_profile).fw_path_postfix.is_null() {
		(*out_profile).fw_path = devm_kasprintf(
			dev,
			GFP_KERNEL,
			b"%s/%s\0".as_ptr() as *const c_char,
			(*desc).default_fw_path[ipc_type as usize],
			(*base_profile).fw_path_postfix,
		);
		if (*out_profile).fw_path.is_null() {
			return -ENOMEM;
		}

		fw_path_allocated = true;
	} else {
		(*out_profile).fw_path = (*desc).default_fw_path[ipc_type as usize];
	}

	/* firmware filename */
	if !(*base_profile).fw_name.is_null() {
		(*out_profile).fw_name = (*base_profile).fw_name;
	} else {
		(*out_profile).fw_name = (*desc).default_fw_filename[ipc_type as usize];
	}

	/*
	 * Check the custom firmware path/filename and adjust the ipc_type to
	 * match with the existing file for the remaining path configuration.
	 *
	 * For default path and firmware name do a verification before
	 * continuing further.
	 */
	if ((!(*base_profile).fw_path.is_null() || !(*base_profile).fw_name.is_null())
		&& sof_platform_uses_generic_loader(sdev))
	{
		ret = sof_test_firmware_file(dev, out_profile, &mut ipc_type);
		if ret != 0 {
			return ret;
		}

		if ((*desc).ipc_supported_mask & BIT(ipc_type)) == 0 {
			dev_err(
				dev,
				b"Unsupported IPC type %d needed by %s/%s\n\0".as_ptr() as *const c_char,
				ipc_type,
				(*out_profile).fw_path,
				(*out_profile).fw_name,
			);
			return -EINVAL;
		}
	}

	/* firmware library path */
	if !(*base_profile).fw_lib_path.is_null() {
		(*out_profile).fw_lib_path = (*base_profile).fw_lib_path;
	} else if !(*desc).default_lib_path[ipc_type as usize].is_null() {
		if !(*base_profile).fw_lib_path_postfix.is_null() {
			(*out_profile).fw_lib_path = devm_kasprintf(
				dev,
				GFP_KERNEL,
				b"%s/%s\0".as_ptr() as *const c_char,
				(*desc).default_lib_path[ipc_type as usize],
				(*base_profile).fw_lib_path_postfix,
			);
			if (*out_profile).fw_lib_path.is_null() {
				ret = -ENOMEM;
				goto_out(
					ret,
					dev,
					out_profile,
					fw_path_allocated,
					fw_lib_path_allocated,
				);
				return ret;
			}

			fw_lib_path_allocated = true;
		} else {
			(*out_profile).fw_lib_path = (*desc).default_lib_path[ipc_type as usize];
		}
	}

	if !(*base_profile).fw_path_postfix.is_null() {
		(*out_profile).fw_path_postfix = (*base_profile).fw_path_postfix;
	}

	if !(*base_profile).fw_lib_path_postfix.is_null() {
		(*out_profile).fw_lib_path_postfix = (*base_profile).fw_lib_path_postfix;
	}

	/* topology path */
	if !(*base_profile).tplg_path.is_null() {
		(*out_profile).tplg_path = (*base_profile).tplg_path;
	} else {
		(*out_profile).tplg_path = (*desc).default_tplg_path[ipc_type as usize];
	}

	/* topology name */
	(*out_profile).tplg_name = (*plat_data).tplg_filename;

	(*out_profile).ipc_type = ipc_type;

	/* Test only default firmware file */
	if ((*base_profile).fw_path.is_null() && (*base_profile).fw_name.is_null())
		&& sof_platform_uses_generic_loader(sdev)
	{
		ret = sof_test_firmware_file(dev, out_profile, ptr::null_mut());
	}

	if ret == 0 {
		ret = sof_test_topology_file(dev, out_profile);
	}

	goto_out(ret, dev, out_profile, fw_path_allocated, fw_lib_path_allocated);

	return ret;
}

unsafe fn goto_out(
	ret: c_int,
	dev: *mut device,
	out_profile: *mut sof_loadable_file_profile,
	fw_path_allocated: bool,
	fw_lib_path_allocated: bool,
) {
	if ret != 0 {
		/* Free up path strings created with devm_kasprintf */
		if fw_path_allocated {
			devm_kfree(dev, (*out_profile).fw_path as *const c_void);
		}
		if fw_lib_path_allocated {
			devm_kfree(dev, (*out_profile).fw_lib_path as *const c_void);
		}

		memset(
			out_profile as *mut c_void,
			0,
			mem::size_of::<sof_loadable_file_profile>(),
		);
	}
}

unsafe fn sof_print_missing_firmware_info(
	sdev: *mut snd_sof_dev,
	ipc_type: sof_ipc_type,
	base_profile: *mut sof_loadable_file_profile,
) {
	let plat_data: *mut snd_sof_pdata = (*sdev).pdata;
	let desc: *const sof_dev_desc = (*plat_data).desc;
	let dev: *mut device = (*sdev).dev;
	let ipc_type_count: c_int;
	let mut i: c_int;
	let marker: *const c_char;

	dev_err(
		dev,
		b"SOF firmware and/or topology file not found.\n\0".as_ptr() as *const c_char,
	);
	dev_info(dev, b"Supported default profiles\n\0".as_ptr() as *const c_char);

	if CONFIG_SND_SOC_SOF_ALLOW_FALLBACK_TO_NEWER_IPC_VERSION_ENABLED {
		ipc_type_count = SOF_IPC_TYPE_COUNT - 1;
	} else {
		ipc_type_count = (*base_profile).ipc_type;
	}

	i = 0;
	while i <= ipc_type_count {
		if ((*desc).ipc_supported_mask & BIT(i)) == 0 {
			i += 1;
			continue;
		}

		if i == ipc_type {
			marker = b"Requested\0".as_ptr() as *const c_char;
		} else {
			marker = b"Fallback\0".as_ptr() as *const c_char;
		}

		dev_info(
			dev,
			b"- ipc type %d (%s):\n\0".as_ptr() as *const c_char,
			i,
			marker,
		);
		if !(*base_profile).fw_path_postfix.is_null() {
			dev_info(
				dev,
				b" Firmware file: %s/%s/%s\n\0".as_ptr() as *const c_char,
				(*desc).default_fw_path[i as usize],
				(*base_profile).fw_path_postfix,
				(*desc).default_fw_filename[i as usize],
			);
		} else {
			dev_info(
				dev,
				b" Firmware file: %s/%s\n\0".as_ptr() as *const c_char,
				(*desc).default_fw_path[i as usize],
				(*desc).default_fw_filename[i as usize],
			);
		}

		dev_info(
			dev,
			b" Topology file: %s/%s\n\0".as_ptr() as *const c_char,
			(*desc).default_tplg_path[i as usize],
			(*plat_data).tplg_filename,
		);
		i += 1;
	}

	if !(*base_profile).fw_path.is_null()
		|| !(*base_profile).fw_name.is_null()
		|| !(*base_profile).tplg_path.is_null()
		|| !(*base_profile).tplg_name.is_null()
	{
		dev_info(
			dev,
			b"Verify the path/name override module parameters.\n\0".as_ptr() as *const c_char,
		);
	}

	dev_info(
		dev,
		b"Check if you have 'sof-firmware' package installed.\n\0".as_ptr() as *const c_char,
	);
	dev_info(
		dev,
		b"Optionally it can be manually downloaded from:\n\0".as_ptr() as *const c_char,
	);
	dev_info(
		dev,
		b"   https://github.com/thesofproject/sof-bin/\n\0".as_ptr() as *const c_char,
	);
}

unsafe fn sof_print_profile_info(
	sdev: *mut snd_sof_dev,
	ipc_type: sof_ipc_type,
	profile: *mut sof_loadable_file_profile,
) {
	let plat_data: *mut snd_sof_pdata = (*sdev).pdata;
	let dev: *mut device = (*sdev).dev;

	if ipc_type != (*profile).ipc_type {
		dev_info(
			dev,
			b"Using fallback IPC type %d (requested type was %d)\n\0".as_ptr() as *const c_char,
			(*profile).ipc_type,
			ipc_type,
		);
	}

	dev_info(
		dev,
		b"Firmware paths/files for ipc type %d:\n\0".as_ptr() as *const c_char,
		(*profile).ipc_type,
	);

	/* The firmware path is only valid when generic loader is used */
	if sof_platform_uses_generic_loader(sdev) {
		dev_info(
			dev,
			b" Firmware file:     %s/%s\n\0".as_ptr() as *const c_char,
			(*profile).fw_path,
			(*profile).fw_name,
		);
	}

	if !(*profile).fw_lib_path.is_null() {
		dev_info(
			dev,
			b" Firmware lib path: %s\n\0".as_ptr() as *const c_char,
			(*profile).fw_lib_path,
		);
	}

	if !(*plat_data).machine.is_null()
		&& !(*(*plat_data).machine).get_function_tplg_files.is_null()
		&& !(*plat_data).disable_function_topology
	{
		dev_info(
			dev,
			b" Topology file:     function topologies\n\0".as_ptr() as *const c_char,
		);
	} else {
		dev_info(
			dev,
			b" Topology file:     %s/%s\n\0".as_ptr() as *const c_char,
			(*profile).tplg_path,
			(*profile).tplg_name,
		);
	}
}

#[no_mangle]
pub unsafe extern "C" fn sof_create_ipc_file_profile(
	sdev: *mut snd_sof_dev,
	base_profile: *mut sof_loadable_file_profile,
	out_profile: *mut sof_loadable_file_profile,
) -> c_int {
	let desc: *const sof_dev_desc = (*(*sdev).pdata).desc;
	let ipc_fallback_start: c_int;
	let mut ret: c_int;
	let mut i: c_int;

	memset(
		out_profile as *mut c_void,
		0,
		mem::size_of::<sof_loadable_file_profile>(),
	);

	ret = sof_file_profile_for_ipc_type(
		sdev,
		(*base_profile).ipc_type,
		desc,
		base_profile,
		out_profile,
	);
	if ret == 0 {
		goto_out_create(sdev, base_profile, out_profile, ret);
		return ret;
	}

	/*
	 * No firmware file was found for the requested IPC type, as fallback
	 * if SND_SOC_SOF_ALLOW_FALLBACK_TO_NEWER_IPC_VERSION is selected, check
	 * all IPC versions in a backwards direction (from newer to older)
	 * if SND_SOC_SOF_ALLOW_FALLBACK_TO_NEWER_IPC_VERSION is not selected,
	 * check only older IPC versions than the selected/default version
	 */
	if CONFIG_SND_SOC_SOF_ALLOW_FALLBACK_TO_NEWER_IPC_VERSION_ENABLED {
		ipc_fallback_start = SOF_IPC_TYPE_COUNT - 1;
	} else {
		ipc_fallback_start = (*base_profile).ipc_type as c_int - 1;
	}

	i = ipc_fallback_start;
	while i >= 0 {
		if i == (*base_profile).ipc_type || ((*desc).ipc_supported_mask & BIT(i)) == 0 {
			i -= 1;
			continue;
		}

		ret = sof_file_profile_for_ipc_type(sdev, i, desc, base_profile, out_profile);
		if ret == 0 {
			break;
		}
		i -= 1;
	}

	goto_out_create(sdev, base_profile, out_profile, ret);

	return ret;
}

unsafe fn goto_out_create(
	sdev: *mut snd_sof_dev,
	base_profile: *mut sof_loadable_file_profile,
	out_profile: *mut sof_loadable_file_profile,
	ret: c_int,
) {
	if ret != 0 {
		sof_print_missing_firmware_info(sdev, (*base_profile).ipc_type, base_profile);
	} else {
		sof_print_profile_info(sdev, (*base_profile).ipc_type, out_profile);
	}
}

/* EXPORT_SYMBOL(sof_create_ipc_file_profile); */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
