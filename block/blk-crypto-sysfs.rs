// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright 2021 Google LLC
 *
 * sysfs support for blk-crypto.  This file contains the code which exports the
 * crypto capabilities of devices via /sys/block/$disk/queue/crypto/.
 */

// Dependencies supplied by the surrounding kernel translation.

#[repr(C)]
struct blk_crypto_kobj {
    kobj: kobject,
    profile: *mut blk_crypto_profile,
}

#[repr(C)]
struct blk_crypto_attr {
    attr: attribute,
    show: Option<unsafe extern "C" fn(*mut blk_crypto_profile, *const blk_crypto_attr, *mut i8) -> ssize_t>,
}

unsafe fn kobj_to_crypto_profile(kobj: *mut kobject) -> *mut blk_crypto_profile {
    (*(kobj as *mut blk_crypto_kobj)).profile
}

unsafe fn attr_to_crypto_attr(attr: *const attribute) -> *const blk_crypto_attr {
    attr as *const blk_crypto_attr
}

unsafe extern "C" fn hw_wrapped_keys_show(
    _profile: *mut blk_crypto_profile,
    _attr: *const blk_crypto_attr,
    page: *mut i8,
) -> ssize_t {
    // Always show supported, since the file doesn't exist otherwise.
    sysfs_emit(page, "supported\n\0" as *const str as *const i8)
}

unsafe extern "C" fn max_dun_bits_show(
    profile: *mut blk_crypto_profile,
    _attr: *const blk_crypto_attr,
    page: *mut i8,
) -> ssize_t {
    sysfs_emit(page, "%u\n\0" as *const str as *const i8, 8 * (*profile).max_dun_bytes_supported)
}

unsafe extern "C" fn num_keyslots_show(
    profile: *mut blk_crypto_profile,
    _attr: *const blk_crypto_attr,
    page: *mut i8,
) -> ssize_t {
    sysfs_emit(page, "%u\n\0" as *const str as *const i8, (*profile).num_slots)
}

unsafe extern "C" fn raw_keys_show(
    _profile: *mut blk_crypto_profile,
    _attr: *const blk_crypto_attr,
    page: *mut i8,
) -> ssize_t {
    // Always show supported, since the file doesn't exist otherwise.
    sysfs_emit(page, "supported\n\0" as *const str as *const i8)
}

static mut hw_wrapped_keys_attr: blk_crypto_attr = blk_crypto_attr {
    attr: attribute { name: "hw_wrapped_keys\0" as *const str as *const i8, mode: 0 },
    show: Some(hw_wrapped_keys_show),
};
static mut max_dun_bits_attr: blk_crypto_attr = blk_crypto_attr {
    attr: attribute { name: "max_dun_bits\0" as *const str as *const i8, mode: 0 },
    show: Some(max_dun_bits_show),
};
static mut num_keyslots_attr: blk_crypto_attr = blk_crypto_attr {
    attr: attribute { name: "num_keyslots\0" as *const str as *const i8, mode: 0 },
    show: Some(num_keyslots_show),
};
static mut raw_keys_attr: blk_crypto_attr = blk_crypto_attr {
    attr: attribute { name: "raw_keys\0" as *const str as *const i8, mode: 0 },
    show: Some(raw_keys_show),
};

unsafe extern "C" fn blk_crypto_is_visible(kobj: *mut kobject, attr: *const attribute, _n: i32) -> umode_t {
    let profile = kobj_to_crypto_profile(kobj);
    let a = attr_to_crypto_attr(attr);
    if a == &hw_wrapped_keys_attr && (*profile).key_types_supported & BLK_CRYPTO_KEY_TYPE_HW_WRAPPED == 0 { return 0; }
    if a == &raw_keys_attr && (*profile).key_types_supported & BLK_CRYPTO_KEY_TYPE_RAW == 0 { return 0; }
    0o444
}

static mut blk_crypto_attrs: [*const attribute; 5] = [
    unsafe { &hw_wrapped_keys_attr.attr }, unsafe { &max_dun_bits_attr.attr },
    unsafe { &num_keyslots_attr.attr }, unsafe { &raw_keys_attr.attr }, core::ptr::null(),
];
static mut blk_crypto_attr_group: attribute_group = attribute_group {
    attrs_const: unsafe { &blk_crypto_attrs as *const _ }, is_visible_const: Some(blk_crypto_is_visible),
};

/* The encryption mode attributes are initialized at boot by blk_crypto_sysfs_init(). */
static mut __blk_crypto_mode_attrs: [blk_crypto_attr; BLK_ENCRYPTION_MODE_MAX] = [unsafe { core::mem::zeroed() }; BLK_ENCRYPTION_MODE_MAX];
static mut blk_crypto_mode_attrs: [*const attribute; BLK_ENCRYPTION_MODE_MAX + 1] = [core::ptr::null(); BLK_ENCRYPTION_MODE_MAX + 1];

unsafe extern "C" fn blk_crypto_mode_is_visible(kobj: *mut kobject, attr: *const attribute, _n: i32) -> umode_t {
    let profile = kobj_to_crypto_profile(kobj);
    let a = attr_to_crypto_attr(attr);
    let mode_num = a.offset_from(__blk_crypto_mode_attrs.as_ptr()) as usize;
    if (*profile).modes_supported[mode_num] != 0 { 0o444 } else { 0 }
}
unsafe extern "C" fn blk_crypto_mode_show(profile: *mut blk_crypto_profile, attr: *const blk_crypto_attr, page: *mut i8) -> ssize_t {
    let mode_num = attr.offset_from(__blk_crypto_mode_attrs.as_ptr()) as usize;
    sysfs_emit(page, "0x%x\n\0" as *const str as *const i8, (*profile).modes_supported[mode_num])
}

static mut blk_crypto_modes_attr_group: attribute_group = attribute_group { name: "modes\0" as *const str as *const i8, attrs_const: unsafe { &blk_crypto_mode_attrs as *const _ }, is_visible_const: Some(blk_crypto_mode_is_visible) };
static mut blk_crypto_attr_groups: [*const attribute_group; 3] = [unsafe { &blk_crypto_attr_group }, unsafe { &blk_crypto_modes_attr_group }, core::ptr::null()];

unsafe extern "C" fn blk_crypto_attr_show(kobj: *mut kobject, attr: *mut attribute, page: *mut i8) -> ssize_t {
    let profile = kobj_to_crypto_profile(kobj); let a = attr_to_crypto_attr(attr);
    ((*a).show.unwrap())(profile, a, page)
}
static mut blk_crypto_attr_ops: sysfs_ops = sysfs_ops { show: Some(blk_crypto_attr_show) };
unsafe extern "C" fn blk_crypto_release(kobj: *mut kobject) { kfree(kobj as *mut blk_crypto_kobj); }
static mut blk_crypto_ktype: kobj_type = kobj_type { default_groups: unsafe { &blk_crypto_attr_groups }, sysfs_ops: unsafe { &blk_crypto_attr_ops }, release: Some(blk_crypto_release) };

unsafe extern "C" fn blk_crypto_sysfs_register(disk: *mut gendisk) -> i32 {
    let q = (*disk).queue; if (*q).crypto_profile.is_null() { return 0; }
    let obj = kzalloc_obj::<blk_crypto_kobj>(); if obj.is_null() { return -12; }
    (*obj).profile = (*q).crypto_profile;
    let err = kobject_init_and_add(&mut (*obj).kobj, &blk_crypto_ktype, &mut (*disk).queue_kobj, "crypto\0" as *const str as *const i8);
    if err != 0 { kobject_put(&mut (*obj).kobj); return err; }
    (*q).crypto_kobject = &mut (*obj).kobj; 0
}
unsafe extern "C" fn blk_crypto_sysfs_unregister(disk: *mut gendisk) { kobject_put((*disk).queue.crypto_kobject); }

unsafe extern "C" fn blk_crypto_sysfs_init() -> i32 {
    for i in 1..BLK_ENCRYPTION_MODE_MAX { let attr = &mut __blk_crypto_mode_attrs[i]; attr.attr.name = blk_crypto_modes[i].name; attr.attr.mode = 0o444; attr.show = Some(blk_crypto_mode_show); blk_crypto_mode_attrs[i - 1] = &attr.attr; }
    0
}

// subsys_initcall(blk_crypto_sysfs_init)

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
