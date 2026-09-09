/*
 * Copyright 2011 Red Hat Inc.
 * All Rights Reserved.
 *
 * Permission is hereby granted, free of charge, to any person obtaining a
 * copy of this software and associated documentation files (the
 * "Software"), to deal in the Software without restriction, including
 * without limitation the rights to use, copy, modify, merge, publish,
 * distribute, sub license, and/or sell copies of the Software, and to permit
 * persons to whom the Software is furnished to do so, subject to the following
 * conditions:
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NON-INFRINGEMENT. IN NO EVENT SHALL
 * THE COPYRIGHT HOLDERS, AUTHORS AND/OR ITS SUPPLIERS BE LIABLE FOR ANY CLAIM,
 * DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR
 * OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE
 * USE OR OTHER DEALINGS IN THE SOFTWARE.
 */
/*
 * Authors:
 *    Jerome Glisse <glisse@freedesktop.org>
 */
/* Algorithm:
 *
 * We store the last allocated bo in "hole", we always try to allocate
 * after the last allocated bo. Principle is that in a linear GPU ring
 * progression was is after last is the oldest bo we allocated and thus
 * the first one that should no longer be in use by the GPU.
 *
 * If it's not the case we skip over the bo after last to the closest
 * done bo if such one exist. If none exist and we are not asked to
 * block we report failure to allocate.
 *
 * If we are asked to block we wait on all the oldest fence of all
 * rings. We just wait for any of those fence to complete.
 */

// Dependency declarations supplied by the surrounding kernel translation.

pub unsafe fn amdgpu_sa_bo_manager_init(
    adev: *mut amdgpu_device,
    sa_manager: *mut amdgpu_sa_manager,
    size: c_uint,
    gfp_flags: gfp_t,
) -> c_int {
    let r: c_int;

    (*sa_manager).gfp_flags = gfp_flags;
    r = amdgpu_bo_create_kernel(
        adev,
        size,
        AMDGPU_GPU_PAGE_SIZE,
        AMDGPU_GEM_DOMAIN_GTT,
        &mut (*sa_manager).bo,
        &mut (*sa_manager).gpu_addr,
        &mut (*sa_manager).cpu_ptr,
    );
    if r != 0 {
        dev_err(
            (*adev).dev,
            "(%d) failed to allocate bo for manager\n",
            r,
        );
        return r;
    }

    memset((*sa_manager).cpu_ptr, 0, size as usize);
    drm_suballoc_manager_init(&mut (*sa_manager).base, size, 256);

    r
}

pub unsafe fn amdgpu_sa_bo_manager_fini(
    adev: *mut amdgpu_device,
    sa_manager: *mut amdgpu_sa_manager,
) {
    if (*sa_manager).bo.is_null() {
        dev_err((*adev).dev, "no bo for sa manager\n");
        return;
    }

    drm_suballoc_manager_fini(&mut (*sa_manager).base);

    amdgpu_bo_free_kernel(
        &mut (*sa_manager).bo,
        &mut (*sa_manager).gpu_addr,
        &mut (*sa_manager).cpu_ptr,
    );
}

pub unsafe fn amdgpu_sa_bo_new(
    sa_manager: *mut amdgpu_sa_manager,
    sa_bo: *mut *mut drm_suballoc,
    size: c_uint,
) -> c_int {
    let sa = drm_suballoc_new(
        &mut (*sa_manager).base,
        size,
        (*sa_manager).gfp_flags,
        false,
        0,
    );

    if IS_ERR(sa) {
        *sa_bo = core::ptr::null_mut();
        return PTR_ERR(sa);
    }

    *sa_bo = sa;
    0
}

pub unsafe fn amdgpu_sa_bo_free(
    sa_bo: *mut *mut drm_suballoc,
    fence: *mut dma_fence,
) {
    if sa_bo.is_null() || (*sa_bo).is_null() {
        return;
    }

    drm_suballoc_free(*sa_bo, fence);
    *sa_bo = core::ptr::null_mut();
}

#[cfg(CONFIG_DEBUG_FS)]
pub unsafe fn amdgpu_sa_bo_dump_debug_info(
    sa_manager: *mut amdgpu_sa_manager,
    m: *mut seq_file,
) {
    let mut p = drm_seq_file_printer(m);

    drm_suballoc_dump_debug_info(&mut (*sa_manager).base, &mut p, (*sa_manager).gpu_addr);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
