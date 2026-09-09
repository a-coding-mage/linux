/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 *   Copyright (C) 2018 Samsung Electronics Co., Ltd.
 *
 *   linux-ksmbd-devel@lists.sourceforge.net
 */

/* Userspace ABI between ksmbd and the user IPC daemon using netlink. */

pub const KSMBD_GENL_NAME: &str = "SMBD_GENL";
pub const KSMBD_GENL_VERSION: u32 = 0x01;
pub const KSMBD_REQ_MAX_ACCOUNT_NAME_SZ: usize = 48;
pub const KSMBD_REQ_MAX_HASH_SZ: usize = 18;
pub const KSMBD_REQ_MAX_SHARE_NAME: usize = 64;

#[repr(C)]
pub struct ksmbd_heartbeat { pub handle: u32 }

pub const KSMBD_GLOBAL_FLAG_INVALID: u32 = 0;
pub const KSMBD_GLOBAL_FLAG_SMB2_LEASES: u32 = 1 << 0;
pub const KSMBD_GLOBAL_FLAG_SMB2_ENCRYPTION: u32 = 1 << 1;
pub const KSMBD_GLOBAL_FLAG_SMB3_MULTICHANNEL: u32 = 1 << 2;
pub const KSMBD_GLOBAL_FLAG_SMB2_ENCRYPTION_OFF: u32 = 1 << 3;
pub const KSMBD_GLOBAL_FLAG_DURABLE_HANDLE: u32 = 1 << 4;

#[repr(C, packed)]
pub struct ksmbd_startup_request {
    pub flags: u32, pub signing: i32, pub min_prot: [i8; 16], pub max_prot: [i8; 16],
    pub netbios_name: [i8; 16], pub work_group: [i8; 64], pub server_string: [i8; 64],
    pub tcp_port: u16, pub ipc_timeout: u16, pub deadtime: u32, pub file_max: u32,
    pub smb2_max_write: u32, pub smb2_max_read: u32, pub smb2_max_trans: u32,
    pub share_fake_fscaps: u32, pub sub_auth: [u32; 3], pub smb2_max_credits: u32,
    pub smbd_max_io_size: u32, pub max_connections: u32, pub bind_interfaces_only: i8,
    pub max_ip_connections: u32, pub aapl_model: [i8; 32], pub reserved: [i8; 467],
    pub ifc_list_sz: u32, pub ____payload: [i8; 0],
}
pub const KSMBD_STARTUP_CONFIG_INTERFACES_OFFSET: usize = 0; // ((s)->____payload)

#[repr(C)] pub struct ksmbd_shutdown_request { pub reserved: [i32; 16] }
#[repr(C)] pub struct ksmbd_login_request { pub handle: u32, pub account: [i8; 48], pub reserved: [u32; 16] }
#[repr(C)] pub struct ksmbd_login_response {
    pub handle: u32, pub gid: u32, pub uid: u32, pub account: [i8; 48], pub status: u16,
    pub hash_sz: u16, pub hash: [i8; 18], pub reserved: [u32; 16],
}
#[repr(C)] pub struct ksmbd_login_response_ext { pub handle: u32, pub ngroups: i32, pub reserved: [i8; 128], pub ____payload: [i8; 0] }
#[repr(C)] pub struct ksmbd_share_config_request { pub handle: u32, pub share_name: [i8; 64], pub reserved: [u32; 16] }
#[repr(C)] pub struct ksmbd_share_config_response {
    pub handle: u32, pub flags: u32, pub create_mask: u16, pub directory_mask: u16,
    pub force_create_mode: u16, pub force_directory_mode: u16, pub force_uid: u16, pub force_gid: u16,
    pub share_name: [i8; 64], pub reserved: [u32; 111], pub payload_sz: u32, pub veto_list_sz: u32,
    pub ____payload: [i8; 0],
}

#[inline] pub unsafe fn ksmbd_share_config_path(sc: *mut ksmbd_share_config_response) -> *mut i8 {
    let mut p = (*sc).____payload.as_mut_ptr();
    if (*sc).veto_list_sz != 0 { p = p.add((*sc).veto_list_sz as usize + 1); }
    p
}

#[repr(C)] pub struct ksmbd_tree_connect_request {
    pub handle: u32, pub account_flags: u16, pub flags: u16, pub session_id: u64, pub connect_id: u64,
    pub account: [i8; 48], pub share: [i8; 64], pub peer_addr: [i8; 64], pub reserved: [u32; 16],
}
#[repr(C)] pub struct ksmbd_tree_connect_response { pub handle: u32, pub status: u16, pub connection_flags: u16, pub reserved: [u32; 16] }
#[repr(C)] pub struct ksmbd_tree_disconnect_request { pub session_id: u64, pub connect_id: u64, pub reserved: [u32; 16] }
#[repr(C)] pub struct ksmbd_logout_request { pub account: [i8; 48], pub account_flags: u32, pub reserved: [u32; 16] }
#[repr(C)] pub struct ksmbd_rpc_command { pub handle: u32, pub flags: u32, pub payload_sz: u32, pub payload: [u8; 0] }
#[repr(C)] pub struct ksmbd_spnego_authen_request { pub handle: u32, pub spnego_blob_len: u16, pub spnego_blob: [u8; 0] }
#[repr(C)] pub struct ksmbd_spnego_authen_response {
    pub handle: u32, pub login_response: ksmbd_login_response, pub session_key_len: u16,
    pub spnego_blob_len: u16, pub session_expiry: u64, pub payload: [u8; 0],
}

#[repr(i32)] pub enum ksmbd_event {
    KSMBD_EVENT_UNSPEC = 0, KSMBD_EVENT_HEARTBEAT_REQUEST, KSMBD_EVENT_STARTING_UP,
    KSMBD_EVENT_SHUTTING_DOWN, KSMBD_EVENT_LOGIN_REQUEST, KSMBD_EVENT_LOGIN_RESPONSE = 5,
    KSMBD_EVENT_SHARE_CONFIG_REQUEST, KSMBD_EVENT_SHARE_CONFIG_RESPONSE,
    KSMBD_EVENT_TREE_CONNECT_REQUEST, KSMBD_EVENT_TREE_CONNECT_RESPONSE,
    KSMBD_EVENT_TREE_DISCONNECT_REQUEST = 10, KSMBD_EVENT_LOGOUT_REQUEST,
    KSMBD_EVENT_RPC_REQUEST, KSMBD_EVENT_RPC_RESPONSE, KSMBD_EVENT_SPNEGO_AUTHEN_REQUEST,
    KSMBD_EVENT_SPNEGO_AUTHEN_RESPONSE = 15, KSMBD_EVENT_LOGIN_REQUEST_EXT,
    KSMBD_EVENT_LOGIN_RESPONSE_EXT, __KSMBD_EVENT_MAX, KSMBD_EVENT_MAX = __KSMBD_EVENT_MAX - 1,
}
#[repr(i32)] pub enum KSMBD_TREE_CONN_STATUS { KSMBD_TREE_CONN_STATUS_OK=0, KSMBD_TREE_CONN_STATUS_NOMEM, KSMBD_TREE_CONN_STATUS_NO_SHARE, KSMBD_TREE_CONN_STATUS_NO_USER, KSMBD_TREE_CONN_STATUS_INVALID_USER, KSMBD_TREE_CONN_STATUS_HOST_DENIED=5, KSMBD_TREE_CONN_STATUS_CONN_EXIST, KSMBD_TREE_CONN_STATUS_TOO_MANY_CONNS, KSMBD_TREE_CONN_STATUS_TOO_MANY_SESSIONS, KSMBD_TREE_CONN_STATUS_ERROR }

pub const KSMBD_USER_FLAG_INVALID:u32=0; pub const KSMBD_USER_FLAG_OK:u32=1<<0; pub const KSMBD_USER_FLAG_BAD_PASSWORD:u32=1<<1; pub const KSMBD_USER_FLAG_BAD_UID:u32=1<<2; pub const KSMBD_USER_FLAG_BAD_USER:u32=1<<3; pub const KSMBD_USER_FLAG_GUEST_ACCOUNT:u32=1<<4; pub const KSMBD_USER_FLAG_DELAY_SESSION:u32=1<<5; pub const KSMBD_USER_FLAG_EXTENSION:u32=1<<6;
pub const KSMBD_SHARE_FLAG_INVALID:u32=0;
pub const KSMBD_SHARE_FLAG_AVAILABLE:u32=1<<0; pub const KSMBD_SHARE_FLAG_BROWSEABLE:u32=1<<1; pub const KSMBD_SHARE_FLAG_WRITEABLE:u32=1<<2; pub const KSMBD_SHARE_FLAG_READONLY:u32=1<<3; pub const KSMBD_SHARE_FLAG_GUEST_OK:u32=1<<4; pub const KSMBD_SHARE_FLAG_GUEST_ONLY:u32=1<<5; pub const KSMBD_SHARE_FLAG_STORE_DOS_ATTRS:u32=1<<6; pub const KSMBD_SHARE_FLAG_OPLOCKS:u32=1<<7; pub const KSMBD_SHARE_FLAG_PIPE:u32=1<<8; pub const KSMBD_SHARE_FLAG_HIDE_DOT_FILES:u32=1<<9; pub const KSMBD_SHARE_FLAG_INHERIT_OWNER:u32=1<<10; pub const KSMBD_SHARE_FLAG_STREAMS:u32=1<<11; pub const KSMBD_SHARE_FLAG_FOLLOW_SYMLINKS:u32=1<<12; pub const KSMBD_SHARE_FLAG_ACL_XATTR:u32=1<<13; pub const KSMBD_SHARE_FLAG_UPDATE:u32=1<<14; pub const KSMBD_SHARE_FLAG_CROSSMNT:u32=1<<15; pub const KSMBD_SHARE_FLAG_CONTINUOUS_AVAILABILITY:u32=1<<16; pub const KSMBD_SHARE_FLAG_HIDE_UNREADABLE:u32=1<<17; pub const KSMBD_SHARE_FLAG_TIME_MACHINE:u32=1<<18; pub const KSMBD_SHARE_FLAG_ENCRYPT_DATA:u32=1<<20;
pub const KSMBD_TREE_CONN_FLAG_REQUEST_SMB1:u32=0; pub const KSMBD_TREE_CONN_FLAG_REQUEST_IPV6:u32=1<<0; pub const KSMBD_TREE_CONN_FLAG_REQUEST_SMB2:u32=1<<1;
pub const KSMBD_TREE_CONN_FLAG_GUEST_ACCOUNT:u32=1<<0; pub const KSMBD_TREE_CONN_FLAG_READ_ONLY:u32=1<<1; pub const KSMBD_TREE_CONN_FLAG_WRITABLE:u32=1<<2; pub const KSMBD_TREE_CONN_FLAG_ADMIN_ACCOUNT:u32=1<<3; pub const KSMBD_TREE_CONN_FLAG_UPDATE:u32=1<<4;
pub const KSMBD_RPC_METHOD_RETURN:u32=1<<0; pub const KSMBD_RPC_SRVSVC_METHOD_INVOKE:u32=1<<1; pub const KSMBD_RPC_SRVSVC_METHOD_RETURN:u32=(1<<1)|(1<<0); pub const KSMBD_RPC_WKSSVC_METHOD_INVOKE:u32=1<<2; pub const KSMBD_RPC_WKSSVC_METHOD_RETURN:u32=(1<<2)|(1<<0); pub const KSMBD_RPC_IOCTL_METHOD:u32=(1<<3)|(1<<0); pub const KSMBD_RPC_OPEN_METHOD:u32=1<<4; pub const KSMBD_RPC_WRITE_METHOD:u32=1<<5; pub const KSMBD_RPC_READ_METHOD:u32=(1<<6)|(1<<0); pub const KSMBD_RPC_CLOSE_METHOD:u32=1<<7; pub const KSMBD_RPC_RAP_METHOD:u32=(1<<8)|(1<<0); pub const KSMBD_RPC_RESTRICTED_CONTEXT:u32=1<<9; pub const KSMBD_RPC_SAMR_METHOD_INVOKE:u32=1<<10; pub const KSMBD_RPC_SAMR_METHOD_RETURN:u32=(1<<10)|(1<<0); pub const KSMBD_RPC_LSARPC_METHOD_INVOKE:u32=1<<11; pub const KSMBD_RPC_LSARPC_METHOD_RETURN:u32=(1<<11)|(1<<0);
pub const KSMBD_RPC_OK:u32=0; pub const KSMBD_RPC_EBAD_FUNC:u32=0x00000001; pub const KSMBD_RPC_EACCESS_DENIED:u32=0x00000005; pub const KSMBD_RPC_EBAD_FID:u32=0x00000006; pub const KSMBD_RPC_ENOMEM:u32=0x00000008; pub const KSMBD_RPC_EBAD_DATA:u32=0x0000000D; pub const KSMBD_RPC_ENOTIMPLEMENTED:u32=0x00000040; pub const KSMBD_RPC_EINVALID_PARAMETER:u32=0x00000057; pub const KSMBD_RPC_EMORE_DATA:u32=0x000000EA; pub const KSMBD_RPC_EINVALID_LEVEL:u32=0x0000007C; pub const KSMBD_RPC_SOME_NOT_MAPPED:u32=0x00000107;
pub const KSMBD_CONFIG_OPT_DISABLED:u32=0; pub const KSMBD_CONFIG_OPT_ENABLED:u32=1; pub const KSMBD_CONFIG_OPT_AUTO:u32=2; pub const KSMBD_CONFIG_OPT_MANDATORY:u32=3;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
