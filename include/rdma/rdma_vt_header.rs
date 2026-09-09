/* SPDX-License-Identifier: GPL-2.0 OR BSD-3-Clause */
/* Copyright(c) 2016 - 2019 Intel Corporation. */

// Dependencies supplied by the surrounding kernel/RDMA translation.
use core::ffi::c_void;

pub const RVT_MAX_PKEY_VALUES: usize = 16;
pub const RVT_MAX_TRAP_LEN: usize = 100;
pub const RVT_MAX_TRAP_LISTS: usize = 5;
pub const RVT_TRAP_TIMEOUT: usize = 4096;
pub const RVT_CQN_MAX: usize = 16;
pub const RVT_SGE_COPY_MEMCPY: u32 = 0;
pub const RVT_SGE_COPY_CACHELESS: u32 = 1;
pub const RVT_SGE_COPY_ADAPTIVE: u32 = 2;

#[repr(C)] pub struct trap_list { pub list_len: u32, pub list: list_head }
#[repr(C)] pub struct rvt_qp;
#[repr(C)] pub struct rvt_qpn_table;
#[repr(C)] pub struct rvt_dev_info;
#[repr(C)] pub struct rvt_swqe;
#[repr(C)] pub struct rvt_qp_ibdev;
#[repr(C)] pub struct rvt_mcast;
#[repr(C)] pub struct rvt_sge;
#[repr(C)] pub struct rvt_lkey_table;
#[repr(C)] pub struct rvt_mregion;
#[repr(C)] pub struct ib_device_attr;
#[repr(C)] pub struct ib_ucontext;
#[repr(C)] pub struct ib_pd;
#[repr(C)] pub struct ib_ah;
#[repr(C)] pub struct rdma_ah_attr;
#[repr(C)] pub struct ib_qp_init_attr;
#[repr(C)] pub struct ib_qp_attr;
#[repr(C)] pub struct ib_udata;
#[repr(C)] pub struct pci_dev;
#[repr(C)] pub struct ib_port_attr;
#[repr(C)] pub struct ib_wc_opcode;
#[repr(C)] pub struct ib_mr;
#[repr(C)] pub struct ib_sge;
#[repr(C)] pub struct ib_mad_agent;
#[repr(C)] pub struct timer_list;
#[repr(C)] pub struct kref;
#[repr(C)] pub struct vm_area_struct;
#[repr(C)] pub struct ib_device { pub dev: device }
#[repr(C)] pub struct device;
#[repr(C)] pub struct list_head;
#[repr(C)] pub struct rb_root;
#[repr(C)] pub struct spinlock_t;
#[repr(C)] pub struct atomic_t;
#[repr(C)] pub struct ib_qp_type;
#[repr(C)] pub struct ib_gid;
#[repr(C)] pub struct rvt_operation_params;

pub type __be64 = u64; pub type __be16 = u16; pub type __u64 = u64;

#[repr(C)]
pub struct rvt_ibport {
    pub qp: [*mut rvt_qp; 2], pub send_agent: *mut ib_mad_agent,
    pub mcast_tree: rb_root, pub lock: spinlock_t,
    pub mkey_lease_timeout: usize, pub trap_timeout: usize,
    pub gid_prefix: __be64, pub mkey: __be64, pub tid: u64,
    pub port_cap_flags: u32, pub port_cap3_flags: u16,
    pub pma_sample_start: u32, pub pma_sample_interval: u32,
    pub pma_counter_select: [__be16; 5], pub pma_tag: u16,
    pub mkey_lease_period: u16, pub sm_lid: u32, pub sm_sl: u8,
    pub mkeyprot: u8, pub subnet_timeout: u8, pub vl_high_limit: u8,
    pub n_rc_resends: u64, pub n_seq_naks: u64, pub n_rdma_seq: u64,
    pub n_rnr_naks: u64, pub n_other_naks: u64, pub n_loop_pkts: u64,
    pub n_pkt_drops: u64, pub n_vl15_dropped: u64, pub n_rc_timeouts: u64,
    pub n_dmawait: u64, pub n_unaligned: u64, pub n_rc_dupreq: u64,
    pub n_rc_seqnak: u64, pub n_rc_crwaits: u64,
    pub pkey_violations: u16, pub qkey_violations: u16, pub mkey_violations: u16,
    pub z_rc_acks: u64, pub z_rc_qacks: u64, pub z_rc_delayed_comp: u64,
    pub rc_acks: *mut u64, pub rc_qacks: *mut u64, pub rc_delayed_comp: *mut u64,
    pub priv_: *mut c_void, pub pkey_table: *mut u16, pub sm_ah: *mut rvt_ah,
    pub trap_lists: [trap_list; RVT_MAX_TRAP_LISTS], pub trap_timer: timer_list,
}

#[repr(C)] pub struct rvt_driver_params {
    pub props: ib_device_attr, pub lkey_table_size: u32, pub qp_table_size: u32,
    pub sge_copy_mode: u32, pub wss_threshold: u32, pub wss_clean_period: u32,
    pub qpn_start: i32, pub qpn_inc: i32, pub qpn_res_start: i32, pub qpn_res_end: i32,
    pub nports: i32, pub npkeys: i32, pub node: i32, pub psn_mask: i32,
    pub psn_shift: i32, pub psn_modify_mask: i32, pub core_cap_flags: u32,
    pub max_mad_size: u32, pub qos_shift: u8, pub max_rdma_atomic: u8,
    pub extra_rdma_atomic: u8, pub reserved_operations: u8,
}
#[repr(C)] pub struct rvt_ucontext { pub ibucontext: ib_ucontext, pub priv_: *mut c_void }
#[repr(C)] pub struct rvt_pd { pub ibpd: ib_pd, pub user: bool }
#[repr(C)] pub struct rvt_ah { pub ibah: ib_ah, pub attr: rdma_ah_attr, pub vl: u8, pub log_pmtu: u8 }
#[repr(C)] pub struct rvt_mmap_info { pub pending_mmaps: list_head, pub context: *mut ib_ucontext, pub obj: *mut c_void, pub offset: __u64, pub ref_: kref, pub size: u32 }
#[repr(C)] pub struct rvt_wss { pub entries: *mut usize, pub total_count: atomic_t, pub clean_counter: atomic_t, pub clean_entry: atomic_t, pub threshold: i32, pub num_entries: i32, pub pages_mask: isize, pub clean_period: u32 }

#[repr(C)] pub struct rvt_driver_provided {
    pub schedule_send: Option<unsafe extern "C" fn(*mut rvt_qp) -> bool>,
    pub schedule_send_no_lock: Option<unsafe extern "C" fn(*mut rvt_qp) -> bool>,
    pub setup_wqe: Option<unsafe extern "C" fn(*mut rvt_qp, *mut rvt_swqe, *mut bool) -> i32>,
    pub do_send: Option<unsafe extern "C" fn(*mut rvt_qp)>,
    pub get_pci_dev: Option<unsafe extern "C" fn(*mut rvt_dev_info) -> *mut pci_dev>,
    pub qp_priv_alloc: Option<unsafe extern "C" fn(*mut rvt_dev_info, *mut rvt_qp) -> *mut c_void>,
    pub qp_priv_init: Option<unsafe extern "C" fn(*mut rvt_dev_info, *mut rvt_qp, *mut ib_qp_init_attr) -> i32>,
    pub qp_priv_free: Option<unsafe extern "C" fn(*mut rvt_dev_info, *mut rvt_qp)>,
    pub notify_qp_reset: Option<unsafe extern "C" fn(*mut rvt_qp)>,
    pub get_pmtu_from_attr: Option<unsafe extern "C" fn(*mut rvt_dev_info, *mut rvt_qp, *mut ib_qp_attr) -> i32>,
    pub flush_qp_waiters: Option<unsafe extern "C" fn(*mut rvt_qp)>, pub stop_send_queue: Option<unsafe extern "C" fn(*mut rvt_qp)>,
    pub quiesce_qp: Option<unsafe extern "C" fn(*mut rvt_qp)>, pub notify_error_qp: Option<unsafe extern "C" fn(*mut rvt_qp)>,
    pub mtu_from_qp: Option<unsafe extern "C" fn(*mut rvt_dev_info, *mut rvt_qp, u32) -> u32>,
    pub mtu_to_path_mtu: Option<unsafe extern "C" fn(u32) -> i32>,
    pub get_guid_be: Option<unsafe extern "C" fn(*mut rvt_dev_info, *mut rvt_ibport, i32, *mut __be64) -> i32>,
    pub query_port_state: Option<unsafe extern "C" fn(*mut rvt_dev_info, u32, *mut ib_port_attr) -> i32>,
    pub shut_down_port: Option<unsafe extern "C" fn(*mut rvt_dev_info, u32) -> i32>,
    pub cap_mask_chg: Option<unsafe extern "C" fn(*mut rvt_dev_info, u32)>,
    pub free_all_qps: Option<unsafe extern "C" fn(*mut rvt_dev_info) -> u32>,
    pub check_ah: Option<unsafe extern "C" fn(*mut ib_device, *mut rdma_ah_attr) -> i32>,
    pub notify_new_ah: Option<unsafe extern "C" fn(*mut ib_device, *mut rdma_ah_attr, *mut rvt_ah)>,
    pub alloc_qpn: Option<unsafe extern "C" fn(*mut rvt_dev_info, *mut rvt_qpn_table, ib_qp_type, u32) -> i32>,
    pub check_modify_qp: Option<unsafe extern "C" fn(*mut rvt_qp, *mut ib_qp_attr, i32, *mut ib_udata) -> i32>,
    pub modify_qp: Option<unsafe extern "C" fn(*mut rvt_qp, *mut ib_qp_attr, i32, *mut ib_udata)>,
    pub notify_create_mad_agent: Option<unsafe extern "C" fn(*mut rvt_dev_info, i32)>, pub notify_free_mad_agent: Option<unsafe extern "C" fn(*mut rvt_dev_info, i32)>,
    pub notify_restart_rc: Option<unsafe extern "C" fn(*mut rvt_qp, u32, i32)>, pub comp_vect_cpu_lookup: Option<unsafe extern "C" fn(*mut rvt_dev_info, i32) -> i32>,
    pub alloc_ucontext: Option<unsafe extern "C" fn(*mut ib_ucontext, *mut ib_udata) -> i32>, pub dealloc_ucontext: Option<unsafe extern "C" fn(*mut ib_ucontext)>,
    pub mmap: Option<unsafe extern "C" fn(*mut ib_ucontext, *mut vm_area_struct) -> i32>,
}

#[repr(C)] pub struct rvt_dev_info {
    pub ibdev: ib_device, pub dparms: rvt_driver_params, pub post_parms: *const rvt_operation_params,
    pub wc_opcode: *const ib_wc_opcode, pub driver_f: rvt_driver_provided, pub dma_mr: *mut rvt_mregion,
    pub lkey_table: rvt_lkey_table, pub n_pds_allocated: i32, pub n_pds_lock: spinlock_t,
    pub n_ahs_allocated: i32, pub n_ahs_lock: spinlock_t, pub n_srqs_allocated: u32, pub n_srqs_lock: spinlock_t,
    pub flags: i32, pub ports: *mut *mut rvt_ibport, pub qp_dev: *mut rvt_qp_ibdev, pub n_qps_allocated: u32,
    pub n_rc_qps: u32, pub busy_jiffies: u32, pub n_qps_lock: spinlock_t, pub pending_mmaps: list_head,
    pub mmap_offset_lock: spinlock_t, pub mmap_offset: u32, pub pending_lock: spinlock_t,
    pub n_cqs_allocated: u32, pub n_cqs_lock: spinlock_t, pub n_mcast_grps_allocated: u32,
    pub n_mcast_grps_lock: spinlock_t, pub wss: *mut rvt_wss,
}

extern "C" {
    pub fn rvt_alloc_device(size: usize, nports: i32) -> *mut rvt_dev_info;
    pub fn rvt_dealloc_device(rdi: *mut rvt_dev_info); pub fn rvt_register_device(rvd: *mut rvt_dev_info) -> i32;
    pub fn rvt_unregister_device(rvd: *mut rvt_dev_info); pub fn rvt_check_ah(ibdev: *mut ib_device, ah_attr: *mut rdma_ah_attr) -> i32;
    pub fn rvt_init_port(rdi: *mut rvt_dev_info, port: *mut rvt_ibport, port_index: i32, pkey_table: *mut u16) -> i32;
    pub fn rvt_fast_reg_mr(qp: *mut rvt_qp, ibmr: *mut ib_mr, key: u32, access: i32) -> i32;
    pub fn rvt_invalidate_rkey(qp: *mut rvt_qp, rkey: u32) -> i32;
    pub fn rvt_rkey_ok(qp: *mut rvt_qp, sge: *mut rvt_sge, len: u32, vaddr: u64, rkey: u32, acc: i32) -> i32;
    pub fn rvt_lkey_ok(rkt: *mut rvt_lkey_table, pd: *mut rvt_pd, isge: *mut rvt_sge, last_sge: *mut rvt_sge, sge: *mut ib_sge, acc: i32) -> i32;
    pub fn rvt_mcast_find(ibp: *mut rvt_ibport, mgid: *mut ib_gid, lid: u16) -> *mut rvt_mcast;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
