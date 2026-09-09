/* SPDX-License-Identifier: GPL-2.0 */

// Translated from rdma_transport.h.
// Dependencies supplied by <rdma/ib_verbs.h>, <rdma/rdma_cm.h>, and "rds.h"
// remain external to this translation.

/* RDMA_CM also uses 16385 as the listener port. */
pub const RDS_CM_PORT: u32 = 16385;

pub const RDS_RDMA_RESOLVE_TIMEOUT_MS: u32 = 5000;

/* Below reject reason is for legacy interoperability issue with non-linux
 * RDS endpoints where older version incompatibility is conveyed via value 1.
 * For future version(s), proper encoded reject reason should be used.
 */
pub const RDS_RDMA_REJ_INCOMPAT: u32 = 1;

// Opaque declarations corresponding to external C types.
#[repr(C)]
pub struct rdma_cm_id {
    _private: [u8; 0],
}

#[repr(C)]
pub struct rdma_cm_event {
    _private: [u8; 0],
}

#[repr(C)]
pub struct rds_transport {
    _private: [u8; 0],
}

extern "C" {
    pub fn rds_rdma_cm_event_handler(
        cm_id: *mut rdma_cm_id,
        event: *mut rdma_cm_event,
    ) -> ::core::ffi::c_int;

    pub fn rds6_rdma_cm_event_handler(
        cm_id: *mut rdma_cm_id,
        event: *mut rdma_cm_event,
    ) -> ::core::ffi::c_int;

    /* from ib.c */
    pub static mut rds_ib_transport: rds_transport;
    pub fn rds_ib_init() -> ::core::ffi::c_int;
    pub fn rds_ib_exit();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
