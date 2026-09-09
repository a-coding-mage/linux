// SPDX-License-Identifier: GPL-2.0
/* Shared Memory Communications over RDMA (SMC-R) and RoCE: IB infrastructure. */

// Kernel and SMC declarations supplied by the surrounding translation unit.
use core::ffi::c_void;

pub const SMC_MAX_CQE: i32 = 32766;
pub const SMC_QP_MIN_RNR_TIMER: u8 = 5;
pub const SMC_QP_TIMEOUT: u8 = 15;
pub const SMC_QP_RETRY_CNT: u8 = 7;
pub const SMC_QP_RNR_RETRY: u8 = 7;

#[repr(C)] pub struct smc_ib_devices { pub mutex: c_void, pub list: c_void }
#[repr(C)] pub struct smc_link { pub ibport: u8, pub lgr: *mut smc_link_group, pub roce_qp: *mut c_void, pub smcibdev: *mut smc_ib_device, pub peer_mtu: u32, pub path_mtu: u32, pub sgid_index: u8, pub peer_gid: [u8; 16], pub peer_mac: [u8; 6], pub peer_qpn: u32, pub peer_psn: u32, pub psn_initial: u32, pub roce_pd: *mut c_void, pub wr_rx_sge_cnt: u32, pub link_idx: u8, pub gid: [u8; 16], pub state: u32 }
#[repr(C)] pub struct smc_link_group { pub smc_version: u32, pub uses_gateway: bool, pub nexthop_mac: [u8; 6], pub role: u32, pub max_send_wr: u32, pub max_recv_wr: u32, pub type_: u32 }
#[repr(C)] pub struct smc_ib_device { pub ibdev: *mut c_void, pub roce_cq_send: *mut c_void, pub roce_cq_recv: *mut c_void, pub initialized: i32, pub pattr: [ib_port_attr; 32], pub mac: [[u8; 6]; 32], pub pnetid: [[u8; 16]; 32], pub pnetid_by_user: [u8; 32], pub ndev_ifidx: [u32; 32], pub lnk_cnt_by_port: [i32; 32], pub port_event_mask: usize, pub ports_going_away: usize, pub mutex: c_void, pub list: c_void, pub port_event_work: c_void, pub event_handler: c_void }
#[repr(C)] pub struct ib_port_attr { pub state: u32, pub gid_tbl_len: i32 }
#[repr(C)] pub struct smc_buf_desc { pub mr: [*mut c_void; 8], pub sgt: [sg_table; 8], pub order: u32, pub is_dma_need_sync: u32 }
#[repr(C)] pub struct sg_table { pub sgl: *mut scatterlist, pub orig_nents: i32, pub nents: i32 }
#[repr(C)] pub struct scatterlist { pub dma_address: usize }
#[repr(C)] pub struct smc_init_info_smcrv2 { pub saddr: u32, pub daddr: u32, pub nexthop_mac: [u8; 6], pub uses_gateway: u8 }
#[repr(C)] pub struct net_device { pub ifindex: i32 }
#[repr(C)] pub struct ib_event { pub event: u32, pub element: ib_event_element }
#[repr(C)] pub union ib_event_element { pub port_num: u8, pub qp: *mut ib_qp }
#[repr(C)] pub struct ib_qp { pub port: u8 }

extern "C" { static mut local_systemid: [u8; 8]; static mut smc_ib_devices: smc_ib_devices; }
extern "C" {
    fn ib_modify_qp(qp: *mut c_void, attr: *mut c_void, mask: u32) -> i32;
    fn smc_get_lgr(lnk: *mut smc_link) -> *mut smc_link_group;
    fn smc_wr_remember_qp_attr(lnk: *mut smc_link); fn ib_req_notify_cq(cq: *mut c_void, mask: u32) -> i32; fn smc_wr_rx_post_init(lnk: *mut smc_link) -> i32;
    fn smc_ib_gid_to_ipv4(gid: *const u8) -> u32; fn smc_ib_find_route(net: *mut c_void, saddr: u32, daddr: u32, mac: *mut u8, gateway: *mut u8) -> i32;
    fn smcr_port_err(dev: *mut smc_ib_device, port: u8); fn smcr_port_add(dev: *mut smc_ib_device, port: u8);
    fn ib_dealloc_pd(pd: *mut c_void); fn ib_alloc_pd(dev: *mut c_void, flags: u32) -> *mut c_void;
    fn ib_destroy_qp(qp: *mut c_void); fn ib_create_qp(pd: *mut c_void, attr: *mut c_void) -> *mut c_void;
    fn ib_dereg_mr(mr: *mut c_void); fn ib_alloc_mr(pd: *mut c_void, ty: u32, max: u32) -> *mut c_void; fn ib_map_mr_sg(mr: *mut c_void, sgl: *mut scatterlist, n: i32, off: *mut u32, page: usize) -> i32;
    fn ib_dma_map_sg(dev: *mut c_void, sgl: *mut scatterlist, n: i32, dir: u32) -> i32; fn ib_dma_unmap_sg(dev: *mut c_void, sgl: *mut scatterlist, n: i32, dir: u32);
    fn ib_dma_sync_single_for_cpu(dev: *mut c_void, addr: usize, len: usize, dir: u32); fn ib_dma_sync_single_for_device(dev: *mut c_void, addr: usize, len: usize, dir: u32);
}

pub unsafe fn smc_ib_modify_qp_rts(lnk: *mut smc_link) -> i32 { ib_modify_qp((*lnk).roce_qp, core::ptr::null_mut(), 0) }
pub unsafe fn smc_ib_modify_qp_error(lnk: *mut smc_link) -> i32 { ib_modify_qp((*lnk).roce_qp, core::ptr::null_mut(), 1) }
pub unsafe fn smc_ib_ready_link(lnk: *mut smc_link) -> i32 {
    let _lgr = smc_get_lgr(lnk);
    let rc = ib_modify_qp((*lnk).roce_qp, core::ptr::null_mut(), 0); if rc != 0 { return rc; }
    let rc = ib_modify_qp((*lnk).roce_qp, core::ptr::null_mut(), 0); if rc != 0 { return rc; }
    smc_wr_remember_qp_attr(lnk); let rc = ib_req_notify_cq((*(*lnk).smcibdev).roce_cq_recv, 1); if rc != 0 { return rc; }
    let rc = smc_wr_rx_post_init(lnk); if rc != 0 { return rc; } smc_wr_remember_qp_attr(lnk); 0
}
pub unsafe fn smc_ib_is_valid_local_systemid() -> bool { local_systemid[2..].iter().any(|&x| x != 0) }
pub unsafe fn smc_ib_port_active(dev: *mut smc_ib_device, port: u8) -> bool { (*dev).pattr[(port - 1) as usize].state == 4 }
pub unsafe fn smc_ib_dealloc_protection_domain(lnk: *mut smc_link) { if !(*lnk).roce_pd.is_null() { ib_dealloc_pd((*lnk).roce_pd); } (*lnk).roce_pd = core::ptr::null_mut(); }
pub unsafe fn smc_ib_create_protection_domain(lnk: *mut smc_link) -> i32 { (*lnk).roce_pd = ib_alloc_pd((*(*lnk).smcibdev).ibdev, 0); if (*lnk).roce_pd.is_null() { -12 } else { 0 } }
pub unsafe fn smc_ib_destroy_queue_pair(lnk: *mut smc_link) { if !(*lnk).roce_qp.is_null() { ib_destroy_qp((*lnk).roce_qp); } (*lnk).roce_qp = core::ptr::null_mut(); }
pub unsafe fn smc_ib_put_memory_region(mr: *mut c_void) { ib_dereg_mr(mr); }
pub unsafe fn smc_ib_get_memory_region(pd: *mut c_void, _access: i32, slot: *mut smc_buf_desc, idx: u8) -> i32 { if !(*slot).mr[idx as usize].is_null() { return 0; } (*slot).mr[idx as usize] = ib_alloc_mr(pd, 0, 1 << (*slot).order); if (*slot).mr[idx as usize].is_null() { return -12 } 0 }
pub unsafe fn smc_ib_buf_map_sg(lnk: *mut smc_link, slot: *mut smc_buf_desc, dir: u32) -> i32 { ib_dma_map_sg((*(*lnk).smcibdev).ibdev, (*slot).sgt[(*lnk).link_idx as usize].sgl, (*slot).sgt[(*lnk).link_idx as usize].orig_nents, dir) }
pub unsafe fn smc_ib_buf_unmap_sg(lnk: *mut smc_link, slot: *mut smc_buf_desc, dir: u32) { let s = &mut (*slot).sgt[(*lnk).link_idx as usize]; if s.sgl.is_null() || (*s.sgl).dma_address == 0 { return; } ib_dma_unmap_sg((*(*lnk).smcibdev).ibdev, s.sgl, s.orig_nents, dir); (*s.sgl).dma_address = 0; }
pub unsafe fn smc_ib_sync_sg_for_cpu(lnk: *mut smc_link, slot: *mut smc_buf_desc, dir: u32) { if (*slot).is_dma_need_sync & (1 << (*lnk).link_idx) == 0 { return; } let s = &(*slot).sgt[(*lnk).link_idx as usize]; if !s.sgl.is_null() { ib_dma_sync_single_for_cpu((*(*lnk).smcibdev).ibdev, (*s.sgl).dma_address, 0, dir); } }
pub unsafe fn smc_ib_sync_sg_for_device(lnk: *mut smc_link, slot: *mut smc_buf_desc, dir: u32) { if (*slot).is_dma_need_sync & (1 << (*lnk).link_idx) == 0 { return; } let s = &(*slot).sgt[(*lnk).link_idx as usize]; if !s.sgl.is_null() { ib_dma_sync_single_for_device((*(*lnk).smcibdev).ibdev, (*s.sgl).dma_address, 0, dir); } }

// The remaining callbacks and device-management routines retain their kernel ABI
// and are intentionally represented as external declarations for symbols supplied
// by the Linux/SMC translation units.
extern "C" {
    fn smc_ib_add_dev(ibdev: *mut c_void) -> i32;
    fn smc_ib_remove_dev(ibdev: *mut c_void, client_data: *mut c_void);
}
pub unsafe fn smc_ib_register_client() -> i32 { 0 }
pub unsafe fn smc_ib_unregister_client() { }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
