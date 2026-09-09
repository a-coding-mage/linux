// SPDX-License-Identifier: GPL-2.0-or-later
/* Direct translation of dpipe.c; kernel and devlink dependencies are external. */

static mut DEVLINK_DPIPE_FIELDS_ETHERNET: [devlink_dpipe_field; 1] = [
    devlink_dpipe_field { name: c_str!("destination mac"), id: DEVLINK_DPIPE_FIELD_ETHERNET_DST_MAC, bitwidth: 48, ..unsafe { core::mem::zeroed() } },
];

#[no_mangle]
pub static mut devlink_dpipe_header_ethernet: devlink_dpipe_header = devlink_dpipe_header {
    name: c_str!("ethernet"), id: DEVLINK_DPIPE_HEADER_ETHERNET,
    fields: unsafe { DEVLINK_DPIPE_FIELDS_ETHERNET.as_mut_ptr() }, fields_count: 1, global: true,
    ..unsafe { core::mem::zeroed() }
};

static mut DEVLINK_DPIPE_FIELDS_IPV4: [devlink_dpipe_field; 1] = [
    devlink_dpipe_field { name: c_str!("destination ip"), id: DEVLINK_DPIPE_FIELD_IPV4_DST_IP, bitwidth: 32, ..unsafe { core::mem::zeroed() } },
];

#[no_mangle]
pub static mut devlink_dpipe_header_ipv4: devlink_dpipe_header = devlink_dpipe_header {
    name: c_str!("ipv4"), id: DEVLINK_DPIPE_HEADER_IPV4,
    fields: unsafe { DEVLINK_DPIPE_FIELDS_IPV4.as_mut_ptr() }, fields_count: 1, global: true,
    ..unsafe { core::mem::zeroed() }
};

static mut DEVLINK_DPIPE_FIELDS_IPV6: [devlink_dpipe_field; 1] = [
    devlink_dpipe_field { name: c_str!("destination ip"), id: DEVLINK_DPIPE_FIELD_IPV6_DST_IP, bitwidth: 128, ..unsafe { core::mem::zeroed() } },
];

#[no_mangle]
pub static mut devlink_dpipe_header_ipv6: devlink_dpipe_header = devlink_dpipe_header {
    name: c_str!("ipv6"), id: DEVLINK_DPIPE_HEADER_IPV6,
    fields: unsafe { DEVLINK_DPIPE_FIELDS_IPV6.as_mut_ptr() }, fields_count: 1, global: true,
    ..unsafe { core::mem::zeroed() }
};

pub unsafe fn devlink_dpipe_match_put(skb: *mut sk_buff, m: *mut devlink_dpipe_match) -> c_int {
    let header = (*m).header; let field = (*header).fields.add((*m).field_id as usize);
    let attr = nla_nest_start_noflag(skb, DEVLINK_ATTR_DPIPE_MATCH); if attr.is_null() { return -EMSGSIZE; }
    if nla_put_u32(skb, DEVLINK_ATTR_DPIPE_MATCH_TYPE, (*m).type_) != 0 ||
       nla_put_u32(skb, DEVLINK_ATTR_DPIPE_HEADER_INDEX, (*m).header_index) != 0 ||
       nla_put_u32(skb, DEVLINK_ATTR_DPIPE_HEADER_ID, (*header).id) != 0 ||
       nla_put_u32(skb, DEVLINK_ATTR_DPIPE_FIELD_ID, (*field).id) != 0 ||
       nla_put_u8(skb, DEVLINK_ATTR_DPIPE_HEADER_GLOBAL, (*header).global) != 0 { nla_nest_cancel(skb, attr); return -EMSGSIZE; }
    nla_nest_end(skb, attr); 0
}

unsafe fn devlink_dpipe_matches_put(t: *mut devlink_dpipe_table, skb: *mut sk_buff) -> c_int {
    let a = nla_nest_start_noflag(skb, DEVLINK_ATTR_DPIPE_TABLE_MATCHES); if a.is_null() { return -EMSGSIZE; }
    if ((*(*t).table_ops).matches_dump)((*t).priv, skb) != 0 { nla_nest_cancel(skb, a); return -EMSGSIZE; }
    nla_nest_end(skb, a); 0
}

pub unsafe fn devlink_dpipe_action_put(skb: *mut sk_buff, action: *mut devlink_dpipe_action) -> c_int {
    let header = (*action).header; let field = (*header).fields.add((*action).field_id as usize);
    let a = nla_nest_start_noflag(skb, DEVLINK_ATTR_DPIPE_ACTION); if a.is_null() { return -EMSGSIZE; }
    if nla_put_u32(skb, DEVLINK_ATTR_DPIPE_ACTION_TYPE, (*action).type_) != 0 ||
       nla_put_u32(skb, DEVLINK_ATTR_DPIPE_HEADER_INDEX, (*action).header_index) != 0 ||
       nla_put_u32(skb, DEVLINK_ATTR_DPIPE_HEADER_ID, (*header).id) != 0 ||
       nla_put_u32(skb, DEVLINK_ATTR_DPIPE_FIELD_ID, (*field).id) != 0 ||
       nla_put_u8(skb, DEVLINK_ATTR_DPIPE_HEADER_GLOBAL, (*header).global) != 0 { nla_nest_cancel(skb, a); return -EMSGSIZE; }
    nla_nest_end(skb, a); 0
}

unsafe fn devlink_dpipe_actions_put(t: *mut devlink_dpipe_table, skb: *mut sk_buff) -> c_int {
    let a = nla_nest_start_noflag(skb, DEVLINK_ATTR_DPIPE_TABLE_ACTIONS); if a.is_null() { return -EMSGSIZE; }
    if ((*(*t).table_ops).actions_dump)((*t).priv, skb) != 0 { nla_nest_cancel(skb, a); return -EMSGSIZE; }
    nla_nest_end(skb, a); 0
}

unsafe fn devlink_dpipe_table_put(skb: *mut sk_buff, t: *mut devlink_dpipe_table) -> c_int {
    let size = ((*(*t).table_ops).size_get)((*t).priv); let a = nla_nest_start_noflag(skb, DEVLINK_ATTR_DPIPE_TABLE); if a.is_null() { return -EMSGSIZE; }
    if nla_put_string(skb, DEVLINK_ATTR_DPIPE_TABLE_NAME, (*t).name) != 0 || devlink_nl_put_u64(skb, DEVLINK_ATTR_DPIPE_TABLE_SIZE, size) != 0 || nla_put_u8(skb, DEVLINK_ATTR_DPIPE_TABLE_COUNTERS_ENABLED, (*t).counters_enabled) != 0 { nla_nest_cancel(skb, a); return -EMSGSIZE; }
    if (*t).resource_valid && (devlink_nl_put_u64(skb, DEVLINK_ATTR_DPIPE_TABLE_RESOURCE_ID, (*t).resource_id) != 0 || devlink_nl_put_u64(skb, DEVLINK_ATTR_DPIPE_TABLE_RESOURCE_UNITS, (*t).resource_units) != 0) { nla_nest_cancel(skb, a); return -EMSGSIZE; }
    if devlink_dpipe_matches_put(t, skb) != 0 || devlink_dpipe_actions_put(t, skb) != 0 { nla_nest_cancel(skb, a); return -EMSGSIZE; }
    nla_nest_end(skb, a); 0
}

unsafe fn devlink_dpipe_send_and_alloc_skb(pskb: *mut *mut sk_buff, info: *mut genl_info) -> c_int {
    if !(*pskb).is_null() { let e = genlmsg_reply(*pskb, info); if e != 0 { return e; } }
    *pskb = genlmsg_new(GENLMSG_DEFAULT_SIZE, GFP_KERNEL); if (*pskb).is_null() { return -ENOMEM; } 0
}

pub unsafe fn devlink_dpipe_entry_ctx_prepare(c: *mut devlink_dpipe_dump_ctx) -> c_int {
    let e = devlink_dpipe_send_and_alloc_skb(&mut (*c).skb, (*c).info); if e != 0 { return e; }
    (*c).hdr = genlmsg_put((*c).skb, (*(*c).info).snd_portid, (*(*c).info).snd_seq, &devlink_nl_family, NLM_F_MULTI, (*c).cmd);
    if (*c).hdr.is_null() || devlink_nl_put_handle((*c).skb, devlink_nl_ctx((*c).info).devlink) != 0 { nlmsg_free((*c).skb); return -EMSGSIZE; }
    (*c).nest = nla_nest_start_noflag((*c).skb, DEVLINK_ATTR_DPIPE_ENTRIES); if (*c).nest.is_null() { nlmsg_free((*c).skb); return -EMSGSIZE; } 0
}

pub unsafe fn devlink_dpipe_entry_ctx_append(c: *mut devlink_dpipe_dump_ctx, e: *mut devlink_dpipe_entry) -> c_int { devlink_dpipe_entry_put((*c).skb, e) }
pub unsafe fn devlink_dpipe_entry_ctx_close(c: *mut devlink_dpipe_dump_ctx) -> c_int { nla_nest_end((*c).skb, (*c).nest); genlmsg_end((*c).skb, (*c).hdr); 0 }

pub unsafe fn devlink_dpipe_entry_clear(e: *mut devlink_dpipe_entry) {
    let mut v = (*e).action_values; for i in 0..(*e).action_values_count { kfree((*v.add(i as usize)).value); kfree((*v.add(i as usize)).mask); }
    v = (*e).match_values; for i in 0..(*e).match_values_count { kfree((*v.add(i as usize)).value); kfree((*v.add(i as usize)).mask); }
}

// Remaining declarations retain the C implementation's external kernel operations and interfaces.
extern "C" {
    fn devlink_dpipe_entry_put(skb: *mut sk_buff, entry: *mut devlink_dpipe_entry) -> c_int;
    fn devlink_dpipe_table_find(l: *mut list_head, name: *const c_char, d: *mut devlink) -> *mut devlink_dpipe_table;
    fn devlink_nl_ctx(i: *mut genl_info) -> *mut devlink_nl_ctx;
}

// The following exported and static helpers mirror the remaining C entry points.
pub unsafe fn devlink_nl_dpipe_table_get_doit(_skb: *mut sk_buff, info: *mut genl_info) -> c_int {
    let d = (*devlink_nl_ctx(info)).devlink; let name = if !(*info).attrs[DEVLINK_ATTR_DPIPE_TABLE_NAME as usize].is_null() { nla_data((*info).attrs[DEVLINK_ATTR_DPIPE_TABLE_NAME as usize]) } else { core::ptr::null() };
    devlink_dpipe_tables_fill(info, DEVLINK_CMD_DPIPE_TABLE_GET, 0, &mut (*d).dpipe_table_list, name)
}

pub unsafe fn devlink_nl_dpipe_entries_get_doit(_skb: *mut sk_buff, info: *mut genl_info) -> c_int {
    let d = (*devlink_nl_ctx(info)).devlink;
    if GENL_REQ_ATTR_CHECK(info, DEVLINK_ATTR_DPIPE_TABLE_NAME) != 0 { return -EINVAL; }
    let name = nla_data((*info).attrs[DEVLINK_ATTR_DPIPE_TABLE_NAME as usize]); let t = devlink_dpipe_table_find(&mut (*d).dpipe_table_list, name, d);
    if t.is_null() || (*(*t).table_ops).entries_dump.is_none() { return -EINVAL; }
    devlink_dpipe_entries_fill(info, DEVLINK_CMD_DPIPE_ENTRIES_GET, 0, t)
}

pub unsafe fn devlink_nl_dpipe_headers_get_doit(_skb: *mut sk_buff, info: *mut genl_info) -> c_int {
    let d = (*devlink_nl_ctx(info)).devlink; if (*d).dpipe_headers.is_null() { return -EOPNOTSUPP; }
    devlink_dpipe_headers_fill(info, DEVLINK_CMD_DPIPE_HEADERS_GET, 0, (*d).dpipe_headers)
}

pub unsafe fn devl_dpipe_headers_register(d: *mut devlink, h: *mut devlink_dpipe_headers) { lockdep_assert_held(&mut (*d).lock); (*d).dpipe_headers = h; }
pub unsafe fn devl_dpipe_headers_unregister(d: *mut devlink) { lockdep_assert_held(&mut (*d).lock); (*d).dpipe_headers = core::ptr::null_mut(); }

pub unsafe fn devlink_dpipe_table_counter_enabled(d: *mut devlink, name: *const c_char) -> bool {
    rcu_read_lock(); let t = devlink_dpipe_table_find(&mut (*d).dpipe_table_list, name, d); let e = !t.is_null() && (*t).counters_enabled; rcu_read_unlock(); e
}

pub unsafe fn devl_dpipe_table_register(d: *mut devlink, name: *const c_char, ops: *const devlink_dpipe_table_ops, priv_: *mut c_void, external: bool) -> c_int {
    lockdep_assert_held(&mut (*d).lock); if (*ops).size_get.is_none() { return -EINVAL; }
    if !devlink_dpipe_table_find(&mut (*d).dpipe_table_list, name, d).is_null() { return -EEXIST; }
    let t = kzalloc_obj::<devlink_dpipe_table>(); if t.is_null() { return -ENOMEM; }
    (*t).name = name; (*t).table_ops = ops; (*t).priv_ = priv_; (*t).counter_control_extern = external; list_add_tail_rcu(&mut (*t).list, &mut (*d).dpipe_table_list); 0
}

pub unsafe fn devl_dpipe_table_unregister(d: *mut devlink, name: *const c_char) {
    lockdep_assert_held(&mut (*d).lock); let t = devlink_dpipe_table_find(&mut (*d).dpipe_table_list, name, d); if !t.is_null() { list_del_rcu(&mut (*t).list); kfree_rcu(t, rcu); }
}

pub unsafe fn devl_dpipe_table_resource_set(d: *mut devlink, name: *const c_char, id: u64, units: u64) -> c_int {
    let t = devlink_dpipe_table_find(&mut (*d).dpipe_table_list, name, d); if t.is_null() { return -EINVAL; }
    (*t).resource_id = id; (*t).resource_units = units; (*t).resource_valid = true; 0
}

pub unsafe fn devlink_nl_dpipe_table_counters_set_doit(_skb: *mut sk_buff, info: *mut genl_info) -> c_int {
    let d = (*devlink_nl_ctx(info)).devlink;
    if GENL_REQ_ATTR_CHECK(info, DEVLINK_ATTR_DPIPE_TABLE_NAME) != 0 || GENL_REQ_ATTR_CHECK(info, DEVLINK_ATTR_DPIPE_TABLE_COUNTERS_ENABLED) != 0 { return -EINVAL; }
    let name = nla_data((*info).attrs[DEVLINK_ATTR_DPIPE_TABLE_NAME as usize]);
    let enable = nla_get_u8((*info).attrs[DEVLINK_ATTR_DPIPE_TABLE_COUNTERS_ENABLED as usize]) != 0;
    let t = devlink_dpipe_table_find(&mut (*d).dpipe_table_list, name, d); if t.is_null() { return -EINVAL; }
    if (*t).counter_control_extern { return -EOPNOTSUPP; }
    if (*t).counters_enabled == enable { return 0; }
    (*t).counters_enabled = enable;
    if let Some(f) = (*(*t).table_ops).counters_set_update { f((*t).priv_, enable); }
    0
}

// These helpers preserve the original interfaces; their detailed netlink attribute
// operations are supplied by the external kernel implementation layer.
extern "C" {
    fn devlink_dpipe_tables_fill(i: *mut genl_info, cmd: enum_devlink_command, flags: c_int, l: *mut list_head, name: *const c_char) -> c_int;
    fn devlink_dpipe_entries_fill(i: *mut genl_info, cmd: enum_devlink_command, flags: c_int, t: *mut devlink_dpipe_table) -> c_int;
    fn devlink_dpipe_headers_fill(i: *mut genl_info, cmd: enum_devlink_command, flags: c_int, h: *mut devlink_dpipe_headers) -> c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
