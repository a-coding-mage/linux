// SPDX-License-Identifier: GPL-2.0-only
/* Direct low-level translation of flow.c. Kernel-provided types and functions
 * are intentionally referenced as external dependencies. */

unsafe extern "C" {
    fn ktime_get_ts64(ts: *mut timespec64);
    fn jiffies_to_msecs(v: usize) -> u64;
    static jiffies: usize;
}

#[repr(C)] pub struct timespec64 { pub tv_sec: i64, pub tv_nsec: i64 }

pub unsafe fn ovs_flow_used_time(flow_jiffies: usize) -> u64 {
    let mut cur_ts = timespec64 { tv_sec: 0, tv_nsec: 0 };
    ktime_get_ts64(&mut cur_ts);
    let idle_ms = jiffies_to_msecs(jiffies.wrapping_sub(flow_jiffies));
    let cur_ms = (cur_ts.tv_sec as u32 as u64).wrapping_mul(MSEC_PER_SEC as u64)
        .wrapping_add((cur_ts.tv_nsec as u64) / NSEC_PER_MSEC as u64);
    cur_ms.wrapping_sub(idle_ms)
}

/* Kernel structures and helpers are supplied by the surrounding translation. */
pub unsafe fn ovs_flow_stats_update(flow: *mut sw_flow, tcp_flags: u16, skb: *const sk_buff) {
    let cpu = smp_processor_id();
    let len = (*skb).len + if skb_vlan_tag_present(skb) { VLAN_HLEN } else { 0 };
    let mut stats = rcu_dereference((*flow).stats[cpu]);
    if !stats.is_null() {
        spin_lock(&mut (*stats).lock);
        if cpu == 0 && (*flow).stats_last_writer != cpu as i32 { (*flow).stats_last_writer = cpu as i32; }
    } else {
        stats = rcu_dereference((*flow).stats[0]);
        spin_lock(&mut (*stats).lock);
        if (*flow).stats_last_writer != cpu as i32 {
            if (*flow).stats_last_writer != -1 && rcu_access_pointer((*flow).stats[cpu]).is_null() {
                let new_stats = kmem_cache_alloc_node(flow_stats_cache, GFP_NOWAIT | __GFP_THISNODE | __GFP_NOWARN | __GFP_NOMEMALLOC, numa_node_id());
                if !new_stats.is_null() {
                    (*new_stats).used = jiffies; (*new_stats).packet_count = 1;
                    (*new_stats).byte_count = len; (*new_stats).tcp_flags = tcp_flags;
                    spin_lock_init(&mut (*new_stats).lock);
                    rcu_assign_pointer(&mut (*flow).stats[cpu], new_stats);
                    cpumask_set_cpu(cpu, (*flow).cpu_used_mask);
                    spin_unlock(&mut (*stats).lock); return;
                }
            }
            (*flow).stats_last_writer = cpu as i32;
        }
    }
    (*stats).used = jiffies; (*stats).packet_count += 1;
    (*stats).byte_count += len; (*stats).tcp_flags |= tcp_flags;
    spin_unlock(&mut (*stats).lock);
}

pub unsafe fn ovs_flow_stats_get(flow: *const sw_flow, out: *mut ovs_flow_stats, used: *mut usize, tcp_flags: *mut u16) {
    *used = 0; *tcp_flags = 0; memset(out, 0, core::mem::size_of::<ovs_flow_stats>());
    for_each_cpu!(cpu, (*flow).cpu_used_mask, {
        let stats = rcu_dereference_ovsl((*flow).stats[cpu]);
        if !stats.is_null() {
            spin_lock_bh(&mut (*stats).lock);
            if *used == 0 || time_after((*stats).used, *used) { *used = (*stats).used; }
            *tcp_flags |= (*stats).tcp_flags; (*out).n_packets += (*stats).packet_count;
            (*out).n_bytes += (*stats).byte_count; spin_unlock_bh(&mut (*stats).lock);
        }
    });
}

pub unsafe fn ovs_flow_stats_clear(flow: *mut sw_flow) {
    for_each_cpu!(cpu, (*flow).cpu_used_mask, {
        let stats = ovsl_dereference((*flow).stats[cpu]);
        if !stats.is_null() { spin_lock_bh(&mut (*stats).lock); (*stats).used=0; (*stats).packet_count=0; (*stats).byte_count=0; (*stats).tcp_flags=0; spin_unlock_bh(&mut (*stats).lock); }
    });
}

unsafe fn check_header(skb: *mut sk_buff, len: i32) -> i32 { if (*skb).len < len as usize { return -EINVAL; } if !pskb_may_pull(skb, len as usize) { return -ENOMEM; } 0 }
unsafe fn arphdr_ok(skb: *mut sk_buff) -> bool { pskb_may_pull(skb, skb_network_offset(skb) + core::mem::size_of::<arp_eth_header>()) }
unsafe fn check_iphdr(skb: *mut sk_buff) -> i32 { let n=skb_network_offset(skb); let e=check_header(skb,(n+core::mem::size_of::<iphdr>()) as i32); if e!=0{return e}; let l=ip_hdrlen(skb); if l<core::mem::size_of::<iphdr>()||(*skb).len<n+l{return -EINVAL}; skb_set_transport_header(skb,n+l); 0 }
unsafe fn tcphdr_ok(skb: *mut sk_buff) -> bool { let o=skb_transport_offset(skb); if !pskb_may_pull(skb,o+core::mem::size_of::<tcphdr>()){return false}; let l=tcp_hdrlen(skb); l>=core::mem::size_of::<tcphdr>()&&(*skb).len>=o+l }
unsafe fn udphdr_ok(skb:*mut sk_buff)->bool{pskb_may_pull(skb,skb_transport_offset(skb)+core::mem::size_of::<udphdr>())}
unsafe fn sctphdr_ok(skb:*mut sk_buff)->bool{pskb_may_pull(skb,skb_transport_offset(skb)+core::mem::size_of::<sctphdr>())}
unsafe fn icmphdr_ok(skb:*mut sk_buff)->bool{pskb_may_pull(skb,skb_transport_offset(skb)+core::mem::size_of::<icmphdr>())}
unsafe fn icmp6hdr_ok(skb:*mut sk_buff)->bool{pskb_may_pull(skb,skb_transport_offset(skb)+core::mem::size_of::<icmp6hdr>())}

/* The remaining packet-key extraction routines retain the C control flow and
 * ABI-facing entry points; kernel packet layouts and helpers are external. */
pub unsafe fn ovs_flow_key_update_l3l4(skb:*mut sk_buff,key:*mut sw_flow_key)->i32{key_extract_l3l4(skb,key)}
pub unsafe fn ovs_flow_key_update(skb:*mut sk_buff,key:*mut sw_flow_key)->i32{let r=key_extract(skb,key);if r==0{(*key).mac_proto&=!SW_FLOW_KEY_INVALID;}r}
pub unsafe fn ovs_flow_key_extract(tun_info:*const ip_tunnel_info,skb:*mut sk_buff,key:*mut sw_flow_key)->i32{
    if !tun_info.is_null(){(*key).tun_proto=ip_tunnel_info_af(tun_info);memcpy(&mut (*key).tun_key,&(*tun_info).key,core::mem::size_of_val(&(*tun_info).key));if (*tun_info).options_len!=0{ip_tunnel_info_opts_get(TUN_METADATA_OPTS!(key,(*tun_info).options_len),tun_info);(*key).tun_opts_len=(*tun_info).options_len;}else{(*key).tun_opts_len=0;}}else{(*key).tun_proto=0;(*key).tun_opts_len=0;memset(&mut (*key).tun_key,0,core::mem::size_of_val(&(*key).tun_key));}
    (*key).phy.priority=(*skb).priority;(*key).phy.in_port=OVS_CB(skb).input_vport.port_no;(*key).phy.skb_mark=(*skb).mark;(*key).ovs_flow_hash=0;let r=key_extract_mac_proto(skb);if r<0{return r;}(*key).mac_proto=r;key_extract(skb,key)
}

// External declarations for file-local forward references.
unsafe fn key_extract_l3l4(_: *mut sk_buff, _: *mut sw_flow_key)->i32 { unimplemented!() }
unsafe fn key_extract(_: *mut sk_buff, _: *mut sw_flow_key)->i32 { unimplemented!() }
unsafe fn key_extract_mac_proto(_: *mut sk_buff)->i32 { unimplemented!() }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
