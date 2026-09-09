/* SPDX-License-Identifier: GPL-2.0 */
// Translated from net/udp_tunnel.h. C header dependencies are supplied externally.

pub const UDP_TUNNEL_PARTIAL_FEATURES: _ = NETIF_F_GSO_ENCAP_ALL;
pub const UDP_TUNNEL_STRIPPED_GSO_TYPES: _ =
    (UDP_TUNNEL_PARTIAL_FEATURES | NETIF_F_GSO_PARTIAL) >> NETIF_F_GSO_SHIFT;

#[repr(C)]
pub struct udp_port_cfg {
    pub family: u8,
    pub local_ip: in_addr,
    #[cfg(CONFIG_IPV6)]
    pub local_ip6: in6_addr,
    pub peer_ip: in_addr,
    #[cfg(CONFIG_IPV6)]
    pub peer_ip6: in6_addr,
    pub local_udp_port: __be16,
    pub peer_udp_port: __be16,
    pub bind_ifindex: i32,
    // C bitfields, represented as their containing integer.
    pub flags: u32,
}

extern "C" {
    pub fn udp_sock_create4(net: *mut net, cfg: *mut udp_port_cfg, sockp: *mut *mut socket) -> i32;
    #[cfg(CONFIG_IPV6)]
    pub fn udp_sock_create6(net: *mut net, cfg: *mut udp_port_cfg, sockp: *mut *mut socket) -> i32;
}

#[inline]
pub unsafe fn udp_sock_create(net: *mut net, cfg: *mut udp_port_cfg, sockp: *mut *mut socket) -> i32 {
    if (*cfg).family as _ == AF_INET { return udp_sock_create4(net, cfg, sockp); }
    if (*cfg).family as _ == AF_INET6 {
        #[cfg(CONFIG_IPV6)] { return udp_sock_create6(net, cfg, sockp); }
        #[cfg(not(CONFIG_IPV6))] { return -EPFNOSUPPORT; }
    }
    -EPFNOSUPPORT
}

pub type udp_tunnel_encap_rcv_t = unsafe extern "C" fn(*mut sock, *mut sk_buff) -> i32;
pub type udp_tunnel_encap_err_lookup_t = unsafe extern "C" fn(*mut sock, *mut sk_buff) -> i32;
pub type udp_tunnel_encap_err_rcv_t = unsafe extern "C" fn(*mut sock, *mut sk_buff, i32, __be16, u32, *mut u8);
pub type udp_tunnel_encap_destroy_t = unsafe extern "C" fn(*mut sock);
pub type udp_tunnel_gro_receive_t = unsafe extern "C" fn(*mut sock, *mut list_head, *mut sk_buff) -> *mut sk_buff;
pub type udp_tunnel_gro_complete_t = unsafe extern "C" fn(*mut sock, *mut sk_buff, i32) -> i32;

#[repr(C)]
pub struct udp_tunnel_sock_cfg {
    pub sk_user_data: *mut core::ffi::c_void,
    pub encap_type: u8,
    pub encap_rcv: Option<udp_tunnel_encap_rcv_t>,
    pub encap_err_lookup: Option<udp_tunnel_encap_err_lookup_t>,
    pub encap_err_rcv: Option<udp_tunnel_encap_err_rcv_t>,
    pub encap_destroy: Option<udp_tunnel_encap_destroy_t>,
    pub gro_receive: Option<udp_tunnel_gro_receive_t>,
    pub gro_complete: Option<udp_tunnel_gro_complete_t>,
}

extern "C" {
    pub fn setup_udp_tunnel_sock(net: *mut net, sk: *mut sock, sock_cfg: *mut udp_tunnel_sock_cfg);
}

#[repr(C)]
pub enum udp_parsable_tunnel_type {
    UDP_TUNNEL_TYPE_VXLAN = BIT(0),
    UDP_TUNNEL_TYPE_GENEVE = BIT(1),
    UDP_TUNNEL_TYPE_VXLAN_GPE = BIT(2),
}

#[repr(C)]
pub struct udp_tunnel_info { pub type_: u16, pub sa_family: sa_family_t, pub port: __be16, pub hw_priv: u8 }

extern "C" {
    pub fn udp_tunnel_push_rx_port(dev: *mut net_device, sk: *mut sock, type_: u16);
    pub fn udp_tunnel_drop_rx_port(dev: *mut net_device, sk: *mut sock, type_: u16);
    pub fn udp_tunnel_notify_add_rx_port(sk: *mut sock, type_: u16);
    pub fn udp_tunnel_notify_del_rx_port(sk: *mut sock, type_: u16);
    pub fn udp_tunnel_xmit_skb(rt: *mut rtable, sk: *mut sock, skb: *mut sk_buff, src: __be32, dst: __be32, tos: u8, ttl: u8, df: __be16, src_port: __be16, dst_port: __be16, xnet: bool, nocheck: bool, ipcb_flags: u16);
    pub fn udp_tunnel6_xmit_skb(dst: *mut dst_entry, sk: *mut sock, skb: *mut sk_buff, dev: *mut net_device, saddr: *const in6_addr, daddr: *const in6_addr, prio: u8, ttl: u8, label: __be32, src_port: __be16, dst_port: __be16, nocheck: bool, ip6cb_flags: u16);
}

#[inline]
pub unsafe fn udp_tunnel_handle_partial(skb: *mut sk_buff) -> bool {
    let double_encap = ((*skb_shinfo(skb)).gso_type & SKB_GSO_PARTIAL) != 0;
    if double_encap { (*skb_shinfo(skb)).gso_type &= !UDP_TUNNEL_STRIPPED_GSO_TYPES; }
    double_encap
}

#[inline]
pub unsafe fn udp_tunnel_set_inner_protocol(skb: *mut sk_buff, double_encap: bool, inner_proto: __be16) {
    if !double_encap { skb_set_inner_protocol(skb, inner_proto); }
}

extern "C" {
    pub fn udp_tunnel_sock_release(sk: *mut sock);
    pub fn udp_tunnel_dst_lookup(skb: *mut sk_buff, dev: *mut net_device, net: *mut net, oif: i32, saddr: *mut __be32, key: *const ip_tunnel_key, sport: __be16, dport: __be16, tos: u8, dst_cache: *mut dst_cache) -> *mut rtable;
    pub fn udp_tunnel6_dst_lookup(skb: *mut sk_buff, dev: *mut net_device, net: *mut net, sk: *mut sock, oif: i32, saddr: *mut in6_addr, key: *const ip_tunnel_key, sport: __be16, dport: __be16, dsfield: u8, dst_cache: *mut dst_cache) -> *mut dst_entry;
    pub fn udp_tun_rx_dst(skb: *mut sk_buff, family: u16, flags: *const c_ulong, tunnel_id: __be64, md_size: i32) -> *mut metadata_dst;
}

#[cfg(CONFIG_INET)]
#[inline]
pub unsafe fn udp_tunnel_handle_offloads(skb: *mut sk_buff, udp_csum: bool) -> i32 {
    let type_ = if udp_csum { SKB_GSO_UDP_TUNNEL_CSUM } else { SKB_GSO_UDP_TUNNEL };
    iptunnel_handle_offloads(skb, type_)
}

extern "C" {
    pub fn udp_tunnel_update_gro_lookup(net: *mut net, sk: *mut sock, add: bool);
    pub fn udp_tunnel_update_gro_rcv(sk: *mut sock, add: bool);
}

#[inline] pub unsafe fn udp_tunnel_cleanup_gro(sk: *mut sock) { udp_tunnel_update_gro_rcv(sk, false); udp_tunnel_update_gro_lookup(sock_net(sk), sk, false); }
#[inline] pub unsafe fn udp_tunnel_encap_enable(sk: *mut sock) { if udp_test_and_set_bit(ENCAP_ENABLED, sk) { return; } #[cfg(CONFIG_IPV6)] if READ_ONCE((*sk).sk_family) == PF_INET6 { udpv6_encap_enable(); } udp_encap_enable(); }

pub const UDP_TUNNEL_NIC_MAX_TABLES: usize = 4;
pub const UDP_TUNNEL_NIC_MAX_SHARING_DEVICES: u16 = U16_MAX / 2;
pub const UDP_TUNNEL_NIC_INFO_OPEN_ONLY: u32 = BIT(0);
pub const UDP_TUNNEL_NIC_INFO_IPV4_ONLY: u32 = BIT(1);
pub const UDP_TUNNEL_NIC_INFO_STATIC_IANA_VXLAN: u32 = BIT(2);

#[repr(C)] pub struct udp_tunnel_nic;
#[repr(C)] pub struct udp_tunnel_nic_shared { pub udp_tunnel_nic_info: *mut udp_tunnel_nic, pub devices: list_head }
#[repr(C)] pub struct udp_tunnel_nic_shared_node { pub dev: *mut net_device, pub list: list_head }

#[repr(C)]
pub struct udp_tunnel_nic_info {
    pub set_port: Option<unsafe extern "C" fn(*mut net_device, u32, u32, *mut udp_tunnel_info) -> i32>,
    pub unset_port: Option<unsafe extern "C" fn(*mut net_device, u32, u32, *mut udp_tunnel_info) -> i32>,
    pub sync_table: Option<unsafe extern "C" fn(*mut net_device, u32) -> i32>,
    pub shared: *mut udp_tunnel_nic_shared,
    pub flags: u32,
    pub tables: [udp_tunnel_nic_table_info; UDP_TUNNEL_NIC_MAX_TABLES],
}
#[repr(C)] pub struct udp_tunnel_nic_table_info { pub n_entries: u32, pub tunnel_types: u32 }

#[repr(C)]
pub struct udp_tunnel_nic_ops {
    pub get_port: Option<unsafe extern "C" fn(*mut net_device, u32, u32, *mut udp_tunnel_info)>,
    pub set_port_priv: Option<unsafe extern "C" fn(*mut net_device, u32, u32, u8)>,
    pub add_port: Option<unsafe extern "C" fn(*mut net_device, *mut udp_tunnel_info)>,
    pub del_port: Option<unsafe extern "C" fn(*mut net_device, *mut udp_tunnel_info)>,
    pub reset_ntf: Option<unsafe extern "C" fn(*mut net_device)>,
    pub dump_size: Option<unsafe extern "C" fn(*mut net_device, u32) -> usize>,
    pub dump_write: Option<unsafe extern "C" fn(*mut net_device, u32, *mut sk_buff) -> i32>,
    pub assert_locked: Option<unsafe extern "C" fn(*mut net_device)>,
    pub lock: Option<unsafe extern "C" fn(*mut net_device)>,
    pub unlock: Option<unsafe extern "C" fn(*mut net_device)>,
}

#[cfg(CONFIG_INET)] extern "C" { pub static mut udp_tunnel_nic_ops: *const udp_tunnel_nic_ops; }
#[cfg(not(CONFIG_INET))] pub const udp_tunnel_nic_ops: *const udp_tunnel_nic_ops = core::ptr::null();

#[inline] pub unsafe fn udp_tunnel_nic_get_port(dev: *mut net_device, table: u32, idx: u32, ti: *mut udp_tunnel_info) { memset(ti, 0, core::mem::size_of::<udp_tunnel_info>()); if !udp_tunnel_nic_ops.is_null() { ((*udp_tunnel_nic_ops).get_port.unwrap())(dev, table, idx, ti); } }
#[inline] pub unsafe fn udp_tunnel_nic_set_port_priv(dev: *mut net_device, table: u32, idx: u32, priv_: u8) { if !udp_tunnel_nic_ops.is_null() { ((*udp_tunnel_nic_ops).assert_locked.unwrap())(dev); ((*udp_tunnel_nic_ops).set_port_priv.unwrap())(dev, table, idx, priv_); } }
#[inline] pub unsafe fn udp_tunnel_nic_assert_locked(dev: *mut net_device) { if !udp_tunnel_nic_ops.is_null() { ((*udp_tunnel_nic_ops).assert_locked.unwrap())(dev); } }
#[inline] pub unsafe fn udp_tunnel_nic_lock(dev: *mut net_device) { if !udp_tunnel_nic_ops.is_null() { ((*udp_tunnel_nic_ops).lock.unwrap())(dev); } }
#[inline] pub unsafe fn udp_tunnel_nic_unlock(dev: *mut net_device) { if !udp_tunnel_nic_ops.is_null() { ((*udp_tunnel_nic_ops).unlock.unwrap())(dev); } }
#[inline] pub unsafe fn udp_tunnel_nic_add_port(dev: *mut net_device, ti: *mut udp_tunnel_info) { if (*dev).features & NETIF_F_RX_UDP_TUNNEL_PORT == 0 { return; } if !udp_tunnel_nic_ops.is_null() { ((*udp_tunnel_nic_ops).add_port.unwrap())(dev, ti); } }
#[inline] pub unsafe fn udp_tunnel_nic_del_port(dev: *mut net_device, ti: *mut udp_tunnel_info) { if (*dev).features & NETIF_F_RX_UDP_TUNNEL_PORT == 0 { return; } if !udp_tunnel_nic_ops.is_null() { ((*udp_tunnel_nic_ops).del_port.unwrap())(dev, ti); } }
#[inline] pub unsafe fn udp_tunnel_nic_reset_ntf(dev: *mut net_device) { if !udp_tunnel_nic_ops.is_null() { ((*udp_tunnel_nic_ops).reset_ntf.unwrap())(dev); } }
#[inline] pub unsafe fn udp_tunnel_nic_dump_size(dev: *mut net_device, table: u32) -> usize { if udp_tunnel_nic_ops.is_null() { return 0; } udp_tunnel_nic_lock(dev); let ret = ((*udp_tunnel_nic_ops).dump_size.unwrap())(dev, table); udp_tunnel_nic_unlock(dev); ret }
#[inline] pub unsafe fn udp_tunnel_nic_dump_write(dev: *mut net_device, table: u32, skb: *mut sk_buff) -> i32 { if udp_tunnel_nic_ops.is_null() { return 0; } udp_tunnel_nic_lock(dev); let ret = ((*udp_tunnel_nic_ops).dump_write.unwrap())(dev, table, skb); udp_tunnel_nic_unlock(dev); ret }
#[inline] pub unsafe fn udp_tunnel_get_rx_info(dev: *mut net_device) { ASSERT_RTNL(); if (*dev).features & NETIF_F_RX_UDP_TUNNEL_PORT == 0 { return; } udp_tunnel_nic_assert_locked(dev); call_netdevice_notifiers(NETDEV_UDP_TUNNEL_PUSH_INFO, dev); }
#[inline] pub unsafe fn udp_tunnel_drop_rx_info(dev: *mut net_device) { ASSERT_RTNL(); if (*dev).features & NETIF_F_RX_UDP_TUNNEL_PORT == 0 { return; } udp_tunnel_nic_assert_locked(dev); call_netdevice_notifiers(NETDEV_UDP_TUNNEL_DROP_INFO, dev); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
