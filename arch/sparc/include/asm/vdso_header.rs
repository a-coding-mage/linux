/*
 * Copyright (c) 2017 Oracle and/or its affiliates. All rights reserved.
 */

#[repr(C)]
pub struct vdso_image {
    pub data: *mut core::ffi::c_void,
    pub size: usize, /* Always a multiple of PAGE_SIZE */
}

#[cfg(CONFIG_SPARC64)]
extern "C" {
    pub static vdso_image_64_builtin: vdso_image;
}

#[cfg(CONFIG_COMPAT)]
extern "C" {
    pub static vdso_image_32_builtin: vdso_image;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
