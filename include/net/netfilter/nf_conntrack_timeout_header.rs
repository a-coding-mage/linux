/* SPDX-License-Identifier: GPL-2.0 */

// Dependencies supplied by the surrounding translation.

pub const CTNL_TIMEOUT_NAME_MAX: usize = 32;

#[repr(C)]
pub struct nf_ct_timeout {
    pub refcnt: refcount_t,
    pub l3num: __u16,
    pub l4proto: *const nf_conntrack_l4proto,
    pub rcu: rcu_head,
    pub data: [::core::ffi::c_char; 0],
}

#[repr(C)]
pub struct nf_conn_timeout {
    pub timeout: *mut nf_ct_timeout,
}

#[inline]
pub unsafe fn nf_ct_timeout_put(ct: *const nf_conn) {
    // CONFIG_NF_CONNTRACK_TIMEOUT is a build-time configuration condition.
    #[cfg(feature = "CONFIG_NF_CONNTRACK_TIMEOUT")]
    {
        let timeout_ext: *mut nf_conn_timeout;
        let timeout: *mut nf_ct_timeout;

        timeout_ext = nf_ct_ext_find(ct, NF_CT_EXT_TIMEOUT);
        if timeout_ext.is_null() {
            return;
        }

        timeout = rcu_dereference((*timeout_ext).timeout);
        if !timeout.is_null() && refcount_dec_and_test(&mut (*timeout).refcnt) {
            kfree_rcu(timeout, rcu);
        }
    }
}

#[inline]
pub unsafe fn nf_ct_timeout_data(
    t: *const nf_conn_timeout,
) -> *mut ::core::ffi::c_uint {
    #[cfg(feature = "CONFIG_NF_CONNTRACK_TIMEOUT")]
    {
        let timeout: *mut nf_ct_timeout;

        timeout = rcu_dereference((*t).timeout);
        if timeout.is_null() {
            return ::core::ptr::null_mut();
        }

        return (*timeout).data.as_mut_ptr() as *mut ::core::ffi::c_uint;
    }
    #[cfg(not(feature = "CONFIG_NF_CONNTRACK_TIMEOUT"))]
    {
        ::core::ptr::null_mut()
    }
}

#[inline]
pub unsafe fn nf_ct_timeout_find(ct: *const nf_conn) -> *mut nf_conn_timeout {
    #[cfg(feature = "CONFIG_NF_CONNTRACK_TIMEOUT")]
    {
        return nf_ct_ext_find(ct, NF_CT_EXT_TIMEOUT);
    }
    #[cfg(not(feature = "CONFIG_NF_CONNTRACK_TIMEOUT"))]
    {
        ::core::ptr::null_mut()
    }
}

#[inline]
pub unsafe fn nf_ct_timeout_ext_add(
    ct: *mut nf_conn,
    timeout: *mut nf_ct_timeout,
    gfp: gfp_t,
) -> *mut nf_conn_timeout {
    #[cfg(feature = "CONFIG_NF_CONNTRACK_TIMEOUT")]
    {
        if timeout.is_null() {
            return ::core::ptr::null_mut();
        }

        let timeout_ext = nf_ct_ext_add(ct, NF_CT_EXT_TIMEOUT, gfp);
        if timeout_ext.is_null() || !(*timeout_ext).timeout.is_null() {
            return ::core::ptr::null_mut();
        }

        if !refcount_inc_not_zero(&mut (*timeout).refcnt) {
            return ::core::ptr::null_mut();
        }

        rcu_assign_pointer(&mut (*timeout_ext).timeout, timeout);
        return timeout_ext;
    }
    #[cfg(not(feature = "CONFIG_NF_CONNTRACK_TIMEOUT"))]
    {
        ::core::ptr::null_mut()
    }
}

#[inline]
pub unsafe fn nf_ct_timeout_lookup(ct: *const nf_conn) -> *mut ::core::ffi::c_uint {
    let mut timeouts: *mut ::core::ffi::c_uint = ::core::ptr::null_mut();
    #[cfg(feature = "CONFIG_NF_CONNTRACK_TIMEOUT")]
    {
        let timeout_ext = nf_ct_timeout_find(ct);
        if !timeout_ext.is_null() && !rcu_access_pointer((*timeout_ext).timeout).is_null() {
            timeouts = nf_ct_timeout_data(timeout_ext);
        }
    }
    timeouts
}

#[cfg(feature = "CONFIG_NF_CONNTRACK_TIMEOUT")]
extern "C" {
    pub fn nf_ct_untimeout(net: *mut net, timeout: *mut nf_ct_timeout);
    pub fn nf_ct_set_timeout(
        net: *mut net,
        ct: *mut nf_conn,
        l3num: u8,
        l4num: u8,
        timeout_name: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    pub fn nf_ct_destroy_timeout(ct: *mut nf_conn);
}

#[cfg(not(feature = "CONFIG_NF_CONNTRACK_TIMEOUT"))]
#[inline]
pub unsafe fn nf_ct_set_timeout(
    _net: *mut net,
    _ct: *mut nf_conn,
    _l3num: u8,
    _l4num: u8,
    _timeout_name: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    -EOPNOTSUPP
}

#[cfg(not(feature = "CONFIG_NF_CONNTRACK_TIMEOUT"))]
#[inline]
pub unsafe fn nf_ct_destroy_timeout(_ct: *mut nf_conn) {}

#[cfg(feature = "CONFIG_NF_CONNTRACK_TIMEOUT")]
#[repr(C)]
pub struct nf_ct_timeout_hooks {
    pub timeout_find_get:
        Option<unsafe extern "C" fn(*mut net, *const ::core::ffi::c_char) -> *mut nf_ct_timeout>,
    pub timeout_put: Option<unsafe extern "C" fn(*mut nf_ct_timeout)>,
}

#[cfg(feature = "CONFIG_NF_CONNTRACK_TIMEOUT")]
extern "C" {
    pub static mut nf_ct_timeout_hook: *const nf_ct_timeout_hooks;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
