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

unsafe fn dmub_hw_lock_has_inbox0_lock(dc: *const dc) -> bool {
    !(*dc).ctx.is_null()
        && !(*(*dc).ctx).dmub_srv.is_null()
        && (*dc).hwss.dmub_hw_control_lock
        && (*dc).hwss.dmub_hw_control_lock_fast
        && (*(*(*(*dc).ctx).dmub_srv).dmub)
            .meta_info
            .feature_bits
            .bits
            .inbox0_lock_support
}

pub unsafe fn dmub_hw_lock_mgr_cmd(
    dmub_srv: *mut dc_dmub_srv,
    lock: bool,
    hw_locks: *mut dmub_hw_lock_flags,
    inst_flags: *const dmub_hw_lock_inst_flags,
) {
    let mut cmd: dmub_rb_cmd = core::mem::zeroed();
    (*cmd.lock_hw).header.type_ = DMUB_CMD__HW_LOCK;
    (*cmd.lock_hw).header.sub_type = 0;
    (*cmd.lock_hw).header.payload_bytes = core::mem::size_of::<dmub_cmd_lock_hw_data>();
    (*cmd.lock_hw).lock_hw_data.client = HW_LOCK_CLIENT_DRIVER;
    (*cmd.lock_hw).lock_hw_data.lock = lock;
    (*cmd.lock_hw).lock_hw_data.hw_locks.u8All = (*hw_locks).u8All;
    core::ptr::copy_nonoverlapping(
        inst_flags,
        &mut (*cmd.lock_hw).lock_hw_data.inst_flags,
        1,
    );

    if !lock {
        (*cmd.lock_hw).lock_hw_data.should_release = 1;
    }

    dc_wake_and_execute_dmub_cmd((*dmub_srv).ctx, &mut cmd, DM_DMUB_WAIT_TYPE_WAIT);
}

pub unsafe fn dmub_hw_lock_mgr_inbox0_cmd(
    dmub_srv: *mut dc_dmub_srv,
    hw_lock_cmd: dmub_inbox0_cmd_lock_hw,
) {
    let mut data: dmub_inbox0_data_register = core::mem::zeroed();
    data.inbox0_cmd_lock_hw = hw_lock_cmd;
    dc_dmub_srv_clear_inbox0_ack(dmub_srv);
    dc_dmub_srv_send_inbox0_cmd(dmub_srv, &mut data);
    dc_dmub_srv_wait_for_inbox0_ack(dmub_srv);
}

pub unsafe fn dmub_hw_lock_mgr_does_link_require_lock(
    dc: *const dc,
    link: *const dc_link,
) -> bool {
    if link.is_null() {
        return false;
    }

    if (*link).psr_settings.psr_version == DC_PSR_VERSION_SU_1 {
        return true;
    }

    if (*link).replay_settings.replay_feature_enabled
        && dc_is_embedded_signal((*link).connector_signal)
    {
        return true;
    }

    if (*link).psr_settings.psr_version == DC_PSR_VERSION_1 {
        let mut edp_links: [*mut dc_link; MAX_NUM_EDP] = [core::ptr::null_mut(); MAX_NUM_EDP];
        let mut edp_num: u32 = 0;
        dc_get_edp_links(dc, edp_links.as_mut_ptr(), &mut edp_num);
        if edp_num == 1 {
            return true;
        }
    }
    false
}

pub unsafe fn dmub_hw_lock_mgr_does_context_require_lock(
    dc: *const dc,
    context: *const dc_state,
) -> bool {
    if context.is_null() {
        return false;
    }
    for i in 0..(*context).stream_count {
        let link: *const dc_link = (*(*context).streams.add(i as usize)).link;
        if dmub_hw_lock_mgr_does_link_require_lock(dc, link) {
            return true;
        }
    }
    false
}

pub unsafe fn should_use_dmub_inbox1_lock(dc: *const dc, link: *const dc_link) -> bool {
    /* ASIC doesn't support DMUB */
    if (*(*dc).ctx).dmub_srv.is_null() {
        return false;
    }

    if (*(*dc).ctx).dce_version >= DCN_VERSION_4_01 {
        return false;
    }

    if dmub_hw_lock_has_inbox0_lock(dc) {
        return false;
    }

    dmub_hw_lock_mgr_does_link_require_lock(dc, link)
}

pub unsafe fn should_use_dmub_inbox0_lock_for_link(
    dc: *const dc,
    link: *const dc_link,
) -> bool {
    dmub_hw_lock_has_inbox0_lock(dc) && dmub_hw_lock_mgr_does_link_require_lock(dc, link)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
