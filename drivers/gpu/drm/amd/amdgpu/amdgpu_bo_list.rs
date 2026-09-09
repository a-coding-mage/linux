/*
 * Copyright 2015 Advanced Micro Devices, Inc.
 * All Rights Reserved.
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
 * FITNESS FOR A PARTICULAR PURPOSE AND NON-INFRINGEMENT. IN NO EVENT SHALL
 * THE COPYRIGHT HOLDERS, AUTHORS AND/OR ITS SUPPLIERS BE LIABLE FOR ANY CLAIM,
 * DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR
 * OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE
 * USE OR OTHER DEALINGS IN THE SOFTWARE.
 */
/* Authors: Christian König <deathsimple@vodafone.de> */

const AMDGPU_BO_LIST_MAX_PRIORITY: u32 = 32;
const AMDGPU_BO_LIST_NUM_BUCKETS: u32 = AMDGPU_BO_LIST_MAX_PRIORITY + 1;
const AMDGPU_BO_LIST_MAX_ENTRIES: usize = 128 * 1024;

unsafe fn amdgpu_bo_list_free(ref_: *mut kref) {
    let list = container_of(ref_, amdgpu_bo_list, refcount);
    let mut e: *mut amdgpu_bo_list_entry;

    amdgpu_bo_list_for_each_entry!(e, list);
    while !e.is_null() {
        amdgpu_bo_unref(&mut (*e).bo);
        e = amdgpu_bo_list_next_entry(e, list);
    }
    kvfree(list as *mut core::ffi::c_void);
}

unsafe extern "C" fn amdgpu_bo_list_entry_cmp(_a: *const core::ffi::c_void,
                                                _b: *const core::ffi::c_void) -> i32 {
    let a = _a as *const amdgpu_bo_list_entry;
    let b = _b as *const amdgpu_bo_list_entry;

    // BUILD_BUG_ON(AMDGPU_BO_LIST_MAX_PRIORITY >= INT_MAX);
    ((*a).priority as i32).wrapping_sub((*b).priority as i32)
}

unsafe fn amdgpu_bo_list_create(adev: *mut amdgpu_device, filp: *mut drm_file,
                                info: *mut drm_amdgpu_bo_list_entry,
                                num_entries: usize) -> *mut amdgpu_bo_list {
    let mut last_entry: u32 = 0;
    let mut first_userptr = num_entries;
    let mut list: *mut amdgpu_bo_list;
    let mut total_size: u64 = 0;
    let mut r: i32;

    list = kvzalloc_flex::<amdgpu_bo_list, amdgpu_bo_list_entry>(num_entries);
    if list.is_null() { return ERR_PTR(-12); }
    kref_init(&mut (*list).refcount);
    (*list).num_entries = num_entries;
    let array = (*list).entries.as_mut_ptr();

    for i in 0..num_entries {
        let entry: *mut amdgpu_bo_list_entry;
        let gobj = drm_gem_object_lookup(filp, (*info.add(i)).bo_handle);
        if gobj.is_null() { r = -2; goto_error!(list, array, last_entry, first_userptr, num_entries, r); }
        let bo = amdgpu_bo_ref(gem_to_amdgpu_bo(gobj));
        drm_gem_object_put(gobj);
        let usermm = amdgpu_ttm_tt_get_usermm((*bo).tbo.ttm);
        if !usermm.is_null() {
            if usermm != (*current).mm {
                amdgpu_bo_unref(&mut (bo as *mut amdgpu_bo));
                r = -1; goto_error!(list, array, last_entry, first_userptr, num_entries, r);
            }
            first_userptr -= 1;
            entry = array.add(first_userptr);
        } else {
            entry = array.add(last_entry as usize);
            last_entry += 1;
        }
        (*entry).priority = core::cmp::min((*info.add(i)).bo_priority, AMDGPU_BO_LIST_MAX_PRIORITY);
        (*entry).bo = bo;
        if (*bo).preferred_domains == AMDGPU_GEM_DOMAIN_GDS { (*list).gds_obj = bo; }
        if (*bo).preferred_domains == AMDGPU_GEM_DOMAIN_GWS { (*list).gws_obj = bo; }
        if (*bo).preferred_domains == AMDGPU_GEM_DOMAIN_OA { (*list).oa_obj = bo; }
        total_size = total_size.wrapping_add(amdgpu_bo_size(bo));
        trace_amdgpu_bo_list_set!(list, bo);
    }
    (*list).first_userptr = first_userptr;
    sort(array as *mut core::ffi::c_void, last_entry as usize,
         core::mem::size_of::<amdgpu_bo_list_entry>(), amdgpu_bo_list_entry_cmp, core::ptr::null_mut());
    trace_amdgpu_cs_bo_status!((*list).num_entries, total_size);
    list
}

unsafe fn amdgpu_bo_list_get(fpriv: *mut amdgpu_fpriv, id: u32) -> *mut amdgpu_bo_list {
    xa_lock(&mut (*fpriv).bo_list_handles);
    let mut list = xa_load(&mut (*fpriv).bo_list_handles, id);
    if !list.is_null() { kref_get(&mut (*list).refcount); } else { list = ERR_PTR(-2); }
    xa_unlock(&mut (*fpriv).bo_list_handles);
    list
}

unsafe fn amdgpu_bo_list_put(list: *mut amdgpu_bo_list) {
    if !list.is_null() { kref_put(&mut (*list).refcount, amdgpu_bo_list_free); }
}

unsafe fn amdgpu_bo_create_list_entry_array(in_: *mut drm_amdgpu_bo_list_in)
    -> *mut drm_amdgpu_bo_list_entry {
    let uptr = u64_to_user_ptr((*in_).bo_info_ptr);
    let bo_number = (*in_).bo_number;
    if bo_number > AMDGPU_BO_LIST_MAX_ENTRIES as u32 ||
       (*in_).bo_info_size != core::mem::size_of::<drm_amdgpu_bo_list_entry>() { return ERR_PTR(-22); }
    vmemdup_array_user(uptr, bo_number as usize, core::mem::size_of::<drm_amdgpu_bo_list_entry>())
}

unsafe fn amdgpu_bo_list_ioctl(dev: *mut drm_device, data: *mut core::ffi::c_void,
                               filp: *mut drm_file) -> i32 {
    let fpriv = (*filp).driver_priv as *mut amdgpu_fpriv;
    let adev = drm_to_adev(dev);
    let args = data as *mut drm_amdgpu_bo_list;
    let mut handle = (*args).in_.list_handle;
    let mut list: *mut amdgpu_bo_list;
    let mut r: i32;
    match (*args).in_.operation {
        AMDGPU_BO_LIST_OP_CREATE | AMDGPU_BO_LIST_OP_UPDATE => {
            let info = amdgpu_bo_create_list_entry_array(&mut (*args).in_);
            if IS_ERR(info) { return PTR_ERR(info); }
            list = amdgpu_bo_list_create(adev, filp, info, (*args).in_.bo_number as usize);
            kvfree(info as *mut core::ffi::c_void);
            if IS_ERR(list) { return PTR_ERR(list); }
        }
        AMDGPU_BO_LIST_OP_DESTROY => { list = xa_erase(&mut (*fpriv).bo_list_handles, handle); amdgpu_bo_list_put(list); handle = 0; }
        _ => return -22,
    }
    match (*args).in_.operation {
        AMDGPU_BO_LIST_OP_CREATE => { r = xa_alloc(&mut (*fpriv).bo_list_handles, &mut handle, list, xa_limit_32b, GFP_KERNEL); if r != 0 { amdgpu_bo_list_put(list); return r; } }
        AMDGPU_BO_LIST_OP_UPDATE => {
            let curr = xa_load(&mut (*fpriv).bo_list_handles, handle);
            if curr.is_null() { amdgpu_bo_list_put(list); return -2; }
            let prev = xa_cmpxchg(&mut (*fpriv).bo_list_handles, handle, curr, list, GFP_KERNEL);
            if xa_is_err(prev) { r = xa_err(prev); amdgpu_bo_list_put(list); return r; }
            if prev != curr { amdgpu_bo_list_put(list); return -2; }
            amdgpu_bo_list_put(curr);
        }
        _ => {}
    }
    core::ptr::write_bytes(args, 0, 1);
    (*args).out.list_handle = handle;
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
