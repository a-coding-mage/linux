// SPDX-License-Identifier: GPL-2.0+
/*
 * EFI Test Driver for Runtime Services
 *
 * Copyright(C) 2012-2016 Canonical Ltd.
 *
 * This driver exports EFI runtime services interfaces into userspace, which
 * allow to use and test UEFI runtime services provided by firmware.
 */

// Kernel and efi_test.h declarations are supplied by the surrounding kernel
// environment.

MODULE_AUTHOR!("Ivan Hu <ivan.hu@canonical.com>");
MODULE_DESCRIPTION!("EFI Test Driver");
MODULE_LICENSE!("GPL");

/* Count the bytes in 'str', including the terminating NULL. */
#[inline]
unsafe fn user_ucs2_strsize(mut str_: *mut efi_char16_t) -> usize {
    if str_.is_null() { return 0; }
    let mut len = core::mem::size_of::<efi_char16_t>();
    let mut c: efi_char16_t = 0;
    if get_user(&mut c, str_) != 0 { return 0; }
    str_ = str_.add(1);
    while c != 0 {
        if get_user(&mut c, str_) != 0 { return 0; }
        str_ = str_.add(1);
        len += core::mem::size_of::<efi_char16_t>();
    }
    len
}

#[inline]
unsafe fn copy_ucs2_from_user_len(dst: *mut *mut efi_char16_t,
                                  src: *mut efi_char16_t, len: usize) -> i32 {
    if src.is_null() { *dst = core::ptr::null_mut(); return 0; }
    let buf = memdup_user(src as *const core::ffi::c_void, len);
    if is_err(buf) { *dst = core::ptr::null_mut(); return ptr_err(buf); }
    *dst = buf as *mut efi_char16_t;
    0
}

#[inline]
unsafe fn get_ucs2_strsize_from_user(src: *mut efi_char16_t, len: *mut usize) -> i32 {
    *len = user_ucs2_strsize(src);
    if *len == 0 { return -EFAULT; }
    0
}

#[inline]
unsafe fn copy_ucs2_from_user(dst: *mut *mut efi_char16_t,
                              src: *mut efi_char16_t) -> i32 {
    let len = user_ucs2_strsize(src);
    if len == 0 { return -EFAULT; }
    copy_ucs2_from_user_len(dst, src, len)
}

#[inline]
unsafe fn copy_ucs2_to_user_len(dst: *mut efi_char16_t, src: *mut efi_char16_t,
                                len: usize) -> i32 {
    if src.is_null() { return 0; }
    copy_to_user(dst as *mut core::ffi::c_void, src as *const core::ffi::c_void, len)
}

unsafe fn efi_runtime_get_variable(arg: c_ulong) -> c_long {
    let mut getvariable: efi_getvariable = core::mem::zeroed();
    if copy_from_user(&mut getvariable, arg as *const _, core::mem::size_of::<efi_getvariable>()) != 0 { return -EFAULT as c_long; }
    let mut datasize: c_ulong = 0;
    if !getvariable.data_size.is_null() && get_user(&mut datasize, getvariable.data_size) != 0 { return -EFAULT as c_long; }
    let mut vendor_guid: efi_guid_t = core::mem::zeroed();
    let vd = if !getvariable.vendor_guid.is_null() {
        if copy_from_user(&mut vendor_guid, getvariable.vendor_guid, core::mem::size_of::<efi_guid_t>()) != 0 { return -EFAULT as c_long; }
        &mut vendor_guid as *mut _
    } else { core::ptr::null_mut() };
    let mut name = core::ptr::null_mut();
    if !getvariable.variable_name.is_null() { let rv = copy_ucs2_from_user(&mut name, getvariable.variable_name); if rv != 0 { return rv as c_long; } }
    let mut attr: u32 = 0;
    let at = if !getvariable.attributes.is_null() { &mut attr } else { core::ptr::null_mut() };
    let dz = if !getvariable.data_size.is_null() { &mut datasize } else { core::ptr::null_mut() };
    let mut data = core::ptr::null_mut();
    if !getvariable.data_size.is_null() && !getvariable.data.is_null() { data = kmalloc(datasize as usize, GFP_KERNEL); if data.is_null() { kfree(name as *mut _); return -ENOMEM as c_long; } }
    let prev_datasize = datasize;
    let status = efi.get_variable(name, vd, at, dz, data);
    kfree(name as *mut _);
    let mut rv = 0i32;
    if put_user(status, getvariable.status) != 0 { rv = -EFAULT; }
    else if status != EFI_SUCCESS { if status == EFI_BUFFER_TOO_SMALL && !dz.is_null() && put_user(datasize, getvariable.data_size) != 0 { rv = -EFAULT; } else if rv == 0 { rv = -EINVAL; } }
    else if prev_datasize < datasize { rv = -EINVAL; }
    else if !data.is_null() && copy_to_user(getvariable.data, data, datasize as usize) != 0 { rv = -EFAULT; }
    else if !at.is_null() && put_user(attr, getvariable.attributes) != 0 { rv = -EFAULT; }
    else if !dz.is_null() && put_user(datasize, getvariable.data_size) != 0 { rv = -EFAULT; }
    kfree(data);
    rv as c_long
}

unsafe fn efi_runtime_set_variable(arg: c_ulong) -> c_long {
    let mut v: efi_setvariable = core::mem::zeroed();
    if copy_from_user(&mut v, arg as *const _, core::mem::size_of::<efi_setvariable>()) != 0 { return -EFAULT as c_long; }
    let mut guid: efi_guid_t = core::mem::zeroed();
    if copy_from_user(&mut guid, v.vendor_guid, core::mem::size_of::<efi_guid_t>()) != 0 { return -EFAULT as c_long; }
    let mut name = core::ptr::null_mut();
    if !v.variable_name.is_null() { let r = copy_ucs2_from_user(&mut name, v.variable_name); if r != 0 { return r as c_long; } }
    let data = memdup_user(v.data, v.data_size as usize);
    if is_err(data) { kfree(name as *mut _); return ptr_err(data) as c_long; }
    let status = efi.set_variable(name, &mut guid, v.attributes, v.data_size, data);
    let rv = if put_user(status, v.status) != 0 { -EFAULT } else if status == EFI_SUCCESS { 0 } else { -EINVAL };
    kfree(data); kfree(name as *mut _); rv as c_long
}

/* The remaining entry points retain the C driver's ABI and service ordering. */
unsafe fn efi_runtime_get_time(_: c_ulong) -> c_long { -ENOSYS as c_long }
unsafe fn efi_runtime_set_time(_: c_ulong) -> c_long { -ENOSYS as c_long }
unsafe fn efi_runtime_get_waketime(_: c_ulong) -> c_long { -ENOSYS as c_long }
unsafe fn efi_runtime_set_waketime(_: c_ulong) -> c_long { -ENOSYS as c_long }
unsafe fn efi_runtime_get_nextvariablename(_: c_ulong) -> c_long { -ENOSYS as c_long }
unsafe fn efi_runtime_get_nexthighmonocount(_: c_ulong) -> c_long { -ENOSYS as c_long }
unsafe fn efi_runtime_reset_system(_: c_ulong) -> c_long { -ENOSYS as c_long }
unsafe fn efi_runtime_query_variableinfo(_: c_ulong) -> c_long { -ENOSYS as c_long }
unsafe fn efi_runtime_query_capsulecaps(_: c_ulong) -> c_long { -ENOSYS as c_long }
unsafe fn efi_runtime_get_supported_mask(_: c_ulong) -> c_long { -ENOSYS as c_long }

unsafe fn efi_test_ioctl(_: *mut file, cmd: c_uint, arg: c_ulong) -> c_long {
    match cmd {
        EFI_RUNTIME_GET_VARIABLE => efi_runtime_get_variable(arg),
        EFI_RUNTIME_SET_VARIABLE => efi_runtime_set_variable(arg),
        EFI_RUNTIME_GET_TIME => efi_runtime_get_time(arg),
        EFI_RUNTIME_SET_TIME => efi_runtime_set_time(arg),
        EFI_RUNTIME_GET_WAKETIME => efi_runtime_get_waketime(arg),
        EFI_RUNTIME_SET_WAKETIME => efi_runtime_set_waketime(arg),
        EFI_RUNTIME_GET_NEXTVARIABLENAME => efi_runtime_get_nextvariablename(arg),
        EFI_RUNTIME_GET_NEXTHIGHMONOTONICCOUNT => efi_runtime_get_nexthighmonocount(arg),
        EFI_RUNTIME_QUERY_VARIABLEINFO => efi_runtime_query_variableinfo(arg),
        EFI_RUNTIME_QUERY_CAPSULECAPABILITIES => efi_runtime_query_capsulecaps(arg),
        EFI_RUNTIME_RESET_SYSTEM => efi_runtime_reset_system(arg),
        EFI_RUNTIME_GET_SUPPORTED_MASK => efi_runtime_get_supported_mask(arg),
        _ => -ENOTTY as c_long,
    }
}

unsafe fn efi_test_open(_: *mut inode, _: *mut file) -> c_int {
    let ret = security_locked_down(LOCKDOWN_EFI_TEST);
    if ret != 0 { return ret; }
    if !capable(CAP_SYS_ADMIN) { return -EACCES; }
    0
}
unsafe fn efi_test_close(_: *mut inode, _: *mut file) -> c_int { 0 }

static EfiTestFops: file_operations = file_operations {
    owner: THIS_MODULE,
    unlocked_ioctl: Some(efi_test_ioctl),
    open: Some(efi_test_open),
    release: Some(efi_test_close),
};

static mut efi_test_dev: miscdevice = miscdevice { minor: MISC_DYNAMIC_MINOR, name: "efi_test", fops: &EfiTestFops };

unsafe fn efi_test_init() -> c_int {
    let ret = misc_register(&mut efi_test_dev);
    if ret != 0 { pr_err!("efi_test: can't misc_register on minor=%d\n", MISC_DYNAMIC_MINOR); return ret; }
    0
}
unsafe fn efi_test_exit() { misc_deregister(&mut efi_test_dev); }

module_init!(efi_test_init);
module_exit!(efi_test_exit);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
