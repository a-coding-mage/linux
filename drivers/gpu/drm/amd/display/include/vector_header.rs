/*
 * Copyright 2012-15 Advanced Micro Devices, Inc.
 *
 * Permission is hereby granted, free of charge, to any person obtaining a
 * copy of this software and associated documentation files (the "Software"),
 * to deal in the Software without restriction, including without limitation
 * the rights to use, copy, modify, merge, publish, distribute, sublicense,
 * and/or sell copies of the Software, and to permit persons to whom the
 * Software is furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in
 * all copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.  IN NO EVENT SHALL
 * THE COPYRIGHT HOLDER(S) OR AUTHOR(S) BE LIABLE FOR ANY CLAIM, DAMAGES OR
 * OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
 * ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
 * OTHER DEALINGS IN THE SOFTWARE.
 *
 * Authors: AMD
 */

#[repr(C)]
pub struct vector {
    pub container: *mut u8,
    pub struct_size: u32,
    pub count: u32,
    pub capacity: u32,
    pub ctx: *mut dc_context,
}

#[repr(C)]
pub struct dc_context {
    _private: [u8; 0],
}

extern "C" {
    pub fn dal_vector_construct(
        vector: *mut vector,
        ctx: *mut dc_context,
        capacity: u32,
        struct_size: u32,
    ) -> bool;

    pub fn dal_vector_create(
        ctx: *mut dc_context,
        capacity: u32,
        struct_size: u32,
    ) -> *mut vector;

    /* 'initial_value' is optional. If initial_value not supplied,
     * each "structure" in the vector will contain zeros by default. */
    pub fn dal_vector_presized_create(
        ctx: *mut dc_context,
        size: u32,
        initial_value: *mut core::ffi::c_void,
        struct_size: u32,
    ) -> *mut vector;

    pub fn dal_vector_destruct(vector: *mut vector);

    pub fn dal_vector_destroy(vector: *mut *mut vector);

    pub fn dal_vector_get_count(vector: *const vector) -> u32;

    /* dal_vector_insert_at
     * reallocate container if necessary
     * then shell items at right and insert
     * return if the container modified
     * do not check that index belongs to container
     * since the function is private and index is going to be calculated
     * either with by function or as get_count+1 */
    pub fn dal_vector_insert_at(
        vector: *mut vector,
        what: *const core::ffi::c_void,
        position: u32,
    ) -> bool;

    pub fn dal_vector_append(
        vector: *mut vector,
        item: *const core::ffi::c_void,
    ) -> bool;

    /* operator[] */
    pub fn dal_vector_at_index(
        vector: *const vector,
        index: u32,
    ) -> *mut core::ffi::c_void;

    pub fn dal_vector_set_at_index(
        vector: *const vector,
        what: *const core::ffi::c_void,
        index: u32,
    );

    /* create a clone (copy) of a vector */
    pub fn dal_vector_clone(vector_other: *const vector) -> *mut vector;

    /* dal_vector_remove_at_index
     * Shifts elements on the right from remove position to the left,
     * removing an element at position by overwrite means*/
    pub fn dal_vector_remove_at_index(vector: *mut vector, index: u32) -> bool;

    pub fn dal_vector_capacity(vector: *const vector) -> u32;

    pub fn dal_vector_reserve(vector: *mut vector, capacity: u32) -> bool;

    pub fn dal_vector_clear(vector: *mut vector);
}

/**
 * Macro definitions of TYPE-SAFE versions of vector set/get functions.
 */

#[macro_export]
macro_rules! DAL_VECTOR_INSERT_AT {
    ($function_name:ident, $type_t:ty) => {
        fn $function_name(
            vector: *mut $crate::vector,
            what: $type_t,
            position: u32,
        ) -> bool {
            unsafe {
                $crate::dal_vector_insert_at(
                    vector,
                    what as *const _ as *const core::ffi::c_void,
                    position,
                )
            }
        }
    };
}

#[macro_export]
macro_rules! DAL_VECTOR_APPEND {
    ($function_name:ident, $type_t:ty) => {
        fn $function_name(
            vector: *mut $crate::vector,
            item: $type_t,
        ) -> bool {
            unsafe {
                $crate::dal_vector_append(
                    vector,
                    item as *const _ as *const core::ffi::c_void,
                )
            }
        }
    };
}

#[macro_export]
macro_rules! DAL_VECTOR_AT_INDEX {
    ($function_name:ident, $type_t:ty) => {
        fn $function_name(
            vector: *const $crate::vector,
            index: u32,
        ) -> $type_t {
            unsafe {
                $crate::dal_vector_at_index(vector, index) as *mut $type_t
            }
        }
    };
}

#[macro_export]
macro_rules! DAL_VECTOR_SET_AT_INDEX {
    ($function_name:ident, $type_t:ty) => {
        fn $function_name(
            vector: *const $crate::vector,
            what: $type_t,
            index: u32,
        ) {
            unsafe {
                $crate::dal_vector_set_at_index(
                    vector,
                    what as *const _ as *const core::ffi::c_void,
                    index,
                );
            }
        }
    };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
