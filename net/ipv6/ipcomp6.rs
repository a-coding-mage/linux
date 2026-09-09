// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * IP Payload Compression Protocol (IPComp) for IPv6 - RFC3173
 *
 * Copyright (C)2003 USAGI/WIDE Project
 *
 * Author\tMitsuru KANDA  <mk@linux-ipv6.org>
 */
/*
 * [Memo]
 *
 * Outbound:
 *  The compression of IP datagram MUST be done before AH/ESP processing,
 *  fragmentation, and the addition of Hop-by-Hop/Routing header.
 *
 * Inbound:
 *  The decompression of IP datagram MUST be done after the reassembly,
 *  AH/ESP processing.
 */

// C headers supplied by the surrounding kernel translation unit.

unsafe fn ipcomp6_err(
    skb: *mut sk_buff,
    _opt: *mut inet6_skb_parm,
    typ: u8,
    _code: u8,
    offset: i32,
    info: __be32,
) -> i32 {
    let net = dev_net((*skb).dev);
    let spi: __be32;
    let iph = (*skb).data as *const ipv6hdr;
    let ipcomph = ((*skb).data.add(offset as usize)) as *mut ip_comp_hdr;
    let x: *mut xfrm_state;

    if typ != ICMPV6_PKT_TOOBIG && typ != NDISC_REDIRECT {
        return 0;
    }

    spi = htonl(ntohs((*ipcomph).cpi) as u32);
    x = xfrm_state_lookup(
        net,
        (*skb).mark,
        &(*iph).daddr as *const _ as *const xfrm_address_t,
        spi,
        IPPROTO_COMP,
        AF_INET6,
    );
    if x.is_null() {
        return 0;
    }

    if typ == NDISC_REDIRECT {
        ip6_redirect(skb, net, (*(*skb).dev).ifindex, 0, sock_net_uid(net, core::ptr::null_mut()));
    } else {
        ip6_update_pmtu(skb, net, info, 0, 0, sock_net_uid(net, core::ptr::null_mut()));
    }
    xfrm_state_put(x);

    0
}

static mut xfrm_state_lock_key: lock_class_key = lock_class_key {};

unsafe fn ipcomp6_tunnel_create(x: *mut xfrm_state) -> *mut xfrm_state {
    let net = xs_net(x);
    let mut t: *mut xfrm_state = core::ptr::null_mut();

    t = xfrm_state_alloc(net);
    if t.is_null() {
        return t;
    }
    lockdep_set_class(&mut (*t).lock, &raw mut xfrm_state_lock_key);

    (*t).id.proto = IPPROTO_IPV6;
    (*t).id.spi = xfrm6_tunnel_alloc_spi(net, &mut (*x).props.saddr as *mut _ as *mut xfrm_address_t);
    if (*t).id.spi == 0 {
        goto_error(t);
        return core::ptr::null_mut();
    }

    memcpy((*t).id.daddr.a6.as_mut_ptr() as *mut _, (*x).id.daddr.a6.as_ptr() as *const _, size_of::<in6_addr>());
    memcpy(&mut (*t).sel as *mut _, &(*x).sel as *const _, size_of_val(&(*t).sel));
    (*t).props.family = AF_INET6;
    (*t).props.mode = (*x).props.mode;
    memcpy((*t).props.saddr.a6.as_mut_ptr() as *mut _, (*x).props.saddr.a6.as_ptr() as *const _, size_of::<in6_addr>());
    memcpy(&mut (*t).mark as *mut _, &(*x).mark as *const _, size_of_val(&(*t).mark));
    (*t).if_id = (*x).if_id;

    if xfrm_init_state(t, core::ptr::null_mut()) != 0 {
        goto_error(t);
        return core::ptr::null_mut();
    }

    atomic_set(&mut (*t).tunnel_users, 1);
    t
}

unsafe fn goto_error(t: *mut xfrm_state) {
    (*t).km.state = XFRM_STATE_DEAD;
    xfrm_state_put(t);
}

unsafe fn ipcomp6_tunnel_attach(x: *mut xfrm_state) -> i32 {
    let net = xs_net(x);
    let mut err = 0;
    let mut t: *mut xfrm_state = core::ptr::null_mut();
    let spi = xfrm6_tunnel_spi_lookup(net, &mut (*x).props.saddr as *mut _ as *mut xfrm_address_t);
    let mark = (*x).mark.m & (*x).mark.v;

    if spi != 0 {
        t = xfrm_state_lookup(net, mark, &mut (*x).id.daddr as *mut _ as *mut xfrm_address_t, spi, IPPROTO_IPV6, AF_INET6);
    }
    if t.is_null() {
        t = ipcomp6_tunnel_create(x);
        if t.is_null() {
            err = -EINVAL;
            return err;
        }
        xfrm_state_insert(t);
        xfrm_state_hold(t);
    }
    (*x).tunnel = t;
    atomic_inc(&mut (*t).tunnel_users);
    err
}

unsafe fn ipcomp6_init_state(x: *mut xfrm_state, extack: *mut netlink_ext_ack) -> i32 {
    let mut err = -EINVAL;
    (*x).props.header_len = 0;
    match (*x).props.mode {
        XFRM_MODE_TRANSPORT => (),
        XFRM_MODE_TUNNEL => (*x).props.header_len += size_of::<ipv6hdr>(),
        _ => {
            NL_SET_ERR_MSG(extack, "Unsupported XFRM mode for IPcomp");
            return err;
        }
    }
    err = ipcomp_init_state(x, extack);
    if err != 0 { return err; }
    if (*x).props.mode == XFRM_MODE_TUNNEL {
        err = ipcomp6_tunnel_attach(x);
        if err != 0 {
            NL_SET_ERR_MSG(extack, "Kernel error: failed to initialize the associated state");
            return err;
        }
    }
    0
}

unsafe fn ipcomp6_rcv_cb(_skb: *mut sk_buff, _err: i32) -> i32 { 0 }

static mut ipcomp6_type: xfrm_type = xfrm_type {
    owner: THIS_MODULE,
    proto: IPPROTO_COMP,
    init_state: Some(ipcomp6_init_state),
    destructor: Some(ipcomp_destroy),
    input: Some(ipcomp_input),
    output: Some(ipcomp_output),
};

static mut ipcomp6_protocol: xfrm6_protocol = xfrm6_protocol {
    handler: Some(xfrm6_rcv),
    input_handler: Some(xfrm_input),
    cb_handler: Some(ipcomp6_rcv_cb),
    err_handler: Some(ipcomp6_err),
    priority: 0,
};

unsafe fn ipcomp6_init() -> i32 {
    if xfrm_register_type(&ipcomp6_type, AF_INET6) < 0 {
        pr_info!("%s: can't add xfrm type\n", "ipcomp6_init");
        return -EAGAIN;
    }
    if xfrm6_protocol_register(&ipcomp6_protocol, IPPROTO_COMP) < 0 {
        pr_info!("%s: can't add protocol\n", "ipcomp6_init");
        xfrm_unregister_type(&ipcomp6_type, AF_INET6);
        return -EAGAIN;
    }
    0
}

unsafe fn ipcomp6_fini() {
    if xfrm6_protocol_deregister(&ipcomp6_protocol, IPPROTO_COMP) < 0 {
        pr_info!("%s: can't remove protocol\n", "ipcomp6_fini");
    }
    xfrm_unregister_type(&ipcomp6_type, AF_INET6);
}

module_init!(ipcomp6_init);
module_exit!(ipcomp6_fini);
MODULE_LICENSE!("GPL");
MODULE_DESCRIPTION!("IP Payload Compression Protocol (IPComp) for IPv6 - RFC3173");
MODULE_AUTHOR!("Mitsuru KANDA <mk@linux-ipv6.org>");
MODULE_ALIAS_XFRM_TYPE!(AF_INET6, XFRM_PROTO_COMP);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
