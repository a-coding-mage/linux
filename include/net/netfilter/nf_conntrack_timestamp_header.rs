/* SPDX-License-Identifier: GPL-2.0 */

// Dependencies supplied by the surrounding kernel translation.

#[repr(C)]
pub struct nf_conn_tstamp {
    pub start: u_int64_t,
    pub stop: u_int64_t,
}

#[inline]
pub unsafe fn nf_conn_tstamp_find(ct: *const nf_conn) -> *mut nf_conn_tstamp {
    #[cfg(feature = "CONFIG_NF_CONNTRACK_TIMESTAMP")]
    {
        return nf_ct_ext_find(ct, NF_CT_EXT_TSTAMP);
    }
    #[cfg(not(feature = "CONFIG_NF_CONNTRACK_TIMESTAMP"))]
    {
        return core::ptr::null_mut();
    }
}

#[inline]
pub unsafe fn nf_ct_tstamp_ext_add(ct: *mut nf_conn, gfp: gfp_t) -> *mut nf_conn_tstamp {
    #[cfg(feature = "CONFIG_NF_CONNTRACK_TIMESTAMP")]
    {
        let net: *mut net = nf_ct_net(ct);

        if !(*net).ct.sysctl_tstamp {
            return core::ptr::null_mut();
        }

        return nf_ct_ext_add(ct, NF_CT_EXT_TSTAMP, gfp);
    }
    #[cfg(not(feature = "CONFIG_NF_CONNTRACK_TIMESTAMP"))]
    {
        return core::ptr::null_mut();
    }
}

#[cfg(feature = "CONFIG_NF_CONNTRACK_TIMESTAMP")]
pub unsafe extern "C" fn nf_conntrack_tstamp_pernet_init(net: *mut net);

#[cfg(not(feature = "CONFIG_NF_CONNTRACK_TIMESTAMP"))]
#[inline]
pub unsafe fn nf_conntrack_tstamp_pernet_init(_net: *mut net) {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
