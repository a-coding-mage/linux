/* SPDX-License-Identifier: ((GPL-2.0 WITH Linux-syscall-note) OR BSD-3-Clause) */
/* Do not edit directly, auto-generated from Documentation/netlink/specs/devlink.yaml. */
/* Rust translation of the generated Linux devlink netlink header. */

// Dependencies supplied by the surrounding kernel/Rust translation.
use core::ffi::c_int;

extern "C" {
    pub static devlink_dl_parent_dev_nl_policy:
        [nla_policy; DEVLINK_ATTR_INDEX + 1];
    pub static devlink_dl_port_function_nl_policy:
        [nla_policy; DEVLINK_PORT_FN_ATTR_CAPS + 1];
    pub static devlink_dl_rate_tc_bws_nl_policy:
        [nla_policy; DEVLINK_RATE_TC_ATTR_BW + 1];
    pub static devlink_dl_selftest_id_nl_policy:
        [nla_policy; DEVLINK_ATTR_SELFTEST_ID_FLASH + 1];

    pub static devlink_nl_ops: [genl_split_ops; 75];

    pub fn devlink_nl_pre_doit(
        ops: *const genl_split_ops,
        skb: *mut sk_buff,
        info: *mut genl_info,
    ) -> c_int;
    pub fn devlink_nl_pre_doit_port(
        ops: *const genl_split_ops,
        skb: *mut sk_buff,
        info: *mut genl_info,
    ) -> c_int;
    pub fn devlink_nl_pre_doit_port_optional(
        ops: *const genl_split_ops,
        skb: *mut sk_buff,
        info: *mut genl_info,
    ) -> c_int;
    pub fn devlink_nl_pre_doit_dev_lock(
        ops: *const genl_split_ops,
        skb: *mut sk_buff,
        info: *mut genl_info,
    ) -> c_int;
    pub fn devlink_nl_pre_doit_parent_dev_optional(
        ops: *const genl_split_ops,
        skb: *mut sk_buff,
        info: *mut genl_info,
    ) -> c_int;
    pub fn devlink_nl_post_doit(
        ops: *const genl_split_ops,
        skb: *mut sk_buff,
        info: *mut genl_info,
    );
    pub fn devlink_nl_post_doit_dev_lock(
        ops: *const genl_split_ops,
        skb: *mut sk_buff,
        info: *mut genl_info,
    );
    pub fn devlink_nl_post_doit_parent_dev_optional(
        ops: *const genl_split_ops,
        skb: *mut sk_buff,
        info: *mut genl_info,
    );
}

macro_rules! devlink_doit {
    ($($name:ident),* $(,)?) => {
        extern "C" {
            $(pub fn $name(skb: *mut sk_buff, info: *mut genl_info) -> c_int;)*
        }
    };
}

macro_rules! devlink_dumpit {
    ($($name:ident),* $(,)?) => {
        extern "C" {
            $(pub fn $name(skb: *mut sk_buff, cb: *mut netlink_callback) -> c_int;)*
        }
    };
}

devlink_doit!(
    devlink_nl_get_doit, devlink_nl_port_get_doit, devlink_nl_port_set_doit,
    devlink_nl_port_new_doit, devlink_nl_port_del_doit, devlink_nl_port_split_doit,
    devlink_nl_port_unsplit_doit, devlink_nl_sb_get_doit, devlink_nl_sb_pool_get_doit,
    devlink_nl_sb_pool_set_doit, devlink_nl_sb_port_pool_get_doit,
    devlink_nl_sb_port_pool_set_doit, devlink_nl_sb_tc_pool_bind_get_doit,
    devlink_nl_sb_tc_pool_bind_set_doit, devlink_nl_sb_occ_snapshot_doit,
    devlink_nl_sb_occ_max_clear_doit, devlink_nl_eswitch_get_doit,
    devlink_nl_eswitch_set_doit, devlink_nl_dpipe_table_get_doit,
    devlink_nl_dpipe_entries_get_doit, devlink_nl_dpipe_headers_get_doit,
    devlink_nl_dpipe_table_counters_set_doit, devlink_nl_resource_set_doit,
    devlink_nl_resource_dump_doit, devlink_nl_reload_doit, devlink_nl_param_get_doit,
    devlink_nl_param_set_doit, devlink_nl_region_get_doit, devlink_nl_region_new_doit,
    devlink_nl_region_del_doit, devlink_nl_port_param_get_doit,
    devlink_nl_port_param_set_doit, devlink_nl_info_get_doit,
    devlink_nl_health_reporter_get_doit, devlink_nl_health_reporter_set_doit,
    devlink_nl_health_reporter_recover_doit, devlink_nl_health_reporter_diagnose_doit,
    devlink_nl_health_reporter_dump_clear_doit, devlink_nl_flash_update_doit,
    devlink_nl_trap_get_doit, devlink_nl_trap_set_doit, devlink_nl_trap_group_get_doit,
    devlink_nl_trap_group_set_doit, devlink_nl_trap_policer_get_doit,
    devlink_nl_trap_policer_set_doit, devlink_nl_health_reporter_test_doit,
    devlink_nl_rate_get_doit, devlink_nl_rate_set_doit, devlink_nl_rate_new_doit,
    devlink_nl_rate_del_doit, devlink_nl_linecard_get_doit, devlink_nl_linecard_set_doit,
    devlink_nl_selftests_get_doit, devlink_nl_selftests_run_doit,
    devlink_nl_notify_filter_set_doit,
);

devlink_dumpit!(
    devlink_nl_get_dumpit, devlink_nl_port_get_dumpit, devlink_nl_sb_get_dumpit,
    devlink_nl_sb_pool_get_dumpit, devlink_nl_sb_port_pool_get_dumpit,
    devlink_nl_sb_tc_pool_bind_get_dumpit, devlink_nl_resource_dump_dumpit,
    devlink_nl_param_get_dumpit, devlink_nl_region_get_dumpit,
    devlink_nl_region_read_dumpit, devlink_nl_port_param_get_dumpit,
    devlink_nl_info_get_dumpit, devlink_nl_health_reporter_get_dumpit,
    devlink_nl_health_reporter_dump_get_dumpit, devlink_nl_trap_get_dumpit,
    devlink_nl_trap_group_get_dumpit, devlink_nl_trap_policer_get_dumpit,
    devlink_nl_rate_get_dumpit, devlink_nl_linecard_get_dumpit,
    devlink_nl_selftests_get_dumpit,
);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
