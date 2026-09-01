// SPDX-License-Identifier: (GPL-2.0-only OR BSD-3-Clause)
//
// This file is provided under a dual BSD/GPLv2 license.  When using or
// redistributing this file, you may do so under either license.
//
// Copyright(c) 2022 Intel Corporation
//
// Authors: Rander Wang <rander.wang@linux.intel.com>
//	    Peter Ujfalusi <peter.ujfalusi@linux.intel.com>
//
// C dependencies:
// <linux/firmware.h>
// <sound/sof/header.h>
// <sound/sof/ipc4/header.h>
// "sof-priv.h"
// "sof-audio.h"
// "ipc4-fw-reg.h"
// "ipc4-priv.h"
// "ipc4-topology.h"
// "ipc4-telemetry.h"
// "ops.h"

use core::ffi::{c_char, c_int, c_ulong, c_void};
use core::mem::size_of;
use core::ptr;

#[repr(C)]
struct sof_ipc4_fw_status {
    status: c_int,
    msg: *const c_char,
}

static ipc4_status: [sof_ipc4_fw_status; 63] = [
    sof_ipc4_fw_status { status: 0, msg: c"The operation was successful".as_ptr() },
    sof_ipc4_fw_status { status: 1, msg: c"Invalid parameter specified".as_ptr() },
    sof_ipc4_fw_status { status: 2, msg: c"Unknown message type specified".as_ptr() },
    sof_ipc4_fw_status { status: 3, msg: c"Not enough space in the IPC reply buffer to complete the request".as_ptr() },
    sof_ipc4_fw_status { status: 4, msg: c"The system or resource is busy".as_ptr() },
    sof_ipc4_fw_status { status: 5, msg: c"Replaced ADSP IPC PENDING (unused)".as_ptr() },
    sof_ipc4_fw_status { status: 6, msg: c"Unknown error while processing the request".as_ptr() },
    sof_ipc4_fw_status { status: 7, msg: c"Unsupported operation requested".as_ptr() },
    sof_ipc4_fw_status { status: 8, msg: c"Reserved (ADSP_STAGE_UNINITIALIZED removed)".as_ptr() },
    sof_ipc4_fw_status { status: 9, msg: c"Specified resource not found".as_ptr() },
    sof_ipc4_fw_status { status: 10, msg: c"A resource's ID requested to be created is already assigned".as_ptr() },
    sof_ipc4_fw_status { status: 11, msg: c"Reserved (ADSP_IPC_OUT_OF_MIPS removed)".as_ptr() },
    sof_ipc4_fw_status { status: 12, msg: c"Required resource is in invalid state".as_ptr() },
    sof_ipc4_fw_status { status: 13, msg: c"Requested power transition failed to complete".as_ptr() },
    sof_ipc4_fw_status { status: 14, msg: c"Manifest of the library being loaded is invalid".as_ptr() },
    sof_ipc4_fw_status { status: 15, msg: c"Requested service or data is unavailable on the target platform".as_ptr() },
    sof_ipc4_fw_status { status: 42, msg: c"Library target address is out of storage memory range".as_ptr() },
    sof_ipc4_fw_status { status: 43, msg: c"Reserved".as_ptr() },
    sof_ipc4_fw_status { status: 44, msg: c"Image verification by CSE failed".as_ptr() },
    sof_ipc4_fw_status { status: 100, msg: c"General module management error".as_ptr() },
    sof_ipc4_fw_status { status: 101, msg: c"Module loading failed".as_ptr() },
    sof_ipc4_fw_status { status: 102, msg: c"Integrity check of the loaded module content failed".as_ptr() },
    sof_ipc4_fw_status { status: 103, msg: c"Attempt to unload code of the module in use".as_ptr() },
    sof_ipc4_fw_status { status: 104, msg: c"Other failure of module instance initialization request".as_ptr() },
    sof_ipc4_fw_status { status: 105, msg: c"Reserved (ADSP_IPC_OUT_OF_MIPS removed)".as_ptr() },
    sof_ipc4_fw_status { status: 106, msg: c"Reserved (ADSP_IPC_CONFIG_GET_ERROR removed)".as_ptr() },
    sof_ipc4_fw_status { status: 107, msg: c"Reserved (ADSP_IPC_CONFIG_SET_ERROR removed)".as_ptr() },
    sof_ipc4_fw_status { status: 108, msg: c"Reserved (ADSP_IPC_LARGE_CONFIG_GET_ERROR removed)".as_ptr() },
    sof_ipc4_fw_status { status: 109, msg: c"Reserved (ADSP_IPC_LARGE_CONFIG_SET_ERROR removed)".as_ptr() },
    sof_ipc4_fw_status { status: 110, msg: c"Invalid (out of range) module ID provided".as_ptr() },
    sof_ipc4_fw_status { status: 111, msg: c"Invalid module instance ID provided".as_ptr() },
    sof_ipc4_fw_status { status: 112, msg: c"Invalid queue (pin) ID provided".as_ptr() },
    sof_ipc4_fw_status { status: 113, msg: c"Invalid destination queue (pin) ID provided".as_ptr() },
    sof_ipc4_fw_status { status: 114, msg: c"Reserved (ADSP_IPC_BIND_UNBIND_DST_SINK_UNSUPPORTED removed)".as_ptr() },
    sof_ipc4_fw_status { status: 115, msg: c"Reserved (ADSP_IPC_UNLOAD_INST_EXISTS removed)".as_ptr() },
    sof_ipc4_fw_status { status: 116, msg: c"Invalid target code ID provided".as_ptr() },
    sof_ipc4_fw_status { status: 117, msg: c"Injection DMA buffer is too small for probing the input pin".as_ptr() },
    sof_ipc4_fw_status { status: 118, msg: c"Extraction DMA buffer is too small for probing the output pin".as_ptr() },
    sof_ipc4_fw_status { status: 120, msg: c"Invalid ID of configuration item provided in TLV list".as_ptr() },
    sof_ipc4_fw_status { status: 121, msg: c"Invalid length of configuration item provided in TLV list".as_ptr() },
    sof_ipc4_fw_status { status: 122, msg: c"Invalid structure of configuration item provided".as_ptr() },
    sof_ipc4_fw_status { status: 140, msg: c"Initialization of DMA Gateway failed".as_ptr() },
    sof_ipc4_fw_status { status: 141, msg: c"Invalid ID of gateway provided".as_ptr() },
    sof_ipc4_fw_status { status: 142, msg: c"Setting state of DMA Gateway failed".as_ptr() },
    sof_ipc4_fw_status { status: 143, msg: c"DMA_CONTROL message targeting gateway not allocated yet".as_ptr() },
    sof_ipc4_fw_status { status: 150, msg: c"Attempt to configure SCLK while I2S port is running".as_ptr() },
    sof_ipc4_fw_status { status: 151, msg: c"Attempt to configure MCLK while I2S port is running".as_ptr() },
    sof_ipc4_fw_status { status: 152, msg: c"Attempt to stop SCLK that is not running".as_ptr() },
    sof_ipc4_fw_status { status: 153, msg: c"Attempt to stop MCLK that is not running".as_ptr() },
    sof_ipc4_fw_status { status: 160, msg: c"Reserved (ADSP_IPC_PIPELINE_NOT_INITIALIZED removed)".as_ptr() },
    sof_ipc4_fw_status { status: 161, msg: c"Reserved (ADSP_IPC_PIPELINE_NOT_EXIST removed)".as_ptr() },
    sof_ipc4_fw_status { status: 162, msg: c"Reserved (ADSP_IPC_PIPELINE_SAVE_FAILED removed)".as_ptr() },
    sof_ipc4_fw_status { status: 163, msg: c"Reserved (ADSP_IPC_PIPELINE_RESTORE_FAILED removed)".as_ptr() },
    sof_ipc4_fw_status { status: 165, msg: c"Reserved (ADSP_IPC_PIPELINE_ALREADY_EXISTS removed)".as_ptr() },
];

type ipc4_notification_handler =
    Option<unsafe extern "C" fn(sdev: *mut snd_sof_dev, msg: *mut sof_ipc4_msg)>;

unsafe fn sof_ipc4_check_reply_status(sdev: *mut snd_sof_dev, mut status: u32) -> c_int {
    let mut ret: c_int;

    status &= SOF_IPC4_REPLY_STATUS;

    if status == 0 {
        return 0;
    }

    let mut i = 0usize;
    while i < ipc4_status.len() {
        if ipc4_status[i].status == status as c_int {
            dev_err(
                (*sdev).dev,
                c"FW reported error: %u - %s\n".as_ptr(),
                status,
                ipc4_status[i].msg,
            );
            break;
        }
        i += 1;
    }

    if i == ipc4_status.len() {
        dev_err((*sdev).dev, c"FW reported error: %u - Unknown\n".as_ptr(), status);
    }

    ret = match status {
        2 | 15 => -EOPNOTSUPP,
        8 | 11 | 105..=109 | 114..=115 | 160..=163 | 165 => -ENOENT,
        4 | 150 | 151 => -EBUSY,
        _ => -EINVAL,
    };

    ret
}

// IS_ENABLED(CONFIG_SND_SOC_SOF_DEBUG_VERBOSE_IPC)
const ipc4_dbg_mod_msg_type: [*const c_char; SOF_IPC4_MOD_TYPE_LAST as usize] = [
    /* [SOF_IPC4_MOD_INIT_INSTANCE] = */ c"MOD_INIT_INSTANCE".as_ptr(),
    /* [SOF_IPC4_MOD_CONFIG_GET] = */ c"MOD_CONFIG_GET".as_ptr(),
    /* [SOF_IPC4_MOD_CONFIG_SET] = */ c"MOD_CONFIG_SET".as_ptr(),
    /* [SOF_IPC4_MOD_LARGE_CONFIG_GET] = */ c"MOD_LARGE_CONFIG_GET".as_ptr(),
    /* [SOF_IPC4_MOD_LARGE_CONFIG_SET] = */ c"MOD_LARGE_CONFIG_SET".as_ptr(),
    /* [SOF_IPC4_MOD_BIND] = */ c"MOD_BIND".as_ptr(),
    /* [SOF_IPC4_MOD_UNBIND] = */ c"MOD_UNBIND".as_ptr(),
    /* [SOF_IPC4_MOD_SET_DX] = */ c"MOD_SET_DX".as_ptr(),
    /* [SOF_IPC4_MOD_SET_D0IX] = */ c"MOD_SET_D0IX".as_ptr(),
    /* [SOF_IPC4_MOD_ENTER_MODULE_RESTORE] = */ c"MOD_ENTER_MODULE_RESTORE".as_ptr(),
    /* [SOF_IPC4_MOD_EXIT_MODULE_RESTORE] = */ c"MOD_EXIT_MODULE_RESTORE".as_ptr(),
    /* [SOF_IPC4_MOD_DELETE_INSTANCE] = */ c"MOD_DELETE_INSTANCE".as_ptr(),
];

const ipc4_dbg_glb_msg_type: [*const c_char; SOF_IPC4_GLB_TYPE_LAST as usize] = [
    /* [SOF_IPC4_GLB_BOOT_CONFIG] = */ c"GLB_BOOT_CONFIG".as_ptr(),
    /* [SOF_IPC4_GLB_ROM_CONTROL] = */ c"GLB_ROM_CONTROL".as_ptr(),
    /* [SOF_IPC4_GLB_IPCGATEWAY_CMD] = */ c"GLB_IPCGATEWAY_CMD".as_ptr(),
    /* [SOF_IPC4_GLB_PERF_MEASUREMENTS_CMD] = */ c"GLB_PERF_MEASUREMENTS_CMD".as_ptr(),
    /* [SOF_IPC4_GLB_CHAIN_DMA] = */ c"GLB_CHAIN_DMA".as_ptr(),
    /* [SOF_IPC4_GLB_LOAD_MULTIPLE_MODULES] = */ c"GLB_LOAD_MULTIPLE_MODULES".as_ptr(),
    /* [SOF_IPC4_GLB_UNLOAD_MULTIPLE_MODULES] = */ c"GLB_UNLOAD_MULTIPLE_MODULES".as_ptr(),
    /* [SOF_IPC4_GLB_CREATE_PIPELINE] = */ c"GLB_CREATE_PIPELINE".as_ptr(),
    /* [SOF_IPC4_GLB_DELETE_PIPELINE] = */ c"GLB_DELETE_PIPELINE".as_ptr(),
    /* [SOF_IPC4_GLB_SET_PIPELINE_STATE] = */ c"GLB_SET_PIPELINE_STATE".as_ptr(),
    /* [SOF_IPC4_GLB_GET_PIPELINE_STATE] = */ c"GLB_GET_PIPELINE_STATE".as_ptr(),
    /* [SOF_IPC4_GLB_GET_PIPELINE_CONTEXT_SIZE] = */ c"GLB_GET_PIPELINE_CONTEXT_SIZE".as_ptr(),
    /* [SOF_IPC4_GLB_SAVE_PIPELINE] = */ c"GLB_SAVE_PIPELINE".as_ptr(),
    /* [SOF_IPC4_GLB_RESTORE_PIPELINE] = */ c"GLB_RESTORE_PIPELINE".as_ptr(),
    /* [SOF_IPC4_GLB_LOAD_LIBRARY] = */ c"GLB_LOAD_LIBRARY".as_ptr(),
    /* [SOF_IPC4_GLB_LOAD_LIBRARY_PREPARE] = */ c"GLB_LOAD_LIBRARY_PREPARE".as_ptr(),
    /* [SOF_IPC4_GLB_INTERNAL_MESSAGE] = */ c"GLB_INTERNAL_MESSAGE".as_ptr(),
    /* [SOF_IPC4_GLB_NOTIFICATION] = */ c"GLB_NOTIFICATION".as_ptr(),
];

const ipc4_dbg_notification_type: [*const c_char; SOF_IPC4_NOTIFY_TYPE_LAST as usize] = [
    /* [SOF_IPC4_NOTIFY_PHRASE_DETECTED] = */ c"PHRASE_DETECTED".as_ptr(),
    /* [SOF_IPC4_NOTIFY_RESOURCE_EVENT] = */ c"RESOURCE_EVENT".as_ptr(),
    /* [SOF_IPC4_NOTIFY_LOG_BUFFER_STATUS] = */ c"LOG_BUFFER_STATUS".as_ptr(),
    /* [SOF_IPC4_NOTIFY_TIMESTAMP_CAPTURED] = */ c"TIMESTAMP_CAPTURED".as_ptr(),
    /* [SOF_IPC4_NOTIFY_FW_READY] = */ c"FW_READY".as_ptr(),
    /* [SOF_IPC4_NOTIFY_FW_AUD_CLASS_RESULT] = */ c"FW_AUD_CLASS_RESULT".as_ptr(),
    /* [SOF_IPC4_NOTIFY_EXCEPTION_CAUGHT] = */ c"EXCEPTION_CAUGHT".as_ptr(),
    /* [SOF_IPC4_NOTIFY_MODULE_NOTIFICATION] = */ c"MODULE_NOTIFICATION".as_ptr(),
    /* [SOF_IPC4_NOTIFY_PROBE_DATA_AVAILABLE] = */ c"PROBE_DATA_AVAILABLE".as_ptr(),
    /* [SOF_IPC4_NOTIFY_ASYNC_MSG_SRVC_MESSAGE] = */ c"ASYNC_MSG_SRVC_MESSAGE".as_ptr(),
];

unsafe fn sof_ipc4_log_header(
    dev: *mut device,
    text: *mut u8,
    msg: *mut sof_ipc4_msg,
    data_size_valid: bool,
) {
    let val: u32;
    let type_: u32;
    let mut str2: *const u8 = ptr::null();
    let mut str_: *const u8 = ptr::null();

    val = (*msg).primary & SOF_IPC4_MSG_TARGET_MASK;
    type_ = SOF_IPC4_MSG_TYPE_GET((*msg).primary);

    if val == SOF_IPC4_MSG_TARGET(SOF_IPC4_MODULE_MSG) {
        /* Module message */
        if type_ < SOF_IPC4_MOD_TYPE_LAST {
            str_ = ipc4_dbg_mod_msg_type[type_ as usize] as *const u8;
        }
        if str_.is_null() {
            str_ = c"Unknown Module message type".as_ptr() as *const u8;
        }
    } else {
        /* Global FW message */
        if type_ < SOF_IPC4_GLB_TYPE_LAST {
            str_ = ipc4_dbg_glb_msg_type[type_ as usize] as *const u8;
        }
        if str_.is_null() {
            str_ = c"Unknown Global message type".as_ptr() as *const u8;
        }

        if type_ == SOF_IPC4_GLB_NOTIFICATION {
            /* Notification message */
            let notif: u32 = SOF_IPC4_NOTIFICATION_TYPE_GET((*msg).primary);

            /* Do not print log buffer notification if not desired */
            if notif == SOF_IPC4_NOTIFY_LOG_BUFFER_STATUS
                && !sof_debug_check_flag(SOF_DBG_PRINT_DMA_POSITION_UPDATE_LOGS)
            {
                return;
            }

            if notif < SOF_IPC4_NOTIFY_TYPE_LAST {
                str2 = ipc4_dbg_notification_type[notif as usize] as *const u8;
            }
            if str2.is_null() {
                str2 = c"Unknown Global notification".as_ptr() as *const u8;
            }
        }
    }

    if !str2.is_null() {
        if data_size_valid && (*msg).data_size != 0 {
            dev_dbg(
                dev,
                c"%s: %#x|%#x: %s|%s [data size: %zu]\n".as_ptr(),
                text,
                (*msg).primary,
                (*msg).extension,
                str_,
                str2,
                (*msg).data_size,
            );
        } else {
            dev_dbg(
                dev,
                c"%s: %#x|%#x: %s|%s\n".as_ptr(),
                text,
                (*msg).primary,
                (*msg).extension,
                str_,
                str2,
            );
        }
    } else if data_size_valid && (*msg).data_size != 0 {
        dev_dbg(
            dev,
            c"%s: %#x|%#x: %s [data size: %zu]\n".as_ptr(),
            text,
            (*msg).primary,
            (*msg).extension,
            str_,
            (*msg).data_size,
        );
    } else {
        dev_dbg(
            dev,
            c"%s: %#x|%#x: %s\n".as_ptr(),
            text,
            (*msg).primary,
            (*msg).extension,
            str_,
        );
    }
}

#[no_mangle]
pub unsafe extern "C" fn sof_ipc4_pipeline_state_str(
    state: sof_ipc4_pipeline_state,
) -> *const c_char {
    match state {
        SOF_IPC4_PIPE_INVALID_STATE => c" (INVALID_STATE)".as_ptr(),
        SOF_IPC4_PIPE_UNINITIALIZED => c" (UNINITIALIZED)".as_ptr(),
        SOF_IPC4_PIPE_RESET => c" (RESET)".as_ptr(),
        SOF_IPC4_PIPE_PAUSED => c" (PAUSED)".as_ptr(),
        SOF_IPC4_PIPE_RUNNING => c" (RUNNING)".as_ptr(),
        SOF_IPC4_PIPE_EOS => c" (EOS)".as_ptr(),
        _ => c" (<unknown>)".as_ptr(),
    }
}

// #else /* CONFIG_SND_SOC_SOF_DEBUG_VERBOSE_IPC */
// Non-verbose configuration maps sof_ipc4_log_header() to a compact logger and
// sof_ipc4_pipeline_state_str() to "".

unsafe fn sof_ipc4_dump_payload(sdev: *mut snd_sof_dev, ipc_data: *mut c_void, size: usize) {
    print_hex_dump_debug(
        c"Message payload: ".as_ptr(),
        DUMP_PREFIX_OFFSET,
        16,
        4,
        ipc_data,
        size,
        false,
    );
}

unsafe fn sof_ipc4_resource_type_str(type_: u32) -> *const c_char {
    match type_ {
        SOF_IPC4_MODULE_INSTANCE => c"resource: MODULE_INSTANCE".as_ptr(),
        SOF_IPC4_PIPELINE => c"resource: PIPELINE".as_ptr(),
        SOF_IPC4_GATEWAY => c"resource: GATEWAY".as_ptr(),
        SOF_IPC4_EDF_TASK => c"resource: EDF_TASK".as_ptr(),
        SOF_IPC4_INVALID_RESOURCE_TYPE => c"Resource is invalid".as_ptr(),
        _ => c"Unknown resource type".as_ptr(),
    }
}

unsafe fn sof_ipc4_resource_event_type_str(event_type: u32) -> *const c_char {
    match event_type {
        SOF_IPC4_MIXER_UNDERRUN_DETECTED => c"event:    MIXER_UNDERRUN_DETECTED".as_ptr(),
        SOF_IPC4_PROCESS_DATA_ERROR => c"event:    PROCESS_DATA_ERROR".as_ptr(),
        SOF_IPC4_GATEWAY_UNDERRUN_DETECTED => c"event:    GATEWAY_UNDERRUN_DETECTED".as_ptr(),
        SOF_IPC4_GATEWAY_OVERRUN_DETECTED => c"event:    GATEWAY_OVERRUN_DETECTED".as_ptr(),
        _ => c"Unknown event type".as_ptr(),
    }
}

unsafe extern "C" fn sof_ipc4_resource_event_handler(
    sdev: *mut snd_sof_dev,
    ipc4_msg: *mut sof_ipc4_msg,
) {
    let data = (*ipc4_msg).data_ptr as *mut sof_ipc4_notify_resource_data;

    /* Print event details */
    match (*data).event_type {
        SOF_IPC4_MIXER_UNDERRUN_DETECTED => {
            dev_dbg(
                (*sdev).dev,
                c"%s (%u): eos %u, mixed %u, expected %u\n".as_ptr(),
                sof_ipc4_resource_event_type_str((*data).event_type),
                (*data).event_type,
                (*data).data.mixer_underrun.eos_flag,
                (*data).data.mixer_underrun.data_mixed,
                (*data).data.mixer_underrun.expected_data_mixed,
            );
        }
        SOF_IPC4_PROCESS_DATA_ERROR => {
            dev_dbg(
                (*sdev).dev,
                c"%s (%u): error_code %#x\n".as_ptr(),
                sof_ipc4_resource_event_type_str((*data).event_type),
                (*data).event_type,
                (*data).data.process_data_error.error_code,
            );
        }
        SOF_IPC4_GATEWAY_UNDERRUN_DETECTED | SOF_IPC4_GATEWAY_OVERRUN_DETECTED => {
            dev_dbg(
                (*sdev).dev,
                c"%s (%u)\n".as_ptr(),
                sof_ipc4_resource_event_type_str((*data).event_type),
                (*data).event_type,
            );
        }
        _ => {
            dev_dbg(
                (*sdev).dev,
                c"%s (%u): raw dws %#x %#x %#x %#x %#x %#x\n".as_ptr(),
                sof_ipc4_resource_event_type_str((*data).event_type),
                (*data).event_type,
                (*data).data.dws[0],
                (*data).data.dws[1],
                (*data).data.dws[2],
                (*data).data.dws[3],
                (*data).data.dws[4],
                (*data).data.dws[5],
            );
        }
    }

    /* Print resource details */
    if (*data).resource_type == SOF_IPC4_MODULE_INSTANCE {
        let module_id: u32 = SOF_IPC4_MOD_ID_GET((*data).resource_id);
        let instance_id: u32 = SOF_IPC4_MOD_INSTANCE_GET((*data).resource_id);

        dev_dbg(
            (*sdev).dev,
            c"%s (%u), module_id %u, instance_id %u\n".as_ptr(),
            sof_ipc4_resource_type_str((*data).resource_type),
            (*data).resource_type,
            module_id,
            instance_id,
        );
    } else if (*data).resource_type != SOF_IPC4_INVALID_RESOURCE_TYPE {
        dev_dbg(
            (*sdev).dev,
            c"%s (%u), id %u\n".as_ptr(),
            sof_ipc4_resource_type_str((*data).resource_type),
            (*data).resource_type,
            (*data).resource_id,
        );
    }
}

unsafe fn sof_ipc4_get_reply(sdev: *mut snd_sof_dev) -> c_int {
    let msg: *mut snd_sof_ipc_msg = (*sdev).msg;
    let ipc4_reply: *mut sof_ipc4_msg;
    let ret: c_int;

    /* get the generic reply */
    ipc4_reply = (*msg).reply_data as *mut sof_ipc4_msg;

    sof_ipc4_log_header((*sdev).dev, c"ipc tx reply".as_ptr() as *mut u8, ipc4_reply, false);

    ret = sof_ipc4_check_reply_status(sdev, (*ipc4_reply).primary);
    if ret != 0 {
        return ret;
    }

    /* No other information is expected for non large config get replies */
    if (*msg).reply_size == 0
        || !SOF_IPC4_MSG_IS_MODULE_MSG((*ipc4_reply).primary)
        || SOF_IPC4_MSG_TYPE_GET((*ipc4_reply).primary) != SOF_IPC4_MOD_LARGE_CONFIG_GET
    {
        return 0;
    }

    /* Read the requested payload */
    snd_sof_dsp_mailbox_read(
        sdev,
        (*sdev).dsp_box.offset,
        (*ipc4_reply).data_ptr,
        (*msg).reply_size,
    );

    0
}

/* wait for IPC message reply */
unsafe fn ipc4_wait_tx_done(ipc: *mut snd_sof_ipc, reply_data: *mut c_void) -> c_int {
    let msg: *mut snd_sof_ipc_msg = &mut (*ipc).msg;
    let ipc4_msg = (*msg).msg_data as *mut sof_ipc4_msg;
    let sdev: *mut snd_sof_dev = (*ipc).sdev;
    let mut ret: c_int;

    /* wait for DSP IPC completion */
    ret = wait_event_timeout(
        &mut (*msg).waitq,
        (*msg).ipc_complete,
        msecs_to_jiffies((*sdev).ipc_timeout),
    );
    if ret == 0 {
        dev_err(
            (*sdev).dev,
            c"ipc timed out for %#x|%#x\n".as_ptr(),
            (*ipc4_msg).primary,
            (*ipc4_msg).extension,
        );
        snd_sof_handle_fw_exception((*ipc).sdev, c"IPC timeout".as_ptr());
        return -ETIMEDOUT;
    }

    if (*msg).reply_error != 0 {
        dev_err(
            (*sdev).dev,
            c"ipc error for msg %#x|%#x\n".as_ptr(),
            (*ipc4_msg).primary,
            (*ipc4_msg).extension,
        );
        ret = (*msg).reply_error;
    } else {
        if !reply_data.is_null() {
            let ipc4_reply = (*msg).reply_data as *mut sof_ipc4_msg;
            let ipc4_reply_data = reply_data as *mut sof_ipc4_msg;

            /* Copy the header */
            (*ipc4_reply_data).header_u64 = (*ipc4_reply).header_u64;
            if (*msg).reply_size != 0 && !(*ipc4_reply_data).data_ptr.is_null() {
                /* copy the payload returned from DSP */
                memcpy(
                    (*ipc4_reply_data).data_ptr,
                    (*ipc4_reply).data_ptr,
                    (*msg).reply_size,
                );
                (*ipc4_reply_data).data_size = (*msg).reply_size;
            }
        }

        ret = 0;
        sof_ipc4_log_header((*sdev).dev, c"ipc tx done ".as_ptr() as *mut u8, ipc4_msg, true);
    }

    /* re-enable dumps after successful IPC tx */
    if (*sdev).ipc_dump_printed {
        (*sdev).dbg_dump_printed = false;
        (*sdev).ipc_dump_printed = false;
    }

    ret
}

unsafe fn ipc4_tx_msg_unlocked(
    ipc: *mut snd_sof_ipc,
    msg_data: *mut c_void,
    msg_bytes: usize,
    reply_data: *mut c_void,
    reply_bytes: usize,
) -> c_int {
    let ipc4_msg = msg_data as *mut sof_ipc4_msg;
    let sdev: *mut snd_sof_dev = (*ipc).sdev;
    let mut ret: c_int;

    if msg_bytes > (*ipc).max_payload_size || reply_bytes > (*ipc).max_payload_size {
        return -EINVAL;
    }

    sof_ipc4_log_header((*sdev).dev, c"ipc tx      ".as_ptr() as *mut u8, msg_data as *mut sof_ipc4_msg, true);

    ret = sof_ipc_send_msg(sdev, msg_data, msg_bytes, reply_bytes);
    if ret != 0 {
        dev_err_ratelimited(
            (*sdev).dev,
            c"%s: ipc message send for %#x|%#x failed: %d\n".as_ptr(),
            c"ipc4_tx_msg_unlocked".as_ptr(),
            (*ipc4_msg).primary,
            (*ipc4_msg).extension,
            ret,
        );
        return ret;
    }

    /* now wait for completion */
    ipc4_wait_tx_done(ipc, reply_data)
}

unsafe extern "C" fn sof_ipc4_tx_msg(
    sdev: *mut snd_sof_dev,
    msg_data: *mut c_void,
    msg_bytes: usize,
    reply_data: *mut c_void,
    reply_bytes: usize,
    no_pm: bool,
) -> c_int {
    let ipc: *mut snd_sof_ipc = (*sdev).ipc;
    let mut ret: c_int;

    if msg_data.is_null() {
        return -EINVAL;
    }

    if !no_pm {
        let target_state = sof_dsp_power_state {
            state: SOF_DSP_PM_D0,
        };

        /* ensure the DSP is in D0i0 before sending a new IPC */
        ret = snd_sof_dsp_set_power_state(sdev, &target_state);
        if ret < 0 {
            return ret;
        }
    }

    /* Serialise IPC TX */
    mutex_lock(&mut (*ipc).tx_mutex);
    ret = ipc4_tx_msg_unlocked(ipc, msg_data, msg_bytes, reply_data, reply_bytes);
    mutex_unlock(&mut (*ipc).tx_mutex);

    if sof_debug_check_flag(SOF_DBG_DUMP_IPC_MESSAGE_PAYLOAD) {
        let mut msg: *mut sof_ipc4_msg = ptr::null_mut();

        /* payload is indicated by non zero msg/reply_bytes */
        if msg_bytes != 0 {
            msg = msg_data as *mut sof_ipc4_msg;
        } else if reply_bytes != 0 {
            msg = reply_data as *mut sof_ipc4_msg;
        }

        if !msg.is_null() {
            sof_ipc4_dump_payload(sdev, (*msg).data_ptr, (*msg).data_size);
        }
    }

    ret
}

unsafe fn sof_ipc4_tx_payload_for_get_data(tx: *mut sof_ipc4_msg) -> bool {
    /*
     * Messages that require TX payload with LARGE_CONFIG_GET.
     * The TX payload is placed into the IPC message data section by caller,
     * which needs to be copied to temporary buffer since the received data
     * will overwrite it.
     */
    match (*tx).extension & SOF_IPC4_MOD_EXT_MSG_PARAM_ID_MASK {
        x if x == SOF_IPC4_MOD_EXT_MSG_PARAM_ID(SOF_IPC4_SWITCH_CONTROL_PARAM_ID) => true,
        x if x == SOF_IPC4_MOD_EXT_MSG_PARAM_ID(SOF_IPC4_ENUM_CONTROL_PARAM_ID) => true,
        x if x == SOF_IPC4_MOD_EXT_MSG_PARAM_ID(SOF_IPC4_BYTES_CONTROL_PARAM_ID) => true,
        _ => false,
    }
}

unsafe extern "C" fn sof_ipc4_set_get_data(
    sdev: *mut snd_sof_dev,
    data: *mut c_void,
    payload_bytes: usize,
    set: bool,
) -> c_int {
    let target_state = sof_dsp_power_state {
        state: SOF_DSP_PM_D0,
    };
    let payload_limit: usize = (*(*sdev).ipc).max_payload_size;
    let ipc4_msg = data as *mut sof_ipc4_msg;
    let mut tx: sof_ipc4_msg = core::mem::zeroed();
    let mut rx: sof_ipc4_msg = core::mem::zeroed();
    let mut remaining: usize = payload_bytes;
    let mut tx_payload_for_get: *mut c_void = ptr::null_mut();
    let mut tx_data_size: usize = 0;
    let mut offset: usize = 0;
    let mut chunk_size: usize;
    let mut ret: c_int;

    if data.is_null() {
        return -EINVAL;
    }

    if ((*ipc4_msg).primary & SOF_IPC4_MSG_TARGET_MASK) != SOF_IPC4_MSG_TARGET(SOF_IPC4_MODULE_MSG)
    {
        return -EINVAL;
    }

    (*ipc4_msg).primary &= !SOF_IPC4_MSG_TYPE_MASK;
    tx.primary = (*ipc4_msg).primary;
    tx.extension = (*ipc4_msg).extension;

    if set {
        tx.primary |= SOF_IPC4_MSG_TYPE_SET(SOF_IPC4_MOD_LARGE_CONFIG_SET);
    } else {
        tx.primary |= SOF_IPC4_MSG_TYPE_SET(SOF_IPC4_MOD_LARGE_CONFIG_GET);
    }

    tx.extension &= !SOF_IPC4_MOD_EXT_MSG_SIZE_MASK;
    tx.extension |= SOF_IPC4_MOD_EXT_MSG_SIZE(payload_bytes);

    tx.extension |= SOF_IPC4_MOD_EXT_MSG_FIRST_BLOCK(1);

    if sof_ipc4_tx_payload_for_get_data(&mut tx) {
        tx_data_size = min((*ipc4_msg).data_size, payload_limit);
        tx_payload_for_get = kmemdup((*ipc4_msg).data_ptr, tx_data_size, GFP_KERNEL);
        if tx_payload_for_get.is_null() {
            return -ENOMEM;
        }
    }

    /* ensure the DSP is in D0i0 before sending IPC */
    ret = snd_sof_dsp_set_power_state(sdev, &target_state);
    if ret < 0 {
        kfree(tx_payload_for_get);
        return ret;
    }

    /* Serialise IPC TX */
    mutex_lock(&mut (*(*sdev).ipc).tx_mutex);

    loop {
        let tx_size: usize;
        let mut rx_size: usize;

        if remaining > payload_limit {
            chunk_size = payload_limit;
        } else {
            chunk_size = remaining;
            if set {
                tx.extension |= SOF_IPC4_MOD_EXT_MSG_LAST_BLOCK(1);
            }
        }

        if offset != 0 {
            tx.extension &= !SOF_IPC4_MOD_EXT_MSG_FIRST_BLOCK_MASK;
            tx.extension &= !SOF_IPC4_MOD_EXT_MSG_SIZE_MASK;
            tx.extension |= SOF_IPC4_MOD_EXT_MSG_SIZE(offset);
        }

        if set {
            tx.data_size = chunk_size;
            tx.data_ptr = (*ipc4_msg).data_ptr.add(offset);

            tx_size = chunk_size;
            rx_size = 0;
        } else {
            rx.primary = 0;
            rx.extension = 0;
            rx.data_size = chunk_size;
            rx.data_ptr = (*ipc4_msg).data_ptr.add(offset);

            if !tx_payload_for_get.is_null() {
                tx_size = tx_data_size;
                tx.data_size = tx_size;
                tx.data_ptr = tx_payload_for_get;
            } else {
                tx_size = 0;
                tx.data_size = 0;
                tx.data_ptr = ptr::null_mut();
            }
            rx_size = chunk_size;
        }

        /* Send the message for the current chunk */
        ret = ipc4_tx_msg_unlocked(
            (*sdev).ipc,
            &mut tx as *mut sof_ipc4_msg as *mut c_void,
            tx_size,
            &mut rx as *mut sof_ipc4_msg as *mut c_void,
            rx_size,
        );
        if ret < 0 {
            dev_err(
                (*sdev).dev,
                c"%s: large config %s failed at offset %zu: %d\n".as_ptr(),
                c"sof_ipc4_set_get_data".as_ptr(),
                if set { c"set".as_ptr() } else { c"get".as_ptr() },
                offset,
                ret,
            );
            break;
        }

        if !set && (rx.extension & SOF_IPC4_MOD_EXT_MSG_FIRST_BLOCK_MASK) != 0 {
            /* Verify the firmware reported total payload size */
            rx_size = (rx.extension & SOF_IPC4_MOD_EXT_MSG_SIZE_MASK) as usize;

            if rx_size > payload_bytes {
                dev_err(
                    (*sdev).dev,
                    c"%s: Receive buffer (%zu) is too small for %zu\n".as_ptr(),
                    c"sof_ipc4_set_get_data".as_ptr(),
                    payload_bytes,
                    rx_size,
                );
                ret = -ENOMEM;
                break;
            }

            if rx_size < chunk_size {
                chunk_size = rx_size;
                remaining = rx_size;
            } else if rx_size < payload_bytes {
                remaining = rx_size;
            }
        }

        offset += chunk_size;
        remaining -= chunk_size;
        if remaining == 0 {
            break;
        }
    }

    mutex_unlock(&mut (*(*sdev).ipc).tx_mutex);

    /* Adjust the received data size if needed */
    if !set && payload_bytes != offset {
        (*ipc4_msg).data_size = offset;
    }

    if sof_debug_check_flag(SOF_DBG_DUMP_IPC_MESSAGE_PAYLOAD) {
        sof_ipc4_dump_payload(sdev, (*ipc4_msg).data_ptr, (*ipc4_msg).data_size);
    }

    kfree(tx_payload_for_get);

    ret
}

unsafe fn sof_ipc4_init_msg_memory(sdev: *mut snd_sof_dev) -> c_int {
    let ipc4_msg: *mut sof_ipc4_msg;
    let msg: *mut snd_sof_ipc_msg = &mut (*(*sdev).ipc).msg;

    /* TODO: get max_payload_size from firmware */
    (*(*sdev).ipc).max_payload_size = SOF_IPC4_MSG_MAX_SIZE;

    /* Allocate memory for the ipc4 container and the maximum payload */
    (*msg).reply_data = devm_kzalloc(
        (*sdev).dev,
        (*(*sdev).ipc).max_payload_size + size_of::<sof_ipc4_msg>(),
        GFP_KERNEL,
    );
    if (*msg).reply_data.is_null() {
        return -ENOMEM;
    }

    ipc4_msg = (*msg).reply_data as *mut sof_ipc4_msg;
    (*ipc4_msg).data_ptr = ((*msg).reply_data as *mut u8).add(size_of::<sof_ipc4_msg>()) as *mut c_void;

    0
}

#[no_mangle]
pub unsafe extern "C" fn sof_ipc4_find_debug_slot_offset_by_type(
    sdev: *mut snd_sof_dev,
    slot_type: u32,
) -> usize {
    let mut slot_desc_type_offset: usize;
    let mut type_: u32 = 0;
    let mut i: c_int;

    /* The type is the second u32 in the slot descriptor */
    slot_desc_type_offset = (*sdev).debug_box.offset + size_of::<u32>();
    i = 0;
    while i < SOF_IPC4_MAX_DEBUG_SLOTS {
        sof_mailbox_read(
            sdev,
            slot_desc_type_offset,
            &mut type_ as *mut u32 as *mut c_void,
            size_of::<u32>(),
        );

        if type_ == slot_type {
            return (*sdev).debug_box.offset + (i as usize + 1) * SOF_IPC4_DEBUG_SLOT_SIZE;
        }

        slot_desc_type_offset += SOF_IPC4_DEBUG_DESCRIPTOR_SIZE;
        i += 1;
    }

    dev_dbg(
        (*sdev).dev,
        c"Slot type %#x is not available in debug window\n".as_ptr(),
        slot_type,
    );
    0
}
// EXPORT_SYMBOL(sof_ipc4_find_debug_slot_offset_by_type);

unsafe fn ipc4_fw_ready(sdev: *mut snd_sof_dev, ipc4_msg: *mut sof_ipc4_msg) -> c_int {
    if !(*sdev).first_boot {
        let ipc4_data = (*sdev).private as *mut sof_ipc4_fw_data;

        /*
         * After the initial boot only check if the libraries have been
         * restored when full context save is not enabled
         */
        if !(*ipc4_data).fw_context_save {
            (*ipc4_data).libraries_restored =
                ((*ipc4_msg).primary & SOF_IPC4_FW_READY_LIB_RESTORED) != 0;
        }

        return 0;
    }

    sof_ipc4_create_exception_debugfs_node(sdev);

    sof_ipc4_init_msg_memory(sdev)
}

unsafe extern "C" fn sof_ipc4_module_notification_handler(
    sdev: *mut snd_sof_dev,
    ipc4_msg: *mut sof_ipc4_msg,
) {
    let mut data = (*ipc4_msg).data_ptr as *mut sof_ipc4_notify_module_data;

    /*
     * If the notification includes additional, module specific data, then
     * we need to re-allocate the buffer and re-read the whole payload,
     * including the event_data
     */
    if (*data).event_data_size != 0 {
        let new: *mut c_void;
        let ret: c_int;

        (*ipc4_msg).data_size += (*data).event_data_size as usize;

        new = krealloc((*ipc4_msg).data_ptr, (*ipc4_msg).data_size, GFP_KERNEL);
        if new.is_null() {
            (*ipc4_msg).data_size -= (*data).event_data_size as usize;
            return;
        }

        /* re-read the whole payload */
        (*ipc4_msg).data_ptr = new;
        ret = snd_sof_ipc_msg_data(sdev, ptr::null_mut(), (*ipc4_msg).data_ptr, (*ipc4_msg).data_size);
        if ret < 0 {
            dev_err(
                (*sdev).dev,
                c"Failed to read the full module notification: %d\n".as_ptr(),
                ret,
            );
            return;
        }
        data = (*ipc4_msg).data_ptr as *mut sof_ipc4_notify_module_data;
    }

    /* Handle ALSA kcontrol notification */
    if ((*data).event_id & SOF_IPC4_NOTIFY_MODULE_EVENTID_ALSA_MAGIC_MASK)
        == SOF_IPC4_NOTIFY_MODULE_EVENTID_ALSA_MAGIC_VAL
    {
        let tplg_ops: *const sof_ipc_tplg_ops = (*(*(*sdev).ipc).ops).tplg;

        if (*(*tplg_ops).control).update.is_some() {
            ((*(*tplg_ops).control).update.unwrap())(sdev, ipc4_msg);
        }
    }
}

unsafe extern "C" fn sof_ipc4_rx_msg(sdev: *mut snd_sof_dev) {
    let ipc4_msg = (*(*sdev).ipc).msg.rx_data as *mut sof_ipc4_msg;
    let mut handler_func: ipc4_notification_handler = None;
    let mut data_size: usize = 0;
    let mut err: c_int;

    if ipc4_msg.is_null() || !SOF_IPC4_MSG_IS_NOTIFICATION((*ipc4_msg).primary) {
        return;
    }

    (*ipc4_msg).data_ptr = ptr::null_mut();
    (*ipc4_msg).data_size = 0;

    sof_ipc4_log_header((*sdev).dev, c"ipc rx      ".as_ptr() as *mut u8, ipc4_msg, false);

    match SOF_IPC4_NOTIFICATION_TYPE_GET((*ipc4_msg).primary) {
        SOF_IPC4_NOTIFY_FW_READY => {
            /* check for FW boot completion */
            if (*sdev).fw_state == SOF_FW_BOOT_IN_PROGRESS {
                err = ipc4_fw_ready(sdev, ipc4_msg);
                if err < 0 {
                    sof_set_fw_state(sdev, SOF_FW_BOOT_READY_FAILED);
                } else {
                    sof_set_fw_state(sdev, SOF_FW_BOOT_READY_OK);
                }

                /* wake up firmware loader */
                wake_up(&mut (*sdev).boot_wait);
            }
        }
        SOF_IPC4_NOTIFY_RESOURCE_EVENT => {
            data_size = size_of::<sof_ipc4_notify_resource_data>();
            handler_func = Some(sof_ipc4_resource_event_handler);
        }
        SOF_IPC4_NOTIFY_LOG_BUFFER_STATUS => {
            sof_ipc4_mtrace_update_pos(sdev, SOF_IPC4_LOG_CORE_GET((*ipc4_msg).primary));
        }
        SOF_IPC4_NOTIFY_EXCEPTION_CAUGHT => {
            snd_sof_dsp_panic(sdev, 0, true);
        }
        SOF_IPC4_NOTIFY_MODULE_NOTIFICATION => {
            data_size = size_of::<sof_ipc4_notify_module_data>();
            handler_func = Some(sof_ipc4_module_notification_handler);
        }
        _ => {
            dev_dbg(
                (*sdev).dev,
                c"Unhandled DSP message: %#x|%#x\n".as_ptr(),
                (*ipc4_msg).primary,
                (*ipc4_msg).extension,
            );
        }
    }

    if data_size != 0 {
        (*ipc4_msg).data_ptr = kmalloc(data_size, GFP_KERNEL);
        if (*ipc4_msg).data_ptr.is_null() {
            return;
        }

        (*ipc4_msg).data_size = data_size;
        err = snd_sof_ipc_msg_data(sdev, ptr::null_mut(), (*ipc4_msg).data_ptr, (*ipc4_msg).data_size);
        if err < 0 {
            dev_err((*sdev).dev, c"failed to read IPC notification data: %d\n".as_ptr(), err);
            kfree((*ipc4_msg).data_ptr);
            (*ipc4_msg).data_ptr = ptr::null_mut();
            (*ipc4_msg).data_size = 0;
            return;
        }
    }

    /* Handle notifications with payload */
    if let Some(handler) = handler_func {
        handler(sdev, ipc4_msg);
    }

    sof_ipc4_log_header((*sdev).dev, c"ipc rx done ".as_ptr() as *mut u8, ipc4_msg, true);

    if data_size != 0 {
        if sof_debug_check_flag(SOF_DBG_DUMP_IPC_MESSAGE_PAYLOAD) {
            sof_ipc4_dump_payload(sdev, (*ipc4_msg).data_ptr, (*ipc4_msg).data_size);
        }

        kfree((*ipc4_msg).data_ptr);
        (*ipc4_msg).data_ptr = ptr::null_mut();
        (*ipc4_msg).data_size = 0;
    }
}

unsafe extern "C" fn sof_ipc4_set_core_state(
    sdev: *mut snd_sof_dev,
    core_idx: c_int,
    on: bool,
) -> c_int {
    let mut dx_state: sof_ipc4_dx_state_info = core::mem::zeroed();
    let mut msg: sof_ipc4_msg = core::mem::zeroed();

    dx_state.core_mask = BIT(core_idx);
    if on {
        dx_state.dx_mask = BIT(core_idx);
    } else {
        dx_state.dx_mask = 0;
    }

    msg.primary = SOF_IPC4_MSG_TYPE_SET(SOF_IPC4_MOD_SET_DX);
    msg.primary |= SOF_IPC4_MSG_DIR(SOF_IPC4_MSG_REQUEST);
    msg.primary |= SOF_IPC4_MSG_TARGET(SOF_IPC4_MODULE_MSG);
    msg.extension = 0;
    msg.data_ptr = &mut dx_state as *mut sof_ipc4_dx_state_info as *mut c_void;
    msg.data_size = size_of::<sof_ipc4_dx_state_info>();

    sof_ipc4_tx_msg(
        sdev,
        &mut msg as *mut sof_ipc4_msg as *mut c_void,
        msg.data_size,
        ptr::null_mut(),
        0,
        false,
    )
}

/*
 * The context save callback is used to send a message to the firmware notifying
 * it that the primary core is going to be turned off, which is used as an
 * indication to prepare for a full power down, thus preparing for IMR boot
 * (when supported)
 *
 * Note: in IPC4 there is no message used to restore context, thus no context
 * restore callback is implemented
 */
unsafe extern "C" fn sof_ipc4_ctx_save(sdev: *mut snd_sof_dev) -> c_int {
    sof_ipc4_set_core_state(sdev, SOF_DSP_PRIMARY_CORE, false)
}

unsafe extern "C" fn sof_ipc4_set_pm_gate(sdev: *mut snd_sof_dev, flags: u32) -> c_int {
    let mut msg: sof_ipc4_msg = core::mem::zeroed();

    msg.primary = SOF_IPC4_MSG_TYPE_SET(SOF_IPC4_MOD_SET_D0IX);
    msg.primary |= SOF_IPC4_MSG_DIR(SOF_IPC4_MSG_REQUEST);
    msg.primary |= SOF_IPC4_MSG_TARGET(SOF_IPC4_MODULE_MSG);
    msg.extension = flags;

    sof_ipc4_tx_msg(
        sdev,
        &mut msg as *mut sof_ipc4_msg as *mut c_void,
        0,
        ptr::null_mut(),
        0,
        true,
    )
}

static ipc4_pm_ops: sof_ipc_pm_ops = sof_ipc_pm_ops {
    ctx_save: Some(sof_ipc4_ctx_save),
    set_core_state: Some(sof_ipc4_set_core_state),
    set_pm_gate: Some(sof_ipc4_set_pm_gate),
};

unsafe extern "C" fn sof_ipc4_init(sdev: *mut snd_sof_dev) -> c_int {
    let ipc4_data = (*sdev).private as *mut sof_ipc4_fw_data;
    let inbox_offset: c_int;

    mutex_init(&mut (*ipc4_data).pipeline_state_mutex);

    xa_init_flags(&mut (*ipc4_data).fw_lib_xa, XA_FLAGS_ALLOC);

    /* Set up the windows for IPC communication */
    inbox_offset = snd_sof_dsp_get_mailbox_offset(sdev);
    if inbox_offset < 0 {
        dev_err((*sdev).dev, c"%s: No mailbox offset\n".as_ptr(), c"sof_ipc4_init".as_ptr());
        return inbox_offset;
    }

    (*sdev).dsp_box.offset = inbox_offset as usize;
    (*sdev).dsp_box.size = SOF_IPC4_MSG_MAX_SIZE;
    (*sdev).host_box.offset = snd_sof_dsp_get_window_offset(sdev, SOF_IPC4_OUTBOX_WINDOW_IDX) as usize;
    (*sdev).host_box.size = SOF_IPC4_MSG_MAX_SIZE;

    (*sdev).debug_box.offset = snd_sof_dsp_get_window_offset(sdev, SOF_IPC4_DEBUG_WINDOW_IDX) as usize;

    (*sdev).fw_info_box.offset = snd_sof_dsp_get_window_offset(sdev, SOF_IPC4_INBOX_WINDOW_IDX) as usize;
    (*sdev).fw_info_box.size = size_of::<sof_ipc4_fw_registers>();

    dev_dbg(
        (*sdev).dev,
        c"mailbox upstream %#x - size %#x\n".as_ptr(),
        (*sdev).dsp_box.offset,
        SOF_IPC4_MSG_MAX_SIZE,
    );
    dev_dbg(
        (*sdev).dev,
        c"mailbox downstream %#x - size %#x\n".as_ptr(),
        (*sdev).host_box.offset,
        SOF_IPC4_MSG_MAX_SIZE,
    );
    dev_dbg((*sdev).dev, c"debug box %#x\n".as_ptr(), (*sdev).debug_box.offset);

    0
}

unsafe extern "C" fn sof_ipc4_exit(sdev: *mut snd_sof_dev) {
    let ipc4_data = (*sdev).private as *mut sof_ipc4_fw_data;
    let mut fw_lib: *mut sof_ipc4_fw_library = ptr::null_mut();
    let mut lib_id: c_ulong = 0;

    xa_for_each(&mut (*ipc4_data).fw_lib_xa, &mut lib_id, &mut fw_lib, {
        /*
         * The basefw (ID == 0) is handled by generic code, it is not
         * loaded by IPC4 code.
         */
        if lib_id != 0 {
            release_firmware((*fw_lib).sof_fw.fw);
        }

        (*fw_lib).sof_fw.fw = ptr::null_mut();
    });

    xa_destroy(&mut (*ipc4_data).fw_lib_xa);
}

unsafe extern "C" fn sof_ipc4_post_boot(sdev: *mut snd_sof_dev) -> c_int {
    if (*sdev).first_boot {
        let ret: c_int = sof_ipc4_complete_split_release(sdev);

        if ret != 0 {
            return ret;
        }

        return sof_ipc4_query_fw_configuration(sdev);
    }

    sof_ipc4_reload_fw_libraries(sdev)
}

#[no_mangle]
pub static ipc4_ops: sof_ipc_ops = sof_ipc_ops {
    init: Some(sof_ipc4_init),
    exit: Some(sof_ipc4_exit),
    post_fw_boot: Some(sof_ipc4_post_boot),
    tx_msg: Some(sof_ipc4_tx_msg),
    rx_msg: Some(sof_ipc4_rx_msg),
    set_get_data: Some(sof_ipc4_set_get_data),
    get_reply: Some(sof_ipc4_get_reply),
    pm: &ipc4_pm_ops,
    fw_loader: unsafe { &ipc4_loader_ops },
    tplg: unsafe { &ipc4_tplg_ops },
    pcm: unsafe { &ipc4_pcm_ops },
    fw_tracing: unsafe { &ipc4_mtrace_ops },
};

#[no_mangle]
pub unsafe extern "C" fn sof_ipc4_mic_privacy_state_change(
    sdev: *mut snd_sof_dev,
    state: bool,
) {
    let mut msg: sof_ipc4_msg = core::mem::zeroed();
    let mut data: u32 = state as u32;

    /*
     * The mic privacy change notification's role is to notify the running
     * firmware that there is a change in mic privacy state from whatever
     * the state was before - since the firmware booted up or since the
     * previous change during runtime.
     *
     * If the firmware has not been booted up, there is no need to send
     * change notification (the firmware is not booted up).
     * The firmware checks the current state during its boot.
     */
    if (*sdev).fw_state != SOF_FW_BOOT_COMPLETE {
        return;
    }

    msg.primary = SOF_IPC4_MSG_TARGET(SOF_IPC4_MODULE_MSG);
    msg.primary |= SOF_IPC4_MSG_DIR(SOF_IPC4_MSG_REQUEST);
    msg.primary |= SOF_IPC4_MOD_ID(SOF_IPC4_MOD_INIT_BASEFW_MOD_ID);
    msg.primary |= SOF_IPC4_MOD_INSTANCE(SOF_IPC4_MOD_INIT_BASEFW_INSTANCE_ID);
    msg.extension = SOF_IPC4_MOD_EXT_MSG_PARAM_ID(SOF_IPC4_FW_PARAM_MIC_PRIVACY_STATE_CHANGE);

    msg.data_size = size_of::<u32>();
    msg.data_ptr = &mut data as *mut u32 as *mut c_void;

    sof_ipc4_set_get_data(
        sdev,
        &mut msg as *mut sof_ipc4_msg as *mut c_void,
        msg.data_size,
        true,
    );
}
// EXPORT_SYMBOL(sof_ipc4_mic_privacy_state_change);

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
