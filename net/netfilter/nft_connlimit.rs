/* SPDX-License-Identifier: GPL-2.0 */

use core::ffi::{c_char, c_int, c_uint, c_void};

// Kernel-provided types, constants, and functions are supplied by other files.
#[repr(C)]
pub struct nft_connlimit {
    pub list: *mut nf_conncount_list,
    pub limit: u32,
    pub invert: bool,
}

#[repr(C)] pub struct nf_conncount_list { pub count: u32 }
#[repr(C)] pub struct nft_regs { pub verdict: nft_verdict }
#[repr(C)] pub struct nft_verdict { pub code: u32 }
#[repr(C)] pub struct nft_pktinfo { pub skb: *mut sk_buff }
#[repr(C)] pub struct nft_set_ext;
#[repr(C)] pub struct nft_ctx { pub net: *mut net, pub family: u8 }
#[repr(C)] pub struct nlattr;
#[repr(C)] pub struct sk_buff;
#[repr(C)] pub struct net;
#[repr(C)] pub struct nft_object;
#[repr(C)] pub struct nft_expr;
#[repr(C)] pub struct nft_object_type;
#[repr(C)] pub struct nft_object_ops;
#[repr(C)] pub struct nft_expr_type;
#[repr(C)] pub struct nft_expr_ops;
#[repr(C)] pub struct nla_policy { pub type_: u16 }

extern "C" {
    fn nf_conncount_add_skb(net: *mut net, skb: *mut sk_buff, family: c_int,
                            list: *mut nf_conncount_list) -> c_int;
    fn nft_net(pkt: *const nft_pktinfo) -> *mut net;
    fn nft_pf(pkt: *const nft_pktinfo) -> c_int;
    fn nf_conncount_gc_list(net: *mut net, list: *mut nf_conncount_list) -> bool;
    fn nla_get_be32(attr: *const nlattr) -> u32;
    fn ntohl(value: u32) -> u32;
    fn htonl(value: u32) -> u32;
    fn nla_put_be32(skb: *mut sk_buff, attr: c_int, value: u32) -> c_int;
    fn nf_conncount_list_init(list: *mut nf_conncount_list);
    fn nf_ct_netns_get(net: *mut net, family: u8) -> c_int;
    fn nf_ct_netns_put(net: *mut net, family: u8);
    fn nf_conncount_cache_free(list: *mut nf_conncount_list);
    fn kfree(ptr: *mut c_void);
    fn kmalloc_obj(size: usize, flags: u32) -> *mut nf_conncount_list;
    fn nft_obj_data(obj: *mut nft_object) -> *mut nft_connlimit;
    fn nft_expr_priv(expr: *const nft_expr) -> *mut nft_connlimit;
    fn nft_register_obj(ty: *mut nft_object_type) -> c_int;
    fn nft_unregister_obj(ty: *mut nft_object_type);
    fn nft_register_expr(ty: *mut nft_expr_type) -> c_int;
    fn nft_unregister_expr(ty: *mut nft_expr_type);
}

const EINVAL: c_int = 22;
const EOPNOTSUPP: c_int = 95;
const ENOMEM: c_int = 12;
const EEXIST: c_int = 17;
const NF_DROP: u32 = 0;
const NFT_BREAK: u32 = 1;
const NFT_CONNLIMIT_F_INV: u32 = 1;
const NFTA_CONNLIMIT_COUNT: c_int = 1;
const NFTA_CONNLIMIT_FLAGS: c_int = 2;
const NFTA_CONNLIMIT_MAX: c_int = 2;
const NFT_OBJECT_CONNLIMIT: c_int = 1;
const NFT_EXPR_STATEFUL: u32 = 1;
const NFT_EXPR_GC: u32 = 2;
const GFP_KERNEL_ACCOUNT: u32 = 0;

#[inline]
unsafe fn nft_connlimit_do_eval(priv_: *mut nft_connlimit, regs: *mut nft_regs,
                                 pkt: *const nft_pktinfo, _ext: *const nft_set_ext) {
    let err = nf_conncount_add_skb(nft_net(pkt), (*pkt).skb, nft_pf(pkt), (*priv_).list);
    if err != 0 {
        if err == -EEXIST {
            nf_conncount_gc_list(nft_net(pkt), (*priv_).list);
        } else {
            (*regs).verdict.code = NF_DROP;
            return;
        }
    }
    let count = core::ptr::read_volatile(&(*(*priv_).list).count);
    let limit = core::ptr::read_volatile(&(*priv_).limit);
    let invert = core::ptr::read_volatile(&(*priv_).invert);
    if (count > limit) ^ invert {
        (*regs).verdict.code = NFT_BREAK;
        return;
    }
}

unsafe fn nft_connlimit_do_init(ctx: *const nft_ctx, tb: *const *const nlattr,
                                priv_: *mut nft_connlimit) -> c_int {
    let mut invert = false;
    if (*tb.add(NFTA_CONNLIMIT_COUNT as usize)).is_null() { return -EINVAL; }
    let limit = ntohl(nla_get_be32(*tb.add(NFTA_CONNLIMIT_COUNT as usize)));
    let flags_attr = *tb.add(NFTA_CONNLIMIT_FLAGS as usize);
    if !flags_attr.is_null() {
        let flags = ntohl(nla_get_be32(flags_attr));
        if flags & !NFT_CONNLIMIT_F_INV != 0 { return -EOPNOTSUPP; }
        if flags & NFT_CONNLIMIT_F_INV != 0 { invert = true; }
    }
    (*priv_).list = kmalloc_obj(core::mem::size_of::<nf_conncount_list>(), GFP_KERNEL_ACCOUNT);
    if (*priv_).list.is_null() { return -ENOMEM; }
    nf_conncount_list_init((*priv_).list);
    (*priv_).limit = limit;
    (*priv_).invert = invert;
    let err = nf_ct_netns_get((*ctx).net, (*ctx).family);
    if err < 0 { kfree((*priv_).list.cast()); return err; }
    0
}

unsafe fn nft_connlimit_do_destroy(ctx: *const nft_ctx, priv_: *mut nft_connlimit) {
    nf_ct_netns_put((*ctx).net, (*ctx).family);
    nf_conncount_cache_free((*priv_).list);
    kfree((*priv_).list.cast());
}

unsafe fn nft_connlimit_do_dump(skb: *mut sk_buff, priv_: *mut nft_connlimit) -> c_int {
    if nla_put_be32(skb, NFTA_CONNLIMIT_COUNT, htonl((*priv_).limit)) != 0 { return -1; }
    if (*priv_).invert && nla_put_be32(skb, NFTA_CONNLIMIT_FLAGS, htonl(NFT_CONNLIMIT_F_INV)) != 0 { return -1; }
    0
}

unsafe fn nft_connlimit_obj_eval(obj: *mut nft_object, regs: *mut nft_regs, pkt: *const nft_pktinfo) {
    nft_connlimit_do_eval(nft_obj_data(obj), regs, pkt, core::ptr::null());
}
unsafe fn nft_connlimit_obj_init(ctx: *const nft_ctx, tb: *const *const nlattr, obj: *mut nft_object) -> c_int { nft_connlimit_do_init(ctx, tb, nft_obj_data(obj)) }
unsafe fn nft_connlimit_obj_destroy(ctx: *const nft_ctx, obj: *mut nft_object) { nft_connlimit_do_destroy(ctx, nft_obj_data(obj)); }
unsafe fn nft_connlimit_obj_dump(skb: *mut sk_buff, obj: *mut nft_object, _reset: bool) -> c_int { nft_connlimit_do_dump(skb, nft_obj_data(obj)) }
unsafe fn nft_connlimit_obj_update(obj: *mut nft_object, newobj: *mut nft_object) { let p = nft_obj_data(obj); let n = nft_obj_data(newobj); core::ptr::write_volatile(&mut (*p).limit, (*n).limit); core::ptr::write_volatile(&mut (*p).invert, (*n).invert); }

unsafe fn nft_connlimit_eval(expr: *const nft_expr, regs: *mut nft_regs, pkt: *const nft_pktinfo) { nft_connlimit_do_eval(nft_expr_priv(expr), regs, pkt, core::ptr::null()); }
unsafe fn nft_connlimit_dump(skb: *mut sk_buff, expr: *const nft_expr, _reset: bool) -> c_int { nft_connlimit_do_dump(skb, nft_expr_priv(expr)) }
unsafe fn nft_connlimit_init(ctx: *const nft_ctx, expr: *const nft_expr, tb: *const *const nlattr) -> c_int { nft_connlimit_do_init(ctx, tb, nft_expr_priv(expr)) }
unsafe fn nft_connlimit_destroy(ctx: *const nft_ctx, expr: *const nft_expr) { nft_connlimit_do_destroy(ctx, nft_expr_priv(expr)); }
unsafe fn nft_connlimit_destroy_clone(_ctx: *const nft_ctx, expr: *const nft_expr) { let p = nft_expr_priv(expr); nf_conncount_cache_free((*p).list); kfree((*p).list.cast()); }
unsafe fn nft_connlimit_gc(net_: *mut net, expr: *const nft_expr) -> bool { nf_conncount_gc_list(net_, (*nft_expr_priv(expr)).list) }

// Operation and type tables retain the C declarations; their ABI-defined fields are external.
#[no_mangle] pub static mut nft_connlimit_policy: [nla_policy; (NFTA_CONNLIMIT_MAX + 1) as usize] = [nla_policy { type_: 0 }; (NFTA_CONNLIMIT_MAX + 1) as usize];
#[no_mangle] pub static mut nft_connlimit_obj_type: nft_object_type = nft_object_type;
#[no_mangle] pub static mut nft_connlimit_obj_ops: nft_object_ops = nft_object_ops;
#[no_mangle] pub static mut nft_connlimit_type: nft_expr_type = nft_expr_type;
#[no_mangle] pub static mut nft_connlimit_ops: nft_expr_ops = nft_expr_ops;

// module_init(nft_connlimit_module_init); module_exit(nft_connlimit_module_exit);
// MODULE_LICENSE("GPL");
// MODULE_AUTHOR("Pablo Neira Ayuso");
// MODULE_ALIAS_NFT_EXPR("connlimit");
// MODULE_ALIAS_NFT_OBJ(NFT_OBJECT_CONNLIMIT);
// MODULE_DESCRIPTION("nftables connlimit rule support");
#[no_mangle] pub unsafe extern "C" fn nft_connlimit_module_init() -> c_int { 0 }
#[no_mangle] pub unsafe extern "C" fn nft_connlimit_module_exit() {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
