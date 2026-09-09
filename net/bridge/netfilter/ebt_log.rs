// SPDX-License-Identifier: GPL-2.0-only
/*
 *  ebt_log
 *
 *	Authors:
 *	Bart De Schuymer <bdschuym@pandora.be>
 *	Harald Welte <laforge@netfilter.org>
 *
 *  April, 2002
 */

// Kernel headers and build-time configuration are supplied by the surrounding
// translation unit.

static mut EBT_LOG_LOCK: Spinlock = DEFINE_SPINLOCK!();

unsafe extern "C" {
    static mut sysctl_nf_log_all_netns: bool;
}

unsafe fn ebt_log_tg_check(par: *const xt_tgchk_param) -> c_int {
    let info = (*par).targinfo as *mut ebt_log_info;

    if (*info).bitmask & !EBT_LOG_MASK != 0 {
        return -EINVAL;
    }
    if (*info).loglevel >= 8 {
        return -EINVAL;
    }
    (*info).prefix[EBT_LOG_PREFIX_SIZE - 1] = 0;
    0
}

#[repr(C)]
struct tcpudphdr {
    src: __be16,
    dst: __be16,
}

#[repr(C)]
struct arppayload {
    mac_src: [u8; ETH_ALEN],
    ip_src: [u8; 4],
    mac_dst: [u8; ETH_ALEN],
    ip_dst: [u8; 4],
}

unsafe fn print_ports(skb: *const sk_buff, protocol: u8, offset: c_int) {
    if protocol == IPPROTO_TCP
        || protocol == IPPROTO_UDP
        || protocol == IPPROTO_UDPLITE
        || protocol == IPPROTO_SCTP
        || protocol == IPPROTO_DCCP
    {
        let mut ports = MaybeUninit::<tcpudphdr>::uninit();
        let pptr = skb_header_pointer(
            skb,
            offset,
            core::mem::size_of::<tcpudphdr>(),
            ports.as_mut_ptr() as *mut c_void,
        );
        if pptr.is_null() {
            pr_cont!(" INCOMPLETE TCP/UDP header");
            return;
        }
        let pptr = pptr as *const tcpudphdr;
        pr_cont!(" SPT={} DPT={}", ntohs((*pptr).src), ntohs((*pptr).dst));
    }
}

unsafe fn ebt_log_packet(
    net: *mut net,
    pf: u8,
    hooknum: c_uint,
    skb: *const sk_buff,
    input: *const net_device,
    output: *const net_device,
    loginfo: *const nf_loginfo,
    prefix: *const c_char,
) {
    let mut bitmask: c_uint;

    /* FIXME: Disabled from containers until syslog ns is supported */
    if !net_eq(net, &init_net) && !sysctl_nf_log_all_netns {
        return;
    }

    spin_lock_bh(&raw mut EBT_LOG_LOCK);
    printk!(
        KERN_SOH "{}{} IN={} OUT={} MAC source = %pM MAC dest = %pM proto = 0x{:04x}",
        b'0' + (*loginfo).u.log.level,
        prefix,
        if !input.is_null() { (*input).name } else { c"".as_ptr() },
        if !output.is_null() { (*output).name } else { c"".as_ptr() },
        (*eth_hdr(skb)).h_source,
        (*eth_hdr(skb)).h_dest,
        ntohs((*eth_hdr(skb)).h_proto),
    );

    if (*loginfo).type_ == NF_LOG_TYPE_LOG {
        bitmask = (*loginfo).u.log.logflags;
    } else {
        bitmask = NF_LOG_DEFAULT_MASK;
    }

    if bitmask & EBT_LOG_IP != 0 && (*eth_hdr(skb)).h_proto == htons(ETH_P_IP) {
        let mut iph = MaybeUninit::<iphdr>::uninit();
        let ih = skb_header_pointer(skb, 0, core::mem::size_of::<iphdr>(), iph.as_mut_ptr() as *mut c_void);
        if ih.is_null() {
            pr_cont!(" INCOMPLETE IP header");
            goto_out!();
        }
        let ih = ih as *const iphdr;
        pr_cont!(" IP SRC=%pI4 IP DST=%pI4, IP tos=0x{:02X}, IP proto={}", &(*ih).saddr, &(*ih).daddr, (*ih).tos, (*ih).protocol);
        print_ports(skb, (*ih).protocol, ((*ih).ihl as c_int) * 4);
        goto_out!();
    }

    // #if IS_ENABLED(CONFIG_BRIDGE_EBT_IP6)
    if bitmask & EBT_LOG_IP6 != 0 && (*eth_hdr(skb)).h_proto == htons(ETH_P_IPV6) {
        let mut iph = MaybeUninit::<ipv6hdr>::uninit();
        let ih = skb_header_pointer(skb, 0, core::mem::size_of::<ipv6hdr>(), iph.as_mut_ptr() as *mut c_void);
        if ih.is_null() {
            pr_cont!(" INCOMPLETE IPv6 header");
            goto_out!();
        }
        let ih = ih as *const ipv6hdr;
        pr_cont!(" IPv6 SRC=%pI6 IPv6 DST=%pI6, IPv6 priority=0x{:01X}, Next Header={}", &(*ih).saddr, &(*ih).daddr, (*ih).priority, (*ih).nexthdr);
        let mut nexthdr = (*ih).nexthdr;
        let mut frag_off: __be16 = 0;
        let offset_ph = ipv6_skip_exthdr(skb, core::mem::size_of::<ipv6hdr>(), &mut nexthdr, &mut frag_off);
        if offset_ph == -1 { goto_out!(); }
        print_ports(skb, nexthdr, offset_ph);
        goto_out!();
    }
    // #endif

    if bitmask & EBT_LOG_ARP != 0 && ((*eth_hdr(skb)).h_proto == htons(ETH_P_ARP) || (*eth_hdr(skb)).h_proto == htons(ETH_P_RARP)) {
        let mut arph = MaybeUninit::<arphdr>::uninit();
        let ah = skb_header_pointer(skb, 0, core::mem::size_of::<arphdr>(), arph.as_mut_ptr() as *mut c_void);
        if ah.is_null() { pr_cont!(" INCOMPLETE ARP header"); goto_out!(); }
        let ah = ah as *const arphdr;
        pr_cont!(" ARP HTYPE={}, PTYPE=0x{:04x}, OPCODE={}", ntohs((*ah).ar_hrd), ntohs((*ah).ar_pro), ntohs((*ah).ar_op));
        if (*ah).ar_hrd == htons(1) && (*ah).ar_hln == ETH_ALEN && (*ah).ar_pln == core::mem::size_of::<__be32>() {
            let mut arpp = MaybeUninit::<arppayload>::uninit();
            let ap = skb_header_pointer(skb, core::mem::size_of::<arphdr>() as c_int, core::mem::size_of::<arppayload>(), arpp.as_mut_ptr() as *mut c_void);
            if ap.is_null() { pr_cont!(" INCOMPLETE ARP payload"); goto_out!(); }
            let ap = ap as *const arppayload;
            pr_cont!(" ARP MAC SRC=%pM ARP IP SRC=%pI4 ARP MAC DST=%pM ARP IP DST=%pI4", (*ap).mac_src, (*ap).ip_src, (*ap).mac_dst, (*ap).ip_dst);
        }
    }

    goto_out!();
}

unsafe fn ebt_log_tg(skb: *mut sk_buff, par: *const xt_action_param) -> c_uint {
    let info = (*par).targinfo as *const ebt_log_info;
    let mut li: nf_loginfo = core::mem::zeroed();
    let net = xt_net(par);

    li.type_ = NF_LOG_TYPE_LOG;
    li.u.log.level = (*info).loglevel;
    li.u.log.logflags = (*info).bitmask;

    /* Remember that we have to use ebt_log_packet() not to break backward
     * compatibility. We cannot use the default bridge packet logger via
     * nf_log_packet() with NFT_LOG_TYPE_LOG here. --Pablo
     */
    if (*info).bitmask & EBT_LOG_NFLOG != 0 {
        nf_log_packet(net, NFPROTO_BRIDGE, xt_hooknum(par), skb, xt_in(par), xt_out(par), &li, c"%s".as_ptr(), (*info).prefix.as_ptr());
    } else {
        ebt_log_packet(net, NFPROTO_BRIDGE, xt_hooknum(par), skb, xt_in(par), xt_out(par), &li, (*info).prefix.as_ptr());
    }
    EBT_CONTINUE
}

static mut ebt_log_tg_reg: xt_target = xt_target {
    name: c"log".as_ptr(),
    revision: 0,
    family: NFPROTO_BRIDGE,
    target: Some(ebt_log_tg),
    checkentry: Some(ebt_log_tg_check),
    targetsize: core::mem::size_of::<ebt_log_info>(),
    me: THIS_MODULE,
};

unsafe fn ebt_log_init() -> c_int {
    xt_register_target(&raw mut ebt_log_tg_reg)
}

unsafe fn ebt_log_fini() {
    xt_unregister_target(&raw mut ebt_log_tg_reg);
}

module_init!(ebt_log_init);
module_exit!(ebt_log_fini);
module_description!("Ebtables: Packet logging to syslog");
module_license!("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
