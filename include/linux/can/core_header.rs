/* SPDX-License-Identifier: (GPL-2.0 OR BSD-3-Clause) */
/*
 * linux/can/core.h
 *
 * Prototypes and definitions for CAN protocol modules using the PF_CAN core
 *
 * Authors: Oliver Hartkopp <oliver.hartkopp@volkswagen.de>
 *          Urs Thuermann   <urs.thuermann@volkswagen.de>
 * Copyright (c) 2002-2017 Volkswagen Group Electronic Research
 * All rights reserved.
 */

// Dependencies supplied by the surrounding Linux translation.

#[macro_export]
macro_rules! DNAME {
    ($dev:expr) => {
        if !$dev.is_null() {
            (*$dev).name
        } else {
            b"any\0".as_ptr() as *const _
        }
    };
}

/**
 * struct can_proto - CAN protocol structure
 * @type:       type argument in socket() syscall, e.g. SOCK_DGRAM.
 * @protocol:   protocol number in socket() syscall.
 * @ops:        pointer to struct proto_ops for sock->ops.
 * @prot:       pointer to struct proto structure.
 */
#[repr(C)]
pub struct can_proto {
    pub type_: ::core::ffi::c_int,
    pub protocol: ::core::ffi::c_int,
    pub ops: *const proto_ops,
    pub prot: *mut proto,
}

/* required_size
 * macro to find the minimum size of a struct
 * that includes a requested member
 *
 * Rust's `offset_of!` and `size_of` provide the corresponding layout
 * operations; the member expression is supplied by the caller.
 */
#[macro_export]
macro_rules! CAN_REQUIRED_SIZE {
    ($struct_type:ty, $member:ident) => {
        ::core::mem::offset_of!($struct_type, $member)
            + ::core::mem::size_of_val(unsafe {
                &(*(::core::ptr::null::<$struct_type>())).$member
            })
    };
}

/* function prototypes for the CAN networklayer core (af_can.c) */

unsafe extern "C" {
    pub fn can_proto_register(cp: *const can_proto) -> ::core::ffi::c_int;
    pub fn can_proto_unregister(cp: *const can_proto);

    pub fn can_rx_register(
        net: *mut net,
        dev: *mut net_device,
        can_id: canid_t,
        mask: canid_t,
        func: Option<unsafe extern "C" fn(*mut sk_buff, *mut ::core::ffi::c_void)>,
        data: *mut ::core::ffi::c_void,
        ident: *mut ::core::ffi::c_char,
        sk: *mut sock,
    ) -> ::core::ffi::c_int;

    pub fn can_rx_unregister(
        net: *mut net,
        dev: *mut net_device,
        can_id: canid_t,
        mask: canid_t,
        func: Option<unsafe extern "C" fn(*mut sk_buff, *mut ::core::ffi::c_void)>,
        data: *mut ::core::ffi::c_void,
    );

    pub fn can_send(skb: *mut sk_buff, loop_: ::core::ffi::c_int) -> ::core::ffi::c_int;
    pub fn can_set_skb_uid(skb: *mut sk_buff);
    pub fn can_sock_destruct(sk: *mut sock);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
