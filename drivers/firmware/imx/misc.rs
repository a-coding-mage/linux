// SPDX-License-Identifier: GPL-2.0+
/*
 * Copyright (C) 2016 Freescale Semiconductor, Inc.
 * Copyright 2017~2018 NXP
 *  Author: Dong Aisheng <aisheng.dong@nxp.com>
 *
 * File containing client-side RPC functions for the MISC service. These
 * function are ported to clients that communicate to the SC.
 */

// Dependency supplied by linux/firmware/imx/svc/misc.h.

#[repr(C, packed(1), align(4))]
struct imx_sc_msg_req_misc_set_ctrl {
    hdr: imx_sc_rpc_msg,
    ctrl: u32,
    val: u32,
    resource: u16,
}

#[repr(C, packed(1), align(4))]
struct imx_sc_msg_req_cpu_start {
    hdr: imx_sc_rpc_msg,
    address_hi: u32,
    address_lo: u32,
    resource: u16,
    enable: u8,
}

#[repr(C, packed(1), align(4))]
struct imx_sc_msg_req_misc_get_ctrl {
    hdr: imx_sc_rpc_msg,
    ctrl: u32,
    resource: u16,
}

#[repr(C, packed(1), align(4))]
struct imx_sc_msg_resp_misc_get_ctrl {
    hdr: imx_sc_rpc_msg,
    val: u32,
}

/*
 * This function sets a miscellaneous control value.
 *
 * @param[in]     ipc         IPC handle
 * @param[in]     resource    resource the control is associated with
 * @param[in]     ctrl        control to change
 * @param[in]     val         value to apply to the control
 *
 * @return Returns 0 for success and < 0 for errors.
 */
pub unsafe fn imx_sc_misc_set_control(
    ipc: *mut imx_sc_ipc,
    resource: u32,
    ctrl: u8,
    val: u32,
) -> i32 {
    let mut msg: imx_sc_msg_req_misc_set_ctrl = core::mem::zeroed();
    let hdr: *mut imx_sc_rpc_msg = &mut msg.hdr;

    (*hdr).ver = IMX_SC_RPC_VERSION;
    (*hdr).svc = IMX_SC_RPC_SVC_MISC as u8;
    (*hdr).func = IMX_SC_MISC_FUNC_SET_CONTROL as u8;
    (*hdr).size = 4;

    msg.ctrl = ctrl as u32;
    msg.val = val;
    msg.resource = resource as u16;

    imx_scu_call_rpc(ipc, &mut msg as *mut _ as *mut core::ffi::c_void, true)
}

/*
 * This function gets a miscellaneous control value.
 *
 * @param[in]     ipc         IPC handle
 * @param[in]     resource    resource the control is associated with
 * @param[in]     ctrl        control to get
 * @param[out]    val         pointer to return the control value
 *
 * @return Returns 0 for success and < 0 for errors.
 */
pub unsafe fn imx_sc_misc_get_control(
    ipc: *mut imx_sc_ipc,
    resource: u32,
    ctrl: u8,
    val: *mut u32,
) -> i32 {
    let mut msg: imx_sc_msg_req_misc_get_ctrl = core::mem::zeroed();

    let hdr: *mut imx_sc_rpc_msg = &mut msg.hdr;
    (*hdr).ver = IMX_SC_RPC_VERSION;
    (*hdr).svc = IMX_SC_RPC_SVC_MISC as u8;
    (*hdr).func = IMX_SC_MISC_FUNC_GET_CONTROL as u8;
    (*hdr).size = 3;

    msg.ctrl = ctrl as u32;
    msg.resource = resource as u16;

    let ret = imx_scu_call_rpc(ipc, &mut msg as *mut _ as *mut core::ffi::c_void, true);
    if ret != 0 {
        return ret;
    }

    let resp = &*((&mut msg as *mut imx_sc_msg_req_misc_get_ctrl)
        as *mut imx_sc_msg_resp_misc_get_ctrl);
    if !val.is_null() {
        *val = resp.val;
    }

    0
}

/*
 * This function starts/stops a CPU identified by @resource
 *
 * @param[in]     ipc         IPC handle
 * @param[in]     resource    resource the control is associated with
 * @param[in]     enable      true for start, false for stop
 * @param[in]     phys_addr   initial instruction address to be executed
 *
 * @return Returns 0 for success and < 0 for errors.
 */
pub unsafe fn imx_sc_pm_cpu_start(
    ipc: *mut imx_sc_ipc,
    resource: u32,
    enable: bool,
    phys_addr: u64,
) -> i32 {
    let mut msg: imx_sc_msg_req_cpu_start = core::mem::zeroed();
    let hdr: *mut imx_sc_rpc_msg = &mut msg.hdr;

    (*hdr).ver = IMX_SC_RPC_VERSION;
    (*hdr).svc = IMX_SC_RPC_SVC_PM;
    (*hdr).func = IMX_SC_PM_FUNC_CPU_START;
    (*hdr).size = 4;

    msg.address_hi = (phys_addr >> 32) as u32;
    msg.address_lo = phys_addr as u32;
    msg.resource = resource as u16;
    msg.enable = enable as u8;

    imx_scu_call_rpc(ipc, &mut msg as *mut _ as *mut core::ffi::c_void, true)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
