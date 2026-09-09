/* SPDX-License-Identifier: GPL-2.0-or-later */
/* Rust translation of linux/etherdevice.h. C includes and build conditions are
 * supplied by the surrounding kernel translation. */

#[cfg(feature = "kernel")]
extern "C" {
    pub fn eth_platform_get_mac_address(dev: *mut device, mac_addr: *mut u8) -> i32;
    pub fn platform_get_ethdev_address(dev: *mut device, netdev: *mut net_device) -> i32;
    pub fn arch_get_platform_mac_address() -> *mut u8;
    pub fn nvmem_get_mac_address(dev: *mut device, addrbuf: *mut core::ffi::c_void) -> i32;
    pub fn device_get_mac_address(dev: *mut device, addr: *mut i8) -> i32;
    pub fn device_get_ethdev_address(dev: *mut device, netdev: *mut net_device) -> i32;
    pub fn fwnode_get_mac_address(fwnode: *mut fwnode_handle, addr: *mut i8) -> i32;
    pub fn eth_get_headlen(dev: *const net_device, data: *const core::ffi::c_void, len: u32) -> u32;
    pub fn eth_type_trans(skb: *mut sk_buff, dev: *mut net_device) -> __be16;
    pub static eth_header_ops: header_ops;
    pub fn eth_header(skb: *mut sk_buff, dev: *mut net_device, type_: u16, daddr: *const core::ffi::c_void, saddr: *const core::ffi::c_void, len: u32) -> i32;
    pub fn eth_header_parse(skb: *const sk_buff, dev: *const net_device, haddr: *mut u8) -> i32;
    pub fn eth_header_cache(neigh: *const neighbour, hh: *mut hh_cache, type_: __be16) -> i32;
    pub fn eth_header_cache_update(hh: *mut hh_cache, dev: *const net_device, haddr: *const u8);
    pub fn eth_header_parse_protocol(skb: *const sk_buff) -> __be16;
    pub fn eth_prepare_mac_addr_change(dev: *mut net_device, p: *mut core::ffi::c_void) -> i32;
    pub fn eth_commit_mac_addr_change(dev: *mut net_device, p: *mut core::ffi::c_void);
    pub fn eth_mac_addr(dev: *mut net_device, p: *mut core::ffi::c_void) -> i32;
    pub fn eth_validate_addr(dev: *mut net_device) -> i32;
    pub fn alloc_etherdev_mqs(sizeof_priv: i32, txqs: u32, rxqs: u32) -> *mut net_device;
    pub fn devm_alloc_etherdev_mqs(dev: *mut device, sizeof_priv: i32, txqs: u32, rxqs: u32) -> *mut net_device;
    pub fn eth_gro_receive(head: *mut list_head, skb: *mut sk_buff) -> *mut sk_buff;
    pub fn eth_gro_complete(skb: *mut sk_buff, nhoff: i32) -> i32;
}

pub const ETH_ALEN: usize = 6;
#[cfg(feature = "kernel")]
pub const eth_reserved_addr_base: [u8; ETH_ALEN] = [0x01, 0x80, 0xc2, 0, 0, 0];
#[cfg(feature = "kernel")]
pub const eth_stp_addr: [u8; ETH_ALEN] = eth_reserved_addr_base;
#[cfg(feature = "kernel")]
pub const eth_ipv4_mcast_addr_base: [u8; ETH_ALEN] = [0x01, 0, 0x5e, 0, 0, 0];
#[cfg(feature = "kernel")]
pub const eth_ipv6_mcast_addr_base: [u8; ETH_ALEN] = [0x33, 0x33, 0, 0, 0, 0];

#[inline]
pub unsafe fn is_link_local_ether_addr(addr: *const u8) -> bool {
    let a = addr as *const u16;
    let b = eth_reserved_addr_base.as_ptr() as *const u16;
    ((*a ^ *b) | (*a.add(1) ^ *b.add(1)) | ((*a.add(2) ^ *b.add(2)) & 0xfff0u16)) == 0
}
#[inline]
pub unsafe fn is_zero_ether_addr(addr: *const u8) -> bool {
    ((*((addr) as *const u16)) | *((addr.add(2)) as *const u16) | *((addr.add(4)) as *const u16)) == 0
}
#[inline]
pub unsafe fn is_multicast_ether_addr(addr: *const u8) -> bool { *addr & 1 != 0 }
#[inline]
pub unsafe fn is_multicast_ether_addr_64bits(addr: *const u8) -> bool { is_multicast_ether_addr(addr) }
#[inline]
pub unsafe fn is_local_ether_addr(addr: *const u8) -> bool { *addr & 2 != 0 }
#[inline]
pub unsafe fn is_broadcast_ether_addr(addr: *const u8) -> bool {
    (*((addr) as *const u16) & *((addr.add(2)) as *const u16) & *((addr.add(4)) as *const u16)) == 0xffff
}
#[inline] pub unsafe fn is_unicast_ether_addr(addr: *const u8) -> bool { !is_multicast_ether_addr(addr) }
#[inline] pub unsafe fn is_valid_ether_addr(addr: *const u8) -> bool { !is_multicast_ether_addr(addr) && !is_zero_ether_addr(addr) }
#[inline] pub fn eth_proto_is_802_3(proto: __be16) -> bool { proto as u16 >= 0x0600 }

#[inline]
pub unsafe fn eth_random_addr(addr: *mut u8) {
    get_random_bytes(addr, ETH_ALEN);
    *addr &= 0xfe; *addr |= 0x02;
}
#[inline] pub unsafe fn eth_broadcast_addr(addr: *mut u8) { core::ptr::write_bytes(addr, 0xff, ETH_ALEN); }
#[inline] pub unsafe fn eth_zero_addr(addr: *mut u8) { core::ptr::write_bytes(addr, 0, ETH_ALEN); }
#[inline] pub unsafe fn ether_addr_copy(dst: *mut u8, src: *const u8) { core::ptr::copy_nonoverlapping(src, dst, ETH_ALEN); }
#[inline] pub unsafe fn ether_addr_equal(a: *const u8, b: *const u8) -> bool { core::slice::from_raw_parts(a, ETH_ALEN) == core::slice::from_raw_parts(b, ETH_ALEN) }
#[inline] pub unsafe fn ether_addr_equal_64bits(a: *const u8, b: *const u8) -> bool { ether_addr_equal(a,b) }
#[inline] pub unsafe fn ether_addr_equal_unaligned(a: *const u8, b: *const u8) -> bool { ether_addr_equal(a,b) }
#[inline]
pub unsafe fn ether_addr_equal_masked(a: *const u8, b: *const u8, mask: *const u8) -> bool {
    for i in 0..ETH_ALEN { if ((*a.add(i) ^ *b.add(i)) & *mask.add(i)) != 0 { return false; } } true
}
#[inline] pub unsafe fn ether_addr_is_ipv4_mcast(a: *const u8) -> bool { ether_addr_equal_masked(a, eth_ipv4_mcast_addr_base.as_ptr(), [0xff,0xff,0xff,0x80,0,0].as_ptr()) }
#[inline] pub unsafe fn ether_addr_is_ipv6_mcast(a: *const u8) -> bool { ether_addr_equal_masked(a, eth_ipv6_mcast_addr_base.as_ptr(), [0xff,0xff,0,0,0,0].as_ptr()) }
#[inline] pub unsafe fn ether_addr_is_ip_mcast(a: *const u8) -> bool { ether_addr_is_ipv4_mcast(a) || ether_addr_is_ipv6_mcast(a) }
#[inline] pub unsafe fn ether_addr_to_u64(addr: *const u8) -> u64 { let mut u=0; for i in 0..ETH_ALEN { u = (u << 8) | *addr.add(i) as u64; } u }
#[inline] pub unsafe fn u64_to_ether_addr(mut u: u64, addr: *mut u8) { for i in (0..ETH_ALEN).rev() { *addr.add(i)=(u&0xff) as u8; u >>= 8; } }
#[inline] pub unsafe fn eth_addr_dec(addr: *mut u8) { let u=ether_addr_to_u64(addr).wrapping_sub(1); u64_to_ether_addr(u,addr); }
#[inline] pub unsafe fn eth_addr_inc(addr: *mut u8) { let u=ether_addr_to_u64(addr).wrapping_add(1); u64_to_ether_addr(u,addr); }
#[inline] pub unsafe fn eth_addr_add(addr: *mut u8, offset: isize) { let u=ether_addr_to_u64(addr).wrapping_add(offset as u64); u64_to_ether_addr(u,addr); }
#[inline] pub unsafe fn compare_ether_header(a: *const core::ffi::c_void, b: *const core::ffi::c_void) -> usize { let x=core::slice::from_raw_parts(a as *const u8,14); let y=core::slice::from_raw_parts(b as *const u8,14); x.iter().zip(y).fold(0usize, |v,(&p,&q)| v | (p^q) as usize) }

// External kernel types and helpers used by the declarations above.
#[cfg(feature = "kernel")]
extern "C" { fn get_random_bytes(buf: *mut u8, len: usize); }
#[cfg(feature = "kernel")] pub type __be16 = u16;
#[cfg(feature = "kernel")] pub enum device {}
#[cfg(feature = "kernel")] pub enum fwnode_handle {}
#[cfg(feature = "kernel")] pub enum net_device {}
#[cfg(feature = "kernel")] pub enum sk_buff {}
#[cfg(feature = "kernel")] pub enum header_ops {}
#[cfg(feature = "kernel")] pub enum neighbour {}
#[cfg(feature = "kernel")] pub enum hh_cache {}
#[cfg(feature = "kernel")] pub enum list_head {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
