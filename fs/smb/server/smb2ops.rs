// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *   Copyright (C) 2016 Namjae Jeon <linkinjeon@kernel.org>
 *   Copyright (C) 2018 Samsung Electronics Co., Ltd.
 */

// Linux and project headers from the C implementation provide the referenced
// types, constants, globals, and external functions.

static mut smb21_server_values: smb_version_values = smb_version_values {
    version_string: SMB21_VERSION_STRING,
    protocol_id: SMB21_PROT_ID,
    req_capabilities: SMB2_GLOBAL_CAP_LARGE_MTU,
    max_read_size: SMB21_DEFAULT_IOSIZE,
    max_write_size: SMB21_DEFAULT_IOSIZE,
    max_trans_size: SMB21_DEFAULT_IOSIZE,
    max_credits: SMB2_MAX_CREDITS,
    large_lock_type: 0,
    exclusive_lock_type: SMB2_LOCKFLAG_EXCLUSIVE,
    shared_lock_type: SMB2_LOCKFLAG_SHARED,
    unlock_lock_type: SMB2_LOCKFLAG_UNLOCK,
    header_size: core::mem::size_of::<smb2_hdr>(),
    max_header_size: MAX_SMB2_HDR_SIZE,
    read_rsp_size: core::mem::size_of::<smb2_read_rsp>(),
    lock_cmd: SMB2_LOCK,
    cap_unix: 0,
    cap_nt_find: SMB2_NT_FIND,
    cap_large_files: SMB2_LARGE_FILES,
    create_lease_size: core::mem::size_of::<create_lease>(),
    create_durable_size: core::mem::size_of::<create_durable_rsp>(),
    create_mxac_size: core::mem::size_of::<create_mxac_rsp>(),
    create_disk_id_size: core::mem::size_of::<create_disk_id_rsp>(),
    create_posix_size: core::mem::size_of::<create_posix_rsp>(),
    create_aapl_size: AAPL_RSP_MAX_SIZE,
};

static mut smb30_server_values: smb_version_values = smb_version_values {
    version_string: SMB30_VERSION_STRING, protocol_id: SMB30_PROT_ID,
    req_capabilities: SMB2_GLOBAL_CAP_LARGE_MTU, max_read_size: SMB3_DEFAULT_IOSIZE,
    max_write_size: SMB3_DEFAULT_IOSIZE, max_trans_size: SMB3_DEFAULT_TRANS_SIZE,
    max_credits: SMB2_MAX_CREDITS, large_lock_type: 0,
    exclusive_lock_type: SMB2_LOCKFLAG_EXCLUSIVE, shared_lock_type: SMB2_LOCKFLAG_SHARED,
    unlock_lock_type: SMB2_LOCKFLAG_UNLOCK, header_size: core::mem::size_of::<smb2_hdr>(),
    max_header_size: MAX_SMB2_HDR_SIZE, read_rsp_size: core::mem::size_of::<smb2_read_rsp>(),
    lock_cmd: SMB2_LOCK, cap_unix: 0, cap_nt_find: SMB2_NT_FIND, cap_large_files: SMB2_LARGE_FILES,
    create_lease_size: core::mem::size_of::<create_lease_v2>(),
    create_durable_size: core::mem::size_of::<create_durable_rsp>(),
    create_durable_v2_size: core::mem::size_of::<create_durable_rsp_v2>(),
    create_mxac_size: core::mem::size_of::<create_mxac_rsp>(), create_disk_id_size: core::mem::size_of::<create_disk_id_rsp>(),
    create_posix_size: core::mem::size_of::<create_posix_rsp>(), create_aapl_size: AAPL_RSP_MAX_SIZE,
};

static mut smb302_server_values: smb_version_values = smb30_server_values;
static mut smb311_server_values: smb_version_values = smb30_server_values;

static mut smb2_0_server_ops: smb_version_ops = smb_version_ops {
    get_cmd_val: get_smb2_cmd_val, inc_reqs: ksmbd_counter_inc_reqs,
    init_rsp_hdr: init_smb2_rsp_hdr, set_rsp_status: set_smb2_rsp_status,
    allocate_rsp_buf: smb2_allocate_rsp_buf, set_rsp_credits: smb2_set_rsp_credits,
    check_user_session: smb2_check_user_session, get_ksmbd_tcon: smb2_get_ksmbd_tcon,
    is_sign_req: smb2_is_sign_req, check_sign_req: smb2_check_sign_req, set_sign_rsp: smb2_set_sign_rsp,
};
static mut smb3_0_server_ops: smb_version_ops = smb2_0_server_ops;
static mut smb3_11_server_ops: smb_version_ops = smb3_0_server_ops;

static mut smb2_0_server_cmds: [smb_version_cmd; NUMBER_OF_SMB2_COMMANDS] = [
    smb_version_cmd { proc: smb2_negotiate_request }, smb_version_cmd { proc: smb2_sess_setup },
    smb_version_cmd { proc: smb2_tree_connect }, smb_version_cmd { proc: smb2_tree_disconnect },
    smb_version_cmd { proc: smb2_session_logoff }, smb_version_cmd { proc: smb2_open },
    smb_version_cmd { proc: smb2_query_info }, smb_version_cmd { proc: smb2_query_dir },
    smb_version_cmd { proc: smb2_close }, smb_version_cmd { proc: smb2_echo },
    smb_version_cmd { proc: smb2_set_info }, smb_version_cmd { proc: smb2_read },
    smb_version_cmd { proc: smb2_write }, smb_version_cmd { proc: smb2_flush },
    smb_version_cmd { proc: smb2_cancel }, smb_version_cmd { proc: smb2_lock },
    smb_version_cmd { proc: smb2_ioctl }, smb_version_cmd { proc: smb2_oplock_break },
    smb_version_cmd { proc: smb2_notify },
];

pub unsafe fn init_smb2_1_server(conn: *mut ksmbd_conn) {
    (*conn).vals = &mut smb21_server_values; (*conn).ops = &mut smb2_0_server_ops;
    (*conn).cmds = smb2_0_server_cmds.as_mut_ptr(); (*conn).max_cmds = smb2_0_server_cmds.len();
    (*conn).signing_algorithm = SIGNING_ALG_HMAC_SHA256_LE;
    if server_conf.flags & KSMBD_GLOBAL_FLAG_SMB2_LEASES != 0 { (*conn).vals.req_capabilities |= SMB2_GLOBAL_CAP_LEASING; }
}

pub unsafe fn init_smb3_0_server(conn: *mut ksmbd_conn) {
    (*conn).vals = &mut smb30_server_values; (*conn).ops = &mut smb3_0_server_ops;
    (*conn).cmds = smb2_0_server_cmds.as_mut_ptr(); (*conn).max_cmds = smb2_0_server_cmds.len();
    (*conn).signing_algorithm = SIGNING_ALG_AES_CMAC_LE;
    if server_conf.flags & KSMBD_GLOBAL_FLAG_SMB2_LEASES != 0 { (*conn).vals.req_capabilities |= SMB2_GLOBAL_CAP_LEASING | SMB2_GLOBAL_CAP_DIRECTORY_LEASING; }
    if server_conf.flags & KSMBD_GLOBAL_FLAG_SMB2_ENCRYPTION != 0 && (*conn).cli_cap & SMB2_GLOBAL_CAP_ENCRYPTION != 0 { (*conn).vals.req_capabilities |= SMB2_GLOBAL_CAP_ENCRYPTION; }
    if server_conf.flags & KSMBD_GLOBAL_FLAG_SMB2_ENCRYPTION != 0 || (server_conf.flags & KSMBD_GLOBAL_FLAG_SMB2_ENCRYPTION_OFF == 0 && (*conn).cli_cap & SMB2_GLOBAL_CAP_ENCRYPTION != 0) { (*conn).vals.req_capabilities |= SMB2_GLOBAL_CAP_ENCRYPTION; }
    if server_conf.flags & KSMBD_GLOBAL_FLAG_SMB3_MULTICHANNEL != 0 { (*conn).vals.req_capabilities |= SMB2_GLOBAL_CAP_MULTI_CHANNEL; }
}

pub unsafe fn init_smb3_02_server(conn: *mut ksmbd_conn) {
    (*conn).vals = &mut smb302_server_values; (*conn).ops = &mut smb3_0_server_ops;
    (*conn).cmds = smb2_0_server_cmds.as_mut_ptr(); (*conn).max_cmds = smb2_0_server_cmds.len();
    (*conn).signing_algorithm = SIGNING_ALG_AES_CMAC_LE;
    if server_conf.flags & KSMBD_GLOBAL_FLAG_SMB2_LEASES != 0 { (*conn).vals.req_capabilities |= SMB2_GLOBAL_CAP_LEASING | SMB2_GLOBAL_CAP_DIRECTORY_LEASING; }
    if server_conf.flags & KSMBD_GLOBAL_FLAG_SMB2_ENCRYPTION != 0 || (server_conf.flags & KSMBD_GLOBAL_FLAG_SMB2_ENCRYPTION_OFF == 0 && (*conn).cli_cap & SMB2_GLOBAL_CAP_ENCRYPTION != 0) { (*conn).vals.req_capabilities |= SMB2_GLOBAL_CAP_ENCRYPTION; }
    if server_conf.flags & KSMBD_GLOBAL_FLAG_SMB3_MULTICHANNEL != 0 { (*conn).vals.req_capabilities |= SMB2_GLOBAL_CAP_MULTI_CHANNEL; }
    // Durable handles are in-memory only; persistent handles require CA recovery and fencing.
}

pub unsafe fn init_smb3_11_server(conn: *mut ksmbd_conn) -> i32 { (*conn).vals = &mut smb311_server_values; (*conn).ops = &mut smb3_11_server_ops; (*conn).cmds = smb2_0_server_cmds.as_mut_ptr(); (*conn).max_cmds = smb2_0_server_cmds.len(); (*conn).signing_algorithm = SIGNING_ALG_AES_CMAC_LE; if server_conf.flags & KSMBD_GLOBAL_FLAG_SMB2_LEASES != 0 { (*conn).vals.req_capabilities |= SMB2_GLOBAL_CAP_LEASING | SMB2_GLOBAL_CAP_DIRECTORY_LEASING; } if server_conf.flags & KSMBD_GLOBAL_FLAG_SMB3_MULTICHANNEL != 0 { (*conn).vals.req_capabilities |= SMB2_GLOBAL_CAP_MULTI_CHANNEL; } 0 }

pub unsafe fn init_smb2_max_read_size(mut sz: u32) { sz = clamp_val(sz, SMB3_MIN_IOSIZE, SMB3_MAX_IOSIZE); smb21_server_values.max_read_size = sz; smb30_server_values.max_read_size = sz; smb302_server_values.max_read_size = sz; smb311_server_values.max_read_size = sz; }
pub unsafe fn init_smb2_max_write_size(mut sz: u32) { sz = clamp_val(sz, SMB3_MIN_IOSIZE, SMB3_MAX_IOSIZE); smb21_server_values.max_write_size = sz; smb30_server_values.max_write_size = sz; smb302_server_values.max_write_size = sz; smb311_server_values.max_write_size = sz; }
pub unsafe fn init_smb2_max_trans_size(mut sz: u32) { sz = clamp_val(sz, SMB3_MIN_IOSIZE, SMB3_MAX_IOSIZE); smb21_server_values.max_trans_size = sz; smb30_server_values.max_trans_size = sz; smb302_server_values.max_trans_size = sz; smb311_server_values.max_trans_size = sz; }
pub unsafe fn init_smb2_max_credits(mut sz: u32) { if sz > SMB2_MAX_CREDITS { sz = SMB2_MAX_CREDITS; } smb21_server_values.max_credits = sz; smb30_server_values.max_credits = sz; smb302_server_values.max_credits = sz; smb311_server_values.max_credits = sz; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
