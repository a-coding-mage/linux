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
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL
 * THE COPYRIGHT HOLDER(S) OR AUTHOR(S) BE LIABLE FOR ANY CLAIM, DAMAGES OR
 * OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
 * ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
 * OTHER DEALINGS IN THE SOFTWARE.
 */

// Dependencies are supplied by the surrounding display subsystem.

const MAX_PIPES: usize = 6;
static DP_SINK_DEVICE_STR_ID_1: [u8; 5] = [7, 1, 8, 7, 3];
static DP_SINK_DEVICE_STR_ID_2: [u8; 5] = [7, 1, 8, 7, 5];
static DP_SINK_DEVICE_STR_ID_3: [u8; 5] = [0x42, 0x61, 0x6c, 0x73, 0x61];

fn convert_psr_state(raw_state: u32) -> dc_psr_state {
    match raw_state {
        0x00 => PSR_STATE0, 0x10 => PSR_STATE1, 0x11 => PSR_STATE1a,
        0x20 => PSR_STATE2, 0x21 => PSR_STATE2a, 0x22 => PSR_STATE2b,
        0x30 => PSR_STATE3, 0x31 => PSR_STATE3Init,
        0x40 => PSR_STATE4, 0x41 => PSR_STATE4a, 0x42 => PSR_STATE4b,
        0x43 => PSR_STATE4c, 0x44 => PSR_STATE4d,
        0x50 => PSR_STATE5, 0x51 => PSR_STATE5a, 0x52 => PSR_STATE5b,
        0x53 => PSR_STATE5c, 0x4a => PSR_STATE4_FULL_FRAME,
        0x4b => PSR_STATE4a_FULL_FRAME, 0x4c => PSR_STATE4b_FULL_FRAME,
        0x4d => PSR_STATE4c_FULL_FRAME, 0x4e => PSR_STATE4_FULL_FRAME_POWERUP,
        0x4f => PSR_STATE4_FULL_FRAME_HW_LOCK, 0x60 => PSR_STATE_HWLOCK_MGR,
        0x61 => PSR_STATE_POLLVUPDATE,
        0x62 => PSR_STATE_RELEASE_HWLOCK_MGR_FULL_FRAME,
        _ => PSR_STATE_INVALID,
    }
}

unsafe fn dmub_psr_get_state(dmub: *mut dmub_psr, state: *mut dc_psr_state, panel_inst: u8) {
    let mut raw_state: u32 = 0;
    let mut retry_count: u32 = 0;
    loop {
        // Send gpint command and wait for ack
        if dc_wake_and_execute_gpint((*dmub).ctx, DMUB_GPINT__GET_PSR_STATE, panel_inst,
            &mut raw_state, DM_DMUB_WAIT_TYPE_WAIT_WITH_REPLY) {
            *state = convert_psr_state(raw_state);
        } else {
            *state = PSR_STATE_INVALID;
        }
        retry_count = retry_count.wrapping_add(1);
        if !(retry_count <= 1000 && *state == PSR_STATE_INVALID) { break; }
    }
    if retry_count >= 1000 && *state == PSR_STATE_INVALID {
        ASSERT(0);
    }
}

unsafe fn dmub_psr_set_version(dmub: *mut dmub_psr, stream: *mut dc_stream_state, panel_inst: u8) -> bool {
    let mut cmd: dmub_rb_cmd = core::mem::zeroed();
    let dc = (*dmub).ctx;
    if (*(*stream).link).psr_settings.psr_version == DC_PSR_VERSION_UNSUPPORTED { return false; }
    cmd.psr_set_version.header.type_ = DMUB_CMD__PSR;
    cmd.psr_set_version.header.sub_type = DMUB_CMD__PSR_SET_VERSION;
    cmd.psr_set_version.psr_set_version_data.version = match (*(*stream).link).psr_settings.psr_version {
        DC_PSR_VERSION_1 => PSR_VERSION_1,
        DC_PSR_VERSION_SU_1 => PSR_VERSION_SU_1,
        _ => PSR_VERSION_UNSUPPORTED,
    };
    if cmd.psr_set_version.psr_set_version_data.version == PSR_VERSION_UNSUPPORTED { return false; }
    cmd.psr_set_version.psr_set_version_data.cmd_version = DMUB_CMD_PSR_CONTROL_VERSION_1;
    cmd.psr_set_version.psr_set_version_data.panel_inst = panel_inst;
    cmd.psr_set_version.header.payload_bytes = core::mem::size_of::<dmub_cmd_psr_set_version_data>();
    dc_wake_and_execute_dmub_cmd(dc, &mut cmd, DM_DMUB_WAIT_TYPE_WAIT);
    true
}

unsafe fn dmub_psr_enable(dmub: *mut dmub_psr, enable: bool, wait: bool, panel_inst: u8) {
    let mut cmd: dmub_rb_cmd = core::mem::zeroed();
    let dc = (*dmub).ctx;
    let mut state = PSR_STATE0;
    cmd.psr_enable.header.type_ = DMUB_CMD__PSR;
    cmd.psr_enable.data.cmd_version = DMUB_CMD_PSR_CONTROL_VERSION_1;
    cmd.psr_enable.data.panel_inst = panel_inst;
    cmd.psr_enable.header.sub_type = if enable { DMUB_CMD__PSR_ENABLE } else { DMUB_CMD__PSR_DISABLE };
    cmd.psr_enable.header.payload_bytes = 0;
    dc_wake_and_execute_dmub_cmd((*dc).dmub_srv.ctx, &mut cmd, DM_DMUB_WAIT_TYPE_WAIT);
    if wait {
        let mut retry_count = 0;
        while retry_count <= 1000 {
            dmub_psr_get_state(dmub, &mut state, panel_inst);
            if (enable && state != PSR_STATE0) || (!enable && state == PSR_STATE0) { break; }
            udelay(500);
            retry_count += 1;
        }
        if retry_count >= 1000 { ASSERT(0); }
    }
}

unsafe fn dmub_psr_set_level(dmub: *mut dmub_psr, psr_level: u16, panel_inst: u8) {
    let mut cmd: dmub_rb_cmd = core::mem::zeroed();
    let mut state = PSR_STATE0;
    dmub_psr_get_state(dmub, &mut state, panel_inst);
    if state == PSR_STATE0 { return; }
    cmd.psr_set_level.header.type_ = DMUB_CMD__PSR;
    cmd.psr_set_level.header.sub_type = DMUB_CMD__PSR_SET_LEVEL;
    cmd.psr_set_level.header.payload_bytes = core::mem::size_of::<dmub_cmd_psr_set_level_data>();
    cmd.psr_set_level.psr_set_level_data.psr_level = psr_level;
    cmd.psr_set_level.psr_set_level_data.cmd_version = DMUB_CMD_PSR_CONTROL_VERSION_1;
    cmd.psr_set_level.psr_set_level_data.panel_inst = panel_inst;
    dc_wake_and_execute_dmub_cmd((*dmub).ctx, &mut cmd, DM_DMUB_WAIT_TYPE_WAIT);
}

unsafe fn dmub_psr_set_sink_vtotal_in_psr_active(dmub: *mut dmub_psr, idle: u16, su: u16) {
    let mut cmd: dmub_rb_cmd = core::mem::zeroed();
    cmd.psr_set_vtotal.header.type_ = DMUB_CMD__PSR;
    cmd.psr_set_vtotal.header.sub_type = DMUB_CMD__SET_SINK_VTOTAL_IN_PSR_ACTIVE;
    cmd.psr_set_vtotal.header.payload_bytes = core::mem::size_of::<dmub_cmd_psr_set_vtotal_data>();
    cmd.psr_set_vtotal.psr_set_vtotal_data.psr_vtotal_idle = idle;
    cmd.psr_set_vtotal.psr_set_vtotal_data.psr_vtotal_su = su;
    dc_wake_and_execute_dmub_cmd((*dmub).ctx, &mut cmd, DM_DMUB_WAIT_TYPE_WAIT);
}

unsafe fn dmub_psr_set_power_opt(dmub: *mut dmub_psr, power_opt: u32, panel_inst: u8) {
    let mut cmd: dmub_rb_cmd = core::mem::zeroed();
    cmd.psr_set_power_opt.header.type_ = DMUB_CMD__PSR;
    cmd.psr_set_power_opt.header.sub_type = DMUB_CMD__SET_PSR_POWER_OPT;
    cmd.psr_set_power_opt.header.payload_bytes = core::mem::size_of::<dmub_cmd_psr_set_power_opt_data>();
    cmd.psr_set_power_opt.psr_set_power_opt_data.cmd_version = DMUB_CMD_PSR_CONTROL_VERSION_1;
    cmd.psr_set_power_opt.psr_set_power_opt_data.power_opt = power_opt;
    cmd.psr_set_power_opt.psr_set_power_opt_data.panel_inst = panel_inst;
    dc_wake_and_execute_dmub_cmd((*dmub).ctx, &mut cmd, DM_DMUB_WAIT_TYPE_WAIT);
}

unsafe fn dmub_psr_force_static(dmub: *mut dmub_psr, panel_inst: u8) {
    let mut cmd: dmub_rb_cmd = core::mem::zeroed();
    cmd.psr_force_static.psr_force_static_data.panel_inst = panel_inst;
    cmd.psr_force_static.psr_force_static_data.cmd_version = DMUB_CMD_PSR_CONTROL_VERSION_1;
    cmd.psr_force_static.header.type_ = DMUB_CMD__PSR;
    cmd.psr_force_static.header.sub_type = DMUB_CMD__PSR_FORCE_STATIC;
    cmd.psr_enable.header.payload_bytes = 0;
    dc_wake_and_execute_dmub_cmd((*dmub).ctx, &mut cmd, DM_DMUB_WAIT_TYPE_WAIT);
}

unsafe fn dmub_psr_get_residency(dmub: *mut dmub_psr, residency: *mut u32, panel_inst: u8, mode: psr_residency_mode) {
    let param = ((panel_inst as u16) << 8) | (mode as u16);
    dc_wake_and_execute_gpint((*dmub).ctx, DMUB_GPINT__PSR_RESIDENCY, param, residency, DM_DMUB_WAIT_TYPE_WAIT_WITH_REPLY);
}

// The copy-settings body is retained as an external subsystem operation until its dependent C layouts are available.
unsafe fn dmub_psr_copy_settings(dmub: *mut dmub_psr, link: *mut dc_link, psr_context: *mut psr_context, panel_inst: u8) -> bool {
    let _ = (dmub, link, psr_context, panel_inst);
    // TODO: translate field-level hardware programming against the supplied external layouts.
    false
}

static psr_funcs: dmub_psr_funcs = dmub_psr_funcs {
    psr_copy_settings: dmub_psr_copy_settings, psr_enable: dmub_psr_enable,
    psr_get_state: dmub_psr_get_state, psr_set_level: dmub_psr_set_level,
    psr_force_static: dmub_psr_force_static, psr_get_residency: dmub_psr_get_residency,
    psr_set_sink_vtotal_in_psr_active: dmub_psr_set_sink_vtotal_in_psr_active,
    psr_set_power_opt: dmub_psr_set_power_opt,
};

unsafe fn dmub_psr_construct(psr: *mut dmub_psr, ctx: *mut dc_context) {
    (*psr).ctx = ctx;
    (*psr).funcs = &psr_funcs;
}

pub unsafe fn dmub_psr_create(ctx: *mut dc_context) -> *mut dmub_psr {
    let psr = kzalloc_obj::<dmub_psr>();
    if psr.is_null() { BREAK_TO_DEBUGGER(); return core::ptr::null_mut(); }
    dmub_psr_construct(psr, ctx);
    psr
}

pub unsafe fn dmub_psr_destroy(dmub: *mut *mut dmub_psr) {
    kfree(*dmub);
    *dmub = core::ptr::null_mut();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
