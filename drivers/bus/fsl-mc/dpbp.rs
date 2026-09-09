// SPDX-License-Identifier: (GPL-2.0+ OR BSD-3-Clause)
/*
 * Copyright 2013-2016 Freescale Semiconductor Inc.
 *
 */
// Dependencies supplied by the Linux MC interfaces and fsl-mc-private.h.

/**
 * dpbp_open() - Open a control session for the specified object.
 * @mc_io: Pointer to MC portal's I/O object
 * @cmd_flags: Command flags; one or more of 'MC_CMD_FLAG_'
 * @dpbp_id: DPBP unique ID
 * @token: Returned token; use in subsequent API calls
 *
 * This function can be used to open a control session for an
 * already created object; an object may have been declared in
 * the DPL or by calling the dpbp_create function.
 * This function returns a unique authentication token,
 * associated with the specific object ID and the specific MC
 * portal; this token must be used in all subsequent commands for
 * this specific object
 *
 * Return: '0' on Success; Error code otherwise.
 */
pub unsafe extern "C" fn dpbp_open(
    mc_io: *mut fsl_mc_io,
    cmd_flags: u32,
    dpbp_id: i32,
    token: *mut u16,
) -> i32 {
    let mut cmd: fsl_mc_command = core::mem::zeroed();
    let cmd_params: *mut dpbp_cmd_open;
    let err: i32;

    /* prepare command */
    cmd.header = mc_encode_cmd_header(DPBP_CMDID_OPEN, cmd_flags, 0);
    cmd_params = cmd.params.as_mut_ptr() as *mut dpbp_cmd_open;
    (*cmd_params).dpbp_id = cpu_to_le32(dpbp_id as u32);

    /* send command to mc*/
    err = mc_send_command(mc_io, &mut cmd);
    if err != 0 {
        return err;
    }

    /* retrieve response parameters */
    *token = mc_cmd_hdr_read_token(&cmd);

    err
}

/**
 * dpbp_close() - Close the control session of the object
 * @mc_io: Pointer to MC portal's I/O object
 * @cmd_flags: Command flags; one or more of 'MC_CMD_FLAG_'
 * @token: Token of DPBP object
 *
 * After this function is called, no further operations are
 * allowed on the object without opening a new control session.
 *
 * Return: '0' on Success; Error code otherwise.
 */
pub unsafe extern "C" fn dpbp_close(
    mc_io: *mut fsl_mc_io,
    cmd_flags: u32,
    token: u16,
) -> i32 {
    let mut cmd: fsl_mc_command = core::mem::zeroed();

    /* prepare command */
    cmd.header = mc_encode_cmd_header(DPBP_CMDID_CLOSE, cmd_flags, token);

    /* send command to mc*/
    mc_send_command(mc_io, &mut cmd)
}

/**
 * dpbp_enable() - Enable the DPBP.
 * @mc_io: Pointer to MC portal's I/O object
 * @cmd_flags: Command flags; one or more of 'MC_CMD_FLAG_'
 * @token: Token of DPBP object
 *
 * Return: '0' on Success; Error code otherwise.
 */
pub unsafe extern "C" fn dpbp_enable(
    mc_io: *mut fsl_mc_io,
    cmd_flags: u32,
    token: u16,
) -> i32 {
    let mut cmd: fsl_mc_command = core::mem::zeroed();

    /* prepare command */
    cmd.header = mc_encode_cmd_header(DPBP_CMDID_ENABLE, cmd_flags, token);

    /* send command to mc*/
    mc_send_command(mc_io, &mut cmd)
}

/**
 * dpbp_disable() - Disable the DPBP.
 * @mc_io: Pointer to MC portal's I/O object
 * @cmd_flags: Command flags; one or more of 'MC_CMD_FLAG_'
 * @token: Token of DPBP object
 *
 * Return: '0' on Success; Error code otherwise.
 */
pub unsafe extern "C" fn dpbp_disable(
    mc_io: *mut fsl_mc_io,
    cmd_flags: u32,
    token: u16,
) -> i32 {
    let mut cmd: fsl_mc_command = core::mem::zeroed();

    /* prepare command */
    cmd.header = mc_encode_cmd_header(DPBP_CMDID_DISABLE, cmd_flags, token);

    /* send command to mc*/
    mc_send_command(mc_io, &mut cmd)
}

/**
 * dpbp_reset() - Reset the DPBP, returns the object to initial state.
 * @mc_io: Pointer to MC portal's I/O object
 * @cmd_flags: Command flags; one or more of 'MC_CMD_FLAG_'
 * @token: Token of DPBP object
 *
 * Return: '0' on Success; Error code otherwise.
 */
pub unsafe extern "C" fn dpbp_reset(
    mc_io: *mut fsl_mc_io,
    cmd_flags: u32,
    token: u16,
) -> i32 {
    let mut cmd: fsl_mc_command = core::mem::zeroed();

    /* prepare command */
    cmd.header = mc_encode_cmd_header(DPBP_CMDID_RESET, cmd_flags, token);

    /* send command to mc*/
    mc_send_command(mc_io, &mut cmd)
}

/**
 * dpbp_get_attributes - Retrieve DPBP attributes.
 *
 * @mc_io: Pointer to MC portal's I/O object
 * @cmd_flags: Command flags; one or more of 'MC_CMD_FLAG_'
 * @token: Token of DPBP object
 * @attr: Returned object's attributes
 *
 * Return: '0' on Success; Error code otherwise.
 */
pub unsafe extern "C" fn dpbp_get_attributes(
    mc_io: *mut fsl_mc_io,
    cmd_flags: u32,
    token: u16,
    attr: *mut dpbp_attr,
) -> i32 {
    let mut cmd: fsl_mc_command = core::mem::zeroed();
    let rsp_params: *mut dpbp_rsp_get_attributes;
    let err: i32;

    /* prepare command */
    cmd.header = mc_encode_cmd_header(DPBP_CMDID_GET_ATTR, cmd_flags, token);

    /* send command to mc*/
    err = mc_send_command(mc_io, &mut cmd);
    if err != 0 {
        return err;
    }

    /* retrieve response parameters */
    rsp_params = cmd.params.as_mut_ptr() as *mut dpbp_rsp_get_attributes;
    (*attr).bpid = le16_to_cpu((*rsp_params).bpid);
    (*attr).id = le32_to_cpu((*rsp_params).id);

    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
