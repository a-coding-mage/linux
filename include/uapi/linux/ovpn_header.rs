/* SPDX-License-Identifier: ((GPL-2.0 WITH Linux-syscall-note) OR BSD-3-Clause) */
/* Do not edit directly, auto-generated from: */
/* Documentation/netlink/specs/ovpn.yaml */
/* YNL-GEN uapi header */
/* To regenerate run: tools/net/ynl/ynl-regen.sh */

pub const OVPN_FAMILY_NAME: &str = "ovpn";
pub const OVPN_FAMILY_VERSION: i32 = 1;

pub const OVPN_NONCE_TAIL_SIZE: usize = 8;

#[repr(i32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ovpn_cipher_alg {
    OVPN_CIPHER_ALG_NONE = 0,
    OVPN_CIPHER_ALG_AES_GCM = 1,
    OVPN_CIPHER_ALG_CHACHA20_POLY1305 = 2,
}

#[repr(i32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ovpn_del_peer_reason {
    OVPN_DEL_PEER_REASON_TEARDOWN = 0,
    OVPN_DEL_PEER_REASON_USERSPACE = 1,
    OVPN_DEL_PEER_REASON_EXPIRED = 2,
    OVPN_DEL_PEER_REASON_TRANSPORT_ERROR = 3,
    OVPN_DEL_PEER_REASON_TRANSPORT_DISCONNECT = 4,
}

#[repr(i32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ovpn_key_slot {
    OVPN_KEY_SLOT_PRIMARY = 0,
    OVPN_KEY_SLOT_SECONDARY = 1,
}

pub const OVPN_A_PEER_ID: i32 = 1;
pub const OVPN_A_PEER_REMOTE_IPV4: i32 = 2;
pub const OVPN_A_PEER_REMOTE_IPV6: i32 = 3;
pub const OVPN_A_PEER_REMOTE_IPV6_SCOPE_ID: i32 = 4;
pub const OVPN_A_PEER_REMOTE_PORT: i32 = 5;
pub const OVPN_A_PEER_SOCKET: i32 = 6;
pub const OVPN_A_PEER_SOCKET_NETNSID: i32 = 7;
pub const OVPN_A_PEER_VPN_IPV4: i32 = 8;
pub const OVPN_A_PEER_VPN_IPV6: i32 = 9;
pub const OVPN_A_PEER_LOCAL_IPV4: i32 = 10;
pub const OVPN_A_PEER_LOCAL_IPV6: i32 = 11;
pub const OVPN_A_PEER_LOCAL_PORT: i32 = 12;
pub const OVPN_A_PEER_KEEPALIVE_INTERVAL: i32 = 13;
pub const OVPN_A_PEER_KEEPALIVE_TIMEOUT: i32 = 14;
pub const OVPN_A_PEER_DEL_REASON: i32 = 15;
pub const OVPN_A_PEER_VPN_RX_BYTES: i32 = 16;
pub const OVPN_A_PEER_VPN_TX_BYTES: i32 = 17;
pub const OVPN_A_PEER_VPN_RX_PACKETS: i32 = 18;
pub const OVPN_A_PEER_VPN_TX_PACKETS: i32 = 19;
pub const OVPN_A_PEER_LINK_RX_BYTES: i32 = 20;
pub const OVPN_A_PEER_LINK_TX_BYTES: i32 = 21;
pub const OVPN_A_PEER_LINK_RX_PACKETS: i32 = 22;
pub const OVPN_A_PEER_LINK_TX_PACKETS: i32 = 23;
pub const OVPN_A_PEER_TX_ID: i32 = 24;
pub const __OVPN_A_PEER_MAX: i32 = 25;
pub const OVPN_A_PEER_MAX: i32 = __OVPN_A_PEER_MAX - 1;

pub const OVPN_A_KEYCONF_PEER_ID: i32 = 1;
pub const OVPN_A_KEYCONF_SLOT: i32 = 2;
pub const OVPN_A_KEYCONF_KEY_ID: i32 = 3;
pub const OVPN_A_KEYCONF_CIPHER_ALG: i32 = 4;
pub const OVPN_A_KEYCONF_ENCRYPT_DIR: i32 = 5;
pub const OVPN_A_KEYCONF_DECRYPT_DIR: i32 = 6;
pub const __OVPN_A_KEYCONF_MAX: i32 = 7;
pub const OVPN_A_KEYCONF_MAX: i32 = __OVPN_A_KEYCONF_MAX - 1;

pub const OVPN_A_KEYDIR_CIPHER_KEY: i32 = 1;
pub const OVPN_A_KEYDIR_NONCE_TAIL: i32 = 2;
pub const __OVPN_A_KEYDIR_MAX: i32 = 3;
pub const OVPN_A_KEYDIR_MAX: i32 = __OVPN_A_KEYDIR_MAX - 1;

pub const OVPN_A_IFINDEX: i32 = 1;
pub const OVPN_A_PEER: i32 = 2;
pub const OVPN_A_KEYCONF: i32 = 3;
pub const __OVPN_A_MAX: i32 = 4;
pub const OVPN_A_MAX: i32 = __OVPN_A_MAX - 1;

pub const OVPN_CMD_PEER_NEW: i32 = 1;
pub const OVPN_CMD_PEER_SET: i32 = 2;
pub const OVPN_CMD_PEER_GET: i32 = 3;
pub const OVPN_CMD_PEER_DEL: i32 = 4;
pub const OVPN_CMD_PEER_DEL_NTF: i32 = 5;
pub const OVPN_CMD_KEY_NEW: i32 = 6;
pub const OVPN_CMD_KEY_GET: i32 = 7;
pub const OVPN_CMD_KEY_SWAP: i32 = 8;
pub const OVPN_CMD_KEY_SWAP_NTF: i32 = 9;
pub const OVPN_CMD_KEY_DEL: i32 = 10;
pub const OVPN_CMD_PEER_FLOAT_NTF: i32 = 11;
pub const __OVPN_CMD_MAX: i32 = 12;
pub const OVPN_CMD_MAX: i32 = __OVPN_CMD_MAX - 1;

pub const OVPN_MCGRP_PEERS: &str = "peers";

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
