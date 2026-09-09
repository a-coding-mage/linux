// SPDX-License-Identifier: (GPL-2.0+ OR BSD-3-Clause)
/*
 * Copyright 2013-2016 Freescale Semiconductor Inc.
 *
 */

// Declarations and constants supplied by <linux/kernel.h>, <linux/fsl/mc.h>,
// and "fsl-mc-private.h" are expected to be provided by the surrounding
// translation unit.

/**
 * dpmcp_open() - Open a control session for the specified object.
 * @mc_io:      Pointer to MC portal's I/O object
 * @cmd_flags:  Command flags; one or more of 'MC_CMD_FLAG_'
 * @dpmcp_id:   DPMCP unique ID
 * @token:      Returned token; use in subsequent API calls
 *
 * This function can be used to open a control session for an
 * already created object; an object may have been declared in
 * the DPL or by calling the dpmcp_create function.
 * This function returns a unique authentication token,
 * associated with the specific object ID and the specific MC
 * portal; this token must be used in all subsequent commands for
 * this specific object
 *
 * Return:       '0' on Success; Error code otherwise.
 */
pub unsafe fn dpmcp_open(
    mc_io: *mut crate::fsl_mc_io,
    cmd_flags: u32,
    dpmcp_id: i32,
    token: *mut u16,
) -> i32 {
    let mut cmd: crate::fsl_mc_command = core::mem::zeroed();
    let cmd_params: *mut crate::dpmcp_cmd_open;
    let mut err: i32;

    /* prepare command */
    cmd.header = crate::mc_encode_cmd_header(crate::DPMCP_CMDID_OPEN, cmd_flags, 0);
    cmd_params = cmd.params.as_mut_ptr() as *mut crate::dpmcp_cmd_open;
    (*cmd_params).dpmcp_id = dpmcp_id as u32;
    (*cmd_params).dpmcp_id = (*cmd_params).dpmcp_id.to_le();

    /* send command to mc*/
    err = crate::mc_send_command(mc_io, &mut cmd);
    if err != 0 {
        return err;
    }

    /* retrieve response parameters */
    *token = crate::mc_cmd_hdr_read_token(&cmd);

    err
}

/**
 * dpmcp_close() - Close the control session of the object
 * @mc_io:      Pointer to MC portal's I/O object
 * @cmd_flags:  Command flags; one or more of 'MC_CMD_FLAG_'
 * @token:      Token of DPMCP object
 *
 * After this function is called, no further operations are
 * allowed on the object without opening a new control session.
 *
 * Return:       '0' on Success; Error code otherwise.
 */
pub unsafe fn dpmcp_close(
    mc_io: *mut crate::fsl_mc_io,
    cmd_flags: u32,
    token: u16,
) -> i32 {
    let mut cmd: crate::fsl_mc_command = core::mem::zeroed();

    /* prepare command */
    cmd.header = crate::mc_encode_cmd_header(crate::DPMCP_CMDID_CLOSE, cmd_flags, token);

    /* send command to mc*/
    crate::mc_send_command(mc_io, &mut cmd)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
