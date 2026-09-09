/* SPDX-License-Identifier: GPL-2.0 */
/*
 * connection tracking helpers.
 *
 * 16 Dec 2003: Yasuyuki Kozakai @USAGI <yasuyuki.kozakai@toshiba.co.jp>
 *	- generalize L3 protocol dependent part.
 *
 * Derived from include/linux/netfiter_ipv4/ip_conntrack_helper.h
 */

/* C dependencies: linux/refcount.h and net/netfilter conntrack headers. */

pub const NF_NAT_HELPER_PREFIX: &str = "ip_nat_";
macro_rules! NF_NAT_HELPER_NAME {
    ($name:expr) => {
        concat!(NF_NAT_HELPER_PREFIX, $name)
    };
}
/* MODULE_ALIAS_NF_NAT_HELPER(name) expands to MODULE_ALIAS(NF_NAT_HELPER_NAME(name)). */

pub const NF_CT_HELPER_F_USERSPACE: u32 = 1 << 0;
pub const NF_CT_HELPER_F_CONFIGURED: u32 = 1 << 1;

pub const NF_CT_HELPER_NAME_LEN: usize = 16;
pub const NF_CT_MAX_EXPECT_CLASSES: usize = 4;

#[repr(C)]
pub struct nf_conntrack_helper {
    pub hnode: hlist_node,
    pub rcu: rcu_head,
    pub name: [core::ffi::c_char; NF_CT_HELPER_NAME_LEN],
    pub me: *mut module,
    pub expect_policy: [nf_conntrack_expect_policy; NF_CT_MAX_EXPECT_CLASSES],
    pub ct_refcnt: refcount_t,
    pub nfproto: u8,
    pub l4proto: u8,
    pub help: Option<unsafe extern "C" fn(
        skb: *mut sk_buff,
        protoff: core::ffi::c_uint,
        ct: *mut nf_conn,
        conntrackinfo: ip_conntrack_info,
    ) -> core::ffi::c_int>,
    pub destroy: Option<unsafe extern "C" fn(ct: *mut nf_conn)>,
    pub from_nlattr: Option<unsafe extern "C" fn(attr: *mut nlattr, ct: *mut nf_conn) -> core::ffi::c_int>,
    pub to_nlattr: Option<unsafe extern "C" fn(skb: *mut sk_buff, ct: *const nf_conn) -> core::ffi::c_int>,
    pub expect_class_max: core::ffi::c_uint,
    pub flags: core::ffi::c_uint,
    pub queue_num: core::ffi::c_uint,
    pub data_len: u16,
    pub nat_mod_name: [core::ffi::c_char; NF_CT_HELPER_NAME_LEN],
}

#[repr(C, align(8))]
pub struct nf_conn_help {
    pub helper: *mut nf_conntrack_helper,
    pub expectations: hlist_head,
    pub expecting: [u8; NF_CT_MAX_EXPECT_CLASSES],
    pub data: [core::ffi::c_char; 32],
}

macro_rules! NF_CT_HELPER_BUILD_BUG_ON {
    ($structsize:expr) => {
        /* C: BUILD_BUG_ON(($structsize) > sizeof_field(struct nf_conn_help, data)) */
        const _: () = assert!($structsize <= 32);
    };
}

extern "C" {
    pub fn __nf_conntrack_helper_find(name: *const core::ffi::c_char, l3num: u16, protonum: u8) -> *mut nf_conntrack_helper;
    pub fn nf_conntrack_helper_try_module_get(name: *const core::ffi::c_char, l3num: u16, protonum: u8) -> *mut nf_conntrack_helper;
    pub fn nf_conntrack_helper_put(helper: *mut nf_conntrack_helper);
    pub fn nf_ct_helper_init(helper: *mut nf_conntrack_helper, l3num: u8, protonum: u16, name: *const core::ffi::c_char, exp_pol: *const nf_conntrack_expect_policy, expect_class_max: u32, help: Option<unsafe extern "C" fn(*mut sk_buff, core::ffi::c_uint, *mut nf_conn, ip_conntrack_info) -> core::ffi::c_int>, from_nlattr: Option<unsafe extern "C" fn(*mut nlattr, *mut nf_conn) -> core::ffi::c_int>, module: *mut module);
    pub fn nf_conntrack_helper_register(helper: *mut nf_conntrack_helper, old: *mut *mut nf_conntrack_helper) -> core::ffi::c_int;
    pub fn __nf_conntrack_helper_register(helper: *mut nf_conntrack_helper) -> core::ffi::c_int;
    pub fn nf_conntrack_helper_unregister(helper: *mut nf_conntrack_helper);
    pub fn nf_conntrack_helper_release(helper: *mut nf_conntrack_helper);
    pub fn nf_conntrack_helpers_register(helper: *mut nf_conntrack_helper, count: core::ffi::c_uint, old: *mut *mut nf_conntrack_helper) -> core::ffi::c_int;
    pub fn nf_conntrack_helpers_unregister(old: *mut *mut nf_conntrack_helper, count: core::ffi::c_uint);
    pub fn nf_ct_helper_ext_add(ct: *mut nf_conn, gfp: gfp_t) -> *mut nf_conn_help;
    pub fn __nf_ct_try_assign_helper(ct: *mut nf_conn, tmpl: *mut nf_conn, flags: gfp_t) -> core::ffi::c_int;
    pub fn nf_ct_helper(skb: *mut sk_buff, ct: *mut nf_conn, ctinfo: ip_conntrack_info, proto: u16) -> core::ffi::c_int;
    pub fn nf_ct_add_helper(ct: *mut nf_conn, name: *const core::ffi::c_char, family: u8, proto: u8, nat: bool, hp: *mut *mut nf_conntrack_helper) -> core::ffi::c_int;
    pub fn nf_ct_helper_destroy(ct: *mut nf_conn);
    pub fn nf_conntrack_broadcast_help(skb: *mut sk_buff, ct: *mut nf_conn, ctinfo: ip_conntrack_info, timeout: core::ffi::c_uint) -> core::ffi::c_int;
    pub fn nf_conntrack_helper_init() -> core::ffi::c_int;
    pub fn nf_conntrack_helper_fini();
}

#[repr(C)]
pub struct nf_ct_helper_expectfn {
    pub head: list_head,
    pub name: *const core::ffi::c_char,
    pub expectfn: Option<unsafe extern "C" fn(*mut nf_conn, *mut nf_conntrack_expect)>,
}

extern "C" {
    pub fn nf_ct_helper_log(skb: *mut sk_buff, ct: *const nf_conn, fmt: *const core::ffi::c_char, ...);
    pub fn nf_ct_helper_expectfn_register(n: *mut nf_ct_helper_expectfn);
    pub fn nf_ct_helper_expectfn_unregister(n: *mut nf_ct_helper_expectfn);
    pub fn nf_ct_helper_expectfn_destroy(n: *const nf_ct_helper_expectfn);
    pub fn nf_ct_helper_expectfn_find_by_name(name: *const core::ffi::c_char) -> *mut nf_ct_helper_expectfn;
    pub fn nf_ct_helper_expectfn_find_by_symbol(symbol: *const core::ffi::c_void) -> *mut nf_ct_helper_expectfn;
}

/* nf_conntrack_helper_deprecated(name) emits the kernel warning with the helper name. */
macro_rules! nf_conntrack_helper_deprecated {
    ($name:expr) => {
        pr_warn!("The %s conntrack helper is scheduled for removal.\nPlease contact the netfilter-devel mailing list if you still need this.\n", $name)
    };
}

#[inline]
pub unsafe fn nfct_help(ct: *const nf_conn) -> *mut nf_conn_help {
    nf_ct_ext_find(ct, NF_CT_EXT_HELPER)
}

#[inline]
pub unsafe fn nfct_help_data(ct: *const nf_conn) -> *mut core::ffi::c_void {
    let help = nf_ct_ext_find(ct, NF_CT_EXT_HELPER);
    if help.is_null() {
        return core::ptr::null_mut();
    }
    (*help).data.as_mut_ptr() as *mut core::ffi::c_void
}

#[inline]
pub unsafe fn nf_ct_help_put(ct: *const nf_conn) {
    let help = nfct_help(ct);
    if help.is_null() {
        return;
    }
    let helper = rcu_dereference((*help).helper);
    if !helper.is_null() && refcount_dec_and_test(&mut (*helper).ct_refcnt) {
        kfree_rcu(helper, rcu);
    }
}

/* The remaining declarations depend on types and inline primitives supplied by the included kernel headers. */
extern "C" {
    pub static mut nf_ct_helper_hash: *mut hlist_head;
    pub static mut nf_ct_helper_hsize: core::ffi::c_uint;
}

#[repr(C)]
pub struct nf_conntrack_nat_helper {
    pub list: list_head,
    pub mod_name: [core::ffi::c_char; NF_CT_HELPER_NAME_LEN],
    pub module: *mut module,
}

/* NF_CT_NAT_HELPER_INIT(name): .mod_name = NF_NAT_HELPER_NAME(name), .module = THIS_MODULE */

extern "C" {
    pub fn nf_nat_helper_register(nat: *mut nf_conntrack_nat_helper);
    pub fn nf_nat_helper_unregister(nat: *mut nf_conntrack_nat_helper);
    pub fn nf_nat_helper_try_module_get(name: *const core::ffi::c_char, l3num: u16, protonum: u8) -> core::ffi::c_int;
    pub fn nf_nat_helper_put(helper: *mut nf_conntrack_helper);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
