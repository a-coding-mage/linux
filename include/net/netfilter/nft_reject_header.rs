/* SPDX-License-Identifier: GPL-2.0 */

// Dependencies supplied by the corresponding kernel translation units:
// linux/types.h, net/netlink.h, net/netfilter/nf_tables.h,
// uapi/linux/netfilter/nf_tables.h

#[repr(C)]
pub struct nft_reject {
    // C declaration: enum nft_reject_types type:8;
    pub type_: u8,
    pub icmp_code: u8,
}

extern "C" {
    pub static nft_reject_policy: [nla_policy; 0];

    pub fn nft_reject_validate(
        ctx: *const nft_ctx,
        expr: *const nft_expr,
    ) -> ::core::ffi::c_int;

    pub fn nft_reject_init(
        ctx: *const nft_ctx,
        expr: *const nft_expr,
        tb: *const *const nlattr,
    ) -> ::core::ffi::c_int;

    pub fn nft_reject_dump(
        skb: *mut sk_buff,
        expr: *const nft_expr,
        reset: bool,
    ) -> ::core::ffi::c_int;

    pub fn nft_reject_icmp_code(code: u8) -> ::core::ffi::c_int;
    pub fn nft_reject_icmpv6_code(code: u8) -> ::core::ffi::c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
