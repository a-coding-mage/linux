// SPDX-License-Identifier: GPL-2.0-only
/* Support ct functions for openvswitch and used by OVS and TC conntrack. */

// Dependencies supplied by the surrounding kernel translation.

/* 'skb' should already be pulled to nh_ofs. */
pub unsafe fn nf_ct_helper(
    skb: *mut sk_buff,
    ct: *mut nf_conn,
    mut ctinfo: ip_conntrack_info,
    mut proto: u16,
) -> i32 {
    let mut helper_cb: Option<unsafe extern "C" fn(*mut sk_buff, u32, *mut nf_conn, ip_conntrack_info) -> i32>;
    let helper: *const nf_conntrack_helper;
    let help: *const nf_conn_help;
    let mut protoff: u32;
    let mut err: i32;

    if ctinfo == IP_CT_RELATED_REPLY {
        return NF_ACCEPT;
    }

    help = nfct_help(ct);
    if help.is_null() {
        return NF_ACCEPT;
    }

    helper = rcu_dereference((*help).helper);
    if helper.is_null() {
        return NF_ACCEPT;
    }

    if (*helper).nfproto != NFPROTO_UNSPEC && (*helper).nfproto != proto {
        return NF_ACCEPT;
    }

    match proto {
        NFPROTO_IPV4 => {
            protoff = ip_hdrlen(skb);
            proto = (*ip_hdr(skb)).protocol;
        }
        NFPROTO_IPV6 => {
            let mut nexthdr: u8 = (*ipv6_hdr(skb)).nexthdr;
            let mut frag_off: __be16 = 0;
            let ofs: i32 = ipv6_skip_exthdr(skb, core::mem::size_of::<ipv6hdr>() as i32, &mut nexthdr, &mut frag_off);
            if ofs < 0 || (frag_off & htons(!0x7)) != 0 {
                pr_debug!("proto header not found\n");
                return NF_ACCEPT;
            }
            protoff = ofs as u32;
            proto = nexthdr as u16;
        }
        _ => {
            WARN_ONCE!(true, "helper invoked on non-IP family!");
            return NF_DROP;
        }
    }

    if (*helper).l4proto != proto {
        return NF_ACCEPT;
    }

    helper_cb = rcu_dereference((*helper).help);
    if helper_cb.is_none() {
        return NF_ACCEPT;
    }

    err = helper_cb.unwrap()(skb, protoff, ct, ctinfo);
    if err != NF_ACCEPT {
        return err;
    }

    /* Adjust seqs after helper.  This is needed due to some helpers (e.g.,
     * FTP with NAT) adusting the TCP payload size when mangling IP
     * addresses and/or port numbers in the text-based control connection.
     */
    if test_bit(IPS_SEQ_ADJUST_BIT, &(*ct).status) && !nf_ct_seq_adjust(skb, ct, ctinfo, protoff) {
        return NF_DROP;
    }
    NF_ACCEPT
}

pub unsafe fn nf_ct_add_helper(
    ct: *mut nf_conn,
    name: *const i8,
    family: u8,
    proto: u8,
    nat: bool,
    hp: *mut *mut nf_conntrack_helper,
) -> i32 {
    let helper: *mut nf_conntrack_helper = nf_conntrack_helper_try_module_get(name, family, proto);
    let mut ret: i32 = 0;

    if helper.is_null() {
        return -EINVAL;
    }

    let help: *mut nf_conn_help = nf_ct_helper_ext_add(ct, GFP_KERNEL);
    if help.is_null() {
        nf_conntrack_helper_put(helper);
        return -ENOMEM;
    }

    // Preserves the build-time CONFIG_NF_NAT conditional from the C source.
    if nat {
        ret = nf_nat_helper_try_module_get(name, family, proto);
        if ret != 0 {
            nf_conntrack_helper_put(helper);
            return ret;
        }
    }
    rcu_assign_pointer!((*help).helper, helper);
    *hp = helper;
    ret
}

/* Trim the skb to the length specified by the IP/IPv6 header,
 * removing any trailing lower-layer padding. This prepares the skb
 * for higher-layer processing that assumes skb->len excludes padding
 * (such as nf_ip_checksum). The caller needs to pull the skb to the
 * network header, and ensure ip_hdr/ipv6_hdr points to valid data.
 */
pub unsafe fn nf_ct_skb_network_trim(skb: *mut sk_buff, family: i32) -> i32 {
    let mut len: u32;

    match family {
        NFPROTO_IPV4 => len = skb_ip_totlen(skb),
        NFPROTO_IPV6 => {
            len = skb_ipv6_payload_len(skb);
            if (*ipv6_hdr(skb)).nexthdr == NEXTHDR_HOP {
                let err: i32 = nf_ip6_check_hbh_len(skb, &mut len);
                if err != 0 {
                    return err;
                }
            }
            len += core::mem::size_of::<ipv6hdr>() as u32;
        }
        _ => len = (*skb).len,
    }

    pskb_trim_rcsum(skb, len)
}

/* Returns 0 on success, -EINPROGRESS if 'skb' is stolen, or other nonzero
 * value if 'skb' is freed.
 */
pub unsafe fn nf_ct_handle_fragments(
    net: *mut net,
    skb: *mut sk_buff,
    zone: u16,
    family: u8,
    proto: *mut u8,
    mru: *mut u16,
) -> i32 {
    let mut err: i32;

    if family == NFPROTO_IPV4 {
        let user = IP_DEFRAG_CONNTRACK_IN + zone;
        memset(IPCB(skb), 0, core::mem::size_of::<inet_skb_parm>());
        local_bh_disable();
        err = ip_defrag(net, skb, user);
        local_bh_enable();
        if err != 0 {
            return err;
        }
        *mru = (*IPCB(skb)).frag_max_size;
    // Preserves the build-time CONFIG_NF_DEFRAG_IPV6 conditional from C.
    } else if family == NFPROTO_IPV6 {
        let user = IP6_DEFRAG_CONNTRACK_IN + zone;
        memset(IP6CB(skb), 0, core::mem::size_of::<inet6_skb_parm>());
        err = nf_ct_frag6_gather(net, skb, user);
        if err != 0 {
            if err != -EINPROGRESS {
                kfree_skb(skb);
            }
            return err;
        }
        *proto = (*ipv6_hdr(skb)).nexthdr;
        *mru = (*IP6CB(skb)).frag_max_size;
    } else {
        kfree_skb(skb);
        return -EPFNOSUPPORT;
    }

    skb_clear_hash(skb);
    (*skb).ignore_df = 1;
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
