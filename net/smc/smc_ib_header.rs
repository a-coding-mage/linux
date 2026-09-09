/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Shared Memory Communications over RDMA (SMC-R) and RoCE
 *
 * Definitions for IB environment
 *
 * Copyright IBM Corp. 2016
 *
 * Author(s): Ursula Braun <Ursula Braun@linux.vnet.ibm.com>
 */

// C dependencies supplied by the surrounding kernel translation.

pub const SMC_MAX_PORTS: usize = 2; /* Max # of ports */
pub const SMC_GID_SIZE: usize = core::mem::size_of::<union_ib_gid>();
pub const SMC_IB_MAX_SEND_SGE: usize = 2;

#[repr(C)]
pub struct smc_ib_devices {
    pub list: list_head,
    pub mutex: mutex,
}

extern "C" {
    pub static mut smc_ib_devices: smc_ib_devices;
    pub static mut smc_lgr_list: smc_lgr_list;
}

#[repr(C)]
pub struct smc_ib_device {
    pub list: list_head,
    pub ibdev: *mut ib_device,
    pub pattr: [ib_port_attr; SMC_MAX_PORTS],
    pub event_handler: ib_event_handler,
    pub roce_cq_send: *mut ib_cq,
    pub roce_cq_recv: *mut ib_cq,
    pub send_tasklet: tasklet_struct,
    pub recv_tasklet: tasklet_struct,
    pub mac: [[i8; ETH_ALEN]; SMC_MAX_PORTS],
    pub pnetid: [[u8; SMC_MAX_PNETID_LEN]; SMC_MAX_PORTS],
    pub pnetid_by_user: [bool; SMC_MAX_PORTS],
    pub initialized: u8,
    pub port_event_work: work_struct,
    pub port_event_mask: usize,
    pub ports_going_away: [core::ffi::c_ulong; 1],
    pub lnk_cnt: atomic_t,
    pub lnks_deleted: wait_queue_head_t,
    pub mutex: mutex,
    pub lnk_cnt_by_port: [atomic_t; SMC_MAX_PORTS],
    pub ndev_ifidx: [i32; SMC_MAX_PORTS],
}

pub unsafe fn smc_ib_gid_to_ipv4(gid: *mut u8) -> __be32 {
    let addr6 = gid as *mut in6_addr;
    if ipv6_addr_v4mapped(addr6) ||
        (((*addr6).s6_addr32[0] | (*addr6).s6_addr32[1] | (*addr6).s6_addr32[2]) == 0)
    {
        (*addr6).s6_addr32[3]
    } else {
        cpu_to_be32(INADDR_NONE)
    }
}

pub unsafe fn smc_ib_net(smcibdev: *mut smc_ib_device) -> *mut net {
    if !smcibdev.is_null() && !(*smcibdev).ibdev.is_null() {
        read_pnet(&mut (*(*smcibdev).ibdev).coredev.rdma_net)
    } else {
        core::ptr::null_mut()
    }
}

pub struct smc_init_info_smcrv2;
pub struct smc_buf_desc;
pub struct smc_link;

extern "C" {
    pub fn smc_ib_ndev_change(ndev: *mut net_device, event: usize);
    pub fn smc_ib_register_client() -> i32;
    pub fn smc_ib_unregister_client();
    pub fn smc_ib_port_active(smcibdev: *mut smc_ib_device, ibport: u8) -> bool;
    pub fn smc_ib_buf_map_sg(lnk: *mut smc_link, buf_slot: *mut smc_buf_desc,
                             data_direction: dma_data_direction) -> i32;
    pub fn smc_ib_buf_unmap_sg(lnk: *mut smc_link, buf_slot: *mut smc_buf_desc,
                               data_direction: dma_data_direction);
    pub fn smc_ib_dealloc_protection_domain(lnk: *mut smc_link);
    pub fn smc_ib_create_protection_domain(lnk: *mut smc_link) -> i32;
    pub fn smc_ib_destroy_queue_pair(lnk: *mut smc_link);
    pub fn smc_ib_create_queue_pair(lnk: *mut smc_link) -> i32;
    pub fn smc_ib_ready_link(lnk: *mut smc_link) -> i32;
    pub fn smc_ib_modify_qp_rts(lnk: *mut smc_link) -> i32;
    pub fn smc_ib_modify_qp_error(lnk: *mut smc_link) -> i32;
    pub fn smc_ib_setup_per_ibdev(smcibdev: *mut smc_ib_device) -> isize;
    pub fn smc_ib_get_memory_region(pd: *mut ib_pd, access_flags: i32,
                                    buf_slot: *mut smc_buf_desc, link_idx: u8) -> i32;
    pub fn smc_ib_put_memory_region(mr: *mut ib_mr);
    pub fn smc_ib_is_sg_need_sync(lnk: *mut smc_link, buf_slot: *mut smc_buf_desc) -> bool;
    pub fn smc_ib_sync_sg_for_cpu(lnk: *mut smc_link, buf_slot: *mut smc_buf_desc,
                                  data_direction: dma_data_direction);
    pub fn smc_ib_sync_sg_for_device(lnk: *mut smc_link, buf_slot: *mut smc_buf_desc,
                                     data_direction: dma_data_direction);
    pub fn smc_ib_determine_gid(smcibdev: *mut smc_ib_device, ibport: u8,
                                vlan_id: u16, gid: *mut u8, sgid_index: *mut u8,
                                smcrv2: *mut smc_init_info_smcrv2) -> i32;
    pub fn smc_ib_find_route(net: *mut net, saddr: __be32, daddr: __be32,
                             nexthop_mac: *mut u8, uses_gateway: *mut u8) -> i32;
    pub fn smc_ib_is_valid_local_systemid() -> bool;
    pub fn smcr_nl_get_device(skb: *mut sk_buff, cb: *mut netlink_callback) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
