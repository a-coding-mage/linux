// SPDX-License-Identifier: GPL-2.0-only
/* tunnel4.c: Generic IP tunnel transformer.
 *
 * Copyright (C) 2003 David S. Miller (davem@redhat.com)
 */

// Linux kernel dependencies supplied by other translation units.

static mut tunnel4_handlers: *mut xfrm_tunnel = core::ptr::null_mut();
static mut tunnel64_handlers: *mut xfrm_tunnel = core::ptr::null_mut();
static mut tunnelmpls4_handlers: *mut xfrm_tunnel = core::ptr::null_mut();
static mut tunnel4_mutex: mutex = mutex {};

#[inline]
unsafe fn fam_handlers(family: u16) -> *mut *mut xfrm_tunnel {
    if family == AF_INET {
        &raw mut tunnel4_handlers
    } else if family == AF_INET6 {
        &raw mut tunnel64_handlers
    } else {
        &raw mut tunnelmpls4_handlers
    }
}

unsafe fn xfrm4_tunnel_register(handler: *mut xfrm_tunnel, family: u16) -> i32 {
    let mut pprev: *mut *mut xfrm_tunnel;
    let mut t: *mut xfrm_tunnel;

    let mut ret: i32 = -EEXIST;
    let priority = (*handler).priority;

    mutex_lock(&raw mut tunnel4_mutex);

    pprev = fam_handlers(family);
    loop {
        t = rcu_dereference_protected(*pprev, lockdep_is_held(&raw mut tunnel4_mutex));
        if t.is_null() {
            break;
        }
        if (*t).priority > priority {
            break;
        }
        if (*t).priority == priority {
            mutex_unlock(&raw mut tunnel4_mutex);
            return ret;
        }
        pprev = &raw mut (*t).next;
    }

    (*handler).next = *pprev;
    rcu_assign_pointer(pprev, handler);

    ret = 0;

    mutex_unlock(&raw mut tunnel4_mutex);
    ret
}

unsafe fn xfrm4_tunnel_deregister(handler: *mut xfrm_tunnel, family: u16) -> i32 {
    let mut pprev: *mut *mut xfrm_tunnel;
    let mut t: *mut xfrm_tunnel;
    let mut ret: i32 = -ENOENT;

    mutex_lock(&raw mut tunnel4_mutex);

    pprev = fam_handlers(family);
    loop {
        t = rcu_dereference_protected(*pprev, lockdep_is_held(&raw mut tunnel4_mutex));
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

    mutex_unlock(&raw mut tunnel4_mutex);
    synchronize_net();
    ret
}

unsafe fn tunnel4_rcv(skb: *mut sk_buff) -> i32 {
    if !pskb_may_pull(skb, core::mem::size_of::<iphdr>()) {
        kfree_skb(skb);
        return 0;
    }

    let mut handler = rcu_dereference(tunnel4_handlers);
    while !handler.is_null() {
        if ((*handler).handler)(skb) == 0 {
            return 0;
        }
        handler = rcu_dereference((*handler).next);
    }

    icmp_send(skb, ICMP_DEST_UNREACH, ICMP_PORT_UNREACH, 0);
    kfree_skb(skb);
    0
}

#[cfg(CONFIG_INET_XFRM_TUNNEL)]
unsafe fn tunnel4_rcv_cb(skb: *mut sk_buff, proto: u8, err: i32) -> i32 {
    let head = if proto == IPPROTO_IPIP { tunnel4_handlers } else { tunnel64_handlers };
    let mut handler = rcu_dereference(head);

    while !handler.is_null() {
        if let Some(cb_handler) = (*handler).cb_handler {
            let ret = cb_handler(skb, err);
            if ret <= 0 {
                return ret;
            }
        }
        handler = rcu_dereference((*handler).next);
    }
    0
}

#[cfg(CONFIG_INET_XFRM_TUNNEL)]
static tunnel4_input_afinfo: xfrm_input_afinfo = xfrm_input_afinfo {
    family: AF_INET,
    is_ipip: true,
    callback: Some(tunnel4_rcv_cb),
};

#[cfg(CONFIG_IPV6)]
unsafe fn tunnel64_rcv(skb: *mut sk_buff) -> i32 {
    if !pskb_may_pull(skb, core::mem::size_of::<ipv6hdr>()) {
        kfree_skb(skb);
        return 0;
    }
    let mut handler = rcu_dereference(tunnel64_handlers);
    while !handler.is_null() {
        if ((*handler).handler)(skb) == 0 {
            return 0;
        }
        handler = rcu_dereference((*handler).next);
    }
    icmp_send(skb, ICMP_DEST_UNREACH, ICMP_PORT_UNREACH, 0);
    kfree_skb(skb);
    0
}

#[cfg(CONFIG_MPLS)]
unsafe fn tunnelmpls4_rcv(skb: *mut sk_buff) -> i32 {
    if !pskb_may_pull(skb, core::mem::size_of::<mpls_label>()) {
        kfree_skb(skb);
        return 0;
    }
    let mut handler = rcu_dereference(tunnelmpls4_handlers);
    while !handler.is_null() {
        if ((*handler).handler)(skb) == 0 {
            return 0;
        }
        handler = rcu_dereference((*handler).next);
    }
    icmp_send(skb, ICMP_DEST_UNREACH, ICMP_PORT_UNREACH, 0);
    kfree_skb(skb);
    0
}

unsafe fn tunnel4_err(skb: *mut sk_buff, info: u32) -> i32 {
    let mut handler = rcu_dereference(tunnel4_handlers);
    while !handler.is_null() {
        if ((*handler).err_handler)(skb, info) == 0 {
            return 0;
        }
        handler = rcu_dereference((*handler).next);
    }
    -ENOENT
}

#[cfg(CONFIG_IPV6)]
unsafe fn tunnel64_err(skb: *mut sk_buff, info: u32) -> i32 {
    let mut handler = rcu_dereference(tunnel64_handlers);
    while !handler.is_null() {
        if ((*handler).err_handler)(skb, info) == 0 {
            return 0;
        }
        handler = rcu_dereference((*handler).next);
    }
    -ENOENT
}

#[cfg(CONFIG_MPLS)]
unsafe fn tunnelmpls4_err(skb: *mut sk_buff, info: u32) -> i32 {
    let mut handler = rcu_dereference(tunnelmpls4_handlers);
    while !handler.is_null() {
        if ((*handler).err_handler)(skb, info) == 0 {
            return 0;
        }
        handler = rcu_dereference((*handler).next);
    }
    -ENOENT
}

static tunnel4_protocol: net_protocol = net_protocol {
    handler: Some(tunnel4_rcv),
    err_handler: Some(tunnel4_err),
    no_policy: 1,
};

#[cfg(CONFIG_IPV6)]
static tunnel64_protocol: net_protocol = net_protocol {
    handler: Some(tunnel64_rcv),
    err_handler: Some(tunnel64_err),
    no_policy: 1,
};

#[cfg(CONFIG_MPLS)]
static tunnelmpls4_protocol: net_protocol = net_protocol {
    handler: Some(tunnelmpls4_rcv),
    err_handler: Some(tunnelmpls4_err),
    no_policy: 1,
};

unsafe fn tunnel4_init() -> i32 {
    if inet_add_protocol(&tunnel4_protocol, IPPROTO_IPIP) != 0 {
        return tunnel4_init_err();
    }
    #[cfg(CONFIG_IPV6)]
    if inet_add_protocol(&tunnel64_protocol, IPPROTO_IPV6) != 0 {
        inet_del_protocol(&tunnel4_protocol, IPPROTO_IPIP);
        return tunnel4_init_err();
    }
    #[cfg(CONFIG_MPLS)]
    if inet_add_protocol(&tunnelmpls4_protocol, IPPROTO_MPLS) != 0 {
        inet_del_protocol(&tunnel4_protocol, IPPROTO_IPIP);
        #[cfg(CONFIG_IPV6)]
        inet_del_protocol(&tunnel64_protocol, IPPROTO_IPV6);
        return tunnel4_init_err();
    }
    #[cfg(CONFIG_INET_XFRM_TUNNEL)]
    if xfrm_input_register_afinfo(&tunnel4_input_afinfo) != 0 {
        inet_del_protocol(&tunnel4_protocol, IPPROTO_IPIP);
        #[cfg(CONFIG_IPV6)]
        inet_del_protocol(&tunnel64_protocol, IPPROTO_IPV6);
        #[cfg(CONFIG_MPLS)]
        inet_del_protocol(&tunnelmpls4_protocol, IPPROTO_MPLS);
        return tunnel4_init_err();
    }
    0
}

unsafe fn tunnel4_init_err() -> i32 {
    pr_err!("%s: can't add protocol\n", "tunnel4_init");
    -EAGAIN
}

unsafe fn tunnel4_fini() {
    #[cfg(CONFIG_INET_XFRM_TUNNEL)]
    if xfrm_input_unregister_afinfo(&tunnel4_input_afinfo) != 0 {
        pr_err!("tunnel4 close: can't remove input afinfo\n");
    }
    #[cfg(CONFIG_MPLS)]
    if inet_del_protocol(&tunnelmpls4_protocol, IPPROTO_MPLS) != 0 {
        pr_err!("tunnelmpls4 close: can't remove protocol\n");
    }
    #[cfg(CONFIG_IPV6)]
    if inet_del_protocol(&tunnel64_protocol, IPPROTO_IPV6) != 0 {
        pr_err!("tunnel64 close: can't remove protocol\n");
    }
    if inet_del_protocol(&tunnel4_protocol, IPPROTO_IPIP) != 0 {
        pr_err!("tunnel4 close: can't remove protocol\n");
    }
}

// module_init(tunnel4_init);
// module_exit(tunnel4_fini);
// MODULE_DESCRIPTION("IPv4 XFRM tunnel library");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
