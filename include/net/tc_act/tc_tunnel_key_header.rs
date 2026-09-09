/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Copyright (c) 2016, Amir Vadai <amir@vadai.me>
 * Copyright (c) 2016, Mellanox Technologies. All rights reserved.
 */

// Dependencies supplied by the surrounding kernel translation:
// net/act_api.h, linux/tc_act/tc_tunnel_key.h, and net/dst_metadata.h.

#[repr(C)]
pub struct tcf_tunnel_key_params {
    pub rcu: rcu_head,
    pub tcft_action: ::core::ffi::c_int,
    pub action: ::core::ffi::c_int,
    pub tcft_enc_metadata: *mut metadata_dst,
}

#[repr(C)]
pub struct tcf_tunnel_key {
    pub common: tc_action,
    pub params: *mut tcf_tunnel_key_params,
}

#[inline]
pub unsafe fn to_tunnel_key(a: *mut tc_action) -> *mut tcf_tunnel_key {
    a as *mut tcf_tunnel_key
}

#[inline]
pub unsafe fn is_tcf_tunnel_set(a: *const tc_action) -> bool {
    // C conditional: CONFIG_NET_CLS_ACT
    #[cfg(CONFIG_NET_CLS_ACT)]
    {
        let t = to_tunnel_key(a as *mut tc_action);
        let params: *mut tcf_tunnel_key_params =
            rcu_dereference_protected((*t).params,
                lockdep_is_held(&(*a).tcfa_lock));
        if !(*a).ops.is_null() && (*(*a).ops).id == TCA_ID_TUNNEL_KEY {
            return (*params).tcft_action == TCA_TUNNEL_KEY_ACT_SET;
        }
    }
    false
}

#[inline]
pub unsafe fn is_tcf_tunnel_release(a: *const tc_action) -> bool {
    // C conditional: CONFIG_NET_CLS_ACT
    #[cfg(CONFIG_NET_CLS_ACT)]
    {
        let t = to_tunnel_key(a as *mut tc_action);
        let params: *mut tcf_tunnel_key_params =
            rcu_dereference_protected((*t).params,
                lockdep_is_held(&(*a).tcfa_lock));
        if !(*a).ops.is_null() && (*(*a).ops).id == TCA_ID_TUNNEL_KEY {
            return (*params).tcft_action == TCA_TUNNEL_KEY_ACT_RELEASE;
        }
    }
    false
}

#[inline]
pub unsafe fn tcf_tunnel_info(a: *const tc_action) -> *mut ip_tunnel_info {
    // C conditional: CONFIG_NET_CLS_ACT
    #[cfg(CONFIG_NET_CLS_ACT)]
    {
        let t = to_tunnel_key(a as *mut tc_action);
        let params: *mut tcf_tunnel_key_params =
            rcu_dereference_protected((*t).params,
                lockdep_is_held(&(*a).tcfa_lock));
        return &mut (*(*params).tcft_enc_metadata).u.tun_info;
    }
    ::core::ptr::null_mut()
}

#[inline]
pub unsafe fn tcf_tunnel_info_copy(a: *const tc_action) -> *mut ip_tunnel_info {
    // C conditional: CONFIG_NET_CLS_ACT
    #[cfg(CONFIG_NET_CLS_ACT)]
    {
        let tun = tcf_tunnel_info(a);
        if !tun.is_null() {
            let tun_size = ::core::mem::size_of::<ip_tunnel_info>()
                + (*tun).options_len as usize;
            return kmemdup(tun as *const _, tun_size, GFP_ATOMIC);
        }
    }
    ::core::ptr::null_mut()
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
