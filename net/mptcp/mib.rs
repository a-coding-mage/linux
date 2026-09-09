// SPDX-License-Identifier: GPL-2.0-or-later

use core::ffi::{c_char, c_int, c_ulong, c_void};

// Types and symbols supplied by the surrounding kernel sources.
#[repr(C)]
pub struct snmp_mib {
    pub name: *const c_char,
    pub entry: c_int,
}

#[repr(C)]
pub struct mptcp_mib;

#[repr(C)]
pub struct net_mib {
    pub mptcp_statistics: *mut mptcp_mib,
}

#[repr(C)]
pub struct net {
    pub mib: net_mib,
}

#[repr(C)]
pub struct seq_file {
    pub private: *mut c_void,
}

extern "C" {
    fn alloc_percpu(size: usize) -> *mut mptcp_mib;
    fn free_percpu(ptr: *mut mptcp_mib);
    fn seq_puts(seq: *mut seq_file, s: *const c_char);
    fn seq_putc(seq: *mut seq_file, c: c_int);
    fn seq_printf(seq: *mut seq_file, fmt: *const c_char, ...);
    fn memset(dst: *mut c_void, value: c_int, size: usize) -> *mut c_void;
    fn snmp_get_cpu_field_batch_cnt(
        sum: *mut c_ulong,
        list: *const snmp_mib,
        cnt: c_int,
        mib: *mut mptcp_mib,
    );
}

// The MPTCP_MIB_* values are provided by mptcp's MIB declarations.
macro_rules! snmp_mib_item {
    ($name:literal, $entry:expr) => {
        snmp_mib { name: concat!($name, "\0").as_ptr() as *const c_char, entry: $entry }
    };
}

static mptcp_snmp_list: [snmp_mib; 83] = [
    snmp_mib_item!("MPCapableSYNRX", MPTCP_MIB_MPCAPABLEPASSIVE), snmp_mib_item!("MPCapableSYNTX", MPTCP_MIB_MPCAPABLEACTIVE),
    snmp_mib_item!("MPCapableSYNACKRX", MPTCP_MIB_MPCAPABLEACTIVEACK), snmp_mib_item!("MPCapableACKRX", MPTCP_MIB_MPCAPABLEPASSIVEACK),
    snmp_mib_item!("MPCapableFallbackACK", MPTCP_MIB_MPCAPABLEPASSIVEFALLBACK), snmp_mib_item!("MPCapableFallbackSYNACK", MPTCP_MIB_MPCAPABLEACTIVEFALLBACK),
    snmp_mib_item!("MPCapableSYNTXDrop", MPTCP_MIB_MPCAPABLEACTIVEDROP), snmp_mib_item!("MPCapableSYNTXDisabled", MPTCP_MIB_MPCAPABLEACTIVEDISABLED),
    snmp_mib_item!("MPCapableEndpAttempt", MPTCP_MIB_MPCAPABLEENDPATTEMPT), snmp_mib_item!("MPFallbackTokenInit", MPTCP_MIB_TOKENFALLBACKINIT),
    snmp_mib_item!("MPTCPRetrans", MPTCP_MIB_RETRANSSEGS), snmp_mib_item!("MPJoinNoTokenFound", MPTCP_MIB_JOINNOTOKEN),
    snmp_mib_item!("MPJoinNoIdFound", MPTCP_MIB_MPJOINNOIDFOUND), snmp_mib_item!("MPJoinSynRx", MPTCP_MIB_JOINSYNRX),
    snmp_mib_item!("MPJoinSynBackupRx", MPTCP_MIB_JOINSYNBACKUPRX), snmp_mib_item!("MPJoinSynAckRx", MPTCP_MIB_JOINSYNACKRX),
    snmp_mib_item!("MPJoinSynAckBackupRx", MPTCP_MIB_JOINSYNACKBACKUPRX), snmp_mib_item!("MPJoinSynAckHMacFailure", MPTCP_MIB_JOINSYNACKMAC),
    snmp_mib_item!("MPJoinSynAckNoMPJoin", MPTCP_MIB_MPJOINSYNACKNOMPJOIN), snmp_mib_item!("MPJoinAckRx", MPTCP_MIB_JOINACKRX),
    snmp_mib_item!("MPJoinAckHMacFailure", MPTCP_MIB_JOINACKMAC), snmp_mib_item!("MPJoinAckNoMPJoin", MPTCP_MIB_MPJOINACKNOMPJOIN),
    snmp_mib_item!("MPJoinAckNoCtx", MPTCP_MIB_MPJOINACKNOCTX), snmp_mib_item!("MPJoinRejected", MPTCP_MIB_JOINREJECTED),
    snmp_mib_item!("MPJoinNotEstablished", MPTCP_MIB_MPJOINNOTESTABLISHED), snmp_mib_item!("MPJoinSynTx", MPTCP_MIB_JOINSYNTX),
    snmp_mib_item!("MPJoinSynTxCreatSkErr", MPTCP_MIB_JOINSYNTXCREATSKERR), snmp_mib_item!("MPJoinSynTxBindErr", MPTCP_MIB_JOINSYNTXBINDERR),
    snmp_mib_item!("MPJoinSynTxConnectErr", MPTCP_MIB_JOINSYNTXCONNECTERR), snmp_mib_item!("DSSNotMatching", MPTCP_MIB_DSSNOMATCH),
    snmp_mib_item!("DSSCorruptionFallback", MPTCP_MIB_DSSCORRUPTIONFALLBACK), snmp_mib_item!("DSSCorruptionReset", MPTCP_MIB_DSSCORRUPTIONRESET),
    snmp_mib_item!("InfiniteMapTx", MPTCP_MIB_INFINITEMAPTX), snmp_mib_item!("InfiniteMapRx", MPTCP_MIB_INFINITEMAPRX),
    snmp_mib_item!("DSSNoMatchTCP", MPTCP_MIB_DSSTCPMISMATCH), snmp_mib_item!("DataCsumErr", MPTCP_MIB_DATACSUMERR),
    snmp_mib_item!("OFOQueueTail", MPTCP_MIB_OFOQUEUETAIL), snmp_mib_item!("OFOQueue", MPTCP_MIB_OFOQUEUE), snmp_mib_item!("OFOMerge", MPTCP_MIB_OFOMERGE),
    snmp_mib_item!("NoDSSInWindow", MPTCP_MIB_NODSSWINDOW), snmp_mib_item!("DuplicateData", MPTCP_MIB_DUPDATA), snmp_mib_item!("AddAddr", MPTCP_MIB_ADDADDR),
    snmp_mib_item!("AddAddrTx", MPTCP_MIB_ADDADDRTX), snmp_mib_item!("AddAddrTxDrop", MPTCP_MIB_ADDADDRTXDROP), snmp_mib_item!("EchoAdd", MPTCP_MIB_ECHOADD),
    snmp_mib_item!("EchoAddTx", MPTCP_MIB_ECHOADDTX), snmp_mib_item!("EchoAddTxDrop", MPTCP_MIB_ECHOADDTXDROP), snmp_mib_item!("PortAdd", MPTCP_MIB_PORTADD),
    snmp_mib_item!("AddAddrDrop", MPTCP_MIB_ADDADDRDROP), snmp_mib_item!("MPJoinPortSynRx", MPTCP_MIB_JOINPORTSYNRX), snmp_mib_item!("MPJoinPortSynAckRx", MPTCP_MIB_JOINPORTSYNACKRX),
    snmp_mib_item!("MPJoinPortAckRx", MPTCP_MIB_JOINPORTACKRX), snmp_mib_item!("MismatchPortSynRx", MPTCP_MIB_MISMATCHPORTSYNRX), snmp_mib_item!("MismatchPortAckRx", MPTCP_MIB_MISMATCHPORTACKRX),
    snmp_mib_item!("RmAddr", MPTCP_MIB_RMADDR), snmp_mib_item!("RmAddrDrop", MPTCP_MIB_RMADDRDROP), snmp_mib_item!("RmAddrTx", MPTCP_MIB_RMADDRTX),
    snmp_mib_item!("RmAddrTxDrop", MPTCP_MIB_RMADDRTXDROP), snmp_mib_item!("RmSubflow", MPTCP_MIB_RMSUBFLOW), snmp_mib_item!("MPPrioTx", MPTCP_MIB_MPPRIOTX),
    snmp_mib_item!("MPPrioRx", MPTCP_MIB_MPPRIORX), snmp_mib_item!("MPFailTx", MPTCP_MIB_MPFAILTX), snmp_mib_item!("MPFailRx", MPTCP_MIB_MPFAILRX),
    snmp_mib_item!("MPFastcloseTx", MPTCP_MIB_MPFASTCLOSETX), snmp_mib_item!("MPFastcloseRx", MPTCP_MIB_MPFASTCLOSERX), snmp_mib_item!("MPRstTx", MPTCP_MIB_MPRSTTX),
    snmp_mib_item!("MPRstRx", MPTCP_MIB_MPRSTRX), snmp_mib_item!("SubflowStale", MPTCP_MIB_SUBFLOWSTALE), snmp_mib_item!("SubflowRecover", MPTCP_MIB_SUBFLOWRECOVER),
    snmp_mib_item!("SndWndShared", MPTCP_MIB_SNDWNDSHARED), snmp_mib_item!("RcvWndShared", MPTCP_MIB_RCVWNDSHARED), snmp_mib_item!("RcvWndConflictUpdate", MPTCP_MIB_RCVWNDCONFLICTUPDATE),
    snmp_mib_item!("RcvWndConflict", MPTCP_MIB_RCVWNDCONFLICT), snmp_mib_item!("MPCurrEstab", MPTCP_MIB_CURRESTAB), snmp_mib_item!("Blackhole", MPTCP_MIB_BLACKHOLE),
    snmp_mib_item!("MPCapableDataFallback", MPTCP_MIB_MPCAPABLEDATAFALLBACK), snmp_mib_item!("MD5SigFallback", MPTCP_MIB_MD5SIGFALLBACK), snmp_mib_item!("MD5SigReset", MPTCP_MIB_MD5SIGRESET),
    snmp_mib_item!("DssFallback", MPTCP_MIB_DSSFALLBACK), snmp_mib_item!("DssReset", MPTCP_MIB_DSSRESET), snmp_mib_item!("SimultConnectFallback", MPTCP_MIB_SIMULTCONNFALLBACK),
    snmp_mib_item!("FallbackFailed", MPTCP_MIB_FALLBACKFAILED), snmp_mib_item!("WinProbe", MPTCP_MIB_WINPROBE), snmp_mib_item!("BacklogDrop", MPTCP_MIB_BACKLOGDROP),
    snmp_mib_item!("RcvPruned", MPTCP_MIB_RCVPRUNED), snmp_mib_item!("OFOPruned", MPTCP_MIB_OFOPRUNED),
];

pub unsafe fn mptcp_mib_alloc(net: *mut net) -> bool {
    let mib = alloc_percpu(core::mem::size_of::<mptcp_mib>());
    if mib.is_null() { return false; }
    if !(*net).mib.mptcp_statistics.is_null() { free_percpu(mib); }
    true
}

pub unsafe fn mptcp_seq_show(seq: *mut seq_file) {
    let mut sum = [0 as c_ulong; mptcp_snmp_list.len()];
    let cnt = mptcp_snmp_list.len() as c_int;
    let net = (*seq).private as *mut net;
    seq_puts(seq, b"MPTcpExt:\0".as_ptr() as *const c_char);
    for item in &mptcp_snmp_list { seq_printf(seq, b" %s\0".as_ptr() as *const c_char, item.name); }
    seq_puts(seq, b"\nMPTcpExt:\0".as_ptr() as *const c_char);
    memset(sum.as_mut_ptr() as *mut c_void, 0, core::mem::size_of_val(&sum));
    if !(*net).mib.mptcp_statistics.is_null() { snmp_get_cpu_field_batch_cnt(sum.as_mut_ptr(), mptcp_snmp_list.as_ptr(), cnt, (*net).mib.mptcp_statistics); }
    for value in &sum { seq_printf(seq, b" %lu\0".as_ptr() as *const c_char, *value); }
    seq_putc(seq, b'\n' as c_int);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
