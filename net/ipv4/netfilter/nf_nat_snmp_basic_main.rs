// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * nf_nat_snmp_basic.c
 *
 * Basic SNMP Application Layer Gateway
 *
 * This IP NAT module is intended for use with SNMP network
 * discovery and monitoring applications where target networks use
 * conflicting private address realms.
 *
 * Static NAT is used to remap the networks from the view of the network
 * management system at the IP layer, and this module remaps some application
 * layer addresses to match.
 *
 * The simplest form of ALG is performed, where only tagged IP addresses
 * are modified.  The module does not need to be MIB aware and only scans
 * messages at the ASN.1/BER level.
 *
 * Currently, only SNMPv1 and SNMPv2 are supported.
 *
 * More information on ALG and associated issues can be found in
 * RFC 2962
 *
 * The ASB.1/BER parsing code is derived from the gxsnmp package by Gregory
 * McLean & Jochen Friedrich, stripped down for use in the kernel.
 *
 * Copyright (c) 2000 RP Internet (www.rpi.net.au).
 *
 * Author: James Morris <jmorris@intercode.com.au>
 *
 * Copyright (c) 2006-2010 Patrick McHardy <kaber@trash.net>
 */

// External kernel declarations supplied by the surrounding translation unit.
extern "C" {
    static mut nf_nat_snmp_hook: *mut core::ffi::c_void;
    static nf_nat_snmp_basic_decoder: core::ffi::c_void;
}

const SNMP_PORT: u16 = 161;
const SNMP_TRAP_PORT: u16 = 162;

static mut snmp_lock: core::ffi::c_void = core::ffi::c_void;

#[repr(C)]
struct snmp_ctx {
    begin: *mut u8,
    check: *mut u16,
    from: u32,
    to: u32,
}

unsafe fn fast_csum(ctx: *mut snmp_ctx, offset: u8) {
    let mut s = [0u8; 12];
    let mut size: usize;

    if offset & 1 != 0 {
        core::ptr::copy_nonoverlapping((&(*ctx).from as *const u32).cast::<u8>(), s[1..].as_mut_ptr(), 4);
        core::ptr::copy_nonoverlapping((&(*ctx).to as *const u32).cast::<u8>(), s[7..].as_mut_ptr(), 4);
        s[0] = !0;
        s[1] = !s[1];
        s[2] = !s[2];
        s[3] = !s[3];
        s[4] = !s[4];
        s[5] = !0;
        size = 12;
    } else {
        core::ptr::copy_nonoverlapping((&(*ctx).from as *const u32).cast::<u8>(), s.as_mut_ptr(), 4);
        core::ptr::copy_nonoverlapping((&(*ctx).to as *const u32).cast::<u8>(), s[4..].as_mut_ptr(), 4);
        s[0] = !s[0];
        s[1] = !s[1];
        s[2] = !s[2];
        s[3] = !s[3];
        size = 8;
    }
    // csum_fold(csum_partial(s, size, ~csum_unfold(*ctx->check)))
    extern "C" {
        fn csum_fold(sum: u32) -> u16;
        fn csum_partial(buf: *const u8, len: usize, sum: u32) -> u32;
        fn csum_unfold(sum: u16) -> u32;
    }
    *(*ctx).check = csum_fold(csum_partial(s.as_ptr(), size, !csum_unfold(*(*ctx).check)));
}

#[no_mangle]
pub unsafe extern "C" fn snmp_version(
    _context: *mut core::ffi::c_void,
    _hdrlen: usize,
    _tag: u8,
    data: *const core::ffi::c_void,
    datalen: usize,
) -> i32 {
    if datalen != 1 { return -22; }
    if *(data as *const u8) > 1 { return -95; }
    1
}

#[no_mangle]
pub unsafe extern "C" fn snmp_helper(
    context: *mut core::ffi::c_void,
    _hdrlen: usize,
    _tag: u8,
    data: *const core::ffi::c_void,
    datalen: usize,
) -> i32 {
    let ctx = context as *mut snmp_ctx;
    if datalen != 4 { return -22; }
    let pdata = data as *mut u32;
    if *pdata == (*ctx).from {
        if *(*ctx).check != 0 {
            fast_csum(ctx, pdata.cast::<u8>().offset_from((*ctx).begin) as u8);
        }
        *pdata = (*ctx).to;
    }
    1
}

// The following kernel structures and helpers are supplied externally.
#[repr(C)] struct nf_conn { _private: [u8; 0] }
#[repr(C)] struct sk_buff { len: u32 }
#[repr(C)] struct iphdr { ihl: u8 }
#[repr(C)] struct udphdr { source: u16, dest: u16, check: u16 }

extern "C" {
    fn ip_hdr(skb: *mut sk_buff) -> *mut iphdr;
    fn udp_get_len_short(udph: *const udphdr) -> u16;
    fn asn1_ber_decoder(decoder: *const core::ffi::c_void, context: *mut snmp_ctx, data: *mut u8, len: u16) -> i32;
    fn nf_ct_helper_log(skb: *mut sk_buff, ct: *mut nf_conn, msg: *const u8);
    fn skb_ensure_writable(skb: *mut sk_buff, len: u32) -> i32;
    fn spin_lock_bh(lock: *mut core::ffi::c_void);
    fn spin_unlock_bh(lock: *mut core::ffi::c_void);
}

unsafe fn snmp_translate(_ct: *mut nf_conn, _dir: i32, _skb: *mut sk_buff) -> i32 {
    // TODO: the iphdr/udphdr and nf_conn tuplehash field layouts are supplied
    // by the external kernel headers. The source-level operations are:
    // let iph = ip_hdr(skb);
    // let udph = (iph as *mut u32).add((*iph).ihl as usize) as *mut udphdr;
    // let datalen = udp_get_len_short(udph) - size_of::<udphdr>() as u16;
    // let data = (udph as *mut u8).add(size_of::<udphdr>());
    // Select ctx.from/ctx.to from ct->tuplehash[dir] according to dir,
    // return NF_ACCEPT when equal, then call asn1_ber_decoder and return
    // NF_DROP on a negative parser result, otherwise NF_ACCEPT.
    1
}

// We don't actually set up expectations, just adjust internal IP
// addresses if this is being NATted
unsafe fn help(_skb: *mut sk_buff, _protoff: u32, _ct: *mut nf_conn, _ctinfo: i32) -> i32 {
    // TODO: packet and conntrack layouts are external. Preserve the source
    // ordering: reject non-reply SNMP requests and non-original traps early;
    // accept non-NAT packets; validate UDP length against the IP payload;
    // ensure writability; lock snmp_lock; call snmp_translate; unlock; return.
    1
}

#[repr(C)]
struct nf_conntrack_expect_policy { max_expected: u32, timeout: u32 }

static snmp_exp_policy: nf_conntrack_expect_policy = nf_conntrack_expect_policy { max_expected: 0, timeout: 180 };

#[repr(C)] struct nf_conntrack_helper { _private: [u8; 0] }
static mut snmp_trap_helper: nf_conntrack_helper = nf_conntrack_helper { _private: [] };
static mut snmp_trap_helper_ptr: *mut nf_conntrack_helper = core::ptr::null_mut();

unsafe fn nf_nat_snmp_basic_init() -> i32 {
    nf_nat_snmp_hook = help as *mut core::ffi::c_void;
    let _ = &snmp_exp_policy;
    0
}

unsafe fn nf_nat_snmp_basic_fini() {
    nf_nat_snmp_hook = core::ptr::null_mut();
}

// module_init(nf_nat_snmp_basic_init);
// module_exit(nf_nat_snmp_basic_fini);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
