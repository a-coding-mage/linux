// SPDX-License-Identifier: GPL-2.0 OR BSD-3-Clause
/*
 * Copyright (c) 2024 Oracle.  All rights reserved.
 */

// External Linux/RDMA declarations and trace infrastructure are supplied by
// the surrounding translation unit.

#[repr(C)]
pub struct rpcrdma_device {
    pub rd_kref: kref,
    pub rd_flags: ::core::ffi::c_ulong,
    pub rd_device: *mut ib_device,
    pub rd_xa: xarray,
    pub rd_done: completion,
}

pub const RPCRDMA_RD_F_REMOVING: usize = 0;

static mut rpcrdma_ib_client: ib_client = ib_client {
    name: "rpcrdma".as_ptr() as *const ::core::ffi::c_char,
    add: Some(rpcrdma_add_one),
    remove: Some(rpcrdma_remove_one),
};

/*
 * Listeners have no associated device, so we never register them.
 * Note that ib_get_client_data() does not check if @device is
 * NULL for us.
 */
unsafe fn rpcrdma_get_client_data(device: *mut ib_device) -> *mut rpcrdma_device {
    if device.is_null() {
        return core::ptr::null_mut();
    }
    ib_get_client_data(device, &raw mut rpcrdma_ib_client) as *mut rpcrdma_device
}

pub unsafe fn rpcrdma_rn_register(
    device: *mut ib_device,
    rn: *mut rpcrdma_notification,
    done: Option<unsafe extern "C" fn(*mut rpcrdma_notification)>,
) -> i32 {
    let rd = rpcrdma_get_client_data(device);

    if rd.is_null() || test_bit(RPCRDMA_RD_F_REMOVING, &raw mut (*rd).rd_flags) != 0 {
        return -ENETUNREACH;
    }

    (*rn).rn_done = done;
    if xa_alloc(
        &raw mut (*rd).rd_xa,
        &raw mut (*rn).rn_index,
        rn,
        xa_limit_32b,
        GFP_KERNEL,
    ) < 0 {
        (*rn).rn_done = None;
        return -ENOMEM;
    }
    kref_get(&raw mut (*rd).rd_kref);
    trace_rpcrdma_client_register(device, rn);
    0
}

unsafe extern "C" fn rpcrdma_rn_release(kref: *mut kref) {
    let rd = container_of!(kref, rpcrdma_device, rd_kref);

    trace_rpcrdma_client_completion((*rd).rd_device);
    complete(&raw mut (*rd).rd_done);
}

pub unsafe fn rpcrdma_rn_unregister(
    device: *mut ib_device,
    rn: *mut rpcrdma_notification,
) {
    let rd = rpcrdma_get_client_data(device);

    if rd.is_null() {
        return;
    }
    if (*rn).rn_done.is_none() {
        return;
    }
    (*rn).rn_done = None;

    trace_rpcrdma_client_unregister(device, rn);
    xa_erase(&raw mut (*rd).rd_xa, (*rn).rn_index);
    kref_put(&raw mut (*rd).rd_kref, Some(rpcrdma_rn_release));
}

unsafe extern "C" fn rpcrdma_add_one(device: *mut ib_device) -> i32 {
    let rd = kzalloc_obj::<rpcrdma_device>();
    if rd.is_null() {
        return -ENOMEM;
    }

    kref_init(&raw mut (*rd).rd_kref);
    xa_init_flags(&raw mut (*rd).rd_xa, XA_FLAGS_ALLOC);
    (*rd).rd_device = device;
    init_completion(&raw mut (*rd).rd_done);
    ib_set_client_data(device, &raw mut rpcrdma_ib_client, rd as *mut _);

    trace_rpcrdma_client_add_one(device);
    0
}

unsafe extern "C" fn rpcrdma_remove_one(
    device: *mut ib_device,
    client_data: *mut ::core::ffi::c_void,
) {
    let rd = client_data as *mut rpcrdma_device;
    let mut index: ::core::ffi::c_ulong = 0;

    trace_rpcrdma_client_remove_one(device);

    set_bit(RPCRDMA_RD_F_REMOVING, &raw mut (*rd).rd_flags);
    xa_for_each!(&raw mut (*rd).rd_xa, index, rn: *mut rpcrdma_notification, {
        ((*rn).rn_done.unwrap())(rn);
    });

    if !refcount_dec_and_test(&raw mut (*rd).rd_kref.refcount) {
        trace_rpcrdma_client_wait_on(device);
        wait_for_completion(&raw mut (*rd).rd_done);
    }

    trace_rpcrdma_client_remove_one_done(device);
    xa_destroy(&raw mut (*rd).rd_xa);
    kfree(rd as *mut ::core::ffi::c_void);
}

pub unsafe fn rpcrdma_ib_client_unregister() {
    ib_unregister_client(&raw mut rpcrdma_ib_client);
}

pub unsafe fn rpcrdma_ib_client_register() -> i32 {
    ib_register_client(&raw mut rpcrdma_ib_client)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
