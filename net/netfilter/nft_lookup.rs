// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (c) 2009 Patrick McHardy <kaber@trash.net>
 *
 * Development of this code funded by Astaro AG (http://www.astaro.com/)
 */

#[repr(C)]
struct nft_lookup {
    set: *mut nft_set,
    sreg: u8,
    dreg: u8,
    dreg_set: bool,
    invert: bool,
    binding: nft_set_binding,
}

unsafe fn __nft_set_do_lookup(
    net: *const net,
    set: *const nft_set,
    key: *const u32,
) -> *const nft_set_ext {
    // CONFIG_MITIGATION_RETPOLINE controls the direct operation dispatch.
    #[cfg(CONFIG_MITIGATION_RETPOLINE)]
    {
        if (*set).ops == &nft_set_hash_fast_type.ops {
            return nft_hash_lookup_fast(net, set, key);
        }
        if (*set).ops == &nft_set_hash_type.ops {
            return nft_hash_lookup(net, set, key);
        }
        if (*set).ops == &nft_set_rhash_type.ops {
            return nft_rhash_lookup(net, set, key);
        }
        if (*set).ops == &nft_set_bitmap_type.ops {
            return nft_bitmap_lookup(net, set, key);
        }
        if (*set).ops == &nft_set_pipapo_type.ops {
            return nft_pipapo_lookup(net, set, key);
        }
        // CONFIG_X86_64 && !CONFIG_UML controls the AVX2 operation.
        #[cfg(all(CONFIG_X86_64, not(CONFIG_UML)))]
        if (*set).ops == &nft_set_pipapo_avx2_type.ops {
            return nft_pipapo_avx2_lookup(net, set, key);
        }
        if (*set).ops == &nft_set_rbtree_type.ops {
            return nft_rbtree_lookup(net, set, key);
        }
        DEBUG_NET_WARN_ON_ONCE(1);
    }
    ((*(*set).ops).lookup)(net, set, key)
}

unsafe fn nft_base_seq(net: *const net) -> unsigned_int {
    /* pairs with smp_store_release() in nf_tables_commit() */
    smp_load_acquire(&(*net).nft.base_seq)
}

unsafe fn nft_lookup_should_retry(net: *const net, seq: unsigned_int) -> bool {
    unlikely(seq != nft_base_seq(net))
}

#[no_mangle]
pub unsafe extern "C" fn nft_set_do_lookup(
    net: *const net,
    set: *const nft_set,
    key: *const u32,
) -> *const nft_set_ext {
    let mut ext: *const nft_set_ext;
    let mut base_seq: unsigned_int;

    loop {
        base_seq = nft_base_seq(net);
        ext = __nft_set_do_lookup(net, set, key);
        if !ext.is_null() {
            break;
        }
        /* No match?  There is a small chance that lookup was
         * performed in the old generation, but nf_tables_commit()
         * already unlinked a (matching) element.
         *
         * We need to repeat the lookup to make sure that we didn't
         * miss a matching element in the new generation.
         */
        if !nft_lookup_should_retry(net, base_seq) {
            break;
        }
    }
    ext
}

unsafe fn nft_lookup_eval(
    expr: *const nft_expr,
    regs: *mut nft_regs,
    pkt: *const nft_pktinfo,
) {
    let priv_: *const nft_lookup = nft_expr_priv(expr);
    let set = (*priv_).set;
    let net = nft_net(pkt);
    let mut ext = nft_set_do_lookup(net, set, &(*regs).data[(*priv_).sreg as usize]);
    if ext.is_null() {
        ext = nft_set_catchall_lookup(net, set);
    }

    let found = (!ext.is_null()) ^ (*priv_).invert;
    if !found {
        (*regs).verdict.code = NFT_BREAK;
        return;
    }

    if !ext.is_null() {
        if (*priv_).dreg_set {
            nft_data_copy(
                &mut (*regs).data[(*priv_).dreg as usize],
                nft_set_ext_data(ext),
                (*set).dlen,
            );
        }
        nft_set_elem_update_expr(ext, regs, pkt);
    }
}

// The nla_policy table mirrors the C designated initializers; its concrete
// policy helper representations are supplied by the kernel bindings.
static nft_lookup_policy: [nla_policy; NFTA_LOOKUP_MAX as usize + 1] = [
    nla_policy { type_: NLA_STRING, len: NFT_SET_MAXNAMELEN - 1 },
    nla_policy { type_: NLA_U32, len: 0 },
    nla_policy { type_: NLA_BE32, len: NFT_REG32_MAX },
    nla_policy { type_: NLA_BE32, len: NFT_REG32_MAX },
    nla_policy { type_: NLA_BE32, len: NFT_LOOKUP_F_INV },
];

unsafe fn nft_lookup_init(
    ctx: *const nft_ctx,
    expr: *const nft_expr,
    tb: *const *const nlattr,
) -> int {
    let priv_ = nft_expr_priv(expr);
    let genmask: u8 = nft_genmask_next((*ctx).net);
    let mut set: *mut nft_set;
    let mut flags: u32;
    let mut err: int;

    if (*tb.add(NFTA_LOOKUP_SET as usize)).is_null()
        || (*tb.add(NFTA_LOOKUP_SREG as usize)).is_null()
    {
        return -EINVAL;
    }

    set = nft_set_lookup_global(
        (*ctx).net,
        (*ctx).table,
        *tb.add(NFTA_LOOKUP_SET as usize),
        *tb.add(NFTA_LOOKUP_SET_ID as usize),
        genmask,
    );
    if IS_ERR(set) {
        return PTR_ERR(set);
    }

    err = nft_parse_register_load(
        ctx,
        *tb.add(NFTA_LOOKUP_SREG as usize),
        &mut (*priv_).sreg,
        (*set).klen,
    );
    if err < 0 {
        return err;
    }

    if !(*tb.add(NFTA_LOOKUP_FLAGS as usize)).is_null() {
        flags = ntohl(nla_get_be32(*tb.add(NFTA_LOOKUP_FLAGS as usize)));
        if flags & NFT_LOOKUP_F_INV != 0 {
            (*priv_).invert = true;
        }
    }

    if !(*tb.add(NFTA_LOOKUP_DREG as usize)).is_null() {
        if (*priv_).invert || (*set).flags & NFT_SET_MAP == 0 {
            return -EINVAL;
        }
        err = nft_parse_register_store(
            ctx,
            *tb.add(NFTA_LOOKUP_DREG as usize),
            &mut (*priv_).dreg,
            core::ptr::null_mut(),
            nft_set_datatype(set),
            (*set).dlen,
        );
        if err < 0 { return err; }
        (*priv_).dreg_set = true;
    } else if (*set).flags & NFT_SET_MAP != 0 {
        /* Map given, but user asks for lookup only (i.e. to
         * ignore value associated with key).
         *
         * This makes no sense for anonymous maps since they are
         * scoped to the rule, but for named sets this can be useful.
         */
        if (*set).flags & NFT_SET_ANONYMOUS != 0 { return -EINVAL; }
    }

    (*priv_).binding.flags = (*set).flags & NFT_SET_MAP;
    err = nf_tables_bind_set(ctx, set, &mut (*priv_).binding);
    if err < 0 { return err; }
    (*priv_).set = set;
    0
}

unsafe fn nft_lookup_deactivate(ctx: *const nft_ctx, expr: *const nft_expr, phase: nft_trans_phase) {
    let priv_ = nft_expr_priv(expr);
    nf_tables_deactivate_set(ctx, (*priv_).set, &mut (*priv_).binding, phase);
}

unsafe fn nft_lookup_activate(ctx: *const nft_ctx, expr: *const nft_expr) {
    let priv_ = nft_expr_priv(expr);
    nf_tables_activate_set(ctx, (*priv_).set);
}

unsafe fn nft_lookup_destroy(ctx: *const nft_ctx, expr: *const nft_expr) {
    let priv_ = nft_expr_priv(expr);
    nf_tables_destroy_set(ctx, (*priv_).set);
}

unsafe fn nft_lookup_dump(skb: *mut sk_buff, expr: *const nft_expr, _reset: bool) -> int {
    let priv_ = nft_expr_priv(expr);
    let flags: u32 = if (*priv_).invert { NFT_LOOKUP_F_INV } else { 0 };
    if nla_put_string(skb, NFTA_LOOKUP_SET, (*(*priv_).set).name) != 0 { return -1; }
    if nft_dump_register(skb, NFTA_LOOKUP_SREG, (*priv_).sreg) != 0 { return -1; }
    if (*priv_).dreg_set && nft_dump_register(skb, NFTA_LOOKUP_DREG, (*priv_).dreg) != 0 { return -1; }
    if nla_put_be32(skb, NFTA_LOOKUP_FLAGS, htonl(flags)) != 0 { return -1; }
    0
}

unsafe fn nft_lookup_validate(ctx: *const nft_ctx, expr: *const nft_expr) -> int {
    let priv_ = nft_expr_priv(expr);
    let mut iter = nft_set_iter {
        genmask: nft_genmask_next((*ctx).net),
        type_: NFT_ITER_UPDATE,
        fn_: nft_setelem_validate,
        ..core::mem::zeroed()
    };
    if (*priv_).set.flags & NFT_SET_MAP == 0 || (*priv_).set.dtype != NFT_DATA_VERDICT { return 0; }
    ((*(*(*priv_).set).ops).walk)(ctx, (*priv_).set, &mut iter);
    if iter.err == 0 { iter.err = nft_set_catchall_validate(ctx, (*priv_).set); }
    if iter.err < 0 { return iter.err; }
    0
}

static nft_lookup_ops: nft_expr_ops = nft_expr_ops {
    type_: &nft_lookup_type,
    size: NFT_EXPR_SIZE(core::mem::size_of::<nft_lookup>()),
    eval: nft_lookup_eval,
    init: nft_lookup_init,
    activate: nft_lookup_activate,
    deactivate: nft_lookup_deactivate,
    destroy: nft_lookup_destroy,
    dump: nft_lookup_dump,
    validate: nft_lookup_validate,
};

#[no_mangle]
static mut nft_lookup_type: nft_expr_type = nft_expr_type {
    name: "lookup",
    ops: &nft_lookup_ops,
    policy: &nft_lookup_policy,
    maxattr: NFTA_LOOKUP_MAX,
    owner: THIS_MODULE,
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
