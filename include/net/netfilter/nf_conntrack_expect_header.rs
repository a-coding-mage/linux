/* SPDX-License-Identifier: GPL-2.0 */
/*
 * connection tracking expectations.
 *
 * Translated from nf_conntrack_expect.h.  C include dependencies are
 * intentionally left as external Rust types and declarations.
 */

extern "C" {
    pub static mut nf_ct_expect_hsize: ::core::ffi::c_uint;
    pub static mut nf_ct_expect_max: ::core::ffi::c_uint;
    pub static mut nf_ct_expect_hash: *mut hlist_head;
}

#[repr(C)]
pub struct nf_conntrack_expect {
    /* Conntrack expectation list member */
    pub lnode: hlist_node,

    /* Hash member */
    pub hnode: hlist_node,

    /* Network namespace */
    pub net: possible_net_t,

    /* We expect this tuple, with the following mask */
    pub master_tuple: nf_conntrack_tuple,
    pub tuple: nf_conntrack_tuple,
    pub mask: nf_conntrack_tuple_mask,

    /* The CONFIG_NF_CONNTRACK_ZONES field is present when that configuration
     * option is enabled in the C build. */
    #[cfg(CONFIG_NF_CONNTRACK_ZONES)]
    pub zone: nf_conntrack_zone,

    /* Usage count. */
    pub use_: refcount_t,

    /* Flags */
    pub flags: ::core::ffi::c_uint,

    /* Expectation class */
    pub class: ::core::ffi::c_uint,

    /* Event filter mask */
    pub event_mask: u16,

    /* Function to call after setup and insertion */
    pub expectfn: Option<unsafe extern "C" fn(new: *mut nf_conn, this: *mut nf_conntrack_expect)>,

    /* Helper that created this expectation */
    pub helper: *mut nf_conntrack_helper,

    /* Helper to assign to new connection */
    pub assign_helper: *mut nf_conntrack_helper,

    /* The conntrack of the master connection */
    pub master: *mut nf_conn,

    /* jiffies32 when this expectation expires */
    pub timeout: u32,

    /* These fields are present when NAT support is enabled. */
    #[cfg(CONFIG_NF_NAT)]
    pub saved_addr: nf_inet_addr,
    #[cfg(CONFIG_NF_NAT)]
    pub saved_proto: nf_conntrack_man_proto,
    #[cfg(CONFIG_NF_NAT)]
    pub dir: ip_conntrack_dir,

    pub rcu: rcu_head,
}

#[inline]
pub unsafe fn nf_ct_exp_is_expired(exp: *const nf_conntrack_expect) -> bool {
    if (core::ptr::read_volatile(&(*exp).flags) & NF_CT_EXPECT_DEAD) != 0 {
        return true;
    }

    (core::ptr::read_volatile(&(*exp).timeout) as i32)
        .wrapping_sub(nfct_time_stamp as i32)
        <= 0
}

#[inline]
pub unsafe fn nf_ct_exp_net(exp: *mut nf_conntrack_expect) -> *mut net {
    read_pnet(&mut (*exp).net)
}

#[inline]
pub unsafe fn nf_ct_exp_zone_equal_any(
    a: *const nf_conntrack_expect,
    b: *const nf_conntrack_zone,
) -> bool {
    #[cfg(CONFIG_NF_CONNTRACK_ZONES)]
    {
        return (*a).zone.id == (*b).id;
    }
    #[cfg(not(CONFIG_NF_CONNTRACK_ZONES))]
    {
        let _ = (a, b);
        true
    }
}

pub const NF_CT_EXP_POLICY_NAME_LEN: usize = 16;

#[repr(C)]
pub struct nf_conntrack_expect_policy {
    pub max_expected: ::core::ffi::c_uint,
    pub timeout: ::core::ffi::c_uint,
    pub name: [::core::ffi::c_char; NF_CT_EXP_POLICY_NAME_LEN],
}

pub const NF_CT_EXPECT_CLASS_DEFAULT: ::core::ffi::c_uint = 0;
pub const NF_CT_EXPECT_MAX_CNT: ::core::ffi::c_uint = 255;

/* Allow to reuse expectations with the same tuples from different master
 * conntracks.
 */
pub const NF_CT_EXP_F_SKIP_MASTER: ::core::ffi::c_uint = 0x1;

extern "C" {
    pub fn nf_conntrack_expect_pernet_init(net: *mut net) -> ::core::ffi::c_int;
    pub fn nf_conntrack_expect_pernet_fini(net: *mut net);

    pub fn nf_conntrack_expect_init() -> ::core::ffi::c_int;
    pub fn nf_conntrack_expect_fini();

    pub fn __nf_ct_expect_find(
        net: *mut net,
        zone: *const nf_conntrack_zone,
        tuple: *const nf_conntrack_tuple,
    ) -> *mut nf_conntrack_expect;

    pub fn nf_ct_expect_find_get(
        net: *mut net,
        zone: *const nf_conntrack_zone,
        tuple: *const nf_conntrack_tuple,
    ) -> *mut nf_conntrack_expect;

    pub fn nf_ct_find_expectation(
        net: *mut net,
        zone: *const nf_conntrack_zone,
        tuple: *const nf_conntrack_tuple,
        unlink: bool,
    ) -> *mut nf_conntrack_expect;

    pub fn nf_ct_unlink_expect_report(exp: *mut nf_conntrack_expect, portid: u32, report: ::core::ffi::c_int);

    pub fn nf_ct_remove_expectations(ct: *mut nf_conn);
    pub fn nf_ct_unexpect_related(exp: *mut nf_conntrack_expect);

    pub fn nf_ct_expect_iterate_destroy(
        iter: Option<unsafe extern "C" fn(e: *mut nf_conntrack_expect, data: *mut ::core::ffi::c_void) -> bool>,
        data: *mut ::core::ffi::c_void,
    );
    pub fn nf_ct_expect_iterate_net(
        net: *mut net,
        iter: Option<unsafe extern "C" fn(e: *mut nf_conntrack_expect, data: *mut ::core::ffi::c_void) -> bool>,
        data: *mut ::core::ffi::c_void,
        portid: u32,
        report: ::core::ffi::c_int,
    );

    /* Allocate space for an expectation: this is mandatory before calling
       nf_ct_expect_related.  You will have to call put afterwards. */
    pub fn nf_ct_expect_alloc(me: *mut nf_conn) -> *mut nf_conntrack_expect;
    pub fn nf_ct_expect_init(
        exp: *mut nf_conntrack_expect,
        class: ::core::ffi::c_uint,
        l3num: u8,
        saddr: *const nf_inet_addr,
        daddr: *const nf_inet_addr,
        l4num: u8,
        src: *const __be16,
        dst: *const __be16,
    );
    pub fn nf_ct_expect_put(exp: *mut nf_conntrack_expect);
    pub fn nf_ct_expect_related_report(
        expect: *mut nf_conntrack_expect,
        portid: u32,
        report: ::core::ffi::c_int,
        flags: ::core::ffi::c_uint,
    ) -> ::core::ffi::c_int;

    pub fn nf_ct_expect_related_pair(
        expect: *mut *mut nf_conntrack_expect,
        flag: ::core::ffi::c_uint,
    ) -> ::core::ffi::c_int;

    pub fn nf_ct_expectation_gc(master_help: *mut nf_conn_help);
}

#[inline]
pub unsafe fn nf_ct_unlink_expect(exp: *mut nf_conntrack_expect) {
    nf_ct_unlink_expect_report(exp, 0, 0);
}

#[inline]
pub unsafe fn nf_ct_expect_related(
    expect: *mut nf_conntrack_expect,
    flags: ::core::ffi::c_uint,
) -> ::core::ffi::c_int {
    nf_ct_expect_related_report(expect, 0, 0, flags)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
