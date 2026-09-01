/*
 * SPDX-License-Identifier: GPL-2.0
 *
 * Copyright(C) 2015-2018 Linaro Limited.
 *
 * Author: Tor Jeremiassen <tor@ti.com>
 * Author: Mathieu Poirier <mathieu.poirier@linaro.org>
 */

/* C dependencies: <linux/types.h>, <opencsd/ocsd_if_types.h>, <stdio.h>. */

use core::ffi::{c_char, c_int, c_void};

pub type u8 = ::core::ffi::c_uchar;
pub type u32 = ::core::ffi::c_uint;
pub type u64 = ::core::ffi::c_ulonglong;
pub type size_t = usize;

#[repr(C)]
pub struct cs_etm_decoder {
	_private: [u8; 0],
}

#[repr(C)]
pub struct cs_etm_packet {
	_private: [u8; 0],
}

#[repr(C)]
pub struct cs_etm_packet_queue {
	_private: [u8; 0],
}

#[repr(C)]
pub struct cs_etm_queue {
	_private: [u8; 0],
}

/* External type from <opencsd/ocsd_if_types.h>. */
pub type ocsd_mem_space_acc_t = c_int;

pub type cs_etm_mem_cb_type = Option<
	unsafe extern "C" fn(
		*mut cs_etm_queue,
		u8,
		u64,
		size_t,
		*mut u8,
		ocsd_mem_space_acc_t,
	) -> u32,
>;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cs_etmv3_trace_params {
	pub reg_ctrl: u32,
	pub reg_trc_id: u32,
	pub reg_ccer: u32,
	pub reg_idr: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cs_etmv4_trace_params {
	pub reg_idr0: u32,
	pub reg_idr1: u32,
	pub reg_idr2: u32,
	pub reg_idr8: u32,
	pub reg_configr: u32,
	pub reg_traceidr: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cs_ete_trace_params {
	pub reg_idr0: u32,
	pub reg_idr1: u32,
	pub reg_idr2: u32,
	pub reg_idr8: u32,
	pub reg_configr: u32,
	pub reg_traceidr: u32,
	pub reg_devarch: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union cs_etm_trace_params__bindgen_ty_1 {
	pub etmv3: cs_etmv3_trace_params,
	pub etmv4: cs_etmv4_trace_params,
	pub ete: cs_ete_trace_params,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cs_etm_trace_params {
	pub protocol: c_int,
	pub __bindgen_anon_1: cs_etm_trace_params__bindgen_ty_1,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cs_etm_decoder_params {
	pub operation: c_int,
	pub packet_printer: Option<unsafe extern "C" fn(msg: *const c_char, data: *mut c_void)>,
	pub mem_acc_cb: cs_etm_mem_cb_type,
	pub formatted: bool,
	pub fsyncs: bool,
	pub hsyncs: bool,
	pub frame_aligned: bool,
	pub data: *mut c_void,
}

/*
 * The following enums are indexed starting with 1 to align with the
 * open source coresight trace decoder library.
 */
pub const CS_ETM_PROTO_ETMV3: c_int = 1;
pub const CS_ETM_PROTO_ETMV4i: c_int = 2;
pub const CS_ETM_PROTO_ETMV4d: c_int = 3;
pub const CS_ETM_PROTO_PTM: c_int = 4;
pub const CS_ETM_PROTO_ETE: c_int = 5;

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum cs_etm_decoder_operation {
	CS_ETM_OPERATION_PRINT = 1,
	CS_ETM_OPERATION_DECODE = 2,
	CS_ETM_OPERATION_MAX = 3,
}

unsafe extern "C" {
	pub fn cs_etm_decoder__process_data_block(
		decoder: *mut cs_etm_decoder,
		indx: u64,
		buf: *const u8,
		len: size_t,
		consumed: *mut size_t,
	) -> c_int;

	pub fn cs_etm_decoder__new(
		num_cpu: c_int,
		d_params: *mut cs_etm_decoder_params,
		t_params: *mut cs_etm_trace_params,
	) -> *mut cs_etm_decoder;

	pub fn cs_etm_decoder__free(decoder: *mut cs_etm_decoder);

	pub fn cs_etm_decoder__add_mem_access_cb(
		decoder: *mut cs_etm_decoder,
		start: u64,
		end: u64,
		cb_func: cs_etm_mem_cb_type,
	) -> c_int;

	pub fn cs_etm_decoder__get_packet(
		packet_queue: *mut cs_etm_packet_queue,
		packet: *mut cs_etm_packet,
	) -> c_int;

	pub fn cs_etm_decoder__reset(decoder: *mut cs_etm_decoder) -> c_int;
	pub fn cs_etm_decoder__get_name(decoder: *mut cs_etm_decoder) -> *const c_char;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
