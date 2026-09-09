// SPDX-License-Identifier: GPL-2.0-or-later
/* IRC extension for TCP NAT alteration.
 *
 * (C) 2000-2001 by Harald Welte <laforge@gnumonks.org>
 * (C) 2004 Rusty Russell <rusty@rustcorp.com.au> IBM Corporation
 * based on a copy of RR's ip_nat_ftp.c
 */

// Dependencies supplied by the kernel and other translation units are intentionally external.

const NAT_HELPER_NAME: &[u8] = b"irc\0";

extern "C" {
    static mut nat_helper_irc: nf_conntrack_nat_helper;
    static mut nf_nat_irc_hook: Option<unsafe extern "C" fn(
        *mut sk_buff, *mut nf_conn, ip_conntrack_info, u32, u32, u32,
        *mut nf_conntrack_expect,
    ) -> u32>;

    fn nf_nat_follow_master(exp: *mut nf_conntrack_expect);
    fn nf_nat_exp_find_port(exp: *mut nf_conntrack_expect, port: u16) -> u16;
    fn nf_ct_helper_log(skb: *mut sk_buff, ct: *mut nf_conn, msg: *const u8);
    fn nf_nat_mangle_tcp_packet(
        skb: *mut sk_buff, ct: *mut nf_conn, ctinfo: ip_conntrack_info,
        protoff: u32, matchoff: u32, matchlen: u32, buffer: *const u8,
        len: usize,
    ) -> bool;
    fn nf_ct_unexpect_related(exp: *mut nf_conntrack_expect);
    fn nf_nat_helper_unregister(helper: *mut nf_conntrack_nat_helper);
    fn nf_nat_helper_register(helper: *mut nf_conntrack_nat_helper) -> i32;
    fn synchronize_rcu();
}

#[repr(C)]
pub struct sk_buff;
#[repr(C)]
pub struct nf_conn;
#[repr(C)]
pub struct nf_conntrack_expect;
#[repr(C)]
pub struct nf_conntrack_nat_helper;
pub type ip_conntrack_info = i32;

const IP_CT_DIR_REPLY: usize = 1;
const NF_DROP: u32 = 0;
const NF_ACCEPT: u32 = 1;

#[repr(C)]
struct InAddr { ip: u32 }

// The surrounding kernel translation supplies the complete tuple and expectation layouts.
#[repr(C)]
struct TupleHash { tuple: Tuple }
#[repr(C)]
struct Tuple { dst: TupleEndpoint }
#[repr(C)]
struct TupleEndpoint { u3: InAddr, u: TupleProto }
#[repr(C)]
union TupleProto { tcp: TcpProto }
#[repr(C)]
struct TcpProto { port: u16 }

#[repr(C)]
struct ExpectProto { tcp: TcpProto }
#[repr(C)]
struct ExpectTuple { dst: TupleEndpoint }

#[inline]
unsafe fn ntohs(x: u16) -> u16 { u16::from_be(x) }
#[inline]
unsafe fn ntohl(x: u32) -> u32 { u32::from_be(x) }

unsafe fn help(
    skb: *mut sk_buff,
    ct: *mut nf_conn,
    ctinfo: ip_conntrack_info,
    protoff: u32,
    matchoff: u32,
    matchlen: u32,
    exp: *mut nf_conntrack_expect,
) -> u32 {
    let mut buffer = [0u8; 17];
    let mut newaddr: InAddr;
    let port: u16;

    // Reply comes from server.
    // Equivalent field access is provided by the complete nf_conn definition.
    newaddr = (*(ct as *mut ConnLayout)).tuplehash[IP_CT_DIR_REPLY].tuple.dst.u3;

    (* (exp as *mut ExpectLayout)).saved_proto.tcp.port =
        (* (exp as *mut ExpectLayout)).tuple.dst.u.tcp.port;
    (* (exp as *mut ExpectLayout)).dir = IP_CT_DIR_REPLY as u32;
    (* (exp as *mut ExpectLayout)).expectfn = Some(nf_nat_follow_master);

    port = nf_nat_exp_find_port(
        exp,
        ntohs((* (exp as *mut ExpectLayout)).saved_proto.tcp.port),
    );
    if port == 0 {
        nf_ct_helper_log(skb, ct, b"all ports in use\0".as_ptr());
        return NF_DROP;
    }

    // The original comments document the IRC DCC message layouts and field widths.
    let text = format!("{} {}", ntohl(newaddr.ip), port);
    let bytes = text.as_bytes();
    let copy_len = core::cmp::min(bytes.len(), buffer.len() - 1);
    buffer[..copy_len].copy_from_slice(&bytes[..copy_len]);

    if !nf_nat_mangle_tcp_packet(skb, ct, ctinfo, protoff, matchoff, matchlen,
                                 buffer.as_ptr(), copy_len) {
        nf_ct_helper_log(skb, ct, b"cannot mangle packet\0".as_ptr());
        nf_ct_unexpect_related(exp);
        return NF_DROP;
    }
    NF_ACCEPT
}

unsafe fn nf_nat_irc_fini() {
    nf_nat_helper_unregister(&mut nat_helper_irc);
    nf_nat_irc_hook = None;
    synchronize_rcu();
}

unsafe fn nf_nat_irc_init() -> i32 {
    // BUG_ON(nf_nat_irc_hook != NULL);
    nf_nat_helper_register(&mut nat_helper_irc);
    nf_nat_irc_hook = Some(help);
    0
}

/* Prior to 2.6.11, we had a ports param.  No longer, but don't break users. */
unsafe fn warn_set(_val: *const i8, _kp: *const core::ffi::c_void) -> i32 {
    // pr_info("kernel >= 2.6.10 only uses 'ports' for conntrack modules");
    0
}

#[repr(C)]
struct ConnLayout { tuplehash: [TupleHash; 2] }
#[repr(C)]
struct ExpectLayout {
    saved_proto: ExpectProto,
    tuple: ExpectTuple,
    dir: u32,
    expectfn: Option<unsafe extern "C" fn(*mut nf_conntrack_expect)>,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
