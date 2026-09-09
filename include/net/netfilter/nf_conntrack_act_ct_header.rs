/* SPDX-License-Identifier: GPL-2.0 */

/*
 * Dependencies supplied by the corresponding netfilter modules:
 * nf_conn, sk_buff, ip_conntrack_info, IP_CT_DIR_MAX, CTINFO2DIR,
 * nf_ct_ext_find, nf_ct_ext_add, NF_CT_EXT_ACT_CT, GFP_ATOMIC, dev_net,
 * init_net, and the associated network-device fields.
 */

#[repr(C)]
pub struct nf_conn_act_ct_ext {
    pub ifindex: [::core::ffi::c_int; IP_CT_DIR_MAX],
}

#[inline]
pub unsafe fn nf_conn_act_ct_ext_find(
    ct: *const nf_conn,
) -> *mut nf_conn_act_ct_ext {
    // Equivalent of: #if IS_ENABLED(CONFIG_NET_ACT_CT)
    #[cfg(feature = "CONFIG_NET_ACT_CT")]
    {
        nf_ct_ext_find(ct, NF_CT_EXT_ACT_CT) as *mut nf_conn_act_ct_ext
    }
    // Equivalent of the disabled CONFIG_NET_ACT_CT branch.
    #[cfg(not(feature = "CONFIG_NET_ACT_CT"))]
    {
        core::ptr::null_mut()
    }
}

#[inline]
pub unsafe fn nf_conn_act_ct_ext_fill(
    skb: *mut sk_buff,
    ct: *mut nf_conn,
    ctinfo: ip_conntrack_info,
) {
    // Equivalent of: #if IS_ENABLED(CONFIG_NET_ACT_CT)
    #[cfg(feature = "CONFIG_NET_ACT_CT")]
    {
        let act_ct_ext: *mut nf_conn_act_ct_ext = nf_conn_act_ct_ext_find(ct);

        if dev_net((*skb).dev) == &init_net && !act_ct_ext.is_null() {
            (*act_ct_ext).ifindex[CTINFO2DIR(ctinfo)] = (*(*skb).dev).ifindex;
        }
    }
}

#[inline]
pub unsafe fn nf_conn_act_ct_ext_add(
    skb: *mut sk_buff,
    ct: *mut nf_conn,
    ctinfo: ip_conntrack_info,
) -> *mut nf_conn_act_ct_ext {
    // Equivalent of: #if IS_ENABLED(CONFIG_NET_ACT_CT)
    #[cfg(feature = "CONFIG_NET_ACT_CT")]
    {
        let mut act_ct: *mut nf_conn_act_ct_ext =
            nf_ct_ext_find(ct, NF_CT_EXT_ACT_CT) as *mut nf_conn_act_ct_ext;

        if !act_ct.is_null() {
            return act_ct;
        }

        act_ct = nf_ct_ext_add(ct, NF_CT_EXT_ACT_CT, GFP_ATOMIC)
            as *mut nf_conn_act_ct_ext;
        nf_conn_act_ct_ext_fill(skb, ct, ctinfo);
        act_ct
    }
    // Equivalent of the disabled CONFIG_NET_ACT_CT branch.
    #[cfg(not(feature = "CONFIG_NET_ACT_CT"))]
    {
        core::ptr::null_mut()
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
