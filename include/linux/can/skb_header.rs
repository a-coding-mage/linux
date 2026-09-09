/* SPDX-License-Identifier: (GPL-2.0 OR BSD-3-Clause) */
/*
 * linux/can/skb.h
 *
 * Definitions for the CAN network socket buffer
 *
 * Copyright (C) 2012 Oliver Hartkopp <socketcan@hartkopp.net>
 */

extern "C" {
    pub fn can_flush_echo_skb(dev: *mut net_device);
    pub fn can_put_echo_skb(
        skb: *mut sk_buff,
        dev: *mut net_device,
        idx: ::core::ffi::c_uint,
        frame_len: ::core::ffi::c_uint,
    ) -> ::core::ffi::c_int;
    pub fn __can_get_echo_skb(
        dev: *mut net_device,
        idx: ::core::ffi::c_uint,
        len_ptr: *mut ::core::ffi::c_uint,
        frame_len_ptr: *mut ::core::ffi::c_uint,
    ) -> *mut sk_buff;
    pub fn can_get_echo_skb(
        dev: *mut net_device,
        idx: ::core::ffi::c_uint,
        frame_len_ptr: *mut ::core::ffi::c_uint,
    ) -> ::core::ffi::c_uint;
    pub fn can_free_echo_skb(
        dev: *mut net_device,
        idx: ::core::ffi::c_uint,
        frame_len_ptr: *mut ::core::ffi::c_uint,
    );
    pub fn alloc_can_skb(dev: *mut net_device, cf: *mut *mut can_frame) -> *mut sk_buff;
    pub fn alloc_canfd_skb(dev: *mut net_device, cfd: *mut *mut canfd_frame) -> *mut sk_buff;
    pub fn alloc_canxl_skb(
        dev: *mut net_device,
        cxl: *mut *mut canxl_frame,
        data_len: ::core::ffi::c_uint,
    ) -> *mut sk_buff;
    pub fn alloc_can_err_skb(dev: *mut net_device, cf: *mut *mut can_frame) -> *mut sk_buff;
    pub fn can_dropped_invalid_skb(dev: *mut net_device, skb: *mut sk_buff) -> bool;
}

pub unsafe fn can_skb_ext_add(skb: *mut sk_buff) -> *mut can_skb_ext {
    let csx: *mut can_skb_ext = skb_ext_add(skb, SKB_EXT_CAN);

    /* skb_ext_add() returns uninitialized space */
    if !csx.is_null() {
        (*csx).can_gw_hops = 0;
    }

    csx
}

pub unsafe fn can_skb_ext_find(skb: *mut sk_buff) -> *mut can_skb_ext {
    skb_ext_find(skb, SKB_EXT_CAN)
}

pub unsafe fn can_skb_set_owner(skb: *mut sk_buff, sk: *mut sock) {
    /* If the socket has already been closed by user space, the
     * refcount may already be 0 (and the socket will be freed
     * after the last TX skb has been freed). So only increase
     * socket refcount if the refcount is > 0.
     */
    if !sk.is_null() && refcount_inc_not_zero(&mut (*sk).sk_refcnt) {
        (*skb).destructor = Some(sock_efree);
        (*skb).sk = sk;
    }
}

/*
 * returns an unshared skb owned by the original sock to be echo'ed back
 */
pub unsafe fn can_create_echo_skb(skb: *mut sk_buff) -> *mut sk_buff {
    let nskb: *mut sk_buff = skb_clone(skb, GFP_ATOMIC);
    if nskb.is_null() {
        kfree_skb(skb);
        return ::core::ptr::null_mut();
    }

    can_skb_set_owner(nskb, (*skb).sk);
    consume_skb(skb);
    nskb
}

pub unsafe fn can_is_can_skb(skb: *const sk_buff) -> bool {
    let cf: *const can_frame = (*skb).data as *const can_frame;

    /* the CAN specific type of skb is identified by its data length */
    (*skb).len == CAN_MTU && (*cf).len <= CAN_MAX_DLEN
}

pub unsafe fn can_is_canfd_skb(skb: *const sk_buff) -> bool {
    let cfd: *const canfd_frame = (*skb).data as *const canfd_frame;

    /* the CAN specific type of skb is identified by its data length */
    (*skb).len == CANFD_MTU && (*cfd).len <= CANFD_MAX_DLEN
}

pub unsafe fn can_is_canxl_skb(skb: *const sk_buff) -> bool {
    let cxl: *const canxl_frame = (*skb).data as *const canxl_frame;

    if (*skb).len < CANXL_HDR_SIZE + CANXL_MIN_DLEN || (*skb).len > CANXL_MTU {
        return false;
    }

    /* this also checks valid CAN XL data length boundaries */
    if (*skb).len != CANXL_HDR_SIZE + (*cxl).len {
        return false;
    }

    (*cxl).flags & CANXL_XLF != 0
}

/* get length element value from can[|fd|xl]_frame structure */
pub unsafe fn can_skb_get_len_val(skb: *mut sk_buff) -> ::core::ffi::c_uint {
    let cxl: *const canxl_frame = (*skb).data as *const canxl_frame;
    let cfd: *const canfd_frame = (*skb).data as *const canfd_frame;

    if can_is_canxl_skb(skb) {
        return (*cxl).len;
    }

    (*cfd).len
}

/* get needed data length inside CAN frame for all frame types (RTR aware) */
pub unsafe fn can_skb_get_data_len(skb: *mut sk_buff) -> ::core::ffi::c_uint {
    let len = can_skb_get_len_val(skb);
    let cf: *const can_frame = (*skb).data as *const can_frame;

    /* RTR frames have an actual length of zero */
    if can_is_can_skb(skb) && (*cf).can_id & CAN_RTR_FLAG != 0 {
        return 0;
    }

    len
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
