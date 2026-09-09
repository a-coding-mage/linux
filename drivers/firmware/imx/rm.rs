// SPDX-License-Identifier: GPL-2.0+
/*
 * Copyright 2020 NXP
 *
 * File containing client-side RPC functions for the RM service. These
 * function are ported to clients that communicate to the SC.
 */

// Dependency declarations and constants are supplied by linux/firmware/imx/svc/rm.h.

#[repr(C, packed(4))]
pub struct imx_sc_msg_rm_rsrc_owned {
    pub hdr: imx_sc_rpc_msg,
    pub resource: u16,
}

/*
 * This function check @resource is owned by current partition or not
 *
 * @param[in]     ipc         IPC handle
 * @param[in]     resource    resource the control is associated with
 *
 * @return Returns 0 for not owned and 1 for owned.
 */
pub unsafe fn imx_sc_rm_is_resource_owned(
    ipc: *mut imx_sc_ipc,
    resource: u16,
) -> bool {
    let mut msg: imx_sc_msg_rm_rsrc_owned = core::mem::zeroed();
    let hdr: *mut imx_sc_rpc_msg = core::ptr::addr_of_mut!(msg.hdr);

    (*hdr).ver = IMX_SC_RPC_VERSION;
    (*hdr).svc = IMX_SC_RPC_SVC_RM;
    (*hdr).func = IMX_SC_RM_FUNC_IS_RESOURCE_OWNED;
    (*hdr).size = 2;

    msg.resource = resource;

    /*
     * SCU firmware only returns value 0 or 1
     * for resource owned check which means not owned or owned.
     * So it is always successful.
     */
    imx_scu_call_rpc(ipc, core::ptr::addr_of_mut!(msg).cast(), true);

    (*hdr).func != 0
}

#[repr(C)]
pub union imx_sc_msg_rm_get_resource_owner_data {
    pub req: imx_sc_msg_rm_get_resource_owner_req,
    pub resp: imx_sc_msg_rm_get_resource_owner_resp,
}

#[repr(C)]
pub struct imx_sc_msg_rm_get_resource_owner_req {
    pub resource: u16,
}

#[repr(C)]
pub struct imx_sc_msg_rm_get_resource_owner_resp {
    pub val: u8,
}

#[repr(C, packed(4))]
pub struct imx_sc_msg_rm_get_resource_owner {
    pub hdr: imx_sc_rpc_msg,
    pub data: imx_sc_msg_rm_get_resource_owner_data,
}

/*
 * This function get @resource partition number
 *
 * @param[in]     ipc         IPC handle
 * @param[in]     resource    resource the control is associated with
 * @param[out]    pt          pointer to return the partition number
 *
 * @return Returns 0 for success and < 0 for errors.
 */
pub unsafe fn imx_sc_rm_get_resource_owner(
    ipc: *mut imx_sc_ipc,
    resource: u16,
    pt: *mut u8,
) -> i32 {
    let mut msg: imx_sc_msg_rm_get_resource_owner = core::mem::zeroed();
    let hdr: *mut imx_sc_rpc_msg = core::ptr::addr_of_mut!(msg.hdr);

    (*hdr).ver = IMX_SC_RPC_VERSION;
    (*hdr).svc = IMX_SC_RPC_SVC_RM;
    (*hdr).func = IMX_SC_RM_FUNC_GET_RESOURCE_OWNER;
    (*hdr).size = 2;

    (*core::ptr::addr_of_mut!(msg.data)).req.resource = resource;

    let ret = imx_scu_call_rpc(ipc, core::ptr::addr_of_mut!(msg).cast(), true);
    if ret != 0 {
        return ret;
    }

    if !pt.is_null() {
        *pt = (*core::ptr::addr_of!(msg.data)).resp.val;
    }

    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
