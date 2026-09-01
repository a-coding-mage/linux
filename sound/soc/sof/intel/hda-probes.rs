// SPDX-License-Identifier: (GPL-2.0-only OR BSD-3-Clause)
//
// This file is provided under a dual BSD/GPLv2 license.  When using or
// redistributing this file, you may do so under either license.
//
// Copyright(c) 2019-2021 Intel Corporation
//
// Author: Cezary Rojewski <cezary.rojewski@intel.com>
// Converted to SOF client:
//  Ranjani Sridharan <ranjani.sridharan@linux.intel.com>
//  Peter Ujfalusi <peter.ujfalusi@linux.intel.com>
//

// C dependencies: linux/module.h, sound/hdaudio_ext.h, sound/soc.h,
// ../sof-priv.h, ../sof-client-probes.h, ../sof-client.h, hda.h

use core::ffi::{c_char, c_int, c_void};
use core::mem::size_of;
use core::ptr;

type u32 = u32;

const EBUSY: c_int = 16;
const SNDRV_PCM_FORMAT_S32_LE: c_int = 10;

#[repr(C)]
pub struct device {
	_private: [u8; 0],
}

#[repr(C)]
pub struct sof_client_dev {
	_private: [u8; 0],
}

#[repr(C)]
pub struct snd_sof_dev {
	pub dev: *mut device,
}

#[repr(C)]
pub struct snd_compr_stream {
	pub runtime: *mut snd_compr_runtime,
	pub direction: c_int,
}

#[repr(C)]
pub struct snd_compr_runtime {
	pub private_data: *mut c_void,
	pub dma_buffer_p: *mut snd_dma_buffer,
	pub buffer_size: usize,
	pub fragment_size: usize,
}

#[repr(C)]
pub struct snd_compr_params {
	pub codec: snd_codec,
}

#[repr(C)]
pub struct snd_codec {
	pub sample_rate: u32,
	pub ch_out: u32,
}

#[repr(C)]
pub struct snd_compr_tstamp64 {
	pub copied_total: u64,
	pub sampling_rate: u32,
}

#[repr(C)]
pub struct snd_soc_dai {
	pub driver: *mut snd_soc_dai_driver,
}

#[repr(C)]
pub struct snd_soc_dai_driver {
	pub capture: snd_soc_pcm_stream,
}

#[repr(C)]
pub struct snd_soc_pcm_stream {
	pub rates: u32,
}

#[repr(C)]
pub struct snd_dma_buffer {
	_private: [u8; 0],
}

#[repr(C)]
pub struct hdac_ext_stream {
	_private: [u8; 0],
}

#[repr(C)]
pub struct hdac_stream {
	pub curr_pos: u64,
	pub cstream: *mut snd_compr_stream,
	pub stream_tag: u32,
	pub format_val: u32,
	pub bufsize: usize,
	pub period_bytes: usize,
	pub no_period_wakeup: c_int,
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

extern "C" {
	fn sof_client_dev_to_sof_dev(cdev: *mut sof_client_dev) -> *mut snd_sof_dev;
	fn hda_dsp_stream_get(
		sdev: *mut snd_sof_dev,
		direction: c_int,
		flags: c_int,
	) -> *mut hdac_ext_stream;
	fn hda_dsp_stream_put(
		sdev: *mut snd_sof_dev,
		direction: c_int,
		stream_tag: u32,
	) -> c_int;
	fn hdac_stream(hext_stream: *mut hdac_ext_stream) -> *mut hdac_stream;
	fn dev_dbg(dev: *mut device, fmt: *const c_char, ...);
	fn dev_err(dev: *mut device, fmt: *const c_char, ...);
	fn snd_pcm_format_physical_width(format: c_int) -> c_int;
	fn hda_dsp_get_bits(sdev: *mut snd_sof_dev, bps: c_int) -> u32;
	fn hda_dsp_get_mult_div(sdev: *mut snd_sof_dev, rate: u32) -> u32;
	fn hda_dsp_stream_hw_params(
		sdev: *mut snd_sof_dev,
		hext_stream: *mut hdac_ext_stream,
		dmab: *mut snd_dma_buffer,
		pages: *mut c_void,
	) -> c_int;
	fn hda_dsp_stream_trigger(
		sdev: *mut snd_sof_dev,
		hext_stream: *mut hdac_ext_stream,
		cmd: c_int,
	) -> c_int;
	fn snd_pcm_rate_bit_to_rate(rate_bit: u32) -> u32;
	fn sof_client_dev_register(
		sdev: *mut snd_sof_dev,
		name: *const c_char,
		id: c_int,
		ops: *const sof_probes_host_ops,
		size: usize,
	) -> c_int;
	fn sof_client_dev_unregister(sdev: *mut snd_sof_dev, name: *const c_char, id: c_int);
}

#[inline]
unsafe fn hda_compr_get_stream(cstream: *mut snd_compr_stream) -> *mut hdac_ext_stream {
	(*(*cstream).runtime).private_data as *mut hdac_ext_stream
}

unsafe extern "C" fn hda_probes_compr_startup(
	cdev: *mut sof_client_dev,
	cstream: *mut snd_compr_stream,
	_dai: *mut snd_soc_dai,
	stream_id: *mut u32,
) -> c_int {
	let sdev = sof_client_dev_to_sof_dev(cdev);
	let hext_stream: *mut hdac_ext_stream;

	hext_stream = hda_dsp_stream_get(sdev, (*cstream).direction, 0);
	if hext_stream.is_null() {
		return -EBUSY;
	}

	(*hdac_stream(hext_stream)).curr_pos = 0;
	(*hdac_stream(hext_stream)).cstream = cstream;
	(*(*cstream).runtime).private_data = hext_stream as *mut c_void;

	*stream_id = (*hdac_stream(hext_stream)).stream_tag;

	0
}

unsafe extern "C" fn hda_probes_compr_shutdown(
	cdev: *mut sof_client_dev,
	cstream: *mut snd_compr_stream,
	_dai: *mut snd_soc_dai,
) -> c_int {
	let hext_stream = hda_compr_get_stream(cstream);
	let sdev = sof_client_dev_to_sof_dev(cdev);
	let ret: c_int;

	ret = hda_dsp_stream_put(
		sdev,
		(*cstream).direction,
		(*hdac_stream(hext_stream)).stream_tag,
	);
	if ret < 0 {
		dev_dbg((*sdev).dev, c"stream put failed: %d\n".as_ptr(), ret);
		return ret;
	}

	(*hdac_stream(hext_stream)).cstream = ptr::null_mut();
	(*(*cstream).runtime).private_data = ptr::null_mut();

	0
}

unsafe extern "C" fn hda_probes_compr_set_params(
	cdev: *mut sof_client_dev,
	cstream: *mut snd_compr_stream,
	params: *mut snd_compr_params,
	_dai: *mut snd_soc_dai,
) -> c_int {
	let hext_stream = hda_compr_get_stream(cstream);
	let sdev = sof_client_dev_to_sof_dev(cdev);
	let hstream = hdac_stream(hext_stream);
	let dmab: *mut snd_dma_buffer;
	let bits: u32;
	let rate: u32;
	let bps: c_int;
	let ret: c_int;

	dmab = (*(*cstream).runtime).dma_buffer_p;
	/* compr params do not store bit depth, default to S32_LE */
	bps = snd_pcm_format_physical_width(SNDRV_PCM_FORMAT_S32_LE);
	if bps < 0 {
		return bps;
	}
	bits = hda_dsp_get_bits(sdev, bps);
	rate = hda_dsp_get_mult_div(sdev, (*params).codec.sample_rate);

	(*hstream).format_val = rate | bits | ((*params).codec.ch_out - 1);
	(*hstream).bufsize = (*(*cstream).runtime).buffer_size;
	(*hstream).period_bytes = (*(*cstream).runtime).fragment_size;
	(*hstream).no_period_wakeup = 0;

	ret = hda_dsp_stream_hw_params(sdev, hext_stream, dmab, ptr::null_mut());
	if ret < 0 {
		dev_err((*sdev).dev, c"error: hdac prepare failed: %d\n".as_ptr(), ret);
		return ret;
	}

	0
}

unsafe extern "C" fn hda_probes_compr_trigger(
	cdev: *mut sof_client_dev,
	cstream: *mut snd_compr_stream,
	cmd: c_int,
	_dai: *mut snd_soc_dai,
) -> c_int {
	let hext_stream = hda_compr_get_stream(cstream);
	let sdev = sof_client_dev_to_sof_dev(cdev);

	hda_dsp_stream_trigger(sdev, hext_stream, cmd)
}

unsafe extern "C" fn hda_probes_compr_pointer(
	_cdev: *mut sof_client_dev,
	cstream: *mut snd_compr_stream,
	tstamp: *mut snd_compr_tstamp64,
	dai: *mut snd_soc_dai,
) -> c_int {
	let hext_stream = hda_compr_get_stream(cstream);
	let pstream: *mut snd_soc_pcm_stream;

	pstream = &mut (*(*dai).driver).capture;
	(*tstamp).copied_total = (*hdac_stream(hext_stream)).curr_pos;
	(*tstamp).sampling_rate = snd_pcm_rate_bit_to_rate((*pstream).rates);

	0
}

/* SOF client implementation */
static HDA_PROBES_OPS: sof_probes_host_ops = sof_probes_host_ops {
	startup: Some(hda_probes_compr_startup),
	shutdown: Some(hda_probes_compr_shutdown),
	set_params: Some(hda_probes_compr_set_params),
	trigger: Some(hda_probes_compr_trigger),
	pointer: Some(hda_probes_compr_pointer),
};

#[no_mangle]
pub unsafe extern "C" fn hda_probes_register(sdev: *mut snd_sof_dev) -> c_int {
	sof_client_dev_register(
		sdev,
		c"hda-probes".as_ptr(),
		0,
		&HDA_PROBES_OPS,
		size_of::<sof_probes_host_ops>(),
	)
}

// EXPORT_SYMBOL_NS(hda_probes_register, "SND_SOC_SOF_INTEL_HDA_COMMON");

#[no_mangle]
pub unsafe extern "C" fn hda_probes_unregister(sdev: *mut snd_sof_dev) {
	sof_client_dev_unregister(sdev, c"hda-probes".as_ptr(), 0);
}

// EXPORT_SYMBOL_NS(hda_probes_unregister, "SND_SOC_SOF_INTEL_HDA_COMMON");

// MODULE_IMPORT_NS("SND_SOC_SOF_CLIENT");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
