/* SPDX-License-Identifier: GPL-2.0 */
/* Rust translation of smc_core.h. Kernel/RDMA types are supplied externally. */

pub const SMC_RMBS_PER_LGR_MAX: usize = 255;
pub const SMC_CONN_PER_LGR_MIN: usize = 16;
pub const SMC_CONN_PER_LGR_MAX: usize = 255;
pub const SMC_CONN_PER_LGR_PREFER: usize = 255;
pub const SMCR_MAX_SEND_WR_DEF: usize = 16;
pub const SMCR_MAX_RECV_WR_DEF: usize = 48;

#[repr(C)] pub struct smc_lgr_list { pub list: list_head, pub lock: spinlock_t, pub num: u32 }
#[repr(C)] pub enum smc_lgr_role { SMC_CLNT, SMC_SERV }
#[repr(C)] pub enum smc_link_state { SMC_LNK_UNUSED, SMC_LNK_INACTIVE, SMC_LNK_ACTIVATING, SMC_LNK_ACTIVE }

pub const SMC_WR_BUF_SIZE: usize = 48;
pub const SMC_WR_BUF_V2_SIZE: usize = 8192;
#[repr(C)] pub struct smc_wr_buf { pub raw: [u8; SMC_WR_BUF_SIZE] }
#[repr(C)] pub struct smc_wr_v2_buf { pub raw: [u8; SMC_WR_BUF_V2_SIZE] }
pub const SMC_WR_REG_MR_WAIT_TIME: usize = 5 * HZ;
#[repr(C)] pub enum smc_wr_reg_state { POSTED, CONFIRMED, FAILED }
#[repr(C)] pub struct smc_rdma_sge { pub wr_tx_rdma_sge: [ib_sge; SMC_IB_MAX_SEND_SGE] }
pub const SMC_MAX_RDMA_WRITES: usize = 2;
#[repr(C)] pub struct smc_rdma_sges { pub tx_rdma_sge: [smc_rdma_sge; SMC_MAX_RDMA_WRITES] }
#[repr(C)] pub struct smc_rdma_wr { pub wr_tx_rdma: [ib_rdma_wr; SMC_MAX_RDMA_WRITES] }
pub const SMC_LGR_ID_SIZE: usize = 4;

#[repr(C)] pub struct smc_link {
    pub smcibdev: *mut smc_ib_device, pub ibport: u8, pub roce_pd: *mut ib_pd, pub roce_qp: *mut ib_qp,
    pub qp_attr: ib_qp_attr, pub wr_tx_bufs: *mut smc_wr_buf, pub wr_tx_ibs: *mut ib_send_wr,
    pub wr_tx_sges: *mut ib_sge, pub wr_tx_rdma_sges: *mut smc_rdma_sges, pub wr_tx_rdmas: *mut smc_rdma_wr,
    pub wr_tx_pends: *mut smc_wr_tx_pend, pub wr_tx_compl: *mut completion, pub wr_tx_v2_ib: *mut ib_send_wr,
    pub wr_tx_v2_sge: *mut ib_sge, pub wr_tx_v2_pend: *mut smc_wr_tx_pend, pub wr_tx_dma_addr: dma_addr_t,
    pub wr_tx_v2_dma_addr: dma_addr_t, pub wr_tx_id: atomic_long_t, pub wr_tx_mask: *mut c_ulong,
    pub wr_tx_cnt: u32, pub wr_tx_wait: wait_queue_head_t, pub wr_tx_refs: percpu_ref, pub tx_ref_comp: completion,
    pub wr_rx_bufs: *mut u8, pub wr_rx_ibs: *mut ib_recv_wr, pub wr_rx_sges: *mut ib_sge, pub wr_rx_sge_cnt: i32,
    pub wr_rx_buflen: i32, pub wr_rx_dma_addr: dma_addr_t, pub wr_rx_v2_dma_addr: dma_addr_t, pub wr_rx_id: u64,
    pub wr_rx_id_compl: u64, pub wr_rx_cnt: u32, pub wr_rx_tstamp: c_ulong, pub wr_rx_empty_wait: wait_queue_head_t,
    pub wr_reg: ib_reg_wr, pub wr_reg_wait: wait_queue_head_t, pub wr_reg_refs: percpu_ref, pub reg_ref_comp: completion,
    pub wr_reg_state: smc_wr_reg_state, pub gid: [u8; SMC_GID_SIZE], pub sgid_index: u8, pub peer_qpn: u32,
    pub path_mtu: ib_mtu, pub peer_mtu: ib_mtu, pub psn_initial: u32, pub peer_psn: u32,
    pub peer_mac: [u8; ETH_ALEN], pub peer_gid: [u8; SMC_GID_SIZE], pub link_id: u8,
    pub link_uid: [u8; SMC_LGR_ID_SIZE], pub peer_link_uid: [u8; SMC_LGR_ID_SIZE], pub link_idx: u8,
    pub link_is_asym: u8, pub clearing: u8, pub refcnt: refcount_t, pub lgr: *mut smc_link_group,
    pub link_down_wrk: work_struct, pub ibname: [c_char; IB_DEVICE_NAME_MAX], pub ndev_ifidx: i32,
    pub state: smc_link_state, pub llc_testlink_wrk: delayed_work, pub llc_testlink_resp: completion,
    pub llc_testlink_time: i32, pub conn_cnt: atomic_t, pub max_send_wr: u16, pub max_recv_wr: u16,
}

pub const SMC_LINKS_PER_LGR_MAX: usize = 3;
pub const SMC_SINGLE_LINK: usize = 0;
pub const SMC_LINKS_ADD_LNK_MIN: usize = 1;
pub const SMC_LINKS_ADD_LNK_MAX: usize = 2;
pub const SMC_LINKS_PER_LGR_MAX_PREFER: usize = 2;

#[repr(C)] pub struct smc_buf_desc {
    pub list: list_head, pub cpu_addr: *mut c_void, pub pages: *mut page, pub len: i32, pub used: u32,
    pub smcr: smc_buf_desc_smcr, pub smcd: smc_buf_desc_smcd,
}
#[repr(C)] pub struct smc_buf_desc_smcr { pub sgt: [sg_table; SMC_LINKS_PER_LGR_MAX], pub mr: [*mut ib_mr; SMC_LINKS_PER_LGR_MAX], pub order: u32, pub is_conf_rkey: u8, pub is_reg_mr: [u8; SMC_LINKS_PER_LGR_MAX], pub is_map_ib: [u8; SMC_LINKS_PER_LGR_MAX], pub is_dma_need_sync: u8, pub is_reg_err: u8, pub is_vm: u8 }
#[repr(C)] pub struct smc_buf_desc_smcd { pub is_attached: bool, pub sba_idx: u16, pub token: u64, pub dma_addr: dma_addr_t }
#[repr(C)] pub struct smc_rtoken { pub dma_addr: u64, pub rkey: u32 }
pub const SMC_BUF_MIN_SIZE: usize = 16384;
pub const SMC_RMBE_SIZES: usize = 16;
pub enum smc_lgr_type { SMC_LGR_NONE, SMC_LGR_SINGLE, SMC_LGR_SYMMETRIC, SMC_LGR_ASYMMETRIC_PEER, SMC_LGR_ASYMMETRIC_LOCAL }
pub enum smcr_buf_type { SMCR_PHYS_CONT_BUFS = 0, SMCR_VIRT_CONT_BUFS = 1, SMCR_MIXED_BUFS = 2 }
pub enum smc_llc_flowtype { SMC_LLC_FLOW_NONE = 0, SMC_LLC_FLOW_ADD_LINK = 2, SMC_LLC_FLOW_DEL_LINK = 4, SMC_LLC_FLOW_REQ_ADD_LINK = 5, SMC_LLC_FLOW_RKEY = 6 }
pub struct smcd_dev;
pub struct smc_llc_qentry;
#[repr(C)] pub struct smc_llc_flow { pub r#type: smc_llc_flowtype, pub qentry: *mut smc_llc_qentry }

/* The remaining aggregate contains kernel-owned synchronization and protocol fields. */
#[repr(C)] pub struct smc_link_group {
    pub list: list_head, pub conns_all: rb_root, pub conns_lock: rwlock_t, pub conns_num: c_uint, pub vlan_id: c_ushort,
    pub sndbufs: [list_head; SMC_RMBE_SIZES], pub sndbufs_lock: rw_semaphore, pub rmbs: [list_head; SMC_RMBE_SIZES], pub rmbs_lock: rw_semaphore,
    pub alloc_sndbufs: u64, pub alloc_rmbs: u64, pub id: [u8; SMC_LGR_ID_SIZE], pub free_work: delayed_work, pub terminate_work: work_struct,
    pub tx_wq: *mut workqueue_struct, pub sync_err: u8, pub terminating: u8, pub freeing: u8, pub refcnt: refcount_t, pub is_smcd: bool,
    pub smc_version: u8, pub negotiated_eid: [u8; SMC_MAX_EID_LEN], pub peer_os: u8, pub peer_smc_release: u8, pub peer_hostname: [u8; SMC_MAX_HOSTNAME_LEN],
}
#[repr(C)] pub struct smc_rtoken_placeholder;
pub struct smc_clc_msg_local;
pub const GID_LIST_SIZE: usize = 2;
#[repr(C)] pub struct smc_gidlist { pub len: u8, pub list: [[u8; SMC_GID_SIZE]; GID_LIST_SIZE] }
#[repr(C)] pub struct smc_init_info_smcrv2 { pub saddr: __be32, pub clc_sk: *mut sock, pub daddr: __be32, pub ib_dev_v2: *mut smc_ib_device, pub ib_port_v2: u8, pub ib_gid_v2: [u8; SMC_GID_SIZE], pub uses_gateway: u8, pub nexthop_mac: [u8; ETH_ALEN], pub gidlist: smc_gidlist }
pub const SMC_MAX_V2_ISM_DEVS: usize = SMCD_CLC_MAX_V2_GID_ENTRIES;
#[repr(C)] pub struct smc_init_info { pub is_smcd: u8, pub smc_type_v1: u8, pub smc_type_v2: u8, pub release_nr: u8, pub max_conns: u8, pub max_links: u8, pub first_contact_peer: u8, pub first_contact_local: u8, pub feature_mask: u16, pub vlan_id: c_ushort, pub rc: u32, pub negotiated_eid: [u8; SMC_MAX_EID_LEN], pub smcr_version: u8, pub check_smcrv2: u8, pub peer_gid: [u8; SMC_GID_SIZE], pub peer_mac: [u8; ETH_ALEN], pub peer_systemid: [u8; SMC_SYSTEMID_LEN], pub ib_dev: *mut smc_ib_device, pub ib_gid: [u8; SMC_GID_SIZE], pub ib_port: u8, pub ib_clcqpn: u32, pub smcrv2: smc_init_info_smcrv2, pub ism_peer_gid: [smcd_gid; SMC_MAX_V2_ISM_DEVS + 1], pub ism_dev: [*mut smcd_dev; SMC_MAX_V2_ISM_DEVS + 1], pub ism_chid: [u16; SMC_MAX_V2_ISM_DEVS + 1], pub ism_offered_cnt: u8, pub ism_selected: u8, pub smcd_version: u8 }

pub unsafe fn smc_conn_lgr_valid(conn: *mut smc_connection) -> bool { !(*conn).lgr.is_null() && (*conn).alert_token_local != 0 }
pub unsafe fn smc_link_usable(lnk: *mut smc_link) -> bool { (*lnk).state != smc_link_state::SMC_LNK_UNUSED && (*lnk).state != smc_link_state::SMC_LNK_INACTIVE }
pub unsafe fn smc_link_sendable(lnk: *mut smc_link) -> bool { smc_link_usable(lnk) && (*lnk).qp_attr.cur_qp_state == IB_QPS_RTS }
pub unsafe fn smc_link_active(lnk: *mut smc_link) -> bool { (*lnk).state == smc_link_state::SMC_LNK_ACTIVE }
pub unsafe fn smc_link_shared_v2_rxbuf(lnk: *mut smc_link) -> bool { (*lnk).wr_rx_sge_cnt > 1 }
pub unsafe fn smc_get_lgr(link: *mut smc_link) -> *mut smc_link_group { (*link).lgr }

#[repr(C)] pub struct smc_pci_dev { pub pci_fid: u32, pub pci_pchid: u16, pub pci_vendor: u16, pub pci_device: u16, pub pci_id: [u8; SMC_PCI_ID_STR_LEN] }
pub unsafe fn smc_set_pci_values(pci_dev: *mut pci_dev, smc_dev: *mut smc_pci_dev) {
    (*smc_dev).pci_vendor = (*pci_dev).vendor; (*smc_dev).pci_device = (*pci_dev).device;
    /* snprintf(pci_id, sizeof(pci_id), "%s", pci_name(pci_dev)); */
}

extern "C" {
    pub fn smc_conn_free(conn: *mut smc_connection); pub fn smc_conn_create(smc: *mut smc_sock, ini: *mut smc_init_info) -> i32;
    pub fn smcr_link_init(lgr: *mut smc_link_group, lnk: *mut smc_link, link_idx: u8, ini: *mut smc_init_info) -> i32;
    pub fn smcr_link_clear(lnk: *mut smc_link, log: bool); pub fn smcr_link_hold(lnk: *mut smc_link); pub fn smcr_link_put(lnk: *mut smc_link);
    pub fn smc_switch_link_and_count(conn: *mut smc_connection, to_lnk: *mut smc_link);
    pub fn smcr_buf_map_lgr(lnk: *mut smc_link) -> i32; pub fn smcr_buf_reg_lgr(lnk: *mut smc_link) -> i32;
    pub fn smcr_lgr_set_type(lgr: *mut smc_link_group, new_type: smc_lgr_type); pub fn smcr_link_reg_buf(link: *mut smc_link, desc: *mut smc_buf_desc) -> i32;
    pub fn smcr_link_down_cond(lnk: *mut smc_link); pub fn smcr_link_down_cond_sched(lnk: *mut smc_link);
    pub fn smc_sndbuf_sync_sg_for_device(conn: *mut smc_connection); pub fn smc_rmb_sync_sg_for_cpu(conn: *mut smc_connection);
}

extern "C" {
    pub fn smc_lgr_cleanup_early(lgr: *mut smc_link_group); pub fn smc_lgr_terminate_sched(lgr: *mut smc_link_group); pub fn smc_lgr_hold(lgr: *mut smc_link_group); pub fn smc_lgr_put(lgr: *mut smc_link_group);
    pub fn smcr_port_add(dev: *mut smc_ib_device, port: u8); pub fn smcr_port_err(dev: *mut smc_ib_device, port: u8); pub fn smc_smcd_terminate(dev: *mut smcd_dev, gid: *mut smcd_gid, vlan: c_ushort); pub fn smc_smcd_terminate_all(dev: *mut smcd_dev); pub fn smc_smcr_terminate_all(dev: *mut smc_ib_device);
    pub fn smc_buf_create(smc: *mut smc_sock, is_smcd: bool) -> i32; pub fn smcd_buf_attach(smc: *mut smc_sock) -> i32; pub fn smc_uncompress_bufsize(compressed: u8) -> i32; pub fn smc_core_init() -> i32; pub fn smc_core_exit();
}
pub struct smc_sock;
#[repr(C)] pub struct smc_connection { pub lgr: *mut smc_link_group, pub alert_token_local: u32 }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
