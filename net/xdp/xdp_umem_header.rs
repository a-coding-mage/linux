/* SPDX-License-Identifier: GPL-2.0 */
/* XDP user-space packet buffer
 * Copyright(c) 2018 Intel Corporation.
 */

// Dependency equivalent of <net/xdp_sock_drv.h> is supplied externally.

#[repr(C)]
pub struct xdp_umem {
    _private: [u8; 0],
}

#[repr(C)]
pub struct xdp_umem_reg {
    _private: [u8; 0],
}

unsafe extern "C" {
    pub fn xdp_get_umem(umem: *mut xdp_umem);
    pub fn xdp_put_umem(umem: *mut xdp_umem, defer_cleanup: bool);
    pub fn xdp_umem_create(mr: *mut xdp_umem_reg) -> *mut xdp_umem;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
