/*
 * videobuf2-memops.h - generic memory handling routines for videobuf2
 *
 * Copyright (C) 2010 Samsung Electronics
 *
 * Author: Pawel Osciak <pawel@osciak.com>
 *        Marek Szyprowski <m.szyprowski@samsung.com>
 *
 * This program is free software; you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation.
 */

// Dependencies supplied by the surrounding kernel translation.
use core::ffi::c_void;

#[repr(C)]
pub struct refcount_t {
    _private: [u8; 0],
}

#[repr(C)]
pub struct vm_operations_struct {
    _private: [u8; 0],
}

#[repr(C)]
pub struct frame_vector {
    _private: [u8; 0],
}

/**
 * struct vb2_vmarea_handler - common vma refcount tracking handler.
 *
 * @refcount: pointer to &refcount_t entry in the buffer.
 * @put:      callback to function that decreases buffer refcount.
 * @arg:      argument for @put callback.
 */
#[repr(C)]
pub struct vb2_vmarea_handler {
    pub refcount: *mut refcount_t,
    pub put: Option<unsafe extern "C" fn(arg: *mut c_void)>,
    pub arg: *mut c_void,
}

unsafe extern "C" {
    pub static vb2_common_vm_ops: vm_operations_struct;

    pub fn vb2_create_framevec(
        start: usize,
        length: usize,
        write: bool,
    ) -> *mut frame_vector;

    pub fn vb2_destroy_framevec(vec: *mut frame_vector);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
