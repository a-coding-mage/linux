// SPDX-License-Identifier: GPL-2.0-only
/*
 *
 * Generic part shared by ipv4 and ipv6 backends.
 */

// Dependencies supplied by the surrounding kernel translation unit.

const NFTA_FIB_F_ALL: u32 = NFTA_FIB_F_SADDR
    | NFTA_FIB_F_DADDR
    | NFTA_FIB_F_MARK
    | NFTA_FIB_F_IIF
    | NFTA_FIB_F_OIF
    | NFTA_FIB_F_PRESENT;

#[no_mangle]
pub static mut nft_fib_policy: [nla_policy; (NFTA_FIB_MAX + 1) as usize] = [
    nla_policy::default_const();
    (NFTA_FIB_MAX + 1) as usize
];

#[no_mangle]
pub unsafe extern "C" fn nft_fib_validate(
    ctx: *const nft_ctx,
    expr: *const nft_expr,
) -> c_int {
    let priv_: *const nft_fib = nft_expr_priv(expr);
    let hooks: c_uint;

    match (*ctx).family {
        NFPROTO_IPV4 | NFPROTO_IPV6 | NFPROTO_INET => {}
        _ => return -EOPNOTSUPP,
    }

    match (*priv_).result {
        NFT_FIB_RESULT_OIF | NFT_FIB_RESULT_OIFNAME => {
            hooks = (1 << NF_INET_PRE_ROUTING)
                | (1 << NF_INET_LOCAL_IN)
                | (1 << NF_INET_FORWARD);
        }
        NFT_FIB_RESULT_ADDRTYPE => {
            if (*priv_).flags & NFTA_FIB_F_IIF != 0 {
                hooks = (1 << NF_INET_PRE_ROUTING)
                    | (1 << NF_INET_LOCAL_IN)
                    | (1 << NF_INET_FORWARD);
            } else if (*priv_).flags & NFTA_FIB_F_OIF != 0 {
                hooks = (1 << NF_INET_LOCAL_OUT)
                    | (1 << NF_INET_POST_ROUTING)
                    | (1 << NF_INET_FORWARD);
            } else {
                hooks = (1 << NF_INET_LOCAL_IN)
                    | (1 << NF_INET_LOCAL_OUT)
                    | (1 << NF_INET_FORWARD)
                    | (1 << NF_INET_PRE_ROUTING)
                    | (1 << NF_INET_POST_ROUTING);
            }
        }
        _ => return -EINVAL,
    }

    nft_chain_validate_hooks((*ctx).chain, hooks)
}

#[no_mangle]
pub unsafe extern "C" fn nft_fib_init(
    ctx: *const nft_ctx,
    expr: *const nft_expr,
    tb: *const *const nlattr,
) -> c_int {
    let priv_: *mut nft_fib = nft_expr_priv(expr);
    let len: c_uint;
    let err: c_int;

    if (*tb.add(NFTA_FIB_DREG as usize)).is_null()
        || (*tb.add(NFTA_FIB_RESULT as usize)).is_null()
        || (*tb.add(NFTA_FIB_FLAGS as usize)).is_null()
    {
        return -EINVAL;
    }

    (*priv_).flags = ntohl(nla_get_be32(*tb.add(NFTA_FIB_FLAGS as usize)));
    if (*priv_).flags == 0 {
        return -EINVAL;
    }
    if (*priv_).flags & (NFTA_FIB_F_SADDR | NFTA_FIB_F_DADDR)
        == (NFTA_FIB_F_SADDR | NFTA_FIB_F_DADDR)
    {
        return -EINVAL;
    }
    if (*priv_).flags & (NFTA_FIB_F_IIF | NFTA_FIB_F_OIF)
        == (NFTA_FIB_F_IIF | NFTA_FIB_F_OIF)
    {
        return -EINVAL;
    }
    if (*priv_).flags & (NFTA_FIB_F_SADDR | NFTA_FIB_F_DADDR) == 0 {
        return -EINVAL;
    }

    (*priv_).result = ntohl(nla_get_be32(*tb.add(NFTA_FIB_RESULT as usize)));
    match (*priv_).result {
        NFT_FIB_RESULT_OIF => {
            if (*priv_).flags & NFTA_FIB_F_OIF != 0 { return -EINVAL; }
            len = core::mem::size_of::<c_int>() as c_uint;
        }
        NFT_FIB_RESULT_OIFNAME => {
            if (*priv_).flags & NFTA_FIB_F_OIF != 0 { return -EINVAL; }
            len = IFNAMSIZ;
        }
        NFT_FIB_RESULT_ADDRTYPE => len = core::mem::size_of::<u32>() as c_uint,
        _ => return -EINVAL,
    }
    let len = if (*priv_).flags & NFTA_FIB_F_PRESENT != 0 {
        if (*priv_).result != NFT_FIB_RESULT_OIF { return -EINVAL; }
        core::mem::size_of::<u8>() as c_uint
    } else { len };

    err = nft_parse_register_store(ctx, *tb.add(NFTA_FIB_DREG as usize), &mut (*priv_).dreg,
        core::ptr::null_mut(), NFT_DATA_VALUE, len);
    if err < 0 { return err; }
    0
}

#[no_mangle]
pub unsafe extern "C" fn nft_fib_dump(
    skb: *mut sk_buff,
    expr: *const nft_expr,
    _reset: bool,
) -> c_int {
    let priv_: *const nft_fib = nft_expr_priv(expr);
    if nft_dump_register(skb, NFTA_FIB_DREG, (*priv_).dreg) != 0 { return -1; }
    if nla_put_be32(skb, NFTA_FIB_RESULT, htonl((*priv_).result)) != 0 { return -1; }
    if nla_put_be32(skb, NFTA_FIB_FLAGS, htonl((*priv_).flags)) != 0 { return -1; }
    0
}

#[no_mangle]
pub unsafe extern "C" fn nft_fib_store_result(
    reg: *mut c_void,
    priv_: *const nft_fib,
    dev: *const net_device,
) {
    let dreg = reg as *mut u32;
    let index: c_int;
    match (*priv_).result {
        NFT_FIB_RESULT_OIF => {
            index = if !dev.is_null() { (*dev).ifindex } else { 0 };
            if (*priv_).flags & NFTA_FIB_F_PRESENT != 0 {
                nft_reg_store8(dreg, (index != 0) as u8);
            } else { *dreg = index as u32; }
        }
        NFT_FIB_RESULT_OIFNAME => {
            if (*priv_).flags & NFTA_FIB_F_PRESENT != 0 {
                nft_reg_store8(dreg, (!dev.is_null()) as u8);
            } else {
                strscpy_pad(reg, if !dev.is_null() { (*dev).name } else { "" }, IFNAMSIZ);
            }
        }
        _ => { DEBUG_NET_WARN_ON_ONCE(1); *dreg = 0; }
    }
}

// MODULE_LICENSE("GPL");
// MODULE_DESCRIPTION("Query routing table from nftables");
// MODULE_AUTHOR("Florian Westphal <fw@strlen.de>");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
