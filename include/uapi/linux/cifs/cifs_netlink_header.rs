/* SPDX-License-Identifier: LGPL-2.1+ WITH Linux-syscall-note */
/*
 * Netlink routines for CIFS
 *
 * Copyright (c) 2020 Samuel Cabrero <scabrero@suse.de>
 */

// Translated from the C header; the original include guard is omitted.

pub const CIFS_GENL_NAME: &str = "cifs";
pub const CIFS_GENL_VERSION: u32 = 0x1;

pub const CIFS_GENL_MCGRP_SWN_NAME: &str = "cifs_mcgrp_swn";

#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum cifs_genl_multicast_groups {
    CIFS_GENL_MCGRP_SWN = 0,
}

#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum cifs_genl_attributes {
    CIFS_GENL_ATTR_UNSPEC = 0,
    CIFS_GENL_ATTR_SWN_REGISTRATION_ID,
    CIFS_GENL_ATTR_SWN_NET_NAME,
    CIFS_GENL_ATTR_SWN_SHARE_NAME,
    CIFS_GENL_ATTR_SWN_IP,
    CIFS_GENL_ATTR_SWN_NET_NAME_NOTIFY,
    CIFS_GENL_ATTR_SWN_SHARE_NAME_NOTIFY,
    CIFS_GENL_ATTR_SWN_IP_NOTIFY,
    CIFS_GENL_ATTR_SWN_KRB_AUTH,
    CIFS_GENL_ATTR_SWN_USER_NAME,
    CIFS_GENL_ATTR_SWN_PASSWORD,
    CIFS_GENL_ATTR_SWN_DOMAIN_NAME,
    CIFS_GENL_ATTR_SWN_NOTIFICATION_TYPE,
    CIFS_GENL_ATTR_SWN_RESOURCE_STATE,
    CIFS_GENL_ATTR_SWN_RESOURCE_NAME,
    __CIFS_GENL_ATTR_MAX,
}

pub const CIFS_GENL_ATTR_MAX: i32 = __CIFS_GENL_ATTR_MAX as i32 - 1;

#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum cifs_genl_commands {
    CIFS_GENL_CMD_UNSPEC = 0,
    CIFS_GENL_CMD_SWN_REGISTER,
    CIFS_GENL_CMD_SWN_UNREGISTER,
    CIFS_GENL_CMD_SWN_NOTIFY,
    __CIFS_GENL_CMD_MAX,
}

pub const CIFS_GENL_CMD_MAX: i32 = __CIFS_GENL_CMD_MAX as i32 - 1;

#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum cifs_swn_notification_type {
    CIFS_SWN_NOTIFICATION_RESOURCE_CHANGE = 0x01,
    CIFS_SWN_NOTIFICATION_CLIENT_MOVE = 0x02,
    CIFS_SWN_NOTIFICATION_SHARE_MOVE = 0x03,
    CIFS_SWN_NOTIFICATION_IP_CHANGE = 0x04,
}

#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum cifs_swn_resource_state {
    CIFS_SWN_RESOURCE_STATE_UNKNOWN = 0x00,
    CIFS_SWN_RESOURCE_STATE_AVAILABLE = 0x01,
    CIFS_SWN_RESOURCE_STATE_UNAVAILABLE = 0xFF,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
