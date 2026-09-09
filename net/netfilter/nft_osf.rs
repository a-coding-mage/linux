// SPDX-License-Identifier: GPL-2.0-only
// Dependencies supplied by the surrounding kernel/Rust bindings.

#[repr(C)]
pub struct nft_osf {
    pub dreg: u8,
    pub ttl: u8,
    pub flags: u32,
}

// Equivalent of the C nla_policy table; constants and policy types are
// supplied by the netfilter bindings.
static NFT_OSF_POLICY: [nla_policy; NFTA_OSF_MAX as usize + 1] = [
    [NFTA_OSF_DREG as usize] = NLA_POLICY_MAX(NLA_BE32, NFT_REG32_MAX),
    [NFTA_OSF_TTL as usize] = nla_policy { type_: NLA_U8 },
    [NFTA_OSF_FLAGS as usize] = NLA_POLICY_MASK(NLA_BE32, NFT_OSF_F_VERSION),
];

unsafe fn nft_osf_eval(
    expr: *const nft_expr,
    regs: *mut nft_regs,
    pkt: *const nft_pktinfo,
) {
    let priv_: *mut nft_osf = nft_expr_priv(expr);
    let dest: *mut u32 = (*regs).data.as_mut_ptr().add((*priv_).dreg as usize);
    let skb: *mut sk_buff = (*pkt).skb;
    let mut os_match = [0i8; NFT_OSF_MAXGENRELEN as usize];
    let tcp: *const tcphdr;
    let mut data: nf_osf_data = core::mem::zeroed();
    let mut _tcph: tcphdr = core::mem::zeroed();

    if nft_pf(pkt) != NFPROTO_IPV4 {
        (*regs).verdict.code = NFT_BREAK;
        return;
    }

    if (*pkt).tprot != IPPROTO_TCP || (*pkt).fragoff != 0 {
        (*regs).verdict.code = NFT_BREAK;
        return;
    }

    tcp = skb_header_pointer(
        skb,
        ip_hdrlen(skb),
        core::mem::size_of::<tcphdr>(),
        &mut _tcph as *mut tcphdr as *mut core::ffi::c_void,
    );
    if tcp.is_null() {
        (*regs).verdict.code = NFT_BREAK;
        return;
    }
    if (*tcp).syn == 0 {
        (*regs).verdict.code = NFT_BREAK;
        return;
    }

    if !nf_osf_find(skb, nf_osf_fingers, (*priv_).ttl, &mut data) {
        strscpy_pad(
            dest as *mut i8,
            b"unknown\0".as_ptr() as *const i8,
            NFT_OSF_MAXGENRELEN,
        );
    } else {
        if (*priv_).flags & NFT_OSF_F_VERSION != 0 {
            snprintf(
                os_match.as_mut_ptr(),
                NFT_OSF_MAXGENRELEN,
                b"%s:%s\0".as_ptr() as *const i8,
                data.genre,
                data.version,
            );
        } else {
            strscpy(
                os_match.as_mut_ptr(),
                data.genre,
                NFT_OSF_MAXGENRELEN,
            );
        }
        strscpy_pad(dest as *mut i8, os_match.as_ptr(), NFT_OSF_MAXGENRELEN);
    }
}

unsafe fn nft_osf_init(
    ctx: *const nft_ctx,
    expr: *const nft_expr,
    tb: *const *const nlattr,
) -> i32 {
    let priv_: *mut nft_osf = nft_expr_priv(expr);
    let flags: u32;
    let ttl: u8;

    if (*tb.add(NFTA_OSF_DREG as usize)).is_null() {
        return -EINVAL;
    }

    if !(*tb.add(NFTA_OSF_TTL as usize)).is_null() {
        ttl = nla_get_u8(*tb.add(NFTA_OSF_TTL as usize));
        if ttl > 2 { return -EINVAL; }
        (*priv_).ttl = ttl;
    }

    if !(*tb.add(NFTA_OSF_FLAGS as usize)).is_null() {
        flags = ntohl(nla_get_be32(*tb.add(NFTA_OSF_FLAGS as usize)));
        if flags != NFT_OSF_F_VERSION { return -EINVAL; }
        (*priv_).flags = flags;
    }

    nft_parse_register_store(ctx, *tb.add(NFTA_OSF_DREG as usize), &mut (*priv_).dreg,
                             core::ptr::null_mut(), NFT_DATA_VALUE,
                             NFT_OSF_MAXGENRELEN)
}

unsafe fn nft_osf_dump(skb: *mut sk_buff, expr: *const nft_expr, _reset: bool) -> i32 {
    let priv_: *const nft_osf = nft_expr_priv(expr);
    if nla_put_u8(skb, NFTA_OSF_TTL, (*priv_).ttl) != 0 { return -1; }
    if nla_put_u32(skb, NFTA_OSF_FLAGS, ntohl((*priv_).flags)) != 0 { return -1; }
    if nft_dump_register(skb, NFTA_OSF_DREG, (*priv_).dreg) != 0 { return -1; }
    0
}

unsafe fn nft_osf_validate(ctx: *const nft_ctx, _expr: *const nft_expr) -> i32 {
    let hooks: u32;
    match (*ctx).family {
        NFPROTO_IPV4 | NFPROTO_INET => {
            hooks = (1 << NF_INET_LOCAL_IN) | (1 << NF_INET_PRE_ROUTING) | (1 << NF_INET_FORWARD);
        }
        _ => return -EOPNOTSUPP,
    }
    nft_chain_validate_hooks((*ctx).chain, hooks)
}

static mut NFT_OSF_TYPE: nft_expr_type = nft_expr_type::zeroed();

static NFT_OSF_OP: nft_expr_ops = nft_expr_ops {
    eval: Some(nft_osf_eval),
    size: NFT_EXPR_SIZE(core::mem::size_of::<nft_osf>()),
    init: Some(nft_osf_init),
    dump: Some(nft_osf_dump),
    type_: unsafe { &mut NFT_OSF_TYPE },
    validate: Some(nft_osf_validate),
};

// The C definition uses __read_mostly and a self-referential expression type.
static mut NFT_OSF_TYPE_INITIALIZED: nft_expr_type = nft_expr_type {
    ops: &NFT_OSF_OP,
    name: b"osf\0".as_ptr() as *const i8,
    owner: THIS_MODULE,
    policy: &NFT_OSF_POLICY,
    maxattr: NFTA_OSF_MAX,
};

unsafe fn nft_osf_module_init() -> i32 {
    nft_register_expr(&mut NFT_OSF_TYPE_INITIALIZED)
}

unsafe fn nft_osf_module_exit() {
    nft_unregister_expr(&mut NFT_OSF_TYPE_INITIALIZED);
}

// module_init(nft_osf_module_init);
// module_exit(nft_osf_module_exit);
// MODULE_LICENSE("GPL");
// MODULE_AUTHOR("Fernando Fernandez <ffmancera@riseup.net>");
// MODULE_ALIAS_NFT_EXPR("osf");
// MODULE_DESCRIPTION("nftables passive OS fingerprint support");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
