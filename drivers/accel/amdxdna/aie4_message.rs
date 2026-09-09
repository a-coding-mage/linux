// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (C) 2026, Advanced Micro Devices, Inc.
 */

// C dependencies and build-provided declarations are supplied by the surrounding
// translation unit.

pub unsafe fn aie4_suspend_fw(ndev: *mut amdxdna_dev_hdl) -> i32 {
    // C: DECLARE_AIE_MSG(aie4_msg_suspend, AIE4_MSG_OP_SUSPEND);
    DECLARE_AIE_MSG!(aie4_msg_suspend, AIE4_MSG_OP_SUSPEND);
    let ret: i32;

    ret = aie_send_mgmt_msg_wait(&mut (*ndev).aie, &mut msg);
    if ret != 0 {
        XDNA_ERR!((*ndev).aie.xdna, "Failed to suspend fw, ret %d", ret);
    }

    ret
}

pub unsafe fn aie4_query_aie_metadata(
    ndev: *mut amdxdna_dev_hdl,
    metadata: *mut amdxdna_drm_query_aie_metadata,
) -> i32 {
    // C: DECLARE_AIE_MSG(aie4_msg_aie4_tile_info, AIE4_MSG_OP_AIE_TILE_INFO);
    DECLARE_AIE_MSG!(aie4_msg_aie4_tile_info, AIE4_MSG_OP_AIE_TILE_INFO);
    let ret: i32;

    ret = aie_send_mgmt_msg_wait(&mut (*ndev).aie, &mut msg);
    if ret != 0 {
        return ret;
    }

    (*metadata).col_size = resp.info.size;
    (*metadata).cols = resp.info.cols;
    (*metadata).rows = resp.info.rows;

    (*metadata).version.major = resp.info.major;
    (*metadata).version.minor = resp.info.minor;

    (*metadata).core.row_count = resp.info.core_rows;
    (*metadata).core.row_start = resp.info.core_row_start;
    (*metadata).core.dma_channel_count = resp.info.core_dma_channels;
    (*metadata).core.lock_count = resp.info.core_locks;
    (*metadata).core.event_reg_count = resp.info.core_events;

    (*metadata).mem.row_count = resp.info.mem_rows;
    (*metadata).mem.row_start = resp.info.mem_row_start;
    (*metadata).mem.dma_channel_count = resp.info.mem_dma_channels;
    (*metadata).mem.lock_count = resp.info.mem_locks;
    (*metadata).mem.event_reg_count = resp.info.mem_events;

    (*metadata).shim.row_count = resp.info.shim_rows;
    (*metadata).shim.row_start = resp.info.shim_row_start;
    (*metadata).shim.dma_channel_count = resp.info.shim_dma_channels;
    (*metadata).shim.lock_count = resp.info.shim_locks;
    (*metadata).shim.event_reg_count = resp.info.shim_events;

    0
}

pub unsafe fn aie4_attach_work_buffer(ndev: *mut amdxdna_dev_hdl) -> i32 {
    // C: DECLARE_AIE_MSG(aie4_msg_attach_work_buffer, AIE4_MSG_OP_ATTACH_WORK_BUFFER);
    DECLARE_AIE_MSG!(aie4_msg_attach_work_buffer, AIE4_MSG_OP_ATTACH_WORK_BUFFER);
    let xdna: *mut amdxdna_dev = (*ndev).aie.xdna;
    let ret: i32;

    req.buff_addr = (*ndev).work_buf_addr;
    req.buff_size = AIE4_WORK_BUFFER_MIN_SIZE;

    ret = aie_send_mgmt_msg_wait(&mut (*ndev).aie, &mut msg);
    if ret != 0 {
        XDNA_ERR!(xdna, "Failed to attach work buffer, ret %d", ret);
    } else {
        XDNA_DBG!(xdna, "Attached work buffer");
    }

    ret
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
