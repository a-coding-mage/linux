/* SPDX-License-Identifier: ((GPL-2.0 WITH Linux-syscall-note) OR BSD-3-Clause) */
/* Do not edit directly, auto-generated from: */
/*	Documentation/netlink/specs/psp.yaml */
/* YNL-GEN uapi header */
/* To regenerate run: tools/net/ynl/ynl-regen.sh */

pub const PSP_FAMILY_NAME: &str = "psp";
pub const PSP_FAMILY_VERSION: i32 = 1;

#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum PspVersion {
    PSP_VERSION_HDR0_AES_GCM_128 = 0,
    PSP_VERSION_HDR0_AES_GCM_256 = 1,
    PSP_VERSION_HDR0_AES_GMAC_128 = 2,
    PSP_VERSION_HDR0_AES_GMAC_256 = 3,
}

pub const PSP_A_ASSOC_DEV_INFO_IFINDEX: i32 = 1;
pub const PSP_A_ASSOC_DEV_INFO_NSID: i32 = 2;
pub const __PSP_A_ASSOC_DEV_INFO_MAX: i32 = 3;
pub const PSP_A_ASSOC_DEV_INFO_MAX: i32 = __PSP_A_ASSOC_DEV_INFO_MAX - 1;

pub const PSP_A_DEV_ID: i32 = 1;
pub const PSP_A_DEV_IFINDEX: i32 = 2;
pub const PSP_A_DEV_PSP_VERSIONS_CAP: i32 = 3;
pub const PSP_A_DEV_PSP_VERSIONS_ENA: i32 = 4;
pub const PSP_A_DEV_ASSOC_LIST: i32 = 5;
pub const PSP_A_DEV_NSID: i32 = 6;
pub const PSP_A_DEV_BY_ASSOCIATION: i32 = 7;
pub const __PSP_A_DEV_MAX: i32 = 8;
pub const PSP_A_DEV_MAX: i32 = __PSP_A_DEV_MAX - 1;

pub const PSP_A_ASSOC_DEV_ID: i32 = 1;
pub const PSP_A_ASSOC_VERSION: i32 = 2;
pub const PSP_A_ASSOC_RX_KEY: i32 = 3;
pub const PSP_A_ASSOC_TX_KEY: i32 = 4;
pub const PSP_A_ASSOC_SOCK_FD: i32 = 5;
pub const __PSP_A_ASSOC_MAX: i32 = 6;
pub const PSP_A_ASSOC_MAX: i32 = __PSP_A_ASSOC_MAX - 1;

pub const PSP_A_KEYS_KEY: i32 = 1;
pub const PSP_A_KEYS_SPI: i32 = 2;
pub const __PSP_A_KEYS_MAX: i32 = 3;
pub const PSP_A_KEYS_MAX: i32 = __PSP_A_KEYS_MAX - 1;

pub const PSP_A_STATS_DEV_ID: i32 = 1;
pub const PSP_A_STATS_KEY_ROTATIONS: i32 = 2;
pub const PSP_A_STATS_STALE_EVENTS: i32 = 3;
pub const PSP_A_STATS_RX_PACKETS: i32 = 4;
pub const PSP_A_STATS_RX_BYTES: i32 = 5;
pub const PSP_A_STATS_RX_AUTH_FAIL: i32 = 6;
pub const PSP_A_STATS_RX_ERROR: i32 = 7;
pub const PSP_A_STATS_RX_BAD: i32 = 8;
pub const PSP_A_STATS_TX_PACKETS: i32 = 9;
pub const PSP_A_STATS_TX_BYTES: i32 = 10;
pub const PSP_A_STATS_TX_ERROR: i32 = 11;
pub const __PSP_A_STATS_MAX: i32 = 12;
pub const PSP_A_STATS_MAX: i32 = __PSP_A_STATS_MAX - 1;

pub const PSP_CMD_DEV_GET: i32 = 1;
pub const PSP_CMD_DEV_ADD_NTF: i32 = 2;
pub const PSP_CMD_DEV_DEL_NTF: i32 = 3;
pub const PSP_CMD_DEV_SET: i32 = 4;
pub const PSP_CMD_DEV_CHANGE_NTF: i32 = 5;
pub const PSP_CMD_KEY_ROTATE: i32 = 6;
pub const PSP_CMD_KEY_ROTATE_NTF: i32 = 7;
pub const PSP_CMD_RX_ASSOC: i32 = 8;
pub const PSP_CMD_TX_ASSOC: i32 = 9;
pub const PSP_CMD_GET_STATS: i32 = 10;
pub const PSP_CMD_DEV_ASSOC: i32 = 11;
pub const PSP_CMD_DEV_DISASSOC: i32 = 12;
pub const __PSP_CMD_MAX: i32 = 13;
pub const PSP_CMD_MAX: i32 = __PSP_CMD_MAX - 1;

pub const PSP_MCGRP_MGMT: &str = "mgmt";
pub const PSP_MCGRP_USE: &str = "use";

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
