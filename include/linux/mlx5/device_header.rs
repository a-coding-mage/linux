/* Direct low-level Rust translation of linux/mlx5/device.h. */

// Dependencies supplied by the surrounding kernel translation.
use core::mem::size_of;

pub const MLX5_SET_HOST_ENDIANNESS: u32 = 0;

#[macro_export]
macro_rules! __mlx5_bit_sz { ($typ:ty, $fld:tt) => { size_of::<$typ>() }; }
#[macro_export]
macro_rules! MLX5_FLD_SZ_BYTES { ($typ:ty, $fld:tt) => { __mlx5_bit_sz!($typ, $fld) / 8 }; }
#[macro_export]
macro_rules! MLX5_ST_SZ_BYTES { ($typ:ty) => { size_of::<$typ>() / 8 }; }
#[macro_export]
macro_rules! MLX5_ST_SZ_DW { ($typ:ty) => { size_of::<$typ>() / 32 }; }
#[macro_export]
macro_rules! MLX5_ST_SZ_QW { ($typ:ty) => { size_of::<$typ>() / 64 }; }
#[macro_export]
macro_rules! MLX5_UN_SZ_BYTES { ($typ:ty) => { size_of::<$typ>() / 8 }; }
#[macro_export]
macro_rules! MLX5_UN_SZ_DW { ($typ:ty) => { size_of::<$typ>() / 32 }; }
#[macro_export]
macro_rules! MLX5_BYTE_OFF { ($typ:ty, $fld:tt) => { 0usize }; }
#[macro_export]
macro_rules! MLX5_ADDR_OF { ($typ:ty, $p:expr, $fld:tt) => { ($p as *mut u8).wrapping_add(MLX5_BYTE_OFF!($typ, $fld)) as *mut core::ffi::c_void }; }

pub const MLX5_MAX_COMMANDS: u32 = 32;
pub const MLX5_CMD_DATA_BLOCK_SIZE: usize = 512;
pub const MLX5_PCI_CMD_XPORT: u32 = 7;
pub const MLX5_MKEY_BSF_OCTO_SIZE: u32 = 4;
pub const MLX5_MAX_PSVS: u32 = 4;
pub const MLX5_EXTENDED_UD_AV: u32 = 0x80000000;
pub const MLX5_CQ_STATE_ARMED: u32 = 9;
pub const MLX5_CQ_STATE_ALWAYS_ARMED: u32 = 0xb;
pub const MLX5_CQ_STATE_FIRED: u32 = 0xa;
pub const MLX5_STAT_RATE_OFFSET: u32 = 5;
pub const MLX5_INLINE_SEG: u32 = 0x80000000;
pub const MLX5_HW_START_PADDING: u32 = MLX5_INLINE_SEG;
pub const MLX5_MIN_PKEY_TABLE_SIZE: u32 = 128;
pub const MLX5_MAX_LOG_PKEY_TABLE: u32 = 5;
pub const MLX5_MKEY_INBOX_PG_ACCESS: u32 = 1 << 31;
pub const MLX5_PFAULT_SUBTYPE_WQE: u32 = 0;
pub const MLX5_PFAULT_SUBTYPE_RDMA: u32 = 1;
pub const MLX5_PFAULT_SUBTYPE_MEMORY: u32 = 2;
pub const MLX5_PERM_LOCAL_READ: u32 = 1 << 2;
pub const MLX5_PERM_LOCAL_WRITE: u32 = 1 << 3;
pub const MLX5_PERM_REMOTE_READ: u32 = 1 << 4;
pub const MLX5_PERM_REMOTE_WRITE: u32 = 1 << 5;
pub const MLX5_PERM_ATOMIC: u32 = 1 << 6;
pub const MLX5_PERM_UMR_EN: u32 = 1 << 7;
pub const MLX5_PCIE_CTRL_SMALL_FENCE: u32 = 1;
pub const MLX5_PCIE_CTRL_RELAXED_ORDERING: u32 = 1 << 2;
pub const MLX5_PCIE_CTRL_NO_SNOOP: u32 = 1 << 3;
pub const MLX5_PCIE_CTRL_TLP_PROCE_EN: u32 = 1 << 6;
pub const MLX5_PCIE_CTRL_TPH_MASK: u32 = 3 << 4;
pub const MLX5_EN_RD: u64 = 1;
pub const MLX5_EN_WR: u64 = 2;
pub const MLX5_ADAPTER_PAGE_SHIFT: u32 = 12;
pub const MLX5_ADAPTER_PAGE_SIZE: usize = 1 << MLX5_ADAPTER_PAGE_SHIFT;

#[repr(u32)] pub enum Mlx5InlineModes { None, L2, Ip, TcpUdp }
#[repr(u32)] pub enum WqePageFaultType { Rmp, ReqSendOrWrite, Resp, ReqReadOrAtomic }
#[repr(u32)] pub enum Mlx5Event { NotifyAny=0, Comp=0, PathMig=1, CommEst=2, SqDrained=3, CqError=4, WqCatasError=5, PathMigFailed=7, InternalError=8, PortChange=9, Cmd=0xa, PageRequest=0xb, PageFault=0xc, NicVportChange=0xd, EswFunctionsChanged=0xe, VhcaStateChange=0xf, WqInvalReqError=0x10, WqAccessError=0x11, SrqCatasError=0x12, SrqLastWqe=0x13, SrqRqLimit=0x14, GpioEvent=0x15, PortModuleEvent=0x16, TempWarnEvent=0x17, XrqError=0x18, RemoteConfig=0x19, DbBfCongestion=0x1a, StallEvent=0x1b, DctDrained=0x1c, DctKeyViolation=0x1d, FpgaError=0x20, FpgaQpError=0x21, GeneralEvent=0x22, MonitorCounter=0x24, PpsEvent=0x25, DeviceTracer=0x26, ObjectChange=0x27, Max=0x100 }
#[repr(u32)] pub enum Mlx5DriverEvent { Trap=0, UplinkNetdev, MacsecSaAdded, MacsecSaDeleted, SfPeerDevlink, AffiliationDone, AffiliationRemoved, ActiveBackupLagChangeLowerstate }

#[repr(C)] pub struct Mlx5WqeTlsStaticParamsSeg { pub ctx: [u8; 0] }
#[repr(C)] pub struct Mlx5WqeTlsProgressParamsSeg { pub tis_tir_num: u32, pub ctx: [u8; 0] }
#[repr(C)] pub struct Mlx5OdpCaps { pub reserved: [u8;16], pub rc_odp_caps:u32, pub uc_odp_caps:u32, pub ud_odp_caps:u32, pub reserved2:[u8;0xe4] }
#[repr(C)] pub struct Mlx5CmdLayout { pub type_:u8, pub rsvd0:[u8;3], pub inlen:u32, pub in_ptr:u64, pub in_: [u32;4], pub out:[u32;4], pub out_ptr:u64, pub outlen:u32, pub token:u8, pub sig:u8, pub rsvd1:u8, pub status_own:u8 }
#[repr(C)] pub struct HealthBuffer { pub assert_var:[u32;6], pub rsvd0:[u32;2], pub assert_exit_ptr:u32, pub assert_callra:u32, pub rsvd1:u32, pub time:u32, pub fw_ver:u32, pub hw_id:u32, pub rfr_severity:u8, pub rsvd2:[u8;3], pub irisc_index:u8, pub synd:u8, pub ext_synd:u16 }
#[repr(C)] pub struct Mlx5InitSeg { pub fw_rev:u32, pub cmdif_rev_fw_sub:u32, pub rsvd0:[u32;2], pub cmdq_addr_h:u32, pub cmdq_addr_l_sz:u32, pub cmd_dbell:u32, pub rsvd1:[u32;120], pub initializing:u32, pub health:HealthBuffer, pub rsvd2:[u32;878], pub cmd_exec_to:u32, pub cmd_q_init_to:u32, pub internal_timer_h:u32, pub internal_timer_l:u32, pub rsvd3:[u32;2], pub health_counter:u32, pub rsvd4:[u32;11], pub real_time_h:u32, pub real_time_l:u32, pub rsvd5:[u32;1006], pub ieee1588_clk:u64, pub ieee1588_clk_type:u32, pub clr_intx:u32 }

#[repr(C)] pub struct Mlx5EqeComp { pub reserved:[u32;6], pub cqn:u32 }
#[repr(C)] pub struct Mlx5EqeQpSrq { pub reserved1:[u32;5], pub type_:u8, pub reserved2:[u8;3], pub qp_srq_n:u32 }
#[repr(C)] pub struct Mlx5EqeCqErr { pub cqn:u32, pub reserved1:[u8;7], pub syndrome:u8 }
#[repr(C)] pub struct Mlx5EqeXrqErr { pub reserved1:[u32;5], pub type_xrqn:u32, pub reserved2:u32 }
#[repr(C)] pub struct Mlx5EqePortState { pub reserved0:[u8;8], pub port:u8 }
#[repr(C)] pub struct Mlx5EqeGpio { pub reserved0:[u32;2], pub gpio_event:u64 }
#[repr(C)] pub struct Mlx5EqeCongestion { pub type_:u8, pub rsvd0:u8, pub congestion_level:u8 }
#[repr(C)] pub struct Mlx5EqeStallVl { pub rsvd0:[u8;3], pub port_vl:u8 }
#[repr(C)] pub struct Mlx5EqeCmd { pub vector:u32, pub rsvd:[u32;6] }
#[repr(C)] pub struct Mlx5EqePageReq { pub ec_function:u16, pub func_id:u16, pub num_pages:u32, pub rsvd1:[u32;5] }

#[repr(C)] pub struct Mlx5Cqe64 { pub tls_outer_l3_tunneled:u8, pub rsvd0:u8, pub wqe_id:u16, pub lro_tcppsh_abort_dupack:u8, pub lro_min_ttl:u8, pub lro_tcp_win:u16, pub lro_ack_seq_num:u32, pub rss_hash_result:u32, pub rss_hash_type:u8, pub ml_path:u8, pub rsvd20:[u8;2], pub check_sum:u16, pub slid:u16, pub flags_rqpn:u32, pub hds_ip_ext:u8, pub l4_l3_hdr_type:u8, pub vlan_info:u16, pub srqn:u32, pub immediate:u32, pub rsvd40:[u8;4], pub byte_cnt:u32, pub timestamp_h:u32, pub timestamp_l:u32, pub sop_drop_qpn:u32, pub wqe_counter:u16, pub signature:u8, pub op_own:u8 }
#[repr(C)] pub struct Mlx5MiniCqe8 { pub rx_hash_result:u32, pub byte_cnt:u32 }
#[repr(C)] pub struct Mlx5SigErrCqe { pub rsvd0:[u8;16], pub expected_trans_sig:u32, pub actual_trans_sig:u32, pub expected_reftag:u32, pub actual_reftag:u32, pub syndrome:u16, pub rsvd22:[u8;2], pub mkey:u32, pub err_offset:u64, pub rsvd30:[u8;8], pub qpn:u32, pub rsvd38:[u8;2], pub signature:u8, pub op_own:u8 }
#[repr(C)] pub struct Mlx5WqeSrqNextSeg { pub rsvd0:[u8;2], pub next_wqe_index:u16, pub signature:u8, pub rsvd1:[u8;11] }
#[repr(C)] pub struct Mlx5Cqe128 { pub inl_grh:[u8;64], pub cqe64:Mlx5Cqe64 }
#[repr(C)] pub struct Mlx5MkeySeg { pub status:u8, pub pcie_control:u8, pub flags:u8, pub version:u8, pub qpn_mkey7_0:u32, pub rsvd1:[u8;4], pub flags_pd:u32, pub start_addr:u64, pub len:u64, pub bsfs_octo_size:u32, pub rsvd2:[u8;16], pub xlt_oct_size:u32, pub rsvd3:[u8;3], pub log2_page_size:u8, pub rsvd4:[u8;4] }

pub const MLX5_MINI_CQE_ARRAY_SIZE: usize = 8;
#[inline] pub unsafe fn mlx5_get_cqe_format(cqe:*const Mlx5Cqe64)->u8 { ((*cqe).op_own >> 2) & 3 }
#[inline] pub unsafe fn get_cqe_opcode(cqe:*const Mlx5Cqe64)->u8 { (*cqe).op_own >> 4 }
#[inline] pub unsafe fn get_cqe_enhanced_num_mini_cqes(cqe:*const Mlx5Cqe64)->u8 { get_cqe_opcode(cqe).wrapping_add(1) }
#[inline] pub unsafe fn get_cqe_lro_tcppsh(cqe:*const Mlx5Cqe64)->u8 { ((*cqe).lro_tcppsh_abort_dupack >> 6)&1 }
#[inline] pub unsafe fn get_cqe_l4_hdr_type(cqe:*const Mlx5Cqe64)->u8 { ((*cqe).l4_l3_hdr_type >> 4)&7 }
#[inline] pub unsafe fn cqe_is_tunneled(cqe:*const Mlx5Cqe64)->bool { (*cqe).tls_outer_l3_tunneled & 1 != 0 }
#[inline] pub unsafe fn get_cqe_tls_offload(cqe:*const Mlx5Cqe64)->u8 { ((*cqe).tls_outer_l3_tunneled >> 3)&3 }
#[inline] pub unsafe fn cqe_has_vlan(cqe:*const Mlx5Cqe64)->bool { (*cqe).l4_l3_hdr_type & 1 != 0 }
#[inline] pub unsafe fn get_cqe_ts(cqe:*const Mlx5Cqe64)->u64 { u32::from_be((*cqe).timestamp_l) as u64 | ((u32::from_be((*cqe).timestamp_h) as u64)<<32) }
#[inline] pub unsafe fn get_cqe_flow_tag(cqe:*const Mlx5Cqe64)->u16 { (u32::from_be((*cqe).sop_drop_qpn)&0xfff) as u16 }
#[inline] pub unsafe fn get_cqe_lro_num_seg(cqe:*const Mlx5Cqe64)->u8 { (u32::from_be((*cqe).srqn)>>24) as u8 }

pub const MLX5_CMD_STAT_OK:u32=0; pub const MLX5_CMD_STAT_INT_ERR:u32=1; pub const MLX5_CMD_STAT_BAD_OP_ERR:u32=2; pub const MLX5_CMD_STAT_BAD_PARAM_ERR:u32=3; pub const MLX5_CMD_STAT_BAD_SYS_STATE_ERR:u32=4; pub const MLX5_CMD_STAT_BAD_RES_ERR:u32=5; pub const MLX5_CMD_STAT_RES_BUSY:u32=6; pub const MLX5_CMD_STAT_NOT_READY:u32=7; pub const MLX5_CMD_STAT_LIM_ERR:u32=8; pub const MLX5_CMD_STAT_BAD_RES_STATE_ERR:u32=9; pub const MLX5_CMD_STAT_IX_ERR:u32=0xa; pub const MLX5_CMD_STAT_NO_RES_ERR:u32=0xf;
pub const MLX5_MIN_PKEY_TABLE_SIZE_U16:u16=128;
#[inline] pub fn mlx5_to_sw_pkey_sz(pkey_sz:i32)->u16 { if pkey_sz > 5 { 0 } else { (128u16).wrapping_shl(pkey_sz as u32) } }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
