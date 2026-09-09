/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * pcrypt - Parallel crypto engine.
 *
 * Copyright (C) 2009 secunet Security Networks AG
 * Copyright (C) 2009 Steffen Klassert <steffen.klassert@secunet.com>
 */

// Dependencies supplied by the surrounding kernel translation.
use core::ffi::c_void;

#[repr(C)]
pub struct pcrypt_request {
    pub padata: padata_priv,
    pub data: *mut c_void,
    pub __ctx: [*mut c_void; 0],
}

#[inline]
pub unsafe fn pcrypt_request_ctx(req: *mut pcrypt_request) -> *mut c_void {
    (*req).__ctx.as_mut_ptr() as *mut c_void
}

#[inline]
pub unsafe fn pcrypt_request_padata(req: *mut pcrypt_request) -> *mut padata_priv {
    core::ptr::addr_of_mut!((*req).padata)
}

#[inline]
pub unsafe fn pcrypt_padata_request(padata: *mut padata_priv) -> *mut pcrypt_request {
    (padata as *mut u8).sub(core::mem::offset_of!(pcrypt_request, padata))
        as *mut pcrypt_request
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
