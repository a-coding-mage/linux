// SPDX-License-Identifier: GPL-2.0-only
//
// Copyright(c) 2020 Intel Corporation
//
// Author: Cezary Rojewski <cezary.rojewski@intel.com>
//

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_void};
use core::mem::{offset_of, size_of};
use core::ptr::{copy, copy_nonoverlapping, null_mut};

type u8 = ::core::ffi::c_uchar;
type u32 = ::core::ffi::c_uint;
type u64 = ::core::ffi::c_ulonglong;
type size_t = usize;
type off_t = isize;

const GFP_KERNEL: u32 = 0;
const ENOMEM: i32 = 12;

#[repr(C)]
pub struct catpt_dev {
    _private: [u8; 0],
}

#[repr(C)]
pub struct catpt_fw_version {
    _private: [u8; 0],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct catpt_audio_format {
    _private: [u8; 0],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct catpt_ring_info {
    _private: [u8; 0],
}

#[repr(C)]
pub struct catpt_module_entry {
    _private: [u8; 0],
}

#[repr(C)]
pub struct catpt_stream_info {
    _private: [u8; 0],
}

#[repr(C)]
pub struct catpt_ssp_device_format {
    _private: [u8; 0],
}

#[repr(C)]
pub struct catpt_dx_context {
    _private: [u8; 0],
}

#[repr(C)]
pub struct catpt_mixer_stream_info {
    _private: [u8; 0],
}

#[repr(C)]
pub struct resource {
    pub start: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct catpt_memory_info {
    pub offset: u32,
    pub size: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct catpt_ipc_msg {
    pub header: u32,
    pub size: size_t,
    pub data: *mut c_void,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union catpt_global_msg {
    pub val: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union catpt_stream_msg {
    pub val: u32,
    pub stream_hw_id: u8,
}

pub type catpt_path_id = u32;
pub type catpt_stream_type = u32;
pub type catpt_format_id = u32;
pub type catpt_dx_state = u32;
pub type catpt_audio_curve_type = u32;

unsafe extern "C" {
    fn CATPT_GLOBAL_MSG(command: u32) -> catpt_global_msg;
    fn CATPT_STREAM_MSG(command: u32) -> catpt_stream_msg;
    fn CATPT_STAGE_MSG(command: u32) -> catpt_stream_msg;

    static GET_FW_VERSION: u32;
    static ALLOCATE_STREAM: u32;
    static FREE_STREAM: u32;
    static SET_DEVICE_FORMATS: u32;
    static ENTER_DX_STATE: u32;
    static GET_MIXER_STREAM_INFO: u32;
    static RESET_STREAM: u32;
    static PAUSE_STREAM: u32;
    static RESUME_STREAM: u32;
    static SET_VOLUME: u32;
    static SET_WRITE_POSITION: u32;
    static MUTE_LOOPBACK: u32;
    static CATPT_FORMAT_PCM: catpt_format_id;

    fn catpt_dsp_send_msg(
        cdev: *mut catpt_dev,
        request: catpt_ipc_msg,
        reply: *mut catpt_ipc_msg,
        msg: *const c_char,
    ) -> i32;
    fn catpt_to_dsp_offset(addr: u64) -> u32;
    fn resource_size(res: *mut resource) -> u32;
    fn kzalloc(size: size_t, flags: u32) -> *mut c_void;
    fn kfree(ptr: *mut c_void);
}

pub unsafe fn catpt_ipc_get_fw_version(
    cdev: *mut catpt_dev,
    version: *mut catpt_fw_version,
) -> i32 {
    let msg = CATPT_GLOBAL_MSG(GET_FW_VERSION);
    let mut request = catpt_ipc_msg {
        header: 0,
        size: 0,
        data: null_mut(),
    };
    let mut reply = catpt_ipc_msg {
        header: 0,
        size: 0,
        data: null_mut(),
    };

    request.header = msg.val;
    reply.size = size_of::<catpt_fw_version>();
    reply.data = version as *mut c_void;

    catpt_dsp_send_msg(cdev, request, &mut reply, c"get fw version".as_ptr())
}

#[repr(C, packed)]
pub struct catpt_alloc_stream_input {
    pub path_id: u8,
    pub stream_type: u8,
    pub format_id: u8,
    pub reserved: u8,
    pub input_format: catpt_audio_format,
    pub ring_info: catpt_ring_info,
    pub num_entries: u8,
    /* flex array with entries here */
    pub persistent_mem: catpt_memory_info,
    pub scratch_mem: catpt_memory_info,
    pub num_notifications: u32, /* obsolete */
}

pub unsafe fn catpt_ipc_alloc_stream(
    cdev: *mut catpt_dev,
    path_id: catpt_path_id,
    type_: catpt_stream_type,
    afmt: *mut catpt_audio_format,
    rinfo: *mut catpt_ring_info,
    num_modules: u8,
    modules: *mut catpt_module_entry,
    persistent: *mut resource,
    scratch: *mut resource,
    sinfo: *mut catpt_stream_info,
) -> i32 {
    let msg = CATPT_GLOBAL_MSG(ALLOCATE_STREAM);
    let mut input = catpt_alloc_stream_input {
        path_id: 0,
        stream_type: 0,
        format_id: 0,
        reserved: 0,
        input_format: catpt_audio_format { _private: [] },
        ring_info: catpt_ring_info { _private: [] },
        num_entries: 0,
        persistent_mem: catpt_memory_info { offset: 0, size: 0 },
        scratch_mem: catpt_memory_info { offset: 0, size: 0 },
        num_notifications: 0,
    };
    let mut request: catpt_ipc_msg;
    let mut reply: catpt_ipc_msg;
    let size: size_t;
    let arrsz: size_t;
    let payload: *mut u8;
    let off: off_t;
    let ret: i32;

    off = offset_of!(catpt_alloc_stream_input, persistent_mem) as off_t;
    arrsz = size_of::<catpt_module_entry>() * num_modules as size_t;
    size = size_of::<catpt_alloc_stream_input>() + arrsz;

    payload = kzalloc(size, GFP_KERNEL) as *mut u8;
    if payload.is_null() {
        return -ENOMEM;
    }

    input.path_id = path_id as u8;
    input.stream_type = type_ as u8;
    input.format_id = CATPT_FORMAT_PCM as u8;
    input.input_format = *afmt;
    input.ring_info = *rinfo;
    input.num_entries = num_modules;
    input.persistent_mem.offset = catpt_to_dsp_offset((*persistent).start);
    input.persistent_mem.size = resource_size(persistent);
    if !scratch.is_null() {
        input.scratch_mem.offset = catpt_to_dsp_offset((*scratch).start);
        input.scratch_mem.size = resource_size(scratch);
    }

    /* re-arrange the input: account for flex array 'entries' */
    copy_nonoverlapping(
        &input as *const catpt_alloc_stream_input as *const u8,
        payload,
        size_of::<catpt_alloc_stream_input>(),
    );
    copy(
        payload.offset(off),
        payload.offset(off + arrsz as off_t),
        size_of::<catpt_alloc_stream_input>() - off as size_t,
    );
    copy_nonoverlapping(modules as *const u8, payload.offset(off), arrsz);

    request = catpt_ipc_msg {
        header: msg.val,
        size,
        data: payload as *mut c_void,
    };
    reply = catpt_ipc_msg {
        header: 0,
        size: size_of::<catpt_stream_info>(),
        data: sinfo as *mut c_void,
    };

    ret = catpt_dsp_send_msg(cdev, request, &mut reply, c"alloc stream".as_ptr());
    kfree(payload as *mut c_void);
    ret
}

pub unsafe fn catpt_ipc_free_stream(cdev: *mut catpt_dev, mut stream_hw_id: u8) -> i32 {
    let msg = CATPT_GLOBAL_MSG(FREE_STREAM);
    let request: catpt_ipc_msg;

    request = catpt_ipc_msg {
        header: msg.val,
        size: size_of::<u8>(),
        data: &mut stream_hw_id as *mut u8 as *mut c_void,
    };

    catpt_dsp_send_msg(cdev, request, null_mut(), c"free stream".as_ptr())
}

pub unsafe fn catpt_ipc_set_device_format(
    cdev: *mut catpt_dev,
    devfmt: *mut catpt_ssp_device_format,
) -> i32 {
    let msg = CATPT_GLOBAL_MSG(SET_DEVICE_FORMATS);
    let request: catpt_ipc_msg;

    request = catpt_ipc_msg {
        header: msg.val,
        size: size_of::<catpt_ssp_device_format>(),
        data: devfmt as *mut c_void,
    };

    catpt_dsp_send_msg(cdev, request, null_mut(), c"set device format".as_ptr())
}

pub unsafe fn catpt_ipc_enter_dxstate(
    cdev: *mut catpt_dev,
    mut state: catpt_dx_state,
    context: *mut catpt_dx_context,
) -> i32 {
    let msg = CATPT_GLOBAL_MSG(ENTER_DX_STATE);
    let request: catpt_ipc_msg;
    let mut reply: catpt_ipc_msg;

    request = catpt_ipc_msg {
        header: msg.val,
        size: size_of::<catpt_dx_state>(),
        data: &mut state as *mut catpt_dx_state as *mut c_void,
    };
    reply = catpt_ipc_msg {
        header: 0,
        size: size_of::<catpt_dx_context>(),
        data: context as *mut c_void,
    };

    catpt_dsp_send_msg(cdev, request, &mut reply, c"enter dx state".as_ptr())
}

pub unsafe fn catpt_ipc_get_mixer_stream_info(
    cdev: *mut catpt_dev,
    info: *mut catpt_mixer_stream_info,
) -> i32 {
    let msg = CATPT_GLOBAL_MSG(GET_MIXER_STREAM_INFO);
    let mut request = catpt_ipc_msg {
        header: 0,
        size: 0,
        data: null_mut(),
    };
    let mut reply = catpt_ipc_msg {
        header: 0,
        size: 0,
        data: null_mut(),
    };

    request.header = msg.val;
    reply.size = size_of::<catpt_mixer_stream_info>();
    reply.data = info as *mut c_void;

    catpt_dsp_send_msg(cdev, request, &mut reply, c"get mixer info".as_ptr())
}

pub unsafe fn catpt_ipc_reset_stream(cdev: *mut catpt_dev, stream_hw_id: u8) -> i32 {
    let mut msg = CATPT_STREAM_MSG(RESET_STREAM);
    let mut request = catpt_ipc_msg {
        header: 0,
        size: 0,
        data: null_mut(),
    };

    msg.stream_hw_id = stream_hw_id;
    request.header = msg.val;

    catpt_dsp_send_msg(cdev, request, null_mut(), c"reset stream".as_ptr())
}

pub unsafe fn catpt_ipc_pause_stream(cdev: *mut catpt_dev, stream_hw_id: u8) -> i32 {
    let mut msg = CATPT_STREAM_MSG(PAUSE_STREAM);
    let mut request = catpt_ipc_msg {
        header: 0,
        size: 0,
        data: null_mut(),
    };

    msg.stream_hw_id = stream_hw_id;
    request.header = msg.val;

    catpt_dsp_send_msg(cdev, request, null_mut(), c"pause stream".as_ptr())
}

pub unsafe fn catpt_ipc_resume_stream(cdev: *mut catpt_dev, stream_hw_id: u8) -> i32 {
    let mut msg = CATPT_STREAM_MSG(RESUME_STREAM);
    let mut request = catpt_ipc_msg {
        header: 0,
        size: 0,
        data: null_mut(),
    };

    msg.stream_hw_id = stream_hw_id;
    request.header = msg.val;

    catpt_dsp_send_msg(cdev, request, null_mut(), c"resume stream".as_ptr())
}

#[repr(C, packed)]
pub struct catpt_set_volume_input {
    pub channel: u32,
    pub target_volume: u32,
    pub curve_duration: u64,
    pub curve_type: u32,
}

pub unsafe fn catpt_ipc_set_volume(
    cdev: *mut catpt_dev,
    stream_hw_id: u8,
    channel: u32,
    volume: u32,
    curve_duration: u32,
    curve_type: catpt_audio_curve_type,
) -> i32 {
    let mut msg = CATPT_STAGE_MSG(SET_VOLUME);
    let request: catpt_ipc_msg;
    let mut input: catpt_set_volume_input;

    msg.stream_hw_id = stream_hw_id;
    input = catpt_set_volume_input {
        channel,
        target_volume: volume,
        curve_duration: curve_duration as u64,
        curve_type,
    };

    request = catpt_ipc_msg {
        header: msg.val,
        size: size_of::<catpt_set_volume_input>(),
        data: &mut input as *mut catpt_set_volume_input as *mut c_void,
    };

    catpt_dsp_send_msg(cdev, request, null_mut(), c"set stream volume".as_ptr())
}

#[repr(C, packed)]
pub struct catpt_set_write_pos_input {
    pub new_write_pos: u32,
    pub end_of_buffer: bool,
    pub low_latency: bool,
}

pub unsafe fn catpt_ipc_set_write_pos(
    cdev: *mut catpt_dev,
    stream_hw_id: u8,
    pos: u32,
    eob: bool,
    ll: bool,
) -> i32 {
    let mut msg = CATPT_STAGE_MSG(SET_WRITE_POSITION);
    let request: catpt_ipc_msg;
    let mut input: catpt_set_write_pos_input;

    msg.stream_hw_id = stream_hw_id;
    input = catpt_set_write_pos_input {
        new_write_pos: pos,
        end_of_buffer: eob,
        low_latency: ll,
    };

    request = catpt_ipc_msg {
        header: msg.val,
        size: size_of::<catpt_set_write_pos_input>(),
        data: &mut input as *mut catpt_set_write_pos_input as *mut c_void,
    };

    catpt_dsp_send_msg(cdev, request, null_mut(), c"set stream write pos".as_ptr())
}

pub unsafe fn catpt_ipc_mute_loopback(
    cdev: *mut catpt_dev,
    stream_hw_id: u8,
    mut mute: bool,
) -> i32 {
    let mut msg = CATPT_STAGE_MSG(MUTE_LOOPBACK);
    let request: catpt_ipc_msg;

    msg.stream_hw_id = stream_hw_id;
    request = catpt_ipc_msg {
        header: msg.val,
        size: size_of::<bool>(),
        data: &mut mute as *mut bool as *mut c_void,
    };

    catpt_dsp_send_msg(cdev, request, null_mut(), c"mute loopback".as_ptr())
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
