// SPDX-License-Identifier: GPL-2.0-only
// Translated from C; compiled when CONFIG_NFT_CT is enabled.

#[cfg(CONFIG_NFT_CT)]
pub unsafe fn nft_ct_get_fast_eval(
    expr: *const nft_expr,
    regs: *mut nft_regs,
    pkt: *const nft_pktinfo,
) {
    let priv_: *const nft_ct = nft_expr_priv(expr);
    let dest: *mut u32 = (*regs).data.as_mut_ptr().add((*priv_).dreg as usize);
    let mut ctinfo: ip_conntrack_info = core::mem::zeroed();
    let ct: *const nf_conn;
    let mut state: u32;

    ct = nf_ct_get((*pkt).skb, &mut ctinfo);

    match (*priv_).key {
        NFT_CT_STATE => {
            if !ct.is_null() {
                state = NF_CT_STATE_BIT(ctinfo);
            } else if ctinfo == IP_CT_UNTRACKED {
                state = NF_CT_STATE_UNTRACKED_BIT;
            } else {
                state = NF_CT_STATE_INVALID_BIT;
            }
            *dest = state;
            return;
        }
        _ => {}
    }

    if ct.is_null() || nf_ct_is_template(ct) {
        (*regs).verdict.code = NFT_BREAK;
        return;
    }

    match (*priv_).key {
        NFT_CT_DIRECTION => {
            nft_reg_store8(dest, CTINFO2DIR(ctinfo));
            return;
        }
        NFT_CT_STATUS => {
            *dest = (*ct).status;
            return;
        }
        // #ifdef CONFIG_NF_CONNTRACK_MARK
        NFT_CT_MARK => {
            *dest = (*ct).mark;
            return;
        }
        // #endif
        // #ifdef CONFIG_NF_CONNTRACK_SECMARK
        NFT_CT_SECMARK => {
            *dest = (*ct).secmark;
            return;
        }
        // #endif
        _ => {
            DEBUG_NET_WARN_ON_ONCE(1);
            (*regs).verdict.code = NFT_BREAK;
        }
    }
}

// EXPORT_SYMBOL_GPL(nft_ct_get_fast_eval);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
