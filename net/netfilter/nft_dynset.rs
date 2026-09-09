// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (c) 2015 Patrick McHardy <kaber@trash.net>
 */

// Linux kernel and netfilter dependencies are supplied externally.

#[repr(C)]
pub struct nft_dynset {
    pub set: *mut nft_set,
    pub tmpl: nft_set_ext_tmpl,
    pub op: nft_dynset_ops,
    pub sreg_key: u8,
    pub sreg_data: u8,
    pub invert: bool,
    pub expr: bool,
    pub override_exprs: bool,
    pub num_exprs: u8,
    pub timeout: u64,
    pub expr_array: [*mut nft_expr; NFT_SET_EXPR_MAX],
    pub binding: nft_set_binding,
}

unsafe fn nft_dynset_expr_setup(priv_: *const nft_dynset, ext: *const nft_set_ext) -> i32 {
    let elem_expr = nft_set_ext_expr(ext);
    let ctx = nft_ctx {
        net: read_pnet(&(*(*priv_).set).net),
        family: (*(*priv_).set).table.as_ref().unwrap().family,
    };
    let mut i = 0;

    while i < (*priv_).num_exprs as usize {
        let expr = nft_setelem_expr_at(elem_expr, (*elem_expr).size);
        if nft_expr_clone(expr, (*priv_).expr_array[i], GFP_ATOMIC) < 0 {
            nft_set_elem_expr_destroy(&ctx, elem_expr);
            return -1;
        }
        (*elem_expr).size += (*(*priv_).expr_array[i]).ops.as_ref().unwrap().size;
        i += 1;
    }
    0
}

pub unsafe fn nft_dynset_new(
    set: *mut nft_set,
    expr: *const nft_expr,
    regs: *mut nft_regs,
) -> *mut nft_elem_priv {
    let priv_ = nft_expr_priv(expr);
    if !atomic_add_unless(&mut (*set).nelems, 1, (*set).size) { return core::ptr::null_mut(); }

    let timeout = if (*priv_).timeout != 0 { (*priv_).timeout } else { READ_ONCE((*set).timeout) };
    let elem_priv = nft_set_elem_init(
        set, &(*priv_).tmpl, (*regs).data.as_ptr().add((*priv_).sreg_key as usize),
        core::ptr::null(), (*regs).data.as_ptr().add((*priv_).sreg_data as usize),
        timeout, 0, GFP_ATOMIC,
    );
    if IS_ERR(elem_priv) { if (*set).size != 0 { atomic_dec(&mut (*set).nelems); } return core::ptr::null_mut(); }

    let ext = nft_set_elem_ext(set, elem_priv);
    if (*priv_).num_exprs != 0 && nft_dynset_expr_setup(priv_, ext) < 0 {
        nft_set_elem_destroy(set, elem_priv, false);
        if (*set).size != 0 { atomic_dec(&mut (*set).nelems); }
        return core::ptr::null_mut();
    }
    elem_priv
}

pub unsafe fn nft_dynset_eval(expr: *const nft_expr, regs: *mut nft_regs, pkt: *const nft_pktinfo) {
    let priv_ = nft_expr_priv(expr);
    let set = (*priv_).set;
    if (*priv_).op == NFT_DYNSET_OP_DELETE {
        ((*(*set).ops).delete.unwrap())(set, (*regs).data.as_ptr().add((*priv_).sreg_key as usize));
        return;
    }
    let ext = ((*(*set).ops).update.unwrap())(set, (*regs).data.as_ptr().add((*priv_).sreg_key as usize), expr, regs);
    if !ext.is_null() {
        if (*priv_).op == NFT_DYNSET_OP_UPDATE && nft_set_ext_exists(ext, NFT_SET_EXT_TIMEOUT) && READ_ONCE((*nft_set_ext_timeout(ext)).timeout) != 0 {
            let timeout = if (*priv_).timeout != 0 { (*priv_).timeout } else { READ_ONCE((*set).timeout) };
            WRITE_ONCE((*nft_set_ext_timeout(ext)).expiration, get_jiffies_64().wrapping_add(timeout));
        }
        nft_set_elem_update_expr(ext, regs, pkt);
        if (*priv_).invert { (*regs).verdict.code = NFT_BREAK; }
    } else if !(*priv_).invert { (*regs).verdict.code = NFT_BREAK; }
}

unsafe fn nft_dynset_ext_add_expr(priv_: *mut nft_dynset) {
    let mut size: u8 = 0;
    for i in 0..(*priv_).num_exprs as usize { size = size.wrapping_add((*(*(*priv_).expr_array[i]).ops).size as u8); }
    nft_set_ext_add_length(&mut (*priv_).tmpl, NFT_SET_EXT_EXPRESSIONS, core::mem::size_of::<nft_set_elem_expr>() + size as usize);
}

unsafe fn nft_dynset_expr_alloc(ctx: *const nft_ctx, set: *const nft_set, attr: *const nlattr, pos: i32) -> *mut nft_expr {
    let expr = nft_set_elem_expr_alloc(ctx, set, attr);
    if IS_ERR(expr) { return expr; }
    if !(*set).exprs[pos as usize].is_null() && (*(*set).exprs[pos as usize]).ops != (*expr).ops {
        nft_expr_destroy(ctx, expr); return ERR_PTR(-EOPNOTSUPP);
    }
    expr
}

static mut nft_dynset_policy: [nla_policy; NFTA_DYNSET_MAX + 1] = [nla_policy { type_: 0 }; NFTA_DYNSET_MAX + 1];

unsafe fn nft_dynset_init(ctx: *const nft_ctx, expr: *const nft_expr, tb: *const *const nlattr) -> i32 {
    let priv_ = nft_expr_priv(expr);
    let set = nft_set_lookup_global((*ctx).net, (*ctx).table, *tb.add(NFTA_DYNSET_SET_NAME), *tb.add(NFTA_DYNSET_SET_ID), nft_genmask_next((*ctx).net));
    if IS_ERR(set) { return PTR_ERR(set); }
    if (*set).flags & NFT_SET_OBJECT != 0 || (*(*set).ops).update.is_none() { return -EOPNOTSUPP; }
    if (*set).flags & NFT_SET_CONSTANT != 0 { return -EBUSY; }
    (*priv_).set = set;
    0
}

unsafe fn nft_dynset_deactivate(_ctx: *const nft_ctx, _expr: *const nft_expr, _phase: nft_trans_phase) {}
unsafe fn nft_dynset_activate(_ctx: *const nft_ctx, _expr: *const nft_expr) {}
unsafe fn nft_dynset_destroy(_ctx: *const nft_ctx, _expr: *const nft_expr) {}
unsafe fn nft_dynset_dump(_skb: *mut sk_buff, _expr: *const nft_expr, _reset: bool) -> i32 { 0 }

#[repr(C)]
pub struct nft_expr_type {
    pub name: *const u8,
    pub ops: *const nft_expr_ops,
    pub policy: *const nla_policy,
    pub maxattr: u32,
    pub owner: *mut module,
}

static nft_dynset_ops: nft_expr_ops = nft_expr_ops {
    type_: core::ptr::null(), size: 0, eval: nft_dynset_eval, init: nft_dynset_init,
    destroy: nft_dynset_destroy, activate: nft_dynset_activate,
    deactivate: nft_dynset_deactivate, dump: nft_dynset_dump,
};

pub static mut nft_dynset_type: nft_expr_type = nft_expr_type {
    name: b"dynset\0".as_ptr(), ops: &nft_dynset_ops, policy: unsafe { nft_dynset_policy.as_ptr() },
    maxattr: NFTA_DYNSET_MAX as u32, owner: THIS_MODULE,
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
