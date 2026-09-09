/* SPDX-License-Identifier: ((GPL-2.0 WITH Linux-syscall-note) OR BSD-3-Clause) */
/* Do not edit directly, auto-generated from: */
/*\tDocumentation/netlink/specs/mptcp_pm.yaml */
/* YNL-GEN kernel header */
/* To regenerate run: tools/net/ynl/ynl-regen.sh */

/* C header guards and include directives omitted; required symbols are supplied by dependencies. */

/* Common nested types */
extern "C" {
    pub static mptcp_pm_address_nl_policy:
        [nla_policy; MPTCP_PM_ADDR_ATTR_IF_IDX as usize + 1];

    pub static mptcp_pm_add_addr_nl_policy:
        [nla_policy; MPTCP_PM_ENDPOINT_ADDR as usize + 1];

    pub static mptcp_pm_del_addr_nl_policy:
        [nla_policy; MPTCP_PM_ENDPOINT_ADDR as usize + 1];

    pub static mptcp_pm_get_addr_nl_policy:
        [nla_policy; MPTCP_PM_ATTR_TOKEN as usize + 1];

    pub static mptcp_pm_flush_addrs_nl_policy:
        [nla_policy; MPTCP_PM_ENDPOINT_ADDR as usize + 1];

    pub static mptcp_pm_set_limits_nl_policy:
        [nla_policy; MPTCP_PM_ATTR_SUBFLOWS as usize + 1];

    pub static mptcp_pm_get_limits_nl_policy:
        [nla_policy; MPTCP_PM_ATTR_SUBFLOWS as usize + 1];

    pub static mptcp_pm_set_flags_nl_policy:
        [nla_policy; MPTCP_PM_ATTR_ADDR_REMOTE as usize + 1];

    pub static mptcp_pm_announce_nl_policy:
        [nla_policy; MPTCP_PM_ATTR_TOKEN as usize + 1];

    pub static mptcp_pm_remove_nl_policy:
        [nla_policy; MPTCP_PM_ATTR_LOC_ID as usize + 1];

    pub static mptcp_pm_subflow_create_nl_policy:
        [nla_policy; MPTCP_PM_ATTR_ADDR_REMOTE as usize + 1];

    pub static mptcp_pm_subflow_destroy_nl_policy:
        [nla_policy; MPTCP_PM_ATTR_ADDR_REMOTE as usize + 1];

    /* Ops table for mptcp_pm */
    pub static mptcp_pm_nl_ops: [genl_ops; 11];

    pub fn mptcp_pm_nl_add_addr_doit(skb: *mut sk_buff, info: *mut genl_info) -> i32;
    pub fn mptcp_pm_nl_del_addr_doit(skb: *mut sk_buff, info: *mut genl_info) -> i32;
    pub fn mptcp_pm_nl_get_addr_doit(skb: *mut sk_buff, info: *mut genl_info) -> i32;
    pub fn mptcp_pm_nl_get_addr_dumpit(
        skb: *mut sk_buff,
        cb: *mut netlink_callback,
    ) -> i32;
    pub fn mptcp_pm_nl_flush_addrs_doit(skb: *mut sk_buff, info: *mut genl_info) -> i32;
    pub fn mptcp_pm_nl_set_limits_doit(skb: *mut sk_buff, info: *mut genl_info) -> i32;
    pub fn mptcp_pm_nl_get_limits_doit(skb: *mut sk_buff, info: *mut genl_info) -> i32;
    pub fn mptcp_pm_nl_set_flags_doit(skb: *mut sk_buff, info: *mut genl_info) -> i32;
    pub fn mptcp_pm_nl_announce_doit(skb: *mut sk_buff, info: *mut genl_info) -> i32;
    pub fn mptcp_pm_nl_remove_doit(skb: *mut sk_buff, info: *mut genl_info) -> i32;
    pub fn mptcp_pm_nl_subflow_create_doit(
        skb: *mut sk_buff,
        info: *mut genl_info,
    ) -> i32;
    pub fn mptcp_pm_nl_subflow_destroy_doit(
        skb: *mut sk_buff,
        info: *mut genl_info,
    ) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
