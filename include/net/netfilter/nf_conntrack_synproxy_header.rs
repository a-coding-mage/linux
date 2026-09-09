/* SPDX-License-Identifier: GPL-2.0 */

// Translated dependencies:
// #include <net/netfilter/nf_conntrack_seqadj.h>
// #include <net/netns/generic.h>

use core::ffi::c_void;

// External types and symbols supplied by the surrounding translation unit.
#[repr(C)]
pub struct nf_conn {
    _private: [u8; 0],
}

pub const NF_CT_EXT_SYNPROXY: i32 = 0;
pub const GFP_ATOMIC: i32 = 0;

unsafe extern "C" {
    fn nf_ct_ext_find(
        ct: *const nf_conn,
        id: i32,
    ) -> *mut nf_conn_synproxy;
    fn nf_ct_ext_add(
        ct: *mut nf_conn,
        id: i32,
        gfp: i32,
    ) -> *mut nf_conn_synproxy;
    fn nfct_seqadj_ext_add(ct: *mut nf_conn) -> *mut c_void;
}

#[repr(C)]
pub struct nf_conn_synproxy {
    pub isn: u32,
    pub its: u32,
    pub tsoff: u32,
}

#[inline]
pub unsafe fn nfct_synproxy(ct: *const nf_conn) -> *mut nf_conn_synproxy {
    // IS_ENABLED(CONFIG_NETFILTER_SYNPROXY)
    #[cfg(feature = "CONFIG_NETFILTER_SYNPROXY")]
    {
        nf_ct_ext_find(ct, NF_CT_EXT_SYNPROXY)
    }
    #[cfg(not(feature = "CONFIG_NETFILTER_SYNPROXY"))]
    {
        core::ptr::null_mut()
    }
}

#[inline]
pub unsafe fn nfct_synproxy_ext_add(ct: *mut nf_conn) -> *mut nf_conn_synproxy {
    // IS_ENABLED(CONFIG_NETFILTER_SYNPROXY)
    #[cfg(feature = "CONFIG_NETFILTER_SYNPROXY")]
    {
        nf_ct_ext_add(ct, NF_CT_EXT_SYNPROXY, GFP_ATOMIC)
    }
    #[cfg(not(feature = "CONFIG_NETFILTER_SYNPROXY"))]
    {
        core::ptr::null_mut()
    }
}

#[inline]
pub unsafe fn nf_ct_add_synproxy(
    ct: *mut nf_conn,
    tmpl: *const nf_conn,
) -> bool {
    // IS_ENABLED(CONFIG_NETFILTER_SYNPROXY)
    #[cfg(feature = "CONFIG_NETFILTER_SYNPROXY")]
    {
        if !tmpl.is_null() && !nfct_synproxy(tmpl).is_null() {
            if nfct_seqadj_ext_add(ct).is_null() {
                return false;
            }

            if nfct_synproxy_ext_add(ct).is_null() {
                return false;
            }
        }
    }

    true
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
