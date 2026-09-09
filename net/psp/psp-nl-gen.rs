// SPDX-License-Identifier: ((GPL-2.0 WITH Linux-syscall-note) OR BSD-3-Clause)
/* Do not edit directly, auto-generated from:
 * Documentation/netlink/specs/psp.yaml
 * YNL-GEN kernel source
 * To regenerate run: tools/net/ynl/ynl-regen.sh
 */

// Dependencies supplied by the kernel headers and psp-nl-gen.h are intentionally
// left external; this file is the direct Rust representation of the generated C.

#[allow(non_camel_case_types, non_upper_case_globals, dead_code)]
use crate::*;

pub static mut psp_keys_nl_policy: [nla_policy; PSP_A_KEYS_SPI as usize + 1] = [
    /* PSP_A_KEYS_KEY */ nla_policy { nla_type: NLA_BINARY },
    /* intervening attributes */ nla_policy { nla_type: 0 },
    /* PSP_A_KEYS_SPI */ nla_policy { nla_type: NLA_U32 },
];

/* PSP_CMD_DEV_GET - do */
static mut psp_dev_get_nl_policy: [nla_policy; PSP_A_DEV_ID as usize + 1] =
    [nla_policy { nla_type: NLA_U32 | (1 << 16) }; PSP_A_DEV_ID as usize + 1];

/* PSP_CMD_DEV_SET - do */
static mut psp_dev_set_nl_policy: [nla_policy; PSP_A_DEV_PSP_VERSIONS_ENA as usize + 1] =
    [nla_policy { nla_type: NLA_U32 | (1 << 16) }; PSP_A_DEV_PSP_VERSIONS_ENA as usize + 1];

/* PSP_CMD_KEY_ROTATE - do */
static mut psp_key_rotate_nl_policy: [nla_policy; PSP_A_DEV_ID as usize + 1] =
    [nla_policy { nla_type: NLA_U32 | (1 << 16) }; PSP_A_DEV_ID as usize + 1];

/* PSP_CMD_RX_ASSOC - do */
static mut psp_rx_assoc_nl_policy: [nla_policy; PSP_A_ASSOC_SOCK_FD as usize + 1] =
    [nla_policy { nla_type: 0 }; PSP_A_ASSOC_SOCK_FD as usize + 1];

/* PSP_CMD_TX_ASSOC - do */
static mut psp_tx_assoc_nl_policy: [nla_policy; PSP_A_ASSOC_SOCK_FD as usize + 1] =
    [nla_policy { nla_type: 0 }; PSP_A_ASSOC_SOCK_FD as usize + 1];

/* PSP_CMD_GET_STATS - do */
static mut psp_get_stats_nl_policy: [nla_policy; PSP_A_STATS_DEV_ID as usize + 1] =
    [nla_policy { nla_type: NLA_U32 | (1 << 16) }; PSP_A_STATS_DEV_ID as usize + 1];

/* PSP_CMD_DEV_ASSOC - do */
static mut psp_dev_assoc_nl_policy: [nla_policy; PSP_A_DEV_NSID as usize + 1] =
    [nla_policy { nla_type: 0 }; PSP_A_DEV_NSID as usize + 1];

/* PSP_CMD_DEV_DISASSOC - do */
static mut psp_dev_disassoc_nl_policy: [nla_policy; PSP_A_DEV_NSID as usize + 1] =
    [nla_policy { nla_type: 0 }; PSP_A_DEV_NSID as usize + 1];

/* Ops table for psp */
static psp_nl_ops: [genl_split_ops; 10] = [
    genl_split_ops { cmd: PSP_CMD_DEV_GET, pre_doit: Some(psp_device_get_locked), doit: Some(psp_nl_dev_get_doit), post_doit: Some(psp_device_unlock), policy: Some(psp_dev_get_nl_policy.as_ptr()), maxattr: PSP_A_DEV_ID, flags: GENL_CMD_CAP_DO },
    genl_split_ops { cmd: PSP_CMD_DEV_GET, dumpit: Some(psp_nl_dev_get_dumpit), flags: GENL_CMD_CAP_DUMP, ..Default::default() },
    genl_split_ops { cmd: PSP_CMD_DEV_SET, pre_doit: Some(psp_device_get_locked_admin), doit: Some(psp_nl_dev_set_doit), post_doit: Some(psp_device_unlock), policy: Some(psp_dev_set_nl_policy.as_ptr()), maxattr: PSP_A_DEV_PSP_VERSIONS_ENA, flags: GENL_ADMIN_PERM | GENL_CMD_CAP_DO },
    genl_split_ops { cmd: PSP_CMD_KEY_ROTATE, pre_doit: Some(psp_device_get_locked_admin), doit: Some(psp_nl_key_rotate_doit), post_doit: Some(psp_device_unlock), policy: Some(psp_key_rotate_nl_policy.as_ptr()), maxattr: PSP_A_DEV_ID, flags: GENL_ADMIN_PERM | GENL_CMD_CAP_DO },
    genl_split_ops { cmd: PSP_CMD_RX_ASSOC, pre_doit: Some(psp_assoc_device_get_locked), doit: Some(psp_nl_rx_assoc_doit), post_doit: Some(psp_device_unlock), policy: Some(psp_rx_assoc_nl_policy.as_ptr()), maxattr: PSP_A_ASSOC_SOCK_FD, flags: GENL_CMD_CAP_DO },
    genl_split_ops { cmd: PSP_CMD_TX_ASSOC, pre_doit: Some(psp_assoc_device_get_locked), doit: Some(psp_nl_tx_assoc_doit), post_doit: Some(psp_device_unlock), policy: Some(psp_tx_assoc_nl_policy.as_ptr()), maxattr: PSP_A_ASSOC_SOCK_FD, flags: GENL_CMD_CAP_DO },
    genl_split_ops { cmd: PSP_CMD_GET_STATS, pre_doit: Some(psp_device_get_locked), doit: Some(psp_nl_get_stats_doit), post_doit: Some(psp_device_unlock), policy: Some(psp_get_stats_nl_policy.as_ptr()), maxattr: PSP_A_STATS_DEV_ID, flags: GENL_CMD_CAP_DO },
    genl_split_ops { cmd: PSP_CMD_GET_STATS, dumpit: Some(psp_nl_get_stats_dumpit), flags: GENL_CMD_CAP_DUMP, ..Default::default() },
    genl_split_ops { cmd: PSP_CMD_DEV_ASSOC, pre_doit: Some(psp_device_get_locked_dev_assoc), doit: Some(psp_nl_dev_assoc_doit), post_doit: Some(psp_device_unlock), policy: Some(psp_dev_assoc_nl_policy.as_ptr()), maxattr: PSP_A_DEV_NSID, flags: GENL_ADMIN_PERM | GENL_CMD_CAP_DO },
    genl_split_ops { cmd: PSP_CMD_DEV_DISASSOC, pre_doit: Some(psp_device_get_locked), doit: Some(psp_nl_dev_disassoc_doit), post_doit: Some(psp_device_unlock), policy: Some(psp_dev_disassoc_nl_policy.as_ptr()), maxattr: PSP_A_DEV_NSID, flags: GENL_ADMIN_PERM | GENL_CMD_CAP_DO },
];

static psp_nl_mcgrps: [genl_multicast_group; 2] = [
    genl_multicast_group { name: c"mgmt".as_ptr() },
    genl_multicast_group { name: c"use".as_ptr() },
];

#[no_mangle]
pub static mut psp_nl_family: genl_family = genl_family {
    name: PSP_FAMILY_NAME,
    version: PSP_FAMILY_VERSION,
    netnsok: true,
    parallel_ops: true,
    module: THIS_MODULE,
    split_ops: psp_nl_ops.as_ptr(),
    n_split_ops: psp_nl_ops.len(),
    mcgrps: psp_nl_mcgrps.as_ptr(),
    n_mcgrps: psp_nl_mcgrps.len(),
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
