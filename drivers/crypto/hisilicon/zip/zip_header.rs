/* SPDX-License-Identifier: GPL-2.0 */
/* Copyright (c) 2019 HiSilicon Limited. */

// C header guard: HISI_ZIP_H
// C preprocessor formatting macro: pr_fmt(fmt) = "hisi_zip: " fmt
// Dependencies supplied by the surrounding kernel translation are intentionally
// referenced by their corresponding Rust names below.

#[repr(i32)]
pub enum hisi_zip_error_type {
    /* negative compression */
    HZIP_NC_ERR = 0x0d,
}

#[repr(C)]
pub struct hisi_zip_dfx {
    pub send_cnt: atomic64_t,
    pub recv_cnt: atomic64_t,
    pub send_busy_cnt: atomic64_t,
    pub err_bd_cnt: atomic64_t,
}

#[repr(C)]
pub struct hisi_zip_ctrl {
    _private: [u8; 0],
}

#[repr(C)]
pub struct hisi_zip {
    pub qm: hisi_qm,
    pub ctrl: *mut hisi_zip_ctrl,
    pub dfx: hisi_zip_dfx,
}

#[repr(C)]
pub struct hisi_zip_sqe {
    pub consumed: u32,
    pub produced: u32,
    pub comp_data_length: u32,
    /*
     * status: 0~7 bits
     * rsvd: 8~31 bits
     */
    pub dw3: u32,
    pub input_data_length: u32,
    pub dw5: u32,
    pub dw6: u32,
    /*
     * in_sge_data_offset: 0~23 bits
     * rsvd: 24~27 bits
     * sqe_type: 29~31 bits
     */
    pub dw7: u32,
    /*
     * out_sge_data_offset: 0~23 bits
     * rsvd: 24~31 bits
     */
    pub dw8: u32,
    /*
     * request_type: 0~7 bits
     * buffer_type: 8~11 bits
     * rsvd: 13~31 bits
     */
    pub dw9: u32,
    pub dw10: u32,
    pub dw11: u32,
    pub dw12: u32,
    /* tag: in sqe type 0 */
    pub dw13: u32,
    pub dest_avail_out: u32,
    pub dw15: u32,
    pub dw16: u32,
    pub dw17: u32,
    pub source_addr_l: u32,
    pub source_addr_h: u32,
    pub dest_addr_l: u32,
    pub dest_addr_h: u32,
    pub dw22: u32,
    pub dw23: u32,
    pub dw24: u32,
    pub dw25: u32,
    /* tag: in sqe type 3 */
    pub dw26: u32,
    pub dw27: u32,
    pub rsvd1: [u32; 4],
}

#[repr(i32)]
pub enum zip_cap_table_type {
    QM_RAS_NFE_TYPE,
    QM_RAS_NFE_RESET,
    QM_RAS_CE_TYPE,
    ZIP_RAS_NFE_TYPE,
    ZIP_RAS_NFE_RESET,
    ZIP_RAS_CE_TYPE,
    ZIP_CORE_INFO,
    ZIP_CORE_EN,
    ZIP_DRV_ALG_BITMAP_TB,
    ZIP_ALG_BITMAP,
    ZIP_CORE1_BITMAP,
    ZIP_CORE2_BITMAP,
    ZIP_CORE3_BITMAP,
    ZIP_CORE4_BITMAP,
    ZIP_CORE5_BITMAP,
}

extern "C" {
    pub fn zip_create_qps(qps: *mut *mut hisi_qp, qp_num: i32, node: i32, alg_type: *mut u8) -> i32;
    pub fn hisi_zip_register_to_crypto(qm: *mut hisi_qm) -> i32;
    pub fn hisi_zip_unregister_from_crypto(qm: *mut hisi_qm);
    pub fn hisi_zip_alg_support(qm: *mut hisi_qm, alg: u32) -> bool;
    pub fn hisi_dae_set_user_domain(qm: *mut hisi_qm) -> i32;
    pub fn hisi_dae_set_alg(qm: *mut hisi_qm) -> i32;
    pub fn hisi_dae_hw_error_disable(qm: *mut hisi_qm);
    pub fn hisi_dae_hw_error_enable(qm: *mut hisi_qm);
    pub fn hisi_dae_open_axi_master_ooo(qm: *mut hisi_qm);
    pub fn hisi_dae_close_axi_master_ooo(qm: *mut hisi_qm) -> i32;
    pub fn hisi_dae_dev_is_abnormal(qm: *mut hisi_qm) -> bool;
    pub fn hisi_dae_get_err_result(qm: *mut hisi_qm) -> acc_err_result;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
