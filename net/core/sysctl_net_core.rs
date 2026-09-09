// SPDX-License-Identifier: GPL-2.0
/* Translation of sysctl_net_core.c. Kernel-provided types, constants, and
 * functions referenced below are intentionally left as external dependencies.
 */

use core::ffi::{c_char, c_int, c_long, c_uint, c_ulong, c_void};

static mut INT_3600: c_int = 3600;
static mut MIN_SNDBUF: c_int = SOCK_MIN_SNDBUF;
static mut MIN_RCVBUF: c_int = SOCK_MIN_RCVBUF;
static mut MAX_SKB_FRAGS_LOCAL: c_int = MAX_SKB_FRAGS;
static mut MIN_MEM_PCPU_RSV: c_int = SK_MEMORY_PCPU_RESERVE;
static mut NETDEV_BUDGET_USECS_MIN: c_int = 2 * USEC_PER_SEC / HZ;
static mut NET_MSG_WARN: c_int = 0;

#[no_mangle]
pub static mut sysctl_fb_tunnels_only_for_init_net: c_int = 0;
#[no_mangle]
pub static mut sysctl_devconf_inherit_init_net: c_int = 0;

#[cfg(any(feature = "CONFIG_NET_FLOW_LIMIT", feature = "CONFIG_RPS"))]
unsafe fn dump_cpumask(buffer: *mut c_void, lenp: *mut usize, ppos: *mut loff_t,
                        mask: *mut cpumask) -> c_int {
    if *ppos != 0 || *lenp == 0 { *lenp = 0; return 0; }
    let mut len = core::cmp::min((((nr_cpumask_bits + 31) / 32) * 9), *lenp);
    let kbuf = kmalloc(len, GFP_KERNEL) as *mut c_char;
    if kbuf.is_null() { *lenp = 0; return -ENOMEM; }
    len = scnprintf(kbuf, len, c"%*pb", cpumask_pr_args(mask));
    if len == 0 { *lenp = 0; kfree(kbuf as *mut c_void); return 0; }
    *kbuf.add(len) = b'\n' as c_char;
    len += 1;
    memcpy(buffer, kbuf as *const c_void, len);
    *lenp = len;
    *ppos += len as loff_t;
    kfree(kbuf as *mut c_void);
    0
}

#[cfg(feature = "CONFIG_RPS")]
static mut rps_default_mask_mutex: mutex = DEFINE_MUTEX!();

#[cfg(feature = "CONFIG_RPS")]
unsafe fn rps_default_mask_sysctl(table: *const ctl_table, write: c_int, buffer: *mut c_void,
                                   lenp: *mut usize, ppos: *mut loff_t) -> c_int {
    let net = (*table).data as *mut net;
    mutex_lock(&raw mut rps_default_mask_mutex);
    let mut mask = (*net).core.rps_default_mask;
    let mut err = 0;
    if write != 0 {
        if mask.is_null() { mask = kzalloc(cpumask_size(), GFP_KERNEL) as *mut cpumask; (*net).core.rps_default_mask = mask; }
        if mask.is_null() { err = -ENOMEM; } else { err = cpumask_parse(buffer, mask); if err == 0 { err = rps_cpumask_housekeeping(mask); } }
    } else { err = dump_cpumask(buffer, lenp, ppos, if !mask.is_null() { mask } else { cpu_none_mask }); }
    mutex_unlock(&raw mut rps_default_mask_mutex);
    err
}

#[cfg(feature = "CONFIG_RPS")]
unsafe fn rps_sock_flow_sysctl(table: *const ctl_table, write: c_int, buffer: *mut c_void,
                               lenp: *mut usize, ppos: *mut loff_t) -> c_int {
    static mut SOCK_FLOW_MUTEX: mutex = DEFINE_MUTEX!();
    mutex_lock(&raw mut SOCK_FLOW_MUTEX);
    let old = net_hotdata.rps_sock_flow_table;
    let mut size = if old != 0 { rps_tag_to_mask(old) + 1 } else { 0 };
    let original = size;
    let mut tmp = ctl_table { data: &raw mut size as *mut c_void, maxlen: core::mem::size_of::<c_uint>(), mode: (*table).mode, ..zeroed() };
    let ret = proc_dointvec(&raw mut tmp, write, buffer, lenp, ppos);
    if write == 0 { mutex_unlock(&raw mut SOCK_FLOW_MUTEX); return ret; }
    if size > (1 << 29) { mutex_unlock(&raw mut SOCK_FLOW_MUTEX); return -EINVAL; }
    let mut tag = old;
    let mut sock_table = rps_tag_to_table(old);
    if size != 0 {
        size = roundup_pow_of_two(size);
        if size != original {
            sock_table = vmalloc_huge(size * core::mem::size_of::<rps_sock_flow_table>(), GFP_KERNEL) as *mut rps_sock_flow_table;
            if sock_table.is_null() { mutex_unlock(&raw mut SOCK_FLOW_MUTEX); return -ENOMEM; }
            net_hotdata.rps_cpu_mask = roundup_pow_of_two(nr_cpu_ids) - 1;
            tag = sock_table as usize | ilog2(size) as usize;
        }
        for i in 0..size { (*sock_table.add(i)).ent = RPS_NO_CPU; }
    } else { sock_table = core::ptr::null_mut(); tag = 0; }
    if tag != old {
        smp_store_release(&raw mut net_hotdata.rps_sock_flow_table, tag);
        if !sock_table.is_null() { static_branch_inc(&raw mut rps_needed); static_branch_inc(&raw mut rfs_needed); }
        if !rps_tag_to_table(old).is_null() { static_branch_dec(&raw mut rps_needed); static_branch_dec(&raw mut rfs_needed); }
    }
    mutex_unlock(&raw mut SOCK_FLOW_MUTEX);
    kvfree_rcu_mightsleep(rps_tag_to_table(old) as *mut c_void);
    ret
}

#[cfg(feature = "CONFIG_NET_FLOW_LIMIT")]
static mut flow_limit_update_mutex: mutex = DEFINE_MUTEX!();

#[cfg(feature = "CONFIG_NET_FLOW_LIMIT")]
unsafe fn flow_limit_table_len_sysctl(table: *const ctl_table, write: c_int, buffer: *mut c_void,
                                      lenp: *mut usize, ppos: *mut loff_t) -> c_int {
    mutex_lock(&raw mut flow_limit_update_mutex);
    let ptr = (*table).data as *mut c_uint;
    let old = *ptr;
    let mut ret = proc_dointvec(table, write, buffer, lenp, ppos);
    if ret == 0 && write != 0 && !is_power_of_2(*ptr) { *ptr = old; ret = -EINVAL; }
    mutex_unlock(&raw mut flow_limit_update_mutex);
    ret
}

#[cfg(feature = "CONFIG_NET_SCHED")]
unsafe fn set_default_qdisc(table: *const ctl_table, write: c_int, buffer: *mut c_void,
                            lenp: *mut usize, ppos: *mut loff_t) -> c_int {
    let mut id = [0 as c_char; IFNAMSIZ];
    qdisc_get_default(id.as_mut_ptr(), IFNAMSIZ);
    let mut tbl = ctl_table { data: id.as_mut_ptr() as *mut c_void, maxlen: IFNAMSIZ, ..zeroed() };
    let mut ret = proc_dostring(&raw mut tbl, write, buffer, lenp, ppos);
    if write != 0 && ret == 0 { ret = qdisc_set_default(id.as_ptr()); }
    ret
}

unsafe fn proc_do_dev_weight(table: *const ctl_table, write: c_int, buffer: *mut c_void,
                             lenp: *mut usize, ppos: *mut loff_t) -> c_int {
    static mut DEV_WEIGHT_MUTEX: mutex = DEFINE_MUTEX!();
    mutex_lock(&raw mut DEV_WEIGHT_MUTEX);
    let ret = proc_dointvec_minmax(table, write, buffer, lenp, ppos);
    if ret == 0 && write != 0 { let weight = READ_ONCE(weight_p); WRITE_ONCE(net_hotdata.dev_rx_weight, weight * dev_weight_rx_bias); WRITE_ONCE(net_hotdata.dev_tx_weight, weight * dev_weight_tx_bias); }
    mutex_unlock(&raw mut DEV_WEIGHT_MUTEX);
    ret
}

unsafe fn proc_do_rss_key(table: *const ctl_table, write: c_int, buffer: *mut c_void,
                          lenp: *mut usize, ppos: *mut loff_t) -> c_int {
    let mut buf = [0 as c_char; NETDEV_RSS_KEY_LEN * 3];
    let mut pos = buf.as_mut_ptr();
    for i in 0..NETDEV_RSS_KEY_LEN { pos = hex_byte_pack(pos, netdev_rss_key[i]); *pos = b':' as c_char; pos = pos.add(1); }
    *pos.sub(1) = 0;
    let mut fake = ctl_table { data: buf.as_mut_ptr() as *mut c_void, maxlen: buf.len(), ..zeroed() };
    proc_dostring(&raw mut fake, write, buffer, lenp, ppos)
}

unsafe fn proc_do_skb_defer_max(table: *const ctl_table, write: c_int, buffer: *mut c_void,
                                lenp: *mut usize, ppos: *mut loff_t) -> c_int {
    static mut M: mutex = DEFINE_MUTEX!();
    mutex_lock(&raw mut M);
    let old = net_hotdata.sysctl_skb_defer_max == 0;
    let ret = proc_dointvec_minmax(table, write, buffer, lenp, ppos);
    let new = net_hotdata.sysctl_skb_defer_max == 0;
    if new != old { if new { static_branch_enable(&raw mut skb_defer_disable_key); } else { static_branch_disable(&raw mut skb_defer_disable_key); } }
    mutex_unlock(&raw mut M); ret
}

#[cfg(feature = "CONFIG_BPF_JIT")]
unsafe fn proc_dointvec_minmax_bpf_enable(table: *const ctl_table, write: c_int, buffer: *mut c_void,
                                          lenp: *mut usize, ppos: *mut loff_t) -> c_int {
    let mut value = *((*table).data as *mut c_int);
    if write != 0 && capable(CAP_SYS_ADMIN) == 0 { return -EPERM; }
    let mut tmp = *table; tmp.data = &raw mut value as *mut c_void;
    let mut ret = proc_dointvec_minmax(&raw mut tmp, write, buffer, lenp, ppos);
    if write != 0 && ret == 0 { if value < 2 || (value == 2 && bpf_dump_raw_ok(current_cred()) != 0) { *((*table).data as *mut c_int) = value; } else { ret = -EPERM; } }
    ret
}

// The remaining sysctl table and per-network registration are direct data
// declarations; their kernel ABI types and handler symbols are supplied by
// the surrounding translation unit.
extern "C" {
    static mut net_core_table: [ctl_table; 0];
    static mut netns_core_table: [ctl_table; 0];
}

unsafe fn fb_tunnels_only_for_init_net_sysctl_setup(str_: *mut c_char) -> c_int {
    if strncmp(str_, c"initns".as_ptr(), 6) == 0 { sysctl_fb_tunnels_only_for_init_net = 1; }
    else if strncmp(str_, c"none".as_ptr(), 4) == 0 { sysctl_fb_tunnels_only_for_init_net = 2; }
    1
}

unsafe fn sysctl_core_init() -> c_int {
    register_net_sysctl(&raw mut init_net, c"net/core".as_ptr(), net_core_table.as_mut_ptr());
    register_pernet_subsys(&raw mut sysctl_core_ops)
}

// fs_initcall(sysctl_core_init)

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
