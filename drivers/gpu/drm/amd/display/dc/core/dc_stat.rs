/*
 * Copyright 2020 Advanced Micro Devices, Inc.
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

// Declarations supplied by the DC and DMUB interfaces.
use crate::dc::dc;
use crate::dmub::dmub_notification;
use crate::dmub::dmub_srv;
use crate::dmub::dmub_status;
use crate::dmub::{DMUB_NOTIFICATION_AUX_REPLY, DMUB_NOTIFICATION_DPIA_NOTIFICATION};
use crate::dmub::{DMUB_NOTIFICATION_HPD, DMUB_NOTIFICATION_HPD_IRQ};
use crate::dmub::DMUB_NOTIFICATION_SET_CONFIG_REPLY;
use crate::dmub::DMUB_STATUS_OK;

extern "C" {
    fn dmub_srv_stat_get_notification(
        dmub: *mut dmub_srv,
        notify: *mut dmub_notification,
    ) -> dmub_status;
    fn dmub_srv_get_gpint_dataout(dmub: *mut dmub_srv, dataout: *mut u32) -> dmub_status;
    fn get_link_index_from_dpia_port_index(dc: *const dc, instance: u32) -> u32;
}

extern "C" {
    fn ASSERT(condition: bool);
}

/**
 * DOC: DC STAT Interface
 *
 * These interfaces are called without acquiring DAL and DC locks.
 * Hence, there is limitations on whese interfaces can access. Only
 * variables exclusively defined for these interfaces can be modified.
 */

/**
 *  dc_stat_get_dmub_notification
 *
 * Calls dmub layer to retrieve dmub notification
 *
 * @dc: dc structure
 * @notify: dmub notification structure
 *
 * Returns
 *     None
 */
pub unsafe fn dc_stat_get_dmub_notification(dc: *const dc, notify: *mut dmub_notification) {
    /**
     * This function is called without dal and dc locks, so
     * we shall not modify any dc, dc_dmub_srv or dmub variables
     * except variables exclusively accessed by this function
     */
    let dmub = (*(*(*dc).ctx).dmub_srv).dmub;
    let status: dmub_status;

    status = dmub_srv_stat_get_notification(dmub, notify);
    ASSERT(status == DMUB_STATUS_OK);

    /* For HPD/HPD RX, convert dpia port index into link index */
    if (*notify).type_ == DMUB_NOTIFICATION_HPD
        || (*notify).type_ == DMUB_NOTIFICATION_HPD_IRQ
        || (*notify).type_ == DMUB_NOTIFICATION_AUX_REPLY
        || (*notify).type_ == DMUB_NOTIFICATION_DPIA_NOTIFICATION
        || (*notify).type_ == DMUB_NOTIFICATION_SET_CONFIG_REPLY
    {
        (*notify).link_index =
            get_link_index_from_dpia_port_index(dc, (*notify).instance);
    }
}

/**
 * dc_stat_get_dmub_dataout
 *
 * Calls dmub layer to retrieve dmub gpint dataout
 *
 * @dc: dc structure
 * @dataout: dmub gpint dataout
 *
 * Returns
 *     None
 */
pub unsafe fn dc_stat_get_dmub_dataout(dc: *const dc, dataout: *mut u32) {
    let dmub = (*(*(*dc).ctx).dmub_srv).dmub;
    let status: dmub_status;

    status = dmub_srv_get_gpint_dataout(dmub, dataout);
    ASSERT(status == DMUB_STATUS_OK);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
