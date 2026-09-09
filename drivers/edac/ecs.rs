// SPDX-License-Identifier: GPL-2.0
/*
 * The generic ECS driver is designed to support control of on-die error
 * check scrub (e.g., DDR5 ECS). The common sysfs ECS interface abstracts the
 * control of various ECS functionalities into a unified set of functions.
 *
 * Copyright (c) 2024-2025 HiSilicon Limited.
 */

// Dependency declarations supplied by the Linux EDAC implementation are
// intentionally omitted; the names below are provided by the surrounding
// kernel translation.

pub const EDAC_ECS_FRU_NAME: &str = "ecs_fru";

#[repr(C)]
pub enum edac_ecs_attributes {
    ECS_LOG_ENTRY_TYPE,
    ECS_MODE,
    ECS_RESET,
    ECS_THRESHOLD,
    ECS_MAX_ATTRS,
}

#[repr(C)]
pub struct edac_ecs_dev_attr {
    pub dev_attr: device_attribute,
    pub fru_id: i32,
}

#[repr(C)]
pub struct edac_ecs_fru_context {
    pub name: [core::ffi::c_char; EDAC_FEAT_NAME_LEN],
    pub dev_attr: [edac_ecs_dev_attr; ECS_MAX_ATTRS],
    pub ecs_attrs: [*mut attribute; ECS_MAX_ATTRS + 1],
    pub group: attribute_group,
}

#[repr(C)]
pub struct edac_ecs_context {
    pub num_media_frus: u16,
    pub fru_ctxs: *mut edac_ecs_fru_context,
}

#[inline]
unsafe fn to_ecs_dev_attr(dev_attr: *mut device_attribute) -> *mut edac_ecs_dev_attr {
    container_of!(dev_attr, edac_ecs_dev_attr, dev_attr)
}

unsafe extern "C" fn log_entry_type_show(
    ras_feat_dev: *mut device,
    attr: *mut device_attribute,
    buf: *mut core::ffi::c_char,
) -> ssize_t {
    let dev_attr = &*to_ecs_dev_attr(attr);
    let ctx = &*(dev_get_drvdata(ras_feat_dev) as *mut edac_dev_feat_ctx);
    let ops = &*ctx.ecs.ecs_ops;
    let mut data: u32 = 0;
    let ret = ((*ops).get_log_entry_type)(
        (*ras_feat_dev).parent, ctx.ecs.private, dev_attr.fru_id, &mut data,
    );
    if ret != 0 { return ret as ssize_t; }
    sysfs_emit!(buf, "%u\n", data)
}

unsafe extern "C" fn mode_show(
    ras_feat_dev: *mut device,
    attr: *mut device_attribute,
    buf: *mut core::ffi::c_char,
) -> ssize_t {
    let dev_attr = &*to_ecs_dev_attr(attr);
    let ctx = &*(dev_get_drvdata(ras_feat_dev) as *mut edac_dev_feat_ctx);
    let ops = &*ctx.ecs.ecs_ops;
    let mut data: u32 = 0;
    let ret = ((*ops).get_mode)(
        (*ras_feat_dev).parent, ctx.ecs.private, dev_attr.fru_id, &mut data,
    );
    if ret != 0 { return ret as ssize_t; }
    sysfs_emit!(buf, "%u\n", data)
}

unsafe extern "C" fn threshold_show(
    ras_feat_dev: *mut device,
    attr: *mut device_attribute,
    buf: *mut core::ffi::c_char,
) -> ssize_t {
    let dev_attr = &*to_ecs_dev_attr(attr);
    let ctx = &*(dev_get_drvdata(ras_feat_dev) as *mut edac_dev_feat_ctx);
    let ops = &*ctx.ecs.ecs_ops;
    let mut data: u32 = 0;
    let ret = ((*ops).get_threshold)(
        (*ras_feat_dev).parent, ctx.ecs.private, dev_attr.fru_id, &mut data,
    );
    if ret != 0 { return ret as ssize_t; }
    sysfs_emit!(buf, "%u\n", data)
}

unsafe fn ecs_attr_store(
    ras_feat_dev: *mut device, attr: *mut device_attribute,
    buf: *const core::ffi::c_char, len: usize,
    convert: unsafe extern "C" fn(*const core::ffi::c_char, u32, *mut c_ulong) -> i32,
    set: unsafe extern "C" fn(*mut device, *mut core::ffi::c_void, i32, c_ulong) -> i32,
) -> ssize_t {
    let dev_attr = &*to_ecs_dev_attr(attr);
    let ctx = &*(dev_get_drvdata(ras_feat_dev) as *mut edac_dev_feat_ctx);
    let mut data: c_ulong = 0;
    let ret = convert(buf, 0, &mut data);
    if ret < 0 { return ret as ssize_t; }
    let ret = set((*ras_feat_dev).parent, ctx.ecs.private, dev_attr.fru_id, data);
    if ret != 0 { return ret as ssize_t; }
    len as ssize_t
}

unsafe extern "C" fn log_entry_type_store(d: *mut device, a: *mut device_attribute, b: *const c_char, l: usize) -> ssize_t { ecs_attr_store(d, a, b, l, kstrtoul, (*(*((dev_get_drvdata(d) as *mut edac_dev_feat_ctx)).ecs.ecs_ops)).set_log_entry_type) }
unsafe extern "C" fn mode_store(d: *mut device, a: *mut device_attribute, b: *const c_char, l: usize) -> ssize_t { ecs_attr_store(d, a, b, l, kstrtoul, (*(*((dev_get_drvdata(d) as *mut edac_dev_feat_ctx)).ecs.ecs_ops)).set_mode) }
unsafe extern "C" fn reset_store(d: *mut device, a: *mut device_attribute, b: *const c_char, l: usize) -> ssize_t { ecs_attr_store(d, a, b, l, kstrtoul, (*(*((dev_get_drvdata(d) as *mut edac_dev_feat_ctx)).ecs.ecs_ops)).reset) }
unsafe extern "C" fn threshold_store(d: *mut device, a: *mut device_attribute, b: *const c_char, l: usize) -> ssize_t { ecs_attr_store(d, a, b, l, kstrtoul, (*(*((dev_get_drvdata(d) as *mut edac_dev_feat_ctx)).ecs.ecs_ops)).set_threshold) }

unsafe extern "C" fn ecs_attr_visible(kobj: *mut kobject, a: *mut attribute, attr_id: i32) -> umode_t {
    let ras_feat_dev = kobj_to_dev(kobj);
    let ctx = dev_get_drvdata(ras_feat_dev) as *mut edac_dev_feat_ctx;
    let ops = (*ctx).ecs.ecs_ops;
    match attr_id {
        ECS_LOG_ENTRY_TYPE if !(*ops).get_log_entry_type.is_none() => if !(*ops).set_log_entry_type.is_none() { (*a).mode } else { 0o444 },
        ECS_MODE if !(*ops).get_mode.is_none() => if !(*ops).set_mode.is_none() { (*a).mode } else { 0o444 },
        ECS_RESET if !(*ops).reset.is_none() => (*a).mode,
        ECS_THRESHOLD if !(*ops).get_threshold.is_none() => if !(*ops).set_threshold.is_none() { (*a).mode } else { 0o444 },
        _ => 0,
    }
}

unsafe fn ecs_create_desc(ecs_dev: *mut device, attr_groups: *mut *const attribute_group, num_media_frus: u16) -> i32 {
    let ecs_ctx = devm_kzalloc(ecs_dev, core::mem::size_of::<edac_ecs_context>(), GFP_KERNEL) as *mut edac_ecs_context;
    if ecs_ctx.is_null() { return -ENOMEM; }
    (*ecs_ctx).num_media_frus = num_media_frus;
    (*ecs_ctx).fru_ctxs = devm_kcalloc(ecs_dev, num_media_frus as usize, core::mem::size_of::<edac_ecs_fru_context>(), GFP_KERNEL) as *mut edac_ecs_fru_context;
    if (*ecs_ctx).fru_ctxs.is_null() { return -ENOMEM; }
    for fru in 0..num_media_frus as usize {
        let fru_ctx = &mut *(*ecs_ctx).fru_ctxs.add(fru);
        let group = &mut fru_ctx.group;
        fru_ctx.dev_attr[ECS_LOG_ENTRY_TYPE] = EDAC_ECS_ATTR_RW!(log_entry_type, fru);
        fru_ctx.dev_attr[ECS_MODE] = EDAC_ECS_ATTR_RW!(mode, fru);
        fru_ctx.dev_attr[ECS_RESET] = EDAC_ECS_ATTR_WO!(reset, fru);
        fru_ctx.dev_attr[ECS_THRESHOLD] = EDAC_ECS_ATTR_RW!(threshold, fru);
        for i in 0..ECS_MAX_ATTRS { sysfs_attr_init!(&mut fru_ctx.dev_attr[i].dev_attr.attr); fru_ctx.ecs_attrs[i] = &mut fru_ctx.dev_attr[i].dev_attr.attr; }
        sprintf!(&mut fru_ctx.name, "{}{}", EDAC_ECS_FRU_NAME, fru);
        group.name = fru_ctx.name.as_mut_ptr(); group.attrs = fru_ctx.ecs_attrs.as_mut_ptr(); group.is_visible = Some(ecs_attr_visible);
        *attr_groups.add(fru) = group;
    }
    0
}

pub unsafe extern "C" fn edac_ecs_get_desc(ecs_dev: *mut device, attr_groups: *mut *const attribute_group, num_media_frus: u16) -> i32 {
    if ecs_dev.is_null() || attr_groups.is_null() || num_media_frus == 0 { return -EINVAL; }
    ecs_create_desc(ecs_dev, attr_groups, num_media_frus)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
