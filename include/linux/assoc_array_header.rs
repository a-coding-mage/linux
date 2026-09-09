/* SPDX-License-Identifier: GPL-2.0-or-later */
/* Generic associative array implementation.
 *
 * See Documentation/core-api/assoc_array.rst for information.
 *
 * Copyright (C) 2013 Red Hat, Inc. All Rights Reserved.
 * Written by David Howells (dhowells@redhat.com)
 */

/* Translated from the C header.  The CONFIG_ASSOCIATIVE_ARRAY condition is a
 * build-time condition supplied by the surrounding kernel configuration. */

#[cfg(CONFIG_ASSOCIATIVE_ARRAY)]
pub const ASSOC_ARRAY_KEY_CHUNK_SIZE: usize = BITS_PER_LONG;
/* Key data retrieved in chunks of this size. */

/* External type supplied by the associative-array implementation. */
#[cfg(CONFIG_ASSOCIATIVE_ARRAY)]
#[repr(C)]
pub struct assoc_array_ptr {
    _private: [u8; 0],
}

/* Generic associative array. */
#[cfg(CONFIG_ASSOCIATIVE_ARRAY)]
#[repr(C)]
pub struct assoc_array {
    /* The node at the root of the tree */
    pub root: *mut assoc_array_ptr,
    pub nr_leaves_on_tree: ::core::ffi::c_ulong,
}

/* Operations on objects and index keys for use by array manipulation routines. */
#[cfg(CONFIG_ASSOCIATIVE_ARRAY)]
#[repr(C)]
pub struct assoc_array_ops {
    /* Method to get a chunk of an index key from caller-supplied data */
    pub get_key_chunk:
        Option<unsafe extern "C" fn(index_key: *const ::core::ffi::c_void, level: ::core::ffi::c_int) -> ::core::ffi::c_ulong>,

    /* Method to get a piece of an object's index key */
    pub get_object_key_chunk:
        Option<unsafe extern "C" fn(object: *const ::core::ffi::c_void, level: ::core::ffi::c_int) -> ::core::ffi::c_ulong>,

    /* Is this the object we're looking for? */
    pub compare_object: Option<unsafe extern "C" fn(
        object: *const ::core::ffi::c_void,
        index_key: *const ::core::ffi::c_void,
    ) -> bool>,

    /* How different is an object from an index key, to a bit position in
     * their keys? (or -1 if they're the same)
     */
    pub diff_objects: Option<unsafe extern "C" fn(
        object: *const ::core::ffi::c_void,
        index_key: *const ::core::ffi::c_void,
    ) -> ::core::ffi::c_int>,

    /* Method to free an object. */
    pub free_object: Option<unsafe extern "C" fn(object: *mut ::core::ffi::c_void)>,
}

/* Access and manipulation functions. */
#[cfg(CONFIG_ASSOCIATIVE_ARRAY)]
#[repr(C)]
pub struct assoc_array_edit {
    _private: [u8; 0],
}

#[cfg(CONFIG_ASSOCIATIVE_ARRAY)]
#[inline]
pub unsafe fn assoc_array_init(array: *mut assoc_array) {
    (*array).root = ::core::ptr::null_mut();
    (*array).nr_leaves_on_tree = 0;
}

#[cfg(CONFIG_ASSOCIATIVE_ARRAY)]
extern "C" {
    pub fn assoc_array_iterate(
        array: *const assoc_array,
        iterator: Option<unsafe extern "C" fn(
            object: *const ::core::ffi::c_void,
            iterator_data: *mut ::core::ffi::c_void,
        ) -> ::core::ffi::c_int>,
        iterator_data: *mut ::core::ffi::c_void,
    ) -> ::core::ffi::c_int;
    pub fn assoc_array_find(
        array: *const assoc_array,
        ops: *const assoc_array_ops,
        index_key: *const ::core::ffi::c_void,
    ) -> *mut ::core::ffi::c_void;
    pub fn assoc_array_destroy(array: *mut assoc_array, ops: *const assoc_array_ops);
    pub fn assoc_array_insert(
        array: *mut assoc_array,
        ops: *const assoc_array_ops,
        index_key: *const ::core::ffi::c_void,
        object: *mut ::core::ffi::c_void,
    ) -> *mut assoc_array_edit;
    pub fn assoc_array_insert_set_object(edit: *mut assoc_array_edit, object: *mut ::core::ffi::c_void);
    pub fn assoc_array_delete(
        array: *mut assoc_array,
        ops: *const assoc_array_ops,
        index_key: *const ::core::ffi::c_void,
    ) -> *mut assoc_array_edit;
    pub fn assoc_array_clear(array: *mut assoc_array, ops: *const assoc_array_ops) -> *mut assoc_array_edit;
    pub fn assoc_array_apply_edit(edit: *mut assoc_array_edit);
    pub fn assoc_array_cancel_edit(edit: *mut assoc_array_edit);
    pub fn assoc_array_gc(
        array: *mut assoc_array,
        ops: *const assoc_array_ops,
        iterator: Option<unsafe extern "C" fn(
            object: *mut ::core::ffi::c_void,
            iterator_data: *mut ::core::ffi::c_void,
        ) -> bool>,
        iterator_data: *mut ::core::ffi::c_void,
    ) -> ::core::ffi::c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
