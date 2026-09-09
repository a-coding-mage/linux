/* SPDX-License-Identifier: ((GPL-2.0 WITH Linux-syscall-note) OR BSD-3-Clause) */
/* Do not edit directly, auto-generated from: */
/*	Documentation/netlink/specs/wireguard.yaml */
/* YNL-GEN uapi header */
/* To regenerate run: tools/net/ynl/ynl-regen.sh */

pub const WG_GENL_NAME: &str = "wireguard";
pub const WG_GENL_VERSION: i32 = 1;

pub const WG_KEY_LEN: usize = 32;

#[repr(i32)]
pub enum WgdeviceFlag {
    WGDEVICE_F_REPLACE_PEERS = 1,
}

#[repr(i32)]
pub enum WgpeerFlag {
    WGPEER_F_REMOVE_ME = 1,
    WGPEER_F_REPLACE_ALLOWEDIPS = 2,
    WGPEER_F_UPDATE_ONLY = 4,
}

#[repr(i32)]
pub enum WgallowedipFlag {
    WGALLOWEDIP_F_REMOVE_ME = 1,
}

#[repr(i32)]
pub enum WgdeviceAttribute {
    WGDEVICE_A_UNSPEC = 0,
    WGDEVICE_A_IFINDEX,
    WGDEVICE_A_IFNAME,
    WGDEVICE_A_PRIVATE_KEY,
    WGDEVICE_A_PUBLIC_KEY,
    WGDEVICE_A_FLAGS,
    WGDEVICE_A_LISTEN_PORT,
    WGDEVICE_A_FWMARK,
    WGDEVICE_A_PEERS,
    __WGDEVICE_A_LAST,
}
pub const WGDEVICE_A_MAX: i32 = WgdeviceAttribute::__WGDEVICE_A_LAST as i32 - 1;

#[repr(i32)]
pub enum WgpeerAttribute {
    WGPEER_A_UNSPEC = 0,
    WGPEER_A_PUBLIC_KEY,
    WGPEER_A_PRESHARED_KEY,
    WGPEER_A_FLAGS,
    WGPEER_A_ENDPOINT,
    WGPEER_A_PERSISTENT_KEEPALIVE_INTERVAL,
    WGPEER_A_LAST_HANDSHAKE_TIME,
    WGPEER_A_RX_BYTES,
    WGPEER_A_TX_BYTES,
    WGPEER_A_ALLOWEDIPS,
    WGPEER_A_PROTOCOL_VERSION,
    WGPEER_A_LAST,
}
pub const WGPEER_A_MAX: i32 = WgpeerAttribute::WGPEER_A_LAST as i32 - 1;

#[repr(i32)]
pub enum WgallowedipAttribute {
    WGALLOWEDIP_A_UNSPEC = 0,
    WGALLOWEDIP_A_FAMILY,
    WGALLOWEDIP_A_IPADDR,
    WGALLOWEDIP_A_CIDR_MASK,
    WGALLOWEDIP_A_FLAGS,
    __WGALLOWEDIP_A_LAST,
}
pub const WGALLOWEDIP_A_MAX: i32 = WgallowedipAttribute::__WGALLOWEDIP_A_LAST as i32 - 1;

#[repr(i32)]
pub enum WgCmd {
    WG_CMD_GET_DEVICE = 0,
    WG_CMD_SET_DEVICE,
    __WG_CMD_MAX,
}
pub const WG_CMD_MAX: i32 = WgCmd::__WG_CMD_MAX as i32 - 1;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
