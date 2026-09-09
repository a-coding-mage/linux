/* SPDX-License-Identifier: GPL-2.0 */

// Dependencies supplied by the surrounding kernel/RDS translation unit are intentionally external.

pub const RDS_IB_MAX_SGE: usize = 8;
pub const RDS_IB_RECV_SGE: usize = 2;
pub const RDS_IB_DEFAULT_RECV_WR: u32 = 1024;
pub const RDS_IB_DEFAULT_SEND_WR: u32 = 256;
pub const RDS_IB_DEFAULT_FR_WR: u32 = 512;
pub const RDS_IB_DEFAULT_RETRY_COUNT: u32 = 1;
pub const RDS_IB_SUPPORTED_PROTOCOLS: u32 = 0x00000003;
pub const RDS_IB_RECYCLE_BATCH_COUNT: u32 = 32;
pub const RDS_IB_WC_MAX: usize = 32;

extern "C" {
    pub static mut rds_ib_devices_lock: rw_semaphore;
    pub static mut rds_ib_devices: list_head;
}

#[repr(C)] pub struct rds_page_frag { pub f_item: list_head, pub f_cache_entry: list_head, pub f_sg: scatterlist }
#[repr(C)] pub struct rds_ib_incoming { pub ii_frags: list_head, pub ii_cache_entry: list_head, pub ii_inc: rds_incoming }
#[repr(C)] pub struct rds_ib_cache_head { pub first: *mut list_head, pub count: c_ulong }
#[repr(C)] pub struct rds_ib_refill_cache { pub percpu: *mut rds_ib_cache_head, pub xfer: *mut list_head, pub ready: *mut list_head }

#[repr(C)] pub struct rds_ib_conn_priv_cmn {
    pub ricpc_protocol_major: u8, pub ricpc_protocol_minor: u8,
    pub ricpc_protocol_minor_mask: __be16, pub ricpc_dp_toss: u8,
    pub ripc_reserved1: u8, pub ripc_reserved2: __be16,
    pub ricpc_ack_seq: __be64, pub ricpc_credit: __be32,
}
#[repr(C)] pub struct rds_ib_connect_private { pub dp_saddr: __be32, pub dp_daddr: __be32, pub dp_cmn: rds_ib_conn_priv_cmn }
#[repr(C)] pub struct rds6_ib_connect_private { pub dp_saddr: in6_addr, pub dp_daddr: in6_addr, pub dp_cmn: rds_ib_conn_priv_cmn }
#[repr(C)] pub union rds_ib_conn_priv { pub ricp_v4: rds_ib_connect_private, pub ricp_v6: rds6_ib_connect_private }

#[repr(C)] pub union rds_ib_send_work__bindgen_ty_1 { pub s_wr: ib_send_wr, pub s_rdma_wr: ib_rdma_wr, pub s_atomic_wr: ib_atomic_wr }
#[repr(C)] pub struct rds_ib_send_work { pub s_op: *mut c_void, pub __bindgen_anon_1: rds_ib_send_work__bindgen_ty_1, pub s_sge: [ib_sge; RDS_IB_MAX_SGE], pub s_queued: c_ulong }
#[repr(C)] pub struct rds_ib_recv_work { pub r_ibinc: *mut rds_ib_incoming, pub r_frag: *mut rds_page_frag, pub r_wr: ib_recv_wr, pub r_sge: [ib_sge; 2] }
#[repr(C)] pub struct rds_ib_work_ring { pub w_nr: u32, pub w_alloc_ptr: u32, pub w_alloc_ctr: u32, pub w_free_ptr: u32, pub w_free_ctr: atomic_t }
#[repr(C)] pub struct rds_ib_ack_state { pub ack_next: u64, pub ack_recv: u64, pub ack_required: u32, pub ack_next_valid: u32, pub ack_recv_valid: u32 }

pub struct rds_ib_device;
#[repr(C)] pub struct rds_ib_connection {
    pub ib_node: list_head, pub rds_ibdev: *mut rds_ib_device, pub conn: *mut rds_connection,
    pub i_cm_id: *mut rdma_cm_id, pub i_pd: *mut ib_pd, pub i_send_cq: *mut ib_cq, pub i_recv_cq: *mut ib_cq,
    pub i_send_wc: [ib_wc; RDS_IB_WC_MAX], pub i_recv_wc: [ib_wc; RDS_IB_WC_MAX],
    pub i_fastreg_wrs: atomic_t, pub i_fastreg_inuse_count: atomic_t,
    pub i_send_tasklet: tasklet_struct, pub i_recv_tasklet: tasklet_struct,
    pub i_send_ring: rds_ib_work_ring, pub i_data_op: *mut rm_data_op, pub i_send_hdrs: *mut *mut rds_header,
    pub i_send_hdrs_dma: *mut dma_addr_t, pub i_sends: *mut rds_ib_send_work, pub i_signaled_sends: atomic_t,
    pub i_recv_mutex: mutex, pub i_recv_ring: rds_ib_work_ring, pub i_ibinc: *mut rds_ib_incoming,
    pub i_recv_data_rem: u32, pub i_recv_hdrs: *mut *mut rds_header, pub i_recv_hdrs_dma: *mut dma_addr_t,
    pub i_recvs: *mut rds_ib_recv_work, pub i_ack_recv: u64, pub i_cache_incs: rds_ib_refill_cache,
    pub i_cache_frags: rds_ib_refill_cache, pub i_cache_allocs: atomic_t, pub i_ack_flags: c_ulong,
    #[cfg(not(KERNEL_HAS_ATOMIC64))] pub i_ack_lock: spinlock_t,
    pub i_ack_next: u64, pub i_ack: *mut rds_header, pub i_ack_wr: ib_send_wr, pub i_ack_sge: ib_sge,
    pub i_ack_dma: dma_addr_t, pub i_ack_queued: c_ulong, pub i_credits: atomic_t,
    pub i_flowctl: u32, pub i_unsignaled_wrs: u32, pub i_active_side: bool, pub i_cq_quiesce: atomic_t,
    pub i_scq_vector: c_int, pub i_rcq_vector: c_int, pub i_sl: u8,
}

pub const IB_ACK_IN_FLIGHT: u32 = 0;
pub const IB_ACK_REQUESTED: u32 = 1;
pub const RDS_IB_ACK_WR_ID: u64 = !0u64;

#[repr(C)] pub struct rds_ib_ipaddr { pub list: list_head, pub ipaddr: __be32, pub rcu: rcu_head }
pub const RDS_IB_MR_8K_POOL: u32 = 0;
pub const RDS_IB_MR_1M_POOL: u32 = 1;

#[repr(C)] pub struct rds_ib_device {
    pub list: list_head, pub ipaddr_list: list_head, pub conn_list: list_head, pub dev: *mut ib_device, pub pd: *mut ib_pd,
    pub odp_capable: u8, pub max_mrs: c_uint, pub mr_1m_pool: *mut rds_ib_mr_pool, pub mr_8k_pool: *mut rds_ib_mr_pool,
    pub max_8k_mrs: c_uint, pub max_1m_mrs: c_uint, pub max_sge: c_int, pub max_wrs: c_uint,
    pub max_initiator_depth: c_uint, pub max_responder_resources: c_uint, pub spinlock: spinlock_t,
    pub refcount: refcount_t, pub free_work: work_struct, pub vector_load: *mut c_int,
}

#[repr(C)] pub struct rds_ib_statistics { pub values: [u64; 40] }

extern "C" {
    pub static mut rds_ib_wq: *mut workqueue_struct;
    pub static mut rds_ib_transport: rds_transport;
    pub fn rds_ib_get_client_data(device: *mut ib_device) -> *mut rds_ib_device;
    pub fn rds_ib_dev_put(rds_ibdev: *mut rds_ib_device);
    pub static mut rds_ib_client: ib_client;
    pub static mut rds_ib_retry_count: c_uint;
    pub static mut ib_nodev_conns_lock: spinlock_t;
    pub static mut ib_nodev_conns: list_head;
}

pub unsafe fn rds_ib_dma_sync_sg_for_cpu(dev: *mut ib_device, sglist: *mut scatterlist, sg_dma_len: c_uint, direction: c_int) {
    let mut sg = sglist; for _ in 0..sg_dma_len { ib_dma_sync_single_for_cpu(dev, sg_dma_address(sg), sg_dma_len_one(sg), direction); sg = (*sg).next; }
}
pub unsafe fn rds_ib_dma_sync_sg_for_device(dev: *mut ib_device, sglist: *mut scatterlist, sg_dma_len: c_uint, direction: c_int) {
    let mut sg = sglist; for _ in 0..sg_dma_len { ib_dma_sync_single_for_device(dev, sg_dma_address(sg), sg_dma_len_one(sg), direction); sg = (*sg).next; }
}

extern "C" {
    pub fn rds_ib_conn_alloc(conn: *mut rds_connection, gfp: gfp_t) -> c_int;
    pub fn rds_ib_conn_free(arg: *mut c_void);
    pub fn rds_ib_conn_path_connect(cp: *mut rds_conn_path) -> c_int;
    pub fn rds_ib_conn_path_shutdown(cp: *mut rds_conn_path);
    pub fn __rds_ib_conn_error(conn: *mut rds_connection, fmt: *const c_char, ...);
    pub fn rds_ib_cm_handle_connect(cm_id: *mut rdma_cm_id, event: *mut rdma_cm_event, isv6: bool) -> c_int;
    pub fn rds_ib_cm_initiate_connect(cm_id: *mut rdma_cm_id, isv6: bool) -> c_int;
    pub fn rds_ib_cm_connect_complete(conn: *mut rds_connection, event: *mut rdma_cm_event);
    pub fn rds_ib_get_device(ipaddr: __be32) -> *mut rds_ib_device;
    pub fn rds_ib_update_ipaddr(dev: *mut rds_ib_device, ipaddr: *mut in6_addr) -> c_int;
    pub fn rds_ib_add_conn(dev: *mut rds_ib_device, conn: *mut rds_connection);
    pub fn rds_ib_remove_conn(dev: *mut rds_ib_device, conn: *mut rds_connection);
    pub fn rds_ib_destroy_nodev_conns();
    pub fn rds_ib_mr_cqe_handler(ic: *mut rds_ib_connection, wc: *mut ib_wc);
    pub fn rds_ib_recv_init() -> c_int; pub fn rds_ib_recv_exit();
    pub fn rds_ib_recv_path(conn: *mut rds_conn_path) -> c_int;
    pub fn rds_ib_recv_alloc_caches(ic: *mut rds_ib_connection, gfp: gfp_t) -> c_int;
    pub fn rds_ib_recv_free_caches(ic: *mut rds_ib_connection);
    pub fn rds_ib_recv_refill(conn: *mut rds_connection, prefill: c_int, gfp: gfp_t);
    pub fn rds_ib_inc_free(inc: *mut rds_incoming);
    pub fn rds_ib_inc_copy_to_user(inc: *mut rds_incoming, to: *mut iov_iter) -> c_int;
    pub fn rds_ib_recv_cqe_handler(ic: *mut rds_ib_connection, wc: *mut ib_wc, state: *mut rds_ib_ack_state);
    pub fn rds_ib_recv_init_ring(ic: *mut rds_ib_connection); pub fn rds_ib_recv_clear_ring(ic: *mut rds_ib_connection);
    pub fn rds_ib_recv_init_ack(ic: *mut rds_ib_connection); pub fn rds_ib_attempt_ack(ic: *mut rds_ib_connection);
    pub fn rds_ib_ack_send_complete(ic: *mut rds_ib_connection); pub fn rds_ib_piggyb_ack(ic: *mut rds_ib_connection) -> u64;
    pub fn rds_ib_set_ack(ic: *mut rds_ib_connection, seq: u64, ack_required: c_int);
    pub fn rds_ib_ring_init(ring: *mut rds_ib_work_ring, nr: u32); pub fn rds_ib_ring_resize(ring: *mut rds_ib_work_ring, nr: u32);
    pub fn rds_ib_ring_alloc(ring: *mut rds_ib_work_ring, val: u32, pos: *mut u32) -> u32;
    pub fn rds_ib_ring_free(ring: *mut rds_ib_work_ring, val: u32); pub fn rds_ib_ring_unalloc(ring: *mut rds_ib_work_ring, val: u32);
    pub fn rds_ib_ring_empty(ring: *mut rds_ib_work_ring) -> c_int; pub fn rds_ib_ring_low(ring: *mut rds_ib_work_ring) -> c_int;
    pub fn rds_ib_ring_oldest(ring: *mut rds_ib_work_ring) -> u32; pub fn rds_ib_ring_completed(ring: *mut rds_ib_work_ring, wr_id: u32, oldest: u32) -> u32;
    pub fn rds_ib_xmit_path_complete(cp: *mut rds_conn_path); pub fn rds_ib_xmit(conn: *mut rds_connection, rm: *mut rds_message, hdr_off: c_uint, sg: c_uint, off: c_uint) -> c_int;
    pub fn rds_ib_send_cqe_handler(ic: *mut rds_ib_connection, wc: *mut ib_wc); pub fn rds_ib_send_init_ring(ic: *mut rds_ib_connection); pub fn rds_ib_send_clear_ring(ic: *mut rds_ib_connection);
    pub fn rds_ib_xmit_rdma(conn: *mut rds_connection, op: *mut rm_rdma_op) -> c_int; pub fn rds_ib_send_add_credits(conn: *mut rds_connection, credits: c_uint);
    pub fn rds_ib_advertise_credits(conn: *mut rds_connection, posted: c_uint); pub fn rds_ib_xmit_atomic(conn: *mut rds_connection, op: *mut rm_atomic_op) -> c_int;
    pub fn rds_ib_send_grab_credits(ic: *mut rds_ib_connection, wanted: u32, adv_credits: *mut u32, need_posted: c_int, max_posted: c_int) -> c_int;
    pub fn rds_ib_stats_info_copy(iter: *mut rds_info_iterator, avail: c_uint) -> c_uint;
    pub fn rds_ib_sysctl_init() -> c_int; pub fn rds_ib_sysctl_exit();
    pub static mut rds_ib_sysctl_max_send_wr: c_ulong; pub static mut rds_ib_sysctl_max_recv_wr: c_ulong;
    pub static mut rds_ib_sysctl_max_unsig_wrs: c_ulong; pub static mut rds_ib_sysctl_max_unsig_bytes: c_ulong;
    pub static mut rds_ib_sysctl_max_recv_allocation: c_ulong; pub static mut rds_ib_sysctl_flow_control: c_uint;
}

// C field-access aliases: dp_protocol_major, dp_protocol_minor,
// dp_protocol_minor_mask, dp_ack_seq, and dp_credit designate the corresponding
// fields of dp_cmn in the two connection-private structures.
// rdsibdev_to_node(rdsibdev) expands to ibdev_to_node(rdsibdev->dev).
// rds_ib_conn_error(conn, fmt...) expands to __rds_ib_conn_error(conn, ...).

// The following aliases preserve the original credit macros.
#[inline] pub const fn IB_GET_SEND_CREDITS(v: u32) -> u32 { v & 0xffff }
#[inline] pub const fn IB_GET_POST_CREDITS(v: u32) -> u32 { v >> 16 }
#[inline] pub const fn IB_SET_SEND_CREDITS(v: u32) -> u32 { v & 0xffff }
#[inline] pub const fn IB_SET_POST_CREDITS(v: u32) -> u32 { v << 16 }

// External kernel/RDS types and helper functions referenced above are supplied by other translated units.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
