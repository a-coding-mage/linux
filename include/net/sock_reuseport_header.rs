/* SPDX-License-Identifier: GPL-2.0 */

// Translated from sock_reuseport.h.  The following types and functions are
// supplied by the corresponding kernel dependencies.

extern "C" {
    pub static mut reuseport_lock: spinlock_t;

    pub fn reuseport_alloc(sk: *mut sock, bind_inany: bool) -> ::std::os::raw::c_int;
    pub fn reuseport_add_sock(
        sk: *mut sock,
        sk2: *mut sock,
        bind_inany: bool,
    ) -> ::std::os::raw::c_int;
    pub fn reuseport_detach_sock(sk: *mut sock);
    pub fn reuseport_stop_listen_sock(sk: *mut sock);
    pub fn reuseport_select_sock(
        sk: *mut sock,
        hash: u32,
        skb: *mut sk_buff,
        hdr_len: ::std::os::raw::c_int,
    ) -> *mut sock;
    pub fn reuseport_migrate_sock(
        sk: *mut sock,
        migrating_sk: *mut sock,
        skb: *mut sk_buff,
    ) -> *mut sock;
    pub fn reuseport_attach_prog(
        sk: *mut sock,
        prog: *mut bpf_prog,
    ) -> ::std::os::raw::c_int;
    pub fn reuseport_detach_prog(sk: *mut sock) -> ::std::os::raw::c_int;
    pub fn reuseport_has_conns_set(sk: *mut sock);
    pub fn reuseport_update_incoming_cpu(sk: *mut sock, val: ::std::os::raw::c_int);
}

#[repr(C)]
pub struct sock_reuseport {
    pub rcu: rcu_head,

    pub max_socks: u16,          /* length of socks */
    pub num_socks: u16,          /* elements in socks */
    pub num_closed_socks: u16,  /* closed elements in socks */
    pub incoming_cpu: u16,
    /* The last synq overflow event timestamp of this
     * reuse->socks[] group.
     */
    pub synq_overflow_ts: ::std::os::raw::c_uint,
    /* ID stays the same even after the size of socks[] grows. */
    pub reuseport_id: ::std::os::raw::c_uint,
    pub bind_inany: ::std::os::raw::c_uint,
    pub has_conns: ::std::os::raw::c_uint,
    pub prog: *mut bpf_prog,     /* optional BPF sock selector */
    // Flexible array member: struct sock *socks[] __counted_by(max_socks)
    pub socks: [*mut sock; 0],
}

#[inline]
pub unsafe fn reuseport_has_conns(sk: *mut sock) -> bool {
    let mut reuse: *mut sock_reuseport;
    let mut ret = false;

    rcu_read_lock();
    reuse = rcu_dereference((*sk).sk_reuseport_cb);
    if !reuse.is_null() && (*reuse).has_conns != 0 {
        ret = true;
    }
    rcu_read_unlock();

    ret
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
