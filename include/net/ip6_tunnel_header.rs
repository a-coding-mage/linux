/* SPDX-License-Identifier: GPL-2.0 */

// Dependencies supplied by the surrounding kernel translation unit:
// linux/ipv6.h, linux/netdevice.h, linux/if_tunnel.h, linux/ip6_tunnel.h,
// net/ip_tunnels.h, and net/dst_cache.h.

pub const IP6TUNNEL_ERR_TIMEO: usize = 30 * HZ;

/* capable of sending packets */
pub const IP6_TNL_F_CAP_XMIT: u32 = 0x10000;
/* capable of receiving packets */
pub const IP6_TNL_F_CAP_RCV: u32 = 0x20000;
/* determine capability on a per-packet basis */
pub const IP6_TNL_F_CAP_PER_PACKET: u32 = 0x40000;

#[repr(C)]
pub struct __ip6_tnl_parm {
    pub name: [::std::os::raw::c_char; IFNAMSIZ],
    pub link: ::std::os::raw::c_int,
    pub proto: __u8,
    pub encap_limit: __u8,
    pub hop_limit: __u8,
    pub collect_md: bool,
    pub flowinfo: __be32,
    pub flags: __u32,
    pub laddr: in6_addr,
    pub raddr: in6_addr,
    pub i_flags: ip_tunnel_flags,
    pub o_flags: ip_tunnel_flags,
    pub i_key: __be32,
    pub o_key: __be32,
    pub fwmark: __u32,
    pub index: __u32,
    pub erspan_ver: __u8,
    pub dir: __u8,
    pub hwid: __u16,
}

#[repr(C)]
pub struct ip6_tnl {
    pub next: *mut ip6_tnl,
    pub dev: *mut net_device,
    pub dev_tracker: netdevice_tracker,
    pub net: *mut net,
    pub parms: __ip6_tnl_parm,
    pub fl: flowi,
    pub dst_cache: dst_cache,
    pub gro_cells: gro_cells,
    pub err_count: ::std::os::raw::c_int,
    pub err_time: ::std::os::raw::c_ulong,
    pub i_seqno: __u32,
    pub o_seqno: atomic_t,
    pub hlen: ::std::os::raw::c_int,
    pub tun_hlen: ::std::os::raw::c_int,
    pub encap_hlen: ::std::os::raw::c_int,
    pub encap: ip_tunnel_encap,
    pub mlink: ::std::os::raw::c_int,
}

#[repr(C)]
pub struct ip6_tnl_encap_ops {
    pub encap_hlen: Option<unsafe extern "C" fn(e: *mut ip_tunnel_encap) -> usize>,
    pub build_header: Option<unsafe extern "C" fn(
        skb: *mut sk_buff,
        e: *mut ip_tunnel_encap,
        protocol: *mut u8,
        fl6: *mut flowi6,
    ) -> ::std::os::raw::c_int>,
    pub err_handler: Option<unsafe extern "C" fn(
        skb: *mut sk_buff,
        opt: *mut inet6_skb_parm,
        type_: u8,
        code: u8,
        offset: ::std::os::raw::c_int,
        info: __be32,
    ) -> ::std::os::raw::c_int>,
}

// The following declarations and inline functions are enabled when CONFIG_INET
// is enabled in the kernel build.
#[cfg(CONFIG_INET)]
extern "C" {
    pub static mut ip6tun_encaps: [*mut ip6_tnl_encap_ops; MAX_IPTUN_ENCAP_OPS];
    pub fn ip6_tnl_encap_add_ops(ops: *const ip6_tnl_encap_ops, num: ::std::os::raw::c_uint) -> ::std::os::raw::c_int;
    pub fn ip6_tnl_encap_del_ops(ops: *const ip6_tnl_encap_ops, num: ::std::os::raw::c_uint) -> ::std::os::raw::c_int;
    pub fn ip6_tnl_encap_setup(t: *mut ip6_tnl, ipencap: *mut ip_tunnel_encap) -> ::std::os::raw::c_int;
}

#[cfg(CONFIG_INET)]
pub unsafe fn ip6_encap_hlen(e: *mut ip_tunnel_encap) -> ::std::os::raw::c_int {
    let mut hlen: ::std::os::raw::c_int = -EINVAL;
    if (*e).type_ == TUNNEL_ENCAP_NONE { return 0; }
    if (*e).type_ >= MAX_IPTUN_ENCAP_OPS { return -EINVAL; }
    rcu_read_lock();
    let ops = rcu_dereference(ip6tun_encaps[(*e).type_]);
    if !ops.is_null() && (*ops).encap_hlen.is_some() {
        hlen = ((*ops).encap_hlen.unwrap())(e) as ::std::os::raw::c_int;
    }
    rcu_read_unlock();
    hlen
}

#[cfg(CONFIG_INET)]
pub unsafe fn ip6_tnl_encap(skb: *mut sk_buff, t: *mut ip6_tnl, protocol: *mut u8, fl6: *mut flowi6) -> ::std::os::raw::c_int {
    let mut ret: ::std::os::raw::c_int = -EINVAL;
    if (*t).encap.type_ == TUNNEL_ENCAP_NONE { return 0; }
    if (*t).encap.type_ >= MAX_IPTUN_ENCAP_OPS { return -EINVAL; }
    rcu_read_lock();
    let ops = rcu_dereference(ip6tun_encaps[(*t).encap.type_]);
    if !ops.is_null() && (*ops).build_header.is_some() {
        ret = ((*ops).build_header.unwrap())(skb, &mut (*t).encap, protocol, fl6);
    }
    rcu_read_unlock();
    ret
}

pub unsafe fn ip6tunnel_xmit(sk: *mut sock, skb: *mut sk_buff, dev: *mut net_device, ip6cb_flags: u16) {
    let mut pkt_len: ::std::os::raw::c_int;
    let mut err: ::std::os::raw::c_int;
    if dev_recursion_level() > IP_TUNNEL_RECURSION_LIMIT {
        if !dev.is_null() {
            net_crit_ratelimited(c"Dead loop on virtual device %s, fix it urgently!\n".as_ptr(), (*dev).name.as_ptr());
            DEV_STATS_INC(dev, tx_errors);
        }
        kfree_skb_reason(skb, SKB_DROP_REASON_RECURSION_LIMIT);
        return;
    }
    dev_xmit_recursion_inc();
    memset((*skb).cb.as_mut_ptr() as *mut _, 0, size_of::<inet6_skb_parm>());
    (*IP6CB(skb)).flags = ip6cb_flags;
    pkt_len = (*skb).len as ::std::os::raw::c_int - skb_inner_network_offset(skb) as ::std::os::raw::c_int;
    err = ip6_local_out(skb_dst_dev_net(skb), sk, skb);
    if !dev.is_null() {
        if net_xmit_eval(err) != 0 { pkt_len = -1; }
        iptunnel_xmit_stats(dev, pkt_len);
    }
    dev_xmit_recursion_dec();
}

#[repr(C, packed)]
pub struct ipv6_tlv_tnl_enc_lim {
    pub type_: __u8,
    pub length: __u8,
    pub encap_limit: __u8,
}

extern "C" {
    pub fn ip6_tnl_rcv_ctl(t: *mut ip6_tnl, laddr: *const in6_addr, raddr: *const in6_addr) -> ::std::os::raw::c_int;
    pub fn ip6_tnl_rcv(tunnel: *mut ip6_tnl, skb: *mut sk_buff, tpi: *const tnl_ptk_info, tun_dst: *mut metadata_dst, log_ecn_error: bool) -> ::std::os::raw::c_int;
    pub fn ip6_tnl_xmit_ctl(t: *mut ip6_tnl, laddr: *const in6_addr, raddr: *const in6_addr) -> ::std::os::raw::c_int;
    pub fn ip6_tnl_xmit(skb: *mut sk_buff, dev: *mut net_device, dsfield: __u8, fl6: *mut flowi6, encap_limit: ::std::os::raw::c_int, pmtu: *mut __u32, proto: __u8) -> ::std::os::raw::c_int;
    pub fn ip6_tnl_parse_tlv_enc_lim(skb: *mut sk_buff, raw: *mut __u8) -> __u16;
    pub fn ip6_tnl_get_cap(t: *mut ip6_tnl, laddr: *const in6_addr, raddr: *const in6_addr) -> __u32;
    pub fn ip6_tnl_get_link_net(dev: *const net_device) -> *mut net;
    pub fn ip6_tnl_get_iflink(dev: *const net_device) -> ::std::os::raw::c_int;
    pub fn ip6_tnl_change_mtu(dev: *mut net_device, new_mtu: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
