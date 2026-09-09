// SPDX-License-Identifier: GPL-2.0-only
/*
 *  ebt_ip
 *
 *	Authors:
 *	Bart De Schuymer <bdschuym@pandora.be>
 *
 *  April, 2002
 *
 *  Changes:
 *    added ip-sport and ip-dport
 *    Innominate Security Technologies AG <mhopf@innominate.com>
 *    September, 2002
 */
// Kernel and netfilter declarations are supplied by the surrounding build.

#[repr(C)]
pub union pkthdr {
    pub tcpudphdr: TcpUdpHdr,
    pub icmphdr: IcmpHdr,
    pub igmphdr: IgmpHdr,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct TcpUdpHdr {
    pub src: __be16,
    pub dst: __be16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct IcmpHdr {
    pub type_: u8,
    pub code: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct IgmpHdr {
    pub type_: u8,
}

unsafe fn ebt_ip_mt(skb: *const sk_buff, par: *mut xt_action_param) -> bool {
    let info = unsafe { (*par).matchinfo as *const ebt_ip_info };
    let mut _iph: iphdr = core::mem::zeroed();
    let ih = unsafe { skb_header_pointer(skb, 0, core::mem::size_of::<iphdr>(), &mut _iph as *mut _ as *mut core::ffi::c_void) as *const iphdr };
    if ih.is_null() { return false; }
    let info = unsafe { &*info };
    let ih = unsafe { &*ih };
    if info.bitmask & EBT_IP_TOS != 0 && NF_INVF(info, EBT_IP_TOS, info.tos != ih.tos) { return false; }
    if info.bitmask & EBT_IP_SOURCE != 0 && NF_INVF(info, EBT_IP_SOURCE, (ih.saddr & info.smask) != ih.saddr) { return false; }
    if info.bitmask & EBT_IP_DEST != 0 && NF_INVF(info, EBT_IP_DEST, (ih.daddr & info.dmask) != ih.daddr) { return false; }
    if info.bitmask & EBT_IP_PROTO != 0 {
        if NF_INVF(info, EBT_IP_PROTO, info.protocol != ih.protocol) { return false; }
        if info.bitmask & (EBT_IP_DPORT | EBT_IP_SPORT | EBT_IP_ICMP | EBT_IP_IGMP) == 0 { return true; }
        if ntohs(ih.frag_off) & IP_OFFSET != 0 { return false; }
        // min icmp/igmp headersize is 4, so sizeof(_pkthdr) is ok.
        let mut _pkthdr: pkthdr = unsafe { core::mem::zeroed() };
        let pptr = skb_header_pointer(skb, (ih.ihl as usize) * 4, core::mem::size_of::<pkthdr>(), &mut _pkthdr as *mut _ as *mut core::ffi::c_void) as *const pkthdr;
        if pptr.is_null() { return false; }
        if info.bitmask & EBT_IP_DPORT != 0 {
            let dst = unsafe { ntohs((*pptr).tcpudphdr.dst) } as u32;
            if NF_INVF(info, EBT_IP_DPORT, dst < info.dport[0] || dst > info.dport[1]) { return false; }
        }
        if info.bitmask & EBT_IP_SPORT != 0 {
            let src = unsafe { ntohs((*pptr).tcpudphdr.src) } as u32;
            if NF_INVF(info, EBT_IP_SPORT, src < info.sport[0] || src > info.sport[1]) { return false; }
        }
        if info.bitmask & EBT_IP_ICMP != 0 && NF_INVF(info, EBT_IP_ICMP, unsafe { (*pptr).icmphdr.type_ } < info.icmp_type[0] || unsafe { (*pptr).icmphdr.type_ } > info.icmp_type[1] || unsafe { (*pptr).icmphdr.code } < info.icmp_code[0] || unsafe { (*pptr).icmphdr.code } > info.icmp_code[1]) { return false; }
        if info.bitmask & EBT_IP_IGMP != 0 && NF_INVF(info, EBT_IP_IGMP, unsafe { (*pptr).igmphdr.type_ } < info.igmp_type[0] || unsafe { (*pptr).igmphdr.type_ } > info.igmp_type[1]) { return false; }
    }
    true
}

unsafe fn ebt_ip_mt_check(par: *const xt_mtchk_param) -> c_int {
    let info = unsafe { &*((*par).matchinfo as *const ebt_ip_info) };
    let e = unsafe { &*((*par).entryinfo as *const ebt_entry) };
    if e.ethproto != htons(ETH_P_IP) || e.invflags & EBT_IPROTO != 0 { return -EINVAL; }
    if info.bitmask & !EBT_IP_MASK != 0 || info.invflags & !EBT_IP_MASK != 0 { return -EINVAL; }
    if info.bitmask & (EBT_IP_DPORT | EBT_IP_SPORT) != 0 {
        if info.invflags & EBT_IP_PROTO != 0 { return -EINVAL; }
        if info.protocol != IPPROTO_TCP && info.protocol != IPPROTO_UDP && info.protocol != IPPROTO_UDPLITE && info.protocol != IPPROTO_SCTP && info.protocol != IPPROTO_DCCP { return -EINVAL; }
    }
    if info.bitmask & EBT_IP_DPORT != 0 && info.dport[0] > info.dport[1] { return -EINVAL; }
    if info.bitmask & EBT_IP_SPORT != 0 && info.sport[0] > info.sport[1] { return -EINVAL; }
    if info.bitmask & EBT_IP_ICMP != 0 {
        if info.invflags & EBT_IP_PROTO != 0 || info.protocol != IPPROTO_ICMP || info.icmp_type[0] > info.icmp_type[1] || info.icmp_code[0] > info.icmp_code[1] { return -EINVAL; }
    }
    if info.bitmask & EBT_IP_IGMP != 0 {
        if info.invflags & EBT_IP_PROTO != 0 || info.protocol != IPPROTO_IGMP || info.igmp_type[0] > info.igmp_type[1] { return -EINVAL; }
    }
    0
}

static mut ebt_ip_mt_reg: xt_match = xt_match {
    name: b"ip\0".as_ptr() as *const _, revision: 0, family: NFPROTO_BRIDGE,
    match_: Some(ebt_ip_mt), checkentry: Some(ebt_ip_mt_check),
    matchsize: core::mem::size_of::<ebt_ip_info>(), me: THIS_MODULE,
};

unsafe fn ebt_ip_init() -> c_int { xt_register_match(&mut ebt_ip_mt_reg) }
unsafe fn ebt_ip_fini() { xt_unregister_match(&mut ebt_ip_mt_reg); }

// module_init(ebt_ip_init); module_exit(ebt_ip_fini);
// MODULE_DESCRIPTION("Ebtables: IPv4 protocol packet match");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
