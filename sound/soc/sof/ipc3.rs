// SPDX-License-Identifier: (GPL-2.0-only OR BSD-3-Clause)
//
// This file is provided under a dual BSD/GPLv2 license.  When using or
// redistributing this file, you may do so under either license.
//
// Copyright(c) 2021 Intel Corporation
//
//

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::mem::{size_of, zeroed};
use core::ptr;

type u8 = u8;
type u32 = u32;
type size_t = usize;
type bool_ = bool;

type ipc3_rx_callback = Option<unsafe extern "C" fn(sdev: *mut snd_sof_dev, msg_buf: *mut c_void)>;

#[repr(C)]
pub struct device {
	_private: [u8; 0],
}

#[repr(C)]
pub struct snd_sof_dev {
	pub dev: *mut device,
	pub msg: *mut snd_sof_ipc_msg,
	pub ipc: *mut snd_sof_ipc,
	pub host_box: sof_mailbox,
	pub dsp_box: sof_mailbox,
	pub stream_box: sof_mailbox,
	pub debug_box: sof_mailbox,
	pub ipc_dump_printed: bool,
	pub dbg_dump_printed: bool,
	pub info_window: *mut sof_ipc_window,
	pub cc_version: *mut sof_ipc_cc_version,
	pub first_boot: bool,
	pub fw_ready: sof_ipc_fw_ready,
	pub fw_version: sof_ipc_fw_version,
	pub component: *mut snd_soc_component,
	pub dsp_oops_offset: c_int,
	pub ipc_timeout: c_uint,
	pub boot_wait: wait_queue_head_t,
	pub fw_state: c_int,
	pub enabled_cores_mask: u32,
}

#[repr(C)]
pub struct snd_sof_ipc {
	pub msg: snd_sof_ipc_msg,
	pub sdev: *mut snd_sof_dev,
	pub tx_mutex: mutex,
	pub max_payload_size: size_t,
	pub ops: *const sof_ipc_ops,
}

#[repr(C)]
pub struct snd_sof_ipc_msg {
	pub msg_data: *mut c_void,
	pub reply_data: *mut c_void,
	pub reply_size: size_t,
	pub reply_error: c_int,
	pub ipc_complete: bool,
	pub waitq: wait_queue_head_t,
}

#[repr(C)]
pub struct sof_mailbox {
	pub offset: u32,
	pub size: u32,
}

#[repr(C)]
pub struct sof_ipc_cmd_hdr {
	pub size: u32,
	pub cmd: u32,
}

#[repr(C)]
pub struct sof_ipc_reply {
	pub hdr: sof_ipc_cmd_hdr,
	pub error: c_int,
}

#[repr(C)]
pub struct sof_ipc_ctrl_data {
	pub rhdr: sof_ipc_reply,
	pub type_: u32,
	pub num_elems: u32,
	pub elems_remaining: u32,
	pub msg_index: u32,
	pub chanv: [sof_ipc_ctrl_value_chan; 0],
	pub data: *mut sof_ipc_ctrl_data_data,
}

#[repr(C)]
pub struct sof_ipc_ctrl_value_chan {
	_private: [u8; 0],
}

#[repr(C)]
pub struct sof_ipc_ctrl_data_data {
	pub data: [u8; 0],
}

#[repr(C)]
pub struct sof_abi_hdr {
	_private: [u8; 0],
}

#[repr(C)]
pub struct sof_ipc_ext_data_hdr {
	pub hdr: sof_ipc_cmd_hdr,
	pub type_: u32,
}

#[repr(C)]
pub struct sof_ipc_window {
	pub ext_hdr: sof_ipc_ext_data_hdr,
	pub num_windows: u32,
	pub window: [sof_ipc_window_elem; 0],
}

#[repr(C)]
pub struct sof_ipc_window_elem {
	pub id: u32,
	pub type_: u32,
	pub offset: u32,
	pub size: u32,
}

#[repr(C)]
pub struct sof_ipc_cc_version {
	pub ext_hdr: sof_ipc_ext_data_hdr,
	pub name: *const c_char,
	pub major: u32,
	pub minor: u32,
	pub micro: u32,
	pub desc: *const c_char,
	pub optim: *const c_char,
}

#[repr(C)]
pub struct sof_ipc_fw_ready {
	pub hdr: sof_ipc_cmd_hdr,
	pub version: sof_ipc_fw_version,
	pub flags: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sof_ipc_fw_version {
	pub major: u32,
	pub minor: u32,
	pub micro: u32,
	pub abi_version: u32,
	pub build: u32,
	pub date: *const c_char,
	pub time: *const c_char,
	pub tag: *const c_char,
}

#[repr(C)]
pub struct sof_ipc_stream_posn {
	pub host_posn: u64,
	pub xrun_comp_id: u32,
	pub xrun_size: u32,
}

#[repr(C)]
pub struct snd_soc_component {
	_private: [u8; 0],
}

#[repr(C)]
pub struct snd_sof_pcm {
	pub stream: [snd_sof_pcm_stream; 2],
	pub pcm: snd_sof_pcm_runtime,
}

#[repr(C)]
pub struct snd_sof_pcm_runtime {
	pub compress: bool,
}

#[repr(C)]
pub struct snd_sof_pcm_stream {
	pub posn: sof_ipc_stream_posn,
	pub cstream: *mut c_void,
	pub substream: *mut snd_pcm_substream,
}

#[repr(C)]
pub struct snd_pcm_substream {
	pub runtime: *mut snd_pcm_runtime,
}

#[repr(C)]
pub struct snd_pcm_runtime {
	pub no_period_wakeup: bool,
}

#[repr(C)]
pub struct sof_ipc_pm_core_config {
	pub hdr: sof_ipc_cmd_hdr,
	pub enable_mask: u32,
}

#[repr(C)]
pub struct sof_ipc_pm_ctx {
	pub hdr: sof_ipc_cmd_hdr,
}

#[repr(C)]
pub struct sof_ipc_pm_gate {
	pub hdr: sof_ipc_cmd_hdr,
	pub flags: u32,
}

#[repr(C)]
pub struct sof_dsp_power_state {
	pub state: c_int,
}

#[repr(C)]
pub struct sof_ipc_tplg_ops {
	pub control: *const sof_ipc_tplg_control_ops,
}

#[repr(C)]
pub struct sof_ipc_tplg_control_ops {
	pub update: Option<unsafe extern "C" fn(*mut snd_sof_dev, *mut c_void)>,
}

#[repr(C)]
pub struct sof_ipc_pm_ops {
	pub ctx_save: Option<unsafe extern "C" fn(*mut snd_sof_dev) -> c_int>,
	pub ctx_restore: Option<unsafe extern "C" fn(*mut snd_sof_dev) -> c_int>,
	pub set_core_state: Option<unsafe extern "C" fn(*mut snd_sof_dev, c_int, bool) -> c_int>,
	pub set_pm_gate: Option<unsafe extern "C" fn(*mut snd_sof_dev, u32) -> c_int>,
}

#[repr(C)]
pub struct sof_ipc_ops {
	pub tplg: *const sof_ipc_tplg_ops,
	pub pm: *const sof_ipc_pm_ops,
	pub pcm: *const c_void,
	pub fw_loader: *const c_void,
	pub fw_tracing: *const c_void,
	pub tx_msg: Option<
		unsafe extern "C" fn(
			*mut snd_sof_dev,
			*mut c_void,
			size_t,
			*mut c_void,
			size_t,
			bool,
		) -> c_int,
	>,
	pub rx_msg: Option<unsafe extern "C" fn(*mut snd_sof_dev)>,
	pub set_get_data: Option<unsafe extern "C" fn(*mut snd_sof_dev, *mut c_void, size_t, bool) -> c_int>,
	pub get_reply: Option<unsafe extern "C" fn(*mut snd_sof_dev) -> c_int>,
}

#[repr(C)]
pub struct wait_queue_head_t {
	_private: [u8; 0],
}

#[repr(C)]
pub struct mutex {
	_private: [u8; 0],
}

const EINVAL: c_int = 22;
const ENOMEM: c_int = 12;
const ETIMEDOUT: c_int = 110;
const GFP_KERNEL: c_uint = 0;
const PAGE_SIZE: size_t = 4096;
const DUMP_PREFIX_OFFSET: c_int = 0;
const SOF_DSP_PM_D0: c_int = 0;

extern "C" {
	static ipc3_tplg_ops: sof_ipc_tplg_ops;
	static ipc3_pcm_ops: c_void;
	static ipc3_loader_ops: c_void;
	static ipc3_dtrace_ops: c_void;

	static SOF_GLB_TYPE_MASK: u32;
	static SOF_CMD_TYPE_MASK: u32;
	static SOF_IPC_GLB_REPLY: u32;
	static SOF_IPC_GLB_COMPOUND: u32;
	static SOF_IPC_GLB_TPLG_MSG: u32;
	static SOF_IPC_TPLG_COMP_NEW: u32;
	static SOF_IPC_TPLG_COMP_FREE: u32;
	static SOF_IPC_TPLG_COMP_CONNECT: u32;
	static SOF_IPC_TPLG_PIPE_NEW: u32;
	static SOF_IPC_TPLG_PIPE_FREE: u32;
	static SOF_IPC_TPLG_PIPE_CONNECT: u32;
	static SOF_IPC_TPLG_PIPE_COMPLETE: u32;
	static SOF_IPC_TPLG_BUFFER_NEW: u32;
	static SOF_IPC_TPLG_BUFFER_FREE: u32;
	static SOF_IPC_GLB_PM_MSG: u32;
	static SOF_IPC_PM_CTX_SAVE: u32;
	static SOF_IPC_PM_CTX_RESTORE: u32;
	static SOF_IPC_PM_CTX_SIZE: u32;
	static SOF_IPC_PM_CLK_SET: u32;
	static SOF_IPC_PM_CLK_GET: u32;
	static SOF_IPC_PM_CLK_REQ: u32;
	static SOF_IPC_PM_CORE_ENABLE: u32;
	static SOF_IPC_PM_GATE: u32;
	static SOF_IPC_GLB_COMP_MSG: u32;
	static SOF_IPC_COMP_SET_VALUE: u32;
	static SOF_IPC_COMP_GET_VALUE: u32;
	static SOF_IPC_COMP_SET_DATA: u32;
	static SOF_IPC_COMP_GET_DATA: u32;
	static SOF_IPC_GLB_STREAM_MSG: u32;
	static SOF_IPC_STREAM_PCM_PARAMS: u32;
	static SOF_IPC_STREAM_PCM_PARAMS_REPLY: u32;
	static SOF_IPC_STREAM_PCM_FREE: u32;
	static SOF_IPC_STREAM_TRIG_START: u32;
	static SOF_IPC_STREAM_TRIG_STOP: u32;
	static SOF_IPC_STREAM_TRIG_PAUSE: u32;
	static SOF_IPC_STREAM_TRIG_RELEASE: u32;
	static SOF_IPC_STREAM_TRIG_DRAIN: u32;
	static SOF_IPC_STREAM_TRIG_XRUN: u32;
	static SOF_IPC_STREAM_POSITION: u32;
	static SOF_IPC_STREAM_VORBIS_PARAMS: u32;
	static SOF_IPC_STREAM_VORBIS_FREE: u32;
	static SOF_IPC_FW_READY: u32;
	static SOF_IPC_GLB_DAI_MSG: u32;
	static SOF_IPC_DAI_CONFIG: u32;
	static SOF_IPC_DAI_LOOPBACK: u32;
	static SOF_IPC_GLB_TRACE_MSG: u32;
	static SOF_IPC_TRACE_DMA_PARAMS: u32;
	static SOF_IPC_TRACE_DMA_POSITION: u32;
	static SOF_IPC_TRACE_DMA_PARAMS_EXT: u32;
	static SOF_IPC_TRACE_FILTER_UPDATE: u32;
	static SOF_IPC_TRACE_DMA_FREE: u32;
	static SOF_IPC_GLB_TEST_MSG: u32;
	static SOF_IPC_TEST_IPC_FLOOD: u32;
	static SOF_IPC_GLB_DEBUG: u32;
	static SOF_IPC_DEBUG_MEM_USAGE: u32;
	static SOF_IPC_GLB_PROBE: u32;
	static SOF_IPC_PROBE_INIT: u32;
	static SOF_IPC_PROBE_DEINIT: u32;
	static SOF_IPC_PROBE_DMA_ADD: u32;
	static SOF_IPC_PROBE_DMA_INFO: u32;
	static SOF_IPC_PROBE_DMA_REMOVE: u32;
	static SOF_IPC_PROBE_POINT_ADD: u32;
	static SOF_IPC_PROBE_POINT_INFO: u32;
	static SOF_IPC_PROBE_POINT_REMOVE: u32;
	static SOF_DBG_PRINT_DMA_POSITION_UPDATE_LOGS: u32;
	static SOF_DBG_PRINT_IPC_SUCCESS_LOGS: u32;
	static SOF_DBG_DUMP_IPC_MESSAGE_PAYLOAD: u32;
	static SOF_CTRL_TYPE_VALUE_CHAN_GET: u32;
	static SOF_CTRL_TYPE_VALUE_CHAN_SET: u32;
	static SOF_CTRL_TYPE_DATA_GET: u32;
	static SOF_CTRL_TYPE_DATA_SET: u32;
	static SOF_IPC_MAX_ELEMS: u32;
	static SOF_IPC_EXT_WINDOW: u32;
	static SOF_IPC_EXT_CC_INFO: u32;
	static SOF_IPC_EXT_UNUSED: u32;
	static SOF_IPC_EXT_PROBE_INFO: u32;
	static SOF_IPC_EXT_USER_ABI_INFO: u32;
	static SOF_FW_BLK_TYPE_SRAM: u32;
	static SOF_IPC_REGION_UPBOX: u32;
	static SOF_IPC_REGION_DOWNBOX: u32;
	static SOF_IPC_REGION_TRACE: u32;
	static SOF_IPC_REGION_DEBUG: u32;
	static SOF_IPC_REGION_STREAM: u32;
	static SOF_IPC_REGION_REGS: u32;
	static SOF_IPC_REGION_EXCEPTION: u32;
	static SOF_DEBUGFS_ACCESS_D0_ONLY: u32;
	static SOF_IPC_MSG_MAX_SIZE: size_t;
	static SOF_ABI_VERSION: u32;
	static SOF_ABI_MAJOR: u32;
	static SOF_ABI_MINOR: u32;
	static SOF_ABI_PATCH: u32;
	static SOF_IPC_INFO_BUILD: u32;
	static SOF_IPC_INFO_GDB: u32;
	static SOF_IPC_INFO_LOCKS: u32;
	static SOF_IPC_INFO_LOCKSV: u32;
	static SOF_FW_BOOT_IN_PROGRESS: c_int;
	static SOF_FW_BOOT_READY_FAILED: c_int;
	static SOF_FW_BOOT_READY_OK: c_int;

	fn sof_debug_check_flag(flag: u32) -> bool;
	fn trace_sof_stream_position_ipc_rx(dev: *mut device);
	fn trace_sof_ipc3_period_elapsed_position(sdev: *mut snd_sof_dev, posn: *const sof_ipc_stream_posn);
	fn dev_dbg(dev: *mut device, fmt: *const c_char, ...);
	fn dev_err(dev: *mut device, fmt: *const c_char, ...);
	fn dev_err_ratelimited(dev: *mut device, fmt: *const c_char, ...);
	fn dev_warn(dev: *mut device, fmt: *const c_char, ...);
	fn dev_info(dev: *mut device, fmt: *const c_char, ...);
	fn print_hex_dump_debug(prefix: *const c_char, prefix_type: c_int, rowsize: c_int, groupsize: c_int, buf: *const c_void, len: size_t, ascii: bool);
	fn snd_sof_dsp_mailbox_read(sdev: *mut snd_sof_dev, offset: u32, dest: *mut c_void, size: size_t);
	fn wait_event_timeout(waitq: wait_queue_head_t, condition: bool, timeout: c_uint) -> c_int;
	fn msecs_to_jiffies(msecs: c_uint) -> c_uint;
	fn snd_sof_handle_fw_exception(sdev: *mut snd_sof_dev, msg: *const c_char);
	fn memcpy(dest: *mut c_void, src: *const c_void, n: size_t) -> *mut c_void;
	fn memcmp(s1: *const c_void, s2: *const c_void, n: size_t) -> c_int;
	fn memset(s: *mut c_void, c: c_int, n: size_t) -> *mut c_void;
	fn sof_ipc_send_msg(sdev: *mut snd_sof_dev, msg_data: *mut c_void, msg_bytes: size_t, reply_bytes: size_t) -> c_int;
	fn snd_sof_dsp_set_power_state(sdev: *mut snd_sof_dev, target_state: *const sof_dsp_power_state) -> c_int;
	fn mutex_lock(mutex: *mut mutex);
	fn mutex_unlock(mutex: *mut mutex);
	fn kzalloc(size: size_t, flags: c_uint) -> *mut c_void;
	fn kmalloc(size: size_t, flags: c_uint) -> *mut c_void;
	fn kfree(ptr: *mut c_void);
	fn devm_kmemdup(dev: *mut device, src: *const c_void, len: size_t, flags: c_uint) -> *mut c_void;
	fn devm_kzalloc(dev: *mut device, size: size_t, flags: c_uint) -> *mut c_void;
	fn snd_sof_debugfs_buf_item(sdev: *mut snd_sof_dev, buf: *mut c_void, size: size_t, name: *const c_char, mode: c_uint) -> c_int;
	fn snd_sof_dsp_block_read(sdev: *mut snd_sof_dev, blk_type: u32, offset: u32, dest: *mut c_void, size: size_t) -> c_int;
	fn snd_sof_dsp_get_window_offset(sdev: *mut snd_sof_dev, id: u32) -> c_int;
	fn snd_sof_debugfs_add_region_item(sdev: *mut snd_sof_dev, blk_type: u32, offset: u32, size: u32, name: *const c_char, access: u32);
	fn SOF_ABI_VERSION_MAJOR(version: u32) -> u32;
	fn SOF_ABI_VERSION_MINOR(version: u32) -> u32;
	fn SOF_ABI_VERSION_PATCH(version: u32) -> u32;
	fn SOF_ABI_VERSION_INCOMPATIBLE(kernel: u32, fw: u32) -> bool;
	fn str_enabled_disabled(value: u32) -> *const c_char;
	fn snd_sof_dsp_get_mailbox_offset(sdev: *mut snd_sof_dev) -> c_int;
	fn snd_sof_find_spcm_comp(scomp: *mut snd_soc_component, msg_id: u32, direction: *mut c_int) -> *mut snd_sof_pcm;
	fn snd_sof_ipc_msg_data(sdev: *mut snd_sof_dev, stream: *mut snd_sof_pcm_stream, data: *mut c_void, size: size_t) -> c_int;
	fn snd_sof_compr_fragment_elapsed(cstream: *mut c_void);
	fn snd_sof_pcm_period_elapsed(substream: *mut snd_pcm_substream);
	fn snd_pcm_stop_xrun(substream: *mut snd_pcm_substream);
	fn SOF_IPC_MESSAGE_ID(cmd: u32) -> u32;
	fn ipc3_dtrace_posn_update(sdev: *mut snd_sof_dev, msg_buf: *mut c_void);
	fn sof_set_fw_state(sdev: *mut snd_sof_dev, state: c_int);
	fn wake_up(waitq: *mut wait_queue_head_t);
	fn sof_client_ipc_rx_dispatcher(sdev: *mut snd_sof_dev, msg_buf: *mut c_void);
	fn sof_ipc_tx_message_no_pm_no_reply(ipc: *mut snd_sof_ipc, msg_data: *mut c_void, msg_bytes: size_t) -> c_int;
}

unsafe fn BIT(nr: c_int) -> u32 {
	1u32 << nr
}

unsafe fn DIV_ROUND_UP(n: size_t, d: size_t) -> u32 {
	((n + d - 1) / d) as u32
}

unsafe fn ipc3_log_header(dev: *mut device, text: *const u8, cmd: u32) {
	/* #if IS_ENABLED(CONFIG_SND_SOC_SOF_DEBUG_VERBOSE_IPC) */
	let mut str_: *const u8;
	let mut str2: *const u8 = ptr::null();
	let glb: u32;
	let type_: u32;
	let mut is_sof_ipc_stream_position = false;

	glb = cmd & SOF_GLB_TYPE_MASK;
	type_ = cmd & SOF_CMD_TYPE_MASK;

	if glb == SOF_IPC_GLB_REPLY {
		str_ = b"GLB_REPLY\0".as_ptr();
	} else if glb == SOF_IPC_GLB_COMPOUND {
		str_ = b"GLB_COMPOUND\0".as_ptr();
	} else if glb == SOF_IPC_GLB_TPLG_MSG {
		str_ = b"GLB_TPLG_MSG\0".as_ptr();
		if type_ == SOF_IPC_TPLG_COMP_NEW {
			str2 = b"COMP_NEW\0".as_ptr();
		} else if type_ == SOF_IPC_TPLG_COMP_FREE {
			str2 = b"COMP_FREE\0".as_ptr();
		} else if type_ == SOF_IPC_TPLG_COMP_CONNECT {
			str2 = b"COMP_CONNECT\0".as_ptr();
		} else if type_ == SOF_IPC_TPLG_PIPE_NEW {
			str2 = b"PIPE_NEW\0".as_ptr();
		} else if type_ == SOF_IPC_TPLG_PIPE_FREE {
			str2 = b"PIPE_FREE\0".as_ptr();
		} else if type_ == SOF_IPC_TPLG_PIPE_CONNECT {
			str2 = b"PIPE_CONNECT\0".as_ptr();
		} else if type_ == SOF_IPC_TPLG_PIPE_COMPLETE {
			str2 = b"PIPE_COMPLETE\0".as_ptr();
		} else if type_ == SOF_IPC_TPLG_BUFFER_NEW {
			str2 = b"BUFFER_NEW\0".as_ptr();
		} else if type_ == SOF_IPC_TPLG_BUFFER_FREE {
			str2 = b"BUFFER_FREE\0".as_ptr();
		} else {
			str2 = b"unknown type\0".as_ptr();
		}
	} else if glb == SOF_IPC_GLB_PM_MSG {
		str_ = b"GLB_PM_MSG\0".as_ptr();
		if type_ == SOF_IPC_PM_CTX_SAVE {
			str2 = b"CTX_SAVE\0".as_ptr();
		} else if type_ == SOF_IPC_PM_CTX_RESTORE {
			str2 = b"CTX_RESTORE\0".as_ptr();
		} else if type_ == SOF_IPC_PM_CTX_SIZE {
			str2 = b"CTX_SIZE\0".as_ptr();
		} else if type_ == SOF_IPC_PM_CLK_SET {
			str2 = b"CLK_SET\0".as_ptr();
		} else if type_ == SOF_IPC_PM_CLK_GET {
			str2 = b"CLK_GET\0".as_ptr();
		} else if type_ == SOF_IPC_PM_CLK_REQ {
			str2 = b"CLK_REQ\0".as_ptr();
		} else if type_ == SOF_IPC_PM_CORE_ENABLE {
			str2 = b"CORE_ENABLE\0".as_ptr();
		} else if type_ == SOF_IPC_PM_GATE {
			str2 = b"GATE\0".as_ptr();
		} else {
			str2 = b"unknown type\0".as_ptr();
		}
	} else if glb == SOF_IPC_GLB_COMP_MSG {
		str_ = b"GLB_COMP_MSG\0".as_ptr();
		if type_ == SOF_IPC_COMP_SET_VALUE {
			str2 = b"SET_VALUE\0".as_ptr();
		} else if type_ == SOF_IPC_COMP_GET_VALUE {
			str2 = b"GET_VALUE\0".as_ptr();
		} else if type_ == SOF_IPC_COMP_SET_DATA {
			str2 = b"SET_DATA\0".as_ptr();
		} else if type_ == SOF_IPC_COMP_GET_DATA {
			str2 = b"GET_DATA\0".as_ptr();
		} else {
			str2 = b"unknown type\0".as_ptr();
		}
	} else if glb == SOF_IPC_GLB_STREAM_MSG {
		str_ = b"GLB_STREAM_MSG\0".as_ptr();
		if type_ == SOF_IPC_STREAM_POSITION {
			is_sof_ipc_stream_position = true;
			str2 = b"POSITION\0".as_ptr();
		} else if type_ == SOF_IPC_STREAM_TRIG_XRUN {
			str2 = b"TRIG_XRUN\0".as_ptr();
		} else {
			str2 = b"unknown type\0".as_ptr();
		}
	} else if glb == SOF_IPC_FW_READY {
		str_ = b"FW_READY\0".as_ptr();
	} else if glb == SOF_IPC_GLB_DAI_MSG {
		str_ = b"GLB_DAI_MSG\0".as_ptr();
		if type_ == SOF_IPC_DAI_CONFIG {
			str2 = b"CONFIG\0".as_ptr();
		} else if type_ == SOF_IPC_DAI_LOOPBACK {
			str2 = b"LOOPBACK\0".as_ptr();
		} else {
			str2 = b"unknown type\0".as_ptr();
		}
	} else if glb == SOF_IPC_GLB_TRACE_MSG {
		str_ = b"GLB_TRACE_MSG\0".as_ptr();
		if type_ == SOF_IPC_TRACE_DMA_POSITION {
			if !sof_debug_check_flag(SOF_DBG_PRINT_DMA_POSITION_UPDATE_LOGS) {
				return;
			}
			str2 = b"DMA_POSITION\0".as_ptr();
		} else {
			str2 = b"unknown type\0".as_ptr();
		}
	} else if glb == SOF_IPC_GLB_TEST_MSG {
		str_ = b"GLB_TEST_MSG\0".as_ptr();
		if type_ == SOF_IPC_TEST_IPC_FLOOD {
			str2 = b"IPC_FLOOD\0".as_ptr();
		} else {
			str2 = b"unknown type\0".as_ptr();
		}
	} else if glb == SOF_IPC_GLB_DEBUG {
		str_ = b"GLB_DEBUG\0".as_ptr();
		if type_ == SOF_IPC_DEBUG_MEM_USAGE {
			str2 = b"MEM_USAGE\0".as_ptr();
		} else {
			str2 = b"unknown type\0".as_ptr();
		}
	} else if glb == SOF_IPC_GLB_PROBE {
		str_ = b"GLB_PROBE\0".as_ptr();
		str2 = b"unknown type\0".as_ptr();
	} else {
		str_ = b"unknown GLB command\0".as_ptr();
	}

	if !str2.is_null() {
		if is_sof_ipc_stream_position {
			trace_sof_stream_position_ipc_rx(dev);
		} else {
			dev_dbg(dev, b"%s: 0x%x: %s: %s\n\0".as_ptr() as *const c_char, text, cmd, str_, str2);
		}
	} else {
		dev_dbg(dev, b"%s: 0x%x: %s\n\0".as_ptr() as *const c_char, text, cmd, str_);
	}
}

unsafe fn sof_ipc3_dump_payload(sdev: *mut snd_sof_dev, ipc_data: *mut c_void, size: size_t) {
	dev_dbg((*sdev).dev, b"Size of payload following the header: %zu\n\0".as_ptr() as *const c_char, size);
	print_hex_dump_debug(
		b"Message payload: \0".as_ptr() as *const c_char,
		DUMP_PREFIX_OFFSET,
		16,
		4,
		ipc_data,
		size,
		false,
	);
}

unsafe extern "C" fn sof_ipc3_get_reply(sdev: *mut snd_sof_dev) -> c_int {
	let msg = (*sdev).msg;
	let reply = (*msg).reply_data as *mut sof_ipc_reply;
	let mut ret = 0;

	snd_sof_dsp_mailbox_read(sdev, (*sdev).host_box.offset, reply as *mut c_void, size_of::<sof_ipc_reply>());

	if (*reply).error < 0 {
		return (*reply).error;
	}

	if (*reply).hdr.size == 0 {
		if (*msg).reply_size != 0 {
			dev_err((*sdev).dev, b"empty reply received, expected %zu bytes\n\0".as_ptr() as *const c_char, (*msg).reply_size);
		} else {
			dev_err((*sdev).dev, b"empty reply received\n\0".as_ptr() as *const c_char);
		}
		return -EINVAL;
	}

	if (*msg).reply_size > 0 {
		if (*reply).hdr.size as size_t == (*msg).reply_size {
			ret = 0;
		} else if ((*reply).hdr.size as size_t) < (*msg).reply_size {
			dev_dbg((*sdev).dev, b"reply size (%u) is less than expected (%zu)\n\0".as_ptr() as *const c_char, (*reply).hdr.size, (*msg).reply_size);
			(*msg).reply_size = (*reply).hdr.size as size_t;
			ret = 0;
		} else {
			dev_err((*sdev).dev, b"reply size (%u) exceeds the buffer size (%zu)\n\0".as_ptr() as *const c_char, (*reply).hdr.size, (*msg).reply_size);
			ret = -EINVAL;
		}

		if ret == 0 && (*msg).reply_size > size_of::<sof_ipc_reply>() {
			snd_sof_dsp_mailbox_read(sdev, (*sdev).host_box.offset, (*msg).reply_data, (*msg).reply_size);
		}
	}

	ret
}

unsafe fn ipc3_wait_tx_done(ipc: *mut snd_sof_ipc, reply_data: *mut c_void) -> c_int {
	let msg = &mut (*ipc).msg as *mut snd_sof_ipc_msg;
	let hdr = (*msg).msg_data as *mut sof_ipc_cmd_hdr;
	let sdev = (*ipc).sdev;
	let mut ret: c_int;

	ret = wait_event_timeout((*msg).waitq, (*msg).ipc_complete, msecs_to_jiffies((*sdev).ipc_timeout));

	if ret == 0 {
		dev_err((*sdev).dev, b"ipc tx timed out for %#x (msg/reply size: %d/%zu)\n\0".as_ptr() as *const c_char, (*hdr).cmd, (*hdr).size, (*msg).reply_size);
		snd_sof_handle_fw_exception((*ipc).sdev, b"IPC timeout\0".as_ptr() as *const c_char);
		ret = -ETIMEDOUT;
	} else {
		ret = (*msg).reply_error;
		if ret < 0 {
			dev_err((*sdev).dev, b"ipc tx error for %#x (msg/reply size: %d/%zu): %d\n\0".as_ptr() as *const c_char, (*hdr).cmd, (*hdr).size, (*msg).reply_size, ret);
		} else {
			if sof_debug_check_flag(SOF_DBG_PRINT_IPC_SUCCESS_LOGS) {
				ipc3_log_header((*sdev).dev, b"ipc tx succeeded\0".as_ptr(), (*hdr).cmd);
			}
			if !reply_data.is_null() && (*msg).reply_size != 0 {
				memcpy(reply_data, (*msg).reply_data, (*msg).reply_size);
			}
		}

		if (*sdev).ipc_dump_printed {
			(*sdev).dbg_dump_printed = false;
			(*sdev).ipc_dump_printed = false;
		}
	}

	ret
}

unsafe fn ipc3_tx_msg_unlocked(ipc: *mut snd_sof_ipc, msg_data: *mut c_void, msg_bytes: size_t, reply_data: *mut c_void, reply_bytes: size_t) -> c_int {
	let hdr = msg_data as *mut sof_ipc_cmd_hdr;
	let sdev = (*ipc).sdev;
	let ret: c_int;

	ipc3_log_header((*sdev).dev, b"ipc tx\0".as_ptr(), (*hdr).cmd);
	ret = sof_ipc_send_msg(sdev, msg_data, msg_bytes, reply_bytes);

	if ret != 0 {
		dev_err_ratelimited((*sdev).dev, b"%s: ipc message send for %#x failed: %d\n\0".as_ptr() as *const c_char, b"ipc3_tx_msg_unlocked\0".as_ptr(), (*hdr).cmd, ret);
		return ret;
	}

	ipc3_wait_tx_done(ipc, reply_data)
}

unsafe extern "C" fn sof_ipc3_tx_msg(sdev: *mut snd_sof_dev, msg_data: *mut c_void, msg_bytes: size_t, reply_data: *mut c_void, reply_bytes: size_t, no_pm: bool) -> c_int {
	let ipc = (*sdev).ipc;
	let mut ret: c_int;

	if msg_data.is_null() || msg_bytes < size_of::<sof_ipc_cmd_hdr>() {
		dev_err_ratelimited((*sdev).dev, b"No IPC message to send\n\0".as_ptr() as *const c_char);
		return -EINVAL;
	}

	if !no_pm {
		let target_state = sof_dsp_power_state { state: SOF_DSP_PM_D0 };
		ret = snd_sof_dsp_set_power_state(sdev, &target_state);
		if ret < 0 {
			dev_err((*sdev).dev, b"%s: resuming DSP failed: %d\n\0".as_ptr() as *const c_char, b"sof_ipc3_tx_msg\0".as_ptr(), ret);
			return ret;
		}
	}

	mutex_lock(&mut (*ipc).tx_mutex);
	ret = ipc3_tx_msg_unlocked(ipc, msg_data, msg_bytes, reply_data, reply_bytes);
	mutex_unlock(&mut (*ipc).tx_mutex);

	if sof_debug_check_flag(SOF_DBG_DUMP_IPC_MESSAGE_PAYLOAD) {
		let mut payload_bytes: size_t = 0;
		let mut header_bytes: size_t = 0;
		let mut payload: *mut u8 = ptr::null_mut();

		if msg_bytes > size_of::<sof_ipc_cmd_hdr>() {
			payload = msg_data as *mut u8;
			header_bytes = size_of::<sof_ipc_cmd_hdr>();
			payload_bytes = msg_bytes - header_bytes;
		} else if reply_bytes > size_of::<sof_ipc_reply>() {
			payload = reply_data as *mut u8;
			header_bytes = size_of::<sof_ipc_reply>();
			payload_bytes = reply_bytes - header_bytes;
		}

		if !payload.is_null() {
			payload = payload.add(header_bytes);
			sof_ipc3_dump_payload(sdev, payload as *mut c_void, payload_bytes);
		}
	}

	ret
}

unsafe extern "C" fn sof_ipc3_set_get_data(sdev: *mut snd_sof_dev, data: *mut c_void, data_bytes: size_t, set: bool) -> c_int {
	let mut msg_bytes: size_t;
	let hdr_bytes: size_t;
	let payload_size: size_t;
	let mut send_bytes: size_t;
	let cdata = data as *mut sof_ipc_ctrl_data;
	let mut cdata_chunk: *mut sof_ipc_ctrl_data;
	let ipc = (*sdev).ipc;
	let mut offset: size_t = 0;
	let src: *mut u8;
	let dst: *mut u8;
	let num_msg: u32;
	let mut ret = 0;
	let mut i: c_int;

	if cdata.is_null() || data_bytes < size_of::<sof_ipc_ctrl_data>() {
		return -EINVAL;
	}

	if ((*cdata).rhdr.hdr.cmd & SOF_GLB_TYPE_MASK) != SOF_IPC_GLB_COMP_MSG {
		dev_err((*sdev).dev, b"%s: Not supported message type of %#x\n\0".as_ptr() as *const c_char, b"sof_ipc3_set_get_data\0".as_ptr(), (*cdata).rhdr.hdr.cmd);
		return -EINVAL;
	}

	if ((*cdata).rhdr.hdr.size as size_t) <= (*ipc).max_payload_size {
		return sof_ipc3_tx_msg(sdev, cdata as *mut c_void, (*cdata).rhdr.hdr.size as size_t, cdata as *mut c_void, (*cdata).rhdr.hdr.size as size_t, false);
	}

	cdata_chunk = kzalloc((*ipc).max_payload_size, GFP_KERNEL) as *mut sof_ipc_ctrl_data;
	if cdata_chunk.is_null() {
		return -ENOMEM;
	}

	if (*cdata).type_ == SOF_CTRL_TYPE_VALUE_CHAN_GET || (*cdata).type_ == SOF_CTRL_TYPE_VALUE_CHAN_SET {
		hdr_bytes = size_of::<sof_ipc_ctrl_data>();
		if set {
			src = (*cdata).chanv.as_ptr() as *mut u8;
			dst = (*cdata_chunk).chanv.as_mut_ptr() as *mut u8;
		} else {
			src = (*cdata_chunk).chanv.as_mut_ptr() as *mut u8;
			dst = (*cdata).chanv.as_ptr() as *mut u8;
		}
	} else if (*cdata).type_ == SOF_CTRL_TYPE_DATA_GET || (*cdata).type_ == SOF_CTRL_TYPE_DATA_SET {
		hdr_bytes = size_of::<sof_ipc_ctrl_data>() + size_of::<sof_abi_hdr>();
		if set {
			src = (*(*cdata).data).data.as_ptr() as *mut u8;
			dst = (*(*cdata_chunk).data).data.as_mut_ptr() as *mut u8;
		} else {
			src = (*(*cdata_chunk).data).data.as_mut_ptr() as *mut u8;
			dst = (*(*cdata).data).data.as_ptr() as *mut u8;
		}
	} else {
		kfree(cdata_chunk as *mut c_void);
		return -EINVAL;
	}

	msg_bytes = (*cdata).rhdr.hdr.size as size_t - hdr_bytes;
	payload_size = (*ipc).max_payload_size - hdr_bytes;
	num_msg = DIV_ROUND_UP(msg_bytes, payload_size);

	memcpy(cdata_chunk as *mut c_void, cdata as *const c_void, hdr_bytes);

	mutex_lock(&mut (*ipc).tx_mutex);
	i = 0;
	while (i as u32) < num_msg {
		send_bytes = if msg_bytes < payload_size { msg_bytes } else { payload_size };
		(*cdata_chunk).num_elems = send_bytes as u32;
		(*cdata_chunk).rhdr.hdr.size = (hdr_bytes + send_bytes) as u32;
		(*cdata_chunk).msg_index = i as u32;
		msg_bytes -= send_bytes;
		(*cdata_chunk).elems_remaining = msg_bytes as u32;

		if set {
			memcpy(dst as *mut c_void, src.add(offset) as *const c_void, send_bytes);
		}

		ret = ipc3_tx_msg_unlocked((*sdev).ipc, cdata_chunk as *mut c_void, (*cdata_chunk).rhdr.hdr.size as size_t, cdata_chunk as *mut c_void, (*cdata_chunk).rhdr.hdr.size as size_t);
		if ret < 0 {
			break;
		}

		if !set {
			memcpy(dst.add(offset) as *mut c_void, src as *const c_void, send_bytes);
		}

		offset += payload_size;
		i += 1;
	}
	mutex_unlock(&mut (*ipc).tx_mutex);

	if sof_debug_check_flag(SOF_DBG_DUMP_IPC_MESSAGE_PAYLOAD) {
		let header_bytes = size_of::<sof_ipc_reply>();
		let payload = (cdata as *mut u8).add(header_bytes);
		sof_ipc3_dump_payload(sdev, payload as *mut c_void, data_bytes - header_bytes);
	}

	kfree(cdata_chunk as *mut c_void);
	ret
}

#[no_mangle]
pub unsafe extern "C" fn sof_ipc3_get_ext_windows(sdev: *mut snd_sof_dev, ext_hdr: *const sof_ipc_ext_data_hdr) -> c_int {
	let w = ext_hdr as *const sof_ipc_window;

	if (*w).num_windows == 0 || (*w).num_windows > SOF_IPC_MAX_ELEMS {
		return -EINVAL;
	}

	if !(*sdev).info_window.is_null() {
		if memcmp((*sdev).info_window as *const c_void, w as *const c_void, (*ext_hdr).hdr.size as size_t) != 0 {
			dev_err((*sdev).dev, b"mismatch between window descriptor from extended manifest and mailbox\0".as_ptr() as *const c_char);
			return -EINVAL;
		}
		return 0;
	}

	(*sdev).info_window = devm_kmemdup((*sdev).dev, w as *const c_void, (*ext_hdr).hdr.size as size_t, GFP_KERNEL) as *mut sof_ipc_window;
	if (*sdev).info_window.is_null() {
		return -ENOMEM;
	}

	0
}

#[no_mangle]
pub unsafe extern "C" fn sof_ipc3_get_cc_info(sdev: *mut snd_sof_dev, ext_hdr: *const sof_ipc_ext_data_hdr) -> c_int {
	let ret: c_int;
	let cc = ext_hdr as *const sof_ipc_cc_version;

	if !(*sdev).cc_version.is_null() {
		if memcmp((*sdev).cc_version as *const c_void, cc as *const c_void, (*cc).ext_hdr.hdr.size as size_t) != 0 {
			dev_err((*sdev).dev, b"Receive diverged cc_version descriptions\0".as_ptr() as *const c_char);
			return -EINVAL;
		}
		return 0;
	}

	dev_dbg((*sdev).dev, b"Firmware info: used compiler %s %d:%d:%d%s used optimization flags %s\n\0".as_ptr() as *const c_char, (*cc).name, (*cc).major, (*cc).minor, (*cc).micro, (*cc).desc, (*cc).optim);

	if (*sdev).first_boot {
		(*sdev).cc_version = devm_kmemdup((*sdev).dev, cc as *const c_void, (*cc).ext_hdr.hdr.size as size_t, GFP_KERNEL) as *mut sof_ipc_cc_version;
		if (*sdev).cc_version.is_null() {
			return -ENOMEM;
		}

		ret = snd_sof_debugfs_buf_item(sdev, (*sdev).cc_version as *mut c_void, (*cc).ext_hdr.hdr.size as size_t, b"cc_version\0".as_ptr() as *const c_char, 0o444);
		if ret < 0 {
			dev_err((*sdev).dev, b"snd_sof_debugfs_buf_item failed\n\0".as_ptr() as *const c_char);
			return ret;
		}
	}

	0
}

unsafe fn ipc3_fw_parse_ext_data(sdev: *mut snd_sof_dev, mut offset: u32) -> c_int {
	let mut ext_hdr: *mut sof_ipc_ext_data_hdr;
	let ext_data: *mut c_void;
	let mut ret = 0;

	ext_data = kzalloc(PAGE_SIZE, GFP_KERNEL);
	if ext_data.is_null() {
		return -ENOMEM;
	}

	snd_sof_dsp_block_read(sdev, SOF_FW_BLK_TYPE_SRAM, offset, ext_data, size_of::<sof_ipc_ext_data_hdr>());
	ext_hdr = ext_data as *mut sof_ipc_ext_data_hdr;

	while (*ext_hdr).hdr.cmd == SOF_IPC_FW_READY {
		snd_sof_dsp_block_read(
			sdev,
			SOF_FW_BLK_TYPE_SRAM,
			offset + size_of::<sof_ipc_ext_data_hdr>() as u32,
			(ext_data as *mut u8).add(size_of::<sof_ipc_ext_data_hdr>()) as *mut c_void,
			(*ext_hdr).hdr.size as size_t - size_of::<sof_ipc_ext_data_hdr>(),
		);

		dev_dbg((*sdev).dev, b"found ext header type %d size 0x%x\n\0".as_ptr() as *const c_char, (*ext_hdr).type_, (*ext_hdr).hdr.size);

		if (*ext_hdr).type_ == SOF_IPC_EXT_WINDOW {
			ret = sof_ipc3_get_ext_windows(sdev, ext_hdr);
		} else if (*ext_hdr).type_ == SOF_IPC_EXT_CC_INFO {
			ret = sof_ipc3_get_cc_info(sdev, ext_hdr);
		} else if (*ext_hdr).type_ == SOF_IPC_EXT_UNUSED || (*ext_hdr).type_ == SOF_IPC_EXT_PROBE_INFO || (*ext_hdr).type_ == SOF_IPC_EXT_USER_ABI_INFO {
		} else {
			dev_info((*sdev).dev, b"unknown ext header type %d size 0x%x\n\0".as_ptr() as *const c_char, (*ext_hdr).type_, (*ext_hdr).hdr.size);
			ret = 0;
		}

		if ret < 0 {
			dev_err((*sdev).dev, b"Failed to parse ext data type %d\n\0".as_ptr() as *const c_char, (*ext_hdr).type_);
			break;
		}

		offset += (*ext_hdr).hdr.size;
		snd_sof_dsp_block_read(sdev, SOF_FW_BLK_TYPE_SRAM, offset, ext_data, size_of::<sof_ipc_ext_data_hdr>());
		ext_hdr = ext_data as *mut sof_ipc_ext_data_hdr;
	}

	kfree(ext_data);
	ret
}

unsafe fn ipc3_get_windows(sdev: *mut snd_sof_dev) {
	let mut elem: *mut sof_ipc_window_elem;
	let mut outbox_offset: u32 = 0;
	let mut stream_offset: u32 = 0;
	let mut inbox_offset: u32 = 0;
	let mut outbox_size: u32 = 0;
	let mut stream_size: u32 = 0;
	let mut inbox_size: u32 = 0;
	let mut debug_size: u32 = 0;
	let mut debug_offset: u32 = 0;
	let mut window_offset: c_int;
	let mut i: c_int;

	if (*sdev).info_window.is_null() {
		dev_err((*sdev).dev, b"%s: No window info present\n\0".as_ptr() as *const c_char, b"ipc3_get_windows\0".as_ptr());
		return;
	}

	i = 0;
	while (i as u32) < (*(*sdev).info_window).num_windows {
		elem = (*(*sdev).info_window).window.as_mut_ptr().add(i as usize);
		window_offset = snd_sof_dsp_get_window_offset(sdev, (*elem).id);
		if window_offset < 0 {
			dev_warn((*sdev).dev, b"No offset for window %d\n\0".as_ptr() as *const c_char, (*elem).id);
			i += 1;
			continue;
		}

		if (*elem).type_ == SOF_IPC_REGION_UPBOX {
			inbox_offset = window_offset as u32 + (*elem).offset;
			inbox_size = (*elem).size;
			snd_sof_debugfs_add_region_item(sdev, SOF_FW_BLK_TYPE_SRAM, inbox_offset, (*elem).size, b"inbox\0".as_ptr() as *const c_char, SOF_DEBUGFS_ACCESS_D0_ONLY);
		} else if (*elem).type_ == SOF_IPC_REGION_DOWNBOX {
			outbox_offset = window_offset as u32 + (*elem).offset;
			outbox_size = (*elem).size;
			snd_sof_debugfs_add_region_item(sdev, SOF_FW_BLK_TYPE_SRAM, outbox_offset, (*elem).size, b"outbox\0".as_ptr() as *const c_char, SOF_DEBUGFS_ACCESS_D0_ONLY);
		} else if (*elem).type_ == SOF_IPC_REGION_TRACE {
			snd_sof_debugfs_add_region_item(sdev, SOF_FW_BLK_TYPE_SRAM, window_offset as u32 + (*elem).offset, (*elem).size, b"etrace\0".as_ptr() as *const c_char, SOF_DEBUGFS_ACCESS_D0_ONLY);
		} else if (*elem).type_ == SOF_IPC_REGION_DEBUG {
			debug_offset = window_offset as u32 + (*elem).offset;
			debug_size = (*elem).size;
			snd_sof_debugfs_add_region_item(sdev, SOF_FW_BLK_TYPE_SRAM, window_offset as u32 + (*elem).offset, (*elem).size, b"debug\0".as_ptr() as *const c_char, SOF_DEBUGFS_ACCESS_D0_ONLY);
		} else if (*elem).type_ == SOF_IPC_REGION_STREAM {
			stream_offset = window_offset as u32 + (*elem).offset;
			stream_size = (*elem).size;
			snd_sof_debugfs_add_region_item(sdev, SOF_FW_BLK_TYPE_SRAM, stream_offset, (*elem).size, b"stream\0".as_ptr() as *const c_char, SOF_DEBUGFS_ACCESS_D0_ONLY);
		} else if (*elem).type_ == SOF_IPC_REGION_REGS {
			snd_sof_debugfs_add_region_item(sdev, SOF_FW_BLK_TYPE_SRAM, window_offset as u32 + (*elem).offset, (*elem).size, b"regs\0".as_ptr() as *const c_char, SOF_DEBUGFS_ACCESS_D0_ONLY);
		} else if (*elem).type_ == SOF_IPC_REGION_EXCEPTION {
			(*sdev).dsp_oops_offset = window_offset + (*elem).offset as c_int;
			snd_sof_debugfs_add_region_item(sdev, SOF_FW_BLK_TYPE_SRAM, window_offset as u32 + (*elem).offset, (*elem).size, b"exception\0".as_ptr() as *const c_char, SOF_DEBUGFS_ACCESS_D0_ONLY);
		} else {
			dev_err((*sdev).dev, b"%s: Illegal window info: %u\n\0".as_ptr() as *const c_char, b"ipc3_get_windows\0".as_ptr(), (*elem).type_);
			return;
		}
		i += 1;
	}

	if outbox_size == 0 || inbox_size == 0 {
		dev_err((*sdev).dev, b"%s: Illegal mailbox window\n\0".as_ptr() as *const c_char, b"ipc3_get_windows\0".as_ptr());
		return;
	}

	(*sdev).dsp_box.offset = inbox_offset;
	(*sdev).dsp_box.size = inbox_size;
	(*sdev).host_box.offset = outbox_offset;
	(*sdev).host_box.size = outbox_size;
	(*sdev).stream_box.offset = stream_offset;
	(*sdev).stream_box.size = stream_size;
	(*sdev).debug_box.offset = debug_offset;
	(*sdev).debug_box.size = debug_size;

	dev_dbg((*sdev).dev, b" mailbox upstream 0x%x - size 0x%x\n\0".as_ptr() as *const c_char, inbox_offset, inbox_size);
	dev_dbg((*sdev).dev, b" mailbox downstream 0x%x - size 0x%x\n\0".as_ptr() as *const c_char, outbox_offset, outbox_size);
	dev_dbg((*sdev).dev, b" stream region 0x%x - size 0x%x\n\0".as_ptr() as *const c_char, stream_offset, stream_size);
	dev_dbg((*sdev).dev, b" debug region 0x%x - size 0x%x\n\0".as_ptr() as *const c_char, debug_offset, debug_size);
}

unsafe fn ipc3_init_reply_data_buffer(sdev: *mut snd_sof_dev) -> c_int {
	let msg = &mut (*(*sdev).ipc).msg as *mut snd_sof_ipc_msg;
	(*msg).reply_data = devm_kzalloc((*sdev).dev, SOF_IPC_MSG_MAX_SIZE, GFP_KERNEL);
	if (*msg).reply_data.is_null() {
		return -ENOMEM;
	}
	(*(*sdev).ipc).max_payload_size = SOF_IPC_MSG_MAX_SIZE;
	0
}

#[no_mangle]
pub unsafe extern "C" fn sof_ipc3_validate_fw_version(sdev: *mut snd_sof_dev) -> c_int {
	let ready = &mut (*sdev).fw_ready as *mut sof_ipc_fw_ready;
	let v = &mut (*ready).version as *mut sof_ipc_fw_version;

	dev_info((*sdev).dev, b"Firmware info: version %d:%d:%d-%s\n\0".as_ptr() as *const c_char, (*v).major, (*v).minor, (*v).micro, (*v).tag);
	dev_info((*sdev).dev, b"Firmware: ABI %d:%d:%d Kernel ABI %d:%d:%d\n\0".as_ptr() as *const c_char, SOF_ABI_VERSION_MAJOR((*v).abi_version), SOF_ABI_VERSION_MINOR((*v).abi_version), SOF_ABI_VERSION_PATCH((*v).abi_version), SOF_ABI_MAJOR, SOF_ABI_MINOR, SOF_ABI_PATCH);

	if SOF_ABI_VERSION_INCOMPATIBLE(SOF_ABI_VERSION, (*v).abi_version) {
		dev_err((*sdev).dev, b"incompatible FW ABI version\n\0".as_ptr() as *const c_char);
		return -EINVAL;
	}

	/* IS_ENABLED(CONFIG_SND_SOC_SOF_STRICT_ABI_CHECKS) condition preserved as dependency intent. */

	if ((*ready).flags & SOF_IPC_INFO_BUILD) != 0 {
		dev_info((*sdev).dev, b"Firmware debug build %d on %s-%s - options:\n GDB: %s\n lock debug: %s\n lock vdebug: %s\n\0".as_ptr() as *const c_char, (*v).build, (*v).date, (*v).time, str_enabled_disabled((*ready).flags & SOF_IPC_INFO_GDB), str_enabled_disabled((*ready).flags & SOF_IPC_INFO_LOCKS), str_enabled_disabled((*ready).flags & SOF_IPC_INFO_LOCKSV));
	}

	(*sdev).fw_version = *v;
	0
}

unsafe fn ipc3_fw_ready(sdev: *mut snd_sof_dev, cmd: u32) -> c_int {
	let fw_ready = &mut (*sdev).fw_ready as *mut sof_ipc_fw_ready;
	let offset: c_int;
	let mut ret: c_int;

	offset = snd_sof_dsp_get_mailbox_offset(sdev);
	if offset < 0 {
		dev_err((*sdev).dev, b"%s: no mailbox offset\n\0".as_ptr() as *const c_char, b"ipc3_fw_ready\0".as_ptr());
		return offset;
	}

	dev_dbg((*sdev).dev, b"DSP is ready 0x%8.8x offset 0x%x\n\0".as_ptr() as *const c_char, cmd, offset);

	if !(*sdev).first_boot {
		return 0;
	}

	ret = snd_sof_dsp_block_read(sdev, SOF_FW_BLK_TYPE_SRAM, offset as u32, fw_ready as *mut c_void, size_of::<sof_ipc_fw_ready>());
	if ret != 0 {
		dev_err((*sdev).dev, b"Unable to read fw_ready, read from TYPE_SRAM failed\n\0".as_ptr() as *const c_char);
		return ret;
	}

	ret = sof_ipc3_validate_fw_version(sdev);
	if ret < 0 {
		return ret;
	}

	ipc3_fw_parse_ext_data(sdev, offset as u32 + size_of::<sof_ipc_fw_ready>() as u32);
	ipc3_get_windows(sdev);
	ipc3_init_reply_data_buffer(sdev)
}

unsafe fn ipc3_period_elapsed(sdev: *mut snd_sof_dev, msg_id: u32) {
	let scomp = (*sdev).component;
	let mut direction: c_int = 0;
	let mut posn: sof_ipc_stream_posn = zeroed();
	let spcm = snd_sof_find_spcm_comp(scomp, msg_id, &mut direction);
	let stream: *mut snd_sof_pcm_stream;
	let ret: c_int;

	if spcm.is_null() {
		dev_err((*sdev).dev, b"period elapsed for unknown stream, msg_id %d\n\0".as_ptr() as *const c_char, msg_id);
		return;
	}

	stream = &mut (*spcm).stream[direction as usize];
	ret = snd_sof_ipc_msg_data(sdev, stream, &mut posn as *mut _ as *mut c_void, size_of::<sof_ipc_stream_posn>());
	if ret < 0 {
		dev_warn((*sdev).dev, b"failed to read stream position: %d\n\0".as_ptr() as *const c_char, ret);
		return;
	}

	trace_sof_ipc3_period_elapsed_position(sdev, &posn);
	memcpy(&mut (*stream).posn as *mut _ as *mut c_void, &posn as *const _ as *const c_void, size_of::<sof_ipc_stream_posn>());

	if (*spcm).pcm.compress {
		snd_sof_compr_fragment_elapsed((*stream).cstream);
	} else if !(*stream).substream.is_null() && !(*(*stream).substream).runtime.is_null() && !(*(*(*stream).substream).runtime).no_period_wakeup {
		snd_sof_pcm_period_elapsed((*stream).substream);
	}
}

unsafe fn ipc3_xrun(sdev: *mut snd_sof_dev, msg_id: u32) {
	let scomp = (*sdev).component;
	let mut direction: c_int = 0;
	let mut posn: sof_ipc_stream_posn = zeroed();
	let spcm = snd_sof_find_spcm_comp(scomp, msg_id, &mut direction);
	let stream: *mut snd_sof_pcm_stream;
	let ret: c_int;

	if spcm.is_null() {
		dev_err((*sdev).dev, b"XRUN for unknown stream, msg_id %d\n\0".as_ptr() as *const c_char, msg_id);
		return;
	}

	stream = &mut (*spcm).stream[direction as usize];
	ret = snd_sof_ipc_msg_data(sdev, stream, &mut posn as *mut _ as *mut c_void, size_of::<sof_ipc_stream_posn>());
	if ret < 0 {
		dev_warn((*sdev).dev, b"failed to read overrun position: %d\n\0".as_ptr() as *const c_char, ret);
		return;
	}

	dev_dbg((*sdev).dev, b"posn XRUN: host %llx comp %d size %d\n\0".as_ptr() as *const c_char, posn.host_posn, posn.xrun_comp_id, posn.xrun_size);

	/* #if defined(CONFIG_SND_SOC_SOF_DEBUG_XRUN_STOP)
	 * stop PCM on XRUN - used for pipeline debug
	 */
}

unsafe extern "C" fn ipc3_stream_message(sdev: *mut snd_sof_dev, msg_buf: *mut c_void) {
	let hdr = msg_buf as *mut sof_ipc_cmd_hdr;
	let msg_type = (*hdr).cmd & SOF_CMD_TYPE_MASK;
	let msg_id = SOF_IPC_MESSAGE_ID((*hdr).cmd);

	if msg_type == SOF_IPC_STREAM_POSITION {
		ipc3_period_elapsed(sdev, msg_id);
	} else if msg_type == SOF_IPC_STREAM_TRIG_XRUN {
		ipc3_xrun(sdev, msg_id);
	} else {
		dev_err((*sdev).dev, b"unhandled stream message %#x\n\0".as_ptr() as *const c_char, msg_id);
	}
}

unsafe extern "C" fn ipc3_comp_notification(sdev: *mut snd_sof_dev, msg_buf: *mut c_void) {
	let tplg_ops = (*(*(*sdev).ipc).ops).tplg;
	let hdr = msg_buf as *mut sof_ipc_cmd_hdr;
	let msg_type = (*hdr).cmd & SOF_CMD_TYPE_MASK;

	if msg_type == SOF_IPC_COMP_GET_VALUE || msg_type == SOF_IPC_COMP_GET_DATA {
	} else {
		dev_err((*sdev).dev, b"unhandled component message %#x\n\0".as_ptr() as *const c_char, msg_type);
		return;
	}

	if !(*tplg_ops).control.is_null() {
		if let Some(update) = (*(*tplg_ops).control).update {
			update(sdev, msg_buf);
		}
	}
}

unsafe extern "C" fn ipc3_trace_message(sdev: *mut snd_sof_dev, msg_buf: *mut c_void) {
	let hdr = msg_buf as *mut sof_ipc_cmd_hdr;
	let msg_type = (*hdr).cmd & SOF_CMD_TYPE_MASK;

	if msg_type == SOF_IPC_TRACE_DMA_POSITION {
		ipc3_dtrace_posn_update(sdev, msg_buf);
	} else {
		dev_err((*sdev).dev, b"unhandled trace message %#x\n\0".as_ptr() as *const c_char, msg_type);
	}
}

#[no_mangle]
pub unsafe extern "C" fn sof_ipc3_do_rx_work(sdev: *mut snd_sof_dev, hdr: *mut sof_ipc_cmd_hdr, msg_buf: *mut c_void) {
	let mut rx_callback: ipc3_rx_callback = None;
	let cmd: u32;
	let mut err: c_int;

	ipc3_log_header((*sdev).dev, b"ipc rx\0".as_ptr(), (*hdr).cmd);

	if (*hdr).size < size_of::<sof_ipc_cmd_hdr>() as u32 || ((*hdr).size as size_t) > SOF_IPC_MSG_MAX_SIZE {
		dev_err((*sdev).dev, b"The received message size is invalid: %u\n\0".as_ptr() as *const c_char, (*hdr).size);
		return;
	}

	cmd = (*hdr).cmd & SOF_GLB_TYPE_MASK;

	if cmd == SOF_IPC_GLB_REPLY {
		dev_err((*sdev).dev, b"ipc reply unknown\n\0".as_ptr() as *const c_char);
	} else if cmd == SOF_IPC_FW_READY {
		if (*sdev).fw_state == SOF_FW_BOOT_IN_PROGRESS {
			err = ipc3_fw_ready(sdev, cmd);
			if err < 0 {
				sof_set_fw_state(sdev, SOF_FW_BOOT_READY_FAILED);
			} else {
				sof_set_fw_state(sdev, SOF_FW_BOOT_READY_OK);
			}
			wake_up(&mut (*sdev).boot_wait);
		}
	} else if cmd == SOF_IPC_GLB_COMPOUND || cmd == SOF_IPC_GLB_TPLG_MSG || cmd == SOF_IPC_GLB_PM_MSG {
	} else if cmd == SOF_IPC_GLB_COMP_MSG {
		rx_callback = Some(ipc3_comp_notification);
	} else if cmd == SOF_IPC_GLB_STREAM_MSG {
		rx_callback = Some(ipc3_stream_message);
	} else if cmd == SOF_IPC_GLB_TRACE_MSG {
		rx_callback = Some(ipc3_trace_message);
	} else {
		dev_err((*sdev).dev, b"%s: Unknown DSP message: 0x%x\n\0".as_ptr() as *const c_char, b"sof_ipc3_do_rx_work\0".as_ptr(), cmd);
	}

	if let Some(callback) = rx_callback {
		callback(sdev, msg_buf);
	}

	sof_client_ipc_rx_dispatcher(sdev, msg_buf);
	ipc3_log_header((*sdev).dev, b"ipc rx done\0".as_ptr(), (*hdr).cmd);
}

unsafe extern "C" fn sof_ipc3_rx_msg(sdev: *mut snd_sof_dev) {
	let mut hdr: sof_ipc_cmd_hdr = zeroed();
	let msg_buf: *mut c_void;
	let mut err: c_int;

	err = snd_sof_ipc_msg_data(sdev, ptr::null_mut(), &mut hdr as *mut _ as *mut c_void, size_of::<sof_ipc_cmd_hdr>());
	if err < 0 {
		dev_warn((*sdev).dev, b"failed to read IPC header: %d\n\0".as_ptr() as *const c_char, err);
		return;
	}

	if hdr.size < size_of::<sof_ipc_cmd_hdr>() as u32 || (hdr.size as size_t) > SOF_IPC_MSG_MAX_SIZE {
		dev_err((*sdev).dev, b"The received message size is invalid\n\0".as_ptr() as *const c_char);
		return;
	}

	msg_buf = kmalloc(hdr.size as size_t, GFP_KERNEL);
	if msg_buf.is_null() {
		return;
	}

	err = snd_sof_ipc_msg_data(sdev, ptr::null_mut(), msg_buf, hdr.size as size_t);
	if err < 0 {
		dev_err((*sdev).dev, b"%s: Failed to read message: %d\n\0".as_ptr() as *const c_char, b"sof_ipc3_rx_msg\0".as_ptr(), err);
		kfree(msg_buf);
		return;
	}

	sof_ipc3_do_rx_work(sdev, &mut hdr, msg_buf);
	kfree(msg_buf);
}

unsafe extern "C" fn sof_ipc3_set_core_state(sdev: *mut snd_sof_dev, core_idx: c_int, on: bool) -> c_int {
	let mut core_cfg = sof_ipc_pm_core_config {
		hdr: sof_ipc_cmd_hdr {
			size: size_of::<sof_ipc_pm_core_config>() as u32,
			cmd: SOF_IPC_GLB_PM_MSG | SOF_IPC_PM_CORE_ENABLE,
		},
		enable_mask: 0,
	};

	if on {
		core_cfg.enable_mask = (*sdev).enabled_cores_mask | BIT(core_idx);
	} else {
		core_cfg.enable_mask = (*sdev).enabled_cores_mask & !BIT(core_idx);
	}

	sof_ipc3_tx_msg(sdev, &mut core_cfg as *mut _ as *mut c_void, size_of::<sof_ipc_pm_core_config>(), ptr::null_mut(), 0, false)
}

unsafe fn sof_ipc3_ctx_ipc(sdev: *mut snd_sof_dev, cmd: c_int) -> c_int {
	let mut pm_ctx = sof_ipc_pm_ctx {
		hdr: sof_ipc_cmd_hdr {
			size: size_of::<sof_ipc_pm_ctx>() as u32,
			cmd: SOF_IPC_GLB_PM_MSG | cmd as u32,
		},
	};

	sof_ipc3_tx_msg(sdev, &mut pm_ctx as *mut _ as *mut c_void, size_of::<sof_ipc_pm_ctx>(), ptr::null_mut(), 0, false)
}

unsafe extern "C" fn sof_ipc3_ctx_save(sdev: *mut snd_sof_dev) -> c_int {
	sof_ipc3_ctx_ipc(sdev, SOF_IPC_PM_CTX_SAVE as c_int)
}

unsafe extern "C" fn sof_ipc3_ctx_restore(sdev: *mut snd_sof_dev) -> c_int {
	sof_ipc3_ctx_ipc(sdev, SOF_IPC_PM_CTX_RESTORE as c_int)
}

unsafe extern "C" fn sof_ipc3_set_pm_gate(sdev: *mut snd_sof_dev, flags: u32) -> c_int {
	let mut pm_gate: sof_ipc_pm_gate = zeroed();

	pm_gate.hdr.size = size_of::<sof_ipc_pm_gate>() as u32;
	pm_gate.hdr.cmd = SOF_IPC_GLB_PM_MSG | SOF_IPC_PM_GATE;
	pm_gate.flags = flags;

	sof_ipc_tx_message_no_pm_no_reply((*sdev).ipc, &mut pm_gate as *mut _ as *mut c_void, size_of::<sof_ipc_pm_gate>())
}

static ipc3_pm_ops: sof_ipc_pm_ops = sof_ipc_pm_ops {
	ctx_save: Some(sof_ipc3_ctx_save),
	ctx_restore: Some(sof_ipc3_ctx_restore),
	set_core_state: Some(sof_ipc3_set_core_state),
	set_pm_gate: Some(sof_ipc3_set_pm_gate),
};

#[no_mangle]
pub static ipc3_ops: sof_ipc_ops = sof_ipc_ops {
	tplg: unsafe { &ipc3_tplg_ops },
	pm: &ipc3_pm_ops,
	pcm: unsafe { &ipc3_pcm_ops },
	fw_loader: unsafe { &ipc3_loader_ops },
	fw_tracing: unsafe { &ipc3_dtrace_ops },
	tx_msg: Some(sof_ipc3_tx_msg),
	rx_msg: Some(sof_ipc3_rx_msg),
	set_get_data: Some(sof_ipc3_set_get_data),
	get_reply: Some(sof_ipc3_get_reply),
};

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
