/* SPDX-License-Identifier: ((GPL-2.0 WITH Linux-syscall-note) OR Linux-OpenIB) */
/*
 * Copyright (c) 2016 Mellanox Technologies Ltd. All rights reserved.
 *
 * This software is available under a choice of one of two licenses.  You may
 * choose to be licensed under the terms of the GNU General Public License
 * (GPL) Version 2, or the OpenIB.org BSD license.
 */

// C dependencies: linux/types.h, linux/socket.h, linux/in.h, linux/in6.h.

#[repr(u32)]
pub enum RxeNetworkType {
    RXE_NETWORK_TYPE_IPV4 = 1,
    RXE_NETWORK_TYPE_IPV6 = 2,
}

#[repr(C)]
pub union rxe_gid {
    pub raw: [u8; 16],
    pub global: rxe_gid_global,
}

#[repr(C)]
pub struct rxe_gid_global {
    pub subnet_prefix: u64,
    pub interface_id: u64,
}

#[repr(C)]
pub struct rxe_global_route {
    pub dgid: rxe_gid,
    pub flow_label: u32,
    pub sgid_index: u8,
    pub hop_limit: u8,
    pub traffic_class: u8,
}

#[repr(C)]
pub struct rxe_av {
    pub port_num: u8,
    // From RXE_NETWORK_TYPE_*.
    pub network_type: u8,
    pub dmac: [u8; 6],
    pub grh: rxe_global_route,
    pub sgid_addr: rxe_av_sockaddr,
    pub dgid_addr: rxe_av_sockaddr,
}

#[repr(C)]
pub union rxe_av_sockaddr {
    pub _sockaddr_in: sockaddr_in,
    pub _sockaddr_in6: sockaddr_in6,
}

#[repr(C)]
pub struct rxe_send_wr {
    pub wr_id: u64,
    pub reserved: u32,
    pub opcode: u32,
    pub send_flags: u32,
    pub ex: rxe_send_wr_ex,
    pub wr: rxe_send_wr_wr,
}

#[repr(C)]
pub union rxe_send_wr_ex {
    pub imm_data: u32,
    pub invalidate_rkey: u32,
}

#[repr(C)]
pub union rxe_send_wr_wr {
    pub flush: rxe_send_wr_flush,
    pub rdma: rxe_send_wr_rdma,
    pub atomic: rxe_send_wr_atomic,
    pub ud: rxe_send_wr_ud,
    pub mw: rxe_send_wr_mw,
    #[cfg(feature = "kernel")]
    pub reg: rxe_send_wr_reg,
}

#[repr(C)]
pub struct rxe_send_wr_flush {
    pub remote_addr: u64,
    pub length: u32,
    pub rkey: u32,
    pub type_: u8,
    pub level: u8,
}

#[repr(C)]
pub struct rxe_send_wr_rdma {
    pub remote_addr: u64,
    pub rkey: u32,
    pub reserved: u32,
}

#[repr(C)]
pub struct rxe_send_wr_atomic {
    pub remote_addr: u64,
    pub compare_add: u64,
    pub swap: u64,
    pub rkey: u32,
    pub reserved: u32,
}

#[repr(C)]
pub struct rxe_send_wr_ud {
    pub remote_qpn: u32,
    pub remote_qkey: u32,
    pub pkey_index: u16,
    pub reserved: u16,
    pub ah_num: u32,
    pub pad: [u32; 4],
    pub av: rxe_av,
}

#[repr(C)]
pub struct rxe_send_wr_mw {
    pub addr: u64,
    pub length: u64,
    pub mr_lkey: u32,
    pub mw_rkey: u32,
    pub rkey: u32,
    pub access: u32,
}

#[cfg(feature = "kernel")]
#[repr(C)]
pub struct rxe_send_wr_reg {
    pub mr_or_reserved: rxe_send_wr_reg_mr,
    pub key: u32,
    pub access: u32,
}

#[cfg(feature = "kernel")]
#[repr(C)]
pub union rxe_send_wr_reg_mr {
    pub mr: *mut ib_mr,
    pub reserved: u64,
}

#[repr(C)]
pub struct rxe_sge {
    pub addr: u64,
    pub length: u32,
    pub lkey: u32,
}

#[repr(C)]
pub struct mminfo {
    pub offset: u64,
    pub size: u32,
    pub pad: u32,
}

#[repr(C)]
pub union rxe_dma_info_data {
    pub inline_data: [u8; 0],
    pub atomic_wr: [u8; 0],
    pub sge: [rxe_sge; 0],
}

#[repr(C)]
pub struct rxe_dma_info {
    pub length: u32,
    pub resid: u32,
    pub cur_sge: u32,
    pub num_sge: u32,
    pub sge_offset: u32,
    pub reserved: u32,
    pub data: rxe_dma_info_data,
}

#[repr(C)]
pub struct rxe_send_wqe {
    pub wr: rxe_send_wr,
    pub status: u32,
    pub state: u32,
    pub iova: u64,
    pub mask: u32,
    pub first_psn: u32,
    pub last_psn: u32,
    pub ack_length: u32,
    pub ssn: u32,
    pub has_rd_atomic: u32,
    pub dma: rxe_dma_info,
}

#[repr(C)]
pub struct rxe_recv_wqe {
    pub wr_id: u64,
    pub reserved: u32,
    pub padding: u32,
    pub dma: rxe_dma_info,
}

#[repr(C)]
pub struct rxe_create_ah_resp {
    pub ah_num: u32,
    pub reserved: u32,
}

#[repr(C)]
pub struct rxe_create_cq_resp {
    pub mi: mminfo,
}

#[repr(C)]
pub struct rxe_resize_cq_resp {
    pub mi: mminfo,
}

#[repr(C)]
pub struct rxe_create_qp_resp {
    pub rq_mi: mminfo,
    pub sq_mi: mminfo,
}

#[repr(C)]
pub struct rxe_create_srq_resp {
    pub mi: mminfo,
    pub srq_num: u32,
    pub reserved: u32,
}

#[repr(C)]
pub struct rxe_modify_srq_cmd {
    pub mmap_info_addr: u64,
}

/*
 * This data structure is stored at the base of work and completion queues
 * shared between user space and kernel space. It contains producer and
 * consumer indices and a copy of queue size parameters for user space.
 * For performance, producer and consumer indices occupy separate cache lines.
 */
#[repr(C)]
pub struct rxe_queue_buf {
    pub log2_elem_size: u32,
    pub index_mask: u32,
    pub pad_1: [u32; 30],
    pub producer_index: u32,
    pub pad_2: [u32; 31],
    pub consumer_index: u32,
    pub pad_3: [u32; 31],
    pub data: [u8; 0],
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
