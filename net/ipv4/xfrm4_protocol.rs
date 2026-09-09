// SPDX-License-Identifier: GPL-2.0-or-later
/* xfrm4_protocol.c - Generic xfrm protocol multiplexer.
 *
 * Copyright (C) 2013 secunet Security Networks AG
 *
 * Author:
 * Steffen Klassert <steffen.klassert@secunet.com>
 *
 * Based on:
 * net/ipv4/tunnel4.c
 */

// Kernel headers supplied by other translation units.

static mut esp4_handlers: *mut xfrm4_protocol = core::ptr::null_mut();
static mut ah4_handlers: *mut xfrm4_protocol = core::ptr::null_mut();
static mut ipcomp4_handlers: *mut xfrm4_protocol = core::ptr::null_mut();
static mut xfrm4_protocol_mutex: mutex = mutex {};

unsafe fn proto_handlers(protocol: u8) -> *mut *mut xfrm4_protocol {
    match protocol {
        IPPROTO_ESP => &raw mut esp4_handlers,
        IPPROTO_AH => &raw mut ah4_handlers,
        IPPROTO_COMP => &raw mut ipcomp4_handlers,
        _ => core::ptr::null_mut(),
    }
}

unsafe fn xfrm4_rcv_cb(skb: *mut sk_buff, protocol: u8, err: i32) -> i32 {
    let head = proto_handlers(protocol);
    if head.is_null() {
        return 0;
    }

    let mut handler = rcu_dereference(*head);
    while !handler.is_null() {
        let ret = ((*handler).cb_handler)(skb, err);
        if ret <= 0 {
            return ret;
        }
        handler = rcu_dereference((*handler).next);
    }
    0
}

pub unsafe fn xfrm4_rcv_encap(
    skb: *mut sk_buff,
    nexthdr: i32,
    spi: __be32,
    encap_type: i32,
) -> i32 {
    let head = proto_handlers(nexthdr as u8);

    (*XFRM_TUNNEL_SKB_CB(skb)).tunnel.ip4 = core::ptr::null_mut();
    (*XFRM_SPI_SKB_CB(skb)).family = AF_INET;
    (*XFRM_SPI_SKB_CB(skb)).daddroff = core::mem::offset_of!(iphdr, daddr);

    if head.is_null() {
        icmp_send(skb, ICMP_DEST_UNREACH, ICMP_PORT_UNREACH, 0);
        kfree_skb(skb);
        return 0;
    }

    if skb_dst(skb).is_null() {
        let iph = ip_hdr(skb);
        if ip_route_input_noref(skb, (*iph).daddr, (*iph).saddr,
                                ip4h_dscp(iph), (*skb).dev) != 0 {
            kfree_skb(skb);
            return 0;
        }
    }

    let mut handler = rcu_dereference(*head);
    while !handler.is_null() {
        let ret = ((*handler).input_handler)(skb, nexthdr as u8, spi, encap_type);
        if ret != -EINVAL {
            return ret;
        }
        handler = rcu_dereference((*handler).next);
    }

    icmp_send(skb, ICMP_DEST_UNREACH, ICMP_PORT_UNREACH, 0);
    kfree_skb(skb);
    0
}

unsafe fn xfrm4_esp_rcv(skb: *mut sk_buff) -> i32 {
    (*XFRM_TUNNEL_SKB_CB(skb)).tunnel.ip4 = core::ptr::null_mut();
    let mut handler = rcu_dereference(esp4_handlers);
    while !handler.is_null() {
        let ret = ((*handler).handler)(skb);
        if ret != -EINVAL { return ret; }
        handler = rcu_dereference((*handler).next);
    }
    icmp_send(skb, ICMP_DEST_UNREACH, ICMP_PORT_UNREACH, 0);
    kfree_skb(skb);
    0
}

unsafe fn xfrm4_esp_err(skb: *mut sk_buff, info: u32) -> i32 {
    let mut handler = rcu_dereference(esp4_handlers);
    while !handler.is_null() {
        if !((*handler).err_handler)(skb, info) { return 0; }
        handler = rcu_dereference((*handler).next);
    }
    -ENOENT
}

unsafe fn xfrm4_ah_rcv(skb: *mut sk_buff) -> i32 {
    (*XFRM_TUNNEL_SKB_CB(skb)).tunnel.ip4 = core::ptr::null_mut();
    let mut handler = rcu_dereference(ah4_handlers);
    while !handler.is_null() {
        let ret = ((*handler).handler)(skb);
        if ret != -EINVAL { return ret; }
        handler = rcu_dereference((*handler).next);
    }
    icmp_send(skb, ICMP_DEST_UNREACH, ICMP_PORT_UNREACH, 0);
    kfree_skb(skb);
    0
}

unsafe fn xfrm4_ah_err(skb: *mut sk_buff, info: u32) -> i32 {
    let mut handler = rcu_dereference(ah4_handlers);
    while !handler.is_null() {
        if !((*handler).err_handler)(skb, info) { return 0; }
        handler = rcu_dereference((*handler).next);
    }
    -ENOENT
}

unsafe fn xfrm4_ipcomp_rcv(skb: *mut sk_buff) -> i32 {
    (*XFRM_TUNNEL_SKB_CB(skb)).tunnel.ip4 = core::ptr::null_mut();
    let mut handler = rcu_dereference(ipcomp4_handlers);
    while !handler.is_null() {
        let ret = ((*handler).handler)(skb);
        if ret != -EINVAL { return ret; }
        handler = rcu_dereference((*handler).next);
    }
    icmp_send(skb, ICMP_DEST_UNREACH, ICMP_PORT_UNREACH, 0);
    kfree_skb(skb);
    0
}

unsafe fn xfrm4_ipcomp_err(skb: *mut sk_buff, info: u32) -> i32 {
    let mut handler = rcu_dereference(ipcomp4_handlers);
    while !handler.is_null() {
        if !((*handler).err_handler)(skb, info) { return 0; }
        handler = rcu_dereference((*handler).next);
    }
    -ENOENT
}

static esp4_protocol: net_protocol = net_protocol {
    handler: xfrm4_esp_rcv, err_handler: xfrm4_esp_err, no_policy: 1,
};
static ah4_protocol: net_protocol = net_protocol {
    handler: xfrm4_ah_rcv, err_handler: xfrm4_ah_err, no_policy: 1,
};
static ipcomp4_protocol: net_protocol = net_protocol {
    handler: xfrm4_ipcomp_rcv, err_handler: xfrm4_ipcomp_err, no_policy: 1,
};
static xfrm4_input_afinfo: xfrm_input_afinfo = xfrm_input_afinfo {
    family: AF_INET, callback: xfrm4_rcv_cb,
};

unsafe fn netproto(protocol: u8) -> *const net_protocol {
    match protocol {
        IPPROTO_ESP => &esp4_protocol,
        IPPROTO_AH => &ah4_protocol,
        IPPROTO_COMP => &ipcomp4_protocol,
        _ => core::ptr::null(),
    }
}

pub unsafe fn xfrm4_protocol_register(handler: *mut xfrm4_protocol, protocol: u8) -> i32 {
    let mut ret = -EEXIST;
    let priority = (*handler).priority;
    if proto_handlers(protocol).is_null() || netproto(protocol).is_null() { return -EINVAL; }
    mutex_lock(&raw mut xfrm4_protocol_mutex);
    let head = proto_handlers(protocol);
    let mut add_netproto = (*head).is_null();
    let mut pprev = head;
    let mut t = rcu_dereference_protected(*pprev, lockdep_is_held(&raw mut xfrm4_protocol_mutex));
    while !t.is_null() {
        if (*t).priority < priority { break; }
        if (*t).priority == priority {
            mutex_unlock(&raw mut xfrm4_protocol_mutex);
            return ret;
        }
        pprev = &raw mut (*t).next;
        t = rcu_dereference_protected(*pprev, lockdep_is_held(&raw mut xfrm4_protocol_mutex));
    }
    (*handler).next = *pprev;
    rcu_assign_pointer(*pprev, handler);
    ret = 0;
    mutex_unlock(&raw mut xfrm4_protocol_mutex);
    if add_netproto && inet_add_protocol(netproto(protocol), protocol) != 0 {
        pr_err("%s: can't add protocol\n", "xfrm4_protocol_register");
        ret = -EAGAIN;
    }
    ret
}

pub unsafe fn xfrm4_protocol_deregister(handler: *mut xfrm4_protocol, protocol: u8) -> i32 {
    let mut ret = -ENOENT;
    if proto_handlers(protocol).is_null() || netproto(protocol).is_null() { return -EINVAL; }
    mutex_lock(&raw mut xfrm4_protocol_mutex);
    let head = proto_handlers(protocol);
    let mut pprev = head;
    let mut t = rcu_dereference_protected(*pprev, lockdep_is_held(&raw mut xfrm4_protocol_mutex));
    while !t.is_null() {
        if t == handler {
            *pprev = (*handler).next;
            ret = 0;
            break;
        }
        pprev = &raw mut (*t).next;
        t = rcu_dereference_protected(*pprev, lockdep_is_held(&raw mut xfrm4_protocol_mutex));
    }
    if rcu_dereference_protected(*head, lockdep_is_held(&raw mut xfrm4_protocol_mutex)).is_null()
        && inet_del_protocol(netproto(protocol), protocol) < 0 {
        pr_err("%s: can't remove protocol\n", "xfrm4_protocol_deregister");
        ret = -EAGAIN;
    }
    mutex_unlock(&raw mut xfrm4_protocol_mutex);
    synchronize_net();
    ret
}

pub unsafe fn xfrm4_protocol_init() {
    xfrm_input_register_afinfo(&xfrm4_input_afinfo);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
