/* SPDX-License-Identifier: GPL-2.0-or-later */
/* Rust translation of iscsi_if.h. */

pub const ISCSI_NL_GRP_ISCSID: u32 = 1;
pub const ISCSI_NL_GRP_UIP: u32 = 2;
pub const UEVENT_BASE: u32 = 10;
pub const KEVENT_BASE: u32 = 100;
pub const ISCSI_ERR_BASE: u32 = 1000;

#[repr(i32)]
#[derive(Copy, Clone)]
pub enum iscsi_uevent_e {
    ISCSI_UEVENT_UNKNOWN = 0,
    ISCSI_UEVENT_CREATE_SESSION = 11, ISCSI_UEVENT_DESTROY_SESSION, ISCSI_UEVENT_CREATE_CONN,
    ISCSI_UEVENT_DESTROY_CONN, ISCSI_UEVENT_BIND_CONN, ISCSI_UEVENT_SET_PARAM,
    ISCSI_UEVENT_START_CONN, ISCSI_UEVENT_STOP_CONN, ISCSI_UEVENT_SEND_PDU,
    ISCSI_UEVENT_GET_STATS, ISCSI_UEVENT_GET_PARAM, ISCSI_UEVENT_TRANSPORT_EP_CONNECT,
    ISCSI_UEVENT_TRANSPORT_EP_POLL, ISCSI_UEVENT_TRANSPORT_EP_DISCONNECT,
    ISCSI_UEVENT_TGT_DSCVR, ISCSI_UEVENT_SET_HOST_PARAM, ISCSI_UEVENT_UNBIND_SESSION,
    ISCSI_UEVENT_CREATE_BOUND_SESSION, ISCSI_UEVENT_TRANSPORT_EP_CONNECT_THROUGH_HOST,
    ISCSI_UEVENT_PATH_UPDATE, ISCSI_UEVENT_SET_IFACE_PARAMS, ISCSI_UEVENT_PING,
    ISCSI_UEVENT_GET_CHAP, ISCSI_UEVENT_DELETE_CHAP, ISCSI_UEVENT_SET_FLASHNODE_PARAMS,
    ISCSI_UEVENT_NEW_FLASHNODE, ISCSI_UEVENT_DEL_FLASHNODE, ISCSI_UEVENT_LOGIN_FLASHNODE,
    ISCSI_UEVENT_LOGOUT_FLASHNODE, ISCSI_UEVENT_LOGOUT_FLASHNODE_SID,
    ISCSI_UEVENT_SET_CHAP, ISCSI_UEVENT_GET_HOST_STATS, ISCSI_UEVENT_DESTROY_SESSION_ASYNC,
    ISCSI_KEVENT_RECV_PDU = 101, ISCSI_KEVENT_CONN_ERROR, ISCSI_KEVENT_IF_ERROR,
    ISCSI_KEVENT_DESTROY_SESSION, ISCSI_KEVENT_UNBIND_SESSION, ISCSI_KEVENT_CREATE_SESSION,
    ISCSI_KEVENT_PATH_REQ, ISCSI_KEVENT_IF_DOWN, ISCSI_KEVENT_CONN_LOGIN_STATE,
    ISCSI_KEVENT_HOST_EVENT, ISCSI_KEVENT_PING_COMP,
}
#[repr(i32)] #[derive(Copy, Clone)] pub enum iscsi_tgt_dscvr { ISCSI_TGT_DSCVR_SEND_TARGETS=1, ISCSI_TGT_DSCVR_ISNS, ISCSI_TGT_DSCVR_SLP }
#[repr(i32)] #[derive(Copy, Clone)] pub enum iscsi_host_event_code { ISCSI_EVENT_LINKUP=1, ISCSI_EVENT_LINKDOWN, ISCSI_EVENT_MAX }

#[repr(C)] #[derive(Copy, Clone)] pub struct msg_create_session { pub initial_cmdsn:u32, pub cmds_max:u16, pub queue_depth:u16 }
#[repr(C)] #[derive(Copy, Clone)] pub struct msg_create_bound_session { pub ep_handle:u64, pub initial_cmdsn:u32, pub cmds_max:u16, pub queue_depth:u16 }
#[repr(C)] #[derive(Copy, Clone)] pub struct msg_destroy_session { pub sid:u32 }
#[repr(C)] #[derive(Copy, Clone)] pub struct msg_create_conn { pub sid:u32, pub cid:u32 }
#[repr(C)] #[derive(Copy, Clone)] pub struct msg_destroy_conn { pub sid:u32, pub cid:u32 }
#[repr(C)] #[derive(Copy, Clone)] pub struct msg_bind_conn { pub sid:u32, pub cid:u32, pub transport_eph:u64, pub is_leading:u32 }
#[repr(C)] #[derive(Copy, Clone)] pub struct msg_send_pdu { pub sid:u32,pub cid:u32,pub hdr_size:u32,pub data_size:u32 }
#[repr(C)] #[derive(Copy, Clone)] pub struct msg_set_param { pub sid:u32,pub cid:u32,pub param:u32,pub len:u32 }
#[repr(C)] #[derive(Copy, Clone)] pub struct msg_stop_conn { pub sid:u32,pub cid:u32,pub conn_handle:u64,pub flag:u32 }
#[repr(C)] #[derive(Copy, Clone)] pub struct msg_start_conn { pub sid:u32, pub cid:u32 }
#[repr(C)] #[derive(Copy, Clone)] pub struct msg_get_stats { pub sid:u32, pub cid:u32 }
#[repr(C)] #[derive(Copy, Clone)] pub struct msg_transport_connect { pub non_blocking:u32 }
#[repr(C)] #[derive(Copy, Clone)] pub struct msg_transport_connect_through_host { pub host_no:u32,pub non_blocking:u32 }
#[repr(C)] #[derive(Copy, Clone)] pub struct msg_transport_poll { pub ep_handle:u64,pub timeout_ms:u32 }
#[repr(C)] #[derive(Copy, Clone)] pub struct msg_transport_disconnect { pub ep_handle:u64 }
#[repr(C)] #[derive(Copy, Clone)] pub struct msg_tgt_dscvr { pub r#type:iscsi_tgt_dscvr,pub host_no:u32,pub enable:u32 }
#[repr(C)] #[derive(Copy, Clone)] pub struct msg_set_host_param { pub host_no:u32,pub param:u32,pub len:u32 }
#[repr(C)] #[derive(Copy, Clone)] pub struct msg_set_path { pub host_no:u32 }
#[repr(C)] #[derive(Copy, Clone)] pub struct msg_set_iface_params { pub host_no:u32,pub count:u32 }
#[repr(C)] #[derive(Copy, Clone)] pub struct msg_iscsi_ping { pub host_no:u32,pub iface_num:u32,pub iface_type:u32,pub payload_size:u32,pub pid:u32 }
#[repr(C)] #[derive(Copy, Clone)] pub struct msg_get_chap { pub host_no:u32,pub num_entries:u32,pub chap_tbl_idx:u16 }
#[repr(C)] #[derive(Copy, Clone)] pub struct msg_delete_chap { pub host_no:u32,pub chap_tbl_idx:u16 }
#[repr(C)] #[derive(Copy, Clone)] pub struct msg_set_flashnode_param { pub host_no:u32,pub flashnode_idx:u32,pub count:u32 }
#[repr(C)] #[derive(Copy, Clone)] pub struct msg_new_flashnode { pub host_no:u32,pub len:u32 }
#[repr(C)] #[derive(Copy, Clone)] pub struct msg_del_flashnode { pub host_no:u32,pub flashnode_idx:u32 }
#[repr(C)] #[derive(Copy, Clone)] pub struct msg_login_flashnode { pub host_no:u32,pub flashnode_idx:u32 }
#[repr(C)] #[derive(Copy, Clone)] pub struct msg_logout_flashnode { pub host_no:u32,pub flashnode_idx:u32 }
#[repr(C)] #[derive(Copy, Clone)] pub struct msg_logout_flashnode_sid { pub host_no:u32,pub sid:u32 }
#[repr(C)] #[derive(Copy, Clone)] pub struct msg_get_host_stats { pub host_no:u32 }
#[repr(C)] pub union iscsi_uevent_u { pub c_session:msg_create_session,pub c_bound_session:msg_create_bound_session,pub d_session:msg_destroy_session,pub c_conn:msg_create_conn,pub b_conn:msg_bind_conn,pub d_conn:msg_destroy_conn,pub send_pdu:msg_send_pdu,pub set_param:msg_set_param,pub start_conn:msg_start_conn,pub stop_conn:msg_stop_conn,pub get_stats:msg_get_stats,pub ep_connect:msg_transport_connect,pub ep_connect_through_host:msg_transport_connect_through_host,pub ep_poll:msg_transport_poll,pub ep_disconnect:msg_transport_disconnect,pub tgt_dscvr:msg_tgt_dscvr,pub set_host_param:msg_set_host_param,pub set_path:msg_set_path,pub set_iface_params:msg_set_iface_params,pub iscsi_ping:msg_iscsi_ping,pub get_chap:msg_get_chap,pub delete_chap:msg_delete_chap,pub set_flashnode:msg_set_flashnode_param,pub new_flashnode:msg_new_flashnode,pub del_flashnode:msg_del_flashnode,pub login_flashnode:msg_login_flashnode,pub logout_flashnode:msg_logout_flashnode,pub logout_flashnode_sid:msg_logout_flashnode_sid,pub get_host_stats:msg_get_host_stats }
#[repr(C)] #[derive(Copy, Clone)] pub struct msg_create_session_ret { pub sid:u32,pub host_no:u32 }
#[repr(C)] #[derive(Copy, Clone)] pub struct msg_create_conn_ret { pub sid:u32,pub cid:u32 }
#[repr(C)] #[derive(Copy, Clone)] pub struct msg_unbind_session { pub sid:u32,pub host_no:u32 }
#[repr(C)] #[derive(Copy, Clone)] pub struct msg_recv_req { pub sid:u32,pub cid:u32,pub recv_handle:u64 }
#[repr(C)] #[derive(Copy, Clone)] pub struct msg_conn_login { pub sid:u32,pub cid:u32,pub state:u32 }
#[repr(C)] #[derive(Copy, Clone)] pub struct msg_conn_error { pub sid:u32,pub cid:u32,pub error:u32 }
#[repr(C)] #[derive(Copy, Clone)] pub struct msg_transport_connect_ret { pub handle:u64 }
#[repr(C)] #[derive(Copy, Clone)] pub struct msg_req_path { pub host_no:u32 }
#[repr(C)] #[derive(Copy, Clone)] pub struct msg_notify_if_down { pub host_no:u32 }
#[repr(C)] #[derive(Copy, Clone)] pub struct msg_host_event { pub host_no:u32,pub data_size:u32,pub code:iscsi_host_event_code }
#[repr(C)] #[derive(Copy, Clone)] pub struct msg_ping_comp { pub host_no:u32,pub status:u32,pub pid:u32,pub data_size:u32 }
#[repr(C)] #[derive(Copy, Clone)] pub struct msg_new_flashnode_ret { pub flashnode_idx:u32 }
#[repr(C)] pub union iscsi_uevent_r { pub retcode:i32,pub c_session_ret:msg_create_session_ret,pub c_conn_ret:msg_create_conn_ret,pub unbind_session:msg_unbind_session,pub recv_req:msg_recv_req,pub conn_login:msg_conn_login,pub connerror:msg_conn_error,pub ep_connect_ret:msg_transport_connect_ret,pub req_path:msg_req_path,pub notify_if_down:msg_notify_if_down,pub host_event:msg_host_event,pub ping_comp:msg_ping_comp,pub new_flashnode_ret:msg_new_flashnode_ret }
#[repr(C, align(8))] pub struct iscsi_uevent { pub r#type:u32,pub iferror:u32,pub transport_handle:u64,pub u:iscsi_uevent_u,pub r:iscsi_uevent_r }

#[repr(i32)] #[derive(Copy, Clone)] pub enum iscsi_param_type { ISCSI_PARAM, ISCSI_HOST_PARAM, ISCSI_NET_PARAM, ISCSI_FLASHNODE_PARAM, ISCSI_CHAP_PARAM, ISCSI_IFACE_PARAM }
#[repr(C, packed)] pub struct iscsi_param_info { pub len:u32,pub param:u16,pub value:[u8;0] }
#[repr(C, packed)] pub struct iscsi_iface_param_info { pub iface_num:u32,pub len:u32,pub param:u16,pub iface_type:u8,pub param_type:u8,pub value:[u8;0] }
#[repr(C)] pub union iscsi_path_src { pub v4_addr:crate::in_addr,pub v6_addr:crate::in6_addr }
#[repr(C)] pub union iscsi_path_dst { pub v4_addr:crate::in_addr,pub v6_addr:crate::in6_addr }
#[repr(C, align(8))] pub struct iscsi_path { pub handle:u64,pub mac_addr:[u8;6],pub mac_addr_old:[u8;6],pub ip_addr_len:u32,pub src:iscsi_path_src,pub dst:iscsi_path_dst,pub vlan_id:u16,pub pmtu:u16 }

pub const ISCSI_IFACE_DISABLE:u32=1; pub const ISCSI_IFACE_ENABLE:u32=2; pub const ISCSI_BOOTPROTO_STATIC:u32=1; pub const ISCSI_BOOTPROTO_DHCP:u32=2;
pub const ISCSI_IPV6_AUTOCFG_DISABLE:u32=1; pub const ISCSI_IPV6_AUTOCFG_ND_ENABLE:u32=2; pub const ISCSI_IPV6_AUTOCFG_DHCPV6_ENABLE:u32=3;
pub const ISCSI_IPV6_LINKLOCAL_AUTOCFG_ENABLE:u32=1; pub const ISCSI_IPV6_LINKLOCAL_AUTOCFG_DISABLE:u32=2; pub const ISCSI_IPV6_ROUTER_AUTOCFG_ENABLE:u32=1; pub const ISCSI_IPV6_ROUTER_AUTOCFG_DISABLE:u32=2;
pub const ISCSI_IFACE_TYPE_IPV4:u32=1; pub const ISCSI_IFACE_TYPE_IPV6:u32=2; pub const ISCSI_MAX_VLAN_ID:u32=4095; pub const ISCSI_MAX_VLAN_PRIORITY:u32=7;
pub const ISCSI_VLAN_DISABLE:u32=1; pub const ISCSI_VLAN_ENABLE:u32=2; pub const ISCSI_NET_PARAM_DISABLE:u32=1; pub const ISCSI_NET_PARAM_ENABLE:u32=2;

#[repr(i32)] #[derive(Copy, Clone)] pub enum iscsi_net_param { ISCSI_NET_PARAM_IPV4_ADDR=1, ISCSI_NET_PARAM_IPV4_SUBNET, ISCSI_NET_PARAM_IPV4_GW, ISCSI_NET_PARAM_IPV4_BOOTPROTO, ISCSI_NET_PARAM_MAC, ISCSI_NET_PARAM_IPV6_LINKLOCAL, ISCSI_NET_PARAM_IPV6_ADDR, ISCSI_NET_PARAM_IPV6_ROUTER, ISCSI_NET_PARAM_IPV6_ADDR_AUTOCFG, ISCSI_NET_PARAM_IPV6_LINKLOCAL_AUTOCFG, ISCSI_NET_PARAM_IPV6_ROUTER_AUTOCFG, ISCSI_NET_PARAM_IFACE_ENABLE, ISCSI_NET_PARAM_VLAN_ID, ISCSI_NET_PARAM_VLAN_PRIORITY, ISCSI_NET_PARAM_VLAN_ENABLED, ISCSI_NET_PARAM_VLAN_TAG, ISCSI_NET_PARAM_IFACE_TYPE, ISCSI_NET_PARAM_IFACE_NAME, ISCSI_NET_PARAM_MTU, ISCSI_NET_PARAM_PORT, ISCSI_NET_PARAM_IPADDR_STATE, ISCSI_NET_PARAM_IPV6_LINKLOCAL_STATE, ISCSI_NET_PARAM_IPV6_ROUTER_STATE, ISCSI_NET_PARAM_DELAYED_ACK_EN, ISCSI_NET_PARAM_TCP_NAGLE_DISABLE, ISCSI_NET_PARAM_TCP_WSF_DISABLE, ISCSI_NET_PARAM_TCP_WSF, ISCSI_NET_PARAM_TCP_TIMER_SCALE, ISCSI_NET_PARAM_TCP_TIMESTAMP_EN, ISCSI_NET_PARAM_CACHE_ID, ISCSI_NET_PARAM_IPV4_DHCP_DNS_ADDR_EN, ISCSI_NET_PARAM_IPV4_DHCP_SLP_DA_EN, ISCSI_NET_PARAM_IPV4_TOS_EN, ISCSI_NET_PARAM_IPV4_TOS, ISCSI_NET_PARAM_IPV4_GRAT_ARP_EN, ISCSI_NET_PARAM_IPV4_DHCP_ALT_CLIENT_ID_EN, ISCSI_NET_PARAM_IPV4_DHCP_ALT_CLIENT_ID, ISCSI_NET_PARAM_IPV4_DHCP_REQ_VENDOR_ID_EN, ISCSI_NET_PARAM_IPV4_DHCP_USE_VENDOR_ID_EN, ISCSI_NET_PARAM_IPV4_DHCP_VENDOR_ID, ISCSI_NET_PARAM_IPV4_DHCP_LEARN_IQN_EN, ISCSI_NET_PARAM_IPV4_FRAGMENT_DISABLE, ISCSI_NET_PARAM_IPV4_IN_FORWARD_EN, ISCSI_NET_PARAM_IPV4_TTL, ISCSI_NET_PARAM_IPV6_GRAT_NEIGHBOR_ADV_EN, ISCSI_NET_PARAM_IPV6_MLD_EN, ISCSI_NET_PARAM_IPV6_FLOW_LABEL, ISCSI_NET_PARAM_IPV6_TRAFFIC_CLASS, ISCSI_NET_PARAM_IPV6_HOP_LIMIT, ISCSI_NET_PARAM_IPV6_ND_REACHABLE_TMO, ISCSI_NET_PARAM_IPV6_ND_REXMIT_TIME, ISCSI_NET_PARAM_IPV6_ND_STALE_TMO, ISCSI_NET_PARAM_IPV6_DUP_ADDR_DETECT_CNT, ISCSI_NET_PARAM_IPV6_RTR_ADV_LINK_MTU, ISCSI_NET_PARAM_REDIRECT_EN }
#[repr(i32)] #[derive(Copy, Clone)] pub enum iscsi_ipaddress_state { ISCSI_IPDDRESS_STATE_UNCONFIGURED, ISCSI_IPDDRESS_STATE_ACQUIRING, ISCSI_IPDDRESS_STATE_TENTATIVE, ISCSI_IPDDRESS_STATE_VALID, ISCSI_IPDDRESS_STATE_DISABLING, ISCSI_IPDDRESS_STATE_INVALID, ISCSI_IPDDRESS_STATE_DEPRECATED }
#[repr(i32)] #[derive(Copy, Clone)] pub enum iscsi_router_state { ISCSI_ROUTER_STATE_UNKNOWN, ISCSI_ROUTER_STATE_ADVERTISED, ISCSI_ROUTER_STATE_MANUAL, ISCSI_ROUTER_STATE_STALE }
#[repr(i32)] #[derive(Copy, Clone)] pub enum iscsi_iface_param { ISCSI_IFACE_PARAM_DEF_TASKMGMT_TMO, ISCSI_IFACE_PARAM_HDRDGST_EN, ISCSI_IFACE_PARAM_DATADGST_EN, ISCSI_IFACE_PARAM_IMM_DATA_EN, ISCSI_IFACE_PARAM_INITIAL_R2T_EN, ISCSI_IFACE_PARAM_DATASEQ_INORDER_EN, ISCSI_IFACE_PARAM_PDU_INORDER_EN, ISCSI_IFACE_PARAM_ERL, ISCSI_IFACE_PARAM_MAX_RECV_DLENGTH, ISCSI_IFACE_PARAM_FIRST_BURST, ISCSI_IFACE_PARAM_MAX_R2T, ISCSI_IFACE_PARAM_MAX_BURST, ISCSI_IFACE_PARAM_CHAP_AUTH_EN, ISCSI_IFACE_PARAM_BIDI_CHAP_EN, ISCSI_IFACE_PARAM_DISCOVERY_AUTH_OPTIONAL, ISCSI_IFACE_PARAM_DISCOVERY_LOGOUT_EN, ISCSI_IFACE_PARAM_STRICT_LOGIN_COMP_EN, ISCSI_IFACE_PARAM_INITIATOR_NAME }
#[repr(i32)] #[derive(Copy, Clone)] pub enum iscsi_conn_state { ISCSI_CONN_STATE_FREE, ISCSI_CONN_STATE_XPT_WAIT, ISCSI_CONN_STATE_IN_LOGIN, ISCSI_CONN_STATE_LOGGED_IN, ISCSI_CONN_STATE_IN_LOGOUT, ISCSI_CONN_STATE_LOGOUT_REQUESTED, ISCSI_CONN_STATE_CLEANUP_WAIT }

#[repr(i32)] #[derive(Copy, Clone)] pub enum iscsi_err { ISCSI_OK=0, ISCSI_ERR_DATASN=1001, ISCSI_ERR_DATA_OFFSET, ISCSI_ERR_MAX_CMDSN, ISCSI_ERR_EXP_CMDSN, ISCSI_ERR_BAD_OPCODE, ISCSI_ERR_DATALEN, ISCSI_ERR_AHSLEN, ISCSI_ERR_PROTO, ISCSI_ERR_LUN, ISCSI_ERR_BAD_ITT, ISCSI_ERR_CONN_FAILED, ISCSI_ERR_R2TSN, ISCSI_ERR_SESSION_FAILED, ISCSI_ERR_HDR_DGST, ISCSI_ERR_DATA_DGST, ISCSI_ERR_PARAM_NOT_FOUND, ISCSI_ERR_NO_SCSI_CMD, ISCSI_ERR_INVALID_HOST, ISCSI_ERR_XMIT_FAILED, ISCSI_ERR_TCP_CONN_CLOSE, ISCSI_ERR_SCSI_EH_SESSION_RST, ISCSI_ERR_NOP_TIMEDOUT }
#[repr(i32)] #[derive(Copy, Clone)] pub enum iscsi_param { ISCSI_PARAM_MAX_RECV_DLENGTH, ISCSI_PARAM_MAX_XMIT_DLENGTH, ISCSI_PARAM_HDRDGST_EN, ISCSI_PARAM_DATADGST_EN, ISCSI_PARAM_INITIAL_R2T_EN, ISCSI_PARAM_MAX_R2T, ISCSI_PARAM_IMM_DATA_EN, ISCSI_PARAM_FIRST_BURST, ISCSI_PARAM_MAX_BURST, ISCSI_PARAM_PDU_INORDER_EN, ISCSI_PARAM_DATASEQ_INORDER_EN, ISCSI_PARAM_ERL, ISCSI_PARAM_IFMARKER_EN, ISCSI_PARAM_OFMARKER_EN, ISCSI_PARAM_EXP_STATSN, ISCSI_PARAM_TARGET_NAME, ISCSI_PARAM_TPGT, ISCSI_PARAM_PERSISTENT_ADDRESS, ISCSI_PARAM_PERSISTENT_PORT, ISCSI_PARAM_SESS_RECOVERY_TMO, ISCSI_PARAM_CONN_PORT, ISCSI_PARAM_CONN_ADDRESS, ISCSI_PARAM_USERNAME, ISCSI_PARAM_USERNAME_IN, ISCSI_PARAM_PASSWORD, ISCSI_PARAM_PASSWORD_IN, ISCSI_PARAM_FAST_ABORT, ISCSI_PARAM_ABORT_TMO, ISCSI_PARAM_LU_RESET_TMO, ISCSI_PARAM_HOST_RESET_TMO, ISCSI_PARAM_PING_TMO, ISCSI_PARAM_RECV_TMO, ISCSI_PARAM_IFACE_NAME, ISCSI_PARAM_ISID, ISCSI_PARAM_INITIATOR_NAME, ISCSI_PARAM_TGT_RESET_TMO, ISCSI_PARAM_TARGET_ALIAS, ISCSI_PARAM_CHAP_IN_IDX, ISCSI_PARAM_CHAP_OUT_IDX, ISCSI_PARAM_BOOT_ROOT, ISCSI_PARAM_BOOT_NIC, ISCSI_PARAM_BOOT_TARGET, ISCSI_PARAM_AUTO_SND_TGT_DISABLE, ISCSI_PARAM_DISCOVERY_SESS, ISCSI_PARAM_PORTAL_TYPE, ISCSI_PARAM_CHAP_AUTH_EN, ISCSI_PARAM_DISCOVERY_LOGOUT_EN, ISCSI_PARAM_BIDI_CHAP_EN, ISCSI_PARAM_DISCOVERY_AUTH_OPTIONAL, ISCSI_PARAM_DEF_TIME2WAIT, ISCSI_PARAM_DEF_TIME2RETAIN, ISCSI_PARAM_MAX_SEGMENT_SIZE, ISCSI_PARAM_STATSN, ISCSI_PARAM_KEEPALIVE_TMO, ISCSI_PARAM_LOCAL_PORT, ISCSI_PARAM_TSID, ISCSI_PARAM_DEF_TASKMGMT_TMO, ISCSI_PARAM_TCP_TIMESTAMP_STAT, ISCSI_PARAM_TCP_WSF_DISABLE, ISCSI_PARAM_TCP_NAGLE_DISABLE, ISCSI_PARAM_TCP_TIMER_SCALE, ISCSI_PARAM_TCP_TIMESTAMP_EN, ISCSI_PARAM_TCP_XMIT_WSF, ISCSI_PARAM_TCP_RECV_WSF, ISCSI_PARAM_IP_FRAGMENT_DISABLE, ISCSI_PARAM_IPV4_TOS, ISCSI_PARAM_IPV6_TC, ISCSI_PARAM_IPV6_FLOW_LABEL, ISCSI_PARAM_IS_FW_ASSIGNED_IPV6, ISCSI_PARAM_DISCOVERY_PARENT_IDX, ISCSI_PARAM_DISCOVERY_PARENT_TYPE, ISCSI_PARAM_LOCAL_IPADDR, ISCSI_PARAM_MAX }
#[repr(i32)] #[derive(Copy, Clone)] pub enum iscsi_host_param { ISCSI_HOST_PARAM_HWADDRESS, ISCSI_HOST_PARAM_INITIATOR_NAME, ISCSI_HOST_PARAM_NETDEV_NAME, ISCSI_HOST_PARAM_IPADDRESS, ISCSI_HOST_PARAM_PORT_STATE, ISCSI_HOST_PARAM_PORT_SPEED, ISCSI_HOST_PARAM_MAX }
pub const PORTAL_TYPE_IPV4:&str="ipv4"; pub const PORTAL_TYPE_IPV6:&str="ipv6";
#[repr(i32)] #[derive(Copy, Clone)] pub enum iscsi_flashnode_param { ISCSI_FLASHNODE_IS_FW_ASSIGNED_IPV6, ISCSI_FLASHNODE_PORTAL_TYPE, ISCSI_FLASHNODE_AUTO_SND_TGT_DISABLE, ISCSI_FLASHNODE_DISCOVERY_SESS, ISCSI_FLASHNODE_ENTRY_EN, ISCSI_FLASHNODE_HDR_DGST_EN, ISCSI_FLASHNODE_DATA_DGST_EN, ISCSI_FLASHNODE_IMM_DATA_EN, ISCSI_FLASHNODE_INITIAL_R2T_EN, ISCSI_FLASHNODE_DATASEQ_INORDER, ISCSI_FLASHNODE_PDU_INORDER, ISCSI_FLASHNODE_CHAP_AUTH_EN, ISCSI_FLASHNODE_SNACK_REQ_EN, ISCSI_FLASHNODE_DISCOVERY_LOGOUT_EN, ISCSI_FLASHNODE_BIDI_CHAP_EN, ISCSI_FLASHNODE_DISCOVERY_AUTH_OPTIONAL, ISCSI_FLASHNODE_ERL, ISCSI_FLASHNODE_TCP_TIMESTAMP_STAT, ISCSI_FLASHNODE_TCP_NAGLE_DISABLE, ISCSI_FLASHNODE_TCP_WSF_DISABLE, ISCSI_FLASHNODE_TCP_TIMER_SCALE, ISCSI_FLASHNODE_TCP_TIMESTAMP_EN, ISCSI_FLASHNODE_IP_FRAG_DISABLE, ISCSI_FLASHNODE_MAX_RECV_DLENGTH, ISCSI_FLASHNODE_MAX_XMIT_DLENGTH, ISCSI_FLASHNODE_FIRST_BURST, ISCSI_FLASHNODE_DEF_TIME2WAIT, ISCSI_FLASHNODE_DEF_TIME2RETAIN, ISCSI_FLASHNODE_MAX_R2T, ISCSI_FLASHNODE_KEEPALIVE_TMO, ISCSI_FLASHNODE_ISID, ISCSI_FLASHNODE_TSID, ISCSI_FLASHNODE_PORT, ISCSI_FLASHNODE_MAX_BURST, ISCSI_FLASHNODE_DEF_TASKMGMT_TMO, ISCSI_FLASHNODE_IPADDR, ISCSI_FLASHNODE_ALIAS, ISCSI_FLASHNODE_REDIRECT_IPADDR, ISCSI_FLASHNODE_MAX_SEGMENT_SIZE, ISCSI_FLASHNODE_LOCAL_PORT, ISCSI_FLASHNODE_IPV4_TOS, ISCSI_FLASHNODE_IPV6_TC, ISCSI_FLASHNODE_IPV6_FLOW_LABEL, ISCSI_FLASHNODE_NAME, ISCSI_FLASHNODE_TPGT, ISCSI_FLASHNODE_LINK_LOCAL_IPV6, ISCSI_FLASHNODE_DISCOVERY_PARENT_IDX, ISCSI_FLASHNODE_DISCOVERY_PARENT_TYPE, ISCSI_FLASHNODE_TCP_XMIT_WSF, ISCSI_FLASHNODE_TCP_RECV_WSF, ISCSI_FLASHNODE_CHAP_IN_IDX, ISCSI_FLASHNODE_CHAP_OUT_IDX, ISCSI_FLASHNODE_USERNAME, ISCSI_FLASHNODE_USERNAME_IN, ISCSI_FLASHNODE_PASSWORD, ISCSI_FLASHNODE_PASSWORD_IN, ISCSI_FLASHNODE_STATSN, ISCSI_FLASHNODE_EXP_STATSN, ISCSI_FLASHNODE_IS_BOOT_TGT, ISCSI_FLASHNODE_MAX }
#[repr(C, packed)] pub struct iscsi_flashnode_param_info { pub len:u32,pub param:u16,pub value:[u8;0] }
#[repr(i32)] #[derive(Copy, Clone)] pub enum iscsi_discovery_parent_type { ISCSI_DISC_PARENT_UNKNOWN=1, ISCSI_DISC_PARENT_SENDTGT, ISCSI_DISC_PARENT_ISNS }
#[repr(i32)] #[derive(Copy, Clone)] pub enum iscsi_port_speed { ISCSI_PORT_SPEED_UNKNOWN=1, ISCSI_PORT_SPEED_10MBPS=2, ISCSI_PORT_SPEED_100MBPS=4, ISCSI_PORT_SPEED_1GBPS=8, ISCSI_PORT_SPEED_10GBPS=0x10, ISCSI_PORT_SPEED_25GBPS=0x20, ISCSI_PORT_SPEED_40GBPS=0x40 }
#[repr(i32)] #[derive(Copy, Clone)] pub enum iscsi_port_state { ISCSI_PORT_STATE_DOWN=1, ISCSI_PORT_STATE_UP=2 }
#[repr(i32)] #[derive(Copy, Clone)] pub enum iscsi_ping_status_code { ISCSI_PING_SUCCESS=0, ISCSI_PING_FW_DISABLED=1, ISCSI_PING_IPADDR_INVALID, ISCSI_PING_LINKLOCAL_IPV6_ADDR_INVALID, ISCSI_PING_TIMEOUT, ISCSI_PING_INVALID_DEST_ADDR, ISCSI_PING_OVERSIZE_PACKET, ISCSI_PING_ICMP_ERROR, ISCSI_PING_MAX_REQ_EXCEEDED, ISCSI_PING_NO_ARP_RECEIVED }

#[inline] pub unsafe fn iscsi_ptr(handle:u64)->*mut core::ffi::c_void { handle as usize as *mut core::ffi::c_void }
#[inline] pub unsafe fn iscsi_handle(ptr:*const core::ffi::c_void)->u64 { ptr as usize as u64 }
pub const CAP_RECOVERY_L0:u32=1; pub const CAP_RECOVERY_L1:u32=2; pub const CAP_RECOVERY_L2:u32=4; pub const CAP_MULTI_R2T:u32=8; pub const CAP_HDRDGST:u32=0x10; pub const CAP_DATADGST:u32=0x20; pub const CAP_MULTI_CONN:u32=0x40; pub const CAP_TEXT_NEGO:u32=0x80; pub const CAP_MARKERS:u32=0x100; pub const CAP_FW_DB:u32=0x200; pub const CAP_SENDTARGETS_OFFLOAD:u32=0x400; pub const CAP_DATA_PATH_OFFLOAD:u32=0x800; pub const CAP_DIGEST_OFFLOAD:u32=0x1000; pub const CAP_PADDING_OFFLOAD:u32=0x2000; pub const CAP_LOGIN_OFFLOAD:u32=0x4000;
pub const STOP_CONN_TERM:u32=1; pub const STOP_CONN_SUSPEND:u32=2; pub const STOP_CONN_RECOVER:u32=3;
pub const ISCSI_STATS_CUSTOM_MAX:usize=32; pub const ISCSI_STATS_CUSTOM_DESC_MAX:usize=64;
#[repr(C)] pub struct iscsi_stats_custom { pub desc:[core::ffi::c_char;64], pub value:u64 }
#[repr(C)] pub struct iscsi_stats { pub txdata_octets:u64,pub rxdata_octets:u64,pub noptx_pdus:u32,pub scsicmd_pdus:u32,pub tmfcmd_pdus:u32,pub login_pdus:u32,pub text_pdus:u32,pub dataout_pdus:u32,pub logout_pdus:u32,pub snack_pdus:u32,pub noprx_pdus:u32,pub scsirsp_pdus:u32,pub tmfrsp_pdus:u32,pub textrsp_pdus:u32,pub datain_pdus:u32,pub logoutrsp_pdus:u32,pub r2t_pdus:u32,pub async_pdus:u32,pub rjt_pdus:u32,pub digest_err:u32,pub timeout_err:u32,pub custom_length:u32,pub custom:[iscsi_stats_custom;0] }
#[repr(i32)] #[derive(Copy, Clone)] pub enum chap_type_e { CHAP_TYPE_OUT, CHAP_TYPE_IN }
#[repr(i32)] #[derive(Copy, Clone)] pub enum iscsi_chap_param { ISCSI_CHAP_PARAM_INDEX, ISCSI_CHAP_PARAM_CHAP_TYPE, ISCSI_CHAP_PARAM_USERNAME, ISCSI_CHAP_PARAM_PASSWORD, ISCSI_CHAP_PARAM_PASSWORD_LEN }
pub const ISCSI_CHAP_AUTH_NAME_MAX_LEN:usize=256; pub const ISCSI_CHAP_AUTH_SECRET_MAX_LEN:usize=256;
#[repr(C)] pub struct iscsi_chap_rec { pub chap_tbl_idx:u16,pub chap_type:chap_type_e,pub username:[core::ffi::c_char;256],pub password:[u8;256],pub password_length:u8 }
pub const ISCSI_HOST_STATS_CUSTOM_MAX:usize=32; pub const ISCSI_HOST_STATS_CUSTOM_DESC_MAX:usize=64;
#[repr(C)] pub struct iscsi_host_stats_custom { pub desc:[core::ffi::c_char;64],pub value:u64 }
#[repr(C)] pub struct iscsi_offload_host_stats {
    pub mactx_frames:u64,pub mactx_bytes:u64,pub mactx_multicast_frames:u64,pub mactx_broadcast_frames:u64,pub mactx_pause_frames:u64,pub mactx_control_frames:u64,pub mactx_deferral:u64,pub mactx_excess_deferral:u64,pub mactx_late_collision:u64,pub mactx_abort:u64,pub mactx_single_collision:u64,pub mactx_multiple_collision:u64,pub mactx_collision:u64,pub mactx_frames_dropped:u64,pub mactx_jumbo_frames:u64,pub macrx_frames:u64,pub macrx_bytes:u64,pub macrx_unknown_control_frames:u64,pub macrx_pause_frames:u64,pub macrx_control_frames:u64,pub macrx_dribble:u64,pub macrx_frame_length_error:u64,pub macrx_jabber:u64,pub macrx_carrier_sense_error:u64,pub macrx_frame_discarded:u64,pub macrx_frames_dropped:u64,pub mac_crc_error:u64,pub mac_encoding_error:u64,pub macrx_length_error_large:u64,pub macrx_length_error_small:u64,pub macrx_multicast_frames:u64,pub macrx_broadcast_frames:u64,
    pub iptx_packets:u64,pub iptx_bytes:u64,pub iptx_fragments:u64,pub iprx_packets:u64,pub iprx_bytes:u64,pub iprx_fragments:u64,pub ip_datagram_reassembly:u64,pub ip_invalid_address_error:u64,pub ip_error_packets:u64,pub ip_fragrx_overlap:u64,pub ip_fragrx_outoforder:u64,pub ip_datagram_reassembly_timeout:u64,pub ipv6tx_packets:u64,pub ipv6tx_bytes:u64,pub ipv6tx_fragments:u64,pub ipv6rx_packets:u64,pub ipv6rx_bytes:u64,pub ipv6rx_fragments:u64,pub ipv6_datagram_reassembly:u64,pub ipv6_invalid_address_error:u64,pub ipv6_error_packets:u64,pub ipv6_fragrx_overlap:u64,pub ipv6_fragrx_outoforder:u64,pub ipv6_datagram_reassembly_timeout:u64,
    pub tcptx_segments:u64,pub tcptx_bytes:u64,pub tcprx_segments:u64,pub tcprx_byte:u64,pub tcp_duplicate_ack_retx:u64,pub tcp_retx_timer_expired:u64,pub tcprx_duplicate_ack:u64,pub tcprx_pure_ackr:u64,pub tcptx_delayed_ack:u64,pub tcptx_pure_ack:u64,pub tcprx_segment_error:u64,pub tcprx_segment_outoforder:u64,pub tcprx_window_probe:u64,pub tcprx_window_update:u64,pub tcptx_window_probe_persist:u64,
    pub ecc_error_correction:u64,pub iscsi_pdu_tx:u64,pub iscsi_data_bytes_tx:u64,pub iscsi_pdu_rx:u64,pub iscsi_data_bytes_rx:u64,pub iscsi_io_completed:u64,pub iscsi_unexpected_io_rx:u64,pub iscsi_format_error:u64,pub iscsi_hdr_digest_error:u64,pub iscsi_data_digest_error:u64,pub iscsi_sequence_error:u64,pub custom_length:u32,pub custom:[iscsi_host_stats_custom;0]
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
