// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (c) 2017 Pablo M. Bermudo Garay <pablombg@gmail.com>
 *
 * This code is based on net/netfilter/nft_fib_inet.c, written by
 * Florian Westphal <fw@strlen.de>.
 */

// Linux kernel dependencies:
// linux/kernel.h, linux/init.h, linux/module.h, linux/netlink.h,
// linux/netfilter.h, linux/netfilter/nf_tables.h,
// net/netfilter/nf_tables_core.h, net/netfilter/nf_tables.h,
// net/ipv6.h, and net/netfilter/nft_fib.h

extern "C" {
    fn ntohs(value: u16) -> u16;
    fn ipv6_mod_enabled() -> bool;
    fn nft_fib4_eval(expr: *const nft_expr, regs: *mut nft_regs, pkt: *const nft_pktinfo);
    fn nft_fib4_eval_type(expr: *const nft_expr, regs: *mut nft_regs, pkt: *const nft_pktinfo);
    fn nft_fib6_eval(expr: *const nft_expr, regs: *mut nft_regs, pkt: *const nft_pktinfo);
    fn nft_fib6_eval_type(expr: *const nft_expr, regs: *mut nft_regs, pkt: *const nft_pktinfo);
    fn nft_expr_priv(expr: *const nft_expr) -> *const nft_fib;
    fn nft_chain_validate_hooks(chain: *mut nft_chain, hooks: u32) -> i32;
    fn nft_fib_init(expr: *const nft_expr, tb: *mut nlattr, ctx: *const nft_ctx) -> i32;
    fn nft_fib_dump(expr: *const nft_expr, skb: *mut sk_buff, cb: *mut nft_dump_control);
    fn nft_register_expr(expr_type: *mut nft_expr_type) -> i32;
    fn nft_unregister_expr(expr_type: *mut nft_expr_type);
}

#[repr(C)]
pub struct nft_expr { _private: [u8; 0] }
#[repr(C)]
pub struct nft_regs { pub verdict: nft_verdict }
#[repr(C)]
pub struct nft_pktinfo { pub skb: *mut sk_buff }
#[repr(C)]
pub struct sk_buff { pub protocol: u16 }
#[repr(C)]
pub struct nft_fib { pub result: u32, pub flags: u32 }
#[repr(C)]
pub struct nft_ctx { pub chain: *mut nft_chain }
#[repr(C)]
pub struct nft_chain { _private: [u8; 0] }
#[repr(C)]
pub struct nlattr { _private: [u8; 0] }
#[repr(C)]
pub struct nft_dump_control { _private: [u8; 0] }
#[repr(C)]
pub struct nft_verdict { pub code: i32 }

#[repr(C)]
pub struct nft_expr_ops {
    pub r#type: *mut nft_expr_type,
    pub size: usize,
    pub eval: Option<unsafe extern "C" fn(*const nft_expr, *mut nft_regs, *const nft_pktinfo)>,
    pub init: Option<unsafe extern "C" fn(*const nft_expr, *mut nlattr, *const nft_ctx) -> i32>,
    pub dump: Option<unsafe extern "C" fn(*const nft_expr, *mut sk_buff, *mut nft_dump_control)>,
    pub validate: Option<unsafe extern "C" fn(*const nft_ctx, *const nft_expr) -> i32>,
}

#[repr(C)]
pub struct nft_expr_type {
    pub family: u32,
    pub name: *const core::ffi::c_char,
    pub ops: *const nft_expr_ops,
    pub policy: *const core::ffi::c_void,
    pub maxattr: u32,
    pub owner: *mut core::ffi::c_void,
}

const NFT_BREAK: i32 = 0;
const NFT_FIB_RESULT_OIF: u32 = 0;
const NFT_FIB_RESULT_OIFNAME: u32 = 1;
const NFT_FIB_RESULT_ADDRTYPE: u32 = 2;
const NFTA_FIB_F_IIF: u32 = 1;
const NFTA_FIB_F_OIF: u32 = 2;
const NF_NETDEV_INGRESS: u32 = 0;
const NF_NETDEV_EGRESS: u32 = 1;
const ETH_P_IP: u16 = 0x0800;
const ETH_P_IPV6: u16 = 0x86dd;
const NFPROTO_NETDEV: u32 = 5;
const NFTA_FIB_MAX: u32 = 4;
const EINVAL: i32 = 22;

unsafe extern "C" fn nft_fib_netdev_eval(
    expr: *const nft_expr,
    regs: *mut nft_regs,
    pkt: *const nft_pktinfo,
) {
    let priv_data = &*nft_expr_priv(expr);

    match ntohs((*(*pkt).skb).protocol) {
        ETH_P_IP => match priv_data.result {
            NFT_FIB_RESULT_OIF | NFT_FIB_RESULT_OIFNAME => nft_fib4_eval(expr, regs, pkt),
            NFT_FIB_RESULT_ADDRTYPE => nft_fib4_eval_type(expr, regs, pkt),
            _ => (*regs).verdict.code = NFT_BREAK,
        },
        ETH_P_IPV6 => {
            if !ipv6_mod_enabled() {
                (*regs).verdict.code = NFT_BREAK;
                return;
            }
            match priv_data.result {
                NFT_FIB_RESULT_OIF | NFT_FIB_RESULT_OIFNAME => nft_fib6_eval(expr, regs, pkt),
                NFT_FIB_RESULT_ADDRTYPE => nft_fib6_eval_type(expr, regs, pkt),
                _ => (*regs).verdict.code = NFT_BREAK,
            }
        }
        _ => (*regs).verdict.code = NFT_BREAK,
    }
}

unsafe extern "C" fn nft_fib_netdev_validate(
    ctx: *const nft_ctx,
    expr: *const nft_expr,
) -> i32 {
    let priv_data = &*nft_expr_priv(expr);
    let hooks: u32;

    match priv_data.result {
        NFT_FIB_RESULT_OIF | NFT_FIB_RESULT_OIFNAME => hooks = 1 << NF_NETDEV_INGRESS,
        NFT_FIB_RESULT_ADDRTYPE => {
            if priv_data.flags & NFTA_FIB_F_IIF != 0 {
                hooks = 1 << NF_NETDEV_INGRESS;
            } else if priv_data.flags & NFTA_FIB_F_OIF != 0 {
                hooks = 1 << NF_NETDEV_EGRESS;
            } else {
                hooks = (1 << NF_NETDEV_INGRESS) | (1 << NF_NETDEV_EGRESS);
            }
        }
        _ => return -EINVAL,
    }

    nft_chain_validate_hooks((*ctx).chain, hooks)
}

static mut nft_fib_netdev_type: nft_expr_type = nft_expr_type {
    family: NFPROTO_NETDEV,
    name: b"fib\0".as_ptr() as *const core::ffi::c_char,
    ops: core::ptr::addr_of!(nft_fib_netdev_ops),
    policy: core::ptr::null(),
    maxattr: NFTA_FIB_MAX,
    owner: core::ptr::null_mut(),
};

static nft_fib_netdev_ops: nft_expr_ops = nft_expr_ops {
    r#type: core::ptr::addr_of_mut!(nft_fib_netdev_type),
    size: core::mem::size_of::<nft_fib>(),
    eval: Some(nft_fib_netdev_eval),
    init: Some(nft_fib_init),
    dump: Some(nft_fib_dump),
    validate: Some(nft_fib_netdev_validate),
};

#[no_mangle]
pub unsafe extern "C" fn nft_fib_netdev_module_init() -> i32 {
    nft_register_expr(core::ptr::addr_of_mut!(nft_fib_netdev_type))
}

#[no_mangle]
pub unsafe extern "C" fn nft_fib_netdev_module_exit() {
    nft_unregister_expr(core::ptr::addr_of_mut!(nft_fib_netdev_type));
}

// module_init(nft_fib_netdev_module_init);
// module_exit(nft_fib_netdev_module_exit);
// MODULE_LICENSE("GPL");
// MODULE_AUTHOR("Pablo M. Bermudo Garay <pablombg@gmail.com>");
// MODULE_ALIAS_NFT_AF_EXPR(5, "fib");
// MODULE_DESCRIPTION("nftables netdev fib lookups support");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
