/* SPDX-License-Identifier: MIT */
/*
 * Copyright (c) 2016, Citrix Systems Inc
 */

// C header guard: __XEN_PUBLIC_HVM_DM_OP_H__

#[repr(C)]
pub struct xen_dm_op_buf {
    pub h: *mut core::ffi::c_void,
    pub size: xen_ulong_t,
}

// DEFINE_GUEST_HANDLE_STRUCT(xen_dm_op_buf);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
