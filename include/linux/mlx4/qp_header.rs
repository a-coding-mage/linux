/* Translated from qp.h. */

pub const MLX4_INVALID_LKEY: u32 = 0x100;

#[repr(C)]
#[derive(Copy, Clone)]
pub enum mlx4_qp_optpar { MLX4_QP_OPTPAR_ALT_ADDR_PATH = 1 << 0, MLX4_QP_OPTPAR_RRE = 1 << 1, MLX4_QP_OPTPAR_RAE = 1 << 2, MLX4_QP_OPTPAR_RWE = 1 << 3, MLX4_QP_OPTPAR_PKEY_INDEX = 1 << 4, MLX4_QP_OPTPAR_Q_KEY = 1 << 5, MLX4_QP_OPTPAR_RNR_TIMEOUT = 1 << 6, MLX4_QP_OPTPAR_PRIMARY_ADDR_PATH = 1 << 7, MLX4_QP_OPTPAR_SRA_MAX = 1 << 8, MLX4_QP_OPTPAR_RRA_MAX = 1 << 9, MLX4_QP_OPTPAR_PM_STATE = 1 << 10, MLX4_QP_OPTPAR_RETRY_COUNT = 1 << 12, MLX4_QP_OPTPAR_RNR_RETRY = 1 << 13, MLX4_QP_OPTPAR_ACK_TIMEOUT = 1 << 14, MLX4_QP_OPTPAR_SCHED_QUEUE = 1 << 16, MLX4_QP_OPTPAR_COUNTER_INDEX = 1 << 20, MLX4_QP_OPTPAR_VLAN_STRIPPING = 1 << 21 }
#[repr(C)]
#[derive(Copy, Clone)]
pub enum mlx4_qp_state { MLX4_QP_STATE_RST=0, MLX4_QP_STATE_INIT=1, MLX4_QP_STATE_RTR=2, MLX4_QP_STATE_RTS=3, MLX4_QP_STATE_SQER=4, MLX4_QP_STATE_SQD=5, MLX4_QP_STATE_ERR=6, MLX4_QP_STATE_SQ_DRAINING=7, MLX4_QP_NUM_STATE }

pub const MLX4_QP_ST_RC:u32=0; pub const MLX4_QP_ST_UC:u32=1; pub const MLX4_QP_ST_RD:u32=2; pub const MLX4_QP_ST_UD:u32=3; pub const MLX4_QP_ST_XRC:u32=6; pub const MLX4_QP_ST_MLX:u32=7;
pub const MLX4_QP_PM_MIGRATED:u32=3; pub const MLX4_QP_PM_ARMED:u32=0; pub const MLX4_QP_PM_REARM:u32=1;

pub const MLX4_QP_BIT_SRE:u32=1<<15; pub const MLX4_QP_BIT_SWE:u32=1<<14; pub const MLX4_QP_BIT_SAE:u32=1<<13; pub const MLX4_QP_BIT_RRE:u32=1<<15; pub const MLX4_QP_BIT_RWE:u32=1<<14; pub const MLX4_QP_BIT_RAE:u32=1<<13; pub const MLX4_QP_BIT_FPP:u32=1<<3; pub const MLX4_QP_BIT_RIC:u32=1<<4;
pub const MLX4_RSS_HASH_XOR:u32=0; pub const MLX4_RSS_HASH_TOP:u32=1; pub const MLX4_RSS_UDP_IPV6:u32=1; pub const MLX4_RSS_UDP_IPV4:u32=1<<1; pub const MLX4_RSS_TCP_IPV6:u32=1<<2; pub const MLX4_RSS_IPV6:u32=1<<3; pub const MLX4_RSS_TCP_IPV4:u32=1<<4; pub const MLX4_RSS_IPV4:u32=1<<5; pub const MLX4_RSS_BY_OUTER_HEADERS:u32=0; pub const MLX4_RSS_BY_INNER_HEADERS:u32=2<<6; pub const MLX4_RSS_BY_INNER_HEADERS_IPONLY:u32=3<<6; pub const MLX4_RSS_OFFSET_IN_QPC_PRI_PATH:u32=0x24; pub const MLX4_RSS_QPC_FLAG_OFFSET:u32=13;
pub const MLX4_EN_RSS_KEY_SIZE:usize=40;

#[repr(C)] pub struct mlx4_rss_context { pub base_qpn: u32, pub default_qpn:u32, pub reserved:u16, pub hash_fn:u8, pub flags:u8, pub rss_key:[u32;10], pub base_qpn_udp:u32 }
#[repr(C)] pub union mlx4_qp_path_vlan { pub vlan_control:u8, pub control:u8 }
#[repr(C)] pub struct mlx4_qp_path { pub fl:u8, pub vlan_control: mlx4_qp_path_vlan, pub disable_pkey_check:u8, pub pkey_index:u8, pub counter_index:u8, pub grh_mylmc:u8, pub rlid:u16, pub ackto:u8, pub mgid_index:u8, pub static_rate:u8, pub hop_limit:u8, pub tclass_flowlabel:u32, pub rgid:[u8;16], pub sched_queue:u8, pub vlan_index:u8, pub feup:u8, pub fvl_rx:u8, pub reserved4:[u8;2], pub dmac:[u8;6] }
pub const MLX4_FL_CV:u8=1<<6; pub const MLX4_FL_SV:u8=1<<5; pub const MLX4_FL_ETH_HIDE_CQE_VLAN:u8=1<<2; pub const MLX4_FL_ETH_SRC_CHECK_MC_LB:u8=1<<1; pub const MLX4_FL_ETH_SRC_CHECK_UC_LB:u8=1; pub const MLX4_CTRL_ETH_SRC_CHECK_IF_COUNTER:u8=1<<7;
pub const MLX4_VLAN_CTRL_ETH_TX_BLOCK_TAGGED:u8=1<<6; pub const MLX4_VLAN_CTRL_ETH_TX_BLOCK_PRIO_TAGGED:u8=1<<5; pub const MLX4_VLAN_CTRL_ETH_TX_BLOCK_UNTAGGED:u8=1<<4; pub const MLX4_VLAN_CTRL_ETH_RX_BLOCK_TAGGED:u8=1<<2; pub const MLX4_VLAN_CTRL_ETH_RX_BLOCK_PRIO_TAGGED:u8=1<<1; pub const MLX4_VLAN_CTRL_ETH_RX_BLOCK_UNTAGGED:u8=1;
pub const MLX4_FEUP_FORCE_ETH_UP:u8=1<<6; pub const MLX4_FSM_FORCE_ETH_SRC_MAC:u8=1<<5; pub const MLX4_FVL_FORCE_ETH_VLAN:u8=1<<3; pub const MLX4_FVL_RX_FORCE_ETH_VLAN:u8=1;

#[repr(C)] pub struct mlx4_qp_context { pub flags:u32,pub pd:u32,pub mtu_msgmax:u8,pub rq_size_stride:u8,pub sq_size_stride:u8,pub rlkey_roce_mode:u8,pub usr_page:u32,pub local_qpn:u32,pub remote_qpn:u32,pub pri_path:mlx4_qp_path,pub alt_path:mlx4_qp_path,pub params1:u32,pub reserved1:u32,pub next_send_psn:u32,pub cqn_send:u32,pub roce_entropy:u16,pub reserved2:[u16;3],pub last_acked_psn:u32,pub ssn:u32,pub params2:u32,pub rnr_nextrecvpsn:u32,pub xrcd:u32,pub cqn_recv:u32,pub db_rec_addr:u64,pub qkey:u32,pub srqn:u32,pub msn:u32,pub rq_wqe_counter:u16,pub sq_wqe_counter:u16,pub reserved3:u32,pub rate_limit_params:u16,pub reserved4:u8,pub qos_vport:u8,pub param3:u32,pub nummmcpeers_basemkey:u32,pub log_page_size:u8,pub reserved5:[u8;2],pub mtt_base_addr_h:u8,pub mtt_base_addr_l:u32,pub reserved6:[u32;10] }
#[repr(C)] pub struct mlx4_update_qp_context { pub qp_mask:u64,pub primary_addr_path_mask:u64,pub secondary_addr_path_mask:u64,pub reserved1:u64,pub qp_context:mlx4_qp_context,pub reserved2:[u64;58] }

pub const MLX4_UPD_QP_MASK_PM_STATE:u32=32; pub const MLX4_UPD_QP_MASK_VSD:u32=33; pub const MLX4_UPD_QP_MASK_QOS_VPP:u32=34; pub const MLX4_UPD_QP_MASK_RATE_LIMIT:u32=35;
pub const MLX4_UPD_QP_PATH_MASK_PKEY_INDEX:u32=32; pub const MLX4_UPD_QP_PATH_MASK_FSM:u32=33; pub const MLX4_UPD_QP_PATH_MASK_MAC_INDEX:u32=34; pub const MLX4_UPD_QP_PATH_MASK_FVL:u32=35; pub const MLX4_UPD_QP_PATH_MASK_CV:u32=36; pub const MLX4_UPD_QP_PATH_MASK_VLAN_INDEX:u32=37; pub const MLX4_UPD_QP_PATH_MASK_ETH_HIDE_CQE_VLAN:u32=38; pub const MLX4_UPD_QP_PATH_MASK_ETH_TX_BLOCK_UNTAGGED:u32=39; pub const MLX4_UPD_QP_PATH_MASK_ETH_TX_BLOCK_1P:u32=40; pub const MLX4_UPD_QP_PATH_MASK_ETH_TX_BLOCK_TAGGED:u32=41; pub const MLX4_UPD_QP_PATH_MASK_ETH_RX_BLOCK_UNTAGGED:u32=42; pub const MLX4_UPD_QP_PATH_MASK_ETH_RX_BLOCK_1P:u32=43; pub const MLX4_UPD_QP_PATH_MASK_ETH_RX_BLOCK_TAGGED:u32=44; pub const MLX4_UPD_QP_PATH_MASK_FEUP:u32=45; pub const MLX4_UPD_QP_PATH_MASK_SCHED_QUEUE:u32=46; pub const MLX4_UPD_QP_PATH_MASK_IF_COUNTER_INDEX:u32=47; pub const MLX4_UPD_QP_PATH_MASK_FVL_RX:u32=48; pub const MLX4_UPD_QP_PATH_MASK_ETH_SRC_CHECK_UC_LB:u32=50; pub const MLX4_UPD_QP_PATH_MASK_ETH_SRC_CHECK_MC_LB:u32=51; pub const MLX4_UPD_QP_PATH_MASK_SV:u32=54;
pub const MLX4_STRIP_VLAN:u32=1<<30; pub const MLX4_WQE_CTRL_NEC:u32=1<<29; pub const MLX4_WQE_CTRL_IIP:u32=1<<28; pub const MLX4_WQE_CTRL_ILP:u32=1<<27; pub const MLX4_WQE_CTRL_FENCE:u32=1<<6; pub const MLX4_WQE_CTRL_CQ_UPDATE:u32=3<<2; pub const MLX4_WQE_CTRL_SOLICITED:u32=1<<1; pub const MLX4_WQE_CTRL_IP_CSUM:u32=1<<4; pub const MLX4_WQE_CTRL_TCP_UDP_CSUM:u32=1<<5; pub const MLX4_WQE_CTRL_INS_CVLAN:u32=1<<6; pub const MLX4_WQE_CTRL_INS_SVLAN:u32=1<<7; pub const MLX4_WQE_CTRL_STRONG_ORDER:u32=1<<7; pub const MLX4_WQE_CTRL_FORCE_LOOPBACK:u32=1;
pub const MLX4_WQE_FMR_PERM_LOCAL_READ:u32=1<<27; pub const MLX4_WQE_FMR_PERM_LOCAL_WRITE:u32=1<<28; pub const MLX4_WQE_FMR_AND_BIND_PERM_REMOTE_READ:u32=1<<29; pub const MLX4_WQE_FMR_AND_BIND_PERM_REMOTE_WRITE:u32=1<<30; pub const MLX4_WQE_FMR_AND_BIND_PERM_ATOMIC:u32=1<<31;
#[repr(C)] pub union mlx4_wqe_qpn_vlan { pub vlan_tag:u16, pub bf_qpn:u32, pub fields: mlx4_wqe_qpn_vlan_fields }
#[repr(C)] pub struct mlx4_wqe_qpn_vlan_fields { pub vlan_tag:u16,pub ins_vlan:u8,pub fence_size:u8 }
#[repr(C)] pub union mlx4_wqe_ctrl_flags { pub srcrb_flags:u32,pub srcrb_flags16:[u16;2] }
#[repr(C)] pub struct mlx4_wqe_ctrl_seg { pub owner_opcode:u32,pub qpn_vlan:mlx4_wqe_qpn_vlan,pub srcrb_flags:mlx4_wqe_ctrl_flags,pub imm:u32 }
pub const MLX4_WQE_MLX_VL15:u32=1<<17; pub const MLX4_WQE_MLX_SLR:u32=1<<16;
#[repr(C)] pub struct mlx4_wqe_mlx_seg { pub owner:u8,pub reserved1:[u8;2],pub opcode:u8,pub sched_prio:u16,pub reserved2:u8,pub size:u8,pub flags:u32,pub rlid:u16,pub reserved3:u16 }
#[repr(C)] pub struct mlx4_wqe_datagram_seg { pub av:[u32;8],pub dqpn:u32,pub qkey:u32,pub vlan:u16,pub mac:[u8;6] }
#[repr(C)] pub struct mlx4_wqe_lso_seg { pub mss_hdr_size:u32,pub header:[u32;0] }
#[repr(C)] pub enum mlx4_wqe_bind_seg_flags2 { MLX4_WQE_BIND_ZERO_BASED=1<<30, MLX4_WQE_BIND_TYPE_2=1<<31 }
#[repr(C)] pub struct mlx4_wqe_bind_seg { pub flags1:u32,pub flags2:u32,pub new_rkey:u32,pub lkey:u32,pub addr:u64,pub length:u64 }
#[repr(C)] pub struct mlx4_wqe_fmr_seg { pub flags:u32,pub mem_key:u32,pub buf_list:u64,pub start_addr:u64,pub reg_len:u64,pub offset:u32,pub page_size:u32,pub reserved:[u32;2] }
#[repr(C)] pub struct mlx4_wqe_fmr_ext_seg { pub flags:u8,pub reserved:u8,pub app_mask:u16,pub wire_app_tag:u16,pub mem_app_tag:u16,pub wire_ref_tag_base:u32,pub mem_ref_tag_base:u32 }
#[repr(C)] pub struct mlx4_wqe_local_inval_seg { pub reserved1:u64,pub mem_key:u32,pub reserved2:u32,pub reserved3:[u64;2] }
#[repr(C)] pub struct mlx4_wqe_raddr_seg { pub raddr:u64,pub rkey:u32,pub reserved:u32 }
#[repr(C)] pub struct mlx4_wqe_atomic_seg { pub swap_add:u64,pub compare:u64 }
#[repr(C)] pub struct mlx4_wqe_masked_atomic_seg { pub swap_add:u64,pub compare:u64,pub swap_add_mask:u64,pub compare_mask:u64 }
#[repr(C)] pub struct mlx4_wqe_data_seg { pub byte_count:u32,pub lkey:u32,pub addr:u64 }
pub const MLX4_INLINE_ALIGN:u32=64; pub const MLX4_INLINE_SEG:u32=1<<31;
#[repr(C)] pub struct mlx4_wqe_inline_seg { pub byte_count:u32,pub data:[u8;0] }
#[repr(C)] pub enum mlx4_update_qp_attr { MLX4_UPDATE_QP_SMAC=1,MLX4_UPDATE_QP_VSD=2,MLX4_UPDATE_QP_RATE_LIMIT=4,MLX4_UPDATE_QP_QOS_VPORT=8,MLX4_UPDATE_QP_ETH_SRC_CHECK_MC_LB=16,MLX4_UPDATE_QP_SUPPORTED_ATTRS=31 }
#[repr(C)] pub enum mlx4_update_qp_params_flags { MLX4_UPDATE_QP_PARAMS_FLAGS_ETH_CHECK_MC_LB=1,MLX4_UPDATE_QP_PARAMS_FLAGS_VSD_ENABLE=2 }
#[repr(C)] pub struct mlx4_update_qp_params { pub smac_index:u8,pub qos_vport:u8,pub flags:u32,pub rate_unit:u16,pub rate_val:u16 }

extern "C" { pub fn mlx4_qp_lookup(dev:*mut mlx4_dev,qpn:u32)->*mut mlx4_qp; pub fn mlx4_update_qp(dev:*mut mlx4_dev,qpn:u32,attr:mlx4_update_qp_attr,params:*mut mlx4_update_qp_params)->i32; pub fn mlx4_qp_modify(dev:*mut mlx4_dev,mtt:*mut mlx4_mtt,cur_state:mlx4_qp_state,new_state:mlx4_qp_state,context:*mut mlx4_qp_context,optpar:mlx4_qp_optpar,sqd_event:i32,qp:*mut mlx4_qp)->i32; pub fn mlx4_qp_query(dev:*mut mlx4_dev,qp:*mut mlx4_qp,context:*mut mlx4_qp_context)->i32; pub fn mlx4_qp_to_ready(dev:*mut mlx4_dev,mtt:*mut mlx4_mtt,context:*mut mlx4_qp_context,qp:*mut mlx4_qp,qp_state:*mut mlx4_qp_state)->i32; pub fn mlx4_qp_remove(dev:*mut mlx4_dev,qp:*mut mlx4_qp); pub fn mlx4_qp_roce_entropy(dev:*mut mlx4_dev,qpn:u32)->u16; pub fn mlx4_put_qp(qp:*mut mlx4_qp); }
#[repr(C)] pub struct mlx4_dev; #[repr(C)] pub struct mlx4_mtt; #[repr(C)] pub struct mlx4_qp;
#[inline] pub unsafe fn folded_qp(q:u32)->u16 { (((q & 0xff) ^ ((q & 0xff0000)>>16)) | (q & 0xff00)) as u16 }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
