// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * taskstats.c - Export per-task statistics to userland
 *
 * Copyright (C) Shailabh Nagar, IBM Corp. 2006
 *           (C) Balbir Singh,   IBM Corp. 2006
 */

// Kernel headers and symbols are supplied by the surrounding kernel bindings.
// Build-time kernel configuration and registration macros are preserved by
// their corresponding Rust declarations/calls below.

const TASKSTATS_CPUMASK_MAXLEN: usize = 100 + 6 * NR_CPUS;

static mut TASKSTATS_SEQNUM: PerCpu<u32> = DEFINE_PER_CPU!();
static mut family_registered: i32 = 0;
static mut taskstats_cache: *mut kmem_cache = core::ptr::null_mut();

static mut family: genl_family = genl_family::zeroed();

static taskstats_cmd_get_policy: [nla_policy; 5] = [
    nla_policy { type_: NLA_UNSPEC },
    nla_policy { type_: NLA_U32 },
    nla_policy { type_: NLA_U32 },
    nla_policy { type_: NLA_STRING },
    nla_policy { type_: NLA_STRING },
];

static cgroupstats_cmd_get_policy: [nla_policy; 2] = [
    nla_policy { type_: NLA_UNSPEC },
    nla_policy { type_: NLA_U32 },
];

#[repr(C)]
struct listener {
    list: list_head,
    pid: pid_t,
    valid: i8,
}

#[repr(C)]
struct listener_list {
    sem: rw_semaphore,
    list: list_head,
}

static mut listener_array: PerCpu<listener_list> = DEFINE_PER_CPU!();

#[repr(C)]
enum actions { REGISTER, DEREGISTER, CPU_DONT_CARE }

unsafe fn prepare_reply(info: *mut genl_info, cmd: u8, skbp: *mut *mut sk_buff, size: usize) -> i32 {
    let skb = genlmsg_new(size, GFP_KERNEL);
    if skb.is_null() { return -ENOMEM; }
    let reply = if info.is_null() {
        let seq = this_cpu_inc_return(&mut TASKSTATS_SEQNUM) - 1;
        genlmsg_put(skb, 0, seq, &mut family, 0, cmd)
    } else { genlmsg_put_reply(skb, info, &mut family, 0, cmd) };
    if reply.is_null() { nlmsg_free(skb); return -EINVAL; }
    *skbp = skb; 0
}

unsafe fn send_reply(skb: *mut sk_buff, info: *mut genl_info) -> i32 {
    let genlhdr = nlmsg_data(nlmsg_hdr(skb));
    let reply = genlmsg_data(genlhdr);
    genlmsg_end(skb, reply);
    genlmsg_reply(skb, info)
}

unsafe fn send_cpu_listeners(skb: *mut sk_buff, listeners: *mut listener_list) {
    let genlhdr = nlmsg_data(nlmsg_hdr(skb));
    let mut skb_cur = skb;
    let mut delcount = 0;
    let reply = genlmsg_data(genlhdr);
    genlmsg_end(skb, reply);
    down_read(&mut (*listeners).sem);
    let mut s = list_first_entry(&(*listeners).list);
    while !s.is_null() {
        let mut skb_next = core::ptr::null_mut();
        if !list_is_last(&(*s).list, &(*listeners).list) {
            skb_next = skb_clone(skb_cur, GFP_KERNEL);
            if skb_next.is_null() { break; }
        }
        let rc = genlmsg_unicast(&init_net, skb_cur, (*s).pid);
        if rc == -ECONNREFUSED { (*s).valid = 0; delcount += 1; }
        skb_cur = skb_next;
        s = list_next_entry(s);
    }
    up_read(&mut (*listeners).sem);
    if !skb_cur.is_null() { nlmsg_free(skb_cur); }
    if delcount == 0 { return; }
    down_write(&mut (*listeners).sem);
    let mut s = list_first_entry(&(*listeners).list);
    while !s.is_null() {
        let next = list_next_entry(s);
        if (*s).valid == 0 { list_del(&mut (*s).list); kfree(s as *mut _); }
        s = next;
    }
    up_write(&mut (*listeners).sem);
}

unsafe fn exe_add_tsk(stats: *mut taskstats, tsk: *mut task_struct) {
    let exe_file = get_task_exe_file(tsk);
    if !exe_file.is_null() {
        (*stats).ac_exe_dev = huge_encode_dev((*(*exe_file).f_inode).i_sb.s_dev);
        (*stats).ac_exe_inode = (*(*exe_file).f_inode).i_ino;
        fput(exe_file);
    } else { (*stats).ac_exe_dev = 0; (*stats).ac_exe_inode = 0; }
}

unsafe fn fill_stats(user_ns: *mut user_namespace, pid_ns: *mut pid_namespace, tsk: *mut task_struct, stats: *mut taskstats) {
    memset(stats as *mut _, 0, core::mem::size_of::<taskstats>());
    delayacct_add_tsk(stats, tsk);
    (*stats).version = TASKSTATS_VERSION;
    (*stats).nvcsw = (*tsk).nvcsw;
    (*stats).nivcsw = (*tsk).nivcsw;
    bacct_add_tsk(user_ns, pid_ns, stats, tsk);
    xacct_add_tsk(stats, tsk);
    exe_add_tsk(stats, tsk);
}

unsafe fn fill_stats_for_pid(pid: pid_t, stats: *mut taskstats) -> i32 {
    let tsk = find_get_task_by_vpid(pid);
    if tsk.is_null() { return -ESRCH; }
    fill_stats(current_user_ns(), task_active_pid_ns(current), tsk, stats);
    put_task_struct(tsk); 0
}

unsafe fn tgid_stats_add_task(stats: *mut taskstats, tsk: *mut task_struct, now_ns: u64) {
    delayacct_add_tsk(stats, tsk);
    let mut delta = now_ns.wrapping_sub((*tsk).start_time);
    delta /= NSEC_PER_USEC;
    (*stats).ac_etime += delta;
    let (mut utime, mut stime) = (0u64, 0u64);
    task_cputime(tsk, &mut utime, &mut stime);
    (*stats).ac_utime += utime / NSEC_PER_USEC;
    (*stats).ac_stime += stime / NSEC_PER_USEC;
    (*stats).nvcsw += (*tsk).nvcsw;
    (*stats).nivcsw += (*tsk).nivcsw;
}

unsafe fn fill_stats_for_tgid(tgid: pid_t, stats: *mut taskstats) -> i32 {
    rcu_read_lock();
    let first = find_task_by_vpid(tgid);
    if first.is_null() { rcu_read_unlock(); (*stats).version = TASKSTATS_VERSION; return -ESRCH; }
    let mut flags = 0;
    if !lock_task_sighand(first, &mut flags) { rcu_read_unlock(); (*stats).version = TASKSTATS_VERSION; return -ESRCH; }
    if !(*(*first).signal).stats.is_null() { memcpy(stats as *mut _, (*(*first).signal).stats as *const _, core::mem::size_of::<taskstats>()); }
    else { memset(stats as *mut _, 0, core::mem::size_of::<taskstats>()); }
    let now_ns = ktime_get_ns();
    for_each_thread!(first, tsk, { if (*tsk).exit_state == 0 { tgid_stats_add_task(stats, tsk, now_ns); } });
    unlock_task_sighand(first, &mut flags); rcu_read_unlock(); (*stats).version = TASKSTATS_VERSION; 0
}

unsafe fn taskstats_packet_size() -> usize { nla_total_size(core::mem::size_of::<u32>()) + nla_total_size_64bit(core::mem::size_of::<taskstats>()) + nla_total_size(0) }

#[no_mangle]
pub unsafe extern "C" fn taskstats_exit(tsk: *mut task_struct, group_dead: i32) {
    if family_registered == 0 { return; }
    let mut size = taskstats_packet_size();
    let is_thread_group = !taskstats_tgid_alloc(tsk).is_null();
    if is_thread_group { size *= 2; fill_tgid_exit(tsk); }
    let listeners = raw_cpu_ptr(&mut listener_array);
    if list_empty(&(*listeners).list) { return; }
    let mut rep_skb = core::ptr::null_mut();
    if prepare_reply(core::ptr::null_mut(), TASKSTATS_CMD_NEW, &mut rep_skb, size) < 0 { return; }
    let stats = mk_reply(rep_skb, TASKSTATS_TYPE_PID, task_pid_nr_ns(tsk, &init_pid_ns));
    if stats.is_null() { nlmsg_free(rep_skb); return; }
    fill_stats(&init_user_ns, &init_pid_ns, tsk, stats);
    if group_dead != 0 { (*stats).ac_flag |= AGROUP; }
    if is_thread_group && group_dead != 0 {
        let stats2 = mk_reply(rep_skb, TASKSTATS_TYPE_TGID, task_tgid_nr_ns(tsk, &init_pid_ns));
        if stats2.is_null() { nlmsg_free(rep_skb); return; }
        memcpy(stats2 as *mut _, (*(*tsk).signal).stats as *const _, core::mem::size_of::<taskstats>());
        (*stats2).version = TASKSTATS_VERSION;
    }
    send_cpu_listeners(rep_skb, listeners);
}

unsafe fn fill_tgid_exit(tsk: *mut task_struct) {
    let mut flags = 0;
    spin_lock_irqsave(&mut (*(*tsk).sighand).siglock, &mut flags);
    if !(*(*tsk).signal).stats.is_null() { tgid_stats_add_task((*(*tsk).signal).stats, tsk, ktime_get_ns()); }
    spin_unlock_irqrestore(&mut (*(*tsk).sighand).siglock, flags);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
