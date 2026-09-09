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

// Dependencies supplied by the surrounding translation unit:
// dc.h, dc_dmub_srv.h, dmub_outbox.h, and dmub/inc/dmub_cmd.h

extern "C" {
    fn dc_wake_and_execute_dmub_cmd(
        ctx: *mut core::ffi::c_void,
        cmd: *mut dmub_rb_cmd,
        wait_type: i32,
    );
}

// Function: dmub_enable_outbox_notification
//
// @brief
//     Sends inbox cmd to dmub for enabling outbox notifications to x86.
//
// @param
//     [in] dmub_srv: dmub_srv structure
pub unsafe fn dmub_enable_outbox_notification(dmub_srv: *mut dc_dmub_srv) {
    let mut cmd: dmub_rb_cmd = core::mem::zeroed();

    (*core::ptr::addr_of_mut!(cmd)).outbox1_enable.header.type_ = DMUB_CMD__OUTBOX1_ENABLE;
    (*core::ptr::addr_of_mut!(cmd)).outbox1_enable.header.sub_type = 0;
    (*core::ptr::addr_of_mut!(cmd)).outbox1_enable.header.payload_bytes =
        core::mem::size_of_val(&(*core::ptr::addr_of!(cmd)).outbox1_enable)
            - core::mem::size_of_val(&(*core::ptr::addr_of!(cmd)).outbox1_enable.header);
    (*core::ptr::addr_of_mut!(cmd)).outbox1_enable.enable = true;

    dc_wake_and_execute_dmub_cmd(
        (*dmub_srv).ctx,
        &mut cmd,
        DM_DMUB_WAIT_TYPE_WAIT,
    );
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
