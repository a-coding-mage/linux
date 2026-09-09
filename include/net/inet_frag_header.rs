/* SPDX-License-Identifier: GPL-2.0 */

/* Dependencies supplied by other translated headers. */

/* Per netns frag queues directory */
#[repr(C)]
pub struct fqdir {
    /* sysctls */
    pub high_thresh: ::core::ffi::c_long,
    pub low_thresh: ::core::ffi::c_long,
    pub timeout: ::core::ffi::c_int,
    pub max_dist: ::core::ffi::c_int,
    pub f: *mut inet_frags,
    pub net: *mut net,
    pub dead: bool,

    pub rhashtable: rhashtable,

    /* Keep atomic mem on separate cachelines in structs that include it */
    pub mem: atomic_long_t,
    pub destroy_work: work_struct,
    pub free_list: llist_node,
}

/**
 * enum: fragment queue flags
 *
 * @INET_FRAG_FIRST_IN: first fragment has arrived
 * @INET_FRAG_LAST_IN: final fragment has arrived
 * @INET_FRAG_COMPLETE: frag queue has been processed and is due for destruction
 * @INET_FRAG_HASH_DEAD: inet_frag_kill() has not removed fq from rhashtable
 * @INET_FRAG_DROP: if skbs must be dropped (instead of being consumed)
 */
pub const INET_FRAG_FIRST_IN: u32 = 1 << 0;
pub const INET_FRAG_LAST_IN: u32 = 1 << 1;
pub const INET_FRAG_COMPLETE: u32 = 1 << 2;
pub const INET_FRAG_HASH_DEAD: u32 = 1 << 3;
pub const INET_FRAG_DROP: u32 = 1 << 4;

#[repr(C)]
pub struct frag_v4_compare_key {
    pub saddr: __be32,
    pub daddr: __be32,
    pub user: u32,
    pub vif: u32,
    pub id: __be16,
    pub protocol: u16,
}

#[repr(C)]
pub struct frag_v6_compare_key {
    pub saddr: in6_addr,
    pub daddr: in6_addr,
    pub user: u32,
    pub id: __be32,
    pub iif: u32,
}

#[repr(C)]
pub union inet_frag_queue_key {
    pub v4: frag_v4_compare_key,
    pub v6: frag_v6_compare_key,
}

/**
 * struct inet_frag_queue - fragment queue
 *
 * @node: rhash node
 * @key: keys identifying this frag.
 * @timer: queue expiration timer
 * @lock: spinlock protecting this frag
 * @refcnt: reference count of the queue
 * @rb_fragments: received fragments rb-tree root
 * @fragments_tail: received fragments tail
 * @last_run_head: the head of the last "run". see ip_fragment.c
 * @stamp: timestamp of the last received fragment
 * @len: total length of the original datagram
 * @meat: length of received fragments so far
 * @tstamp_type: stamp has a mono delivery time (EDT)
 * @flags: fragment queue flags
 * @max_size: maximum received fragment size
 * @fqdir: pointer to struct fqdir
 * @rcu: rcu head for freeing deferall
 */
#[repr(C)]
pub struct inet_frag_queue {
    pub node: rhash_head,
    pub key: inet_frag_queue_key,
    pub timer: timer_list,
    pub lock: spinlock_t,
    pub refcnt: refcount_t,
    pub rb_fragments: rb_root,
    pub fragments_tail: *mut sk_buff,
    pub last_run_head: *mut sk_buff,
    pub stamp: ktime_t,
    pub len: ::core::ffi::c_int,
    pub meat: ::core::ffi::c_int,
    pub tstamp_type: u8,
    pub flags: __u8,
    pub max_size: u16,
    pub fqdir: *mut fqdir,
    pub rcu: rcu_head,
}

#[repr(C)]
pub struct inet_frags {
    pub qsize: ::core::ffi::c_uint,
    pub constructor: Option<unsafe extern "C" fn(*mut inet_frag_queue, *const ::core::ffi::c_void)>,
    pub destructor: Option<unsafe extern "C" fn(*mut inet_frag_queue)>,
    pub frag_expire: Option<unsafe extern "C" fn(*mut timer_list)>,
    pub frags_cachep: *mut kmem_cache,
    pub frags_cache_name: *const ::core::ffi::c_char,
    pub rhash_params: rhashtable_params,
    pub refcnt: refcount_t,
    pub completion: completion,
}

extern "C" {
    pub fn inet_frags_init(f: *mut inet_frags) -> ::core::ffi::c_int;
    pub fn inet_frags_fini(f: *mut inet_frags);
    pub fn fqdir_init(fqdirp: *mut *mut fqdir, f: *mut inet_frags, net: *mut net) -> ::core::ffi::c_int;
    pub fn fqdir_pre_exit(fqdir: *mut fqdir);
    pub fn fqdir_exit(fqdir: *mut fqdir);
    pub fn inet_frag_kill(q: *mut inet_frag_queue, refs: *mut ::core::ffi::c_int);
    pub fn inet_frag_destroy(q: *mut inet_frag_queue);
    pub fn inet_frag_find(fqdir: *mut fqdir, key: *mut ::core::ffi::c_void) -> *mut inet_frag_queue;
    pub fn inet_frag_queue_flush(q: *mut inet_frag_queue, reason: skb_drop_reason);

    pub fn refcount_sub_and_test(refs: ::core::ffi::c_int, r: *mut refcount_t) -> bool;
    pub fn atomic_long_read(v: *const atomic_long_t) -> ::core::ffi::c_long;
    pub fn atomic_long_sub(val: ::core::ffi::c_long, v: *mut atomic_long_t);
    pub fn atomic_long_add(val: ::core::ffi::c_long, v: *mut atomic_long_t);

    pub fn inet_frag_queue_insert(q: *mut inet_frag_queue, skb: *mut sk_buff, offset: ::core::ffi::c_int, end: ::core::ffi::c_int) -> ::core::ffi::c_int;
    pub fn inet_frag_reasm_prepare(q: *mut inet_frag_queue, skb: *mut sk_buff, parent: *mut sk_buff) -> *mut ::core::ffi::c_void;
    pub fn inet_frag_reasm_finish(q: *mut inet_frag_queue, head: *mut sk_buff, reasm_data: *mut ::core::ffi::c_void, try_coalesce: bool);
    pub fn inet_frag_pull_head(q: *mut inet_frag_queue) -> *mut sk_buff;
}

pub unsafe fn inet_frag_putn(q: *mut inet_frag_queue, refs: ::core::ffi::c_int) {
    if refs != 0 && refcount_sub_and_test(refs, &mut (*q).refcnt) {
        inet_frag_destroy(q);
    }
}

/* Memory Tracking Functions. */
pub unsafe fn frag_mem_limit(fqdir: *const fqdir) -> ::core::ffi::c_long {
    atomic_long_read(&(*fqdir).mem)
}
pub unsafe fn sub_frag_mem_limit(fqdir: *mut fqdir, val: ::core::ffi::c_long) {
    atomic_long_sub(val, &mut (*fqdir).mem)
}
pub unsafe fn add_frag_mem_limit(fqdir: *mut fqdir, val: ::core::ffi::c_long) {
    atomic_long_add(val, &mut (*fqdir).mem)
}

/* RFC 3168 support :
 * We want to check ECN values of all fragments, do detect invalid combinations.
 * In ipq->ecn, we store the OR value of each ip4_frag_ecn() fragment value.
 */
pub const IPFRAG_ECN_NOT_ECT: u8 = 0x01;
pub const IPFRAG_ECN_ECT_1: u8 = 0x02;
pub const IPFRAG_ECN_ECT_0: u8 = 0x04;
pub const IPFRAG_ECN_CE: u8 = 0x08;

pub static mut ip_frag_ecn_table: [u8; 16] = [0; 16];

/* Return values of inet_frag_queue_insert() */
pub const IPFRAG_OK: ::core::ffi::c_int = 0;
pub const IPFRAG_DUP: ::core::ffi::c_int = 1;
pub const IPFRAG_OVERLAP: ::core::ffi::c_int = 2;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
