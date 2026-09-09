// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (c) 2008-2009 Patrick McHardy <kaber@trash.net>
 * Copyright (c) 2012-2014 Pablo Neira Ayuso <pablo@netfilter.org>
 *
 * Development of this code funded by Astaro AG (http://www.astaro.com/)
 */

// Linux kernel dependencies supplied by the surrounding translation unit.

static NFT_LOG_NULL_PREFIX: &[u8] = b"\0";

#[repr(C)]
struct NftLog {
    loginfo: nf_loginfo,
    prefix: *mut i8,
}

unsafe fn nft_log_eval_audit(pkt: *const nft_pktinfo) {
    let skb = (*pkt).skb;
    let mut ab: *mut audit_buffer;

    if !audit_enabled {
        return;
    }

    ab = audit_log_start(core::ptr::null_mut(), GFP_ATOMIC, AUDIT_NETFILTER_PKT);
    if ab.is_null() {
        return;
    }

    audit_log_format(ab, b"mark=%#x\0".as_ptr() as *const i8, (*skb).mark);
    audit_log_nf_skb(ab, skb, nft_pf(pkt));
    audit_log_end(ab);
}

unsafe fn nft_log_eval(
    expr: *const nft_expr,
    _regs: *mut nft_regs,
    pkt: *const nft_pktinfo,
) {
    let priv_: *const NftLog = nft_expr_priv(expr);

    if (*priv_).loginfo.type_ == NF_LOG_TYPE_LOG
        && (*priv_).loginfo.u.log.level == NFT_LOGLEVEL_AUDIT
    {
        nft_log_eval_audit(pkt);
        return;
    }

    nf_log_packet(
        nft_net(pkt),
        nft_pf(pkt),
        nft_hook(pkt),
        (*pkt).skb,
        nft_in(pkt),
        nft_out(pkt),
        &(*priv_).loginfo,
        b"%s\0".as_ptr() as *const i8,
        (*priv_).prefix,
    );
}

static NFT_LOG_POLICY: [nla_policy; NFTA_LOG_MAX as usize + 1] = [
    [NFTA_LOG_GROUP as usize] = nla_policy { type_: NLA_U16 },
    [NFTA_LOG_PREFIX as usize] = nla_policy { type_: NLA_STRING, len: NF_LOG_PREFIXLEN - 1 },
    [NFTA_LOG_SNAPLEN as usize] = nla_policy { type_: NLA_U32 },
    [NFTA_LOG_QTHRESHOLD as usize] = nla_policy { type_: NLA_U16 },
    [NFTA_LOG_LEVEL as usize] = nla_policy { type_: NLA_U32 },
    [NFTA_LOG_FLAGS as usize] = NLA_POLICY_MASK(NLA_BE32, NF_LOG_MASK),
];

unsafe fn nft_log_modprobe(net: *mut net, t: nf_log_type) -> i32 {
    match t {
        NF_LOG_TYPE_LOG => nft_request_module(net, b"%s\0".as_ptr() as *const i8, b"nf_log_syslog\0".as_ptr() as *const i8),
        NF_LOG_TYPE_ULOG => nft_request_module(net, b"%s\0".as_ptr() as *const i8, b"nfnetlink_log\0".as_ptr() as *const i8),
        NF_LOG_TYPE_MAX => {}
    }
    -ENOENT
}

unsafe fn nft_log_init(
    ctx: *const nft_ctx,
    expr: *const nft_expr,
    tb: *const *const nlattr,
) -> i32 {
    let priv_: *mut NftLog = nft_expr_priv(expr);
    let li: *mut nf_loginfo = &mut (*priv_).loginfo;
    let mut nla: *const nlattr;
    let mut err: i32;

    (*li).type_ = NF_LOG_TYPE_LOG;
    if !(*tb.add(NFTA_LOG_LEVEL as usize)).is_null() && !(*tb.add(NFTA_LOG_GROUP as usize)).is_null() { return -EINVAL; }
    if !(*tb.add(NFTA_LOG_GROUP as usize)).is_null() {
        (*li).type_ = NF_LOG_TYPE_ULOG;
        if !(*tb.add(NFTA_LOG_FLAGS as usize)).is_null() { return -EINVAL; }
    }

    nla = *tb.add(NFTA_LOG_PREFIX as usize);
    if !nla.is_null() {
        (*priv_).prefix = kmalloc(nla_len(nla) + 1, GFP_KERNEL_ACCOUNT) as *mut i8;
        if (*priv_).prefix.is_null() { return -ENOMEM; }
        nla_strscpy((*priv_).prefix, nla, nla_len(nla) + 1);
    } else {
        (*priv_).prefix = NFT_LOG_NULL_PREFIX.as_ptr() as *mut i8;
    }

    match (*li).type_ {
        NF_LOG_TYPE_LOG => {
            if !(*tb.add(NFTA_LOG_LEVEL as usize)).is_null() { (*li).u.log.level = ntohl(nla_get_be32(*tb.add(NFTA_LOG_LEVEL as usize))); }
            else { (*li).u.log.level = NFT_LOGLEVEL_WARNING; }
            if (*li).u.log.level > NFT_LOGLEVEL_AUDIT { err = -EINVAL; return nft_log_init_err(priv_, err); }
            if !(*tb.add(NFTA_LOG_FLAGS as usize)).is_null() {
                (*li).u.log.logflags = ntohl(nla_get_be32(*tb.add(NFTA_LOG_FLAGS as usize)));
                if (*li).u.log.logflags & !NF_LOG_MASK != 0 { err = -EINVAL; return nft_log_init_err(priv_, err); }
            }
        }
        NF_LOG_TYPE_ULOG => {
            (*li).u.ulog.group = ntohs(nla_get_be16(*tb.add(NFTA_LOG_GROUP as usize)));
            if !(*tb.add(NFTA_LOG_SNAPLEN as usize)).is_null() { (*li).u.ulog.flags |= NF_LOG_F_COPY_LEN; (*li).u.ulog.copy_len = ntohl(nla_get_be32(*tb.add(NFTA_LOG_SNAPLEN as usize))); }
            if !(*tb.add(NFTA_LOG_QTHRESHOLD as usize)).is_null() { (*li).u.ulog.qthreshold = ntohs(nla_get_be16(*tb.add(NFTA_LOG_QTHRESHOLD as usize))); }
        }
        _ => {}
    }
    if (*li).u.log.level == NFT_LOGLEVEL_AUDIT { return 0; }
    err = nf_logger_find_get((*ctx).family, (*li).type_);
    if err < 0 { if nft_log_modprobe((*ctx).net, (*li).type_) == -EAGAIN { err = -EAGAIN; } return nft_log_init_err(priv_, err); }
    0
}

unsafe fn nft_log_init_err(priv_: *mut NftLog, err: i32) -> i32 {
    if (*priv_).prefix != NFT_LOG_NULL_PREFIX.as_ptr() as *mut i8 { kfree((*priv_).prefix as *mut core::ffi::c_void); }
    err
}

unsafe fn nft_log_destroy(ctx: *const nft_ctx, expr: *const nft_expr) {
    let priv_: *mut NftLog = nft_expr_priv(expr);
    if (*priv_).prefix != NFT_LOG_NULL_PREFIX.as_ptr() as *mut i8 { kfree((*priv_).prefix as *mut core::ffi::c_void); }
    if (*priv_).loginfo.u.log.level == NFT_LOGLEVEL_AUDIT { return; }
    nf_logger_put((*ctx).family, (*priv_).loginfo.type_);
}

unsafe fn nft_log_dump(skb: *mut sk_buff, expr: *const nft_expr, _reset: bool) -> i32 {
    let priv_: *const NftLog = nft_expr_priv(expr);
    let li = &(*priv_).loginfo;
    if (*priv_).prefix != NFT_LOG_NULL_PREFIX.as_ptr() as *mut i8 && nla_put_string(skb, NFTA_LOG_PREFIX, (*priv_).prefix) != 0 { return -1; }
    match li.type_ {
        NF_LOG_TYPE_LOG => { if nla_put_be32(skb, NFTA_LOG_LEVEL, htonl(li.u.log.level)) != 0 { return -1; } if li.u.log.logflags != 0 && nla_put_be32(skb, NFTA_LOG_FLAGS, htonl(li.u.log.logflags)) != 0 { return -1; } }
        NF_LOG_TYPE_ULOG => { if nla_put_be16(skb, NFTA_LOG_GROUP, htons(li.u.ulog.group)) != 0 { return -1; } if li.u.ulog.flags & NF_LOG_F_COPY_LEN != 0 && nla_put_be32(skb, NFTA_LOG_SNAPLEN, htonl(li.u.ulog.copy_len)) != 0 { return -1; } if li.u.ulog.qthreshold != 0 && nla_put_be16(skb, NFTA_LOG_QTHRESHOLD, htons(li.u.ulog.qthreshold)) != 0 { return -1; } }
        _ => {}
    }
    0
}

// The remaining registration objects and module metadata map directly to the kernel's
// nft_expr_type/nft_expr_ops structures and supplied module-registration macros.
static mut NFT_LOG_TYPE: nft_expr_type = nft_expr_type { name: b"log\0".as_ptr() as *const i8, ops: &NFT_LOG_OPS, policy: NFT_LOG_POLICY.as_ptr(), maxattr: NFTA_LOG_MAX, owner: THIS_MODULE };
static NFT_LOG_OPS: nft_expr_ops = nft_expr_ops { type_: unsafe { &NFT_LOG_TYPE }, size: NFT_EXPR_SIZE(core::mem::size_of::<NftLog>()), eval: nft_log_eval, init: nft_log_init, destroy: nft_log_destroy, dump: nft_log_dump };

unsafe fn nft_log_module_init() -> i32 { nft_register_expr(&NFT_LOG_TYPE) }
unsafe fn nft_log_module_exit() { nft_unregister_expr(&NFT_LOG_TYPE); }

// module_init(nft_log_module_init);
// module_exit(nft_log_module_exit);
// MODULE_LICENSE("GPL");
// MODULE_AUTHOR("Patrick McHardy <kaber@trash.net>");
// MODULE_ALIAS_NFT_EXPR("log");
// MODULE_DESCRIPTION("Netfilter nf_tables log module");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
