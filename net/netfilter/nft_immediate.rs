// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (c) 2008-2009 Patrick McHardy <kaber@trash.net>
 *
 * Development of this code funded by Astaro AG (http://www.astaro.com/)
 */

// Dependencies supplied by the surrounding kernel translation.

pub unsafe extern "C" fn nft_immediate_eval(
    expr: *const nft_expr,
    regs: *mut nft_regs,
    _pkt: *const nft_pktinfo,
) {
    let priv_ = nft_expr_priv(expr);

    nft_data_copy(
        unsafe { (*regs).data.as_mut_ptr().add((*priv_).dreg as usize) },
        &(*priv_).data,
        (*priv_).dlen,
    );
}

static mut NFT_IMMEDIATE_POLICY: [nla_policy; NFTA_IMMEDIATE_MAX as usize + 1] = [
    nla_policy { type_: NLA_BE32, max: NFT_REG32_MAX },
    nla_policy { type_: NLA_NESTED, max: 0 },
];

unsafe fn nft_reg_to_type(nla: *const nlattr) -> nft_data_types {
    let reg: u8 = ntohl(nla_get_be32(nla)) as u8;
    if reg as u32 == NFT_REG_VERDICT {
        NFT_DATA_VERDICT
    } else {
        NFT_DATA_VALUE
    }
}

unsafe extern "C" fn nft_immediate_init(
    ctx: *const nft_ctx,
    expr: *const nft_expr,
    tb: *const *const nlattr,
) -> i32 {
    let priv_ = nft_expr_priv_mut(expr);
    let mut desc = nft_data_desc {
        size: core::mem::size_of_val(&(*priv_).data),
        ..core::mem::zeroed()
    };
    let mut err: i32;

    if (*tb.add(NFTA_IMMEDIATE_DREG as usize)).is_null()
        || (*tb.add(NFTA_IMMEDIATE_DATA as usize)).is_null()
    {
        return -EINVAL;
    }

    desc.type_ = nft_reg_to_type(*tb.add(NFTA_IMMEDIATE_DREG as usize));
    err = nft_data_init(
        ctx,
        &mut (*priv_).data,
        &mut desc,
        *tb.add(NFTA_IMMEDIATE_DATA as usize),
    );
    if err < 0 {
        return err;
    }

    (*priv_).dlen = desc.len;

    err = nft_parse_register_store(
        ctx,
        *tb.add(NFTA_IMMEDIATE_DREG as usize),
        &mut (*priv_).dreg,
        &mut (*priv_).data,
        desc.type_,
        desc.len,
    );
    if err < 0 {
        nft_data_release(&mut (*priv_).data, desc.type_);
        return err;
    }

    if (*priv_).dreg == NFT_REG_VERDICT {
        let chain = (*priv_).data.verdict.chain;
        match (*priv_).data.verdict.code {
            NFT_JUMP | NFT_GOTO => {
                err = nf_tables_bind_chain(ctx, chain);
                if err < 0 {
                    nft_data_release(&mut (*priv_).data, desc.type_);
                    return err;
                }
            }
            _ => {}
        }
    }

    0
}

unsafe extern "C" fn nft_immediate_activate(ctx: *const nft_ctx, expr: *const nft_expr) {
    let priv_ = nft_expr_priv(expr);
    let data = &(*priv_).data;
    let mut chain_ctx: nft_ctx;
    let chain: *mut nft_chain;

    if (*priv_).dreg == NFT_REG_VERDICT {
        match data.verdict.code {
            NFT_JUMP | NFT_GOTO => {
                chain = data.verdict.chain;
                if !nft_chain_binding(chain) {
                    return nft_data_hold(&(*priv_).data, nft_dreg_to_type((*priv_).dreg));
                }
                chain_ctx = *ctx;
                chain_ctx.chain = chain;
                list_for_each_entry!(rule, &(*chain).rules, list, {
                    nft_rule_expr_activate(&mut chain_ctx, rule);
                });
                nft_clear((*ctx).net, chain);
            }
            _ => {}
        }
    }

    nft_data_hold(&(*priv_).data, nft_dreg_to_type((*priv_).dreg));
}

unsafe fn nft_immediate_chain_deactivate(
    ctx: *const nft_ctx,
    chain: *mut nft_chain,
    phase: nft_trans_phase,
) {
    let mut chain_ctx = *ctx;
    chain_ctx.chain = chain;
    list_for_each_entry!(rule, &(*chain).rules, list, {
        nft_rule_expr_deactivate(&mut chain_ctx, rule, phase);
    });
}

unsafe extern "C" fn nft_immediate_deactivate(
    ctx: *const nft_ctx,
    expr: *const nft_expr,
    phase: nft_trans_phase,
) {
    let priv_ = nft_expr_priv(expr);
    let data = &(*priv_).data;

    if (*priv_).dreg == NFT_REG_VERDICT {
        match data.verdict.code {
            NFT_JUMP | NFT_GOTO => {
                let chain = data.verdict.chain;
                if !nft_chain_binding(chain) { return; }
                match phase {
                    NFT_TRANS_PREPARE_ERROR => {
                        nf_tables_unbind_chain(ctx, chain);
                        nft_deactivate_next((*ctx).net, chain);
                    }
                    NFT_TRANS_PREPARE => {
                        nft_immediate_chain_deactivate(ctx, chain, phase);
                        nft_deactivate_next((*ctx).net, chain);
                    }
                    _ => {
                        nft_immediate_chain_deactivate(ctx, chain, phase);
                        nft_chain_del(chain);
                        (*chain).bound = false;
                        nft_use_dec(&mut (*(*chain).table).use_);
                    }
                }
            }
            _ => {}
        }
    }

    if phase == NFT_TRANS_COMMIT { return; }
    nft_data_release(&mut (*priv_).data, nft_dreg_to_type((*priv_).dreg));
}

unsafe extern "C" fn nft_immediate_destroy(ctx: *const nft_ctx, expr: *const nft_expr) {
    let priv_ = nft_expr_priv(expr);
    let data = &(*priv_).data;
    if (*priv_).dreg != NFT_REG_VERDICT { return; }
    match data.verdict.code {
        NFT_JUMP | NFT_GOTO => {
            let chain = data.verdict.chain;
            if !nft_chain_binding(chain) { return; }
            if (*chain).bound {
                nft_use_dec(&mut (*chain).use_);
                return;
            }
            let mut chain_ctx = *ctx;
            chain_ctx.chain = chain;
            nft_use_dec(&mut (*chain).use_);
            list_for_each_entry_safe!(rule, n, &mut (*chain).rules, list, {
                nft_use_dec(&mut (*chain).use_);
                list_del(&mut (*rule).list);
                nf_tables_rule_destroy(&mut chain_ctx, rule);
            });
            nf_tables_chain_destroy(chain);
        }
        _ => {}
    }
}

unsafe extern "C" fn nft_immediate_dump(
    skb: *mut sk_buff,
    expr: *const nft_expr,
    _reset: bool,
) -> i32 {
    let priv_ = nft_expr_priv(expr);
    if nft_dump_register(skb, NFTA_IMMEDIATE_DREG, (*priv_).dreg) != 0 { return -1; }
    nft_data_dump(skb, NFTA_IMMEDIATE_DATA, &(*priv_).data,
        nft_dreg_to_type((*priv_).dreg), (*priv_).dlen)
}

unsafe extern "C" fn nft_immediate_validate(ctx: *const nft_ctx, expr: *const nft_expr) -> i32 {
    let priv_ = nft_expr_priv(expr);
    if (*priv_).dreg != NFT_REG_VERDICT { return 0; }
    let pctx = ctx as *mut nft_ctx;
    match (*priv_).data.verdict.code {
        NFT_JUMP | NFT_GOTO => {
            (*pctx).level += 1;
            let err = nft_chain_validate(ctx, (*priv_).data.verdict.chain);
            if err < 0 { return err; }
            (*pctx).level -= 1;
        }
        _ => {}
    }
    0
}

unsafe extern "C" fn nft_immediate_offload_verdict(
    ctx: *mut nft_offload_ctx,
    flow: *mut nft_flow_rule,
    priv_: *const nft_immediate_expr,
) -> i32 {
    let entry = nft_flow_action_entry_next(ctx, flow);
    if entry.is_null() { return -E2BIG; }
    match (*priv_).data.verdict.code {
        NF_ACCEPT => (*entry).id = FLOW_ACTION_ACCEPT,
        NF_DROP => (*entry).id = FLOW_ACTION_DROP,
        _ => return -EOPNOTSUPP,
    }
    0
}

unsafe extern "C" fn nft_immediate_offload(
    ctx: *mut nft_offload_ctx,
    flow: *mut nft_flow_rule,
    expr: *const nft_expr,
) -> i32 {
    let priv_ = nft_expr_priv(expr);
    if (*priv_).dreg == NFT_REG_VERDICT { return nft_immediate_offload_verdict(ctx, flow, priv_); }
    core::ptr::copy_nonoverlapping(
        &(*priv_).data as *const nft_data as *const u8,
        (*ctx).regs[(*priv_).dreg as usize].data.as_mut_ptr() as *mut u8,
        core::mem::size_of::<nft_data>(),
    );
    0
}

unsafe extern "C" fn nft_immediate_offload_action(expr: *const nft_expr) -> bool {
    nft_expr_priv(expr).as_ref().map_or(false, |p| p.dreg == NFT_REG_VERDICT)
}

static mut NFT_IMM_OPS: nft_expr_ops = nft_expr_ops {
    type_: &nft_imm_type,
    size: NFT_EXPR_SIZE(core::mem::size_of::<nft_immediate_expr>()),
    eval: Some(nft_immediate_eval), init: Some(nft_immediate_init),
    activate: Some(nft_immediate_activate), deactivate: Some(nft_immediate_deactivate),
    destroy: Some(nft_immediate_destroy), dump: Some(nft_immediate_dump),
    validate: Some(nft_immediate_validate), offload: Some(nft_immediate_offload),
    offload_action: Some(nft_immediate_offload_action),
};

#[no_mangle]
pub static mut nft_imm_type: nft_expr_type = nft_expr_type {
    name: c"immediate".as_ptr(),
    ops: unsafe { &NFT_IMM_OPS },
    policy: unsafe { &NFT_IMMEDIATE_POLICY },
    maxattr: NFTA_IMMEDIATE_MAX,
    owner: THIS_MODULE,
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
