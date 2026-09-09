// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * xfrm_proc.c
 *
 * Copyright (C)2006-2007 USAGI/WIDE Project
 *
 * Authors: Masahide NAKAMURA <nakam@linux-ipv6.org>
 */

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};

// Kernel-provided types and functions from the included headers are external
// dependencies of this translation.
#[repr(C)]
pub struct snmp_mib {
    pub name: *const c_char,
    pub entry: c_int,
}

#[repr(C)]
pub struct seq_file {
    pub private: *mut net,
}

#[repr(C)]
pub struct net {
    pub proc_net: *mut c_void,
    pub mib: mib_data,
}

#[repr(C)]
pub struct mib_data {
    pub xfrm_statistics: *mut c_ulong,
}

extern "C" {
    fn memset(s: *mut c_void, c: c_int, n: usize) -> *mut c_void;
    fn xfrm_state_update_stats(net: *mut net);
    fn snmp_get_cpu_field_batch_cnt(
        buff: *mut c_ulong,
        mib_list: *const snmp_mib,
        cnt: c_int,
        mib: *mut c_ulong,
    );
    fn seq_printf(seq: *mut seq_file, format: *const c_char, ...);
    fn proc_create_net_single(
        name: *const c_char,
        mode: c_uint,
        parent: *mut c_void,
        show: unsafe extern "C" fn(*mut seq_file, *mut c_void) -> c_int,
        data: *mut c_void,
    ) -> *mut c_void;
    fn remove_proc_entry(name: *const c_char, parent: *mut c_void);
}

// The LINUX_MIB_* values are supplied by the kernel SNMP definitions.
extern "C" {
    static LINUX_MIB_XFRMINERROR: c_int;
    static LINUX_MIB_XFRMINBUFFERERROR: c_int;
    static LINUX_MIB_XFRMINHDRERROR: c_int;
    static LINUX_MIB_XFRMINNOSTATES: c_int;
    static LINUX_MIB_XFRMINSTATEPROTOERROR: c_int;
    static LINUX_MIB_XFRMINSTATEMODEERROR: c_int;
    static LINUX_MIB_XFRMINSTATESEQERROR: c_int;
    static LINUX_MIB_XFRMINSTATEEXPIRED: c_int;
    static LINUX_MIB_XFRMINSTATEMISMATCH: c_int;
    static LINUX_MIB_XFRMINSTATEINVALID: c_int;
    static LINUX_MIB_XFRMINTMPLMISMATCH: c_int;
    static LINUX_MIB_XFRMINNOPOLS: c_int;
    static LINUX_MIB_XFRMINPOLBLOCK: c_int;
    static LINUX_MIB_XFRMINPOLERROR: c_int;
    static LINUX_MIB_XFRMOUTERROR: c_int;
    static LINUX_MIB_XFRMOUTBUNDLEGENERROR: c_int;
    static LINUX_MIB_XFRMOUTBUNDLECHECKERROR: c_int;
    static LINUX_MIB_XFRMOUTNOSTATES: c_int;
    static LINUX_MIB_XFRMOUTSTATEPROTOERROR: c_int;
    static LINUX_MIB_XFRMOUTSTATEMODEERROR: c_int;
    static LINUX_MIB_XFRMOUTSTATESEQERROR: c_int;
    static LINUX_MIB_XFRMOUTSTATEEXPIRED: c_int;
    static LINUX_MIB_XFRMOUTPOLBLOCK: c_int;
    static LINUX_MIB_XFRMOUTPOLDEAD: c_int;
    static LINUX_MIB_XFRMOUTPOLERROR: c_int;
    static LINUX_MIB_XFRMFWDHDRERROR: c_int;
    static LINUX_MIB_XFRMOUTSTATEINVALID: c_int;
    static LINUX_MIB_XFRMACQUIREERROR: c_int;
    static LINUX_MIB_XFRMOUTSTATEDIRERROR: c_int;
    static LINUX_MIB_XFRMINSTATEDIRERROR: c_int;
    static LINUX_MIB_XFRMINIPTFSERROR: c_int;
    static LINUX_MIB_XFRMOUTNOQSPACE: c_int;
}

macro_rules! snmp_mib_item {
    ($name:literal, $entry:ident) => {
        snmp_mib { name: concat!($name, "\0").as_ptr() as *const c_char, entry: unsafe { $entry } }
    };
}

static XFRM_MIB_LIST: [snmp_mib; 32] = [
    snmp_mib_item!("XfrmInError", LINUX_MIB_XFRMINERROR), snmp_mib_item!("XfrmInBufferError", LINUX_MIB_XFRMINBUFFERERROR), snmp_mib_item!("XfrmInHdrError", LINUX_MIB_XFRMINHDRERROR), snmp_mib_item!("XfrmInNoStates", LINUX_MIB_XFRMINNOSTATES),
    snmp_mib_item!("XfrmInStateProtoError", LINUX_MIB_XFRMINSTATEPROTOERROR), snmp_mib_item!("XfrmInStateModeError", LINUX_MIB_XFRMINSTATEMODEERROR), snmp_mib_item!("XfrmInStateSeqError", LINUX_MIB_XFRMINSTATESEQERROR), snmp_mib_item!("XfrmInStateExpired", LINUX_MIB_XFRMINSTATEEXPIRED),
    snmp_mib_item!("XfrmInStateMismatch", LINUX_MIB_XFRMINSTATEMISMATCH), snmp_mib_item!("XfrmInStateInvalid", LINUX_MIB_XFRMINSTATEINVALID), snmp_mib_item!("XfrmInTmplMismatch", LINUX_MIB_XFRMINTMPLMISMATCH), snmp_mib_item!("XfrmInNoPols", LINUX_MIB_XFRMINNOPOLS),
    snmp_mib_item!("XfrmInPolBlock", LINUX_MIB_XFRMINPOLBLOCK), snmp_mib_item!("XfrmInPolError", LINUX_MIB_XFRMINPOLERROR), snmp_mib_item!("XfrmOutError", LINUX_MIB_XFRMOUTERROR), snmp_mib_item!("XfrmOutBundleGenError", LINUX_MIB_XFRMOUTBUNDLEGENERROR),
    snmp_mib_item!("XfrmOutBundleCheckError", LINUX_MIB_XFRMOUTBUNDLECHECKERROR), snmp_mib_item!("XfrmOutNoStates", LINUX_MIB_XFRMOUTNOSTATES), snmp_mib_item!("XfrmOutStateProtoError", LINUX_MIB_XFRMOUTSTATEPROTOERROR), snmp_mib_item!("XfrmOutStateModeError", LINUX_MIB_XFRMOUTSTATEMODEERROR),
    snmp_mib_item!("XfrmOutStateSeqError", LINUX_MIB_XFRMOUTSTATESEQERROR), snmp_mib_item!("XfrmOutStateExpired", LINUX_MIB_XFRMOUTSTATEEXPIRED), snmp_mib_item!("XfrmOutPolBlock", LINUX_MIB_XFRMOUTPOLBLOCK), snmp_mib_item!("XfrmOutPolDead", LINUX_MIB_XFRMOUTPOLDEAD),
    snmp_mib_item!("XfrmOutPolError", LINUX_MIB_XFRMOUTPOLERROR), snmp_mib_item!("XfrmFwdHdrError", LINUX_MIB_XFRMFWDHDRERROR), snmp_mib_item!("XfrmOutStateInvalid", LINUX_MIB_XFRMOUTSTATEINVALID), snmp_mib_item!("XfrmAcquireError", LINUX_MIB_XFRMACQUIREERROR),
    snmp_mib_item!("XfrmOutStateDirError", LINUX_MIB_XFRMOUTSTATEDIRERROR), snmp_mib_item!("XfrmInStateDirError", LINUX_MIB_XFRMINSTATEDIRERROR), snmp_mib_item!("XfrmInIptfsError", LINUX_MIB_XFRMINIPTFSERROR), snmp_mib_item!("XfrmOutNoQueueSpace", LINUX_MIB_XFRMOUTNOQSPACE),
];

unsafe extern "C" fn xfrm_statistics_seq_show(seq: *mut seq_file, _v: *mut c_void) -> c_int {
    let mut buff = [0 as c_ulong; XFRM_MIB_LIST.len()];
    let cnt = XFRM_MIB_LIST.len() as c_int;
    let net = (*seq).private;
    memset(buff.as_mut_ptr() as *mut c_void, 0, core::mem::size_of_val(&buff));
    xfrm_state_update_stats(net);
    snmp_get_cpu_field_batch_cnt(buff.as_mut_ptr(), XFRM_MIB_LIST.as_ptr(), cnt, (*net).mib.xfrm_statistics);
    let mut i = 0;
    while i < cnt {
        seq_printf(seq, b"%-24s\t%lu\n\0".as_ptr() as *const c_char, XFRM_MIB_LIST[i as usize].name, buff[i as usize]);
        i += 1;
    }
    0
}

pub unsafe extern "C" fn xfrm_proc_init(net: *mut net) -> c_int {
    if proc_create_net_single(b"xfrm_stat\0".as_ptr() as *const c_char, 0o444, (*net).proc_net, xfrm_statistics_seq_show, core::ptr::null_mut()).is_null() {
        return -12;
    }
    0
}

pub unsafe extern "C" fn xfrm_proc_fini(net: *mut net) {
    remove_proc_entry(b"xfrm_stat\0".as_ptr() as *const c_char, (*net).proc_net);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
