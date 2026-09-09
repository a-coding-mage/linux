/* SPDX-License-Identifier: (BSD-3-Clause OR GPL-2.0-only) */
/* Copyright(c) 2022 Intel Corporation */

// C dependencies: <linux/list.h> and "adf_transport_internal.h".

#[repr(C)]
pub struct qat_instance_backlog {
    pub list: list_head,
    pub lock: spinlock_t, /* protects backlog list */
}

#[repr(C)]
pub struct qat_alg_req {
    pub fw_req: *mut u32,
    pub tx_ring: *mut adf_etr_ring_data,
    pub base: *mut crypto_async_request,
    pub list: list_head,
    pub backlog: *mut qat_instance_backlog,
}

extern "C" {
    pub fn qat_alg_send_message(req: *mut qat_alg_req) -> i32;
    pub fn qat_alg_send_backlog(backlog: *mut qat_instance_backlog);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
