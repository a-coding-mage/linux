// SPDX-License-Identifier: GPL-2.0-only
/*
 *  sst_stream.c - Intel SST Driver for audio engine
 *
 *  Copyright (C) 2008-14 Intel Corp
 *  Authors:	Vinod Koul <vinod.koul@intel.com>
 *		Harsha Priya <priya.harsha@intel.com>
 *		Dharageswari R <dharageswari.r@intel.com>
 *		KP Jeeja <jeeja.kp@intel.com>
 *  ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
 *
 * ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
 */

// Original C dependencies:
// <linux/pci.h>, <linux/firmware.h>, <linux/sched.h>, <linux/delay.h>,
// <sound/core.h>, <sound/pcm.h>, <sound/soc.h>, <sound/compress_driver.h>,
// <asm/platform_sst_audio.h>, "../sst-mfld-platform.h", "sst.h"

use core::ffi::c_void;
use core::mem::size_of;
use core::ptr;

pub unsafe fn sst_alloc_stream_mrfld(
    sst_drv_ctx: *mut intel_sst_drv,
    params: *mut c_void,
) -> i32 {
    let pcm_params: *mut snd_pcm_params;
    let str_params: *mut snd_sst_params;
    let fw_tstamp: snd_sst_tstamp = core::mem::zeroed();
    let str_info: *mut stream_info;
    let mut i: i32;
    let num_ch: i32;
    let mut str_id: i32;

    dev_dbg((*sst_drv_ctx).dev, c_str!("Enter\n"));

    str_params = params as *mut snd_sst_params;
    str_id = (*str_params).stream_id;
    str_info = get_stream_info(sst_drv_ctx, str_id);
    if str_info.is_null() {
        return -EINVAL;
    }

    memset(
        &mut (*str_info).alloc_param as *mut _ as *mut c_void,
        0,
        size_of_val(&(*str_info).alloc_param),
    );
    (*str_info).alloc_param.operation = (*str_params).ops;
    (*str_info).alloc_param.codec_type = (*str_params).codec;
    (*str_info).alloc_param.sg_count = (*str_params).aparams.sg_count;
    (*str_info).alloc_param.ring_buf_info[0].addr =
        (*str_params).aparams.ring_buf_info[0].addr;
    (*str_info).alloc_param.ring_buf_info[0].size =
        (*str_params).aparams.ring_buf_info[0].size;
    (*str_info).alloc_param.frag_size = (*str_params).aparams.frag_size;

    memcpy(
        &mut (*str_info).alloc_param.codec_params as *mut _ as *mut c_void,
        &mut (*str_params).sparams as *mut _ as *const c_void,
        size_of::<snd_sst_stream_params>(),
    );

    /*
     * fill channel map params for multichannel support.
     * Ideally channel map should be received from upper layers
     * for multichannel support.
     * Currently hardcoding as per FW reqm.
     */
    num_ch = sst_get_num_channel(str_params);
    pcm_params = &mut (*str_info).alloc_param.codec_params.uc.pcm_params;
    i = 0;
    while i < 8 {
        if i < num_ch {
            (*pcm_params).channel_map[i as usize] = i as _;
        } else {
            (*pcm_params).channel_map[i as usize] = 0xff;
        }
        i += 1;
    }

    (*sst_drv_ctx).streams[str_id as usize].status = STREAM_INIT;
    (*sst_drv_ctx).streams[str_id as usize].prev = STREAM_UN_INIT;
    (*sst_drv_ctx).streams[str_id as usize].pipe_id = (*str_params).device_type;
    (*sst_drv_ctx).streams[str_id as usize].task_id = (*str_params).task;
    (*sst_drv_ctx).streams[str_id as usize].num_ch = num_ch;

    if (*sst_drv_ctx).info.lpe_viewpt_rqd {
        (*str_info).alloc_param.ts = (*sst_drv_ctx).info.mailbox_start
            + (*sst_drv_ctx).tstamp
            + (str_id as usize * size_of_val(&fw_tstamp)) as _;
    } else {
        (*str_info).alloc_param.ts = (*sst_drv_ctx).mailbox_add
            + (*sst_drv_ctx).tstamp
            + (str_id as usize * size_of_val(&fw_tstamp)) as _;
    }

    dev_dbg(
        (*sst_drv_ctx).dev,
        c_str!("alloc tstamp location = 0x%x\n"),
        (*str_info).alloc_param.ts,
    );
    dev_dbg(
        (*sst_drv_ctx).dev,
        c_str!("assigned pipe id 0x%x to task %d\n"),
        (*str_info).pipe_id,
        (*str_info).task_id,
    );

    sst_realloc_stream(sst_drv_ctx, str_id)
}

/**
 * sst_realloc_stream - Send msg for (re-)allocating a stream using the
 * @sst_drv_ctx: intel_sst_drv context pointer
 * @str_id: stream ID
 *
 * Send a msg for (re-)allocating a stream using the parameters previously
 * passed to sst_alloc_stream_mrfld() for the same stream ID.
 * Return: 0 or negative errno value.
 */
pub unsafe fn sst_realloc_stream(sst_drv_ctx: *mut intel_sst_drv, mut str_id: i32) -> i32 {
    let response: *mut snd_sst_alloc_response;
    let str_info: *mut stream_info;
    let mut data: *mut c_void = ptr::null_mut();
    let mut ret: i32;

    str_info = get_stream_info(sst_drv_ctx, str_id);
    if str_info.is_null() {
        return -EINVAL;
    }

    dev_dbg(
        (*sst_drv_ctx).dev,
        c_str!("Alloc for str %d pipe %#x\n"),
        str_id,
        (*str_info).pipe_id,
    );

    ret = sst_prepare_and_post_msg(
        sst_drv_ctx,
        (*str_info).task_id,
        IPC_CMD,
        IPC_IA_ALLOC_STREAM_MRFLD,
        (*str_info).pipe_id,
        size_of_val(&(*str_info).alloc_param),
        &mut (*str_info).alloc_param as *mut _ as *mut c_void,
        &mut data,
        true,
        true,
        false,
        true,
    );

    if ret < 0 {
        dev_err((*sst_drv_ctx).dev, c_str!("FW alloc failed ret %d\n"), ret);
        /* alloc failed, so reset the state to uninit */
        (*str_info).status = STREAM_UN_INIT;
        str_id = ret;
    } else if !data.is_null() {
        response = data as *mut snd_sst_alloc_response;
        ret = (*response).str_type.result;
        if ret == 0 {
            kfree(data);
            return str_id;
        }
        dev_err((*sst_drv_ctx).dev, c_str!("FW alloc failed ret %d\n"), ret);
        if ret == SST_ERR_STREAM_IN_USE {
            dev_err(
                (*sst_drv_ctx).dev,
                c_str!("FW not in clean state, send free for:%d\n"),
                str_id,
            );
            sst_free_stream(sst_drv_ctx, str_id);
        }
        str_id = -ret;
    }

    kfree(data);
    str_id
}

/**
 * sst_start_stream - Send msg for a starting stream
 * @sst_drv_ctx: intel_sst_drv context pointer
 * @str_id: stream ID
 *
 * This function is called by any function which wants to start
 * a stream.
 */
pub unsafe fn sst_start_stream(sst_drv_ctx: *mut intel_sst_drv, str_id: i32) -> i32 {
    let mut retval: i32 = 0;
    let str_info: *mut stream_info;
    let mut data: u16 = 0;

    dev_dbg((*sst_drv_ctx).dev, c_str!("sst_start_stream for %d\n"), str_id);
    str_info = get_stream_info(sst_drv_ctx, str_id);
    if str_info.is_null() {
        return -EINVAL;
    }
    if (*str_info).status != STREAM_RUNNING {
        return -EBADRQC;
    }

    retval = sst_prepare_and_post_msg(
        sst_drv_ctx,
        (*str_info).task_id,
        IPC_CMD,
        IPC_IA_START_STREAM_MRFLD,
        (*str_info).pipe_id,
        size_of::<u16>(),
        &mut data as *mut _ as *mut c_void,
        ptr::null_mut(),
        true,
        true,
        true,
        false,
    );

    retval
}

pub unsafe fn sst_send_byte_stream_mrfld(
    sst_drv_ctx: *mut intel_sst_drv,
    bytes: *mut snd_sst_bytes_v2,
) -> i32 {
    let mut msg: *mut ipc_post = ptr::null_mut();
    let length: u32;
    let pvt_id: i32;
    let mut ret: i32 = 0;
    let mut block: *mut sst_block = ptr::null_mut();
    let bytes_block: u8 = (*bytes).block;

    dev_dbg(
        (*sst_drv_ctx).dev,
        c_str!("type:%u ipc_msg:%u block:%u task_id:%u pipe: %#x length:%#x\n"),
        (*bytes).type_,
        (*bytes).ipc_msg,
        bytes_block,
        (*bytes).task_id,
        (*bytes).pipe_id,
        (*bytes).len,
    );

    if sst_create_ipc_msg(&mut msg, true) != 0 {
        return -ENOMEM;
    }

    pvt_id = sst_assign_pvt_id(sst_drv_ctx);
    sst_fill_header_mrfld(
        &mut (*msg).mrfld_header,
        (*bytes).ipc_msg,
        (*bytes).task_id,
        1,
        pvt_id,
    );
    (*msg).mrfld_header.p.header_high.part.res_rqd = bytes_block;
    length = (*bytes).len;
    (*msg).mrfld_header.p.header_low_payload = length;
    dev_dbg((*sst_drv_ctx).dev, c_str!("length is %d\n"), length);
    memcpy(
        (*msg).mailbox_data as *mut c_void,
        &mut (*bytes).bytes as *mut _ as *const c_void,
        (*bytes).len as usize,
    );
    if bytes_block != 0 {
        block = sst_create_block(sst_drv_ctx, (*bytes).ipc_msg, pvt_id);
        if block.is_null() {
            kfree(msg as *mut c_void);
            ret = -ENOMEM;
            test_and_clear_bit(pvt_id, &mut (*sst_drv_ctx).pvt_id);
            return ret;
        }
    }

    sst_add_to_dispatch_list_and_post(sst_drv_ctx, msg);
    dev_dbg(
        (*sst_drv_ctx).dev,
        c_str!("msg->mrfld_header.p.header_low_payload:%d"),
        (*msg).mrfld_header.p.header_low_payload,
    );

    if bytes_block != 0 {
        ret = sst_wait_timeout(sst_drv_ctx, block);
        if ret != 0 {
            dev_err((*sst_drv_ctx).dev, c_str!("fw returned err %d\n"), ret);
            sst_free_block(sst_drv_ctx, block);
            test_and_clear_bit(pvt_id, &mut (*sst_drv_ctx).pvt_id);
            return ret;
        }
    }
    if (*bytes).type_ == SND_SST_BYTES_GET {
        /*
         * copy the reply and send back
         * we need to update only sz and payload
         */
        if bytes_block != 0 {
            let r: *mut u8 = (*block).data;

            dev_dbg((*sst_drv_ctx).dev, c_str!("read back %d bytes"), (*bytes).len);
            memcpy(
                (*bytes).bytes as *mut c_void,
                r as *const c_void,
                (*bytes).len as usize,
            );
        }
    }
    if bytes_block != 0 {
        sst_free_block(sst_drv_ctx, block);
    }
    test_and_clear_bit(pvt_id, &mut (*sst_drv_ctx).pvt_id);
    ret
}

/**
 * sst_pause_stream - Send msg for a pausing stream
 * @sst_drv_ctx: intel_sst_drv context pointer
 * @str_id: stream ID
 *
 * This function is called by any function which wants to pause
 * an already running stream.
 */
pub unsafe fn sst_pause_stream(sst_drv_ctx: *mut intel_sst_drv, str_id: i32) -> i32 {
    let mut retval: i32 = 0;
    let str_info: *mut stream_info;

    dev_dbg(
        (*sst_drv_ctx).dev,
        c_str!("SST DBG:sst_pause_stream for %d\n"),
        str_id,
    );
    str_info = get_stream_info(sst_drv_ctx, str_id);
    if str_info.is_null() {
        return -EINVAL;
    }
    if (*str_info).status == STREAM_PAUSED {
        return 0;
    }
    if (*str_info).status == STREAM_RUNNING || (*str_info).status == STREAM_INIT {
        if (*str_info).prev == STREAM_UN_INIT {
            return -EBADRQC;
        }

        retval = sst_prepare_and_post_msg(
            sst_drv_ctx,
            (*str_info).task_id,
            IPC_CMD,
            IPC_IA_PAUSE_STREAM_MRFLD,
            (*str_info).pipe_id,
            0,
            ptr::null_mut(),
            ptr::null_mut(),
            true,
            true,
            false,
            true,
        );

        if retval == 0 {
            (*str_info).prev = (*str_info).status;
            (*str_info).status = STREAM_PAUSED;
        } else if retval == -SST_ERR_INVALID_STREAM_ID {
            retval = -EINVAL;
            mutex_lock(&mut (*sst_drv_ctx).sst_lock);
            sst_clean_stream(str_info);
            mutex_unlock(&mut (*sst_drv_ctx).sst_lock);
        }
    } else {
        retval = -EBADRQC;
        dev_dbg((*sst_drv_ctx).dev, c_str!("SST DBG:BADRQC for stream\n"));
    }

    retval
}

/**
 * sst_resume_stream - Send msg for resuming stream
 * @sst_drv_ctx: intel_sst_drv context pointer
 * @str_id: stream ID
 *
 * This function is called by any function which wants to resume
 * an already paused stream.
 */
pub unsafe fn sst_resume_stream(sst_drv_ctx: *mut intel_sst_drv, str_id: i32) -> i32 {
    let mut retval: i32 = 0;
    let str_info: *mut stream_info;

    dev_dbg(
        (*sst_drv_ctx).dev,
        c_str!("SST DBG:sst_resume_stream for %d\n"),
        str_id,
    );
    str_info = get_stream_info(sst_drv_ctx, str_id);
    if str_info.is_null() {
        return -EINVAL;
    }
    if (*str_info).status == STREAM_RUNNING {
        return 0;
    }

    if (*str_info).resume_status == STREAM_PAUSED && (*str_info).resume_prev == STREAM_RUNNING {
        /*
         * Stream was running before suspend and re-created on resume,
         * start it to get back to running state.
         */
        dev_dbg(
            (*sst_drv_ctx).dev,
            c_str!("restart recreated stream after resume\n"),
        );
        (*str_info).status = STREAM_RUNNING;
        (*str_info).prev = STREAM_PAUSED;
        retval = sst_start_stream(sst_drv_ctx, str_id);
        (*str_info).resume_status = STREAM_UN_INIT;
    } else if (*str_info).resume_status == STREAM_PAUSED
        && (*str_info).resume_prev == STREAM_INIT
    {
        /*
         * Stream was idle before suspend and re-created on resume,
         * keep it as is.
         */
        dev_dbg(
            (*sst_drv_ctx).dev,
            c_str!("leaving recreated stream idle after resume\n"),
        );
        (*str_info).status = STREAM_INIT;
        (*str_info).prev = STREAM_PAUSED;
        (*str_info).resume_status = STREAM_UN_INIT;
    } else if (*str_info).status == STREAM_PAUSED {
        retval = sst_prepare_and_post_msg(
            sst_drv_ctx,
            (*str_info).task_id,
            IPC_CMD,
            IPC_IA_RESUME_STREAM_MRFLD,
            (*str_info).pipe_id,
            0,
            ptr::null_mut(),
            ptr::null_mut(),
            true,
            true,
            false,
            true,
        );

        if retval == 0 {
            if (*str_info).prev == STREAM_RUNNING {
                (*str_info).status = STREAM_RUNNING;
            } else {
                (*str_info).status = STREAM_INIT;
            }
            (*str_info).prev = STREAM_PAUSED;
        } else if retval == -SST_ERR_INVALID_STREAM_ID {
            retval = -EINVAL;
            mutex_lock(&mut (*sst_drv_ctx).sst_lock);
            sst_clean_stream(str_info);
            mutex_unlock(&mut (*sst_drv_ctx).sst_lock);
        }
    } else {
        retval = -EBADRQC;
        dev_err((*sst_drv_ctx).dev, c_str!("SST ERR: BADQRC for stream\n"));
    }

    retval
}

/**
 * sst_drop_stream - Send msg for stopping stream
 * @sst_drv_ctx: intel_sst_drv context pointer
 * @str_id: stream ID
 *
 * This function is called by any function which wants to stop
 * a stream.
 */
pub unsafe fn sst_drop_stream(sst_drv_ctx: *mut intel_sst_drv, str_id: i32) -> i32 {
    let mut retval: i32 = 0;
    let str_info: *mut stream_info;

    dev_dbg(
        (*sst_drv_ctx).dev,
        c_str!("SST DBG:sst_drop_stream for %d\n"),
        str_id,
    );
    str_info = get_stream_info(sst_drv_ctx, str_id);
    if str_info.is_null() {
        return -EINVAL;
    }

    if (*str_info).status != STREAM_UN_INIT {
        (*str_info).prev = STREAM_UN_INIT;
        (*str_info).status = STREAM_INIT;
        (*str_info).cumm_bytes = 0;
        retval = sst_prepare_and_post_msg(
            sst_drv_ctx,
            (*str_info).task_id,
            IPC_CMD,
            IPC_IA_DROP_STREAM_MRFLD,
            (*str_info).pipe_id,
            0,
            ptr::null_mut(),
            ptr::null_mut(),
            true,
            true,
            true,
            false,
        );
    } else {
        retval = -EBADRQC;
        dev_dbg(
            (*sst_drv_ctx).dev,
            c_str!("BADQRC for stream, state %x\n"),
            (*str_info).status,
        );
    }
    retval
}

/**
 * sst_drain_stream - Send msg for draining stream
 * @sst_drv_ctx: intel_sst_drv context pointer
 * @str_id: stream ID
 * @partial_drain: boolean indicating if a gapless transition is taking place
 *
 * This function is called by any function which wants to drain
 * a stream.
 */
pub unsafe fn sst_drain_stream(
    sst_drv_ctx: *mut intel_sst_drv,
    str_id: i32,
    mut partial_drain: bool,
) -> i32 {
    let mut retval: i32 = 0;
    let str_info: *mut stream_info;

    dev_dbg(
        (*sst_drv_ctx).dev,
        c_str!("SST DBG:sst_drain_stream for %d\n"),
        str_id,
    );
    str_info = get_stream_info(sst_drv_ctx, str_id);
    if str_info.is_null() {
        return -EINVAL;
    }
    if (*str_info).status != STREAM_RUNNING
        && (*str_info).status != STREAM_INIT
        && (*str_info).status != STREAM_PAUSED
    {
        dev_err(
            (*sst_drv_ctx).dev,
            c_str!("SST ERR: BADQRC for stream = %d\n"),
            (*str_info).status,
        );
        return -EBADRQC;
    }

    retval = sst_prepare_and_post_msg(
        sst_drv_ctx,
        (*str_info).task_id,
        IPC_CMD,
        IPC_IA_DRAIN_STREAM_MRFLD,
        (*str_info).pipe_id,
        size_of::<u8>(),
        &mut partial_drain as *mut _ as *mut c_void,
        ptr::null_mut(),
        true,
        true,
        false,
        false,
    );
    /*
     * with new non blocked drain implementation in core we dont need to
     * wait for respsonse, and need to only invoke callback for drain
     * complete
     */

    retval
}

/**
 * sst_free_stream - Frees a stream
 * @sst_drv_ctx: intel_sst_drv context pointer
 * @str_id: stream ID
 *
 * This function is called by any function which wants to free
 * a stream.
 */
pub unsafe fn sst_free_stream(sst_drv_ctx: *mut intel_sst_drv, str_id: i32) -> i32 {
    let mut retval: i32 = 0;
    let str_info: *mut stream_info;

    dev_dbg(
        (*sst_drv_ctx).dev,
        c_str!("SST DBG:sst_free_stream for %d\n"),
        str_id,
    );

    mutex_lock(&mut (*sst_drv_ctx).sst_lock);
    if (*sst_drv_ctx).sst_state == SST_RESET {
        mutex_unlock(&mut (*sst_drv_ctx).sst_lock);
        return -ENODEV;
    }
    mutex_unlock(&mut (*sst_drv_ctx).sst_lock);
    str_info = get_stream_info(sst_drv_ctx, str_id);
    if str_info.is_null() {
        return -EINVAL;
    }

    mutex_lock(&mut (*str_info).lock);
    if (*str_info).status != STREAM_UN_INIT {
        (*str_info).prev = (*str_info).status;
        (*str_info).status = STREAM_UN_INIT;
        mutex_unlock(&mut (*str_info).lock);

        dev_dbg(
            (*sst_drv_ctx).dev,
            c_str!("Free for str %d pipe %#x\n"),
            str_id,
            (*str_info).pipe_id,
        );
        retval = sst_prepare_and_post_msg(
            sst_drv_ctx,
            (*str_info).task_id,
            IPC_CMD,
            IPC_IA_FREE_STREAM_MRFLD,
            (*str_info).pipe_id,
            0,
            ptr::null_mut(),
            ptr::null_mut(),
            true,
            true,
            false,
            true,
        );

        dev_dbg(
            (*sst_drv_ctx).dev,
            c_str!("sst: wait for free returned %d\n"),
            retval,
        );
        mutex_lock(&mut (*sst_drv_ctx).sst_lock);
        sst_clean_stream(str_info);
        mutex_unlock(&mut (*sst_drv_ctx).sst_lock);
        dev_dbg((*sst_drv_ctx).dev, c_str!("SST DBG:Stream freed\n"));
    } else {
        mutex_unlock(&mut (*str_info).lock);
        retval = -EBADRQC;
        dev_dbg((*sst_drv_ctx).dev, c_str!("SST DBG:BADQRC for stream\n"));
    }

    retval
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
