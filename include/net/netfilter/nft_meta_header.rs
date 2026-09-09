/* SPDX-License-Identifier: GPL-2.0 */

// Dependency declarations supplied by the surrounding nftables translation.
#[repr(C)]
pub struct nft_ctx {
    _private: [u8; 0],
}

#[repr(C)]
pub struct nft_expr {
    _private: [u8; 0],
}

#[repr(C)]
pub struct nlattr {
    _private: [u8; 0],
}

#[repr(C)]
pub struct sk_buff {
    _private: [u8; 0],
}

#[repr(C)]
pub struct nft_regs {
    _private: [u8; 0],
}

#[repr(C)]
pub struct nft_pktinfo {
    _private: [u8; 0],
}

#[repr(C)]
pub struct nft_inner_tun_ctx {
    _private: [u8; 0],
}

pub type nft_meta_keys = u8;
pub type nla_policy = u8;

#[repr(C)]
pub union nft_meta_dreg_sreg {
    pub dreg: u8,
    pub sreg: u8,
}

#[repr(C)]
pub struct nft_meta {
    // C bit-field `key:8`; the field occupies one byte.
    pub key: u8,
    pub len: u8,
    pub dreg_sreg: nft_meta_dreg_sreg,
}

extern "C" {
    pub static nft_meta_policy: [nla_policy; 0];

    pub fn nft_meta_get_init(
        ctx: *const nft_ctx,
        expr: *const nft_expr,
        tb: *const *const nlattr,
    ) -> i32;

    pub fn nft_meta_set_init(
        ctx: *const nft_ctx,
        expr: *const nft_expr,
        tb: *const *const nlattr,
    ) -> i32;

    pub fn nft_meta_get_dump(
        skb: *mut sk_buff,
        expr: *const nft_expr,
        reset: bool,
    ) -> i32;

    pub fn nft_meta_set_dump(
        skb: *mut sk_buff,
        expr: *const nft_expr,
        reset: bool,
    ) -> i32;

    pub fn nft_meta_get_eval(
        expr: *const nft_expr,
        regs: *mut nft_regs,
        pkt: *const nft_pktinfo,
    );

    pub fn nft_meta_set_eval(
        expr: *const nft_expr,
        regs: *mut nft_regs,
        pkt: *const nft_pktinfo,
    );

    pub fn nft_meta_set_destroy(ctx: *const nft_ctx, expr: *const nft_expr);

    pub fn nft_meta_get_validate(ctx: *const nft_ctx, expr: *const nft_expr) -> i32;
    pub fn nft_meta_set_validate(ctx: *const nft_ctx, expr: *const nft_expr) -> i32;

    pub fn nft_meta_inner_eval(
        expr: *const nft_expr,
        regs: *mut nft_regs,
        pkt: *const nft_pktinfo,
        tun_ctx: *mut nft_inner_tun_ctx,
    );
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
