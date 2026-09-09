// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * INET		An implementation of the TCP/IP protocol suite for the LINUX
 *		operating system.  INET is implemented using the BSD Socket
 *		interface as the means of communication with the user level.
 *
 *		This file implements the various access functions for the
 *		PROC file system.  This is very similar to the IPv4 version,
 *		except it reports the sockets in the INET6 address family.
 *
 * Authors:	David S. Miller (davem@caip.rutgers.edu)
 *		YOSHIFUJI Hideaki <yoshfuji@linux-ipv6.org>
 */

// Kernel headers and symbols are supplied by the surrounding translation unit.

const fn max4(a: u32, b: u32, c: u32, d: u32) -> u32 {
    core::cmp::max(core::cmp::max(a, b), core::cmp::max(c, d))
}

const SNMP_MIB_MAX: usize = max4(UDP_MIB_MAX, TCP_MIB_MAX, IPSTATS_MIB_MAX, ICMP_MIB_MAX) as usize;

unsafe fn sockstat6_seq_show(seq: *mut seq_file, _v: *mut core::ffi::c_void) -> i32 {
    let net = (*seq).private as *mut net;
    seq_printf(seq, c"TCP6: inuse %d\n", sock_prot_inuse_get(net, &tcpv6_prot));
    seq_printf(seq, c"UDP6: inuse %d\n", sock_prot_inuse_get(net, &udpv6_prot));
    seq_printf(seq, c"RAW6: inuse %d\n", sock_prot_inuse_get(net, &rawv6_prot));
    seq_printf(seq, c"FRAG6: inuse %u memory %lu\n",
        atomic_read(&(*(*net).ipv6.fqdir).rhashtable.nelems), frag_mem_limit(net));
    0
}

static SNMP_MIB_MAX_PLACEHOLDER: () = ();

static snmp6_ipstats_list: [snmp_mib; 34] = [
    snmp_mib_item(c"Ip6InReceives", IPSTATS_MIB_INPKTS), snmp_mib_item(c"Ip6InHdrErrors", IPSTATS_MIB_INHDRERRORS),
    snmp_mib_item(c"Ip6InTooBigErrors", IPSTATS_MIB_INTOOBIGERRORS), snmp_mib_item(c"Ip6InNoRoutes", IPSTATS_MIB_INNOROUTES),
    snmp_mib_item(c"Ip6InAddrErrors", IPSTATS_MIB_INADDRERRORS), snmp_mib_item(c"Ip6InUnknownProtos", IPSTATS_MIB_INUNKNOWNPROTOS),
    snmp_mib_item(c"Ip6InTruncatedPkts", IPSTATS_MIB_INTRUNCATEDPKTS), snmp_mib_item(c"Ip6InDiscards", IPSTATS_MIB_INDISCARDS),
    snmp_mib_item(c"Ip6InDelivers", IPSTATS_MIB_INDELIVERS), snmp_mib_item(c"Ip6OutForwDatagrams", IPSTATS_MIB_OUTFORWDATAGRAMS),
    snmp_mib_item(c"Ip6OutRequests", IPSTATS_MIB_OUTREQUESTS), snmp_mib_item(c"Ip6OutDiscards", IPSTATS_MIB_OUTDISCARDS),
    snmp_mib_item(c"Ip6OutNoRoutes", IPSTATS_MIB_OUTNOROUTES), snmp_mib_item(c"Ip6ReasmTimeout", IPSTATS_MIB_REASMTIMEOUT),
    snmp_mib_item(c"Ip6ReasmReqds", IPSTATS_MIB_REASMREQDS), snmp_mib_item(c"Ip6ReasmOKs", IPSTATS_MIB_REASMOKS),
    snmp_mib_item(c"Ip6ReasmFails", IPSTATS_MIB_REASMFAILS), snmp_mib_item(c"Ip6FragOKs", IPSTATS_MIB_FRAGOKS),
    snmp_mib_item(c"Ip6FragFails", IPSTATS_MIB_FRAGFAILS), snmp_mib_item(c"Ip6FragCreates", IPSTATS_MIB_FRAGCREATES),
    snmp_mib_item(c"Ip6InMcastPkts", IPSTATS_MIB_INMCASTPKTS), snmp_mib_item(c"Ip6OutMcastPkts", IPSTATS_MIB_OUTMCASTPKTS),
    snmp_mib_item(c"Ip6InOctets", IPSTATS_MIB_INOCTETS), snmp_mib_item(c"Ip6OutOctets", IPSTATS_MIB_OUTOCTETS),
    snmp_mib_item(c"Ip6InMcastOctets", IPSTATS_MIB_INMCASTOCTETS), snmp_mib_item(c"Ip6OutMcastOctets", IPSTATS_MIB_OUTMCASTOCTETS),
    snmp_mib_item(c"Ip6InBcastOctets", IPSTATS_MIB_INBCASTOCTETS), snmp_mib_item(c"Ip6OutBcastOctets", IPSTATS_MIB_OUTBCASTOCTETS),
    // IPSTATS_MIB_CSUMERRORS is not relevant in IPv6 (no checksum)
    snmp_mib_item(c"Ip6InNoECTPkts", IPSTATS_MIB_NOECTPKTS), snmp_mib_item(c"Ip6InECT1Pkts", IPSTATS_MIB_ECT1PKTS),
    snmp_mib_item(c"Ip6InECT0Pkts", IPSTATS_MIB_ECT0PKTS), snmp_mib_item(c"Ip6InCEPkts", IPSTATS_MIB_CEPKTS),
    snmp_mib_item(c"Ip6OutTransmits", IPSTATS_MIB_OUTPKTS),
];

static snmp6_icmp6_list: [snmp_mib; 6] = [
    snmp_mib_item(c"Icmp6InMsgs", ICMP6_MIB_INMSGS), snmp_mib_item(c"Icmp6InErrors", ICMP6_MIB_INERRORS),
    snmp_mib_item(c"Icmp6OutMsgs", ICMP6_MIB_OUTMSGS), snmp_mib_item(c"Icmp6OutErrors", ICMP6_MIB_OUTERRORS),
    snmp_mib_item(c"Icmp6InCsumErrors", ICMP6_MIB_CSUMERRORS),
    // ICMP6_MIB_RATELIMITHOST needs to be last, see snmp6_dev_seq_show().
    snmp_mib_item(c"Icmp6OutRateLimitHost", ICMP6_MIB_RATELIMITHOST),
];

static snmp6_udp6_list: [snmp_mib; 9] = [
    snmp_mib_item(c"Udp6InDatagrams", UDP_MIB_INDATAGRAMS), snmp_mib_item(c"Udp6NoPorts", UDP_MIB_NOPORTS),
    snmp_mib_item(c"Udp6InErrors", UDP_MIB_INERRORS), snmp_mib_item(c"Udp6OutDatagrams", UDP_MIB_OUTDATAGRAMS),
    snmp_mib_item(c"Udp6RcvbufErrors", UDP_MIB_RCVBUFERRORS), snmp_mib_item(c"Udp6SndbufErrors", UDP_MIB_SNDBUFERRORS),
    snmp_mib_item(c"Udp6InCsumErrors", UDP_MIB_CSUMERRORS), snmp_mib_item(c"Udp6IgnoredMulti", UDP_MIB_IGNOREDMULTI),
    snmp_mib_item(c"Udp6MemErrors", UDP_MIB_MEMERRORS),
];

// The remaining functions retain the C ABI and kernel pointer semantics.
// Their external helpers, structures, constants, and formatting primitives are supplied elsewhere.
unsafe fn snmp6_seq_show_icmpv6msg(seq: *mut seq_file, smib: *mut atomic_long_t) {
    let mut name = [0i8; 32];
    for i in 0..ICMP6MSG_MIB_MAX {
        let mut p: *const i8 = core::ptr::null();
        match i & 0xff {
            ICMPV6_DEST_UNREACH => p = c"DestUnreachs".as_ptr(), ICMPV6_PKT_TOOBIG => p = c"PktTooBigs".as_ptr(),
            ICMPV6_TIME_EXCEED => p = c"TimeExcds".as_ptr(), ICMPV6_PARAMPROB => p = c"ParmProblems".as_ptr(),
            ICMPV6_ECHO_REQUEST => p = c"Echos".as_ptr(), ICMPV6_ECHO_REPLY => p = c"EchoReplies".as_ptr(),
            ICMPV6_MGM_QUERY => p = c"GroupMembQueries".as_ptr(), ICMPV6_MGM_REPORT => p = c"GroupMembResponses".as_ptr(),
            ICMPV6_MGM_REDUCTION => p = c"GroupMembReductions".as_ptr(), ICMPV6_MLD2_REPORT => p = c"MLDv2Reports".as_ptr(),
            NDISC_ROUTER_ADVERTISEMENT => p = c"RouterAdvertisements".as_ptr(), NDISC_ROUTER_SOLICITATION => p = c"RouterSolicits".as_ptr(),
            NDISC_NEIGHBOUR_ADVERTISEMENT => p = c"NeighborAdvertisements".as_ptr(), NDISC_NEIGHBOUR_SOLICITATION => p = c"NeighborSolicits".as_ptr(),
            NDISC_REDIRECT => p = c"Redirects".as_ptr(), _ => {}
        }
        if p.is_null() { continue; }
        snprintf(name.as_mut_ptr(), name.len(), c"Icmp6%s%s", if i & 0x100 != 0 { c"Out".as_ptr() } else { c"In".as_ptr() }, p);
        seq_printf(seq, c"%-32s\t%lu\n", name.as_ptr(), atomic_long_read(smib.add(i)));
    }
    for i in 0..ICMP6MSG_MIB_MAX {
        let val = atomic_long_read(smib.add(i));
        if val == 0 { continue; }
        snprintf(name.as_mut_ptr(), name.len(), c"Icmp6%sType%u", if i & 0x100 != 0 { c"Out".as_ptr() } else { c"In".as_ptr() }, i & 0xff);
        seq_printf(seq, c"%-32s\t%lu\n", name.as_ptr(), val);
    }
}

// can be called either with percpu mib (pcpumib != NULL), or shared one (smib != NULL)
unsafe fn snmp6_seq_show_item(seq: *mut seq_file, pcpumib: *mut core::ffi::c_void, smib: *mut atomic_long_t, itemlist: *const snmp_mib, cnt: i32) {
    let mut buff = [0usize; SNMP_MIB_MAX];
    if !pcpumib.is_null() {
        core::ptr::write_bytes(buff.as_mut_ptr(), 0, cnt as usize);
        snmp_get_cpu_field_batch_cnt(buff.as_mut_ptr(), itemlist, cnt, pcpumib);
        for i in 0..cnt { seq_printf(seq, c"%-32s\t%lu\n", (*itemlist.add(i as usize)).name, buff[i as usize]); }
    } else {
        for i in 0..cnt { let item = &*itemlist.add(i as usize); seq_printf(seq, c"%-32s\t%lu\n", item.name, atomic_long_read(smib.add(item.entry as usize))); }
    }
}

unsafe fn snmp6_seq_show_item64(seq: *mut seq_file, mib: *mut core::ffi::c_void, itemlist: *const snmp_mib, cnt: i32, syncpoff: usize) {
    let mut buff64 = [0u64; SNMP_MIB_MAX];
    core::ptr::write_bytes(buff64.as_mut_ptr(), 0, cnt as usize);
    snmp_get_cpu_field64_batch_cnt(buff64.as_mut_ptr(), itemlist, cnt, mib, syncpoff);
    for i in 0..cnt { seq_printf(seq, c"%-32s\t%llu\n", (*itemlist.add(i as usize)).name, buff64[i as usize]); }
}

// The remaining registration and sequence-show entry points are a direct ABI-preserving translation.
unsafe fn snmp6_seq_show(seq: *mut seq_file, _v: *mut core::ffi::c_void) -> i32 {
    let net = (*seq).private as *mut net;
    snmp6_seq_show_item64(seq, (*net).mib.ipv6_statistics, snmp6_ipstats_list.as_ptr(), snmp6_ipstats_list.len() as i32, core::mem::offset_of!(ipstats_mib, syncp));
    snmp6_seq_show_item(seq, (*net).mib.icmpv6_statistics, core::ptr::null_mut(), snmp6_icmp6_list.as_ptr(), snmp6_icmp6_list.len() as i32);
    snmp6_seq_show_icmpv6msg(seq, (*net).mib.icmpv6msg_statistics.mibs);
    snmp6_seq_show_item(seq, (*net).mib.udp_stats_in6, core::ptr::null_mut(), snmp6_udp6_list.as_ptr(), snmp6_udp6_list.len() as i32);
    0
}

// External declarations and the remaining net registration functions are intentionally kept as kernel-facing symbols.
extern "C" {
    static tcpv6_prot: proto;
    static udpv6_prot: proto;
    static rawv6_prot: proto;
}

unsafe fn snmp6_dev_seq_show(seq: *mut seq_file, _v: *mut core::ffi::c_void) -> i32 {
    let idev = (*seq).private as *mut inet6_dev;
    seq_printf(seq, c"%-32s\t%u\n", c"ifIndex", (*(*idev).dev).ifindex);
    snmp6_seq_show_item64(seq, (*idev).stats.ipv6, snmp6_ipstats_list.as_ptr(), snmp6_ipstats_list.len() as i32, core::mem::offset_of!(ipstats_mib, syncp));
    // Per idev icmp stats do not have ICMP6_MIB_RATELIMITHOST
    snmp6_seq_show_item(seq, core::ptr::null_mut(), (*idev).stats.icmpv6dev.mibs, snmp6_icmp6_list.as_ptr(), (snmp6_icmp6_list.len() - 1) as i32);
    snmp6_seq_show_icmpv6msg(seq, (*idev).stats.icmpv6msgdev.mibs);
    0
}

#[no_mangle]
pub unsafe extern "C" fn snmp6_register_dev(idev: *mut inet6_dev) -> i32 {
    if idev.is_null() || (*idev).dev.is_null() { return -EINVAL; }
    let net = dev_net((*idev).dev);
    if (*net).mib.proc_net_devsnmp6.is_null() { return -ENOENT; }
    let p = proc_create_single_data((*(*idev).dev).name.as_ptr(), 0o444, (*net).mib.proc_net_devsnmp6, snmp6_dev_seq_show, idev as *mut core::ffi::c_void);
    if p.is_null() { return -ENOMEM; }
    (*idev).stats.proc_dir_entry = p;
    0
}

#[no_mangle]
pub unsafe extern "C" fn snmp6_unregister_dev(idev: *mut inet6_dev) -> i32 {
    let net = dev_net((*idev).dev);
    if (*net).mib.proc_net_devsnmp6.is_null() { return -ENOENT; }
    if (*idev).stats.proc_dir_entry.is_null() { return -EINVAL; }
    proc_remove((*idev).stats.proc_dir_entry);
    (*idev).stats.proc_dir_entry = core::ptr::null_mut();
    0
}

unsafe fn ipv6_proc_init_net(net: *mut net) -> i32 {
    if proc_create_net_single(c"sockstat6", 0o444, (*net).proc_net, sockstat6_seq_show, core::ptr::null_mut()).is_null() { return -ENOMEM; }
    if proc_create_net_single(c"snmp6", 0o444, (*net).proc_net, snmp6_seq_show, core::ptr::null_mut()).is_null() {
        remove_proc_entry(c"sockstat6", (*net).proc_net); return -ENOMEM;
    }
    (*net).mib.proc_net_devsnmp6 = proc_mkdir(c"dev_snmp6", (*net).proc_net);
    if (*net).mib.proc_net_devsnmp6.is_null() {
        remove_proc_entry(c"snmp6", (*net).proc_net); remove_proc_entry(c"sockstat6", (*net).proc_net); return -ENOMEM;
    }
    0
}

unsafe fn ipv6_proc_exit_net(net: *mut net) {
    remove_proc_entry(c"sockstat6", (*net).proc_net);
    remove_proc_entry(c"dev_snmp6", (*net).proc_net);
    remove_proc_entry(c"snmp6", (*net).proc_net);
}

static mut ipv6_proc_ops: pernet_operations = pernet_operations { init: Some(ipv6_proc_init_net), exit: Some(ipv6_proc_exit_net) };

#[no_mangle]
pub unsafe extern "C" fn ipv6_misc_proc_init() -> i32 { register_pernet_subsys(&raw mut ipv6_proc_ops) }

#[no_mangle]
pub unsafe extern "C" fn ipv6_misc_proc_exit() { unregister_pernet_subsys(&raw mut ipv6_proc_ops); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
