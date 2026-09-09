/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (c) 2022 MediaTek Inc.
 */

// Dependencies supplied by the surrounding kernel translation.

pub const MTK_ADSP_IPC_REQ: ::core::ffi::c_int = 0;
pub const MTK_ADSP_IPC_RSP: ::core::ffi::c_int = 1;
pub const MTK_ADSP_IPC_OP_REQ: ::core::ffi::c_int = 0x1;
pub const MTK_ADSP_IPC_OP_RSP: ::core::ffi::c_int = 0x2;

pub const MTK_ADSP_MBOX_REPLY: usize = 0;
pub const MTK_ADSP_MBOX_REQUEST: usize = 1;
pub const MTK_ADSP_MBOX_NUM: usize = 2;

#[repr(C)]
pub struct mtk_adsp_ipc_ops {
    pub handle_reply: Option<unsafe extern "C" fn(ipc: *mut mtk_adsp_ipc)>,
    pub handle_request: Option<unsafe extern "C" fn(ipc: *mut mtk_adsp_ipc)>,
}

#[repr(C)]
pub struct mtk_adsp_chan {
    pub ipc: *mut mtk_adsp_ipc,
    pub cl: mbox_client,
    pub ch: *mut mbox_chan,
    pub name: *mut ::core::ffi::c_char,
    pub idx: ::core::ffi::c_int,
}

#[repr(C)]
pub struct mtk_adsp_ipc {
    pub chans: [mtk_adsp_chan; MTK_ADSP_MBOX_NUM],
    pub dev: *mut device,
    pub ops: *const mtk_adsp_ipc_ops,
    pub private_data: *mut ::core::ffi::c_void,
}

#[inline]
pub unsafe fn mtk_adsp_ipc_set_data(ipc: *mut mtk_adsp_ipc, data: *mut ::core::ffi::c_void) {
    (*ipc).private_data = data;
}

#[inline]
pub unsafe fn mtk_adsp_ipc_get_data(ipc: *mut mtk_adsp_ipc) -> *mut ::core::ffi::c_void {
    (*ipc).private_data
}

unsafe extern "C" {
    pub fn mtk_adsp_ipc_send(
        ipc: *mut mtk_adsp_ipc,
        idx: ::core::ffi::c_uint,
        op: u32,
    ) -> ::core::ffi::c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
