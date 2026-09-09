/* SPDX-License-Identifier: GPL-2.0-or-later */

// Dependencies supplied by the surrounding kernel translation:
// linux/types.h, linux/static_key.h, net/sock.h, net/hotdata.h,
// and, when CONFIG_RPS is enabled, net/rps-types.h.

#[cfg(CONFIG_RPS)]
extern "C" {
    pub static mut rps_needed: static_key_false;
    pub static mut rfs_needed: static_key_false;
}

/*
 * This structure holds an RPS map which can be of variable length.  The
 * map is an array of CPUs.
 */
#[repr(C)]
pub struct rps_map {
    pub len: ::core::ffi::c_uint,
    pub rcu: rcu_head,
    pub cpus: [u16; 0],
}

#[inline]
pub const fn RPS_MAP_SIZE(num: usize) -> usize {
    core::mem::size_of::<rps_map>() + (num * core::mem::size_of::<u16>())
}

/*
 * The rps_dev_flow structure contains the mapping of a flow to a CPU, the
 * tail pointer for that CPU's input queue at the time of last enqueue, a
 * hardware filter index, and the hash of the flow if aRFS is enabled.
 */
#[repr(C)]
pub struct rps_dev_flow {
    pub cpu: u16,
    pub filter: u16,
    pub last_qtail: ::core::ffi::c_uint,
    #[cfg(CONFIG_RFS_ACCEL)]
    pub hash: u32,
}

pub const RPS_NO_FILTER: u16 = 0xffff;

/*
 * The rps_sock_flow_table contains mappings of flows to the last CPU
 * on which they were processed by the application (set in recvmsg).
 * Each entry is a 32bit value. Upper part is the high-order bits
 * of flow hash, lower part is CPU number.
 * rps_cpu_mask is used to partition the space, depending on number of
 * possible CPUs : rps_cpu_mask = roundup_pow_of_two(nr_cpu_ids) - 1
 * For example, if 64 CPUs are possible, rps_cpu_mask = 0x3f,
 * meaning we use 32-6=26 bits for the hash.
 */
#[repr(C)]
pub struct rps_sock_flow_table {
    pub ent: u32,
}

pub const RPS_NO_CPU: u16 = 0xffff;

#[cfg(CONFIG_RPS)]
#[inline]
pub unsafe fn rps_record_sock_flow(tag_ptr: rps_tag_ptr, hash: u32) {
    let index: usize = (hash & rps_tag_to_mask(tag_ptr)) as usize;
    let mut val: u32 = hash & !net_hotdata.rps_cpu_mask;
    let table: *mut rps_sock_flow_table;

    /* We only give a hint, preemption can change CPU under us */
    val |= raw_smp_processor_id();

    table = rps_tag_to_table(tag_ptr);
    /* The following WRITE_ONCE() is paired with the READ_ONCE()
     * here, and another one in get_rps_cpu().
     */
    if READ_ONCE((*table.add(index)).ent) != val {
        WRITE_ONCE((*table.add(index)).ent, val);
    }
}

#[cfg(CONFIG_RPS)]
#[inline]
pub unsafe fn _sock_rps_record_flow_hash(hash: u32) {
    let tag_ptr: rps_tag_ptr;

    if hash == 0 {
        return;
    }
    rcu_read_lock();
    tag_ptr = READ_ONCE(net_hotdata.rps_sock_flow_table);
    if !tag_ptr.is_null() {
        rps_record_sock_flow(tag_ptr, hash);
    }
    rcu_read_unlock();
}

#[cfg(CONFIG_RPS)]
#[inline]
pub unsafe fn _sock_rps_record_flow(sk: *const sock) {
    /* Reading sk->sk_rxhash might incur an expensive cache line
     * miss.
     *
     * TCP_ESTABLISHED does cover almost all states where RFS
     * might be useful, and is cheaper [1] than testing :
     *	IPv4: inet_sk(sk)->inet_daddr
     *	IPv6: ipv6_addr_any(&sk->sk_v6_daddr)
     * OR	an additional socket flag
     * [1] : sk_state and sk_prot are in the same cache line.
     */
    if (*sk).sk_state == TCP_ESTABLISHED {
        /* This READ_ONCE() is paired with the WRITE_ONCE()
         * from sock_rps_save_rxhash() and sock_rps_reset_rxhash().
         */
        _sock_rps_record_flow_hash(READ_ONCE((*sk).sk_rxhash));
    }
}

#[cfg(CONFIG_RPS)]
#[inline]
pub unsafe fn _sock_rps_delete_flow(sk: *const sock) {
    let table: *mut rps_sock_flow_table;
    let tag_ptr: rps_tag_ptr;
    let hash: u32;
    let index: usize;

    hash = READ_ONCE((*sk).sk_rxhash);
    if hash == 0 {
        return;
    }

    rcu_read_lock();
    tag_ptr = READ_ONCE(net_hotdata.rps_sock_flow_table);
    if !tag_ptr.is_null() {
        index = (hash & rps_tag_to_mask(tag_ptr)) as usize;
        table = rps_tag_to_table(tag_ptr);
        if READ_ONCE((*table.add(index)).ent) != RPS_NO_CPU as u32 {
            WRITE_ONCE((*table.add(index)).ent, RPS_NO_CPU as u32);
        }
    }
    rcu_read_unlock();
}

#[inline]
pub unsafe fn rfs_is_needed() -> bool {
    // CONFIG_RPS controls whether the static branch is available.
    #[cfg(CONFIG_RPS)]
    {
        return static_branch_unlikely(&raw mut rfs_needed);
    }
    #[cfg(not(CONFIG_RPS))]
    {
        false
    }
}

#[inline]
pub unsafe fn sock_rps_record_flow_hash(hash: u32) {
    #[cfg(CONFIG_RPS)]
    {
        if !rfs_is_needed() { return; }
        _sock_rps_record_flow_hash(hash);
    }
}

#[inline]
pub unsafe fn sock_rps_record_flow(sk: *const sock) {
    #[cfg(CONFIG_RPS)]
    {
        if !rfs_is_needed() { return; }
        _sock_rps_record_flow(sk);
    }
}

#[inline]
pub unsafe fn sock_rps_delete_flow(sk: *const sock) {
    #[cfg(CONFIG_RPS)]
    {
        if !rfs_is_needed() { return; }
        _sock_rps_delete_flow(sk);
    }
}

#[inline]
pub unsafe fn rps_input_queue_tail_incr(sd: *mut softnet_data) -> u32 {
    #[cfg(CONFIG_RPS)]
    {
        (*sd).input_queue_tail = (*sd).input_queue_tail.wrapping_add(1);
        return (*sd).input_queue_tail;
    }
    #[cfg(not(CONFIG_RPS))]
    { 0 }
}

#[inline]
pub unsafe fn rps_input_queue_tail_save(dest: *mut u32, tail: u32) {
    #[cfg(CONFIG_RPS)]
    { WRITE_ONCE(*dest, tail); }
}

#[inline]
pub unsafe fn rps_input_queue_head_add(sd: *mut softnet_data, val: i32) {
    #[cfg(CONFIG_RPS)]
    { WRITE_ONCE((*sd).input_queue_head, (*sd).input_queue_head.wrapping_add(val as u32)); }
}

#[inline]
pub unsafe fn rps_input_queue_head_incr(sd: *mut softnet_data) {
    rps_input_queue_head_add(sd, 1);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
