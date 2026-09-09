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
 *
 * THE COPYRIGHT HOLDERS DISCLAIM ALL WARRANTIES WITH REGARD TO THIS SOFTWARE,
 * INCLUDING ALL IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS, IN NO
 * EVENT SHALL THE COPYRIGHT HOLDERS BE LIABLE FOR ANY SPECIAL, INDIRECT, OR
 * CONSEQUENTIAL DAMAGES OR ANY DAMAGES WHATSOEVER RESULTING FROM LOSS OF USE,
 * DATA OR PROFITS, WHETHER IN AN ACTION OF CONTRACT, NEGLIGENCE OR OTHER
 * TORTIOUS ACTION, ARISING OUT OF OR IN CONNECTION WITH THE USE OR PERFORMANCE
 * OF THIS SOFTWARE.
 */

// Dependencies supplied by other translation units:
// linux/kref.h, drm/drm_lease.h

use core::ffi::c_char;

pub const DRM_OBJECT_MAX_PROPERTY: usize = 64;

pub enum drm_object_properties {}
pub enum drm_property {}
pub enum drm_device {}
pub enum drm_file {}

#[repr(C)]
pub struct kref {
    _opaque: [u8; 0],
}

/**
 * struct drm_mode_object - base structure for modeset objects
 * @id: userspace visible identifier
 * @type: type of the object, one of DRM_MODE_OBJECT_\*
 * @properties: properties attached to this object, including values
 * @refcount: reference count for objects with dynamic lifetime
 * @free_cb: free function callback, only set for objects with dynamic lifetime
 *
 * Base structure for modeset objects visible to userspace. Objects can be
 * looked up using drm_mode_object_find(). Besides basic uapi interface
 * properties like @id and @type it provides two services:
 *
 * - It tracks attached properties and their values. This is used by &drm_crtc,
 *   &drm_plane and &drm_connector. Properties are attached by calling
 *   drm_object_attach_property() before the object is visible to userspace.
 *
 * - For objects with dynamic lifetimes (as indicated by a non-NULL @free_cb) it
 *   provides reference counting through drm_mode_object_get() and
 *   drm_mode_object_put(). This is used by &drm_framebuffer, &drm_connector
 *   and &drm_property_blob. These objects provide specialized reference
 *   counting wrappers.
 */
#[repr(C)]
pub struct drm_mode_object {
    pub id: u32,
    pub type_: u32,
    pub properties: *mut drm_object_properties,
    pub refcount: kref,
    pub free_cb: Option<unsafe extern "C" fn(kref: *mut kref)>,
}

/** Property tracking for drm_mode_object. */
#[repr(C)]
pub struct drm_object_properties {
    pub count: core::ffi::c_int,
    pub properties: [*mut drm_property; DRM_OBJECT_MAX_PROPERTY],
    pub values: [u64; DRM_OBJECT_MAX_PROPERTY],
}

/* Avoid boilerplate. */
#[macro_export]
macro_rules! DRM_ENUM_NAME_FN {
    ($fnname:ident, $list:expr) => {
        unsafe extern "C" fn $fnname(val: core::ffi::c_int) -> *const c_char {
            let mut i = 0usize;
            while i < ($list).len() {
                if ($list)[i].type_ == val {
                    return ($list)[i].name;
                }
                i += 1;
            }
            b"(unknown)\0".as_ptr() as *const c_char
        }
    };
}

extern "C" {
    pub fn drm_mode_object_find(
        dev: *mut drm_device,
        file_priv: *mut drm_file,
        id: u32,
        type_: u32,
    ) -> *mut drm_mode_object;
    pub fn drm_mode_object_get(obj: *mut drm_mode_object);
    pub fn drm_mode_object_put(obj: *mut drm_mode_object);

    pub fn drm_object_property_set_value(
        obj: *mut drm_mode_object,
        property: *mut drm_property,
        val: u64,
    ) -> core::ffi::c_int;
    pub fn drm_object_property_get_value(
        obj: *mut drm_mode_object,
        property: *mut drm_property,
        value: *mut u64,
    ) -> core::ffi::c_int;
    pub fn drm_object_property_get_default_value(
        obj: *mut drm_mode_object,
        property: *mut drm_property,
        val: *mut u64,
    ) -> core::ffi::c_int;
    pub fn drm_object_immutable_property_get_value(
        obj: *mut drm_mode_object,
        property: *mut drm_property,
        val: *mut u64,
    ) -> core::ffi::c_int;

    pub fn drm_object_attach_property(
        obj: *mut drm_mode_object,
        property: *mut drm_property,
        init_val: u64,
    );

    pub fn drm_mode_object_lease_required(type_: u32) -> bool;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
