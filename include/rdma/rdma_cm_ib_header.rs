/* SPDX-License-Identifier: GPL-2.0 OR Linux-OpenIB */
/*
 * Copyright (c) 2006 Intel Corporation.  All rights reserved.
 */

// Dependency supplied by the surrounding RDMA bindings:
// #include <rdma/rdma_cm.h>

/**
 * rdma_set_ib_path - Manually sets the path record used to establish a
 *   connection.
 * @id: Connection identifier associated with the request.
 * @path_rec: Reference to the path record
 *
 * This call permits a user to specify routing information for rdma_cm_id's
 * bound to InfiniBand devices. It is called on the client side of a
 * connection and replaces the call to rdma_resolve_route.
 */
unsafe extern "C" {
    pub fn rdma_set_ib_path(
        id: *mut rdma_cm_id,
        path_rec: *mut sa_path_rec,
    ) -> ::std::os::raw::c_int;
}

/* Global qkey for UDP QPs and multicast groups. */
pub const RDMA_UDP_QKEY: u32 = 0x01234567;


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
