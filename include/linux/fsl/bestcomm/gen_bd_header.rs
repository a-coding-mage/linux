/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Header for Bestcomm General Buffer Descriptor tasks driver
 *
 * Copyright (C) 2007 Sylvain Munaut <tnt@246tNt.com>
 * Copyright (C) 2006 AppSpec Computer Technologies Corp.
 *                    Jeff Gibbons <jeff.gibbons@appspec.com>
 */

#[repr(C)]
pub struct bcom_gen_bd {
    pub status: u32,
    pub buf_pa: u32,
}

extern "C" {
    pub fn bcom_gen_bd_rx_init(
        queue_len: ::core::ffi::c_int,
        fifo: phys_addr_t,
        initiator: ::core::ffi::c_int,
        ipr: ::core::ffi::c_int,
        maxbufsize: ::core::ffi::c_int,
    ) -> *mut bcom_task;

    pub fn bcom_gen_bd_rx_reset(tsk: *mut bcom_task) -> ::core::ffi::c_int;

    pub fn bcom_gen_bd_rx_release(tsk: *mut bcom_task);

    pub fn bcom_gen_bd_tx_init(
        queue_len: ::core::ffi::c_int,
        fifo: phys_addr_t,
        initiator: ::core::ffi::c_int,
        ipr: ::core::ffi::c_int,
    ) -> *mut bcom_task;

    pub fn bcom_gen_bd_tx_reset(tsk: *mut bcom_task) -> ::core::ffi::c_int;

    pub fn bcom_gen_bd_tx_release(tsk: *mut bcom_task);

    /* PSC support utility wrappers */
    pub fn bcom_psc_gen_bd_rx_init(
        psc_num: ::core::ffi::c_uint,
        queue_len: ::core::ffi::c_int,
        fifo: phys_addr_t,
        maxbufsize: ::core::ffi::c_int,
    ) -> *mut bcom_task;

    pub fn bcom_psc_gen_bd_tx_init(
        psc_num: ::core::ffi::c_uint,
        queue_len: ::core::ffi::c_int,
        fifo: phys_addr_t,
    ) -> *mut bcom_task;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
