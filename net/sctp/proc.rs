// SPDX-License-Identifier: GPL-2.0-or-later
/* SCTP kernel implementation
 * Copyright (c) 2003 International Business Machines, Corp.
 *
 * This file is part of the SCTP kernel implementation
 *
 * Please send any bug reports or fixes you make to the
 * email address(es):
 *    lksctp developers <linux-sctp@vger.kernel.org>
 *
 * Written or modified by:
 *    Sridhar Samudrala <sri@us.ibm.com>
 */

// C includes are supplied by the surrounding kernel translation unit.

static SCTP_SNMP_LIST: [snmp_mib; 32] = [
    SNMP_MIB_ITEM!("SctpCurrEstab", SCTP_MIB_CURRESTAB),
    SNMP_MIB_ITEM!("SctpActiveEstabs", SCTP_MIB_ACTIVEESTABS),
    SNMP_MIB_ITEM!("SctpPassiveEstabs", SCTP_MIB_PASSIVEESTABS),
    SNMP_MIB_ITEM!("SctpAborteds", SCTP_MIB_ABORTEDS),
    SNMP_MIB_ITEM!("SctpShutdowns", SCTP_MIB_SHUTDOWNS),
    SNMP_MIB_ITEM!("SctpOutOfBlues", SCTP_MIB_OUTOFBLUES),
    SNMP_MIB_ITEM!("SctpChecksumErrors", SCTP_MIB_CHECKSUMERRORS),
    SNMP_MIB_ITEM!("SctpOutCtrlChunks", SCTP_MIB_OUTCTRLCHUNKS),
    SNMP_MIB_ITEM!("SctpOutOrderChunks", SCTP_MIB_OUTORDERCHUNKS),
    SNMP_MIB_ITEM!("SctpOutUnorderChunks", SCTP_MIB_OUTUNORDERCHUNKS),
    SNMP_MIB_ITEM!("SctpInCtrlChunks", SCTP_MIB_INCTRLCHUNKS),
    SNMP_MIB_ITEM!("SctpInOrderChunks", SCTP_MIB_INORDERCHUNKS),
    SNMP_MIB_ITEM!("SctpInUnorderChunks", SCTP_MIB_INUNORDERCHUNKS),
    SNMP_MIB_ITEM!("SctpFragUsrMsgs", SCTP_MIB_FRAGUSRMSGS),
    SNMP_MIB_ITEM!("SctpReasmUsrMsgs", SCTP_MIB_REASMUSRMSGS),
    SNMP_MIB_ITEM!("SctpOutSCTPPacks", SCTP_MIB_OUTSCTPPACKS),
    SNMP_MIB_ITEM!("SctpInSCTPPacks", SCTP_MIB_INSCTPPACKS),
    SNMP_MIB_ITEM!("SctpT1InitExpireds", SCTP_MIB_T1_INIT_EXPIREDS),
    SNMP_MIB_ITEM!("SctpT1CookieExpireds", SCTP_MIB_T1_COOKIE_EXPIREDS),
    SNMP_MIB_ITEM!("SctpT2ShutdownExpireds", SCTP_MIB_T2_SHUTDOWN_EXPIREDS),
    SNMP_MIB_ITEM!("SctpT3RtxExpireds", SCTP_MIB_T3_RTX_EXPIREDS),
    SNMP_MIB_ITEM!("SctpT4RtoExpireds", SCTP_MIB_T4_RTO_EXPIREDS),
    SNMP_MIB_ITEM!("SctpT5ShutdownGuardExpireds", SCTP_MIB_T5_SHUTDOWN_GUARD_EXPIREDS),
    SNMP_MIB_ITEM!("SctpDelaySackExpireds", SCTP_MIB_DELAY_SACK_EXPIREDS),
    SNMP_MIB_ITEM!("SctpAutocloseExpireds", SCTP_MIB_AUTOCLOSE_EXPIREDS),
    SNMP_MIB_ITEM!("SctpT3Retransmits", SCTP_MIB_T3_RETRANSMITS),
    SNMP_MIB_ITEM!("SctpPmtudRetransmits", SCTP_MIB_PMTUD_RETRANSMITS),
    SNMP_MIB_ITEM!("SctpFastRetransmits", SCTP_MIB_FAST_RETRANSMITS),
    SNMP_MIB_ITEM!("SctpInPktSoftirq", SCTP_MIB_IN_PKT_SOFTIRQ),
    SNMP_MIB_ITEM!("SctpInPktBacklog", SCTP_MIB_IN_PKT_BACKLOG),
    SNMP_MIB_ITEM!("SctpInPktDiscards", SCTP_MIB_IN_PKT_DISCARDS),
    SNMP_MIB_ITEM!("SctpInDataChunkDiscards", SCTP_MIB_IN_DATA_CHUNK_DISCARDS),
];

/* Display sctp snmp mib statistics(/proc/net/sctp/snmp). */
unsafe fn sctp_snmp_seq_show(seq: *mut seq_file, _v: *mut core::ffi::c_void) -> i32 {
    let mut buff = [0ul; SCTP_SNMP_LIST.len()];
    let cnt = SCTP_SNMP_LIST.len() as i32;
    let net = (*seq).private as *mut net;

    memset(buff.as_mut_ptr() as *mut core::ffi::c_void, 0, core::mem::size_of_val(&buff));
    snmp_get_cpu_field_batch_cnt(buff.as_mut_ptr(), SCTP_SNMP_LIST.as_ptr(), cnt,
                                 (*net).sctp.sctp_statistics);
    for i in 0..cnt {
        seq_printf(seq, "%-32s\t%ld\n", SCTP_SNMP_LIST[i as usize].name, buff[i as usize]);
    }
    0
}

/* Dump local addresses of an association/endpoint. */
unsafe fn sctp_seq_dump_local_addrs(seq: *mut seq_file, epb: *mut sctp_ep_common) {
    let mut primary: *mut sctp_addr = core::ptr::null_mut();
    if (*epb).type_ == SCTP_EP_TYPE_ASSOCIATION {
        let asoc = sctp_assoc(epb);
        let peer = (*asoc).peer.primary_path;
        if peer.is_null() { WARN!(1, "Association %p with NULL primary path!\n", asoc); return; }
        primary = &mut (*peer).saddr;
    }
    rcu_read_lock();
    list_for_each_entry_rcu!(laddr, (*epb).bind_addr.address_list, list, {
        if !laddr.valid { continue; }
        let addr = &mut laddr.a;
        let af = sctp_get_af_specific(addr.sa.sa_family);
        if !primary.is_null() && ((*af).cmp_addr)(addr, primary) { seq_printf(seq, "*"); }
        ((*af).seq_dump_addr)(seq, addr);
    });
    rcu_read_unlock();
}

/* Dump remote addresses of an association. */
unsafe fn sctp_seq_dump_remote_addrs(seq: *mut seq_file, assoc: *mut sctp_association) {
    let primary = &mut (*assoc).peer.primary_addr;
    list_for_each_entry_rcu!(transport, (*assoc).peer.transport_addr_list, transports, {
        let addr = &mut transport.ipaddr;
        let af = sctp_get_af_specific(addr.sa.sa_family);
        if ((*af).cmp_addr)(addr, primary) { seq_printf(seq, "*"); }
        ((*af).seq_dump_addr)(seq, addr);
    });
}

unsafe fn sctp_eps_seq_start(seq: *mut seq_file, pos: *mut loff_t) -> *mut core::ffi::c_void {
    if *pos >= sctp_ep_hashsize { return core::ptr::null_mut(); }
    if *pos < 0 { *pos = 0; }
    if *pos == 0 { seq_printf(seq, " ENDPT     SOCK   STY SST HBKT LPORT   UID INODE LADDRS\n"); }
    pos as *mut core::ffi::c_void
}
unsafe fn sctp_eps_seq_stop(_seq: *mut seq_file, _v: *mut core::ffi::c_void) {}
unsafe fn sctp_eps_seq_next(_seq: *mut seq_file, _v: *mut core::ffi::c_void, pos: *mut loff_t) -> *mut core::ffi::c_void {
    *pos += 1;
    if *pos >= sctp_ep_hashsize { return core::ptr::null_mut(); }
    pos as *mut core::ffi::c_void
}

/* Display sctp endpoints (/proc/net/sctp/eps). */
unsafe fn sctp_eps_seq_show(seq: *mut seq_file, v: *mut core::ffi::c_void) -> i32 {
    let hash = *(v as *mut loff_t);
    if hash >= sctp_ep_hashsize { return -ENOMEM; }
    let head = &mut sctp_ep_hashtable[hash as usize];
    read_lock_bh(&mut head.lock);
    sctp_for_each_hentry!(ep, head.chain, {
        let sk = ep.base.sk;
        if !net_eq(sock_net(sk), seq_file_net(seq)) { continue; }
        seq_printf(seq, "%8pK %8pK %-3d %-3d %-4d %-5d %5u %5llu ", ep, sk,
                   (*sctp_sk(sk)).type_, (*sk).sk_state, hash,
                   ep.base.bind_addr.port, from_kuid_munged(seq_user_ns(seq), sk_uid(sk)), sock_i_ino(sk));
        sctp_seq_dump_local_addrs(seq, &mut ep.base);
        seq_printf(seq, "\n");
    });
    read_unlock_bh(&mut head.lock);
    0
}

static sctp_eps_ops: seq_operations = seq_operations { start: Some(sctp_eps_seq_start), next: Some(sctp_eps_seq_next), stop: Some(sctp_eps_seq_stop), show: Some(sctp_eps_seq_show) };

struct sctp_ht_iter { p: seq_net_private, hti: rhashtable_iter }

unsafe fn sctp_transport_seq_start(seq: *mut seq_file, pos: *mut loff_t) -> *mut core::ffi::c_void {
    let iter = (*seq).private as *mut sctp_ht_iter;
    sctp_transport_walk_start(&mut (*iter).hti);
    sctp_transport_get_idx(seq_file_net(seq), &mut (*iter).hti, *pos)
}
unsafe fn sctp_transport_seq_stop(seq: *mut seq_file, v: *mut core::ffi::c_void) {
    let iter = (*seq).private as *mut sctp_ht_iter;
    if !v.is_null() && v != SEQ_START_TOKEN { sctp_transport_put(v as *mut sctp_transport); }
    sctp_transport_walk_stop(&mut (*iter).hti);
}
unsafe fn sctp_transport_seq_next(seq: *mut seq_file, v: *mut core::ffi::c_void, pos: *mut loff_t) -> *mut core::ffi::c_void {
    let iter = (*seq).private as *mut sctp_ht_iter;
    if !v.is_null() && v != SEQ_START_TOKEN { sctp_transport_put(v as *mut sctp_transport); }
    *pos += 1;
    sctp_transport_get_next(seq_file_net(seq), &mut (*iter).hti)
}

/* Display sctp associations (/proc/net/sctp/assocs). */
unsafe fn sctp_assocs_seq_show(seq: *mut seq_file, v: *mut core::ffi::c_void) -> i32 {
    if v == SEQ_START_TOKEN {
        seq_printf(seq, " ASSOC     SOCK   STY SST ST HBKT ASSOC-ID TX_QUEUE RX_QUEUE UID INODE LPORT RPORT LADDRS <-> RADDRS HBINT INS OUTS MAXRT T1X T2X RTXC wmema wmemq sndbuf rcvbuf\n");
        return 0;
    }
    let transport = v as *mut sctp_transport;
    let assoc = (*transport).asoc;
    let epb = &mut (*assoc).base;
    let sk = (*epb).sk;
    seq_printf(seq, "%8pK %8pK %-3d %-3d %-2d %-4d %4d %8d %8d %7u %5llu %-5d %5d ", assoc, sk, (*sctp_sk(sk)).type_, (*sk).sk_state, (*assoc).state, 0, (*assoc).assoc_id, (*assoc).sndbuf_used, atomic_read(&mut (*assoc).rmem_alloc), from_kuid_munged(seq_user_ns(seq), sk_uid(sk)), sock_i_ino(sk), (*epb).bind_addr.port, (*assoc).peer.port);
    seq_printf(seq, " "); sctp_seq_dump_local_addrs(seq, epb); seq_printf(seq, "<-> "); sctp_seq_dump_remote_addrs(seq, assoc);
    seq_printf(seq, "\t%8lu %5d %5d %4d %4d %4d %8d %8d %8d %8d %8d", (*assoc).hbinterval, (*assoc).stream.incnt, (*assoc).stream.outcnt, (*assoc).max_retrans, (*assoc).init_retries, (*assoc).shutdown_retries, (*assoc).rtx_data_chunks, refcount_read(&mut (*sk).sk_wmem_alloc), READ_ONCE!((*sk).sk_wmem_queued), (*sk).sk_sndbuf, (*sk).sk_rcvbuf);
    seq_printf(seq, "\n"); 0
}

static sctp_assoc_ops: seq_operations = seq_operations { start: Some(sctp_transport_seq_start), next: Some(sctp_transport_seq_next), stop: Some(sctp_transport_seq_stop), show: Some(sctp_assocs_seq_show) };

unsafe fn sctp_remaddr_seq_show(seq: *mut seq_file, v: *mut core::ffi::c_void) -> i32 {
    if v == SEQ_START_TOKEN { seq_printf(seq, "ADDR ASSOC_ID HB_ACT RTO MAX_PATH_RTX REM_ADDR_RTX START STATE\n"); return 0; }
    let transport = v as *mut sctp_transport; let assoc = (*transport).asoc;
    list_for_each_entry_rcu!(tsp, (*assoc).peer.transport_addr_list, transports, {
        ((*tsp).af_specific).seq_dump_addr(seq, &mut (*tsp).ipaddr); seq_printf(seq, " ");
        seq_printf(seq, "%d %d %lu %d ", (*tsp).asoc.assoc_id, timer_pending(&mut (*tsp).hb_timer), (*tsp).rto, (*tsp).pathmaxrxt);
        seq_puts(seq, "0 0 "); seq_printf(seq, "%d\n", (*tsp).state);
    }); 0
}
static sctp_remaddr_ops: seq_operations = seq_operations { start: Some(sctp_transport_seq_start), next: Some(sctp_transport_seq_next), stop: Some(sctp_transport_seq_stop), show: Some(sctp_remaddr_seq_show) };

/* Set up the proc fs entry for the SCTP protocol. */
unsafe fn sctp_proc_init(net: *mut net) -> i32 {
    (*net).sctp.proc_net_sctp = proc_net_mkdir(net, "sctp", (*net).proc_net);
    if (*net).sctp.proc_net_sctp.is_null() { return -ENOMEM; }
    if proc_create_net_single("snmp", 0o444, (*net).sctp.proc_net_sctp, sctp_snmp_seq_show, core::ptr::null_mut()).is_null() {
        remove_proc_subtree("sctp", (*net).proc_net); (*net).sctp.proc_net_sctp = core::ptr::null_mut(); return -ENOMEM;
    }
    if proc_create_net("eps", 0o444, (*net).sctp.proc_net_sctp, &sctp_eps_ops, core::mem::size_of::<seq_net_private>()).is_null() {
        remove_proc_subtree("sctp", (*net).proc_net); (*net).sctp.proc_net_sctp = core::ptr::null_mut(); return -ENOMEM;
    }
    if proc_create_net("assocs", 0o444, (*net).sctp.proc_net_sctp, &sctp_assoc_ops, core::mem::size_of::<sctp_ht_iter>()).is_null() {
        remove_proc_subtree("sctp", (*net).proc_net); (*net).sctp.proc_net_sctp = core::ptr::null_mut(); return -ENOMEM;
    }
    if proc_create_net("remaddr", 0o444, (*net).sctp.proc_net_sctp, &sctp_remaddr_ops, core::mem::size_of::<sctp_ht_iter>()).is_null() {
        remove_proc_subtree("sctp", (*net).proc_net); (*net).sctp.proc_net_sctp = core::ptr::null_mut(); return -ENOMEM;
    }
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
