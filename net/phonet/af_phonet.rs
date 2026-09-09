// SPDX-License-Identifier: GPL-2.0-only
/*
 * File: af_phonet.c
 *
 * Phonet protocols family
 *
 * Copyright (C) 2008 Nokia Corporation.
 *
 * Authors: Sakari Ailus <sakari.ailus@nokia.com>
 *          Rémi Denis-Courmont
 */

// Linux kernel dependencies supplied by the surrounding translation unit.

static mut proto_tab: [*const phonet_protocol; PHONET_NPROTO] = [core::ptr::null(); PHONET_NPROTO];

unsafe fn phonet_proto_get(protocol: u32) -> *const phonet_protocol {
    if protocol >= PHONET_NPROTO as u32 { return core::ptr::null(); }
    rcu_read_lock();
    let mut pp = rcu_dereference(proto_tab[protocol as usize]);
    if !pp.is_null() && !try_module_get((*(*pp).prot).owner) { pp = core::ptr::null(); }
    rcu_read_unlock();
    pp
}

#[inline]
unsafe fn phonet_proto_put(pp: *const phonet_protocol) { module_put((*(*pp).prot).owner); }

unsafe fn pn_socket_create(net: *mut net, sock: *mut socket, mut protocol: i32, kern: i32) -> i32 {
    if !capable(CAP_SYS_ADMIN) { return -EPERM; }
    if protocol == 0 {
        protocol = match (*sock).type_ {
            SOCK_DGRAM => PN_PROTO_PHONET,
            SOCK_SEQPACKET => PN_PROTO_PIPE,
            _ => return -EPROTONOSUPPORT,
        };
    }
    let mut pnp = phonet_proto_get(protocol as u32);
    if pnp.is_null() && request_module("net-pf-%d-proto-%d", PF_PHONET, protocol) == 0 { pnp = phonet_proto_get(protocol as u32); }
    if pnp.is_null() { return -EPROTONOSUPPORT; }
    let mut err;
    if (*sock).type_ != (*pnp).sock_type { err = -EPROTONOSUPPORT; } else {
        let sk = sk_alloc(net, PF_PHONET, GFP_KERNEL, (*pnp).prot, kern);
        if sk.is_null() { err = -ENOMEM; } else {
            sock_init_data(sock, sk);
            (*sock).state = SS_UNCONNECTED;
            (*sock).ops = (*pnp).ops;
            (*sk).sk_backlog_rcv = (*(*sk).sk_prot).backlog_rcv;
            (*sk).sk_protocol = protocol as u8;
            let pn = pn_sk(sk);
            (*pn).sobject = 0; (*pn).dobject = 0; (*pn).resource = 0;
            ((*sk).sk_prot).init.unwrap()(sk);
            err = 0;
        }
    }
    phonet_proto_put(pnp); err
}

static phonet_proto_family: net_proto_family = net_proto_family { family: PF_PHONET, create: Some(pn_socket_create), owner: THIS_MODULE };

unsafe fn pn_header_create(skb: *mut sk_buff, dev: *mut net_device, type_: u16, _daddr: *const c_void, saddr: *const c_void, _len: u32) -> i32 {
    let media = skb_push(skb, 1);
    if type_ != ETH_P_PHONET { return -1; }
    let source = if saddr.is_null() { (*dev).dev_addr } else { saddr as *const u8 };
    *media = *source; 1
}
unsafe fn pn_header_parse(skb: *const sk_buff, _dev: *const net_device, haddr: *mut u8) -> i32 { *haddr = *skb_mac_header(skb); 1 }
const phonet_header_ops: header_ops = header_ops { create: Some(pn_header_create), parse: Some(pn_header_parse) };

unsafe fn pn_send(skb: *mut sk_buff, dev: *mut net_device, dst: u16, src: u16, res: u8) -> i32 {
    if (*skb).len + 2 > 0xffff || (*skb).len + core::mem::size_of::<phonethdr>() > (*dev).mtu { kfree_skb(skb); return -EMSGSIZE; }
    if pn_addr(dst) == PNADDR_BROADCAST { kfree_skb(skb); return -EOPNOTSUPP; }
    skb_reset_transport_header(skb); skb_push(skb, core::mem::size_of::<phonethdr>()); skb_reset_network_header(skb);
    let ph = pn_hdr(skb); (*ph).pn_rdev = pn_dev(dst); (*ph).pn_sdev = pn_dev(src); (*ph).pn_res = res;
    (*ph).pn_length = __cpu_to_be16(((*skb).len + 2 - core::mem::size_of::<phonethdr>()) as u16); (*ph).pn_robj = pn_obj(dst); (*ph).pn_sobj = pn_obj(src);
    (*skb).protocol = htons(ETH_P_PHONET); (*skb).priority = 0; (*skb).dev = dev;
    if (*skb).pkt_type == PACKET_LOOPBACK { skb_reset_mac_header(skb); skb_orphan(skb); return if netif_rx(skb) != 0 { -ENOBUFS } else { 0 }; }
    let mut err = dev_hard_header(skb, dev, ntohs((*skb).protocol), core::ptr::null(), core::ptr::null(), (*skb).len);
    if err < 0 { kfree_skb(skb); return -EHOSTUNREACH; }
    err = dev_queue_xmit(skb); if err > 0 { err = net_xmit_errno(err); } err
}

unsafe fn pn_raw_send(data: *const c_void, len: i32, dev: *mut net_device, dst: u16, src: u16, res: u8) -> i32 {
    let skb = alloc_skb((MAX_PHONET_HEADER as i32 + len) as u32, GFP_ATOMIC); if skb.is_null() { return -ENOMEM; }
    if phonet_address_lookup(dev_net(dev), pn_addr(dst)) == 0 { (*skb).pkt_type = PACKET_LOOPBACK; }
    skb_reserve(skb, MAX_PHONET_HEADER); __skb_put(skb, len as u32); skb_copy_to_linear_data(skb, data, len as usize); pn_send(skb, dev, dst, src, res)
}

unsafe fn pn_skb_send(sk: *mut sock, skb: *mut sk_buff, target: *const sockaddr_pn) -> i32 {
    let net = sock_net(sk); let pn = pn_sk(sk); let mut src = (*pn).sobject; let (mut dst, res);
    if !target.is_null() { dst = pn_sockaddr_get_object(target); res = pn_sockaddr_get_resource(target); } else { dst = (*pn).dobject; res = (*pn).resource; }
    let daddr = pn_addr(dst); let mut dev;
    if (*sk).sk_bound_dev_if != 0 { dev = dev_get_by_index(net, (*sk).sk_bound_dev_if); }
    else if phonet_address_lookup(net, daddr) == 0 { dev = phonet_device_get(net); (*skb).pkt_type = PACKET_LOOPBACK; }
    else if dst == 0 { let other = pn_find_sock_by_res(net, res); if !other.is_null() { sock_put(other); dev = phonet_device_get(net); (*skb).pkt_type = PACKET_LOOPBACK; } else { dev = phonet_route_output(net, daddr); } }
    else { dev = phonet_route_output(net, daddr); }
    if dev.is_null() || (*dev).flags & IFF_UP == 0 { kfree_skb(skb); dev_put(dev); return -EHOSTUNREACH; }
    let saddr = phonet_address_get(dev, daddr); if saddr == PN_NO_ADDR { kfree_skb(skb); dev_put(dev); return -EHOSTUNREACH; }
    if pn_addr(src) == 0 { src = pn_object(saddr, pn_obj(src)); }
    let err = pn_send(skb, dev, dst, src, res); dev_put(dev); err
}

#[inline] unsafe fn can_respond(skb: *mut sk_buff) -> i32 {
    if !pskb_may_pull(skb, 3) { return 0; } let ph = pn_hdr(skb); if (*ph).pn_res == PN_PREFIX && !pskb_may_pull(skb, 5) { return 0; } if (*ph).pn_res == PN_COMMGR { return 0; }
    let pm = pn_msg(skb); if (*pm).pn_msg_id != PN_COMMON_MESSAGE { return 1; }
    let sub = if (*ph).pn_res == PN_PREFIX { (*pm).pn_e_submsg_id } else { (*pm).pn_submsg_id };
    if sub != PN_COMM_ISA_ENTITY_NOT_REACHABLE_RESP && (*pm).pn_e_submsg_id != PN_COMM_SERVICE_NOT_IDENTIFIED_RESP { 1 } else { 0 }
}

unsafe fn send_obj_unreachable(rskb: *mut sk_buff) -> i32 {
    let oph = pn_hdr(rskb); let opm = pn_msg(rskb); let mut resp: phonetmsg = core::mem::zeroed(); resp.pn_trans_id = (*opm).pn_trans_id; resp.pn_msg_id = PN_COMMON_MESSAGE;
    if (*oph).pn_res == PN_PREFIX { resp.pn_e_res_id = (*opm).pn_e_res_id; resp.pn_e_submsg_id = PN_COMM_ISA_ENTITY_NOT_REACHABLE_RESP; resp.pn_e_orig_msg_id = (*opm).pn_msg_id; resp.pn_e_status = 0; }
    else { resp.pn_submsg_id = PN_COMM_ISA_ENTITY_NOT_REACHABLE_RESP; resp.pn_orig_msg_id = (*opm).pn_msg_id; resp.pn_status = 0; }
    pn_raw_send(&resp as *const _ as *const c_void, core::mem::size_of::<phonetmsg>() as i32, (*rskb).dev, pn_object(((*oph).pn_sdev), (*oph).pn_sobj), pn_object((*oph).pn_rdev, (*oph).pn_robj), (*oph).pn_res)
}

unsafe fn send_reset_indications(rskb: *mut sk_buff) -> i32 { let oph = pn_hdr(rskb); let data = [0, 0x10, 0, 0]; pn_raw_send(data.as_ptr() as *const c_void, 4, (*rskb).dev, pn_object((*oph).pn_sdev, 0), pn_object((*oph).pn_rdev, (*oph).pn_robj), PN_COMMGR) }

unsafe fn phonet_rcv(mut skb: *mut sk_buff, dev: *mut net_device, _pkttype: *mut packet_type, _orig_dev: *mut net_device) -> i32 {
    let net = dev_net(dev); skb = skb_share_check(skb, GFP_ATOMIC); if skb.is_null() { return NET_RX_DROP; }
    if !pskb_pull(skb, core::mem::size_of::<phonethdr>()) { kfree_skb(skb); return NET_RX_DROP; }
    let ph = pn_hdr(skb); let mut len = get_unaligned_be16(&(*ph).pn_length); if len < 2 { kfree_skb(skb); return NET_RX_DROP; } len -= 2; if len as usize > (*skb).len || pskb_trim(skb, len as usize) { kfree_skb(skb); return NET_RX_DROP; } skb_reset_transport_header(skb);
    let mut sa: sockaddr_pn = core::mem::zeroed(); pn_skb_get_dst_sockaddr(skb, &mut sa);
    if pn_sockaddr_get_addr(&sa) == PNADDR_BROADCAST { pn_deliver_sock_broadcast(net, skb); kfree_skb(skb); return NET_RX_DROP; }
    if pn_sockaddr_get_object(&sa) == 0 { let sk = pn_find_sock_by_res(net, sa.spn_resource); if !sk.is_null() { return sk_receive_skb(sk, skb, 0); } }
    if phonet_address_lookup(net, pn_sockaddr_get_addr(&sa)) == 0 { let sk = pn_find_sock_by_sa(net, &sa); if !sk.is_null() { return sk_receive_skb(sk, skb, 0); } if can_respond(skb) != 0 { send_obj_unreachable(skb); send_reset_indications(skb); } kfree_skb(skb); return NET_RX_DROP; }
    if (*skb).pkt_type == PACKET_LOOPBACK { kfree_skb(skb); return NET_RX_DROP; }
    let out = phonet_route_output(net, pn_sockaddr_get_addr(&sa)); if out.is_null() { kfree_skb(skb); return NET_RX_DROP; } __skb_push(skb, core::mem::size_of::<phonethdr>()); (*skb).dev = out; if out == dev || skb_cow_head(skb, (*out).hard_header_len) != 0 { dev_put(out); kfree_skb(skb); return NET_RX_DROP; } if dev_hard_header(skb, out, ETH_P_PHONET, core::ptr::null(), core::ptr::null(), (*skb).len) < 0 { dev_put(out); kfree_skb(skb); return NET_RX_DROP; } dev_queue_xmit(skb); dev_put(out); NET_RX_SUCCESS
}

static mut phonet_packet_type: packet_type = packet_type { type_: cpu_to_be16(ETH_P_PHONET), func: Some(phonet_rcv) };
static mut proto_tab_lock: mutex = DEFINE_MUTEX!();

unsafe fn phonet_proto_register(protocol: u32, pp: *const phonet_protocol) -> i32 { if protocol >= PHONET_NPROTO as u32 { return -EINVAL; } let mut err = proto_register((*pp).prot, 1); if err != 0 { return err; } mutex_lock(&mut proto_tab_lock); if !proto_tab[protocol as usize].is_null() { err = -EBUSY; } else { rcu_assign_pointer(&mut proto_tab[protocol as usize], pp); } mutex_unlock(&mut proto_tab_lock); err }
unsafe fn phonet_proto_unregister(protocol: u32, pp: *const phonet_protocol) { mutex_lock(&mut proto_tab_lock); BUG_ON(rcu_access_pointer(proto_tab[protocol as usize]) != pp); RCU_INIT_POINTER(&mut proto_tab[protocol as usize], core::ptr::null()); mutex_unlock(&mut proto_tab_lock); synchronize_rcu(); proto_unregister((*pp).prot); }

unsafe fn phonet_init() -> i32 { let mut err = phonet_device_init(); if err != 0 { return err; } pn_sock_init(); err = sock_register(&phonet_proto_family); if err != 0 { printk!(KERN_ALERT, "phonet protocol family initialization failed\n"); phonet_device_exit(); return err; } dev_add_pack(&mut phonet_packet_type); phonet_sysctl_init(); err = isi_register(); if err != 0 { phonet_sysctl_exit(); sock_unregister(PF_PHONET); dev_remove_pack(&mut phonet_packet_type); phonet_device_exit(); } err }
unsafe fn phonet_exit() { isi_unregister(); phonet_sysctl_exit(); sock_unregister(PF_PHONET); dev_remove_pack(&mut phonet_packet_type); phonet_device_exit(); }

// module_init(phonet_init); module_exit(phonet_exit);
// MODULE_DESCRIPTION("Phonet protocol stack for Linux"); MODULE_LICENSE("GPL"); MODULE_ALIAS_NETPROTO(PF_PHONET);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
