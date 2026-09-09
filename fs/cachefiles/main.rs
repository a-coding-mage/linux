// SPDX-License-Identifier: GPL-2.0-or-later
/* Network filesystem caching backend to use cache files on a premounted
 * filesystem
 *
 * Copyright (C) 2021 Red Hat, Inc. All Rights Reserved.
 * Written by David Howells (dhowells@redhat.com)
 */

use core::ffi::{c_char, c_int, c_uint, c_void};

// Kernel headers and build-time registration macros from the C source are
// supplied by the surrounding kernel translation unit.

#[repr(C)]
pub struct kmem_cache {
    _private: [u8; 0],
}

#[repr(C)]
pub struct file_operations {
    _private: [u8; 0],
}

#[repr(C)]
pub struct miscdevice {
    pub minor: c_int,
    pub name: *const c_char,
    pub fops: *const file_operations,
}

extern "C" {
    static cachefiles_daemon_fops: file_operations;

    fn cachefiles_register_error_injection() -> c_int;
    fn misc_register(dev: *mut miscdevice) -> c_int;
    fn misc_deregister(dev: *mut miscdevice);
    fn kmem_cache_create(
        name: *const c_char,
        size: usize,
        align: usize,
        flags: usize,
        ctor: *mut c_void,
    ) -> *mut kmem_cache;
    fn kmem_cache_destroy(cache: *mut kmem_cache);
    fn cachefiles_unregister_error_injection();
}

pub static mut cachefiles_debug: c_uint = 0;
pub static mut cachefiles_object_jar: *mut kmem_cache = core::ptr::null_mut();

// module_param_named(debug, cachefiles_debug, uint, S_IWUSR | S_IRUGO);
// MODULE_PARM_DESC(cachefiles_debug, "CacheFiles debugging mask");
// MODULE_DESCRIPTION("Mounted-filesystem based cache");
// MODULE_AUTHOR("Red Hat, Inc.");
// MODULE_LICENSE("GPL");

const MISC_DYNAMIC_MINOR: c_int = 255;
const SLAB_HWCACHE_ALIGN: usize = 0;
const ENOMEM: c_int = 12;

static mut cachefiles_dev: miscdevice = miscdevice {
    minor: MISC_DYNAMIC_MINOR,
    name: b"cachefiles\0".as_ptr() as *const c_char,
    fops: unsafe { &cachefiles_daemon_fops as *const file_operations },
};

// The definition is supplied by internal.h in the C source.
#[repr(C)]
pub struct cachefiles_object {
    _private: [u8; 0],
}

/*
 * initialise the fs caching module
 */
#[allow(non_snake_case)]
unsafe fn cachefiles_init() -> c_int {
    let mut ret: c_int;

    ret = cachefiles_register_error_injection();
    if ret < 0 {
        // goto error_einj;
        // pr_err("failed to register: %d\n", ret);
        return ret;
    }
    ret = misc_register(&mut cachefiles_dev);
    if ret < 0 {
        // goto error_dev;
        cachefiles_unregister_error_injection();
        // pr_err("failed to register: %d\n", ret);
        return ret;
    }

    /* create an object jar */
    ret = -ENOMEM;
    cachefiles_object_jar = kmem_cache_create(
        b"cachefiles_object_jar\0".as_ptr() as *const c_char,
        core::mem::size_of::<cachefiles_object>(),
        0,
        SLAB_HWCACHE_ALIGN,
        core::ptr::null_mut(),
    );
    if cachefiles_object_jar.is_null() {
        // pr_notice("Failed to allocate an object jar\n");
        misc_deregister(&mut cachefiles_dev);
        cachefiles_unregister_error_injection();
        // pr_err("failed to register: %d\n", ret);
        return ret;
    }

    // pr_info("Loaded\n");
    return 0;
}

// fs_initcall(cachefiles_init);

/*
 * clean up on module removal
 */
#[allow(non_snake_case)]
unsafe fn cachefiles_exit() {
    // pr_info("Unloading\n");

    kmem_cache_destroy(cachefiles_object_jar);
    misc_deregister(&mut cachefiles_dev);
    cachefiles_unregister_error_injection();
}

// module_exit(cachefiles_exit);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
