/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
/*
 * Copyright (C) 2017 Facebook.  All rights reserved.
 *
 * This program is free software; you can redistribute it and/or
 * modify it under the terms of the GNU General Public
 * License as published by the Free Software Foundation.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
 * General Public License for more details.
 *
 * You should have received a copy of the GNU General Public
 * License along with this program; if not, write to the
 * Free Software Foundation, Inc., 59 Temple Place - Suite 330,
 * Boston, MA 021110-1307, USA.
 */

pub const NBD_GENL_FAMILY_NAME: &str = "nbd";
pub const NBD_GENL_VERSION: i32 = 0x1;
pub const NBD_GENL_MCAST_GROUP_NAME: &str = "nbd_mc_group";

/* Configuration policy attributes, used for CONNECT */
pub const NBD_ATTR_UNSPEC: i32 = 0;
pub const NBD_ATTR_INDEX: i32 = 1;
pub const NBD_ATTR_SIZE_BYTES: i32 = 2;
pub const NBD_ATTR_BLOCK_SIZE_BYTES: i32 = 3;
pub const NBD_ATTR_TIMEOUT: i32 = 4;
pub const NBD_ATTR_SERVER_FLAGS: i32 = 5;
pub const NBD_ATTR_CLIENT_FLAGS: i32 = 6;
pub const NBD_ATTR_SOCKETS: i32 = 7;
pub const NBD_ATTR_DEAD_CONN_TIMEOUT: i32 = 8;
pub const NBD_ATTR_DEVICE_LIST: i32 = 9;
pub const NBD_ATTR_BACKEND_IDENTIFIER: i32 = 10;
pub const __NBD_ATTR_MAX: i32 = 11;
pub const NBD_ATTR_MAX: i32 = __NBD_ATTR_MAX - 1;

/*
 * This is the format for multiple devices with NBD_ATTR_DEVICE_LIST
 *
 * [NBD_ATTR_DEVICE_LIST]
 *   [NBD_DEVICE_ITEM]
 *     [NBD_DEVICE_INDEX]
 *     [NBD_DEVICE_CONNECTED]
 */
pub const NBD_DEVICE_ITEM_UNSPEC: i32 = 0;
pub const NBD_DEVICE_ITEM: i32 = 1;
pub const __NBD_DEVICE_ITEM_MAX: i32 = 2;
pub const NBD_DEVICE_ITEM_MAX: i32 = __NBD_DEVICE_ITEM_MAX - 1;

pub const NBD_DEVICE_UNSPEC: i32 = 0;
pub const NBD_DEVICE_INDEX: i32 = 1;
pub const NBD_DEVICE_CONNECTED: i32 = 2;
pub const __NBD_DEVICE_MAX: i32 = 3;
pub const NBD_DEVICE_ATTR_MAX: i32 = __NBD_DEVICE_MAX - 1;

/*
 * This is the format for multiple sockets with NBD_ATTR_SOCKETS
 *
 * [NBD_ATTR_SOCKETS]
 *   [NBD_SOCK_ITEM]
 *     [NBD_SOCK_FD]
 *   [NBD_SOCK_ITEM]
 *     [NBD_SOCK_FD]
 */
pub const NBD_SOCK_ITEM_UNSPEC: i32 = 0;
pub const NBD_SOCK_ITEM: i32 = 1;
pub const __NBD_SOCK_ITEM_MAX: i32 = 2;
pub const NBD_SOCK_ITEM_MAX: i32 = __NBD_SOCK_ITEM_MAX - 1;

pub const NBD_SOCK_UNSPEC: i32 = 0;
pub const NBD_SOCK_FD: i32 = 1;
pub const __NBD_SOCK_MAX: i32 = 2;
pub const NBD_SOCK_MAX: i32 = __NBD_SOCK_MAX - 1;

pub const NBD_CMD_UNSPEC: i32 = 0;
pub const NBD_CMD_CONNECT: i32 = 1;
pub const NBD_CMD_DISCONNECT: i32 = 2;
pub const NBD_CMD_RECONFIGURE: i32 = 3;
pub const NBD_CMD_LINK_DEAD: i32 = 4;
pub const NBD_CMD_STATUS: i32 = 5;
pub const __NBD_CMD_MAX: i32 = 6;
pub const NBD_CMD_MAX: i32 = __NBD_CMD_MAX - 1;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
