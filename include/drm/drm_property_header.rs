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

#[repr(C)]
pub struct drm_property_enum {
    pub value: u64,
    pub head: list_head,
    pub name: [core::ffi::c_char; DRM_PROP_NAME_LEN],
}

#[repr(C)]
pub struct drm_property {
    pub head: list_head,
    pub base: drm_mode_object,
    pub flags: u32,
    pub name: [core::ffi::c_char; DRM_PROP_NAME_LEN],
    pub num_values: u32,
    pub values: *mut u64,
    pub dev: *mut drm_device,
    pub enum_list: list_head,
}

#[repr(C)]
pub struct drm_property_blob {
    pub base: drm_mode_object,
    pub dev: *mut drm_device,
    pub head_global: list_head,
    pub head_file: list_head,
    pub length: usize,
    pub data: *mut core::ffi::c_void,
}

#[repr(C)]
pub struct drm_prop_enum_list {
    pub type_: core::ffi::c_int,
    pub name: *const core::ffi::c_char,
}

/* These macros depend on the shared container_of implementation. */
macro_rules! obj_to_property {
    ($x:expr) => { container_of!($x, drm_property, base) };
}
macro_rules! obj_to_blob {
    ($x:expr) => { container_of!($x, drm_property_blob, base) };
}

#[inline]
pub unsafe fn drm_property_type_is(property: *mut drm_property, type_: u32) -> bool {
    /* instanceof for props.. handles extended type vs original types: */
    if (*property).flags & DRM_MODE_PROP_EXTENDED_TYPE != 0 {
        return ((*property).flags & DRM_MODE_PROP_EXTENDED_TYPE) == type_;
    }
    (*property).flags & type_ != 0
}

unsafe extern "C" {
    pub fn drm_property_create(
        dev: *mut drm_device, flags: u32, name: *const core::ffi::c_char,
        num_values: core::ffi::c_int,
    ) -> *mut drm_property;
    pub fn drm_property_create_enum(
        dev: *mut drm_device, flags: u32, name: *const core::ffi::c_char,
        props: *const drm_prop_enum_list, num_values: core::ffi::c_int,
    ) -> *mut drm_property;
    pub fn drm_property_create_bitmask(
        dev: *mut drm_device, flags: u32, name: *const core::ffi::c_char,
        props: *const drm_prop_enum_list, num_props: core::ffi::c_int,
        supported_bits: u64,
    ) -> *mut drm_property;
    pub fn drm_property_create_range(
        dev: *mut drm_device, flags: u32, name: *const core::ffi::c_char,
        min: u64, max: u64,
    ) -> *mut drm_property;
    pub fn drm_property_create_signed_range(
        dev: *mut drm_device, flags: u32, name: *const core::ffi::c_char,
        min: i64, max: i64,
    ) -> *mut drm_property;
    pub fn drm_property_create_object(
        dev: *mut drm_device, flags: u32, name: *const core::ffi::c_char,
        type_: u32,
    ) -> *mut drm_property;
    pub fn drm_property_create_bool(
        dev: *mut drm_device, flags: u32, name: *const core::ffi::c_char,
    ) -> *mut drm_property;
    pub fn drm_property_add_enum(
        property: *mut drm_property, value: u64, name: *const core::ffi::c_char,
    ) -> core::ffi::c_int;
    pub fn drm_property_destroy(dev: *mut drm_device, property: *mut drm_property);
    pub fn drm_property_create_blob(
        dev: *mut drm_device, length: usize, data: *const core::ffi::c_void,
    ) -> *mut drm_property_blob;
    pub fn drm_property_lookup_blob(dev: *mut drm_device, id: u32) -> *mut drm_property_blob;
    pub fn drm_property_replace_blob_from_id(
        dev: *mut drm_device, blob: *mut *mut drm_property_blob, blob_id: u64,
        expected_size: isize, expected_elem_size: isize, max_size: isize,
        replaced: *mut bool,
    ) -> core::ffi::c_int;
    pub fn drm_property_replace_global_blob(
        dev: *mut drm_device, replace: *mut *mut drm_property_blob, length: usize,
        data: *const core::ffi::c_void, obj_holds_id: *mut drm_mode_object,
        prop_holds_id: *mut drm_property,
    ) -> core::ffi::c_int;
    pub fn drm_property_replace_blob(
        blob: *mut *mut drm_property_blob, new_blob: *mut drm_property_blob,
    ) -> bool;
    pub fn drm_property_blob_get(blob: *mut drm_property_blob) -> *mut drm_property_blob;
    pub fn drm_property_blob_put(blob: *mut drm_property_blob);
}

#[inline]
pub unsafe fn drm_property_find(
    dev: *mut drm_device, file_priv: *mut drm_file, id: u32,
) -> *mut drm_property {
    let mo = drm_mode_object_find(dev, file_priv, id, DRM_MODE_OBJECT_PROPERTY);
    if !mo.is_null() { obj_to_property!(mo) } else { core::ptr::null_mut() }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
