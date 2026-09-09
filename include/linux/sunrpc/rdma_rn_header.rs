/* SPDX-License-Identifier: GPL-2.0 */
/*
 * * Copyright (c) 2024, Oracle and/or its affiliates.
 */

// Dependency intent preserved from <rdma/ib_verbs.h>.

#[repr(C)]
pub struct rpcrdma_notification {
    pub rn_done: Option<unsafe extern "C" fn(rn: *mut rpcrdma_notification)>,
    pub rn_index: u32,
}

unsafe extern "C" {
    pub fn rpcrdma_rn_register(
        device: *mut ib_device,
        rn: *mut rpcrdma_notification,
        done: Option<unsafe extern "C" fn(rn: *mut rpcrdma_notification)>,
    ) -> i32;
    pub fn rpcrdma_rn_unregister(
        device: *mut ib_device,
        rn: *mut rpcrdma_notification,
    );
    pub fn rpcrdma_ib_client_register() -> i32;
    pub fn rpcrdma_ib_client_unregister();
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
