// SPDX-License-Identifier: (GPL-2.0+ OR BSD-3-Clause)
/*
 * Copyright 2021 NXP
 *
 */

use std::ffi::c_char;

extern "C" {
    fn strcmp(lhs: *const c_char, rhs: *const c_char) -> i32;
}

#[repr(C)]
struct FslMcDevId {
    cmd_id: i32,
    type_: *const c_char,
}

unsafe fn fsl_mc_get_open_cmd_id(type_: *const c_char) -> i32 {
    static DEV_IDS: &[FslMcDevId] = &[
        FslMcDevId { cmd_id: DPRTC_CMDID_OPEN, type_: b"dprtc\0".as_ptr() as *const c_char },
        FslMcDevId { cmd_id: DPRC_CMDID_OPEN, type_: b"dprc\0".as_ptr() as *const c_char },
        FslMcDevId { cmd_id: DPNI_CMDID_OPEN, type_: b"dpni\0".as_ptr() as *const c_char },
        FslMcDevId { cmd_id: DPIO_CMDID_OPEN, type_: b"dpio\0".as_ptr() as *const c_char },
        FslMcDevId { cmd_id: DPSW_CMDID_OPEN, type_: b"dpsw\0".as_ptr() as *const c_char },
        FslMcDevId { cmd_id: DPBP_CMDID_OPEN, type_: b"dpbp\0".as_ptr() as *const c_char },
        FslMcDevId { cmd_id: DPCON_CMDID_OPEN, type_: b"dpcon\0".as_ptr() as *const c_char },
        FslMcDevId { cmd_id: DPMCP_CMDID_OPEN, type_: b"dpmcp\0".as_ptr() as *const c_char },
        FslMcDevId { cmd_id: DPMAC_CMDID_OPEN, type_: b"dpmac\0".as_ptr() as *const c_char },
        FslMcDevId { cmd_id: DPSECI_CMDID_OPEN, type_: b"dpseci\0".as_ptr() as *const c_char },
        FslMcDevId { cmd_id: DPDMUX_CMDID_OPEN, type_: b"dpdmux\0".as_ptr() as *const c_char },
        FslMcDevId { cmd_id: DPDCEI_CMDID_OPEN, type_: b"dpdcei\0".as_ptr() as *const c_char },
        FslMcDevId { cmd_id: DPAIOP_CMDID_OPEN, type_: b"dpaiop\0".as_ptr() as *const c_char },
        FslMcDevId { cmd_id: DPCI_CMDID_OPEN, type_: b"dpci\0".as_ptr() as *const c_char },
        FslMcDevId { cmd_id: DPDMAI_CMDID_OPEN, type_: b"dpdmai\0".as_ptr() as *const c_char },
        FslMcDevId { cmd_id: DPDBG_CMDID_OPEN, type_: b"dpdbg\0".as_ptr() as *const c_char },
        FslMcDevId { cmd_id: 0, type_: std::ptr::null() },
    ];

    let mut i = 0usize;
    while !DEV_IDS[i].type_.is_null() {
        if strcmp(DEV_IDS[i].type_, type_) == 0 {
            return DEV_IDS[i].cmd_id;
        }
        i += 1;
    }

    -1
}

pub unsafe fn fsl_mc_obj_open(
    mc_io: *mut fsl_mc_io,
    cmd_flags: u32,
    obj_id: i32,
    obj_type: *mut c_char,
    token: *mut u16,
) -> i32 {
    let mut cmd: fsl_mc_command = std::mem::zeroed();
    let cmd_id = fsl_mc_get_open_cmd_id(obj_type as *const c_char);

    if cmd_id == -1 {
        return -ENODEV;
    }

    // prepare command
    cmd.header = mc_encode_cmd_header(cmd_id, cmd_flags, 0);
    let cmd_params = cmd.params.as_mut_ptr() as *mut fsl_mc_obj_cmd_open;
    (*cmd_params).obj_id = cpu_to_le32(obj_id);

    // send command to mc
    let err = mc_send_command(mc_io, &mut cmd);
    if err != 0 {
        return err;
    }

    // retrieve response parameters
    *token = mc_cmd_hdr_read_token(&cmd);

    err
}

pub unsafe fn fsl_mc_obj_close(
    mc_io: *mut fsl_mc_io,
    cmd_flags: u32,
    token: u16,
) -> i32 {
    let mut cmd: fsl_mc_command = std::mem::zeroed();

    // prepare command
    cmd.header = mc_encode_cmd_header(OBJ_CMDID_CLOSE, cmd_flags, token);

    // send command to mc
    mc_send_command(mc_io, &mut cmd)
}

pub unsafe fn fsl_mc_obj_reset(
    mc_io: *mut fsl_mc_io,
    cmd_flags: u32,
    token: u16,
) -> i32 {
    let mut cmd: fsl_mc_command = std::mem::zeroed();

    // prepare command
    cmd.header = mc_encode_cmd_header(OBJ_CMDID_RESET, cmd_flags, token);

    // send command to mc
    mc_send_command(mc_io, &mut cmd)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
