/*
 * Copyright 2018 Advanced Micro Devices, Inc.
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
 */

// Dependency declarations supplied by other headers/files:
// struct hmm_range;
// struct drm_file;
// struct amdgpu_device;
// struct amdgpu_bo;
// struct amdgpu_bo_va;
// struct amdgpu_fpriv;
// struct amdgpu_hmm_range;
// struct kref;
// struct drm_amdgpu_bo_list_entry;
// struct drm_amdgpu_bo_list_in;

#[repr(C)]
pub struct amdgpu_bo_list_entry {
    pub bo: *mut amdgpu_bo,
    pub bo_va: *mut amdgpu_bo_va,
    pub priority: u32,
    pub range: *mut amdgpu_hmm_range,
    pub user_invalidated: bool,
}

#[repr(C)]
pub struct amdgpu_bo_list {
    pub refcount: kref,
    pub gds_obj: *mut amdgpu_bo,
    pub gws_obj: *mut amdgpu_bo,
    pub oa_obj: *mut amdgpu_bo,
    pub first_userptr: u32,
    pub num_entries: u32,
    // Flexible array member, counted by num_entries in the C source.
    pub entries: [amdgpu_bo_list_entry; 0],
}

unsafe extern "C" {
    pub fn amdgpu_bo_list_get(fpriv: *mut amdgpu_fpriv, id: u32) -> *mut amdgpu_bo_list;
    pub fn amdgpu_bo_list_put(list: *mut amdgpu_bo_list);
    pub fn amdgpu_bo_create_list_entry_array(
        input: *mut drm_amdgpu_bo_list_in,
    ) -> *mut drm_amdgpu_bo_list_entry;
    pub fn amdgpu_bo_list_create(
        adev: *mut amdgpu_device,
        filp: *mut drm_file,
        info: *mut drm_amdgpu_bo_list_entry,
        num_entries: usize,
    ) -> *mut amdgpu_bo_list;
}

#[macro_export]
macro_rules! amdgpu_bo_list_for_each_entry {
    ($e:ident, $list:expr, $body:block) => {
        for $e in unsafe {
            core::slice::from_raw_parts_mut(
                ($list).entries.as_mut_ptr(),
                ($list).num_entries as usize,
            )
        } {
            $body
        }
    };
}

#[macro_export]
macro_rules! amdgpu_bo_list_for_each_userptr_entry {
    ($e:ident, $list:expr, $body:block) => {
        for $e in unsafe {
            core::slice::from_raw_parts_mut(
                ($list).entries.as_mut_ptr().add(($list).first_userptr as usize),
                (($list).num_entries - ($list).first_userptr) as usize,
            )
        } {
            $body
        }
    };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
