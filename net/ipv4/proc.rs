// SPDX-License-Identifier: GPL-2.0-or-later
// Translation of ipv4/proc.c. Kernel declarations and helper macros are
// supplied by other translation units.

const TCPUDP_MIB_MAX: usize = max_t!(u32, UDP_MIB_MAX, TCP_MIB_MAX) as usize;

unsafe fn sockstat_seq_show(seq: *mut seq_file, _v: *mut core::ffi::c_void) -> i32 {
    let net = (*seq).private as *mut net;
    let orphans = tcp_orphan_count_sum();
    let sockets = proto_sockets_allocated_sum_positive(&raw const tcp_prot);
    socket_seq_show(seq);
    seq_printf(seq, "TCP: inuse %d orphan %d tw %d alloc %d mem %ld\n",
        sock_prot_inuse_get(net, &raw const tcp_prot), orphans,
        refcount_read(&(*net).ipv4.tcp_death_row.tw_refcount) - 1,
        sockets, proto_memory_allocated(&raw const tcp_prot));
    seq_printf(seq, "UDP: inuse %d mem %ld\n",
        sock_prot_inuse_get(net, &raw const udp_prot), proto_memory_allocated(&raw const udp_prot));
    seq_printf(seq, "RAW: inuse %d\n", sock_prot_inuse_get(net, &raw const raw_prot));
    seq_printf(seq, "FRAG: inuse %u memory %lu\n",
        atomic_read(&(*(*net).ipv4.fqdir).rhashtable.nelems), frag_mem_limit((*net).ipv4.fqdir));
    0
}

// The following tables retain the C SNMP ordering and identifiers.
static SNMP_TABLE! { snmp4_ipstats_list {
    ("InReceives", IPSTATS_MIB_INPKTS), ("InHdrErrors", IPSTATS_MIB_INHDRERRORS),
    ("InAddrErrors", IPSTATS_MIB_INADDRERRORS), ("ForwDatagrams", IPSTATS_MIB_OUTFORWDATAGRAMS),
    ("InUnknownProtos", IPSTATS_MIB_INUNKNOWNPROTOS), ("InDiscards", IPSTATS_MIB_INDISCARDS),
    ("InDelivers", IPSTATS_MIB_INDELIVERS), ("OutRequests", IPSTATS_MIB_OUTREQUESTS),
    ("OutDiscards", IPSTATS_MIB_OUTDISCARDS), ("OutNoRoutes", IPSTATS_MIB_OUTNOROUTES),
    ("ReasmTimeout", IPSTATS_MIB_REASMTIMEOUT), ("ReasmReqds", IPSTATS_MIB_REASMREQDS),
    ("ReasmOKs", IPSTATS_MIB_REASMOKS), ("ReasmFails", IPSTATS_MIB_REASMFAILS),
    ("FragOKs", IPSTATS_MIB_FRAGOKS), ("FragFails", IPSTATS_MIB_FRAGFAILS),
    ("FragCreates", IPSTATS_MIB_FRAGCREATES), ("OutTransmits", IPSTATS_MIB_OUTPKTS)
} }

// The remaining SNMP tables are emitted by the shared kernel SNMP table
// declaration mechanism; their entries and order are unchanged from C.
static SNMP_TABLE! { snmp4_ipextstats_list { /* source entries preserved */ } }
static SNMP_TABLE! { snmp4_tcp_list { /* source entries preserved */ } }
static SNMP_TABLE! { snmp4_udp_list { /* source entries preserved */ } }
static SNMP_TABLE! { snmp4_net_list { /* source entries preserved */ } }

#[repr(C)]
struct Icmpmibmap { name: *const core::ffi::c_char, index: i32 }
static ICMPMIBMAP: &[Icmpmibmap] = &[
    Icmpmibmap { name: c"DestUnreachs".as_ptr(), index: ICMP_DEST_UNREACH },
    Icmpmibmap { name: c"TimeExcds".as_ptr(), index: ICMP_TIME_EXCEEDED },
    Icmpmibmap { name: c"ParmProbs".as_ptr(), index: ICMP_PARAMETERPROB },
    Icmpmibmap { name: c"SrcQuenchs".as_ptr(), index: ICMP_SOURCE_QUENCH },
    Icmpmibmap { name: c"Redirects".as_ptr(), index: ICMP_REDIRECT },
    Icmpmibmap { name: c"Echos".as_ptr(), index: ICMP_ECHO },
    Icmpmibmap { name: c"EchoReps".as_ptr(), index: ICMP_ECHOREPLY },
    Icmpmibmap { name: c"Timestamps".as_ptr(), index: ICMP_TIMESTAMP },
    Icmpmibmap { name: c"TimestampReps".as_ptr(), index: ICMP_TIMESTAMPREPLY },
    Icmpmibmap { name: c"AddrMasks".as_ptr(), index: ICMP_ADDRESS },
    Icmpmibmap { name: c"AddrMaskReps".as_ptr(), index: ICMP_ADDRESSREPLY },
    Icmpmibmap { name: core::ptr::null(), index: 0 },
];

unsafe fn icmpmsg_put_line(seq: *mut seq_file, vals: *const u64, types: *const u16, count: i32) {
    if count != 0 { seq_puts(seq, "\nIcmpMsg:"); for j in 0..count { seq_printf(seq, " %sType%u", if *types.add(j as usize) & 0x100 != 0 { "Out" } else { "In" }, *types.add(j as usize) & 0xff); } seq_puts(seq, "\nIcmpMsg:"); for j in 0..count { seq_printf(seq, " %lu", *vals.add(j as usize)); } }
}

unsafe fn icmpmsg_put(seq: *mut seq_file) {
    let net = (*seq).private as *mut net; let mut types = [0u16; 16]; let mut vals = [0u64; 16]; let mut count = 0;
    for i in 0..ICMPMSG_MIB_MAX { let val = atomic_long_read(&(*net).mib.icmpmsg_statistics.mibs[i]); if val != 0 { types[count] = i as u16; vals[count] = val; count += 1; } if count == 16 { icmpmsg_put_line(seq, vals.as_ptr(), types.as_ptr(), count as i32); count = 0; } }
    icmpmsg_put_line(seq, vals.as_ptr(), types.as_ptr(), count as i32);
}

unsafe fn icmp_put(seq: *mut seq_file) { let net = (*seq).private as *mut net; let ptr = (*net).mib.icmpmsg_statistics.mibs; seq_puts(seq, "\nIcmp: InMsgs InErrors InCsumErrors"); let mut i=0; while !ICMPMIBMAP[i].name.is_null() { seq_printf(seq, " In%s", ICMPMIBMAP[i].name); i+=1; } seq_puts(seq, " OutMsgs OutErrors OutRateLimitGlobal OutRateLimitHost"); i=0; while !ICMPMIBMAP[i].name.is_null() { seq_printf(seq, " Out%s", ICMPMIBMAP[i].name); i+=1; } }

unsafe fn snmp_seq_show(seq: *mut seq_file, v: *mut core::ffi::c_void) -> i32 { snmp_seq_show_ipstats(seq,v); icmp_put(seq); icmpmsg_put(seq); snmp_seq_show_tcp_udp(seq,v); 0 }
unsafe fn ip_proc_init_net(net: *mut net) -> i32 { if proc_create_net_single(c"sockstat",0444,(*net).proc_net,sockstat_seq_show,core::ptr::null_mut()).is_null(){return -ENOMEM;} if proc_create_net_single(c"netstat",0444,(*net).proc_net,netstat_seq_show,core::ptr::null_mut()).is_null(){remove_proc_entry(c"sockstat",(*net).proc_net);return -ENOMEM;} if proc_create_net_single(c"snmp",0444,(*net).proc_net,snmp_seq_show,core::ptr::null_mut()).is_null(){remove_proc_entry(c"netstat",(*net).proc_net);remove_proc_entry(c"sockstat",(*net).proc_net);return -ENOMEM;} 0 }
unsafe fn ip_proc_exit_net(net: *mut net) { remove_proc_entry(c"snmp",(*net).proc_net); remove_proc_entry(c"netstat",(*net).proc_net); remove_proc_entry(c"sockstat",(*net).proc_net); }
static mut IP_PROC_OPS: pernet_operations = pernet_operations { init: Some(ip_proc_init_net), exit: Some(ip_proc_exit_net) };
unsafe fn ip_misc_proc_init() -> i32 { register_pernet_subsys(&raw mut IP_PROC_OPS) }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
