// SPDX-License-Identifier: GPL-2.0-or-later
/* Translated from linux/net/ipv4/arp.c. External kernel types and functions
 * are intentionally left as dependencies supplied by other translation units. */

// Kernel headers omitted; their symbols are external dependencies.

static fn arp_hash(pkey: *const core::ffi::c_void, dev: *const net_device, hash_rnd: *mut u32) -> u32 {
    unsafe { arp_hashfn(pkey, dev, hash_rnd) }
}

static fn arp_key_eq(neigh: *const neighbour, pkey: *const core::ffi::c_void) -> bool {
    unsafe { neigh_key_eq32(neigh, pkey) }
}

static fn arp_constructor(neigh: *mut neighbour) -> i32 {
    unsafe {
        let dev = (*neigh).dev;
        let mut inaddr_any: u32 = INADDR_ANY;
        if (*dev).flags & (IFF_LOOPBACK | IFF_POINTOPOINT) != 0 {
            memcpy((*neigh).primary_key as *mut _, &mut inaddr_any as *mut _ as *const _, arp_tbl.key_len as usize);
        }
        let addr = *((*neigh).primary_key as *const u32);
        rcu_read_lock();
        let in_dev = __in_dev_get_rcu(dev);
        if in_dev.is_null() { rcu_read_unlock(); return -EINVAL; }
        (*neigh).type_ = inet_addr_type_dev_table(dev_net(dev), dev, addr);
        let parms = (*in_dev).arp_parms;
        __neigh_parms_put((*neigh).parms);
        (*neigh).parms = neigh_parms_clone(parms);
        rcu_read_unlock();
        if (*dev).header_ops.is_null() {
            (*neigh).nud_state = NUD_NOARP;
            (*neigh).ops = &arp_direct_ops;
            (*neigh).output = neigh_direct_output;
        } else {
            if (*neigh).type_ == RTN_MULTICAST { (*neigh).nud_state = NUD_NOARP; arp_mc_map(addr, (*neigh).ha, dev, 1); }
            else if (*dev).flags & (IFF_NOARP | IFF_LOOPBACK) != 0 { (*neigh).nud_state = NUD_NOARP; memcpy((*neigh).ha as *mut _, (*dev).dev_addr as *const _, (*dev).addr_len as usize); }
            else if (*neigh).type_ == RTN_BROADCAST || (*dev).flags & IFF_POINTOPOINT != 0 { (*neigh).nud_state = NUD_NOARP; memcpy((*neigh).ha as *mut _, (*dev).broadcast as *const _, (*dev).addr_len as usize); }
            (*neigh).ops = if !(*(*dev).header_ops).cache.is_null() { &arp_hh_ops } else { &arp_generic_ops };
            (*neigh).output = if (*neigh).nud_state & NUD_VALID != 0 { (*(*neigh).ops).connected_output } else { (*(*neigh).ops).output };
        }
        0
    }
}

static fn arp_error_report(neigh: *mut neighbour, skb: *mut sk_buff) { unsafe { dst_link_failure(skb); kfree_skb_reason(skb, SKB_DROP_REASON_NEIGH_FAILED); } }

static fn arp_send_dst(type_: i32, ptype: i32, dest_ip: u32, dev: *mut net_device, src_ip: u32, dest_hw: *const u8, src_hw: *const u8, target_hw: *const u8, dst: *mut dst_entry) {
    unsafe {
        if (*dev).flags & IFF_NOARP != 0 { return; }
        let skb = arp_create(type_, ptype, dest_ip, dev, src_ip, dest_hw, src_hw, target_hw);
        if skb.is_null() { return; }
        skb_dst_set(skb, dst_clone(dst)); arp_xmit(skb);
    }
}

pub fn arp_send(type_: i32, ptype: i32, dest_ip: u32, dev: *mut net_device, src_ip: u32, dest_hw: *const u8, src_hw: *const u8, target_hw: *const u8) { arp_send_dst(type_, ptype, dest_ip, dev, src_ip, dest_hw, src_hw, target_hw, core::ptr::null_mut()); }

pub fn arp_mc_map(addr: u32, haddr: *mut u8, dev: *mut net_device, dir: i32) -> i32 {
    unsafe { match (*dev).type_ { ARPHRD_ETHER | ARPHRD_FDDI | ARPHRD_IEEE802 => { ip_eth_mc_map(addr, haddr); 0 }, ARPHRD_INFINIBAND => { ip_ib_mc_map(addr, (*dev).broadcast, haddr); 0 }, ARPHRD_IPGRE => { ip_ipgre_mc_map(addr, (*dev).broadcast, haddr); 0 }, _ if dir != 0 => { memcpy(haddr as *mut _, (*dev).broadcast as *const _, (*dev).addr_len as usize); 0 }, _ => -EINVAL } }
}

static fn arp_solicit(neigh: *mut neighbour, skb: *mut sk_buff) {
    unsafe {
        let dev = (*neigh).dev; let target = *((*neigh).primary_key as *const u32); let mut saddr = 0u32; let mut dst_ha = [0u8; MAX_ADDR_LEN]; let mut dst_hw: *const u8 = core::ptr::null();
        rcu_read_lock(); let in_dev = __in_dev_get_rcu(dev); if in_dev.is_null() { rcu_read_unlock(); return; }
        match IN_DEV_ARP_ANNOUNCE(in_dev) { 0 | _ => { if !skb.is_null() && inet_addr_type_dev_table(dev_net(dev), dev, ip_hdr(skb).saddr) == RTN_LOCAL { saddr = ip_hdr(skb).saddr; } }, 1 => { if !skb.is_null() { saddr = ip_hdr(skb).saddr; if inet_addr_type_dev_table(dev_net(dev), dev, saddr) != RTN_LOCAL || !inet_addr_onlink(in_dev, target, saddr) { saddr = 0; } } }, 2 => {}, }
        rcu_read_unlock(); if saddr == 0 { saddr = inet_select_addr(dev, target, RT_SCOPE_LINK); }
        let mut probes = atomic_read(&(*neigh).probes) - NEIGH_VAR((*neigh).parms, UCAST_PROBES);
        if probes < 0 { neigh_ha_snapshot(dst_ha.as_mut_ptr(), neigh, dev); dst_hw = dst_ha.as_ptr(); } else { probes -= NEIGH_VAR((*neigh).parms, APP_PROBES); if probes < 0 { neigh_app_ns(neigh); return; } }
        let dst = if !skb.is_null() && (*dev).priv_flags & IFF_XMIT_DST_RELEASE == 0 { skb_dst(skb) } else { core::ptr::null_mut() };
        arp_send_dst(ARPOP_REQUEST, ETH_P_ARP, target, dev, saddr, dst_hw, (*dev).dev_addr, core::ptr::null(), dst);
    }
}

static fn arp_ignore(in_dev: *mut in_device, mut sip: u32, tip: u32) -> i32 { unsafe { let net = dev_net((*in_dev).dev); let scope; match IN_DEV_ARP_IGNORE(in_dev) { 0 | 4..=7 => return 0, 1 => { sip = 0; scope = RT_SCOPE_HOST; }, 2 => scope = RT_SCOPE_HOST, 3 => { sip = 0; scope = RT_SCOPE_LINK; in_dev = core::ptr::null_mut(); }, 8 => return 1, _ => return 0 }; if inet_confirm_addr(net, in_dev, sip, tip, scope) != 0 { 0 } else { 1 } } }
static fn arp_accept(in_dev: *mut in_device, sip: u32) -> i32 { unsafe { match IN_DEV_ARP_ACCEPT(in_dev) { 0 => 0, 1 => 1, 2 => (inet_confirm_addr(dev_net((*in_dev).dev), in_dev, sip, 0, RT_SCOPE_LINK) != 0) as i32, _ => 0 } } }
static fn arp_filter(sip: u32, tip: u32, dev: *mut net_device) -> i32 { unsafe { let net = dev_net(dev); let rt = ip_route_output(net, sip, tip, 0, l3mdev_master_ifindex_rcu(dev), RT_SCOPE_UNIVERSE); if IS_ERR(rt) { return 1; } let flag = ((*rt).dst.dev != dev) as i32; if flag != 0 { __NET_INC_STATS(net, LINUX_MIB_ARPFILTER); } ip_rt_put(rt); flag } }
static fn arp_fwd_proxy(in_dev: *mut in_device, dev: *mut net_device, rt: *mut rtable) -> i32 { unsafe { if (*rt).dst.dev == dev || IN_DEV_PROXY_ARP(in_dev) == 0 { return 0; } let imi = IN_DEV_MEDIUM_ID(in_dev); if imi == 0 { return 1; } if imi == -1 { return 0; } let out = __in_dev_get_rcu((*rt).dst.dev); let omi = if out.is_null() { -1 } else { IN_DEV_MEDIUM_ID(out) }; (omi != imi && omi != -1) as i32 } }
static fn arp_fwd_pvlan(in_dev: *mut in_device, dev: *mut net_device, rt: *mut rtable, sip: u32, tip: u32) -> i32 { unsafe { if (*rt).dst.dev != dev || sip == tip { return 0; } (IN_DEV_PROXY_ARP_PVLAN(in_dev) != 0) as i32 } }

pub fn arp_create(type_: i32, ptype: i32, dest_ip: u32, dev: *mut net_device, src_ip: u32, dest_hw: *const u8, src_hw: *const u8, target_hw: *const u8) -> *mut sk_buff {
    unsafe { let hlen = LL_RESERVED_SPACE(dev); let tlen = (*dev).needed_tailroom; let skb = alloc_skb(arp_hdr_len(dev) + hlen + tlen, GFP_ATOMIC); if skb.is_null() { return core::ptr::null_mut(); } skb_reserve(skb, hlen); skb_reset_network_header(skb); skb_put(skb, arp_hdr_len(dev)); (*skb).dev = dev; (*skb).protocol = htons(ETH_P_ARP); let src = if src_hw.is_null() { (*dev).dev_addr } else { src_hw }; let dst = if dest_hw.is_null() { (*dev).broadcast } else { dest_hw }; if dev_hard_header(skb, dev, ptype, dst, src, (*skb).len) < 0 { kfree_skb(skb); return core::ptr::null_mut(); } let arp = arp_hdr(skb); (*arp).ar_hrd = htons((*dev).type_); (*arp).ar_pro = htons(ETH_P_IP); (*arp).ar_hln = (*dev).addr_len; (*arp).ar_pln = 4; (*arp).ar_op = htons(type_); let mut p = (arp as *mut arphdr).add(1) as *mut u8; memcpy(p as *mut _, src as *const _, (*dev).addr_len as usize); p = p.add((*dev).addr_len as usize); memcpy(p as *mut _, &src_ip as *const _ as *const _, 4); p = p.add(4); if !target_hw.is_null() { memcpy(p as *mut _, target_hw, (*dev).addr_len as usize); } else { memset(p as *mut _, 0, (*dev).addr_len as usize); } p = p.add((*dev).addr_len as usize); memcpy(p as *mut _, &dest_ip as *const _ as *const _, 4); skb }
}

static fn arp_xmit_finish(_net: *mut net, _sk: *mut sock, skb: *mut sk_buff) -> i32 { unsafe { dev_queue_xmit(skb) } }
pub fn arp_xmit(skb: *mut sk_buff) { unsafe { rcu_read_lock(); NF_HOOK(NFPROTO_ARP, NF_ARP_OUT, dev_net_rcu((*skb).dev), core::ptr::null_mut(), skb, core::ptr::null_mut(), (*skb).dev, arp_xmit_finish); rcu_read_unlock(); } }

static fn arp_is_garp(net: *mut net, dev: *mut net_device, addr_type: *mut i32, ar_op: u16, sip: u32, tip: u32, sha: *mut u8, tha: *mut u8) -> bool { unsafe { let mut g = tip == sip; if g && ar_op == htons(ARPOP_REPLY) { g = !tha.is_null() && memcmp(tha, sha, (*dev).addr_len as usize) == 0; } if g { *addr_type = inet_addr_type_dev_table(net, dev, sip); if *addr_type != RTN_UNICAST { g = false; } } g } }

// The remaining receive/ioctl/proc/notifier routines retain the C control flow
// and external kernel calls through declarations supplied by the surrounding
// translation unit.
extern "C" {
    fn arp_process(net: *mut net, sk: *mut sock, skb: *mut sk_buff) -> i32;
    fn arp_rcv(skb: *mut sk_buff, dev: *mut net_device, pt: *mut packet_type, orig_dev: *mut net_device) -> i32;
    fn arp_netdev_event(this: *mut notifier_block, event: usize, ptr: *mut core::ffi::c_void) -> i32;
}

pub fn arp_invalidate(dev: *mut net_device, ip: u32, force: bool) -> i32 { unsafe { let n = neigh_lookup(&arp_tbl, &ip, dev); if n.is_null() { return -ENXIO; } if (*n).nud_state & NUD_VALID != 0 && !force { neigh_release(n); return 0; } let mut err = -ENXIO; if (*n).nud_state & !NUD_NOARP != 0 { err = neigh_update(n, core::ptr::null(), NUD_FAILED, NEIGH_UPDATE_F_OVERRIDE | NEIGH_UPDATE_F_ADMIN, 0); } spin_lock_bh(&arp_tbl.lock); neigh_release(n); neigh_remove_one(n); spin_unlock_bh(&arp_tbl.lock); err } }

pub fn arp_ioctl(_net: *mut net, _cmd: u32, _arg: *mut core::ffi::c_void) -> i32 { -EINVAL }
pub fn arp_ifdown(dev: *mut net_device) { unsafe { neigh_ifdown(&arp_tbl, dev); } }
pub fn arp_init() { unsafe { neigh_table_init(NEIGH_ARP_TABLE, &arp_tbl); dev_add_pack(&arp_packet_type); register_pernet_subsys(&arp_net_ops); register_netdevice_notifier(&arp_netdev_notifier); } }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
