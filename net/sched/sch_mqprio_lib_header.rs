/* SPDX-License-Identifier: GPL-2.0 */

// Translated from sch_mqprio_lib.h.
// linux/types.h and the definitions of TC_QOPT_MAX_QUEUE are supplied by
// external dependencies in the surrounding translation unit.

#[repr(C)]
pub struct net_device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct netlink_ext_ack {
    _private: [u8; 0],
}

#[repr(C)]
pub struct tc_mqprio_qopt {
    _private: [u8; 0],
}

#[repr(C)]
pub struct tc_mqprio_qopt_offload {
    _private: [u8; 0],
}

extern "C" {
    pub fn mqprio_validate_qopt(
        dev: *mut net_device,
        qopt: *mut tc_mqprio_qopt,
        validate_queue_counts: bool,
        allow_overlapping_txqs: bool,
        extack: *mut netlink_ext_ack,
    ) -> core::ffi::c_int;

    pub fn mqprio_qopt_reconstruct(
        dev: *mut net_device,
        qopt: *mut tc_mqprio_qopt,
    );

    pub fn mqprio_fp_to_offload(
        fp: *mut u32,
        mqprio: *mut tc_mqprio_qopt_offload,
    );
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
