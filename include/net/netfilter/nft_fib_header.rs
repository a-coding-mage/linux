/* SPDX-License-Identifier: GPL-2.0 */

/* Dependencies supplied by the surrounding kernel translation unit. */

#[repr(C)]
pub struct nft_fib {
    pub dreg: u8,
    pub result: u8,
    pub flags: u32,
}

unsafe extern "C" {
    pub static nft_fib_policy: [nla_policy; 0];
}

#[inline]
pub unsafe fn nft_fib_is_loopback(skb: *const sk_buff, input: *const net_device) -> bool {
    (*skb).pkt_type == PACKET_LOOPBACK || ((*input).flags & IFF_LOOPBACK) != 0
}

#[inline]
pub unsafe fn nft_fib_can_skip(pkt: *const nft_pktinfo) -> bool {
    let indev: *const net_device = nft_in(pkt);
    let sk: *const sock;

    match nft_hook(pkt) {
        NF_INET_PRE_ROUTING | NF_INET_INGRESS | NF_INET_LOCAL_IN => {}
        _ => return false,
    }

    sk = (*(*pkt).skb).sk;
    if !sk.is_null() && sk_fullsock(sk) {
        return (*sk).sk_rx_dst_ifindex == (*indev).ifindex;
    }

    nft_fib_is_loopback((*pkt).skb, indev)
}

#[inline]
pub unsafe fn nft_fib_l3mdev_master_ifindex_rcu(
    pkt: *const nft_pktinfo,
    iif: *const net_device,
) -> i32 {
    let dev: *const net_device = if !iif.is_null() { iif } else { (*(*pkt).skb).dev };

    l3mdev_master_ifindex_rcu(dev)
}

unsafe extern "C" {
    pub fn nft_fib_dump(skb: *mut sk_buff, expr: *const nft_expr, reset: bool) -> i32;
    pub fn nft_fib_init(
        ctx: *const nft_ctx,
        expr: *const nft_expr,
        tb: *const *const nlattr,
    ) -> i32;
    pub fn nft_fib_validate(ctx: *const nft_ctx, expr: *const nft_expr) -> i32;

    pub fn nft_fib4_eval_type(
        expr: *const nft_expr,
        regs: *mut nft_regs,
        pkt: *const nft_pktinfo,
    );
    pub fn nft_fib4_eval(
        expr: *const nft_expr,
        regs: *mut nft_regs,
        pkt: *const nft_pktinfo,
    );

    pub fn nft_fib6_eval_type(
        expr: *const nft_expr,
        regs: *mut nft_regs,
        pkt: *const nft_pktinfo,
    );
    pub fn nft_fib6_eval(
        expr: *const nft_expr,
        regs: *mut nft_regs,
        pkt: *const nft_pktinfo,
    );

    pub fn nft_fib_store_result(
        reg: *mut core::ffi::c_void,
        priv_: *const nft_fib,
        dev: *const net_device,
    );
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
