/* SPDX-License-Identifier: GPL-2.0 OR Linux-OpenIB */
/* Translated from ib_addr.h. Included C headers provide the referenced types/constants. */

use core::ffi::c_void;

#[repr(C)]
pub struct rdma_dev_addr {
    pub src_dev_addr: [u8; MAX_ADDR_LEN],
    pub dst_dev_addr: [u8; MAX_ADDR_LEN],
    pub broadcast: [u8; MAX_ADDR_LEN],
    pub dev_type: u16,
    pub bound_dev_if: i32,
    pub transport: rdma_transport_type,
    pub net: *mut net,
    pub sgid_attr: *const ib_gid_attr,
    pub network: rdma_network_type,
    pub hoplimit: i32,
}

extern "C" {
    pub fn rdma_translate_ip(addr: *const sockaddr, dev_addr: *mut rdma_dev_addr) -> i32;
    pub fn rdma_resolve_ip(
        src_addr: *mut sockaddr,
        dst_addr: *const sockaddr,
        addr: *mut rdma_dev_addr,
        timeout_ms: c_ulong,
        callback: Option<unsafe extern "C" fn(i32, *mut sockaddr, *mut rdma_dev_addr, *mut c_void)>,
        resolve_by_gid_attr: bool,
        context: *mut c_void,
    ) -> i32;
    pub fn rdma_addr_cancel(addr: *mut rdma_dev_addr);
    pub fn rdma_addr_size(addr: *const sockaddr) -> i32;
    pub fn rdma_addr_size_in6(addr: *mut sockaddr_in6) -> i32;
    pub fn rdma_addr_size_kss(addr: *mut __kernel_sockaddr_storage) -> i32;
}

#[inline]
pub unsafe fn ib_addr_get_pkey(dev_addr: *mut rdma_dev_addr) -> u16 {
    ((*dev_addr).broadcast[8] as u16) << 8 | (*dev_addr).broadcast[9] as u16
}

#[inline]
pub unsafe fn ib_addr_set_pkey(dev_addr: *mut rdma_dev_addr, pkey: u16) {
    (*dev_addr).broadcast[8] = (pkey >> 8) as u8;
    (*dev_addr).broadcast[9] = pkey as u8;
}

#[inline]
pub unsafe fn ib_addr_get_mgid(dev_addr: *mut rdma_dev_addr, gid: *mut ib_gid) {
    core::ptr::copy_nonoverlapping((*dev_addr).broadcast.as_ptr().add(4), (*gid).raw.as_mut_ptr(), core::mem::size_of::<ib_gid>());
}

#[inline]
pub unsafe fn rdma_addr_gid_offset(dev_addr: *mut rdma_dev_addr) -> i32 {
    if (*dev_addr).dev_type as i32 == ARPHRD_INFINIBAND { 4 } else { 0 }
}

#[inline]
pub unsafe fn rdma_vlan_dev_vlan_id(dev: *const net_device) -> u16 {
    if is_vlan_dev(dev) { vlan_dev_vlan_id(dev) } else { 0xffff }
}

#[inline]
pub unsafe fn rdma_ip2gid(addr: *mut sockaddr, gid: *mut ib_gid) -> i32 {
    match (*addr).sa_family as i32 {
        AF_INET => {
            ipv6_addr_set_v4mapped((*(addr as *mut sockaddr_in)).sin_addr.s_addr, gid as *mut in6_addr);
        }
        AF_INET6 => {
            (*(gid).raw.as_mut_ptr() as *mut in6_addr).write((*(addr as *mut sockaddr_in6)).sin6_addr);
        }
        _ => return -EINVAL,
    }
    0
}

#[inline]
pub unsafe fn rdma_gid2ip(out: *mut sockaddr, gid: *const ib_gid) {
    if ipv6_addr_v4mapped(gid as *const in6_addr) {
        let out_in = out as *mut sockaddr_in;
        core::ptr::write_bytes(out_in, 0, 1);
        (*out_in).sin_family = AF_INET as _;
        core::ptr::copy_nonoverlapping((*gid).raw.as_ptr().add(12), &mut (*out_in).sin_addr.s_addr, 4);
    } else {
        let out_in = out as *mut sockaddr_in6;
        core::ptr::write_bytes(out_in, 0, 1);
        (*out_in).sin6_family = AF_INET6 as _;
        core::ptr::copy_nonoverlapping((*gid).raw.as_ptr(), (*out_in).sin6_addr.s6_addr.as_mut_ptr(), 16);
    }
}

#[inline]
pub unsafe fn rdma_addr_get_sgid(dev_addr: *mut rdma_dev_addr, gid: *mut ib_gid) {
    core::ptr::copy_nonoverlapping((*dev_addr).src_dev_addr.as_ptr().add(rdma_addr_gid_offset(dev_addr) as usize), (*gid).raw.as_mut_ptr(), core::mem::size_of::<ib_gid>());
}
#[inline]
pub unsafe fn rdma_addr_set_sgid(dev_addr: *mut rdma_dev_addr, gid: *mut ib_gid) {
    core::ptr::copy_nonoverlapping((*gid).raw.as_ptr(), (*dev_addr).src_dev_addr.as_mut_ptr().add(rdma_addr_gid_offset(dev_addr) as usize), core::mem::size_of::<ib_gid>());
}
#[inline]
pub unsafe fn rdma_addr_get_dgid(dev_addr: *mut rdma_dev_addr, gid: *mut ib_gid) {
    core::ptr::copy_nonoverlapping((*dev_addr).dst_dev_addr.as_ptr().add(rdma_addr_gid_offset(dev_addr) as usize), (*gid).raw.as_mut_ptr(), core::mem::size_of::<ib_gid>());
}
#[inline]
pub unsafe fn rdma_addr_set_dgid(dev_addr: *mut rdma_dev_addr, gid: *mut ib_gid) {
    core::ptr::copy_nonoverlapping((*gid).raw.as_ptr(), (*dev_addr).dst_dev_addr.as_mut_ptr().add(rdma_addr_gid_offset(dev_addr) as usize), core::mem::size_of::<ib_gid>());
}

#[inline]
pub fn iboe_get_mtu(mut mtu: i32) -> ib_mtu {
    mtu -= IB_GRH_BYTES + IB_UDP_BYTES + IB_BTH_BYTES + IB_EXT_XRC_BYTES + IB_EXT_ATOMICETH_BYTES + IB_ICRC_BYTES;
    if mtu >= ib_mtu_enum_to_int(IB_MTU_4096) { IB_MTU_4096 }
    else if mtu >= ib_mtu_enum_to_int(IB_MTU_2048) { IB_MTU_2048 }
    else if mtu >= ib_mtu_enum_to_int(IB_MTU_1024) { IB_MTU_1024 }
    else if mtu >= ib_mtu_enum_to_int(IB_MTU_512) { IB_MTU_512 }
    else if mtu >= ib_mtu_enum_to_int(IB_MTU_256) { IB_MTU_256 }
    else { 0 as ib_mtu }
}

#[inline]
pub unsafe fn rdma_link_local_addr(addr: *mut in6_addr) -> i32 {
    if (*addr).s6_addr32[0] == htonl(0xfe800000) && (*addr).s6_addr32[1] == 0 { 1 } else { 0 }
}
#[inline]
pub unsafe fn rdma_get_ll_mac(addr: *mut in6_addr, mac: *mut u8) {
    core::ptr::copy_nonoverlapping((*addr).s6_addr.as_ptr().add(8), mac, 3);
    core::ptr::copy_nonoverlapping((*addr).s6_addr.as_ptr().add(13), mac.add(3), 3);
    *mac ^= 2;
}
#[inline]
pub unsafe fn rdma_is_multicast_addr(addr: *mut in6_addr) -> i32 {
    if (*addr).s6_addr[0] == 0xff { return 1; }
    let ipv4_addr = (*addr).s6_addr32[3];
    if ipv6_addr_v4mapped(addr) && ipv4_is_multicast(ipv4_addr) { 1 } else { 0 }
}
#[inline]
pub unsafe fn rdma_get_mcast_mac(addr: *mut in6_addr, mac: *mut u8) {
    *mac = 0x33; *mac.add(1) = 0x33;
    for i in 2..6 { *mac.add(i) = (*addr).s6_addr[i + 10]; }
}
#[inline]
pub unsafe fn rdma_get_vlan_id(dgid: *mut ib_gid) -> u16 {
    let vid = ((*dgid).raw[11] as u16) << 8 | (*dgid).raw[12] as u16;
    if vid < 0x1000 { vid } else { 0xffff }
}

#[inline]
pub unsafe fn rdma_vlan_dev_real_dev(dev: *const net_device) -> *mut net_device {
    if is_vlan_dev(dev) { vlan_dev_real_dev(dev) } else { core::ptr::null_mut() }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
