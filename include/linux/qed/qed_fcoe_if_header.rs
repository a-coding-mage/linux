/* SPDX-License-Identifier: (GPL-2.0-only OR BSD-3-Clause) */
/* Copyright (c) 2019-2020 Marvell International Ltd. */

/* Translated from qed_fcoe_if.h. */

#[repr(C)]
pub struct qed_fcoe_stats {
    pub fcoe_rx_byte_cnt: u64,
    pub fcoe_rx_data_pkt_cnt: u64,
    pub fcoe_rx_xfer_pkt_cnt: u64,
    pub fcoe_rx_other_pkt_cnt: u64,
    pub fcoe_silent_drop_pkt_cmdq_full_cnt: u32,
    pub fcoe_silent_drop_pkt_rq_full_cnt: u32,
    pub fcoe_silent_drop_pkt_crc_error_cnt: u32,
    pub fcoe_silent_drop_pkt_task_invalid_cnt: u32,
    pub fcoe_silent_drop_total_pkt_cnt: u32,

    pub fcoe_tx_byte_cnt: u64,
    pub fcoe_tx_data_pkt_cnt: u64,
    pub fcoe_tx_xfer_pkt_cnt: u64,
    pub fcoe_tx_other_pkt_cnt: u64,
}

#[repr(C)]
pub struct qed_dev_fcoe_info {
    pub common: qed_dev_info,

    pub primary_dbq_rq_addr: *mut core::ffi::c_void,
    pub secondary_bdq_rq_addr: *mut core::ffi::c_void,

    pub wwpn: u64,
    pub wwnn: u64,

    pub num_cqs: u8,
}

#[repr(C)]
pub struct qed_fcoe_params_offload {
    pub sq_pbl_addr: dma_addr_t,
    pub sq_curr_page_addr: dma_addr_t,
    pub sq_next_page_addr: dma_addr_t,

    pub src_mac: [u8; ETH_ALEN],
    pub dst_mac: [u8; ETH_ALEN],

    pub tx_max_fc_pay_len: u16,
    pub e_d_tov_timer_val: u16,
    pub rec_tov_timer_val: u16,
    pub rx_max_fc_pay_len: u16,
    pub vlan_tag: u16,

    pub s_id: fc_addr_nw,
    pub max_conc_seqs_c3: u8,
    pub d_id: fc_addr_nw,
    pub flags: u8,
    pub def_q_idx: u8,
}

pub const MAX_TID_BLOCKS_FCOE: usize = 512;

#[repr(C)]
pub struct qed_fcoe_tid {
    pub size: u32, // In bytes per task
    pub num_tids_per_block: u32,
    pub blocks: [*mut u8; MAX_TID_BLOCKS_FCOE],
}

#[repr(C)]
pub struct qed_fcoe_cb_ops {
    pub common: qed_common_cb_ops,
    pub get_login_failures:
        Option<unsafe extern "C" fn(cookie: *mut core::ffi::c_void) -> u32>,
}

/**
 * struct qed_fcoe_ops - qed FCoE operations.
 * @common:              common operations pointer
 * @fill_dev_info:       fills FCoE specific information
 *                       @param cdev
 *                       @param info
 *                       @return 0 on success, otherwise error value.
 * @register_ops:        register FCoE operations
 *                       @param cdev
 *                       @param ops - specified using qed_iscsi_cb_ops
 *                       @param cookie - driver private
 * @ll2:                 light L2 operations pointer
 * @start:               fcoe in FW
 *                       @param cdev
 *                       @param tasks - qed will fill information about tasks
 *                       return 0 on success, otherwise error value.
 * @stop:                stops fcoe in FW
 *                       @param cdev
 *                       return 0 on success, otherwise error value.
 * @acquire_conn:        acquire a new fcoe connection
 *                       @param cdev
 *                       @param handle - qed will fill handle that should be
 *                               used henceforth as identifier of the
 *                               connection.
 *                       @param p_doorbell - qed will fill the address of the
 *                               doorbell.
 *                       return 0 on success, otherwise error value.
 * @release_conn:        release a previously acquired fcoe connection
 *                       @param cdev
 *                       @param handle - the connection handle.
 *                       return 0 on success, otherwise error value.
 * @offload_conn:        configures an offloaded connection
 *                       @param cdev
 *                       @param handle - the connection handle.
 *                       @param conn_info - the configuration to use for the
 *                               offload.
 *                       return 0 on success, otherwise error value.
 * @destroy_conn:        stops an offloaded connection
 *                       @param cdev
 *                       @param handle - the connection handle.
 *                       @param terminate_params
 *                       return 0 on success, otherwise error value.
 * @get_stats:           gets FCoE related statistics
 *                       @param cdev
 *                       @param stats - pointer to struck that would be filled
 *                               we stats
 *                       return 0 on success, error otherwise.
 */
#[repr(C)]
pub struct qed_fcoe_ops {
    pub common: *const qed_common_ops,

    pub fill_dev_info:
        Option<unsafe extern "C" fn(cdev: *mut qed_dev, info: *mut qed_dev_fcoe_info) -> i32>,

    pub register_ops: Option<unsafe extern "C" fn(
        cdev: *mut qed_dev,
        ops: *mut qed_fcoe_cb_ops,
        cookie: *mut core::ffi::c_void,
    )>,

    pub ll2: *const qed_ll2_ops,

    pub start: Option<unsafe extern "C" fn(cdev: *mut qed_dev, tasks: *mut qed_fcoe_tid) -> i32>,

    pub stop: Option<unsafe extern "C" fn(cdev: *mut qed_dev) -> i32>,

    pub acquire_conn: Option<unsafe extern "C" fn(
        cdev: *mut qed_dev,
        handle: *mut u32,
        fw_cid: *mut u32,
        p_doorbell: *mut *mut core::ffi::c_void,
    ) -> i32>,

    pub release_conn: Option<unsafe extern "C" fn(cdev: *mut qed_dev, handle: u32) -> i32>,

    pub offload_conn: Option<unsafe extern "C" fn(
        cdev: *mut qed_dev,
        handle: u32,
        conn_info: *mut qed_fcoe_params_offload,
    ) -> i32>,
    pub destroy_conn: Option<unsafe extern "C" fn(
        cdev: *mut qed_dev,
        handle: u32,
        terminate_params: dma_addr_t,
    ) -> i32>,

    pub get_stats: Option<unsafe extern "C" fn(cdev: *mut qed_dev, stats: *mut qed_fcoe_stats) -> i32>,
}

unsafe extern "C" {
    pub fn qed_get_fcoe_ops() -> *const qed_fcoe_ops;
    pub fn qed_put_fcoe_ops();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
