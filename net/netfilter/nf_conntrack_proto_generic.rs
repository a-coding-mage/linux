// SPDX-License-Identifier: GPL-2.0-only
/* (C) 1999-2001 Paul `Rusty' Russell
 * (C) 2002-2004 Netfilter Core Team <coreteam@netfilter.org>
 */

// Dependencies supplied by the surrounding kernel translation.

static NF_CT_GENERIC_TIMEOUT: ::core::ffi::c_uint = 600 * HZ;

// CONFIG_NF_CONNTRACK_TIMEOUT conditionally includes the following items.
#[cfg(CONFIG_NF_CONNTRACK_TIMEOUT)]
unsafe fn generic_timeout_nlattr_to_obj(
    tb: *mut *mut nlattr,
    net: *mut net,
    data: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    let gn: *mut nf_generic_net = nf_generic_pernet(net);
    let mut timeout: *mut ::core::ffi::c_uint = data.cast();

    if timeout.is_null() {
        timeout = &mut (*gn).timeout;
    }

    if !(*tb.add(CTA_TIMEOUT_GENERIC_TIMEOUT as usize)).is_null() {
        *timeout = ntohl(nla_get_be32(
            *tb.add(CTA_TIMEOUT_GENERIC_TIMEOUT as usize),
        )) * HZ;
    } else {
        // Set default generic timeout.
        *timeout = (*gn).timeout;
    }

    0
}

#[cfg(CONFIG_NF_CONNTRACK_TIMEOUT)]
unsafe fn generic_timeout_obj_to_nlattr(
    skb: *mut sk_buff,
    data: *const ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    let timeout: *const ::core::ffi::c_uint = data.cast();

    if nla_put_be32(
        skb,
        CTA_TIMEOUT_GENERIC_TIMEOUT,
        htonl(*timeout / HZ),
    ) != 0 {
        return -ENOSPC;
    }

    0
}

#[cfg(CONFIG_NF_CONNTRACK_TIMEOUT)]
static GENERIC_TIMEOUT_NLA_POLICY: [nla_policy; CTA_TIMEOUT_GENERIC_MAX as usize + 1] = {
    let mut policy = [nla_policy { type_: 0 }; CTA_TIMEOUT_GENERIC_MAX as usize + 1];
    policy[CTA_TIMEOUT_GENERIC_TIMEOUT as usize] = nla_policy { type_: NLA_U32 };
    policy
};

unsafe fn nf_conntrack_generic_init_net(net: *mut net) {
    let gn: *mut nf_generic_net = nf_generic_pernet(net);

    (*gn).timeout = NF_CT_GENERIC_TIMEOUT;
}

static NF_CONNTRACK_L4PROTO_GENERIC: nf_conntrack_l4proto = nf_conntrack_l4proto {
    l4proto: 255,
    allow_clash: true,
    #[cfg(CONFIG_NF_CONNTRACK_TIMEOUT)]
    ctnl_timeout: nf_conntrack_l4proto_ctnl_timeout {
        nlattr_to_obj: Some(generic_timeout_nlattr_to_obj),
        obj_to_nlattr: Some(generic_timeout_obj_to_nlattr),
        nlattr_max: CTA_TIMEOUT_GENERIC_MAX,
        obj_size: ::core::mem::size_of::<::core::ffi::c_uint>(),
        nla_policy: GENERIC_TIMEOUT_NLA_POLICY.as_ptr(),
    },
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
