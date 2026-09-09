// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (c) 2008-2009 Patrick McHardy <kaber@trash.net>
 * Copyright (c) 2013 Eric Leblond <eric@regit.org>
 *
 * Development of this code funded by Astaro AG (http://www.astaro.com/)
 */

// Translated from the Linux kernel implementation.  The declarations below
// are supplied by the corresponding kernel networking dependencies.

use core::ffi::{c_char, c_int, c_void};

extern "C" {
    static THIS_MODULE: *mut c_void;
    static nft_reject_policy: *const c_void;

    fn nft_expr_priv(expr: *const nft_expr) -> *mut nft_reject;
    fn nft_net(pkt: *const nft_pktinfo) -> *mut c_void;
    fn nft_sk(pkt: *const nft_pktinfo) -> *mut c_void;
    fn nft_hook(pkt: *const nft_pktinfo) -> c_int;
    fn nf_send_unreach6(net: *mut c_void, skb: *mut c_void, code: u8, hook: c_int);
    fn nf_send_reset6(net: *mut c_void, sk: *mut c_void, skb: *mut c_void, hook: c_int);
    fn nft_reject_init(expr: *const nft_expr, tb: *const c_void) -> c_int;
    fn nft_reject_dump(skb: *mut c_void, expr: *const nft_expr);
    fn nft_reject_validate(expr: *const nft_expr, tb: *const c_void) -> c_int;
    fn nft_register_expr(ty: *mut nft_expr_type) -> c_int;
    fn nft_unregister_expr(ty: *mut nft_expr_type);
}

#[repr(C)]
struct nft_expr {
    _private: [u8; 0],
}

#[repr(C)]
struct nft_regs {
    verdict: nft_verdict,
}

#[repr(C)]
struct nft_verdict {
    code: c_int,
}

#[repr(C)]
struct nft_pktinfo {
    _private: [u8; 0],
}

#[repr(C)]
struct nft_reject {
    type_: c_int,
    icmp_code: u8,
}

type NftEval = unsafe extern "C" fn(*const nft_expr, *mut nft_regs, *const nft_pktinfo);
type NftInit = unsafe extern "C" fn(*const nft_expr, *const c_void) -> c_int;
type NftDump = unsafe extern "C" fn(*mut c_void, *const nft_expr);
type NftValidate = unsafe extern "C" fn(*const nft_expr, *const c_void) -> c_int;

#[repr(C)]
struct nft_expr_ops {
    type_: *mut nft_expr_type,
    size: usize,
    eval: Option<NftEval>,
    init: Option<NftInit>,
    dump: Option<NftDump>,
    validate: Option<NftValidate>,
}

#[repr(C)]
struct nft_expr_type {
    family: c_int,
    name: *const c_char,
    ops: *const nft_expr_ops,
    policy: *const c_void,
    maxattr: c_int,
    owner: *mut c_void,
}

const NFT_REJECT_ICMP_UNREACH: c_int = 0;
const NFT_REJECT_TCP_RST: c_int = 1;
const NF_DROP: c_int = 0;
const NFPROTO_IPV6: c_int = 10;
const NFTA_REJECT_MAX: c_int = 1;

const fn nft_expr_size(size: usize) -> usize {
    size
}

unsafe extern "C" fn nft_reject_ipv6_eval(
    expr: *const nft_expr,
    regs: *mut nft_regs,
    pkt: *const nft_pktinfo,
) {
    let priv_ = nft_expr_priv(expr);

    match (*priv_).type_ {
        NFT_REJECT_ICMP_UNREACH => {
            nf_send_unreach6(
                nft_net(pkt),
                (*pkt as *const nft_pktinfo as *mut c_void),
                (*priv_).icmp_code,
                nft_hook(pkt),
            );
        }
        NFT_REJECT_TCP_RST => {
            nf_send_reset6(
                nft_net(pkt),
                nft_sk(pkt),
                (*pkt as *const nft_pktinfo as *mut c_void),
                nft_hook(pkt),
            );
        }
        _ => {}
    }

    (*regs).verdict.code = NF_DROP;
}

static mut nft_reject_ipv6_type: nft_expr_type = nft_expr_type {
    family: NFPROTO_IPV6,
    name: b"reject\0".as_ptr() as *const c_char,
    ops: core::ptr::null(),
    policy: core::ptr::null(),
    maxattr: NFTA_REJECT_MAX,
    owner: core::ptr::null_mut(),
};

static nft_reject_ipv6_ops: nft_expr_ops = nft_expr_ops {
    type_: unsafe { &mut nft_reject_ipv6_type },
    size: nft_expr_size(core::mem::size_of::<nft_reject>()),
    eval: Some(nft_reject_ipv6_eval),
    init: Some(nft_reject_init),
    dump: Some(nft_reject_dump),
    validate: Some(nft_reject_validate),
};

unsafe fn nft_reject_ipv6_module_init() -> c_int {
    nft_reject_ipv6_type.ops = &nft_reject_ipv6_ops;
    nft_reject_ipv6_type.policy = nft_reject_policy;
    nft_reject_ipv6_type.owner = THIS_MODULE;
    nft_register_expr(&mut nft_reject_ipv6_type)
}

unsafe fn nft_reject_ipv6_module_exit() {
    nft_unregister_expr(&mut nft_reject_ipv6_type);
}

// module_init(nft_reject_ipv6_module_init);
// module_exit(nft_reject_ipv6_module_exit);
// MODULE_LICENSE("GPL");
// MODULE_AUTHOR("Patrick McHardy <kaber@trash.net>");
// MODULE_ALIAS_NFT_AF_EXPR(AF_INET6, "reject");
// MODULE_DESCRIPTION("IPv6 packet rejection for nftables");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
