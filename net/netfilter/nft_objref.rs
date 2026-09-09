// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (c) 2012-2016 Pablo Neira Ayuso <pablo@netfilter.org>
 */

// C dependencies supplied by the surrounding kernel translation.

extern "C" {
    static mut nft_objref_type: nft_expr_type;

    fn nft_chain_validate_hooks(chain: *mut nft_chain, hooks: u32) -> c_int;
    fn nft_genmask_next(net: *mut net) -> u8;
    fn nft_obj_lookup(net: *mut net, table: *mut nft_table, name: *const nlattr,
                      objtype: u32, genmask: u8) -> *mut nft_object;
    fn nft_use_inc(use_: *mut refcount_t) -> bool;
    fn nft_use_dec(use_: *mut refcount_t);
    fn nft_use_inc_restore(use_: *mut refcount_t);
    fn nla_get_be32(attr: *const nlattr) -> u32;
    fn ntohl(value: u32) -> u32;
    fn htonl(value: u32) -> u32;
    fn nla_put_string(skb: *mut sk_buff, attrtype: u16, value: *const c_char) -> c_int;
    fn nla_put_be32(skb: *mut sk_buff, attrtype: u16, value: u32) -> c_int;
    fn nft_set_do_lookup(net: *mut net, set: *const nft_set, key: *const u32) -> *const nft_set_ext;
    fn nft_set_catchall_lookup(net: *mut net, set: *const nft_set) -> *const nft_set_ext;
    fn nft_set_ext_obj(ext: *const nft_set_ext) -> *mut *mut nft_object;
    fn nft_net(pkt: *const nft_pktinfo) -> *mut net;
    fn nft_set_lookup_global(net: *mut net, table: *mut nft_table, name: *const nlattr,
                             id: *const nlattr, genmask: u8) -> *mut nft_set;
    fn nft_parse_register_load(ctx: *const nft_ctx, attr: *const nlattr,
                               sreg: *mut u8, len: u32) -> c_int;
    fn nf_tables_bind_set(ctx: *const nft_ctx, set: *mut nft_set,
                          binding: *mut nft_set_binding) -> c_int;
    fn nft_dump_register(skb: *mut sk_buff, attrtype: u16, reg: u8) -> c_int;
    fn nf_tables_deactivate_set(ctx: *const nft_ctx, set: *mut nft_set,
                                binding: *mut nft_set_binding, phase: nft_trans_phase);
    fn nf_tables_activate_set(ctx: *const nft_ctx, set: *mut nft_set);
    fn nf_tables_destroy_set(ctx: *const nft_ctx, set: *mut nft_set);
}

use core::ffi::{c_char, c_int, c_void};

#[repr(C)] pub struct nft_expr { _private: [u8; 0] }
#[repr(C)] pub struct nft_regs { pub data: [u32; 20], pub verdict: nft_verdict }
#[repr(C)] pub struct nft_pktinfo { _private: [u8; 0] }
#[repr(C)] pub struct nft_ctx { pub net: *mut net, pub table: *mut nft_table, pub chain: *mut nft_chain, pub family: u8 }
#[repr(C)] pub struct nlattr { _private: [u8; 0] }
#[repr(C)] pub struct sk_buff { _private: [u8; 0] }
#[repr(C)] pub struct net { _private: [u8; 0] }
#[repr(C)] pub struct nft_chain { _private: [u8; 0] }
#[repr(C)] pub struct nft_table { _private: [u8; 0] }
#[repr(C)] pub struct nft_set_ext { _private: [u8; 0] }
#[repr(C)] pub struct refcount_t { _private: [u8; 0] }
#[repr(C)] pub struct nft_verdict { pub code: u32 }
#[repr(C)] pub struct nft_object_key { pub name: *const c_char }
#[repr(C)] pub struct nft_object_type { pub r#type: u32 }
#[repr(C)] pub struct nft_object_ops { pub r#type: *const nft_object_type, pub eval: Option<unsafe extern "C" fn(*mut nft_object, *mut nft_regs, *const nft_pktinfo)> }
#[repr(C)] pub struct nft_object { pub key: nft_object_key, pub ops: *const nft_object_ops, pub use_: refcount_t }
#[repr(C)] pub struct nft_set { pub flags: u32, pub klen: u32, pub name: *const c_char, pub objtype: u32 }
#[repr(C)] pub struct nft_set_binding { pub flags: u32 }
#[repr(C)] pub struct nft_expr_ops { pub r#type: *const nft_expr_type, pub size: usize, pub eval: Option<unsafe extern "C" fn(*const nft_expr, *mut nft_regs, *const nft_pktinfo)> }
#[repr(C)] pub struct nft_expr_type { pub name: *const c_char, pub select_ops: Option<unsafe extern "C" fn(*const nft_ctx, *const *const nlattr) -> *const nft_expr_ops>, pub policy: *const nla_policy, pub maxattr: u16, pub owner: *mut c_void }
#[repr(C)] pub struct nla_policy { pub r#type: u16, pub len: u16 }
#[repr(C)] pub enum nft_trans_phase { NFT_TRANS_PREPARE, NFT_TRANS_ABORT, NFT_TRANS_COMMIT }

const NFT_OBJECT_SYNPROXY: u32 = 1;
const NFPROTO_IPV4: u8 = 2;
const NFPROTO_IPV6: u8 = 10;
const NFPROTO_INET: u8 = 1;
const NF_INET_LOCAL_IN: u32 = 0;
const NF_INET_FORWARD: u32 = 2;
const NFT_OBJECT_MAXNAMELEN: u16 = 32;
const NFT_SET_MAXNAMELEN: u16 = 32;
const NFT_SET_OBJECT: u32 = 1 << 4;
const NFT_BREAK: u32 = 5;
const EOPNOTSUPP: c_int = 95;
const EINVAL: c_int = 22;
const ENOENT: c_int = 2;
const EMFILE: c_int = 24;

const NFTA_OBJREF_IMM_NAME: usize = 1;
const NFTA_OBJREF_IMM_TYPE: usize = 2;
const NFTA_OBJREF_SET_SREG: usize = 3;
const NFTA_OBJREF_SET_NAME: usize = 4;
const NFTA_OBJREF_SET_ID: usize = 5;
const NFTA_OBJREF_MAX: usize = 5;
const NFT_REG32_MAX: u8 = 20;

#[inline] unsafe fn nft_objref_priv(expr: *const nft_expr) -> *mut *mut nft_object { expr as *mut *mut nft_object }

pub unsafe extern "C" fn nft_objref_eval(expr: *const nft_expr, regs: *mut nft_regs, pkt: *const nft_pktinfo) {
    let obj = *nft_objref_priv(expr);
    ((*(*obj).ops).eval.unwrap())(obj, regs, pkt);
}

unsafe fn nft_objref_validate_obj_type(ctx: *const nft_ctx, objtype: u32) -> c_int {
    match objtype {
        NFT_OBJECT_SYNPROXY => {
            if (*ctx).family != NFPROTO_IPV4 && (*ctx).family != NFPROTO_IPV6 && (*ctx).family != NFPROTO_INET { return -EOPNOTSUPP; }
            nft_chain_validate_hooks((*ctx).chain, (1 << NF_INET_LOCAL_IN) | (1 << NF_INET_FORWARD))
        }
        _ => 0,
    }
}

unsafe fn nft_objref_validate(ctx: *const nft_ctx, expr: *const nft_expr) -> c_int {
    let obj = *nft_objref_priv(expr);
    nft_objref_validate_obj_type(ctx, (*(*obj).ops).r#type.as_ref().unwrap().r#type)
}

unsafe fn nft_objref_init(ctx: *const nft_ctx, expr: *const nft_expr, tb: *const *const nlattr) -> c_int {
    if (*tb.add(NFTA_OBJREF_IMM_NAME)).is_null() || (*tb.add(NFTA_OBJREF_IMM_TYPE)).is_null() { return -EINVAL; }
    let objtype = ntohl(nla_get_be32(*tb.add(NFTA_OBJREF_IMM_TYPE)));
    let obj = nft_obj_lookup((*ctx).net, (*ctx).table, *tb.add(NFTA_OBJREF_IMM_NAME), objtype, nft_genmask_next((*ctx).net));
    if obj.is_null() { return -ENOENT; }
    if !nft_use_inc(&mut (*obj).use_) { return -EMFILE; }
    *nft_objref_priv(expr) = obj;
    0
}

unsafe fn nft_objref_dump(skb: *mut sk_buff, expr: *const nft_expr, _reset: bool) -> c_int {
    let obj = *nft_objref_priv(expr);
    if nla_put_string(skb, NFTA_OBJREF_IMM_NAME as u16, (*obj).key.name) != 0 || nla_put_be32(skb, NFTA_OBJREF_IMM_TYPE as u16, htonl((*(*obj).ops).r#type.as_ref().unwrap().r#type)) != 0 { return -1; }
    0
}

unsafe fn nft_objref_deactivate(_ctx: *const nft_ctx, expr: *const nft_expr, phase: nft_trans_phase) { if let nft_trans_phase::NFT_TRANS_COMMIT = phase { return; } nft_use_dec(&mut (*(*nft_objref_priv(expr))).use_); }
unsafe fn nft_objref_activate(_ctx: *const nft_ctx, expr: *const nft_expr) { nft_use_inc_restore(&mut (*(*nft_objref_priv(expr))).use_); }

#[repr(C)] pub struct nft_objref_map { pub set: *mut nft_set, pub sreg: u8, pub binding: nft_set_binding }

pub unsafe extern "C" fn nft_objref_map_eval(expr: *const nft_expr, regs: *mut nft_regs, pkt: *const nft_pktinfo) {
    let priv_ = expr as *const nft_objref_map; let set = (*priv_).set; let net_ = nft_net(pkt);
    let mut ext = nft_set_do_lookup(net_, set, (*regs).data.as_ptr().add((*priv_).sreg as usize));
    if ext.is_null() { ext = nft_set_catchall_lookup(net_, set); if ext.is_null() { (*regs).verdict.code = NFT_BREAK; return; } }
    let obj = *nft_set_ext_obj(ext); ((*(*obj).ops).eval.unwrap())(obj, regs, pkt);
}

unsafe fn nft_objref_map_init(ctx: *const nft_ctx, expr: *const nft_expr, tb: *const *const nlattr) -> c_int {
    let priv_ = expr as *mut nft_objref_map; let set = nft_set_lookup_global((*ctx).net, (*ctx).table, *tb.add(NFTA_OBJREF_SET_NAME), *tb.add(NFTA_OBJREF_SET_ID), nft_genmask_next((*ctx).net));
    if set.is_null() { return -ENOENT; } if (*set).flags & NFT_SET_OBJECT == 0 { return -EINVAL; }
    let err = nft_parse_register_load(ctx, *tb.add(NFTA_OBJREF_SET_SREG), &mut (*priv_).sreg, (*set).klen); if err < 0 { return err; }
    (*priv_).binding.flags = (*set).flags & NFT_SET_OBJECT; let err = nf_tables_bind_set(ctx, set, &mut (*priv_).binding); if err < 0 { return err; }
    (*priv_).set = set; 0
}

unsafe fn nft_objref_map_dump(skb: *mut sk_buff, expr: *const nft_expr, _reset: bool) -> c_int { let p = expr as *const nft_objref_map; if nft_dump_register(skb, NFTA_OBJREF_SET_SREG as u16, (*p).sreg) != 0 || nla_put_string(skb, NFTA_OBJREF_SET_NAME as u16, (*(*p).set).name) != 0 { return -1; } 0 }
unsafe fn nft_objref_map_deactivate(ctx: *const nft_ctx, expr: *const nft_expr, phase: nft_trans_phase) { let p = expr as *mut nft_objref_map; nf_tables_deactivate_set(ctx, (*p).set, &mut (*p).binding, phase); }
unsafe fn nft_objref_map_activate(ctx: *const nft_ctx, expr: *const nft_expr) { nf_tables_activate_set(ctx, (*(expr as *const nft_objref_map)).set); }
unsafe fn nft_objref_map_destroy(ctx: *const nft_ctx, expr: *const nft_expr) { nf_tables_destroy_set(ctx, (*(expr as *const nft_objref_map)).set); }
unsafe fn nft_objref_map_validate(ctx: *const nft_ctx, expr: *const nft_expr) -> c_int { nft_objref_validate_obj_type(ctx, (*(*(expr as *const nft_objref_map)).set).objtype) }

unsafe fn nft_objref_select_ops(_ctx: *const nft_ctx, tb: *const *const nlattr) -> *const nft_expr_ops {
    if !(*tb.add(NFTA_OBJREF_SET_SREG)).is_null() && (!(*tb.add(NFTA_OBJREF_SET_NAME)).is_null() || !(*tb.add(NFTA_OBJREF_SET_ID)).is_null()) { &nft_objref_map_ops }
    else if !(*tb.add(NFTA_OBJREF_IMM_NAME)).is_null() && !(*tb.add(NFTA_OBJREF_IMM_TYPE)).is_null() { &nft_objref_ops }
    else { core::ptr::null() }
}

static nft_objref_ops: nft_expr_ops = nft_expr_ops { r#type: unsafe { &nft_objref_type }, size: core::mem::size_of::<*mut nft_object>(), eval: Some(nft_objref_eval) };
static nft_objref_map_ops: nft_expr_ops = nft_expr_ops { r#type: unsafe { &nft_objref_type }, size: core::mem::size_of::<nft_objref_map>(), eval: Some(nft_objref_map_eval) };
static nft_objref_policy: [nla_policy; NFTA_OBJREF_MAX + 1] = [nla_policy { r#type: 0, len: 0 }; NFTA_OBJREF_MAX + 1];

#[no_mangle] pub static mut nft_objref_type: nft_expr_type = nft_expr_type { name: b"objref\0".as_ptr() as *const c_char, select_ops: Some(nft_objref_select_ops), policy: nft_objref_policy.as_ptr(), maxattr: NFTA_OBJREF_MAX as u16, owner: core::ptr::null_mut() };

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
