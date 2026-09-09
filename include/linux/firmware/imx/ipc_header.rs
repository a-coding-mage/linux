/* SPDX-License-Identifier: GPL-2.0+ */
/*
 * Copyright 2018 NXP
 *
 * Header file for the IPC implementation.
 */

// C dependencies: <linux/device.h> and <linux/types.h>.

pub const IMX_SC_RPC_VERSION: u32 = 1;
pub const IMX_SC_RPC_MAX_MSG: u32 = 8;

pub struct imx_sc_ipc {
    _private: [u8; 0],
}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum imx_sc_rpc_svc {
    IMX_SC_RPC_SVC_UNKNOWN = 0,
    IMX_SC_RPC_SVC_RETURN = 1,
    IMX_SC_RPC_SVC_PM = 2,
    IMX_SC_RPC_SVC_RM = 3,
    IMX_SC_RPC_SVC_TIMER = 5,
    IMX_SC_RPC_SVC_PAD = 6,
    IMX_SC_RPC_SVC_MISC = 7,
    IMX_SC_RPC_SVC_IRQ = 8,
}

#[repr(C)]
pub struct imx_sc_rpc_msg {
    pub ver: u8,
    pub size: u8,
    pub svc: u8,
    pub func: u8,
}

#[cfg(feature = "CONFIG_IMX_SCU")]
extern "C" {
    /*
     * This is an function to send an RPC message over an IPC channel.
     * It is called by client-side SCFW API function shims.
     *
     * @param[in]     ipc         IPC handle
     * @param[in,out] msg         handle to a message
     * @param[in]     have_resp   response flag
     *
     * If have_resp is true then this function waits for a response
     * and returns the result in msg.
     */
    pub fn imx_scu_call_rpc(
        ipc: *mut imx_sc_ipc,
        msg: *mut core::ffi::c_void,
        have_resp: bool,
    ) -> i32;

    /*
     * This function gets the default ipc handle used by SCU
     *
     * @param[out]  ipc sc ipc handle
     *
     * @return Returns an error code (0 = success, failed if < 0)
     */
    pub fn imx_scu_get_handle(ipc: *mut *mut imx_sc_ipc) -> i32;
}

#[cfg(not(feature = "CONFIG_IMX_SCU"))]
#[inline]
pub unsafe fn imx_scu_call_rpc(
    _ipc: *mut imx_sc_ipc,
    _msg: *mut core::ffi::c_void,
    _have_resp: bool,
) -> i32 {
    // C dependency: ENOTSUPP (Linux errno value).
    -524
}

#[cfg(not(feature = "CONFIG_IMX_SCU"))]
#[inline]
pub unsafe fn imx_scu_get_handle(_ipc: *mut *mut imx_sc_ipc) -> i32 {
    // C dependency: ENOTSUPP (Linux errno value).
    -524
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
