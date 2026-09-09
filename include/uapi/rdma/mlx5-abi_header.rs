/* SPDX-License-Identifier: ((GPL-2.0 WITH Linux-syscall-note) OR Linux-OpenIB) */
/* Copyright (c) 2013-2015, Mellanox Technologies. All rights reserved. */

pub type __u8 = u8;
pub type __u16 = u16;
pub type __u32 = u32;
pub type __u64 = u64;
pub type __aligned_u64 = u64;
pub const ETH_ALEN: usize = 6;

pub const MLX5_QP_FLAG_SIGNATURE: u32 = 1 << 0;
pub const MLX5_QP_FLAG_SCATTER_CQE: u32 = 1 << 1;
pub const MLX5_QP_FLAG_TUNNEL_OFFLOADS: u32 = 1 << 2;
pub const MLX5_QP_FLAG_BFREG_INDEX: u32 = 1 << 3;
pub const MLX5_QP_FLAG_TYPE_DCT: u32 = 1 << 4;
pub const MLX5_QP_FLAG_TYPE_DCI: u32 = 1 << 5;
pub const MLX5_QP_FLAG_TIR_ALLOW_SELF_LB_UC: u32 = 1 << 6;
pub const MLX5_QP_FLAG_TIR_ALLOW_SELF_LB_MC: u32 = 1 << 7;
pub const MLX5_QP_FLAG_ALLOW_SCATTER_CQE: u32 = 1 << 8;
pub const MLX5_QP_FLAG_PACKET_BASED_CREDIT_MODE: u32 = 1 << 9;
pub const MLX5_QP_FLAG_UAR_PAGE_INDEX: u32 = 1 << 10;
pub const MLX5_QP_FLAG_DCI_STREAM: u32 = 1 << 11;
pub const MLX5_SRQ_FLAG_SIGNATURE: u32 = 1 << 0;
pub const MLX5_WQ_FLAG_SIGNATURE: u32 = 1 << 0;
pub const MLX5_IB_UVERBS_ABI_VERSION: u32 = 1;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ib_alloc_ucontext_req { pub total_num_bfregs: __u32, pub num_low_latency_bfregs: __u32 }

pub const MLX5_LIB_CAP_4K_UAR: __u64 = 1 << 0;
pub const MLX5_LIB_CAP_DYN_UAR: __u64 = 1 << 1;
pub const MLX5_IB_ALLOC_UCTX_DEVX: __u32 = 1 << 0;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ib_alloc_ucontext_req_v2 { pub total_num_bfregs: __u32, pub num_low_latency_bfregs: __u32, pub flags: __u32, pub comp_mask: __u32, pub max_cqe_version: __u8, pub reserved0: __u8, pub reserved1: __u16, pub reserved2: __u32, pub lib_caps: __aligned_u64 }

pub const MLX5_IB_ALLOC_UCONTEXT_RESP_MASK_CORE_CLOCK_OFFSET: u64 = 1 << 0;
pub const MLX5_IB_ALLOC_UCONTEXT_RESP_MASK_DUMP_FILL_MKEY: u64 = 1 << 1;
pub const MLX5_IB_ALLOC_UCONTEXT_RESP_MASK_ECE: u64 = 1 << 2;
pub const MLX5_IB_ALLOC_UCONTEXT_RESP_MASK_SQD2RTS: u64 = 1 << 3;
pub const MLX5_IB_ALLOC_UCONTEXT_RESP_MASK_REAL_TIME_TS: u64 = 1 << 4;
pub const MLX5_IB_ALLOC_UCONTEXT_RESP_MASK_MKEY_UPDATE_TAG: u64 = 1 << 5;
pub const MLX5_USER_CMDS_SUPP_UHW_QUERY_DEVICE: u32 = 1 << 0;
pub const MLX5_USER_CMDS_SUPP_UHW_CREATE_AH: u32 = 1 << 1;
pub const MLX5_USER_INLINE_MODE_NA: u32 = 0;
pub const MLX5_USER_INLINE_MODE_NONE: u32 = 1;
pub const MLX5_USER_INLINE_MODE_L2: u32 = 2;
pub const MLX5_USER_INLINE_MODE_IP: u32 = 3;
pub const MLX5_USER_INLINE_MODE_TCP_UDP: u32 = 4;
pub const MLX5_USER_ALLOC_UCONTEXT_FLOW_ACTION_FLAGS_ESP_AES_GCM: u32 = 1 << 0;
pub const MLX5_USER_ALLOC_UCONTEXT_FLOW_ACTION_FLAGS_ESP_AES_GCM_REQ_METADATA: u32 = 1 << 1;
pub const MLX5_USER_ALLOC_UCONTEXT_FLOW_ACTION_FLAGS_ESP_AES_GCM_SPI_STEERING: u32 = 1 << 2;
pub const MLX5_USER_ALLOC_UCONTEXT_FLOW_ACTION_FLAGS_ESP_AES_GCM_FULL_OFFLOAD: u32 = 1 << 3;
pub const MLX5_USER_ALLOC_UCONTEXT_FLOW_ACTION_FLAGS_ESP_AES_GCM_TX_IV_IS_ESN: u32 = 1 << 4;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ib_alloc_ucontext_resp { pub qp_tab_size: __u32, pub bf_reg_size: __u32, pub tot_bfregs: __u32, pub cache_line_size: __u32, pub max_sq_desc_sz: __u16, pub max_rq_desc_sz: __u16, pub max_send_wqebb: __u32, pub max_recv_wr: __u32, pub max_srq_recv_wr: __u32, pub num_ports: __u16, pub flow_action_flags: __u16, pub comp_mask: __u32, pub response_length: __u32, pub cqe_version: __u8, pub cmds_supp_uhw: __u8, pub eth_min_inline: __u8, pub clock_info_versions: __u8, pub hca_core_clock_offset: __aligned_u64, pub log_uar_size: __u32, pub num_uars_per_page: __u32, pub num_dyn_bfregs: __u32, pub dump_fill_mkey: __u32 }
#[repr(C)] #[derive(Copy, Clone)] pub struct mlx5_ib_alloc_pd_resp { pub pdn: __u32 }
#[repr(C)] #[derive(Copy, Clone)] pub struct mlx5_ib_tso_caps { pub max_tso: __u32, pub supported_qpts: __u32 }
#[repr(C)] #[derive(Copy, Clone)] pub struct mlx5_ib_rss_caps { pub rx_hash_fields_mask: __aligned_u64, pub rx_hash_function: __u8, pub reserved: [__u8; 7] }
pub const MLX5_IB_CQE_RES_FORMAT_HASH: u32 = 1 << 0;
pub const MLX5_IB_CQE_RES_FORMAT_CSUM: u32 = 1 << 1;
pub const MLX5_IB_CQE_RES_FORMAT_CSUM_STRIDX: u32 = 1 << 2;
#[repr(C)] #[derive(Copy, Clone)] pub struct mlx5_ib_cqe_comp_caps { pub max_num: __u32, pub supported_format: __u32 }
pub const MLX5_IB_PP_SUPPORT_BURST: u32 = 1 << 0;
#[repr(C)] #[derive(Copy, Clone)] pub struct mlx5_packet_pacing_caps { pub qp_rate_limit_min: __u32, pub qp_rate_limit_max: __u32, pub supported_qpts: __u32, pub cap_flags: __u8, pub reserved: [__u8; 3] }
pub const MPW_RESERVED: u32 = 1 << 0; pub const MLX5_IB_ALLOW_MPW: u32 = 1 << 1; pub const MLX5_IB_SUPPORT_EMPW: u32 = 1 << 2;
pub const MLX5_IB_SW_PARSING: u32 = 1 << 0; pub const MLX5_IB_SW_PARSING_CSUM: u32 = 1 << 1; pub const MLX5_IB_SW_PARSING_LSO: u32 = 1 << 2;
#[repr(C)] #[derive(Copy, Clone)] pub struct mlx5_ib_sw_parsing_caps { pub sw_parsing_offloads: __u32, pub supported_qpts: __u32 }
#[repr(C)] #[derive(Copy, Clone)] pub struct mlx5_ib_striding_rq_caps { pub min_single_stride_log_num_of_bytes: __u32, pub max_single_stride_log_num_of_bytes: __u32, pub min_single_wqe_log_num_of_strides: __u32, pub max_single_wqe_log_num_of_strides: __u32, pub supported_qpts: __u32, pub reserved: __u32 }
#[repr(C)] #[derive(Copy, Clone)] pub struct mlx5_ib_dci_streams_caps { pub max_log_num_concurent: __u8, pub max_log_num_errored: __u8 }
pub const MLX5_IB_QUERY_DEV_RESP_FLAGS_CQE_128B_COMP: u32 = 1 << 0; pub const MLX5_IB_QUERY_DEV_RESP_FLAGS_CQE_128B_PAD: u32 = 1 << 1; pub const MLX5_IB_QUERY_DEV_RESP_PACKET_BASED_CREDIT_MODE: u32 = 1 << 2; pub const MLX5_IB_QUERY_DEV_RESP_FLAGS_SCAT2CQE_DCT: u32 = 1 << 3; pub const MLX5_IB_QUERY_DEV_RESP_FLAGS_OOO_DP: u32 = 1 << 4;
pub const MLX5_IB_TUNNELED_OFFLOADS_VXLAN: u32 = 1 << 0; pub const MLX5_IB_TUNNELED_OFFLOADS_GRE: u32 = 1 << 1; pub const MLX5_IB_TUNNELED_OFFLOADS_GENEVE: u32 = 1 << 2; pub const MLX5_IB_TUNNELED_OFFLOADS_MPLS_GRE: u32 = 1 << 3; pub const MLX5_IB_TUNNELED_OFFLOADS_MPLS_UDP: u32 = 1 << 4;
#[repr(C)] #[derive(Copy, Clone)] pub struct mlx5_ib_query_device_resp { pub comp_mask: __u32, pub response_length: __u32, pub tso_caps: mlx5_ib_tso_caps, pub rss_caps: mlx5_ib_rss_caps, pub cqe_comp_caps: mlx5_ib_cqe_comp_caps, pub packet_pacing_caps: mlx5_packet_pacing_caps, pub mlx5_ib_support_multi_pkt_send_wqes: __u32, pub flags: __u32, pub sw_parsing_caps: mlx5_ib_sw_parsing_caps, pub striding_rq_caps: mlx5_ib_striding_rq_caps, pub tunnel_offloads_caps: __u32, pub dci_streams_caps: mlx5_ib_dci_streams_caps, pub reserved: __u16, pub reg_c0: mlx5_ib_uapi_reg }
pub const MLX5_IB_CREATE_CQ_FLAGS_CQE_128B_PAD: u16 = 1 << 0; pub const MLX5_IB_CREATE_CQ_FLAGS_UAR_PAGE_INDEX: u16 = 1 << 1; pub const MLX5_IB_CREATE_CQ_FLAGS_REAL_TIME_TS: u16 = 1 << 2;
#[repr(C)] #[derive(Copy, Clone)] pub struct mlx5_ib_create_cq { pub buf_addr: __aligned_u64, pub db_addr: __aligned_u64, pub cqe_size: __u32, pub cqe_comp_en: __u8, pub cqe_comp_res_format: __u8, pub flags: __u16, pub uar_page_index: __u16, pub reserved0: __u16, pub reserved1: __u32 }
#[repr(C)] #[derive(Copy, Clone)] pub struct mlx5_ib_create_cq_resp { pub cqn: __u32, pub reserved: __u32 }
#[repr(C)] #[derive(Copy, Clone)] pub struct mlx5_ib_resize_cq { pub buf_addr: __aligned_u64, pub cqe_size: __u16, pub reserved0: __u16, pub reserved1: __u32 }
#[repr(C)] #[derive(Copy, Clone)] pub struct mlx5_ib_create_srq { pub buf_addr: __aligned_u64, pub db_addr: __aligned_u64, pub flags: __u32, pub reserved0: __u32, pub uidx: __u32, pub reserved1: __u32 }
#[repr(C)] #[derive(Copy, Clone)] pub struct mlx5_ib_create_srq_resp { pub srqn: __u32, pub reserved: __u32 }
#[repr(C)] #[derive(Copy, Clone)] pub struct mlx5_ib_create_qp_dci_streams { pub log_num_concurent: __u8, pub log_num_errored: __u8 }
#[repr(C)] pub union mlx5_ib_create_qp_union { pub sq_buf_addr: __aligned_u64, pub access_key: __aligned_u64 }
#[repr(C)] pub struct mlx5_ib_create_qp { pub buf_addr: __aligned_u64, pub db_addr: __aligned_u64, pub sq_wqe_count: __u32, pub rq_wqe_count: __u32, pub rq_wqe_shift: __u32, pub flags: __u32, pub uidx: __u32, pub bfreg_index: __u32, pub sq_buf_addr: mlx5_ib_create_qp_union, pub ece_options: __u32, pub dci_streams: mlx5_ib_create_qp_dci_streams, pub reserved: __u16 }
pub const MLX5_RX_HASH_FUNC_TOEPLITZ: u32 = 1 << 0;
pub const MLX5_RX_HASH_SRC_IPV4: u64 = 1 << 0; pub const MLX5_RX_HASH_DST_IPV4: u64 = 1 << 1; pub const MLX5_RX_HASH_SRC_IPV6: u64 = 1 << 2; pub const MLX5_RX_HASH_DST_IPV6: u64 = 1 << 3; pub const MLX5_RX_HASH_SRC_PORT_TCP: u64 = 1 << 4; pub const MLX5_RX_HASH_DST_PORT_TCP: u64 = 1 << 5; pub const MLX5_RX_HASH_SRC_PORT_UDP: u64 = 1 << 6; pub const MLX5_RX_HASH_DST_PORT_UDP: u64 = 1 << 7; pub const MLX5_RX_HASH_IPSEC_SPI: u64 = 1 << 8; pub const MLX5_RX_HASH_INNER: u64 = 1u64 << 31;
#[repr(C)] #[derive(Copy, Clone)] pub struct mlx5_ib_create_qp_rss { pub rx_hash_fields_mask: __aligned_u64, pub rx_hash_function: __u8, pub rx_key_len: __u8, pub reserved: [__u8; 6], pub rx_hash_key: [__u8; 128], pub comp_mask: __u32, pub flags: __u32 }
pub const MLX5_IB_CREATE_QP_RESP_MASK_TIRN: u64 = 1 << 0; pub const MLX5_IB_CREATE_QP_RESP_MASK_TISN: u64 = 1 << 1; pub const MLX5_IB_CREATE_QP_RESP_MASK_RQN: u64 = 1 << 2; pub const MLX5_IB_CREATE_QP_RESP_MASK_SQN: u64 = 1 << 3; pub const MLX5_IB_CREATE_QP_RESP_MASK_TIR_ICM_ADDR: u64 = 1 << 4;
#[repr(C)] #[derive(Copy, Clone)] pub struct mlx5_ib_create_qp_resp { pub bfreg_index: __u32, pub ece_options: __u32, pub comp_mask: __u32, pub tirn: __u32, pub tisn: __u32, pub rqn: __u32, pub sqn: __u32, pub reserved1: __u32, pub tir_icm_addr: __u64 }
#[repr(C)] #[derive(Copy, Clone)] pub struct mlx5_ib_alloc_mw { pub comp_mask: __u32, pub num_klms: __u8, pub reserved1: __u8, pub reserved2: __u16 }
pub const MLX5_IB_CREATE_WQ_STRIDING_RQ: u32 = 1 << 0;
#[repr(C)] #[derive(Copy, Clone)] pub struct mlx5_ib_create_wq { pub buf_addr: __aligned_u64, pub db_addr: __aligned_u64, pub rq_wqe_count: __u32, pub rq_wqe_shift: __u32, pub user_index: __u32, pub flags: __u32, pub comp_mask: __u32, pub single_stride_log_num_of_bytes: __u32, pub single_wqe_log_num_of_strides: __u32, pub two_byte_shift_en: __u32 }
#[repr(C)] #[derive(Copy, Clone)] pub struct mlx5_ib_create_ah_resp { pub response_length: __u32, pub dmac: [__u8; ETH_ALEN], pub reserved: [__u8; 6] }
#[repr(C)] #[derive(Copy, Clone)] pub struct mlx5_ib_burst_info { pub max_burst_sz: __u32, pub typical_pkt_sz: __u16, pub reserved: __u16 }
pub const MLX5_IB_MODIFY_QP_OOO_DP: u32 = 1 << 0;
#[repr(C)] #[derive(Copy, Clone)] pub struct mlx5_ib_modify_qp { pub comp_mask: __u32, pub burst_info: mlx5_ib_burst_info, pub ece_options: __u32 }
#[repr(C)] #[derive(Copy, Clone)] pub struct mlx5_ib_modify_qp_resp { pub response_length: __u32, pub dctn: __u32, pub ece_options: __u32, pub reserved: __u32 }
#[repr(C)] #[derive(Copy, Clone)] pub struct mlx5_ib_create_wq_resp { pub response_length: __u32, pub reserved: __u32 }
#[repr(C)] #[derive(Copy, Clone)] pub struct mlx5_ib_create_rwq_ind_tbl_resp { pub response_length: __u32, pub reserved: __u32 }
#[repr(C)] #[derive(Copy, Clone)] pub struct mlx5_ib_modify_wq { pub comp_mask: __u32, pub reserved: __u32 }
pub type mlx5_ib_clock_info = ib_uverbs_clock_info;
pub const MLX5_IB_MMAP_REGULAR_PAGE: u32 = 0; pub const MLX5_IB_MMAP_GET_CONTIGUOUS_PAGES: u32 = 1; pub const MLX5_IB_MMAP_WC_PAGE: u32 = 2; pub const MLX5_IB_MMAP_NC_PAGE: u32 = 3; pub const MLX5_IB_MMAP_CORE_CLOCK: u32 = 5; pub const MLX5_IB_MMAP_ALLOC_WC: u32 = 6; pub const MLX5_IB_MMAP_CLOCK_INFO: u32 = 7; pub const MLX5_IB_MMAP_DEVICE_MEM: u32 = 8;
pub const MLX5_IB_CLOCK_INFO_KERNEL_UPDATING: u32 = 1;
pub const MLX5_IB_CLOCK_INFO_V1: u32 = 0;
#[repr(C)] #[derive(Copy, Clone)] pub struct mlx5_ib_flow_counters_desc { pub description: __u32, pub index: __u32 }
#[repr(C)] #[derive(Copy, Clone)] pub struct mlx5_ib_flow_counters_data { pub counters_data: *mut mlx5_ib_flow_counters_desc, pub ncounters: __u32, pub reserved: __u32 }
#[repr(C)] pub struct mlx5_ib_create_flow { pub ncounters_data: __u32, pub reserved: __u32, pub data: [mlx5_ib_flow_counters_data; 0] }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
