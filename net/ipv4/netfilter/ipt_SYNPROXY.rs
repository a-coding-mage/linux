// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (c) 2013 Patrick McHardy <kaber@trash.net>
 */

// Linux kernel dependencies supplied by other translation units.

unsafe fn synproxy_tg4(
    skb: *mut sk_buff,
    par: *const xt_action_param,
) -> c_uint {
    let info: *const xt_synproxy_info = (*par).targinfo as *const xt_synproxy_info;
    let net: *mut net = xt_net(par);
    let snet: *mut synproxy_net = synproxy_pernet(net);
    let mut opts: synproxy_options = core::mem::zeroed();
    let mut th: *mut tcphdr;
    let mut _th: tcphdr = core::mem::zeroed();

    if nf_ip_checksum(skb, xt_hooknum(par), (*par).thoff, IPPROTO_TCP) != 0 {
        return NF_DROP;
    }

    th = skb_header_pointer(
        skb,
        (*par).thoff,
        core::mem::size_of::<tcphdr>(),
        &mut _th as *mut tcphdr as *mut c_void,
    );
    if th.is_null() {
        return NF_DROP;
    }

    if !synproxy_parse_options(skb, (*par).thoff, th, &mut opts) {
        return NF_DROP;
    }

    if (*th).syn && !((*th).ack || (*th).fin || (*th).rst) {
        /* Initial SYN from client */
        this_cpu_inc(&mut (*(*snet).stats).syn_received);

        if (*th).ece && (*th).cwr {
            opts.options |= XT_SYNPROXY_OPT_ECN;
        }

        opts.options &= (*info).options;
        opts.mss_encode = opts.mss_option;
        opts.mss_option = (*info).mss;
        if opts.options & XT_SYNPROXY_OPT_TIMESTAMP != 0 {
            synproxy_init_timestamp_cookie(info, &mut opts);
        } else {
            opts.options &= !(XT_SYNPROXY_OPT_WSCALE
                | XT_SYNPROXY_OPT_SACK_PERM
                | XT_SYNPROXY_OPT_ECN);
        }

        synproxy_send_client_synack(net, skb, th, &mut opts);
        consume_skb(skb);
        return NF_STOLEN;
    } else if (*th).ack && !((*th).fin || (*th).rst || (*th).syn) {
        /* ACK from client */
        if synproxy_recv_client_ack(net, skb, th, &mut opts, ntohl((*th).seq)) {
            consume_skb(skb);
            return NF_STOLEN;
        } else {
            return NF_DROP;
        }
    }

    XT_CONTINUE
}

unsafe fn synproxy_tg4_check(par: *const xt_tgchk_param) -> c_int {
    let snet: *mut synproxy_net = synproxy_pernet((*par).net);
    let e: *const ipt_entry = (*par).entryinfo;
    let mut err: c_int;

    if (*e).ip.proto != IPPROTO_TCP
        || ((*e).ip.invflags & XT_INV_PROTO) != 0
    {
        return -EINVAL;
    }

    err = nf_ct_netns_get((*par).net, (*par).family);
    if err != 0 {
        return err;
    }

    err = nf_synproxy_ipv4_init(snet, (*par).net);
    if err != 0 {
        nf_ct_netns_put((*par).net, (*par).family);
        return err;
    }

    err
}

unsafe fn synproxy_tg4_destroy(par: *const xt_tgdtor_param) {
    let snet: *mut synproxy_net = synproxy_pernet((*par).net);

    nf_synproxy_ipv4_fini(snet, (*par).net);
    nf_ct_netns_put((*par).net, (*par).family);
}

static mut synproxy_tg4_reg: xt_target = xt_target {
    name: "SYNPROXY",
    family: NFPROTO_IPV4,
    hooks: (1 << NF_INET_LOCAL_IN) | (1 << NF_INET_FORWARD),
    target: Some(synproxy_tg4),
    targetsize: core::mem::size_of::<xt_synproxy_info>(),
    checkentry: Some(synproxy_tg4_check),
    destroy: Some(synproxy_tg4_destroy),
    me: THIS_MODULE,
};

unsafe fn synproxy_tg4_init() -> c_int {
    xt_register_target(&mut synproxy_tg4_reg)
}

unsafe fn synproxy_tg4_exit() {
    xt_unregister_target(&mut synproxy_tg4_reg);
}

module_init!(synproxy_tg4_init);
module_exit!(synproxy_tg4_exit);

MODULE_LICENSE!("GPL");
MODULE_AUTHOR!("Patrick McHardy <kaber@trash.net>");
MODULE_DESCRIPTION!("Intercept TCP connections and establish them using syncookies");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
