// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *      SNMP service broadcast connection tracking helper
 *
 *      (c) 2011 Jiri Olsa <jolsa@redhat.com>
 */

// Linux kernel, netfilter, and module declarations are supplied by the
// surrounding kernel translation environment.

use core::ffi::{c_char, c_int, c_uint, c_void};

// MODULE_AUTHOR!("Jiri Olsa <jolsa@redhat.com>");
// MODULE_DESCRIPTION!("SNMP service broadcast connection tracking helper");
// MODULE_LICENSE!("GPL");
// MODULE_ALIAS_NFCT_HELPER!("snmp");

static mut timeout: c_uint = 30;
// module_param!(timeout, uint, 0400);
// MODULE_PARM_DESC!(timeout, "timeout for master connection/replies in seconds");

// Type and function declarations provided by the imported kernel headers.
type nf_nat_snmp_hook_fn = unsafe extern "C" fn(
    skb: *mut sk_buff,
    protoff: c_uint,
    ct: *mut nf_conn,
    ctinfo: ip_conntrack_info,
) -> c_int;

#[repr(C)]
pub struct sk_buff {
    _private: [u8; 0],
}

#[repr(C)]
pub struct nf_conn {
    pub status: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum ip_conntrack_info {
    CtInfo0 = 0,
}

#[repr(C)]
pub struct nf_conntrack_expect_policy {
    pub max_expected: c_uint,
    pub timeout: c_uint,
}

#[repr(C)]
pub struct nf_conntrack_helper {
    _private: [u8; 0],
}

extern "C" {
    static mut nf_nat_snmp_hook: *mut nf_nat_snmp_hook_fn;

    fn nf_conntrack_broadcast_help(
        skb: *mut sk_buff,
        ct: *mut nf_conn,
        ctinfo: ip_conntrack_info,
        timeout: c_uint,
    );
    fn rcu_dereference(ptr: *mut nf_nat_snmp_hook_fn) -> *mut nf_nat_snmp_hook_fn;
    fn nf_ct_helper_init(
        helper: *mut nf_conntrack_helper,
        family: c_int,
        protocol: c_int,
        name: *const c_char,
        policy: *mut nf_conntrack_expect_policy,
        flags: c_uint,
        help: Option<unsafe extern "C" fn(
            *mut sk_buff,
            c_uint,
            *mut nf_conn,
            ip_conntrack_info,
        ) -> c_int>,
        from_nlattr: *mut c_void,
        module: *mut c_void,
    );
    fn nf_conntrack_helper_register(
        helper: *mut nf_conntrack_helper,
        helper_ptr: *mut *mut nf_conntrack_helper,
    ) -> c_int;
    fn nf_conntrack_helper_unregister(helper_ptr: *mut nf_conntrack_helper);
}

// EXPORT_SYMBOL_GPL!(nf_nat_snmp_hook);

const IPS_NAT_MASK: c_uint = 0;
const NF_ACCEPT: c_int = 1;
const AF_INET: c_int = 2;
const IPPROTO_UDP: c_int = 17;

unsafe extern "C" fn snmp_conntrack_help(
    skb: *mut sk_buff,
    protoff: c_uint,
    ct: *mut nf_conn,
    ctinfo: ip_conntrack_info,
) -> c_int {
    let nf_nat_snmp: *mut nf_nat_snmp_hook_fn;

    nf_conntrack_broadcast_help(skb, ct, ctinfo, timeout);

    nf_nat_snmp = rcu_dereference(nf_nat_snmp_hook);
    if !nf_nat_snmp.is_null() && ((*ct).status & IPS_NAT_MASK) != 0 {
        return (*nf_nat_snmp)(skb, protoff, ct, ctinfo);
    }

    NF_ACCEPT
}

static mut exp_policy: nf_conntrack_expect_policy = nf_conntrack_expect_policy {
    max_expected: 1,
    timeout: 0,
};

static mut helper: nf_conntrack_helper = nf_conntrack_helper { _private: [] };
static mut helper_ptr: *mut nf_conntrack_helper = core::ptr::null_mut();

unsafe extern "C" fn nf_conntrack_snmp_init() -> c_int {
    exp_policy.timeout = timeout;

    nf_ct_helper_init(
        &mut helper,
        AF_INET,
        IPPROTO_UDP,
        b"snmp\0".as_ptr() as *const c_char,
        &mut exp_policy,
        0,
        Some(snmp_conntrack_help),
        core::ptr::null_mut(),
        core::ptr::null_mut(),
    );

    nf_conntrack_helper_register(&mut helper, &mut helper_ptr)
}

unsafe extern "C" fn nf_conntrack_snmp_fini() {
    nf_conntrack_helper_unregister(helper_ptr);
}

// module_init!(nf_conntrack_snmp_init);
// module_exit!(nf_conntrack_snmp_fini);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
