// SPDX-License-Identifier: GPL-2.0
// Dependencies supplied by the surrounding kernel/MPTCP translation.

#[repr(C)]
pub struct join_entry {
    pub token: u32,
    pub remote_nonce: u32,
    pub local_nonce: u32,
    pub join_id: u8,
    pub local_id: u8,
    pub backup: u8,
    pub valid: u8,
}

const COOKIE_JOIN_SLOTS: usize = 1024;

static mut join_entries: [join_entry; COOKIE_JOIN_SLOTS] = unsafe { core::mem::zeroed() };
static mut join_entry_locks: [spinlock_t; COOKIE_JOIN_SLOTS] = unsafe { core::mem::zeroed() };
static mut mptcp_join_hash_secret: u32 = 0;

unsafe extern "C" {
    fn net_get_random_once(ptr: *mut u32, size: usize);
    fn tcp_hdr(skb: *mut sk_buff) -> *mut tcphdr;
    fn tcp_skb_cb(skb: *mut sk_buff) -> *mut tcp_skb_cb_t;
    fn net_hash_mix(net: *mut net) -> u32;
    fn jhash_3words(a: u32, b: u32, c: u32, initval: u32) -> u32;
    fn read_pnet(net: *const net) -> *mut net;
    fn spin_lock_bh(lock: *mut spinlock_t);
    fn spin_unlock_bh(lock: *mut spinlock_t);
    fn spin_lock_init(lock: *mut spinlock_t);
    fn mptcp_token_get_sock(net: *mut net, token: u32) -> *mut mptcp_sock;
}

// Opaque types and constants are provided by the surrounding translation.
extern "C" {
    type sk_buff;
    type tcphdr;
    type tcp_skb_cb_t;
    type net;
    type spinlock_t;
    type mptcp_subflow_request_sock;
    type mptcp_sock;
}

unsafe fn mptcp_join_entry_hash(skb: *mut sk_buff, net: *mut net) -> u32 {
    let th = tcp_hdr(skb);
    let seq: u32;
    if (*th).syn != 0 {
        seq = (*tcp_skb_cb(skb)).seq;
    } else {
        seq = (*tcp_skb_cb(skb)).seq.wrapping_sub(1);
    }

    let i = jhash_3words(
        seq,
        net_hash_mix(net),
        ((*th).source as u32) << 16 | (*th).dest as u32,
        mptcp_join_hash_secret,
    );

    i % COOKIE_JOIN_SLOTS as u32
}

unsafe fn mptcp_join_store_state(
    entry: *mut join_entry,
    subflow_req: *const mptcp_subflow_request_sock,
) {
    (*entry).token = (*subflow_req).token;
    (*entry).remote_nonce = (*subflow_req).remote_nonce;
    (*entry).local_nonce = (*subflow_req).local_nonce;
    (*entry).backup = (*subflow_req).backup;
    (*entry).join_id = (*subflow_req).remote_id;
    (*entry).local_id = (*subflow_req).local_id;
    (*entry).valid = 1;
}

pub unsafe fn subflow_init_req_cookie_join_save(
    subflow_req: *const mptcp_subflow_request_sock,
    skb: *mut sk_buff,
) {
    let net = read_pnet(&(*subflow_req).sk.req.ireq_net);
    let i = mptcp_join_entry_hash(skb, net) as usize;

    spin_lock_bh(&mut join_entry_locks[i]);
    mptcp_join_store_state(&mut join_entries[i], subflow_req);
    spin_unlock_bh(&mut join_entry_locks[i]);
}

pub unsafe fn mptcp_token_join_cookie_init_state(
    subflow_req: *mut mptcp_subflow_request_sock,
    skb: *mut sk_buff,
) -> bool {
    let net = read_pnet(&(*subflow_req).sk.req.ireq_net);
    let i = mptcp_join_entry_hash(skb, net) as usize;
    let e = &mut join_entries[i];

    spin_lock_bh(&mut join_entry_locks[i]);

    if e.valid == 0 {
        spin_unlock_bh(&mut join_entry_locks[i]);
        return false;
    }

    e.valid = 0;

    let msk = mptcp_token_get_sock(net, e.token);
    if msk.is_null() {
        spin_unlock_bh(&mut join_entry_locks[i]);
        return false;
    }

    (*subflow_req).remote_nonce = e.remote_nonce;
    (*subflow_req).local_nonce = e.local_nonce;
    (*subflow_req).backup = e.backup;
    (*subflow_req).remote_id = e.join_id;
    (*subflow_req).local_id = e.local_id;
    (*subflow_req).token = e.token;
    (*subflow_req).msk = msk;
    spin_unlock_bh(&mut join_entry_locks[i]);
    true
}

pub unsafe fn mptcp_join_cookie_init() {
    for i in 0..COOKIE_JOIN_SLOTS {
        spin_lock_init(&mut join_entry_locks[i]);
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
