// SPDX-License-Identifier: GPL-2.0-only
/***********************************************************************

    AudioScience HPI driver
    Functions for reading DSP code using hotplug firmware loader

    Copyright (C) 1997-2014  AudioScience Inc. <support@audioscience.com>


***********************************************************************/
// SOURCEFILE_NAME "hpidspcd.c"
// Dependencies from hpidspcd.h, hpidebug.h, and hpi_version.h are expected
// to be supplied by the surrounding driver translation.

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_short, c_void};
use core::mem::size_of;
use core::ptr;

pub type u32 = u32;
pub type size_t = usize;

#[repr(C)]
pub struct device {
	_private: [u8; 0],
}

#[repr(C)]
pub struct pci_dev {
	pub dev: device,
}

#[repr(C)]
pub struct firmware {
	pub size: size_t,
	pub data: *const u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct code_header {
	pub type_: u32,
	pub adapter: u32,
	pub size: u32,
	pub version: u32,
}

#[repr(C)]
pub struct dsp_code {
	pub pvt: *mut dsp_code_private,
	pub header: code_header,
	pub block_length: size_t,
	pub word_count: size_t,
}

#[repr(C)]
pub struct dsp_code_private {
	/**  Firmware descriptor */
	pub firmware: *const firmware,
	pub dev: *mut pci_dev,
}

unsafe extern "C" {
	static HPI_VER: u32;
	static HPI_ERROR_DSP_FILE_NOT_FOUND: c_short;
	static HPI_ERROR_MEMORY_ALLOC: c_short;
	static HPI_ERROR_DSP_FILE_FORMAT: c_short;
	static DEBUG: c_int;

	fn HPI_VER_MAJOR(version: u32) -> u32;
	fn request_firmware(
		firmware_p: *mut *const firmware,
		name: *const c_char,
		device: *mut device,
	) -> c_int;
	fn release_firmware(firmware: *const firmware);
	fn kmalloc(size: size_t, flags: c_int) -> *mut c_void;
	fn kfree(ptr: *mut c_void);
	fn memcpy(dest: *mut c_void, src: *const c_void, n: size_t) -> *mut c_void;
	fn sprintf(buf: *mut c_char, fmt: *const c_char, ...) -> c_int;
	fn dev_err(dev: *mut device, fmt: *const c_char, ...);
	fn dev_warn(dev: *mut device, fmt: *const c_char, ...);
	fn HPI_DEBUG_LOG(level: c_int, fmt: *const c_char, ...);
}

const GFP_KERNEL: c_int = 0;

/*-------------------------------------------------------------------*/
#[unsafe(no_mangle)]
pub unsafe extern "C" fn hpi_dsp_code_open(
	adapter: u32,
	os_data: *mut c_void,
	dsp_code: *mut dsp_code,
	os_error_code: *mut u32,
) -> c_short {
	let mut firmware: *const firmware = ptr::null();
	let dev: *mut pci_dev = os_data as *mut pci_dev;
	let mut header: code_header = core::mem::zeroed();
	let mut fw_name: [c_char; 20] = [0; 20];
	let mut err_ret: c_short = HPI_ERROR_DSP_FILE_NOT_FOUND;
	let err: c_int;

	let _ = os_error_code;

	sprintf(
		fw_name.as_mut_ptr(),
		c"asihpi/dsp%04x.bin".as_ptr(),
		adapter,
	);

	err = request_firmware(&mut firmware, fw_name.as_ptr(), &mut (*dev).dev);

	if err != 0 || firmware.is_null() {
		dev_err(
			&mut (*dev).dev,
			c"%d, request_firmware failed for %s\n".as_ptr(),
			err,
			fw_name.as_ptr(),
		);
		goto_error(dsp_code, firmware, err_ret)
	} else if (*firmware).size < size_of::<code_header>() {
		dev_err(
			&mut (*dev).dev,
			c"Header size too small %s\n".as_ptr(),
			fw_name.as_ptr(),
		);
		goto_error(dsp_code, firmware, err_ret)
	} else {
		memcpy(
			&mut header as *mut code_header as *mut c_void,
			(*firmware).data as *const c_void,
			size_of::<code_header>(),
		);

		if (header.type_ != 0x45444F43)
			|| (header.adapter != adapter)
			|| ((header.size as size_t) != (*firmware).size)
		{
			dev_err(
				&mut (*dev).dev,
				c"Invalid firmware header size %d != file %zd\n".as_ptr(),
				header.size,
				(*firmware).size,
			);
			goto_error(dsp_code, firmware, err_ret)
		} else if HPI_VER_MAJOR(header.version) != HPI_VER_MAJOR(HPI_VER) {
			/* Major version change probably means Host-DSP protocol change */
			dev_err(
				&mut (*dev).dev,
				c"Incompatible firmware version DSP image %X != Driver %X\n".as_ptr(),
				header.version,
				HPI_VER,
			);
			goto_error(dsp_code, firmware, err_ret)
		} else {
			if header.version != HPI_VER {
				dev_warn(
					&mut (*dev).dev,
					c"Firmware version mismatch: DSP image %X != Driver %X\n".as_ptr(),
					header.version,
					HPI_VER,
				);
			}

			HPI_DEBUG_LOG(DEBUG, c"dsp code %s opened\n".as_ptr(), fw_name.as_ptr());
			(*dsp_code).pvt = kmalloc(size_of::<dsp_code_private>(), GFP_KERNEL)
				as *mut dsp_code_private;
			if (*dsp_code).pvt.is_null() {
				err_ret = HPI_ERROR_MEMORY_ALLOC;
				goto_error(dsp_code, firmware, err_ret)
			} else {
				(*(*dsp_code).pvt).dev = dev;
				(*(*dsp_code).pvt).firmware = firmware;
				(*dsp_code).header = header;
				(*dsp_code).block_length = (header.size as size_t) / size_of::<u32>();
				(*dsp_code).word_count = size_of::<code_header>() / size_of::<u32>();
				0
			}
		}
	}
}

unsafe fn goto_error(
	dsp_code: *mut dsp_code,
	firmware: *const firmware,
	err_ret: c_short,
) -> c_short {
	if !firmware.is_null() {
		release_firmware(firmware);
	}
	(*dsp_code).block_length = 0;
	err_ret
}

/*-------------------------------------------------------------------*/
#[unsafe(no_mangle)]
pub unsafe extern "C" fn hpi_dsp_code_close(dsp_code: *mut dsp_code) {
	HPI_DEBUG_LOG(DEBUG, c"dsp code closed\n".as_ptr());
	release_firmware((*(*dsp_code).pvt).firmware);
	kfree((*dsp_code).pvt as *mut c_void);
}

/*-------------------------------------------------------------------*/
#[unsafe(no_mangle)]
pub unsafe extern "C" fn hpi_dsp_code_rewind(dsp_code: *mut dsp_code) {
	/* Go back to start of  data, after header */
	(*dsp_code).word_count = size_of::<code_header>() / size_of::<u32>();
}

/*-------------------------------------------------------------------*/
#[unsafe(no_mangle)]
pub unsafe extern "C" fn hpi_dsp_code_read_word(
	dsp_code: *mut dsp_code,
	pword: *mut u32,
) -> c_short {
	if (*dsp_code).word_count + 1 > (*dsp_code).block_length {
		return HPI_ERROR_DSP_FILE_FORMAT;
	}

	*pword = *((*(*dsp_code).pvt).firmware.as_ref().unwrap().data as *const u32)
		.add((*dsp_code).word_count);
	(*dsp_code).word_count += 1;
	0
}

/*-------------------------------------------------------------------*/
#[unsafe(no_mangle)]
pub unsafe extern "C" fn hpi_dsp_code_read_block(
	words_requested: size_t,
	dsp_code: *mut dsp_code,
	ppblock: *mut *mut u32,
) -> c_short {
	if (*dsp_code).word_count + words_requested > (*dsp_code).block_length {
		return HPI_ERROR_DSP_FILE_FORMAT;
	}

	*ppblock = ((*(*dsp_code).pvt).firmware.as_ref().unwrap().data as *mut u32)
		.add((*dsp_code).word_count);
	(*dsp_code).word_count += words_requested;
	0
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
