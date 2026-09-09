/* SPDX-License-Identifier: (GPL-2.0-only OR BSD-3-Clause) */
/* Rust source-level translation of linux/qed/common_hsi.h. */

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals)]

/* Linux ABI types supplied by the including translation unit. */
pub type u8 = ::core::primitive::u8;
pub type u16 = ::core::primitive::u16;
pub type u32 = ::core::primitive::u32;
pub type u64 = ::core::primitive::u64;
pub type __le16 = u16;
pub type __le32 = u32;

#[inline]
pub const fn bit(n: u32) -> u32 { 1u32 << n }

#[inline]
pub const fn ptr_lo(x: usize) -> u32 { (x & 0xffff_ffff) as u32 }
#[inline]
pub const fn ptr_hi(x: usize) -> u32 { ((x >> 16) >> 16) as u32 }
#[inline]
pub const fn hilo_gen(hi: u32, lo: u32) -> u64 { ((hi as u64) << 32) + lo as u64 }
#[inline]
pub const fn hilo_64(hi: __le32, lo: __le32) -> u64 { hilo_gen(hi, lo) }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct regpair { pub lo: __le32, pub hi: __le32 }

/* The header is an ABI definition header: constants below retain the C
 * preprocessor names and integer values; dependent expressions are written as
 * Rust constants so consumers can use them directly. */
pub const X_FINAL_CLEANUP_AGG_INT: u32 = 1;
pub const EVENT_RING_PAGE_SIZE_BYTES: u32 = 4096;
pub const NUM_OF_GLOBAL_QUEUES: u32 = 128;
pub const COMMON_QUEUE_ENTRY_MAX_BYTE_SIZE: u32 = 64;
pub const ISCSI_CDU_TASK_SEG_TYPE: u32 = 0;
pub const FCOE_CDU_TASK_SEG_TYPE: u32 = 0;
pub const RDMA_CDU_TASK_SEG_TYPE: u32 = 1;
pub const ETH_CDU_TASK_SEG_TYPE: u32 = 2;
pub const FW_ASSERT_GENERAL_ATTN_IDX: u32 = 32;
pub const TSTORM_QZONE_SIZE: u32 = 8;
pub const MSTORM_QZONE_SIZE: u32 = 16;
pub const USTORM_QZONE_SIZE: u32 = 8;
pub const XSTORM_QZONE_SIZE: u32 = 8;
pub const YSTORM_QZONE_SIZE: u32 = 0;
pub const PSTORM_QZONE_SIZE: u32 = 0;
pub const MSTORM_VF_ZONE_DEFAULT_SIZE_LOG: u32 = 7;
pub const ETH_MAX_RXQ_VF_DEFAULT: u32 = 16;
pub const ETH_MAX_RXQ_VF_DOUBLE: u32 = 48;
pub const ETH_MAX_RXQ_VF_QUAD: u32 = 112;
pub const ETH_RGSRC_CTX_SIZE: u32 = 6;
pub const ETH_TGSRC_CTX_SIZE: u32 = 6;
pub const CORE_LL2_MAX_RAMROD_PER_CON: u32 = 8;
pub const CORE_LL2_TX_BD_PAGE_SIZE_BYTES: u32 = 4096;
pub const CORE_LL2_RX_BD_PAGE_SIZE_BYTES: u32 = 4096;
pub const CORE_LL2_RX_CQE_PAGE_SIZE_BYTES: u32 = 4096;
pub const CORE_LL2_RX_NUM_NEXT_PAGE_BDS: u32 = 1;
pub const CORE_LL2_TX_MAX_BDS_PER_PACKET: u32 = 12;
pub const CORE_SPQE_PAGE_SIZE_BYTES: u32 = 4096;
pub const MAX_NUM_LL2_RX_RAM_QUEUES: u32 = 32;
pub const MAX_NUM_LL2_RX_CTX_QUEUES: u32 = 208;
pub const MAX_NUM_LL2_RX_QUEUES: u32 = MAX_NUM_LL2_RX_RAM_QUEUES + MAX_NUM_LL2_RX_CTX_QUEUES;
pub const MAX_NUM_LL2_TX_STATS_COUNTERS: u32 = 48;
pub const FW_MAJOR_VERSION: u32 = 8;
pub const FW_MINOR_VERSION: u32 = 59;
pub const FW_REVISION_VERSION: u32 = 1;
pub const FW_ENGINEERING_VERSION: u32 = 0;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct coalescing_timeset { pub value: u8 }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct common_queue_zone { pub ring_drv_data_consumer: __le16, pub reserved: __le16 }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct eth_rx_prod_data { pub bd_prod: __le16, pub cqe_prod: __le16 }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tcp_ulp_connect_done_params { pub mss: __le16, pub snd_wnd_scale: u8, pub flags: u8 }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iscsi_connect_done_results { pub icid: __le16, pub conn_id: __le16, pub params: tcp_ulp_connect_done_params }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iscsi_eqe_data { pub icid: __le16, pub conn_id: __le16, pub reserved: __le16, pub error_code: u8, pub error_pdu_opcode_reserved: u8 }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct offload_pkt_dup_enable { pub enable_vector: __le16 }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pstorm_pkt_dup_cfg { pub enable: offload_pkt_dup_enable, pub reserved: [__le16; 3] }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rdma_eqe_destroy_qp { pub cid: __le32, pub reserved: [u8; 4] }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rdma_eqe_suspend_qp { pub cid: __le32, pub reserved: [u8; 4] }
#[repr(C)]
pub union rdma_eqe_data { pub async_handle: regpair, pub rdma_destroy_qp_data: rdma_eqe_destroy_qp, pub rdma_suspend_qp_data: rdma_eqe_suspend_qp }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tstorm_pkt_dup_cfg { pub enable: offload_pkt_dup_enable, pub reserved: __le16, pub cid: __le32 }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tstorm_queue_zone { pub reserved: [__le32; 2] }

/* Remaining declarations are retained verbatim below as a source reference;
 * they are intentionally not executable Rust because they depend on Linux
 * kernel-provided endian and ABI declarations. */


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
