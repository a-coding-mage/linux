// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * ip_vs_nfct.c: Netfilter connection tracking support for IPVS
 *
 * Rust translation of the implementation source. Kernel and IPVS symbols
 * referenced here are supplied by external dependencies.
 */

// C includes omitted; their symbols remain external dependencies.

const FMT_TUPLE: &str = "%s:%u->%s:%u/%u";
const FMT_CONN: &str = "%s:%u->%s:%u->%s:%u/%u:%u";

pub unsafe fn ip_vs_update_conntrack(
    skb: *mut sk_buff,
    cp: *mut ip_vs_conn,
    outin: ::core::ffi::c_int,
) {
    let mut ctinfo: ip_conntrack_info = ::core::mem::zeroed();
    let ct: *mut nf_conn = nf_ct_get(skb, &mut ctinfo);
    let mut new_tuple: nf_conntrack_tuple;

    if ct.is_null() || nf_ct_is_confirmed(ct) || nf_ct_is_dying(ct) {
        return;
    }

    /* Never alter conntrack for non-NAT conns */
    if IP_VS_FWD_METHOD(cp) != IP_VS_CONN_F_MASQ {
        return;
    }

    /* Never alter conntrack for OPS conns (no reply is expected) */
    if ((*cp).flags & IP_VS_CONN_F_ONE_PACKET) != 0 {
        return;
    }

    /* Alter reply only in original direction */
    if CTINFO2DIR(ctinfo) != IP_CT_DIR_ORIGINAL {
        return;
    }

    /* Applications may adjust TCP seqs */
    if !(*cp).app.is_null()
        && nf_ct_protonum(ct) == IPPROTO_TCP
        && !nfct_seqadj(ct)
        && !nfct_seqadj_ext_add(ct)
    {
        return;
    }

    /*
     * The connection is not yet in the hashtable, so we update it.
     * CIP->VIP will remain the same, so leave the original tuple untouched.
     */
    new_tuple = (*ct).tuplehash[IP_CT_DIR_REPLY as usize].tuple;
    /* This will also take care of UDP and other protocols. */
    if outin != 0 {
        new_tuple.src.u3 = (*cp).daddr;
        if new_tuple.dst.protonum != IPPROTO_ICMP
            && new_tuple.dst.protonum != IPPROTO_ICMPV6
        {
            new_tuple.src.u.tcp.port = (*cp).dport;
        }
    } else {
        new_tuple.dst.u3 = (*cp).vaddr;
        if new_tuple.dst.protonum != IPPROTO_ICMP
            && new_tuple.dst.protonum != IPPROTO_ICMPV6
        {
            new_tuple.dst.u.tcp.port = (*cp).vport;
        }
    }
    IP_VS_DBG_BUF(7, "%s: Updating conntrack", __func__, ct, (*ct).status, ctinfo,
                  (*ct).tuplehash[IP_CT_DIR_REPLY as usize].tuple);
    IP_VS_DBG_BUF(7, "%s: Updating conntrack", __func__, ct, (*ct).status, ctinfo, new_tuple);
    nf_conntrack_alter_reply(ct, &mut new_tuple);
    IP_VS_DBG_BUF(7, "%s: Updated conntrack", __func__, ct, cp);
}

pub unsafe fn ip_vs_confirm_conntrack(skb: *mut sk_buff) -> ::core::ffi::c_int {
    nf_conntrack_confirm(skb)
}

/* Called from init_conntrack() as expectfn handler. */
unsafe fn ip_vs_nfct_expect_callback(ct: *mut nf_conn, exp: *mut nf_conntrack_expect) {
    let orig: *mut nf_conntrack_tuple;
    let mut new_reply: nf_conntrack_tuple;
    let mut cp: *mut ip_vs_conn;
    let mut p: ip_vs_conn_param = ::core::mem::zeroed();
    let net: *mut net = nf_ct_net(ct);

    /* RS->CLIENT */
    orig = &mut (*ct).tuplehash[IP_CT_DIR_ORIGINAL as usize].tuple;
    ip_vs_conn_fill_param(
        net_ipvs(net), (*exp).tuple.src.l3num, (*orig).dst.protonum,
        &(*orig).src.u3, (*orig).src.u.tcp.port,
        &(*orig).dst.u3, (*orig).dst.u.tcp.port, &mut p,
    );
    cp = ip_vs_conn_out_get(&p);
    if !cp.is_null() {
        /* Change reply CLIENT->RS to CLIENT->VS */
        new_reply = (*ct).tuplehash[IP_CT_DIR_REPLY as usize].tuple;
        new_reply.dst.u3 = (*cp).vaddr;
        new_reply.dst.u.tcp.port = (*cp).vport;
        if IP_VS_FWD_METHOD(cp) == IP_VS_CONN_F_MASQ {
            nf_conntrack_alter_reply(ct, &mut new_reply);
        }
        ip_vs_conn_put(cp);
        return;
    }

    /* CLIENT->VS */
    cp = ip_vs_conn_in_get(&p);
    if !cp.is_null() {
        /* Change reply VS->CLIENT to RS->CLIENT */
        new_reply = (*ct).tuplehash[IP_CT_DIR_REPLY as usize].tuple;
        new_reply.src.u3 = (*cp).daddr;
        new_reply.src.u.tcp.port = (*cp).dport;
        if IP_VS_FWD_METHOD(cp) == IP_VS_CONN_F_MASQ {
            nf_conntrack_alter_reply(ct, &mut new_reply);
        }
        ip_vs_conn_put(cp);
        return;
    }

    IP_VS_DBG_BUF(7, "%s: unknown expect", __func__, ct, (*ct).status, orig);
}

pub unsafe fn ip_vs_nfct_expect_related(
    skb: *mut sk_buff,
    ct: *mut nf_conn,
    cp: *mut ip_vs_conn,
    proto: u8,
    port: __be16,
    from_rs: ::core::ffi::c_int,
) {
    if ct.is_null() {
        return;
    }
    let exp = nf_ct_expect_alloc(ct);
    if exp.is_null() {
        return;
    }
    nf_ct_expect_init(
        exp, NF_CT_EXPECT_CLASS_DEFAULT, nf_ct_l3num(ct),
        if from_rs != 0 { &(*cp).daddr } else { &(*cp).caddr },
        if from_rs != 0 { &(*cp).caddr } else { &(*cp).vaddr },
        proto, if port != 0 { &port } else { ::core::ptr::null() },
        if from_rs != 0 { &(*cp).cport } else { &(*cp).vport },
    );
    (*exp).expectfn = Some(ip_vs_nfct_expect_callback);
    IP_VS_DBG_BUF(7, "%s: ct=%p, expect tuple=" FMT_TUPLE, __func__, ct, &(*exp).tuple);
    nf_ct_expect_related(exp, 0);
    nf_ct_expect_put(exp);
}

pub unsafe fn ip_vs_conn_drop_conntrack(cp: *mut ip_vs_conn) {
    if (*cp).cport == 0 {
        return;
    }
    let mut tuple: nf_conntrack_tuple = ::core::mem::zeroed();
    tuple.dst.protonum = (*cp).protocol;
    tuple.dst.dir = IP_CT_DIR_ORIGINAL;
    tuple.src.u3 = (*cp).caddr;
    tuple.src.u.all = (*cp).cport;
    tuple.src.l3num = (*cp).af;
    tuple.dst.u3 = (*cp).vaddr;
    tuple.dst.u.all = (*cp).vport;
    let h = nf_conntrack_find_get((*(*cp).ipvs).net, &nf_ct_zone_dflt, &tuple);
    if !h.is_null() {
        let ct = nf_ct_tuplehash_to_ctrack(h);
        nf_ct_kill(ct);
        nf_ct_put(ct);
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
