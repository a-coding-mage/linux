/* SPDX-License-Identifier: GPL-2.0 OR Linux-OpenIB */
/*
 * Copyright (c) 2005-2006 Intel Corporation.  All rights reserved.
 */

// Dependencies supplied by the corresponding RDMA headers:
// <rdma/ib_verbs.h>
// <rdma/ib_sa.h>
// <rdma/ib_user_verbs.h>
// <rdma/ib_user_sa.h>

extern "C" {
    pub fn ib_copy_qp_attr_to_user(
        device: *mut ib_device,
        dst: *mut ib_uverbs_qp_attr,
        src: *mut ib_qp_attr,
    );

    pub fn ib_copy_ah_attr_to_user(
        device: *mut ib_device,
        dst: *mut ib_uverbs_ah_attr,
        src: *mut rdma_ah_attr,
    );

    pub fn ib_copy_path_rec_to_user(
        dst: *mut ib_user_path_rec,
        src: *mut sa_path_rec,
    );
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
