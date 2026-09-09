/* Faithful low-level Rust translation of linux/mlx4/device.h. */
#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, dead_code)]

use core::ffi::{c_char, c_int, c_void};
pub type u8 = core::primitive::u8; pub type u16 = core::primitive::u16;
pub type u32 = core::primitive::u32; pub type u64 = core::primitive::u64;
pub type __be16=u16; pub type __be32=u32; pub type __be64=u64; pub type dma_addr_t=u64;
pub type size_t=usize; pub const PAGE_SHIFT:u32=12; pub const PAGE_SIZE:usize=1<<PAGE_SHIFT;
pub const ETH_ALEN:usize=6; pub const EINVAL:c_int=22;
#[repr(C)] pub struct list_head { pub next:*mut list_head,pub prev:*mut list_head }
#[repr(C)] pub struct completion; #[repr(C)] pub struct refcount_t; #[repr(C)] pub struct mutex;
#[repr(C)] pub struct work_struct; #[repr(C)] pub struct workqueue_struct; #[repr(C)] pub struct pci_dev;
#[repr(C)] pub struct auxiliary_device; #[repr(C)] pub struct radix_tree_root; #[repr(C)] pub struct devlink_region; #[repr(C)] pub struct cpu_rmap;
pub type irqreturn_t=c_int;

pub const DEFAULT_UAR_PAGE_SHIFT:u32=12; pub const MAX_MSIX:u32=128; pub const MIN_MSIX_P_PORT:u32=5;
pub const MLX4_MAX_100M_UNITS_VAL:u32=255; pub const MLX4_RATELIMIT_100M_UNITS:u32=3; pub const MLX4_RATELIMIT_1G_UNITS:u32=4; pub const MLX4_RATELIMIT_DEFAULT:u32=0x00ff;
pub const MLX4_ROCE_MAX_GIDS:usize=128; pub const MLX4_ROCE_PF_GIDS:usize=16;
pub const MLX4_FLAG_MSI_X:u32=1<<0; pub const MLX4_FLAG_OLD_PORT_CMDS:u32=1<<1; pub const MLX4_FLAG_MASTER:u32=1<<2; pub const MLX4_FLAG_SLAVE:u32=1<<3; pub const MLX4_FLAG_SRIOV:u32=1<<4; pub const MLX4_FLAG_OLD_REG_MAC:u32=1<<6; pub const MLX4_FLAG_BONDED:u32=1<<7; pub const MLX4_FLAG_SECURE_HOST:u32=1<<8;
pub const MLX4_PORT_CAP_IS_SM:u32=1<<1; pub const MLX4_PORT_CAP_DEV_MGMT_SUP:u32=1<<19; pub const MLX4_MAX_PORTS:usize=2; pub const MLX4_MAX_PORT_PKEYS:usize=128; pub const MLX4_MAX_PORT_GIDS:usize=128;
pub const MLX4_RESERVED_QKEY_BASE:u32=0xffff0000; pub const MLX4_RESERVED_QKEY_MASK:u32=0xffff0000; pub const MLX4_BOARD_ID_LEN:usize=64;
pub const MLX4_MAX_NUM_PF:usize=16; pub const MLX4_MAX_NUM_VF:usize=126; pub const MLX4_MAX_NUM_VF_P_PORT:usize=64; pub const MLX4_MFUNC_MAX:usize=128; pub const MLX4_MAX_EQ_NUM:usize=1024; pub const MLX4_MFUNC_EQ_NUM:usize=4; pub const MLX4_MFUNC_MAX_EQES:usize=8; pub const MLX4_MFUNC_EQE_MASK:usize=7;
pub const MLX4_STEERING_MODE_A0:i32=0; pub const MLX4_STEERING_MODE_B0:i32=1; pub const MLX4_STEERING_MODE_DEVICE_MANAGED:i32=2;
pub const MLX4_STEERING_DMFS_A0_DEFAULT:i32=0; pub const MLX4_STEERING_DMFS_A0_DYNAMIC:i32=1; pub const MLX4_STEERING_DMFS_A0_STATIC:i32=2; pub const MLX4_STEERING_DMFS_A0_DISABLE:i32=3; pub const MLX4_STEERING_DMFS_A0_NOT_SUPPORTED:i32=4;
pub const MLX4_TUNNEL_OFFLOAD_MODE_NONE:i32=0; pub const MLX4_TUNNEL_OFFLOAD_MODE_VXLAN:i32=1;
pub const MLX4_MAX_SGE_RD:usize=(512-16-16)/16; pub const MLX4_NUM_QP_REGION:usize=6; pub const MLX4_DB_PER_PAGE:usize=PAGE_SIZE/4;

#[repr(C)] #[derive(Copy,Clone)] pub struct mlx4_rate_limit_caps { pub num_rates:u16,pub min_unit:u8,pub min_val:u16,pub max_unit:u8,pub max_val:u16 }
#[repr(C)] pub struct mlx4_spec_qps { pub qp0_qkey:u32,pub qp0_proxy:u32,pub qp0_tunnel:u32,pub qp1_proxy:u32,pub qp1_tunnel:u32 }
#[repr(C)] pub struct mlx4_phys_caps { pub gid_phys_table_len:[u32;3],pub pkey_phys_table_len:[u32;3],pub num_phys_eqs:u32,pub base_sqpn:u32,pub base_proxy_sqpn:u32,pub base_tunnel_sqpn:u32 }
#[repr(C)] pub struct mlx4_mtt { pub offset:u32,pub order:c_int,pub page_shift:c_int }
#[repr(C)] pub struct mlx4_buf_list { pub buf:*mut c_void,pub map:dma_addr_t }
#[repr(C)] pub struct mlx4_buf { pub direct:mlx4_buf_list,pub page_list:*mut mlx4_buf_list,pub nbufs:c_int,pub npages:c_int,pub page_shift:c_int }
#[repr(C)] pub struct mlx4_uar { pub pfn:usize,pub index:c_int,pub bf_list:list_head,pub free_bf_bmap:u32,pub map:*mut c_void,pub bf_map:*mut c_void }
#[repr(C)] pub struct mlx4_bf { pub offset:u32,pub buf_size:c_int,pub uar:*mut mlx4_uar,pub reg:*mut c_void }
#[repr(C)] pub struct mlx4_db_pgdir { pub list:list_head,pub bits:[*mut usize;2],pub db_page:*mut __be32,pub db_dma:dma_addr_t }
#[repr(C)] pub union mlx4_db_union { pub pgdir:*mut mlx4_db_pgdir,pub user_page:*mut c_void }
#[repr(C)] pub struct mlx4_db { pub db:*mut __be32,pub u:mlx4_db_union,pub dma:dma_addr_t,pub index:c_int,pub order:c_int }
#[repr(C)] pub struct mlx4_hwq_resources { pub db:mlx4_db,pub mtt:mlx4_mtt,pub buf:mlx4_buf }
#[repr(C)] pub struct mlx4_mr { pub mtt:mlx4_mtt,pub iova:u64,pub size:u64,pub key:u32,pub pd:u32,pub access:u32,pub enabled:c_int }
#[repr(C)] pub struct mlx4_mw { pub key:u32,pub pd:u32,pub type_:c_int,pub enabled:c_int }

#[repr(C)] pub struct mlx4_caps {
 pub fw_ver:u64,pub function:u32,pub num_ports:c_int,pub vl_cap:[c_int;3],pub ib_mtu_cap:[c_int;3],pub ib_port_def_cap:[__be32;3],pub def_mac:[u64;3],pub eth_mtu_cap:[c_int;3],pub gid_table_len:[c_int;3],pub pkey_table_len:[c_int;3],pub trans_type:[c_int;3],pub vendor_oui:[c_int;3],pub wavelength:[c_int;3],pub trans_code:[u64;3],pub local_ca_ack_delay:c_int,pub num_uars:c_int,pub uar_page_size:u32,pub bf_reg_size:c_int,pub bf_regs_per_page:c_int,pub max_sq_sg:c_int,pub max_rq_sg:c_int,pub num_qps:c_int,pub max_wqes:c_int,pub max_sq_desc_sz:c_int,pub max_rq_desc_sz:c_int,pub max_qp_init_rdma:c_int,pub max_qp_dest_rdma:c_int,pub max_tc_eth:c_int,pub spec_qps:*mut mlx4_spec_qps,pub num_srqs:c_int,pub max_srq_wqes:c_int,pub max_srq_sge:c_int,pub reserved_srqs:c_int,pub num_cqs:c_int,pub max_cqes:c_int,pub reserved_cqs:c_int,pub num_sys_eqs:c_int,pub num_eqs:c_int,pub reserved_eqs:c_int,pub num_comp_vectors:c_int,pub num_mpts:c_int,pub num_mtts:c_int,pub fmr_reserved_mtts:c_int,pub reserved_mtts:c_int,pub reserved_mrws:c_int,pub reserved_uars:c_int,pub num_mgms:c_int,pub num_amgms:c_int,pub reserved_mcgs:c_int,pub num_qp_per_mgm:c_int,pub steering_mode:c_int,pub dmfs_high_steer_mode:c_int,pub fs_log_max_ucast_qp_range_size:c_int,pub num_pds:c_int,pub reserved_pds:c_int,pub max_xrcds:c_int,pub reserved_xrcds:c_int,pub mtt_entry_sz:c_int,pub max_msg_sz:u32,pub page_size_cap:u32,pub flags:u64,pub flags2:u64,pub bmme_flags:u32,pub reserved_lkey:u32,pub stat_rate_support:u16,pub port_width_cap:[u8;3],pub max_gso_sz:c_int,pub max_rss_tbl_sz:c_int,pub reserved_qps_cnt:[c_int;6],pub reserved_qps:c_int,pub reserved_qps_base:[c_int;6],pub log_num_macs:c_int,pub log_num_vlans:c_int,pub port_type:[c_int;3],pub supported_type:[u8;3],pub suggested_type:[u8;3],pub default_sense:[u8;3],pub port_mask:[u32;3],pub possible_type:[c_int;3],pub max_counters:u32,pub port_ib_mtu:[u8;3],pub sqp_demux:u16,pub eqe_size:u32,pub cqe_size:u32,pub eqe_factor:u8,pub userspace_caps:u32,pub function_caps:u32,pub hca_core_clock:u16,pub phys_port_id:[u64;3],pub tunnel_offload_mode:c_int,pub rx_checksum_flags_port:[u8;3],pub phv_bit:[u8;3],pub alloc_res_qp_mask:u8,pub dmfs_high_rate_qpn_base:u32,pub dmfs_high_rate_qpn_range:u32,pub vf_caps:u32,pub wol_port:[bool;3],pub rl_caps:mlx4_rate_limit_caps,pub health_buffer_addrs:u32,pub map_clock_to_user:bool }
}
#[repr(C)] pub struct mlx4_quotas { pub qp:c_int,pub cq:c_int,pub srq:c_int,pub mpt:c_int,pub mtt:c_int,pub counter:c_int,pub xrcd:c_int }
#[repr(C)] pub struct mlx4_fw_crdump { pub snapshot_enable:bool,pub region_crspace:*mut devlink_region,pub region_fw_health:*mut devlink_region }
#[repr(C)] pub struct mlx4_dev_persistent { pub pdev:*mut pci_dev,pub dev:*mut mlx4_dev,pub nvfs:[c_int;3],pub num_vfs:c_int,pub curr_port_type:[c_int;3],pub curr_port_poss_type:[c_int;3],pub catas_work:work_struct,pub catas_wq:*mut workqueue_struct,pub device_state_mutex:mutex,pub state:u8,pub interface_state_mutex:mutex,pub interface_state:u8,pub pci_status_mutex:mutex,pub pci_status:c_int,pub crdump:mlx4_fw_crdump }
#[repr(C)] pub struct mlx4_dev { pub persist:*mut mlx4_dev_persistent,pub flags:usize,pub num_slaves:usize,pub caps:mlx4_caps,pub phys_caps:mlx4_phys_caps,pub quotas:mlx4_quotas,pub qp_table_tree:radix_tree_root,pub rev_id:u8,pub port_random_macs:u8,pub board_id:[c_char;64],pub numa_node:c_int,pub oper_log_mgm_entry_size:c_int,pub regid_promisc_array:[u64;3],pub regid_allmulti_array:[u64;3],pub dev_vfs:*mut c_void,pub uar_page_shift:u8 }

#[repr(C)] pub struct mlx4_cq { pub comp:Option<unsafe extern "C" fn(*mut mlx4_cq)>,pub event:Option<unsafe extern "C" fn(*mut mlx4_cq,c_int)>,pub uar:*mut mlx4_uar,pub cons_index:u32,pub irq:u16,pub set_ci_db:*mut __be32,pub arm_db:*mut __be32,pub arm_sn:c_int,pub cqn:c_int,pub vector:u32,pub refcount:refcount_t,pub free:completion,pub usage:u8 }
#[repr(C)] pub struct mlx4_qp { pub event:Option<unsafe extern "C" fn(*mut mlx4_qp,c_int)>,pub qpn:c_int,pub refcount:refcount_t,pub free:completion,pub usage:u8 }
#[repr(C)] pub struct mlx4_srq { pub event:Option<unsafe extern "C" fn(*mut mlx4_srq,c_int)>,pub srqn:c_int,pub max:c_int,pub max_gs:c_int,pub wqe_shift:c_int,pub refcount:refcount_t,pub free:completion }
#[repr(C)] pub struct mlx4_av { pub port_pd:__be32,pub reserved1:u8,pub g_slid:u8,pub dlid:__be16,pub reserved2:u8,pub gid_index:u8,pub stat_rate:u8,pub hop_limit:u8,pub sl_tclass_flowlabel:__be32,pub dgid:[u8;16] }
#[repr(C)] pub union mlx4_ext_av { pub ib:mlx4_av,pub eth:mlx4_av }

#[inline] pub unsafe fn mlx4_fw_ver(major:u64,minor:u64,subminor:u64)->u64 {(major<<32)|(minor<<16)|subminor}
#[inline] pub unsafe fn mlx4_master_func_num(dev:*mut mlx4_dev)->c_int {(*dev).caps.function as c_int}
#[inline] pub unsafe fn mlx4_is_master(dev:*mut mlx4_dev)->c_int {((*dev).flags & MLX4_FLAG_MASTER as usize) as c_int}
#[inline] pub unsafe fn mlx4_is_slave(dev:*mut mlx4_dev)->c_int {((*dev).flags & MLX4_FLAG_SLAVE as usize) as c_int}
#[inline] pub unsafe fn mlx4_is_bonded(dev:*mut mlx4_dev)->c_int {(((*dev).flags & MLX4_FLAG_BONDED as usize)!=0) as c_int}
#[inline] pub unsafe fn mlx4_is_eth(dev:*mut mlx4_dev,port:usize)->c_int {((*dev).caps.port_type[port]!=1) as c_int}
#[inline] pub unsafe fn mlx4_to_hw_uar_index(dev:*mut mlx4_dev,index:c_int)->c_int {index << (PAGE_SHIFT-(*dev).uar_page_shift as u32)}
#[inline] pub unsafe fn mlx4_get_num_reserved_uar(dev:*mut mlx4_dev)->c_int {(128 >> (PAGE_SHIFT-(*dev).uar_page_shift as u32)) as c_int}

extern "C" { pub fn handle_port_mgmt_change_event(work:*mut work_struct); pub fn mlx4_buf_alloc(dev:*mut mlx4_dev,size:c_int,max_direct:c_int,buf:*mut mlx4_buf)->c_int; pub fn mlx4_buf_free(dev:*mut mlx4_dev,size:c_int,buf:*mut mlx4_buf); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
