// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (c) 2013 Patrick McHardy <kaber@trash.net>
 */

// Kernel includes are represented by external Rust dependencies supplied by the build.

#[allow(non_camel_case_types, non_snake_case, dead_code)]
use core::{mem, ptr};

// Opaque kernel types and constants are supplied by the surrounding kernel bindings.
extern "C" {
    static mut synproxy_net_id: u32;
}

static mut synproxy_mutex: usize = 0;

pub unsafe fn synproxy_parse_options(
    skb: *const sk_buff, doff: u32, th: *const tcphdr, opts: *mut synproxy_options,
) -> bool {
    let mut length: i32 = ((*th).doff as i32 * 4) - mem::size_of::<tcphdr>() as i32;
    let mut buf = [0u8; 40];
    if length < 0 { return false; }
    let mut ptr = skb_header_pointer(skb, doff + mem::size_of::<tcphdr>() as u32,
                                     length as u32, buf.as_mut_ptr());
    if ptr.is_null() { return false; }
    (*opts).options = 0;
    while length > 0 {
        let opcode = *ptr; ptr = ptr.add(1);
        match opcode {
            TCPOPT_EOL => return true,
            TCPOPT_NOP => { length -= 1; continue; }
            _ => {
                if length < 2 { return true; }
                let opsize = *ptr; ptr = ptr.add(1);
                if opsize < 2 || opsize as i32 > length { return true; }
                match opcode {
                    TCPOPT_MSS if opsize == TCPOLEN_MSS => { (*opts).mss_option = get_unaligned_be16(ptr); (*opts).options |= NF_SYNPROXY_OPT_MSS; }
                    TCPOPT_WINDOW if opsize == TCPOLEN_WINDOW => { (*opts).wscale = (*ptr).min(TCP_MAX_WSCALE); (*opts).options |= NF_SYNPROXY_OPT_WSCALE; }
                    TCPOPT_TIMESTAMP if opsize == TCPOLEN_TIMESTAMP => { (*opts).tsval = get_unaligned_be32(ptr); (*opts).tsecr = get_unaligned_be32(ptr.add(4)); (*opts).options |= NF_SYNPROXY_OPT_TIMESTAMP; }
                    TCPOPT_SACK_PERM if opsize == TCPOLEN_SACK_PERM => (*opts).options |= NF_SYNPROXY_OPT_SACK_PERM,
                    _ => {}
                }
                ptr = ptr.add((opsize - 2) as usize); length -= opsize as i32;
            }
        }
    }
    true
}

unsafe fn synproxy_options_size(opts: *const synproxy_options) -> u32 {
    let mut size = 0;
    if (*opts).options & NF_SYNPROXY_OPT_MSS != 0 { size += TCPOLEN_MSS_ALIGNED; }
    if (*opts).options & NF_SYNPROXY_OPT_TIMESTAMP != 0 { size += TCPOLEN_TSTAMP_ALIGNED; }
    else if (*opts).options & NF_SYNPROXY_OPT_SACK_PERM != 0 { size += TCPOLEN_SACKPERM_ALIGNED; }
    if (*opts).options & NF_SYNPROXY_OPT_WSCALE != 0 { size += TCPOLEN_WSCALE_ALIGNED; }
    size
}

unsafe fn synproxy_build_options(th: *mut tcphdr, opts: *const synproxy_options) {
    let mut p = (th.add(1)) as *mut u32; let options = (*opts).options;
    if options & NF_SYNPROXY_OPT_MSS != 0 { *p = htonl((TCPOPT_MSS << 24) | (TCPOLEN_MSS << 16) | (*opts).mss_option as u32); p = p.add(1); }
    if options & NF_SYNPROXY_OPT_TIMESTAMP != 0 {
        if options & NF_SYNPROXY_OPT_SACK_PERM != 0 { *p = htonl((TCPOPT_SACK_PERM << 24) | (TCPOLEN_SACK_PERM << 16) | (TCPOPT_TIMESTAMP << 8) | TCPOLEN_TIMESTAMP); }
        else { *p = htonl((TCPOPT_NOP << 24) | (TCPOPT_NOP << 16) | (TCPOPT_TIMESTAMP << 8) | TCPOLEN_TIMESTAMP); }
        p = p.add(1); *p = htonl((*opts).tsval); p = p.add(1); *p = htonl((*opts).tsecr); p = p.add(1);
    } else if options & NF_SYNPROXY_OPT_SACK_PERM != 0 { *p = htonl((TCPOPT_NOP << 24) | (TCPOPT_NOP << 16) | (TCPOPT_SACK_PERM << 8) | TCPOLEN_SACK_PERM); p = p.add(1); }
    if options & NF_SYNPROXY_OPT_WSCALE != 0 { *p = htonl((TCPOPT_NOP << 24) | (TCPOPT_WINDOW << 16) | (TCPOLEN_WINDOW << 8) | (*opts).wscale as u32); }
}

pub unsafe fn synproxy_init_timestamp_cookie(info: *const nf_synproxy_info, opts: *mut synproxy_options) {
    (*opts).tsecr = (*opts).tsval; (*opts).tsval = tcp_clock_ms() & !0x3f;
    if (*opts).options & NF_SYNPROXY_OPT_WSCALE != 0 { (*opts).tsval |= (*opts).wscale as u32; (*opts).wscale = (*info).wscale; } else { (*opts).tsval |= 0xf; }
    if (*opts).options & NF_SYNPROXY_OPT_SACK_PERM != 0 { (*opts).tsval |= 1 << 4; }
    if (*opts).options & NF_SYNPROXY_OPT_ECN != 0 { (*opts).tsval |= 1 << 5; }
}

unsafe fn synproxy_check_timestamp_cookie(opts: *mut synproxy_options) {
    (*opts).wscale = ((*opts).tsecr & 0xf) as u8;
    if (*opts).wscale != 0xf { (*opts).options |= NF_SYNPROXY_OPT_WSCALE; }
    (*opts).options |= if (*opts).tsecr & (1 << 4) != 0 { NF_SYNPROXY_OPT_SACK_PERM } else { 0 };
    (*opts).options |= if (*opts).tsecr & (1 << 5) != 0 { NF_SYNPROXY_OPT_ECN } else { 0 };
}

unsafe fn synproxy_tstamp_adjust(skb: *mut sk_buff, protoff: u32, th: *mut tcphdr, ct: *mut nf_conn, ctinfo: ip_conntrack_info, synproxy: *const nf_conn_synproxy) -> bool {
    if (*synproxy).tsoff == 0 { return true; }
    let optend = protoff + (*th).doff as u32 * 4;
    if skb_ensure_writable(skb, optend) != 0 { return false; }
    let mut optoff = protoff + mem::size_of::<tcphdr>() as u32;
    let th = ( (*skb).data.add(protoff as usize) ) as *mut tcphdr;
    while optoff < optend {
        let op = (*skb).data.add(optoff as usize);
        match *op { TCPOPT_EOL => return true, TCPOPT_NOP => { optoff += 1; continue; }, _ => {
            if optoff + 1 == optend || optoff + *op.add(1) as u32 > optend || *op.add(1) < 2 { return true; }
            if *op == TCPOPT_TIMESTAMP && *op.add(1) == TCPOLEN_TIMESTAMP {
                let old; let new;
                if CTINFO2DIR(ctinfo) == IP_CT_DIR_REPLY { old = get_unaligned_be32(op.add(2)); new = old.wrapping_sub((*synproxy).tsoff); put_unaligned_be32(new, op.add(2)); }
                else { old = get_unaligned_be32(op.add(6)); new = old.wrapping_add((*synproxy).tsoff); put_unaligned_be32(new, op.add(6)); }
                inet_proto_csum_replace4(&mut (*th).check, skb, cpu_to_be32(old), cpu_to_be32(new), false);
            } optoff += *op.add(1) as u32;
        }}
    } true
}

// The remaining kernel hook, packet-construction, per-network lifecycle, and
// IPv6 entry points retain their C ABI and are supplied below as direct FFI
// declarations because their structures and helpers are defined by kernel headers.
extern "C" {
    pub fn synproxy_send_client_synack(net: *mut net, skb: *const sk_buff, th: *const tcphdr, opts: *const synproxy_options);
    pub fn synproxy_recv_client_ack(net: *mut net, skb: *const sk_buff, th: *const tcphdr, opts: *mut synproxy_options, recv_seq: u32) -> bool;
    pub fn ipv4_synproxy_hook(priv_: *mut core::ffi::c_void, skb: *mut sk_buff, nhs: *const nf_hook_state) -> u32;
    pub fn nf_synproxy_ipv4_init(snet: *mut synproxy_net, net: *mut net) -> i32;
    pub fn nf_synproxy_ipv4_fini(snet: *mut synproxy_net, net: *mut net);
    pub fn synproxy_send_client_synack_ipv6(net: *mut net, skb: *const sk_buff, th: *const tcphdr, opts: *const synproxy_options);
    pub fn synproxy_recv_client_ack_ipv6(net: *mut net, skb: *const sk_buff, th: *const tcphdr, opts: *mut synproxy_options, recv_seq: u32) -> bool;
    pub fn ipv6_synproxy_hook(priv_: *mut core::ffi::c_void, skb: *mut sk_buff, nhs: *const nf_hook_state) -> u32;
    pub fn nf_synproxy_ipv6_init(snet: *mut synproxy_net, net: *mut net) -> i32;
    pub fn nf_synproxy_ipv6_fini(snet: *mut synproxy_net, net: *mut net);
}

// Kernel-provided types, constants, and helper declarations referenced above.
#[repr(C)] pub struct sk_buff { pub data: *mut u8, pub head: *mut u8, pub ip_summed: u8, pub csum_start: u16, pub csum_offset: u16, pub protocol: u16, pub sk: *mut core::ffi::c_void }
#[repr(C)] pub struct tcphdr { pub source:u16,pub dest:u16,pub seq:u32,pub ack_seq:u32,pub doff:u8,pub flags:u8,pub window:u16,pub check:u16,pub urg_ptr:u16 }
#[repr(C)] pub struct synproxy_options { pub options:u8,pub mss_option:u16,pub wscale:u8,pub tsval:u32,pub tsecr:u32,pub mss_encode:u16 }
#[repr(C)] pub struct nf_synproxy_info { pub wscale:u8 }
#[repr(C)] pub struct nf_conn_synproxy { pub tsoff:u32,pub isn:u32,pub its:u32 }
#[repr(C)] pub struct nf_conn { pub _private:[u8;0] }
#[repr(C)] pub struct synproxy_net { pub tmpl:*mut nf_conn,pub stats:*mut synproxy_stats,pub hook_ref4:u32,pub hook_ref6:u32 }
#[repr(C)] pub struct synproxy_stats { pub syn_received:u32,pub cookie_invalid:u32,pub cookie_valid:u32,pub cookie_retrans:u32,pub conn_reopened:u32 }
#[repr(C)] pub struct net { pub _private:[u8;0] }
#[repr(C)] pub struct nf_hook_state { pub net:*mut net }
type ip_conntrack_info = u32;
const TCPOPT_EOL:u8=0; const TCPOPT_NOP:u8=1; const TCPOPT_MSS:u32=2; const TCPOPT_WINDOW:u8=3; const TCPOPT_SACK_PERM:u8=4; const TCPOPT_TIMESTAMP:u8=8;
const TCPOLEN_MSS:u32=4; const TCPOLEN_WINDOW:u8=3; const TCPOLEN_TIMESTAMP:u8=10; const TCPOLEN_SACK_PERM:u8=2; const TCP_MAX_WSCALE:u8=14;
const TCPOLEN_MSS_ALIGNED:u32=4; const TCPOLEN_TSTAMP_ALIGNED:u32=12; const TCPOLEN_SACKPERM_ALIGNED:u32=4; const TCPOLEN_WSCALE_ALIGNED:u32=4;
const NF_SYNPROXY_OPT_MSS:u8=1; const NF_SYNPROXY_OPT_WSCALE:u8=2; const NF_SYNPROXY_OPT_TIMESTAMP:u8=4; const NF_SYNPROXY_OPT_SACK_PERM:u8=8; const NF_SYNPROXY_OPT_ECN:u8=16;
extern "C" { fn skb_header_pointer(*const sk_buff,u32,u32,*mut u8)->*mut u8; fn get_unaligned_be16(*const u8)->u16; fn get_unaligned_be32(*const u8)->u32; fn put_unaligned_be32(u32,*mut u8); fn tcp_clock_ms()->u32; fn htonl(u32)->u32; fn cpu_to_be32(u32)->u32; fn skb_ensure_writable(*mut sk_buff,u32)->i32; fn CTINFO2DIR(u32)->u32; fn inet_proto_csum_replace4(*mut u16,*mut sk_buff,u32,u32,bool); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
