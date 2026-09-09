/*
 * Copyright Samuel Mendoza-Jonas, IBM Corporation 2018.
 *
 * This program is free software; you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation; either version 2 of the License, or
 * (at your option) any later version.
 */

/**
 * enum ncsi_nl_commands - supported NCSI commands
 *
 * @NCSI_CMD_UNSPEC: unspecified command to catch errors
 * @NCSI_CMD_PKG_INFO: list package and channel attributes. Requires
 *\tNCSI_ATTR_IFINDEX. If NCSI_ATTR_PACKAGE_ID is specified returns the
 *\tspecific package and its channels - otherwise a dump request returns
 *\tall packages and their associated channels.
 * @NCSI_CMD_SET_INTERFACE: set preferred package and channel combination.
 *\tRequires NCSI_ATTR_IFINDEX and the preferred NCSI_ATTR_PACKAGE_ID and
 *\toptionally the preferred NCSI_ATTR_CHANNEL_ID.
 * @NCSI_CMD_CLEAR_INTERFACE: clear any preferred package/channel combination.
 *\tRequires NCSI_ATTR_IFINDEX.
 * @NCSI_CMD_SEND_CMD: send NC-SI command to network card.
 *\tRequires NCSI_ATTR_IFINDEX, NCSI_ATTR_PACKAGE_ID
 *\tand NCSI_ATTR_CHANNEL_ID.
 * @NCSI_CMD_SET_PACKAGE_MASK: set a whitelist of allowed packages.
 *\tRequires NCSI_ATTR_IFINDEX and NCSI_ATTR_PACKAGE_MASK.
 * @NCSI_CMD_SET_CHANNEL_MASK: set a whitelist of allowed channels.
 *\tRequires NCSI_ATTR_IFINDEX, NCSI_ATTR_PACKAGE_ID, and
 *\tNCSI_ATTR_CHANNEL_MASK. If NCSI_ATTR_CHANNEL_ID is present it sets
 *\tthe primary channel.
 * @NCSI_CMD_MAX: highest command number
 */
#[repr(i32)]
pub enum NcsiNlCommands {
    NCSI_CMD_UNSPEC = 0,
    NCSI_CMD_PKG_INFO = 1,
    NCSI_CMD_SET_INTERFACE = 2,
    NCSI_CMD_CLEAR_INTERFACE = 3,
    NCSI_CMD_SEND_CMD = 4,
    NCSI_CMD_SET_PACKAGE_MASK = 5,
    NCSI_CMD_SET_CHANNEL_MASK = 6,
    __NCSI_CMD_AFTER_LAST = 7,
}
pub const NCSI_CMD_MAX: i32 = __NCSI_CMD_AFTER_LAST as i32 - 1;

/** General NCSI netlink attributes. */
#[repr(i32)]
pub enum NcsiNlAttrs {
    NCSI_ATTR_UNSPEC = 0,
    NCSI_ATTR_IFINDEX = 1,
    NCSI_ATTR_PACKAGE_LIST = 2,
    NCSI_ATTR_PACKAGE_ID = 3,
    NCSI_ATTR_CHANNEL_ID = 4,
    NCSI_ATTR_DATA = 5,
    NCSI_ATTR_MULTI_FLAG = 6,
    NCSI_ATTR_PACKAGE_MASK = 7,
    NCSI_ATTR_CHANNEL_MASK = 8,
    __NCSI_ATTR_AFTER_LAST = 9,
}
pub const NCSI_ATTR_MAX: i32 = __NCSI_ATTR_AFTER_LAST as i32 - 1;

/** NCSI netlink package-specific attributes. */
#[repr(i32)]
pub enum NcsiNlPkgAttrs {
    NCSI_PKG_ATTR_UNSPEC = 0,
    NCSI_PKG_ATTR = 1,
    NCSI_PKG_ATTR_ID = 2,
    NCSI_PKG_ATTR_FORCED = 3,
    NCSI_PKG_ATTR_CHANNEL_LIST = 4,
    __NCSI_PKG_ATTR_AFTER_LAST = 5,
}
pub const NCSI_PKG_ATTR_MAX: i32 = __NCSI_PKG_ATTR_AFTER_LAST as i32 - 1;

/** NCSI netlink channel-specific attributes. */
#[repr(i32)]
pub enum NcsiNlChannelAttrs {
    NCSI_CHANNEL_ATTR_UNSPEC = 0,
    NCSI_CHANNEL_ATTR = 1,
    NCSI_CHANNEL_ATTR_ID = 2,
    NCSI_CHANNEL_ATTR_VERSION_MAJOR = 3,
    NCSI_CHANNEL_ATTR_VERSION_MINOR = 4,
    NCSI_CHANNEL_ATTR_VERSION_STR = 5,
    NCSI_CHANNEL_ATTR_LINK_STATE = 6,
    NCSI_CHANNEL_ATTR_ACTIVE = 7,
    NCSI_CHANNEL_ATTR_FORCED = 8,
    NCSI_CHANNEL_ATTR_VLAN_LIST = 9,
    NCSI_CHANNEL_ATTR_VLAN_ID = 10,
    __NCSI_CHANNEL_ATTR_AFTER_LAST = 11,
}
pub const NCSI_CHANNEL_ATTR_MAX: i32 = __NCSI_CHANNEL_ATTR_AFTER_LAST as i32 - 1;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
