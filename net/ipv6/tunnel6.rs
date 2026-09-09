// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright (C)2003,2004 USAGI/WIDE Project
 *
 * Authors Mitsuru KANDA  <mk@linux-ipv6.org>
 *         YOSHIFUJI Hideaki <yoshfuji@linux-ipv6.org>
 */

// Linux kernel dependencies supplied by the surrounding translation unit.

static mut tunnel6_handlers: *mut xfrm6_tunnel = core::ptr::null_mut();
static mut tunnel46_handlers: *mut xfrm6_tunnel = core::ptr::null_mut();
static mut tunnelmpls6_handlers: *mut xfrm6_tunnel = core::ptr::null_mut();
static mut tunnel6_mutex: mutex = mutex::new();

#[inline]
unsafe fn xfrm6_tunnel_mpls_supported() -> i32 {
    IS_ENABLED(CONFIG_MPLS)
}

unsafe fn xfrm6_tunnel_register(handler: *mut xfrm6_tunnel, family: u16) -> i32 {
    let mut pprev: *mut *mut xfrm6_tunnel;
    let mut t: *mut xfrm6_tunnel;
    let mut ret: i32 = -EEXIST;
    let priority = (*handler).priority;

    mutex_lock(&raw mut tunnel6_mutex);

    pprev = match family {
        AF_INET6 => &raw mut tunnel6_handlers,
        AF_INET => &raw mut tunnel46_handlers,
        AF_MPLS => &raw mut tunnelmpls6_handlers,
        _ => {
            mutex_unlock(&raw mut tunnel6_mutex);
            return ret;
        }
    };

    loop {
        t = rcu_dereference_protected(*pprev, lockdep_is_held(&raw mut tunnel6_mutex));
        if t.is_null() {
            break;
        }
        if (*t).priority > priority {
            break;
        }
        if (*t).priority == priority {
            mutex_unlock(&raw mut tunnel6_mutex);
            return ret;
        }
        pprev = &raw mut (*t).next;
    }

    (*handler).next = *pprev;
    rcu_assign_pointer(pprev, handler);
    ret = 0;

    mutex_unlock(&raw mut tunnel6_mutex);
    ret
}

unsafe fn xfrm6_tunnel_deregister(handler: *mut xfrm6_tunnel, family: u16) -> i32 {
    let mut pprev: *mut *mut xfrm6_tunnel;
    let mut t: *mut xfrm6_tunnel;
    let mut ret: i32 = -ENOENT;

    mutex_lock(&raw mut tunnel6_mutex);

    pprev = match family {
        AF_INET6 => &raw mut tunnel6_handlers,
        AF_INET => &raw mut tunnel46_handlers,
        AF_MPLS => &raw mut tunnelmpls6_handlers,
        _ => {
            mutex_unlock(&raw mut tunnel6_mutex);
            synchronize_net();
            return ret;
        }
    };

    loop {
        t = rcu_dereference_protected(*pprev, lockdep_is_held(&raw mut tunnel6_mutex));
        if t.is_null() {
            break;
        }
        if t == handler {
            *pprev = (*handler).next;
            ret = 0;
            break;
        }
        pprev = &raw mut (*t).next;
    }

    mutex_unlock(&raw mut tunnel6_mutex);
    synchronize_net();
    ret
}

unsafe fn tunnelmpls6_rcv(skb: *mut sk_buff) -> i32 {
    if !pskb_may_pull(skb, core::mem::size_of::<ipv6hdr>()) {
        kfree_skb(skb);
        return 0;
    }
    let mut handler = rcu_dereference(tunnelmpls6_handlers);
    while !handler.is_null() {
        if ((*handler).handler)(skb) == 0 { return 0; }
        handler = rcu_dereference((*handler).next);
    }
    icmpv6_send(skb, ICMPV6_DEST_UNREACH, ICMPV6_PORT_UNREACH, 0);
    kfree_skb(skb);
    0
}

unsafe fn tunnel6_rcv(skb: *mut sk_buff) -> i32 {
    if !pskb_may_pull(skb, core::mem::size_of::<ipv6hdr>()) {
        kfree_skb(skb);
        return 0;
    }
    let mut handler = rcu_dereference(tunnel6_handlers);
    while !handler.is_null() {
        if ((*handler).handler)(skb) == 0 { return 0; }
        handler = rcu_dereference((*handler).next);
    }
    icmpv6_send(skb, ICMPV6_DEST_UNREACH, ICMPV6_PORT_UNREACH, 0);
    kfree_skb(skb);
    0
}

#[cfg(CONFIG_INET6_XFRM_TUNNEL)]
unsafe fn tunnel6_rcv_cb(skb: *mut sk_buff, proto: u8, err: i32) -> i32 {
    let mut handler = if proto == IPPROTO_IPV6 { tunnel6_handlers } else { tunnel46_handlers };
    while !handler.is_null() {
        if let Some(cb) = (*handler).cb_handler {
            let ret = cb(skb, err);
            if ret <= 0 { return ret; }
        }
        handler = rcu_dereference((*handler).next);
    }
    0
}

unsafe fn tunnel46_rcv(skb: *mut sk_buff) -> i32 {
    if !pskb_may_pull(skb, core::mem::size_of::<iphdr>()) {
        kfree_skb(skb); return 0;
    }
    let mut handler = rcu_dereference(tunnel46_handlers);
    while !handler.is_null() {
        if ((*handler).handler)(skb) == 0 { return 0; }
        handler = rcu_dereference((*handler).next);
    }
    icmpv6_send(skb, ICMPV6_DEST_UNREACH, ICMPV6_PORT_UNREACH, 0);
    kfree_skb(skb); 0
}

unsafe fn tunnel6_err(skb: *mut sk_buff, opt: *mut inet6_skb_parm, ty: u8, code: u8, offset: i32, info: __be32) -> i32 {
    let mut handler = rcu_dereference(tunnel6_handlers);
    while !handler.is_null() { if ((*handler).err_handler)(skb, opt, ty, code, offset, info) == 0 { return 0; } handler = rcu_dereference((*handler).next); }
    -ENOENT
}

unsafe fn tunnel46_err(skb: *mut sk_buff, opt: *mut inet6_skb_parm, ty: u8, code: u8, offset: i32, info: __be32) -> i32 {
    let mut handler = rcu_dereference(tunnel46_handlers);
    while !handler.is_null() { if ((*handler).err_handler)(skb, opt, ty, code, offset, info) == 0 { return 0; } handler = rcu_dereference((*handler).next); }
    -ENOENT
}

unsafe fn tunnelmpls6_err(skb: *mut sk_buff, opt: *mut inet6_skb_parm, ty: u8, code: u8, offset: i32, info: __be32) -> i32 {
    let mut handler = rcu_dereference(tunnelmpls6_handlers);
    while !handler.is_null() { if ((*handler).err_handler)(skb, opt, ty, code, offset, info) == 0 { return 0; } handler = rcu_dereference((*handler).next); }
    -ENOENT
}

static tunnel6_protocol: inet6_protocol = inet6_protocol { handler: tunnel6_rcv, err_handler: tunnel6_err, flags: INET6_PROTO_NOPOLICY | INET6_PROTO_FINAL };
static tunnel46_protocol: inet6_protocol = inet6_protocol { handler: tunnel46_rcv, err_handler: tunnel46_err, flags: INET6_PROTO_NOPOLICY | INET6_PROTO_FINAL };
static tunnelmpls6_protocol: inet6_protocol = inet6_protocol { handler: tunnelmpls6_rcv, err_handler: tunnelmpls6_err, flags: INET6_PROTO_NOPOLICY | INET6_PROTO_FINAL };

unsafe fn tunnel6_init() -> i32 {
    if inet6_add_protocol(&tunnel6_protocol, IPPROTO_IPV6) != 0 { pr_err!("%s: can't add protocol\n", "tunnel6_init"); return -EAGAIN; }
    if inet6_add_protocol(&tunnel46_protocol, IPPROTO_IPIP) != 0 { pr_err!("%s: can't add protocol\n", "tunnel6_init"); inet6_del_protocol(&tunnel6_protocol, IPPROTO_IPV6); return -EAGAIN; }
    if xfrm6_tunnel_mpls_supported() != 0 && inet6_add_protocol(&tunnelmpls6_protocol, IPPROTO_MPLS) != 0 { pr_err!("%s: can't add protocol\n", "tunnel6_init"); inet6_del_protocol(&tunnel6_protocol, IPPROTO_IPV6); inet6_del_protocol(&tunnel46_protocol, IPPROTO_IPIP); return -EAGAIN; }
    0
}

unsafe fn tunnel6_fini() {
    if inet6_del_protocol(&tunnel46_protocol, IPPROTO_IPIP) != 0 { pr_err!("%s: can't remove protocol\n", "tunnel6_fini"); }
    if inet6_del_protocol(&tunnel6_protocol, IPPROTO_IPV6) != 0 { pr_err!("%s: can't remove protocol\n", "tunnel6_fini"); }
    if xfrm6_tunnel_mpls_supported() != 0 && inet6_del_protocol(&tunnelmpls6_protocol, IPPROTO_MPLS) != 0 { pr_err!("%s: can't remove protocol\n", "tunnel6_fini"); }
}

// module_init(tunnel6_init); module_exit(tunnel6_fini);
// MODULE_DESCRIPTION("IP-in-IPv6 tunnel driver"); MODULE_LICENSE("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
