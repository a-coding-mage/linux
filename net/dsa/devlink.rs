// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * DSA devlink handling
 */

// External dependencies supplied by the surrounding kernel translation.

unsafe fn dsa_devlink_info_get(
    dl: *mut devlink,
    req: *mut devlink_info_req,
    extack: *mut netlink_ext_ack,
) -> i32 {
    let ds = dsa_devlink_to_ds(dl);

    if (*(*ds).ops).devlink_info_get.is_some() {
        return ((*(*ds).ops).devlink_info_get.unwrap())(ds, req, extack);
    }

    -EOPNOTSUPP
}

unsafe fn dsa_devlink_sb_pool_get(
    dl: *mut devlink,
    sb_index: u32,
    pool_index: u16,
    pool_info: *mut devlink_sb_pool_info,
) -> i32 {
    let ds = dsa_devlink_to_ds(dl);

    if (*(*ds).ops).devlink_sb_pool_get.is_none() {
        return -EOPNOTSUPP;
    }

    ((*(*ds).ops).devlink_sb_pool_get.unwrap())(ds, sb_index, pool_index, pool_info)
}

unsafe fn dsa_devlink_sb_pool_set(
    dl: *mut devlink,
    sb_index: u32,
    pool_index: u16,
    size: u32,
    threshold_type: devlink_sb_threshold_type,
    extack: *mut netlink_ext_ack,
) -> i32 {
    let ds = dsa_devlink_to_ds(dl);

    if (*(*ds).ops).devlink_sb_pool_set.is_none() {
        return -EOPNOTSUPP;
    }

    ((*(*ds).ops).devlink_sb_pool_set.unwrap())(
        ds, sb_index, pool_index, size, threshold_type, extack,
    )
}

unsafe fn dsa_devlink_sb_port_pool_get(
    dlp: *mut devlink_port,
    sb_index: u32,
    pool_index: u16,
    p_threshold: *mut u32,
) -> i32 {
    let ds = dsa_devlink_port_to_ds(dlp);
    let port = dsa_devlink_port_to_port(dlp);

    if (*(*ds).ops).devlink_sb_port_pool_get.is_none() {
        return -EOPNOTSUPP;
    }

    ((*(*ds).ops).devlink_sb_port_pool_get.unwrap())(
        ds, port, sb_index, pool_index, p_threshold,
    )
}

unsafe fn dsa_devlink_sb_port_pool_set(
    dlp: *mut devlink_port,
    sb_index: u32,
    pool_index: u16,
    threshold: u32,
    extack: *mut netlink_ext_ack,
) -> i32 {
    let ds = dsa_devlink_port_to_ds(dlp);
    let port = dsa_devlink_port_to_port(dlp);

    if (*(*ds).ops).devlink_sb_port_pool_set.is_none() {
        return -EOPNOTSUPP;
    }

    ((*(*ds).ops).devlink_sb_port_pool_set.unwrap())(
        ds, port, sb_index, pool_index, threshold, extack,
    )
}

unsafe fn dsa_devlink_sb_tc_pool_bind_get(
    dlp: *mut devlink_port,
    sb_index: u32,
    tc_index: u16,
    pool_type: devlink_sb_pool_type,
    p_pool_index: *mut u16,
    p_threshold: *mut u32,
) -> i32 {
    let ds = dsa_devlink_port_to_ds(dlp);
    let port = dsa_devlink_port_to_port(dlp);

    if (*(*ds).ops).devlink_sb_tc_pool_bind_get.is_none() {
        return -EOPNOTSUPP;
    }

    ((*(*ds).ops).devlink_sb_tc_pool_bind_get.unwrap())(
        ds, port, sb_index, tc_index, pool_type, p_pool_index, p_threshold,
    )
}

unsafe fn dsa_devlink_sb_tc_pool_bind_set(
    dlp: *mut devlink_port,
    sb_index: u32,
    tc_index: u16,
    pool_type: devlink_sb_pool_type,
    pool_index: u16,
    threshold: u32,
    extack: *mut netlink_ext_ack,
) -> i32 {
    let ds = dsa_devlink_port_to_ds(dlp);
    let port = dsa_devlink_port_to_port(dlp);

    if (*(*ds).ops).devlink_sb_tc_pool_bind_set.is_none() {
        return -EOPNOTSUPP;
    }

    ((*(*ds).ops).devlink_sb_tc_pool_bind_set.unwrap())(
        ds, port, sb_index, tc_index, pool_type, pool_index, threshold, extack,
    )
}

unsafe fn dsa_devlink_sb_occ_snapshot(dl: *mut devlink, sb_index: u32) -> i32 {
    let ds = dsa_devlink_to_ds(dl);

    if (*(*ds).ops).devlink_sb_occ_snapshot.is_none() {
        return -EOPNOTSUPP;
    }

    ((*(*ds).ops).devlink_sb_occ_snapshot.unwrap())(ds, sb_index)
}

unsafe fn dsa_devlink_sb_occ_max_clear(dl: *mut devlink, sb_index: u32) -> i32 {
    let ds = dsa_devlink_to_ds(dl);

    if (*(*ds).ops).devlink_sb_occ_max_clear.is_none() {
        return -EOPNOTSUPP;
    }

    ((*(*ds).ops).devlink_sb_occ_max_clear.unwrap())(ds, sb_index)
}

unsafe fn dsa_devlink_sb_occ_port_pool_get(
    dlp: *mut devlink_port,
    sb_index: u32,
    pool_index: u16,
    p_cur: *mut u32,
    p_max: *mut u32,
) -> i32 {
    let ds = dsa_devlink_port_to_ds(dlp);
    let port = dsa_devlink_port_to_port(dlp);

    if (*(*ds).ops).devlink_sb_occ_port_pool_get.is_none() {
        return -EOPNOTSUPP;
    }

    ((*(*ds).ops).devlink_sb_occ_port_pool_get.unwrap())(
        ds, port, sb_index, pool_index, p_cur, p_max,
    )
}

unsafe fn dsa_devlink_sb_occ_tc_port_bind_get(
    dlp: *mut devlink_port,
    sb_index: u32,
    tc_index: u16,
    pool_type: devlink_sb_pool_type,
    p_cur: *mut u32,
    p_max: *mut u32,
) -> i32 {
    let ds = dsa_devlink_port_to_ds(dlp);
    let port = dsa_devlink_port_to_port(dlp);

    if (*(*ds).ops).devlink_sb_occ_tc_port_bind_get.is_none() {
        return -EOPNOTSUPP;
    }

    ((*(*ds).ops).devlink_sb_occ_tc_port_bind_get.unwrap())(
        ds, port, sb_index, tc_index, pool_type, p_cur, p_max,
    )
}

static dsa_devlink_ops: devlink_ops = devlink_ops {
    info_get: Some(dsa_devlink_info_get),
    sb_pool_get: Some(dsa_devlink_sb_pool_get),
    sb_pool_set: Some(dsa_devlink_sb_pool_set),
    sb_port_pool_get: Some(dsa_devlink_sb_port_pool_get),
    sb_port_pool_set: Some(dsa_devlink_sb_port_pool_set),
    sb_tc_pool_bind_get: Some(dsa_devlink_sb_tc_pool_bind_get),
    sb_tc_pool_bind_set: Some(dsa_devlink_sb_tc_pool_bind_set),
    sb_occ_snapshot: Some(dsa_devlink_sb_occ_snapshot),
    sb_occ_max_clear: Some(dsa_devlink_sb_occ_max_clear),
    sb_occ_port_pool_get: Some(dsa_devlink_sb_occ_port_pool_get),
    sb_occ_tc_port_bind_get: Some(dsa_devlink_sb_occ_tc_port_bind_get),
};

pub unsafe fn dsa_devlink_param_get(
    dl: *mut devlink,
    id: u32,
    ctx: *mut devlink_param_gset_ctx,
    _extack: *mut netlink_ext_ack,
) -> i32 {
    let ds = dsa_devlink_to_ds(dl);

    if (*(*ds).ops).devlink_param_get.is_none() {
        return -EOPNOTSUPP;
    }

    ((*(*ds).ops).devlink_param_get.unwrap())(ds, id, ctx)
}

pub unsafe fn dsa_devlink_param_set(
    dl: *mut devlink,
    id: u32,
    ctx: *mut devlink_param_gset_ctx,
    _extack: *mut netlink_ext_ack,
) -> i32 {
    let ds = dsa_devlink_to_ds(dl);

    if (*(*ds).ops).devlink_param_set.is_none() {
        return -EOPNOTSUPP;
    }

    ((*(*ds).ops).devlink_param_set.unwrap())(ds, id, ctx)
}

pub unsafe fn dsa_devlink_params_register(
    ds: *mut dsa_switch,
    params: *const devlink_param,
    params_count: usize,
) -> i32 {
    devlink_params_register((*ds).devlink, params, params_count)
}

pub unsafe fn dsa_devlink_params_unregister(
    ds: *mut dsa_switch,
    params: *const devlink_param,
    params_count: usize,
) {
    devlink_params_unregister((*ds).devlink, params, params_count);
}

pub unsafe fn dsa_devlink_resource_register(
    ds: *mut dsa_switch,
    resource_name: *const i8,
    resource_size: u64,
    resource_id: u64,
    parent_resource_id: u64,
    size_params: *const devlink_resource_size_params,
) -> i32 {
    let mut ret: i32;

    devl_lock((*ds).devlink);
    ret = devl_resource_register(
        (*ds).devlink,
        resource_name,
        resource_size,
        resource_id,
        parent_resource_id,
        size_params,
    );
    devl_unlock((*ds).devlink);

    ret
}

pub unsafe fn dsa_devlink_resources_unregister(ds: *mut dsa_switch) {
    devlink_resources_unregister((*ds).devlink);
}

pub unsafe fn dsa_devlink_resource_occ_get_register(
    ds: *mut dsa_switch,
    resource_id: u64,
    occ_get: devlink_resource_occ_get_t,
    occ_get_priv: *mut core::ffi::c_void,
) {
    devl_lock((*ds).devlink);
    devl_resource_occ_get_register((*ds).devlink, resource_id, occ_get, occ_get_priv);
    devl_unlock((*ds).devlink);
}

pub unsafe fn dsa_devlink_resource_occ_get_unregister(ds: *mut dsa_switch, resource_id: u64) {
    devl_lock((*ds).devlink);
    devl_resource_occ_get_unregister((*ds).devlink, resource_id);
    devl_unlock((*ds).devlink);
}

pub unsafe fn dsa_devlink_region_create(
    ds: *mut dsa_switch,
    ops: *const devlink_region_ops,
    region_max_snapshots: u32,
    region_size: u64,
) -> *mut devlink_region {
    devlink_region_create((*ds).devlink, ops, region_max_snapshots, region_size)
}

pub unsafe fn dsa_devlink_port_region_create(
    ds: *mut dsa_switch,
    port: i32,
    ops: *const devlink_port_region_ops,
    region_max_snapshots: u32,
    region_size: u64,
) -> *mut devlink_region {
    let dp = dsa_to_port(ds, port);

    devlink_port_region_create(
        &mut (*dp).devlink_port,
        ops,
        region_max_snapshots,
        region_size,
    )
}

pub unsafe fn dsa_devlink_region_destroy(region: *mut devlink_region) {
    devlink_region_destroy(region);
}

pub unsafe fn dsa_port_devlink_setup(dp: *mut dsa_port) -> i32 {
    let dlp = &mut (*dp).devlink_port;
    let dst = (*(*dp).ds).dst;
    let mut attrs: devlink_port_attrs = core::mem::zeroed();
    let dl = (*(*dp).ds).devlink;
    let ds = (*dp).ds;
    let id: *const u8;
    let len: u8;
    let mut err: i32;

    core::ptr::write_bytes(dlp, 0, 1);
    devlink_port_init(dl, dlp);

    if (*(*ds).ops).port_setup.is_some() {
        err = ((*(*ds).ops).port_setup.unwrap())(ds, (*dp).index);
        if err != 0 {
            return err;
        }
    }

    id = &(*dst).index as *const _ as *const u8;
    len = core::mem::size_of_val(&(*dst).index) as u8;

    attrs.phys.port_number = (*dp).index;
    core::ptr::copy_nonoverlapping(id, attrs.switch_id.id.as_mut_ptr(), len as usize);
    attrs.switch_id.id_len = len;

    match (*dp).type_ {
        DSA_PORT_TYPE_UNUSED => attrs.flavour = DEVLINK_PORT_FLAVOUR_UNUSED,
        DSA_PORT_TYPE_CPU => attrs.flavour = DEVLINK_PORT_FLAVOUR_CPU,
        DSA_PORT_TYPE_DSA => attrs.flavour = DEVLINK_PORT_FLAVOUR_DSA,
        DSA_PORT_TYPE_USER => attrs.flavour = DEVLINK_PORT_FLAVOUR_PHYSICAL,
        _ => {}
    }

    devlink_port_attrs_set(dlp, &attrs);
    err = devlink_port_register(dl, dlp, (*dp).index);
    if err != 0 {
        if (*(*ds).ops).port_teardown.is_some() {
            ((*(*ds).ops).port_teardown.unwrap())(ds, (*dp).index);
        }
        return err;
    }

    0
}

pub unsafe fn dsa_port_devlink_teardown(dp: *mut dsa_port) {
    let dlp = &mut (*dp).devlink_port;
    let ds = (*dp).ds;

    devlink_port_unregister(dlp);

    if (*(*ds).ops).port_teardown.is_some() {
        ((*(*ds).ops).port_teardown.unwrap())(ds, (*dp).index);
    }

    devlink_port_fini(dlp);
}

pub unsafe fn dsa_switch_devlink_register(ds: *mut dsa_switch) {
    devlink_register((*ds).devlink);
}

pub unsafe fn dsa_switch_devlink_unregister(ds: *mut dsa_switch) {
    devlink_unregister((*ds).devlink);
}

pub unsafe fn dsa_switch_devlink_alloc(ds: *mut dsa_switch) -> i32 {
    let dl_priv: *mut dsa_devlink_priv;
    let dl: *mut devlink;

    /* Add the switch to devlink before calling setup, so that setup can
     * add dpipe tables
     */
    dl = devlink_alloc(&dsa_devlink_ops, core::mem::size_of::<dsa_devlink_priv>(), (*ds).dev);
    if dl.is_null() {
        return -ENOMEM;
    }

    (*ds).devlink = dl;

    dl_priv = devlink_priv((*ds).devlink);
    (*dl_priv).ds = ds;

    0
}

pub unsafe fn dsa_switch_devlink_free(ds: *mut dsa_switch) {
    devlink_free((*ds).devlink);
    (*ds).devlink = core::ptr::null_mut();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
