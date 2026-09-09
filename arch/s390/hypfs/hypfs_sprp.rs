// SPDX-License-Identifier: GPL-2.0
/*
 *    Hypervisor filesystem for Linux on s390.
 *    Set Partition-Resource Parameter interface.
 *
 *    Copyright IBM Corp. 2013
 *    Author(s): Martin Schwidefsky <schwidefsky@de.ibm.com>
 */

// Dependencies supplied by the surrounding kernel translation.

const DIAG304_SET_WEIGHTS: ::core::ffi::c_ulong = 0;
const DIAG304_QUERY_PRP: ::core::ffi::c_ulong = 1;
const DIAG304_SET_CAPPING: ::core::ffi::c_ulong = 2;

const DIAG304_CMD_MAX: ::core::ffi::c_ulong = 2;

unsafe extern "C" {
    fn virt_to_phys(address: *mut ::core::ffi::c_void) -> ::core::ffi::c_ulong;
    fn diag_stat_inc(stat: ::core::ffi::c_int);
    fn free_page(address: ::core::ffi::c_ulong);
    fn get_zeroed_page(flags: ::core::ffi::c_ulong) -> ::core::ffi::c_ulong;
    fn hypfs_sprp_diag304_external(
        data: *mut ::core::ffi::c_void,
        cmd: ::core::ffi::c_ulong,
    ) -> ::core::ffi::c_ulong;
    fn capable(capability: ::core::ffi::c_int) -> bool;
    fn copy_from_user(
        to: *mut ::core::ffi::c_void,
        from: *const ::core::ffi::c_void,
        size: usize,
    ) -> usize;
    fn copy_to_user(
        to: *mut ::core::ffi::c_void,
        from: *const ::core::ffi::c_void,
        size: usize,
    ) -> usize;
    fn kfree(object: *mut ::core::ffi::c_void);
    fn hypfs_dbfs_create_file(file: *mut hypfs_dbfs_file);
    fn hypfs_dbfs_remove_file(file: *mut hypfs_dbfs_file);
}

#[inline]
unsafe fn __hypfs_sprp_diag304(
    data: *mut ::core::ffi::c_void,
    cmd: ::core::ffi::c_ulong,
) -> ::core::ffi::c_ulong {
    // The C implementation executes DIAG 0x304 with the physical address in
    // register pair r1. The architecture-specific register-pair ABI is an
    // external dependency of this translation.
    hypfs_sprp_diag304_external(data, cmd)
}

unsafe fn hypfs_sprp_diag304(
    data: *mut ::core::ffi::c_void,
    cmd: ::core::ffi::c_ulong,
) -> ::core::ffi::c_ulong {
    diag_stat_inc(DIAG_STAT_X304);
    __hypfs_sprp_diag304(data, cmd)
}

unsafe fn hypfs_sprp_free(data: *const ::core::ffi::c_void) {
    free_page(data as ::core::ffi::c_ulong);
}

unsafe fn hypfs_sprp_create(
    data_ptr: *mut *mut ::core::ffi::c_void,
    free_ptr: *mut *mut ::core::ffi::c_void,
    size: *mut usize,
) -> ::core::ffi::c_int {
    let mut rc: ::core::ffi::c_ulong;
    let data: *mut ::core::ffi::c_void;

    data = get_zeroed_page(GFP_KERNEL) as *mut ::core::ffi::c_void;
    if data.is_null() {
        return -ENOMEM;
    }
    rc = hypfs_sprp_diag304(data, DIAG304_QUERY_PRP);
    if rc != 1 {
        *data_ptr = ::core::ptr::null_mut();
        *free_ptr = ::core::ptr::null_mut();
        *size = 0;
        free_page(data as ::core::ffi::c_ulong);
        return -EIO;
    }
    *data_ptr = data;
    *free_ptr = data;
    *size = PAGE_SIZE;
    0
}

unsafe fn __hypfs_sprp_ioctl(
    user_area: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    let mut diag304: *mut hypfs_diag304 = ::core::ptr::null_mut();
    let mut cmd: ::core::ffi::c_ulong;
    let mut udata: *mut ::core::ffi::c_void;
    let data: *mut ::core::ffi::c_void;
    let mut rc: ::core::ffi::c_int;

    rc = -ENOMEM;
    data = get_zeroed_page(GFP_KERNEL) as *mut ::core::ffi::c_void;
    diag304 = kzalloc_obj_hypfs_diag304();
    if data.is_null() || diag304.is_null() {
        goto_out!(out);
    }

    rc = -EFAULT;
    if copy_from_user(diag304 as *mut ::core::ffi::c_void, user_area, core::mem::size_of::<hypfs_diag304>()) != 0 {
        goto_out!(out);
    }
    rc = -EINVAL;
    if ((*diag304).args[0] >> 8) != 0 || (*diag304).args[1] > DIAG304_CMD_MAX {
        goto_out!(out);
    }

    rc = -EFAULT;
    udata = (*diag304).data as usize as *mut ::core::ffi::c_void;
    if ((*diag304).args[1] == DIAG304_SET_WEIGHTS || (*diag304).args[1] == DIAG304_SET_CAPPING)
        && copy_from_user(data, udata, PAGE_SIZE) != 0
    {
        goto_out!(out);
    }

    cmd = *(&(*diag304).args[0] as *const _ as *const ::core::ffi::c_ulong);
    (*diag304).rc = hypfs_sprp_diag304(data, cmd);

    if (*diag304).args[1] == DIAG304_QUERY_PRP
        && copy_to_user(udata, data, PAGE_SIZE) != 0
    {
        rc = -EFAULT;
        goto_out!(out);
    }

    rc = if copy_to_user(user_area, diag304 as *const ::core::ffi::c_void, core::mem::size_of::<hypfs_diag304>()) != 0 { -EFAULT } else { 0 };

    goto_out!(out);
    out: {
        kfree(diag304 as *mut ::core::ffi::c_void);
        free_page(data as ::core::ffi::c_ulong);
        rc
    }
}

unsafe fn hypfs_sprp_ioctl(
    _file: *mut file,
    cmd: ::core::ffi::c_uint,
    arg: ::core::ffi::c_ulong,
) -> ::core::ffi::c_long {
    if !capable(CAP_SYS_ADMIN) {
        return -EACCES as ::core::ffi::c_long;
    }
    let argp = arg as *mut ::core::ffi::c_void;
    match cmd {
        HYPFS_DIAG304 => __hypfs_sprp_ioctl(argp) as ::core::ffi::c_long,
        _ => -ENOTTY as ::core::ffi::c_long,
    }
}

static mut hypfs_sprp_file: hypfs_dbfs_file = hypfs_dbfs_file {
    name: b"diag_304\0".as_ptr() as *const ::core::ffi::c_char,
    data_create: Some(hypfs_sprp_create),
    data_free: Some(hypfs_sprp_free),
    unlocked_ioctl: Some(hypfs_sprp_ioctl),
};

unsafe fn hypfs_sprp_init() {
    if !sclp.has_sprp {
        return;
    }
    hypfs_dbfs_create_file(&raw mut hypfs_sprp_file);
}

unsafe fn hypfs_sprp_exit() {
    if !sclp.has_sprp {
        return;
    }
    hypfs_dbfs_remove_file(&raw mut hypfs_sprp_file);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
