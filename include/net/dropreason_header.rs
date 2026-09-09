/* SPDX-License-Identifier: GPL-2.0-or-later */

// Dependency: declarations from <net/dropreason-core.h> are supplied externally.

/**
 * enum skb_drop_reason_subsys - subsystem tag for (extended) drop reasons
 */
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum skb_drop_reason_subsys {
    /** @SKB_DROP_REASON_SUBSYS_CORE: core drop reasons defined above */
    SKB_DROP_REASON_SUBSYS_CORE,

    /**
     * @SKB_DROP_REASON_SUBSYS_MAC80211_UNUSABLE: mac80211 drop reasons
     * for unusable frames, see net/mac80211/drop.h
     */
    SKB_DROP_REASON_SUBSYS_MAC80211_UNUSABLE,

    /**
     * @SKB_DROP_REASON_SUBSYS_OPENVSWITCH: openvswitch drop reasons,
     * see net/openvswitch/drop.h
     */
    SKB_DROP_REASON_SUBSYS_OPENVSWITCH,

    /**
     * @SKB_DROP_REASON_SUBSYS_QDISC: TC qdisc drop reasons,
     * see include/net/dropreason-qdisc.h
     */
    SKB_DROP_REASON_SUBSYS_QDISC,

    /** @SKB_DROP_REASON_SUBSYS_NUM: number of subsystems defined */
    SKB_DROP_REASON_SUBSYS_NUM,
}

#[repr(C)]
pub struct drop_reason_list {
    pub reasons: *const *const ::core::ffi::c_char,
    pub n_reasons: usize,
}

// Note: due to dynamic registrations, access must be under RCU.
unsafe extern "C" {
    pub static mut drop_reasons_by_subsys:
        [*const drop_reason_list; SKB_DROP_REASON_SUBSYS_NUM as usize];

    pub fn drop_reasons_register_subsys(
        subsys: skb_drop_reason_subsys,
        list: *const drop_reason_list,
    );
    pub fn drop_reasons_unregister_subsys(subsys: skb_drop_reason_subsys);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
