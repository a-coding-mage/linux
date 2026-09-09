// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright (C), 2008-2021, OPPO Mobile Comm Corp., Ltd.
 *             https://www.oppo.com/
 */
// Linux sysfs, kobject, internal, and compression declarations are supplied by dependencies.

#[repr(C)]
pub struct erofs_attr {
    pub attr: attribute,
    pub attr_id: i16,
    pub struct_type: i32,
    pub offset: i32,
}

pub const attr_feature: i32 = 0;
pub const attr_drop_caches: i32 = 1;
pub const attr_pointer_ui: i32 = 2;
pub const attr_pointer_bool: i32 = 3;
pub const attr_accel: i32 = 4;
pub const struct_erofs_sb_info: i32 = 0;
pub const struct_erofs_mount_opts: i32 = 1;

// Build-time CONFIG_EROFS_FS_ZIP and CONFIG_EROFS_FS_ZIP_ACCEL conditionals are preserved below.
extern "C" {
    static mut erofs_sb_groups: *mut *mut attribute_group;
    static mut erofs_groups: *mut *mut attribute_group;
    static mut erofs_feat_groups: *mut *mut attribute_group;
    fn sysfs_emit(buf: *mut c_char, fmt: *const c_char, ...) -> isize;
    fn z_erofs_crypto_show_engines(buf: *mut c_char, size: usize, delim: c_char) -> isize;
    fn kstrtoul(s: *const c_char, base: u32, result: *mut c_ulong) -> i32;
    fn skip_spaces(s: *const c_char) -> *const c_char;
    fn strcmp(a: *const c_char, b: *const c_char) -> i32;
}

static mut erofs_attr_compr_cfgs: erofs_attr = erofs_attr { attr: attribute { name: core::ptr::null(), mode: 0o444 }, attr_id: attr_feature as i16, struct_type: 0, offset: 0 };
static mut erofs_attr_big_pcluster: erofs_attr = erofs_attr { attr: attribute { name: core::ptr::null(), mode: 0o444 }, attr_id: attr_feature as i16, struct_type: 0, offset: 0 };
static mut erofs_attr_chunked_file: erofs_attr = erofs_attr { attr: attribute { name: core::ptr::null(), mode: 0o444 }, attr_id: attr_feature as i16, struct_type: 0, offset: 0 };
static mut erofs_attr_device_table: erofs_attr = erofs_attr { attr: attribute { name: core::ptr::null(), mode: 0o444 }, attr_id: attr_feature as i16, struct_type: 0, offset: 0 };
static mut erofs_attr_compr_head2: erofs_attr = erofs_attr { attr: attribute { name: core::ptr::null(), mode: 0o444 }, attr_id: attr_feature as i16, struct_type: 0, offset: 0 };
static mut erofs_attr_sb_chksum: erofs_attr = erofs_attr { attr: attribute { name: core::ptr::null(), mode: 0o444 }, attr_id: attr_feature as i16, struct_type: 0, offset: 0 };
static mut erofs_attr_ztailpacking: erofs_attr = erofs_attr { attr: attribute { name: core::ptr::null(), mode: 0o444 }, attr_id: attr_feature as i16, struct_type: 0, offset: 0 };
static mut erofs_attr_fragments: erofs_attr = erofs_attr { attr: attribute { name: core::ptr::null(), mode: 0o444 }, attr_id: attr_feature as i16, struct_type: 0, offset: 0 };
static mut erofs_attr_dedupe: erofs_attr = erofs_attr { attr: attribute { name: core::ptr::null(), mode: 0o444 }, attr_id: attr_feature as i16, struct_type: 0, offset: 0 };
static mut erofs_attr_48bit: erofs_attr = erofs_attr { attr: attribute { name: core::ptr::null(), mode: 0o444 }, attr_id: attr_feature as i16, struct_type: 0, offset: 0 };
static mut erofs_attr_metabox: erofs_attr = erofs_attr { attr: attribute { name: core::ptr::null(), mode: 0o444 }, attr_id: attr_feature as i16, struct_type: 0, offset: 0 };

unsafe fn __struct_ptr(sbi: *mut erofs_sb_info, struct_type: i32, offset: i32) -> *mut u8 {
    if struct_type == struct_erofs_sb_info { return (sbi as *mut u8).add(offset as usize); }
    if struct_type == struct_erofs_mount_opts { return (&mut (*sbi).opt as *mut _ as *mut u8).add(offset as usize); }
    core::ptr::null_mut()
}

pub unsafe extern "C" fn erofs_attr_show(kobj: *mut kobject, attr: *mut attribute, buf: *mut c_char) -> isize {
    let sbi = container_of!(kobj, erofs_sb_info, s_kobj);
    let a = container_of!(attr, erofs_attr, attr);
    let ptr = __struct_ptr(sbi, (*a).struct_type, (*a).offset);
    match (*a).attr_id as i32 {
        attr_feature => return sysfs_emit(buf, c"supported\n".as_ptr(),),
        attr_pointer_ui => { if ptr.is_null() { return 0; } return sysfs_emit(buf, c"%u\n".as_ptr(), *(ptr as *mut u32)); }
        attr_pointer_bool => { if ptr.is_null() { return 0; } return sysfs_emit(buf, c"%d\n".as_ptr(), *(ptr as *mut bool) as i32); }
        attr_accel => return z_erofs_crypto_show_engines(buf, PAGE_SIZE, b'\n' as c_char),
        _ => 0,
    }
}

pub unsafe extern "C" fn erofs_attr_store(kobj: *mut kobject, attr: *mut attribute, buf: *const c_char, len: usize) -> isize {
    let sbi = container_of!(kobj, erofs_sb_info, s_kobj);
    let a = container_of!(attr, erofs_attr, attr);
    let ptr = __struct_ptr(sbi, (*a).struct_type, (*a).offset);
    let mut t: c_ulong = 0;
    match (*a).attr_id as i32 {
        attr_pointer_ui => {
            if ptr.is_null() { return 0; }
            let ret = kstrtoul(skip_spaces(buf), 0, &mut t); if ret != 0 { return ret as isize; }
            if t != (t as u32 as c_ulong) { return -(ERANGE as isize); }
            if cfg!(CONFIG_EROFS_FS_ZIP) && strcmp((*a).attr.name, c"sync_decompress".as_ptr()) == 0 && t > EROFS_SYNC_DECOMPRESS_FORCE_OFF as c_ulong { return -(EINVAL as isize); }
            *(ptr as *mut u32) = t as u32; len as isize
        }
        attr_pointer_bool => {
            if ptr.is_null() { return 0; }
            let ret = kstrtoul(skip_spaces(buf), 0, &mut t); if ret != 0 { return ret as isize; }
            if t != 0 && t != 1 { return -(EINVAL as isize); }
            *(ptr as *mut bool) = t != 0; len as isize
        }
        attr_drop_caches => {
            let ret = kstrtoul(skip_spaces(buf), 0, &mut t); if ret != 0 { return ret as isize; }
            if t < 1 || t > 3 { return -(EINVAL as isize); }
            if t & 2 != 0 { z_erofs_shrink_scan(sbi, !0usize); }
            if t & 1 != 0 { invalidate_mapping_pages(MNGD_MAPPING!(sbi), 0, -1); }
            len as isize
        }
        attr_accel => {
            let mut p = skip_spaces(buf); z_erofs_crypto_disable_all_engines();
            while *p != 0 { let n = strcspn(p, c"\n".as_ptr()); let ret = z_erofs_crypto_enable_engine(p, n); if ret < 0 { return ret as isize; } p = p.add(if *p.add(n) != 0 { n + 1 } else { n }); }
            len as isize
        }
        _ => 0,
    }
}
pub unsafe extern "C" fn erofs_sb_release(kobj: *mut kobject) { let sbi = container_of!(kobj, erofs_sb_info, s_kobj); complete(&mut (*sbi).s_kobj_unregister); }

static erofs_attr_ops: sysfs_ops = sysfs_ops { show: Some(erofs_attr_show), store: Some(erofs_attr_store) };
static erofs_sb_ktype: kobj_type = kobj_type { default_groups: unsafe { &mut erofs_sb_groups }, sysfs_ops: &erofs_attr_ops, release: Some(erofs_sb_release) };
static erofs_ktype: kobj_type = kobj_type { default_groups: unsafe { &mut erofs_groups }, sysfs_ops: &erofs_attr_ops, release: None };

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
