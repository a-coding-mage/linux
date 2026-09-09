/* SPDX-License-Identifier: GPL-2.0 OR BSD-3-Clause */
/* Copyright(c) 2016 - 2020 Intel Corporation. */
// C includes and build-time annotations are supplied by the surrounding crate.

pub const RVT_R_WRID_VALID: u32 = 0;
pub const RVT_R_REWIND_SGE: u32 = 1;
pub const RVT_R_REUSE_SGE: u32 = 0x01;
pub const RVT_R_RDMAR_SEQ: u32 = 0x02;
pub const RVT_R_RSP_NAK: u32 = 0x04;
pub const RVT_R_RSP_SEND: u32 = 0x08;
pub const RVT_R_COMM_EST: u32 = 0x10;
pub const RVT_KDETH_QP_PREFIX: u32 = 0x80;
pub const RVT_KDETH_QP_SUFFIX: u32 = 0xffff;
pub const RVT_KDETH_QP_PREFIX_MASK: u32 = 0x00ff0000;
pub const RVT_KDETH_QP_PREFIX_SHIFT: u32 = 16;
pub const RVT_KDETH_QP_BASE: u32 = RVT_KDETH_QP_PREFIX << RVT_KDETH_QP_PREFIX_SHIFT;
pub const RVT_KDETH_QP_MAX: u32 = RVT_KDETH_QP_BASE + RVT_KDETH_QP_SUFFIX;
pub const RVT_AIP_QP_PREFIX: u32 = 0x81;
pub const RVT_AIP_QP_SUFFIX: u32 = 0xffff;
pub const RVT_AIP_QP_PREFIX_MASK: u32 = 0x00ff0000;
pub const RVT_AIP_QP_PREFIX_SHIFT: u32 = 16;
pub const RVT_AIP_QP_BASE: u32 = RVT_AIP_QP_PREFIX << RVT_AIP_QP_PREFIX_SHIFT;
pub const RVT_AIP_QPN_MAX: u32 = 1 << RVT_AIP_QP_PREFIX_SHIFT;
pub const RVT_AIP_QP_MAX: u32 = RVT_AIP_QP_BASE + RVT_AIP_QPN_MAX - 1;

pub const RVT_S_SIGNAL_REQ_WR: u32 = 0x0001; pub const RVT_S_BUSY: u32 = 0x0002;
pub const RVT_S_TIMER: u32 = 0x0004; pub const RVT_S_RESP_PENDING: u32 = 0x0008;
pub const RVT_S_ACK_PENDING: u32 = 0x0010; pub const RVT_S_WAIT_FENCE: u32 = 0x0020;
pub const RVT_S_WAIT_RDMAR: u32 = 0x0040; pub const RVT_S_WAIT_RNR: u32 = 0x0080;
pub const RVT_S_WAIT_SSN_CREDIT: u32 = 0x0100; pub const RVT_S_WAIT_DMA: u32 = 0x0200;
pub const RVT_S_WAIT_PIO: u32 = 0x0400; pub const RVT_S_WAIT_TX: u32 = 0x0800;
pub const RVT_S_WAIT_DMA_DESC: u32 = 0x1000; pub const RVT_S_WAIT_KMEM: u32 = 0x2000;
pub const RVT_S_WAIT_PSN: u32 = 0x4000; pub const RVT_S_WAIT_ACK: u32 = 0x8000;
pub const RVT_S_SEND_ONE: u32 = 0x10000; pub const RVT_S_UNLIMITED_CREDIT: u32 = 0x20000;
pub const RVT_S_ECN: u32 = 0x40000; pub const RVT_S_MAX_BIT_MASK: u32 = 0x800000;
pub const RVT_S_ANY_WAIT_IO: u32 = RVT_S_WAIT_PIO | RVT_S_WAIT_TX | RVT_S_WAIT_DMA_DESC | RVT_S_WAIT_KMEM;
pub const RVT_S_ANY_WAIT_SEND: u32 = RVT_S_WAIT_FENCE | RVT_S_WAIT_RDMAR | RVT_S_WAIT_RNR | RVT_S_WAIT_SSN_CREDIT | RVT_S_WAIT_DMA | RVT_S_WAIT_PSN | RVT_S_WAIT_ACK;
pub const RVT_S_ANY_WAIT: u32 = RVT_S_ANY_WAIT_IO | RVT_S_ANY_WAIT_SEND;
pub const RVT_OPCODE_QP_MASK: u32 = 0xE0;
pub const RVT_POST_SEND_OK: u32 = 0x01; pub const RVT_POST_RECV_OK: u32 = 0x02;
pub const RVT_PROCESS_RECV_OK: u32 = 0x04; pub const RVT_PROCESS_SEND_OK: u32 = 0x08;
pub const RVT_PROCESS_NEXT_SEND_OK: u32 = 0x10; pub const RVT_FLUSH_SEND: u32 = 0x20;
pub const RVT_FLUSH_RECV: u32 = 0x40;
pub const RVT_PROCESS_OR_FLUSH_SEND: u32 = RVT_PROCESS_SEND_OK | RVT_FLUSH_SEND;
pub const RVT_SEND_OR_FLUSH_OR_RECV_OK: u32 = RVT_PROCESS_SEND_OK | RVT_FLUSH_SEND | RVT_PROCESS_RECV_OK;
pub const RVT_SEND_RESERVE_USED: u32 = IB_SEND_RESERVED_START;
pub const RVT_SEND_COMPLETION_ONLY: u32 = IB_SEND_RESERVED_START << 1;

#[repr(C)] pub struct rvt_ud_wr { pub wr: ib_ud_wr, pub attr: *mut rdma_ah_attr }
#[repr(C)] pub union rvt_swqe_wr { pub wr: ib_send_wr, pub ud_wr: rvt_ud_wr, pub reg_wr: ib_reg_wr, pub rdma_wr: ib_rdma_wr, pub atomic_wr: ib_atomic_wr }
#[repr(C)] pub struct rvt_swqe { pub wr: rvt_swqe_wr, pub psn: u32, pub lpsn: u32, pub ssn: u32, pub length: u32, pub priv_: *mut core::ffi::c_void, pub sg_list: [rvt_sge; 0] }
#[repr(C)] pub struct rvt_krwq { pub p_lock: spinlock_t, pub head: u32, pub c_lock: spinlock_t, pub tail: u32, pub count: u32, pub curr_wq: *mut rvt_rwqe, pub wq: [rvt_rwqe; 0] }

pub unsafe fn rvt_get_swqe_ah(swqe: *mut rvt_swqe) -> *mut rvt_ah { ibah_to_rvtah((*swqe).ud_wr.wr.ah) }
pub unsafe fn rvt_get_swqe_ah_attr(swqe: *mut rvt_swqe) -> *mut rdma_ah_attr { (*swqe).ud_wr.attr }
pub unsafe fn rvt_get_swqe_remote_qpn(swqe: *mut rvt_swqe) -> u32 { (*swqe).ud_wr.wr.remote_qpn }
pub unsafe fn rvt_get_swqe_remote_qkey(swqe: *mut rvt_swqe) -> u32 { (*swqe).ud_wr.wr.remote_qkey }
pub unsafe fn rvt_get_swqe_pkey_index(swqe: *mut rvt_swqe) -> u16 { (*swqe).ud_wr.wr.pkey_index }

#[repr(C)] pub struct rvt_rq { pub wq: *mut rvt_rwq, pub kwq: *mut rvt_krwq, pub size: u32, pub max_sge: u8, pub lock: spinlock_t }
pub unsafe fn rvt_get_rq_count(rq: *mut rvt_rq, head: u32, tail: u32) -> u32 { let mut count = head.wrapping_sub(tail); if (count as i32) < 0 { count = count.wrapping_add((*rq).size); } count }
#[repr(C)] pub struct rvt_ack_entry { pub rdma_sge: rvt_sge, pub atomic_data: u64, pub psn: u32, pub lpsn: u32, pub opcode: u8, pub sent: u8, pub priv_: *mut core::ffi::c_void }
pub const RC_QP_SCALING_INTERVAL: u32 = 5;
pub const RVT_OPERATION_PRIV: u32 = 1; pub const RVT_OPERATION_ATOMIC: u32 = 2; pub const RVT_OPERATION_ATOMIC_SGE: u32 = 4; pub const RVT_OPERATION_LOCAL: u32 = 8; pub const RVT_OPERATION_USE_RESERVE: u32 = 0x10; pub const RVT_OPERATION_IGN_RNR_CNT: u32 = 0x20;
pub const RVT_OPERATION_MAX: u32 = IB_WR_RESERVED10 + 1;
#[repr(C)] pub struct rvt_operation_params { pub length: usize, pub qpt_support: u32, pub flags: u32 }

// The remaining declarations retain the C ABI and depend on types supplied by included headers.
#[repr(C)] pub struct rvt_qp {
    pub ibqp: ib_qp, pub priv_: *mut core::ffi::c_void, pub remote_ah_attr: rdma_ah_attr, pub alt_ah_attr: rdma_ah_attr,
    pub next: *mut rvt_qp, pub s_wq: *mut rvt_swqe, pub ip: *mut rvt_mmap_info, pub timeout_jiffies: c_ulong,
    pub srate_mbps: c_int, pub pid: pid_t, pub remote_qpn: u32, pub qkey: u32, pub s_size: u32, pub pmtu: u16,
    pub log_pmtu: u8, pub state: u8, pub allowed_ops: u8, pub qp_access_flags: u8, pub alt_timeout: u8, pub timeout: u8,
    pub s_srate: u8, pub s_mig_state: u8, pub port_num: u8, pub s_pkey_index: u8, pub s_alt_pkey_index: u8,
    pub r_max_rd_atomic: u8, pub s_max_rd_atomic: u8, pub s_retry_cnt: u8, pub s_rnr_retry_cnt: u8, pub r_min_rnr_timer: u8,
    pub s_max_sge: u8, pub s_draining: u8, pub refcount: atomic_t, pub wait: wait_queue_head_t, pub s_ack_queue: *mut rvt_ack_entry,
    pub s_rdma_read_sge: rvt_sge_state, pub r_lock: spinlock_t, pub r_psn: u32, pub r_aflags: c_ulong, pub r_wr_id: u64,
    pub r_ack_psn: u32, pub r_len: u32, pub r_rcv_len: u32, pub r_msn: u32, pub r_state: u8, pub r_flags: u8,
    pub r_head_ack_queue: u8, pub r_adefered: u8, pub rspwait: list_head, pub r_sge: rvt_sge_state, pub r_rq: rvt_rq,
    pub s_hlock: spinlock_t, pub s_head: u32, pub s_next_psn: u32, pub s_avail: u32, pub s_ssn: u32, pub s_reserved_used: atomic_t,
    pub s_lock: spinlock_t, pub s_flags: u32, pub s_cur_sge: *mut rvt_sge_state, pub s_wqe: *mut rvt_swqe, pub s_sge: rvt_sge_state,
    pub s_rdma_mr: *mut rvt_mregion, pub s_len: u32, pub s_rdma_read_len: u32, pub s_last_psn: u32, pub s_sending_psn: u32,
    pub s_sending_hpsn: u32, pub s_psn: u32, pub s_ack_rdma_psn: u32, pub s_ack_psn: u32, pub s_tail: u32, pub s_cur: u32,
    pub s_acked: u32, pub s_last: u32, pub s_lsn: u32, pub s_ahgpsn: u32, pub s_cur_size: u16, pub s_rdma_ack_cnt: u16,
    pub s_hdrwords: u8, pub s_ahgidx: i8, pub s_state: u8, pub s_ack_state: u8, pub s_nak_state: u8, pub r_nak_state: u8,
    pub s_retry: u8, pub s_rnr_retry: u8, pub s_num_rd_atomic: u8, pub s_tail_ack_queue: u8, pub s_acked_ack_queue: u8,
    pub s_ack_rdma_sge: rvt_sge_state, pub s_timer: timer_list, pub s_rnr_timer: hrtimer, pub local_ops_pending: atomic_t,
    pub r_sg_list: *mut rvt_sge,
}
#[repr(C)] pub struct rvt_srq { pub ibsrq: ib_srq, pub rq: rvt_rq, pub ip: *mut rvt_mmap_info, pub limit: u32 }
pub unsafe fn ibsrq_to_rvtsrq(x: *mut ib_srq) -> *mut rvt_srq { container_of(x) }
pub unsafe fn ibqp_to_rvtqp(x: *mut ib_qp) -> *mut rvt_qp { container_of(x) }
pub const RVT_QPN_MAX: u32 = 1 << 24;
pub const RVT_QPNMAP_ENTRIES: usize = (RVT_QPN_MAX as usize) / PAGE_SIZE / BITS_PER_BYTE;
pub const RVT_BITS_PER_PAGE: usize = PAGE_SIZE * BITS_PER_BYTE;
pub const RVT_BITS_PER_PAGE_MASK: usize = RVT_BITS_PER_PAGE - 1;
pub const RVT_QPN_MASK: u32 = IB_QPN_MASK;
#[repr(C)] pub struct rvt_qpn_map { pub page: *mut core::ffi::c_void }
#[repr(C)] pub struct rvt_qpn_table { pub lock: spinlock_t, pub flags: c_uint, pub last: u32, pub nmaps: u32, pub limit: u16, pub incr: u8, pub map: [rvt_qpn_map; RVT_QPNMAP_ENTRIES] }
#[repr(C)] pub struct rvt_qp_ibdev { pub qp_table_size: u32, pub qp_table_bits: u32, pub qp_table: *mut *mut rvt_qp, pub qpt_lock: spinlock_t, pub qpn_table: rvt_qpn_table }
#[repr(C)] pub struct rvt_mcast_qp { pub list: list_head, pub qp: *mut rvt_qp }
#[repr(C)] pub struct rvt_mcast_addr { pub mgid: ib_gid, pub lid: u16 }
#[repr(C)] pub struct rvt_mcast { pub rb_node: rb_node, pub mcast_addr: rvt_mcast_addr, pub qp_list: list_head, pub wait: wait_queue_head_t, pub refcount: atomic_t, pub n_attached: c_int }

pub unsafe fn rvt_get_swqe_ptr(qp: *mut rvt_qp, n: usize) -> *mut rvt_swqe { ( ( (*qp).s_wq as *mut u8).add((core::mem::size_of::<rvt_swqe>() + (*qp).s_max_sge as usize * core::mem::size_of::<rvt_sge>()) * n) as *mut rvt_swqe) }
pub unsafe fn rvt_get_rwqe_ptr(rq: *mut rvt_rq, n: usize) -> *mut rvt_rwqe { ((*rq).kwq as *mut u8).add((core::mem::size_of::<rvt_rwqe>() + (*rq).max_sge as usize * core::mem::size_of::<ib_sge>()) * n) as *mut rvt_rwqe }
pub unsafe fn rvt_is_user_qp(qp: *mut rvt_qp) -> bool { (*qp).pid != 0 }
pub unsafe fn rvt_get_qp(qp: *mut rvt_qp) { atomic_inc(&mut (*qp).refcount) }
pub unsafe fn rvt_put_qp(qp: *mut rvt_qp) { if !qp.is_null() && atomic_dec_and_test(&mut (*qp).refcount) { wake_up(&mut (*qp).wait); } }
pub unsafe fn rvt_put_swqe(wqe: *mut rvt_swqe) { for i in 0..(*wqe).wr.wr.num_sge { rvt_put_mr((*wqe).sg_list.as_mut_ptr().add(i as usize).read().mr); } }
pub unsafe fn rvt_qp_wqe_reserve(qp: *mut rvt_qp, _wqe: *mut rvt_swqe) { atomic_inc(&mut (*qp).s_reserved_used) }
pub unsafe fn rvt_qp_wqe_unreserve(qp: *mut rvt_qp, flags: c_int) { if (flags as u32 & RVT_SEND_RESERVE_USED) != 0 { atomic_dec(&mut (*qp).s_reserved_used); smp_mb__after_atomic(); } }
extern "C" { pub static ib_rvt_wc_opcode: [ib_wc_opcode; 0]; pub fn rvt_compute_aeth(qp: *mut rvt_qp) -> __be32; pub fn rvt_get_credit(qp: *mut rvt_qp, aeth: u32); pub fn rvt_restart_sge(ss: *mut rvt_sge_state, wqe: *mut rvt_swqe, len: u32) -> u32; }
pub unsafe fn rvt_cmp_msn(a: u32, b: u32) -> c_int { (((a as c_int).wrapping_sub(b as c_int)) << 8) }
pub unsafe fn rvt_div_round_up_mtu(qp: *mut rvt_qp, len: u32) -> u32 { (len + (*qp).pmtu as u32 - 1) >> (*qp).log_pmtu }
pub unsafe fn rvt_div_mtu(qp: *mut rvt_qp, len: u32) -> u32 { len >> (*qp).log_pmtu }
pub unsafe fn rvt_timeout_to_jiffies(mut timeout: u8) -> c_ulong { if timeout > 31 { timeout = 31; } usecs_to_jiffies(1u32 << timeout) * 4096 / 1000 }

extern "C" {
    pub fn rvt_lookup_qpn(rdi: *mut rvt_dev_info, rvp: *mut rvt_ibport, qpn: u32) -> *mut rvt_qp;
    pub fn rvt_mod_retry_timer_ext(qp: *mut rvt_qp, shift: u8);
    pub fn rvt_error_qp(qp: *mut rvt_qp, err: ib_wc_status) -> c_int;
    pub fn rvt_get_rwqe(qp: *mut rvt_qp, wr_id_only: bool) -> c_int;
    pub fn rvt_comm_est(qp: *mut rvt_qp); pub fn rvt_rc_error(qp: *mut rvt_qp, err: ib_wc_status);
    pub fn rvt_rnr_tbl_to_usec(index: u32) -> c_ulong; pub fn rvt_rc_rnr_retry(t: *mut hrtimer) -> hrtimer_restart;
    pub fn rvt_add_rnr_timer(qp: *mut rvt_qp, aeth: u32); pub fn rvt_del_timers_sync(qp: *mut rvt_qp); pub fn rvt_stop_rc_timers(qp: *mut rvt_qp); pub fn rvt_add_retry_timer_ext(qp: *mut rvt_qp, shift: u8);
    pub fn rvt_copy_sge(qp: *mut rvt_qp, ss: *mut rvt_sge_state, data: *mut core::ffi::c_void, length: u32, release: bool, copy_last: bool);
    pub fn rvt_send_complete(qp: *mut rvt_qp, wqe: *mut rvt_swqe, status: ib_wc_status); pub fn rvt_ruc_loopback(qp: *mut rvt_qp);
    pub fn rvt_qp_iter_init(rdi: *mut rvt_dev_info, v: u64, cb: Option<unsafe extern "C" fn(*mut rvt_qp, u64)>) -> *mut rvt_qp_iter;
    pub fn rvt_qp_iter_next(iter: *mut rvt_qp_iter) -> c_int; pub fn rvt_qp_iter(rdi: *mut rvt_dev_info, v: u64, cb: Option<unsafe extern "C" fn(*mut rvt_qp, u64)>);
    pub fn rvt_qp_mr_clean(qp: *mut rvt_qp, lkey: u32);
}
#[repr(C)] pub struct rvt_qp_iter { pub qp: *mut rvt_qp, pub rdi: *mut rvt_dev_info, pub cb: Option<unsafe extern "C" fn(*mut rvt_qp, u64)>, pub v: u64, pub specials: c_int, pub n: c_int }
extern "C" { pub static ib_rvt_state_ops: [c_int; 0]; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
