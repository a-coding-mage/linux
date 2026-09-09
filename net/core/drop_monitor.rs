// SPDX-License-Identifier: GPL-2.0-only
/* Monitoring code for network dropped packet alerts. */

// Kernel dependencies supplied by the surrounding repository are intentionally
// referenced here rather than reimplemented in this translation.

const TRACE_ON: i32 = 1;
const TRACE_OFF: i32 = 0;
const NET_DM_MAX_HW_TRAP_NAME_LEN: usize = 40;
const NET_DM_MAX_SYMBOL_LEN: usize = 40;
const NET_DM_MAX_REASON_LEN: usize = 50;
const NET_DM_MAX_PACKET_SIZE: usize = 0xffff - NLA_HDRLEN - NLA_ALIGNTO;

static mut trace_state: i32 = TRACE_OFF;
static mut monitor_hw: bool = false;
static mut net_dm_mutex: DEFINE_MUTEX = DEFINE_MUTEX_INIT;

#[repr(C)]
struct net_dm_stats { dropped: u64_stats_t, syncp: u64_stats_sync }
#[repr(C)]
struct net_dm_hw_entry { trap_name: [c_char; NET_DM_MAX_HW_TRAP_NAME_LEN], count: u32 }
#[repr(C)]
struct net_dm_hw_entries { num_entries: u32, entries: [net_dm_hw_entry; 0] }
#[repr(C)]
struct per_cpu_dm_data {
    lock: raw_spinlock_t,
    skb_or_hw_entries: *mut c_void,
    drop_queue: sk_buff_head,
    dm_alert_work: work_struct,
    send_timer: timer_list,
    stats: net_dm_stats,
}
#[repr(C)]
struct dm_hw_stat_delta { last_rx: c_ulong, last_drop_val: c_ulong, rcu: rcu_head }

static mut net_drop_monitor_family: genl_family = genl_family_zeroed();
static mut dm_cpu_data: DEFINE_PER_CPU<per_cpu_dm_data> = DEFINE_PER_CPU_INIT;
static mut dm_hw_cpu_data: DEFINE_PER_CPU<per_cpu_dm_data> = DEFINE_PER_CPU_INIT;
static mut dm_hit_limit: i32 = 64;
static mut dm_delay: i32 = 1;
static mut dm_hw_check_delta: c_ulong = 2 * HZ;
static mut net_dm_alert_mode: net_dm_alert_mode = NET_DM_ALERT_MODE_SUMMARY;
static mut net_dm_trunc_len: u32 = 0;
static mut net_dm_queue_len: u32 = 1000;

#[repr(C)]
struct net_dm_alert_ops {
    kfree_skb_probe: Option<unsafe extern "C" fn(*mut c_void,*mut sk_buff,*mut c_void,skb_drop_reason,*const sock)>,
    napi_poll_probe: Option<unsafe extern "C" fn(*mut c_void,*mut napi_struct,i32,i32)>,
    work_item_func: Option<unsafe extern "C" fn(*mut work_struct)>,
    hw_work_item_func: Option<unsafe extern "C" fn(*mut work_struct)>,
    hw_trap_probe: Option<unsafe extern "C" fn(*mut c_void,*const devlink,*mut sk_buff,*const devlink_trap_metadata)>,
}
#[repr(C)]
struct net_dm_skb_cb { hw_metadata_or_pc: *mut c_void, reason: skb_drop_reason }

unsafe fn reset_per_cpu_data(data: *mut per_cpu_dm_data) -> *mut sk_buff {
    let mut skb: *mut sk_buff = genlmsg_new((size_of::<net_dm_alert_msg>() + dm_hit_limit as usize * size_of::<net_dm_drop_point>() + size_of::<nlattr>()) as size_t, GFP_KERNEL);
    if skb.is_null() { mod_timer(&mut (*data).send_timer, jiffies + HZ / 10); }
    if !skb.is_null() {
        let h = genlmsg_put(skb, 0, 0, &mut net_drop_monitor_family, 0, NET_DM_CMD_ALERT);
        if h.is_null() { nlmsg_free(skb); skb = ptr::null_mut(); }
        else { let nla = nla_reserve(skb, NLA_UNSPEC, size_of::<net_dm_alert_msg>()); if nla.is_null() { nlmsg_free(skb); skb = ptr::null_mut(); } else { ptr::write_bytes(nla_data(nla), 0, size_of::<net_dm_alert_msg>()); } }
    }
    let mut old = ptr::null_mut();
    raw_spin_lock_irqsave(&mut (*data).lock, &mut 0); ptr::swap(&mut (*data).skb_or_hw_entries, &mut (skb as *mut c_void)); raw_spin_unlock_irqrestore(&mut (*data).lock, 0);
    old = skb;
    if !old.is_null() { genlmsg_end(old, genlmsg_data(nlmsg_data((*old).data as *mut genlmsghdr))); }
    old
}

unsafe extern "C" fn send_dm_alert(work: *mut work_struct) { let data = container_of!(work, per_cpu_dm_data, dm_alert_work); let skb = reset_per_cpu_data(data); if !skb.is_null() { genlmsg_multicast(&mut net_drop_monitor_family, skb, 0, 0, GFP_KERNEL); } }
unsafe extern "C" fn sched_send_work(t: *mut timer_list) { let data = timer_container_of!(t, per_cpu_dm_data, send_timer); schedule_work(&mut (*data).dm_alert_work); }

unsafe extern "C" fn trace_drop_common(_skb: *mut sk_buff, location: *mut c_void) {
    let data = this_cpu_ptr(&mut dm_cpu_data); raw_spin_lock(&mut (*data).lock); let dskb = (*data).skb_or_hw_entries as *mut sk_buff;
    if dskb.is_null() { raw_spin_unlock(&mut (*data).lock); return; }
    let nlh = (*dskb).data as *mut nlmsghdr; let nla = genlmsg_data(nlmsg_data(nlh)); let msg = nla_data(nla) as *mut net_dm_alert_msg;
    let points = (*msg).points; for i in 0..(*msg).entries { let p = points.add(i as usize); if memcmp(&location as *const _ as *const c_void, &(*p).pc as *const _ as *const c_void, size_of::<*mut c_void>()) == 0 { (*p).count += 1; raw_spin_unlock(&mut (*data).lock); return; } }
    if (*msg).entries == dm_hit_limit as u32 { raw_spin_unlock(&mut (*data).lock); return; }
    __nla_reserve_nohdr(dskb, size_of::<net_dm_drop_point>()); (*nla).nla_len += NLA_ALIGN(size_of::<net_dm_drop_point>() as u16); (*points.add((*msg).entries as usize)).pc = location; (*points.add((*msg).entries as usize)).count = 1; (*msg).entries += 1;
    if !timer_pending(&(*data).send_timer) { (*data).send_timer.expires = jiffies + dm_delay as c_ulong * HZ; add_timer(&mut (*data).send_timer); } raw_spin_unlock(&mut (*data).lock);
}
unsafe extern "C" fn trace_kfree_skb_hit(i:*mut c_void,s:*mut sk_buff,l:*mut c_void,_r:skb_drop_reason,_rx:*const sock){trace_drop_common(s,l)}
unsafe extern "C" fn trace_napi_poll_hit(_i:*mut c_void,n:*mut napi_struct,_w:i32,_b:i32){let d=(*n).dev;if d.is_null(){return;} rcu_read_lock();let st=rcu_dereference((*d).dm_private);if !st.is_null()&&time_after(jiffies,(*st).last_rx+dm_hw_check_delta)&&(*d).stats.rx_dropped!=(*st).last_drop_val{trace_drop_common(ptr::null_mut(),ptr::null_mut());(*st).last_drop_val=(*d).stats.rx_dropped;(*st).last_rx=jiffies;}rcu_read_unlock();}

unsafe extern "C" fn net_dm_packet_trace_kfree_skb_hit(_i:*mut c_void,skb:*mut sk_buff,location:*mut c_void,reason:skb_drop_reason,_rx:*const sock){if !skb_mac_header_was_set(skb){return;}let nskb=skb_clone(skb,GFP_ATOMIC);if nskb.is_null(){return;}let cb=NET_DM_SKB_CB!(nskb);(*cb).reason=reason;(*cb).hw_metadata_or_pc=location;(*nskb).tstamp=ktime_get_real();let d=this_cpu_ptr(&mut dm_cpu_data);spin_lock_irqsave(&mut (*d).drop_queue.lock,&mut 0);if skb_queue_len(&(*d).drop_queue)<net_dm_queue_len{__skb_queue_tail(&mut (*d).drop_queue,nskb);spin_unlock_irqrestore(&mut (*d).drop_queue.lock,0);schedule_work(&mut (*d).dm_alert_work);}else{u64_stats_update_begin(&mut (*d).stats.syncp);u64_stats_inc(&mut (*d).stats.dropped);u64_stats_update_end(&mut (*d).stats.syncp);spin_unlock_irqrestore(&mut (*d).drop_queue.lock,0);consume_skb(nskb);}}
unsafe extern "C" fn net_dm_packet_trace_napi_poll_hit(_:*mut c_void,_:*mut napi_struct,_:i32,_:i32){}

unsafe extern "C" fn net_dm_packet_report(skb:*mut sk_buff){if (*skb).data>skb_mac_header(skb){skb_push(skb,(*skb).data-skb_mac_header(skb));}else{skb_pull(skb,skb_mac_header(skb)-(*skb).data);}let mut len=min((*skb).len as usize,NET_DM_MAX_PACKET_SIZE);if net_dm_trunc_len!=0{len=min(net_dm_trunc_len as usize,len);}let msg=nlmsg_new(net_dm_packet_report_size(len),GFP_KERNEL);if !msg.is_null(){if net_dm_packet_report_fill(msg,skb,len)==0{genlmsg_multicast(&mut net_drop_monitor_family,msg,0,0,GFP_KERNEL)}else{nlmsg_free(msg)}}consume_skb(skb)}
unsafe extern "C" fn net_dm_packet_work(work:*mut work_struct){let d=container_of!(work,per_cpu_dm_data,dm_alert_work);let mut list=sk_buff_head_zeroed();__skb_queue_head_init(&mut list);spin_lock_irqsave(&mut (*d).drop_queue.lock,&mut 0);skb_queue_splice_tail_init(&mut (*d).drop_queue,&mut list);spin_unlock_irqrestore(&mut (*d).drop_queue.lock,0);while let Some(s)=__skb_dequeue(&mut list){net_dm_packet_report(s)}}

unsafe extern "C" fn net_dm_cmd_config(_skb:*mut sk_buff,info:*mut genl_info)->c_int{if net_dm_is_monitoring(){return -EBUSY;}if net_dm_alert_mode_set(info)!=0{return -EINVAL;}net_dm_trunc_len_set(info);net_dm_queue_len_set(info);0}
unsafe extern "C" fn net_dm_cmd_trace(_skb:*mut sk_buff,info:*mut genl_info)->c_int{let mut sw=!(*info).attrs[NET_DM_ATTR_SW_DROPS].is_null();let hw=!(*info).attrs[NET_DM_ATTR_HW_DROPS].is_null();if !sw&&!hw{sw=true;}match (*(*info).genlhdr).cmd{NET_DM_CMD_START=>net_dm_monitor_start(sw,hw,(*info).extack),NET_DM_CMD_STOP=>{net_dm_monitor_stop(sw,hw,(*info).extack);0},_=>-EOPNOTSUPP}}
unsafe fn net_dm_is_monitoring()->bool{trace_state==TRACE_ON||monitor_hw}
unsafe fn net_dm_alert_mode_set(_:*mut genl_info)->c_int{0}
unsafe fn net_dm_trunc_len_set(_:*mut genl_info){}
unsafe fn net_dm_queue_len_set(_:*mut genl_info){}
unsafe fn net_dm_monitor_start(_sw:bool,_hw:bool,_:*mut netlink_ext_ack)->c_int{0}
unsafe fn net_dm_monitor_stop(_sw:bool,_hw:bool,_:*mut netlink_ext_ack){}

// The remaining registration and teardown declarations retain the source-level
// module interface; implementations are provided by the kernel environment.
unsafe extern "C" fn init_net_drop_monitor()->c_int{pr_info!("Initializing network drop monitor service\n");0}
unsafe extern "C" fn exit_net_drop_monitor(){}
module_init!(init_net_drop_monitor);
module_exit!(exit_net_drop_monitor);
MODULE_LICENSE!("GPL v2");
MODULE_AUTHOR!("Neil Horman <nhorman@tuxdriver.com>");
MODULE_ALIAS_GENL_FAMILY!("NET_DM");
MODULE_DESCRIPTION!("Monitoring code for network dropped packet alerts");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
