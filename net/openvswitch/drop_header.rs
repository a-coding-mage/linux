/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * OpenvSwitch drop reason list.
 */

// Dependencies supplied by the surrounding translation unit:
// `sk_buff`, `SKB_DROP_REASON_SUBSYS_OPENVSWITCH`, `SKB_DROP_REASON_SUBSYS_SHIFT`,
// and `kfree_skb_reason`.

macro_rules! OVS_DROP_REASONS {
    ($R:ident) => {
        $R!(OVS_DROP_LAST_ACTION);
        $R!(OVS_DROP_ACTION_ERROR);
        $R!(OVS_DROP_EXPLICIT);
        $R!(OVS_DROP_EXPLICIT_WITH_ERROR);
        $R!(OVS_DROP_METER);
        $R!(OVS_DROP_RECURSION_LIMIT);
        $R!(OVS_DROP_DEFERRED_LIMIT);
        $R!(OVS_DROP_FRAG_L2_TOO_LONG);
        $R!(OVS_DROP_FRAG_INVALID_PROTO);
        $R!(OVS_DROP_CONNTRACK);
        $R!(OVS_DROP_IP_TTL);
        // deliberate comment for trailing \
    };
}

#[repr(i32)]
pub enum ovs_drop_reason {
    __OVS_DROP_REASON = SKB_DROP_REASON_SUBSYS_OPENVSWITCH << SKB_DROP_REASON_SUBSYS_SHIFT,
    OVS_DROP_LAST_ACTION,
    OVS_DROP_ACTION_ERROR,
    OVS_DROP_EXPLICIT,
    OVS_DROP_EXPLICIT_WITH_ERROR,
    OVS_DROP_METER,
    OVS_DROP_RECURSION_LIMIT,
    OVS_DROP_DEFERRED_LIMIT,
    OVS_DROP_FRAG_L2_TOO_LONG,
    OVS_DROP_FRAG_INVALID_PROTO,
    OVS_DROP_CONNTRACK,
    OVS_DROP_IP_TTL,
    OVS_DROP_MAX,
}

#[inline]
pub unsafe fn ovs_kfree_skb_reason(skb: *mut sk_buff, reason: ovs_drop_reason) {
    kfree_skb_reason(skb, reason as u32);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
