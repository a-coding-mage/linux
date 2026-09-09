// SPDX-License-Identifier: (GPL-2.0+ OR BSD-3-Clause)
/*
 * Copyright 2013-2016 Freescale Semiconductor Inc.
 *
 */

use core::mem;

// Dependencies supplied by the surrounding kernel/Rust translation.
extern "C" {
    fn mc_encode_cmd_header(cmd_id: u16, cmd_flags: u32, token: u16) -> u64;
    fn mc_send_command(mc_io: *mut fsl_mc_io, cmd: *mut fsl_mc_command) -> i32;
    fn mc_cmd_hdr_read_token(cmd: *const fsl_mc_command) -> u16;
}

#[no_mangle]
pub unsafe extern "C" fn dpcon_open(
    mc_io: *mut fsl_mc_io,
    cmd_flags: u32,
    dpcon_id: i32,
    token: *mut u16,
) -> i32 {
    let mut cmd: fsl_mc_command = mem::zeroed();
    let dpcon_cmd: *mut dpcon_cmd_open = cmd.params.as_mut_ptr() as *mut dpcon_cmd_open;
    let err: i32;

    /* prepare command */
    cmd.header = mc_encode_cmd_header(DPCON_CMDID_OPEN, cmd_flags, 0);
    (*dpcon_cmd).dpcon_id = cpu_to_le32(dpcon_id as u32);

    /* send command to mc*/
    err = mc_send_command(mc_io, &mut cmd);
    if err != 0 {
        return err;
    }

    /* retrieve response parameters */
    *token = mc_cmd_hdr_read_token(&cmd);

    0
}

#[no_mangle]
pub unsafe extern "C" fn dpcon_close(
    mc_io: *mut fsl_mc_io,
    cmd_flags: u32,
    token: u16,
) -> i32 {
    let mut cmd: fsl_mc_command = mem::zeroed();

    /* prepare command */
    cmd.header = mc_encode_cmd_header(DPCON_CMDID_CLOSE, cmd_flags, token);

    /* send command to mc*/
    mc_send_command(mc_io, &mut cmd)
}

#[no_mangle]
pub unsafe extern "C" fn dpcon_enable(
    mc_io: *mut fsl_mc_io,
    cmd_flags: u32,
    token: u16,
) -> i32 {
    let mut cmd: fsl_mc_command = mem::zeroed();

    /* prepare command */
    cmd.header = mc_encode_cmd_header(DPCON_CMDID_ENABLE, cmd_flags, token);

    /* send command to mc*/
    mc_send_command(mc_io, &mut cmd)
}

#[no_mangle]
pub unsafe extern "C" fn dpcon_disable(
    mc_io: *mut fsl_mc_io,
    cmd_flags: u32,
    token: u16,
) -> i32 {
    let mut cmd: fsl_mc_command = mem::zeroed();

    /* prepare command */
    cmd.header = mc_encode_cmd_header(DPCON_CMDID_DISABLE, cmd_flags, token);

    /* send command to mc*/
    mc_send_command(mc_io, &mut cmd)
}

#[no_mangle]
pub unsafe extern "C" fn dpcon_reset(
    mc_io: *mut fsl_mc_io,
    cmd_flags: u32,
    token: u16,
) -> i32 {
    let mut cmd: fsl_mc_command = mem::zeroed();

    /* prepare command */
    cmd.header = mc_encode_cmd_header(DPCON_CMDID_RESET, cmd_flags, token);

    /* send command to mc*/
    mc_send_command(mc_io, &mut cmd)
}

#[no_mangle]
pub unsafe extern "C" fn dpcon_get_attributes(
    mc_io: *mut fsl_mc_io,
    cmd_flags: u32,
    token: u16,
    attr: *mut dpcon_attr,
) -> i32 {
    let mut cmd: fsl_mc_command = mem::zeroed();
    let err: i32;

    /* prepare command */
    cmd.header = mc_encode_cmd_header(DPCON_CMDID_GET_ATTR, cmd_flags, token);

    /* send command to mc*/
    err = mc_send_command(mc_io, &mut cmd);
    if err != 0 {
        return err;
    }

    /* retrieve response parameters */
    let dpcon_rsp: *mut dpcon_rsp_get_attr = cmd.params.as_mut_ptr() as *mut dpcon_rsp_get_attr;
    (*attr).id = le32_to_cpu((*dpcon_rsp).id);
    (*attr).qbman_ch_id = le16_to_cpu((*dpcon_rsp).qbman_ch_id);
    (*attr).num_priorities = (*dpcon_rsp).num_priorities;

    0
}

#[no_mangle]
pub unsafe extern "C" fn dpcon_set_notification(
    mc_io: *mut fsl_mc_io,
    cmd_flags: u32,
    token: u16,
    cfg: *const dpcon_notification_cfg,
) -> i32 {
    let mut cmd: fsl_mc_command = mem::zeroed();

    /* prepare command */
    cmd.header = mc_encode_cmd_header(DPCON_CMDID_SET_NOTIFICATION, cmd_flags, token);
    let dpcon_cmd: *mut dpcon_cmd_set_notification =
        cmd.params.as_mut_ptr() as *mut dpcon_cmd_set_notification;
    (*dpcon_cmd).dpio_id = cpu_to_le32((*cfg).dpio_id);
    (*dpcon_cmd).priority = (*cfg).priority;
    (*dpcon_cmd).user_ctx = cpu_to_le64((*cfg).user_ctx);

    /* send command to mc*/
    mc_send_command(mc_io, &mut cmd)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
