// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright (c) 2016 Mellanox Technologies. All rights reserved.
 * Copyright (c) 2016 Jiri Pirko <jiri@mellanox.com>
 */

#[repr(C)]
pub struct devlink_linecard {
    pub list: list_head,
    pub devlink: *mut devlink,
    pub index: c_uint,
    pub ops: *const devlink_linecard_ops,
    pub priv_: *mut c_void,
    pub state: devlink_linecard_state,
    pub state_lock: mutex,
    pub type_: *const c_char,
    pub types: *mut devlink_linecard_type,
    pub types_count: c_uint,
    pub rel_index: u32,
}

pub unsafe fn devlink_linecard_index(linecard: *mut devlink_linecard) -> c_uint {
    (*linecard).index
}

unsafe fn devlink_linecard_get_by_index(devlink: *mut devlink, linecard_index: c_uint) -> *mut devlink_linecard {
    let mut devlink_linecard: *mut devlink_linecard = core::ptr::null_mut();
    list_for_each_entry!(devlink_linecard, &mut (*devlink).linecard_list, list, {
        if (*devlink_linecard).index == linecard_index { return devlink_linecard; }
    });
    core::ptr::null_mut()
}

unsafe fn devlink_linecard_index_exists(devlink: *mut devlink, linecard_index: c_uint) -> bool {
    !devlink_linecard_get_by_index(devlink, linecard_index).is_null()
}

unsafe fn devlink_linecard_get_from_attrs(devlink: *mut devlink, attrs: *mut *mut nlattr) -> *mut devlink_linecard {
    if !(*attrs.add(DEVLINK_ATTR_LINECARD_INDEX as usize)).is_null() {
        let linecard_index = nla_get_u32(*attrs.add(DEVLINK_ATTR_LINECARD_INDEX as usize));
        let linecard = devlink_linecard_get_by_index(devlink, linecard_index);
        if linecard.is_null() { return ERR_PTR(-ENODEV); }
        return linecard;
    }
    ERR_PTR(-EINVAL)
}

unsafe fn devlink_linecard_get_from_info(devlink: *mut devlink, info: *mut genl_info) -> *mut devlink_linecard {
    devlink_linecard_get_from_attrs(devlink, (*info).attrs)
}

#[repr(C)]
pub struct devlink_linecard_type { pub type_: *const c_char, pub priv_: *const c_void }

unsafe fn devlink_nl_linecard_fill(msg: *mut sk_buff, devlink: *mut devlink, linecard: *mut devlink_linecard, cmd: devlink_command, portid: u32, seq: u32, flags: c_int, extack: *mut netlink_ext_ack) -> c_int {
    let hdr = genlmsg_put(msg, portid, seq, &devlink_nl_family, flags, cmd);
    if hdr.is_null() { return -EMSGSIZE; }
    if devlink_nl_put_handle(msg, devlink) != 0 { genlmsg_cancel(msg, hdr); return -EMSGSIZE; }
    if nla_put_u32(msg, DEVLINK_ATTR_LINECARD_INDEX, (*linecard).index) != 0 { genlmsg_cancel(msg, hdr); return -EMSGSIZE; }
    if nla_put_u8(msg, DEVLINK_ATTR_LINECARD_STATE, (*linecard).state) != 0 { genlmsg_cancel(msg, hdr); return -EMSGSIZE; }
    if !(*linecard).type_.is_null() && nla_put_string(msg, DEVLINK_ATTR_LINECARD_TYPE, (*linecard).type_) != 0 { genlmsg_cancel(msg, hdr); return -EMSGSIZE; }
    if (*linecard).types_count != 0 {
        let attr = nla_nest_start(msg, DEVLINK_ATTR_LINECARD_SUPPORTED_TYPES);
        if attr.is_null() { genlmsg_cancel(msg, hdr); return -EMSGSIZE; }
        for i in 0..(*linecard).types_count {
            let linecard_type = &*(*linecard).types.add(i as usize);
            if nla_put_string(msg, DEVLINK_ATTR_LINECARD_TYPE, linecard_type.type_) != 0 { nla_nest_cancel(msg, attr); genlmsg_cancel(msg, hdr); return -EMSGSIZE; }
        }
        nla_nest_end(msg, attr);
    }
    if devlink_rel_devlink_handle_put(msg, devlink, (*linecard).rel_index, DEVLINK_ATTR_NESTED_DEVLINK, core::ptr::null_mut()) != 0 { genlmsg_cancel(msg, hdr); return -EMSGSIZE; }
    genlmsg_end(msg, hdr); 0
}

unsafe fn devlink_linecard_notify(linecard: *mut devlink_linecard, cmd: devlink_command) {
    let devlink = (*linecard).devlink;
    WARN_ON!(cmd != DEVLINK_CMD_LINECARD_NEW && cmd != DEVLINK_CMD_LINECARD_DEL);
    if !__devl_is_registered(devlink) || !devlink_nl_notify_need(devlink) { return; }
    let msg = nlmsg_new(NLMSG_DEFAULT_SIZE, GFP_KERNEL);
    if msg.is_null() { return; }
    if devlink_nl_linecard_fill(msg, devlink, linecard, cmd, 0, 0, 0, core::ptr::null_mut()) != 0 { nlmsg_free(msg); return; }
    devlink_nl_notify_send(devlink, msg);
}

pub unsafe fn devlink_linecards_notify_register(devlink: *mut devlink) { list_for_each_entry!(let linecard, &mut (*devlink).linecard_list, list, { devlink_linecard_notify(linecard, DEVLINK_CMD_LINECARD_NEW); }); }
pub unsafe fn devlink_linecards_notify_unregister(devlink: *mut devlink) { list_for_each_entry_reverse!(let linecard, &mut (*devlink).linecard_list, list, { devlink_linecard_notify(linecard, DEVLINK_CMD_LINECARD_DEL); }); }

pub unsafe fn devlink_nl_linecard_get_doit(_skb: *mut sk_buff, info: *mut genl_info) -> c_int {
    let devlink = (*devlink_nl_ctx(info)).devlink;
    let linecard = devlink_linecard_get_from_info(devlink, info); if IS_ERR(linecard) { return PTR_ERR(linecard); }
    let msg = nlmsg_new(NLMSG_DEFAULT_SIZE, GFP_KERNEL); if msg.is_null() { return -ENOMEM; }
    mutex_lock(&mut (*linecard).state_lock);
    let err = devlink_nl_linecard_fill(msg, devlink, linecard, DEVLINK_CMD_LINECARD_NEW, (*info).snd_portid, (*info).snd_seq, 0, (*info).extack);
    mutex_unlock(&mut (*linecard).state_lock);
    if err != 0 { nlmsg_free(msg); return err; } genlmsg_reply(msg, info)
}

unsafe fn devlink_linecard_type_lookup(linecard: *mut devlink_linecard, type_: *const c_char) -> *mut devlink_linecard_type {
    for i in 0..(*linecard).types_count { let t = (*linecard).types.add(i as usize); if strcmp(type_, (*t).type_) == 0 { return t; } }
    core::ptr::null_mut()
}

unsafe fn devlink_linecard_type_set(linecard: *mut devlink_linecard, type_: *const c_char, extack: *mut netlink_ext_ack) -> c_int {
    let ops = (*linecard).ops; mutex_lock(&mut (*linecard).state_lock);
    if (*linecard).state == DEVLINK_LINECARD_STATE_PROVISIONING || (*linecard).state == DEVLINK_LINECARD_STATE_UNPROVISIONING { NL_SET_ERR_MSG(extack, c"Line card is currently busy"); mutex_unlock(&mut (*linecard).state_lock); return -EBUSY; }
    let t = devlink_linecard_type_lookup(linecard, type_); if t.is_null() { NL_SET_ERR_MSG(extack, c"Unsupported line card type provided"); mutex_unlock(&mut (*linecard).state_lock); return -EINVAL; }
    if (*linecard).state != DEVLINK_LINECARD_STATE_UNPROVISIONED && (*linecard).state != DEVLINK_LINECARD_STATE_PROVISIONING_FAILED { let mut err = -EBUSY; NL_SET_ERR_MSG(extack, c"Line card already provisioned"); if !(*ops).same_provision.is_none() && (*ops).same_provision.unwrap()(linecard, (*linecard).priv_, (*t).type_, (*t).priv_) { err = 0; } mutex_unlock(&mut (*linecard).state_lock); return err; }
    (*linecard).state = DEVLINK_LINECARD_STATE_PROVISIONING; (*linecard).type_ = (*t).type_; devlink_linecard_notify(linecard, DEVLINK_CMD_LINECARD_NEW); mutex_unlock(&mut (*linecard).state_lock);
    let err = ((*ops).provision)(linecard, (*linecard).priv_, (*t).type_, (*t).priv_, extack);
    if err != 0 { mutex_lock(&mut (*linecard).state_lock); (*linecard).state = DEVLINK_LINECARD_STATE_UNPROVISIONED; (*linecard).type_ = core::ptr::null(); devlink_linecard_notify(linecard, DEVLINK_CMD_LINECARD_NEW); mutex_unlock(&mut (*linecard).state_lock); } err
}

unsafe fn devlink_linecard_type_unset(linecard: *mut devlink_linecard, extack: *mut netlink_ext_ack) -> c_int {
    mutex_lock(&mut (*linecard).state_lock);
    if (*linecard).state == DEVLINK_LINECARD_STATE_PROVISIONING || (*linecard).state == DEVLINK_LINECARD_STATE_UNPROVISIONING { NL_SET_ERR_MSG(extack, c"Line card is currently busy"); mutex_unlock(&mut (*linecard).state_lock); return -EBUSY; }
    if (*linecard).state == DEVLINK_LINECARD_STATE_PROVISIONING_FAILED { (*linecard).state = DEVLINK_LINECARD_STATE_UNPROVISIONED; (*linecard).type_ = core::ptr::null(); devlink_linecard_notify(linecard, DEVLINK_CMD_LINECARD_NEW); mutex_unlock(&mut (*linecard).state_lock); return 0; }
    if (*linecard).state == DEVLINK_LINECARD_STATE_UNPROVISIONED { NL_SET_ERR_MSG(extack, c"Line card is not provisioned"); mutex_unlock(&mut (*linecard).state_lock); return 0; }
    (*linecard).state = DEVLINK_LINECARD_STATE_UNPROVISIONING; devlink_linecard_notify(linecard, DEVLINK_CMD_LINECARD_NEW); mutex_unlock(&mut (*linecard).state_lock);
    let err = ((*(*linecard).ops).unprovision)(linecard, (*linecard).priv_, extack);
    if err != 0 { mutex_lock(&mut (*linecard).state_lock); (*linecard).state = DEVLINK_LINECARD_STATE_UNPROVISIONED; (*linecard).type_ = core::ptr::null(); devlink_linecard_notify(linecard, DEVLINK_CMD_LINECARD_NEW); mutex_unlock(&mut (*linecard).state_lock); } err
}

pub unsafe fn devlink_nl_linecard_set_doit(_skb: *mut sk_buff, info: *mut genl_info) -> c_int {
    let linecard = devlink_linecard_get_from_info((*devlink_nl_ctx(info)).devlink, info); if IS_ERR(linecard) { return PTR_ERR(linecard); }
    let attr = *(*info).attrs.add(DEVLINK_ATTR_LINECARD_TYPE as usize); if !attr.is_null() { let type_ = nla_data(attr); if *type_ != 0 { let e = devlink_linecard_type_set(linecard, type_, (*info).extack); if e != 0 { return e; } } else { let e = devlink_linecard_type_unset(linecard, (*info).extack); if e != 0 { return e; } } } 0
}

// The dump callback preserves the kernel list/dump traversal semantics.
pub unsafe fn devlink_nl_linecard_get_dumpit(skb: *mut sk_buff, cb: *mut netlink_callback) -> c_int { devlink_nl_dumpit(skb, cb, devlink_nl_linecard_get_dump_one) }
unsafe fn devlink_nl_linecard_get_dump_one(msg: *mut sk_buff, devlink: *mut devlink, cb: *mut netlink_callback, flags: c_int) -> c_int {
    let state = devlink_dump_state(cb); let mut idx = 0; let mut err = 0;
    list_for_each_entry!(let linecard, &mut (*devlink).linecard_list, list, { if idx < (*state).idx { idx += 1; } else { mutex_lock(&mut (*linecard).state_lock); err = devlink_nl_linecard_fill(msg, devlink, linecard, DEVLINK_CMD_LINECARD_NEW, NETLINK_CB!((*cb).skb).portid, (*(*cb).nlh).nlmsg_seq, flags, (*cb).extack); mutex_unlock(&mut (*linecard).state_lock); if err != 0 { (*state).idx = idx; break; } idx += 1; } }); err
}

unsafe fn devlink_linecard_types_init(linecard: *mut devlink_linecard) -> c_int {
    let count = ((*(*linecard).ops).types_count)(linecard, (*linecard).priv_); (*linecard).types = kmalloc_objs!(devlink_linecard_type, count); if (*linecard).types.is_null() { return -ENOMEM; } (*linecard).types_count = count;
    for i in 0..count { let t = (*linecard).types.add(i as usize); ((*(*linecard).ops).types_get)(linecard, (*linecard).priv_, i, &mut (*t).type_, &mut (*t).priv_); } 0
}
unsafe fn devlink_linecard_types_fini(linecard: *mut devlink_linecard) { kfree((*linecard).types as *mut c_void); }

pub unsafe fn devl_linecard_create(devlink: *mut devlink, linecard_index: c_uint, ops: *const devlink_linecard_ops, priv_: *mut c_void) -> *mut devlink_linecard {
    if ops.is_null() || (*ops).provision as usize == 0 || (*ops).unprovision as usize == 0 || (*ops).types_count as usize == 0 || (*ops).types_get as usize == 0 { return ERR_PTR(-EINVAL); }
    if devlink_linecard_index_exists(devlink, linecard_index) { return ERR_PTR(-EEXIST); }
    let l = kzalloc_obj!(devlink_linecard); if l.is_null() { return ERR_PTR(-ENOMEM); }
    (*l).devlink = devlink; (*l).index = linecard_index; (*l).ops = ops; (*l).priv_ = priv_; (*l).state = DEVLINK_LINECARD_STATE_UNPROVISIONED; mutex_init(&mut (*l).state_lock);
    let e = devlink_linecard_types_init(l); if e != 0 { mutex_destroy(&mut (*l).state_lock); kfree(l as *mut c_void); return ERR_PTR(e); }
    list_add_tail(&mut (*l).list, &mut (*devlink).linecard_list); devlink_linecard_notify(l, DEVLINK_CMD_LINECARD_NEW); l
}
pub unsafe fn devl_linecard_destroy(linecard: *mut devlink_linecard) { devlink_linecard_notify(linecard, DEVLINK_CMD_LINECARD_DEL); list_del(&mut (*linecard).list); devlink_linecard_types_fini(linecard); mutex_destroy(&mut (*linecard).state_lock); kfree(linecard as *mut c_void); }

pub unsafe fn devlink_linecard_provision_set(linecard: *mut devlink_linecard, type_: *const c_char) { mutex_lock(&mut (*linecard).state_lock); WARN_ON!(!(*linecard).type_.is_null() && strcmp((*linecard).type_, type_) != 0); (*linecard).state = DEVLINK_LINECARD_STATE_PROVISIONED; (*linecard).type_ = type_; devlink_linecard_notify(linecard, DEVLINK_CMD_LINECARD_NEW); mutex_unlock(&mut (*linecard).state_lock); }
pub unsafe fn devlink_linecard_provision_clear(linecard: *mut devlink_linecard) { mutex_lock(&mut (*linecard).state_lock); (*linecard).state = DEVLINK_LINECARD_STATE_UNPROVISIONED; (*linecard).type_ = core::ptr::null(); devlink_linecard_notify(linecard, DEVLINK_CMD_LINECARD_NEW); mutex_unlock(&mut (*linecard).state_lock); }
pub unsafe fn devlink_linecard_provision_fail(linecard: *mut devlink_linecard) { mutex_lock(&mut (*linecard).state_lock); (*linecard).state = DEVLINK_LINECARD_STATE_PROVISIONING_FAILED; devlink_linecard_notify(linecard, DEVLINK_CMD_LINECARD_NEW); mutex_unlock(&mut (*linecard).state_lock); }
pub unsafe fn devlink_linecard_activate(linecard: *mut devlink_linecard) { mutex_lock(&mut (*linecard).state_lock); WARN_ON!((*linecard).state != DEVLINK_LINECARD_STATE_PROVISIONED); (*linecard).state = DEVLINK_LINECARD_STATE_ACTIVE; devlink_linecard_notify(linecard, DEVLINK_CMD_LINECARD_NEW); mutex_unlock(&mut (*linecard).state_lock); }
pub unsafe fn devlink_linecard_deactivate(linecard: *mut devlink_linecard) { mutex_lock(&mut (*linecard).state_lock); match (*linecard).state { DEVLINK_LINECARD_STATE_ACTIVE => { (*linecard).state = DEVLINK_LINECARD_STATE_PROVISIONED; devlink_linecard_notify(linecard, DEVLINK_CMD_LINECARD_NEW); }, DEVLINK_LINECARD_STATE_UNPROVISIONING => {}, _ => { WARN_ON!(true); } } mutex_unlock(&mut (*linecard).state_lock); }

unsafe fn devlink_linecard_rel_notify_cb(devlink: *mut devlink, linecard_index: u32) { let l = devlink_linecard_get_by_index(devlink, linecard_index); if !l.is_null() { devlink_linecard_notify(l, DEVLINK_CMD_LINECARD_NEW); } }
unsafe fn devlink_linecard_rel_cleanup_cb(devlink: *mut devlink, linecard_index: u32, rel_index: u32) { let l = devlink_linecard_get_by_index(devlink, linecard_index); if !l.is_null() && (*l).rel_index == rel_index { (*l).rel_index = 0; } }
pub unsafe fn devlink_linecard_nested_dl_set(linecard: *mut devlink_linecard, nested_devlink: *mut devlink) -> c_int { devlink_rel_nested_in_add(&mut (*linecard).rel_index, (*(*linecard).devlink).index, (*linecard).index, Some(devlink_linecard_rel_notify_cb), Some(devlink_linecard_rel_cleanup_cb), nested_devlink) }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
