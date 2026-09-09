/*
 * Copyright (c) 2016 Intel Corporation
 *
 * Permission to use, copy, modify, distribute, and sell this software and its
 * documentation for any purpose is hereby granted without fee, provided that
 * the above copyright notice appear in all copies and that both that copyright
 * notice and this permission notice appear in supporting documentation, and
 * that the name of the copyright holders not be used in advertising or
 * publicity pertaining to distribution of the software without specific,
 * written prior permission.  The copyright holders make no representations
 * about the suitability of this software for any purpose.  It is provided "as
 * is" without express or implied warranty.
 */

// Dependencies supplied by the surrounding DRM translation.

#[repr(C)]
pub struct drm_encoder_funcs {
    pub reset: Option<unsafe extern "C" fn(encoder: *mut drm_encoder)>,
    pub destroy: Option<unsafe extern "C" fn(encoder: *mut drm_encoder)>,
    pub late_register: Option<unsafe extern "C" fn(encoder: *mut drm_encoder) -> ::std::os::raw::c_int>,
    pub early_unregister: Option<unsafe extern "C" fn(encoder: *mut drm_encoder)>,
    pub debugfs_init: Option<unsafe extern "C" fn(encoder: *mut drm_encoder, root: *mut dentry)>,
}

#[repr(C)]
pub struct drm_encoder {
    pub dev: *mut drm_device,
    pub head: list_head,
    pub base: drm_mode_object,
    pub name: *mut ::std::os::raw::c_char,
    pub encoder_type: ::std::os::raw::c_int,
    pub index: ::std::os::raw::c_uint,
    pub possible_crtcs: u32,
    pub possible_clones: u32,
    pub crtc: *mut drm_crtc,
    pub bridge_chain: list_head,
    pub bridge_chain_mutex: mutex,
    pub funcs: *const drm_encoder_funcs,
    pub helper_private: *const drm_encoder_helper_funcs,
    pub debugfs_entry: *mut dentry,
}

#[macro_export]
macro_rules! obj_to_encoder {
    ($x:expr) => { container_of!($x, drm_encoder, base) };
}

extern "C" {
    pub fn drm_encoder_init(
        dev: *mut drm_device, encoder: *mut drm_encoder,
        funcs: *const drm_encoder_funcs, encoder_type: ::std::os::raw::c_int,
        name: *const ::std::os::raw::c_char, ...
    ) -> ::std::os::raw::c_int;
    pub fn drmm_encoder_init(
        dev: *mut drm_device, encoder: *mut drm_encoder,
        funcs: *const drm_encoder_funcs, encoder_type: ::std::os::raw::c_int,
        name: *const ::std::os::raw::c_char, ...
    ) -> ::std::os::raw::c_int;
    pub fn __drmm_encoder_alloc(
        dev: *mut drm_device, size: usize, offset: usize,
        funcs: *const drm_encoder_funcs, encoder_type: ::std::os::raw::c_int,
        name: *const ::std::os::raw::c_char, ...
    ) -> *mut ::std::ffi::c_void;
    pub fn drm_encoder_cleanup(encoder: *mut drm_encoder);
}

#[macro_export]
macro_rules! drmm_encoder_alloc {
    ($dev:expr, $ty:ty, $member:ident, $funcs:expr, $encoder_type:expr, $name:expr $(, $args:expr)*) => {
        (__drmm_encoder_alloc($dev, ::std::mem::size_of::<$ty>(),
            ::std::mem::offset_of!($ty, $member), $funcs, $encoder_type, $name $(, $args)*) as *mut $ty)
    };
}

#[macro_export]
macro_rules! drmm_plain_encoder_alloc {
    ($dev:expr, $funcs:expr, $encoder_type:expr, $name:expr $(, $args:expr)*) => {
        (__drmm_encoder_alloc($dev, ::std::mem::size_of::<drm_encoder>(), 0,
            $funcs, $encoder_type, $name $(, $args)*) as *mut drm_encoder)
    };
}

#[inline]
pub unsafe fn drm_encoder_index(encoder: *const drm_encoder) -> ::std::os::raw::c_uint {
    (*encoder).index
}

#[inline]
pub unsafe fn drm_encoder_mask(encoder: *const drm_encoder) -> u32 {
    1u32 << drm_encoder_index(encoder)
}

#[inline]
pub unsafe fn drm_encoder_crtc_ok(encoder: *mut drm_encoder, crtc: *mut drm_crtc) -> bool {
    ((*encoder).possible_crtcs & drm_crtc_mask(crtc)) != 0
}

#[inline]
pub unsafe fn drm_encoder_find(
    dev: *mut drm_device, file_priv: *mut drm_file, id: u32,
) -> *mut drm_encoder {
    let mo = drm_mode_object_find(dev, file_priv, id, DRM_MODE_OBJECT_ENCODER);
    if !mo.is_null() { obj_to_encoder!(mo) } else { ::std::ptr::null_mut() }
}

// The following C iteration macros retain their dependency on the surrounding
// list and conditional-iteration helpers.
#[macro_export]
macro_rules! drm_for_each_encoder_mask {
    ($encoder:expr, $dev:expr, $encoder_mask:expr) => {
        list_for_each_entry!($encoder, &mut (*$dev).mode_config.encoder_list, head);
        for_each_if!(($encoder_mask) & drm_encoder_mask($encoder) != 0)
    };
}

#[macro_export]
macro_rules! drm_for_each_encoder {
    ($encoder:expr, $dev:expr) => {
        list_for_each_entry!($encoder, &mut (*$dev).mode_config.encoder_list, head)
    };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
