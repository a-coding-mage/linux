// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *	Bridge per vlan tunnel port dst_metadata netlink control interface
 *
 *	Authors:
 *	Roopa Prabhu		<roopa@cumulusnetworks.com>
 */

// Linux and bridge-private dependencies are supplied by the surrounding build.

unsafe fn __get_vlan_tinfo_size() -> usize {
    nla_total_size(0) + // nest IFLA_BRIDGE_VLAN_TUNNEL_INFO
        nla_total_size(core::mem::size_of::<u32>()) + // IFLA_BRIDGE_VLAN_TUNNEL_ID
        nla_total_size(core::mem::size_of::<u16>()) + // IFLA_BRIDGE_VLAN_TUNNEL_VID
        nla_total_size(core::mem::size_of::<u16>()) // IFLA_BRIDGE_VLAN_TUNNEL_FLAGS
}

pub unsafe fn vlan_tunid_inrange(
    v_curr: *const net_bridge_vlan,
    v_last: *const net_bridge_vlan,
) -> bool {
    let tunid_curr: __be32 = tunnel_id_to_key32((*v_curr).tinfo.tunnel_id);
    let tunid_last: __be32 = tunnel_id_to_key32((*v_last).tinfo.tunnel_id);

    be32_to_cpu(tunid_curr).wrapping_sub(be32_to_cpu(tunid_last)) == 1
}

unsafe fn __get_num_vlan_tunnel_infos(vg: *mut net_bridge_vlan_group) -> i32 {
    let mut vtbegin: *mut net_bridge_vlan = core::ptr::null_mut();
    let mut vtend: *mut net_bridge_vlan = core::ptr::null_mut();
    let mut num_tinfos = 0;

    // Count number of vlan infos
    list_for_each_entry_rcu!(v, &(*vg).vlan_list, vlist, {
        // only a context, bridge vlan not activated
        if !br_vlan_should_use(v) || (*v).tinfo.tunnel_id == 0 {
            continue;
        }

        if vtbegin.is_null() {
            goto_initvars!();
        } else if ((*v).vid - (*vtend).vid) == 1 && vlan_tunid_inrange(v, vtend) {
            vtend = v;
            continue;
        } else {
            if ((*vtend).vid - (*vtbegin).vid) > 0 {
                num_tinfos += 2;
            } else {
                num_tinfos += 1;
            }
        }
        goto_initvars!();
    });

    if !vtbegin.is_null() && !vtend.is_null() {
        if ((*vtend).vid - (*vtbegin).vid) > 0 {
            num_tinfos += 2;
        } else {
            num_tinfos += 1;
        }
    }

    num_tinfos
}

pub unsafe fn br_get_vlan_tunnel_info_size(vg: *mut net_bridge_vlan_group) -> i32 {
    if vg.is_null() {
        return 0;
    }

    rcu_read_lock();
    let num_tinfos = __get_num_vlan_tunnel_infos(vg);
    rcu_read_unlock();

    num_tinfos * __get_vlan_tinfo_size() as i32
}

unsafe fn br_fill_vlan_tinfo(
    skb: *mut sk_buff,
    vid: u16,
    tunnel_id: __be64,
    flags: u16,
) -> i32 {
    let tid: __be32 = tunnel_id_to_key32(tunnel_id);
    let tmap = nla_nest_start_noflag(skb, IFLA_BRIDGE_VLAN_TUNNEL_INFO);
    if tmap.is_null() {
        return -EMSGSIZE;
    }
    if nla_put_u32(skb, IFLA_BRIDGE_VLAN_TUNNEL_ID, be32_to_cpu(tid)) != 0
        || nla_put_u16(skb, IFLA_BRIDGE_VLAN_TUNNEL_VID, vid) != 0
        || nla_put_u16(skb, IFLA_BRIDGE_VLAN_TUNNEL_FLAGS, flags) != 0
    {
        nla_nest_cancel(skb, tmap);
        return -EMSGSIZE;
    }
    nla_nest_end(skb, tmap);
    0
}

unsafe fn br_fill_vlan_tinfo_range(
    skb: *mut sk_buff,
    vtbegin: *mut net_bridge_vlan,
    vtend: *mut net_bridge_vlan,
) -> i32 {
    if !vtend.is_null() && ((*vtend).vid - (*vtbegin).vid) > 0 {
        let mut err = br_fill_vlan_tinfo(
            skb, (*vtbegin).vid, (*vtbegin).tinfo.tunnel_id,
            BRIDGE_VLAN_INFO_RANGE_BEGIN,
        );
        if err != 0 { return err; }
        err = br_fill_vlan_tinfo(
            skb, (*vtend).vid, (*vtend).tinfo.tunnel_id,
            BRIDGE_VLAN_INFO_RANGE_END,
        );
        if err != 0 { return err; }
    } else {
        let err = br_fill_vlan_tinfo(skb, (*vtbegin).vid, (*vtbegin).tinfo.tunnel_id, 0);
        if err != 0 { return err; }
    }
    0
}

pub unsafe fn br_fill_vlan_tunnel_info(
    skb: *mut sk_buff,
    vg: *mut net_bridge_vlan_group,
) -> i32 {
    let mut vtbegin: *mut net_bridge_vlan = core::ptr::null_mut();
    let mut vtend: *mut net_bridge_vlan = core::ptr::null_mut();

    // Count number of vlan infos
    list_for_each_entry_rcu!(v, &(*vg).vlan_list, vlist, {
        // only a context, bridge vlan not activated
        if !br_vlan_should_use(v) || (*v).tinfo.tunnel_dst.is_null() { continue; }
        if vtbegin.is_null() {
            vtbegin = v;
            vtend = v;
        } else if ((*v).vid - (*vtend).vid) == 1 && vlan_tunid_inrange(v, vtend) {
            vtend = v;
        } else {
            let err = br_fill_vlan_tinfo_range(skb, vtbegin, vtend);
            if err != 0 { return err; }
            vtbegin = v;
            vtend = v;
        }
    });

    if !vtbegin.is_null() {
        let err = br_fill_vlan_tinfo_range(skb, vtbegin, vtend);
        if err != 0 { return err; }
    }
    0
}

// Equivalent to the C nla_policy table; constants and struct layout are supplied externally.
static vlan_tunnel_policy: [nla_policy; IFLA_BRIDGE_VLAN_TUNNEL_MAX + 1] = [
    nla_policy { strict_start_type: IFLA_BRIDGE_VLAN_TUNNEL_FLAGS + 1, ..nla_policy::default() },
    nla_policy { type_: NLA_U32, ..nla_policy::default() },
    nla_policy { type_: NLA_U16, ..nla_policy::default() },
    nla_policy { type_: NLA_U16, ..nla_policy::default() },
];

pub unsafe fn br_vlan_tunnel_info(
    p: *const net_bridge_port, cmd: i32, vid: u16, tun_id: u32, changed: *mut bool,
) -> i32 {
    if p.is_null() { return -EINVAL; }
    let mut err = 0;
    match cmd {
        RTM_SETLINK => { err = nbp_vlan_tunnel_info_add(p, vid, tun_id); if err == 0 { *changed = true; } }
        RTM_DELLINK => { if nbp_vlan_tunnel_info_delete(p, vid) == 0 { *changed = true; } }
        _ => {}
    }
    err
}

pub unsafe fn br_parse_vlan_tunnel_info(attr: *mut nlattr, tinfo: *mut vtunnel_info) -> i32 {
    let mut tb: [*mut nlattr; IFLA_BRIDGE_VLAN_TUNNEL_MAX + 1] = [core::ptr::null_mut(); IFLA_BRIDGE_VLAN_TUNNEL_MAX + 1];
    core::ptr::write_bytes(tinfo, 0, 1);
    let err = nla_parse_nested_deprecated(tb.as_mut_ptr(), IFLA_BRIDGE_VLAN_TUNNEL_MAX, attr, &vlan_tunnel_policy, core::ptr::null_mut());
    if err < 0 { return err; }
    if tb[IFLA_BRIDGE_VLAN_TUNNEL_ID].is_null() || tb[IFLA_BRIDGE_VLAN_TUNNEL_VID].is_null() { return -EINVAL; }
    let tun_id = nla_get_u32(tb[IFLA_BRIDGE_VLAN_TUNNEL_ID]);
    let vid = nla_get_u16(tb[IFLA_BRIDGE_VLAN_TUNNEL_VID]);
    if vid >= VLAN_VID_MASK { return -ERANGE; }
    (*tinfo).tunid = tun_id;
    (*tinfo).vid = vid;
    (*tinfo).flags = if tb[IFLA_BRIDGE_VLAN_TUNNEL_FLAGS].is_null() { 0 } else { nla_get_u16(tb[IFLA_BRIDGE_VLAN_TUNNEL_FLAGS]) };
    0
}

// send a notification if v_curr can't enter the range and start a new one
unsafe fn __vlan_tunnel_handle_range(p: *const net_bridge_port, v_start: *mut *mut net_bridge_vlan, v_end: *mut *mut net_bridge_vlan, v_curr: i32, curr_change: bool) {
    let vg = nbp_vlan_group(p);
    if vg.is_null() { return; }
    let v = br_vlan_find(vg, v_curr);
    if (*v_start).is_null() { *v_start = if curr_change { v } else { core::ptr::null_mut() }; *v_end = *v_start; return; }
    if !v.is_null() && curr_change && br_vlan_can_enter_range(v, *v_end, br_get_pvid(vg)) { *v_end = v; return; }
    br_vlan_notify((*p).br, p, (**v_start).vid, (**v_end).vid, RTM_NEWVLAN);
    *v_start = if curr_change { v } else { core::ptr::null_mut() };
    *v_end = *v_start;
}

pub unsafe fn br_process_vlan_tunnel_info(br: *const net_bridge, p: *const net_bridge_port, cmd: i32, tinfo_curr: *mut vtunnel_info, tinfo_last: *mut vtunnel_info, changed: *mut bool) -> i32 {
    let mut err;
    if (*tinfo_curr).flags & BRIDGE_VLAN_INFO_RANGE_BEGIN != 0 {
        if (*tinfo_last).flags & BRIDGE_VLAN_INFO_RANGE_BEGIN != 0 { return -EINVAL; }
        core::ptr::copy_nonoverlapping(tinfo_curr, tinfo_last, 1);
    } else if (*tinfo_curr).flags & BRIDGE_VLAN_INFO_RANGE_END != 0 {
        if (*tinfo_last).flags & BRIDGE_VLAN_INFO_RANGE_BEGIN == 0 { return -EINVAL; }
        if (*tinfo_curr).vid < (*tinfo_last).vid || ((*tinfo_curr).vid - (*tinfo_last).vid) != ((*tinfo_curr).tunid - (*tinfo_last).tunid) { return -EINVAL; }
        let mut v_start = core::ptr::null_mut(); let mut v_end = core::ptr::null_mut();
        let mut t = (*tinfo_last).tunid;
        let mut v = (*tinfo_last).vid;
        while v <= (*tinfo_curr).vid {
            let mut curr_change = false;
            err = br_vlan_tunnel_info(p, cmd, v, t, &mut curr_change);
            if err != 0 { break; }
            t += 1; if curr_change { *changed = curr_change; }
            __vlan_tunnel_handle_range(p, &mut v_start, &mut v_end, v as i32, curr_change);
            v += 1;
        }
        if !v_start.is_null() && !v_end.is_null() { br_vlan_notify(br, p, (*v_start).vid, (*v_end).vid, RTM_NEWVLAN); }
        if err != 0 { return err; }
        core::ptr::write_bytes(tinfo_last, 0, 1); core::ptr::write_bytes(tinfo_curr, 0, 1);
    } else {
        if (*tinfo_last).flags != 0 { return -EINVAL; }
        err = br_vlan_tunnel_info(p, cmd, (*tinfo_curr).vid, (*tinfo_curr).tunid, changed);
        if err != 0 { return err; }
        br_vlan_notify(br, p, (*tinfo_curr).vid, 0, RTM_NEWVLAN);
        core::ptr::write_bytes(tinfo_last, 0, 1); core::ptr::write_bytes(tinfo_curr, 0, 1);
    }
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
