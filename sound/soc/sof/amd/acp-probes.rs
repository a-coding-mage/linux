// SPDX-License-Identifier: (GPL-2.0-only OR BSD-3-Clause)
//
// This file is provided under a dual BSD/GPLv2 license. When using or
// redistributing this file, you may do so under either license.
//
// Copyright(c) 2023 Advanced Micro Devices, Inc.
//
// Authors: V Sujith Kumar Reddy <Vsujithkumar.Reddy@amd.com>

/*
 * Probe interface for generic AMD audio ACP DSP block
 */

// C dependencies translated from:
// <linux/module.h>, <sound/soc.h>, "../sof-priv.h",
// "../sof-client-probes.h", "../sof-client.h", "../ops.h",
// "acp.h", and "acp-dsp-offset.h".

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::mem::size_of;
use core::ptr;

type u32 = u32;

const ENODEV: c_int = 19;
const ACP_DSP_BAR: c_uint = 0;
const ACP_SCRATCH_REG_0: c_uint = 0;

#[repr(C)]
pub struct sof_client_dev {
	_private: [u8; 0],
}

#[repr(C)]
pub struct device {
	_private: [u8; 0],
}

#[repr(C)]
pub struct snd_dma_buffer {
	_private: [u8; 0],
}

#[repr(C)]
pub struct snd_compr_runtime {
	pub private_data: *mut c_void,
	pub dma_buffer_p: snd_dma_buffer,
	pub dma_bytes: usize,
	pub buffer_size: u32,
}

#[repr(C)]
pub struct snd_compr_stream {
	pub runtime: *mut snd_compr_runtime,
}

#[repr(C)]
pub struct snd_compr_params {
	_private: [u8; 0],
}

#[repr(C)]
pub struct snd_compr_tstamp64 {
	pub copied_total: u64,
	pub sampling_rate: u32,
}

#[repr(C)]
pub struct snd_soc_pcm_stream {
	pub rates: c_uint,
}

#[repr(C)]
pub struct snd_soc_dai_driver {
	pub capture: snd_soc_pcm_stream,
}

#[repr(C)]
pub struct snd_soc_dai {
	pub driver: *mut snd_soc_dai_driver,
}

#[repr(C)]
pub struct snd_sof_pdata {
	pub hw_pdata: *mut acp_dev_data,
}

#[repr(C)]
pub struct snd_sof_debug_box {
	pub offset: c_uint,
}

#[repr(C)]
pub struct snd_sof_dev {
	pub pdata: *mut snd_sof_pdata,
	pub debug_box: snd_sof_debug_box,
	pub dev: *mut device,
}

#[repr(C)]
pub struct acp_dsp_stream {
	pub cstream: *mut snd_compr_stream,
	pub dmab: snd_dma_buffer,
	pub num_pages: c_uint,
	pub stream_tag: u32,
	pub cstream_posn: u64,
}

#[repr(C)]
pub struct acp_dev_data {
	pub probe_stream: *mut acp_dsp_stream,
}

#[repr(C)]
pub struct scratch_reg_conf {
	pub buf_size: u32,
}

#[repr(C)]
pub struct sof_probes_host_ops {
	pub startup: Option<
		unsafe extern "C" fn(
			*mut sof_client_dev,
			*mut snd_compr_stream,
			*mut snd_soc_dai,
			*mut u32,
		) -> c_int,
	>,
	pub shutdown: Option<
		unsafe extern "C" fn(
			*mut sof_client_dev,
			*mut snd_compr_stream,
			*mut snd_soc_dai,
		) -> c_int,
	>,
	pub set_params: Option<
		unsafe extern "C" fn(
			*mut sof_client_dev,
			*mut snd_compr_stream,
			*mut snd_compr_params,
			*mut snd_soc_dai,
		) -> c_int,
	>,
	pub trigger: Option<
		unsafe extern "C" fn(
			*mut sof_client_dev,
			*mut snd_compr_stream,
			c_int,
			*mut snd_soc_dai,
		) -> c_int,
	>,
	pub pointer: Option<
		unsafe extern "C" fn(
			*mut sof_client_dev,
			*mut snd_compr_stream,
			*mut snd_compr_tstamp64,
			*mut snd_soc_dai,
		) -> c_int,
	>,
}

unsafe extern "C" {
	fn sof_client_dev_to_sof_dev(cdev: *mut sof_client_dev) -> *mut snd_sof_dev;
	fn acp_dsp_stream_get(sdev: *mut snd_sof_dev, id: c_int) -> *mut acp_dsp_stream;
	fn acp_dsp_stream_put(sdev: *mut snd_sof_dev, stream: *mut acp_dsp_stream) -> c_int;
	fn acp_dsp_stream_config(sdev: *mut snd_sof_dev, stream: *mut acp_dsp_stream) -> c_int;
	fn snd_sof_dsp_write(sdev: *mut snd_sof_dev, bar: c_uint, offset: c_uint, value: u32);
	fn snd_pcm_rate_bit_to_rate(rate_bit: c_uint) -> u32;
	fn sof_client_dev_register(
		sdev: *mut snd_sof_dev,
		name: *const c_char,
		id: c_int,
		ops: *const sof_probes_host_ops,
		size: usize,
	) -> c_int;
	fn sof_client_dev_unregister(sdev: *mut snd_sof_dev, name: *const c_char, id: c_int);
	fn dev_err(dev: *mut device, fmt: *const c_char, ...);
}

const fn pfn_up(x: usize) -> c_uint {
	((x + 4095) >> 12) as c_uint
}

unsafe extern "C" fn acp_probes_compr_startup(
	cdev: *mut sof_client_dev,
	cstream: *mut snd_compr_stream,
	_dai: *mut snd_soc_dai,
	stream_id: *mut u32,
) -> c_int {
	let sdev: *mut snd_sof_dev = unsafe { sof_client_dev_to_sof_dev(cdev) };
	let stream: *mut acp_dsp_stream;
	let adata: *mut acp_dev_data;

	adata = unsafe { (*(*sdev).pdata).hw_pdata };
	stream = unsafe { acp_dsp_stream_get(sdev, 0) };
	if stream.is_null() {
		return -ENODEV;
	}

	unsafe {
		(*stream).cstream = cstream;
		(*(*cstream).runtime).private_data = stream.cast::<c_void>();

		(*adata).probe_stream = stream;
		*stream_id = (*stream).stream_tag;
	}

	0
}

unsafe extern "C" fn acp_probes_compr_shutdown(
	cdev: *mut sof_client_dev,
	cstream: *mut snd_compr_stream,
	_dai: *mut snd_soc_dai,
) -> c_int {
	let sdev: *mut snd_sof_dev = unsafe { sof_client_dev_to_sof_dev(cdev) };
	let stream: *mut acp_dsp_stream =
		unsafe { (*(*cstream).runtime).private_data.cast::<acp_dsp_stream>() };
	let adata: *mut acp_dev_data;
	let ret: c_int;

	ret = unsafe { acp_dsp_stream_put(sdev, stream) };
	if ret < 0 {
		unsafe {
			dev_err(
				(*sdev).dev,
				c"Failed to release probe compress stream\n".as_ptr(),
			);
		}
		return ret;
	}

	adata = unsafe { (*(*sdev).pdata).hw_pdata };
	unsafe {
		(*stream).cstream = ptr::null_mut();
		(*(*cstream).runtime).private_data = ptr::null_mut();
		(*adata).probe_stream = ptr::null_mut();
	}

	0
}

unsafe extern "C" fn acp_probes_compr_set_params(
	cdev: *mut sof_client_dev,
	cstream: *mut snd_compr_stream,
	_params: *mut snd_compr_params,
	_dai: *mut snd_soc_dai,
) -> c_int {
	let sdev: *mut snd_sof_dev = unsafe { sof_client_dev_to_sof_dev(cdev) };
	let stream: *mut acp_dsp_stream =
		unsafe { (*(*cstream).runtime).private_data.cast::<acp_dsp_stream>() };
	let mut buf_offset: c_uint;
	let index: c_uint;
	let size: u32;
	let ret: c_int;

	unsafe {
		(*stream).dmab = (*(*cstream).runtime).dma_buffer_p;
		(*stream).num_pages = pfn_up((*(*cstream).runtime).dma_bytes);
		size = (*(*cstream).runtime).buffer_size;
	}

	ret = unsafe { acp_dsp_stream_config(sdev, stream) };
	if ret < 0 {
		unsafe {
			acp_dsp_stream_put(sdev, stream);
		}
		return ret;
	}

	/* write buffer size of stream in scratch memory */

	buf_offset = unsafe { (*sdev).debug_box.offset }
		+ core::mem::offset_of!(scratch_reg_conf, buf_size) as c_uint;
	index = unsafe { (*stream).stream_tag - 1 };
	buf_offset = buf_offset + index * 4;

	unsafe {
		snd_sof_dsp_write(
			sdev,
			ACP_DSP_BAR,
			ACP_SCRATCH_REG_0 + buf_offset,
			size,
		);
	}

	0
}

unsafe extern "C" fn acp_probes_compr_trigger(
	_cdev: *mut sof_client_dev,
	_cstream: *mut snd_compr_stream,
	_cmd: c_int,
	_dai: *mut snd_soc_dai,
) -> c_int {
	/* Nothing to do here, as it is a mandatory callback just defined */
	0
}

unsafe extern "C" fn acp_probes_compr_pointer(
	_cdev: *mut sof_client_dev,
	cstream: *mut snd_compr_stream,
	tstamp: *mut snd_compr_tstamp64,
	dai: *mut snd_soc_dai,
) -> c_int {
	let stream: *mut acp_dsp_stream =
		unsafe { (*(*cstream).runtime).private_data.cast::<acp_dsp_stream>() };
	let pstream: *mut snd_soc_pcm_stream;

	pstream = unsafe { &mut (*(*dai).driver).capture };
	unsafe {
		(*tstamp).copied_total = (*stream).cstream_posn;
		(*tstamp).sampling_rate = snd_pcm_rate_bit_to_rate((*pstream).rates);
	}

	0
}

/* SOF client implementation */
static acp_probes_ops: sof_probes_host_ops = sof_probes_host_ops {
	startup: Some(acp_probes_compr_startup),
	shutdown: Some(acp_probes_compr_shutdown),
	set_params: Some(acp_probes_compr_set_params),
	trigger: Some(acp_probes_compr_trigger),
	pointer: Some(acp_probes_compr_pointer),
};

#[unsafe(no_mangle)]
pub unsafe extern "C" fn acp_probes_register(sdev: *mut snd_sof_dev) -> c_int {
	unsafe {
		sof_client_dev_register(
			sdev,
			c"acp-probes".as_ptr(),
			0,
			&acp_probes_ops,
			size_of::<sof_probes_host_ops>(),
		)
	}
}
// EXPORT_SYMBOL_NS(acp_probes_register, "SND_SOC_SOF_AMD_COMMON");

#[unsafe(no_mangle)]
pub unsafe extern "C" fn acp_probes_unregister(sdev: *mut snd_sof_dev) {
	unsafe {
		sof_client_dev_unregister(sdev, c"acp-probes".as_ptr(), 0);
	}
}
// EXPORT_SYMBOL_NS(acp_probes_unregister, "SND_SOC_SOF_AMD_COMMON");

// MODULE_IMPORT_NS("SND_SOC_SOF_CLIENT");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
