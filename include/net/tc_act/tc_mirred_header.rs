/* SPDX-License-Identifier: GPL-2.0 */

// C dependencies:
//   #include <net/act_api.h>
//   #include <linux/tc_act/tc_mirred.h>

#[repr(C)]
pub struct tcf_mirred {
    pub common: tc_action,
    pub tcfm_eaction: ::core::ffi::c_int,
    pub tcfm_blockid: u32,
    pub tcfm_mac_header_xmit: bool,
    pub tcfm_dev: *mut net_device,
    pub tcfm_dev_tracker: netdevice_tracker,
    pub tcfm_list: list_head,
}

#[inline]
pub unsafe fn to_mirred(a: *mut tc_action) -> *mut tcf_mirred {
    a as *mut tcf_mirred
}

#[inline]
pub unsafe fn is_tcf_mirred_egress_redirect(a: *const tc_action) -> bool {
    // Preserves the CONFIG_NET_CLS_ACT conditional from the C header.
    #[cfg(CONFIG_NET_CLS_ACT)]
    {
        if !(*a).ops.is_null() && (*(*a).ops).id == TCA_ID_MIRRED {
            return (*to_mirred(a as *mut tc_action)).tcfm_eaction == TCA_EGRESS_REDIR;
        }
    }
    false
}

#[inline]
pub unsafe fn is_tcf_mirred_egress_mirror(a: *const tc_action) -> bool {
    // Preserves the CONFIG_NET_CLS_ACT conditional from the C header.
    #[cfg(CONFIG_NET_CLS_ACT)]
    {
        if !(*a).ops.is_null() && (*(*a).ops).id == TCA_ID_MIRRED {
            return (*to_mirred(a as *mut tc_action)).tcfm_eaction == TCA_EGRESS_MIRROR;
        }
    }
    false
}

#[inline]
pub unsafe fn is_tcf_mirred_ingress_redirect(a: *const tc_action) -> bool {
    // Preserves the CONFIG_NET_CLS_ACT conditional from the C header.
    #[cfg(CONFIG_NET_CLS_ACT)]
    {
        if !(*a).ops.is_null() && (*(*a).ops).id == TCA_ID_MIRRED {
            return (*to_mirred(a as *mut tc_action)).tcfm_eaction == TCA_INGRESS_REDIR;
        }
    }
    false
}

#[inline]
pub unsafe fn is_tcf_mirred_ingress_mirror(a: *const tc_action) -> bool {
    // Preserves the CONFIG_NET_CLS_ACT conditional from the C header.
    #[cfg(CONFIG_NET_CLS_ACT)]
    {
        if !(*a).ops.is_null() && (*(*a).ops).id == TCA_ID_MIRRED {
            return (*to_mirred(a as *mut tc_action)).tcfm_eaction == TCA_INGRESS_MIRROR;
        }
    }
    false
}

#[inline]
pub unsafe fn tcf_mirred_dev(a: *const tc_action) -> *mut net_device {
    // rtnl_dereference(to_mirred(a)->tcfm_dev)
    (*to_mirred(a as *mut tc_action)).tcfm_dev
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
