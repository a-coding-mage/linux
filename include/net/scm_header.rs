/* SPDX-License-Identifier: GPL-2.0 */

// Dependencies supplied by the surrounding kernel translation are intentionally
// referenced here rather than redefined.

pub const SCM_MAX_FD: usize = 253;

#[repr(C)]
pub struct scm_creds {
    pub pid: u32,
    pub uid: kuid_t,
    pub gid: kgid_t,
}

// #ifdef CONFIG_UNIX
pub struct unix_edge;
// #endif

#[repr(C)]
pub struct scm_fp_list {
    pub count: i16,
    pub count_unix: i16,
    pub max: i16,
    // #ifdef CONFIG_UNIX
    pub inflight: bool,
    pub dead: bool,
    pub vertices: list_head,
    pub edges: *mut unix_edge,
    // #endif
    pub user: *mut user_struct,
    pub fp: [*mut file; SCM_MAX_FD],
}

#[repr(C)]
pub struct scm_cookie {
    pub pid: *mut pid, // Skb credentials
    pub fp: *mut scm_fp_list, // Passed files
    pub creds: scm_creds, // Skb credentials
    // #ifdef CONFIG_SECURITY_NETWORK
    pub secid: u32, // Passed security ID
    // #endif
}

unsafe extern "C" {
    pub fn scm_detach_fds(msg: *mut msghdr, scm: *mut scm_cookie, notrunc: bool);
    pub fn scm_detach_fds_compat(msg: *mut msghdr, scm: *mut scm_cookie, notrunc: bool);
    pub fn __scm_send(sock: *mut socket, msg: *mut msghdr, scm: *mut scm_cookie) -> i32;
    pub fn __scm_destroy(scm: *mut scm_cookie);
    pub fn scm_fp_dup(fpl: *mut scm_fp_list) -> *mut scm_fp_list;
}

#[inline]
pub unsafe fn unix_get_peersec_dgram(sock: *mut socket, scm: *mut scm_cookie) {
    // #ifdef CONFIG_SECURITY_NETWORK
    security_socket_getpeersec_dgram(sock, core::ptr::null_mut(), &mut (*scm).secid);
    // #else: empty function body
    // #endif
}

#[inline]
pub unsafe fn scm_set_cred(
    scm: *mut scm_cookie,
    pid: *mut pid,
    uid: kuid_t,
    gid: kgid_t,
) {
    (*scm).pid = get_pid(pid);
    (*scm).creds.pid = pid_vnr(pid);
    (*scm).creds.uid = uid;
    (*scm).creds.gid = gid;
}

#[inline]
pub unsafe fn scm_destroy_cred(scm: *mut scm_cookie) {
    put_pid((*scm).pid);
    (*scm).pid = core::ptr::null_mut();
}

#[inline]
pub unsafe fn scm_destroy(scm: *mut scm_cookie) {
    scm_destroy_cred(scm);
    if !(*scm).fp.is_null() {
        __scm_destroy(scm);
    }
}

#[inline]
pub unsafe fn scm_send(
    sock: *mut socket,
    msg: *mut msghdr,
    scm: *mut scm_cookie,
    forcecreds: bool,
) -> i32 {
    core::ptr::write_bytes(scm, 0, 1);
    (*scm).creds.uid = INVALID_UID;
    (*scm).creds.gid = INVALID_GID;
    if forcecreds {
        scm_set_cred(scm, task_tgid(current), current_uid(), current_gid());
    }
    unix_get_peersec_dgram(sock, scm);
    if (*msg).msg_controllen <= 0 {
        return 0;
    }
    __scm_send(sock, msg, scm)
}

unsafe extern "C" {
    pub fn scm_recv(sock: *mut socket, msg: *mut msghdr, scm: *mut scm_cookie, flags: i32);
    pub fn scm_recv_unix(sock: *mut socket, msg: *mut msghdr, scm: *mut scm_cookie, flags: i32);
    pub fn scm_recv_one_fd(
        f: *mut file,
        ufd: *mut i32,
        flags: u32,
        notrunc: bool,
    ) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
