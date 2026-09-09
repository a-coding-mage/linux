// SPDX-License-Identifier: GPL-2.0 OR Linux-OpenIB
#![allow(non_camel_case_types, non_snake_case, dead_code)]

// Direct Rust translation of the Linux userspace verbs ABI header.

/* SPDX-License-Identifier: ((GPL-2.0 WITH Linux-syscall-note) OR Linux-OpenIB) */
/*
 * Copyright (c) 2005 Topspin Communications.  All rights reserved.
 * Copyright (c) 2005, 2006 Cisco Systems.  All rights reserved.
 * Copyright (c) 2005 PathScale, Inc.  All rights reserved.
 * Copyright (c) 2006 Mellanox Technologies.  All rights reserved.
 *
 * This software is available to you under a choice of one of two
 * licenses.  You may choose to be licensed under the terms of the GNU
 * General Public License (GPL) Version 2, available from the file
 * COPYING in the main directory of this source tree, or the
 * OpenIB.org BSD license below:
 *
 *     Redistribution and use in source and binary forms, with or
 *     without modification, are permitted provided that the following
 *     conditions are met:
 *
 *      - Redistributions of source code must retain the above
 *        copyright notice, this list of conditions and the following
 *        disclaimer.
 *
 *      - Redistributions in binary form must reproduce the above
 *        copyright notice, this list of conditions and the following
 *        disclaimer in the documentation and/or other materials
 *        provided with the distribution.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
 * EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
 * MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND
 * NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS
 * BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN
 * ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN
 * CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 * SOFTWARE.
 */



/*
 * Increment this value if any changes that break userspace ABI
 * compatibility are made.
 */
pub const IB_USER_VERBS_ABI_VERSION: u64 = 6;
pub const IB_USER_VERBS_CMD_THRESHOLD: u64 = 50;

#[repr(C)]
pub enum ib_uverbs_write_cmds {
	IB_USER_VERBS_CMD_GET_CONTEXT,
	IB_USER_VERBS_CMD_QUERY_DEVICE,
	IB_USER_VERBS_CMD_QUERY_PORT,
	IB_USER_VERBS_CMD_ALLOC_PD,
	IB_USER_VERBS_CMD_DEALLOC_PD,
	IB_USER_VERBS_CMD_CREATE_AH,
	IB_USER_VERBS_CMD_MODIFY_AH,
	IB_USER_VERBS_CMD_QUERY_AH,
	IB_USER_VERBS_CMD_DESTROY_AH,
	IB_USER_VERBS_CMD_REG_MR,
	IB_USER_VERBS_CMD_REG_SMR,
	IB_USER_VERBS_CMD_REREG_MR,
	IB_USER_VERBS_CMD_QUERY_MR,
	IB_USER_VERBS_CMD_DEREG_MR,
	IB_USER_VERBS_CMD_ALLOC_MW,
	IB_USER_VERBS_CMD_BIND_MW,
	IB_USER_VERBS_CMD_DEALLOC_MW,
	IB_USER_VERBS_CMD_CREATE_COMP_CHANNEL,
	IB_USER_VERBS_CMD_CREATE_CQ,
	IB_USER_VERBS_CMD_RESIZE_CQ,
	IB_USER_VERBS_CMD_DESTROY_CQ,
	IB_USER_VERBS_CMD_POLL_CQ,
	IB_USER_VERBS_CMD_PEEK_CQ,
	IB_USER_VERBS_CMD_REQ_NOTIFY_CQ,
	IB_USER_VERBS_CMD_CREATE_QP,
	IB_USER_VERBS_CMD_QUERY_QP,
	IB_USER_VERBS_CMD_MODIFY_QP,
	IB_USER_VERBS_CMD_DESTROY_QP,
	IB_USER_VERBS_CMD_POST_SEND,
	IB_USER_VERBS_CMD_POST_RECV,
	IB_USER_VERBS_CMD_ATTACH_MCAST,
	IB_USER_VERBS_CMD_DETACH_MCAST,
	IB_USER_VERBS_CMD_CREATE_SRQ,
	IB_USER_VERBS_CMD_MODIFY_SRQ,
	IB_USER_VERBS_CMD_QUERY_SRQ,
	IB_USER_VERBS_CMD_DESTROY_SRQ,
	IB_USER_VERBS_CMD_POST_SRQ_RECV,
	IB_USER_VERBS_CMD_OPEN_XRCD,
	IB_USER_VERBS_CMD_CLOSE_XRCD,
	IB_USER_VERBS_CMD_CREATE_XSRQ,
	IB_USER_VERBS_CMD_OPEN_QP,
};

#[repr(C)]
pub enum AnonymousEnum {
	IB_USER_VERBS_EX_CMD_QUERY_DEVICE = IB_USER_VERBS_CMD_QUERY_DEVICE,
	IB_USER_VERBS_EX_CMD_CREATE_CQ = IB_USER_VERBS_CMD_CREATE_CQ,
	IB_USER_VERBS_EX_CMD_CREATE_QP = IB_USER_VERBS_CMD_CREATE_QP,
	IB_USER_VERBS_EX_CMD_MODIFY_QP = IB_USER_VERBS_CMD_MODIFY_QP,
	IB_USER_VERBS_EX_CMD_CREATE_FLOW = IB_USER_VERBS_CMD_THRESHOLD,
	IB_USER_VERBS_EX_CMD_DESTROY_FLOW,
	IB_USER_VERBS_EX_CMD_CREATE_WQ,
	IB_USER_VERBS_EX_CMD_MODIFY_WQ,
	IB_USER_VERBS_EX_CMD_DESTROY_WQ,
	IB_USER_VERBS_EX_CMD_CREATE_RWQ_IND_TBL,
	IB_USER_VERBS_EX_CMD_DESTROY_RWQ_IND_TBL,
	IB_USER_VERBS_EX_CMD_MODIFY_CQ
};

/* see IBA A19.4.1.1 Placement Types */
#[repr(C)]
pub enum ib_placement_type {
	IB_FLUSH_GLOBAL = 1u32 << 0,
	IB_FLUSH_PERSISTENT = 1u32 << 1,
};

/* see IBA A19.4.1.2 Selectivity Level */
#[repr(C)]
pub enum ib_selectivity_level {
	IB_FLUSH_RANGE = 0,
	IB_FLUSH_MR,
};

/*
 * Make sure that all structs defined in this file remain laid out so
 * that they pack the same way on 32-bit and 64-bit architectures (to
 * avoid incompatibility between 32-bit userspace and 64-bit kernels).
 * Specifically:
 *  - Do not use pointer types -- pass pointers in u64 instead.
 *  - Make sure that any structure larger than 4 bytes is padded to a
 *    multiple of 8 bytes.  Otherwise the structure size will be
 *    different between 32-bit and 64-bit architectures.
 */

#[repr(C)]
pub ib_uverbs_async_event_desc {
	pub element: u64,
	pub event_type: u32,	/* enum ib_event_type */
	pub reserved: u32,
};

#[repr(C)]
pub ib_uverbs_comp_event_desc {
	pub cq_handle: u64,
};

#[repr(C)]
pub ib_uverbs_cq_moderation_caps {
	pub max_cq_moderation_count: u16,
	pub max_cq_moderation_period: u16,
	pub reserved: u32,
};

/*
 * All commands from userspace should start with a u32 command field
 * followed by u16 in_words and out_words fields (which give the
 * length of the command block and response buffer if any in 32-bit
 * words).  The kernel driver will read these fields first and read
 * the rest of the command based on these value.
 */

pub const IB_USER_VERBS_CMD_COMMAND_MASK: u64 = 0xff;
pub const IB_USER_VERBS_CMD_FLAG_EXTENDED: u64 = 0x80000000u;

#[repr(C)]
pub ib_uverbs_cmd_hdr {
	pub command: u32,
	pub in_words: u16,
	pub out_words: u16,
};

#[repr(C)]
pub ib_uverbs_ex_cmd_hdr {
	pub response: u64,
	pub provider_in_words: u16,
	pub provider_out_words: u16,
	pub cmd_hdr_reserved: u32,
};

#[repr(C)]
pub ib_uverbs_get_context {
	pub response: u64,
	pub driver_data: [u64; 0],
};

#[repr(C)]
pub ib_uverbs_get_context_resp {
	pub async_fd: u32,
	pub num_comp_vectors: u32,
	pub driver_data: [u64; 0],
};

#[repr(C)]
pub ib_uverbs_query_device {
	pub response: u64,
	pub driver_data: [u64; 0],
};

#[repr(C)]
pub ib_uverbs_query_device_resp {
	pub fw_ver: u64,
	pub node_guid: u64,
	pub sys_image_guid: u64,
	pub max_mr_size: u64,
	pub page_size_cap: u64,
	pub vendor_id: u32,
	pub vendor_part_id: u32,
	pub hw_ver: u32,
	pub max_qp: u32,
	pub max_qp_wr: u32,
	pub device_cap_flags: u32,
	pub max_sge: u32,
	pub max_sge_rd: u32,
	pub max_cq: u32,
	pub max_cqe: u32,
	pub max_mr: u32,
	pub max_pd: u32,
	pub max_qp_rd_atom: u32,
	pub max_ee_rd_atom: u32,
	pub max_res_rd_atom: u32,
	pub max_qp_init_rd_atom: u32,
	pub max_ee_init_rd_atom: u32,
	pub atomic_cap: u32,
	pub max_ee: u32,
	pub max_rdd: u32,
	pub max_mw: u32,
	pub max_raw_ipv6_qp: u32,
	pub max_raw_ethy_qp: u32,
	pub max_mcast_grp: u32,
	pub max_mcast_qp_attach: u32,
	pub max_total_mcast_qp_attach: u32,
	pub max_ah: u32,
	pub max_fmr: u32,
	pub max_map_per_fmr: u32,
	pub max_srq: u32,
	pub max_srq_wr: u32,
	pub max_srq_sge: u32,
	pub max_pkeys: u16,
	pub local_ca_ack_delay: u8,
	pub phys_port_cnt: u8,
	pub reserved: [u8; 4],
};

#[repr(C)]
pub ib_uverbs_ex_query_device {
	pub comp_mask: u32,
	pub reserved: u32,
};

#[repr(C)]
pub enum ib_uverbs_odp_general_cap_bits {
	IB_UVERBS_ODP_SUPPORT          = 1 << 0,
	IB_UVERBS_ODP_SUPPORT_IMPLICIT = 1 << 1,
};

#[repr(C)]
pub enum ib_uverbs_odp_transport_cap_bits {
	IB_UVERBS_ODP_SUPPORT_SEND     = 1 << 0,
	IB_UVERBS_ODP_SUPPORT_RECV     = 1 << 1,
	IB_UVERBS_ODP_SUPPORT_WRITE    = 1 << 2,
	IB_UVERBS_ODP_SUPPORT_READ     = 1 << 3,
	IB_UVERBS_ODP_SUPPORT_ATOMIC   = 1 << 4,
	IB_UVERBS_ODP_SUPPORT_SRQ_RECV = 1 << 5,
	IB_UVERBS_ODP_SUPPORT_FLUSH    = 1 << 6,
	IB_UVERBS_ODP_SUPPORT_ATOMIC_WRITE     = 1 << 7,
};

#[repr(C)]
pub ib_uverbs_odp_caps {
	pub general_caps: u64,
	struct {
		pub rc_odp_caps: u32,
		pub uc_odp_caps: u32,
		pub ud_odp_caps: u32,
	} per_transport_caps;
	pub reserved: u32,
};

#[repr(C)]
pub ib_uverbs_rss_caps {
	/* Corresponding bit will be set if qp type from
	 * 'enum ib_qp_type' is supported, e.g.
	 * supported_qpts |= 1 << IB_QPT_UD
	 */
	pub supported_qpts: u32,
	pub max_rwq_indirection_tables: u32,
	pub max_rwq_indirection_table_size: u32,
	pub reserved: u32,
};

#[repr(C)]
pub ib_uverbs_tm_caps {
	/* Max size of rendezvous request message */
	pub max_rndv_hdr_size: u32,
	/* Max number of entries in tag matching list */
	pub max_num_tags: u32,
	/* TM flags */
	pub flags: u32,
	/* Max number of outstanding list operations */
	pub max_ops: u32,
	/* Max number of SGE in tag matching entry */
	pub max_sge: u32,
	pub reserved: u32,
};

#[repr(C)]
pub ib_uverbs_ex_query_device_resp {
	pub base: ib_uverbs_query_device_resp,
	pub comp_mask: u32,
	pub response_length: u32,
	pub odp_caps: ib_uverbs_odp_caps,
	pub timestamp_mask: u64,
	pub hca_core_clock: u64, /* in KHZ */
	pub device_cap_flags_ex: u64,
	pub rss_caps: ib_uverbs_rss_caps,
	pub max_wq_type_rq: u32,
	pub raw_packet_caps: u32,
	pub tm_caps: ib_uverbs_tm_caps,
	pub cq_moderation_caps: ib_uverbs_cq_moderation_caps,
	pub max_dm_size: u64,
	pub xrc_odp_caps: u32,
	pub reserved: u32,
};

#[repr(C)]
pub ib_uverbs_query_port {
	pub response: u64,
	pub port_num: u8,
	pub reserved: [u8; 7],
	pub driver_data: [u64; 0],
};

#[repr(C)]
pub ib_uverbs_query_port_resp {
	pub port_cap_flags: u32,		/* see ib_uverbs_query_port_cap_flags */
	pub max_msg_sz: u32,
	pub bad_pkey_cntr: u32,
	pub qkey_viol_cntr: u32,
	pub gid_tbl_len: u32,
	pub pkey_tbl_len: u16,
	pub lid: u16,
	pub sm_lid: u16,
	pub state: u8,
	pub max_mtu: u8,
	pub active_mtu: u8,
	pub lmc: u8,
	pub max_vl_num: u8,
	pub sm_sl: u8,
	pub subnet_timeout: u8,
	pub init_type_reply: u8,
	pub active_width: u8,
	pub active_speed: u8,
	pub phys_state: u8,
	pub link_layer: u8,
	pub flags: u8,			/* see ib_uverbs_query_port_flags */
	pub reserved: u8,
};

#[repr(C)]
pub ib_uverbs_alloc_pd {
	pub response: u64,
	pub driver_data: [u64; 0],
};

#[repr(C)]
pub ib_uverbs_alloc_pd_resp {
	pub pd_handle: u32,
	pub driver_data: [u32; 0],
};

#[repr(C)]
pub ib_uverbs_dealloc_pd {
	pub pd_handle: u32,
};

#[repr(C)]
pub ib_uverbs_open_xrcd {
	pub response: u64,
	pub fd: u32,
	pub oflags: u32,
	pub driver_data: [u64; 0],
};

#[repr(C)]
pub ib_uverbs_open_xrcd_resp {
	pub xrcd_handle: u32,
	pub driver_data: [u32; 0],
};

#[repr(C)]
pub ib_uverbs_close_xrcd {
	pub xrcd_handle: u32,
};

#[repr(C)]
pub ib_uverbs_reg_mr {
	pub response: u64,
	pub start: u64,
	pub length: u64,
	pub hca_va: u64,
	pub pd_handle: u32,
	pub access_flags: u32,
	pub driver_data: [u64; 0],
};

#[repr(C)]
pub ib_uverbs_reg_mr_resp {
	pub mr_handle: u32,
	pub lkey: u32,
	pub rkey: u32,
	pub driver_data: [u32; 0],
};

#[repr(C)]
pub ib_uverbs_rereg_mr {
	pub response: u64,
	pub mr_handle: u32,
	pub flags: u32,
	pub start: u64,
	pub length: u64,
	pub hca_va: u64,
	pub pd_handle: u32,
	pub access_flags: u32,
	pub driver_data: [u64; 0],
};

#[repr(C)]
pub ib_uverbs_rereg_mr_resp {
	pub lkey: u32,
	pub rkey: u32,
	pub driver_data: [u64; 0],
};

#[repr(C)]
pub ib_uverbs_dereg_mr {
	pub mr_handle: u32,
};

#[repr(C)]
pub ib_uverbs_alloc_mw {
	pub response: u64,
	pub pd_handle: u32,
	pub mw_type: u8,
	pub reserved: [u8; 3],
	pub driver_data: [u64; 0],
};

#[repr(C)]
pub ib_uverbs_alloc_mw_resp {
	pub mw_handle: u32,
	pub rkey: u32,
	pub driver_data: [u64; 0],
};

#[repr(C)]
pub ib_uverbs_dealloc_mw {
	pub mw_handle: u32,
};

#[repr(C)]
pub ib_uverbs_create_comp_channel {
	pub response: u64,
};

#[repr(C)]
pub ib_uverbs_create_comp_channel_resp {
	pub fd: u32,
};

#[repr(C)]
pub ib_uverbs_create_cq {
	pub response: u64,
	pub user_handle: u64,
	pub cqe: u32,
	pub comp_vector: u32,
	pub comp_channel: i32,
	pub reserved: u32,
	pub driver_data: [u64; 0],
};

#[repr(C)]
pub enum ib_uverbs_ex_create_cq_flags {
	IB_UVERBS_CQ_FLAGS_TIMESTAMP_COMPLETION = 1 << 0,
	IB_UVERBS_CQ_FLAGS_IGNORE_OVERRUN = 1 << 1,
};

#[repr(C)]
pub ib_uverbs_ex_create_cq {
	pub user_handle: u64,
	pub cqe: u32,
	pub comp_vector: u32,
	pub comp_channel: i32,
	pub comp_mask: u32,
	pub flags: u32,  /* bitmask of ib_uverbs_ex_create_cq_flags */
	pub reserved: u32,
};

#[repr(C)]
pub ib_uverbs_create_cq_resp {
	pub cq_handle: u32,
	pub cqe: u32,
	pub driver_data: [u64; 0],
};

#[repr(C)]
pub ib_uverbs_ex_create_cq_resp {
	pub base: ib_uverbs_create_cq_resp,
	pub comp_mask: u32,
	pub response_length: u32,
};

#[repr(C)]
pub ib_uverbs_resize_cq {
	pub response: u64,
	pub cq_handle: u32,
	pub cqe: u32,
	pub driver_data: [u64; 0],
};

#[repr(C)]
pub ib_uverbs_resize_cq_resp {
	pub cqe: u32,
	pub reserved: u32,
	pub driver_data: [u64; 0],
};

#[repr(C)]
pub ib_uverbs_poll_cq {
	pub response: u64,
	pub cq_handle: u32,
	pub ne: u32,
};

#[repr(C)]
pub enum ib_uverbs_wc_opcode {
	IB_UVERBS_WC_SEND = 0,
	IB_UVERBS_WC_RDMA_WRITE = 1,
	IB_UVERBS_WC_RDMA_READ = 2,
	IB_UVERBS_WC_COMP_SWAP = 3,
	IB_UVERBS_WC_FETCH_ADD = 4,
	IB_UVERBS_WC_BIND_MW = 5,
	IB_UVERBS_WC_LOCAL_INV = 6,
	IB_UVERBS_WC_TSO = 7,
	IB_UVERBS_WC_FLUSH = 8,
	IB_UVERBS_WC_ATOMIC_WRITE = 9,
};

#[repr(C)]
pub ib_uverbs_wc {
	pub wr_id: u64,
	pub status: u32,
	pub opcode: u32,
	pub vendor_err: u32,
	pub byte_len: u32,
	#[repr(C)]
pub AnonymousUnion {
		pub imm_data: u32,
		pub invalidate_rkey: u32,
	} ex;
	pub qp_num: u32,
	pub src_qp: u32,
	pub wc_flags: u32,
	pub pkey_index: u16,
	pub slid: u16,
	pub sl: u8,
	pub dlid_path_bits: u8,
	pub port_num: u8,
	pub reserved: u8,
};

#[repr(C)]
pub ib_uverbs_poll_cq_resp {
	pub count: u32,
	pub reserved: u32,
	ib_uverbs_wc wc[];
};

#[repr(C)]
pub ib_uverbs_req_notify_cq {
	pub cq_handle: u32,
	pub solicited_only: u32,
};

#[repr(C)]
pub ib_uverbs_destroy_cq {
	pub response: u64,
	pub cq_handle: u32,
	pub reserved: u32,
};

#[repr(C)]
pub ib_uverbs_destroy_cq_resp {
	pub comp_events_reported: u32,
	pub async_events_reported: u32,
};

#[repr(C)]
pub ib_uverbs_global_route {
	pub dgid: [u8; 16],
	pub flow_label: u32,
	pub sgid_index: u8,
	pub hop_limit: u8,
	pub traffic_class: u8,
	pub reserved: u8,
};

#[repr(C)]
pub ib_uverbs_ah_attr {
	pub grh: ib_uverbs_global_route,
	pub dlid: u16,
	pub sl: u8,
	pub src_path_bits: u8,
	pub static_rate: u8,
	pub is_global: u8,
	pub port_num: u8,
	pub reserved: u8,
};

#[repr(C)]
pub ib_uverbs_qp_attr {
	pub qp_attr_mask: u32,
	pub qp_state: u32,
	pub cur_qp_state: u32,
	pub path_mtu: u32,
	pub path_mig_state: u32,
	pub qkey: u32,
	pub rq_psn: u32,
	pub sq_psn: u32,
	pub dest_qp_num: u32,
	pub qp_access_flags: u32,

	pub ah_attr: ib_uverbs_ah_attr,
	pub alt_ah_attr: ib_uverbs_ah_attr,

	/* ib_qp_cap */
	pub max_send_wr: u32,
	pub max_recv_wr: u32,
	pub max_send_sge: u32,
	pub max_recv_sge: u32,
	pub max_inline_data: u32,

	pub pkey_index: u16,
	pub alt_pkey_index: u16,
	pub en_sqd_async_notify: u8,
	pub sq_draining: u8,
	pub max_rd_atomic: u8,
	pub max_dest_rd_atomic: u8,
	pub min_rnr_timer: u8,
	pub port_num: u8,
	pub timeout: u8,
	pub retry_cnt: u8,
	pub rnr_retry: u8,
	pub alt_port_num: u8,
	pub alt_timeout: u8,
	pub reserved: [u8; 5],
};

#[repr(C)]
pub ib_uverbs_create_qp {
	pub response: u64,
	pub user_handle: u64,
	pub pd_handle: u32,
	pub send_cq_handle: u32,
	pub recv_cq_handle: u32,
	pub srq_handle: u32,
	pub max_send_wr: u32,
	pub max_recv_wr: u32,
	pub max_send_sge: u32,
	pub max_recv_sge: u32,
	pub max_inline_data: u32,
	pub sq_sig_all: u8,
	pub qp_type: u8,
	pub is_srq: u8,
	pub reserved: u8,
	pub driver_data: [u64; 0],
};

#[repr(C)]
pub enum ib_uverbs_create_qp_mask {
	IB_UVERBS_CREATE_QP_MASK_IND_TABLE = 1u64 << 0,
};

#[repr(C)]
pub enum AnonymousEnum {
	IB_UVERBS_CREATE_QP_SUP_COMP_MASK = IB_UVERBS_CREATE_QP_MASK_IND_TABLE,
};

#[repr(C)]
pub ib_uverbs_ex_create_qp {
	pub user_handle: u64,
	pub pd_handle: u32,
	pub send_cq_handle: u32,
	pub recv_cq_handle: u32,
	pub srq_handle: u32,
	pub max_send_wr: u32,
	pub max_recv_wr: u32,
	pub max_send_sge: u32,
	pub max_recv_sge: u32,
	pub max_inline_data: u32,
	pub sq_sig_all: u8,
	pub qp_type: u8,
	pub is_srq: u8,
	pub reserved: u8,
	pub comp_mask: u32,
	pub create_flags: u32,
	pub rwq_ind_tbl_handle: u32,
	pub source_qpn: u32,
};

#[repr(C)]
pub ib_uverbs_open_qp {
	pub response: u64,
	pub user_handle: u64,
	pub pd_handle: u32,
	pub qpn: u32,
	pub qp_type: u8,
	pub reserved: [u8; 7],
	pub driver_data: [u64; 0],
};

/* also used for open response */
#[repr(C)]
pub ib_uverbs_create_qp_resp {
	pub qp_handle: u32,
	pub qpn: u32,
	pub max_send_wr: u32,
	pub max_recv_wr: u32,
	pub max_send_sge: u32,
	pub max_recv_sge: u32,
	pub max_inline_data: u32,
	pub reserved: u32,
	pub driver_data: [u32; 0],
};

#[repr(C)]
pub ib_uverbs_ex_create_qp_resp {
	pub base: ib_uverbs_create_qp_resp,
	pub comp_mask: u32,
	pub response_length: u32,
};

/*
 * This needs to remain a multiple of 8 bytes to keep the
 * alignment of the modify QP parameters.
 */
#[repr(C)]
pub ib_uverbs_qp_dest {
	pub dgid: [u8; 16],
	pub flow_label: u32,
	pub dlid: u16,
	pub reserved: u16,
	pub sgid_index: u8,
	pub hop_limit: u8,
	pub traffic_class: u8,
	pub sl: u8,
	pub src_path_bits: u8,
	pub static_rate: u8,
	pub is_global: u8,
	pub port_num: u8,
};

#[repr(C)]
pub ib_uverbs_query_qp {
	pub response: u64,
	pub qp_handle: u32,
	pub attr_mask: u32,
	pub driver_data: [u64; 0],
};

#[repr(C)]
pub ib_uverbs_query_qp_resp {
	pub dest: ib_uverbs_qp_dest,
	pub alt_dest: ib_uverbs_qp_dest,
	pub max_send_wr: u32,
	pub max_recv_wr: u32,
	pub max_send_sge: u32,
	pub max_recv_sge: u32,
	pub max_inline_data: u32,
	pub qkey: u32,
	pub rq_psn: u32,
	pub sq_psn: u32,
	pub dest_qp_num: u32,
	pub qp_access_flags: u32,
	pub pkey_index: u16,
	pub alt_pkey_index: u16,
	pub qp_state: u8,
	pub cur_qp_state: u8,
	pub path_mtu: u8,
	pub path_mig_state: u8,
	pub sq_draining: u8,
	pub max_rd_atomic: u8,
	pub max_dest_rd_atomic: u8,
	pub min_rnr_timer: u8,
	pub port_num: u8,
	pub timeout: u8,
	pub retry_cnt: u8,
	pub rnr_retry: u8,
	pub alt_port_num: u8,
	pub alt_timeout: u8,
	pub sq_sig_all: u8,
	pub reserved: [u8; 5],
	pub driver_data: [u64; 0],
};

#[repr(C)]
pub ib_uverbs_modify_qp {
	pub dest: ib_uverbs_qp_dest,
	pub alt_dest: ib_uverbs_qp_dest,
	pub qp_handle: u32,
	pub attr_mask: u32,
	pub qkey: u32,
	pub rq_psn: u32,
	pub sq_psn: u32,
	pub dest_qp_num: u32,
	pub qp_access_flags: u32,
	pub pkey_index: u16,
	pub alt_pkey_index: u16,
	pub qp_state: u8,
	pub cur_qp_state: u8,
	pub path_mtu: u8,
	pub path_mig_state: u8,
	pub en_sqd_async_notify: u8,
	pub max_rd_atomic: u8,
	pub max_dest_rd_atomic: u8,
	pub min_rnr_timer: u8,
	pub port_num: u8,
	pub timeout: u8,
	pub retry_cnt: u8,
	pub rnr_retry: u8,
	pub alt_port_num: u8,
	pub alt_timeout: u8,
	pub reserved: [u8; 2],
	pub driver_data: [u64; 0],
};

#[repr(C)]
pub ib_uverbs_ex_modify_qp {
	pub base: ib_uverbs_modify_qp,
	pub rate_limit: u32,
	pub reserved: u32,
};

#[repr(C)]
pub ib_uverbs_ex_modify_qp_resp {
	pub comp_mask: u32,
	pub response_length: u32,
};

#[repr(C)]
pub ib_uverbs_destroy_qp {
	pub response: u64,
	pub qp_handle: u32,
	pub reserved: u32,
};

#[repr(C)]
pub ib_uverbs_destroy_qp_resp {
	pub events_reported: u32,
};

/*
 * The ib_uverbs_sge structure isn't used anywhere, since we assume
 * the ib_sge structure is packed the same way on 32-bit and 64-bit
 * architectures in both kernel and user space.  It's just here to
 * document the ABI.
 */
#[repr(C)]
pub ib_uverbs_sge {
	pub addr: u64,
	pub length: u32,
	pub lkey: u32,
};

#[repr(C)]
pub enum ib_uverbs_wr_opcode {
	IB_UVERBS_WR_RDMA_WRITE = 0,
	IB_UVERBS_WR_RDMA_WRITE_WITH_IMM = 1,
	IB_UVERBS_WR_SEND = 2,
	IB_UVERBS_WR_SEND_WITH_IMM = 3,
	IB_UVERBS_WR_RDMA_READ = 4,
	IB_UVERBS_WR_ATOMIC_CMP_AND_SWP = 5,
	IB_UVERBS_WR_ATOMIC_FETCH_AND_ADD = 6,
	IB_UVERBS_WR_LOCAL_INV = 7,
	IB_UVERBS_WR_BIND_MW = 8,
	IB_UVERBS_WR_SEND_WITH_INV = 9,
	IB_UVERBS_WR_TSO = 10,
	IB_UVERBS_WR_RDMA_READ_WITH_INV = 11,
	IB_UVERBS_WR_MASKED_ATOMIC_CMP_AND_SWP = 12,
	IB_UVERBS_WR_MASKED_ATOMIC_FETCH_AND_ADD = 13,
	IB_UVERBS_WR_FLUSH = 14,
	IB_UVERBS_WR_ATOMIC_WRITE = 15,
	/* Review enum ib_wr_opcode before modifying this */
};

#[repr(C)]
pub ib_uverbs_send_wr {
	pub wr_id: u64,
	pub num_sge: u32,
	pub opcode: u32,		/* see enum ib_uverbs_wr_opcode */
	pub send_flags: u32,
	#[repr(C)]
pub AnonymousUnion {
		pub imm_data: u32,
		pub invalidate_rkey: u32,
	} ex;
	#[repr(C)]
pub AnonymousUnion {
		struct {
			pub remote_addr: u64,
			pub rkey: u32,
			pub reserved: u32,
		} rdma;
		struct {
			pub remote_addr: u64,
			pub compare_add: u64,
			pub swap: u64,
			pub rkey: u32,
			pub reserved: u32,
		} atomic;
		struct {
			pub ah: u32,
			pub remote_qpn: u32,
			pub remote_qkey: u32,
			pub reserved: u32,
		} ud;
	} wr;
};

#[repr(C)]
pub ib_uverbs_post_send {
	pub response: u64,
	pub qp_handle: u32,
	pub wr_count: u32,
	pub sge_count: u32,
	pub wqe_size: u32,
	ib_uverbs_send_wr send_wr[];
};

#[repr(C)]
pub ib_uverbs_post_send_resp {
	pub bad_wr: u32,
};

#[repr(C)]
pub ib_uverbs_recv_wr {
	pub wr_id: u64,
	pub num_sge: u32,
	pub reserved: u32,
};

#[repr(C)]
pub ib_uverbs_post_recv {
	pub response: u64,
	pub qp_handle: u32,
	pub wr_count: u32,
	pub sge_count: u32,
	pub wqe_size: u32,
	ib_uverbs_recv_wr recv_wr[];
};

#[repr(C)]
pub ib_uverbs_post_recv_resp {
	pub bad_wr: u32,
};

#[repr(C)]
pub ib_uverbs_post_srq_recv {
	pub response: u64,
	pub srq_handle: u32,
	pub wr_count: u32,
	pub sge_count: u32,
	pub wqe_size: u32,
	ib_uverbs_recv_wr recv[];
};

#[repr(C)]
pub ib_uverbs_post_srq_recv_resp {
	pub bad_wr: u32,
};

#[repr(C)]
pub ib_uverbs_create_ah {
	pub response: u64,
	pub user_handle: u64,
	pub pd_handle: u32,
	pub reserved: u32,
	pub attr: ib_uverbs_ah_attr,
	pub driver_data: [u64; 0],
};

#[repr(C)]
pub ib_uverbs_create_ah_resp {
	pub ah_handle: u32,
	pub driver_data: [u32; 0],
};

#[repr(C)]
pub ib_uverbs_destroy_ah {
	pub ah_handle: u32,
};

#[repr(C)]
pub ib_uverbs_attach_mcast {
	pub gid: [u8; 16],
	pub qp_handle: u32,
	pub mlid: u16,
	pub reserved: u16,
	pub driver_data: [u64; 0],
};

#[repr(C)]
pub ib_uverbs_detach_mcast {
	pub gid: [u8; 16],
	pub qp_handle: u32,
	pub mlid: u16,
	pub reserved: u16,
	pub driver_data: [u64; 0],
};

#[repr(C)]
pub ib_uverbs_flow_spec_hdr {
	pub type: u32,
	pub size: u16,
	pub reserved: u16,
	/* followed by flow_spec */
	pub flow_spec_data: [u64; 0],
};

#[repr(C)]
pub ib_uverbs_flow_eth_filter {
	pub dst_mac: [u8; 6],
	pub src_mac: [u8; 6],
	pub ether_type: u16,
	pub vlan_tag: u16,
};

#[repr(C)]
pub ib_uverbs_flow_spec_eth {
	#[repr(C)]
pub AnonymousUnion {
		pub hdr: ib_uverbs_flow_spec_hdr,
		struct {
			pub type: u32,
			pub size: u16,
			pub reserved: u16,
		};
	};
	pub val: ib_uverbs_flow_eth_filter,
	pub mask: ib_uverbs_flow_eth_filter,
};

#[repr(C)]
pub ib_uverbs_flow_ipv4_filter {
	pub src_ip: u32,
	pub dst_ip: u32,
	pub proto: u8,
	pub tos: u8,
	pub ttl: u8,
	pub flags: u8,
};

#[repr(C)]
pub ib_uverbs_flow_spec_ipv4 {
	#[repr(C)]
pub AnonymousUnion {
		pub hdr: ib_uverbs_flow_spec_hdr,
		struct {
			pub type: u32,
			pub size: u16,
			pub reserved: u16,
		};
	};
	pub val: ib_uverbs_flow_ipv4_filter,
	pub mask: ib_uverbs_flow_ipv4_filter,
};

#[repr(C)]
pub ib_uverbs_flow_tcp_udp_filter {
	pub dst_port: u16,
	pub src_port: u16,
};

#[repr(C)]
pub ib_uverbs_flow_spec_tcp_udp {
	#[repr(C)]
pub AnonymousUnion {
		pub hdr: ib_uverbs_flow_spec_hdr,
		struct {
			pub type: u32,
			pub size: u16,
			pub reserved: u16,
		};
	};
	pub val: ib_uverbs_flow_tcp_udp_filter,
	pub mask: ib_uverbs_flow_tcp_udp_filter,
};

#[repr(C)]
pub ib_uverbs_flow_ipv6_filter {
	pub src_ip: [u8; 16],
	pub dst_ip: [u8; 16],
	pub flow_label: u32,
	pub next_hdr: u8,
	pub traffic_class: u8,
	pub hop_limit: u8,
	pub reserved: u8,
};

#[repr(C)]
pub ib_uverbs_flow_spec_ipv6 {
	#[repr(C)]
pub AnonymousUnion {
		pub hdr: ib_uverbs_flow_spec_hdr,
		struct {
			pub type: u32,
			pub size: u16,
			pub reserved: u16,
		};
	};
	pub val: ib_uverbs_flow_ipv6_filter,
	pub mask: ib_uverbs_flow_ipv6_filter,
};

#[repr(C)]
pub ib_uverbs_flow_spec_action_tag {
	#[repr(C)]
pub AnonymousUnion {
		pub hdr: ib_uverbs_flow_spec_hdr,
		struct {
			pub type: u32,
			pub size: u16,
			pub reserved: u16,
		};
	};
	pub tag_id: u32,
	pub reserved1: u32,
};

#[repr(C)]
pub ib_uverbs_flow_spec_action_drop {
	#[repr(C)]
pub AnonymousUnion {
		pub hdr: ib_uverbs_flow_spec_hdr,
		struct {
			pub type: u32,
			pub size: u16,
			pub reserved: u16,
		};
	};
};

#[repr(C)]
pub ib_uverbs_flow_spec_action_handle {
	#[repr(C)]
pub AnonymousUnion {
		pub hdr: ib_uverbs_flow_spec_hdr,
		struct {
			pub type: u32,
			pub size: u16,
			pub reserved: u16,
		};
	};
	pub handle: u32,
	pub reserved1: u32,
};

#[repr(C)]
pub ib_uverbs_flow_spec_action_count {
	#[repr(C)]
pub AnonymousUnion {
		pub hdr: ib_uverbs_flow_spec_hdr,
		struct {
			pub type: u32,
			pub size: u16,
			pub reserved: u16,
		};
	};
	pub handle: u32,
	pub reserved1: u32,
};

#[repr(C)]
pub ib_uverbs_flow_tunnel_filter {
	pub tunnel_id: u32,
};

#[repr(C)]
pub ib_uverbs_flow_spec_tunnel {
	#[repr(C)]
pub AnonymousUnion {
		pub hdr: ib_uverbs_flow_spec_hdr,
		struct {
			pub type: u32,
			pub size: u16,
			pub reserved: u16,
		};
	};
	pub val: ib_uverbs_flow_tunnel_filter,
	pub mask: ib_uverbs_flow_tunnel_filter,
};

#[repr(C)]
pub ib_uverbs_flow_spec_esp_filter {
	pub spi: u32,
	pub seq: u32,
};

#[repr(C)]
pub ib_uverbs_flow_spec_esp {
	#[repr(C)]
pub AnonymousUnion {
		pub hdr: ib_uverbs_flow_spec_hdr,
		struct {
			pub type: u32,
			pub size: u16,
			pub reserved: u16,
		};
	};
	pub val: ib_uverbs_flow_spec_esp_filter,
	pub mask: ib_uverbs_flow_spec_esp_filter,
};

#[repr(C)]
pub ib_uverbs_flow_gre_filter {
	/* c_ks_res0_ver field is bits 0-15 in offset 0 of a standard GRE header:
	 * bit 0 - C - checksum bit.
	 * bit 1 - reserved. set to 0.
	 * bit 2 - key bit.
	 * bit 3 - sequence number bit.
	 * bits 4:12 - reserved. set to 0.
	 * bits 13:15 - GRE version.
	 */
	pub c_ks_res0_ver: u16,
	pub protocol: u16,
	pub key: u32,
};

#[repr(C)]
pub ib_uverbs_flow_spec_gre {
	#[repr(C)]
pub AnonymousUnion {
		pub hdr: ib_uverbs_flow_spec_hdr,
		struct {
			pub type: u32,
			pub size: u16,
			pub reserved: u16,
		};
	};
	pub val: ib_uverbs_flow_gre_filter,
	pub mask: ib_uverbs_flow_gre_filter,
};

#[repr(C)]
pub ib_uverbs_flow_mpls_filter {
	/* The field includes the entire MPLS label:
	 * bits 0:19 - label field.
	 * bits 20:22 - traffic class field.
	 * bits 23 - bottom of stack bit.
	 * bits 24:31 - ttl field.
	 */
	pub label: u32,
};

#[repr(C)]
pub ib_uverbs_flow_spec_mpls {
	#[repr(C)]
pub AnonymousUnion {
		pub hdr: ib_uverbs_flow_spec_hdr,
		struct {
			pub type: u32,
			pub size: u16,
			pub reserved: u16,
		};
	};
	pub val: ib_uverbs_flow_mpls_filter,
	pub mask: ib_uverbs_flow_mpls_filter,
};

#[repr(C)]
pub ib_uverbs_flow_attr {
	pub type: u32,
	pub size: u16,
	pub priority: u16,
	pub num_of_specs: u8,
	pub reserved: [u8; 2],
	pub port: u8,
	pub flags: u32,
	/* Following are the optional layers according to user request
	 * ib_flow_spec_xxx
	 * ib_flow_spec_yyy
	 */
	ib_uverbs_flow_spec_hdr flow_specs[];
};

#[repr(C)]
pub ib_uverbs_create_flow {
	pub comp_mask: u32,
	pub qp_handle: u32,
	pub flow_attr: ib_uverbs_flow_attr,
};

#[repr(C)]
pub ib_uverbs_create_flow_resp {
	pub comp_mask: u32,
	pub flow_handle: u32,
};

#[repr(C)]
pub ib_uverbs_destroy_flow {
	pub comp_mask: u32,
	pub flow_handle: u32,
};

#[repr(C)]
pub ib_uverbs_create_srq {
	pub response: u64,
	pub user_handle: u64,
	pub pd_handle: u32,
	pub max_wr: u32,
	pub max_sge: u32,
	pub srq_limit: u32,
	pub driver_data: [u64; 0],
};

#[repr(C)]
pub ib_uverbs_create_xsrq {
	pub response: u64,
	pub user_handle: u64,
	pub srq_type: u32,
	pub pd_handle: u32,
	pub max_wr: u32,
	pub max_sge: u32,
	pub srq_limit: u32,
	pub max_num_tags: u32,
	pub xrcd_handle: u32,
	pub cq_handle: u32,
	pub driver_data: [u64; 0],
};

#[repr(C)]
pub ib_uverbs_create_srq_resp {
	pub srq_handle: u32,
	pub max_wr: u32,
	pub max_sge: u32,
	pub srqn: u32,
	pub driver_data: [u32; 0],
};

#[repr(C)]
pub ib_uverbs_modify_srq {
	pub srq_handle: u32,
	pub attr_mask: u32,
	pub max_wr: u32,
	pub srq_limit: u32,
	pub driver_data: [u64; 0],
};

#[repr(C)]
pub ib_uverbs_query_srq {
	pub response: u64,
	pub srq_handle: u32,
	pub reserved: u32,
	pub driver_data: [u64; 0],
};

#[repr(C)]
pub ib_uverbs_query_srq_resp {
	pub max_wr: u32,
	pub max_sge: u32,
	pub srq_limit: u32,
	pub reserved: u32,
};

#[repr(C)]
pub ib_uverbs_destroy_srq {
	pub response: u64,
	pub srq_handle: u32,
	pub reserved: u32,
};

#[repr(C)]
pub ib_uverbs_destroy_srq_resp {
	pub events_reported: u32,
};

#[repr(C)]
pub ib_uverbs_ex_create_wq {
	pub comp_mask: u32,
	pub wq_type: u32,
	pub user_handle: u64,
	pub pd_handle: u32,
	pub cq_handle: u32,
	pub max_wr: u32,
	pub max_sge: u32,
	pub create_flags: u32, /* Use enum ib_wq_flags */
	pub reserved: u32,
};

#[repr(C)]
pub ib_uverbs_ex_create_wq_resp {
	pub comp_mask: u32,
	pub response_length: u32,
	pub wq_handle: u32,
	pub max_wr: u32,
	pub max_sge: u32,
	pub wqn: u32,
};

#[repr(C)]
pub ib_uverbs_ex_destroy_wq {
	pub comp_mask: u32,
	pub wq_handle: u32,
};

#[repr(C)]
pub ib_uverbs_ex_destroy_wq_resp {
	pub comp_mask: u32,
	pub response_length: u32,
	pub events_reported: u32,
	pub reserved: u32,
};

#[repr(C)]
pub ib_uverbs_ex_modify_wq {
	pub attr_mask: u32,
	pub wq_handle: u32,
	pub wq_state: u32,
	pub curr_wq_state: u32,
	pub flags: u32, /* Use enum ib_wq_flags */
	pub flags_mask: u32, /* Use enum ib_wq_flags */
};

/* Prevent memory allocation rather than max expected size */
pub const IB_USER_VERBS_MAX_LOG_IND_TBL_SIZE: u64 = 0x0d;
#[repr(C)]
pub ib_uverbs_ex_create_rwq_ind_table {
	pub comp_mask: u32,
	pub log_ind_tbl_size: u32,
	/* Following are the wq handles according to log_ind_tbl_size
	 * wq_handle1
	 * wq_handle2
	 */
	pub wq_handles: [u32; 0],
};

#[repr(C)]
pub ib_uverbs_ex_create_rwq_ind_table_resp {
	pub comp_mask: u32,
	pub response_length: u32,
	pub ind_tbl_handle: u32,
	pub ind_tbl_num: u32,
};

#[repr(C)]
pub ib_uverbs_ex_destroy_rwq_ind_table {
	pub comp_mask: u32,
	pub ind_tbl_handle: u32,
};

#[repr(C)]
pub ib_uverbs_cq_moderation {
	pub cq_count: u16,
	pub cq_period: u16,
};

#[repr(C)]
pub ib_uverbs_ex_modify_cq {
	pub cq_handle: u32,
	pub attr_mask: u32,
	pub attr: ib_uverbs_cq_moderation,
	pub reserved: u32,
};

pub const IB_DEVICE_NAME_MAX: u64 = 64;

/*
 * bits 9, 15, 16, 19, 22, 27, 30, 31, 32, 33, 35 and 37 may be set by old
 * kernels and should not be used.
 */
#[repr(C)]
pub enum ib_uverbs_device_cap_flags {
	IB_UVERBS_DEVICE_RESIZE_MAX_WR = 1 << 0,
	IB_UVERBS_DEVICE_BAD_PKEY_CNTR = 1 << 1,
	IB_UVERBS_DEVICE_BAD_QKEY_CNTR = 1 << 2,
	IB_UVERBS_DEVICE_RAW_MULTI = 1 << 3,
	IB_UVERBS_DEVICE_AUTO_PATH_MIG = 1 << 4,
	IB_UVERBS_DEVICE_CHANGE_PHY_PORT = 1 << 5,
	IB_UVERBS_DEVICE_UD_AV_PORT_ENFORCE = 1 << 6,
	IB_UVERBS_DEVICE_CURR_QP_STATE_MOD = 1 << 7,
	IB_UVERBS_DEVICE_SHUTDOWN_PORT = 1 << 8,
	/* IB_UVERBS_DEVICE_INIT_TYPE = 1 << 9, (not in use) */
	IB_UVERBS_DEVICE_PORT_ACTIVE_EVENT = 1 << 10,
	IB_UVERBS_DEVICE_SYS_IMAGE_GUID = 1 << 11,
	IB_UVERBS_DEVICE_RC_RNR_NAK_GEN = 1 << 12,
	IB_UVERBS_DEVICE_SRQ_RESIZE = 1 << 13,
	IB_UVERBS_DEVICE_N_NOTIFY_CQ = 1 << 14,
	IB_UVERBS_DEVICE_MEM_WINDOW = 1 << 17,
	IB_UVERBS_DEVICE_UD_IP_CSUM = 1 << 18,
	IB_UVERBS_DEVICE_XRC = 1 << 20,
	IB_UVERBS_DEVICE_MEM_MGT_EXTENSIONS = 1 << 21,
	IB_UVERBS_DEVICE_MEM_WINDOW_TYPE_2A = 1 << 23,
	IB_UVERBS_DEVICE_MEM_WINDOW_TYPE_2B = 1 << 24,
	IB_UVERBS_DEVICE_RC_IP_CSUM = 1 << 25,
	/* Deprecated. Please use IB_UVERBS_RAW_PACKET_CAP_IP_CSUM. */
	IB_UVERBS_DEVICE_RAW_IP_CSUM = 1 << 26,
	IB_UVERBS_DEVICE_MANAGED_FLOW_STEERING = 1 << 29,
	/* Deprecated. Please use IB_UVERBS_RAW_PACKET_CAP_SCATTER_FCS. */
	IB_UVERBS_DEVICE_RAW_SCATTER_FCS = 1u64 << 34,
	IB_UVERBS_DEVICE_PCI_WRITE_END_PADDING = 1u64 << 36,
	/* Flush placement types */
	IB_UVERBS_DEVICE_FLUSH_GLOBAL = 1u64 << 38,
	IB_UVERBS_DEVICE_FLUSH_PERSISTENT = 1u64 << 39,
	/* Atomic write attributes */
	IB_UVERBS_DEVICE_ATOMIC_WRITE = 1u64 << 40,
	/* CoCo guest with DMA bounce buffering required */
	IB_UVERBS_DEVICE_CC_DMA_BOUNCE = 1u64 << 41,
};

#[repr(C)]
pub enum ib_uverbs_raw_packet_caps {
	IB_UVERBS_RAW_PACKET_CAP_CVLAN_STRIPPING = 1 << 0,
	IB_UVERBS_RAW_PACKET_CAP_SCATTER_FCS = 1 << 1,
	IB_UVERBS_RAW_PACKET_CAP_IP_CSUM = 1 << 2,
	IB_UVERBS_RAW_PACKET_CAP_DELAY_DROP = 1 << 3,
};

/*
 * ib_uverbs_clock_info - timecounter state shared with userspace
 *
 * Drivers that use a software timecounter over a free-running hardware
 * cycle counter can map this page read-only into userspace, allowing
 * conversion of hardware timestamps to system time without a syscall.
 *
 * Synchronization uses a sequence counter (@sign): the kernel sets bit 0
 * before updating, then advances by 2 after. Userspace must retry the read
 * if @sign is odd or changed during the read.
 *
 * @sign:            Sequence counter (bit 0 = update in progress)
 * @resv:            Reserved
 * @nsec:            Nanoseconds at last update
 * @cycles:          Cycle counter value at last update
 * @frac:            Fractional nanoseconds at last update
 * @mult:            Cycle-to-nanosecond multiplier
 * @shift:           Cycle-to-nanosecond shift
 * @mask:            Cycle counter bitmask
 * @overflow_period: Max interval (nsec) between reads before counter wraps
 */
#[repr(C)]
pub ib_uverbs_clock_info {
	pub sign: u32,
	pub resv: u32,
	pub nsec: u64,
	pub cycles: u64,
	pub frac: u64,
	pub mult: u32,
	pub shift: u32,
	pub mask: u64,
	pub overflow_period: u64,
};



// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
