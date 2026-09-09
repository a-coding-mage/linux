// SPDX-License-Identifier: GPL-2.0-only
// Copyright (c) 2020, Nikolay Aleksandrov <nikolay@cumulusnetworks.com>

unsafe fn __vlan_tun_put(skb: *mut sk_buff, v: *const net_bridge_vlan) -> bool {
    let tid: __be32 = tunnel_id_to_key32((*v).tinfo.tunnel_id);
    let mut nest: *mut nlattr;

    if (*v).tinfo.tunnel_dst.is_null() {
        return true;
    }

    nest = nla_nest_start(skb, BRIDGE_VLANDB_ENTRY_TUNNEL_INFO);
    if nest.is_null() {
        return false;
    }
    if nla_put_u32(skb, BRIDGE_VLANDB_TINFO_ID, be32_to_cpu(tid)) != 0 {
        nla_nest_cancel(skb, nest);
        return false;
    }
    nla_nest_end(skb, nest);

    true
}

unsafe fn __vlan_tun_can_enter_range(
    v_curr: *const net_bridge_vlan,
    range_end: *const net_bridge_vlan,
) -> bool {
    ((*v_curr).tinfo.tunnel_dst.is_null() && (*range_end).tinfo.tunnel_dst.is_null())
        || vlan_tunid_inrange(v_curr, range_end)
}

pub unsafe fn br_vlan_opts_eq_range(
    v_curr: *const net_bridge_vlan,
    range_end: *const net_bridge_vlan,
) -> bool {
    let range_mc_rtr: u8 = br_vlan_multicast_router(range_end);
    let curr_mc_rtr: u8 = br_vlan_multicast_router(v_curr);

    if (*v_curr).state != (*range_end).state {
        return false;
    }
    if !__vlan_tun_can_enter_range(v_curr, range_end) {
        return false;
    }
    if curr_mc_rtr != range_mc_rtr {
        return false;
    }
    // Check user-visible priv_flags that affect output
    if (((*v_curr).priv_flags ^ (*range_end).priv_flags)
        & (BR_VLFLAG_NEIGH_SUPPRESS_ENABLED
            | BR_VLFLAG_MCAST_ENABLED
            | BR_VLFLAG_NEIGH_FORWARD_GRAT_ENABLED)) != 0
    {
        return false;
    }

    #[cfg(CONFIG_BRIDGE_IGMP_SNOOPING)]
    {
        if !br_vlan_is_master(v_curr)
            && !br_multicast_port_ctx_vlan_disabled(&(*v_curr).port_mcast_ctx)
            && !br_multicast_port_ctx_options_equal(
                &(*v_curr).port_mcast_ctx,
                &(*range_end).port_mcast_ctx,
            )
        {
            return false;
        }
    }

    true
}

pub unsafe fn br_vlan_opts_fill(
    skb: *mut sk_buff,
    v: *const net_bridge_vlan,
    p: *const net_bridge_port,
) -> bool {
    if nla_put_u8(skb, BRIDGE_VLANDB_ENTRY_STATE, br_vlan_get_state(v)) != 0
        || !__vlan_tun_put(skb, v)
        || nla_put_u8(
            skb,
            BRIDGE_VLANDB_ENTRY_NEIGH_SUPPRESS,
            ((*v).priv_flags & BR_VLFLAG_NEIGH_SUPPRESS_ENABLED != 0) as u8,
        ) != 0
        || nla_put_u8(
            skb,
            BRIDGE_VLANDB_ENTRY_NEIGH_FORWARD_GRAT,
            ((*v).priv_flags & BR_VLFLAG_NEIGH_FORWARD_GRAT_ENABLED != 0) as u8,
        ) != 0
    {
        return false;
    }

    #[cfg(CONFIG_BRIDGE_IGMP_SNOOPING)]
    {
        if nla_put_u8(skb, BRIDGE_VLANDB_ENTRY_MCAST_ROUTER, br_vlan_multicast_router(v)) != 0 {
            return false;
        }
        if !p.is_null()
            && !br_multicast_port_ctx_vlan_disabled(&(*v).port_mcast_ctx)
            && (nla_put_u32(
                skb,
                BRIDGE_VLANDB_ENTRY_MCAST_N_GROUPS,
                br_multicast_ngroups_get(&(*v).port_mcast_ctx),
            ) != 0
                || nla_put_u32(
                    skb,
                    BRIDGE_VLANDB_ENTRY_MCAST_MAX_GROUPS,
                    br_multicast_ngroups_get_max(&(*v).port_mcast_ctx),
                ) != 0)
        {
            return false;
        }
    }

    true
}

pub unsafe fn br_vlan_opts_nl_size() -> usize {
    nla_total_size(size_of::<u8>())
        + nla_total_size(0)
        + nla_total_size(size_of::<u32>())
        #[cfg(CONFIG_BRIDGE_IGMP_SNOOPING)]
        + nla_total_size(size_of::<u8>())
        #[cfg(CONFIG_BRIDGE_IGMP_SNOOPING)]
        + nla_total_size(size_of::<u32>())
        #[cfg(CONFIG_BRIDGE_IGMP_SNOOPING)]
        + nla_total_size(size_of::<u32>())
        + nla_total_size(size_of::<u8>())
        + nla_total_size(size_of::<u8>())
}

unsafe fn br_vlan_modify_state(
    vg: *mut net_bridge_vlan_group,
    v: *mut net_bridge_vlan,
    state: u8,
    changed: *mut bool,
    extack: *mut netlink_ext_ack,
) -> i32 {
    let br: *mut net_bridge;

    ASSERT_RTNL();
    if state > BR_STATE_BLOCKING {
        NL_SET_ERR_MSG_MOD(extack, "Invalid vlan state");
        return -EINVAL;
    }
    if br_vlan_is_brentry(v) {
        br = (*v).br;
    } else {
        br = (*v).port.br;
    }
    if (*br).stp_enabled == BR_KERNEL_STP {
        NL_SET_ERR_MSG_MOD(extack, "Can't modify vlan state when using kernel STP");
        return -EBUSY;
    }
    if br_opt_get(br, BROPT_MST_ENABLED) {
        NL_SET_ERR_MSG_MOD(extack, "Can't modify vlan state directly when MST is enabled");
        return -EBUSY;
    }
    if (*v).state == state {
        return 0;
    }
    if (*v).vid == br_get_pvid(vg) {
        br_vlan_set_pvid_state(vg, state);
    }
    br_vlan_set_state(v, state);
    *changed = true;
    0
}

// The C initializer is retained as a declaration; its concrete nla_policy
// representation is supplied by the surrounding kernel translation.
extern "C" {
    static mut br_vlandb_tinfo_pol: [nla_policy; BRIDGE_VLANDB_TINFO_MAX + 1];
}

unsafe fn br_vlan_modify_tunnel(
    p: *const net_bridge_port,
    v: *mut net_bridge_vlan,
    tb: *mut *mut nlattr,
    changed: *mut bool,
    extack: *mut netlink_ext_ack,
) -> i32 {
    let mut tun_tb: [*mut nlattr; BRIDGE_VLANDB_TINFO_MAX + 1] = [core::ptr::null_mut(); BRIDGE_VLANDB_TINFO_MAX + 1];
    let attr: *mut nlattr;
    let vinfo: *mut bridge_vlan_info;
    let mut tun_id: u32 = 0;
    let cmd: i32;
    let err: i32;

    if p.is_null() {
        NL_SET_ERR_MSG_MOD(extack, "Can't modify tunnel mapping of non-port vlans");
        return -EINVAL;
    }
    if !test_bit(BR_VLAN_TUNNEL_BIT, &(*p).flags) {
        NL_SET_ERR_MSG_MOD(extack, "Port doesn't have tunnel flag set");
        return -EINVAL;
    }
    attr = *tb.add(BRIDGE_VLANDB_ENTRY_TUNNEL_INFO);
    err = nla_parse_nested(tun_tb.as_mut_ptr(), BRIDGE_VLANDB_TINFO_MAX, attr, br_vlandb_tinfo_pol, extack);
    if err != 0 { return err; }
    if tun_tb[BRIDGE_VLANDB_TINFO_CMD].is_null() {
        NL_SET_ERR_MSG_MOD(extack, "Missing tunnel command attribute");
        return -ENOENT;
    }
    cmd = nla_get_u32(tun_tb[BRIDGE_VLANDB_TINFO_CMD]) as i32;
    match cmd {
        RTM_SETLINK => {
            if tun_tb[BRIDGE_VLANDB_TINFO_ID].is_null() {
                NL_SET_ERR_MSG_MOD(extack, "Missing tunnel id attribute");
                return -ENOENT;
            }
            tun_id = nla_get_u32(tun_tb[BRIDGE_VLANDB_TINFO_ID]);
            vinfo = nla_data(*tb.add(BRIDGE_VLANDB_ENTRY_INFO)) as *mut bridge_vlan_info;
            tun_id = tun_id.wrapping_add((*v).vid.wrapping_sub((*vinfo).vid) as u32);
        }
        RTM_DELLINK => {}
        _ => {
            NL_SET_ERR_MSG_MOD(extack, "Unsupported tunnel command");
            return -EINVAL;
        }
    }
    br_vlan_tunnel_info(p, cmd, (*v).vid, tun_id, changed)
}

unsafe fn br_vlan_process_one_opts(
    br: *const net_bridge,
    p: *const net_bridge_port,
    vg: *mut net_bridge_vlan_group,
    v: *mut net_bridge_vlan,
    tb: *mut *mut nlattr,
    changed: *mut bool,
    extack: *mut netlink_ext_ack,
) -> i32 {
    let mut err: i32;
    *changed = false;
    if !(*tb.add(BRIDGE_VLANDB_ENTRY_STATE)).is_null() {
        err = br_vlan_modify_state(vg, v, nla_get_u8(*tb.add(BRIDGE_VLANDB_ENTRY_STATE)), changed, extack);
        if err != 0 { return err; }
    }
    if !(*tb.add(BRIDGE_VLANDB_ENTRY_TUNNEL_INFO)).is_null() {
        err = br_vlan_modify_tunnel(p, v, tb, changed, extack);
        if err != 0 { return err; }
    }
    #[cfg(CONFIG_BRIDGE_IGMP_SNOOPING)]
    {
        if !(*tb.add(BRIDGE_VLANDB_ENTRY_MCAST_ROUTER)).is_null() {
            err = br_multicast_set_vlan_router(v, nla_get_u8(*tb.add(BRIDGE_VLANDB_ENTRY_MCAST_ROUTER)));
            if err != 0 { return err; }
            *changed = true;
        }
        if !(*tb.add(BRIDGE_VLANDB_ENTRY_MCAST_MAX_GROUPS)).is_null() {
            if p.is_null() { NL_SET_ERR_MSG_MOD(extack, "Can't set mcast_max_groups for non-port vlans"); return -EINVAL; }
            if br_multicast_port_ctx_vlan_disabled(&(*v).port_mcast_ctx) { NL_SET_ERR_MSG_MOD(extack, "Multicast snooping disabled on this VLAN"); return -EINVAL; }
            br_multicast_ngroups_set_max(&(*v).port_mcast_ctx, nla_get_u32(*tb.add(BRIDGE_VLANDB_ENTRY_MCAST_MAX_GROUPS)));
            *changed = true;
        }
    }
    if !(*tb.add(BRIDGE_VLANDB_ENTRY_NEIGH_SUPPRESS)).is_null() {
        let enabled = (*v).priv_flags & BR_VLFLAG_NEIGH_SUPPRESS_ENABLED != 0;
        let val = nla_get_u8(*tb.add(BRIDGE_VLANDB_ENTRY_NEIGH_SUPPRESS)) != 0;
        if p.is_null() { NL_SET_ERR_MSG_MOD(extack, "Can't set neigh_suppress for non-port vlans"); return -EINVAL; }
        if val != enabled { (*v).priv_flags ^= BR_VLFLAG_NEIGH_SUPPRESS_ENABLED; *changed = true; }
    }
    if !(*tb.add(BRIDGE_VLANDB_ENTRY_NEIGH_FORWARD_GRAT)).is_null() {
        let enabled = (*v).priv_flags & BR_VLFLAG_NEIGH_FORWARD_GRAT_ENABLED != 0;
        let val = nla_get_u8(*tb.add(BRIDGE_VLANDB_ENTRY_NEIGH_FORWARD_GRAT)) != 0;
        if p.is_null() { NL_SET_ERR_MSG_MOD(extack, "Can't set neigh_forward_grat for non-port vlans"); return -EINVAL; }
        if val != enabled { (*v).priv_flags ^= BR_VLFLAG_NEIGH_FORWARD_GRAT_ENABLED; *changed = true; }
    }
    0
}

pub unsafe fn br_vlan_process_options(
    br: *const net_bridge, p: *const net_bridge_port,
    range_start: *mut net_bridge_vlan, range_end: *mut net_bridge_vlan,
    tb: *mut *mut nlattr, extack: *mut netlink_ext_ack,
) -> i32 {
    let vg = if !p.is_null() { nbp_vlan_group(p) } else { br_vlan_group(br) };
    if range_start.is_null() || !br_vlan_should_use(range_start) { NL_SET_ERR_MSG_MOD(extack, "Vlan range start doesn't exist, can't process options"); return -ENOENT; }
    if range_end.is_null() || !br_vlan_should_use(range_end) { NL_SET_ERR_MSG_MOD(extack, "Vlan range end doesn't exist, can't process options"); return -ENOENT; }
    let pvid = br_get_pvid(vg);
    let mut curr_start: *mut net_bridge_vlan = core::ptr::null_mut();
    let mut curr_end: *mut net_bridge_vlan = core::ptr::null_mut();
    let mut vid = (*range_start).vid;
    let mut err = 0;
    while vid <= (*range_end).vid {
        let mut changed = false;
        let v = br_vlan_find(vg, vid);
        if v.is_null() || !br_vlan_should_use(v) { NL_SET_ERR_MSG_MOD(extack, "Vlan in range doesn't exist, can't process options"); err = -ENOENT; break; }
        err = br_vlan_process_one_opts(br, p, vg, v, tb, &mut changed, extack);
        if err != 0 { break; }
        if changed {
            if curr_start.is_null() { curr_start = v; curr_end = v; vid += 1; continue; }
            if !br_vlan_can_enter_range(v, curr_end, pvid) { br_vlan_notify(br, p, (*curr_start).vid, (*curr_end).vid, RTM_NEWVLAN); curr_start = v; }
            curr_end = v;
        } else if !curr_start.is_null() {
            br_vlan_notify(br, p, (*curr_start).vid, (*curr_end).vid, RTM_NEWVLAN);
            curr_start = core::ptr::null_mut(); curr_end = core::ptr::null_mut();
        }
        vid += 1;
    }
    if !curr_start.is_null() { br_vlan_notify(br, p, (*curr_start).vid, (*curr_end).vid, RTM_NEWVLAN); }
    err
}

pub unsafe fn br_vlan_global_opts_can_enter_range(v_curr: *const net_bridge_vlan, r_end: *const net_bridge_vlan) -> bool {
    (*v_curr).vid - (*r_end).vid == 1 && (*v_curr).msti == (*r_end).msti
        && (((*v_curr).priv_flags ^ (*r_end).priv_flags) & BR_VLFLAG_GLOBAL_MCAST_ENABLED) == 0
        && br_multicast_ctx_options_equal(&(*v_curr).br_mcast_ctx, &(*r_end).br_mcast_ctx)
}

pub unsafe fn br_vlan_global_opts_fill(skb: *mut sk_buff, vid: u16, vid_range: u16, v_opts: *const net_bridge_vlan) -> bool {
    let mut nest2: *mut nlattr = core::ptr::null_mut();
    let mut clockval: u64;
    let nest = nla_nest_start(skb, BRIDGE_VLANDB_GLOBAL_OPTIONS);
    if nest.is_null() { return false; }
    if nla_put_u16(skb, BRIDGE_VLANDB_GOPTS_ID, vid) != 0 { nla_nest_cancel(skb, nest); return false; }
    if vid_range != 0 && vid < vid_range && nla_put_u16(skb, BRIDGE_VLANDB_GOPTS_RANGE, vid_range) != 0 { nla_nest_cancel(skb, nest); return false; }
    #[cfg(CONFIG_BRIDGE_IGMP_SNOOPING)]
    {
        if nla_put_u8(skb, BRIDGE_VLANDB_GOPTS_MCAST_SNOOPING, ((*v_opts).priv_flags & BR_VLFLAG_GLOBAL_MCAST_ENABLED != 0) as u8) != 0
            || nla_put_u8(skb, BRIDGE_VLANDB_GOPTS_MCAST_IGMP_VERSION, (*v_opts).br_mcast_ctx.multicast_igmp_version) != 0
            || nla_put_u32(skb, BRIDGE_VLANDB_GOPTS_MCAST_LAST_MEMBER_CNT, (*v_opts).br_mcast_ctx.multicast_last_member_count) != 0
            || nla_put_u32(skb, BRIDGE_VLANDB_GOPTS_MCAST_STARTUP_QUERY_CNT, (*v_opts).br_mcast_ctx.multicast_startup_query_count) != 0
            || nla_put_u8(skb, BRIDGE_VLANDB_GOPTS_MCAST_QUERIER, (*v_opts).br_mcast_ctx.multicast_querier) != 0
            || br_multicast_dump_querier_state(skb, &(*v_opts).br_mcast_ctx, BRIDGE_VLANDB_GOPTS_MCAST_QUERIER_STATE) != 0 { nla_nest_cancel(skb, nest); return false; }
        clockval = jiffies_to_clock_t((*v_opts).br_mcast_ctx.multicast_last_member_interval);
        if nla_put_u64_64bit(skb, BRIDGE_VLANDB_GOPTS_MCAST_LAST_MEMBER_INTVL, clockval, BRIDGE_VLANDB_GOPTS_PAD) != 0 { nla_nest_cancel(skb, nest); return false; }
        clockval = jiffies_to_clock_t((*v_opts).br_mcast_ctx.multicast_membership_interval);
        if nla_put_u64_64bit(skb, BRIDGE_VLANDB_GOPTS_MCAST_MEMBERSHIP_INTVL, clockval, BRIDGE_VLANDB_GOPTS_PAD) != 0 { nla_nest_cancel(skb, nest); return false; }
        clockval = jiffies_to_clock_t((*v_opts).br_mcast_ctx.multicast_querier_interval);
        if nla_put_u64_64bit(skb, BRIDGE_VLANDB_GOPTS_MCAST_QUERIER_INTVL, clockval, BRIDGE_VLANDB_GOPTS_PAD) != 0 { nla_nest_cancel(skb, nest); return false; }
        clockval = jiffies_to_clock_t((*v_opts).br_mcast_ctx.multicast_query_interval);
        if nla_put_u64_64bit(skb, BRIDGE_VLANDB_GOPTS_MCAST_QUERY_INTVL, clockval, BRIDGE_VLANDB_GOPTS_PAD) != 0 { nla_nest_cancel(skb, nest); return false; }
        clockval = jiffies_to_clock_t((*v_opts).br_mcast_ctx.multicast_query_response_interval);
        if nla_put_u64_64bit(skb, BRIDGE_VLANDB_GOPTS_MCAST_QUERY_RESPONSE_INTVL, clockval, BRIDGE_VLANDB_GOPTS_PAD) != 0 { nla_nest_cancel(skb, nest); return false; }
        clockval = jiffies_to_clock_t((*v_opts).br_mcast_ctx.multicast_startup_query_interval);
        if nla_put_u64_64bit(skb, BRIDGE_VLANDB_GOPTS_MCAST_STARTUP_QUERY_INTVL, clockval, BRIDGE_VLANDB_GOPTS_PAD) != 0 { nla_nest_cancel(skb, nest); return false; }
        if br_rports_have_mc_router(&(*v_opts).br_mcast_ctx) {
            nest2 = nla_nest_start(skb, BRIDGE_VLANDB_GOPTS_MCAST_ROUTER_PORTS);
            if nest2.is_null() { nla_nest_cancel(skb, nest); return false; }
            rcu_read_lock();
            if br_rports_fill_info(skb, &(*v_opts).br_mcast_ctx) != 0 { rcu_read_unlock(); nla_nest_cancel(skb, nest2); nla_nest_cancel(skb, nest); return false; }
            rcu_read_unlock(); nla_nest_end(skb, nest2);
        }
        #[cfg(IS_ENABLED_CONFIG_IPV6)]
        if nla_put_u8(skb, BRIDGE_VLANDB_GOPTS_MCAST_MLD_VERSION, (*v_opts).br_mcast_ctx.multicast_mld_version) != 0 { nla_nest_cancel(skb, nest); return false; }
    }
    if nla_put_u16(skb, BRIDGE_VLANDB_GOPTS_MSTI, (*v_opts).msti) != 0 { nla_nest_cancel(skb, nest); return false; }
    nla_nest_end(skb, nest);
    true
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
