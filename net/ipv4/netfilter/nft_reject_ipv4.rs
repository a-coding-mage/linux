// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (c) 2008-2009 Patrick McHardy <kaber@trash.net>
 * Copyright (c) 2013 Eric Leblond <eric@regit.org>
 *
 * Development of this code funded by Astaro AG (http://www.astaro.com/)
 */

// Dependencies supplied by the surrounding kernel translation unit.

extern "C" {
    fn nft_reject_init(
        expr: *const nft_ctx,
        tb: *const *const nft_attr,
        expr_obj: *mut nft_expr,
    ) -> c_int;
    fn nft_reject_dump(skb: *mut sk_buff, expr: *const nft_expr);
    fn nft_reject_validate(ctx: *const nft_ctx, expr: *const nft_expr) -> c_int;
    fn nft_register_expr(expr_type: *mut nft_expr_type) -> c_int;
    fn nft_unregister_expr(expr_type: *mut nft_expr_type);
    fn nf_send_unreach(skb: *mut sk_buff, code: u8, hook: u8);
    fn nf_send_reset(net: *mut net, sk: *mut sock, skb: *mut sk_buff, hook: u8);
    fn nft_hook(pkt: *const nft_pktinfo) -> u8;
    fn nft_net(pkt: *const nft_pktinfo) -> *mut net;
    fn nft_sk(pkt: *const nft_pktinfo) -> *mut sock;
}

type c_int = i32;

const NFT_REJECT_ICMP_UNREACH: u32 = 0;
const NFT_REJECT_TCP_RST: u32 = 1;
const NF_DROP: i32 = 0;
const NFPROTO_IPV4: u8 = 2;
const NFTA_REJECT_MAX: u32 = 2;
const NFT_EXPR_SIZE_REJECT: usize = core::mem::size_of::<nft_reject>();

#[repr(C)]
pub struct nft_reject {
    pub type_: u32,
    pub icmp_code: u8,
}

#[repr(C)]
pub struct nft_verdict {
    pub code: i32,
}

#[repr(C)]
pub struct nft_regs {
    pub verdict: nft_verdict,
}

#[repr(C)]
pub struct nft_expr {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct nft_pktinfo {
    pub skb: *mut sk_buff,
}

#[repr(C)]
pub struct nft_ctx {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct nft_attr {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct sk_buff {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct net {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct sock {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct nft_expr_type {
    pub family: u8,
    pub name: *const u8,
    pub ops: *const nft_expr_ops,
    pub policy: *const core::ffi::c_void,
    pub maxattr: u32,
    pub owner: *mut core::ffi::c_void,
}

#[repr(C)]
pub struct nft_expr_ops {
    pub type_: *mut nft_expr_type,
    pub size: usize,
    pub eval: unsafe extern "C" fn(
        expr: *const nft_expr,
        regs: *mut nft_regs,
        pkt: *const nft_pktinfo,
    ),
    pub init: unsafe extern "C" fn(
        expr: *const nft_ctx,
        tb: *const *const nft_attr,
        expr_obj: *mut nft_expr,
    ) -> c_int,
    pub dump: unsafe extern "C" fn(skb: *mut sk_buff, expr: *const nft_expr),
    pub validate: unsafe extern "C" fn(ctx: *const nft_ctx, expr: *const nft_expr) -> c_int,
}

unsafe extern "C" fn nft_reject_ipv4_eval(
    expr: *const nft_expr,
    regs: *mut nft_regs,
    pkt: *const nft_pktinfo,
) {
    let priv_: *mut nft_reject = expr.add(1) as *mut nft_reject;

    match (*priv_).type_ {
        NFT_REJECT_ICMP_UNREACH => {
            nf_send_unreach(
                (*(pkt as *const nft_pktinfo)).skb,
                (*priv_).icmp_code,
                nft_hook(pkt),
            );
        }
        NFT_REJECT_TCP_RST => {
            nf_send_reset(
                nft_net(pkt),
                nft_sk(pkt),
                (*(pkt as *const nft_pktinfo)).skb,
                nft_hook(pkt),
            );
        }
        _ => {}
    }

    (*regs).verdict.code = NF_DROP;
}

static mut nft_reject_ipv4_type: nft_expr_type = nft_expr_type {
    family: NFPROTO_IPV4,
    name: b"reject\0".as_ptr(),
    ops: core::ptr::addr_of!(nft_reject_ipv4_ops),
    policy: core::ptr::null(),
    maxattr: NFTA_REJECT_MAX,
    owner: core::ptr::null_mut(),
};

static nft_reject_ipv4_ops: nft_expr_ops = nft_expr_ops {
    type_: core::ptr::addr_of_mut!(nft_reject_ipv4_type),
    size: NFT_EXPR_SIZE_REJECT,
    eval: nft_reject_ipv4_eval,
    init: nft_reject_init,
    dump: nft_reject_dump,
    validate: nft_reject_validate,
};

unsafe extern "C" fn nft_reject_ipv4_module_init() -> c_int {
    nft_register_expr(core::ptr::addr_of_mut!(nft_reject_ipv4_type))
}

unsafe extern "C" fn nft_reject_ipv4_module_exit() {
    nft_unregister_expr(core::ptr::addr_of_mut!(nft_reject_ipv4_type));
}

// module_init(nft_reject_ipv4_module_init);
// module_exit(nft_reject_ipv4_module_exit);
// MODULE_LICENSE("GPL");
// MODULE_AUTHOR("Patrick McHardy <kaber@trash.net>");
// MODULE_ALIAS_NFT_AF_EXPR(AF_INET, "reject");
// MODULE_DESCRIPTION("IPv4 packet rejection for nftables");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
