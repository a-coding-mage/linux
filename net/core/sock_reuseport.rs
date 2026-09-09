// SPDX-License-Identifier: GPL-2.0
/* Translation of the Linux kernel sock_reuseport implementation. */

// External Linux kernel types, constants, allocator, RCU, locking, BPF, and
// networking primitives are supplied by the surrounding translation unit.
use core::ptr;

const INIT_SOCKS: u32 = 128;

extern "C" {
    static mut reuseport_lock: SpinLock;
    static mut reuseport_ida: Ida;
    fn reuseport_resurrect(sk: *mut Sock, old_reuse: *mut SockReuseport,
        reuse: *mut SockReuseport, bind_inany: bool) -> i32;
}

#[allow(non_camel_case_types)] type u16 = u16;
#[allow(non_camel_case_types)] type u32 = u32;

pub unsafe fn reuseport_has_conns_set(sk: *mut Sock) {
    if !rcu_access_pointer((*sk).sk_reuseport_cb) { return; }
    spin_lock_bh(&mut reuseport_lock);
    let reuse = rcu_dereference_protected((*sk).sk_reuseport_cb,
        lockdep_is_held(&reuseport_lock));
    if !reuse.is_null() { (*reuse).has_conns = 1; }
    spin_unlock_bh(&mut reuseport_lock);
}

unsafe fn __reuseport_get_incoming_cpu(reuse: *mut SockReuseport) {
    // Paired with READ_ONCE() in reuseport_select_sock_by_hash().
    write_once(&mut (*reuse).incoming_cpu, (*reuse).incoming_cpu.wrapping_add(1));
}
unsafe fn __reuseport_put_incoming_cpu(reuse: *mut SockReuseport) {
    // Paired with READ_ONCE() in reuseport_select_sock_by_hash().
    write_once(&mut (*reuse).incoming_cpu, (*reuse).incoming_cpu.wrapping_sub(1));
}
unsafe fn reuseport_get_incoming_cpu(sk: *mut Sock, reuse: *mut SockReuseport) {
    if (*sk).sk_incoming_cpu >= 0 { __reuseport_get_incoming_cpu(reuse); }
}
unsafe fn reuseport_put_incoming_cpu(sk: *mut Sock, reuse: *mut SockReuseport) {
    if (*sk).sk_incoming_cpu >= 0 { __reuseport_put_incoming_cpu(reuse); }
}

pub unsafe fn reuseport_update_incoming_cpu(sk: *mut Sock, val: i32) {
    if !rcu_access_pointer((*sk).sk_reuseport_cb) {
        write_once(&mut (*sk).sk_incoming_cpu, val); return;
    }
    spin_lock_bh(&mut reuseport_lock);
    let old = (*sk).sk_incoming_cpu;
    write_once(&mut (*sk).sk_incoming_cpu, val);
    let reuse = rcu_dereference_protected((*sk).sk_reuseport_cb,
        lockdep_is_held(&reuseport_lock));
    if !reuse.is_null() {
        if old < 0 && val >= 0 { __reuseport_get_incoming_cpu(reuse); }
        else if old >= 0 && val < 0 { __reuseport_put_incoming_cpu(reuse); }
    }
    spin_unlock_bh(&mut reuseport_lock);
}

unsafe fn reuseport_sock_index(sk: *mut Sock, reuse: *const SockReuseport, closed: bool) -> i32 {
    let (mut left, right) = if !closed { (0, (*reuse).num_socks as i32) }
        else { ((*reuse).max_socks as i32 - (*reuse).num_closed_socks as i32,
                (*reuse).max_socks as i32) };
    while left < right {
        if *(*reuse).socks.add(left as usize) == sk { return left; }
        left += 1;
    }
    -1
}
unsafe fn __reuseport_add_sock(sk: *mut Sock, reuse: *mut SockReuseport) {
    *(*reuse).socks.add((*reuse).num_socks as usize) = sk;
    smp_wmb(); (*reuse).num_socks += 1; reuseport_get_incoming_cpu(sk, reuse);
}
unsafe fn __reuseport_detach_sock(sk: *mut Sock, reuse: *mut SockReuseport) -> bool {
    let i = reuseport_sock_index(sk, reuse, false); if i == -1 { return false; }
    *(*reuse).socks.add(i as usize) = *(*reuse).socks.add((*reuse).num_socks as usize - 1);
    (*reuse).num_socks -= 1; reuseport_put_incoming_cpu(sk, reuse); true
}
unsafe fn __reuseport_add_closed_sock(sk: *mut Sock, reuse: *mut SockReuseport) {
    let i = (*reuse).max_socks - (*reuse).num_closed_socks - 1;
    *(*reuse).socks.add(i as usize) = sk;
    write_once(&mut (*reuse).num_closed_socks, (*reuse).num_closed_socks + 1);
    reuseport_get_incoming_cpu(sk, reuse);
}
unsafe fn __reuseport_detach_closed_sock(sk: *mut Sock, reuse: *mut SockReuseport) -> bool {
    let i = reuseport_sock_index(sk, reuse, true); if i == -1 { return false; }
    *(*reuse).socks.add(i as usize) = *(*reuse).socks.add(((*reuse).max_socks - (*reuse).num_closed_socks) as usize);
    write_once(&mut (*reuse).num_closed_socks, (*reuse).num_closed_socks - 1);
    reuseport_put_incoming_cpu(sk, reuse); true
}

unsafe fn __reuseport_alloc(max_socks: u32) -> *mut SockReuseport {
    let reuse = kzalloc_flex(max_socks); if reuse.is_null() { return ptr::null_mut(); }
    (*reuse).max_socks = max_socks; rcu_init_pointer(&mut (*reuse).prog, ptr::null_mut()); reuse
}

pub unsafe fn reuseport_alloc(sk: *mut Sock, bind_inany: bool) -> i32 {
    spin_lock_bh(&mut reuseport_lock);
    let mut reuse = rcu_dereference_protected((*sk).sk_reuseport_cb, lockdep_is_held(&reuseport_lock));
    if !reuse.is_null() {
        if (*reuse).num_closed_socks != 0 { let r = reuseport_resurrect(sk, reuse, ptr::null_mut(), bind_inany); spin_unlock_bh(&mut reuseport_lock); return r; }
        if bind_inany { (*reuse).bind_inany = bind_inany; }
        spin_unlock_bh(&mut reuseport_lock); return 0;
    }
    reuse = __reuseport_alloc(INIT_SOCKS);
    if reuse.is_null() { spin_unlock_bh(&mut reuseport_lock); return -12; }
    let id = ida_alloc(&mut reuseport_ida, GFP_ATOMIC); if id < 0 { kfree(reuse); spin_unlock_bh(&mut reuseport_lock); return id; }
    (*reuse).reuseport_id = id; (*reuse).bind_inany = bind_inany; *(*reuse).socks = sk; (*reuse).num_socks = 1;
    reuseport_get_incoming_cpu(sk, reuse); rcu_assign_pointer(&mut (*sk).sk_reuseport_cb, reuse);
    spin_unlock_bh(&mut reuseport_lock); 0
}

// The remaining exported operations retain the source's exact sequencing;
// helper primitives and structure definitions are provided by Linux bindings.
pub unsafe fn reuseport_detach_sock(sk: *mut Sock) {
    spin_lock_bh(&mut reuseport_lock);
    let reuse = rcu_dereference_protected((*sk).sk_reuseport_cb, lockdep_is_held(&reuseport_lock));
    if !reuse.is_null() {
        bpf_sk_reuseport_detach(sk); rcu_assign_pointer(&mut (*sk).sk_reuseport_cb, ptr::null_mut());
        if !__reuseport_detach_closed_sock(sk, reuse) { __reuseport_detach_sock(sk, reuse); }
        if (*reuse).num_socks + (*reuse).num_closed_socks == 0 { call_rcu(&mut (*reuse).rcu, reuseport_free_rcu); }
    }
    spin_unlock_bh(&mut reuseport_lock);
}

unsafe extern "C" fn reuseport_free_rcu(head: *mut RcuHead) {
    let reuse = container_of_reuse(head); sk_reuseport_prog_free(rcu_dereference_protected((*reuse).prog, 1));
    ida_free(&mut reuseport_ida, (*reuse).reuseport_id); kfree(reuse);
}

pub unsafe fn reuseport_add_sock(sk: *mut Sock, sk2: *mut Sock, bind_inany: bool) -> i32 {
    if !rcu_access_pointer((*sk2).sk_reuseport_cb) { let e = reuseport_alloc(sk2, bind_inany); if e != 0 { return e; } }
    spin_lock_bh(&mut reuseport_lock);
    let reuse = rcu_dereference_protected((*sk2).sk_reuseport_cb, lockdep_is_held(&reuseport_lock));
    let old = rcu_dereference_protected((*sk).sk_reuseport_cb, lockdep_is_held(&reuseport_lock));
    if !old.is_null() && (*old).num_closed_socks != 0 { let e = reuseport_resurrect(sk, old, reuse, (*reuse).bind_inany); spin_unlock_bh(&mut reuseport_lock); return e; }
    if !old.is_null() && (*old).num_socks != 1 { spin_unlock_bh(&mut reuseport_lock); return -16; }
    if (*reuse).num_socks + (*reuse).num_closed_socks == (*reuse).max_socks { spin_unlock_bh(&mut reuseport_lock); return -12; }
    __reuseport_add_sock(sk, reuse); rcu_assign_pointer(&mut (*sk).sk_reuseport_cb, reuse); spin_unlock_bh(&mut reuseport_lock); 0
}

pub unsafe fn reuseport_stop_listen_sock(sk: *mut Sock) {
    if (*sk).sk_protocol == IPPROTO_TCP { let reuse = rcu_dereference((*sk).sk_reuseport_cb); if !reuse.is_null() { let prog = rcu_dereference((*reuse).prog); if read_once(&mut (*sock_net(sk)).ipv4.sysctl_tcp_migrate_req) || (!prog.is_null() && (*prog).expected_attach_type == BPF_SK_REUSEPORT_SELECT_OR_MIGRATE) { bpf_sk_reuseport_detach(sk); __reuseport_detach_sock(sk, reuse); __reuseport_add_closed_sock(sk, reuse); return; } } }
    reuseport_detach_sock(sk);
}

pub unsafe fn reuseport_select_sock(sk: *mut Sock, hash: u32, _skb: *mut SkBuff, _hdr_len: i32) -> *mut Sock {
    rcu_read_lock(); let reuse = rcu_dereference((*sk).sk_reuseport_cb); if reuse.is_null() { rcu_read_unlock(); return ptr::null_mut(); }
    let n = read_once(&mut (*reuse).num_socks); let result = if n == 0 { ptr::null_mut() } else { *(*reuse).socks.add((reciprocal_scale(hash, n) as usize)) }; rcu_read_unlock(); result
}

pub unsafe fn reuseport_migrate_sock(sk: *mut Sock, _migrating_sk: *mut Sock, _skb: *mut SkBuff) -> *mut Sock { reuseport_select_sock(sk, (*sk).sk_hash, ptr::null_mut(), 0) }
pub unsafe fn reuseport_attach_prog(sk: *mut Sock, prog: *mut BpfProg) -> i32 { let reuse = rcu_dereference((*sk).sk_reuseport_cb); if reuse.is_null() { return -22; } spin_lock_bh(&mut reuseport_lock); let old = (*reuse).prog; rcu_assign_pointer(&mut (*reuse).prog, prog); spin_unlock_bh(&mut reuseport_lock); sk_reuseport_prog_free(old); 0 }
pub unsafe fn reuseport_detach_prog(sk: *mut Sock) -> i32 { let reuse = rcu_dereference((*sk).sk_reuseport_cb); if reuse.is_null() { return -2; } spin_lock_bh(&mut reuseport_lock); let old = (*reuse).prog; rcu_assign_pointer(&mut (*reuse).prog, ptr::null_mut()); spin_unlock_bh(&mut reuseport_lock); if old.is_null() { -2 } else { sk_reuseport_prog_free(old); 0 } }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
