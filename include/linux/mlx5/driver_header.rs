/* Faithful Rust translation of linux/mlx5/driver.h. Kernel dependencies are external. */

pub const MLX5_ADEV_NAME: &[u8] = b"mlx5_core\0";
pub const MLX5_IRQ_EQ_CTRL: u8 = u8::MAX;
pub const MLX5_BOARD_ID_LEN: usize = 64;
pub const MLX5_CMD_WQ_MAX_NAME: usize = 32;
pub const MLX5_MAX_PORTS: usize = 8;
pub const MLX5_24BIT_MASK: u32 = (1 << 24) - 1;
pub const MLX5_MAX_RESERVED_GIDS: usize = 8;
pub const MLX5_DEFAULT_NUM_DOORBELLS: usize = 8;
pub const MLX5_COMP_EQ_SIZE: usize = 1024;
pub const MLX5_MAX_NUM_TC: usize = 8;

#[repr(C)] pub enum mlx5_sqp_t { MLX5_SQP_SMI=0, MLX5_SQP_GSI, MLX5_SQP_IEEE_1588, MLX5_SQP_SNIFFER, MLX5_SQP_SYNC_UMR }
#[repr(C)] pub enum mlx5_qpts_trust_state { MLX5_QPTS_TRUST_PCP=1, MLX5_QPTS_TRUST_DSCP }
#[repr(C)] pub enum mlx5_dcbx_oper_mode { MLX5E_DCBX_PARAM_VER_OPER_HOST=0, MLX5E_DCBX_PARAM_VER_OPER_AUTO=3 }
#[repr(C)] pub enum dbg_rsc_type { MLX5_DBG_RSC_QP, MLX5_DBG_RSC_EQ, MLX5_DBG_RSC_CQ }
#[repr(C)] pub enum port_state_policy { MLX5_POLICY_DOWN=0, MLX5_POLICY_UP, MLX5_POLICY_FOLLOW, MLX5_POLICY_INVALID=0xffff_ffff }
#[repr(C)] pub enum mlx5_coredev_type { MLX5_COREDEV_PF, MLX5_COREDEV_VF, MLX5_COREDEV_SF }
#[repr(C)] pub enum mlx5_dev_event { MLX5_DEV_EVENT_SYS_ERROR=128, MLX5_DEV_EVENT_PORT_AFFINITY, MLX5_DEV_EVENT_MULTIPORT_ESW }
#[repr(C)] pub enum mlx5_port_status { MLX5_PORT_UP=1, MLX5_PORT_DOWN=2 }
#[repr(C)] pub enum mlx5_cmdif_state { MLX5_CMDIF_STATE_UNINITIALIZED, MLX5_CMDIF_STATE_UP, MLX5_CMDIF_STATE_DOWN }
#[repr(C)] pub enum mlx5_res_type { MLX5_RES_QP, MLX5_RES_RQ, MLX5_RES_SQ, MLX5_RES_SRQ=3, MLX5_RES_XSRQ, MLX5_RES_XRQ }
#[repr(C)] pub enum mlx5_func_type { MLX5_SELF, MLX5_VF, MLX5_SF, MLX5_HOST_PF, MLX5_SPF, MLX5_EC_VF, MLX5_FUNC_TYPE_NUM, MLX5_FUNC_TYPE_NONE=MLX5_FUNC_TYPE_NUM }
#[repr(C)] pub enum mlx5_page_mgt_mode { MLX5_PAGE_MGT_MODE_FUNC_ID, MLX5_PAGE_MGT_MODE_VHCA_ID }
#[repr(C)] pub enum mlx5_device_state { MLX5_DEVICE_STATE_UP=1, MLX5_DEVICE_STATE_INTERNAL_ERROR }
#[repr(C)] pub enum mlx5_interface_state { MLX5_INTERFACE_STATE_UP=1, MLX5_BREAK_FW_WAIT=2 }
#[repr(C)] pub enum mlx5_pci_status { MLX5_PCI_STATUS_DISABLED, MLX5_PCI_STATUS_ENABLED }
#[repr(C)] pub enum mlx5_wc_state { MLX5_WC_STATE_UNINITIALIZED, MLX5_WC_STATE_UNSUPPORTED, MLX5_WC_STATE_SUPPORTED }
#[repr(C)] pub enum mlx5_sw_icm_type { MLX5_SW_ICM_TYPE_STEERING, MLX5_SW_ICM_TYPE_HEADER_MODIFY, MLX5_SW_ICM_TYPE_HEADER_MODIFY_PATTERN, MLX5_SW_ICM_TYPE_SW_ENCAP }
#[repr(C)] pub enum phy_port_state { MLX5_AAA_111 }

#[repr(C)] pub struct mlx5_field_desc { pub i: i32 }
#[repr(C)] pub struct mlx5_rsc_debug { pub dev:*mut mlx5_core_dev, pub object:*mut core::ffi::c_void, pub typ:dbg_rsc_type, pub root:*mut dentry, pub fields:[mlx5_field_desc;0] }
#[repr(C)] pub struct mlx5_cmd_first { pub data:[u32;4] }
#[repr(C)] pub struct mlx5_cmd_msg { pub list:list_head, pub parent:*mut cmd_msg_cache, pub len:u32, pub first:mlx5_cmd_first, pub next:*mut mlx5_cmd_mailbox }
#[repr(C)] pub struct mlx5_cmd_debug { pub dbg_root:*mut dentry, pub in_msg:*mut core::ffi::c_void, pub out_msg:*mut core::ffi::c_void, pub status:u8, pub inlen:u16, pub outlen:u16 }
#[repr(C)] pub struct cmd_msg_cache { pub lock:spinlock_t, pub head:list_head, pub max_inbox_size:u32, pub num_ent:u32 }
#[repr(C)] pub struct mlx5_cmd_stats { pub sum:u64, pub n:u64, pub failed:u64, pub failed_mbox_status:u64, pub last_failed_errno:u32, pub last_failed_mbox_status:u8, pub last_failed_syndrome:u32, pub root:*mut dentry, pub lock:spinlock_t }
#[repr(C)] pub struct mlx5_cmd_mailbox { pub buf:*mut core::ffi::c_void, pub dma:dma_addr_t, pub next:*mut mlx5_cmd_mailbox }
#[repr(C)] pub struct mlx5_buf_list { pub buf:*mut core::ffi::c_void, pub map:dma_addr_t, pub frag_page:*mut mlx5_dma_pool_page }
#[repr(C)] pub struct mlx5_frag_buf { pub frags:*mut mlx5_buf_list, pub npages:i32, pub size:i32, pub page_shift:u8 }
#[repr(C)] pub struct mlx5_frag_buf_ctrl { pub frags:*mut mlx5_buf_list, pub sz_m1:u32, pub frag_sz_m1:u16, pub strides_offset:u16, pub log_sz:u8, pub log_stride:u8, pub log_frag_strides:u8 }
#[repr(C)] pub struct mlx5_core_psv { pub psv_idx:u32, pub psv:psv_layout }
#[repr(C)] pub struct psv_layout { pub pd:u32, pub syndrome:u16, pub reserved:u16, pub bg:u16, pub app_tag:u16, pub ref_tag:u32 }
#[repr(C)] pub struct mlx5_core_sig_ctx { pub psv_memory:mlx5_core_psv, pub psv_wire:mlx5_core_psv, pub err_item:ib_sig_err, pub sig_status_checked:bool, pub sig_err_exists:bool, pub sigerr_count:u32 }
#[repr(C)] pub struct mlx5_core_rsc_common { pub res:mlx5_res_type, pub refcount:refcount_t, pub free:completion, pub invalid:bool }
#[repr(C)] pub struct mlx5_rate_limit { pub rate:u32, pub max_burst_sz:u32, pub typical_pkt_sz:u16 }
#[repr(C)] pub struct mlx5_rl_entry { pub rl_raw:[u8;0], pub refcount:u64, pub index:u16, pub uid:u16, pub dedicated:bool }
#[repr(C)] pub struct mlx5_rl_table { pub rl_lock:mutex, pub max_size:u16, pub max_rate:u32, pub min_rate:u32, pub rl_entry:*mut mlx5_rl_entry, pub refcount:u64 }
#[repr(C)] pub struct mlx5_core_roce { pub ft:*mut mlx5_flow_table, pub fg:*mut mlx5_flow_group, pub allow_rule:*mut mlx5_flow_handle }
#[repr(C)] pub struct mlx5_adev { pub adev:auxiliary_device, pub mdev:*mut mlx5_core_dev, pub idx:i32 }
#[repr(C)] pub struct mlx5_vf_context { pub enabled:i32, pub port_guid:u64, pub node_guid:u64, pub port_guid_valid:bool, pub node_guid_valid:bool, pub policy:port_state_policy, pub notifier:blocking_notifier_head }
#[repr(C)] pub struct mlx5_core_sriov { pub vfs_ctx:*mut mlx5_vf_context, pub num_vfs:i32, pub max_vfs:u16, pub max_ec_vfs:u16 }
#[repr(C)] pub struct mlx5_td { pub list_lock:mutex, pub tirs_list:list_head, pub tdn:u32 }
#[repr(C)] pub struct mlx5_rsvd_gids { pub start:u32, pub count:u32, pub ida:ida }
#[repr(C)] pub struct mlx5_profile { pub mask:u64, pub log_max_qp:u8, pub num_cmd_caches:u8 }
#[repr(C)] pub struct mlx5_hca_cap { pub cur:*mut u32, pub max:*mut u32 }
#[repr(C)] pub struct mlx5_db { pub db:*mut u32, pub u:[u8;0], pub dma:dma_addr_t, pub index:i32 }

/* External kernel and mlx5 types used by the original header. */
#[repr(C)] pub struct mlx5_core_dev { pub device:*mut device, pub coredev_type:mlx5_coredev_type, pub pdev:*mut pci_dev, pub priv_:*mut mlx5_priv }
#[repr(C)] pub struct mlx5_priv { pub numa_node:i32, pub sriov:mlx5_core_sriov, pub rl_table:mlx5_rl_table }
#[repr(C)] pub struct mlx5_dma_pool_page;
#[repr(C)] pub struct dentry; #[repr(C)] pub struct list_head; #[repr(C)] pub struct spinlock_t; #[repr(C)] pub struct completion; #[repr(C)] pub struct refcount_t; #[repr(C)] pub struct mutex; #[repr(C)] pub struct auxiliary_device; #[repr(C)] pub struct ib_sig_err; #[repr(C)] pub struct mlx5_flow_table; #[repr(C)] pub struct mlx5_flow_group; #[repr(C)] pub struct mlx5_flow_handle; #[repr(C)] pub struct device; #[repr(C)] pub struct pci_dev; #[repr(C)] pub struct ida;
pub type dma_addr_t = u64;
extern "C" { pub static mut mlx5_debugfs_root:*mut dentry; }
extern "C" { pub fn mlx5_cmd_use_events(dev:*mut mlx5_core_dev); pub fn mlx5_cmd_use_polling(dev:*mut mlx5_core_dev); pub fn mlx5_cmd_allowed_opcode(dev:*mut mlx5_core_dev, opcode:u16); }
extern "C" { pub fn mlx5_cmd_exec(dev:*mut mlx5_core_dev, input:*mut core::ffi::c_void, in_size:i32, output:*mut core::ffi::c_void, out_size:i32)->i32; pub fn mlx5_db_alloc_node(dev:*mut mlx5_core_dev, db:*mut mlx5_db, node:i32)->i32; pub fn mlx5_db_free(dev:*mut mlx5_core_dev, db:*mut mlx5_db); }

#[inline] pub unsafe fn mlx5_base_mkey(key:u32)->u32 { key & 0xffffff00 }
#[inline] pub fn wq_get_byte_sz(log_sz:u8, log_stride:u8)->u32 { (1u32<<log_sz)<<log_stride }
#[inline] pub unsafe fn mlx5_mkey_to_idx(mkey:u32)->u32 { mkey>>8 }
#[inline] pub unsafe fn mlx5_idx_to_mkey(mkey_idx:u32)->u32 { mkey_idx<<8 }
#[inline] pub unsafe fn mlx5_mkey_variant(mkey:u32)->u8 { (mkey&0xff) as u8 }
#[inline] pub unsafe fn mlx5_db_alloc(dev:*mut mlx5_core_dev, db:*mut mlx5_db)->i32 { mlx5_db_alloc_node(dev,db,(*(*dev).priv_).numa_node) }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
