// SPDX-License-Identifier: GPL-2.0-only

// Translated from nft_fib_inet.c. Kernel declarations are supplied by the
// surrounding build.

use core::ffi::c_char;
use core::ffi::c_int;
use core::ffi::c_void;

#[repr(C)]
pub struct nft_expr {
    _private: [u8; 0],
}
#[repr(C)]
pub struct nft_regs {
    pub verdict: nft_verdict,
}
#[repr(C)]
pub struct nft_pktinfo {
    _private: [u8; 0],
}
#[repr(C)]
pub struct nft_fib {
    _private: [u8; 0],
}
#[repr(C)]
pub struct nft_expr_type {
    _private: [u8; 0],
}
#[repr(C)]
pub struct nft_policy {
    _private: [u8; 0],
}
#[repr(C)]
pub struct nft_verdict {
    pub code: u32,
}
#[repr(C)]
pub struct nft_expr_ops {
    pub type_: *mut nft_expr_type,
    pub size: usize,
    pub eval: Option<unsafe extern "C" fn(*const nft_expr, *mut nft_regs, *const nft_pktinfo)>,
    pub init: Option<unsafe extern "C" fn() -> c_int>,
    pub dump: Option<unsafe extern "C" fn()>,
    pub validate: Option<unsafe extern "C" fn() -> c_int>,
}

extern "C" {
    fn nft_expr_priv(expr: *const nft_expr) -> *const nft_fib;
    fn nft_pf(pkt: *const nft_pktinfo) -> u8;
    fn nft_fib4_eval(expr: *const nft_expr, regs: *mut nft_regs, pkt: *const nft_pktinfo);
    fn nft_fib4_eval_type(expr: *const nft_expr, regs: *mut nft_regs, pkt: *const nft_pktinfo);
    fn nft_fib6_eval(expr: *const nft_expr, regs: *mut nft_regs, pkt: *const nft_pktinfo);
    fn nft_fib6_eval_type(expr: *const nft_expr, regs: *mut nft_regs, pkt: *const nft_pktinfo);
    fn nft_fib_init() -> c_int;
    fn nft_fib_dump();
    fn nft_fib_validate() -> c_int;
    fn nft_register_expr(ty: *mut nft_expr_type) -> c_int;
    fn nft_unregister_expr(ty: *mut nft_expr_type);
}

extern "C" {
    static mut nft_fib_policy: nft_policy;
}

const NFPROTO_IPV4: u8 = 2;
const NFPROTO_IPV6: u8 = 10;
const NFPROTO_INET: u8 = 1;
const NFT_FIB_RESULT_OIF: u32 = 0;
const NFT_FIB_RESULT_OIFNAME: u32 = 1;
const NFT_FIB_RESULT_ADDRTYPE: u32 = 2;
const NF_DROP: u32 = 0;
const NFTA_FIB_MAX: u32 = 0;

unsafe extern "C" fn nft_fib_inet_eval(
    expr: *const nft_expr,
    regs: *mut nft_regs,
    pkt: *const nft_pktinfo,
) {
    let priv_ = &*nft_expr_priv(expr);

    match nft_pf(pkt) {
        NFPROTO_IPV4 => match *(priv_ as *const nft_fib as *const u32) {
            NFT_FIB_RESULT_OIF | NFT_FIB_RESULT_OIFNAME => {
                return nft_fib4_eval(expr, regs, pkt);
            }
            NFT_FIB_RESULT_ADDRTYPE => {
                return nft_fib4_eval_type(expr, regs, pkt);
            }
            _ => {}
        },
        NFPROTO_IPV6 => match *(priv_ as *const nft_fib as *const u32) {
            NFT_FIB_RESULT_OIF | NFT_FIB_RESULT_OIFNAME => {
                return nft_fib6_eval(expr, regs, pkt);
            }
            NFT_FIB_RESULT_ADDRTYPE => {
                return nft_fib6_eval_type(expr, regs, pkt);
            }
            _ => {}
        },
        _ => {}
    }

    (*regs).verdict.code = NF_DROP;
}

static mut nft_fib_inet_type: nft_expr_type = nft_expr_type { _private: [] };

static nft_fib_inet_ops: nft_expr_ops = nft_expr_ops {
    type_: unsafe { &mut nft_fib_inet_type },
    size: core::mem::size_of::<nft_fib>(),
    eval: Some(nft_fib_inet_eval),
    init: Some(nft_fib_init),
    dump: Some(nft_fib_dump),
    validate: Some(nft_fib_validate),
};

// __read_mostly
static mut NFT_FIB_INET_TYPE_INITIALIZED: bool = false;

unsafe extern "C" fn nft_fib_inet_module_init() -> c_int {
    NFT_FIB_INET_TYPE_INITIALIZED = true;
    nft_register_expr(&mut nft_fib_inet_type)
}

unsafe extern "C" fn nft_fib_inet_module_exit() {
    nft_unregister_expr(&mut nft_fib_inet_type);
}

// module_init(nft_fib_inet_module_init);
// module_exit(nft_fib_inet_module_exit);
// MODULE_LICENSE("GPL");
// MODULE_AUTHOR("Florian Westphal <fw@strlen.de>");
// MODULE_ALIAS_NFT_AF_EXPR(1, "fib");
// MODULE_DESCRIPTION("nftables fib inet support");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
