// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * IP Payload Compression Protocol (IPComp) - RFC3173.
 *
 * Copyright (c) 2003 James Morris <jmorris@intercode.com.au>
 *
 * Todo:
 *   - Tunable compression parameters.
 *   - Compression stats.
 *   - Adaptive compression.
 */

// Linux kernel dependencies supplied by the surrounding translation unit.

unsafe fn ipcomp4_err(skb: *mut sk_buff, info: u32) -> i32 {
    let net: *mut net = unsafe { dev_net((*skb).dev) };
    let spi: __be32;
    let iph: *const iphdr = unsafe { (*skb).data as *const iphdr };
    let ipch: *mut ip_comp_hdr = unsafe {
        ((*skb).data.add(((*iph).ihl as usize) << 2)) as *mut ip_comp_hdr
    };
    let x: *mut xfrm_state;

    unsafe {
        match (*icmp_hdr(skb)).type_ {
            ICMP_DEST_UNREACH => {
                if (*icmp_hdr(skb)).code != ICMP_FRAG_NEEDED {
                    return 0;
                }
            }
            ICMP_REDIRECT => {}
            _ => return 0,
        }

        spi = htonl(ntohs((*ipch).cpi) as u32);
        x = xfrm_state_lookup(
            net,
            (*skb).mark,
            &((*iph).daddr as xfrm_address_t),
            spi,
            IPPROTO_COMP,
            AF_INET,
        );
        if x.is_null() {
            return 0;
        }

        if (*icmp_hdr(skb)).type_ == ICMP_DEST_UNREACH {
            ipv4_update_pmtu(skb, net, info, 0, IPPROTO_COMP);
        } else {
            ipv4_redirect(skb, net, 0, IPPROTO_COMP);
        }
        xfrm_state_put(x);
    }

    0
}

/* We always hold one tunnel user reference to indicate a tunnel */
static mut xfrm_state_lock_key: lock_class_key = lock_class_key::default();

unsafe fn ipcomp_tunnel_create(x: *mut xfrm_state) -> *mut xfrm_state {
    let net: *mut net = unsafe { xs_net(x) };
    let mut t: *mut xfrm_state;

    unsafe {
        t = xfrm_state_alloc(net);
        if t.is_null() {
            return t;
        }
        lockdep_set_class(&mut (*t).lock, &xfrm_state_lock_key);

        (*t).id.proto = IPPROTO_IPIP;
        (*t).id.spi = (*x).props.saddr.a4;
        (*t).id.daddr.a4 = (*x).id.daddr.a4;
        memcpy(&mut (*t).sel as *mut _ as *mut c_void, &(*x).sel as *const _ as *const c_void, core::mem::size_of_val(&(*t).sel));
        (*t).props.family = AF_INET;
        (*t).props.mode = (*x).props.mode;
        (*t).props.saddr.a4 = (*x).props.saddr.a4;
        (*t).props.flags = (*x).props.flags;
        (*t).props.extra_flags = (*x).props.extra_flags;
        memcpy(&mut (*t).mark as *mut _ as *mut c_void, &(*x).mark as *const _ as *const c_void, core::mem::size_of_val(&(*t).mark));
        (*t).if_id = (*x).if_id;

        if xfrm_init_state(t, core::ptr::null_mut()) != 0 {
            (*t).km.state = XFRM_STATE_DEAD;
            xfrm_state_put(t);
            return core::ptr::null_mut();
        }

        atomic_set(&mut (*t).tunnel_users, 1);
    }
    t
}

/*
 * Must be protected by xfrm_cfg_mutex.  State and tunnel user references are
 * always incremented on success.
 */
unsafe fn ipcomp_tunnel_attach(x: *mut xfrm_state) -> i32 {
    let net: *mut net = unsafe { xs_net(x) };
    let mut err: i32 = 0;
    let mut t: *mut xfrm_state;
    let mark: u32 = unsafe { (*x).mark.v & (*x).mark.m };

    unsafe {
        t = xfrm_state_lookup(net, mark, &(*x).id.daddr.a4 as *const _ as *const xfrm_address_t, (*x).props.saddr.a4, IPPROTO_IPIP, AF_INET);
        if t.is_null() {
            t = ipcomp_tunnel_create(x);
            if t.is_null() {
                err = -EINVAL;
                return err;
            }
            xfrm_state_insert(t);
            xfrm_state_hold(t);
        }
        (*x).tunnel = t;
        atomic_inc(&mut (*t).tunnel_users);
    }
    err
}

unsafe fn ipcomp4_init_state(x: *mut xfrm_state, extack: *mut netlink_ext_ack) -> i32 {
    let mut err: i32 = -EINVAL;

    unsafe {
        (*x).props.header_len = 0;
        match (*x).props.mode {
            XFRM_MODE_TRANSPORT => {}
            XFRM_MODE_TUNNEL => (*x).props.header_len += core::mem::size_of::<iphdr>(),
            _ => {
                NL_SET_ERR_MSG(extack, "Unsupported XFRM mode for IPcomp");
                return err;
            }
        }

        err = ipcomp_init_state(x, extack);
        if err != 0 {
            return err;
        }

        if (*x).props.mode == XFRM_MODE_TUNNEL {
            err = ipcomp_tunnel_attach(x);
            if err != 0 {
                NL_SET_ERR_MSG(extack, "Kernel error: failed to initialize the associated state");
                return err;
            }
        }
    }
    0
}

unsafe fn ipcomp4_rcv_cb(_skb: *mut sk_buff, _err: i32) -> i32 {
    0
}

static mut ipcomp_type: xfrm_type = xfrm_type {
    owner: THIS_MODULE,
    proto: IPPROTO_COMP,
    init_state: Some(ipcomp4_init_state),
    destructor: Some(ipcomp_destroy),
    input: Some(ipcomp_input),
    output: Some(ipcomp_output),
};

static mut ipcomp4_protocol: xfrm4_protocol = xfrm4_protocol {
    handler: Some(xfrm4_rcv),
    input_handler: Some(xfrm_input),
    cb_handler: Some(ipcomp4_rcv_cb),
    err_handler: Some(ipcomp4_err),
    priority: 0,
};

unsafe fn ipcomp4_init() -> i32 {
    if xfrm_register_type(&mut ipcomp_type, AF_INET) < 0 {
        pr_info!("%s: can't add xfrm type\n", __func__);
        return -EAGAIN;
    }
    if xfrm4_protocol_register(&mut ipcomp4_protocol, IPPROTO_COMP) < 0 {
        pr_info!("%s: can't add protocol\n", __func__);
        xfrm_unregister_type(&mut ipcomp_type, AF_INET);
        return -EAGAIN;
    }
    0
}

unsafe fn ipcomp4_fini() {
    if xfrm4_protocol_deregister(&mut ipcomp4_protocol, IPPROTO_COMP) < 0 {
        pr_info!("%s: can't remove protocol\n", __func__);
    }
    xfrm_unregister_type(&mut ipcomp_type, AF_INET);
}

module_init!(ipcomp4_init);
module_exit!(ipcomp4_fini);

MODULE_LICENSE!("GPL");
MODULE_DESCRIPTION!("IP Payload Compression Protocol (IPComp/IPv4) - RFC3173");
MODULE_AUTHOR!("James Morris <jmorris@intercode.com.au>");

MODULE_ALIAS_XFRM_TYPE!(AF_INET, XFRM_PROTO_COMP);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
