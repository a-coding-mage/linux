/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (c) 2020, Oracle and/or its affiliates.
 */

/*
 * The rpc_rdma_cid struct records completion ID information. A
 * completion ID matches an incoming Send or Receive completion
 * to a Completion Queue and to a previous ib_post_*(). The ID
 * can then be displayed in an error message or recorded in a
 * trace record.
 *
 * This struct is shared between the server and client RPC/RDMA
 * transport implementations.
 */
#[repr(C)]
pub struct rpc_rdma_cid {
    pub ci_queue_id: u32,
    pub ci_completion_id: i32,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
