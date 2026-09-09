/* SPDX-License-Identifier: ((GPL-2.0 WITH Linux-syscall-note) OR BSD-2-Clause) */
/*
 * Copyright (c) 2012-2016 VMware, Inc.  All rights reserved.
 *
 * This program is free software; you can redistribute it and/or
 * modify it under the terms of EITHER the GNU General Public License
 * version 2 as published by the Free Software Foundation or the BSD
 * 2-Clause License. This program is distributed in the hope that it
 * will be useful, but WITHOUT ANY WARRANTY; without even the implied
 * warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.
 */

pub const PVRDMA_UVERBS_ABI_VERSION: u32 = 3;
pub const PVRDMA_UAR_HANDLE_MASK: u32 = 0x00FFFFFF;
pub const PVRDMA_UAR_QP_OFFSET: u32 = 0;
pub const PVRDMA_UAR_QP_SEND: u32 = 1 << 30;
pub const PVRDMA_UAR_QP_RECV: u32 = 1 << 31;
pub const PVRDMA_UAR_CQ_OFFSET: u32 = 4;
pub const PVRDMA_UAR_CQ_ARM_SOL: u32 = 1 << 29;
pub const PVRDMA_UAR_CQ_ARM: u32 = 1 << 30;
pub const PVRDMA_UAR_CQ_POLL: u32 = 1 << 31;
pub const PVRDMA_UAR_SRQ_OFFSET: u32 = 8;
pub const PVRDMA_UAR_SRQ_RECV: u32 = 1 << 30;

#[repr(C)]
#[derive(Copy, Clone)]
pub enum pvrdma_wr_opcode { PVRDMA_WR_RDMA_WRITE, PVRDMA_WR_RDMA_WRITE_WITH_IMM, PVRDMA_WR_SEND, PVRDMA_WR_SEND_WITH_IMM, PVRDMA_WR_RDMA_READ, PVRDMA_WR_ATOMIC_CMP_AND_SWP, PVRDMA_WR_ATOMIC_FETCH_AND_ADD, PVRDMA_WR_LSO, PVRDMA_WR_SEND_WITH_INV, PVRDMA_WR_RDMA_READ_WITH_INV, PVRDMA_WR_LOCAL_INV, PVRDMA_WR_FAST_REG_MR, PVRDMA_WR_MASKED_ATOMIC_CMP_AND_SWP, PVRDMA_WR_MASKED_ATOMIC_FETCH_AND_ADD, PVRDMA_WR_BIND_MW, PVRDMA_WR_REG_SIG_MR, PVRDMA_WR_ERROR }

#[repr(C)]
#[derive(Copy, Clone)]
pub enum pvrdma_wc_status { PVRDMA_WC_SUCCESS, PVRDMA_WC_LOC_LEN_ERR, PVRDMA_WC_LOC_QP_OP_ERR, PVRDMA_WC_LOC_EEC_OP_ERR, PVRDMA_WC_LOC_PROT_ERR, PVRDMA_WC_WR_FLUSH_ERR, PVRDMA_WC_MW_BIND_ERR, PVRDMA_WC_BAD_RESP_ERR, PVRDMA_WC_LOC_ACCESS_ERR, PVRDMA_WC_REM_INV_REQ_ERR, PVRDMA_WC_REM_ACCESS_ERR, PVRDMA_WC_REM_OP_ERR, PVRDMA_WC_RETRY_EXC_ERR, PVRDMA_WC_RNR_RETRY_EXC_ERR, PVRDMA_WC_LOC_RDD_VIOL_ERR, PVRDMA_WC_REM_INV_RD_REQ_ERR, PVRDMA_WC_REM_ABORT_ERR, PVRDMA_WC_INV_EECN_ERR, PVRDMA_WC_INV_EEC_STATE_ERR, PVRDMA_WC_FATAL_ERR, PVRDMA_WC_RESP_TIMEOUT_ERR, PVRDMA_WC_GENERAL_ERR }

#[repr(C)]
#[derive(Copy, Clone)]
pub enum pvrdma_wc_opcode { PVRDMA_WC_SEND, PVRDMA_WC_RDMA_WRITE, PVRDMA_WC_RDMA_READ, PVRDMA_WC_COMP_SWAP, PVRDMA_WC_FETCH_ADD, PVRDMA_WC_BIND_MW, PVRDMA_WC_LSO, PVRDMA_WC_LOCAL_INV, PVRDMA_WC_FAST_REG_MR, PVRDMA_WC_MASKED_COMP_SWAP, PVRDMA_WC_MASKED_FETCH_ADD, PVRDMA_WC_RECV = 1 << 7, PVRDMA_WC_RECV_RDMA_WITH_IMM }

#[repr(C)]
#[derive(Copy, Clone)]
pub enum pvrdma_wc_flags { PVRDMA_WC_GRH = 1 << 0, PVRDMA_WC_WITH_IMM = 1 << 1, PVRDMA_WC_WITH_INVALIDATE = 1 << 2, PVRDMA_WC_IP_CSUM_OK = 1 << 3, PVRDMA_WC_WITH_SMAC = 1 << 4, PVRDMA_WC_WITH_VLAN = 1 << 5, PVRDMA_WC_WITH_NETWORK_HDR_TYPE = 1 << 6, PVRDMA_WC_FLAGS_MAX = PVRDMA_WC_WITH_NETWORK_HDR_TYPE }

#[repr(C)]
#[derive(Copy, Clone)]
pub enum pvrdma_network_type { PVRDMA_NETWORK_IB, PVRDMA_NETWORK_ROCE_V1 = PVRDMA_NETWORK_IB as isize, PVRDMA_NETWORK_IPV4, PVRDMA_NETWORK_IPV6 }

#[repr(C)] pub struct pvrdma_alloc_ucontext_resp { pub qp_tab_size: u32, pub reserved: u32 }
#[repr(C)] pub struct pvrdma_alloc_pd_resp { pub pdn: u32, pub reserved: u32 }
#[repr(C)] pub struct pvrdma_create_cq { pub buf_addr: u64, pub buf_size: u32, pub reserved: u32 }
#[repr(C)] pub struct pvrdma_create_cq_resp { pub cqn: u32, pub reserved: u32 }
#[repr(C)] pub struct pvrdma_resize_cq { pub buf_addr: u64, pub buf_size: u32, pub reserved: u32 }
#[repr(C)] pub struct pvrdma_create_srq { pub buf_addr: u64, pub buf_size: u32, pub reserved: u32 }
#[repr(C)] pub struct pvrdma_create_srq_resp { pub srqn: u32, pub reserved: u32 }
#[repr(C)] pub struct pvrdma_create_qp { pub rbuf_addr: u64, pub sbuf_addr: u64, pub rbuf_size: u32, pub sbuf_size: u32, pub qp_addr: u64 }
#[repr(C)] pub struct pvrdma_create_qp_resp { pub qpn: u32, pub qp_handle: u32 }

#[repr(C)] pub struct pvrdma_ex_cmp_swap { pub swap_val: u64, pub compare_val: u64, pub swap_mask: u64, pub compare_mask: u64 }
#[repr(C)] pub struct pvrdma_ex_fetch_add { pub add_val: u64, pub field_boundary: u64 }
#[repr(C)] pub struct pvrdma_av { pub port_pd: u32, pub sl_tclass_flowlabel: u32, pub dgid: [u8; 16], pub src_path_bits: u8, pub gid_index: u8, pub stat_rate: u8, pub hop_limit: u8, pub dmac: [u8; 6], pub reserved: [u8; 6] }
#[repr(C)] pub struct pvrdma_sge { pub addr: u64, pub length: u32, pub lkey: u32 }
#[repr(C)] pub struct pvrdma_rq_wqe_hdr { pub wr_id: u64, pub num_sge: u32, pub total_len: u32 }

#[repr(C)] pub union pvrdma_sq_wqe_hdr_ex { pub imm_data: u32, pub invalidate_rkey: u32 }
#[repr(C)] pub struct pvrdma_sq_wqe_hdr_rdma { pub remote_addr: u64, pub rkey: u32, pub reserved: [u8; 4] }
#[repr(C)] pub struct pvrdma_sq_wqe_hdr_atomic { pub remote_addr: u64, pub compare_add: u64, pub swap: u64, pub rkey: u32, pub reserved: u32 }
#[repr(C)] pub union pvrdma_sq_wqe_hdr_masked_data { pub cmp_swap: pvrdma_ex_cmp_swap, pub fetch_add: pvrdma_ex_fetch_add }
#[repr(C)] pub struct pvrdma_sq_wqe_hdr_masked_atomics { pub remote_addr: u64, pub log_arg_sz: u32, pub rkey: u32, pub wr_data: pvrdma_sq_wqe_hdr_masked_data }
#[repr(C)] pub struct pvrdma_sq_wqe_hdr_fast_reg { pub iova_start: u64, pub pl_pdir_dma: u64, pub page_shift: u32, pub page_list_len: u32, pub length: u32, pub access_flags: u32, pub rkey: u32, pub reserved: u32 }
#[repr(C)] pub struct pvrdma_sq_wqe_hdr_ud { pub remote_qpn: u32, pub remote_qkey: u32, pub av: pvrdma_av }
#[repr(C)] pub union pvrdma_sq_wqe_hdr_wr { pub rdma: pvrdma_sq_wqe_hdr_rdma, pub atomic: pvrdma_sq_wqe_hdr_atomic, pub masked_atomics: pvrdma_sq_wqe_hdr_masked_atomics, pub fast_reg: pvrdma_sq_wqe_hdr_fast_reg, pub ud: pvrdma_sq_wqe_hdr_ud }
#[repr(C)] pub struct pvrdma_sq_wqe_hdr { pub wr_id: u64, pub num_sge: u32, pub total_len: u32, pub opcode: u32, pub send_flags: u32, pub ex: pvrdma_sq_wqe_hdr_ex, pub reserved: u32, pub wr: pvrdma_sq_wqe_hdr_wr }

#[repr(C)] pub struct pvrdma_cqe { pub wr_id: u64, pub qp: u64, pub opcode: u32, pub status: u32, pub byte_len: u32, pub imm_data: u32, pub src_qp: u32, pub wc_flags: u32, pub vendor_err: u32, pub pkey_index: u16, pub slid: u16, pub sl: u8, pub dlid_path_bits: u8, pub port_num: u8, pub smac: [u8; 6], pub network_hdr_type: u8, pub reserved2: [u8; 6] }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
