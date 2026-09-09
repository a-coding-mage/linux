/* SPDX-License-Identifier: ((GPL-2.0 WITH Linux-syscall-note) OR BSD-3-Clause) */
/* Do not edit directly, auto-generated from: */
/*	Documentation/netlink/specs/mptcp_pm.yaml */
/* YNL-GEN uapi header */
/* To regenerate run: tools/net/ynl/ynl-regen.sh */

pub const MPTCP_PM_NAME: &str = "mptcp_pm";
pub const MPTCP_PM_VER: i32 = 1;

/**
 * enum mptcp_event_type - Netlink MPTCP event types
 * @MPTCP_EVENT_UNSPEC: unused event
 * @MPTCP_EVENT_CREATED: A new MPTCP connection has been created. It is the
 *   good time to allocate memory and send ADD_ADDR if needed. Depending on
 *   the traffic-patterns it can take a long time until the MPTCP_EVENT_ESTABLISHED
 *   is sent. Attributes: token, family, saddr4 | saddr6, daddr4 | daddr6,
 *   sport, dport, [server-side], [flags].
 * @MPTCP_EVENT_ESTABLISHED: A MPTCP connection is established (can start new
 *   subflows). Attributes: token, family, saddr4 | saddr6, daddr4 | daddr6,
 *   sport, dport, [server-side], [flags].
 * @MPTCP_EVENT_CLOSED: A MPTCP connection has stopped. Attribute: token.
 * @MPTCP_EVENT_ANNOUNCED: A new address has been announced by the peer.
 *   Attributes: token, rem_id, family, daddr4 | daddr6 [, dport].
 * @MPTCP_EVENT_REMOVED: An address has been lost by the peer. Attributes:
 *   token, rem_id.
 * @MPTCP_EVENT_SUB_ESTABLISHED: A new subflow has been established. 'error'
 *   should not be set. Attributes: token, family, loc_id, rem_id, saddr4 |
 *   saddr6, daddr4 | daddr6, sport, dport, backup, if-idx [, error].
 * @MPTCP_EVENT_SUB_CLOSED: A subflow has been closed. An error (copy of
 *   sk_err) could be set if an error has been detected for this subflow.
 *   Attributes: token, family, loc_id, rem_id, saddr4 | saddr6, daddr4 |
 *   daddr6, sport, dport, backup, if-idx [, error].
 * @MPTCP_EVENT_SUB_PRIORITY: The priority of a subflow has changed. 'error'
 *   should not be set. Attributes: token, family, loc_id, rem_id, saddr4 |
 *   saddr6, daddr4 | daddr6, sport, dport, backup, if-idx [, error].
 * @MPTCP_EVENT_LISTENER_CREATED: A new PM listener is created. Attributes:
 *   family, sport, saddr4 | saddr6.
 * @MPTCP_EVENT_LISTENER_CLOSED: A PM listener is closed. Attributes: family,
 *   sport, saddr4 | saddr6.
 */
#[repr(i32)]
pub enum MptcpEventType {
    MPTCP_EVENT_UNSPEC = 0,
    MPTCP_EVENT_CREATED = 1,
    MPTCP_EVENT_ESTABLISHED = 2,
    MPTCP_EVENT_CLOSED = 3,
    MPTCP_EVENT_ANNOUNCED = 6,
    MPTCP_EVENT_REMOVED = 7,
    MPTCP_EVENT_SUB_ESTABLISHED = 10,
    MPTCP_EVENT_SUB_CLOSED = 11,
    MPTCP_EVENT_SUB_PRIORITY = 13,
    MPTCP_EVENT_LISTENER_CREATED = 15,
    MPTCP_EVENT_LISTENER_CLOSED = 16,
}

pub const MPTCP_PM_ADDR_ATTR_UNSPEC: i32 = 0;
pub const MPTCP_PM_ADDR_ATTR_FAMILY: i32 = 1;
pub const MPTCP_PM_ADDR_ATTR_ID: i32 = 2;
pub const MPTCP_PM_ADDR_ATTR_ADDR4: i32 = 3;
pub const MPTCP_PM_ADDR_ATTR_ADDR6: i32 = 4;
pub const MPTCP_PM_ADDR_ATTR_PORT: i32 = 5;
pub const MPTCP_PM_ADDR_ATTR_FLAGS: i32 = 6;
pub const MPTCP_PM_ADDR_ATTR_IF_IDX: i32 = 7;
pub const __MPTCP_PM_ADDR_ATTR_MAX: i32 = 8;
pub const MPTCP_PM_ADDR_ATTR_MAX: i32 = __MPTCP_PM_ADDR_ATTR_MAX - 1;

pub const MPTCP_SUBFLOW_ATTR_UNSPEC: i32 = 0;
pub const MPTCP_SUBFLOW_ATTR_TOKEN_REM: i32 = 1;
pub const MPTCP_SUBFLOW_ATTR_TOKEN_LOC: i32 = 2;
pub const MPTCP_SUBFLOW_ATTR_RELWRITE_SEQ: i32 = 3;
pub const MPTCP_SUBFLOW_ATTR_MAP_SEQ: i32 = 4;
pub const MPTCP_SUBFLOW_ATTR_MAP_SFSEQ: i32 = 5;
pub const MPTCP_SUBFLOW_ATTR_SSN_OFFSET: i32 = 6;
pub const MPTCP_SUBFLOW_ATTR_MAP_DATALEN: i32 = 7;
pub const MPTCP_SUBFLOW_ATTR_FLAGS: i32 = 8;
pub const MPTCP_SUBFLOW_ATTR_ID_REM: i32 = 9;
pub const MPTCP_SUBFLOW_ATTR_ID_LOC: i32 = 10;
pub const MPTCP_SUBFLOW_ATTR_PAD: i32 = 11;
pub const __MPTCP_SUBFLOW_ATTR_MAX: i32 = 12;
pub const MPTCP_SUBFLOW_ATTR_MAX: i32 = __MPTCP_SUBFLOW_ATTR_MAX - 1;

pub const MPTCP_PM_ENDPOINT_ADDR: i32 = 1;
pub const __MPTCP_PM_ENDPOINT_MAX: i32 = 2;
pub const MPTCP_PM_ENDPOINT_MAX: i32 = __MPTCP_PM_ENDPOINT_MAX - 1;

pub const MPTCP_PM_ATTR_UNSPEC: i32 = 0;
pub const MPTCP_PM_ATTR_ADDR: i32 = 1;
pub const MPTCP_PM_ATTR_RCV_ADD_ADDRS: i32 = 2;
pub const MPTCP_PM_ATTR_SUBFLOWS: i32 = 3;
pub const MPTCP_PM_ATTR_TOKEN: i32 = 4;
pub const MPTCP_PM_ATTR_LOC_ID: i32 = 5;
pub const MPTCP_PM_ATTR_ADDR_REMOTE: i32 = 6;
pub const __MPTCP_ATTR_AFTER_LAST: i32 = 7;
pub const MPTCP_PM_ATTR_MAX: i32 = __MPTCP_ATTR_AFTER_LAST - 1;

#[repr(i32)]
pub enum MptcpEventAttr {
    MPTCP_ATTR_UNSPEC = 0,
    MPTCP_ATTR_TOKEN = 1,
    MPTCP_ATTR_FAMILY = 2,
    MPTCP_ATTR_LOC_ID = 3,
    MPTCP_ATTR_REM_ID = 4,
    MPTCP_ATTR_SADDR4 = 5,
    MPTCP_ATTR_SADDR6 = 6,
    MPTCP_ATTR_DADDR4 = 7,
    MPTCP_ATTR_DADDR6 = 8,
    MPTCP_ATTR_SPORT = 9,
    MPTCP_ATTR_DPORT = 10,
    MPTCP_ATTR_BACKUP = 11,
    MPTCP_ATTR_ERROR = 12,
    MPTCP_ATTR_FLAGS = 13,
    MPTCP_ATTR_TIMEOUT = 14,
    MPTCP_ATTR_IF_IDX = 15,
    MPTCP_ATTR_RESET_REASON = 16,
    MPTCP_ATTR_RESET_FLAGS = 17,
    MPTCP_ATTR_SERVER_SIDE = 18,
    __MPTCP_ATTR_MAX = 19,
}
pub const MPTCP_ATTR_MAX: i32 = MptcpEventAttr::__MPTCP_ATTR_MAX as i32 - 1;

pub const MPTCP_PM_CMD_UNSPEC: i32 = 0;
pub const MPTCP_PM_CMD_ADD_ADDR: i32 = 1;
pub const MPTCP_PM_CMD_DEL_ADDR: i32 = 2;
pub const MPTCP_PM_CMD_GET_ADDR: i32 = 3;
pub const MPTCP_PM_CMD_FLUSH_ADDRS: i32 = 4;
pub const MPTCP_PM_CMD_SET_LIMITS: i32 = 5;
pub const MPTCP_PM_CMD_GET_LIMITS: i32 = 6;
pub const MPTCP_PM_CMD_SET_FLAGS: i32 = 7;
pub const MPTCP_PM_CMD_ANNOUNCE: i32 = 8;
pub const MPTCP_PM_CMD_REMOVE: i32 = 9;
pub const MPTCP_PM_CMD_SUBFLOW_CREATE: i32 = 10;
pub const MPTCP_PM_CMD_SUBFLOW_DESTROY: i32 = 11;
pub const __MPTCP_PM_CMD_AFTER_LAST: i32 = 12;
pub const MPTCP_PM_CMD_MAX: i32 = __MPTCP_PM_CMD_AFTER_LAST - 1;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
