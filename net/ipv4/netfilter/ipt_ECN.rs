// SPDX-License-Identifier: GPL-2.0-only
/* iptables module for the IPv4 and TCP ECN bits, Version 1.5
 *
 * (C) 2002 by Harald Welte <laforge@netfilter.org>
 */
// pr_fmt(fmt) KBUILD_MODNAME ": " fmt
// C dependencies: linux/in.h, linux/module.h, linux/skbuff.h, linux/ip.h,
// net/ip.h, linux/tcp.h, net/checksum.h, linux/netfilter/x_tables.h,
// linux/netfilter_ipv4/ip_tables.h, linux/netfilter_ipv4/ipt_ECN.h

// MODULE_LICENSE("GPL");
// MODULE_AUTHOR("Harald Welte <laforge@netfilter.org>");
// MODULE_DESCRIPTION("Xtables: Explicit Congestion Notification (ECN) flag modification");

/* set ECT codepoint from IP header.
 *      return false if there was an error. */
#[inline]
unsafe fn set_ect_ip(skb: *mut sk_buff, einfo: *const ipt_ECN_info) -> bool {
    let mut iph: *mut iphdr = ip_hdr(skb);

    if ((*iph).tos & IPT_ECN_IP_MASK) != ((*einfo).ip_ect & IPT_ECN_IP_MASK) {
        let oldtos: __u8;
        if skb_ensure_writable(skb, core::mem::size_of::<iphdr>()) != 0 {
            return false;
        }
        iph = ip_hdr(skb);
        oldtos = (*iph).tos;
        (*iph).tos &= !IPT_ECN_IP_MASK;
        (*iph).tos |= (*einfo).ip_ect & IPT_ECN_IP_MASK;
        csum_replace2(&mut (*iph).check, htons(oldtos), htons((*iph).tos));
    }
    true
}

/* Return false if there was an error. */
#[inline]
unsafe fn set_ect_tcp(skb: *mut sk_buff, einfo: *const ipt_ECN_info) -> bool {
    let mut _tcph: tcphdr = core::mem::zeroed();
    let mut tcph: *mut tcphdr;
    let oldval: __be16;

    /* Not enough header? */
    tcph = skb_header_pointer(
        skb,
        ip_hdrlen(skb),
        core::mem::size_of::<tcphdr>(),
        &mut _tcph as *mut tcphdr as *mut core::ffi::c_void,
    );
    if tcph.is_null() {
        return false;
    }

    if ((!((*einfo).operation & IPT_ECN_OP_SET_ECE != 0)
        || (*tcph).ece == (*einfo).proto.tcp.ece)
        && (!((*einfo).operation & IPT_ECN_OP_SET_CWR != 0)
            || (*tcph).cwr == (*einfo).proto.tcp.cwr))
    {
        return true;
    }

    if skb_ensure_writable(skb, ip_hdrlen(skb) + core::mem::size_of::<tcphdr>()) != 0 {
        return false;
    }
    tcph = (ip_hdr(skb) as *mut u8).add(ip_hdrlen(skb)) as *mut tcphdr;

    oldval = *((tcph as *mut __be16).add(6));
    if (*einfo).operation & IPT_ECN_OP_SET_ECE != 0 {
        (*tcph).ece = (*einfo).proto.tcp.ece;
    }
    if (*einfo).operation & IPT_ECN_OP_SET_CWR != 0 {
        (*tcph).cwr = (*einfo).proto.tcp.cwr;
    }

    inet_proto_csum_replace2(
        &mut (*tcph).check,
        skb,
        oldval,
        *((tcph as *mut __be16).add(6)),
        false,
    );
    true
}

unsafe fn ecn_tg(skb: *mut sk_buff, par: *const xt_action_param) -> c_uint {
    let einfo: *const ipt_ECN_info = (*par).targinfo as *const ipt_ECN_info;

    if (*einfo).operation & IPT_ECN_OP_SET_IP != 0 {
        if !set_ect_ip(skb, einfo) {
            return NF_DROP;
        }
    }

    if (*einfo).operation & (IPT_ECN_OP_SET_ECE | IPT_ECN_OP_SET_CWR) != 0
        && (*ip_hdr(skb)).protocol == IPPROTO_TCP
        && !set_ect_tcp(skb, einfo)
    {
        return NF_DROP;
    }

    XT_CONTINUE
}

unsafe fn ecn_tg_check(par: *const xt_tgchk_param) -> c_int {
    let einfo: *const ipt_ECN_info = (*par).targinfo as *const ipt_ECN_info;
    let e: *const ipt_entry = (*par).entryinfo;

    if (*einfo).operation & IPT_ECN_OP_MASK != 0 {
        return -EINVAL;
    }

    if (*einfo).ip_ect & !IPT_ECN_IP_MASK != 0 {
        return -EINVAL;
    }

    if (*einfo).operation & (IPT_ECN_OP_SET_ECE | IPT_ECN_OP_SET_CWR) != 0
        && ((*e).ip.proto != IPPROTO_TCP || (*e).ip.invflags & XT_INV_PROTO != 0)
    {
        // pr_info_ratelimited("cannot use operation on non-tcp rule\n");
        return -EINVAL;
    }
    0
}

static mut ecn_tg_reg: xt_target = xt_target {
    name: "ECN", // NFPROTO_IPV4
    family: NFPROTO_IPV4,
    target: Some(ecn_tg),
    targetsize: core::mem::size_of::<ipt_ECN_info>(),
    table: "mangle",
    checkentry: Some(ecn_tg_check),
    me: THIS_MODULE,
};

unsafe fn ecn_tg_init() -> c_int {
    xt_register_target(&mut ecn_tg_reg)
}

unsafe fn ecn_tg_exit() {
    xt_unregister_target(&mut ecn_tg_reg);
}

// module_init(ecn_tg_init);
// module_exit(ecn_tg_exit);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
