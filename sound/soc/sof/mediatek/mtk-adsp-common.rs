// SPDX-License-Identifier: (GPL-2.0-only OR BSD-3-Clause)
//
// This file is provided under a dual BSD/GPLv2 license. When using or
// redistributing this file, you may do so under either license.
//
// Copyright(c) 2022 MediaTek Inc. All rights reserved.
//
// Author: YC Hung <yc.hung@mediatek.com>

/*
 * Common helpers for the audio DSP on MediaTek platforms
 */

// C includes translated as external dependencies:
// linux/module.h
// sound/asound.h
// sound/sof/xtensa.h
// ../ops.h
// ../sof-audio.h
// adsp_helper.h
// mtk-adsp-common.h

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_void};
use core::mem::size_of;
use core::ptr;

type u32 = u32;
type size_t = usize;
type snd_pcm_uframes_t = usize;

const MTK_ADSP_STACK_DUMP_SIZE: usize = 32;

extern "C" {
    static KERN_DEBUG: *mut c_char;
    static KERN_ERR: *mut c_char;

    static SOF_DBG_DUMP_OPTIONAL: u32;
    static EXCEPT_MAX_HDR_SIZE: u32;
    static MTK_ADSP_IPC_REQ: c_int;
    static MTK_ADSP_IPC_OP_REQ: c_int;
    static MTK_ADSP_IPC_RSP: c_int;
    static MTK_ADSP_IPC_OP_RSP: c_int;
    static SOF_IPC_PANIC_MAGIC_MASK: u32;
    static SOF_IPC_PANIC_MAGIC: u32;

    fn sof_mailbox_read(
        sdev: *mut snd_sof_dev,
        offset: u32,
        dest: *mut c_void,
        bytes: size_t,
    );
    fn sof_mailbox_write(
        sdev: *mut snd_sof_dev,
        offset: u32,
        src: *const c_void,
        bytes: size_t,
    );
    fn dev_err(dev: *mut c_void, fmt: *const c_char, ...);
    fn dev_warn(dev: *mut c_void, fmt: *const c_char, ...);
    fn dev_warn_ratelimited(dev: *mut c_void, fmt: *const c_char, ...);
    fn sof_print_oops_and_stack(
        sdev: *mut snd_sof_dev,
        level: *mut c_char,
        tracep: u32,
        panic_code: u32,
        xoops: *mut sof_ipc_dsp_oops_xtensa,
        panic_info: *mut sof_ipc_panic_info,
        stack: *mut u32,
        stack_words: size_t,
    );
    fn mtk_adsp_ipc_send(ipc: *mut mtk_adsp_ipc, msg: c_int, op: c_int) -> c_int;
    fn mtk_adsp_ipc_get_data(ipc: *mut mtk_adsp_ipc) -> *mut adsp_priv;
    fn snd_sof_ipc_process_reply(sdev: *mut snd_sof_dev, msg_id: c_int);
    fn snd_sof_dsp_panic(sdev: *mut snd_sof_dev, panic_code: u32, non_recoverable: bool);
    fn snd_sof_ipc_msgs_rx(sdev: *mut snd_sof_dev);
    fn snd_soc_substream_to_rtd(substream: *mut snd_pcm_substream) -> *mut snd_soc_pcm_runtime;
    fn snd_sof_find_spcm_dai(
        scomp: *mut snd_soc_component,
        rtd: *mut snd_soc_pcm_runtime,
    ) -> *mut snd_sof_pcm;
    fn snd_sof_ipc_msg_data(
        sdev: *mut snd_sof_dev,
        stream: *mut snd_sof_pcm_stream,
        data: *mut c_void,
        bytes: size_t,
    ) -> c_int;
    fn bytes_to_frames(runtime: *mut snd_pcm_runtime, bytes: snd_pcm_uframes_t)
        -> snd_pcm_uframes_t;
    fn spin_lock_irqsave(lock: *mut spinlock_t) -> c_ulong;
    fn spin_unlock_irqrestore(lock: *mut spinlock_t, flags: c_ulong);
}

type c_ulong = core::ffi::c_ulong;

#[repr(C)]
pub struct spinlock_t {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_sof_dev {
    pub dev: *mut c_void,
    pub pdata: *mut snd_sof_pdata,
    pub component: *mut snd_soc_component,
    pub dsp_oops_offset: u32,
    pub debug_box: snd_sof_debug_box,
    pub host_box: snd_sof_host_box,
    pub ipc_lock: spinlock_t,
}

#[repr(C)]
pub struct snd_sof_pdata {
    pub hw_pdata: *mut c_void,
}

#[repr(C)]
pub struct snd_sof_debug_box {
    pub offset: u32,
}

#[repr(C)]
pub struct snd_sof_host_box {
    pub offset: u32,
}

#[repr(C)]
pub struct sof_ipc_dsp_oops_xtensa {
    pub arch_hdr: sof_ipc_dsp_oops_arch_hdr,
}

#[repr(C)]
pub struct sof_ipc_dsp_oops_arch_hdr {
    pub totalsize: u32,
}

#[repr(C)]
pub struct sof_ipc_panic_info {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_sof_ipc_msg {
    pub msg_data: *const c_void,
    pub msg_size: size_t,
}

#[repr(C)]
pub struct adsp_priv {
    pub sdev: *mut snd_sof_dev,
    pub dev: *mut c_void,
    pub dsp_ipc: *mut mtk_adsp_ipc,
}

#[repr(C)]
pub struct mtk_adsp_ipc {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_pcm_substream {
    pub stream: usize,
    pub runtime: *mut snd_pcm_runtime,
}

#[repr(C)]
pub struct snd_pcm_runtime {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_pcm_hw_params {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_sof_platform_stream_params {
    pub cont_update_posn: c_int,
}

#[repr(C)]
pub struct snd_soc_pcm_runtime {
    pub dai_link: *mut snd_soc_dai_link,
}

#[repr(C)]
pub struct snd_soc_dai_link {
    pub id: c_int,
}

#[repr(C)]
pub struct snd_soc_component {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_sof_pcm_stream {
    pub posn: sof_ipc_stream_posn,
}

#[repr(C)]
pub struct sof_ipc_stream_posn {
    pub host_posn: snd_pcm_uframes_t,
}

#[repr(C)]
pub struct snd_sof_pcm {
    pub stream: *mut snd_sof_pcm_stream,
}

/**
 * mtk_adsp_get_registers() - This function is called in case of DSP oops
 * in order to gather information about the registers, filename and
 * linenumber and stack.
 * @sdev: SOF device
 * @xoops: Stores information about registers.
 * @panic_info: Stores information about filename and line number.
 * @stack: Stores the stack dump.
 * @stack_words: Size of the stack dump.
 */
unsafe fn mtk_adsp_get_registers(
    sdev: *mut snd_sof_dev,
    xoops: *mut sof_ipc_dsp_oops_xtensa,
    panic_info: *mut sof_ipc_panic_info,
    stack: *mut u32,
    stack_words: size_t,
) {
    let mut offset: u32 = (*sdev).dsp_oops_offset;

    /* first read registers */
    sof_mailbox_read(
        sdev,
        offset,
        xoops as *mut c_void,
        size_of::<sof_ipc_dsp_oops_xtensa>(),
    );

    /* then get panic info */
    if (*xoops).arch_hdr.totalsize > EXCEPT_MAX_HDR_SIZE {
        dev_err(
            (*sdev).dev,
            c"invalid header size 0x%x\n".as_ptr(),
            (*xoops).arch_hdr.totalsize,
        );
        return;
    }
    offset = offset.wrapping_add((*xoops).arch_hdr.totalsize);
    sof_mailbox_read(
        sdev,
        offset,
        panic_info as *mut c_void,
        size_of::<sof_ipc_panic_info>(),
    );

    /* then get the stack */
    offset = offset.wrapping_add(size_of::<sof_ipc_panic_info>() as u32);
    sof_mailbox_read(
        sdev,
        offset,
        stack as *mut c_void,
        stack_words.wrapping_mul(size_of::<u32>()),
    );
}

/**
 * mtk_adsp_dump() - This function is called when a panic message is
 * received from the firmware.
 * @sdev: SOF device
 * @flags: parameter not used but required by ops prototype
 */
#[no_mangle]
pub unsafe extern "C" fn mtk_adsp_dump(sdev: *mut snd_sof_dev, flags: u32) {
    let level: *mut c_char = if flags & SOF_DBG_DUMP_OPTIONAL != 0 {
        KERN_DEBUG
    } else {
        KERN_ERR
    };
    let mut xoops: sof_ipc_dsp_oops_xtensa = core::mem::zeroed();
    let mut panic_info: sof_ipc_panic_info = core::mem::zeroed();
    let mut stack: [u32; MTK_ADSP_STACK_DUMP_SIZE] = [0; MTK_ADSP_STACK_DUMP_SIZE];
    let mut status: u32 = 0;

    /* Get information about the panic status from the debug box area.
     * Compute the trace point based on the status.
     */
    sof_mailbox_read(
        sdev,
        (*sdev).debug_box.offset.wrapping_add(0x4),
        &mut status as *mut u32 as *mut c_void,
        4,
    );

    /* Get information about the registers, the filename and line
     * number and the stack.
     */
    mtk_adsp_get_registers(
        sdev,
        &mut xoops,
        &mut panic_info,
        stack.as_mut_ptr(),
        MTK_ADSP_STACK_DUMP_SIZE,
    );

    /* Print the information to the console */
    sof_print_oops_and_stack(
        sdev,
        level,
        status,
        status,
        &mut xoops,
        &mut panic_info,
        stack.as_mut_ptr(),
        MTK_ADSP_STACK_DUMP_SIZE,
    );
}
// EXPORT_SYMBOL(mtk_adsp_dump);

/**
 * mtk_adsp_send_msg - Send message to Audio DSP
 * @sdev: SOF device
 * @msg: SOF IPC Message to send
 */
#[no_mangle]
pub unsafe extern "C" fn mtk_adsp_send_msg(
    sdev: *mut snd_sof_dev,
    msg: *mut snd_sof_ipc_msg,
) -> c_int {
    let priv_: *mut adsp_priv = (*(*sdev).pdata).hw_pdata as *mut adsp_priv;

    sof_mailbox_write(
        sdev,
        (*sdev).host_box.offset,
        (*msg).msg_data,
        (*msg).msg_size,
    );

    mtk_adsp_ipc_send((*priv_).dsp_ipc, MTK_ADSP_IPC_REQ, MTK_ADSP_IPC_OP_REQ)
}
// EXPORT_SYMBOL(mtk_adsp_send_msg);

/**
 * mtk_adsp_handle_reply - Handle reply from the Audio DSP through Mailbox
 * @ipc: ADSP IPC handle
 */
#[no_mangle]
pub unsafe extern "C" fn mtk_adsp_handle_reply(ipc: *mut mtk_adsp_ipc) {
    let priv_: *mut adsp_priv = mtk_adsp_ipc_get_data(ipc);

    let flags: c_ulong = spin_lock_irqsave(&mut (*(*priv_).sdev).ipc_lock);
    snd_sof_ipc_process_reply((*priv_).sdev, 0);
    spin_unlock_irqrestore(&mut (*(*priv_).sdev).ipc_lock, flags);
}
// EXPORT_SYMBOL(mtk_adsp_handle_reply);

/**
 * mtk_adsp_handle_request - Handle request from the Audio DSP through Mailbox
 * @ipc: ADSP IPC handle
 */
#[no_mangle]
pub unsafe extern "C" fn mtk_adsp_handle_request(ipc: *mut mtk_adsp_ipc) {
    let priv_: *mut adsp_priv = mtk_adsp_ipc_get_data(ipc);
    let mut panic_code: u32 = 0;
    let ret: c_int;

    /* Read the message from the debug box. */
    sof_mailbox_read(
        (*priv_).sdev,
        (*(*priv_).sdev).debug_box.offset.wrapping_add(4),
        &mut panic_code as *mut u32 as *mut c_void,
        size_of::<u32>(),
    );

    /* Check to see if the message is a panic code 0x0dead*** */
    if panic_code & SOF_IPC_PANIC_MAGIC_MASK == SOF_IPC_PANIC_MAGIC {
        snd_sof_dsp_panic((*priv_).sdev, panic_code, true);
    } else {
        snd_sof_ipc_msgs_rx((*priv_).sdev);

        /* Tell DSP cmd is done */
        ret = mtk_adsp_ipc_send((*priv_).dsp_ipc, MTK_ADSP_IPC_RSP, MTK_ADSP_IPC_OP_RSP);
        if ret != 0 {
            dev_err((*priv_).dev, c"request send ipc failed".as_ptr());
        }
    }
}
// EXPORT_SYMBOL(mtk_adsp_handle_request);

/**
 * mtk_adsp_get_bar_index - Map section type with BAR idx
 * @sdev: SOF device
 * @type: Section type as described by snd_sof_fw_blk_type
 *
 * MediaTek Audio DSPs have a 1:1 match between type and BAR idx
 */
#[no_mangle]
pub unsafe extern "C" fn mtk_adsp_get_bar_index(
    _sdev: *mut snd_sof_dev,
    type_: u32,
) -> c_int {
    type_ as c_int
}
// EXPORT_SYMBOL(mtk_adsp_get_bar_index);

/**
 * mtk_adsp_stream_pcm_hw_params - Platform specific host stream hw params
 * @sdev: SOF device
 * @substream: PCM Substream
 * @params: hw params
 * @platform_params: Platform specific SOF stream parameters
 */
#[no_mangle]
pub unsafe extern "C" fn mtk_adsp_stream_pcm_hw_params(
    _sdev: *mut snd_sof_dev,
    _substream: *mut snd_pcm_substream,
    _params: *mut snd_pcm_hw_params,
    platform_params: *mut snd_sof_platform_stream_params,
) -> c_int {
    (*platform_params).cont_update_posn = 1;
    0
}
// EXPORT_SYMBOL(mtk_adsp_stream_pcm_hw_params);

/**
 * mtk_adsp_stream_pcm_pointer - Get host stream pointer
 * @sdev: SOF device
 * @substream: PCM substream
 */
#[no_mangle]
pub unsafe extern "C" fn mtk_adsp_stream_pcm_pointer(
    sdev: *mut snd_sof_dev,
    substream: *mut snd_pcm_substream,
) -> snd_pcm_uframes_t {
    let rtd: *mut snd_soc_pcm_runtime = snd_soc_substream_to_rtd(substream);
    let scomp: *mut snd_soc_component = (*sdev).component;
    let stream: *mut snd_sof_pcm_stream;
    let mut posn: sof_ipc_stream_posn = core::mem::zeroed();
    let spcm: *mut snd_sof_pcm;
    let mut pos: snd_pcm_uframes_t;
    let ret: c_int;

    spcm = snd_sof_find_spcm_dai(scomp, rtd);
    if spcm.is_null() {
        dev_warn_ratelimited(
            (*sdev).dev,
            c"warn: can't find PCM with DAI ID %d\n".as_ptr(),
            (*(*rtd).dai_link).id,
        );
        return 0;
    }

    stream = (*spcm).stream.add((*substream).stream);
    ret = snd_sof_ipc_msg_data(
        sdev,
        stream,
        &mut posn as *mut sof_ipc_stream_posn as *mut c_void,
        size_of::<sof_ipc_stream_posn>(),
    );
    if ret < 0 {
        dev_warn(
            (*sdev).dev,
            c"failed to read stream position: %d\n".as_ptr(),
            ret,
        );
        return 0;
    }

    ptr::copy_nonoverlapping(
        &posn as *const sof_ipc_stream_posn,
        &mut (*stream).posn as *mut sof_ipc_stream_posn,
        1,
    );
    pos = (*(*spcm).stream.add((*substream).stream)).posn.host_posn;
    pos = bytes_to_frames((*substream).runtime, pos);

    pos
}
// EXPORT_SYMBOL(mtk_adsp_stream_pcm_pointer);

// MODULE_LICENSE("Dual BSD/GPL");
// MODULE_DESCRIPTION("SOF helpers for MTK ADSP platforms");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
