/* SPDX-License-Identifier: ((GPL-2.0 WITH Linux-syscall-note) OR Linux-OpenIB) */
/*
 * Copyright (c) 2005-2006 Intel Corporation.  All rights reserved.
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

// External Linux/RDMA types are supplied by the corresponding dependencies.

pub const RDMA_USER_CM_ABI_VERSION: u32 = 4;
pub const RDMA_MAX_PRIVATE_DATA: usize = 256;

pub const RDMA_USER_CM_CMD_CREATE_ID: u32 = 0;
pub const RDMA_USER_CM_CMD_DESTROY_ID: u32 = 1;
pub const RDMA_USER_CM_CMD_BIND_IP: u32 = 2;
pub const RDMA_USER_CM_CMD_RESOLVE_IP: u32 = 3;
pub const RDMA_USER_CM_CMD_RESOLVE_ROUTE: u32 = 4;
pub const RDMA_USER_CM_CMD_QUERY_ROUTE: u32 = 5;
pub const RDMA_USER_CM_CMD_CONNECT: u32 = 6;
pub const RDMA_USER_CM_CMD_LISTEN: u32 = 7;
pub const RDMA_USER_CM_CMD_ACCEPT: u32 = 8;
pub const RDMA_USER_CM_CMD_REJECT: u32 = 9;
pub const RDMA_USER_CM_CMD_DISCONNECT: u32 = 10;
pub const RDMA_USER_CM_CMD_INIT_QP_ATTR: u32 = 11;
pub const RDMA_USER_CM_CMD_GET_EVENT: u32 = 12;
pub const RDMA_USER_CM_CMD_GET_OPTION: u32 = 13;
pub const RDMA_USER_CM_CMD_SET_OPTION: u32 = 14;
pub const RDMA_USER_CM_CMD_NOTIFY: u32 = 15;
pub const RDMA_USER_CM_CMD_JOIN_IP_MCAST: u32 = 16;
pub const RDMA_USER_CM_CMD_LEAVE_MCAST: u32 = 17;
pub const RDMA_USER_CM_CMD_MIGRATE_ID: u32 = 18;
pub const RDMA_USER_CM_CMD_QUERY: u32 = 19;
pub const RDMA_USER_CM_CMD_BIND: u32 = 20;
pub const RDMA_USER_CM_CMD_RESOLVE_ADDR: u32 = 21;
pub const RDMA_USER_CM_CMD_JOIN_MCAST: u32 = 22;
pub const RDMA_USER_CM_CMD_RESOLVE_IB_SERVICE: u32 = 23;
pub const RDMA_USER_CM_CMD_WRITE_CM_EVENT: u32 = 24;

/* See IBTA Annex A11, servies ID bytes 4 & 5 */
#[repr(u16)]
pub enum rdma_ucm_port_space {
    RDMA_PS_IPOIB = 0x0002,
    RDMA_PS_IB = 0x013f,
    RDMA_PS_TCP = 0x0106,
    RDMA_PS_UDP = 0x0111,
}

#[repr(C)]
pub struct rdma_ucm_cmd_hdr { pub cmd: u32, pub r#in: u16, pub out: u16 }
#[repr(C)]
pub struct rdma_ucm_create_id { pub uid: u64, pub response: u64, pub ps: u16, pub qp_type: u8, pub reserved: [u8; 5] }
#[repr(C)]
pub struct rdma_ucm_create_id_resp { pub id: u32 }
#[repr(C)]
pub struct rdma_ucm_destroy_id { pub response: u64, pub id: u32, pub reserved: u32 }
#[repr(C)]
pub struct rdma_ucm_destroy_id_resp { pub events_reported: u32 }
#[repr(C)]
pub struct rdma_ucm_bind_ip { pub response: u64, pub addr: sockaddr_in6, pub id: u32 }
#[repr(C)]
pub struct rdma_ucm_bind { pub id: u32, pub addr_size: u16, pub reserved: u16, pub addr: __kernel_sockaddr_storage }
#[repr(C)]
pub struct rdma_ucm_resolve_ip { pub src_addr: sockaddr_in6, pub dst_addr: sockaddr_in6, pub id: u32, pub timeout_ms: u32 }
#[repr(C)]
pub struct rdma_ucm_resolve_addr { pub id: u32, pub timeout_ms: u32, pub src_size: u16, pub dst_size: u16, pub reserved: u32, pub src_addr: __kernel_sockaddr_storage, pub dst_addr: __kernel_sockaddr_storage }
#[repr(C)]
pub struct rdma_ucm_resolve_route { pub id: u32, pub timeout_ms: u32 }

pub const RDMA_USER_CM_QUERY_ADDR: u32 = 0;
pub const RDMA_USER_CM_QUERY_PATH: u32 = 1;
pub const RDMA_USER_CM_QUERY_GID: u32 = 2;
pub const RDMA_USER_CM_QUERY_IB_SERVICE: u32 = 3;

#[repr(C)]
pub struct rdma_ucm_query { pub response: u64, pub id: u32, pub option: u32 }
#[repr(C)]
pub struct rdma_ucm_query_route_resp { pub node_guid: u64, pub ib_route: [ib_user_path_rec; 2], pub src_addr: sockaddr_in6, pub dst_addr: sockaddr_in6, pub num_paths: u32, pub port_num: u8, pub reserved: [u8; 3], pub ibdev_index: u32, pub reserved1: u32 }
#[repr(C)]
pub struct rdma_ucm_query_addr_resp { pub node_guid: u64, pub port_num: u8, pub reserved: u8, pub pkey: u16, pub src_size: u16, pub dst_size: u16, pub src_addr: __kernel_sockaddr_storage, pub dst_addr: __kernel_sockaddr_storage, pub ibdev_index: u32, pub reserved1: u32 }
#[repr(C)]
pub struct rdma_ucm_query_path_resp { pub num_paths: u32, pub reserved: u32, pub path_data: [ib_path_rec_data; 0] }
#[repr(C)]
pub struct rdma_ucm_query_ib_service_resp { pub num_service_recs: u32, pub reserved: u32, pub recs: [ib_user_service_rec; 0] }
#[repr(C)]
pub struct rdma_ucm_conn_param { pub qp_num: u32, pub qkey: u32, pub private_data: [u8; RDMA_MAX_PRIVATE_DATA], pub private_data_len: u8, pub srq: u8, pub responder_resources: u8, pub initiator_depth: u8, pub flow_control: u8, pub retry_count: u8, pub rnr_retry_count: u8, pub valid: u8 }
#[repr(C)]
pub struct rdma_ucm_ud_param { pub qp_num: u32, pub qkey: u32, pub ah_attr: ib_uverbs_ah_attr, pub private_data: [u8; RDMA_MAX_PRIVATE_DATA], pub private_data_len: u8, pub reserved: [u8; 7] }
#[repr(C)]
pub struct rdma_ucm_ece { pub vendor_id: u32, pub attr_mod: u32 }
#[repr(C)]
pub struct rdma_ucm_connect { pub conn_param: rdma_ucm_conn_param, pub id: u32, pub reserved: u32, pub ece: rdma_ucm_ece }
#[repr(C)]
pub struct rdma_ucm_listen { pub id: u32, pub backlog: u32 }
#[repr(C)]
pub struct rdma_ucm_accept { pub uid: u64, pub conn_param: rdma_ucm_conn_param, pub id: u32, pub reserved: u32, pub ece: rdma_ucm_ece }
#[repr(C)]
pub struct rdma_ucm_reject { pub id: u32, pub private_data_len: u8, pub reason: u8, pub reserved: [u8; 2], pub private_data: [u8; RDMA_MAX_PRIVATE_DATA] }
#[repr(C)]
pub struct rdma_ucm_disconnect { pub id: u32 }
#[repr(C)]
pub struct rdma_ucm_init_qp_attr { pub response: u64, pub id: u32, pub qp_state: u32 }
#[repr(C)]
pub struct rdma_ucm_notify { pub id: u32, pub event: u32 }
#[repr(C)]
pub struct rdma_ucm_join_ip_mcast { pub response: u64, pub uid: u64, pub addr: sockaddr_in6, pub id: u32 }

pub const RDMA_MC_JOIN_FLAG_FULLMEMBER: u32 = 0;
pub const RDMA_MC_JOIN_FLAG_SENDONLY_FULLMEMBER: u32 = 1;
pub const RDMA_MC_JOIN_FLAG_RESERVED: u32 = 2;

#[repr(C)]
pub struct rdma_ucm_join_mcast { pub response: u64, pub uid: u64, pub id: u32, pub addr_size: u16, pub join_flags: u16, pub addr: __kernel_sockaddr_storage }
#[repr(C)]
pub struct rdma_ucm_get_event { pub response: u64 }
#[repr(C)]
pub union rdma_ucm_event_resp_param { pub conn: rdma_ucm_conn_param, pub ud: rdma_ucm_ud_param, pub arg32: [u32; 2] }
#[repr(C)]
pub struct rdma_ucm_event_resp { pub uid: u64, pub id: u32, pub event: u32, pub status: u32, pub param: rdma_ucm_event_resp_param, pub reserved: u32, pub ece: rdma_ucm_ece }

pub const RDMA_OPTION_ID: u32 = 0;
pub const RDMA_OPTION_IB: u32 = 1;
pub const RDMA_OPTION_ID_TOS: u32 = 0;
pub const RDMA_OPTION_ID_REUSEADDR: u32 = 1;
pub const RDMA_OPTION_ID_AFONLY: u32 = 2;
pub const RDMA_OPTION_ID_ACK_TIMEOUT: u32 = 3;
pub const RDMA_OPTION_IB_PATH: u32 = 1;

#[repr(C)]
pub struct rdma_ucm_set_option { pub optval: u64, pub id: u32, pub level: u32, pub optname: u32, pub optlen: u32 }
#[repr(C)]
pub struct rdma_ucm_migrate_id { pub response: u64, pub id: u32, pub fd: u32 }
#[repr(C)]
pub struct rdma_ucm_migrate_resp { pub events_reported: u32 }
pub const RDMA_USER_CM_IB_SERVICE_FLAG_ID: u32 = 1 << 0;
pub const RDMA_USER_CM_IB_SERVICE_FLAG_NAME: u32 = 1 << 1;
pub const RDMA_USER_CM_IB_SERVICE_NAME_SIZE: usize = 64;
#[repr(C)]
pub struct rdma_ucm_ib_service { pub service_id: u64, pub service_name: [u8; RDMA_USER_CM_IB_SERVICE_NAME_SIZE], pub flags: u32, pub reserved: u32 }
#[repr(C)]
pub struct rdma_ucm_resolve_ib_service { pub id: u32, pub reserved: u32, pub ibs: rdma_ucm_ib_service }
#[repr(C)]
pub union rdma_ucm_write_cm_event_param { pub conn: rdma_ucm_conn_param, pub ud: rdma_ucm_ud_param, pub arg: u64 }
#[repr(C)]
pub struct rdma_ucm_write_cm_event { pub id: u32, pub reserved: u32, pub event: u32, pub status: u32, pub param: rdma_ucm_write_cm_event_param }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
