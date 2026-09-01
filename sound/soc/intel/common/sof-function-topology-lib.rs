// SPDX-License-Identifier: (GPL-2.0-only OR BSD-3-Clause)
//
// This file is provided under a dual BSD/GPLv2 license.  When using or
// redistributing this file, you may do so under either license.
//
// Copyright(c) 2025 Intel Corporation.
//

use core::ffi::{c_char, c_int, c_ulong, c_void};

#[repr(C)]
pub struct device {
	_private: [u8; 0],
}

#[repr(C)]
pub struct firmware {
	_private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_acpi_mach_params {
	pub dmic_num: c_int,
}

#[repr(C)]
pub struct snd_soc_acpi_mach {
	pub sof_tplg_filename: *const c_char,
	pub mach_params: snd_soc_acpi_mach_params,
}

#[repr(C)]
pub struct snd_soc_card {
	pub dev: *mut device,
}

#[repr(C)]
pub struct snd_soc_dai_link {
	pub name: *const c_char,
	pub id: c_int,
	pub num_cpus: c_int,
}

#[repr(C)]
enum tplg_device_id {
	TPLG_DEVICE_SDCA_JACK,
	TPLG_DEVICE_SDCA_AMP,
	TPLG_DEVICE_SDCA_MIC,
	TPLG_DEVICE_INTEL_PCH_DMIC,
	TPLG_DEVICE_HDMI,
	TPLG_DEVICE_LOOPBACK_VIRTUAL,
	TPLG_DEVICE_MAX,
}

const fn BIT(nr: c_int) -> c_ulong {
	1_c_ulong << nr
}

const SDCA_DEVICE_MASK: c_ulong = BIT(tplg_device_id::TPLG_DEVICE_SDCA_JACK as c_int)
	| BIT(tplg_device_id::TPLG_DEVICE_SDCA_AMP as c_int)
	| BIT(tplg_device_id::TPLG_DEVICE_SDCA_MIC as c_int);

const SOF_INTEL_PLATFORM_NAME_MAX: usize = 4;
const EINVAL: c_int = 22;
const ENOMEM: c_int = 12;
const GFP_KERNEL: c_uint = 0;

type c_uint = u32;

unsafe extern "C" {
	fn dev_get_platdata(dev: *mut device) -> *mut c_void;
	fn sscanf(s: *const c_char, format: *const c_char, ...) -> c_int;
	fn strstr(haystack: *const c_char, needle: *const c_char) -> *mut c_char;
	fn devm_kasprintf(dev: *mut device, gfp: c_uint, fmt: *const c_char, ...) -> *mut c_char;
	fn firmware_request_nowarn(
		fw: *mut *const firmware,
		name: *const c_char,
		device: *mut device,
	) -> c_int;
	fn release_firmware(fw: *const firmware);
	fn dev_err(dev: *mut device, fmt: *const c_char, ...);
	fn dev_dbg(dev: *mut device, fmt: *const c_char, ...);
	fn dev_warn(dev: *mut device, fmt: *const c_char, ...);
}

/* Supplied by the surrounding kernel/Rust translation environment. */
macro_rules! for_each_card_prelinks {
	($card:expr, $i:ident, $dai_link:ident, $body:block) => {
		compile_error!("for_each_card_prelinks! must be provided by the surrounding translation");
	};
}

#[no_mangle]
pub unsafe extern "C" fn sof_sdw_get_tplg_files(
	card: *mut snd_soc_card,
	mach: *const snd_soc_acpi_mach,
	prefix: *const c_char,
	tplg_files: *mut *mut *const c_char,
	best_effort: bool,
) -> c_int {
	let card_mach: *mut snd_soc_acpi_mach = dev_get_platdata((*card).dev) as *mut snd_soc_acpi_mach;
	/*
	 * Use the acpi mach from the machine driver because the machine driver
	 * may change the dmic_num based on the machine driver quirk.
	 */
	let mach_params: snd_soc_acpi_mach_params = (*card_mach).mach_params;
	let mut dai_link: *mut snd_soc_dai_link;
	let mut fw: *const firmware = core::ptr::null();
	let mut platform: [c_char; SOF_INTEL_PLATFORM_NAME_MAX] = [0; SOF_INTEL_PLATFORM_NAME_MAX];
	let mut tplg_mask: c_ulong = 0;
	let mut tplg_num: c_int = 0;
	let mut tplg_dev: c_int;
	let mut ret: c_int;
	let mut i: c_int;

	ret = sscanf(
		(*mach).sof_tplg_filename,
		b"sof-%3s-*.tplg\0".as_ptr() as *const c_char,
		platform.as_mut_ptr(),
	);
	if ret != 1 {
		dev_err(
			(*card).dev,
			b"Invalid platform name %s of tplg %s\n\0".as_ptr() as *const c_char,
			platform.as_ptr(),
			(*mach).sof_tplg_filename,
		);
		return -EINVAL;
	}

	for_each_card_prelinks!(card, i, dai_link, {
		let mut tplg_dev_name: *mut c_char;

		dev_dbg(
			(*card).dev,
			b"dai_link %s id %d\n\0".as_ptr() as *const c_char,
			(*dai_link).name,
			(*dai_link).id,
		);
		if !strstr(
			(*dai_link).name,
			b"SimpleJack\0".as_ptr() as *const c_char,
		)
		.is_null()
		{
			tplg_dev = tplg_device_id::TPLG_DEVICE_SDCA_JACK as c_int;
			tplg_dev_name = b"sdca-jack\0".as_ptr() as *mut c_char;
		} else if !strstr(
			(*dai_link).name,
			b"SmartAmp\0".as_ptr() as *const c_char,
		)
		.is_null()
		{
			tplg_dev = tplg_device_id::TPLG_DEVICE_SDCA_AMP as c_int;
			tplg_dev_name = devm_kasprintf(
				(*card).dev,
				GFP_KERNEL,
				b"sdca-%damp\0".as_ptr() as *const c_char,
				(*dai_link).num_cpus,
			);
			if tplg_dev_name.is_null() {
				return -ENOMEM;
			}
		} else if !strstr(
			(*dai_link).name,
			b"SmartMic\0".as_ptr() as *const c_char,
		)
		.is_null()
		{
			tplg_dev = tplg_device_id::TPLG_DEVICE_SDCA_MIC as c_int;
			tplg_dev_name = b"sdca-mic\0".as_ptr() as *mut c_char;
		} else if !strstr((*dai_link).name, b"dmic\0".as_ptr() as *const c_char).is_null() {
			match mach_params.dmic_num {
				2 => {
					tplg_dev_name = b"dmic-2ch\0".as_ptr() as *mut c_char;
				}
				4 => {
					tplg_dev_name = b"dmic-4ch\0".as_ptr() as *mut c_char;
				}
				_ => {
					dev_warn(
						(*card).dev,
						b"unsupported number of dmics: %d\n\0".as_ptr() as *const c_char,
						mach_params.dmic_num,
					);
					continue;
				}
			}
			tplg_dev = tplg_device_id::TPLG_DEVICE_INTEL_PCH_DMIC as c_int;
		} else if !strstr((*dai_link).name, b"iDisp\0".as_ptr() as *const c_char).is_null() {
			tplg_dev = tplg_device_id::TPLG_DEVICE_HDMI as c_int;
			tplg_dev_name = b"hdmi-pcm5\0".as_ptr() as *mut c_char;
		} else if !strstr(
			(*dai_link).name,
			b"Loopback_Virtual\0".as_ptr() as *const c_char,
		)
		.is_null()
		{
			tplg_dev = tplg_device_id::TPLG_DEVICE_LOOPBACK_VIRTUAL as c_int;
			/*
			 * Mark the LOOPBACK_VIRTUAL device but no need to create the
			 * LOOPBACK_VIRTUAL topology. Just to avoid the dai_link is not supported
			 * error.
			 */
			tplg_mask |= BIT(tplg_dev);
			continue;
		} else {
			/* The dai link is not supported by separated tplg yet */
			dev_dbg(
				(*card).dev,
				b"dai_link %s is not supported by separated tplg yet\n\0".as_ptr()
					as *const c_char,
				(*dai_link).name,
			);
			if best_effort {
				continue;
			}

			return 0;
		}
		if (tplg_mask & BIT(tplg_dev)) != 0 {
			continue;
		}

		tplg_mask |= BIT(tplg_dev);

		/*
		 * The tplg file naming rule is sof-<platform>-<function>-id<BE id number>.tplg
		 * where <platform> is only required for the DMIC function as the nhlt blob
		 * is platform dependent.
		 */
		match tplg_dev {
			x if x == tplg_device_id::TPLG_DEVICE_INTEL_PCH_DMIC as c_int => {
				*(*tplg_files).add(tplg_num as usize) = devm_kasprintf(
					(*card).dev,
					GFP_KERNEL,
					b"%s/sof-%s-%s-id%d.tplg\0".as_ptr() as *const c_char,
					prefix,
					platform.as_ptr(),
					tplg_dev_name,
					(*dai_link).id,
				);
			}
			_ => {
				*(*tplg_files).add(tplg_num as usize) = devm_kasprintf(
					(*card).dev,
					GFP_KERNEL,
					b"%s/sof-%s-id%d.tplg\0".as_ptr() as *const c_char,
					prefix,
					tplg_dev_name,
					(*dai_link).id,
				);
			}
		}
		if (*(*tplg_files).add(tplg_num as usize)).is_null() {
			return -ENOMEM;
		}
		tplg_num += 1;
	});

	dev_dbg(
		(*card).dev,
		b"tplg_mask %#lx tplg_num %d\n\0".as_ptr() as *const c_char,
		tplg_mask,
		tplg_num,
	);

	/* Check presence of sub-topologies */
	i = 0;
	while i < tplg_num {
		ret = firmware_request_nowarn(
			&mut fw,
			*(*tplg_files).add(i as usize),
			(*card).dev,
		);
		if ret == 0 {
			release_firmware(fw);
		} else {
			dev_warn(
				(*card).dev,
				b"Failed to open topology file: %s, you might need to\n\0".as_ptr()
					as *const c_char,
				*(*tplg_files).add(i as usize),
			);
			dev_warn(
				(*card).dev,
				b"download it from https://github.com/thesofproject/sof-bin/\n\0".as_ptr()
					as *const c_char,
			);
			return 0;
		}
		i += 1;
	}

	tplg_num
}

/* EXPORT_SYMBOL_GPL(sof_sdw_get_tplg_files); */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
