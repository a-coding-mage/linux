// SPDX-License-Identifier: GPL-2.0-only
// Translated from nf_tproxy_ipv6.c. External kernel dependencies are supplied
// by the surrounding translation unit.

#[allow(non_camel_case_types)]
pub unsafe fn nf_tproxy_laddr6(
    skb: *mut sk_buff,
    user_laddr: *const in6_addr,
    daddr: *const in6_addr,
) -> *const in6_addr {
    let mut indev: *mut inet6_dev;
    let mut ifa: *mut inet6_ifaddr;
    let mut laddr: *mut in6_addr;

    if !ipv6_addr_any(user_laddr) {
        return user_laddr;
    }
    laddr = core::ptr::null_mut();

    indev = __in6_dev_get((*skb).dev);
    if !indev.is_null() {
        read_lock_bh(&mut (*indev).lock);
        list_for_each_entry!(ifa, (*indev).addr_list, if_list, {
            if (*ifa).flags & (IFA_F_TENTATIVE | IFA_F_DEPRECATED) != 0 {
                continue;
            }

            laddr = &mut (*ifa).addr;
            break;
        });
        read_unlock_bh(&mut (*indev).lock);
    }

    if !laddr.is_null() { laddr } else { daddr }
}

pub unsafe fn nf_tproxy_handle_time_wait6(
    skb: *mut sk_buff,
    tproto: i32,
    thoff: i32,
    net: *mut net,
    laddr: *const in6_addr,
    lport: __be16,
    mut sk: *mut sock,
) -> *mut sock {
    let iph: *const ipv6hdr = ipv6_hdr(skb);
    let mut _hdr: tcphdr = core::mem::zeroed();

    let hp = skb_header_pointer(
        skb,
        thoff,
        core::mem::size_of::<tcphdr>(),
        &mut _hdr as *mut tcphdr as *mut core::ffi::c_void,
    );
    if hp.is_null() {
        inet_twsk_put(inet_twsk(sk));
        return core::ptr::null_mut();
    }

    if (*hp).syn && !(*hp).rst && !(*hp).ack && !(*hp).fin {
        /* SYN to a TIME_WAIT socket, we'd rather redirect it
         * to a listener socket if there's one */
        let sk2 = nf_tproxy_get_sock_v6(
            net,
            skb,
            thoff,
            tproto as u8,
            &(*iph).saddr,
            nf_tproxy_laddr6(skb, laddr, &(*iph).daddr),
            (*hp).source,
            if lport != 0 { lport } else { (*hp).dest },
            (*skb).dev,
            NF_TPROXY_LOOKUP_LISTENER,
        );
        if !sk2.is_null() {
            nf_tproxy_twsk_deschedule_put(inet_twsk(sk));
            sk = sk2;
        }
    }

    sk
}

pub unsafe fn nf_tproxy_get_sock_v6(
    net: *mut net,
    skb: *mut sk_buff,
    thoff: i32,
    protocol: u8,
    saddr: *const in6_addr,
    daddr: *const in6_addr,
    sport: __be16,
    dport: __be16,
    input: *const net_device,
    lookup_type: nf_tproxy_lookup_t,
) -> *mut sock {
    let mut sk: *mut sock;

    match protocol {
        IPPROTO_TCP => {
            let mut _hdr: tcphdr = core::mem::zeroed();
            let hp = skb_header_pointer(
                skb,
                thoff,
                core::mem::size_of::<tcphdr>(),
                &mut _hdr as *mut tcphdr as *mut core::ffi::c_void,
            );
            if hp.is_null() {
                return core::ptr::null_mut();
            }

            sk = match lookup_type {
                NF_TPROXY_LOOKUP_LISTENER => {
                    let mut result = inet6_lookup_listener(
                        net, skb, thoff + __tcp_hdrlen(hp), saddr, sport,
                        daddr, ntohs(dport), (*input).ifindex, 0,
                    );
                    if !result.is_null() && !refcount_inc_not_zero(&mut (*result).sk_refcnt) {
                        result = core::ptr::null_mut();
                    }
                    /* NOTE: listeners bound to 0.0.0.0 are returned too. */
                    result
                }
                NF_TPROXY_LOOKUP_ESTABLISHED =>
                    __inet6_lookup_established(net, saddr, sport, daddr, ntohs(dport), (*input).ifindex, 0),
                _ => {
                    BUG!();
                    core::ptr::null_mut()
                }
            };
        }
        IPPROTO_UDP => {
            sk = udp6_lib_lookup(net, saddr, sport, daddr, dport, (*input).ifindex);
            if !sk.is_null() {
                let connected = (*sk).sk_state == TCP_ESTABLISHED;
                let wildcard = ipv6_addr_any(&(*sk).sk_v6_rcv_saddr);
                /* NOTE: listeners bound to 0.0.0.0 are returned too. */
                if (lookup_type == NF_TPROXY_LOOKUP_ESTABLISHED && (!connected || wildcard))
                    || (lookup_type == NF_TPROXY_LOOKUP_LISTENER && connected)
                {
                    sock_put(sk);
                    sk = core::ptr::null_mut();
                }
            }
        }
        _ => {
            DEBUG_NET_WARN_ON_ONCE!(1);
            sk = core::ptr::null_mut();
        }
    }

    pr_debug!(
        "tproxy socket lookup: proto %u %pI6:%u -> %pI6:%u, lookup type: %d, sock %p\n",
        protocol, saddr, ntohs(sport), daddr, ntohs(dport), lookup_type, sk
    );

    sk
}

// EXPORT_SYMBOL_GPL(nf_tproxy_laddr6);
// EXPORT_SYMBOL_GPL(nf_tproxy_handle_time_wait6);
// EXPORT_SYMBOL_GPL(nf_tproxy_get_sock_v6);
// MODULE_LICENSE("GPL");
// MODULE_AUTHOR("Balazs Scheidler, Krisztian Kovacs");
// MODULE_DESCRIPTION("Netfilter IPv6 transparent proxy support");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
