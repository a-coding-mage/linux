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
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL
 * THE COPYRIGHT HOLDER(S) OR AUTHOR(S) BE LIABLE FOR ANY CLAIM, DAMAGES OR
 * OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
 * ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
 * OTHER DEALINGS IN THE SOFTWARE.
 */

use core::ffi::c_void;

#[repr(C)]
pub struct dc_context {
    _private: [u8; 0],
}

#[repr(C)]
pub struct vector {
    pub container: *mut u8,
    pub capacity: u32,
    pub struct_size: u32,
    pub count: u32,
    pub ctx: *mut dc_context,
}

extern "C" {
    fn kcalloc(n: usize, size: usize, flags: u32) -> *mut c_void;
    fn kzalloc(size: usize, flags: u32) -> *mut c_void;
    fn krealloc_array(ptr: *mut c_void, n: usize, size: usize, flags: u32) -> *mut c_void;
    fn kfree(ptr: *mut c_void);
    fn memmove(dest: *mut c_void, src: *const c_void, n: usize) -> *mut c_void;
    fn break_to_debugger();
}

const GFP_KERNEL: u32 = 0;

pub unsafe fn dal_vector_construct(
    vector: *mut vector,
    ctx: *mut dc_context,
    capacity: u32,
    struct_size: u32,
) -> bool {
    (*vector).container = core::ptr::null_mut();

    if struct_size == 0 || capacity == 0 {
        break_to_debugger();
        return false;
    }

    (*vector).container = kcalloc(capacity as usize, struct_size as usize, GFP_KERNEL) as *mut u8;
    if (*vector).container.is_null() {
        return false;
    }
    (*vector).capacity = capacity;
    (*vector).struct_size = struct_size;
    (*vector).count = 0;
    (*vector).ctx = ctx;
    true
}

unsafe fn dal_vector_presized_costruct(
    vector: *mut vector,
    _ctx: *mut dc_context,
    count: u32,
    initial_value: *mut c_void,
    struct_size: u32,
) -> bool {
    (*vector).container = core::ptr::null_mut();
    if struct_size == 0 || count == 0 {
        break_to_debugger();
        return false;
    }
    (*vector).container = kcalloc(count as usize, struct_size as usize, GFP_KERNEL) as *mut u8;
    if (*vector).container.is_null() {
        return false;
    }
    if !initial_value.is_null() {
        for i in 0..count {
            memmove(
                (*vector).container.add((i * struct_size) as usize) as *mut c_void,
                initial_value,
                struct_size as usize,
            );
        }
    }
    (*vector).capacity = count;
    (*vector).struct_size = struct_size;
    (*vector).count = count;
    true
}

pub unsafe fn dal_vector_presized_create(ctx: *mut dc_context, size: u32, initial_value: *mut c_void, struct_size: u32) -> *mut vector {
    let vector = kzalloc(core::mem::size_of::<vector>(), GFP_KERNEL) as *mut vector;
    if vector.is_null() { return core::ptr::null_mut(); }
    if dal_vector_presized_costruct(vector, ctx, size, initial_value, struct_size) { return vector; }
    break_to_debugger();
    kfree(vector as *mut c_void);
    core::ptr::null_mut()
}

pub unsafe fn dal_vector_create(ctx: *mut dc_context, capacity: u32, struct_size: u32) -> *mut vector {
    let vector = kzalloc(core::mem::size_of::<vector>(), GFP_KERNEL) as *mut vector;
    if vector.is_null() { return core::ptr::null_mut(); }
    if dal_vector_construct(vector, ctx, capacity, struct_size) { return vector; }
    break_to_debugger();
    kfree(vector as *mut c_void);
    core::ptr::null_mut()
}

pub unsafe fn dal_vector_destruct(vector: *mut vector) {
    kfree((*vector).container as *mut c_void);
    (*vector).count = 0;
    (*vector).capacity = 0;
}

pub unsafe fn dal_vector_destroy(vector: *mut *mut vector) {
    if vector.is_null() || (*vector).is_null() { return; }
    dal_vector_destruct(*vector);
    kfree(*vector as *mut c_void);
    *vector = core::ptr::null_mut();
}

pub unsafe fn dal_vector_get_count(vector: *const vector) -> u32 { (*vector).count }

pub unsafe fn dal_vector_at_index(vector: *const vector, index: u32) -> *mut u8 {
    if (*vector).container.is_null() || index >= (*vector).count { return core::ptr::null_mut(); }
    (*vector).container.add((index * (*vector).struct_size) as usize)
}

pub unsafe fn dal_vector_remove_at_index(vector: *mut vector, index: u32) -> bool {
    if index >= (*vector).count { return false; }
    if index != (*vector).count - 1 {
        memmove(
            (*vector).container.add((index * (*vector).struct_size) as usize) as *mut c_void,
            (*vector).container.add(((index + 1) * (*vector).struct_size) as usize) as *const c_void,
            ((*vector).count - index - 1) as usize * (*vector).struct_size as usize,
        );
    }
    (*vector).count -= 1;
    true
}

pub unsafe fn dal_vector_set_at_index(vector: *const vector, what: *const c_void, index: u32) {
    let where_ = dal_vector_at_index(vector, index);
    if where_.is_null() { break_to_debugger(); return; }
    memmove(where_ as *mut c_void, what, (*vector).struct_size as usize);
}

#[inline]
unsafe fn calc_increased_capacity(old_capacity: u32) -> u32 { old_capacity.wrapping_mul(2) }

pub unsafe fn dal_vector_insert_at(vector: *mut vector, what: *const c_void, position: u32) -> bool {
    if (*vector).count == (*vector).capacity && !dal_vector_reserve(vector, calc_increased_capacity((*vector).capacity)) { return false; }
    let insert_address = (*vector).container.add(((*vector).struct_size * position) as usize);
    if (*vector).count != 0 && position < (*vector).count {
        memmove(
            insert_address.add((*vector).struct_size as usize) as *mut c_void,
            insert_address as *const c_void,
            ((*vector).struct_size * ((*vector).count - position)) as usize,
        );
    }
    memmove(insert_address as *mut c_void, what, (*vector).struct_size as usize);
    (*vector).count += 1;
    true
}

pub unsafe fn dal_vector_append(vector: *mut vector, item: *const c_void) -> bool {
    dal_vector_insert_at(vector, item, (*vector).count)
}

pub unsafe fn dal_vector_clone(vector: *const vector) -> *mut vector {
    let count = dal_vector_get_count(vector);
    let vec_cloned = if count == 0 {
        dal_vector_create((*vector).ctx, (*vector).capacity, (*vector).struct_size)
    } else {
        dal_vector_presized_create((*vector).ctx, count, core::ptr::null_mut(), (*vector).struct_size)
    };
    if vec_cloned.is_null() { break_to_debugger(); return core::ptr::null_mut(); }
    memmove((*vec_cloned).container as *mut c_void, (*vector).container as *const c_void,
        (*vec_cloned).struct_size as usize * (*vec_cloned).capacity as usize);
    vec_cloned
}

pub unsafe fn dal_vector_capacity(vector: *const vector) -> u32 { (*vector).capacity }

pub unsafe fn dal_vector_reserve(vector: *mut vector, capacity: u32) -> bool {
    if capacity <= (*vector).capacity { return true; }
    let new_container = krealloc_array((*vector).container as *mut c_void, capacity as usize, (*vector).struct_size as usize, GFP_KERNEL);
    if !new_container.is_null() {
        (*vector).container = new_container as *mut u8;
        (*vector).capacity = capacity;
        return true;
    }
    false
}

pub unsafe fn dal_vector_clear(vector: *mut vector) { (*vector).count = 0; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
