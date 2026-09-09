// SPDX-License-Identifier: GPL-2.0 OR BSD-3-Clause
/*
 * Copyright (c) 2015, 2017 Oracle.  All rights reserved.
 */

/* rpcrdma.ko module initialization
 */

// Dependencies supplied by the Linux kernel and the surrounding translation unit.

extern "C" {
    fn xprt_rdma_cleanup();
    fn svc_rdma_cleanup();
    fn rpcrdma_ib_client_unregister();
    fn rpcrdma_ib_client_register() -> ::core::ffi::c_int;
    fn svc_rdma_init() -> ::core::ffi::c_int;
    fn xprt_rdma_init() -> ::core::ffi::c_int;
}

// MODULE_AUTHOR("Open Grid Computing and Network Appliance, Inc.");
// MODULE_DESCRIPTION("RPC/RDMA Transport");
// MODULE_LICENSE("Dual BSD/GPL");
// MODULE_ALIAS("svcrdma");
// MODULE_ALIAS("xprtrdma");
// MODULE_ALIAS("rpcrdma6");

unsafe fn rpc_rdma_cleanup() {
    xprt_rdma_cleanup();
    svc_rdma_cleanup();
    rpcrdma_ib_client_unregister();
}

unsafe fn rpc_rdma_init() -> ::core::ffi::c_int {
    let mut rc: ::core::ffi::c_int;

    rc = rpcrdma_ib_client_register();
    if rc != 0 {
        return rc;
    }

    rc = svc_rdma_init();
    if rc != 0 {
        rpcrdma_ib_client_unregister();
        return rc;
    }

    rc = xprt_rdma_init();
    if rc != 0 {
        svc_rdma_cleanup();
        rpcrdma_ib_client_unregister();
        return rc;
    }

    0
}

// module_init(rpc_rdma_init);
// module_exit(rpc_rdma_cleanup);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
