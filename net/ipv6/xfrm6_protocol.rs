// SPDX-License-Identifier: GPL-2.0-or-later
/* xfrm6_protocol.c - Generic xfrm protocol multiplexer for ipv6. */

// Kernel includes and build-time declarations are supplied by other files.

static mut esp6_handlers: *mut xfrm6_protocol = core::ptr::null_mut();
static mut ah6_handlers: *mut xfrm6_protocol = core::ptr::null_mut();
static mut ipcomp6_handlers: *mut xfrm6_protocol = core::ptr::null_mut();
static mut xfrm6_protocol_mutex: mutex = mutex {};

#[inline]
unsafe fn proto_handlers(protocol: u8) -> *mut *mut xfrm6_protocol {
    match protocol {
        IPPROTO_ESP => &raw mut esp6_handlers,
        IPPROTO_AH => &raw mut ah6_handlers,
        IPPROTO_COMP => &raw mut ipcomp6_handlers,
        _ => core::ptr::null_mut(),
    }
}

unsafe fn xfrm6_rcv_cb(skb: *mut sk_buff, protocol: u8, err: i32) -> i32 {
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

pub unsafe fn xfrm6_rcv_encap(
    skb: *mut sk_buff,
    nexthdr: i32,
    spi: __be32,
    encap_type: i32,
) -> i32 {
    let head = proto_handlers(nexthdr as u8);

    (*xfrm_tunnel_skb_cb(skb)).tunnel.ip6 = core::ptr::null_mut();
    (*xfrm_spi_skb_cb(skb)).family = AF_INET6;
    (*xfrm_spi_skb_cb(skb)).daddroff = core::mem::offset_of!(ipv6hdr, daddr);

    if !head.is_null() {
        if skb_dst(skb).is_null() {
            let ip6h = ipv6_hdr(skb);
            let mut fl6 = flowi6 {
                flowi6_iif: (*(*skb).dev).ifindex,
                daddr: (*ip6h).daddr,
                saddr: (*ip6h).saddr,
                flowlabel: ip6_flowinfo(ip6h),
                flowi6_mark: (*skb).mark,
                flowi6_proto: (*ip6h).nexthdr,
            };
            let dst = ip6_route_input_lookup(
                dev_net((*skb).dev), (*skb).dev, &mut fl6, skb, RT6_LOOKUP_F_HAS_SADDR,
            );
            if (*dst).error != 0 {
                dst_release(dst);
                return xfrm6_rcv_encap_drop(skb);
            }
            skb_dst_set(skb, dst);
        }

        let mut handler = rcu_dereference(*head);
        while !handler.is_null() {
            let ret = ((*handler).input_handler)(skb, nexthdr, spi, encap_type);
            if ret != -EINVAL {
                return ret;
            }
            handler = rcu_dereference((*handler).next);
        }
    }

    icmpv6_send(skb, ICMPV6_DEST_UNREACH, ICMPV6_PORT_UNREACH, 0);
    xfrm6_rcv_encap_drop(skb)
}

#[inline]
unsafe fn xfrm6_rcv_encap_drop(skb: *mut sk_buff) -> i32 {
    kfree_skb(skb);
    0
}

unsafe fn xfrm6_esp_rcv(skb: *mut sk_buff) -> i32 {
    (*xfrm_tunnel_skb_cb(skb)).tunnel.ip6 = core::ptr::null_mut();
    let mut handler = rcu_dereference(esp6_handlers);
    while !handler.is_null() {
        let ret = ((*handler).handler)(skb);
        if ret != -EINVAL { return ret; }
        handler = rcu_dereference((*handler).next);
    }
    icmpv6_send(skb, ICMPV6_DEST_UNREACH, ICMPV6_PORT_UNREACH, 0);
    kfree_skb(skb);
    0
}

unsafe fn xfrm6_esp_err(skb: *mut sk_buff, opt: *mut inet6_skb_parm, type_: u8, code: u8, offset: i32, info: __be32) -> i32 {
    let mut handler = rcu_dereference(esp6_handlers);
    while !handler.is_null() { if ((*handler).err_handler)(skb, opt, type_, code, offset, info) == 0 { return 0; } handler = rcu_dereference((*handler).next); }
    -ENOENT
}

unsafe fn xfrm6_ah_rcv(skb: *mut sk_buff) -> i32 {
    (*xfrm_tunnel_skb_cb(skb)).tunnel.ip6 = core::ptr::null_mut();
    let mut handler = rcu_dereference(ah6_handlers);
    while !handler.is_null() { let ret = ((*handler).handler)(skb); if ret != -EINVAL { return ret; } handler = rcu_dereference((*handler).next); }
    icmpv6_send(skb, ICMPV6_DEST_UNREACH, ICMPV6_PORT_UNREACH, 0); kfree_skb(skb); 0
}

unsafe fn xfrm6_ah_err(skb: *mut sk_buff, opt: *mut inet6_skb_parm, type_: u8, code: u8, offset: i32, info: __be32) -> i32 {
    let mut handler = rcu_dereference(ah6_handlers);
    while !handler.is_null() { if ((*handler).err_handler)(skb, opt, type_, code, offset, info) == 0 { return 0; } handler = rcu_dereference((*handler).next); } -ENOENT
}

unsafe fn xfrm6_ipcomp_rcv(skb: *mut sk_buff) -> i32 {
    (*xfrm_tunnel_skb_cb(skb)).tunnel.ip6 = core::ptr::null_mut();
    let mut handler = rcu_dereference(ipcomp6_handlers);
    while !handler.is_null() { let ret = ((*handler).handler)(skb); if ret != -EINVAL { return ret; } handler = rcu_dereference((*handler).next); }
    icmpv6_send(skb, ICMPV6_DEST_UNREACH, ICMPV6_PORT_UNREACH, 0); kfree_skb(skb); 0
}

unsafe fn xfrm6_ipcomp_err(skb: *mut sk_buff, opt: *mut inet6_skb_parm, type_: u8, code: u8, offset: i32, info: __be32) -> i32 {
    let mut handler = rcu_dereference(ipcomp6_handlers);
    while !handler.is_null() { if ((*handler).err_handler)(skb, opt, type_, code, offset, info) == 0 { return 0; } handler = rcu_dereference((*handler).next); } -ENOENT
}

static esp6_protocol: inet6_protocol = inet6_protocol { handler: Some(xfrm6_esp_rcv), err_handler: Some(xfrm6_esp_err), flags: INET6_PROTO_NOPOLICY };
static ah6_protocol: inet6_protocol = inet6_protocol { handler: Some(xfrm6_ah_rcv), err_handler: Some(xfrm6_ah_err), flags: INET6_PROTO_NOPOLICY };
static ipcomp6_protocol: inet6_protocol = inet6_protocol { handler: Some(xfrm6_ipcomp_rcv), err_handler: Some(xfrm6_ipcomp_err), flags: INET6_PROTO_NOPOLICY };
static xfrm6_input_afinfo: xfrm_input_afinfo = xfrm_input_afinfo { family: AF_INET6, callback: Some(xfrm6_rcv_cb) };

unsafe fn netproto(protocol: u8) -> *const inet6_protocol {
    match protocol { IPPROTO_ESP => &raw const esp6_protocol, IPPROTO_AH => &raw const ah6_protocol, IPPROTO_COMP => &raw const ipcomp6_protocol, _ => core::ptr::null() }
}

// Registration and deregistration retain the source locking, priority ordering,
// RCU publication, protocol installation, and network synchronization semantics.
pub unsafe fn xfrm6_protocol_register(handler: *mut xfrm6_protocol, protocol: u8) -> i32 {
    let head = proto_handlers(protocol); let proto = netproto(protocol);
    if head.is_null() || proto.is_null() { return -EINVAL; }
    mutex_lock(&raw mut xfrm6_protocol_mutex);
    let mut add_netproto = rcu_dereference_protected(*head, lockdep_is_held(&raw mut xfrm6_protocol_mutex)).is_null();
    let priority = (*handler).priority; let mut pprev = head; let mut t;
    loop { t = rcu_dereference_protected(*pprev, lockdep_is_held(&raw mut xfrm6_protocol_mutex)); if t.is_null() { break; } if (*t).priority < priority { break; } if (*t).priority == priority { mutex_unlock(&raw mut xfrm6_protocol_mutex); return -EEXIST; } pprev = &mut (*t).next; }
    (*handler).next = *pprev; rcu_assign_pointer(pprev, handler); mutex_unlock(&raw mut xfrm6_protocol_mutex);
    if add_netproto && inet6_add_protocol(proto, protocol) != 0 { pr_err!("%s: can't add protocol\n", "xfrm6_protocol_register"); return -EAGAIN; } 0
}

pub unsafe fn xfrm6_protocol_deregister(handler: *mut xfrm6_protocol, protocol: u8) -> i32 {
    let head = proto_handlers(protocol); let proto = netproto(protocol); if head.is_null() || proto.is_null() { return -EINVAL; }
    mutex_lock(&raw mut xfrm6_protocol_mutex); let mut pprev = head; let mut ret = -ENOENT;
    loop { let t = rcu_dereference_protected(*pprev, lockdep_is_held(&raw mut xfrm6_protocol_mutex)); if t.is_null() { break; } if t == handler { *pprev = (*handler).next; ret = 0; break; } pprev = &mut (*t).next; }
    if rcu_dereference_protected(*head, lockdep_is_held(&raw mut xfrm6_protocol_mutex)).is_null() && inet6_del_protocol(proto, protocol) < 0 { pr_err!("%s: can't remove protocol\n", "xfrm6_protocol_deregister"); ret = -EAGAIN; }
    mutex_unlock(&raw mut xfrm6_protocol_mutex); synchronize_net(); ret
}

pub unsafe fn xfrm6_protocol_init() -> i32 { xfrm_input_register_afinfo(&raw const xfrm6_input_afinfo) }
pub unsafe fn xfrm6_protocol_fini() { xfrm_input_unregister_afinfo(&raw const xfrm6_input_afinfo); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
