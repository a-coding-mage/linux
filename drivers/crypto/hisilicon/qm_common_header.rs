/* SPDX-License-Identifier: GPL-2.0 */
/* Copyright (c) 2022 HiSilicon Limited. */

// Dependency types supplied by the surrounding translation are intentionally
// referenced here rather than redefined.

pub const QM_DBG_READ_LEN: u32 = 256;

#[repr(C)]
pub struct qm_cqe {
    pub rsvd0: __le32,
    pub cmd_id: __le16,
    pub rsvd1: __le16,
    pub sq_head: __le16,
    pub sq_num: __le16,
    pub rsvd2: __le16,
    pub w7: __le16,
}

#[repr(C)]
pub struct qm_eqe {
    pub dw0: __le32,
}

#[repr(C)]
pub struct qm_aeqe {
    pub dw0: __le32,
}

#[repr(C)]
pub struct qm_sqc {
    pub head: __le16,
    pub tail: __le16,
    pub base_l: __le32,
    pub base_h: __le32,
    pub dw3: __le32,
    pub w8: __le16,
    pub rsvd0: __le16,
    pub pasid: __le16,
    pub w11: __le16,
    pub cq_num: __le16,
    pub w13: __le16,
    pub rsvd1: __le32,
}

#[repr(C)]
pub struct qm_cqc {
    pub head: __le16,
    pub tail: __le16,
    pub base_l: __le32,
    pub base_h: __le32,
    pub dw3: __le32,
    pub w8: __le16,
    pub rsvd0: __le16,
    pub pasid: __le16,
    pub w11: __le16,
    pub dw6: __le32,
    pub rsvd1: __le32,
}

#[repr(C)]
pub struct qm_eqc {
    pub head: __le16,
    pub tail: __le16,
    pub base_l: __le32,
    pub base_h: __le32,
    pub dw3: __le32,
    pub rsvd: [__le32; 2],
    pub dw6: __le32,
}

#[repr(C)]
pub struct qm_aeqc {
    pub head: __le16,
    pub tail: __le16,
    pub base_l: __le32,
    pub base_h: __le32,
    pub dw3: __le32,
    pub rsvd: [__le32; 2],
    pub dw6: __le32,
}

unsafe extern "C" {
    pub fn qm_set_and_get_xqc(
        qm: *mut hisi_qm,
        cmd: u8,
        xqc: *mut core::ffi::c_void,
        qp_id: u32,
        op: bool,
    ) -> i32;
    pub fn hisi_qm_show_last_dfx_regs(qm: *mut hisi_qm);
    pub fn hisi_qm_set_algqos_init(qm: *mut hisi_qm);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
