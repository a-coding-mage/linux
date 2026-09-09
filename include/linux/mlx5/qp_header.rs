/* Translated from linux/mlx5/qp.h. */

pub const MLX5_TERMINATE_SCATTER_LIST_LKEY: u32 = 0x100u32.to_be();
pub const MLX5_SIG_WQE_SIZE: u32 = MLX5_SEND_WQE_BB * 8;
pub const MLX5_DIF_SIZE: u32 = 8;
pub const MLX5_STRIDE_BLOCK_OP: u32 = 0x400;
pub const MLX5_CPY_GRD_MASK: u32 = 0xc0;
pub const MLX5_CPY_APP_MASK: u32 = 0x30;
pub const MLX5_CPY_REF_MASK: u32 = 0x0f;
pub const MLX5_BSF_INC_REFTAG: u32 = 1 << 6;
pub const MLX5_BSF_INL_VALID: u32 = 1 << 15;
pub const MLX5_BSF_REFRESH_DIF: u32 = 1 << 14;
pub const MLX5_BSF_REPEAT_BLOCK: u32 = 1 << 7;
pub const MLX5_BSF_APPTAG_ESCAPE: u32 = 0x1;
pub const MLX5_BSF_APPREF_ESCAPE: u32 = 0x2;

#[repr(C)]
#[derive(Copy, Clone)]
pub enum mlx5_qp_optpar { MLX5_QP_OPTPAR_ALT_ADDR_PATH=1<<0, MLX5_QP_OPTPAR_RRE=1<<1, MLX5_QP_OPTPAR_RAE=1<<2, MLX5_QP_OPTPAR_RWE=1<<3, MLX5_QP_OPTPAR_PKEY_INDEX=1<<4, MLX5_QP_OPTPAR_Q_KEY=1<<5, MLX5_QP_OPTPAR_RNR_TIMEOUT=1<<6, MLX5_QP_OPTPAR_PRIMARY_ADDR_PATH=1<<7, MLX5_QP_OPTPAR_SRA_MAX=1<<8, MLX5_QP_OPTPAR_RRA_MAX=1<<9, MLX5_QP_OPTPAR_PM_STATE=1<<10, MLX5_QP_OPTPAR_RETRY_COUNT=1<<12, MLX5_QP_OPTPAR_RNR_RETRY=1<<13, MLX5_QP_OPTPAR_ACK_TIMEOUT=1<<14, MLX5_QP_OPTPAR_LAG_TX_AFF=1<<15, MLX5_QP_OPTPAR_PRI_PORT=1<<16, MLX5_QP_OPTPAR_SRQN=1<<18, MLX5_QP_OPTPAR_CQN_RCV=1<<19, MLX5_QP_OPTPAR_DC_HS=1<<20, MLX5_QP_OPTPAR_DC_KEY=1<<21, MLX5_QP_OPTPAR_PP_INDEX=1<<22, MLX5_QP_OPTPAR_COUNTER_SET_ID=1<<25 }
#[repr(C)] #[derive(Copy, Clone)] pub enum mlx5_qp_state { MLX5_QP_STATE_RST=0, MLX5_QP_STATE_INIT=1, MLX5_QP_STATE_RTR=2, MLX5_QP_STATE_RTS=3, MLX5_QP_STATE_SQER=4, MLX5_QP_STATE_SQD=5, MLX5_QP_STATE_ERR=6, MLX5_QP_STATE_SQ_DRAINING=7, MLX5_QP_STATE_SUSPENDED=9, MLX5_QP_NUM_STATE, MLX5_QP_STATE, MLX5_QP_STATE_BAD }

pub const MLX5_SQ_STATE_NA: u32 = MLX5_SQC_STATE_ERR + 1; pub const MLX5_SQ_NUM_STATE:u32=MLX5_SQ_STATE_NA+1; pub const MLX5_RQ_STATE_NA:u32=MLX5_RQC_STATE_ERR+1; pub const MLX5_RQ_NUM_STATE:u32=MLX5_RQ_STATE_NA+1;
pub const MLX5_QP_ST_RC:u32=0; pub const MLX5_QP_ST_UC:u32=1; pub const MLX5_QP_ST_UD:u32=2; pub const MLX5_QP_ST_XRC:u32=3; pub const MLX5_QP_ST_MLX:u32=4; pub const MLX5_QP_ST_DCI:u32=5; pub const MLX5_QP_ST_DCT:u32=6; pub const MLX5_QP_ST_QP0:u32=7; pub const MLX5_QP_ST_QP1:u32=8; pub const MLX5_QP_ST_RAW_ETHERTYPE:u32=9; pub const MLX5_QP_ST_RAW_IPV6:u32=10; pub const MLX5_QP_ST_SNIFFER:u32=11; pub const MLX5_QP_ST_REG_UMR:u32=12; pub const MLX5_QP_ST_PTP_1588:u32=13; pub const MLX5_QP_ST_SYNC_UMR:u32=14; pub const MLX5_QP_ST_MAX:u32=15;
pub const MLX5_QP_PM_MIGRATED:u32=3; pub const MLX5_QP_PM_ARMED:u32=0; pub const MLX5_QP_PM_REARM:u32=1;
pub const MLX5_NON_ZERO_RQ:u32=0; pub const MLX5_SRQ_RQ:u32=1; pub const MLX5_CRQ_RQ:u32=2; pub const MLX5_ZERO_LEN_RQ:u32=3;
pub const MLX5_QP_BIT_SRE:u32=1<<15; pub const MLX5_QP_BIT_SWE:u32=1<<14; pub const MLX5_QP_BIT_SAE:u32=1<<13; pub const MLX5_QP_BIT_RRE:u32=1<<15; pub const MLX5_QP_BIT_RWE:u32=1<<14; pub const MLX5_QP_BIT_RAE:u32=1<<13; pub const MLX5_QP_BIT_RIC:u32=1<<4; pub const MLX5_QP_BIT_CC_SLAVE_RECV:u32=1<<2; pub const MLX5_QP_BIT_CC_SLAVE_SEND:u32=1<<1; pub const MLX5_QP_BIT_CC_MASTER:u32=1;
pub const MLX5_WQE_CTRL_CQ_UPDATE:u32=2<<2; pub const MLX5_WQE_CTRL_CQ_UPDATE_AND_EQE:u32=3<<2; pub const MLX5_WQE_CTRL_SOLICITED:u32=1<<1; pub const MLX5_WQE_CTRL_INITIATOR_SMALL_FENCE:u32=1<<5;
pub const MLX5_SEND_WQE_DS:u32=16; pub const MLX5_SEND_WQE_BB:u32=64; pub const MLX5_SEND_WQEBB_NUM_DS:u32=MLX5_SEND_WQE_BB/MLX5_SEND_WQE_DS; pub const MLX5_SEND_WQE_MAX_WQEBBS:u32=16; pub const MLX5_SEND_WQE_MAX_SIZE:u32=MLX5_SEND_WQE_MAX_WQEBBS*MLX5_SEND_WQE_BB;
pub const MLX5_WQE_FMR_PERM_LOCAL_READ:u32=1<<27; pub const MLX5_WQE_FMR_PERM_LOCAL_WRITE:u32=1<<28; pub const MLX5_WQE_FMR_PERM_REMOTE_READ:u32=1<<29; pub const MLX5_WQE_FMR_PERM_REMOTE_WRITE:u32=1<<30; pub const MLX5_WQE_FMR_PERM_ATOMIC:u32=1<<31;
pub const MLX5_FENCE_MODE_NONE:u32=0<<5; pub const MLX5_FENCE_MODE_INITIATOR_SMALL:u32=1<<5; pub const MLX5_FENCE_MODE_FENCE:u32=2<<5; pub const MLX5_FENCE_MODE_STRONG_ORDERING:u32=3<<5; pub const MLX5_FENCE_MODE_SMALL_AND_FENCE:u32=4<<5;
pub const MLX5_RCV_DBR:u32=0; pub const MLX5_SND_DBR:u32=1; pub const MLX5_FLAGS_INLINE:u32=1<<7; pub const MLX5_FLAGS_CHECK_FREE:u32=1<<5;

#[repr(C)] pub struct mlx5_wqe_fmr_seg { pub flags: __be32, pub mem_key: __be32, pub buf_list: __be64, pub start_addr: __be64, pub reg_len: __be64, pub offset: __be32, pub page_size: __be32, pub reserved: [u32;2] }
#[repr(C)] pub union mlx5_wqe_ctrl_seg_trailer_union { pub general_id: __be32, pub imm: __be32, pub umr_mkey: __be32, pub tis_tir_num: __be32 }
#[repr(C)] pub struct mlx5_wqe_ctrl_seg { pub opmod_idx_opcode:__be32, pub qpn_ds:__be32, pub signature:u8, pub rsvd:[u8;2], pub fm_ce_se:u8, pub trailer:mlx5_wqe_ctrl_seg_trailer_union }
pub const MLX5_WQE_CTRL_DS_MASK:u32=0x3f; pub const MLX5_WQE_CTRL_QPN_MASK:u32=0xffffff00; pub const MLX5_WQE_CTRL_QPN_SHIFT:u32=8; pub const MLX5_WQE_DS_UNITS:u32=16; pub const MLX5_WQE_CTRL_OPCODE_MASK:u32=0xff; pub const MLX5_WQE_CTRL_WQE_INDEX_MASK:u32=0x00ffff00; pub const MLX5_WQE_CTRL_WQE_INDEX_SHIFT:u32=8;
pub const MLX5_ETH_WQE_L3_INNER_CSUM:u32=1<<4; pub const MLX5_ETH_WQE_L4_INNER_CSUM:u32=1<<5; pub const MLX5_ETH_WQE_L3_CSUM:u32=1<<6; pub const MLX5_ETH_WQE_L4_CSUM:u32=1<<7;
pub const MLX5_ETH_WQE_TRAILER_HDR_OUTER_IP_ASSOC:u32=1<<26; pub const MLX5_ETH_WQE_TRAILER_HDR_OUTER_L4_ASSOC:u32=1<<27; pub const MLX5_ETH_WQE_TRAILER_HDR_INNER_IP_ASSOC:u32=3<<26; pub const MLX5_ETH_WQE_TRAILER_HDR_INNER_L4_ASSOC:u32=1<<28; pub const MLX5_ETH_WQE_INSERT_TRAILER:u32=1<<30;
pub const MLX5_ETH_WQE_SWP_INNER_L3_IPV6:u32=1; pub const MLX5_ETH_WQE_SWP_INNER_L4_UDP:u32=2; pub const MLX5_ETH_WQE_SWP_OUTER_L3_IPV6:u32=1<<4; pub const MLX5_ETH_WQE_SWP_OUTER_L4_UDP:u32=1<<5; pub const MLX5_ETH_WQE_FT_META_SHIFT:u32=8;
pub const MLX5_ETH_WQE_FT_META_IPSEC:u32=1<<8; pub const MLX5_ETH_WQE_FT_META_MACSEC:u32=1<<9; pub const MLX5_ETH_WQE_FT_META_MACSEC_FS_ID_MASK:u32=0x3c<<8;

#[repr(C)] pub struct mlx5_wqe_eth_seg { pub swp_outer_l4_offset:u8,pub swp_outer_l3_offset:u8,pub swp_inner_l4_offset:u8,pub swp_inner_l3_offset:u8,pub cs_flags:u8,pub swp_flags:u8,pub mss:__be16,pub flow_table_metadata:__be32,pub trailer_or_inline:mlx5_wqe_eth_seg_union }
#[repr(C)] pub union mlx5_wqe_eth_seg_union { pub trailer:__be32, pub inline_hdr:mlx5_wqe_eth_seg_inline }
#[repr(C)] pub struct mlx5_wqe_eth_seg_inline { pub sz:__be16, pub start:[u8;2], pub data:[u8;0] }
#[repr(C)] pub struct mlx5_wqe_xrc_seg { pub xrc_srqn:__be32,pub rsvd:[u8;12] }
#[repr(C)] pub struct mlx5_wqe_masked_atomic_seg { pub swap_add:__be64,pub compare:__be64,pub swap_add_mask:__be64,pub compare_mask:__be64 }
#[repr(C)] pub union mlx5_base_av_key { pub qkey:mlx5_base_av_qkey,pub dc_key:__be64 } #[repr(C)] pub struct mlx5_base_av_qkey { pub qkey:__be32,pub reserved:__be32 }
#[repr(C)] pub struct mlx5_base_av { pub key:mlx5_base_av_key,pub dqp_dct:__be32,pub stat_rate_sl:u8,pub fl_mlid:u8,pub rlid_or_udp_sport:__be16 }
#[repr(C)] pub union mlx5_av_key { pub qkey:mlx5_base_av_qkey,pub dc_key:__be64 }
#[repr(C)] pub struct mlx5_av { pub key:mlx5_av_key,pub dqp_dct:__be32,pub stat_rate_sl:u8,pub fl_mlid:u8,pub rlid_or_udp_sport:__be16,pub reserved0:[u8;4],pub rmac:[u8;6],pub tclass:u8,pub hop_limit:u8,pub grh_gid_fl:__be32,pub rgid:[u8;16] }
#[repr(C)] pub struct mlx5_ib_ah { pub ibah:ib_ah,pub av:mlx5_av,pub xmit_port:u8 }
#[inline] pub unsafe fn to_mah(ibah:*mut ib_ah)->*mut mlx5_ib_ah { (ibah as *mut u8).sub(std::mem::offset_of!(mlx5_ib_ah, ibah)) as *mut mlx5_ib_ah }
#[repr(C)] pub struct mlx5_wqe_datagram_seg { pub av:mlx5_av } #[repr(C)] pub struct mlx5_wqe_raddr_seg { pub raddr:__be64,pub rkey:__be32,pub reserved:u32 } #[repr(C)] pub struct mlx5_wqe_atomic_seg { pub swap_add:__be64,pub compare:__be64 } #[repr(C)] pub struct mlx5_wqe_data_seg { pub byte_count:__be32,pub lkey:__be32,pub addr:__be64 }
#[repr(C)] pub struct mlx5_wqe_umr_ctrl_seg { pub flags:u8,pub rsvd0:[u8;3],pub xlt_octowords:__be16,pub xlt_offset_or_bsf:__be16,pub mkey_mask:__be64,pub xlt_offset_47_16:__be32,pub rsvd1:[u8;28] }
#[repr(C)] pub struct mlx5_seg_set_psv { pub psv_num:__be32,pub syndrome:__be16,pub status:__be16,pub transient_sig:__be32,pub ref_tag:__be32 }
#[repr(C)] pub struct mlx5_seg_get_psv { pub rsvd:[u8;19],pub num_psv:u8,pub l_key:__be32,pub va:__be64,pub psv_index:[__be32;4] }
#[repr(C)] pub struct mlx5_seg_check_psv { pub rsvd0:[u8;2],pub err_coalescing_op:__be16,pub rsvd1:[u8;2],pub xport_err_op:__be16,pub rsvd2:[u8;2],pub xport_err_mask:__be16,pub rsvd3:[u8;7],pub num_psv:u8,pub l_key:__be32,pub va:__be64,pub psv_index:[__be32;4] }
#[repr(C)] pub struct mlx5_rwqe_sig { pub rsvd0:[u8;4],pub signature:u8,pub rsvd1:[u8;11] } #[repr(C)] pub struct mlx5_wqe_signature_seg { pub rsvd0:[u8;4],pub signature:u8,pub rsvd1:[u8;11] }
pub const MLX5_WQE_INLINE_SEG_BYTE_COUNT_MASK:u32=0x3ff; #[repr(C)] pub struct mlx5_wqe_inline_seg { pub byte_count:__be32,pub data:[__be32;0] }
#[repr(C)] #[derive(Copy,Clone)] pub enum mlx5_sig_type { MLX5_DIF_CRC=1, MLX5_DIF_IPCS=2 }
#[repr(C)] pub struct mlx5_bsf_inl { pub vld_refresh:__be16,pub dif_apptag:__be16,pub dif_reftag:__be32,pub sig_type:u8,pub rp_inv_seed:u8,pub rsvd:[u8;3],pub dif_inc_ref_guard_check:u8,pub dif_app_bitmask_check:__be16 }
#[repr(C)] pub struct mlx5_bsf_basic { pub bsf_size_sbs:u8,pub check_byte_mask:u8,pub wire:u8,pub mem:u8,pub raw_data_size:__be32,pub w_bfs_psv:__be32,pub m_bfs_psv:__be32 } #[repr(C)] pub struct mlx5_bsf_ext { pub t_init_gen_pro_size:__be32,pub rsvd_epi_size:__be32,pub w_tfs_psv:__be32,pub m_tfs_psv:__be32 } #[repr(C)] pub struct mlx5_bsf { pub basic:mlx5_bsf_basic,pub ext:mlx5_bsf_ext,pub w_inl:mlx5_bsf_inl,pub m_inl:mlx5_bsf_inl }
#[repr(C)] pub struct mlx5_mtt { pub ptag:__be64 } #[repr(C)] pub struct mlx5_klm { pub bcount:__be32,pub key:__be32,pub va:__be64 } #[repr(C)] pub struct mlx5_ksm { pub reserved:__be32,pub key:__be32,pub va:__be64 } #[repr(C)] pub struct mlx5_stride_block_entry { pub stride:__be16,pub bcount:__be16,pub key:__be32,pub va:__be64 } #[repr(C)] pub struct mlx5_stride_block_ctrl_seg { pub bcount_per_cycle:__be32,pub op:__be32,pub repeat_count:__be32,pub rsvd:u16,pub num_entries:__be16 } #[repr(C)] pub struct mlx5_wqe_flow_update_ctrl_seg { pub flow_idx_update:__be32,pub dest_handle:__be32,pub reserved0:[u8;40] } #[repr(C)] pub struct mlx5_wqe_header_modify_argument_update_seg { pub argument_list:[u8;64] }
#[repr(C)] pub struct mlx5_core_qp { pub common:mlx5_rsc_common,pub event:Option<unsafe extern "C" fn(*mut mlx5_core_qp,i32)>,pub qpn:i32,pub dbg:*mut mlx5_rsc_debug,pub pid:i32,pub uid:u16 } #[repr(C)] pub struct mlx5_core_dct { pub mqp:mlx5_core_qp,pub drained:completion }
extern "C" { pub fn mlx5_debug_qp_add(dev:*mut mlx5_core_dev, qp:*mut mlx5_core_qp)->i32; pub fn mlx5_debug_qp_remove(dev:*mut mlx5_core_dev, qp:*mut mlx5_core_qp); }
pub const MLX5_QP_TYPE_STR_INVALID:&str="Invalid transport type";
pub unsafe fn mlx5_qp_type_str(type_:i32)->*const std::ffi::c_char { match type_ as u32 { MLX5_QP_ST_RC=>b"RC\0".as_ptr() as _, MLX5_QP_ST_UC=>b"C\0".as_ptr() as _, MLX5_QP_ST_UD=>b"UD\0".as_ptr() as _, MLX5_QP_ST_XRC=>b"XRC\0".as_ptr() as _, MLX5_QP_ST_MLX=>b"MLX\0".as_ptr() as _, MLX5_QP_ST_QP0=>b"QP0\0".as_ptr() as _, MLX5_QP_ST_QP1=>b"QP1\0".as_ptr() as _, MLX5_QP_ST_RAW_ETHERTYPE=>b"RAW_ETHERTYPE\0".as_ptr() as _, MLX5_QP_ST_RAW_IPV6=>b"RAW_IPV6\0".as_ptr() as _, MLX5_QP_ST_SNIFFER=>b"SNIFFER\0".as_ptr() as _, MLX5_QP_ST_SYNC_UMR=>b"SYNC_UMR\0".as_ptr() as _, MLX5_QP_ST_PTP_1588=>b"PTP_1588\0".as_ptr() as _, MLX5_QP_ST_REG_UMR=>b"REG_UMR\0".as_ptr() as _, _=>b"Invalid transport type\0".as_ptr() as _ } }
pub unsafe fn mlx5_qp_state_str(state:i32)->*const std::ffi::c_char { match state { 0=>b"RST\0",1=>b"INIT\0",2=>b"RTR\0",3=>b"RTS\0",4=>b"SQER\0",5=>b"SQD\0",6=>b"ERR\0",7=>b"SQ_DRAINING\0",9=>b"SUSPENDED\0",_=>b"Invalid QP state\0"}.as_ptr() as _ }
pub unsafe fn mlx5_get_qp_default_ts(dev:*mut mlx5_core_dev)->i32 { let supported_ts_cap:u8=if mlx5_get_roce_state(dev)!=0 { MLX5_CAP_ROCE(dev, qp_ts_format) } else { MLX5_CAP_GEN(dev, sq_ts_format) }; if supported_ts_cap!=0 { MLX5_TIMESTAMP_FORMAT_DEFAULT } else { MLX5_TIMESTAMP_FORMAT_FREE_RUNNING } }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
