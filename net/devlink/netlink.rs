// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright (c) 2016 Mellanox Technologies. All rights reserved.
 * Copyright (c) 2016 Jiri Pirko <jiri@mellanox.com>
 */

// Kernel dependencies supplied by the surrounding translation unit.

const DEVLINK_NL_FLAG_NEED_PORT: u8 = 1 << 0;
const DEVLINK_NL_FLAG_NEED_DEVLINK_OR_PORT: u8 = 1 << 1;
const DEVLINK_NL_FLAG_NEED_DEV_LOCK: u8 = 1 << 2;
const DEVLINK_NL_FLAG_OPTIONAL_PARENT_DEV: u8 = 1 << 3;

static mut devlink_nl_mcgrps: [genl_multicast_group; 1] = [genl_multicast_group {
    name: DEVLINK_GENL_MCGRP_CONFIG_NAME,
}];

#[repr(C)]
struct devlink_nl_sock_priv {
    flt: *mut devlink_obj_desc,
    flt_lock: spinlock_t,
}

unsafe fn devlink_nl_sock_priv_init(priv_: *mut core::ffi::c_void) {
    let sk_priv = priv_ as *mut devlink_nl_sock_priv;
    spin_lock_init(&mut (*sk_priv).flt_lock);
}

unsafe fn devlink_nl_sock_priv_destroy(priv_: *mut core::ffi::c_void) {
    let sk_priv = priv_ as *mut devlink_nl_sock_priv;
    let flt = rcu_dereference_protected((*sk_priv).flt, true);
    kfree_rcu(flt, rcu);
}

unsafe fn devlink_nl_notify_filter_set_doit(
    skb: *mut sk_buff,
    info: *mut genl_info,
) -> i32 {
    let mut sk_priv: *mut devlink_nl_sock_priv;
    let attrs = (*info).attrs;
    let mut flt: *mut devlink_obj_desc;
    let mut data_offset: usize = 0;
    let mut data_size: usize = 0;
    let mut pos: *mut core::ffi::c_char;

    if !(*attrs.add(DEVLINK_ATTR_BUS_NAME as usize)).is_null() {
        data_size = size_add(data_size, nla_len(*attrs.add(DEVLINK_ATTR_BUS_NAME as usize)) + 1);
    }
    if !(*attrs.add(DEVLINK_ATTR_DEV_NAME as usize)).is_null() {
        data_size = size_add(data_size, nla_len(*attrs.add(DEVLINK_ATTR_DEV_NAME as usize)) + 1);
    }

    flt = kzalloc(size_add(core::mem::size_of::<devlink_obj_desc>(), data_size), GFP_KERNEL)
        as *mut devlink_obj_desc;
    if flt.is_null() {
        return -ENOMEM;
    }

    pos = (*flt).data.as_mut_ptr() as *mut core::ffi::c_char;
    if !(*attrs.add(DEVLINK_ATTR_BUS_NAME as usize)).is_null() {
        data_offset += nla_strscpy(pos, *attrs.add(DEVLINK_ATTR_BUS_NAME as usize), data_size) + 1;
        (*flt).bus_name = pos;
        pos = pos.add(data_offset);
    }
    if !(*attrs.add(DEVLINK_ATTR_DEV_NAME as usize)).is_null() {
        nla_strscpy(pos, *attrs.add(DEVLINK_ATTR_DEV_NAME as usize), data_size - data_offset);
        (*flt).dev_name = pos;
    }

    if !(*attrs.add(DEVLINK_ATTR_INDEX as usize)).is_null() {
        (*flt).devlink_index = nla_get_uint(*attrs.add(DEVLINK_ATTR_INDEX as usize));
        (*flt).devlink_index_valid = true;
    }
    if !(*attrs.add(DEVLINK_ATTR_PORT_INDEX as usize)).is_null() {
        (*flt).port_index = nla_get_u32(*attrs.add(DEVLINK_ATTR_PORT_INDEX as usize));
        (*flt).port_index_valid = true;
    }

    if (*flt).bus_name.is_null() && (*flt).dev_name.is_null()
        && !(*flt).devlink_index_valid && !(*flt).port_index_valid
    {
        kfree(flt as *mut core::ffi::c_void);
        flt = core::ptr::null_mut();
    }

    sk_priv = genl_sk_priv_get(&devlink_nl_family, NETLINK_CB(skb).sk);
    if IS_ERR(sk_priv) {
        kfree(flt as *mut core::ffi::c_void);
        return PTR_ERR(sk_priv);
    }
    spin_lock(&mut (*sk_priv).flt_lock);
    flt = rcu_replace_pointer((*sk_priv).flt, flt, lockdep_is_held(&(*sk_priv).flt_lock));
    spin_unlock(&mut (*sk_priv).flt_lock);
    kfree_rcu(flt, rcu);
    0
}

unsafe fn devlink_obj_desc_match(desc: *const devlink_obj_desc, flt: *const devlink_obj_desc) -> bool {
    if (*desc).devlink_index_valid && (*flt).devlink_index_valid
        && (*desc).devlink_index != (*flt).devlink_index { return false; }
    if !(*desc).bus_name.is_null() && !(*flt).bus_name.is_null()
        && strcmp((*desc).bus_name, (*flt).bus_name) != 0 { return false; }
    if !(*desc).dev_name.is_null() && !(*flt).dev_name.is_null()
        && strcmp((*desc).dev_name, (*flt).dev_name) != 0 { return false; }
    if (*desc).port_index_valid && (*flt).port_index_valid
        && (*desc).port_index != (*flt).port_index { return false; }
    true
}

unsafe fn devlink_nl_notify_filter(dsk: *mut sock, _skb: *mut sk_buff, data: *mut core::ffi::c_void) -> i32 {
    let desc = data as *const devlink_obj_desc;
    let mut ret = 0;
    rcu_read_lock();
    let sk_priv = __genl_sk_priv_get(&devlink_nl_family, dsk);
    if !IS_ERR_OR_NULL(sk_priv) {
        let flt = rcu_dereference((*sk_priv).flt);
        if !flt.is_null() { ret = (!devlink_obj_desc_match(desc, flt)) as i32; }
    }
    rcu_read_unlock();
    ret
}

unsafe fn devlink_nl_put_nested_handle(msg: *mut sk_buff, net: *mut net, devlink: *mut devlink, attrtype: i32) -> i32 {
    let nested_attr = nla_nest_start(msg, attrtype);
    if nested_attr.is_null() { return -EMSGSIZE; }
    if devlink_nl_put_handle(msg, devlink) != 0 { nla_nest_cancel(msg, nested_attr); return -EMSGSIZE; }
    rcu_read_lock();
    let devl_net = read_pnet_rcu(&(*devlink)._net);
    if !net_eq(net, devl_net) {
        let id = peernet2id_alloc(net, devl_net, GFP_ATOMIC);
        rcu_read_unlock();
        if nla_put_s32(msg, DEVLINK_ATTR_NETNS_ID, id) != 0 { return -EMSGSIZE; }
    } else { rcu_read_unlock(); }
    nla_nest_end(msg, nested_attr);
    0
}

unsafe fn devlink_nl_msg_reply_and_new(msg: *mut *mut sk_buff, info: *mut genl_info) -> i32 {
    if !(*msg).is_null() {
        let err = genlmsg_reply(*msg, info);
        if err != 0 { return err; }
    }
    *msg = genlmsg_new(GENLMSG_DEFAULT_SIZE, GFP_KERNEL);
    if (*msg).is_null() { return -ENOMEM; }
    0
}

unsafe fn devlink_get_from_attrs_lock(net: *mut net, attrs: *mut *mut nlattr, dev_lock: bool) -> *mut devlink {
    let mut index: libc::c_ulong;
    let devlink;
    if !(*attrs.add(DEVLINK_ATTR_INDEX as usize)).is_null() {
        if !(*attrs.add(DEVLINK_ATTR_BUS_NAME as usize)).is_null() || !(*attrs.add(DEVLINK_ATTR_DEV_NAME as usize)).is_null() { return ERR_PTR(-EINVAL); }
        index = nla_get_u32(*attrs.add(DEVLINK_ATTR_INDEX as usize)) as libc::c_ulong;
        devlink = devlinks_xa_lookup_get(net, index);
        if devlink.is_null() { return ERR_PTR(-ENODEV); }
    } else {
        if (*attrs.add(DEVLINK_ATTR_BUS_NAME as usize)).is_null() || (*attrs.add(DEVLINK_ATTR_DEV_NAME as usize)).is_null() { return ERR_PTR(-EINVAL); }
        let busname = nla_data(*attrs.add(DEVLINK_ATTR_BUS_NAME as usize));
        let devname = nla_data(*attrs.add(DEVLINK_ATTR_DEV_NAME as usize));
        if strcmp(busname, DEVLINK_INDEX_BUS_NAME) == 0 {
            if kstrtoul(devname, 10, &mut index) != 0 { return ERR_PTR(-ENODEV); }
            devlink = devlinks_xa_lookup_get(net, index);
            if devlink.is_null() { return ERR_PTR(-ENODEV); }
        } else {
            let mut found = core::ptr::null_mut();
            devlinks_xa_for_each_registered_get(net, index, devlink) {
                if strcmp(devlink_bus_name(devlink), busname) == 0 && strcmp(devlink_dev_name(devlink), devname) == 0 { found = devlink; break; }
                devlink_put(devlink);
            }
            if found.is_null() { return ERR_PTR(-ENODEV); }
            devlink = found;
        }
    }
    devl_dev_lock(devlink, dev_lock);
    if devl_is_registered(devlink) { return devlink; }
    devl_dev_unlock(devlink, dev_lock);
    devlink_put(devlink);
    ERR_PTR(-ENODEV)
}

unsafe fn devlink_get_parent_from_attrs_lock(net: *mut net, attrs: *mut *mut nlattr) -> *mut devlink {
    let maxtype = core::mem::size_of_val(&devlink_dl_parent_dev_nl_policy) - 1;
    if (*attrs.add(DEVLINK_ATTR_PARENT_DEV as usize)).is_null() { return ERR_PTR(-EINVAL); }
    let tb = kcalloc(maxtype + 1, core::mem::size_of::<*mut nlattr>(), GFP_KERNEL) as *mut *mut nlattr;
    if tb.is_null() { return ERR_PTR(-ENOMEM); }
    let err = nla_parse_nested(tb, maxtype, *attrs.add(DEVLINK_ATTR_PARENT_DEV as usize), devlink_dl_parent_dev_nl_policy, core::ptr::null_mut());
    if err != 0 { kfree(tb as *mut core::ffi::c_void); return ERR_PTR(err); }
    let devlink = devlink_get_from_attrs_lock(net, tb, false);
    kfree(tb as *mut core::ffi::c_void);
    devlink
}

unsafe fn __devlink_nl_pre_doit(skb: *mut sk_buff, info: *mut genl_info, flags: u8) -> i32 {
    let parent_dev = flags & DEVLINK_NL_FLAG_OPTIONAL_PARENT_DEV != 0;
    let dev_lock = flags & DEVLINK_NL_FLAG_NEED_DEV_LOCK != 0;
    let net = genl_info_net(info);
    let attrs = (*info).attrs;
    let mut parent_devlink = core::ptr::null_mut();
    if parent_dev && !(*attrs.add(DEVLINK_ATTR_PARENT_DEV as usize)).is_null() {
        parent_devlink = devlink_get_parent_from_attrs_lock(net, attrs);
        if IS_ERR(parent_devlink) { return PTR_ERR(parent_devlink); }
        devlink_nl_ctx(info).parent_devlink = parent_devlink;
        devl_unlock(parent_devlink);
    }
    let devlink = devlink_get_from_attrs_lock(net, attrs, dev_lock);
    if IS_ERR(devlink) { if parent_dev && !parent_devlink.is_null() { devlink_put(parent_devlink); } return PTR_ERR(devlink); }
    devlink_nl_ctx(info).devlink = devlink;
    if flags & DEVLINK_NL_FLAG_NEED_PORT != 0 {
        let port = devlink_port_get_from_info(devlink, info);
        if IS_ERR(port) { devl_dev_unlock(devlink, dev_lock); devlink_put(devlink); if parent_dev && !parent_devlink.is_null() { devlink_put(parent_devlink); } return PTR_ERR(port); }
        devlink_nl_ctx(info).devlink_port = port;
    } else if flags & DEVLINK_NL_FLAG_NEED_DEVLINK_OR_PORT != 0 {
        let port = devlink_port_get_from_info(devlink, info);
        if !IS_ERR(port) { devlink_nl_ctx(info).devlink_port = port; }
    }
    0
}

unsafe fn devlink_nl_pre_doit(_ops: *const genl_split_ops, skb: *mut sk_buff, info: *mut genl_info) -> i32 { __devlink_nl_pre_doit(skb, info, 0) }
unsafe fn devlink_nl_pre_doit_port(_ops: *const genl_split_ops, skb: *mut sk_buff, info: *mut genl_info) -> i32 { __devlink_nl_pre_doit(skb, info, DEVLINK_NL_FLAG_NEED_PORT) }
unsafe fn devlink_nl_pre_doit_dev_lock(_ops: *const genl_split_ops, skb: *mut sk_buff, info: *mut genl_info) -> i32 { __devlink_nl_pre_doit(skb, info, DEVLINK_NL_FLAG_NEED_DEV_LOCK) }
unsafe fn devlink_nl_pre_doit_port_optional(_ops: *const genl_split_ops, skb: *mut sk_buff, info: *mut genl_info) -> i32 { __devlink_nl_pre_doit(skb, info, DEVLINK_NL_FLAG_NEED_DEVLINK_OR_PORT) }
unsafe fn devlink_nl_pre_doit_parent_dev_optional(_ops: *const genl_split_ops, skb: *mut sk_buff, info: *mut genl_info) -> i32 { __devlink_nl_pre_doit(skb, info, DEVLINK_NL_FLAG_OPTIONAL_PARENT_DEV) }

unsafe fn __devlink_nl_post_doit(_skb: *mut sk_buff, info: *mut genl_info, flags: u8) {
    let devlink = devlink_nl_ctx(info).devlink;
    devl_dev_unlock(devlink, flags & DEVLINK_NL_FLAG_NEED_DEV_LOCK != 0);
    devlink_put(devlink);
    if !devlink_nl_ctx(info).parent_devlink.is_null() { devlink_put(devlink_nl_ctx(info).parent_devlink); }
}
unsafe fn devlink_nl_post_doit(_ops: *const genl_split_ops, skb: *mut sk_buff, info: *mut genl_info) { __devlink_nl_post_doit(skb, info, 0); }
unsafe fn devlink_nl_post_doit_dev_lock(_ops: *const genl_split_ops, skb: *mut sk_buff, info: *mut genl_info) { __devlink_nl_post_doit(skb, info, DEVLINK_NL_FLAG_NEED_DEV_LOCK); }
unsafe fn devlink_nl_post_doit_parent_dev_optional(_ops: *const genl_split_ops, skb: *mut sk_buff, info: *mut genl_info) { __devlink_nl_post_doit(skb, info, DEVLINK_NL_FLAG_OPTIONAL_PARENT_DEV); }

unsafe fn devlink_nl_inst_single_dumpit(msg: *mut sk_buff, cb: *mut netlink_callback, flags: i32, dump_one: devlink_nl_dump_one_func_t, attrs: *mut *mut nlattr) -> i32 {
    let devlink = devlink_get_from_attrs_lock(sock_net((*msg).sk), attrs, false);
    if IS_ERR(devlink) { return PTR_ERR(devlink); }
    let err = dump_one(msg, devlink, cb, flags | NLM_F_DUMP_FILTERED);
    devl_unlock(devlink); devlink_put(devlink);
    if err != -EMSGSIZE { err } else { (*msg).len as i32 }
}

unsafe fn devlink_nl_inst_iter_dumpit(msg: *mut sk_buff, cb: *mut netlink_callback, flags: i32, dump_one: devlink_nl_dump_one_func_t) -> i32 {
    let state = devlink_dump_state(cb);
    let mut err = 0;
    loop {
        let devlink = devlinks_xa_find_get(sock_net((*msg).sk), &mut (*state).instance);
        if devlink.is_null() { break; }
        devl_lock(devlink);
        err = if devl_is_registered(devlink) { dump_one(msg, devlink, cb, flags) } else { 0 };
        devl_unlock(devlink); devlink_put(devlink);
        if err != 0 { break; }
        (*state).instance += 1;
        (*state).idx = 0;
        (*state).port_ctx.index = 0;
        (*state).port_ctx.index_valid = false;
    }
    if err != -EMSGSIZE { err } else { (*msg).len as i32 }
}

unsafe fn devlink_nl_dumpit(msg: *mut sk_buff, cb: *mut netlink_callback, dump_one: devlink_nl_dump_one_func_t) -> i32 {
    let info = genl_info_dump(cb);
    let attrs = (*info).attrs;
    let flags = NLM_F_MULTI;
    if !attrs.is_null() && (!(*attrs.add(DEVLINK_ATTR_BUS_NAME as usize)).is_null() || !(*attrs.add(DEVLINK_ATTR_DEV_NAME as usize)).is_null() || !(*attrs.add(DEVLINK_ATTR_INDEX as usize)).is_null()) {
        devlink_nl_inst_single_dumpit(msg, cb, flags, dump_one, attrs)
    } else { devlink_nl_inst_iter_dumpit(msg, cb, flags, dump_one) }
}

#[no_mangle]
static mut devlink_nl_family: genl_family = genl_family {
    name: DEVLINK_GENL_NAME,
    version: DEVLINK_GENL_VERSION,
    netnsok: true,
    parallel_ops: true,
    module: THIS_MODULE,
    split_ops: devlink_nl_ops,
    n_split_ops: core::mem::size_of_val(&devlink_nl_ops),
    resv_start_op: DEVLINK_CMD_SELFTESTS_RUN + 1,
    mcgrps: devlink_nl_mcgrps.as_ptr(),
    n_mcgrps: core::mem::size_of_val(&devlink_nl_mcgrps),
    sock_priv_size: core::mem::size_of::<devlink_nl_sock_priv>(),
    sock_priv_init: Some(devlink_nl_sock_priv_init),
    sock_priv_destroy: Some(devlink_nl_sock_priv_destroy),
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
