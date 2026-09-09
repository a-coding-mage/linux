/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (C) 2023-2024, Advanced Micro Devices, Inc.
 */

// Translated from amdxdna_mailbox_helper.h.

pub const TX_TIMEOUT: u32 = 2000; /* milliseconds */
pub const RX_TIMEOUT: u32 = 5000; /* milliseconds */

pub struct amdxdna_dev;

#[repr(C)]
pub struct xdna_notify {
    pub comp: completion,
    pub data: *mut u32,
    pub size: usize,
    pub error: i32,
    pub status: *mut u32,
}

/*
 * C macro DECLARE_XDNA_MSG_COMMON.  Rust has no token-pasting equivalent;
 * the request and response types are therefore supplied explicitly.
 */
#[macro_export]
macro_rules! declare_xdna_msg_common {
    ($req_ty:ty, $resp_ty:ty, $op:expr, $status:expr,
     $req:ident, $resp:ident, $hdl:ident, $msg:ident) => {
        let mut $req: $req_ty = unsafe { core::mem::zeroed() };
        let mut $resp: $resp_ty = $resp_ty { status: $status };
        let mut $hdl: xdna_notify = xdna_notify {
            error: 0,
            data: (&mut $resp as *mut $resp_ty).cast::<u32>(),
            size: core::mem::size_of::<$resp_ty>(),
            comp: unsafe { core::mem::zeroed() },
            status: (&mut $resp.status as *mut _).cast::<u32>(),
        };
        let mut $msg: xdna_mailbox_msg = xdna_mailbox_msg {
            send_data: (&mut $req as *mut $req_ty).cast::<u8>(),
            send_size: core::mem::size_of::<$req_ty>(),
            handle: &mut $hdl as *mut xdna_notify,
            opcode: $op,
            notify_cb: Some(xdna_msg_cb),
        };
    };
}

unsafe extern "C" {
    pub fn xdna_msg_cb(
        handle: *mut core::ffi::c_void,
        data: *mut core::ffi::c_void, /* __iomem */
        size: usize,
    ) -> i32;
    pub fn xdna_send_msg_wait(
        xdna: *mut amdxdna_dev,
        chann: *mut mailbox_channel,
        msg: *mut xdna_mailbox_msg,
    ) -> i32;
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
