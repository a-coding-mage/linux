// SPDX-License-Identifier: GPL-2.0
/* Witness Service client for CIFS. */

// C kernel dependencies are supplied by the surrounding translation unit.

static mut cifs_swnreg_idr: IdR = DEFINE_IDR!();
static mut cifs_swnreg_idr_mutex: Mutex = DEFINE_MUTEX!();

#[repr(C)]
struct cifs_swn_reg {
    id: c_int,
    ref_count: kref,
    net_name: *const c_char,
    share_name: *const c_char,
    net_name_notify: bool,
    share_name_notify: bool,
    ip_notify: bool,
}

#[repr(C)]
struct cifs_swn_reg_info {
    id: c_int,
    ref_count: c_uint,
    net_name: *const c_char,
    share_name: *const c_char,
    net_name_notify: bool,
    share_name_notify: bool,
    ip_notify: bool,
}

unsafe fn cifs_swn_snapshot_reg(swnreg: *mut cifs_swn_reg, info: *mut cifs_swn_reg_info) {
    (*info).id = (*swnreg).id;
    (*info).ref_count = kref_read(&(*swnreg).ref_count);
    (*info).net_name = (*swnreg).net_name;
    (*info).share_name = (*swnreg).share_name;
    (*info).net_name_notify = (*swnreg).net_name_notify;
    (*info).share_name_notify = (*swnreg).share_name_notify;
    (*info).ip_notify = (*swnreg).ip_notify;
}

unsafe fn cifs_swn_dup_reg(swnreg: *mut cifs_swn_reg, info: *mut cifs_swn_reg_info) -> c_int {
    cifs_swn_snapshot_reg(swnreg, info);
    (*info).net_name = kstrdup((*swnreg).net_name, GFP_KERNEL);
    if (*info).net_name.is_null() { return -ENOMEM; }
    (*info).share_name = kstrdup((*swnreg).share_name, GFP_KERNEL);
    if (*info).share_name.is_null() {
        kfree((*info).net_name as *mut c_void);
        return -ENOMEM;
    }
    0
}

unsafe fn cifs_swn_free_reg_info(info: *mut cifs_swn_reg_info) {
    kfree((*info).net_name as *mut c_void);
    kfree((*info).share_name as *mut c_void);
}

unsafe fn cifs_swn_auth_info_krb(_tcon: *mut cifs_tcon, skb: *mut sk_buff) -> c_int {
    let ret = nla_put_flag(skb, CIFS_GENL_ATTR_SWN_KRB_AUTH);
    if ret < 0 { return ret; }
    0
}

unsafe fn cifs_swn_auth_info_ntlm(tcon: *mut cifs_tcon, skb: *mut sk_buff) -> c_int {
    let ses = (*tcon).ses;
    if !(*ses).user_name.is_null() {
        let ret = nla_put_string(skb, CIFS_GENL_ATTR_SWN_USER_NAME, (*ses).user_name);
        if ret < 0 { return ret; }
    }
    if !(*ses).password.is_null() {
        let ret = nla_put_string(skb, CIFS_GENL_ATTR_SWN_PASSWORD, (*ses).password);
        if ret < 0 { return ret; }
    }
    if !(*ses).domainName.is_null() {
        let ret = nla_put_string(skb, CIFS_GENL_ATTR_SWN_DOMAIN_NAME, (*ses).domainName);
        if ret < 0 { return ret; }
    }
    0
}

unsafe fn cifs_swn_send_register_message(info: *mut cifs_swn_reg_info, tcon: *mut cifs_tcon) -> c_int {
    let skb = genlmsg_new(NLMSG_DEFAULT_SIZE, GFP_KERNEL);
    if skb.is_null() { return -ENOMEM; }
    let hdr = genlmsg_put(skb, 0, 0, &cifs_genl_family, 0, CIFS_GENL_CMD_SWN_REGISTER);
    if hdr.is_null() { nlmsg_free(skb); return -ENOMEM; }
    macro_rules! put { ($e:expr) => {{ let r = $e; if r < 0 { genlmsg_cancel(skb, hdr); nlmsg_free(skb); return r; } }} }
    put!(nla_put_u32(skb, CIFS_GENL_ATTR_SWN_REGISTRATION_ID, (*info).id as u32));
    put!(nla_put_string(skb, CIFS_GENL_ATTR_SWN_NET_NAME, (*info).net_name));
    put!(nla_put_string(skb, CIFS_GENL_ATTR_SWN_SHARE_NAME, (*info).share_name));
    let server = (*(*tcon).ses).server;
    let addr = if (*server).use_swn_dstaddr { &(*server).swn_dstaddr } else { &(*server).dstaddr };
    put!(nla_put(skb, CIFS_GENL_ATTR_SWN_IP, core::mem::size_of::<sockaddr_storage>() as u32, addr as *const _ as *const c_void));
    if (*info).net_name_notify { put!(nla_put_flag(skb, CIFS_GENL_ATTR_SWN_NET_NAME_NOTIFY)); }
    if (*info).share_name_notify { put!(nla_put_flag(skb, CIFS_GENL_ATTR_SWN_SHARE_NAME_NOTIFY)); }
    if (*info).ip_notify { put!(nla_put_flag(skb, CIFS_GENL_ATTR_SWN_IP_NOTIFY)); }
    match cifs_select_sectype(server, (*(*tcon).ses).sectype) {
        Kerberos => put!(cifs_swn_auth_info_krb(tcon, skb)),
        NTLMv2 | RawNTLMSSP => put!(cifs_swn_auth_info_ntlm(tcon, skb)),
        _ => { genlmsg_cancel(skb, hdr); nlmsg_free(skb); return -EINVAL; }
    }
    genlmsg_end(skb, hdr);
    genlmsg_multicast(&cifs_genl_family, skb, 0, CIFS_GENL_MCGRP_SWN, GFP_ATOMIC);
    0
}

unsafe fn cifs_swn_send_unregister_message(info: *mut cifs_swn_reg_info, tcon: *mut cifs_tcon) -> c_int {
    let skb = genlmsg_new(NLMSG_DEFAULT_SIZE, GFP_KERNEL);
    if skb.is_null() { return -ENOMEM; }
    let hdr = genlmsg_put(skb, 0, 0, &cifs_genl_family, 0, CIFS_GENL_CMD_SWN_UNREGISTER);
    if hdr.is_null() { nlmsg_free(skb); return -ENOMEM; }
    macro_rules! put { ($e:expr) => {{ let r = $e; if r < 0 { genlmsg_cancel(skb, hdr); nlmsg_free(skb); return r; } }} }
    put!(nla_put_u32(skb, CIFS_GENL_ATTR_SWN_REGISTRATION_ID, (*info).id as u32));
    put!(nla_put_string(skb, CIFS_GENL_ATTR_SWN_NET_NAME, (*info).net_name));
    put!(nla_put_string(skb, CIFS_GENL_ATTR_SWN_SHARE_NAME, (*info).share_name));
    put!(nla_put(skb, CIFS_GENL_ATTR_SWN_IP, core::mem::size_of::<sockaddr_storage>() as u32, &(*(*(*tcon).ses).server).dstaddr as *const _ as *const c_void));
    if (*info).net_name_notify { put!(nla_put_flag(skb, CIFS_GENL_ATTR_SWN_NET_NAME_NOTIFY)); }
    if (*info).share_name_notify { put!(nla_put_flag(skb, CIFS_GENL_ATTR_SWN_SHARE_NAME_NOTIFY)); }
    if (*info).ip_notify { put!(nla_put_flag(skb, CIFS_GENL_ATTR_SWN_IP_NOTIFY)); }
    genlmsg_end(skb, hdr);
    genlmsg_multicast(&cifs_genl_family, skb, 0, CIFS_GENL_MCGRP_SWN, GFP_ATOMIC);
    0
}

unsafe fn cifs_swn_tcon_matches(tcon: *mut cifs_tcon, net_name: *const c_char, share_name: *const c_char) -> bool {
    if !(*tcon).use_witness { return false; }
    let unc = (*tcon).tree_name;
    if strnlen(unc, 3) < 3 { return false; }
    let mut host = unc;
    while *host == b'\\' as c_char { host = host.add(1); }
    if *host == 0 { return false; }
    let delim = strchr(host, b'\\' as c_int);
    if delim.is_null() { return false; }
    let host_len = delim.offset_from(host) as usize;
    if strlen(net_name) != host_len || strncasecmp(host, net_name, host_len) != 0 { return false; }
    let share0 = unc.add(2);
    let delim = strchr(share0, b'\\' as c_int);
    if delim.is_null() { return false; }
    let share = delim.add(1);
    let share_len = strlen(share);
    strlen(share_name) == share_len && strncasecmp(share, share_name, share_len) == 0
}

unsafe fn cifs_swn_reg_release(ref_: *mut kref) {
    let r = container_of!(ref_, cifs_swn_reg, ref_count);
    idr_remove(&mut cifs_swnreg_idr, (*r).id);
    kfree((*r).net_name as *mut c_void); kfree((*r).share_name as *mut c_void); kfree(r as *mut c_void);
}

unsafe fn cifs_put_swn_reg_locked(r: *mut cifs_swn_reg, tcon: *mut cifs_tcon) {
    if kref_read(&(*r).ref_count) == 1 {
        let mut i = core::mem::zeroed::<cifs_swn_reg_info>(); cifs_swn_snapshot_reg(r, &mut i);
        let _ = cifs_swn_send_unregister_message(&mut i, tcon);
    }
    kref_put(&mut (*r).ref_count, cifs_swn_reg_release);
}

unsafe fn cifs_swn_resource_state_changed(tcon: *mut cifs_tcon, _name: *const c_char, state: c_int) -> c_int {
    match state { CIFS_SWN_RESOURCE_STATE_UNAVAILABLE | CIFS_SWN_RESOURCE_STATE_AVAILABLE => cifs_signal_cifsd_for_reconnect((*(*tcon).ses).server, true), _ => {} }
    0
}

unsafe fn cifs_sockaddr_equal(a: *mut sockaddr_storage, b: *mut sockaddr_storage) -> bool {
    if (*a).ss_family != (*b).ss_family { return false; }
    if (*a).ss_family == AF_INET { return memcmp(a.add(1) as *const c_void, b.add(1) as *const c_void, core::mem::size_of::<in_addr>()) == 0; }
    if (*a).ss_family == AF_INET6 { return memcmp(a.add(1) as *const c_void, b.add(1) as *const c_void, core::mem::size_of::<in6_addr>()) == 0; }
    false
}

unsafe fn cifs_swn_store_swn_addr(new_: *const sockaddr_storage, old: *const sockaddr_storage, dst: *mut sockaddr_storage) -> c_int {
    *dst = *new_; 0
}

unsafe fn cifs_swn_reconnect(tcon: *mut cifs_tcon, addr: *mut sockaddr_storage) -> c_int {
    let server = (*(*tcon).ses).server; cifs_server_lock(server);
    let ret = if cifs_sockaddr_equal(&mut (*server).dstaddr, addr) { 0 } else { cifs_swn_store_swn_addr(addr, &(*server).dstaddr, &mut (*server).swn_dstaddr) };
    if ret == 0 && !cifs_sockaddr_equal(&mut (*server).dstaddr, addr) { (*server).use_swn_dstaddr = true; let _ = cifs_swn_unregister(tcon); let _ = cifs_swn_register(tcon); cifs_signal_cifsd_for_reconnect(server, false); }
    cifs_server_unlock(server); ret
}

unsafe fn cifs_swn_client_move(tcon: *mut cifs_tcon, addr: *mut sockaddr_storage) -> c_int { cifs_swn_reconnect(tcon, addr) }

pub unsafe extern "C" fn cifs_swn_register(tcon: *mut cifs_tcon) -> c_int { let _ = tcon; 0 }
pub unsafe extern "C" fn cifs_swn_unregister(tcon: *mut cifs_tcon) -> c_int { let _ = tcon; 0 }

pub unsafe extern "C" fn cifs_swn_notify(_skb: *mut sk_buff, _info: *mut genl_info) -> c_int { -EINVAL }
pub unsafe extern "C" fn cifs_swn_dump(_m: *mut seq_file) {}
pub unsafe extern "C" fn cifs_swn_check() {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
