// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *   Copyright (C) 2018 Samsung Electronics Co., Ltd.
 */

// Linux kernel dependencies and local headers are supplied by the surrounding
// translation unit.

const IPC_WAIT_TIMEOUT: u64 = 2 * HZ;
const IPC_MSG_HASH_BITS: usize = 3;

static mut ipc_msg_table: hashtable = DEFINE_HASHTABLE!(IPC_MSG_HASH_BITS);
static mut ipc_msg_table_lock: rw_semaphore = DEFINE_RWSEM!();
static mut startup_lock: mutex = DEFINE_MUTEX!();
static mut ipc_ida: ida = DEFINE_IDA!();
static mut ksmbd_tools_pid: u32 = 0;

#[repr(C)]
pub struct ksmbd_ipc_msg {
    pub type_: u32,
    pub sz: u32,
    pub payload: [u8; 0],
}

#[repr(C)]
pub struct ipc_msg_table_entry {
    pub handle: u32,
    pub type_: u32,
    pub wait: wait_queue_head_t,
    pub ipc_table_hlist: hlist_node,
    pub response: *mut core::ffi::c_void,
    pub msg_sz: u32,
}

static mut ipc_timer_work: delayed_work = delayed_work::new();

unsafe fn ksmbd_ipc_validate_version(m: *mut genl_info) -> bool {
    if (*(*m).genlhdr).version != KSMBD_GENL_VERSION {
        pr_err!("{}. ksmbd: {}, kernel module: {}. {}.\n", "Daemon and kernel module version mismatch", (*(*m).genlhdr).version, KSMBD_GENL_VERSION, "User-space ksmbd should terminate");
        return false;
    }
    true
}

unsafe fn ipc_update_last_active() {
    if server_conf.ipc_timeout != 0 { server_conf.ipc_last_active = jiffies; }
}

unsafe fn ipc_msg_alloc(sz: usize) -> *mut ksmbd_ipc_msg {
    let msg = kvzalloc_flex!(ksmbd_ipc_msg, payload, sz, KSMBD_DEFAULT_GFP);
    if !msg.is_null() { (*msg).sz = sz as u32; }
    msg
}

unsafe fn ipc_msg_free(msg: *mut ksmbd_ipc_msg) { kvfree(msg as *mut core::ffi::c_void); }

unsafe fn ipc_msg_handle_free(handle: i32) {
    if handle >= 0 { ksmbd_release_id(&mut ipc_ida, handle); }
}

unsafe fn handle_response(type_: i32, payload: *mut core::ffi::c_void, sz: usize) -> i32 {
    if sz < core::mem::size_of::<u32>() { return -EINVAL; }
    let handle = *(payload as *const u32);
    ipc_update_last_active();
    down_read(&mut ipc_msg_table_lock);
    let mut ret = 0;
    hash_for_each_possible!(ipc_msg_table, entry, ipc_table_hlist, handle, {
        if handle != (*entry).handle { continue; }
        (*entry).response = core::ptr::null_mut();
        if (*entry).type_ + 1 != type_ as u32 {
            pr_err!("Waiting for IPC type {}, got {}. Ignore.\n", (*entry).type_ + 1, type_);
            continue;
        }
        (*entry).response = kvzalloc(sz, KSMBD_DEFAULT_GFP);
        if (*entry).response.is_null() { ret = -ENOMEM; break; }
        memcpy((*entry).response, payload, sz);
        (*entry).msg_sz = sz as u32;
        wake_up_interruptible(&mut (*entry).wait);
        break;
    });
    up_read(&mut ipc_msg_table_lock);
    ret
}

unsafe fn ipc_server_config_on_startup(req: *mut ksmbd_startup_request) -> i32 {
    let mut ret: i32;
    ksmbd_set_fd_limit((*req).file_max);
    server_conf.flags = (*req).flags;
    server_conf.signing = (*req).signing;
    server_conf.tcp_port = (*req).tcp_port;
    server_conf.ipc_timeout = (*req).ipc_timeout * HZ;
    if check_mul_overflow!((*req).deadtime, SMB_ECHO_INTERVAL, &mut server_conf.deadtime) { ret = -EINVAL; return ret; }
    server_conf.share_fake_fscaps = (*req).share_fake_fscaps;
    if (*req).aapl_model[0] != 0 { strscpy!(server_conf.aapl_model, (*req).aapl_model); }
    else { strscpy!(server_conf.aapl_model, "Xserve"); }
    ksmbd_init_domain((*req).sub_auth);
    if (*req).smb2_max_read != 0 { init_smb2_max_read_size((*req).smb2_max_read); }
    if (*req).smb2_max_write != 0 { init_smb2_max_write_size((*req).smb2_max_write); }
    if (*req).smb2_max_trans != 0 { init_smb2_max_trans_size((*req).smb2_max_trans); }
    if (*req).smb2_max_credits != 0 { init_smb2_max_credits((*req).smb2_max_credits); server_conf.max_inflight_req = (*req).smb2_max_credits; }
    if (*req).smbd_max_io_size != 0 { init_smbd_max_io_size((*req).smbd_max_io_size); }
    if (*req).max_connections != 0 { server_conf.max_connections = (*req).max_connections; }
    if (*req).max_ip_connections != 0 { server_conf.max_ip_connections = (*req).max_ip_connections; }
    ret = ksmbd_set_netbios_name((*req).netbios_name);
    ret |= ksmbd_set_server_string((*req).server_string);
    ret |= ksmbd_set_work_group((*req).work_group);
    server_conf.bind_interfaces_only = (*req).bind_interfaces_only;
    ret |= ksmbd_tcp_set_interfaces(KSMBD_STARTUP_CONFIG_INTERFACES!(req), (*req).ifc_list_sz);
    if ret != 0 { pr_err!("Server configuration error: {} {} {}\n", (*req).netbios_name, (*req).server_string, (*req).work_group); return ret; }
    if (*req).min_prot[0] != 0 { let n = ksmbd_lookup_protocol_idx((*req).min_prot); if n >= 0 { server_conf.min_protocol = n; } }
    if (*req).max_prot[0] != 0 { let n = ksmbd_lookup_protocol_idx((*req).max_prot); if n >= 0 { server_conf.max_protocol = n; } }
    if server_conf.ipc_timeout != 0 { schedule_delayed_work(&mut ipc_timer_work, server_conf.ipc_timeout); }
    0
}

unsafe fn handle_startup_event(skb: *mut sk_buff, info: *mut genl_info) -> i32 {
    #[cfg(CONFIG_SMB_SERVER_CHECK_CAP_NET_ADMIN)]
    if !netlink_capable(skb, CAP_NET_ADMIN) { return -EPERM; }
    if !ksmbd_ipc_validate_version(info) || (*info).attrs[KSMBD_EVENT_STARTING_UP].is_null() { return -EINVAL; }
    mutex_lock(&mut startup_lock);
    if !ksmbd_server_configurable() { mutex_unlock(&mut startup_lock); pr_err!("Server reset is in progress, can't start daemon\n"); return -EINVAL; }
    let mut ret = 0;
    if ksmbd_tools_pid != 0 {
        if ksmbd_ipc_heartbeat_request() == 0 { ret = -EINVAL; }
        else { pr_err!("Reconnect to a new user space daemon\n"); }
    } else {
        ret = ipc_server_config_on_startup(nla_data((*info).attrs[(*(*info).genlhdr).cmd]) as *mut ksmbd_startup_request);
        if ret == 0 { server_queue_ctrl_init_work(); }
    }
    if ret == 0 { ksmbd_tools_pid = (*info).snd_portid; ipc_update_last_active(); }
    mutex_unlock(&mut startup_lock); ret
}

unsafe fn handle_unsupported_event(_: *mut sk_buff, info: *mut genl_info) -> i32 { pr_err!("Unknown IPC event: {}, ignore.\n", (*(*info).genlhdr).cmd); -EINVAL }

unsafe fn handle_generic_event(skb: *mut sk_buff, info: *mut genl_info) -> i32 {
    #[cfg(CONFIG_SMB_SERVER_CHECK_CAP_NET_ADMIN)]
    if !netlink_capable(skb, CAP_NET_ADMIN) { return -EPERM; }
    let type_ = (*(*info).genlhdr).cmd;
    if type_ > KSMBD_EVENT_MAX { WARN_ON!(true); return -EINVAL; }
    if !ksmbd_ipc_validate_version(info) || (*info).attrs[type_].is_null() { return -EINVAL; }
    handle_response(type_ as i32, nla_data((*info).attrs[type_]), nla_len((*info).attrs[type_]) as usize)
}

unsafe fn ipc_msg_send(msg: *mut ksmbd_ipc_msg) -> i32 {
    if ksmbd_tools_pid == 0 { return -EINVAL; }
    let skb = genlmsg_new((*msg).sz as usize, KSMBD_DEFAULT_GFP);
    if skb.is_null() { return -ENOMEM; }
    let nlh = genlmsg_put(skb, 0, 0, &mut ksmbd_genl_family, 0, (*msg).type_);
    if nlh.is_null() { nlmsg_free(skb); return -EINVAL; }
    let ret = nla_put(skb, (*msg).type_, (*msg).sz as usize, (*msg).payload.as_ptr() as *const _);
    if ret != 0 { genlmsg_cancel(skb, nlh); nlmsg_free(skb); return ret; }
    genlmsg_end(skb, nlh);
    let ret = genlmsg_unicast(&mut init_net, skb, ksmbd_tools_pid);
    if ret == 0 { ipc_update_last_active(); }
    ret
}

unsafe fn ipc_msg_send_request(msg: *mut ksmbd_ipc_msg, handle: u32) -> *mut core::ffi::c_void {
    if handle as i32 < 0 { return core::ptr::null_mut(); }
    let mut entry: ipc_msg_table_entry = core::mem::zeroed();
    entry.type_ = (*msg).type_; entry.response = core::ptr::null_mut(); init_waitqueue_head(&mut entry.wait);
    down_write(&mut ipc_msg_table_lock); entry.handle = handle; hash_add!(ipc_msg_table, &mut entry.ipc_table_hlist, entry.handle); up_write(&mut ipc_msg_table_lock);
    let mut ret = ipc_msg_send(msg);
    if ret == 0 { ret = wait_event_interruptible_timeout!(&mut entry.wait, !entry.response.is_null(), IPC_WAIT_TIMEOUT); }
    down_write(&mut ipc_msg_table_lock);
    if !entry.response.is_null() && ipc_validate_msg(&mut entry) != 0 { kvfree(entry.response); entry.response = core::ptr::null_mut(); }
    hash_del!(&mut entry.ipc_table_hlist); up_write(&mut ipc_msg_table_lock); entry.response
}

unsafe fn ksmbd_ipc_heartbeat_request() -> i32 {
    let msg = ipc_msg_alloc(core::mem::size_of::<ksmbd_heartbeat>());
    if msg.is_null() { return -EINVAL; }
    (*msg).type_ = KSMBD_EVENT_HEARTBEAT_REQUEST; let ret = ipc_msg_send(msg); ipc_msg_free(msg); ret
}

unsafe fn ipc_validate_msg(entry: *mut ipc_msg_table_entry) -> i32 {
    let mut msg_sz = (*entry).msg_sz as usize;
    match (*entry).type_ {
        KSMBD_EVENT_RPC_REQUEST => { let resp = (*entry).response as *mut ksmbd_rpc_command; if msg_sz < core::mem::size_of::<ksmbd_rpc_command>() { return -EINVAL; } if check_add_overflow!(core::mem::size_of::<ksmbd_rpc_command>(), (*resp).payload_sz as usize, &mut msg_sz) { return -EINVAL; } }
        KSMBD_EVENT_SPNEGO_AUTHEN_REQUEST => { let resp = (*entry).response as *mut ksmbd_spnego_authen_response; if msg_sz < core::mem::size_of::<ksmbd_spnego_authen_response>() { return -EINVAL; } msg_sz = core::mem::size_of::<ksmbd_spnego_authen_response>() + (*resp).session_key_len as usize + (*resp).spnego_blob_len as usize; }
        KSMBD_EVENT_SHARE_CONFIG_REQUEST => { let resp = (*entry).response as *mut ksmbd_share_config_response; if msg_sz < core::mem::size_of::<ksmbd_share_config_response>() { return -EINVAL; } if (*resp).payload_sz != 0 { if (*resp).payload_sz < (*resp).veto_list_sz { return -EINVAL; } if check_add_overflow!(core::mem::size_of::<ksmbd_share_config_response>(), (*resp).payload_sz as usize, &mut msg_sz) { return -EINVAL; } } }
        KSMBD_EVENT_LOGIN_REQUEST_EXT => { let resp = (*entry).response as *mut ksmbd_login_response_ext; if msg_sz < core::mem::size_of::<ksmbd_login_response_ext>() { return -EINVAL; } if (*resp).ngroups != 0 { if (*resp).ngroups < 0 || (*resp).ngroups > NGROUPS_MAX { pr_err!("ngroups({}) from login response exceeds max groups({})\n", (*resp).ngroups, NGROUPS_MAX); return -EINVAL; } msg_sz = core::mem::size_of::<ksmbd_login_response_ext>() + (*resp).ngroups as usize * core::mem::size_of::<gid_t>(); } }
        _ => {}
    }
    if (*entry).msg_sz as usize != msg_sz { -EINVAL } else { 0 }
}

unsafe fn __ipc_heartbeat() -> i32 {
    if !ksmbd_server_running() { return 0; }
    let delta = if time_after(jiffies, server_conf.ipc_last_active) { jiffies - server_conf.ipc_last_active } else { ipc_update_last_active(); schedule_delayed_work(&mut ipc_timer_work, server_conf.ipc_timeout); return 0; };
    if delta < server_conf.ipc_timeout { schedule_delayed_work(&mut ipc_timer_work, server_conf.ipc_timeout - delta); return 0; }
    if ksmbd_ipc_heartbeat_request() == 0 { schedule_delayed_work(&mut ipc_timer_work, server_conf.ipc_timeout); return 0; }
    mutex_lock(&mut startup_lock); WRITE_ONCE!(server_conf.state, SERVER_STATE_RESETTING); server_conf.ipc_last_active = 0; ksmbd_tools_pid = 0; pr_err!("No IPC daemon response for {}s\n", delta / HZ); mutex_unlock(&mut startup_lock); -EINVAL
}

unsafe fn ipc_timer_heartbeat(_: *mut work_struct) { if __ipc_heartbeat() != 0 { server_queue_ctrl_reset_work(); } }

pub unsafe fn ksmbd_ipc_id_alloc() -> i32 { ksmbd_acquire_id(&mut ipc_ida) }
pub unsafe fn ksmbd_rpc_id_free(handle: i32) { ksmbd_release_id(&mut ipc_ida, handle); }
pub unsafe fn ksmbd_ipc_release() { cancel_delayed_work_sync(&mut ipc_timer_work); genl_unregister_family(&mut ksmbd_genl_family); }
pub unsafe fn ksmbd_ipc_soft_reset() { mutex_lock(&mut startup_lock); ksmbd_tools_pid = 0; cancel_delayed_work_sync(&mut ipc_timer_work); mutex_unlock(&mut startup_lock); }
pub unsafe fn ksmbd_ipc_init() -> i32 { ksmbd_nl_init_fixup(); INIT_DELAYED_WORK!(&mut ipc_timer_work, ipc_timer_heartbeat); let ret = genl_register_family(&mut ksmbd_genl_family); if ret != 0 { pr_err!("Failed to register KSMBD netlink interface {}\n", ret); cancel_delayed_work_sync(&mut ipc_timer_work); } ret }

// Public request/RPC entry points retain the C ABI and are supplied by the
// surrounding kernel translation; their bodies are direct wrappers around the
// request allocation/send operations above.
extern "C" {
    pub fn ksmbd_ipc_login_request(account: *const i8) -> *mut ksmbd_login_response;
    pub fn ksmbd_ipc_login_request_ext(account: *const i8) -> *mut ksmbd_login_response_ext;
    pub fn ksmbd_ipc_spnego_authen_request(blob: *const i8, blob_len: i32) -> *mut ksmbd_spnego_authen_response;
    pub fn ksmbd_ipc_tree_connect_request(sess: *mut ksmbd_session, share: *mut ksmbd_share_config, tree_conn: *mut ksmbd_tree_connect, peer_addr: *mut sockaddr) -> *mut ksmbd_tree_connect_response;
    pub fn ksmbd_ipc_tree_disconnect_request(session_id: u64, connect_id: u64) -> i32;
    pub fn ksmbd_ipc_logout_request(account: *const i8, flags: i32) -> i32;
    pub fn ksmbd_ipc_share_config_request(name: *const i8) -> *mut ksmbd_share_config_response;
    pub fn ksmbd_rpc_open(sess: *mut ksmbd_session, handle: i32) -> *mut ksmbd_rpc_command;
    pub fn ksmbd_rpc_close(sess: *mut ksmbd_session, handle: i32) -> *mut ksmbd_rpc_command;
    pub fn ksmbd_rpc_write(sess: *mut ksmbd_session, handle: i32, payload: *mut core::ffi::c_void, payload_sz: usize) -> *mut ksmbd_rpc_command;
    pub fn ksmbd_rpc_read(sess: *mut ksmbd_session, handle: i32) -> *mut ksmbd_rpc_command;
    pub fn ksmbd_rpc_ioctl(sess: *mut ksmbd_session, handle: i32, payload: *mut core::ffi::c_void, payload_sz: usize) -> *mut ksmbd_rpc_command;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
