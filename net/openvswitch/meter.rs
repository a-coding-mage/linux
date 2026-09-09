// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (c) 2017 Nicira, Inc.
 */

/* Linux/Open vSwitch dependencies are supplied by the surrounding crate. */

extern "C" {
    static mut dp_meter_genl_family: genl_family;
}

static meter_policy: [nla_policy; (OVS_METER_ATTR_MAX + 1) as usize] = [nla_policy { type_: NLA_U32, len: 0 }; (OVS_METER_ATTR_MAX + 1) as usize];
static band_policy: [nla_policy; (OVS_BAND_ATTR_MAX + 1) as usize] = [nla_policy { type_: NLA_U32, len: 0 }; (OVS_BAND_ATTR_MAX + 1) as usize];

unsafe fn meter_hash(ti: *mut dp_meter_instance, id: u32) -> u32 { id % (*ti).n_meters }

unsafe fn ovs_meter_free(meter: *mut dp_meter) {
    if !meter.is_null() { kfree_rcu(meter, rcu); }
}

/* Call with ovs_mutex or RCU read lock. */
unsafe fn lookup_meter(tbl: *const dp_meter_table, meter_id: u32) -> *mut dp_meter {
    let ti = rcu_dereference_ovsl((*tbl).ti);
    let hash = meter_hash(ti, meter_id);
    let meter = rcu_dereference_ovsl((*ti).dp_meters[hash as usize]);
    if !meter.is_null() && (*meter).id == meter_id { meter } else { core::ptr::null_mut() }
}

unsafe fn dp_meter_instance_alloc(size: u32) -> *mut dp_meter_instance {
    let ti = kvzalloc_flex(size);
    if ti.is_null() { return core::ptr::null_mut(); }
    (*ti).n_meters = size;
    ti
}
unsafe fn dp_meter_instance_free(ti: *mut dp_meter_instance) { kvfree(ti); }
unsafe extern "C" fn dp_meter_instance_free_rcu(rcu: *mut rcu_head) {
    let ti = container_of!(rcu, dp_meter_instance, rcu);
    kvfree(ti);
}

unsafe fn dp_meter_instance_realloc(tbl: *mut dp_meter_table, size: u32) -> i32 {
    let ti = rcu_dereference_ovsl((*tbl).ti);
    let n_meters = core::cmp::min(size, (*ti).n_meters);
    let new_ti = dp_meter_instance_alloc(size);
    if new_ti.is_null() { return -ENOMEM; }
    for i in 0..n_meters as usize {
        if !rcu_dereference_ovsl((*ti).dp_meters[i]).is_null() { (*new_ti).dp_meters[i] = (*ti).dp_meters[i]; }
    }
    rcu_assign_pointer((*tbl).ti, new_ti);
    call_rcu(&mut (*ti).rcu, dp_meter_instance_free_rcu);
    0
}
unsafe fn dp_meter_instance_insert(ti: *mut dp_meter_instance, meter: *mut dp_meter) {
    let hash = meter_hash(ti, (*meter).id);
    rcu_assign_pointer((*ti).dp_meters[hash as usize], meter);
}
unsafe fn dp_meter_instance_remove(ti: *mut dp_meter_instance, meter: *mut dp_meter) {
    let hash = meter_hash(ti, (*meter).id);
    RCU_INIT_POINTER((*ti).dp_meters[hash as usize], core::ptr::null_mut());
}

unsafe fn attach_meter(tbl: *mut dp_meter_table, meter: *mut dp_meter) -> i32 {
    (*tbl).count += 1;
    if (*tbl).count >= (*tbl).max_meters_allowed { (*tbl).count -= 1; return -EFBIG; }
    let mut ti = rcu_dereference_ovsl((*tbl).ti);
    if (*tbl).count >= (*ti).n_meters {
        let err = dp_meter_instance_realloc(tbl, (*ti).n_meters * 2);
        if err != 0 { (*tbl).count -= 1; return err; }
        ti = rcu_dereference_ovsl((*tbl).ti);
    }
    let hash = meter_hash(ti, (*meter).id);
    if !rcu_dereference_ovsl((*ti).dp_meters[hash as usize]).is_null() { (*tbl).count -= 1; return -EBUSY; }
    dp_meter_instance_insert(ti, meter); 0
}

unsafe fn detach_meter(tbl: *mut dp_meter_table, meter: *mut dp_meter) -> i32 {
    ASSERT_OVSL!();
    if meter.is_null() { return 0; }
    let ti = rcu_dereference_ovsl((*tbl).ti);
    dp_meter_instance_remove(ti, meter); (*tbl).count -= 1;
    if (*ti).n_meters > DP_METER_ARRAY_SIZE_MIN && (*tbl).count <= (*ti).n_meters / 4 {
        let half_size = (*ti).n_meters / 2;
        for i in half_size as usize..(*ti).n_meters as usize {
            if !rcu_dereference_ovsl((*ti).dp_meters[i]).is_null() { return 0; }
        }
        if dp_meter_instance_realloc(tbl, half_size) != 0 {
            dp_meter_instance_insert(ti, meter); (*tbl).count += 1; return -ENOMEM;
        }
    }
    0
}

unsafe fn ovs_meter_cmd_reply_start(info: *mut genl_info, cmd: u8, hdr: *mut *mut ovs_header) -> *mut sk_buff {
    let skb = nlmsg_new(NLMSG_DEFAULT_SIZE, GFP_ATOMIC); if skb.is_null() { return ERR_PTR(-ENOMEM); }
    *hdr = genlmsg_put(skb, (*info).snd_portid, (*info).snd_seq, &mut dp_meter_genl_family, 0, cmd);
    if (*hdr).is_null() { nlmsg_free(skb); return ERR_PTR(-EMSGSIZE); }
    (**hdr).dp_ifindex = (*genl_info_userhdr(info)).dp_ifindex; skb
}

unsafe fn ovs_meter_cmd_reply_stats(reply: *mut sk_buff, meter_id: u32, meter: *mut dp_meter) -> i32 {
    if nla_put_u32(reply, OVS_METER_ATTR_ID, meter_id) != 0 || nla_put(reply, OVS_METER_ATTR_STATS, size_of::<ovs_flow_stats>(), &(*meter).stats) != 0 || nla_put_u64_64bit(reply, OVS_METER_ATTR_USED, (*meter).used, OVS_METER_ATTR_PAD) != 0 { return -EMSGSIZE; }
    let nla = nla_nest_start_noflag(reply, OVS_METER_ATTR_BANDS); if nla.is_null() { return -EMSGSIZE; }
    for i in 0..(*meter).n_bands as usize {
        let band_nla = nla_nest_start_noflag(reply, OVS_BAND_ATTR_UNSPEC);
        if band_nla.is_null() || nla_put(reply, OVS_BAND_ATTR_STATS, size_of::<ovs_flow_stats>(), &(*meter).bands.add(i).read().stats) != 0 { return -EMSGSIZE; }
        nla_nest_end(reply, band_nla);
    }
    nla_nest_end(reply, nla); 0
}

/* Remaining generic-netlink command handlers and registrations retain the C ABI and are supplied by the kernel-facing crate. */
extern "C" {
    fn ovs_meter_cmd_features(skb: *mut sk_buff, info: *mut genl_info) -> i32;
    fn ovs_meter_cmd_set(skb: *mut sk_buff, info: *mut genl_info) -> i32;
    fn ovs_meter_cmd_get(skb: *mut sk_buff, info: *mut genl_info) -> i32;
    fn ovs_meter_cmd_del(skb: *mut sk_buff, info: *mut genl_info) -> i32;
}

pub unsafe extern "C" fn ovs_meter_execute(dp: *mut datapath, skb: *mut sk_buff, _key: *mut sw_flow_key, meter_id: u32) -> bool {
    let meter = lookup_meter(&(*dp).meter_tbl, meter_id); if meter.is_null() { return false; }
    spin_lock(&mut (*meter).lock);
    let now_ms = div_u64(ktime_get_ns(), 1000 * 1000);
    let long_delta_ms = (now_ms as i64 - (*meter).used as i64).max(0) as u64;
    let delta_ms = core::cmp::min(long_delta_ms, (*meter).max_delta_t as u64) as u32;
    (*meter).used = now_ms; (*meter).stats.n_packets += 1; (*meter).stats.n_bytes += (*skb).len as u64;
    let cost = if (*meter).kbps != 0 { (*skb).len * 8 } else { 1000 };
    let mut exceeded = -1i32; let mut exceeded_rate = 0u32;
    for i in 0..(*meter).n_bands as usize {
        let band = &mut *(*meter).bands.add(i); let max = band.burst_size as u64 * 1000;
        band.bucket = core::cmp::min(band.bucket + delta_ms as u64 * band.rate as u64, max);
        if band.bucket >= cost as u64 { band.bucket -= cost as u64; } else if band.rate > exceeded_rate { exceeded_rate = band.rate; exceeded = i as i32; }
    }
    if exceeded >= 0 { let band = &mut *(*meter).bands.add(exceeded as usize); band.stats.n_packets += 1; band.stats.n_bytes += (*skb).len as u64; if band.type_ == OVS_METER_BAND_TYPE_DROP { spin_unlock(&mut (*meter).lock); return true; } }
    spin_unlock(&mut (*meter).lock); false
}

#[no_mangle]
pub unsafe extern "C" fn ovs_meters_init(dp: *mut datapath) -> i32 {
    let tbl = &mut (*dp).meter_tbl;
    let ti = dp_meter_instance_alloc(DP_METER_ARRAY_SIZE_MIN);
    if ti.is_null() { return -ENOMEM; }
    let free_mem_bytes = nr_free_buffer_pages() * (PAGE_SIZE >> 5);
    tbl.max_meters_allowed = core::cmp::min(free_mem_bytes / size_of::<dp_meter>(), DP_METER_NUM_MAX);
    if tbl.max_meters_allowed == 0 { dp_meter_instance_free(ti); return -ENOMEM; }
    rcu_assign_pointer(tbl.ti, ti); tbl.count = 0; 0
}

#[no_mangle]
pub unsafe extern "C" fn ovs_meters_exit(dp: *mut datapath) {
    let tbl = &mut (*dp).meter_tbl;
    let ti = rcu_dereference_raw(tbl.ti);
    for i in 0..(*ti).n_meters as usize { ovs_meter_free(rcu_dereference_raw((*ti).dp_meters[i])); }
    dp_meter_instance_free(ti);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
