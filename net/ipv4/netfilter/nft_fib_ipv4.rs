// SPDX-License-Identifier: GPL-2.0-only

// Translated from nft_fib_ipv4.c. Kernel dependencies are supplied externally.

use core::ptr;

extern "C" {
    fn ipv4_is_multicast(addr: __be32) -> bool;
    fn ipv4_is_lbcast(addr: __be32) -> bool;
    fn ipv4_is_zeronet(addr: __be32) -> bool;
    fn skb_network_offset(skb: *mut sk_buff) -> i32;
    fn nft_expr_priv(expr: *const nft_expr) -> *const nft_fib;
    fn nft_in(pkt: *const nft_pktinfo) -> *const net_device;
    fn nft_out(pkt: *const nft_pktinfo) -> *const net_device;
    fn skb_header_pointer(skb: *mut sk_buff, offset: i32, len: usize, buffer: *mut iphdr) -> *mut iphdr;
    fn inet_dev_addr_type(net: *mut net, dev: *const net_device, addr: __be32) -> u32;
    fn nft_net(pkt: *const nft_pktinfo) -> *mut net;
    fn inet_addr_type_dev_table(net: *mut net, dev: *const net_device, addr: __be32) -> u32;
    fn nft_fib_can_skip(pkt: *const nft_pktinfo) -> bool;
    fn nft_fib_store_result(dst: *mut u32, priv_: *const nft_fib, dev: *const net_device);
    fn nft_fib_l3mdev_master_ifindex_rcu(pkt: *const nft_pktinfo, oif: *const net_device) -> i32;
    fn sock_net_uid(net: *mut net, sk: *mut core::ffi::c_void) -> u32;
    fn ip4h_dscp(iph: *const iphdr) -> u8;
    fn nft_hook(pkt: *const nft_pktinfo) -> u32;
    fn fib_lookup(net: *mut net, fl4: *mut flowi4, res: *mut fib_result, flags: u32) -> i32;
    fn fib_info_nh_uses_dev(fi: *mut fib_info, dev: *const net_device) -> bool;
    fn fib_res_dev(res: *mut fib_result) -> *const net_device;
    fn nft_fib_init(): i32;
    fn nft_fib_dump(): i32;
    fn nft_fib_validate(): i32;
    fn nft_register_expr(ty: *mut nft_expr_type) -> i32;
    fn nft_unregister_expr(ty: *mut nft_expr_type);
    fn nla_get_be32(attr: *const nlattr) -> __be32;
    fn ntohl(value: __be32) -> u32;
    fn err_ptr(error: i32) -> *const nft_expr_ops;
}

type __be32 = u32;

#[repr(C)] pub struct nft_expr { _private: [u8; 0] }
#[repr(C)] pub struct nft_regs { pub data: [u32; 0], pub verdict: nft_verdict }
#[repr(C)] pub struct nft_pktinfo { pub skb: *mut sk_buff, pub tprot: u8 }
#[repr(C)] pub struct nft_fib { pub dreg: u32, pub flags: u32 }
#[repr(C)] pub struct sk_buff { pub dev: *const net_device, pub mark: u32 }
#[repr(C)] pub struct net_device { pub ifindex: i32 }
#[repr(C)] pub struct iphdr { pub saddr: __be32, pub daddr: __be32 }
#[repr(C)] pub struct fib_result { pub type_: u32, pub fi: *mut fib_info }
#[repr(C)] pub struct fib_info { _private: [u8; 0] }
#[repr(C)] pub struct flowi4 { pub flowi4_scope: u32, pub flowi4_iif: u32, pub flowi4_proto: u8, pub flowi4_uid: u32, pub flowi4_l3mdev: i32, pub flowi4_mark: u32, pub flowi4_dscp: u8, pub daddr: __be32, pub saddr: __be32 }
#[repr(C)] pub struct nft_ctx { _private: [u8; 0] }
#[repr(C)] pub struct nlattr { _private: [u8; 0] }
#[repr(C)] pub struct net { _private: [u8; 0] }
#[repr(C)] pub struct nft_verdict { pub code: u32 }
#[repr(C)] pub struct nft_expr_type { pub name: *const u8, pub select_ops: Option<unsafe extern "C" fn(*const nft_ctx, *const *const nlattr) -> *const nft_expr_ops>, pub policy: *const core::ffi::c_void, pub maxattr: u32, pub family: u8, pub owner: *mut core::ffi::c_void }
#[repr(C)] pub struct nft_expr_ops { pub type_: *mut nft_expr_type, pub size: usize, pub eval: Option<unsafe extern "C" fn(*const nft_expr, *mut nft_regs, *const nft_pktinfo)>, pub init: Option<unsafe extern "C" fn() -> i32>, pub dump: Option<unsafe extern "C" fn() -> i32>, pub validate: Option<unsafe extern "C" fn() -> i32> }

const NFTA_FIB_F_IIF: u32 = 1 << 0;
const NFTA_FIB_F_OIF: u32 = 1 << 1;
const NFTA_FIB_F_DADDR: u32 = 1 << 2;
const NFTA_FIB_F_MARK: u32 = 1 << 3;
const NFT_BREAK: u32 = 0x8000_0000;
const NF_INET_FORWARD: u32 = 2;
const LOOPBACK_IFINDEX: u32 = 1;
const RT_SCOPE_UNIVERSE: u32 = 0;
const FIB_LOOKUP_IGNORE_LINKSTATE: u32 = 1;
const RTN_UNICAST: u32 = 1;
const RTN_LOCAL: u32 = 2;
const NFTA_FIB_RESULT: usize = 1;
const NFT_FIB_RESULT_OIF: u32 = 0;
const NFT_FIB_RESULT_OIFNAME: u32 = 1;
const NFT_FIB_RESULT_ADDRTYPE: u32 = 2;

#[inline]
unsafe fn get_saddr(addr: __be32) -> __be32 {
    // don't try to find route from mcast/bcast/zeronet
    if ipv4_is_multicast(addr) || ipv4_is_lbcast(addr) || ipv4_is_zeronet(addr) { 0 } else { addr }
}

#[no_mangle]
pub unsafe extern "C" fn nft_fib4_eval_type(expr: *const nft_expr, regs: *mut nft_regs, pkt: *const nft_pktinfo) {
    let priv_ = nft_expr_priv(expr); let noff = skb_network_offset((*pkt).skb); let dst = (*regs).data.as_mut_ptr().add((*priv_).dreg as usize); let mut dev = ptr::null(); let mut _iph = iphdr { saddr: 0, daddr: 0 };
    if (*priv_).flags & NFTA_FIB_F_IIF != 0 { dev = nft_in(pkt); } else if (*priv_).flags & NFTA_FIB_F_OIF != 0 { dev = nft_out(pkt); }
    let iph = skb_header_pointer((*pkt).skb, noff, core::mem::size_of::<iphdr>(), &mut _iph);
    if iph.is_null() { (*regs).verdict.code = NFT_BREAK; return; }
    let addr = if (*priv_).flags & NFTA_FIB_F_DADDR != 0 { (*iph).daddr } else { (*iph).saddr };
    if (*priv_).flags & (NFTA_FIB_F_IIF | NFTA_FIB_F_OIF) != 0 { *dst = inet_dev_addr_type(nft_net(pkt), dev, addr); return; }
    *dst = inet_addr_type_dev_table(nft_net(pkt), (*(*pkt).skb).dev, addr);
}

#[no_mangle]
pub unsafe extern "C" fn nft_fib4_eval(expr: *const nft_expr, regs: *mut nft_regs, pkt: *const nft_pktinfo) {
    let priv_ = nft_expr_priv(expr); let noff = skb_network_offset((*pkt).skb); let dest = (*regs).data.as_mut_ptr().add((*priv_).dreg as usize); let mut _iph = iphdr { saddr: 0, daddr: 0 }; let mut res = fib_result { type_: 0, fi: ptr::null_mut() };
    let mut fl4 = flowi4 { flowi4_scope: RT_SCOPE_UNIVERSE, flowi4_iif: LOOPBACK_IFINDEX, flowi4_proto: (*pkt).tprot, flowi4_uid: sock_net_uid(nft_net(pkt), ptr::null_mut()), flowi4_l3mdev: 0, flowi4_mark: 0, flowi4_dscp: 0, daddr: 0, saddr: 0 };
    if nft_fib_can_skip(pkt) { nft_fib_store_result(dest, priv_, nft_in(pkt)); return; }
    let oif = if (*priv_).flags & NFTA_FIB_F_OIF != 0 { nft_out(pkt) } else if (*priv_).flags & NFTA_FIB_F_IIF != 0 { nft_in(pkt) } else { ptr::null() };
    fl4.flowi4_l3mdev = nft_fib_l3mdev_master_ifindex_rcu(pkt, oif);
    let iph = skb_header_pointer((*pkt).skb, noff, core::mem::size_of::<iphdr>(), &mut _iph); if iph.is_null() { (*regs).verdict.code = NFT_BREAK; return; }
    if ipv4_is_zeronet((*iph).saddr) && (ipv4_is_lbcast((*iph).daddr) || ipv4_is_local_multicast((*iph).daddr)) { nft_fib_store_result(dest, priv_, (*pkt).skb.as_ref().unwrap().dev); return; }
    if (*priv_).flags & NFTA_FIB_F_MARK != 0 { fl4.flowi4_mark = (*(*pkt).skb).mark; } fl4.flowi4_dscp = ip4h_dscp(iph);
    if (*priv_).flags & NFTA_FIB_F_DADDR != 0 { fl4.daddr = (*iph).daddr; fl4.saddr = get_saddr((*iph).saddr); } else { if nft_hook(pkt) == NF_INET_FORWARD && (*priv_).flags & NFTA_FIB_F_IIF != 0 { fl4.flowi4_iif = (*nft_out(pkt)).ifindex as u32; } fl4.daddr = (*iph).saddr; fl4.saddr = get_saddr((*iph).daddr); }
    nft_fib_store_result(dest, priv_, ptr::null()); if fib_lookup(nft_net(pkt), &mut fl4, &mut res, FIB_LOOKUP_IGNORE_LINKSTATE) != 0 { return; }
    match res.type_ { RTN_UNICAST => {}, RTN_LOCAL => return, _ => {} }
    let found = if oif.is_null() { fib_res_dev(&mut res) } else { if !fib_info_nh_uses_dev(res.fi, oif) { return; } oif }; nft_fib_store_result(dest, priv_, found);
}

extern "C" { fn ipv4_is_local_multicast(addr: __be32) -> bool; }

static mut nft_fib4_type: nft_expr_type = nft_expr_type { name: b"fib\0".as_ptr(), select_ops: Some(nft_fib4_select_ops), policy: ptr::null(), maxattr: 0, family: 2, owner: ptr::null_mut() };
static nft_fib4_type_ops: nft_expr_ops = nft_expr_ops { type_: unsafe { &mut nft_fib4_type }, size: core::mem::size_of::<nft_fib>(), eval: Some(nft_fib4_eval_type), init: Some(nft_fib_init), dump: Some(nft_fib_dump), validate: Some(nft_fib_validate) };
static nft_fib4_ops: nft_expr_ops = nft_expr_ops { type_: unsafe { &mut nft_fib4_type }, size: core::mem::size_of::<nft_fib>(), eval: Some(nft_fib4_eval), init: Some(nft_fib_init), dump: Some(nft_fib_dump), validate: Some(nft_fib_validate) };

unsafe extern "C" fn nft_fib4_select_ops(_ctx: *const nft_ctx, tb: *const *const nlattr) -> *const nft_expr_ops { let attr = *tb.add(NFTA_FIB_RESULT); if attr.is_null() { return err_ptr(-22); } match ntohl(nla_get_be32(attr)) { NFT_FIB_RESULT_OIF | NFT_FIB_RESULT_OIFNAME => &nft_fib4_ops, NFT_FIB_RESULT_ADDRTYPE => &nft_fib4_type_ops, _ => err_ptr(-95) } }

unsafe extern "C" fn nft_fib4_module_init() -> i32 { nft_register_expr(&mut nft_fib4_type) }
unsafe extern "C" fn nft_fib4_module_exit() { nft_unregister_expr(&mut nft_fib4_type); }

// module_init(nft_fib4_module_init); module_exit(nft_fib4_module_exit);
// MODULE_LICENSE("GPL"); MODULE_AUTHOR("Florian Westphal <fw@strlen.de>");
// MODULE_ALIAS_NFT_AF_EXPR(2, "fib"); MODULE_DESCRIPTION("nftables fib / ip route lookup support");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
