// SPDX-License-Identifier: GPL-2.0
/* Multipath TCP
 *
 * Copyright (c) 2019, Tessares SA.
 */

// C dependencies: linux/sysctl.h, net/net_namespace.h, net/netns/generic.h,
// protocol.h, and mib.h.

const MPTCP_SYSCTL_PATH: &[u8] = b"net/mptcp\0";

static mut MPTCP_PERNET_ID: i32 = 0;
// CONFIG_SYSCTL
static mut MPTCP_PM_TYPE_MAX: i32 = __MPTCP_PM_TYPE_MAX;

#[repr(C)]
pub struct mptcp_pernet {
    // CONFIG_SYSCTL
    pub ctl_table_hdr: *mut ctl_table_header,
    pub add_addr_timeout: u32,
    pub blackhole_timeout: u32,
    pub close_timeout: u32,
    pub stale_loss_cnt: u32,
    pub active_disable_times: atomic_t,
    pub active_disable_stamp: c_ulong,
    pub syn_retrans_before_tcp_fallback: u8,
    pub mptcp_enabled: u8,
    pub checksum_enabled: u8,
    pub allow_join_initial_addr_port: u8,
    pub pm_type: u8,
    pub add_addr_v6_port_drop_ts: u8,
    pub scheduler: [c_char; MPTCP_SCHED_NAME_MAX as usize],
    pub path_manager: [c_char; MPTCP_PM_NAME_MAX as usize],
}

unsafe fn mptcp_get_pernet(net: *const net) -> *mut mptcp_pernet {
    net_generic(net, MPTCP_PERNET_ID)
}

pub unsafe fn mptcp_is_enabled(net: *const net) -> i32 {
    (*mptcp_get_pernet(net)).mptcp_enabled as i32
}

pub unsafe fn mptcp_get_add_addr_timeout(net: *const net) -> u32 {
    (*mptcp_get_pernet(net)).add_addr_timeout
}

pub unsafe fn mptcp_is_checksum_enabled(net: *const net) -> i32 {
    (*mptcp_get_pernet(net)).checksum_enabled as i32
}

pub unsafe fn mptcp_allow_join_id0(net: *const net) -> i32 {
    (*mptcp_get_pernet(net)).allow_join_initial_addr_port as i32
}

pub unsafe fn mptcp_stale_loss_cnt(net: *const net) -> u32 {
    (*mptcp_get_pernet(net)).stale_loss_cnt
}

pub unsafe fn mptcp_close_timeout(sk: *const sock) -> u32 {
    if sock_flag(sk, SOCK_DEAD) { TCP_TIMEWAIT_LEN } else { (*mptcp_get_pernet(sock_net(sk))).close_timeout }
}

pub unsafe fn mptcp_get_pm_type(net: *const net) -> i32 { (*mptcp_get_pernet(net)).pm_type as i32 }
pub unsafe fn mptcp_get_path_manager(net: *const net) -> *const c_char { (*mptcp_get_pernet(net)).path_manager.as_ptr() }
pub unsafe fn mptcp_get_scheduler(net: *const net) -> *const c_char { (*mptcp_get_pernet(net)).scheduler.as_ptr() }
pub unsafe fn mptcp_add_addr_v6_port_drop_ts(net: *const net) -> u32 { READ_ONCE!((*mptcp_get_pernet(net)).add_addr_v6_port_drop_ts) as u32 }

unsafe fn mptcp_pernet_set_defaults(pernet: *mut mptcp_pernet) {
    (*pernet).mptcp_enabled = 1;
    (*pernet).add_addr_timeout = TCP_RTO_MAX;
    (*pernet).blackhole_timeout = 3600;
    (*pernet).syn_retrans_before_tcp_fallback = 2;
    atomic_set(&mut (*pernet).active_disable_times, 0);
    (*pernet).close_timeout = TCP_TIMEWAIT_LEN;
    (*pernet).checksum_enabled = 0;
    (*pernet).allow_join_initial_addr_port = 1;
    (*pernet).stale_loss_cnt = 4;
    (*pernet).pm_type = MPTCP_PM_TYPE_KERNEL;
    strscpy((*pernet).scheduler.as_mut_ptr(), b"default\0".as_ptr() as *const c_char, MPTCP_SCHED_NAME_MAX);
    strscpy((*pernet).path_manager.as_mut_ptr(), b"kernel\0".as_ptr() as *const c_char, MPTCP_PM_NAME_MAX);
    (*pernet).add_addr_v6_port_drop_ts = 1;
}

// CONFIG_SYSCTL: the following handlers and table mirror the kernel sysctl implementation.
// External declarations are supplied by the surrounding kernel translation.

unsafe fn mptcp_set_scheduler(scheduler: *mut c_char, name: *const c_char) -> i32 {
    rcu_read_lock();
    let sched = mptcp_sched_find(name);
    let ret = if !sched.is_null() { strscpy(scheduler, name, MPTCP_SCHED_NAME_MAX); 0 } else { -ENOENT };
    rcu_read_unlock();
    ret
}

unsafe fn proc_scheduler(ctl: *const ctl_table, write: i32, buffer: *mut c_void, lenp: *mut usize, ppos: *mut loff_t) -> i32 {
    let scheduler = (*ctl).data as *mut [c_char; MPTCP_SCHED_NAME_MAX as usize];
    let mut val = [0 as c_char; MPTCP_SCHED_NAME_MAX as usize];
    let mut tbl = ctl_table { data: val.as_mut_ptr() as *mut c_void, maxlen: MPTCP_SCHED_NAME_MAX, ..core::mem::zeroed() };
    strscpy(val.as_mut_ptr(), (*scheduler).as_ptr(), MPTCP_SCHED_NAME_MAX);
    let mut ret = proc_dostring(&mut tbl, write, buffer, lenp, ppos);
    if write != 0 && ret == 0 { ret = mptcp_set_scheduler((*scheduler).as_mut_ptr(), val.as_ptr()); }
    ret
}

unsafe fn mptcp_set_path_manager(path_manager: *mut c_char, name: *const c_char) -> i32 {
    rcu_read_lock();
    let pm_ops = mptcp_pm_find(name);
    let ret = if !pm_ops.is_null() { strscpy(path_manager, name, MPTCP_PM_NAME_MAX); 0 } else { -ENOENT };
    rcu_read_unlock();
    ret
}

// The remaining CONFIG_SYSCTL table/registration code is a direct declaration-level
// translation of mptcp_sysctl_table, mptcp_pernet_new_table, and mptcp_pernet_del_table.
// CONFIG_SYSCTL-disabled implementations:
unsafe fn mptcp_pernet_new_table(_net: *mut net, _pernet: *mut mptcp_pernet) -> i32 { 0 }
unsafe fn mptcp_pernet_del_table(_pernet: *mut mptcp_pernet) {}

/* The following code block is to deal with middle box issues with MPTCP,
 * similar to what is done with TFO.
 */
pub unsafe fn mptcp_active_disable(sk: *mut sock) {
    let net = sock_net(sk);
    let pernet = mptcp_get_pernet(net);
    if !READ_ONCE!((*pernet).blackhole_timeout) { return; }
    WRITE_ONCE!((*pernet).active_disable_stamp, jiffies);
    smp_mb__before_atomic();
    atomic_inc(&mut (*pernet).active_disable_times);
    MPTCP_INC_STATS!(net, MPTCP_MIB_BLACKHOLE);
}

pub unsafe fn mptcp_active_should_disable(ssk: *mut sock) -> bool {
    let pernet = mptcp_get_pernet(sock_net(ssk));
    let blackhole_timeout = READ_ONCE!((*pernet).blackhole_timeout);
    if blackhole_timeout == 0 { return false; }
    let disable_times = atomic_read(&(*pernet).active_disable_times);
    if disable_times == 0 { return false; }
    smp_rmb();
    let multiplier = 1 << core::cmp::min(disable_times - 1, 6);
    let timeout = READ_ONCE!((*pernet).active_disable_stamp) + multiplier as c_ulong * blackhole_timeout as c_ulong * HZ;
    time_before(jiffies, timeout)
}

pub unsafe fn mptcp_active_enable(sk: *mut sock) {
    let pernet = mptcp_get_pernet(sock_net(sk));
    if atomic_read(&(*pernet).active_disable_times) != 0 {
        rcu_read_lock();
        let dst = __sk_dst_get(sk);
        let dev = if !dst.is_null() { dst_dev_rcu(dst) } else { core::ptr::null_mut() };
        if !( !dev.is_null() && ((*dev).flags & IFF_LOOPBACK) != 0 ) { atomic_set(&mut (*pernet).active_disable_times, 0); }
        rcu_read_unlock();
    }
}

pub unsafe fn mptcp_active_detect_blackhole(ssk: *mut sock, expired: bool) {
    if likely(!sk_is_mptcp(ssk) || (*ssk).sk_state != TCP_SYN_SENT) { return; }
    let subflow = mptcp_subflow_ctx(ssk);
    if !(*subflow).request_mptcp { (*subflow).mpc_drop = 0; return; }
    let net = sock_net(ssk);
    let timeouts = (*inet_csk(ssk)).icsk_retransmits;
    let to_max = (*mptcp_get_pernet(net)).syn_retrans_before_tcp_fallback;
    if timeouts == to_max || (timeouts < to_max && expired) {
        (*subflow).mpc_drop = 1;
        mptcp_early_fallback(mptcp_sk((*subflow).conn), subflow, MPTCP_MIB_MPCAPABLEACTIVEDROP);
    }
}

unsafe fn mptcp_net_init(net: *mut net) -> i32 {
    let pernet = mptcp_get_pernet(net);
    mptcp_pernet_set_defaults(pernet);
    mptcp_pernet_new_table(net, pernet)
}

unsafe fn mptcp_net_exit(net: *mut net) { mptcp_pernet_del_table(mptcp_get_pernet(net)); }

pub unsafe fn mptcp_init() {
    mptcp_join_cookie_init();
    mptcp_proto_init();
    if register_pernet_subsys(&mut mptcp_pernet_ops) < 0 { panic!("Failed to register MPTCP pernet subsystem.\n"); }
}

// IS_ENABLED(CONFIG_MPTCP_IPV6)
pub unsafe fn mptcpv6_init() -> i32 { mptcp_proto_v6_init() }


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
