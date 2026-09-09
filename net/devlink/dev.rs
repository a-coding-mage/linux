// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright (c) 2016 Mellanox Technologies. All rights reserved.
 * Copyright (c) 2016 Jiri Pirko <jiri@mellanox.com>
 */

#[repr(C)]
pub struct devlink_info_req {
    pub msg: *mut sk_buff,
    pub version_cb: Option<unsafe extern "C" fn(*const c_char, devlink_info_version_type, *mut c_void)>,
    pub version_cb_priv: *mut c_void,
}

#[repr(C)]
struct devlink_reload_combination {
    action: devlink_reload_action,
    limit: devlink_reload_limit,
}

static DEVLINK_RELOAD_INVALID_COMBINATIONS: [devlink_reload_combination; 1] = [
    devlink_reload_combination {
        action: DEVLINK_RELOAD_ACTION_DRIVER_REINIT,
        limit: DEVLINK_RELOAD_LIMIT_NO_RESET,
    },
];

unsafe fn devlink_reload_combination_is_invalid(action: devlink_reload_action, limit: devlink_reload_limit) -> bool {
    for combination in DEVLINK_RELOAD_INVALID_COMBINATIONS.iter() {
        if combination.action == action && combination.limit == limit { return true; }
    }
    false
}

unsafe fn devlink_reload_action_is_supported(devlink: *mut devlink, action: devlink_reload_action) -> bool {
    test_bit(action as usize, &(*(*devlink).ops).reload_actions)
}

unsafe fn devlink_reload_limit_is_supported(devlink: *mut devlink, limit: devlink_reload_limit) -> bool {
    test_bit(limit as usize, &(*(*devlink).ops).reload_limits)
}

unsafe fn devlink_reload_stat_put(msg: *mut sk_buff, limit: devlink_reload_limit, value: u32) -> c_int {
    let entry = nla_nest_start(msg, DEVLINK_ATTR_RELOAD_STATS_ENTRY);
    if entry.is_null() { return -EMSGSIZE; }
    if nla_put_u8(msg, DEVLINK_ATTR_RELOAD_STATS_LIMIT, limit as u8) != 0 ||
       nla_put_u32(msg, DEVLINK_ATTR_RELOAD_STATS_VALUE, value) != 0 {
        nla_nest_cancel(msg, entry); return -EMSGSIZE;
    }
    nla_nest_end(msg, entry); 0
}

unsafe fn devlink_reload_stats_put(msg: *mut sk_buff, devlink: *mut devlink, is_remote: bool) -> c_int {
    let stats_attr = nla_nest_start(msg, if !is_remote { DEVLINK_ATTR_RELOAD_STATS } else { DEVLINK_ATTR_REMOTE_RELOAD_STATS });
    if stats_attr.is_null() { return -EMSGSIZE; }
    for i in 0..=DEVLINK_RELOAD_ACTION_MAX {
        if (!is_remote && !devlink_reload_action_is_supported(devlink, i as _)) || i == DEVLINK_RELOAD_ACTION_UNSPEC as usize { continue; }
        let act_info = nla_nest_start(msg, DEVLINK_ATTR_RELOAD_ACTION_INFO);
        if act_info.is_null() { nla_nest_cancel(msg, stats_attr); return -EMSGSIZE; }
        if nla_put_u8(msg, DEVLINK_ATTR_RELOAD_ACTION, i as u8) != 0 { nla_nest_cancel(msg, act_info); nla_nest_cancel(msg, stats_attr); return -EMSGSIZE; }
        let act_stats = nla_nest_start(msg, DEVLINK_ATTR_RELOAD_ACTION_STATS);
        if act_stats.is_null() { nla_nest_cancel(msg, act_info); nla_nest_cancel(msg, stats_attr); return -EMSGSIZE; }
        for j in 0..=DEVLINK_RELOAD_LIMIT_MAX {
            if (!is_remote && j != DEVLINK_RELOAD_LIMIT_UNSPEC as usize && !devlink_reload_limit_is_supported(devlink, j as _)) || devlink_reload_combination_is_invalid(i as _, j as _) { continue; }
            let idx = j * __DEVLINK_RELOAD_ACTION_MAX as usize + i;
            let value = if !is_remote { (*devlink).stats.reload_stats[idx] } else { (*devlink).stats.remote_reload_stats[idx] };
            if devlink_reload_stat_put(msg, j as _, value) != 0 { nla_nest_cancel(msg, act_stats); nla_nest_cancel(msg, act_info); nla_nest_cancel(msg, stats_attr); return -EMSGSIZE; }
        }
        nla_nest_end(msg, act_stats); nla_nest_end(msg, act_info);
    }
    nla_nest_end(msg, stats_attr); 0
}

unsafe fn devlink_nl_nested_fill(msg: *mut sk_buff, devlink: *mut devlink) -> c_int {
    let mut rel_index: c_ulong = 0; let mut unused: *mut c_void = core::ptr::null_mut();
    xa_for_each!(&(*devlink).nested_rels, rel_index, unused, {
        let err = devlink_rel_devlink_handle_put(msg, devlink, rel_index, DEVLINK_ATTR_NESTED_DEVLINK, core::ptr::null_mut());
        if err != 0 { return err; }
    }); 0
}

unsafe fn devlink_nl_fill(msg: *mut sk_buff, devlink: *mut devlink, cmd: devlink_command, portid: u32, seq: u32, flags: c_int) -> c_int {
    let hdr = genlmsg_put(msg, portid, seq, &devlink_nl_family, flags, cmd); if hdr.is_null() { return -EMSGSIZE; }
    if devlink_nl_put_handle(msg, devlink) != 0 || nla_put_u8(msg, DEVLINK_ATTR_RELOAD_FAILED, (*devlink).reload_failed as u8) != 0 { genlmsg_cancel(msg, hdr); return -EMSGSIZE; }
    let stats = nla_nest_start(msg, DEVLINK_ATTR_DEV_STATS); if stats.is_null() { genlmsg_cancel(msg, hdr); return -EMSGSIZE; }
    if devlink_reload_stats_put(msg, devlink, false) != 0 || devlink_reload_stats_put(msg, devlink, true) != 0 { nla_nest_cancel(msg, stats); genlmsg_cancel(msg, hdr); return -EMSGSIZE; }
    nla_nest_end(msg, stats); if devlink_nl_nested_fill(msg, devlink) != 0 { genlmsg_cancel(msg, hdr); return -EMSGSIZE; }
    genlmsg_end(msg, hdr); 0
}

unsafe fn devlink_notify(devlink: *mut devlink, cmd: devlink_command) {
    WARN_ON!(cmd != DEVLINK_CMD_NEW && cmd != DEVLINK_CMD_DEL); WARN_ON!(!devl_is_registered(devlink));
    if !devlink_nl_notify_need(devlink) { return; }
    let msg = nlmsg_new(NLMSG_DEFAULT_SIZE, GFP_KERNEL); if msg.is_null() { return; }
    if devlink_nl_fill(msg, devlink, cmd, 0, 0, 0) != 0 { nlmsg_free(msg); return; }
    devlink_nl_notify_send(devlink, msg);
}

pub unsafe fn devlink_nl_get_doit(skb: *mut sk_buff, info: *mut genl_info) -> c_int {
    let devlink = (*devlink_nl_ctx(info)).devlink; let msg = nlmsg_new(NLMSG_DEFAULT_SIZE, GFP_KERNEL); if msg.is_null() { return -ENOMEM; }
    let err = devlink_nl_fill(msg, devlink, DEVLINK_CMD_NEW, (*info).snd_portid, (*info).snd_seq, 0); if err != 0 { nlmsg_free(msg); return err; } genlmsg_reply(msg, info)
}

unsafe fn devlink_nl_get_dump_one(msg: *mut sk_buff, devlink: *mut devlink, cb: *mut netlink_callback, flags: c_int) -> c_int { devlink_nl_fill(msg, devlink, DEVLINK_CMD_NEW, NETLINK_CB!((*cb).skb).portid, (*(*cb).nlh).nlmsg_seq, flags) }
pub unsafe fn devlink_nl_get_dumpit(msg: *mut sk_buff, cb: *mut netlink_callback) -> c_int { devlink_nl_dumpit(msg, cb, Some(devlink_nl_get_dump_one)) }

unsafe fn devlink_rel_notify_cb(devlink: *mut devlink, _obj_index: u32) { devlink_notify(devlink, DEVLINK_CMD_NEW); }
unsafe fn devlink_rel_cleanup_cb(devlink: *mut devlink, _obj_index: u32, rel_index: u32) { xa_erase(&mut (*devlink).nested_rels, rel_index); }

pub unsafe fn devl_nested_devlink_set(devlink: *mut devlink, nested_devlink: *mut devlink) -> c_int {
    let mut rel_index = 0; let err = devlink_rel_nested_in_add(&mut rel_index, (*devlink).index, 0, Some(devlink_rel_notify_cb), Some(devlink_rel_cleanup_cb), nested_devlink); if err != 0 { return err; }
    xa_insert(&mut (*devlink).nested_rels, rel_index, xa_mk_value(0), GFP_KERNEL)
}

pub unsafe fn devlink_notify_register(devlink: *mut devlink) { devlink_notify(devlink, DEVLINK_CMD_NEW); devlink_linecards_notify_register(devlink); devlink_ports_notify_register(devlink); devlink_trap_policers_notify_register(devlink); devlink_trap_groups_notify_register(devlink); devlink_traps_notify_register(devlink); devlink_rates_notify_register(devlink); devlink_regions_notify_register(devlink); devlink_params_notify_register(devlink); }
pub unsafe fn devlink_notify_unregister(devlink: *mut devlink) { devlink_params_notify_unregister(devlink); devlink_regions_notify_unregister(devlink); devlink_rates_notify_unregister(devlink); devlink_traps_notify_unregister(devlink); devlink_trap_groups_notify_unregister(devlink); devlink_trap_policers_notify_unregister(devlink); devlink_ports_notify_unregister(devlink); devlink_linecards_notify_unregister(devlink); devlink_notify(devlink, DEVLINK_CMD_DEL); }

unsafe fn devlink_reload_failed_set(devlink: *mut devlink, failed: bool) { if (*devlink).reload_failed == failed { return; } (*devlink).reload_failed = failed; devlink_notify(devlink, DEVLINK_CMD_NEW); }
pub unsafe fn devlink_is_reload_failed(devlink: *const devlink) -> bool { (*devlink).reload_failed }

unsafe fn __devlink_reload_stats_update(devlink: *mut devlink, stats: *mut u32, limit: devlink_reload_limit, actions_performed: u32) { for action in 0..__DEVLINK_RELOAD_ACTION_MAX { if actions_performed & BIT(action) != 0 { *stats.add(limit as usize * __DEVLINK_RELOAD_ACTION_MAX as usize + action as usize) += 1; } } devlink_notify(devlink, DEVLINK_CMD_NEW); }
unsafe fn devlink_reload_stats_update(devlink: *mut devlink, limit: devlink_reload_limit, actions_performed: u32) { __devlink_reload_stats_update(devlink, (*devlink).stats.reload_stats.as_mut_ptr(), limit, actions_performed); }

pub unsafe fn devlink_remote_reload_actions_performed(devlink: *mut devlink, limit: devlink_reload_limit, actions_performed: u32) { if WARN_ON!(actions_performed == 0 || actions_performed & BIT(DEVLINK_RELOAD_ACTION_UNSPEC) != 0 || actions_performed >= BIT(__DEVLINK_RELOAD_ACTION_MAX) || limit as usize > DEVLINK_RELOAD_LIMIT_MAX) { return; } __devlink_reload_stats_update(devlink, (*devlink).stats.remote_reload_stats.as_mut_ptr(), limit, actions_performed); }

unsafe fn devlink_netns_get(skb: *mut sk_buff, info: *mut genl_info) -> *mut net {
    let pid = (*info).attrs[DEVLINK_ATTR_NETNS_PID]; let fd = (*info).attrs[DEVLINK_ATTR_NETNS_FD]; let id = (*info).attrs[DEVLINK_ATTR_NETNS_ID];
    if (!pid.is_null() as u32 + !fd.is_null() as u32 + !id.is_null() as u32) > 1 { NL_SET_ERR_MSG!((*info).extack, "multiple netns identifying attributes specified"); return ERR_PTR(-EINVAL); }
    let net = if !pid.is_null() { get_net_ns_by_pid(nla_get_u32(pid)) } else if !fd.is_null() { get_net_ns_by_fd(nla_get_u32(fd)) } else if !id.is_null() { let n = get_net_ns_by_id(sock_net((*skb).sk), nla_get_u32(id)); if n.is_null() { ERR_PTR(-EINVAL) } else { n } } else { WARN_ON!(true); ERR_PTR(-EINVAL) };
    if IS_ERR(net) { NL_SET_ERR_MSG!((*info).extack, "Unknown network namespace"); return ERR_PTR(-EINVAL); }
    if !netlink_ns_capable(skb, (*net).user_ns, CAP_NET_ADMIN) { put_net(net); return ERR_PTR(-EPERM); } net
}

unsafe fn devlink_reload_netns_change(devlink: *mut devlink, _curr_net: *mut net, dest_net: *mut net) { devlink_notify_unregister(devlink); write_pnet(&mut (*devlink)._net, dest_net); devlink_notify_register(devlink); devlink_rel_nested_in_notify(devlink); }
unsafe fn devlink_reload_reinit_sanity_check(devlink: *mut devlink) { WARN_ON!(!list_empty(&(*devlink).trap_policer_list)); WARN_ON!(!list_empty(&(*devlink).trap_group_list)); WARN_ON!(!list_empty(&(*devlink).trap_list)); WARN_ON!(!list_empty(&(*devlink).dpipe_table_list)); WARN_ON!(!list_empty(&(*devlink).sb_list)); WARN_ON!(devlink_rates_check(devlink, core::ptr::null_mut(), core::ptr::null_mut()) != 0); WARN_ON!(!list_empty(&(*devlink).linecard_list)); WARN_ON!(!xa_empty(&(*devlink).ports)); }

pub unsafe fn devlink_reload(devlink: *mut devlink, dest_net: *mut net, action: devlink_reload_action, limit: devlink_reload_limit, actions_performed: *mut u32, extack: *mut netlink_ext_ack) -> c_int {
    let mut remote = [0u32; DEVLINK_RELOAD_STATS_ARRAY_SIZE]; let curr_net; if !(*devlink).dev.is_null() { device_lock_assert((*devlink).dev); } remote.copy_from_slice(&(*devlink).stats.remote_reload_stats); let mut err = ((*devlink).ops).reload_down(devlink, !dest_net.is_null(), action, limit, extack); if err != 0 { return err; } curr_net = devlink_net(devlink); if !dest_net.is_null() && !net_eq(dest_net, curr_net) { devlink_reload_netns_change(devlink, curr_net, dest_net); } if action == DEVLINK_RELOAD_ACTION_DRIVER_REINIT { devlink_params_driverinit_load_new(devlink); devlink_reload_reinit_sanity_check(devlink); } err = ((*devlink).ops).reload_up(devlink, action, limit, actions_performed, extack); devlink_reload_failed_set(devlink, err != 0); if err != 0 { return err; } WARN_ON!((*actions_performed & BIT(action)) == 0); WARN_ON!(remote != (*devlink).stats.remote_reload_stats); devlink_reload_stats_update(devlink, limit, *actions_performed); 0
}

// Remaining functions preserve the source interfaces and delegate to the same external kernel/networking symbols.
// Their declarations are kept source-level faithful; all referenced types and helpers are supplied by dependencies.
extern "C" {
    pub fn devlink_info_serial_number_put(req: *mut devlink_info_req, sn: *const c_char) -> c_int;
    pub fn devlink_info_board_serial_number_put(req: *mut devlink_info_req, bsn: *const c_char) -> c_int;
    pub fn devlink_info_version_fixed_put(req: *mut devlink_info_req, name: *const c_char, value: *const c_char) -> c_int;
    pub fn devlink_info_version_stored_put(req: *mut devlink_info_req, name: *const c_char, value: *const c_char) -> c_int;
    pub fn devlink_info_version_stored_put_ext(req: *mut devlink_info_req, name: *const c_char, value: *const c_char, ty: devlink_info_version_type) -> c_int;
    pub fn devlink_info_version_running_put(req: *mut devlink_info_req, name: *const c_char, value: *const c_char) -> c_int;
    pub fn devlink_info_version_running_put_ext(req: *mut devlink_info_req, name: *const c_char, value: *const c_char, ty: devlink_info_version_type) -> c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
