/*
 * Copyright 2019 Advanced Micro Devices, Inc.
 *
 * Permission is hereby granted, free of charge, to any person obtaining a
 * copy of this software and associated documentation files (the "Software"),
 * to deal in the Software without restriction, including without limitation
 * the rights to use, copy, modify, merge, publish, distribute, sublicense,
 * and/or sell copies of the Software, and to permit persons to whom the
 * Software is furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in
 * all copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.  IN NO EVENT SHALL
 * THE COPYRIGHT HOLDER(S) OR AUTHOR(S) BE LIABLE FOR ANY CLAIM, DAMAGES OR
 * OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
 * ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
 * OTHER DEALINGS IN THE SOFTWARE.
 *
 * Authors: AMD
 */

// Dependencies: dmub/dmub_srv_stat.h and dmub/inc/dmub_cmd.h

/**
 * DOC: DMUB_SRV STAT Interface
 *
 * These interfaces are called without acquiring DAL and DC locks.
 * Hence, there is limitations on whese interfaces can access. Only
 * variables exclusively defined for these interfaces can be modified.
 */

/**
 * dmub_srv_stat_get_notification - Retrieves a dmub outbox notification, set up dmub notification
 *                                  structure with message information. Also a pending bit if queue
 *                                  is having more notifications
 *  @dmub: dmub srv structure
 *  @notify: dmub notification structure to be filled up
 *
 *  Returns: dmub_status
 */
pub unsafe fn dmub_srv_stat_get_notification(
    dmub: *mut dmub_srv,
    notify: *mut dmub_notification,
) -> dmub_status {
    /*
     * This function is called without dal and dc locks, so
     * we shall not modify any dmub variables, only dmub->outbox1_rb
     * is exempted as it is exclusively accessed by this function
     */
    let mut cmd: dmub_rb_out_cmd = core::mem::zeroed();

    if !(*dmub).hw_init {
        (*notify).type_ = DMUB_NOTIFICATION_NO_DATA;
        (*notify).pending_notification = false;
        return DMUB_STATUS_INVALID;
    }

    /* Get write pointer which is updated by dmub */
    (*dmub).outbox1_rb.wrpt = ((*dmub).hw_funcs.get_outbox1_wptr)(dmub);

    if !dmub_rb_out_front(&mut (*dmub).outbox1_rb, &mut cmd) {
        (*notify).type_ = DMUB_NOTIFICATION_NO_DATA;
        (*notify).pending_notification = false;
        return DMUB_STATUS_OK;
    }

    match cmd.cmd_common.header.type_ {
        DMUB_OUT_CMD__DP_AUX_REPLY => {
            (*notify).type_ = DMUB_NOTIFICATION_AUX_REPLY;
            (*notify).instance = cmd.dp_aux_reply.control.instance;
            (*notify).result = cmd.dp_aux_reply.control.result;
            dmub_memcpy(
                &mut (*notify).aux_reply as *mut _ as *mut core::ffi::c_void,
                &cmd.dp_aux_reply.reply_data as *const _ as *const core::ffi::c_void,
                core::mem::size_of::<aux_reply_data>(),
            );
        }
        DMUB_OUT_CMD__DP_HPD_NOTIFY => {
            if cmd.dp_hpd_notify.hpd_data.hpd_type == DP_HPD {
                (*notify).type_ = DMUB_NOTIFICATION_HPD;
                (*notify).hpd_status = cmd.dp_hpd_notify.hpd_data.hpd_status;
            } else {
                (*notify).type_ = DMUB_NOTIFICATION_HPD_IRQ;
            }
            (*notify).instance = cmd.dp_hpd_notify.hpd_data.instance;
            (*notify).result = AUX_RET_SUCCESS;
        }
        DMUB_OUT_CMD__SET_CONFIG_REPLY => {
            (*notify).type_ = DMUB_NOTIFICATION_SET_CONFIG_REPLY;
            (*notify).instance = cmd.set_config_reply.set_config_reply_control.instance;
            (*notify).sc_status = cmd.set_config_reply.set_config_reply_control.status;
        }
        DMUB_OUT_CMD__DPIA_NOTIFICATION => {
            (*notify).type_ = DMUB_NOTIFICATION_DPIA_NOTIFICATION;
            (*notify).instance = cmd.dpia_notification.payload.header.instance;
        }
        DMUB_OUT_CMD__HPD_SENSE_NOTIFY => {
            (*notify).type_ = DMUB_NOTIFICATION_HPD_SENSE_NOTIFY;
            dmub_memcpy(
                &mut (*notify).hpd_sense_notify as *mut _ as *mut core::ffi::c_void,
                &cmd.hpd_sense_notify.data as *const _ as *const core::ffi::c_void,
                core::mem::size_of_val(&cmd.hpd_sense_notify.data),
            );
        }
        DMUB_OUT_CMD__FUSED_IO => {
            (*notify).type_ = DMUB_NOTIFICATION_FUSED_IO;
            dmub_memcpy(
                &mut (*notify).fused_request as *mut _ as *mut core::ffi::c_void,
                &cmd.fused_io.request as *const _ as *const core::ffi::c_void,
                core::mem::size_of_val(&cmd.fused_io.request),
            );
        }
        _ => {
            (*notify).type_ = DMUB_NOTIFICATION_NO_DATA;
        }
    }

    /* Pop outbox1 ringbuffer and update read pointer */
    dmub_rb_pop_front(&mut (*dmub).outbox1_rb);
    ((*dmub).hw_funcs.set_outbox1_rptr)(dmub, (*dmub).outbox1_rb.rptr);

    /**
     * Notify dc whether dmub has a pending outbox message,
     * this is to avoid one more call to dmub_srv_stat_get_notification
     */
    (*notify).pending_notification = !dmub_rb_empty(&mut (*dmub).outbox1_rb);

    DMUB_STATUS_OK
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
