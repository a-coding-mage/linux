/* SPDX-License-Identifier: GPL-2.0 */

// Dependencies supplied by the surrounding kernel translation are intentionally
// left external: atomic_t, mutex, sock, path, refcount_t, spinlock_t,
// wait_queue_entry_t, socket_wq, sockaddr_un, unix_vertex, sk_buff, and file.

#[cfg(feature = "CONFIG_UNIX")]
extern "C" {
    pub fn unix_get_socket(filp: *mut file) -> *mut unix_sock;
}

#[cfg(not(feature = "CONFIG_UNIX"))]
#[inline]
pub unsafe fn unix_get_socket(_filp: *mut file) -> *mut unix_sock {
    core::ptr::null_mut()
}

#[repr(C)]
pub struct unix_address {
    pub refcnt: refcount_t,
    pub len: core::ffi::c_int,
    pub name: [sockaddr_un; 0],
}

#[repr(C)]
pub struct scm_stat {
    pub nr_fds: atomic_t,
    pub nr_unix_fds: core::ffi::c_ulong,
}

/* The AF_UNIX socket */
#[repr(C)]
pub struct unix_sock {
    /* WARNING: sk has to be the first member */
    pub sk: sock,
    pub addr: *mut unix_address,
    pub path: path,
    pub iolock: mutex,
    pub bindlock: mutex,
    pub peer: *mut sock,
    pub listener: *mut sock,
    pub vertex: *mut unix_vertex,
    pub lock: spinlock_t,
    pub peer_wq: socket_wq,
    pub peer_wake: wait_queue_entry_t,
    pub scm_stat: scm_stat,
    pub inq_len: core::ffi::c_int,
    pub recvmsg_inq: bool,
    pub scm_rights_notrunc: bool,
    #[cfg(feature = "CONFIG_AF_UNIX_OOB")]
    pub oob_skb: *mut sk_buff,
}

// #define peer_wait peer_wq.wait
// Access the wait member through the surrounding translation's socket_wq type.

#[inline]
pub unsafe fn unix_sk(ptr: *const sock) -> *const unix_sock {
    (ptr as *const u8).sub(core::mem::offset_of!(unix_sock, sk)) as *const unix_sock
}

#[inline]
pub unsafe fn unix_peer(sk: *const sock) -> *mut sock {
    (*unix_sk(sk)).peer
}

#[inline]
pub unsafe fn unix_state_lock(s: *mut sock) {
    spin_lock(&mut (*unix_sk(s)).lock);
}

#[inline]
pub unsafe fn unix_state_unlock(s: *mut sock) {
    spin_unlock(&mut (*unix_sk(s)).lock);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
