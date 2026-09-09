/* SPDX-License-Identifier: GPL-2.0-only WITH Linux-syscall-note */
/*
 * Copyright (c) 2021 Taehee Yoo <ap420073@gmail.com>
 */

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum ifla_amt_mode {
    /* AMT interface works as Gateway mode.
     * The Gateway mode encapsulates IGMP/MLD traffic and decapsulates
     * multicast traffic.
     */
    AMT_MODE_GATEWAY = 0,
    /* AMT interface works as Relay mode.
     * The Relay mode encapsulates multicast traffic and decapsulates
     * IGMP/MLD traffic.
     */
    AMT_MODE_RELAY,
    __AMT_MODE_MAX,
}

pub const AMT_MODE_MAX: i32 = (__AMT_MODE_MAX as i32) - 1;

pub const IFLA_AMT_UNSPEC: i32 = 0;
/* This attribute specify mode etier Gateway or Relay. */
pub const IFLA_AMT_MODE: i32 = 1;
/* This attribute specify Relay port.
 * AMT interface is created as Gateway mode, this attribute is used
 * to specify relay(remote) port.
 * AMT interface is created as Relay mode, this attribute is used
 * as local port.
 */
pub const IFLA_AMT_RELAY_PORT: i32 = 2;
/* This attribute specify Gateway port.
 * AMT interface is created as Gateway mode, this attribute is used
 * as local port.
 * AMT interface is created as Relay mode, this attribute is not used.
 */
pub const IFLA_AMT_GATEWAY_PORT: i32 = 3;
/* This attribute specify physical device */
pub const IFLA_AMT_LINK: i32 = 4;
/* This attribute specify local ip address */
pub const IFLA_AMT_LOCAL_IP: i32 = 5;
/* This attribute specify Relay ip address.
 * So, this is not used by Relay.
 */
pub const IFLA_AMT_REMOTE_IP: i32 = 6;
/* This attribute specify Discovery ip address.
 * When Gateway get started, it send discovery message to find the
 * Relay's ip address.
 * So, this is not used by Relay.
 */
pub const IFLA_AMT_DISCOVERY_IP: i32 = 7;
/* This attribute specify number of maximum tunnel. */
pub const IFLA_AMT_MAX_TUNNELS: i32 = 8;
pub const __IFLA_AMT_MAX: i32 = 9;

pub const IFLA_AMT_MAX: i32 = __IFLA_AMT_MAX - 1;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
