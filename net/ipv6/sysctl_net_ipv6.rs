// SPDX-License-Identifier: GPL-2.0
/*
 * sysctl_net_ipv6.c: sysctl interface to net IPV6 subsystem.
 *
 * Changes:
 * YOSHIFUJI Hideaki @USAGI: added icmp sysctl table.
 */

// Kernel dependencies supplied by the surrounding translation unit.

static mut flowlabel_reflect_max: c_int = 0x7;
static mut auto_flowlabels_max: c_int = IP6_AUTO_FLOW_LABEL_MAX;
static mut rt6_multipath_hash_fields_all_mask: u32 = FIB_MULTIPATH_HASH_FIELD_ALL_MASK;
static mut ioam6_id_max: u32 = IOAM6_DEFAULT_ID;
static mut ioam6_id_wide_max: u64 = IOAM6_DEFAULT_ID_WIDE;

unsafe fn proc_rt6_multipath_hash_policy(
    table: *const ctl_table,
    write: c_int,
    buffer: *mut c_void,
    lenp: *mut size_t,
    ppos: *mut loff_t,
) -> c_int {
    let net: *mut net;
    let ret: c_int;

    net = container_of((*table).data, net, ipv6.sysctl.multipath_hash_policy);
    ret = proc_dou8vec_minmax(table, write, buffer, lenp, ppos);
    if write != 0 && ret == 0 {
        call_netevent_notifiers(NETEVENT_IPV6_MPATH_HASH_UPDATE, net);
    }
    ret
}

unsafe fn proc_rt6_multipath_hash_fields(
    table: *const ctl_table,
    write: c_int,
    buffer: *mut c_void,
    lenp: *mut size_t,
    ppos: *mut loff_t,
) -> c_int {
    let net: *mut net;
    let ret: c_int;

    net = container_of((*table).data, net, ipv6.sysctl.multipath_hash_fields);
    ret = proc_douintvec_minmax(table, write, buffer, lenp, ppos);
    if write != 0 && ret == 0 {
        call_netevent_notifiers(NETEVENT_IPV6_MPATH_HASH_UPDATE, net);
    }
    ret
}

static mut ipv6_table_template: [ctl_table; 20] = [
    ctl_table { procname: "bindv6only", data: &raw mut init_net.ipv6.sysctl.bindv6only as *mut c_void, maxlen: core::mem::size_of::<u8>(), mode: 0o644, proc_handler: Some(proc_dou8vec_minmax), extra1: core::ptr::null_mut(), extra2: core::ptr::null_mut() },
    ctl_table { procname: "anycast_src_echo_reply", data: &raw mut init_net.ipv6.sysctl.anycast_src_echo_reply as *mut c_void, maxlen: core::mem::size_of::<u8>(), mode: 0o644, proc_handler: Some(proc_dou8vec_minmax), extra1: core::ptr::null_mut(), extra2: core::ptr::null_mut() },
    ctl_table { procname: "flowlabel_consistency", data: &raw mut init_net.ipv6.sysctl.flowlabel_consistency as *mut c_void, maxlen: core::mem::size_of::<u8>(), mode: 0o644, proc_handler: Some(proc_dou8vec_minmax), extra1: core::ptr::null_mut(), extra2: core::ptr::null_mut() },
    ctl_table { procname: "auto_flowlabels", data: &raw mut init_net.ipv6.sysctl.auto_flowlabels as *mut c_void, maxlen: core::mem::size_of::<u8>(), mode: 0o644, proc_handler: Some(proc_dou8vec_minmax), extra1: core::ptr::null_mut(), extra2: &raw mut auto_flowlabels_max as *mut c_void },
    ctl_table { procname: "fwmark_reflect", data: &raw mut init_net.ipv6.sysctl.fwmark_reflect as *mut c_void, maxlen: core::mem::size_of::<u8>(), mode: 0o644, proc_handler: Some(proc_dou8vec_minmax), extra1: core::ptr::null_mut(), extra2: core::ptr::null_mut() },
    ctl_table { procname: "idgen_retries", data: &raw mut init_net.ipv6.sysctl.idgen_retries as *mut c_void, maxlen: core::mem::size_of::<c_int>(), mode: 0o644, proc_handler: Some(proc_dointvec), extra1: core::ptr::null_mut(), extra2: core::ptr::null_mut() },
    ctl_table { procname: "idgen_delay", data: &raw mut init_net.ipv6.sysctl.idgen_delay as *mut c_void, maxlen: core::mem::size_of::<c_int>(), mode: 0o644, proc_handler: Some(proc_dointvec_jiffies), extra1: core::ptr::null_mut(), extra2: core::ptr::null_mut() },
    ctl_table { procname: "flowlabel_state_ranges", data: &raw mut init_net.ipv6.sysctl.flowlabel_state_ranges as *mut c_void, maxlen: core::mem::size_of::<u8>(), mode: 0o644, proc_handler: Some(proc_dou8vec_minmax), extra1: core::ptr::null_mut(), extra2: core::ptr::null_mut() },
    ctl_table { procname: "ip_nonlocal_bind", data: &raw mut init_net.ipv6.sysctl.ip_nonlocal_bind as *mut c_void, maxlen: core::mem::size_of::<u8>(), mode: 0o644, proc_handler: Some(proc_dou8vec_minmax), extra1: core::ptr::null_mut(), extra2: core::ptr::null_mut() },
    ctl_table { procname: "flowlabel_reflect", data: &raw mut init_net.ipv6.sysctl.flowlabel_reflect as *mut c_void, maxlen: core::mem::size_of::<c_int>(), mode: 0o644, proc_handler: Some(proc_dointvec_minmax), extra1: SYSCTL_ZERO, extra2: &raw mut flowlabel_reflect_max as *mut c_void },
    ctl_table { procname: "max_dst_opts_number", data: &raw mut init_net.ipv6.sysctl.max_dst_opts_cnt as *mut c_void, maxlen: core::mem::size_of::<c_int>(), mode: 0o644, proc_handler: Some(proc_dointvec), extra1: core::ptr::null_mut(), extra2: core::ptr::null_mut() },
    ctl_table { procname: "max_hbh_opts_number", data: &raw mut init_net.ipv6.sysctl.max_hbh_opts_cnt as *mut c_void, maxlen: core::mem::size_of::<c_int>(), mode: 0o644, proc_handler: Some(proc_dointvec), extra1: core::ptr::null_mut(), extra2: core::ptr::null_mut() },
    ctl_table { procname: "max_dst_opts_length", data: &raw mut init_net.ipv6.sysctl.max_dst_opts_len as *mut c_void, maxlen: core::mem::size_of::<c_int>(), mode: 0o644, proc_handler: Some(proc_dointvec), extra1: core::ptr::null_mut(), extra2: core::ptr::null_mut() },
    ctl_table { procname: "max_hbh_length", data: &raw mut init_net.ipv6.sysctl.max_hbh_opts_len as *mut c_void, maxlen: core::mem::size_of::<c_int>(), mode: 0o644, proc_handler: Some(proc_dointvec), extra1: core::ptr::null_mut(), extra2: core::ptr::null_mut() },
    ctl_table { procname: "fib_multipath_hash_policy", data: &raw mut init_net.ipv6.sysctl.multipath_hash_policy as *mut c_void, maxlen: core::mem::size_of::<u8>(), mode: 0o644, proc_handler: Some(proc_rt6_multipath_hash_policy), extra1: SYSCTL_ZERO, extra2: SYSCTL_THREE },
    ctl_table { procname: "fib_multipath_hash_fields", data: &raw mut init_net.ipv6.sysctl.multipath_hash_fields as *mut c_void, maxlen: core::mem::size_of::<u32>(), mode: 0o644, proc_handler: Some(proc_rt6_multipath_hash_fields), extra1: SYSCTL_ONE, extra2: &raw mut rt6_multipath_hash_fields_all_mask as *mut c_void },
    ctl_table { procname: "seg6_flowlabel", data: &raw mut init_net.ipv6.sysctl.seg6_flowlabel as *mut c_void, maxlen: core::mem::size_of::<c_int>(), mode: 0o644, proc_handler: Some(proc_dointvec), extra1: core::ptr::null_mut(), extra2: core::ptr::null_mut() },
    ctl_table { procname: "fib_notify_on_flag_change", data: &raw mut init_net.ipv6.sysctl.fib_notify_on_flag_change as *mut c_void, maxlen: core::mem::size_of::<u8>(), mode: 0o644, proc_handler: Some(proc_dou8vec_minmax), extra1: SYSCTL_ZERO, extra2: SYSCTL_TWO },
    ctl_table { procname: "ioam6_id", data: &raw mut init_net.ipv6.sysctl.ioam6_id as *mut c_void, maxlen: core::mem::size_of::<u32>(), mode: 0o644, proc_handler: Some(proc_douintvec_minmax), extra1: core::ptr::null_mut(), extra2: &raw mut ioam6_id_max as *mut c_void },
    ctl_table { procname: "ioam6_id_wide", data: &raw mut init_net.ipv6.sysctl.ioam6_id_wide as *mut c_void, maxlen: core::mem::size_of::<u64>(), mode: 0o644, proc_handler: Some(proc_doulongvec_minmax), extra1: core::ptr::null_mut(), extra2: &raw mut ioam6_id_wide_max as *mut c_void },
];

static mut ipv6_rotable: [ctl_table; 2] = [
    ctl_table { procname: "mld_max_msf", data: &raw mut sysctl_mld_max_msf as *mut c_void, maxlen: core::mem::size_of::<c_int>(), mode: 0o644, proc_handler: Some(proc_dointvec), extra1: core::ptr::null_mut(), extra2: core::ptr::null_mut() },
    ctl_table { procname: "mld_qrv", data: &raw mut sysctl_mld_qrv as *mut c_void, maxlen: core::mem::size_of::<c_int>(), mode: 0o644, proc_handler: Some(proc_dointvec_minmax), extra1: SYSCTL_ONE, extra2: core::ptr::null_mut() },
];

unsafe fn ipv6_sysctl_net_init(net: *mut net) -> c_int {
    let table_size = ipv6_table_template.len();
    let mut ipv6_table: *mut ctl_table;
    let mut ipv6_route_table: *mut ctl_table;
    let mut ipv6_icmp_table: *mut ctl_table;
    let mut err: c_int = -ENOMEM;

    ipv6_table = kmemdup(ipv6_table_template.as_ptr() as *const c_void,
                          core::mem::size_of_val(&ipv6_table_template), GFP_KERNEL);
    if ipv6_table.is_null() { return err; }
    for i in 0..table_size {
        (*ipv6_table.add(i)).data = ((*ipv6_table.add(i)).data as usize)
            .wrapping_add((net as usize).wrapping_sub((&raw mut init_net) as *mut _ as usize)) as *mut c_void;
    }
    ipv6_route_table = ipv6_route_sysctl_init(net);
    if ipv6_route_table.is_null() { kfree(ipv6_table as *mut c_void); return err; }
    ipv6_icmp_table = ipv6_icmp_sysctl_init(net);
    if ipv6_icmp_table.is_null() { kfree(ipv6_route_table as *mut c_void); kfree(ipv6_table as *mut c_void); return err; }
    (*net).ipv6.sysctl.hdr = register_net_sysctl_sz(net, "net/ipv6", ipv6_table, table_size);
    if (*net).ipv6.sysctl.hdr.is_null() { kfree(ipv6_icmp_table as *mut c_void); kfree(ipv6_route_table as *mut c_void); kfree(ipv6_table as *mut c_void); return err; }
    (*net).ipv6.sysctl.route_hdr = register_net_sysctl_sz(net, "net/ipv6/route", ipv6_route_table, ipv6_route_sysctl_table_size(net));
    if (*net).ipv6.sysctl.route_hdr.is_null() { unregister_net_sysctl_table((*net).ipv6.sysctl.hdr); kfree(ipv6_icmp_table as *mut c_void); kfree(ipv6_route_table as *mut c_void); kfree(ipv6_table as *mut c_void); return err; }
    (*net).ipv6.sysctl.icmp_hdr = register_net_sysctl_sz(net, "net/ipv6/icmp", ipv6_icmp_table, ipv6_icmp_sysctl_table_size());
    if (*net).ipv6.sysctl.icmp_hdr.is_null() { unregister_net_sysctl_table((*net).ipv6.sysctl.route_hdr); unregister_net_sysctl_table((*net).ipv6.sysctl.hdr); kfree(ipv6_icmp_table as *mut c_void); kfree(ipv6_route_table as *mut c_void); kfree(ipv6_table as *mut c_void); return err; }
    err = 0;
    err
}

unsafe fn ipv6_sysctl_net_exit(net: *mut net) {
    let ipv6_table = (*net).ipv6.sysctl.hdr.ctl_table_arg;
    let ipv6_route_table = (*net).ipv6.sysctl.route_hdr.ctl_table_arg;
    let ipv6_icmp_table = (*net).ipv6.sysctl.icmp_hdr.ctl_table_arg;
    unregister_net_sysctl_table((*net).ipv6.sysctl.icmp_hdr);
    unregister_net_sysctl_table((*net).ipv6.sysctl.route_hdr);
    unregister_net_sysctl_table((*net).ipv6.sysctl.hdr);
    kfree(ipv6_table as *mut c_void); kfree(ipv6_route_table as *mut c_void); kfree(ipv6_icmp_table as *mut c_void);
}

static mut ipv6_sysctl_net_ops: pernet_operations = pernet_operations { init: Some(ipv6_sysctl_net_init), exit: Some(ipv6_sysctl_net_exit) };
static mut ip6_header: *mut ctl_table_header = core::ptr::null_mut();

pub unsafe fn ipv6_sysctl_register() -> c_int {
    let mut err = -ENOMEM;
    ip6_header = register_net_sysctl(&raw mut init_net, "net/ipv6", ipv6_rotable.as_mut_ptr());
    if ip6_header.is_null() { return err; }
    err = register_pernet_subsys(&raw mut ipv6_sysctl_net_ops);
    if err != 0 { unregister_net_sysctl_table(ip6_header); }
    err
}

pub unsafe fn ipv6_sysctl_unregister() {
    unregister_net_sysctl_table(ip6_header);
    unregister_pernet_subsys(&raw mut ipv6_sysctl_net_ops);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
