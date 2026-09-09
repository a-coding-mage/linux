// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *      NetBIOS name service broadcast connection tracking helper
 *
 *      (c) 2005 Patrick McHardy <kaber@trash.net>
 */
/*
 *      This helper tracks locally originating NetBIOS name service
 *      requests by issuing permanent expectations (valid until
 *      timing out) matching all reply connections from the
 *      destination network. The only NetBIOS specific thing is
 *      actually the port number.
 */

// Linux kernel and netfilter headers supplied by the surrounding build.

#[allow(non_camel_case_types)]
pub enum sk_buff {}
#[allow(non_camel_case_types)]
pub enum nf_conn {}
#[allow(non_camel_case_types)]
pub enum nf_conntrack_helper {}

#[repr(C)]
pub struct nf_conntrack_expect_policy {
    pub max_expected: u32,
    pub timeout: u32,
}

#[allow(non_camel_case_types)]
pub type ip_conntrack_info = i32;

extern "C" {
    fn nf_conntrack_broadcast_help(
        skb: *mut sk_buff,
        ct: *mut nf_conn,
        ctinfo: ip_conntrack_info,
        timeout: u32,
    ) -> u32;
    fn nf_ct_helper_init(
        helper: *mut nf_conntrack_helper,
        family: i32,
        protocol: i32,
        name: *const u8,
        policy: *mut nf_conntrack_expect_policy,
        expect_class: u32,
        help: unsafe extern "C" fn(
            *mut sk_buff,
            u32,
            *mut nf_conn,
            ip_conntrack_info,
        ) -> i32,
        destroy: *const (),
        module: *const (),
    );
    fn nf_conntrack_helper_register(
        helper: *mut nf_conntrack_helper,
        helper_ptr: *mut *mut nf_conntrack_helper,
    ) -> i32;
    fn nf_conntrack_helper_unregister(helper_ptr: *mut nf_conntrack_helper);
}

const HELPER_NAME: &[u8] = b"netbios-ns\0";
const AF_INET: i32 = 2;
const IPPROTO_UDP: i32 = 17;

// MODULE_AUTHOR("Patrick McHardy <kaber@trash.net>");
// MODULE_DESCRIPTION("NetBIOS name service broadcast connection tracking helper");
// MODULE_LICENSE("GPL");
// MODULE_ALIAS("ip_conntrack_netbios_ns");
// MODULE_ALIAS_NFCT_HELPER(HELPER_NAME);

static mut timeout: u32 = 3;
// module_param(timeout, uint, 0400);
// MODULE_PARM_DESC(timeout, "timeout for master connection/replies in seconds");

static mut exp_policy: nf_conntrack_expect_policy = nf_conntrack_expect_policy {
    max_expected: 1,
    timeout: 0,
};

unsafe extern "C" fn netbios_ns_help(
    skb: *mut sk_buff,
    _protoff: u32,
    ct: *mut nf_conn,
    ctinfo: ip_conntrack_info,
) -> i32 {
    nf_conntrack_broadcast_help(skb, ct, ctinfo, timeout) as i32
}

static mut helper: nf_conntrack_helper = unsafe { core::mem::zeroed() };
static mut helper_ptr: *mut nf_conntrack_helper = core::ptr::null_mut();

unsafe extern "C" fn nf_conntrack_netbios_ns_init() -> i32 {
    // NF_CT_HELPER_BUILD_BUG_ON(0);

    exp_policy.timeout = timeout;

    nf_ct_helper_init(
        &mut helper,
        AF_INET,
        IPPROTO_UDP,
        HELPER_NAME.as_ptr(),
        &mut exp_policy,
        0,
        netbios_ns_help,
        core::ptr::null(),
        core::ptr::null(),
    );

    nf_conntrack_helper_register(&mut helper, &mut helper_ptr)
}

unsafe extern "C" fn nf_conntrack_netbios_ns_fini() {
    nf_conntrack_helper_unregister(helper_ptr);
}

// module_init(nf_conntrack_netbios_ns_init);
// module_exit(nf_conntrack_netbios_ns_fini);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
