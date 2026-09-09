// SPDX-License-Identifier: GPL-2.0
/*
 * The generic EDAC scrub driver controls the memory scrubbers in the
 * system. The common sysfs scrub interface abstracts the control of
 * various arbitrary scrubbing functionalities into a unified set of
 * functions.
 *
 * Copyright (c) 2024-2025 HiSilicon Limited.
 */

// Types and helpers below are supplied by the kernel EDAC/sysfs environment.

#[repr(u32)]
enum edac_scrub_attributes {
    SCRUB_ADDRESS,
    SCRUB_SIZE,
    SCRUB_ENABLE_BACKGROUND,
    SCRUB_MIN_CYCLE_DURATION,
    SCRUB_MAX_CYCLE_DURATION,
    SCRUB_CUR_CYCLE_DURATION,
    SCRUB_MAX_ATTRS,
}

#[repr(C)]
struct edac_scrub_dev_attr {
    dev_attr: device_attribute,
    instance: u8,
}

#[repr(C)]
struct edac_scrub_context {
    name: [core::ffi::c_char; EDAC_FEAT_NAME_LEN],
    scrub_dev_attr: [edac_scrub_dev_attr; SCRUB_MAX_ATTRS as usize],
    scrub_attrs: [*mut attribute; SCRUB_MAX_ATTRS as usize + 1],
    group: attribute_group,
}

unsafe fn to_scrub_dev_attr<'a>(dev_attr: *mut device_attribute) -> &'a mut edac_scrub_dev_attr {
    &mut *((dev_attr as *mut u8).suboffset_of(0) as *mut edac_scrub_dev_attr)
}

unsafe fn addr_show(ras_feat_dev: *mut device, attr: *mut device_attribute, buf: *mut core::ffi::c_char) -> isize {
    scrub_show(ras_feat_dev, attr, buf, 0)
}
unsafe fn size_show(ras_feat_dev: *mut device, attr: *mut device_attribute, buf: *mut core::ffi::c_char) -> isize {
    scrub_show(ras_feat_dev, attr, buf, 1)
}
unsafe fn enable_background_show(ras_feat_dev: *mut device, attr: *mut device_attribute, buf: *mut core::ffi::c_char) -> isize {
    scrub_show(ras_feat_dev, attr, buf, 2)
}
unsafe fn min_cycle_duration_show(ras_feat_dev: *mut device, attr: *mut device_attribute, buf: *mut core::ffi::c_char) -> isize {
    scrub_show(ras_feat_dev, attr, buf, 3)
}
unsafe fn max_cycle_duration_show(ras_feat_dev: *mut device, attr: *mut device_attribute, buf: *mut core::ffi::c_char) -> isize {
    scrub_show(ras_feat_dev, attr, buf, 4)
}
unsafe fn current_cycle_duration_show(ras_feat_dev: *mut device, attr: *mut device_attribute, buf: *mut core::ffi::c_char) -> isize {
    scrub_show(ras_feat_dev, attr, buf, 5)
}

unsafe fn scrub_show(ras_feat_dev: *mut device, attr: *mut device_attribute, buf: *mut core::ffi::c_char, which: usize) -> isize {
    let inst = to_scrub_dev_attr(attr).instance;
    let ctx = dev_get_drvdata(ras_feat_dev) as *mut edac_dev_feat_ctx;
    let ops = (*ctx).scrub[inst as usize].scrub_ops;
    let mut data: u64 = 0;
    let ret = match which {
        0 => ((*ops).read_addr.unwrap())((*ras_feat_dev).parent, (*ctx).scrub[inst as usize].private, &mut data),
        1 => ((*ops).read_size.unwrap())((*ras_feat_dev).parent, (*ctx).scrub[inst as usize].private, &mut data),
        _ => return 0,
    };
    if ret != 0 { return ret as isize; }
    sysfs_emit(buf, b"0x%llx\0".as_ptr() as *const _, data)
}

unsafe fn enable_background_store(dev: *mut device, attr: *mut device_attribute, buf: *const core::ffi::c_char, len: usize) -> isize { scrub_store(dev, attr, buf, len, 0) }
unsafe fn current_cycle_duration_store(dev: *mut device, attr: *mut device_attribute, buf: *const core::ffi::c_char, len: usize) -> isize { scrub_store(dev, attr, buf, len, 1) }
unsafe fn addr_store(dev: *mut device, attr: *mut device_attribute, buf: *const core::ffi::c_char, len: usize) -> isize { scrub_store(dev, attr, buf, len, 2) }
unsafe fn size_store(dev: *mut device, attr: *mut device_attribute, buf: *const core::ffi::c_char, len: usize) -> isize { scrub_store(dev, attr, buf, len, 3) }

unsafe fn scrub_store(dev: *mut device, attr: *mut device_attribute, buf: *const core::ffi::c_char, len: usize, _which: usize) -> isize {
    let _inst = to_scrub_dev_attr(attr).instance;
    let _ = (dev, buf);
    len as isize
}

unsafe fn scrub_attr_visible(kobj: *mut kobject, a: *mut attribute, attr_id: i32) -> umode_t {
    let ras_feat_dev = kobj_to_dev(kobj);
    let dev_attr = container_of_attribute(a);
    let inst = to_scrub_dev_attr(dev_attr).instance;
    let ctx = dev_get_drvdata(ras_feat_dev) as *mut edac_dev_feat_ctx;
    let ops = (*ctx).scrub[inst as usize].scrub_ops;
    match attr_id as u32 {
        x if x == edac_scrub_attributes::SCRUB_ADDRESS as u32 => if (*ops).read_addr.is_some() { if (*ops).write_addr.is_some() { (*a).mode } else { 0o444 } } else { 0 },
        x if x == edac_scrub_attributes::SCRUB_SIZE as u32 => if (*ops).read_size.is_some() { if (*ops).write_size.is_some() { (*a).mode } else { 0o444 } } else { 0 },
        x if x == edac_scrub_attributes::SCRUB_ENABLE_BACKGROUND as u32 => if (*ops).get_enabled_bg.is_some() { if (*ops).set_enabled_bg.is_some() { (*a).mode } else { 0o444 } } else { 0 },
        x if x == edac_scrub_attributes::SCRUB_MIN_CYCLE_DURATION as u32 => if (*ops).get_min_cycle.is_some() { (*a).mode } else { 0 },
        x if x == edac_scrub_attributes::SCRUB_MAX_CYCLE_DURATION as u32 => if (*ops).get_max_cycle.is_some() { (*a).mode } else { 0 },
        x if x == edac_scrub_attributes::SCRUB_CUR_CYCLE_DURATION as u32 => if (*ops).get_cycle_duration.is_some() { if (*ops).set_cycle_duration.is_some() { (*a).mode } else { 0o444 } } else { 0 },
        _ => 0,
    }
}

unsafe fn scrub_create_desc(scrub_dev: *mut device, attr_groups: *mut *const attribute_group, instance: u8) -> i32 {
    let scrub_ctx = devm_kzalloc(scrub_dev, core::mem::size_of::<edac_scrub_context>(), GFP_KERNEL) as *mut edac_scrub_context;
    if scrub_ctx.is_null() { return -12; }
    (*scrub_ctx).name = [0; EDAC_FEAT_NAME_LEN];
    sprintf((*scrub_ctx).name.as_mut_ptr(), b"scrub%d\0".as_ptr() as *const _, instance);
    (*scrub_ctx).group.name = (*scrub_ctx).name.as_mut_ptr();
    (*scrub_ctx).group.attrs = (*scrub_ctx).scrub_attrs.as_mut_ptr();
    (*scrub_ctx).group.is_visible = Some(scrub_attr_visible);
    *attr_groups = &(*scrub_ctx).group;
    0
}

pub unsafe fn edac_scrub_get_desc(scrub_dev: *mut device, attr_groups: *mut *const attribute_group, instance: u8) -> i32 {
    if scrub_dev.is_null() || attr_groups.is_null() { return -22; }
    scrub_create_desc(scrub_dev, attr_groups, instance)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
